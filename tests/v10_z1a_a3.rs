//! Kernel V10.3 — δ in two parts (Z1a, corrected and confirmed by Origin,
//! Part 9, 23rd), the gain-sign condition (the Verifier's qualifier), and the
//! record-boundary flag (A-3, Part 9, 20th).
//!
//! [IMPL] SYNTHETIC FIXTURES: values are chosen to exercise the forms. They
//! establish that the forms compute as stated, nothing empirical. Values are
//! dyadic, so scaling by ±2^k is exact.

use abr_kernel::ground_variables::*;
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::persistence_change::*;
use abr_kernel::relational_field::boundary_conditioned_edges;

fn rel(n: usize, edges: Vec<(usize, usize)>) -> DeclaredRelations {
    let m = edges.len();
    DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m])
}

fn reg(data: Vec<Vec<f64>>, d: f64) -> Registration {
    let (k, n) = (data.len(), data[0].len());
    Registration::new(NodeField::new(data), vec![vec![d; n]; k])
}

fn run_constant(seq: &[Registration], offset: f64, locus_gain: f64, shared_gain: f64) -> Vec<Registration> {
    let (k, n) = (seq[0].x.k, seq[0].x.n);
    seq.iter().map(|r| r.clone().with_run_constant(
        vec![vec![offset; n]; k], vec![vec![locus_gain; n]; k], vec![shared_gain; k])).collect()
}

fn readings(pc: &PersistenceChange) -> Vec<(usize, usize, usize, Reading)> {
    pc.relations.iter().map(|r| (r.step, r.component, r.edge, r.reading)).collect()
}

// ── Self-contained generator (SplitMix64), as in v10_properties ────────────

const CASES: u64 = 300;

use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
fn hit(c: &AtomicUsize) { c.fetch_add(1, Relaxed); }
fn exercised(c: &AtomicUsize, name: &str) {
    assert!(c.load(Relaxed) > 0, "property {name} was never exercised: the generator is vacuous");
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn int(&mut self, lo: i64, hi: i64) -> i64 { lo + (self.next() % (hi - lo + 1) as u64) as i64 }
    fn usize(&mut self, lo: usize, hi: usize) -> usize { self.int(lo as i64, hi as i64) as usize }
    fn eighth(&mut self, lo: f64, hi: f64) -> f64 { self.int((lo * 8.0) as i64, (hi * 8.0) as i64) as f64 / 8.0 }
    fn delta(&mut self, hi: f64) -> f64 { self.int(0, (hi * 16.0) as i64) as f64 / 16.0 }
    fn chance(&mut self, p_in_8: u64) -> bool { self.next() % 8 < p_in_8 }
}

fn for_all(name: &str, prop: impl Fn(&mut Rng)) {
    for seed in 0..CASES {
        let mut r = Rng(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ 0x5A5A);
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| prop(&mut r)));
        if caught.is_err() { panic!("property {name} failed at seed {seed}"); }
    }
}

fn random_rel(r: &mut Rng) -> DeclaredRelations {
    let n = r.usize(3, 6);
    let m = r.usize(2, 8.min(n * (n - 1)));
    let mut edges: Vec<(usize, usize)> = Vec::new();
    while edges.len() < m {
        let (s, t) = (r.usize(0, n - 1), r.usize(0, n - 1));
        if s != t && !edges.contains(&(s, t)) { edges.push((s, t)); }
    }
    rel(n, edges)
}

fn random_seq(r: &mut Rng, n: usize) -> Vec<Registration> {
    let (k, regs) = (r.usize(2, 3), r.usize(3, 5));
    let mut x: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| r.eighth(-4.0, 4.0)).collect()).collect();
    let delta: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| r.delta(0.5)).collect()).collect();
    let mut out = Vec::new();
    for _ in 0..regs {
        out.push(Registration::new(NodeField::new(x.clone()), delta.clone()));
        for c in x.iter_mut() { for v in c.iter_mut() { if r.chance(5) { *v += r.eighth(-2.0, 2.0); } } }
    }
    out
}

