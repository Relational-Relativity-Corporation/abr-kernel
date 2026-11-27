//! Does the field's evolution sustain the subject? (the 116th amended).
//!
//! [IMPL] Actual text: *Pride and Prejudice* (`tests/data/`). Measurement
//! only; nothing chooses. At eight declared points of the novel (every
//! ninth of its length, moved to the next paragraph), an input passage of
//! 4000 characters forms the field; two continuations of 1000 characters,
//! both actual text, are registered after it into the same field: (a) the
//! book's own next passage, (b) the passage from the paragraph half the book
//! further on (spliced: real text, Origin to judge it a fair stand-in for
//! drifting off the subject). Over the successive registrations of each
//! continuation, the readings of the synapse that fires as the whole field
//! stands (`RadialField::synapse`: A, B, Ξ, the readback's halves), at the
//! character scale (113th) and the word scale (114th). Framework summaries
//! (shares, means) of kernel readings; not a rule.
//!
//! Run: `cargo run --release --example v10_subject`

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

/// Readings over a continuation's successive registrations.
#[derive(Default)]
struct Tally { steps: usize, new_element: usize, by_input: usize, so_far: usize, a: i64, b: i64, passage: i64 }

/// Register the input, then the continuation, into one field over elements
/// declared as observed; read the synapse that fires at each registration
/// of the continuation, as the whole field stands.
fn read<T: Clone + Eq + std::hash::Hash>(input: &[T], cont: &[T]) -> Tally {
    let mut r = RadialField::new(0);
    let mut index: HashMap<T, usize> = HashMap::new();
    let mut id = |t: &T, r: &mut RadialField| *index.entry(t.clone()).or_insert_with(|| r.declare());
    let ids: Vec<usize> = input.iter().map(|t| id(t, &mut r)).collect();
    for w in ids.windows(2) { r.push(w[0], w[1]).unwrap(); }
    let after_input = r.clone();
    let mut t = Tally::default();
    let mut last = *ids.last().unwrap();
    for c in cont {
        let before = r.n();
        let y = id(c, &mut r);
        let s = r.synapse(last, y);
        t.steps += 1;
        t.new_element += (r.n() > before) as usize;
        t.by_input += (last < after_input.n() && y < after_input.n() && after_input.synapse(last, y).a > 0) as usize;
        t.so_far += (s.a > 0) as usize;
        t.a += s.a; t.b += s.b; t.passage += s.passage;
        r.push(last, y).unwrap();
        last = y;
    }
    t
}

fn line(name: &str, t: &Tally) {
    let pct = |x: usize| 100.0 * x as f64 / t.steps as f64;
    let mean = |x: i64| x as f64 / t.steps as f64;
    println!("  {name:<28} {:>7.1}% {:>9.1}% {:>9.1}% {:>9.2} {:>9.1} {:>11.1}",
        pct(t.new_element), pct(t.by_input), pct(t.so_far), mean(t.a), mean(t.b), mean(t.passage));
}

fn main() {
    let start = BOOK.find("It is a truth universally acknowledged").unwrap();
    let end = BOOK.find("*** END OF THE PROJECT GUTENBERG").unwrap_or(BOOK.len());
    let n = &BOOK[start..end];
    let (mut ca, mut cb, mut wa, mut wb) = (Tally::default(), Tally::default(), Tally::default(), Tally::default());
    let add = |x: &mut Tally, y: Tally| { x.steps += y.steps; x.new_element += y.new_element; x.by_input += y.by_input; x.so_far += y.so_far; x.a += y.a; x.b += y.b; x.passage += y.passage; };
    println!("[IMPL] Pride and Prejudice: input 4000 characters, continuations 1000 characters, at 8 declared points\n");
    println!("  {:<28} {:>8} {:>10} {:>10} {:>9} {:>9} {:>11}", "", "new elem", "by input", "so far", "mean A", "mean B", "mean Ξ");
    for i in 1..=8 {
        let s = paragraph(n, i * n.len() / 9);
        let e = after_chars(n, s, 4000);
        let a = &n[e..after_chars(n, e, 1000)];
        let far = paragraph(n, (s + n.len() / 2) % (n.len() - 4000));
        let b = &n[far..after_chars(n, far, 1000)];
        let input = &n[s..e];
        add(&mut ca, read(&project(input), &project(a)));
        add(&mut cb, read(&project(input), &project(b)));
        // Word scale: the continuation's words follow the input's; the first
        // continuation word joins where the input's last word ends.
        let (xa, xb) = (read(&words(input), &words(a)), read(&words(input), &words(b)));
        println!("  point {i}, word scale: new elements (a) {:>2} of {:>3}, (b) {:>2} of {:>3}; by input (a) {:>3}, (b) {:>3}",
            xa.new_element, xa.steps, xb.new_element, xb.steps, xa.by_input, xb.by_input);
        add(&mut wa, xa);
        add(&mut wb, xb);
    }
    println!();
    println!("Character scale (the synapse that fires: one character to the next)");
    line("(a) the book's next passage", &ca);
    line("(b) a distant passage", &cb);
    println!("Word scale (the synapse that fires: one word or separator to the next)");
    line("(a) the book's next passage", &wa);
    line("(b) a distant passage", &wb);
    println!("\nnew elem: the element entered had never been observed; by input: the synapse was stimulated by the input;");
    println!("so far: stimulated by the input or the continuation before it; means: the synapse's readings as the field stands.");
}
