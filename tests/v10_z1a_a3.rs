//! Kernel V10.3 — δ in two parts (Z1a, corrected and confirmed by Origin,
//! Part 9, 23rd), the gain-sign condition (the Verifier's qualifier), and the
//! record-boundary flag (A-3, Part 9, 20th).
//!
//! [IMPL] SYNTHETIC FIXTURES: values are chosen to exercise the forms. They
//! establish that the forms compute as stated, nothing empirical. Values are
//! dyadic, so scaling by ±2^k is exact.

#[path = "support/synthetic_feedback.rs"]
mod synthetic_feedback;
#[path = "support/synthetic_identity.rs"]
mod synthetic_identity;
use abr_kernel::ground_variables::*;
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::persistence_change::*;
use abr_kernel::relational_field::boundary_conditioned_edges;

fn rel(n: usize, edges: Vec<(usize, usize)>) -> DeclaredRelations {
    let m = edges.len();
    synthetic_identity::identify(DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m]))
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
    synthetic_feedback::declare_loops(rel(n, edges))
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
    let off = EdgeSeries::from_edge_values(a, da).unwrap().with_run_constant(vec![vec![0.25, 0.25]], EdgeGains::none(1, 2));
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
    // The gain scales each end's change with its repeatability (35th):
    // γ_0·(|Δx_0| + 2/16) = 1/4·(1/2 + 1/8) = 5/32; γ_1·(0 + 2/16) = 1/16.
    // *Was* γ_0·|Δx_0| = 1/8 and 0 (bound 0.375).
    let g = EdgeSeries::from_registrations(&with_locus_gain(&seq, 0.25, 0.5), &r).unwrap();
    assert_eq!(g.dgain[0][0][0], 0.21875);
    assert_eq!(g.change(0).1[0][0], 0.46875);
    assert_eq!(PersistenceChange::new(&g).relations[0].reading, Reading::Freedom);
    // γ_0 = 1/2 adds 1/2·(5/8) = 5/16: the bound passes |Δ| = 1/2, and Δ is
    // not resolved. *Was* 1/4, bound exactly 1/2.
    let g = EdgeSeries::from_registrations(&with_locus_gain(&seq, 0.5, 0.0), &r).unwrap();
    assert_eq!(g.change(0).1[0][0], 0.5625);
    assert_eq!(PersistenceChange::new(&g).relations[0].reading, Reading::Structure);
}

