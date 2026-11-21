//! Kernel V10 ground variables over a declared sequence — IMPLEMENTATION TESTS ONLY.
//!
//! [IMPL] Every field below is a SYNTHETIC FIXTURE. The chain 0 → 1 → 2 with
//! x = [3, 1, 2] is the existing kernel fixture (tests/common
//! synthetic_primary_registry). Later registrations, extra components and
//! carrier values are [INTERFACE VALUE]s chosen to exercise a code path. These
//! tests verify the forms; they establish no empirical validity.

use abr_kernel::ground_variables::*;
use abr_kernel::operators::{
    compute_rho, operator_a, operator_b, operator_r, DeclaredRelations, EdgeProvenance, NodeField,
};

fn rel(n: usize, edges: &[(usize, usize)]) -> DeclaredRelations {
    DeclaredRelations::from_edges_with_provenance(
        n, edges.to_vec(), vec![EdgeProvenance::Continuation; edges.len()])
}

fn chain() -> DeclaredRelations {
    rel(3, &[(0, 1), (1, 2)])
}

/// A registration with uniform δ.
fn reg(data: Vec<Vec<f64>>, d: f64) -> Registration {
    let delta = data.iter().map(|c| vec![d; c.len()]).collect();
    Registration::new(NodeField::new(data), delta)
}

fn carrier(values: Vec<f64>, d: f64) -> EnergyCarrier {
    let n = values.len();
    EnergyCarrier { name: "synthetic carrier".into(), unit: "J".into(), values, delta: vec![d; n] }
}

// ── The sequence requirement ──────────────────────────────────────────────

#[test]
fn a_single_registration_is_not_a_relational_reading() {
    let seq = vec![reg(vec![vec![3.0, 1.0, 2.0], vec![0.0, 0.0, 5.0]], 0.0)];
    let c = vec![carrier(vec![5.0, 2.0, 2.5], 0.0)];
    let r = ground_reading(&seq, &chain(), Some(&c));
    for f in r.findings() {
        assert_eq!(f.gap, Some(DeclarationGap::NoProcessSequence));
        assert_eq!(f.kind, FindingKind::DomainNotFullyDeclared);
    }
    assert_eq!(r.findings().len(), 3);
}

// ── The minor pattern ─────────────────────────────────────────────────────

#[test]
fn minor_is_antisymmetric_in_edge_order() {
    let a = vec![vec![2.0, -1.0], vec![0.0, -5.0]];
    assert_eq!(minor(&a, 0, 1, 0, 1), -minor(&a, 1, 0, 0, 1));
}

#[test]
fn kappa_summarises_the_pattern() {
    let p = minor_pattern(&reg(vec![vec![3.0, 1.0, 2.0], vec![0.0, 0.0, 5.0]], 0.0), &chain()).unwrap();
    assert_eq!(p.kappa(), Some(1.0));
    let p = minor_pattern(&reg(vec![vec![3.0, 1.0, 2.0], vec![6.0, 2.0, 4.0]], 0.0), &chain()).unwrap();
    assert_eq!(p.kappa(), Some(0.0));
}

#[test]
fn persistence_bound_covers_the_actual_change_and_preserves_sign() {
    let rel = chain();
    let steps = [
        (vec![vec![3.0, 1.0, 2.0], vec![0.0, 0.0, 5.0]], vec![vec![3.2, 1.1, 2.0], vec![0.1, 0.0, 5.3]]),
        (vec![vec![3.0, 1.0, 2.0], vec![1.0, 0.0, 5.0]], vec![vec![3.0, 1.0, 2.0], vec![1.0, 0.0, -5.0]]),
    ];
    for (x0, x1) in steps {
        let a0 = operator_a(&NodeField::new(x0), &rel, &[]).spatial;
        let a1 = operator_a(&NodeField::new(x1), &rel, &[]).spatial;
        let ap: Vec<Vec<f64>> = (0..2).map(|c| (0..2).map(|e| a1[c][e] - a0[c][e]).collect()).collect();
        let (m0, m1) = (minor(&a0, 0, 1, 0, 1), minor(&a1, 0, 1, 0, 1));
        let bound = minor_change_bound(&a0, &ap, 0, 1, 0, 1);
        assert!((m1 - m0).abs() <= bound + 1e-12);
        if step_preserves_sign(m0, bound) {
            assert_eq!(m0.signum(), m1.signum());
        }
    }
}

// ── Freedom ───────────────────────────────────────────────────────────────

