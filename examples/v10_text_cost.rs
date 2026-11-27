//! Where the compute goes in forming text's field (Origin, 2026-11-27:
//! *"I don't think it should take as much as we're doing"*). A measurement.
//!
//! Run: `cargo run --release --example v10_text_cost`

use std::time::Instant;

use abr_kernel::ground_variables::ground_reading_of_relations;
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance};
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::radial::RadialField;
use abr_kernel::relational_field::RelationalField;
use abr_kernel::topology::{FeedbackDeclaration, RelationIdentity};

fn main() {
    for text in ["It was the best of times, it was the worst of times",
                 "It was the best of times, it was the worst of times, it was the age of wisdom, it was the age of foolishness",
                 "It was the best of times, it was the worst of times, it was the age of wisdom, it was the age of foolishness, it was the epoch of belief, it was the epoch of incredulity, it was the season of Light, it was the season of Darkness, it was the spring of hope, it was the winter of despair, we had everything before us, we had nothing before us, we were all going direct to Heaven, we were all going direct the other way"] {
        let cs: Vec<char> = text.chars().collect();
        let mut chars: Vec<char> = Vec::new();
        for &c in &cs { if !chars.contains(&c) { chars.push(c); } }
        let n = chars.len();
        let at = |c: char| chars.iter().position(|&x| x == c).unwrap();
        let edges: Vec<(usize, usize)> = (0..n).flat_map(|s| (0..n).map(move |t| (s, t))).collect();
        let m = edges.len();
        let ids = (0..m).map(|e| RelationIdentity { id: format!("spoke-{e}"), basis: "a spoke".into() }).collect();
        let fb = (0..m).map(|e| FeedbackDeclaration { edge: e, basis: "the order of reading".into() }).collect();
        let rel = DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m]).with_identity(ids).with_feedback(fb);
        let steps: Vec<usize> = cs.windows(2).map(|w| at(w[0]) * n + at(w[1])).collect();
        let mut stand = vec![0.0; m];
        let a: Vec<Vec<Vec<f64>>> = steps.iter().map(|&e| { stand[e] += 1.0; vec![stand.clone()] }).collect();
        let t0 = Instant::now();
        let series = EdgeSeries::from_edge_values_synthetic(a, vec![vec![vec![0.0; m]]; steps.len()]).unwrap();
        let t_series = t0.elapsed();
        let t1 = Instant::now();
        let ground = ground_reading_of_relations(&series, &rel, None);
        let t_ground = t1.elapsed();
        let t2 = Instant::now();
        let _f = RelationalField::from_relations(&rel, series.clone(), None).unwrap();
        let t_field = t2.elapsed();
        let _ = ground;
        // The radial field: the same readings, step by step.
        let t3 = Instant::now();
        let mut r = RadialField::new(n);
        let mut probe = 0i64;
        for &e in &steps {
            r.push(e / n, e % n).unwrap();
            let here = e % n;
            for y in 0..n { probe += r.b(here, y) + r.xi(here, y); } // B and Ξ on every spoke leaving the current element
            probe += r.indications(r.steps()).len() as i64;
        }
        let t_radial = t3.elapsed();
        println!("{} characters: {n} elements, {m} spokes, {} registrations, {} values held",
            cs.len(), steps.len(), m * steps.len());
        println!("  the series:                 {:>8.3} s", t_series.as_secs_f64());
        println!("  the ground (E, F, C) alone: {:>8.3} s", t_ground.as_secs_f64());
        println!("  the whole field:            {:>8.3} s  (forms A, B, Ξ, readback ≈ {:.3} s)", t_field.as_secs_f64(), (t_field.as_secs_f64() - t_ground.as_secs_f64()).max(0.0));
        println!("  the radial field, reading A, B, Ξ and the indications at every point: {:>8.6} s  [{probe}]\n", t_radial.as_secs_f64());
    }
}
