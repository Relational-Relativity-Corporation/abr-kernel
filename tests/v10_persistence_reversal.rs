//! Kernel V10.3: I-SC-D18, beginning, ending and reversal of the three
//! persistences (Proposed, Generator, 2026-11-23, 52nd; V10_README §2.6).
//!
//! Names (W16 (a), Origin, 52nd): **persistent imbalance**, the sign of Ξ(B(A))
//! held over the sequence (V1 sustained grade); **persistent closure**, the
//! state of a closed walk's closure held (K7); **persistent arrangement**,
//! the state of a minor m_ij(e, f) held (PC3, V8). Each is read per
//! registration as a sign at δ: resolved +, resolved −, or not resolved.
//!
//! (i)   **No persistence reverses without freedom.** Over a step, a reading
//!       of any of the three can go from resolved with one sign to resolved
//!       with the other only where PC1 reads freedom at some relation the
//!       reading is formed from (a non-zero coefficient of Ξ(B(·)); an edge
//!       of the walk; an entry of the minor). Proof: if every such relation
//!       holds at PC1, |x_{s+1} − x_s| ≤ δΔ ≤ h_s + h_{s+1} entry by entry,
//!       so one field x* lies within the state bound of both registrations.
//!       Each reading's bound encloses every field within its entries'
//!       bounds, so both readings lie within their bounds of the reading at
//!       x*, and cannot be resolved with opposite signs. It holds under every
//!       declared part of δ (Z1a): an offset widens the state bound and not
//!       the change bound; a node-derived per-locus gain enters the change
//!       bound as γ·(|Δx| + δ_s + δ_{s+1}) ≤ γ·(|x_s| + δ_s) + γ·(|x_{s+1}| +
//!       δ_{s+1}), its share of the two state bounds; an edge-registered gain
//!       widens a reading's bound and not PC1's. The tests declare all three
//!       on some fields.
//! (ii)  **Beginning and ending need none, at δ.** A reading can go from
//!       not resolved to resolved, or back, with every relation it is
//!       formed from held at PC1. At δ = 0 every change of state needs a
//!       non-zero change.
//! (iii) **Persistent closure and persistent arrangement are independent**:
//!       all four combinations occur on edge-registered fields. (Persistent
//!       arrangement and persistent imbalance: I-SC-D14.)
//!
//! [IMPL] SYNTHETIC FIXTURES: dyadic values, so every sum and product is
//! exact. They establish that the forms compute as stated, nothing
//! empirical.

#[path = "support/synthetic_feedback.rs"]
mod synthetic_feedback;
#[path = "support/synthetic_identity.rs"]
mod synthetic_identity;
use abr_kernel::exact::Expansion;
use abr_kernel::ground_variables::{sign_at_delta_exact, AtDelta, Registration, Sign};
use abr_kernel::operators::{circulation_of_accumulation_coefficients, DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::persistence_change::*;
use std::sync::Mutex;

fn rel(n: usize, edges: Vec<(usize, usize)>) -> DeclaredRelations {
    let m = edges.len();
    synthetic_feedback::declare_loops(synthetic_identity::identify(
        DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m])))
}

// ── Self-contained generator (SplitMix64), as in v10_properties ────────────

const CASES: u64 = 300;

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
        let mut r = Rng(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ 0xD18);
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| prop(&mut r)));
        if caught.is_err() { panic!("property {name} failed at seed {seed}"); }
    }
}

fn random_rel(r: &mut Rng) -> DeclaredRelations {
    let n = r.usize(3, 5);
    let m = r.usize(n, 7.min(n * (n - 1) / 2));
    let mut edges: Vec<(usize, usize)> = Vec::new();
    while edges.len() < m {
        let (s, t) = (r.usize(0, n - 1), r.usize(0, n - 1));
        if s != t && !edges.contains(&(s, t)) && !edges.contains(&(t, s)) { edges.push((s, t)); }
    }
    rel(n, edges)
}

/// A walk of values: each moves by a small dyadic step with chance 1/2, so
/// some relations hold and some change, and some readings sit near their
/// bounds.
fn walk(r: &mut Rng, regs: usize, first: impl Fn(&mut Rng) -> f64) -> Vec<f64> {
    let mut v = vec![first(r)];
    for _ in 1..regs { let x = *v.last().unwrap(); v.push(if r.chance(4) { x + r.eighth(-1.0, 1.0) } else { x }); }
    v
}

