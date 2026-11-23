//! Kernel V10.3 — I-SC-D14: the held minor pattern and the sustained
//! circulation are independent grades (closes OC-SC-4).
//!
//! Worked examples realize all four combinations (held/held, held/
//! reorganizing, reorganizing/held, reorganizing/reorganizing) through the
//! kernel's own grade code: `minor_histories` for the pattern and the
//! sustained grade of `ground_reading`. A property test checks the two laws
//! the construction rests on.
//!
//! [IMPL] SYNTHETIC FIXTURES: values are chosen to exercise the result. They
//! establish how the forms relate, nothing empirical. Values are dyadic, so
//! every check is exact.

#[path = "support/synthetic_feedback.rs"]
mod synthetic_feedback;
#[path = "support/synthetic_identity.rs"]
mod synthetic_identity;
use abr_kernel::ground_variables::*;
use abr_kernel::operators::{operator_b, operator_circulation, DeclaredRelations, EdgeField, EdgeProvenance, NodeField};

fn rel(n: usize, edges: Vec<(usize, usize)>) -> DeclaredRelations {
    let m = edges.len();
    synthetic_identity::identify(DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m]))
}

/// The path 0 → 1 → 2 → 3 → 4: a tree, so any edge values are realized by
/// node values (x[t] = x[s] − A[e]); edges 1 and 2 are interior (T7).
fn path() -> DeclaredRelations { rel(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]) }

/// A registration on the path whose A equals `a[c][e]`, δ = 1/32 per locus.
fn reg_from_edges(a: &[Vec<f64>]) -> Registration {
    let x: Vec<Vec<f64>> = a.iter().map(|c| {
        let mut x = vec![0.0];
        for v in c { let last = *x.last().unwrap(); x.push(last - v); }
        x
    }).collect();
    let (k, n) = (x.len(), x[0].len());
    Registration::synthetic(NodeField::new(x), vec![vec![1.0 / 32.0; n]; k])
}

/// The base field F: A_1 = (1, 2, 3, 4), A_2 = (2, 1, 4, 3). Every minor is
/// non-zero, and on the path Ξ(B(A_1)) = (5, 4, −1, −7) and
/// Ξ(B(A_2)) = (5, 4, −2, −7): the two components' circulations agree in sign
/// at every edge.
fn base() -> Vec<Vec<f64>> { vec![vec![1.0, 2.0, 3.0, 4.0], vec![2.0, 1.0, 4.0, 3.0]] }

