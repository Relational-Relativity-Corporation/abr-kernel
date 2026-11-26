//! Kernel V10.3: the observed range (OC-RANGE-1; Declared, Origin 2026-11-23,
//! 47th). A registered value x with repeatability δ resolves |x| / δ
//! distinctions. It is a registration only when |x| ≤ N·δ: N = 10²⁰ by default
//! (the human range), or a declared N at most that. Test fixtures of the
//! forms declare themselves synthetic.
//!
//! [IMPL] These tests exercise the admission rule. Values are synthetic where
//! marked; the rule itself is what is tested.

use abr_kernel::ground_variables::*;
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::persistence_change::EdgeSeries;

fn one(x: f64, d: f64) -> (NodeField, Vec<Vec<f64>>) { (NodeField::new(vec![vec![0.0, x]]), vec![vec![0.0, d]]) }

#[test]
fn the_human_range_admits_up_to_ten_to_the_twenty_distinctions() {
    assert_eq!(HUMAN_RESOLVABLE_COUNT, 1e20);
    let (x, d) = one(1e20, 1.0);
    let _ = Registration::new(x, d);
    assert!(ObservedRange::Human.admits(1e20, 1.0));
    assert!(!ObservedRange::Human.admits(1e20f64.next_up(), 1.0), "the boundary is decided exactly");
    assert!(ObservedRange::Human.admits(0.0, 0.0), "zero with no resolution claims nothing");
}

#[test]
#[should_panic(expected = "outside the observed range")]
fn a_nonzero_value_with_no_resolution_is_not_a_registration() {
    let (x, d) = one(1.0, 0.0);
    let _ = Registration::new(x, d);
}

#[test]
#[should_panic(expected = "outside the observed range")]
fn more_than_ten_to_the_twenty_distinctions_is_not_a_registration() {
    let (x, d) = one(2e20, 1.0);
    let _ = Registration::new(x, d);
}

#[test]
#[should_panic(expected = "outside the observed range")]
fn an_edge_value_with_no_resolution_is_not_a_registration() {
    let _ = EdgeSeries::from_edge_values(vec![vec![vec![1.0]]; 2], vec![vec![vec![0.0]]; 2]);
}

#[test]
#[should_panic(expected = "outside the observed range")]
fn a_carrier_with_no_resolution_is_not_a_registration() {
    let r = DeclaredRelations::from_edges_with_provenance(2, vec![(0, 1)], vec![EdgeProvenance::Continuation]);
    let c = EnergyCarrier { range: ObservedRange::Human, name: "c".into(), unit: "u".into(), values: vec![0.0, 3.0], delta: vec![0.0, 0.0] };
    let _ = signed_exchanges(&c, &r);
}

#[test]
fn a_chain_declares_a_smaller_range_with_its_basis() {
    let range = ObservedRange::declared(1000.0, "EXAMPLE: a balance resolving 1 part in 10³ of its full scale");
    let (x, d) = one(1000.0, 1.0);
    let r = Registration::new(x, d).with_observed_range(range.clone());
    assert_eq!(r.range, range);
    assert!(!range.admits(1001.0, 1.0));
}

#[test]
#[should_panic(expected = "outside the observed range")]
fn a_value_outside_its_declared_range_is_refused() {
    let (x, d) = one(1001.0, 1.0);
    let _ = Registration::new(x, d).with_observed_range(ObservedRange::declared(1000.0, "EXAMPLE"));
}

#[test]
#[should_panic(expected = "at most the human range")]
fn a_declared_range_cannot_exceed_the_human_range() {
    let _ = ObservedRange::declared(1e21, "EXAMPLE");
}

#[test]
#[should_panic(expected = "states the comparison")]
fn a_declared_range_states_its_basis() {
    let _ = ObservedRange::declared(10.0, " ");
}

#[test]
#[should_panic(expected = "cannot be made synthetic after admission")]
fn a_registration_cannot_be_made_synthetic_after_admission() {
    let (x, d) = one(1.0, 1.0);
    let _ = Registration::new(x, d).with_observed_range(ObservedRange::synthetic());
}

#[test]
fn a_synthetic_fixture_is_bounded_only_by_the_arithmetic_range() {
    let (x, d) = one(1.0, 0.0);
    let r = Registration::synthetic(x, d);
    assert!(matches!(r.range, ObservedRange::Synthetic { .. }));
    assert!(EdgeSeries::from_edge_values_synthetic(vec![vec![vec![1.0]]; 2], vec![vec![vec![0.0]]; 2]).is_ok());
}
