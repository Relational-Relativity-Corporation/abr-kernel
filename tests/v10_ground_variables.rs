//! Kernel V10 ground variables — IMPLEMENTATION TESTS ONLY.
//!
//! [IMPL] Every field below is a SYNTHETIC FIXTURE. The chain 0 → 1 → 2 with
//! x = [3, 1, 2] is the existing kernel fixture (tests/common
//! synthetic_primary_registry). Second and third components and carrier
//! values are [INTERFACE VALUE]s chosen to exercise a code path. These tests
//! verify the forms; they establish no empirical validity.

use abr_kernel::ground_variables::*;
use abr_kernel::operators::{
    compute_rho, operator_a, operator_b, operator_r, DeclaredRelations, EdgeProvenance, NodeField,
};

fn rel(n: usize, edges: &[(usize, usize)]) -> DeclaredRelations {
    DeclaredRelations::from_edges_with_provenance(
        n, edges.to_vec(), vec![EdgeProvenance::Continuation; edges.len()])
}

fn zeros(k: usize, n: usize) -> Vec<Vec<f64>> {
    vec![vec![0.0; n]; k]
}

fn chain() -> DeclaredRelations {
    rel(3, &[(0, 1), (1, 2)])
}

// ── Freedom ───────────────────────────────────────────────────────────────

#[test]
fn freedom_two_components_distinguishable() {
    // A[e0] = (2, 0), A[e1] = (−1, −5): minor = 2·(−5) − 0·(−1) = −10.
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0], vec![0.0, 0.0, 5.0]]);
    let f = freedom_degree(&x, &zeros(2, 3), &chain()).unwrap();
    assert_eq!(f.distinguishable_pairs, vec![(0, 1)]);
    assert_eq!(f.f_delta, 1.0);
}

#[test]
fn freedom_minor_is_symmetric_in_edge_order() {
    let a = vec![vec![2.0, -1.0], vec![0.0, -5.0]];
    assert_eq!(freedom_minor(&a, 0, 1, 0, 1), -freedom_minor(&a, 1, 0, 0, 1));
    let d = zeros(2, 2);
    assert_eq!(freedom_pair(&a, &d, 0, 1), freedom_pair(&a, &d, 1, 0));
}

#[test]
fn freedom_zero_edge_is_reported_separately_in_either_order() {
    // The V8 HOLD case: one A vector is 0.0 in every component. The edge is
    // not resolved at δ and enters no pair, whichever edge is indexed first.
    let a = vec![vec![0.0, 3.0], vec![0.0, 1.0]];
    let d = zeros(2, 2);
    assert_eq!(freedom_pair(&a, &d, 0, 1).unwrap(), AtDelta::NotResolved);
    assert_eq!(freedom_pair(&a, &d, 1, 0).unwrap(), AtDelta::NotResolved);
}

#[test]
fn freedom_proportional_field_is_a_finding_not_absence() {
    // Second component = 2 × first: every minor is zero.
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0], vec![6.0, 2.0, 4.0]]);
    let r = ground_reading(&x, &zeros(2, 3), &chain(), None);
    assert_eq!(r.freedom.as_ref().unwrap().f_delta, 0.0);
    assert!(r.findings().contains(&Finding {
        variable: GroundVariable::Freedom, kind: FindingKind::NotResolvedAtDelta, gap: None,
    }));
}

#[test]
fn freedom_coarse_delta_does_not_resolve_the_minor() {
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0], vec![0.0, 0.0, 5.0]]);
    // δ = 0.5 per locus: δ_A = 1, both edges resolved; minor −10, half-width 10.
    let f = freedom_degree(&x, &vec![vec![0.5; 3]; 2], &chain()).unwrap();
    assert_eq!(f.evaluated_pairs, 1);
    assert!(f.distinguishable_pairs.is_empty());
    // δ = 2 per locus: e0 is not resolved, so no pair can be read.
    assert_eq!(freedom_degree(&x, &vec![vec![2.0; 3]; 2], &chain()).unwrap_err(),
        DeclarationGap::NoResolvedPair);
}

#[test]
fn freedom_three_components_reads_every_minor() {
    // Components 0 and 1 are proportional; component 2 is not (OC-V-2).
    let x = NodeField::new(vec![
        vec![3.0, 1.0, 2.0],
        vec![6.0, 2.0, 4.0],
        vec![0.0, 0.0, 5.0],
    ]);
    let f = freedom_degree(&x, &zeros(3, 3), &chain()).unwrap();
    assert_eq!(f.f_delta, 1.0);
}

#[test]
fn freedom_single_component_is_a_declaration_gap() {
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0]]);
    let g = freedom_degree(&x, &zeros(1, 3), &chain()).unwrap_err();
    assert_eq!(g, DeclarationGap::SingleComponent);
    assert_eq!(g.finding(), FindingKind::DomainNotFullyDeclared);
}

// ── Connection ────────────────────────────────────────────────────────────

#[test]
fn connection_sign_matches_r_minus_b_from_the_operators() {
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0]]);
    let rel = chain();
    let c = connection_degree(&x, &zeros(1, 3), &rel).unwrap();

    let a = operator_a(&x, &rel, &[]);
    let b = operator_b(&a, &rel);
    let rho = compute_rho(&a, &rel, 0.5, 1.0);
    let r = operator_r(&b, &rel, &rho, &[]);
    for reading in &c.edges {
        let e = reading.edge;
        let r_minus_b = r.spatial[0][e] - b.spatial[0][e];
        let expected = if r_minus_b > 0.0 { Sign::Positive } else { Sign::Negative };
        assert_eq!(reading.sign[0], AtDelta::Resolved(expected));
    }
    assert_eq!(c.share_connected, 1.0);
}

