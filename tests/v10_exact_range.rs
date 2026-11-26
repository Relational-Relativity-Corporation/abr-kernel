//! Kernel V10.3: the range of exact arithmetic, and the Verifier's batch-2
//! findings (2026-11-23, 46th).
//!
//! A reading is exact or refused, never silently wrong. Every value, bound,
//! offset and gain is admitted only as 0 or with magnitude in [2⁻²⁰⁰, 2²⁰⁰].
//! Every exact operation refuses a sum that overflows or a product that
//! underflows. Inside the admitted range, no V10 reading is refused.
//!
//! [IMPL] SYNTHETIC FIXTURES. They establish that the forms compute as
//! stated, nothing empirical.

#[path = "support/synthetic_feedback.rs"]
mod synthetic_feedback;
#[path = "support/synthetic_identity.rs"]
mod synthetic_identity;
use abr_kernel::exact::*;
use abr_kernel::ground_variables::*;
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::persistence_change::*;
use abr_kernel::relational_field::RelationalField;
use abr_kernel::topology::cycle_basis;

fn rel(n: usize, edges: Vec<(usize, usize)>) -> DeclaredRelations {
    let m = edges.len();
    synthetic_feedback::declare_loops(synthetic_identity::identify(
        DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m])))
}

// ── Finding 1: a false PC3 resolution from underflowing products ─────────

/// The Verifier's record: rows [m, m] and [0.25, 1], bounds [0, 0] and
/// [0.49, 0.49], m = 2⁻¹⁰⁷⁴. The exact minor is 0.75m against an exact bound
/// of about 0.98m, so the state is not resolved; `a0bf7ec`'s successor read
/// `Resolved(Positive)`. The record is now refused at admission.
#[test]
#[should_panic(expected = "outside the range of exact arithmetic")]
fn finding_1_the_subnormal_record_is_refused() {
    let m = f64::from_bits(1);
    let a = vec![vec![vec![m, m], vec![0.25, 1.0]]; 2];
    let d = vec![vec![vec![0.0, 0.0], vec![0.49, 0.49]]; 2];
    let _ = EdgeSeries::from_edge_values_synthetic(a, d);
}

/// The arithmetic itself refuses the product whose residual underflows,
/// whatever reaches it.
#[test]
#[should_panic(expected = "underflows")]
fn finding_1_an_underflowing_product_is_refused() {
    let mut e = Expansion::new();
    e.add_product(f64::from_bits(1), 0.25);
}

// ── Finding 2: an intermediate overflow ───────────────────────────────────

/// `[MAX, MAX, −MAX]` summed to NaN. It is refused now. Outside the admitted
/// range a sum may be refused in one order and computed in another, but it
/// is never wrong. Inside the range, every order gives the same exact sum
/// (`tests/v10_exact_sums.rs`).
#[test]
#[should_panic(expected = "overflowed")]
fn finding_2_an_overflowing_sum_is_refused() {
    let _ = sum_nearest([f64::MAX, f64::MAX, -f64::MAX]);
}

#[test]
fn finding_2_the_other_order_is_exact() {
    assert_eq!(sum_nearest([f64::MAX, -f64::MAX, f64::MAX]), f64::MAX);
}

// ── Admission ─────────────────────────────────────────────────────────────

#[test]
fn the_admitted_range_is_zero_or_two_to_the_plus_minus_200() {
    for x in [0.0, 2f64.powi(-200), -2f64.powi(-200), 2f64.powi(200), 1.0, -3.5] { assert!(in_exact_range(x), "{x}"); }
    for x in [2f64.powi(-201), 2f64.powi(201), f64::MAX, f64::from_bits(1), f64::INFINITY, f64::NAN] { assert!(!in_exact_range(x), "{x}"); }
}

#[test]
#[should_panic(expected = "outside the range of exact arithmetic")]
fn a_registration_below_the_range_is_refused() {
    let _ = Registration::synthetic(NodeField::new(vec![vec![0.0, 2f64.powi(-201)]]), vec![vec![0.0; 2]]);
}

