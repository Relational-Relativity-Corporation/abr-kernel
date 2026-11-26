//! Kernel V10 Df6 relational field — IMPLEMENTATION TESTS ONLY.
//!
//! [IMPL] SYNTHETIC FIXTURES. The chain 0 → 1 → 2 with x = [3, 1, 2] is the
//! existing kernel fixture; later registrations, the second component and
//! carrier values are [INTERFACE VALUE]s. These tests verify the forms; they
//! establish no empirical validity.

#[path = "support/synthetic_identity.rs"]
mod synthetic_identity;
use abr_kernel::ground_variables::*;
use abr_kernel::operators::{
    operator_a, operator_b, operator_passage, DeclaredRelations, EdgeProvenance, NodeField,
};
use abr_kernel::relational_field::*;

fn chain() -> DeclaredRelations {
    synthetic_identity::identify(DeclaredRelations::from_edges_with_provenance(
        3, vec![(0, 1), (1, 2)], vec![EdgeProvenance::Continuation; 2]))
}

fn reg(data: Vec<Vec<f64>>) -> Registration {
    let delta = data.iter().map(|c| vec![0.0; c.len()]).collect();
    Registration::synthetic(NodeField::new(data), delta)
}

fn seq() -> Vec<Registration> {
    vec![
        reg(vec![vec![3.0, 1.0, 2.0], vec![0.0, 0.0, 5.0]]),
        reg(vec![vec![3.1, 1.0, 2.0], vec![0.0, 0.0, 5.2]]),
        reg(vec![vec![3.3, 1.1, 2.0], vec![0.1, 0.0, 5.3]]),
    ]
}

fn close(a: &[Vec<f64>], b: &[Vec<f64>]) -> bool {
    a.iter().flatten().zip(b.iter().flatten()).all(|(x, y)| (x - y).abs() < 1e-12)
}

#[test]
fn one_registration_is_insufficient_information() {
    let one = vec![reg(vec![vec![3.0, 1.0, 2.0]])];
    assert_eq!(RelationalField::new(&chain(), one, None).unwrap_err(),
        DeclarationGap::NoProcessSequence);
}

#[test]
fn every_form_and_ground_variable_is_evaluated_together_without_parameters() {
    let cs: Vec<EnergyCarrier> = [5.0, 5.2, 5.5].iter().map(|&v| EnergyCarrier { range: abr_kernel::ground_variables::ObservedRange::synthetic(),
        name: "synthetic carrier".into(), unit: "J".into(),
        values: vec![v, 2.0, 2.5], delta: vec![0.0; 3],
    }).collect();
    let f = RelationalField::new(&chain(), seq(), Some(&cs)).unwrap();
    assert_eq!(f.spatial.len(), 3);
    assert_eq!(f.n_steps(), 2);
    assert!(f.ground.complete(), "{:?}", f.ground.findings());
}

#[test]
fn spatial_forms_match_the_operators() {
    let rel = chain();
    let f = RelationalField::new(&rel, seq(), None).unwrap();
    for (t, r) in f.registrations.iter().enumerate() {
        let a = operator_a(&r.x, &rel, &[]);
        let b = operator_b(&a, &rel);
        assert_eq!(f.spatial[t].a, a.spatial);
        assert_eq!(f.spatial[t].b, b.spatial);
        assert_eq!(f.spatial[t].passage, operator_passage(&b.spatial, &rel));
    }
}

#[test]
fn persistence_is_the_difference_of_each_form_because_the_forms_are_linear() {
    let f = RelationalField::new(&chain(), seq(), None).unwrap();
    for t in 0..f.n_steps() {
        let (pri, cur, p) = (&f.spatial[t], &f.spatial[t + 1], &f.persistence[t]);
        let diff = |x: &Vec<Vec<f64>>, y: &Vec<Vec<f64>>| -> Vec<Vec<f64>> {
            x.iter().zip(y).map(|(u, v)| u.iter().zip(v).map(|(a, b)| a - b).collect()).collect()
        };
        assert!(close(&p.a, &diff(&cur.a, &pri.a)));
        assert!(close(&p.b, &diff(&cur.b, &pri.b)));
        assert!(close(&p.passage, &diff(&cur.passage, &pri.passage)));
    }
}

// Declared historical comparison with V9 (runs with `--features v9`).
#[cfg(feature = "v9")]
#[test]
fn passage_sign_equals_the_v9_r_minus_b_sign() {
    use abr_kernel::operators::{compute_rho, operator_r};
    // V9's R − B is ρ[src]·Ξ on the spatial term; ρ > 0 here, so the signs agree.
    let rel = chain();
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0]]);
    let a = operator_a(&x, &rel, &[]);
    let b = operator_b(&a, &rel);
    let r = operator_r(&b, &rel, &compute_rho(&a, &rel, 0.5, 1.0), &[]);
    let c = operator_passage(&b.spatial, &rel);
    for e in 0..2 {
        assert_eq!((r.spatial[0][e] - b.spatial[0][e]).signum(), c[0][e].signum());
    }
}

#[test]
fn missing_carrier_is_a_finding_not_a_failure_to_form_the_field() {
    let f = RelationalField::new(&chain(), seq(), None).unwrap();
    assert_eq!(f.ground.energy, Err(DeclarationGap::NoCarrierDeclared));
}