/// Node-derived series: k components, δ fixed over the run.
fn node_series(r: &mut Rng, rel: &DeclaredRelations, k: usize, regs: usize) -> EdgeSeries {
    let n = rel.n_nodes;
    let vals: Vec<Vec<Vec<f64>>> = (0..k).map(|_| (0..n).map(|_| walk(r, regs, |r| r.eighth(-3.0, 3.0))).collect()).collect();
    let d: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| r.delta(0.25)).collect()).collect();
    // Run-constant parts (Z1a) on some fields: offsets and per-locus gains.
    let z1a = r.chance(4);
    let offset: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| if z1a && r.chance(3) { r.delta(0.25) } else { 0.0 }).collect()).collect();
    let gain: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| if z1a && r.chance(3) { 1.0 / 16.0 } else { 0.0 }).collect()).collect();
    let seq: Vec<Registration> = (0..regs).map(|t| Registration::synthetic(
        NodeField::new((0..k).map(|c| (0..n).map(|v| vals[c][v][t]).collect()).collect()), d.clone())
        .with_run_constant(offset.clone(), gain.clone(), vec![0.0; k])).collect();
    EdgeSeries::from_registrations(&seq, rel).unwrap()
}

/// Edge-registered series with declared offsets and, on some fields, gains of
/// every kind (per component, per edge, per entry).
fn edge_series(r: &mut Rng, k: usize, m: usize, regs: usize) -> EdgeSeries {
    let vals: Vec<Vec<Vec<f64>>> = (0..k).map(|_| (0..m).map(|_| walk(r, regs, |r| r.eighth(-3.0, 3.0))).collect()).collect();
    let rep: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| r.delta(0.25)).collect()).collect();
    let a = (0..regs).map(|t| (0..k).map(|c| (0..m).map(|e| vals[c][e][t]).collect()).collect()).collect();
    let da = (0..regs).map(|_| rep.clone()).collect();
    let offset: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| if r.chance(3) { r.delta(0.25) } else { 0.0 }).collect()).collect();
    let g = |r: &mut Rng| if r.chance(2) { 1.0 / 16.0 } else { 0.0 };
    let gains = if r.chance(4) {
        EdgeGains { component: (0..k).map(|_| g(r)).collect(), edge: (0..m).map(|_| g(r)).collect(),
            independent: (0..k).map(|_| (0..m).map(|_| g(r)).collect()).collect() }
    } else { EdgeGains::none(k, m) };
    EdgeSeries::from_edge_values_synthetic(a, da).unwrap().with_run_constant(offset, gains)
}

fn pc1_free(s: &EdgeSeries, st: usize, c: usize, e: usize) -> bool {
    abr_kernel::exact::exceeds(&s.change_exact(st, c, e), &s.change_bound_exact(st, c, e))
}

fn reversed(a: AtDelta<Sign>, b: AtDelta<Sign>) -> bool {
    matches!((a, b), (AtDelta::Resolved(x), AtDelta::Resolved(y)) if x != y)
}

/// What happened to one reading over one step, with what PC1 read at the
/// relations it is formed from.
#[derive(Default)]
struct Tally { reversed: usize, reversed_one_moved: usize, began_held: usize, ended_held: usize }

