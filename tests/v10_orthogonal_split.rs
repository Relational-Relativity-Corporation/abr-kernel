//! The orthogonal split of a field of relations into its two parts (Origin,
//! 2026-11-26, 100th; the 99th, Proposed): the potential part, the image of
//! A, and the balanced part, where what enters every locus equals what
//! leaves, orthogonal under the declared inner product (equal weight on every
//! relation). Legacy mathematics (92nd): Kirchhoff (1847); the discrete Hodge
//! decomposition (Eckmann, 1944); computed exactly in rationals.
//!
//! Each fixture is a whole relational field formed from relations registered
//! directly, with the carrier declared (`whole_of_relations`), or at the loci
//! (`whole`); the split is read from the formed field (`two_parts`), with
//! the inner product declared. Every identity is checked exactly, in
//! rationals, with no tolerance.
//!
//! **Not read by these tests (named):** whether energy, connection and
//! freedom are best read through the split; Origin (100th): *"if our
//! construct reveals observable behaviors that will be our validation"*. That
//! is for labelled confirmations, not synthetic tests.
//!
//! [IMPL] SYNTHETIC FIXTURES, enumerated from declared seeds (W17). They
//! establish that the split computes as stated, nothing empirical.

#[path = "support/synthetic_whole_field.rs"]
mod support;

use std::cell::RefCell;

use abr_kernel::operators::DeclaredRelations;
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::relational_field::{InnerProduct, RelationalField, TwoParts};
use abr_kernel::exact::Ratio;
use abr_kernel::topology::turn_basis;
use support::*;

const K: usize = 2;

/// Relations registered directly with enumerated dyadic values in −3 … 3.
fn registered(r: &mut Seeds, m: usize, regs: usize, d: f64) -> EdgeSeries {
    let a = (0..regs).map(|_| (0..K).map(|_| (0..m).map(|_| r.eighth(-3.0, 3.0)).collect()).collect()).collect();
    EdgeSeries::from_edge_values_synthetic(a, vec![vec![vec![d; m]; K]; regs]).unwrap()
}

fn carrier(m: usize, regs: usize) -> abr_kernel::ground_variables::RelationCarrier {
    relation_carrier((0..regs).map(|t| (0..m).map(|e| ((e + t) % 4) as f64 + 1.0).collect()).collect(), vec![vec![0.125; m]; regs])
}

/// A whole field over `rel` from enumerated registered relations, and its split.
fn split_field(r: &mut Seeds, rel: &DeclaredRelations, regs: usize, d: f64) -> (RelationalField, TwoParts) {
    let f = whole_of_relations(rel, registered(r, rel.n_edges(), regs, d), &carrier(rel.n_edges(), regs));
    let s = f.two_parts(&InnerProduct::equal());
    (f, s)
}

fn g_exact(f: &RelationalField, t: usize, c: usize, e: usize) -> Ratio { Ratio::from_f64(f.spatial[t].a[c][e]) }

/// 1. The two parts account for every relation exactly: potential + balanced
/// = the relation, at every registration and step, component and relation.
#[test]
fn the_two_parts_sum_to_the_relation_exactly() {
    for_all("split: sum", 0x100_01, |r| {
        let rel = enumerated_rel(r);
        let regs = r.usize(2, 3);
        let (f, s) = split_field(r, &rel, regs, 0.125);
        for (t, x) in s.spatial.iter().enumerate() {
            for c in 0..K { for e in 0..rel.n_edges() {
                assert_eq!(x.exact_potential[c][e].add(&x.exact_balanced[c][e]), g_exact(&f, t, c, e), "t {t} c {c} e {e}");
            } }
        }
        for (st, x) in s.persistence.iter().enumerate() {
            for c in 0..K { for e in 0..rel.n_edges() {
                let change = g_exact(&f, st + 1, c, e).sub(&g_exact(&f, st, c, e));
                assert_eq!(x.exact_potential[c][e].add(&x.exact_balanced[c][e]), change, "step {st}");
            } }
        }
    });
}

/// 2. Each part is what it is declared to be, exactly: the potential part
/// sums to zero along every declared turn (K7), and the balanced part balances at every
/// locus (what enters equals what leaves, through the readback, K9).
#[test]
fn the_potential_part_sums_to_zero_along_every_turn_and_the_balanced_part_balances() {
    for_all("split: parts", 0x100_02, |r| {
        let rel = enumerated_rel(r);
        let (_, s) = split_field(r, &rel, 2, 0.125);
        let basis = turn_basis(&rel, &spanning_forest(&rel)).unwrap();
        for x in s.spatial.iter().chain(&s.persistence) {
            for c in 0..K {
                for cy in &basis.turns {
                    let mut turn_sum = Ratio::zero();
                    for &(e, sg) in cy { turn_sum = if sg > 0 { turn_sum.add(&x.exact_potential[c][e]) } else { turn_sum.sub(&x.exact_potential[c][e]) }; }
                    assert!(turn_sum.is_zero(), "the potential part sums to zero along every turn");
                }
                for v in 0..rel.n_nodes {
                    let mut bal = Ratio::zero();
                    for &e in &rel.out[v] { bal = bal.add(&x.exact_balanced[c][e]); }
                    for &e in &rel.inc[v] { bal = bal.sub(&x.exact_balanced[c][e]); }
                    assert!(bal.is_zero(), "the balanced part balances at locus {v}");
                }
            }
        }
    });
}

