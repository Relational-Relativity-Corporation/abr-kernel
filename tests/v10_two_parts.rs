//! **Withdrawn (Origin, 2026-11-27, 115th):** the 99th's candidate was the
//! Generator's; there is no division between potential and balanced. These
//! tests stay as checks of what the readback and the incidence compute.
//!
//! The two parts of every field of relations (Proposed, Origin, 2026-11-26,
//! 99th): the balanced part (at every locus what enters equals what leaves,
//! read through the readback K9, 96th) and the potential part (explained by
//! values at the loci, closing every turn). Their numbers of independent
//! ways are fixed by the declared relations alone.
//!
//! Each fixture is a whole relational field, formed at the loci with the
//! carrier declared (`whole`), or from relations registered directly
//! (`whole_of_relations`), so every variable is evaluated together; each
//! claim is read from the parts the field reports (`spatial`, `persistence`,
//! `ways`). The turn basis is read from the kernel's declared objects (K7)
//! on a spanning forest stated in the test, independently of how the field
//! counts its ways.
//!
//! **Not read by these tests (named):** splitting a given field into its two
//! parts (it needs a declaration, open for Origin, 99th, step 3); energy
//! content (97th); freedom as the range left open by held relations. They are
//! not claimed here.
//!
//! [IMPL] SYNTHETIC FIXTURES, enumerated from declared seeds (W17), dyadic
//! values so every sum is exact. They establish that the structure computes
//! as stated, nothing empirical.

#[path = "support/synthetic_whole_field.rs"]
mod support;

use std::cell::RefCell;

use abr_kernel::operators::DeclaredRelations;
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::relational_field::RelationalField;
use abr_kernel::topology::{basis_turn_sums, turn_basis, TurnBasis};
use support::*;

/// A spanning forest of the declared relations, stated by the test: each
/// relation in order, kept when it joins two pieces not yet joined.
fn forest(rel: &DeclaredRelations) -> Vec<usize> {
    let mut p: Vec<usize> = (0..rel.n_nodes).collect();
    fn root(p: &mut [usize], v: usize) -> usize { let mut v = v; while p[v] != v { v = p[v]; } v }
    let mut tree = Vec::new();
    for (e, &(s, t)) in rel.edges.iter().enumerate() {
        let (a, b) = (root(&mut p, s), root(&mut p, t));
        if a != b { p[a] = b; tree.push(e); }
    }
    tree
}

fn basis(rel: &DeclaredRelations) -> TurnBasis { turn_basis(rel, &forest(rel)).expect("a stated spanning forest") }

/// 1. The count of ways: the potential part has n − c independent ways (the
/// relations of a spanning forest), the balanced part m − n + c (the
/// independent turns), and together they account for every relation.
#[test]
fn the_ways_of_the_two_parts_account_for_every_relation() {
    for_all("two parts: ways", 0x99_01, |r| {
        let rel = enumerated_rel(r);
        let k = r.usize(2, 3);
        let regs = r.usize(2, 4);
        let w = whole(&rel, enumerated_declared(r, &rel, k, regs, 0.5, false, false));
        let b = basis(&rel);
        let ways = &w.field.ways;
        assert_eq!(ways.relations, rel.n_edges());
        assert_eq!(ways.loci, rel.n_nodes);
        assert_eq!(ways.potential, b.tree.len(), "potential ways are the forest's relations");
        assert_eq!(ways.balanced, b.turns.len(), "balanced ways are the independent turns");
        assert_eq!(ways.potential + ways.balanced, ways.relations, "together, every relation");
        assert_eq!(ways.potential, ways.loci - ways.pieces);
    });
}

/// Relations registered directly as integer combinations of the declared
/// turns: each turn's relations carry its coefficient, with the sign the
/// turn travels them.
fn balanced_series(r: &mut Seeds, rel: &DeclaredRelations, b: &TurnBasis, k: usize, regs: usize) -> EdgeSeries {
    let m = rel.n_edges();
    let a: Vec<Vec<Vec<f64>>> = (0..regs).map(|_| (0..k).map(|_| {
        let mut g = vec![0.0; m];
        for cy in &b.turns {
            let z = r.int(-3, 3) as f64;
            for &(e, sg) in cy { g[e] += z * sg as f64; }
        }
        g
    }).collect()).collect();
    EdgeSeries::from_edge_values_synthetic(a, vec![vec![vec![0.125; m]; k]; regs]).unwrap()
}