#[test]
fn freedom_held_when_a_minor_keeps_its_sign() {
    let seq = vec![
        reg(vec![vec![3.0, 1.0, 2.0], vec![0.0, 0.0, 5.0]], 0.0),
        reg(vec![vec![3.1, 1.0, 2.0], vec![0.0, 0.0, 5.2]], 0.0),
    ];
    let f = freedom(&seq, &chain()).unwrap();
    assert_eq!(f.free_pairs, vec![(0, 1)]);
    assert_eq!(f.f_delta, 1.0);
}

#[test]
fn a_crossing_minor_is_reorganization_not_freedom_or_connection() {
    // Minor −9 at the first registration, +11 at the second.
    let seq = vec![
        reg(vec![vec![3.0, 1.0, 2.0], vec![1.0, 0.0, 5.0]], 0.0),
        reg(vec![vec![3.0, 1.0, 2.0], vec![1.0, 0.0, -5.0]], 0.0),
    ];
    let h = minor_histories(&seq, &chain()).unwrap();
    assert_eq!(h.minors[0].4, History::Reorganizing { crossed: true });
    assert_eq!(freedom(&seq, &chain()).unwrap().f_delta, 0.0);
    let c = connection(&seq, &chain()).unwrap();
    assert_eq!(c.reorganizing_pairs, vec![(0, 1)]);
    assert!(c.proportional.unwrap().is_empty());
}

#[test]
fn an_unresolved_edge_enters_no_pair() {
    let rel = rel(4, &[(0, 1), (1, 2), (2, 3)]);
    let seq = vec![
        reg(vec![vec![3.0, 3.0, 1.0, 2.0], vec![0.0, 0.0, 1.0, 5.0]], 0.0),
        reg(vec![vec![3.0, 3.0, 1.0, 2.0], vec![0.0, 0.0, 1.0, 5.0]], 0.0),
    ];
    let f = freedom(&seq, &rel).unwrap();
    assert_eq!(f.unresolved_edges, vec![0]);
    assert_eq!(f.evaluated_pairs, 1);
}

#[test]
fn freedom_with_one_component_is_a_declaration_gap() {
    let seq = vec![reg(vec![vec![3.0, 1.0, 2.0]], 0.0), reg(vec![vec![3.5, 1.0, 2.0]], 0.0)];
    assert_eq!(freedom(&seq, &chain()).unwrap_err(), DeclarationGap::SingleComponent);
}

// ── Connection ────────────────────────────────────────────────────────────

#[test]
fn proportional_grade_holds_while_values_change() {
    // Component 1 = 2 × component 0 at both registrations; values change.
    let seq = vec![
        reg(vec![vec![3.0, 1.0, 2.0], vec![6.0, 2.0, 4.0]], 0.0),
        reg(vec![vec![4.0, 1.0, 2.0], vec![8.0, 2.0, 4.0]], 0.0),
    ];
    let c = connection(&seq, &chain()).unwrap();
    assert_eq!(c.proportional.unwrap(), vec![(0, 1)]);
    let r = ground_reading(&seq, &chain(), None);
    assert!(r.findings().contains(&Finding {
        variable: GroundVariable::Freedom, grade: None, kind: FindingKind::NotResolvedAtDelta, gap: None,
    }));
}

#[test]
fn identity_grade_needs_one_component_and_no_interior() {
    // Two separate edges with the same difference at both registrations.
    let rel = rel(4, &[(0, 1), (2, 3)]);
    let seq = vec![
        reg(vec![vec![2.0, 1.0, 5.0, 4.0]], 0.0),
        reg(vec![vec![2.5, 1.5, 6.0, 5.0]], 0.0),
    ];
    let c = connection(&seq, &rel).unwrap();
    assert_eq!(c.identity, vec![(0, 1)]);
    assert_eq!(c.proportional, Err(DeclarationGap::SingleComponent));
    assert_eq!(c.sustained, Err(DeclarationGap::PrimaryRegion));
}

#[test]
fn at_the_primary_region_only_sustained_connection_is_a_gap() {
    let rel = rel(4, &[(0, 1), (2, 3)]);
    let seq = vec![
        reg(vec![vec![3.0, 1.0, 2.0, 0.0], vec![0.0, 0.0, 5.0, 1.0]], 0.0),
        reg(vec![vec![3.2, 1.0, 2.0, 0.0], vec![0.0, 0.0, 5.1, 1.0]], 0.0),
    ];
    let c = vec![carrier(vec![5.0, 2.0, 1.0, 1.5], 0.0), carrier(vec![5.1, 2.0, 1.0, 1.5], 0.0)];
    let r = ground_reading(&seq, &rel, Some(&c));
    assert!(r.energy.is_ok());
    assert_eq!(r.freedom.as_ref().unwrap().f_delta, 1.0);
    assert!(r.connection.is_ok());
    // The only gap is the sustained grade. The two relations vary
    // independently here, so connection's degree is zero at δ: a finding of
    // the same kind (W2), not absence.
    assert_eq!(r.findings(), vec![
        Finding {
            variable: GroundVariable::Connection,
            grade: Some(ConnectionGrade::Sustained),
            kind: FindingKind::PrimaryRegion,
            gap: Some(DeclarationGap::PrimaryRegion),
        },
        Finding {
            variable: GroundVariable::Connection,
            grade: None,
            kind: FindingKind::NotResolvedAtDelta,
            gap: None,
        },
    ]);
}

