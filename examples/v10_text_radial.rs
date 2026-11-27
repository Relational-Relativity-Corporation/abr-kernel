//! The radial graphs as the field evolves (Origin, 111th), on actual text.
//!
//! [IMPL] Text as the metaphorical basis (108th (4)). Every value printed is
//! the kernel's (`RelationalField::from_relations`); this program only
//! selects which entries to print. The trial carrier is the step itself.
//!
//! At the last point of the text: the radial graph of the current element,
//! every spoke leaving it (where the text can go), with what the operators
//! read on each: A (the movement it has carried), B (it and what leaves
//! where it leads), Ξ (what follows it less what precedes it).
//!
//! Run: `cargo run --example v10_text_radial`

use abr_kernel::ground_variables::{ObservedRange, RelationCarrier};
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
    let mut stand = vec![0.0; m];
    let a: Vec<Vec<Vec<f64>>> = steps.iter().map(|&e| { stand[e] += 1.0; vec![stand.clone()] }).collect();
    let da = vec![vec![vec![0.0; m]]; steps.len()];
    let series = EdgeSeries::from_edge_values_synthetic(a, da.clone()).unwrap();
    let step_of = |e: usize| (0..m).map(|f| if f == e { 1.0 } else { 0.0 }).collect::<Vec<f64>>();
    let carrier = RelationCarrier { name: "TRIAL: the step".into(), unit: "step".into(),
        a: steps.iter().map(|&e| step_of(e)).collect(), da: da.iter().map(|t| t[0].clone()).collect(), range: ObservedRange::synthetic() };
    let f = RelationalField::from_relations(&rel, series, Some(&carrier)).unwrap();
    let show = |c: usize| if chars[c] == ' ' { '␣' } else { chars[c] };
    let last = steps.len() - 1;
    let here = edges[steps[last]].1;
    println!("[IMPL] {text:?}: {n} elements, {m} spokes, {} points of evolution", steps.len());
    println!("At the last point the text stands at {:?}. Its radial graph, every spoke leaving it:\n", show(here));
    println!("  {:<6} {:>4} {:>4} {:>4}", "spoke", "A", "B", "Ξ");
    for e in (0..m).filter(|&e| edges[e].0 == here) {
        let s = &f.spatial[last];
        println!("  {}→{:<4} {:>4} {:>4} {:>4}", show(edges[e].0), show(edges[e].1), s.a[0][e], s.b[0][e], s.passage[0][e]);
    }
    println!("\n  readback at {:?}: {} out so far, {} in", show(here), f.spatial[last].leaving[0][here], f.spatial[last].entering[0][here]);
}
