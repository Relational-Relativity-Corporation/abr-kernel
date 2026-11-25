//! The work of the operator readings (Origin, 2026-11-25, 92nd): forming A,
//! B and Ξ grows with the relations and their neighbours, never with their
//! square. Comparing every relation with every other is the shape of
//! attention, the sign that a reading is not using the primitive.
//!
//! Each field is a whole relational field formed from relations registered
//! directly (`whole_of_relations`), with the carrier declared, so every
//! variable is evaluated together; the claim is read from the work the
//! formation did (`abr_kernel::work`), counted exactly in terms visited.
//!
//! **Not read by this test (named):** the work of the readings that compare
//! every pair of relations (the identity and proportional grades, the V2 held
//! arrangement), of freedom (PC1–PC3) and of energy. They are formed in the
//! same call, but `work` counts only the operator readings; their derivation
//! and scope are stated in `src/derivation.rs`. The test cannot see work in a
//! loop that does not call `work::terms`.
//!
//! [IMPL] SYNTHETIC FIXTURES: declared families of bounded neighbourhood,
//! enumerated values. They establish how the work grows, nothing empirical.

#[path = "support/synthetic_whole_field.rs"]
mod support;

use abr_kernel::operators::DeclaredRelations;
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::work::operator_terms;
use support::{rel, relation_carrier, whole_of_relations};

const K: usize = 2;
const REGS: usize = 3;

/// A declared chain of `m` relations: 0 → 1 → … → m. Every relation has at
/// most one successor and one predecessor.
fn chain(m: usize) -> DeclaredRelations { rel(m + 1, (0..m).map(|i| (i, i + 1)).collect()) }

/// A declared comb of `m` relations (m even): a spine 0 → 2 → 4 → …, and from
/// each spine node a tooth to its own leaf. At most two successors each.
fn comb(m: usize) -> DeclaredRelations {
    let spine = m / 2;
    let mut edges = Vec::new();
    for i in 0..spine { edges.push((2 * i, 2 * i + 2)); edges.push((2 * i, 2 * i + 1)); }
    rel(2 * spine + 1, edges)
}

/// Enumerated registrations of the relations: value (3e + 5c + 7t) mod 13 − 6,
/// plus ½ so no value is zero; bound ⅛.
fn series(m: usize) -> EdgeSeries {
    let a = (0..REGS).map(|t| (0..K).map(|c| (0..m).map(|e| ((3 * e + 5 * c + 7 * t) % 13) as f64 - 5.5).collect()).collect()).collect();
    EdgeSeries::from_edge_values_synthetic(a, vec![vec![vec![0.125; m]; K]; REGS]).unwrap()
}

/// The operator terms visited in forming the whole field over `rel`.
fn work_of(rel: &DeclaredRelations) -> u64 {
    let m = rel.n_edges();
    let carrier = relation_carrier((0..REGS).map(|t| (0..m).map(|e| ((e + t) % 5) as f64 + 1.0).collect()).collect(), vec![vec![0.125; m]; REGS]);
    let (field, w) = operator_terms(|| whole_of_relations(rel, series(m), &carrier));
    assert_eq!(field.spatial.len(), REGS);
    assert!(w > 0, "the formation visits operator terms");
    w
}

/// The work at m, 2m and 4m is affine in m exactly: the second difference of
/// the work over doubling is zero. Work growing with the square of m fails.
fn affine_over_doubling(family: &dyn Fn(usize) -> DeclaredRelations, m: usize) -> Result<(), String> {
    let (w1, w2, w4) = (work_of(&family(m)), work_of(&family(2 * m)), work_of(&family(4 * m)));
    if w4 - w2 == 2 * (w2 - w1) { Ok(()) } else { Err(format!("m = {m}: work {w1}, {w2}, {w4}; not affine in m")) }
}

/// 1. On a chain, the work of A, B and Ξ is affine in the number of relations.
#[test]
fn operator_work_grows_with_the_relations_on_a_chain() {
    for m in [8, 16, 32] { affine_over_doubling(&chain, m).unwrap(); }
}

/// 2. On a comb (two successors at each spine node), the same.
#[test]
fn operator_work_grows_with_the_relations_on_a_comb() {
    for m in [8, 16, 32] { affine_over_doubling(&comb, m).unwrap(); }
}

/// 3. The work counts neighbours: at the same number of relations, the comb
/// (more successors) visits more terms than the chain. A count blind to the
/// neighbourhood would not tell them apart.
#[test]
fn operator_work_counts_the_neighbours() {
    for m in [8, 16, 32] { assert!(work_of(&comb(m)) > work_of(&chain(m)), "m = {m}"); }
}

/// The check can tell: the affine test refuses work growing with the square.
#[test]
fn the_affine_check_can_tell_square_growth() {
    let square = |m: u64| 3 * m * m + 2 * m + 1;
    let (w1, w2, w4) = (square(8), square(16), square(32));
    assert_ne!(w4 - w2, 2 * (w2 - w1));
    let linear = |m: u64| 7 * m + 3;
    assert_eq!(linear(32) - linear(16), 2 * (linear(16) - linear(8)));
}
