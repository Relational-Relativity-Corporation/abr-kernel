//! Kernel V10.3: exact sums (Origin, 2026-11-23, 41st: "we are decomposing
//! complex systems but we're not simplifying them").
//!
//! A closure, an accumulation B or a circulation Ξ is a decomposition of the
//! field into a sum of registered entries. Each is checked here against an
//! exact integer reference: every value is dyadic, scaled by 2¹² to an i128,
//! where every sum is exact. Values span 2⁰ to 2⁵⁴, so f64 addition in
//! sequence would lose the small entries beside the large ones.
//!
//! [IMPL] SYNTHETIC FIXTURES. They establish that the sums compute as stated,
//! nothing empirical.

#[path = "support/synthetic_feedback.rs"]
mod synthetic_feedback;
#[path = "support/synthetic_identity.rs"]
mod synthetic_identity;
use abr_kernel::exact::*;
use abr_kernel::operators::{circulation_of_accumulation, circulation_of_accumulation_half_width, DeclaredRelations, EdgeProvenance};
use abr_kernel::persistence_change::*;
use abr_kernel::topology::cycle_basis;
use std::cmp::Ordering;
use std::sync::Mutex;

fn rel(n: usize, edges: Vec<(usize, usize)>) -> DeclaredRelations {
    let m = edges.len();
    synthetic_feedback::declare_loops(synthetic_identity::identify(
        DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m])))
}

const CASES: u64 = 300;
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
    /// A dyadic value m·2^p/8, with p from {0, 20, 40, 54}: mixed magnitudes.
    fn wide(&mut self) -> f64 { let p = [0, 20, 40, 54][self.usize(0, 3)]; self.int(-8, 8) as f64 * 2f64.powi(p) / 8.0 }
}
fn for_all(name: &str, prop: impl Fn(&mut Seeds)) {
    for seed in 0..CASES {
        let mut r = Seeds(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ 0xE5);
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| prop(&mut r)));
        if caught.is_err() { panic!("property {name} failed at seed {seed}"); }
    }
}

// ── The exact reference ───────────────────────────────────────────────────

const SCALE: f64 = 4096.0; // 2¹²
fn to_i(x: f64) -> i128 { let y = x * SCALE; assert_eq!(y.fract(), 0.0, "fixture not on the 2⁻¹² grid"); y as i128 }
/// The f64 nearest to N/2¹², ties to even (i128 → f64 rounds to nearest).
fn nearest_of(n: i128) -> f64 { n as f64 / SCALE }
/// Compare an f64 with N/2¹² exactly.
fn cmp(x: f64, n: i128) -> Ordering {
    let y = x * SCALE;
    if y.fract() == 0.0 { (y as i128).cmp(&n) } else if (y.floor() as i128) < n { Ordering::Less } else { Ordering::Greater }
}

#[test]
fn prop_exact_sums_round_once_and_bounds_round_up() {
    for_all("exact sums", |r| {
        let xs: Vec<f64> = (0..r.usize(1, 12)).map(|_| r.wide()).collect();
        let n: i128 = xs.iter().map(|&x| to_i(x)).sum();
        assert_eq!(sum_nearest(xs.clone()), nearest_of(n));
        // Every order gives the same sum.
        let mut rev = xs.clone(); rev.reverse();
        assert_eq!(sum_nearest(rev), nearest_of(n));
        // `up`: at or above the exact sum, and the smallest such f64.
        let u = sum_up(xs.clone());
        assert_ne!(cmp(u, n), Ordering::Less);
        assert_eq!(cmp(u.next_down(), n), Ordering::Less);
        // Integer combinations.
        let ks: Vec<i64> = xs.iter().map(|_| r.int(-3, 3)).collect();
        let nk: i128 = ks.iter().zip(&xs).map(|(&k, &x)| k as i128 * to_i(x)).sum();
        assert_eq!(combination_nearest(ks.iter().copied().zip(xs.iter().copied())), nearest_of(nk));
    });
}

#[test]
fn sequential_addition_loses_what_the_exact_sum_keeps() {
    let big = 2f64.powi(54);
    assert_eq!(big + -0.5 - big, 0.0, "f64 in sequence drops the −0.5");
    assert_eq!(sum_nearest([big, -0.5, -big]), -0.5);
    assert_eq!(sum_nearest([-0.5, big, -big]), -0.5);
}

