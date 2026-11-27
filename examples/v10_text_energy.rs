//! Energy along a continuation (117th): is it maintained as the subject is
//! continued, lost as it departs, regained as a subsequent subject is pursued?
//!
//! [IMPL] Actual text: *Pride and Prejudice* (`tests/data/`). Measurement
//! only; nothing chooses. At eight declared points of the novel (every
//! ninth of its length, moved to the next paragraph), an input passage of
//! 4000 characters forms the field; two continuations of 1000 characters,
//! both actual text, are registered after it into the same field: (a) the
//! book's own next passage, (b) the passage from the paragraph half the book
//! further on (spliced: real text, Origin to judge it a fair stand-in for
//! drifting off the subject). The field so far is the input and the
//! continuation together, expanding as it expands; its energy
//! (`RadialField::text_energy`) is read through each quarter of each
//! continuation: the change of the net movement over the quarter (the
//! carried +1 and −1 summed). Kernel readings; the quarters and totals are
//! framework summaries, not a rule.
//!
//! Run: `cargo run --release --example v10_text_energy`

use std::collections::HashMap;

use abr_kernel::radial::RadialField;

const BOOK: &str = include_str!("../tests/data/pride_and_prejudice.txt");
const CAPITAL: char = '⇧';

fn project(text: &str) -> Vec<char> {
    let mut out = Vec::new();
    for c in text.chars() { if c.is_uppercase() { out.push(CAPITAL); out.extend(c.to_lowercase()); } else { out.push(c); } }
    out
}

fn words(text: &str) -> Vec<String> {
    let reg = project(text);
    let letter = |i: usize| reg[i].is_alphabetic();
    let apostrophe = |i: usize| (reg[i] == '’' || reg[i] == '\'') && i > 0 && i + 1 < reg.len() && letter(i - 1) && letter(i + 1);
    let mut out = Vec::new();
    let mut i = 0;
    while i < reg.len() {
        if reg[i] == CAPITAL { out.push(CAPITAL.to_string()); i += 1; continue; }
        let start = i;
        if letter(i) { while i < reg.len() && (letter(i) || apostrophe(i)) { i += 1; } }
        else { while i < reg.len() && reg[i] != CAPITAL && !letter(i) { i += 1; } }
        out.push(reg[start..i].iter().collect());
    }
    out
}

fn paragraph(n: &str, at: usize) -> usize {
    let mut at = at;
    while !n.is_char_boundary(at) { at += 1; }
    n[at..].find("\r\n\r\n").map(|e| at + e + 4).unwrap()
}

fn after_chars(s: &str, at: usize, k: usize) -> usize { s[at..].char_indices().nth(k).map(|(i, _)| at + i).unwrap() }

/// The energy's change through each quarter of the continuation, after the
/// input, in the field so far.
fn quarters<T: Clone + Eq + std::hash::Hash>(input: &[T], cont: &[T]) -> [i64; 4] {
    let mut r = RadialField::new(0);
    let mut index: HashMap<T, usize> = HashMap::new();
    let ids: Vec<usize> = input.iter().chain(cont).map(|t| *index.entry(t.clone()).or_insert_with(|| r.declare())).collect();
    for w in ids.windows(2) { r.push(w[0], w[1]).unwrap(); }
    let net = r.text_energy().net;
    let s = input.len() - 2; // the energy as the input ends
    let len = net.len() - 1 - s;
    let at = |q: usize| net[s + q * len / 4];
    [at(1) - at(0), at(2) - at(1), at(3) - at(2), at(4) - at(3)]
}

fn main() {
    let start = BOOK.find("It is a truth universally acknowledged").unwrap();
    let end = BOOK.find("*** END OF THE PROJECT GUTENBERG").unwrap_or(BOOK.len());
    let n = &BOOK[start..end];
    println!("[IMPL] Pride and Prejudice: input 4000 characters, continuations 1000 characters, at 8 declared points");
    println!("The energy's change through each quarter of the continuation (+ maintained, − lost)\n");
    let mut tot = [[0i64; 4]; 4];
    for i in 1..=8 {
        let s = paragraph(n, i * n.len() / 9);
        let e = after_chars(n, s, 4000);
        let a = &n[e..after_chars(n, e, 1000)];
        let far = paragraph(n, (s + n.len() / 2) % (n.len() - 4000));
        let b = &n[far..after_chars(n, far, 1000)];
        let input = &n[s..e];
        let rows = [quarters(&words(input), &words(a)), quarters(&words(input), &words(b)),
                    quarters(&project(input), &project(a)), quarters(&project(input), &project(b))];
        println!("point {i}  words (a) {:>4?}  (b) {:>4?}   characters (a) {:>4?}  (b) {:>4?}", rows[0], rows[1], rows[2], rows[3]);
        for (t, r) in tot.iter_mut().zip(rows) { for q in 0..4 { t[q] += r[q]; } }
    }
    println!("\ntotal    words (a) {:>4?}  (b) {:>4?}   characters (a) {:>4?}  (b) {:>4?}", tot[0], tot[1], tot[2], tot[3]);
}
