//! The two scales through a whole book (Origin, 114th): does the word scale
//! narrow the next element more sharply than the character scale?
//!
//! [IMPL] Actual text: *Pride and Prejudice* (`tests/data/`), the novel from
//! its first sentence to its last (the Gutenberg header and footer left
//! out of this reading only). Every value is the kernel's (`RadialField`):
//! at every 100th point (a fixed sample, nothing chosen), the indications;
//! the furthest-alongside set is the continuations offered by the earlier
//! points whose arrivals run alongside furthest. Framework counts, not a
//! rule.
//!
//! Run: `cargo run --release --example v10_book_scales`

use std::collections::{BTreeSet, HashMap};

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

/// A radial field over any sequence of elements, declared as observed.
fn field<T: Clone + Eq + std::hash::Hash>(seq: &[T]) -> (RadialField, Vec<T>) {
    let mut r = RadialField::new(0);
    let mut elems: Vec<T> = Vec::new();
    let mut index: HashMap<T, usize> = HashMap::new();
    let ids: Vec<usize> = seq.iter().map(|t| *index.entry(t.clone()).or_insert_with(|| { elems.push(t.clone()); r.declare() })).collect();
    for w in ids.windows(2) { r.push(w[0], w[1]).unwrap(); }
    (r, elems)
}

struct Tally { points: usize, indicated: usize, in_furthest: usize, furthest_sizes: usize, alongside: usize }

fn read<T: Clone + Eq + std::hash::Hash>(r: &RadialField, only: &dyn Fn(usize) -> bool) -> Tally {
    let n = r.n();
    let mut t = Tally { points: 0, indicated: 0, in_furthest: 0, furthest_sizes: 0, alongside: 0 };
    for p in (100..r.steps()).step_by(100) {
        let here = r.step(p - 1).1;
        if !only(here) { continue; }
        let took = r.step(p).1;
        let d = r.indications(p);
        t.points += 1;
        if d.iter().any(|x| x.leaving % n == took) { t.indicated += 1; }
        let best = d.iter().map(|x| x.alongside.1 - x.alongside.0).max().unwrap_or(0);
        let set: BTreeSet<usize> = d.iter().filter(|x| x.alongside.1 - x.alongside.0 == best).map(|x| x.leaving % n).collect();
        if set.contains(&took) { t.in_furthest += 1; }
        t.furthest_sizes += set.len();
        t.alongside += best;
    }
    let _ = std::marker::PhantomData::<T>;
    t
}

fn report(name: &str, t: &Tally) {
    let pct = |a: usize| 100.0 * a as f64 / t.points as f64;
    println!("{name}: {} sampled points", t.points);
    println!("  next element indicated at all:                         {:>6.1}%", pct(t.indicated));
    println!("  next element among the furthest-alongside continuations: {:>4.1}%", pct(t.in_furthest));
    println!("  distinct continuations in that set, on average:        {:>6.2}", t.furthest_sizes as f64 / t.points as f64);
    println!("  how far the furthest arrival runs alongside, on average: {:>4.1} steps\n", t.alongside as f64 / t.points as f64);
}

fn main() {
    let start = BOOK.find("It is a truth universally acknowledged").unwrap();
    let end = BOOK.find("*** END OF THE PROJECT GUTENBERG").unwrap_or(BOOK.len());
    let novel = &BOOK[start..end];
    let (rc, chars) = field(&project(novel));
    let (rw, elems) = field(&words(novel));
    println!("[IMPL] Pride and Prejudice, the novel: {} character steps over {} elements; {} word-scale steps over {} elements\n",
        rc.steps(), chars.len(), rw.steps(), elems.len());
    // Character scale, at every sampled point; and only where a word begins
    // (the current element a space), for comparison with the word scale.
    let space = chars.iter().position(|&c| c == ' ').unwrap();
    report("Character scale, every sampled point (next: a character)", &read::<char>(&rc, &|_| true));
    report("Character scale, where the text stands at a space (next: a word's first letter)", &read::<char>(&rc, &|h| h == space));
    // Word scale, where the text stands at a separator (next: a word).
    let separator: Vec<bool> = elems.iter().map(|e| !e.chars().next().unwrap().is_alphabetic() && e != "⇧").collect();
    report("Word scale, where the text stands at a separator (next: a whole word)", &read::<String>(&rw, &|h| separator[h]));
}