#[test]
fn z1a_an_unshared_gain_enters_the_state_bound_on_the_value() {
    let (r, seq) = one_step();
    let g = EdgeSeries::from_registrations(&with_locus_gain(&seq, 0.25, 0.5), &r).unwrap();
    // Per locus δ + γ·(|x| + δ) (35th): at registration 0,
    // (1/16 + 1/4·(1 + 1/16)) + (1/16 + 1/2·(0 + 1/16)) = 27/64; at 1,
    // (1/16 + 1/4·(1.5 + 1/16)) + 3/32 = 35/64. *Was* 0.375 and 0.5.
    assert_eq!(g.da[0][0][0], 0.421875);
    assert_eq!(g.da[1][0][0], 0.546875);
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
                assert_eq!(dm[c][e], dd[c][e] + gamma * (d[c][e].abs() + dd[c][e]));
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
    let _ = s.with_run_constant(vec![vec![0.0]], EdgeGains { component: vec![0.0], edge: vec![0.0], independent: vec![vec![1.5]] });
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
/// The loop 1 → 2 → 3 → 1 is declared feedback at its closing edge (T8).
fn a3_field() -> DeclaredRelations {
    rel(6, vec![(0, 1), (1, 2), (2, 3), (3, 1), (4, 5)]).with_feedback(vec![
        abr_kernel::topology::FeedbackDeclaration { edge: 3, basis: "SYNTHETIC: return relation 3 → 1".into() },
    ])
}

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
    // The headline counts edges 1 to 3 (both sides registered); edge 0 is
    // reported apart (A-3, 35th); edge 0 stays in the per-edge lists.
    assert!(s.held_edges.contains(&0) || s.reorganizing_edges.contains(&0));
    let both: Vec<usize> = vec![1, 2, 3];
    let held_both = both.iter().filter(|e| s.held_edges.contains(e)).count() as f64;
    assert_eq!(s.share_held, Some(held_both / 3.0));
    assert_eq!(s.share_held_boundary, Some(if s.held_edges.contains(&0) { 1.0 } else { 0.0 }));
    assert_eq!(s.share_held_all, s.held_edges.len() as f64 / 4.0);
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

// ═══════════════════════════════════════════════════════════════════════════
// Gains on edge-registered values (the Verifier's 10.3.0 finding, 34th)
// ═══════════════════════════════════════════════════════════════════════════

/// Two components, two edges, zero repeatability: both changes are (1, 1)
/// at step 0, so the PC2(a) minor is 0 and reads structure.
fn verifier_case(scale_entry: f64) -> Vec<Vec<Vec<f64>>> {
    vec![vec![vec![0.0, 0.0], vec![0.0, 0.0]], vec![vec![1.0 * scale_entry, 1.0], vec![1.0, 1.0]]]
}

fn pc2a(s: &EdgeSeries) -> Vec<(f64, Reading)> {
    coupling_across_components(s).unwrap().readings.iter().map(|x| (x.minor, x.reading)).collect()
}

#[test]
fn z1a_the_verifiers_case_an_independent_gain_enters_the_mixing_minor() {
    let zero = vec![vec![vec![0.0; 2]; 2]; 2];
    let base = EdgeSeries::from_edge_values(verifier_case(1.0), zero.clone()).unwrap();
    assert_eq!(pc2a(&base), vec![(0.0, Reading::Structure)]);
    // A factor 1.125 on one (component, edge) entry moves the minor off zero.
    let moved = EdgeSeries::from_edge_values(verifier_case(1.125), zero.clone()).unwrap();
    assert_eq!(pc2a(&moved), vec![(0.125, Reading::Freedom)]);
    // Declared as an independent gain of 1/8 on that entry, neither reads freedom:
    let mut ind = vec![vec![0.0; 2]; 2];
    ind[0][0] = 0.125;
    let g = || EdgeGains { component: vec![0.0; 2], edge: vec![0.0; 2], independent: ind.clone() };
    let base = EdgeSeries::from_edge_values(verifier_case(1.0), zero.clone()).unwrap().with_run_constant(vec![vec![0.0; 2]; 2], g());
    let moved = EdgeSeries::from_edge_values(verifier_case(1.125), zero.clone()).unwrap().with_run_constant(vec![vec![0.0; 2]; 2], g());
    assert_eq!(pc2a(&base)[0].1, Reading::Structure);
    assert_eq!(pc2a(&moved)[0].1, Reading::Structure);
    // PC1 is unchanged by the declaration.
    let r = |s: &EdgeSeries| PersistenceChange::new(s).relations.iter().map(|x| (x.change, x.half_width, x.reading)).collect::<Vec<_>>();
    assert_eq!(r(&base), r(&EdgeSeries::from_edge_values(verifier_case(1.0), zero).unwrap()));
}

#[test]
fn z1a_a_shared_gain_scales_a_whole_component_or_edge_and_changes_nothing() {
    let zero = vec![vec![vec![0.0; 2]; 2]; 2];
    // The same factor 1.125 on every edge of component 0: the minor stays 0.
    let mut a = verifier_case(1.0);
    for t in 0..2 { for e in 0..2 { a[t][0][e] *= 1.125; } }
    assert_eq!(pc2a(&EdgeSeries::from_edge_values(a, zero.clone()).unwrap()), vec![(0.0, Reading::Structure)]);
    // The same factor on every component of edge 0: the minor stays 0.
    let mut a = verifier_case(1.0);
    for t in 0..2 { for c in 0..2 { a[t][c][0] *= 1.125; } }
    assert_eq!(pc2a(&EdgeSeries::from_edge_values(a, zero).unwrap()), vec![(0.0, Reading::Structure)]);
}

fn random_edge_values(r: &mut Rng) -> (Vec<Vec<Vec<f64>>>, Vec<Vec<Vec<f64>>>) {
    let (m, k, regs) = (r.usize(2, 5), r.usize(2, 3), r.usize(3, 4));
    let mut v: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| r.eighth(-4.0, 4.0)).collect()).collect();
    let d: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| r.delta(0.25)).collect()).collect();
    let (mut aa, mut dd) = (Vec::new(), Vec::new());
    for _ in 0..regs {
        aa.push(v.clone()); dd.push(d.clone());
        for c in v.iter_mut() { for x in c.iter_mut() { if r.chance(5) { *x += r.eighth(-2.0, 2.0); } } }
    }
    (aa, dd)
}