impl Tally {
    fn add(&mut self, before: AtDelta<Sign>, after: AtDelta<Sign>, moved: &[bool]) {
        let any = moved.iter().any(|&m| m);
        if reversed(before, after) {
            assert!(any, "a persistence reversed with every relation it is formed from held at PC1");
            self.reversed += 1;
            if moved.iter().filter(|&&m| m).count() == 1 { self.reversed_one_moved += 1; }
        }
        if !any && !before.is_resolved() && after.is_resolved() { self.began_held += 1; }
        if !any && before.is_resolved() && !after.is_resolved() { self.ended_held += 1; }
    }
    fn require(&self, name: &str) {
        // (i) has content: reversals occur, some with a single relation moved
        // (so no smaller set of relations would do). (ii): beginnings and
        // endings occur with every relation held.
        assert!(self.reversed > 0 && self.reversed_one_moved > 0 && self.began_held > 0 && self.ended_held > 0,
            "{name}: reversed {}, with one relation moved {}, began held {}, ended held {}",
            self.reversed, self.reversed_one_moved, self.began_held, self.ended_held);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Persistent imbalance (V1 sustained grade, K8 on B(A))
// ═══════════════════════════════════════════════════════════════════════════

/// The sign at δ of Ξ(B(A))[e] in component c at registration t, exactly, as
/// `ground_variables::circulation_signs` forms it, with the coefficients.
fn imbalance(s: &EdgeSeries, rel: &DeclaredRelations, t: usize, c: usize, e: usize) -> (AtDelta<Sign>, Vec<i64>) {
    let w = circulation_of_accumulation_coefficients(rel, e, -1);
    let wb = circulation_of_accumulation_coefficients(rel, e, 1);
    let (mut v, mut h) = (Expansion::new(), Expansion::new());
    for (f, &k) in w.iter().enumerate() { if k != 0 { v.add_times(&s.entry(t, c, f), k as f64); } }
    for (f, &k) in wb.iter().enumerate() { if k != 0 { h.add_product(k as f64, s.da[t][c][f]); } }
    (sign_at_delta_exact(&v, &h), w)
}

#[test]
fn d18_persistent_imbalance_reverses_only_with_freedom_among_its_relations() {
    let tally = Mutex::new(Tally::default());
    for_all("d18_imbalance", |r| {
        let rel = random_rel(r);
        let regs = r.usize(2, 4);
        let s = node_series(r, &rel, 2, regs);
        let mut g = tally.lock().unwrap();
        for st in 0..s.n_steps() {
            for c in 0..2 {
                for e in 0..rel.n_edges() {
                    if rel.succ(e).is_empty() && rel.pred(e).is_empty() { continue; }
                    let ((before, w), (after, _)) = (imbalance(&s, &rel, st, c, e), imbalance(&s, &rel, st + 1, c, e));
                    let moved: Vec<bool> = w.iter().enumerate().filter(|(_, &k)| k != 0).map(|(f, _)| pc1_free(&s, st, c, f)).collect();
                    g.add(before, after, &moved);
                }
            }
        }
    });
    tally.into_inner().unwrap().require("persistent imbalance");
}

// ═══════════════════════════════════════════════════════════════════════════
// Persistent closure (K7)
// ═══════════════════════════════════════════════════════════════════════════

/// The diamond 0→1, 1→2, 0→2, 0→3, 3→2 and its three simple closed walks.
fn diamond() -> (DeclaredRelations, Vec<Vec<(usize, i8)>>) {
    (rel(4, vec![(0, 1), (1, 2), (0, 2), (0, 3), (3, 2)]),
     vec![vec![(0, 1), (1, 1), (2, -1)], vec![(3, 1), (4, 1), (2, -1)], vec![(0, 1), (1, 1), (4, -1), (3, -1)]])
}

/// The sign at δ of a walk's closure, exactly, as `closure_exact` forms it:
/// each term's bound δA (offsets included), widened by the closure gain.
fn closure(s: &EdgeSeries, walk: &[(usize, i8)], t: usize, c: usize) -> AtDelta<Sign> {
    let (mut v, mut h) = (Expansion::new(), Expansion::new());
    for &(e, sg) in walk { v.add_times(&s.entry(t, c, e), sg as f64); h = h.plus(&abr_kernel::exact::gain_widened(&s.entry(t, c, e), &Expansion::of(s.da[t][c][e]), s.closure_gain[c][e])); }
    sign_at_delta_exact(&v, &h)
}

#[test]
fn d18_persistent_closure_reverses_only_with_freedom_on_its_walk() {
    let (rel, walks) = diamond();
    let tally = Mutex::new(Tally::default());
    for_all("d18_closure", |r| {
        let regs = r.usize(2, 4);
        let s = edge_series(r, 2, rel.n_edges(), regs);
        let mut g = tally.lock().unwrap();
        for st in 0..s.n_steps() {
            for c in 0..2 {
                for w in &walks {
                    let moved: Vec<bool> = w.iter().map(|&(e, _)| pc1_free(&s, st, c, e)).collect();
                    g.add(closure(&s, w, st, c), closure(&s, w, st + 1, c), &moved);
                }
            }
        }
    });
    tally.into_inner().unwrap().require("persistent closure");
}

// ═══════════════════════════════════════════════════════════════════════════
// Persistent arrangement (PC3, V8)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn d18_persistent_arrangement_reverses_only_with_freedom_at_its_entries() {
    // Through the kernel's own PC3 states, on node-derived fields and on
    // edge-registered fields with offsets. A PC3 crossing is the reversal.
    let tally = Mutex::new(Tally::default());
    for_all("d18_arrangement", |r| {
        let rel = random_rel(r);
        let regs = r.usize(2, 4);
        let node = node_series(r, &rel, 2, regs);
        let edge = edge_series(r, 2, rel.n_edges(), regs);
        let mut g = tally.lock().unwrap();
        for s in [&node, &edge] {
            for x in arrangement_steps(s).unwrap() {
                let moved: Vec<bool> = [(x.i, x.e), (x.j, x.e), (x.i, x.f), (x.j, x.f)].iter()
                    .map(|&(c, e)| pc1_free(s, x.step, c, e)).collect();
                g.add(x.before, x.after, &moved);
            }
        }
    });
    let t = tally.into_inner().unwrap();
    // PC3 enters an edge pair only when both edges are resolved at both
    // registrations, so a beginning from "not resolved" occurs at the minor,
    // not at the edges; it is still required.
    t.require("persistent arrangement");
}

#[test]
fn d18_worked_an_imbalance_ends_with_every_relation_held() {
    // Chain 0 → 1 → 2, one component, δ = 1/16 at each locus. Ξ(B(A)) at
    // edge 0 is A[e1] (its one successor; no predecessor, so the edge is
    // boundary-conditioned, A-3), bound δA[e1] = 1/8. Locus 2 moves by 1/4,
    // exactly δΔ = 1/4, so PC1 holds at every relation. A[e1] goes from 3/8
    // (resolved) to 1/8 (not resolved): the persistence ends with nothing
    // resolved to change.
    let r = rel(3, vec![(0, 1), (1, 2)]);
    let d = vec![vec![1.0 / 16.0; 3]];
    let seq = vec![Registration::synthetic(NodeField::new(vec![vec![0.0, 0.0, -0.375]]), d.clone()),
                   Registration::synthetic(NodeField::new(vec![vec![0.0, 0.0, -0.125]]), d.clone())];
    let s = EdgeSeries::from_registrations(&seq, &r).unwrap();
    let (before, w) = imbalance(&s, &r, 0, 0, 0);
    let (after, _) = imbalance(&s, &r, 1, 0, 0);
    assert_eq!(w, vec![0, 1]);
    assert_eq!(before, AtDelta::Resolved(Sign::Positive));
    assert_eq!(after, AtDelta::NotResolved);
    assert!(!pc1_free(&s, 0, 0, 0) && !pc1_free(&s, 0, 0, 1));
}

// ═══════════════════════════════════════════════════════════════════════════
// (iii) Persistent closure and persistent arrangement are independent
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn d18_iii_closure_and_arrangement_persist_independently() {
    // Over one step: closure persistent when every walk's state, in every
    // component, is the same at both registrations; arrangement persistent
    // when every PC3 reading is structure (with at least one evaluated).
    let (rel, walks) = diamond();
    let seen = Mutex::new([[0usize; 2]; 2]);
    for_all("d18_iii", |r| {
        let s = edge_series(r, 2, rel.n_edges(), 2);
        let arr = arrangement_steps(&s).unwrap();
        if arr.is_empty() { return; }
        let held_arr = arr.iter().all(|x| x.reading == Reading::Structure);
        let held_clo = (0..2).all(|c| walks.iter().all(|w| closure(&s, w, 0, c) == closure(&s, w, 1, c)));
        seen.lock().unwrap()[held_clo as usize][held_arr as usize] += 1;
    });
    let g = seen.into_inner().unwrap();
    assert!(g.iter().flatten().all(|&n| n > 0), "closure × arrangement (changes/persists): {g:?}");
}