// ── Closures: the same from every starting locus ──────────────────────────

fn triangle() -> DeclaredRelations { rel(3, vec![(0, 1), (1, 2), (0, 2)]) }

#[test]
fn k7_a_closure_is_the_same_from_every_starting_locus() {
    let r = triangle();
    let big = 2f64.powi(54);
    let s = EdgeSeries::from_edge_values_synthetic(vec![vec![vec![big, -0.5, big]]; 2], vec![vec![vec![0.0; 3]]; 2]).unwrap();
    // The cycle 0 → 1 → 2 ← 0 read from loci 0, 1 and 2. Before 41st these
    // read 0, 0 and −0.5.
    for w in [vec![(0, 1), (1, 1), (2, -1)], vec![(1, 1), (2, -1), (0, 1)], vec![(2, -1), (0, 1), (1, 1)]] {
        assert_eq!(cycle_closure(&s, &r, &w, 0, 0).unwrap(), (-0.5, 0.0), "from walk {w:?}");
    }
}

fn enumerated_rel(r: &mut Seeds) -> DeclaredRelations {
    let n = r.usize(3, 5);
    let m = r.usize(n, 7.min(n * (n - 1)));
    let mut edges: Vec<(usize, usize)> = Vec::new();
    while edges.len() < m {
        let (s, t) = (r.usize(0, n - 1), r.usize(0, n - 1));
        if s != t && !edges.contains(&(s, t)) { edges.push((s, t)); }
    }
    rel(n, edges)
}

fn forest(rel: &DeclaredRelations) -> Vec<usize> {
    let mut part: Vec<usize> = (0..rel.n_nodes).collect();
    fn root(p: &[usize], v: usize) -> usize { let mut v = v; while p[v] != v { v = p[v]; } v }
    let mut tree = Vec::new();
    for (e, &(s, t)) in rel.edges.iter().enumerate() {
        let (a, b) = (root(&part, s), root(&part, t));
        if a != b { part[a] = b; tree.push(e); }
    }
    tree
}