fn all_readings(s: &EdgeSeries) -> String {
    let p = PersistenceChange::new(s);
    let r: Vec<_> = p.relations.iter().map(|x| (x.step, x.component, x.edge, x.reading)).collect();
    let c: Vec<_> = [&p.across_components, &p.across_steps].iter()
        .map(|cs| cs.as_ref().map(|cs| cs.readings.iter().map(|x| (x.e, x.f, x.over, x.reading)).collect::<Vec<_>>()).ok()).collect();
    let a: Vec<_> = p.arrangement.as_ref().map(|xs| xs.iter().map(|x| (x.step, x.e, x.f, x.i, x.j, x.reading)).collect::<Vec<_>>()).unwrap_or_default();
    format!("{r:?}{c:?}{a:?}")
}

/// Shared factors (1 + γ_c)(1 + γ_e) on the values, with δ scaled by the same
/// factor (δ declared in the registered unit), change no reading.
#[test]
fn prop_z1a_shared_component_and_edge_factors_change_no_reading() {
    for_all("z1a_shared_factors", |r| {
        let (a, d) = random_edge_values(r);
        let (k, m) = (a[0].len(), a[0][0].len());
        let fc: Vec<f64> = (0..k).map(|_| [1.0, 1.125, 1.25, 1.5][r.usize(0, 3)]).collect();
        let fe: Vec<f64> = (0..m).map(|_| [1.0, 1.125, 1.25, 1.5][r.usize(0, 3)]).collect();
        let sc = |x: &Vec<Vec<Vec<f64>>>| x.iter().map(|t| t.iter().enumerate().map(|(c, v)| v.iter().enumerate()
            .map(|(e, y)| y * fc[c] * fe[e]).collect()).collect()).collect::<Vec<Vec<Vec<f64>>>>();
        let (s0, s1) = (EdgeSeries::from_edge_values(a.clone(), d.clone()).unwrap(), EdgeSeries::from_edge_values(sc(&a), sc(&d)).unwrap());
        assert_eq!(all_readings(&s0), all_readings(&s1));
    });
}

