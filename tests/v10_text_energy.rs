//! Energy in text (Origin, 2026-11-27, 117th: *"energy is maintained as the
//! subject is continued and lost as the subject departs, although it may be
//! regained depending upon how that subsequent subject is pursued or not"*;
//! *"with the field so far as the input and generated text together
//! expanding as it expands"*).
//!
//! The carrier: each step registers a signed movement on the synapse that
//! fires, +1 where the synapse has been stimulated in the field so far, −1
//! where it has not. Energy is the net movement accumulated over the
//! successive registrations (W5, 105th).
//!
//! What is shown: (1) the carried movement and the energy are exactly as
//! declared, recomputed here from the text itself at both scales; (2) the
//! field so far expands as it expands: input and continuation read as one
//! field give the same energy as the two read as one text, and the
//! continuation's own steps stimulate synapses it fires again (a subject
//! pursued regains); (3) energy is relative to the field: the same passage
//! carries different movement after a different input; (4) the energy is
//! linear in the carried movement (reversed, it reverses; two summed, sum).
//!
//! [IMPL] Actual text: *Pride and Prejudice* (`tests/data/`). **What these
//! establish:** that energy is formed as declared; not whether its
//! trajectory separates a continuation that sustains the subject from one
//! that departs (measured in `examples/v10_text_energy.rs`). **Not evaluated
//! (named):** the firing rule (open).

#[path = "support/text_field.rs"]
mod text;

use std::collections::{HashMap, HashSet};

use abr_kernel::radial::RadialField;
use text::{project, words};

const BOOK: &str = include_str!("data/pride_and_prejudice.txt");

fn novel() -> &'static str {
    let start = BOOK.find("It is a truth universally acknowledged").unwrap();
    let end = BOOK.find("*** END OF THE PROJECT GUTENBERG").unwrap_or(BOOK.len());
    &BOOK[start..end]
}

fn paragraph(n: &str, at: usize) -> usize {
    let mut at = at;
    while !n.is_char_boundary(at) { at += 1; }
    n[at..].find("\r\n\r\n").map(|e| at + e + 4).unwrap()
}

fn after_chars(s: &str, at: usize, k: usize) -> usize { s[at..].char_indices().nth(k).map(|(i, _)| at + i).unwrap() }

/// An input passage near `at`, (a) the book's next passage, (b) a passage
/// half the book further on (the 116th amended's fixture).
fn fixture(at: usize, input: usize, cont: usize) -> (&'static str, &'static str, &'static str) {
    let n = novel();
    let s = paragraph(n, at);
    let e = after_chars(n, s, input);
    let far = paragraph(n, (s + n.len() / 2) % (n.len() - 4 * cont));
    (&n[s..e], &n[e..after_chars(n, e, cont)], &n[far..after_chars(n, far, cont)])
}

/// One field over a sequence of elements declared as observed.
fn field<T: Clone + Eq + std::hash::Hash>(seq: &[T]) -> (Vec<usize>, RadialField) {
    let mut index: HashMap<T, usize> = HashMap::new();
    let mut r = RadialField::new(0);
    let ids: Vec<usize> = seq.iter().map(|t| *index.entry(t.clone()).or_insert_with(|| r.declare())).collect();
    for w in ids.windows(2) { r.push(w[0], w[1]).unwrap(); }
    (ids, r)
}

/// The declared carrier and energy, from the elements in order: +1 where
/// the pair (this element, the next) has occurred earlier, −1 where not;
/// the energy after each step the running sum.
fn reference(ids: &[usize]) -> (Vec<i64>, Vec<i64>) {
    let mut seen = HashSet::new();
    let mut carried = Vec::new();
    for w in ids.windows(2) { carried.push(if seen.insert((w[0], w[1])) { -1 } else { 1 }); }
    let net = carried.iter().scan(0, |s, &c| { *s += c; Some(*s) }).collect();
    (carried, net)
}

/// 1. The carried movement and the energy are as declared, at the character
/// and the word scale.
#[test]
fn energy_is_formed_as_declared() {
    for at in [100_000, 350_000, 600_000] {
        let (input, a, b) = fixture(at, 4000, 1000);
        for cont in [a, b] {
            let joined = format!("{input}{cont}");
            let (ci, cr) = field(&project(&joined));
            let (wi, wr) = field(&words(&joined));
            for (ids, r) in [(&ci, &cr), (&wi, &wr)] {
                let e = r.text_energy();
                let (carried, net) = reference(ids);
                assert_eq!(e.carried, carried, "the carried movement, step by step");
                assert_eq!(e.net, net, "the energy, the net movement accumulated");
            }
        }
    }
}

/// 2. The field so far expands as it expands: the continuation registered
/// after the input carries what the two read as one text carry; and a
/// synapse the continuation first stimulates carries +1 when it fires again
/// (the continuation's own subject regains), at the word scale.
#[test]
fn the_field_so_far_expands_as_it_expands() {
    let (input, _, b) = fixture(250_000, 4000, 1000);
    let (iw, bw) = (words(input), words(b));
    let joined: Vec<String> = iw.iter().chain(&bw).cloned().collect();
    let (ids, r) = field(&joined);
    let e = r.text_energy();
    let start = iw.len() - 1; // the first step of the continuation leaves the input's last word
    let first_in_cont: HashSet<(usize, usize)> = ids[..start + 1].windows(2).map(|w| (w[0], w[1])).collect();
    let mut regained = 0;
    let mut met = HashSet::new();
    for p in start..ids.len() - 1 {
        let s = (ids[p], ids[p + 1]);
        if !first_in_cont.contains(&s) {
            if !met.insert(s) { assert_eq!(e.carried[p], 1, "a synapse the continuation stimulated carries +1 again"); regained += 1; }
            else { assert_eq!(e.carried[p], -1, "first stimulated by the continuation"); }
        }
    }
    assert!(regained > 0, "the continuation stimulates synapses it fires again");
}

/// 3. Energy is relative to the field: the same passage carries different
/// movement after a different input.
#[test]
fn energy_is_relative_to_the_field_so_far() {
    let (i1, a, _) = fixture(150_000, 4000, 1000);
    let (i2, _, _) = fixture(550_000, 4000, 1000);
    let carried_after = |input: &str| {
        let iw = words(input);
        let joined: Vec<String> = iw.iter().chain(&words(a)).cloned().collect();
        let (_, r) = field(&joined);
        r.text_energy().carried[iw.len()..].to_vec()
    };
    assert_ne!(carried_after(i1), carried_after(i2));
}

/// 4. The energy is linear in the carried movement: reversing every carried
/// step reverses the energy, and the energy of two passages' movements
/// summed is the sum (104th; `v10_no_scalar`).
#[test]
fn energy_is_linear_in_the_carried_movement() {
    let (input, a, b) = fixture(450_000, 3000, 800);
    let (_, ra) = field(&words(&format!("{input}{a}")));
    let (_, rb) = field(&words(&format!("{input}{b}")));
    let (ea, eb) = (ra.text_energy(), rb.text_energy());
    let neg: Vec<i64> = ea.carried.iter().map(|c| -c).collect();
    assert_eq!(abr_kernel::radial::accumulate(&neg), ea.net.iter().map(|x| -x).collect::<Vec<_>>());
    let k = ea.carried.len().min(eb.carried.len());
    let sum: Vec<i64> = (0..k).map(|p| ea.carried[p] + eb.carried[p]).collect();
    assert_eq!(abr_kernel::radial::accumulate(&sum), (0..k).map(|p| ea.net[p] + eb.net[p]).collect::<Vec<_>>());
}
