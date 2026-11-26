//! Self-spokes, declared with a basis (Origin, 2026-11-26, 109th): *"yes
//! include self spokes and declare them with that basis"*, the basis being
//! the order of reading. Text's loci are the elements of the alphabet, and
//! each locus's radial graph has a spoke to every element, itself included
//! (a doubled character).
//!
//! T8 as amended: on relations registered directly, a self-relation (s, s)
//! declared as feedback with a basis is admitted, a turn of one relation;
//! undeclared, or with an empty basis, it is refused as before. Formed from
//! loci it stays refused: its A is x[s] − x[s], zero by identity.
//!
//! The whole-field test forms the complete radial graphs over three
//! elements (nine spokes, three of them self-spokes), every spoke declared
//! with the basis "the order of reading", relations registered directly at
//! each registration, and checks A, B, Ξ and the readback's halves against
//! their definitions. **Not read here (named gaps):** what text registers
//! on each spoke (109th, open); energy (no carrier declared); the scale of
//! the subject.
//!
//! [IMPL] SYNTHETIC FIXTURES, enumerated from declared seeds (W17).

#[path = "support/synthetic_whole_field.rs"]
mod support;

use abr_kernel::exact::Expansion;
use abr_kernel::ground_variables::DeclarationGap;
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance};
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::relational_field::RelationalField;
use abr_kernel::topology::{check_t8, FeedbackDeclaration, RelationIdentity, T8Refusal};
use support::*;

const BASIS: &str = "the order of reading (Origin, 109th)";

/// The complete radial graphs over n elements: a spoke from every element
/// to every element, itself included, in order (s, t) by s then t.
fn radial(n: usize) -> DeclaredRelations {
    let edges: Vec<(usize, usize)> = (0..n).flat_map(|s| (0..n).map(move |t| (s, t))).collect();
    let m = edges.len();
    let ids = (0..m).map(|e| RelationIdentity { id: format!("SYNTHETIC-spoke-{e}"), basis: "SYNTHETIC: a spoke of a radial graph".into() }).collect();
    DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m]).with_identity(ids)
}

fn declared_with(rel: DeclaredRelations, basis: &str) -> DeclaredRelations {
    let fb = (0..rel.n_edges()).map(|e| FeedbackDeclaration { edge: e, basis: basis.into() }).collect();
    rel.with_feedback(fb)
}

/// 1. A self-relation declared as feedback with a basis is admitted, its
/// turn the relation alone.
#[test]
fn one_a_declared_self_spoke_is_admitted() {
    let rel = support::rel(2, vec![(0, 0), (0, 1)]);
    let a = check_t8(&rel, &[FeedbackDeclaration { edge: 0, basis: BASIS.into() }]).expect("admitted");
    assert_eq!(a.feedback, vec![0]);
    assert_eq!(a.turns, vec![vec![0]], "its turn is the relation alone");
}

/// 2. Undeclared, or declared with an empty basis, a self-relation is
/// refused as before.
#[test]
fn two_an_undeclared_self_spoke_is_refused() {
    let rel = support::rel(2, vec![(0, 0), (0, 1)]);
    assert!(check_t8(&rel, &[]).unwrap_err().contains(&T8Refusal::SelfRelation { edge: 0 }));
    let e = check_t8(&rel, &[FeedbackDeclaration { edge: 0, basis: "  ".into() }]).unwrap_err();
    assert!(e.contains(&T8Refusal::FeedbackWithoutBasis { edge: 0 }), "{e:?}");
}

/// 3. Formed from loci, a self-relation registers nothing (x[s] − x[s] is
/// zero by identity): refused even when declared.
#[test]
fn three_from_loci_a_self_spoke_is_refused() {
    let rel = declared_with(radial(2), BASIS);
    let d = Declared::repeatability(vec![vec![vec![0.0, 1.0]], vec![vec![0.5, 1.0]]], vec![vec![1.0 / 16.0; 2]]);
    match RelationalField::new(&rel, registrations(&d), None) {
        Err(DeclarationGap::T8(v)) => assert!(v.iter().any(|x| matches!(x, T8Refusal::SelfRelation { .. })), "{v:?}"),
        other => panic!("refused, with the self-relation named: {other:?}"),
    }
}

/// 4. The complete radial graphs, every spoke declared with the basis,
/// relations registered directly: the whole field forms, and A, B, Ξ and
/// the readback read every spoke, self-spokes included, by their
/// definitions.
#[test]
fn four_the_operators_read_the_complete_radial_graphs() {
    let rel = declared_with(radial(3), BASIS);
    assert!(check_t8(&rel, &rel.feedback).is_ok(), "the complete radial graphs, declared, are admitted");
    for_all("self-spokes: the operators on the radial graphs", 0x109_01, |r| {
        let (m, regs) = (rel.n_edges(), r.usize(2, 4));
        let a: Vec<Vec<Vec<f64>>> = (0..regs).map(|_| vec![(0..m).map(|_| r.eighth(-2.0, 2.0)).collect()]).collect();
        let da: Vec<Vec<Vec<f64>>> = (0..regs).map(|_| vec![(0..m).map(|_| r.delta(0.25)).collect()]).collect();
        let series = EdgeSeries::from_edge_values_synthetic(a.clone(), da).unwrap();
        let f = RelationalField::from_relations(&rel, series, None).expect("the field forms on the radial graphs");
        // Every variable read, or named: no carrier is declared for text yet
        // (109th, open), so energy is the named gap; freedom and connection
        // are read on the radial graphs.
        assert_eq!(f.ground.energy.as_ref().err(), Some(&DeclarationGap::NoCarrierDeclared));
        assert!(f.ground.freedom.is_ok(), "freedom is read");
        assert!(f.ground.connection.is_ok(), "connection is read");
        let edges = &rel.edges;
        let succ = |e: usize| -> Vec<usize> { (0..m).filter(|&g| edges[g].0 == edges[e].1).collect() };
        let pred = |e: usize| -> Vec<usize> { (0..m).filter(|&g| edges[g].1 == edges[e].0).collect() };
        for t in 0..regs {
            let x = |e: usize| Expansion::of(a[t][0][e]);
            let b = |e: usize| -> Expansion { let mut v = x(e); for g in succ(e) { v = v.plus(&x(g)); } v };
            for e in 0..m {
                assert_eq!(f.spatial[t].a[0][e], a[t][0][e], "A: the registered spoke");
                assert_eq!(f.spatial[t].b[0][e], b(e).nearest(), "B: the spoke and what leaves the locus it enters");
                let mut xi = Expansion::new();
                for g in succ(e) { xi = xi.plus(&b(g)); }
                for p in pred(e) { xi = xi.minus(&b(p)); }
                assert_eq!(f.spatial[t].passage[0][e], xi.nearest(), "Ξ: what follows less what precedes");
            }
            for v in 0..rel.n_nodes {
                let mut l = Expansion::new();
                let mut n = Expansion::new();
                for e in 0..m { if rel.edges[e].0 == v { l = l.plus(&x(e)); } if rel.edges[e].1 == v { n = n.plus(&x(e)); } }
                assert_eq!(f.spatial[t].leaving[0][v], l.nearest(), "the readback, leaving: the self-spoke leaves its locus");
                assert_eq!(f.spatial[t].entering[0][v], n.nearest(), "the readback, entering: and enters it");
            }
        }
    });
}