/// An independent factor 1 + γ′ on each entry, |γ′| ≤ γ declared: every
/// PC2(a) and PC3 minor resolved under the declared bound keeps its sign, and
/// PC1 and PC2(b) readings are unchanged when δ scales with the entry.
#[test]
fn prop_z1a_independent_gains_never_reverse_a_resolved_mixing_minor() {
    static RESOLVED: AtomicUsize = AtomicUsize::new(0);
    static WOULD_FLIP: AtomicUsize = AtomicUsize::new(0);
    for_all("z1a_independent_gains", |r| {
        let (a, d) = random_edge_values(r);
        let (k, m) = (a[0].len(), a[0][0].len());
        let gam: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| [0.125, 0.25, 0.5][r.usize(0, 2)]).collect()).collect();
        // γ′ a multiple of γ/4 in [−γ, γ], the same over the run (run-constant).
        let fac: Vec<Vec<f64>> = gam.iter().map(|gc| gc.iter().map(|g| 1.0 + g * (r.usize(0, 8) as f64 - 4.0) / 4.0).collect()).collect();
        let sc = |x: &Vec<Vec<Vec<f64>>>| x.iter().map(|t| t.iter().enumerate().map(|(c, v)| v.iter().enumerate()
            .map(|(e, y)| y * fac[c][e]).collect()).collect()).collect::<Vec<Vec<Vec<f64>>>>();
        let gains = || EdgeGains { component: vec![0.0; k], edge: vec![0.0; m], independent: gam.clone() };
        let declared = EdgeSeries::from_edge_values(a.clone(), d.clone()).unwrap().with_run_constant(vec![vec![0.0; m]; k], gains());
        let truth = EdgeSeries::from_edge_values(sc(&a), d.clone()).unwrap();
        let undeclared = EdgeSeries::from_edge_values(a.clone(), d.clone()).unwrap();
        // PC2(a): a minor resolved with the declared bound has the same sign on the scaled values.
        let (pd, pt) = (PersistenceChange::new(&declared), PersistenceChange::new(&truth));
        let pu = PersistenceChange::new(&undeclared);
        let truth_minor = |x: &ChangeCoupling| pt.across_components.as_ref().ok().and_then(|cs|
            cs.readings.iter().find(|y| (y.e, y.f, y.over) == (x.e, x.f, x.over)).map(|y| y.minor));
        if let Ok(cs) = &pd.across_components {
            for x in cs.readings.iter().filter(|x| x.reading == Reading::Freedom) {
                hit(&RESOLVED);
                if let Some(tm) = truth_minor(x) { assert!(tm * x.minor > 0.0, "a resolved minor reversed"); }
            }
        }
        // Without the declaration, some resolved minors would reverse sign.
        if let (Ok(cu), Ok(ct)) = (&pu.across_components, &pt.across_components) {
            for x in &cu.readings {
                if x.reading == Reading::Freedom {
                    if let Some(y) = ct.readings.iter().find(|y| (y.e, y.f, y.over) == (x.e, x.f, x.over)) {
                        if y.minor * x.minor < 0.0 { hit(&WOULD_FLIP); }
                    }
                }
            }
        }
        // PC3: a resolved state minor under the declared bound keeps its sign.
        if let (Ok(xd), Ok(xt)) = (&pd.arrangement, &pt.arrangement) {
            for x in xd {
                if let Some(y) = xt.iter().find(|y| (y.step, y.e, y.f, y.i, y.j) == (x.step, x.e, x.f, x.i, x.j)) {
                    for (p, q) in [(x.before, y.before), (x.after, y.after)] {
                        if let AtDelta::Resolved(sp) = p { assert!(q == AtDelta::NotResolved || q == AtDelta::Resolved(sp)); }
                    }
                }
            }
        }
        // PC1 and PC2(b): with δ scaled by each entry's factor, unchanged.
        let truth_scaled_d = EdgeSeries::from_edge_values(sc(&a), sc(&d)).unwrap();
        let p1 = |s: &EdgeSeries| PersistenceChange::new(s).relations.iter().map(|x| x.reading).collect::<Vec<_>>();
        assert_eq!(p1(&undeclared), p1(&truth_scaled_d));
        let p2b = |s: &EdgeSeries| PersistenceChange::new(s).across_steps.map(|cs| cs.readings.iter().map(|x| (x.e, x.f, x.over, x.reading)).collect::<Vec<_>>()).ok();
        assert_eq!(p2b(&undeclared), p2b(&truth_scaled_d));
    });
    exercised(&RESOLVED, "z1a_independent_gains: a resolved mixing minor under the declared bound");
    exercised(&WOULD_FLIP, "z1a_independent_gains: an undeclared gain reversing a resolved minor");
}

// ═══════════════════════════════════════════════════════════════════════════
// The scaling model in every gain bound (the Verifier's second 10.3.0
// review, 35th): the gain scales the value together with its bound
// ═══════════════════════════════════════════════════════════════════════════