/// 3. The two parts are orthogonal under the declared inner product: Σ_e
/// potential[e]·balanced[e] = 0 exactly, in every component.
#[test]
fn the_two_parts_are_orthogonal() {
    for_all("split: orthogonal", 0x100_03, |r| {
        let rel = enumerated_rel(r);
        let (_, s) = split_field(r, &rel, 2, 0.125);
        for x in s.spatial.iter().chain(&s.persistence) {
            for c in 0..K {
                let mut dot = Ratio::zero();
                for e in 0..rel.n_edges() { dot = dot.add(&x.exact_potential[c][e].mul(&x.exact_balanced[c][e])); }
                assert!(dot.is_zero(), "orthogonal");
            }
        }
    });
}

/// 4. The split does not depend on the order the relations are declared in
/// (no choice of basis or tree enters, 41st): declared in reverse order, each
/// relation receives the same two parts. The check can tell: a split along a
/// stated tree changes with the tree on some fields.
#[test]
fn the_split_depends_on_no_choice_of_basis() {
    let tree_dependent = RefCell::new(0usize);
    for_all("split: basis", 0x100_04, |r| {
        let rel = enumerated_rel(r);
        let m = rel.n_edges();
        let series = registered(r, m, 2, 0.125);
        let f = whole_of_relations(&rel, series.clone(), &carrier(m, 2));
        let s = f.two_parts(&InnerProduct::equal());
        let rev_edges: Vec<(usize, usize)> = rel.edges.iter().rev().cloned().collect();
        let rev = support::rel(rel.n_nodes, rev_edges);
        let rev_a = series.a.iter().map(|t| t.iter().map(|c| c.iter().rev().cloned().collect()).collect()).collect();
        let rev_series = EdgeSeries::from_edge_values_synthetic(rev_a, series.da.clone()).unwrap();
        let rf = whole_of_relations(&rev, rev_series, &carrier(m, 2));
        let rs = rf.two_parts(&InnerProduct::equal());
        for t in 0..2 { for c in 0..K { for e in 0..m {
            assert_eq!(s.spatial[t].exact_potential[c][e], rs.spatial[t].exact_potential[c][m - 1 - e], "relation {e}");
        } } }
        // A split along a stated tree: the balanced part carried by the chords.
        let tree_split = |tree: &[usize]| -> Vec<Ratio> {
            let b = turn_basis(&rel, tree).unwrap();
            let mut q = vec![Ratio::zero(); m];
            for (k, cy) in b.turns.iter().enumerate() {
                let z = g_exact(&f, 0, 0, b.chords[k]);
                for &(e, sg) in cy { q[e] = if sg > 0 { q[e].add(&z) } else { q[e].sub(&z) }; }
            }
            q
        };
        let forward = spanning_forest(&rel);
        let backward = spanning_forest_reversed(&rel);
        if tree_split(&forward) != tree_split(&backward) { *tree_dependent.borrow_mut() += 1; }
    });
    assert!(*tree_dependent.borrow() > 0, "the check can tell: a split along a tree depends on the tree");
}

/// 5. The split of a change is the change of the split, exactly.
#[test]
fn the_split_of_a_change_is_the_change_of_the_split() {
    for_all("split: change", 0x100_05, |r| {
        let rel = enumerated_rel(r);
        let (_, s) = split_field(r, &rel, 3, 0.125);
        for st in 0..s.persistence.len() {
            for c in 0..K { for e in 0..rel.n_edges() {
                assert_eq!(s.persistence[st].exact_potential[c][e],
                    s.spatial[st + 1].exact_potential[c][e].sub(&s.spatial[st].exact_potential[c][e]), "step {st}");
            } }
        }
    });
}

