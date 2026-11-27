//! The field read along the route the text is taking (Origin, 2026-11-27:
//! *"the field read along the route the text is taking"*). A look, not yet
//! a declaration.
//!
//! [IMPL] Text as the metaphorical basis (108th (4)); the 111th's field. The
//! route's last k steps as a field: R_k = A_n − A_{n−k}, the movement
//! registered along them, formed by the kernel as the change between two
//! registrations (a field of the two, `RelationalField::from_relations`):
//! its B, Ξ and readback are the kernel's. Printed: at the last point, on
//! every spoke leaving the current element (where the text can go), B and
//! Ξ of the route field, for every k. Nothing is weighted by k.
//!
//! Run: `cargo run --release --example v10_text_route`

use abr_kernel::operators::{DeclaredRelations, EdgeProvenance};
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::relational_field::RelationalField;
use abr_kernel::topology::{FeedbackDeclaration, RelationIdentity};

fn main() {
    let text = "he swung the bat. he swung the ball. the cat sat. he swung the ba";
    let cs: Vec<char> = text.chars().collect();
    let mut chars: Vec<char> = Vec::new();
    for &c in &cs { if !chars.contains(&c) { chars.push(c); } }
    let n = chars.len();
    let at = |c: char| chars.iter().position(|&x| x == c).unwrap();
    let edges: Vec<(usize, usize)> = (0..n).flat_map(|s| (0..n).map(move |t| (s, t))).collect();
    let m = edges.len();
    let ids = (0..m).map(|e| RelationIdentity { id: format!("spoke-{e}"), basis: "a spoke of a radial graph (109th)".into() }).collect();
    let fb = (0..m).map(|e| FeedbackDeclaration { edge: e, basis: "the order of reading (Origin, 109th)".into() }).collect();
    let rel = DeclaredRelations::from_edges_with_provenance(n, edges.clone(), vec![EdgeProvenance::Continuation; m]).with_identity(ids).with_feedback(fb);
    let steps: Vec<usize> = cs.windows(2).map(|w| at(w[0]) * n + at(w[1])).collect();
    // The field as it stands at every point.
    let mut stand = vec![0.0; m];
    let a: Vec<Vec<f64>> = steps.iter().map(|&e| { stand[e] += 1.0; stand.clone() }).collect();
    let last = steps.len() - 1;
    let here = edges[steps[last]].1;
    let show = |c: usize| if chars[c] == ' ' { '␣' } else { chars[c] };
    let leaving: Vec<usize> = (0..m).filter(|&e| edges[e].0 == here).collect();
    let route_of = |k: usize| -> String { steps[last + 1 - k..].iter().map(|&e| show(edges[e].1)).collect() };

    println!("[IMPL] {text:?}; the text stands at {:?}.", show(here));
    println!("Ξ(B(R_k)) on each spoke leaving {:?} (where the text can go), for the route's last k steps:\n", show(here));
    print!("{:>3} {:<18}", "k", "route");
    for &e in &leaving { print!("{:>5}", format!("→{}", show(edges[e].1))); }
    println!();
    for k in 1..=last {
        // R_k: the change between the field k steps back and now, formed by
        // the kernel as the step of a field of two registrations.
        let series = EdgeSeries::from_edge_values_synthetic(vec![vec![a[last - k].clone()], vec![a[last].clone()]], vec![vec![vec![0.0; m]]; 2]).unwrap();
        let f = RelationalField::from_relations(&rel, series, None).unwrap();
        let p = &f.persistence[0];
        if !(k <= 20 || k % 10 == 0 || k == last) { continue; }
        print!("{k:>3} {:<18}", format!("…{}", route_of(k.min(16))));
        for &e in &leaving { print!("{:>5}", p.passage[0][e]); }
        println!();
    }
    println!("\n(B(R_k) and the readback are formed too; Ξ is shown.)");
}
