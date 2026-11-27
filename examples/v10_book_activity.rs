//! Coherence through a whole book (116th, under trial; amended: synaptic
//! activity, the relations fired behind each point compared): does the activity
//! at a point narrow the next character more than the 108th's comparison?
//!
//! [IMPL] Actual text: *Pride and Prejudice* (`tests/data/`), the novel from
//! its first sentence to its last (the Gutenberg header and footer left out
//! of this reading only), at the character scale with the capital mark.
//! Every value is the kernel's (`RadialField`). At every 1000th point (a
//! fixed sample, nothing chosen): the furthest-alongside set (the
//! continuations of the degrees whose arrivals run alongside furthest, as in
//! `v10_book_scales`), and the coherent set at several declared reaches (the
//! continuations of the coherent degrees). Framework counts, not a rule;
//! nothing here chooses.
//!
//! Run: `cargo run --release --example v10_book_activity`

use std::collections::{BTreeSet, HashMap};

use abr_kernel::radial::RadialField;

const BOOK: &str = include_str!("../tests/data/pride_and_prejudice.txt");
const CAPITAL: char = '⇧';

fn project(text: &str) -> Vec<char> {
    let mut out = Vec::new();
    for c in text.chars() { if c.is_uppercase() { out.push(CAPITAL); out.extend(c.to_lowercase()); } else { out.push(c); } }
    out
}

struct Tally { name: String, points: usize, holds: usize, sizes: usize }

fn main() {
    let start = BOOK.find("It is a truth universally acknowledged").unwrap();
    let end = BOOK.find("*** END OF THE PROJECT GUTENBERG").unwrap_or(BOOK.len());
    let reg = project(&BOOK[start..end]);
    let mut index: HashMap<char, usize> = HashMap::new();
    let mut r = RadialField::new(0);
    let ids: Vec<usize> = reg.iter().map(|c| *index.entry(*c).or_insert_with(|| r.declare())).collect();
    for w in ids.windows(2) { r.push(w[0], w[1]).unwrap(); }
    let n = r.n();
    println!("[IMPL] Pride and Prejudice, the novel: {} character steps over {} elements\n", r.steps(), n);
    const REACHES: [usize; 5] = [4, 8, 16, 32, 64];
    let mut along = Tally { name: "furthest alongside (108th)".into(), points: 0, holds: 0, sizes: 0 };
    let mut coh: Vec<Tally> = REACHES.iter().map(|k| Tally { name: format!("coherent, reach {k}"), points: 0, holds: 0, sizes: 0 }).collect();
    let mut agree_past = 0usize;
    let mut coherent_degrees = 0usize;
    for p in (1000..r.steps()).step_by(1000) {
        let took = r.step(p).1;
        let d = r.indications(p);
        if d.is_empty() { continue; }
        let best = d.iter().map(|x| x.alongside.1 - x.alongside.0).max().unwrap();
        let set: BTreeSet<usize> = d.iter().filter(|x| x.alongside.1 - x.alongside.0 == best).map(|x| x.leaving % n).collect();
        along.points += 1; along.holds += set.contains(&took) as usize; along.sizes += set.len();
        for (t, &k) in coh.iter_mut().zip(&REACHES) {
            let a = r.activity(p, k).unwrap();
            let set: BTreeSet<usize> = a.coherent.iter().map(|&i| a.agreements[i].leaving % n).collect();
            t.points += 1; t.holds += set.contains(&took) as usize; t.sizes += set.len();
            if k == 16 {
                for &i in &a.coherent {
                    let g = &a.agreements[i];
                    coherent_degrees += 1;
                    if let Some(&first) = g.differs.first() { if g.agrees.iter().any(|&q| q > first) { agree_past += 1; } }
                }
            }
        }
    }
    println!("{:<30} {:>8} {:>22} {:>26}", "set", "points", "next char in the set", "distinct chars in the set");
    for t in std::iter::once(&along).chain(coh.iter()) {
        println!("{:<30} {:>8} {:>21.1}% {:>26.2}", t.name, t.points, 100.0 * t.holds as f64 / t.points as f64, t.sizes as f64 / t.points as f64);
    }
    println!("\nAt reach 16: {coherent_degrees} coherent degrees, of which {agree_past} agree past where their arrival parts ({:.1}%).",
        100.0 * agree_past as f64 / coherent_degrees.max(1) as f64);
}