#[test]
#[should_panic(expected = "outside the range of exact arithmetic")]
fn a_gain_below_the_range_is_refused() {
    let r = Registration::synthetic(NodeField::new(vec![vec![0.0, 1.0]]), vec![vec![0.0; 2]]);
    let _ = r.with_run_constant(vec![vec![0.0; 2]], vec![vec![0.0, 2f64.powi(-210)]], vec![0.0]);
}

#[test]
#[should_panic(expected = "outside the range of exact arithmetic")]
fn a_carrier_above_the_range_is_refused() {
    let c = EnergyCarrier { range: abr_kernel::ground_variables::ObservedRange::synthetic(), name: "SYNTHETIC".into(), unit: "u".into(), values: vec![0.0, 2f64.powi(201)], delta: vec![0.0; 2] };
    let _ = signed_exchanges(&c, &rel(2, vec![(0, 1)]));
}

// ── Finding 3: the sustained grade decides on its exact bound ─────────────

/// The Verifier's case: path 0 → 1 → 2 → 3, x = [0, 1, 0, −2⁻⁵³],
/// δ = [0, 1, 0, 2⁻⁵⁴], repeated. At edge 0, Ξ(B(A)) = 1 + 2⁻⁵³ exactly,
/// above its bound 1 + 2⁻⁵⁴. Rounding that bound up first, to 1 + 2⁻⁵², hid
/// the held circulation.
#[test]
fn finding_3_the_sustained_grade_holds_edge_0() {
    let r = rel(4, vec![(0, 1), (1, 2), (2, 3)]);
    let reg = Registration::synthetic(NodeField::new(vec![vec![0.0, 1.0, 0.0, -2f64.powi(-53)]]),
        vec![vec![0.0, 1.0, 0.0, 2f64.powi(-54)]]);
    let c = connection(&[reg.clone(), reg], &r).unwrap();
    assert!(c.sustained.unwrap().held_edges.contains(&0));
}

/// K7: the public pair carries the one-sided guarantee, and the exact verdict
/// is separate. Triangle closure A[e0] + A[e1] − A[e2] over (1, 2⁻⁵³, 0) with
/// bounds (1, 2⁻⁵⁴, 0). Exactly, 1 + 2⁻⁵³ exceeds 1 + 2⁻⁵⁴: resolved. The
/// pair reports 1 (the tie rounds to even) against 1 + 2⁻⁵² (rounded up), so
/// a caller comparing it reads the closure as holding.
#[test]
fn k7_the_pair_is_one_sided_and_the_verdict_is_exact() {
    let r = rel(3, vec![(0, 1), (1, 2), (0, 2)]);
    let cy = vec![(0, 1), (1, 1), (2, -1)];
    let a = vec![vec![vec![1.0, 2f64.powi(-53), 0.0]]; 2];
    let d = vec![vec![vec![1.0, 2f64.powi(-54), 0.0]]; 2];
    let s = EdgeSeries::from_edge_values_synthetic(a, d).unwrap();
    assert_eq!(cycle_closure(&s, &r, &cy, 0, 0).unwrap(), (1.0, (1.0f64).next_up()));
    assert_eq!(cycle_closure_resolved(&s, &r, &cy, 0, 0), Some(true));
    assert_eq!(cycle_change_closure_resolved(&s, &r, &cy, 0, 0), Some(false));
}

// ── Inside the range, nothing is refused ──────────────────────────────────

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
    /// 0 or ±(1 + j/8)·2^p with p from the ends and middle of [−200, 199].
    fn edge_of_range(&mut self) -> f64 {
        if self.next() % 5 == 0 { return 0.0; }
        let p = [-200, -199, -150, 0, 150, 199][self.usize(0, 5)];
        let m = 1.0 + self.int(0, 7) as f64 / 8.0;
        if self.next() % 2 == 0 { m * 2f64.powi(p) } else { -m * 2f64.powi(p) }
    }
}

