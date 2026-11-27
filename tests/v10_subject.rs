//! The subject sustained over one evolving field (the 116th amended, Origin,
//! 2026-11-27: *"the simultaneous calculation of 26 radial graphs via the
//! operators including energy, freedom, and connection all of which are
//! described by the relation of the inputted text to the generated text and
//! its sustenance with respect to the subject and the forwards inevitable
//! relentless progression of the relational evolution of the field"*;
//! *"There are no independent synapses everything is calculated together"*).
//!
//! Each input or emitted character advances one ordered relational field;
//! the radial graphs, one per character (113th), are simultaneous views of
//! it. What is shown: (1) the readings of the synapse that fires, as the
//! whole field stands, are the operators' definitions on every radial graph
//! together, recomputed here from the registered steps; (2) at every step
//! the whole field changes together: the step enters every spoke's passage,
//! and the change on every spoke is fixed by the step, so no synapse is read
//! apart; (3) a continuation registers into the same field as the input,
//! and the field reads both back exactly (113th); (4) the experiment's two
//! continuations are actual text of equal length: (a) the book's own next
//! passage, (b) a passage from a distant chapter (spliced; real text, never
//! nonsense, 111th; Origin to judge it a fair stand-in for drifting off the
//! subject).
//!
//! [IMPL] Actual text: *Pride and Prejudice* (`tests/data/`). **What these
//! establish:** that the field and its readings are formed as stated over
//! input and continuation together; not whether any reading separates (a)
//! from (b) (measured in `examples/v10_subject.rs`). **Not evaluated
//! (named):** energy beyond the step (text's carrier is undeclared); the
//! firing rule (open).

#[path = "support/text_field.rs"]
mod text;

use std::collections::HashMap;

use abr_kernel::radial::RadialField;
use text::{project, read_back};

const BOOK: &str = include_str!("data/pride_and_prejudice.txt");

/// The novel, from its first sentence to its last.
fn novel() -> &'static str {
    let start = BOOK.find("It is a truth universally acknowledged").unwrap();
    let end = BOOK.find("*** END OF THE PROJECT GUTENBERG").unwrap_or(BOOK.len());
    &BOOK[start..end]
}

/// The start of the paragraph at or after byte `at` of the novel.
fn paragraph(novel: &str, at: usize) -> usize {
    let mut at = at;
    while !novel.is_char_boundary(at) { at += 1; }
    novel[at..].find("\r\n\r\n").map(|e| at + e + 4).unwrap()
}

/// The `n`th character from byte `at`, as a byte offset (found by character:
/// the text holds characters wider than one byte).
fn after_chars(s: &str, at: usize, n: usize) -> usize {
    s[at..].char_indices().nth(n).map(|(i, _)| at + i).unwrap()
}

/// The experiment's fixture: an input passage of `input` characters starting
/// at a paragraph near `at`, (a) the next `cont` characters of the book, and
/// (b) `cont` characters from the paragraph half the book further on.
fn fixture(at: usize, input: usize, cont: usize) -> (&'static str, &'static str, &'static str) {
    let n = novel();
    let s = paragraph(n, at);
    let e = after_chars(n, s, input);
    let a_end = after_chars(n, e, cont);
    let far = paragraph(n, (s + n.len() / 2) % (n.len() - 4 * cont));
    let b_end = after_chars(n, far, cont);
    (&n[s..e], &n[e..a_end], &n[far..b_end])
}

/// One field over input then continuation, its elements declared as observed
/// (an open vocabulary, 113th), every element numbered in order of first use.
fn field(parts: &[&str]) -> (Vec<usize>, Vec<char>, RadialField) {
    let mut index: HashMap<char, usize> = HashMap::new();
    let mut chars = Vec::new();
    let mut r = RadialField::new(0);
    let mut ids = Vec::new();
    for p in parts {
        for c in project(p) {
            ids.push(*index.entry(c).or_insert_with(|| { chars.push(c); r.declare() }));
        }
    }
    for w in ids.windows(2) { r.push(w[0], w[1]).unwrap(); }
    (ids, chars, r)
}