/// The Verifier's case: Δ = 1, δΔ = 1/8, γ = 1/8. The model permits
/// (1 + 1/8)·(1 + 1/8) = 1.265625; the bound's upper endpoint must reach it.
#[test]
fn z1a_the_verifiers_magnitude_case() {
    let a = vec![vec![vec![0.0]], vec![vec![1.0]]];
    let da = vec![vec![vec![0.0625]]; 2];
    let s = EdgeSeries::from_edge_values(a, da).unwrap()
        .with_run_constant(vec![vec![0.0]], EdgeGains { component: vec![0.125], edge: vec![0.0], independent: vec![vec![0.0]] });
    let (d, dm) = s.change_magnitude(0);
    assert_eq!((d[0][0], s.change(0).1[0][0]), (1.0, 0.125));
    assert_eq!(d[0][0] + dm[0][0], 1.265625);
    assert_eq!(d[0][0] - dm[0][0], 1.0 - 0.265625);
}

/// Every corner of the model, (1 + γ′)(v + ε) with γ′ = ±γ and ε = ±h, lies
/// within v ± (h + γ·(|v| + h)): magnitudes, node state and change bounds,
/// and the independent-gain widening.
#[test]
fn prop_z1a_every_gain_bound_covers_the_scaling_model() {
    static N: AtomicUsize = AtomicUsize::new(0);
    for_all("z1a_scaling_model", |r| {
        // Edge-registered: magnitude and the mixing widening.
        let (a, d) = random_edge_values(r);
        let (k, m) = (a[0].len(), a[0][0].len());
        let gc: Vec<f64> = (0..k).map(|_| r.delta(0.5)).collect();
        let gi: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| r.delta(0.5)).collect()).collect();
        let s = EdgeSeries::from_edge_values(a.clone(), d.clone()).unwrap()
            .with_run_constant(vec![vec![0.0; m]; k], EdgeGains { component: gc.clone(), edge: vec![0.0; m], independent: gi.clone() });
        for st in 0..s.n_steps() {
            let ((dl, dd), (_, dm)) = (s.change(st), s.change_magnitude(st));
            for c in 0..k { for e in 0..m {
                let g = (1.0 + gc[c]) * (1.0 + gi[c][e]) - 1.0;
                for (sg, se) in [(1.0, 1.0), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)] {
                    let corner = (1.0 + sg * g) * (dl[c][e] + se * dd[c][e]);
                    assert!((corner - dl[c][e]).abs() <= dm[c][e] + 1e-12, "magnitude");
                }
                hit(&N);
            } }
        }
        // Node-derived: the state and change bounds with per-locus gains.
        let rl = random_rel(r);
        let seq = random_seq(r, rl.n_nodes);
        let (kk, n) = (seq[0].x.k, seq[0].x.n);
        let lg: Vec<Vec<f64>> = (0..kk).map(|_| (0..n).map(|_| r.delta(0.5)).collect()).collect();
        let seq: Vec<Registration> = seq.into_iter().map(|g| g.with_run_constant(vec![vec![0.0; n]; kk], lg.clone(), vec![0.0; kk])).collect();
        let s = EdgeSeries::from_registrations(&seq, &rl).unwrap();
        for t in 0..s.n_registrations() { for c in 0..kk { for (e, &(p, q)) in rl.edges.iter().enumerate() {
            let x = &seq[t].x.data[c];
            let dl = &seq[t].delta[c];
            for (sp, sq, ep, eq) in [(1.0, -1.0, 1.0, -1.0), (-1.0, 1.0, -1.0, 1.0), (1.0, 1.0, 1.0, -1.0), (-1.0, -1.0, -1.0, 1.0)] {
                let tp = (1.0 + sp * lg[c][p]) * (x[p] + ep * dl[p]);
                let tq = (1.0 + sq * lg[c][q]) * (x[q] + eq * dl[q]);
                assert!(((tp - tq) - (x[p] - x[q])).abs() <= s.da[t][c][e] + 1e-12, "node state");
            }
        } } }
        for st in 0..s.n_steps() {
            let (dl, dd) = s.change(st);
            for c in 0..kk { for (e, &(p, q)) in rl.edges.iter().enumerate() {
                let (x0, x1) = (&seq[st].x.data[c], &seq[st + 1].x.data[c]);
                let (d0, d1) = (&seq[st].delta[c], &seq[st + 1].delta[c]);
                // The same gain factor at both registrations (run-constant).
                for (sp, sq) in [(1.0, -1.0), (-1.0, 1.0)] {
                    let end = |i: usize, sg: f64, se: f64| (1.0 + sg * lg[c][i]) * ((x1[i] + se * d1[i]) - (x0[i] - se * d0[i]));
                    let corner = end(p, sp, 1.0) - end(q, sq, -1.0);
                    assert!((corner - dl[c][e]).abs() <= dd[c][e] + 1e-12, "node change");
                }
            } }
        }
    });
    exercised(&N, "z1a_scaling_model");
}

