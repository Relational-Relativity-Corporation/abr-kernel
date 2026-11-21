//! Kernel V10 Df6 relational field — IMPLEMENTATION TESTS ONLY.
//!
//! [IMPL] SYNTHETIC FIXTURES. The chain 0 → 1 → 2 with x = [3, 1, 2] is the
//! existing kernel fixture; later registrations, the second component and
//! carrier values are [INTERFACE VALUE]s. ρ_base = 0.5, χ₀ = 1 are the Origin
//! general values. These tests verify the forms; they establish no empirical
//! validity.

use abr_kernel::ground_variables::*;
use abr_kernel::operators::{
    operator_a_persistence, operator_e, DeclaredRelations, EdgeProvenance, NodeField,
};
use abr_kernel::relational_field::*;

fn chain() -> DeclaredRelations {
    DeclaredRelations::from_edges_with_provenance(
        3, vec![(0, 1), (1, 2)], vec![EdgeProvenance::Continuation; 2])
}

fn params() -> OperatorParameters {
    OperatorParameters { rho_base: 0.5, chi_0: 1.0, pairs: vec![], cc: vec![] }
}

fn reg(data: Vec<Vec<f64>>) -> Registration {
    let delta = data.iter().map(|c| vec![0.0; c.len()]).collect();
    Registration::new(NodeField::new(data), delta)
}

fn seq() -> Vec<Registration> {
    vec![
        reg(vec![vec![3.0, 1.0, 2.0], vec![0.0, 0.0, 5.0]]),
        reg(vec![vec![3.1, 1.0, 2.0], vec![0.0, 0.0, 5.2]]),
        reg(vec![vec![3.3, 1.1, 2.0], vec![0.1, 0.0, 5.3]]),
    ]
}

#[test]
fn one_registration_is_insufficient_information() {
    let one = vec![reg(vec![vec![3.0, 1.0, 2.0]])];
    assert_eq!(RelationalField::new(&chain(), one, &params(), None).unwrap_err(),
        DeclarationGap::NoProcessSequence);
}

#[test]
fn spatial_persistence_and_ground_variables_are_evaluated_together() {
    let cs: Vec<EnergyCarrier> = [5.0, 5.2, 5.5].iter().map(|&v| EnergyCarrier {
        name: "synthetic carrier".into(), unit: "J".into(),
        values: vec![v, 2.0, 2.5], delta: vec![0.0; 3],
    }).collect();
    let f = RelationalField::new(&chain(), seq(), &params(), Some(&cs)).unwrap();
    assert_eq!(f.spatial.len(), 3);
    assert_eq!(f.n_steps(), 2);
    assert!(f.ground.complete(), "{:?}", f.ground.findings());
}

#[test]
fn spatial_output_matches_the_unchanged_operator() {
    let f = RelationalField::new(&chain(), seq(), &params(), None).unwrap();
    for (t, r) in f.registrations.iter().enumerate() {
        let e = operator_e(&r.x, &chain(), &[], &[], 0.5, 1.0);
        assert_eq!(f.spatial[t].spatial, e.spatial);
    }
}

#[test]
fn persistence_pairs_each_registration_with_the_earlier_output() {
    let f = RelationalField::new(&chain(), seq(), &params(), None).unwrap();
    for t in 0..f.n_steps() {
        let expected = operator_a_persistence(&f.spatial[t + 1], &f.spatial[t]);
        assert_eq!(f.persistence[t].a_persistence, expected);
    }
}

#[test]
fn missing_carrier_is_a_finding_not_a_failure_to_form_the_field() {
    let f = RelationalField::new(&chain(), seq(), &params(), None).unwrap();
    assert_eq!(f.ground.energy, Err(DeclarationGap::NoCarrierDeclared));
}