#[test]
fn prop_every_reading_inside_the_range_completes() {
    for seed in 0..200u64 {
        let mut r = Seeds(seed ^ 0x46);
        let n = r.usize(3, 5);
        let mut edges: Vec<(usize, usize)> = Vec::new();
        while edges.len() < n + 1 {
            let (s, t) = (r.usize(0, n - 1), r.usize(0, n - 1));
            if s != t && !edges.contains(&(s, t)) { edges.push((s, t)); }
        }
        let rel = rel(n, edges);
        let m = rel.n_edges();
        let regs = 3;
        let delta: Vec<Vec<f64>> = (0..2).map(|_| (0..n).map(|_| r.edge_of_range().abs()).collect()).collect();
        let gain = |r: &mut Seeds| [0.0, 2f64.powi(-200), 0.5][r.usize(0, 2)];
        let lg: Vec<Vec<f64>> = (0..2).map(|_| (0..n).map(|_| gain(&mut r)).collect()).collect();
        let offs: Vec<Vec<f64>> = (0..2).map(|_| (0..n).map(|_| r.edge_of_range().abs()).collect()).collect();
        let sg = vec![gain(&mut r), gain(&mut r)];
        let seq: Vec<Registration> = (0..regs).map(|_| {
            let x: Vec<Vec<f64>> = (0..2).map(|_| (0..n).map(|_| r.edge_of_range()).collect()).collect();
            Registration::synthetic(NodeField::new(x), delta.clone()).with_run_constant(offs.clone(), lg.clone(), sg.clone())
        }).collect();
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = RelationalField::new(&rel, seq.clone(), None);
            let _ = connection(&seq, &rel);
            let s = EdgeSeries::from_registrations(&seq, &rel).unwrap();
            let _ = PersistenceChange::new(&s);
            for st in 0..s.n_steps() { let _ = s.change_magnitude(st); }
            // Edge-registered, with every kind of gain.
            let a: Vec<Vec<Vec<f64>>> = (0..regs).map(|_| (0..2).map(|_| (0..m).map(|_| r_edge(seed, m)).collect()).collect()).collect();
            let d: Vec<Vec<Vec<f64>>> = vec![(0..2).map(|_| (0..m).map(|_| r_edge(seed + 1, m).abs()).collect()).collect(); regs];
            let g = EdgeGains { component: vec![2f64.powi(-200), 0.5], edge: vec![0.25; m], independent: vec![vec![2f64.powi(-200); m]; 2] };
            let es = EdgeSeries::from_edge_values_synthetic(a, d).unwrap().with_run_constant(vec![vec![2f64.powi(-200); m]; 2], g);
            let _ = PersistenceChange::new(&es);
            let order: Vec<usize> = (0..m).collect();
            let mut part: Vec<usize> = (0..n).collect();
            fn root(p: &[usize], v: usize) -> usize { let mut v = v; while p[v] != v { v = p[v]; } v }
            let mut tree = Vec::new();
            for &e in &order { let (s, t) = rel.edges[e]; let (x, y) = (root(&part, s), root(&part, t)); if x != y { part[x] = y; tree.push(e); } }
            let b = cycle_basis(&rel, &tree).unwrap();
            for cy in &b.cycles { for t in 0..regs { let _ = cycle_closure(&es, &rel, cy, t, 0); } let _ = cycle_change_closure(&es, &rel, cy, 0, 1); }
        }));
        assert!(caught.is_ok(), "a reading inside the admitted range was refused at seed {seed}");
    }
}

/// A deterministic in-range value for the edge-registered part.
fn r_edge(seed: u64, m: usize) -> f64 {
    let mut r = Seeds(seed.wrapping_mul(31).wrapping_add(m as u64));
    r.edge_of_range()
}