#[test]
fn connection_without_interior_is_primary_region() {
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0, 0.0]]);
    let g = connection_degree(&x, &zeros(1, 4), &rel(4, &[(0, 1), (2, 3)])).unwrap_err();
    assert_eq!(g, DeclarationGap::PrimaryRegion);
    assert_eq!(g.finding(), FindingKind::PrimaryRegion);
}

#[test]
fn connection_isolated_edge_is_not_formable_not_decoherent() {
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0, 7.0, 4.0]]);
    let rel = rel(5, &[(0, 1), (1, 2), (3, 4)]);
    let c = connection_degree(&x, &zeros(1, 5), &rel).unwrap();
    assert_eq!(c.not_formable, vec![2]);
    assert_eq!(c.edges.len(), 2);
    assert_eq!(connection_at_edge(&x, &zeros(1, 5), &rel, 2).unwrap_err(),
        DeclarationGap::NoNeighbourInField { edge: 2 });
}

#[test]
fn connection_half_width_propagates_through_b() {
    // δ = 0.5 at every locus: δ_A = 1 per edge; δ_B[e0] = 2, δ_B[e1] = 1.
    // At e0 the bracket is B[e1] = −1 with half-width δ_B[e1] = 1: not resolved.
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0]]);
    let r = connection_at_edge(&x, &vec![vec![0.5; 3]], &chain(), 0).unwrap();
    assert_eq!(r.half_width[0], 1.0);
    assert_eq!(r.sign[0], AtDelta::NotResolved);
}

// ── Energy ────────────────────────────────────────────────────────────────

fn carrier(values: Vec<f64>, delta: Vec<f64>) -> EnergyCarrier {
    EnergyCarrier { name: "synthetic carrier".into(), unit: "J".into(), values, delta }
}

#[test]
fn energy_is_the_exchange_of_the_declared_carrier() {
    let c = carrier(vec![5.0, 2.0, 2.5], vec![0.1, 0.1, 0.1]);
    let e0 = energy_exchange(Some(&c), &chain(), 0).unwrap();
    assert_eq!(e0.epsilon, 3.0);
    assert!((e0.delta_epsilon - 0.2).abs() < 1e-12);
    assert!(e0.floor_resolved);
    assert!((e0.degree().unwrap() - 15.0).abs() < 1e-9);
}

#[test]
fn energy_without_carrier_is_a_declaration_gap() {
    let g = energy_degree(None, &chain()).unwrap_err();
    assert_eq!(g, DeclarationGap::NoCarrierDeclared);
    assert_eq!(g.finding(), FindingKind::DomainNotFullyDeclared);
}

#[test]
fn energy_is_not_multiplied_by_connection() {
    // No interior: connection is a gap, energy is still calculated.
    let rel = rel(4, &[(0, 1), (2, 3)]);
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0, 0.0]]);
    let c = carrier(vec![5.0, 2.0, 1.0, 1.5], vec![0.0; 4]);
    let r = ground_reading(&x, &zeros(1, 4), &rel, Some(&c));
    assert!(r.connection.is_err());
    assert_eq!(r.energy.as_ref().unwrap().share_resolved, 1.0);
}

#[test]
fn connected_exchange_needs_both_resolved() {
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0]]);
    let conn = connection_at_edge(&x, &zeros(1, 3), &chain(), 0).unwrap();
    let c = carrier(vec![5.0, 2.0, 2.5], vec![0.0; 3]);
    let en = energy_exchange(Some(&c), &chain(), 0).unwrap();
    assert_eq!(connected_exchange(&en, &conn), AtDelta::Resolved(3.0));

    let wide = carrier(vec![5.0, 2.0, 2.5], vec![2.0; 3]);
    let en = energy_exchange(Some(&wide), &chain(), 0).unwrap();
    assert_eq!(connected_exchange(&en, &conn), AtDelta::NotResolved);
}

#[test]
fn mapped_carrier_half_width_takes_the_larger_side() {
    let hw = mapped_half_width(|c| c * c, 3.0, 0.5);
    assert!((hw - 3.25).abs() < 1e-12); // 3.5² − 9 = 3.25 > 9 − 2.5² = 2.75
}

// ── All three ─────────────────────────────────────────────────────────────

#[test]
fn ground_reading_complete_when_all_three_calculate() {
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0], vec![0.0, 0.0, 5.0]]);
    let c = carrier(vec![5.0, 2.0, 2.5], vec![0.0; 3]);
    let r = ground_reading(&x, &zeros(2, 3), &chain(), Some(&c));
    assert!(r.complete(), "{:?}", r.findings());
}

#[test]
fn ground_reading_names_every_gap() {
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0, 0.0]]);
    let r = ground_reading(&x, &zeros(1, 4), &rel(4, &[(0, 1), (2, 3)]), None);
    let kinds: Vec<_> = r.findings().iter().map(|f| (f.variable, f.kind)).collect();
    assert_eq!(kinds, vec![
        (GroundVariable::Freedom, FindingKind::DomainNotFullyDeclared),
        (GroundVariable::Connection, FindingKind::PrimaryRegion),
        (GroundVariable::Energy, FindingKind::DomainNotFullyDeclared),
    ]);
}