/// A chain 0 → 1 → 2 → 3, two components, four registrations.
fn chain() -> (DeclaredRelations, Vec<Registration>) {
    let r = rel(4, vec![(0, 1), (1, 2), (2, 3)]);
    let seq = (0..4).map(|t| {
        let t = t as f64;
        reg(vec![vec![5.0, 3.0 + 0.5 * t, 2.0, 0.0], vec![1.0, 2.0, 4.0 + 0.25 * t * t, 1.0]], 0.0625)
    }).collect();
    (r, seq)
}

// ═══════════════════════════════════════════════════════════════════════════
// Z1a: offsets
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn z1a_new_declares_repeatability_alone() {
    let (r, seq) = chain();
    let s = EdgeSeries::from_registrations(&seq, &r).unwrap();
    assert_eq!(s.da, s.da_rep);
    assert!(s.dgain.iter().flatten().flatten().all(|g| *g == 0.0));
    assert!(s.shared_gain.iter().flatten().all(|g| *g == 0.0));
}

#[test]
fn z1a_an_offset_cancels_in_every_change() {
    let (r, seq) = chain();
    let base = EdgeSeries::from_registrations(&seq, &r).unwrap();
    let off = EdgeSeries::from_registrations(&run_constant(&seq, 0.5, 0.0, 0.0), &r).unwrap();
    for s in 0..base.n_steps() { assert_eq!(base.change(s), off.change(s)); }
    assert_eq!(readings(&PersistenceChange::new(&base)), readings(&PersistenceChange::new(&off)));
    let b = PersistenceChange::new(&base);
    let o = PersistenceChange::new(&off);
    assert_eq!(b.across_components, o.across_components);
    assert_eq!(b.across_steps, o.across_steps);
}

#[test]
fn z1a_an_offset_enters_the_state_forms() {
    let (r, seq) = chain();
    let base = EdgeSeries::from_registrations(&seq, &r).unwrap();
    let off = EdgeSeries::from_registrations(&run_constant(&seq, 0.5, 0.0, 0.0), &r).unwrap();
    for t in 0..base.n_registrations() {
        for c in 0..base.k() {
            for e in 0..base.n_edges() { assert_eq!(off.da[t][c][e], base.da[t][c][e] + 1.0); }
        }
    }
    // Component 1, edge (2, 3): A = 3 at registration 0, resolved at 0.125
    // (repeatability) and still at 1.125 (with the offset); edge (0, 1) of
    // component 1: A = −1, resolved at 0.125 and not at 1.125.
    let state = |s: &EdgeSeries, e: usize| s.a[0][1][e].abs() > s.da[0][1][e];
    assert!(state(&base, 0) && !state(&off, 0));
    assert!(state(&base, 2) && state(&off, 2));
}

#[test]
fn z1a_an_edge_valued_offset_enters_the_state_bound_only() {
    let a = vec![vec![vec![1.0, 2.0]], vec![vec![1.5, 2.0]], vec![vec![2.0, 2.0]]];
    let da = vec![vec![vec![0.0625; 2]]; 3];
    let base = EdgeSeries::from_edge_values(a.clone(), da.clone()).unwrap();
    let off = EdgeSeries::from_edge_values(a, da).unwrap().with_run_constant(vec![vec![0.25, 0.25]], vec![vec![0.0, 0.0]]);
    for s in 0..2 { assert_eq!(base.change(s), off.change(s)); }
    assert_eq!(off.da[0][0][0], 0.3125);
    assert_eq!(off.da_rep, base.da_rep);
}

// ═══════════════════════════════════════════════════════════════════════════
// Z1a: gains
// ═══════════════════════════════════════════════════════════════════════════

/// One relation (0, 1): locus 0 rises by 0.5, locus 1 holds. Repeatability
/// 1/16 per locus, so the change bound is 4/16 = 0.25 and Δ = 0.5 is freedom.
fn one_step() -> (DeclaredRelations, Vec<Registration>) {
    let r = rel(2, vec![(0, 1)]);
    (r, vec![reg(vec![vec![1.0, 0.0]], 0.0625), reg(vec![vec![1.5, 0.0]], 0.0625)])
}