/// 1. The readings of the synapse that fires, as the whole field stands at
/// each registration, are the operators' definitions on every radial graph
/// together: A[x→y] the movement registered on the spoke; the readback's
/// halves, what has left x and what has entered y; B[x→y] = A[x→y] +
/// Σ_z A[y→z]; Ξ[x→y] = Σ_z B[y→z] − Σ_w B[w→x]. Recomputed here from the
/// registered steps by summing over every spoke, never from the field's code.
#[test]
fn the_synapse_is_read_on_the_whole_field() {
    let (input, a, _) = fixture(200_000, 500, 200);
    let (ids, _, full) = field(&[input, a]);
    let n = full.n();
    let mut r = RadialField::new(n);
    let mut am = vec![vec![0i64; n]; n];
    for w in ids.windows(2) {
        let (x, y) = (w[0], w[1]);
        let s = r.synapse(x, y);
        let b = |u: usize, v: usize| am[u][v] + (0..n).map(|z| am[v][z]).sum::<i64>();
        let xi = (0..n).map(|z| b(y, z)).sum::<i64>() - (0..n).map(|w| b(w, x)).sum::<i64>();
        assert_eq!((s.from, s.to), (x, y));
        assert_eq!(s.a, am[x][y], "A on the synapse");
        assert_eq!(s.leaving, (0..n).map(|z| am[x][z]).sum::<i64>(), "what has left x");
        assert_eq!(s.entering, (0..n).map(|w| am[w][y]).sum::<i64>(), "what has entered y");
        assert_eq!(s.b, b(x, y), "B on the synapse");
        assert_eq!(s.passage, xi, "Ξ on the synapse");
        r.push(x, y).unwrap();
        am[x][y] += 1;
    }
}

/// 2. At every step the whole field changes together: for the step x→y,
/// every spoke u→v changes its A by [u→v = x→y], its B by [u→v = x→y] +
/// [v = x], and its passage by [v = x] + 1 − [u = y] − n·[u = x]: the step
/// enters the passage of every spoke (the term 1, the steps so far), and
/// nothing on any spoke changes but as the step fixes. Read on the field
/// before and after the step.
#[test]
fn the_whole_field_changes_together_at_every_step() {
    let (input, a, _) = fixture(400_000, 300, 100);
    let (ids, _, full) = field(&[input, a]);
    let n = full.n() as i64;
    let mut r = RadialField::new(full.n());
    for (q, w) in ids.windows(2).enumerate() {
        let (x, y) = (w[0], w[1]);
        let before: Vec<_> = (0..full.n()).flat_map(|u| (0..full.n()).map(move |v| (u, v))).map(|(u, v)| r.synapse(u, v)).collect();
        r.push(x, y).unwrap();
        if q % 7 != 0 { continue; }
        for s0 in before {
            let (u, v) = (s0.from, s0.to);
            let s1 = r.synapse(u, v);
            let is = |c: bool| c as i64;
            assert_eq!(s1.a - s0.a, is((u, v) == (x, y)), "A, step {q}, spoke {u}→{v}");
            assert_eq!(s1.b - s0.b, is((u, v) == (x, y)) + is(v == x), "B, step {q}, spoke {u}→{v}");
            assert_eq!(s1.passage - s0.passage, is(v == x) + 1 - is(u == y) - n * is(u == x), "Ξ, step {q}, spoke {u}→{v}");
        }
    }
}

/// 3. A continuation registers into the same field as the input: the field
/// read registration by registration over input and continuation is the
/// field of the two read as one text, and reads both back exactly (113th).
#[test]
fn a_continuation_registers_into_the_same_field() {
    for at in [100_000, 300_000, 500_000] {
        let (input, a, b) = fixture(at, 2000, 500);
        for cont in [a, b] {
            let (ids, chars, r) = field(&[input, cont]);
            let back: Vec<char> = (0..r.steps()).map(|p| chars[r.step(p).0]).chain(std::iter::once(chars[r.step(r.steps() - 1).1])).collect();
            assert_eq!(read_back(&back), format!("{input}{cont}"), "the field reads input and continuation back exactly");
            let joined = format!("{input}{cont}");
            let (ids2, _, r2) = field(&[&joined]);
            assert_eq!(ids, ids2);
            for (u, v) in (0..r.n()).flat_map(|u| (0..r.n()).map(move |v| (u, v))) {
                assert_eq!(r.synapse(u, v), r2.synapse(u, v), "one field, at {u}→{v}");
            }
        }
    }
}

/// 4. The two continuations are actual text of equal length: (a) follows the
/// input in the book; (b) is a passage from the book beginning a paragraph,
/// far from the input, never the same text.
#[test]
fn the_continuations_are_actual_text_of_equal_length() {
    let n = novel();
    for at in [100_000, 200_000, 300_000, 400_000, 500_000, 600_000] {
        let (input, a, b) = fixture(at, 4000, 1000);
        assert!(n.contains(&format!("{input}{a}")), "(a) follows the input in the book");
        assert!(n.contains(&format!("\r\n\r\n{b}")), "(b) is the book's text, beginning a paragraph");
        assert_eq!((input.chars().count(), a.chars().count(), b.chars().count()), (4000, 1000, 1000));
        let (i0, b0) = (n.find(input).unwrap(), n.find(b).unwrap());
        assert!(i0.abs_diff(b0) > n.len() / 4, "(b) is far from the input");
        assert_ne!(a, b);
    }
}