fn xi(a: &[Vec<f64>], r: &DeclaredRelations) -> Vec<Vec<f64>> {
    let b = operator_b(&EdgeField { spatial: a.to_vec(), comp: vec![], comp_pairs: vec![], k: a.len() }, r).spatial;
    operator_circulation(&b, r)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Grade { Held, Reorganizing }

/// The two grades of a sequence, as the kernel reads them. Pattern: held when
/// no minor's history reorganizes. Sustained: held when no edge is in
/// `reorganizing_edges`. Each "reorganizing" case below has crossings, and
/// each "held" case has every reading held, so the result does not depend on
/// the grade's summary rule.
fn grades(seq: &[Registration], r: &DeclaredRelations) -> (Grade, Grade) {
    let h = minor_histories(seq, r).unwrap();
    let pattern = if h.minors.iter().any(|m| matches!(m.4, History::Reorganizing { .. })) {
        Grade::Reorganizing } else { Grade::Held };
    let s = ground_reading(seq, r, None).connection.unwrap().sustained.unwrap();
    let sustained = if s.reorganizing_edges.is_empty() { Grade::Held } else { Grade::Reorganizing };
    (pattern, sustained)
}

fn crossings(seq: &[Registration], r: &DeclaredRelations) -> usize {
    minor_histories(seq, r).unwrap().minors.iter()
        .filter(|m| m.4 == History::Reorganizing { crossed: true }).count()
}

#[test]
fn isc_d14_base_field_is_as_stated() {
    let r = path();
    assert_eq!(xi(&base(), &r), vec![vec![5.0, 4.0, -1.0, -7.0], vec![5.0, 4.0, -2.0, -7.0]]);
    let a = base();
    for e in 0..4 { for f in (e + 1)..4 { assert_ne!(minor(&a, e, f, 0, 1), 0.0); } }
    // Every minor and every circulation is resolved at δ = 1/32.
    let p = minor_pattern(&reg_from_edges(&a), &r).unwrap();
    assert!(p.minors.iter().all(|m| m.4.is_resolved()));
}

/// Held / held: the field persists.
#[test]
fn isc_d14_held_pattern_held_sustained() {
    let r = path();
    let seq = vec![reg_from_edges(&base()), reg_from_edges(&base()), reg_from_edges(&base())];
    assert_eq!(grades(&seq, &r), (Grade::Held, Grade::Held));
}

/// Held / reorganizing: a per-edge positive scaling, λ = (8, 1, 1, 1). Every
/// minor m(e, f) scales by λ_e·λ_f > 0 and keeps its sign; Ξ at edge 1 is
/// (g_2 + g_3) − (g_0 + g_1), which the larger g_0 reverses in both components.
#[test]
fn isc_d14_held_pattern_reorganizing_sustained() {
    let r = path();
    let scaled: Vec<Vec<f64>> = base().iter().map(|c| {
        c.iter().enumerate().map(|(e, v)| if e == 0 { 8.0 * v } else { *v }).collect()
    }).collect();
    assert_eq!(xi(&scaled, &r)[0][1], -3.0);
    assert_eq!(xi(&scaled, &r)[1][1], -10.0);
    let seq = vec![reg_from_edges(&base()), reg_from_edges(&scaled)];
    assert_eq!(grades(&seq, &r), (Grade::Held, Grade::Reorganizing));
    let s = ground_reading(&seq, &r, None).connection.unwrap().sustained.unwrap();
    assert_eq!(s.reorganizing_edges, vec![1]);
    assert_eq!(s.held_edges, vec![0, 2, 3]);
    assert!(minor_histories(&seq, &r).unwrap().minors.iter().all(|m| matches!(m.4, History::HeldResolved(_))));
}

/// Reorganizing / held: swap the two components. Every minor scales by
/// det = −1 and crosses; Ξ is swapped, and since the components' circulations
/// agree in sign at every edge, every circulation sign is held.
#[test]
fn isc_d14_reorganizing_pattern_held_sustained() {
    let r = path();
    let a = base();
    let swapped = vec![a[1].clone(), a[0].clone()];
    let seq = vec![reg_from_edges(&a), reg_from_edges(&swapped), reg_from_edges(&a)];
    assert_eq!(grades(&seq, &r), (Grade::Reorganizing, Grade::Held));
    assert_eq!(crossings(&seq, &r), 6);
    let s = ground_reading(&seq, &r, None).connection.unwrap().sustained.unwrap();
    assert_eq!(s.held_edges, vec![0, 1, 2, 3]);
}

/// Reorganizing / reorganizing: minus the swap. det = −1 again, so every minor
/// crosses; Ξ maps to −(swapped Ξ), so every resolved circulation crosses.
#[test]
fn isc_d14_reorganizing_pattern_reorganizing_sustained() {
    let r = path();
    let a = base();
    let neg_swap: Vec<Vec<f64>> = vec![a[1].iter().map(|v| -v).collect(), a[0].iter().map(|v| -v).collect()];
    let seq = vec![reg_from_edges(&a), reg_from_edges(&neg_swap)];
    assert_eq!(grades(&seq, &r), (Grade::Reorganizing, Grade::Reorganizing));
    let s = ground_reading(&seq, &r, None).connection.unwrap().sustained.unwrap();
    assert_eq!(s.reorganizing_edges, vec![0, 1, 2, 3]);
}

/// Neither grade implies the other: all four combinations occur on one
/// declaration with the same δ.
#[test]
fn isc_d14_all_four_combinations_occur() {
    let r = path();
    let a = base();
    let scaled: Vec<Vec<f64>> = a.iter().map(|c| c.iter().enumerate().map(|(e, v)| if e == 0 { 8.0 * v } else { *v }).collect()).collect();
    let swapped = vec![a[1].clone(), a[0].clone()];
    let neg_swap: Vec<Vec<f64>> = swapped.iter().map(|c| c.iter().map(|v| -v).collect()).collect();
    let mut seen: Vec<(Grade, Grade)> = [&a, &scaled, &swapped, &neg_swap].iter()
        .map(|b| grades(&[reg_from_edges(&a), reg_from_edges(b)], &r)).collect();
    seen.sort_by_key(|g| format!("{g:?}"));
    seen.dedup();
    assert_eq!(seen.len(), 4);
}

// ═══════════════════════════════════════════════════════════════════════════
// The two laws, over enumerated fields (exact)
// ═══════════════════════════════════════════════════════════════════════════

struct Seeds(u64);
impl Seeds {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn int(&mut self, lo: i64, hi: i64) -> i64 { lo + (self.next() % (hi - lo + 1) as u64) as i64 }
    fn usize(&mut self, lo: usize, hi: usize) -> usize { self.int(lo as i64, hi as i64) as usize }
    fn eighth(&mut self) -> f64 { self.int(-32, 32) as f64 / 8.0 }
}

fn enumerated_rel(r: &mut Seeds) -> DeclaredRelations {
    let n = r.usize(3, 6);
    let m = r.usize(2, 8.min(n * (n - 1)));
    let mut edges: Vec<(usize, usize)> = Vec::new();
    while edges.len() < m {
        let (s, t) = (r.usize(0, n - 1), r.usize(0, n - 1));
        if s != t && !edges.contains(&(s, t)) { edges.push((s, t)); }
    }
    synthetic_feedback::declare_loops(rel(n, edges))
}

/// (i) A per-edge scaling λ_e multiplies m_ij(e, f) by λ_e·λ_f; (ii) mixing
/// the components by a 2×2 matrix R multiplies every minor by det R and maps
/// Ξ(B(A)) to R·Ξ(B(A)). Both exact on dyadic values.
#[test]
fn prop_isc_d14_scaling_and_mixing_laws() {
    for seed in 0..2000u64 {
        let mut r = Seeds(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ 0x3C3C);
        let rl = enumerated_rel(&mut r);
        let m = rl.n_edges();
        let a: Vec<Vec<f64>> = (0..2).map(|_| (0..m).map(|_| r.eighth()).collect()).collect();
        let lam: Vec<f64> = (0..m).map(|_| [0.25, 0.5, 1.0, 2.0, 4.0][r.usize(0, 4)]).collect();
        let scaled: Vec<Vec<f64>> = a.iter().map(|c| c.iter().zip(&lam).map(|(v, l)| v * l).collect()).collect();
        let rm: [[f64; 2]; 2] = [[r.int(-2, 2) as f64, r.int(-2, 2) as f64], [r.int(-2, 2) as f64, r.int(-2, 2) as f64]];
        let det = rm[0][0] * rm[1][1] - rm[0][1] * rm[1][0];
        let mixed: Vec<Vec<f64>> = (0..2).map(|i| (0..m).map(|e| rm[i][0] * a[0][e] + rm[i][1] * a[1][e]).collect()).collect();
        for e in 0..m { for f in 0..m {
            assert_eq!(minor(&scaled, e, f, 0, 1), lam[e] * lam[f] * minor(&a, e, f, 0, 1), "seed {seed}");
            assert_eq!(minor(&mixed, e, f, 0, 1), det * minor(&a, e, f, 0, 1), "seed {seed}");
        } }
        let (x, xm) = (xi(&a, &rl), xi(&mixed, &rl));
        for i in 0..2 { for e in 0..m {
            assert_eq!(xm[i][e], rm[i][0] * x[0][e] + rm[i][1] * x[1][e], "seed {seed}");
        } }
    }
}