fn with_locus_gain(seq: &[Registration], g0: f64, g1: f64) -> Vec<Registration> {
    seq.iter().map(|r| r.clone().with_run_constant(vec![vec![0.0, 0.0]], vec![vec![g0, g1]], vec![0.0])).collect()
}

#[test]
fn z1a_an_unshared_gain_enters_the_change_bound_by_each_ends_change() {
    let (r, seq) = one_step();
    let base = EdgeSeries::from_registrations(&seq, &r).unwrap();
    assert_eq!(base.change(0).1[0][0], 0.25);
    assert_eq!(PersistenceChange::new(&base).relations[0].reading, Reading::Freedom);
    // γ_0 = 1/4 on |Δx_0| = 1/2 adds 1/8; γ_1 on |Δx_1| = 0 adds nothing.
    let g = EdgeSeries::from_registrations(&with_locus_gain(&seq, 0.25, 0.5), &r).unwrap();
    assert_eq!(g.dgain[0][0][0], 0.125);
    assert_eq!(g.change(0).1[0][0], 0.375);
    assert_eq!(PersistenceChange::new(&g).relations[0].reading, Reading::Freedom);
    // γ_0 = 1/2 adds 1/4: the bound reaches |Δ| = 1/2, and Δ is not resolved.
    let g = EdgeSeries::from_registrations(&with_locus_gain(&seq, 0.5, 0.0), &r).unwrap();
    assert_eq!(g.change(0).1[0][0], 0.5);
    assert_eq!(PersistenceChange::new(&g).relations[0].reading, Reading::Structure);
}

#[test]
fn z1a_an_unshared_gain_enters_the_state_bound_on_the_value() {
    let (r, seq) = one_step();
    let g = EdgeSeries::from_registrations(&with_locus_gain(&seq, 0.25, 0.5), &r).unwrap();
    // 2/16 + 1/4·|1| + 1/2·|0| at registration 0; 2/16 + 1/4·1.5 at 1.
    assert_eq!(g.da[0][0][0], 0.375);
    assert_eq!(g.da[1][0][0], 0.5);
    assert_eq!(g.da_rep[0][0][0], 0.125);
}

#[test]
fn z1a_a_shared_gain_changes_no_classification_and_enters_magnitudes() {
    for_all("z1a_shared_gain", |r| {
        let rl = random_rel(r);
        let seq = random_seq(r, rl.n_nodes);
        let gamma = r.int(1, 15) as f64 / 16.0;
        let gained = run_constant(&seq, 0.0, 0.0, gamma);
        let (a, b) = (EdgeSeries::from_registrations(&seq, &rl).unwrap(), EdgeSeries::from_registrations(&gained, &rl).unwrap());
        let (pa, pb) = (PersistenceChange::new(&a), PersistenceChange::new(&b));
        assert_eq!(pa, pb);
        let (ga, gb) = (ground_reading(&seq, &rl, None), ground_reading(&gained, &rl, None));
        assert_eq!(format!("{:?}", ga.connection), format!("{:?}", gb.connection));
        for s in 0..b.n_steps() {
            let ((d, dd), (_, dm)) = (b.change(s), b.change_magnitude(s));
            for c in 0..b.k() { for e in 0..b.n_edges() {
                assert_eq!(dm[c][e], dd[c][e] + gamma * d[c][e].abs());
            } }
        }
    });
}

#[test]
#[should_panic(expected = "a gain bound must lie in [0, 1)")]
fn z1a_a_gain_bound_of_one_or_more_is_refused() {
    let (_, seq) = one_step();
    let _ = seq[0].clone().with_run_constant(vec![vec![0.0, 0.0]], vec![vec![0.0, 0.0]], vec![1.0]);
}

#[test]
#[should_panic(expected = "a gain bound must lie in [0, 1)")]
fn z1a_an_edge_gain_bound_of_one_or_more_is_refused() {
    let s = EdgeSeries::from_edge_values(vec![vec![vec![1.0]], vec![vec![2.0]]], vec![vec![vec![0.0]]; 2]).unwrap();
    let _ = s.with_run_constant(vec![vec![0.0]], vec![vec![1.5]]);
}