/// Over enumerated fields with mixed magnitudes and declared gains: every
/// rotation of every basis cycle reads the same value and bound, in state
/// and on change; the value is the exact sum rounded once; the bound is at or
/// above the exact bound and the smallest f64 that is; and a resolution is
/// read only where it holds exactly.
#[test]
fn prop_closures_are_exact_and_order_free() {
    let (lossy, resolved) = (Mutex::new(0usize), Mutex::new(0usize));
    for_all("exact closures", |r| {
        let rel = enumerated_rel(r);
        let basis = cycle_basis(&rel, &forest(&rel)).unwrap();
        let m = rel.n_edges();
        let a: Vec<Vec<Vec<f64>>> = (0..3).map(|_| vec![(0..m).map(|_| r.wide()).collect()]).collect();
        let d: Vec<Vec<Vec<f64>>> = (0..3).map(|_| vec![(0..m).map(|_| r.int(0, 4) as f64 / 16.0).collect()]).collect();
        let ge: Vec<f64> = (0..m).map(|_| r.int(0, 2) as f64 / 8.0).collect();
        let gi: Vec<f64> = (0..m).map(|_| r.int(0, 2) as f64 / 8.0).collect();
        let s = EdgeSeries::from_edge_values_synthetic(a.clone(), d.clone()).unwrap()
            .with_run_constant(vec![vec![0.0; m]], EdgeGains { component: vec![0.125], edge: ge.clone(), independent: vec![gi.clone()] });
        // The gain that differs between edges, (1 + γ_e)(1 + γ_ce) − 1, exactly on the 2⁻¹² grid.
        let g: Vec<i128> = (0..m).map(|e| to_i(ge[e]) + to_i(gi[e]) + to_i(ge[e]) * to_i(gi[e]) / 4096).collect();
        // Exact term bound h + γ(|v| + h), scaled by 2¹²: γ and h, v are scaled, so divide once.
        let term = |v: i128, h: i128, g: i128| h + g * (v.abs() + h) / 4096;
        for cy in &basis.cycles {
            let rot: Vec<Vec<(usize, i8)>> = (0..cy.len()).map(|i| cy[i..].iter().chain(&cy[..i]).copied().collect()).collect();
            for t in 0..3 {
                let v: i128 = cy.iter().map(|&(e, sg)| sg as i128 * to_i(a[t][0][e])).sum();
                let h: i128 = cy.iter().map(|&(e, _)| term(to_i(a[t][0][e]), to_i(d[t][0][e]), g[e])).sum();
                let read = cycle_closure(&s, &rel, cy, t, 0).unwrap();
                assert_eq!(read.0, nearest_of(v));
                assert_ne!(cmp(read.1, h), Ordering::Less);
                assert_eq!(cmp(read.1.next_down(), h), Ordering::Less);
                for w in &rot { assert_eq!(cycle_closure(&s, &rel, w, t, 0).unwrap(), read); }
                if read.0.abs() > read.1 { *resolved.lock().unwrap() += 1; assert!(v.abs() > h); }
                // Would sequential f64 addition have read this closure differently?
                let seq = cy.iter().fold(0.0, |acc, &(e, sg)| acc + sg as f64 * a[t][0][e]);
                if seq != read.0 { *lossy.lock().unwrap() += 1; }
            }
            for st in 0..2 {
                let dv = |e: usize| to_i(a[st + 1][0][e]) - to_i(a[st][0][e]);
                let v: i128 = cy.iter().map(|&(e, sg)| sg as i128 * dv(e)).sum();
                let h: i128 = cy.iter().map(|&(e, _)| term(dv(e), to_i(d[st][0][e]) + to_i(d[st + 1][0][e]), g[e])).sum();
                let read = cycle_change_closure(&s, &rel, cy, st, 0).unwrap();
                assert_eq!(read.0, nearest_of(v));
                assert_ne!(cmp(read.1, h), Ordering::Less);
                for w in &rot { assert_eq!(cycle_change_closure(&s, &rel, w, st, 0).unwrap(), read); }
                if read.0.abs() > read.1 { assert!(v.abs() > h); }
            }
        }
    });
    assert!(lossy.into_inner().unwrap() > 0, "sequential addition never differed: the fixture does not test exactness");
    assert!(resolved.into_inner().unwrap() > 0);
}

// ── Ξ(B(A)) from A in one exact sum ───────────────────────────────────────

#[test]
fn prop_circulation_of_accumulation_is_exact() {
    let lossy = Mutex::new(0usize);
    for_all("exact Ξ(B(A))", |r| {
        let rel = enumerated_rel(r);
        let m = rel.n_edges();
        let a: Vec<Vec<f64>> = vec![(0..m).map(|_| r.wide()).collect()];
        let da: Vec<Vec<f64>> = vec![(0..m).map(|_| r.int(0, 4) as f64 / 16.0).collect()];
        let b = |e: usize, x: &[f64]| -> i128 { to_i(x[e]) + rel.succ(e).iter().map(|&f| to_i(x[f])).sum::<i128>() };
        let xi = circulation_of_accumulation(&a, &rel);
        let hw = circulation_of_accumulation_half_width(&da, &rel);
        for e in 0..m {
            let v: i128 = rel.succ(e).iter().map(|&f| b(f, &a[0])).sum::<i128>() - rel.pred(e).iter().map(|&p| b(p, &a[0])).sum::<i128>();
            let h: i128 = rel.succ(e).iter().chain(rel.pred(e)).map(|&f| b(f, &da[0])).sum();
            assert_eq!(xi[0][e], nearest_of(v));
            assert_ne!(cmp(hw[0][e], h), Ordering::Less);
            assert_eq!(cmp(hw[0][e].next_down(), h), Ordering::Less);
            // The earlier composition: B summed in sequence, then Ξ of the rounded B.
            let bs = |f: usize| rel.succ(f).iter().fold(a[0][f], |acc, &h| acc + a[0][h]);
            let old = rel.succ(e).iter().map(|&f| bs(f)).sum::<f64>() - rel.pred(e).iter().map(|&p| bs(p)).sum::<f64>();
            if old != xi[0][e] { *lossy.lock().unwrap() += 1; }
        }
    });
    assert!(lossy.into_inner().unwrap() > 0, "the earlier composition never differed: the fixture does not test exactness");
}
