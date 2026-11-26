//! The degrees of freedom across the whole input (Origin, 2026-11-26, 108th).
//!
//! [IMPL] SYNTHETIC: text as the metaphorical basis for calculation (108th
//! (4)), a domain example. It establishes what the readings are on this
//! text, nothing empirical.
//!
//! At every point p of the input, the kernel's `Order::degrees` is read on
//! the order up to p: every earlier point where the order stood at the locus
//! of p, each a distinct degree. The relation the text registered next,
//! `steps[p]`, is the degree the text took. Marking which degrees offered it
//! is a framework reading (a comparison of identities, 104th (2)); nothing
//! here is computed outside the kernel except that comparison and the
//! lengths of the spans the kernel reports.
//!
//! Read with freedom and connection only: energy is a named gap on every
//! degree until text's carrier is declared (108th (2)).
//!
//! Run: `cargo run --example v10_degrees_across`

use abr_kernel::operators::{DeclaredRelations, EdgeProvenance};
use abr_kernel::order::Order;
use abr_kernel::topology::RelationIdentity;

struct Text { chars: Vec<char>, pairs: Vec<(char, char)>, rel: DeclaredRelations, steps: Vec<usize> }

/// Each distinct character a locus, each distinct ordered pair a relation,
/// each step of the text one registered relation.
fn declare(text: &str) -> Text {
    let cs: Vec<char> = text.chars().collect();
    let mut chars: Vec<char> = Vec::new();
    for &c in &cs { if !chars.contains(&c) { chars.push(c); } }
    let at = |c: char| chars.iter().position(|&x| x == c).unwrap();
    let mut pairs: Vec<(char, char)> = Vec::new();
    let mut steps = Vec::new();
    for w in cs.windows(2) {
        let p = (w[0], w[1]);
        if !pairs.contains(&p) { pairs.push(p); }
        steps.push(pairs.iter().position(|&x| x == p).unwrap());
    }
    let edges: Vec<(usize, usize)> = pairs.iter().map(|&(x, y)| (at(x), at(y))).collect();
    let m = edges.len();
    // Identity and basis for every relation (RPPI-V10 G-1), declared
    // synthetically, as in the tests. T8 is not applied by the order (102nd).
    let ids = (0..m).map(|e| RelationIdentity { id: format!("SYNTHETIC-text-e{e}"), basis: "SYNTHETIC: the ordered pair of characters as read".into() }).collect();
    let rel = DeclaredRelations::from_edges_with_provenance(chars.len(), edges, vec![EdgeProvenance::Continuation; m]).with_identity(ids);
    Text { chars, pairs, rel, steps }
}

fn main() {
    let text = "he swung the bat. he swung the ball. the cat sat. he swung the ba";
    let t = declare(text);
    let show = |c: char| if c == ' ' { '␣' } else { c };
    let pair = |e: usize| format!("{}→{}", show(t.pairs[e].0), show(t.pairs[e].1));
    println!("[IMPL] SYNTHETIC: text as the metaphorical basis (108th). Energy: a named gap on every degree.\n");
    println!("{:>3}  {:<14} {:<6} {}", "p", "arriving", "took", "degrees: point leaving (alongside), * = offered what the text took");
    let (mut first, mut offered, mut beyond, mut longest_took, mut longest_other, mut tied) = (0, 0, 0, 0, 0, 0);
    for p in 1..t.steps.len() {
        let order = Order::new(&t.rel, t.steps[..p].to_vec()).unwrap();
        let d = order.degrees();
        let took = t.steps[p];
        let arriving: String = t.steps[p.saturating_sub(10)..p].iter().map(|&e| show(t.pairs[e].1)).collect();
        let cells: Vec<String> = d.iter().map(|x| format!("{}{} {} ({})", if x.leaving == took { "*" } else { "" },
            x.point, pair(x.leaving), x.alongside.1 - x.alongside.0)).collect();
        println!("{p:>3}  {arriving:<14} {:<6} {}", pair(took), if cells.is_empty() { "none: the locus is met for the first time".into() } else { cells.join(", ") });
        if d.is_empty() { first += 1; continue; }
        if !d.iter().any(|x| x.leaving == took) { beyond += 1; continue; }
        offered += 1;
        // Framework reading: did the degree whose arrival runs alongside
        // furthest offer what the text took?
        let len = |x: &abr_kernel::order::Degree| x.alongside.1 - x.alongside.0;
        let best = d.iter().map(len).max().unwrap();
        let at_best: Vec<_> = d.iter().filter(|x| len(x) == best).collect();
        if at_best.iter().all(|x| x.leaving == took) { longest_took += 1; }
        else if at_best.iter().all(|x| x.leaving != took) { longest_other += 1; }
        else { tied += 1; }
    }
    let _ = &t.chars;
    println!("\nFramework readings (counts of points; not a rule, not a model):");
    println!("  locus met for the first time, no degree:          {first}");
    println!("  the text took a relation no degree offered:       {beyond}");
    println!("  the text took a relation some degree offered:     {offered}");
    println!("    of these, the furthest-alongside degrees all offered it:   {longest_took}");
    println!("    of these, the furthest-alongside degrees offered another:  {longest_other}");
    println!("    of these, the furthest-alongside degrees offered both:     {tied}");
}