#[test]
#[should_panic(expected = "run-constant parts of δ must be the same")]
fn z1a_run_constant_parts_must_hold_over_the_run() {
    let (r, seq) = one_step();
    let mixed = vec![seq[0].clone(), with_locus_gain(&seq[1..], 0.25, 0.0).remove(0)];
    let _ = EdgeSeries::from_registrations(&mixed, &r);
}

// ═══════════════════════════════════════════════════════════════════════════
// The gain-sign condition: a negative scale is a declared mapping, not a gain
// ═══════════════════════════════════════════════════════════════════════════

fn flip(s: AtDelta<Sign>) -> AtDelta<Sign> {
    match s {
        AtDelta::Resolved(Sign::Positive) => AtDelta::Resolved(Sign::Negative),
        AtDelta::Resolved(Sign::Negative) => AtDelta::Resolved(Sign::Positive),
        n => n,
    }
}

/// Values of component c scaled by a_c = ±2^j, δ by |a_c|. Every resolution,
/// held state and crossing is unchanged; first-order signs reverse with a_c;
/// a minor over components i, j is multiplied by a_i·a_j, so its sign
/// reverses when exactly one is negative.
#[test]
fn pc_d3_negative_scale_reverses_signs_only() {
    static FLIPPED: AtomicUsize = AtomicUsize::new(0);
    static MINOR: AtomicUsize = AtomicUsize::new(0);
    static ARR: AtomicUsize = AtomicUsize::new(0);
    static SUST: AtomicUsize = AtomicUsize::new(0);
    for_all("negative_scale", |r| {
        let rl = random_rel(r);
        let seq = random_seq(r, rl.n_nodes);
        let k = seq[0].x.k;
        let scale: Vec<f64> = (0..k).map(|c| {
            let m = [0.5, 1.0, 2.0][r.usize(0, 2)];
            if c == 0 || r.chance(4) { -m } else { m }
        }).collect();
        let scaled: Vec<Registration> = seq.iter().map(|g| Registration::new(
            NodeField::new(g.x.data.iter().zip(&scale).map(|(v, a)| v.iter().map(|x| x * a).collect()).collect()),
            g.delta.iter().zip(&scale).map(|(d, a)| d.iter().map(|x| x * a.abs()).collect()).collect(),
        )).collect();
        let (sa, sb) = (EdgeSeries::from_registrations(&seq, &rl).unwrap(), EdgeSeries::from_registrations(&scaled, &rl).unwrap());
        let (pa, pb) = (PersistenceChange::new(&sa), PersistenceChange::new(&sb));

        // PC1: every reading unchanged; Δ reverses with a_c.
        assert_eq!(readings(&pa), readings(&pb));
        for (x, y) in pa.relations.iter().zip(&pb.relations) {
            assert_eq!(y.change, x.change * scale[x.component]);
            if x.reading == Reading::Freedom && scale[x.component] < 0.0 { hit(&FLIPPED); }
        }
        // PC2: every reading unchanged; a minor scales by the product.
        for (ca, cb) in [(&pa.across_components, &pb.across_components), (&pa.across_steps, &pb.across_steps)] {
            if let (Ok(ca), Ok(cb)) = (ca, cb) {
                assert_eq!(ca.readings.len(), cb.readings.len());
                for (x, y) in ca.readings.iter().zip(&cb.readings) {
                    assert_eq!((x.e, x.f, x.over, x.reading), (y.e, y.f, y.over, y.reading));
                    let f = match x.over {
                        ReadingPair::Components { i, j, .. } => scale[i] * scale[j],
                        ReadingPair::Steps { component, .. } => scale[component] * scale[component],
                    };
                    assert_eq!(y.minor, x.minor * f);
                    if f < 0.0 && x.reading == Reading::Structure && x.minor != 0.0 { hit(&MINOR); }
                }
            }
        }
        // PC3: held vs changed unchanged; each sign reverses when a_i·a_j < 0.
        if let (Ok(xa), Ok(xb)) = (&pa.arrangement, &pb.arrangement) {
            assert_eq!(xa.len(), xb.len());
            for (x, y) in xa.iter().zip(xb) {
                assert_eq!((x.step, x.e, x.f, x.i, x.j, x.reading), (y.step, y.e, y.f, y.i, y.j, y.reading));
                let rev = scale[x.i] * scale[x.j] < 0.0;
                assert_eq!(y.before, if rev { flip(x.before) } else { x.before });
                assert_eq!(y.after, if rev { flip(x.after) } else { x.after });
                if rev && x.before.is_resolved() { hit(&ARR); }
            }
        }
        // Sustained (Ξ): the same edges are held and reorganizing.
        let (ga, gb) = (ground_reading(&seq, &rl, None), ground_reading(&scaled, &rl, None));
        if let (Ok(Ok(x)), Ok(Ok(y))) = (ga.connection.as_ref().map(|c| &c.sustained), gb.connection.as_ref().map(|c| &c.sustained)) {
            assert_eq!(x.held_edges, y.held_edges);
            assert_eq!(x.reorganizing_edges, y.reorganizing_edges);
            if !x.held_edges.is_empty() { hit(&SUST); }
        }
    });
    exercised(&FLIPPED, "negative_scale: a resolved change reversed");
    exercised(&MINOR, "negative_scale: a minor reversed by a mixed-sign product");
    exercised(&ARR, "negative_scale: a resolved arrangement sign reversed");
    exercised(&SUST, "negative_scale: a held circulation");
}