fn carrier_for(m: usize, regs: usize) -> abr_kernel::ground_variables::RelationCarrier {
    relation_carrier((0..regs).map(|t| (0..m).map(|e| ((e + t) % 4) as f64 + 1.0).collect()).collect(), vec![vec![0.125; m]; regs])
}

/// 2. Membership. (a) A field formed from the loci lies in the potential
/// part: every declared turn sums to 0 exactly. (b) A field registered as
/// a combination of the declared turns lies in the balanced part: at every
/// locus, in every component and registration, what enters equals what
/// leaves. The check can tell: some locus-formed field is not balanced.
#[test]
fn each_part_is_what_it_is_declared_to_be() {
    let (unbalanced_seen, balanced_formed) = (RefCell::new(0usize), RefCell::new(0usize));
    for_all("two parts: membership", 0x99_02, |r| {
        let rel = enumerated_rel(r);
        let k = r.usize(2, 3);
        let regs = r.usize(2, 4);
        let b = basis(&rel);
        // (a) the potential part
        let w = whole(&rel, enumerated_declared(r, &rel, k, regs, 0.5, false, false));
        for t in 0..regs { for c in 0..k {
            for (v, _) in basis_turn_sums(&w.field.relations, &rel, &b, t, c) { assert_eq!(v, 0.0, "a locus-formed field closes every turn"); }
        } }
        let f = &w.field.spatial;
        if f.iter().any(|x| x.leaving != x.entering) { *unbalanced_seen.borrow_mut() += 1; }
        // (b) the balanced part
        if b.turns.is_empty() { return; }
        let m = rel.n_edges();
        let field: RelationalField = whole_of_relations(&rel, balanced_series(r, &rel, &b, k, regs), &carrier_for(m, regs));
        for (t, x) in field.spatial.iter().enumerate() {
            assert_eq!(x.leaving, x.entering, "registration {t}: what enters each locus equals what leaves");
        }
        for (s, x) in field.persistence.iter().enumerate() {
            assert_eq!(x.leaving, x.entering, "step {s}: the change is balanced too");
        }
        *balanced_formed.borrow_mut() += 1;
    });
    assert!(*unbalanced_seen.borrow() > 0, "the check can tell: some locus-formed fields are not balanced");
    assert!(*balanced_formed.borrow() > 0, "balanced fields were formed");
}

/// 3. Change splits the same way: the readback of each step's change equals
/// the change of the readback, exactly, at every locus, component and step,
/// in what leaves and in what enters.
#[test]
fn the_readback_of_a_change_is_the_change_of_the_readback() {
    for_all("two parts: change", 0x99_03, |r| {
        let rel = enumerated_rel(r);
        let k = r.usize(2, 3);
        let regs = r.usize(2, 4);
        let w = whole(&rel, enumerated_declared(r, &rel, k, regs, 0.5, false, false));
        let (sp, pe) = (&w.field.spatial, &w.field.persistence);
        for s in 0..pe.len() {
            for c in 0..k {
                for v in 0..rel.n_nodes {
                    assert_eq!(pe[s].leaving[c][v], sp[s + 1].leaving[c][v] - sp[s].leaving[c][v], "leaving, step {s}, locus {v}");
                    assert_eq!(pe[s].entering[c][v], sp[s + 1].entering[c][v] - sp[s].entering[c][v], "entering, step {s}, locus {v}");
                }
            }
        }
    });
}

/// 4. The halves are reported with the relations they sum (96th, condition
/// (1)), every direction kept (condition (3)): the relations leaving a locus
/// are exactly those declared to leave it, and those entering, to enter it.
#[test]
fn the_halves_are_reported_with_the_relations_they_sum() {
    for_all("two parts: halves", 0x99_04, |r| {
        let rel = enumerated_rel(r);
        let w = whole(&rel, enumerated_declared(r, &rel, 2, 2, 0.5, false, false));
        let meeting = &w.field.meeting;
        for v in 0..rel.n_nodes {
            let leave: Vec<usize> = (0..rel.n_edges()).filter(|&e| rel.edges[e].0 == v).collect();
            let enter: Vec<usize> = (0..rel.n_edges()).filter(|&e| rel.edges[e].1 == v).collect();
            assert_eq!(meeting.leaving[v], leave, "locus {v}");
            assert_eq!(meeting.entering[v], enter, "locus {v}");
            for c in 0..2 {
                let a = &w.field.spatial[0].a[c];
                assert_eq!(w.field.spatial[0].leaving[c][v], leave.iter().map(|&e| a[e]).sum::<f64>());
                assert_eq!(w.field.spatial[0].entering[c][v], enter.iter().map(|&e| a[e]).sum::<f64>());
            }
        }
    });
}
