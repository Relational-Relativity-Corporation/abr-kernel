//! The operators on text's radial graphs, step by step (109th, 110th; the
//! trial registration: one registration per step, +1 on the spoke taken).
//!
//! [IMPL] SYNTHETIC: text as the metaphorical basis (108th (4)). The trial
//! carrier is the step itself (the Generator's, for Origin). Every value
//! printed is the kernel's: `RelationalField::from_relations` and
//! `coherence::radial_graphs`; only the selection of nonzero entries to
//! print is done here.
//!
//! Run: `cargo run --example v10_text_field`

use abr_kernel::coherence::radial_graphs;
use abr_kernel::ground_variables::{ObservedRange, RelationCarrier};
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance};
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::relational_field::RelationalField;
use abr_kernel::topology::{FeedbackDeclaration, RelationIdentity};

fn main() {
    let text = " a car cras";
    let cs: Vec<char> = text.chars().collect();
    let mut chars: Vec<char> = Vec::new();
    for &c in &cs { if !chars.contains(&c) { chars.push(c); } }
    let n = chars.len();
    let at = |c: char| chars.iter().position(|&x| x == c).unwrap();
    let edges: Vec<(usize, usize)> = (0..n).flat_map(|s| (0..n).map(move |t| (s, t))).collect();
    let m = edges.len();
    let ids = (0..m).map(|e| RelationIdentity { id: format!("SYNTHETIC-spoke-{e}"), basis: "SYNTHETIC: a spoke (109th)".into() }).collect();
    let fb = (0..m).map(|e| FeedbackDeclaration { edge: e, basis: "the order of reading (Origin, 109th)".into() }).collect();
    let rel = DeclaredRelations::from_edges_with_provenance(n, edges.clone(), vec![EdgeProvenance::Continuation; m]).with_identity(ids).with_feedback(fb);
    let steps: Vec<usize> = cs.windows(2).map(|w| at(w[0]) * n + at(w[1])).collect();
    let a: Vec<Vec<Vec<f64>>> = steps.iter().map(|&e| vec![(0..m).map(|f| if f == e { 1.0 } else { 0.0 }).collect()]).collect();
    let da = vec![vec![vec![0.0; m]]; steps.len()];
    let series = EdgeSeries::from_edge_values_synthetic(a.clone(), da.clone()).unwrap();
    let carrier = RelationCarrier { name: "TRIAL: the step".into(), unit: "step".into(),
        a: a.iter().map(|t| t[0].clone()).collect(), da: da.iter().map(|t| t[0].clone()).collect(), range: ObservedRange::synthetic() };
    let f = RelationalField::from_relations(&rel, series, Some(&carrier)).unwrap();
    let show = |c: usize| if chars[c] == ' ' { '␣' } else { chars[c] };
    let sp = |e: usize| format!("{}→{}", show(edges[e].0), show(edges[e].1));
    let nonzero = |v: &[f64]| v.iter().enumerate().filter(|x| *x.1 != 0.0).map(|(e, x)| format!("{}{}", if *x > 0.0 { "+" } else { "−" }, sp(e))).collect::<Vec<_>>().join(" ");

    println!("[IMPL] SYNTHETIC, the trial registration on {text:?}: {n} elements, {m} spokes, {} steps\n", steps.len());
    for (s, &e) in steps.iter().enumerate() {
        println!("step {s}: A = {}   out of {:?} into {:?}", sp(e), show(edges[e].0), show(edges[e].1));
        println!("         B: {}", nonzero(&f.spatial[s].b[0]));
        println!("         Ξ: {}", nonzero(&f.spatial[s].passage[0]));
        if s + 1 < steps.len() { println!("         A_p to the next step: {}", nonzero(&f.persistence[s].a[0])); }
    }
    // W5: the radial graphs accumulated over every step; each spoke's
    // accumulated energy is the net movement across it since the record began.
    let graphs = radial_graphs(&f, &rel, steps.len() - 1).unwrap();
    println!("\nW5, accumulated over all {} steps: the net movement across each spoke (nonzero only)", steps.len());
    for g in &graphs {
        let moved: Vec<String> = g.spokes.iter().filter_map(|s| s.energy.as_ref().ok().filter(|x| x.sum != 0.0).map(|x| format!("{} {}", sp(s.edge), x.sum))).collect();
        if !moved.is_empty() { println!("  at {:?}: {}", show(g.locus), moved.join(", ")); }
    }
}