// ═══════════════════════════════════════════════════════════════════════════
// A-3: record-boundary-conditioned circulation
// ═══════════════════════════════════════════════════════════════════════════

/// 0 → 1 → 2 → 3 → 1, and 4 → 5 apart. Edge (0, 1) has a successor and no
/// predecessor: boundary-conditioned. Edge (4, 5) has neither: not formable.
fn a3_field() -> DeclaredRelations { rel(6, vec![(0, 1), (1, 2), (2, 3), (3, 1), (4, 5)]) }

#[test]
fn a3_one_empty_side_is_flagged_both_empty_is_not_formable() {
    let r = a3_field();
    assert_eq!(boundary_conditioned_edges(&r), vec![0]);
    let seq: Vec<Registration> = (0..3).map(|t| {
        let t = t as f64;
        reg(vec![vec![4.0 + t, 3.0, 1.0 - t, 2.0, 1.0, 0.0]], 0.0625)
    }).collect();
    let s = ground_reading(&seq, &r, None).connection.unwrap().sustained.unwrap();
    assert_eq!(s.boundary_conditioned, vec![0]);
    assert_eq!(s.not_formable, vec![4]);
    assert!(!s.boundary_conditioned.iter().any(|e| s.not_formable.contains(e)));
}

#[test]
fn a3_a_chain_is_boundary_conditioned_at_both_ends() {
    let r = rel(3, vec![(0, 1), (1, 2)]);
    assert_eq!(boundary_conditioned_edges(&r), vec![0, 1]);
    let closed = rel(3, vec![(0, 1), (1, 2), (2, 0)]);
    assert!(boundary_conditioned_edges(&closed).is_empty());
}

#[test]
fn a3_the_flag_agrees_with_the_sustained_grade_on_random_fields() {
    static FLAGGED: AtomicUsize = AtomicUsize::new(0);
    for_all("a3_flag", |r| {
        let rl = random_rel(r);
        let seq = random_seq(r, rl.n_nodes);
        let flagged = boundary_conditioned_edges(&rl);
        for &e in &flagged { assert!(rl.succ(e).is_empty() != rl.pred(e).is_empty()); }
        if let Ok(Ok(s)) = ground_reading(&seq, &rl, None).connection.map(|c| c.sustained) {
            assert_eq!(s.boundary_conditioned, flagged);
            assert!(flagged.iter().all(|e| !s.not_formable.contains(e)));
            if !flagged.is_empty() { hit(&FLAGGED); }
        }
    });
    exercised(&FLAGGED, "a3_flag: a flagged edge in a formed sustained grade");
}