/// PC2(a) and PC3 under the full model (35th): the true values are
/// (1 + γ′)·(v + ε), |γ′| ≤ γ declared independent, |ε| ≤ δ at each
/// registration. Every minor resolved under the declared bounds keeps its sign
/// on the true values.
#[test]
fn prop_z1a_mixing_minors_hold_under_gain_and_repeatability() {
    static N2: AtomicUsize = AtomicUsize::new(0);
    static N3: AtomicUsize = AtomicUsize::new(0);
    for_all("z1a_mixing_full_model", |r| {
        let (a, d) = random_edge_values(r);
        let (regs, k, m) = (a.len(), a[0].len(), a[0][0].len());
        let gam: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| [0.0, 0.125, 0.25, 0.5][r.usize(0, 3)]).collect()).collect();
        let fac: Vec<Vec<f64>> = gam.iter().map(|gc| gc.iter().map(|g| 1.0 + g * (r.usize(0, 8) as f64 - 4.0) / 4.0).collect()).collect();
        let eps: Vec<Vec<Vec<f64>>> = (0..regs).map(|t| (0..k).map(|c| (0..m)
            .map(|e| d[t][c][e] * (r.usize(0, 4) as f64 - 2.0) / 2.0).collect()).collect()).collect();
        let truth: Vec<Vec<Vec<f64>>> = (0..regs).map(|t| (0..k).map(|c| (0..m)
            .map(|e| fac[c][e] * (a[t][c][e] + eps[t][c][e])).collect()).collect()).collect();
        let s = EdgeSeries::from_edge_values(a.clone(), d.clone()).unwrap().with_run_constant(
            vec![vec![0.0; m]; k], EdgeGains { component: vec![0.0; k], edge: vec![0.0; m], independent: gam.clone() });
        let p = PersistenceChange::new(&s);
        let mnr = |v: &dyn Fn(usize, usize) -> f64, i: usize, j: usize, e: usize, f: usize| v(i, e) * v(j, f) - v(j, e) * v(i, f);
        if let Ok(cs) = &p.across_components {
            for x in cs.readings.iter().filter(|x| x.reading == Reading::Freedom) {
                if let ReadingPair::Components { step, i, j } = x.over {
                    let dv = |c: usize, e: usize| truth[step + 1][c][e] - truth[step][c][e];
                    assert!(mnr(&dv, i, j, x.e, x.f) * x.minor > 0.0, "PC2(a) minor reversed");
                    hit(&N2);
                }
            }
        }
        if let Ok(xs) = &p.arrangement {
            for x in xs {
                for (t, st) in [(x.step, x.before), (x.step + 1, x.after)] {
                    if let AtDelta::Resolved(sg) = st {
                        let v = |c: usize, e: usize| truth[t][c][e];
                        let tm = mnr(&v, x.i, x.j, x.e, x.f);
                        assert!(if sg == Sign::Positive { tm > 0.0 } else { tm < 0.0 }, "PC3 state minor reversed");
                        hit(&N3);
                    }
                }
            }
        }
    });
    exercised(&N2, "z1a_mixing_full_model: PC2(a)");
    exercised(&N3, "z1a_mixing_full_model: PC3");
}
