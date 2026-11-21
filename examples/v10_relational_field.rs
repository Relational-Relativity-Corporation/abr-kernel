//! Kernel V10 end-to-end template — run with `cargo run --example v10_relational_field`.
//!
//! A starting point for redoing a repository in Kernel V10. It uses only the
//! public V10 API (no `v9` feature): declare the relations, declare a sequence
//! of at least two consecutive registrations with δ, declare the energy
//! carrier, form the relational field, and read energy, freedom and connection
//! with their declaration findings.
//!
//! To adapt it: replace the relations, registrations, δ and carrier with the
//! domain's own declarations through M (R9a), each traceable to an observable.
//! Keep the V9 result of the repository being redone beside the V10 result as
//! a declared comparison (docs/kernel/v10_migration.md); never overwrite it.
//!
//! [IMPL] SYNTHETIC FIXTURE: the values below are chosen to exercise the forms
//! (one component at one locus is pushed through zero, so some minors
//! reorganize). They establish no empirical validity.

use abr_kernel::ground_variables::*;
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::relational_field::RelationalField;

fn reg(c0: Vec<f64>, c1: Vec<f64>, d: f64) -> Registration {
    let n = c0.len();
    Registration::new(NodeField::new(vec![c0, c1]), vec![vec![d; n]; 2])
}

fn main() {
    // 0 → 1 → 2 → 3, plus 1 → 3: interior loci 1 and 2.
    let rel = DeclaredRelations::from_edges_with_provenance(
        4, vec![(0, 1), (1, 2), (2, 3), (1, 3)], vec![EdgeProvenance::Continuation; 4]);

    // Five registrations. Component 1 at locus 2 moves steadily downward,
    // so some minors cross zero partway through (a reorganization).
    let seq: Vec<Registration> = (0..5).map(|t| {
        let t = t as f64;
        reg(vec![4.0, 2.0 + 0.1 * t, 1.0, 0.0], vec![0.0, 1.0, 3.0 - 1.5 * t, 0.5], 0.05)
    }).collect();
    let carriers: Vec<EnergyCarrier> = (0..5).map(|t| EnergyCarrier {
        name: "synthetic carrier".into(), unit: "J".into(),
        values: vec![10.0, 7.0 + 0.2 * t as f64, 5.0, 1.0], delta: vec![0.1; 4],
    }).collect();

    let f = RelationalField::new(&rel, seq.clone(), Some(&carriers)).expect("field");
    println!("registrations: {}, steps: {}", f.spatial.len(), f.n_steps());
    for (t, s) in f.spatial.iter().enumerate() {
        println!("  t={t}  A[0]={:?}  Ξ[0]={:?}", s.a[0], s.circulation[0]);
    }

    let h = minor_histories(&seq, &rel).unwrap();
    let count = |p: fn(&History) -> bool| h.minors.iter().filter(|m| p(&m.4)).count();
    println!("minors: {} total | held unresolved {} | held resolved {} | reorganizing {} (crossed {})",
        h.minors.len(),
        count(|x| *x == History::HeldUnresolved),
        count(|x| matches!(x, History::HeldResolved(_))),
        count(|x| matches!(x, History::Reorganizing { .. })),
        count(|x| matches!(x, History::Reorganizing { crossed: true })));
    println!("κ_δ per registration: {:?}", h.kappa);

    let g = &f.ground;
    let fr = g.freedom.as_ref().unwrap();
    println!("freedom: F_δ = {:.3} ({} of {} pairs)", fr.f_delta, fr.free_pairs.len(), fr.evaluated_pairs);
    let c = g.connection.as_ref().unwrap();
    println!("connection: identity {:?} | proportional {:?} | reorganizing pairs {:?}",
        c.identity, c.proportional, c.reorganizing_pairs);
    match &c.sustained {
        Ok(s) => println!("  sustained: held {:?}, reorganizing {:?}, share {:.2}", s.held_edges, s.reorganizing_edges, s.share_held),
        Err(e) => println!("  sustained: {:?}", e),
    }
    let en = g.energy.as_ref().unwrap();
    println!("energy: carrier '{}' [{}], floor resolved share {:.2}", en.carrier, en.unit, en.share_resolved);
    println!("findings: {:?}", g.findings());

    // Insufficient information and Primary Region.
    println!("one registration: {:?}", RelationalField::new(&rel, seq[..1].to_vec(), None).err());
    let primary = DeclaredRelations::from_edges_with_provenance(
        4, vec![(0, 1), (2, 3)], vec![EdgeProvenance::Continuation; 2]);
    let pf = RelationalField::new(&primary, seq[..3].to_vec(), None).unwrap();
    println!("primary-region findings: {:?}", pf.ground.findings());
}
