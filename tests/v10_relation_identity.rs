//! Kernel V10.3 — RPPI-V10 G-1 (adopted by Origin, 2026-11-22, 31st): every
//! declared relation carries an identity and a direction basis; a field
//! without them is refused at every V10 reading entry point.
//!
//! [IMPL] SYNTHETIC FIXTURES: declarations are chosen to exercise the check.

use abr_kernel::ground_variables::{ground_reading, DeclarationGap, Registration};
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::relational_field::RelationalField;
use abr_kernel::topology::*;

fn field() -> DeclaredRelations {
    DeclaredRelations::from_edges_with_provenance(3, vec![(0, 1), (1, 2), (0, 1)], vec![EdgeProvenance::Continuation; 3])
}

fn id(i: &str, b: &str) -> RelationIdentity { RelationIdentity { id: i.into(), basis: b.into() } }

fn good() -> Vec<RelationIdentity> {
    vec![id("a", "energy enters at 0 (registered source)"), id("b", "energy moves on from 1"), id("c", "second registered path 0 → 1")]
}

fn seq() -> Vec<Registration> {
    (0..2).map(|t| Registration::new(NodeField::new(vec![vec![3.0 + t as f64, 1.0, 0.0]]), vec![vec![0.0625; 3]])).collect()
}

#[test]
fn identity_declared_for_every_edge_is_admitted() {
    let r = field().with_identity(good());
    assert_eq!(check_identity(&r), Ok(()));
    assert!(EdgeSeries::from_registrations(&seq(), &r).is_ok());
}

#[test]
fn a_field_with_no_identity_is_refused_at_every_entry_point() {
    let r = field();
    let gap = DeclarationGap::RelationIdentity(vec![IdentityRefusal::NotDeclared]);
    assert_eq!(EdgeSeries::from_registrations(&seq(), &r).unwrap_err(), gap);
    assert_eq!(RelationalField::new(&r, seq(), None).unwrap_err(), gap);
    let g = ground_reading(&seq(), &r, None);
    assert_eq!(g.connection.unwrap_err(), gap);
    assert_eq!(g.v2_held_arrangement.unwrap_err(), gap);
    assert!(gap.remedy().contains("a class label is not an identity"));
}

#[test]
fn each_identity_fault_is_named() {
    let mut v = good();
    assert_eq!(check_identity(&field().with_identity(v[..2].to_vec())),
        Err(vec![IdentityRefusal::CountMismatch { declared: 2, edges: 3 }]));
    v[1].id = "  ".into();
    v[2].basis = "".into();
    assert_eq!(check_identity(&field().with_identity(v)),
        Err(vec![IdentityRefusal::EmptyId { edge: 1 }, IdentityRefusal::EmptyBasis { edge: 2 }]));
}

#[test]
fn parallel_edges_are_distinct_relations_with_distinct_ids() {
    let mut v = good();
    v[2].id = "a".into();
    assert_eq!(check_identity(&field().with_identity(v)), Err(vec![IdentityRefusal::DuplicateId { edge: 2, first: 0 }]));
}

#[test]
fn identity_is_checked_before_t8() {
    // An undeclared loop and no identity: the identity refusal comes first.
    let r = DeclaredRelations::from_edges_with_provenance(2, vec![(0, 1), (1, 0)], vec![EdgeProvenance::Continuation; 2]);
    assert!(matches!(admit_field(&r), Err(DeclarationGap::RelationIdentity(_))));
    let r = r.with_identity(vec![id("f", "forward"), id("b", "return")]);
    assert!(matches!(admit_field(&r), Err(DeclarationGap::T8(_))));
}