#[test]
fn sustained_sign_matches_r_minus_b_from_the_operators() {
    let rel = chain();
    let xs = [vec![vec![3.0, 1.0, 2.0]], vec![vec![3.4, 1.2, 2.0]]];
    let seq: Vec<Registration> = xs.iter().map(|x| reg(x.clone(), 0.0)).collect();
    let s = connection(&seq, &rel).unwrap().sustained.unwrap();
    assert_eq!(s.held_edges, vec![0, 1]);
    for x in xs {
        let x = NodeField::new(x);
        let a = operator_a(&x, &rel, &[]);
        let b = operator_b(&a, &rel);
        let r = operator_r(&b, &rel, &compute_rho(&a, &rel, 0.5, 1.0), &[]);
        for e in 0..2 {
            assert!(r.spatial[0][e] - b.spatial[0][e] < 0.0, "fixture: (R − B) negative at both edges");
        }
    }
}

#[test]
fn sustained_edge_with_no_neighbour_is_not_formable() {
    let rel = rel(5, &[(0, 1), (1, 2), (3, 4)]);
    let seq = vec![
        reg(vec![vec![3.0, 1.0, 2.0, 7.0, 4.0]], 0.0),
        reg(vec![vec![3.1, 1.0, 2.0, 7.0, 4.0]], 0.0),
    ];
    let s = connection(&seq, &rel).unwrap().sustained.unwrap();
    assert_eq!(s.not_formable, vec![2]);
    assert_eq!(s.share_held, 1.0);
}

#[test]
fn coarse_delta_leaves_circulation_unresolved() {
    let seq = vec![reg(vec![vec![3.0, 1.0, 2.0]], 0.5), reg(vec![vec![3.0, 1.0, 2.0]], 0.5)];
    let s = connection(&seq, &chain()).unwrap().sustained.unwrap();
    assert!(!s.held_edges.contains(&0));
}

// ── Energy ────────────────────────────────────────────────────────────────

#[test]
fn energy_is_the_exchange_of_the_declared_carrier() {
    let x = energy_exchange(&carrier(vec![5.0, 2.0, 2.5], 0.1), &chain());
    assert_eq!(x[0].epsilon, 3.0);
    assert!((x[0].delta_epsilon - 0.2).abs() < 1e-12);
    assert!(x[0].floor_resolved);
    assert!((x[0].degree().unwrap() - 15.0).abs() < 1e-9);
}

#[test]
fn energy_without_carrier_is_a_declaration_gap() {
    assert_eq!(energy(None, 2, &chain()).unwrap_err(), DeclarationGap::NoCarrierDeclared);
}

#[test]
fn connected_exchange_needs_held_circulation_and_resolved_floor() {
    let seq = vec![reg(vec![vec![3.0, 1.0, 2.0]], 0.0), reg(vec![vec![3.4, 1.2, 2.0]], 0.0)];
    let c = connection(&seq, &chain()).unwrap();
    let cs = vec![carrier(vec![5.0, 2.0, 2.5], 0.0), carrier(vec![5.5, 2.0, 2.5], 0.0)];
    let en = energy(Some(&cs), 2, &chain()).unwrap();
    assert_eq!(connected_exchange(&en, &c), vec![(0, vec![3.0, 3.5]), (1, vec![0.5, 0.5])]);
}

#[test]
fn mapped_carrier_half_width_takes_the_larger_side() {
    assert!((mapped_half_width(|c| c * c, 3.0, 0.5) - 3.25).abs() < 1e-12);
}

// ── All three ─────────────────────────────────────────────────────────────

#[test]
fn ground_reading_complete_over_a_sequence() {
    let seq = vec![
        reg(vec![vec![3.0, 1.0, 2.0], vec![0.0, 0.0, 5.0]], 0.0),
        reg(vec![vec![3.1, 1.0, 2.0], vec![0.0, 0.0, 5.2]], 0.0),
    ];
    let cs = vec![carrier(vec![5.0, 2.0, 2.5], 0.0), carrier(vec![5.5, 2.0, 2.5], 0.0)];
    let r = ground_reading(&seq, &chain(), Some(&cs));
    assert!(r.complete(), "{:?}", r.findings());
}