/// 6. A field formed from the loci is all potential (its balanced part is
/// 0); a field registered as a combination of declared turns is all
/// balanced (its potential part is 0).
#[test]
fn each_kind_of_field_falls_wholly_in_its_part() {
    let balanced_seen = RefCell::new(0usize);
    for_all("split: kinds", 0x100_06, |r| {
        let rel = enumerated_rel(r);
        let w = whole(&rel, enumerated_declared(r, &rel, K, 2, 0.5, false, false));
        let s = w.field.two_parts(&InnerProduct::equal());
        for x in &s.spatial { for c in 0..K { for e in 0..rel.n_edges() { assert!(x.exact_balanced[c][e].is_zero(), "locus-formed: no balanced part"); } } }
        let b = turn_basis(&rel, &spanning_forest(&rel)).unwrap();
        if b.turns.is_empty() { return; }
        let m = rel.n_edges();
        let a: Vec<Vec<Vec<f64>>> = (0..2).map(|_| (0..K).map(|_| {
            let mut g = vec![0.0; m];
            for cy in &b.turns { let z = r.int(-3, 3) as f64; for &(e, sg) in cy { g[e] += z * sg as f64; } }
            g
        }).collect()).collect();
        let f = whole_of_relations(&rel, EdgeSeries::from_edge_values_synthetic(a, vec![vec![vec![0.125; m]; K]; 2]).unwrap(), &carrier(m, 2));
        let s = f.two_parts(&InnerProduct::equal());
        for x in &s.spatial { for c in 0..K { for e in 0..m { assert!(x.exact_potential[c][e].is_zero(), "turn combination: no potential part"); } } }
        *balanced_seen.borrow_mut() += 1;
    });
    assert!(*balanced_seen.borrow() > 0);
}

/// 7. Every part carries its bound (98th): for every sign pattern of the
/// registered bounds (each relation at g ± δ), the change in each part is
/// within its reported bound, and some pattern reaches it (the bound is the
/// split with its signs dropped, applied to δ, 93rd). With δ = 0 every bound
/// is 0. Enumerated over the first 40 seeds of the family, not the usual 300:
/// every sign pattern forms its own whole field (up to 2⁷ per seed).
#[test]
fn every_part_carries_its_bound() {
    for seed in 0..40u64 {
        let r = &mut Seeds(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ 0x100_07);
        let rel = enumerated_rel(r);
        let m = rel.n_edges();
        let d = 0.125;
        let base = registered(r, m, 2, d);
        let f = whole_of_relations(&rel, base.clone(), &carrier(m, 2));
        let s = f.two_parts(&InnerProduct::equal());
        let (t, c) = (0usize, 0usize);
        let mut reached = vec![false; m];
        for pattern in 0..(1u32 << m) {
            let mut a = base.a.clone();
            for e in 0..m { a[t][c][e] += if pattern >> e & 1 == 1 { d } else { -d }; }
            let g = whole_of_relations(&rel, EdgeSeries::from_edge_values_synthetic(a, base.da.clone()).unwrap(), &carrier(m, 2));
            let gs = g.two_parts(&InnerProduct::equal());
            for e in 0..m {
                let dp = gs.spatial[t].exact_potential[c][e].sub(&s.spatial[t].exact_potential[c][e]).abs();
                let bound = Ratio::from_f64(s.spatial[t].potential_bound[c][e]);
                assert!(dp.le(&bound), "relation {e}: within the bound");
                if Ratio::from_f64(dp.to_f64_up()) == bound || dp == bound { reached[e] = true; }
            }
        }
        assert!(reached.iter().all(|&x| x), "the bound is reached by some pattern at every relation");
        let z = whole_of_relations(&rel, EdgeSeries::from_edge_values_synthetic(base.a.clone(), vec![vec![vec![0.0; m]; K]; 2]).unwrap(), &carrier(m, 2));
        let zs = z.two_parts(&InnerProduct::equal());
        for x in &zs.spatial { for c in 0..K { for e in 0..m {
            assert_eq!(x.potential_bound[c][e], 0.0);
            assert_eq!(x.balanced_bound[c][e], 0.0);
        } } }
    }
}

/// A spanning forest, each relation in declared order kept when it joins two
/// pieces not yet joined.
fn spanning_forest(rel: &DeclaredRelations) -> Vec<usize> { forest_in(rel, (0..rel.n_edges()).collect()) }

/// The same, taking the relations in reverse order.
fn spanning_forest_reversed(rel: &DeclaredRelations) -> Vec<usize> { forest_in(rel, (0..rel.n_edges()).rev().collect()) }

fn forest_in(rel: &DeclaredRelations, order: Vec<usize>) -> Vec<usize> {
    let mut p: Vec<usize> = (0..rel.n_nodes).collect();
    fn root(p: &mut [usize], v: usize) -> usize { let mut v = v; while p[v] != v { v = p[v]; } v }
    let mut tree = Vec::new();
    for e in order {
        let (s, t) = rel.edges[e];
        let (a, b) = (root(&mut p, s), root(&mut p, t));
        if a != b { p[a] = b; tree.push(e); }
    }
    tree
}
