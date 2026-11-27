//! What happens if we consistently choose the highest-scoring letter?
//! (Origin, 2026-11-27.)
//!
//! [IMPL] Text as the metaphorical basis (108th (4)); the 111th's field. At
//! each point, the spokes leaving the element the text stands at are read by
//! the kernel (`RelationalField::from_relations`): A, B or Ξ. The spoke with
//! the highest value is chosen (a selection in the framework, not an
//! operator; ties go to the first in the declared order, and are counted),
//! registered into the same field (85th), and the field formed again. Every
//! value chosen from is the kernel's.
//!
//! Run: `cargo run --release --example v10_text_highest`

use abr_kernel::ground_variables::{ObservedRange, RelationCarrier};
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance};
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::relational_field::RelationalField;
use abr_kernel::topology::{FeedbackDeclaration, RelationIdentity};

#[derive(Clone, Copy)]
enum By { A, B, Xi }

fn main() {
    let input = "he swung the bat. he swung the ball. the cat sat. he swung the ba";
    let cs: Vec<char> = input.chars().collect();
    let mut chars: Vec<char> = Vec::new();
    for &c in &cs { if !chars.contains(&c) { chars.push(c); } }
    let n = chars.len();
    let at = |c: char| chars.iter().position(|&x| x == c).unwrap();
    let edges: Vec<(usize, usize)> = (0..n).flat_map(|s| (0..n).map(move |t| (s, t))).collect();
    let m = edges.len();
    let ids: Vec<RelationIdentity> = (0..m).map(|e| RelationIdentity { id: format!("spoke-{e}"), basis: "a spoke of a radial graph (109th)".into() }).collect();
    let fb: Vec<FeedbackDeclaration> = (0..m).map(|e| FeedbackDeclaration { edge: e, basis: "the order of reading (Origin, 109th)".into() }).collect();
    let rel = DeclaredRelations::from_edges_with_provenance(n, edges.clone(), vec![EdgeProvenance::Continuation; m]).with_identity(ids).with_feedback(fb);
    let input_steps: Vec<usize> = cs.windows(2).map(|w| at(w[0]) * n + at(w[1])).collect();
    let show = |c: usize| chars[c];

    let field_of = |steps: &[usize]| -> RelationalField {
        let mut stand = vec![0.0; m];
        let a: Vec<Vec<Vec<f64>>> = steps.iter().map(|&e| { stand[e] += 1.0; vec![stand.clone()] }).collect();
        let da = vec![vec![vec![0.0; m]]; steps.len()];
        let series = EdgeSeries::from_edge_values_synthetic(a, da.clone()).unwrap();
        let step_of = |e: usize| (0..m).map(|f| if f == e { 1.0 } else { 0.0 }).collect::<Vec<f64>>();
        let carrier = RelationCarrier { name: "TRIAL: the step".into(), unit: "step".into(),
            a: steps.iter().map(|&e| step_of(e)).collect(), da: da.iter().map(|t| t[0].clone()).collect(), range: ObservedRange::synthetic() };
        RelationalField::from_relations(&rel, series, Some(&carrier)).unwrap()
    };

    println!("[IMPL] Input: {input:?}");
    println!("Each line: the input continued 30 letters, always choosing the spoke with the highest value.\n");
    for (by, name) in [(By::A, "A (the movement each spoke has carried)"), (By::B, "B (the spoke and what leaves where it leads)"), (By::Xi, "Ξ (what follows less what precedes)")] {
        let mut steps = input_steps.clone();
        let mut out = String::new();
        let mut ties = 0;
        for _ in 0..30 {
            let f = field_of(&steps);
            let last = steps.len() - 1;
            let here = edges[*steps.last().unwrap()].1;
            let s = &f.spatial[last];
            let value = |e: usize| match by { By::A => s.a[0][e], By::B => s.b[0][e], By::Xi => s.passage[0][e] };
            let leaving: Vec<usize> = (0..m).filter(|&e| edges[e].0 == here).collect();
            let best = leaving.iter().map(|&e| value(e)).fold(f64::NEG_INFINITY, f64::max);
            let top: Vec<usize> = leaving.iter().copied().filter(|&e| value(e) == best).collect();
            if top.len() > 1 { ties += 1; }
            let chosen = top[0];
            out.push(show(edges[chosen].1));
            steps.push(chosen);
        }
        println!("highest {name}:\n  …he swung the ba|{out}|   ({ties} of 30 choices were ties)\n");
    }
}
