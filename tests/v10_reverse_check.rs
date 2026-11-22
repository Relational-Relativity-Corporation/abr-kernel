//! Kernel V10.3 — RPPI-V10: the reverse check (B2), kernel side.
//!
//! Every PC1, PC2 and PC3 reading the kernel reports re-derives to MATCH from
//! the declared inputs through `reverse_check`, which shares no code with the
//! forms that produced it; every tampered reading is a MISMATCH naming what
//! differs.
//!
//! [IMPL] SYNTHETIC FIXTURES: values are chosen to exercise the check. They
//! establish that the check computes as stated, nothing empirical.

#[path = "support/synthetic_feedback.rs"]
mod synthetic_feedback;

use abr_kernel::ground_variables::{AtDelta, Registration, Sign};
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::persistence_change::*;
use abr_kernel::reverse_check::*;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn usize(&mut self, lo: usize, hi: usize) -> usize { lo + (self.next() % (hi - lo + 1) as u64) as usize }
    fn eighth(&mut self) -> f64 { (self.next() % 65) as f64 / 8.0 - 4.0 }
    fn delta(&mut self) -> f64 { (self.next() % 9) as f64 / 16.0 }
    fn gain(&mut self) -> f64 { (self.next() % 4) as f64 / 8.0 }
    fn chance(&mut self) -> bool { self.next() % 8 < 5 }
}

fn random_rel(g: &mut Rng) -> DeclaredRelations {
    let n = g.usize(3, 6);
    let m = g.usize(2, 8);
    let edges: Vec<(usize, usize)> = (0..m).map(|_| loop {
        let (s, t) = (g.usize(0, n - 1), g.usize(0, n - 1));
        if s != t { break (s, t); }
    }).collect();
    synthetic_feedback::declare_loops(DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m]))
}

fn random_seq(g: &mut Rng, n: usize, run_constant: bool) -> Vec<Registration> {
    let (k, regs) = (g.usize(2, 3), g.usize(3, 5));
    let mut x: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| g.eighth()).collect()).collect();
    let delta: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| g.delta()).collect()).collect();
    let off: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| g.delta()).collect()).collect();
    let lg: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| g.gain()).collect()).collect();
    let sg: Vec<f64> = (0..k).map(|_| g.gain()).collect();
    let mut out = Vec::new();
    for _ in 0..regs {
        let r = Registration::new(NodeField::new(x.clone()), delta.clone());
        out.push(if run_constant { r.with_run_constant(off.clone(), lg.clone(), sg.clone()) } else { r });
        for c in x.iter_mut() { for v in c.iter_mut() { if g.chance() { *v += g.eighth() / 2.0; } } }
    }
    out
}

fn check_all(inp: Inputs, pc: &PersistenceChange) -> (usize, usize, usize) {
    for x in &pc.relations { assert_eq!(reverse_relation(inp, x), Verdict::Match, "{x:?}"); }
    let mut nc = 0;
    for cs in [&pc.across_components, &pc.across_steps] {
        if let Ok(cs) = cs { for x in &cs.readings { assert_eq!(reverse_coupling(inp, x), Verdict::Match, "{x:?}"); nc += 1; } }
    }
    let mut na = 0;
    if let Ok(xs) = &pc.arrangement { for x in xs { assert_eq!(reverse_arrangement(inp, x), Verdict::Match, "{x:?}"); na += 1; } }
    (pc.relations.len(), nc, na)
}

#[test]
fn prop_b2_every_reading_reverse_checks_to_match_node_derived() {
    let (mut r, mut c, mut a) = (0, 0, 0);
    for seed in 0..1500u64 {
        let mut g = Rng(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ 0x1D1D);
        let rel = random_rel(&mut g);
        let rc = seed % 2 == 1;
        let seq = random_seq(&mut g, rel.n_nodes, rc);
        let pc = PersistenceChange::new(&EdgeSeries::from_registrations(&seq, &rel).unwrap());
        let n = check_all(Inputs::Nodes { seq: &seq, rel: &rel }, &pc);
        r += n.0; c += n.1; a += n.2;
    }
    assert!(r > 1000 && c > 1000 && a > 1000, "vacuous: {r} {c} {a}");
}

#[test]
fn prop_b2_every_reading_reverse_checks_to_match_edge_registered() {
    let (mut c, mut a) = (0, 0);
    for seed in 0..1500u64 {
        let mut g = Rng(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ 0x2E2E);
        let (m, k, regs) = (g.usize(2, 6), g.usize(2, 3), g.usize(3, 5));
        let mut v: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| g.eighth()).collect()).collect();
        let d: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| g.delta()).collect()).collect();
        let off: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| if seed % 2 == 1 { g.delta() } else { 0.0 }).collect()).collect();
        let (mut aa, mut dd) = (Vec::new(), Vec::new());
        for _ in 0..regs {
            aa.push(v.clone()); dd.push(d.clone());
            for cv in v.iter_mut() { for x in cv.iter_mut() { if g.chance() { *x += g.eighth() / 2.0; } } }
        }
        let gain: Vec<Vec<f64>> = vec![vec![0.125; m]; k];
        let s = EdgeSeries::from_edge_values(aa.clone(), dd.clone()).unwrap().with_run_constant(off.clone(), gain);
        let n = check_all(Inputs::Edges { a: &aa, delta: &dd, offset: &off }, &PersistenceChange::new(&s));
        c += n.1; a += n.2;
    }
    assert!(c > 1000 && a > 1000, "vacuous: {c} {a}");
}

// ── Tampering is a MISMATCH ────────────────────────────────────────────────

fn worked() -> (DeclaredRelations, Vec<Registration>) {
    let rel = DeclaredRelations::from_edges_with_provenance(4, vec![(0, 1), (1, 2), (2, 3), (0, 1)],
        vec![EdgeProvenance::Continuation; 4]);
    let seq = (0..3).map(|t| {
        let t = t as f64;
        Registration::new(NodeField::new(vec![vec![5.0, 3.0 + 0.5 * t, 2.0, 0.0], vec![1.0, 2.0, 4.0 + 0.25 * t * t, 1.0]]),
            vec![vec![0.0625; 4]; 2])
    }).collect();
    (rel, seq)
}

fn quantity(v: Verdict) -> &'static str {
    match v { Verdict::Mismatch { quantity, .. } => quantity, Verdict::Match => "MATCH" }
}

#[test]
fn b2_a_tampered_relation_reading_is_a_mismatch() {
    let (rel, seq) = worked();
    let pc = PersistenceChange::new(&EdgeSeries::from_registrations(&seq, &rel).unwrap());
    let inp = Inputs::Nodes { seq: &seq, rel: &rel };
    let x = pc.relations[1];
    assert_eq!(reverse_relation(inp, &x), Verdict::Match);
    assert_eq!(quantity(reverse_relation(inp, &RelationChange { change: x.change + 0.0625, ..x })), "change");
    assert_eq!(quantity(reverse_relation(inp, &RelationChange { half_width: x.half_width * 2.0, ..x })), "half_width");
    let flipped = if x.reading == Reading::Freedom { Reading::Structure } else { Reading::Freedom };
    assert_eq!(quantity(reverse_relation(inp, &RelationChange { reading: flipped, ..x })), "reading");
    assert_eq!(quantity(reverse_relation(inp, &RelationChange { step: 2, ..x })), "step");
    assert_eq!(quantity(reverse_relation(inp, &RelationChange { edge: 9, ..x })), "edge");
}

#[test]
fn b2_a_coupling_of_a_declared_identity_or_of_an_edge_that_never_moved_is_a_mismatch() {
    let (rel, seq) = worked();
    let pc = PersistenceChange::new(&EdgeSeries::from_registrations(&seq, &rel).unwrap());
    let inp = Inputs::Nodes { seq: &seq, rel: &rel };
    let cs = pc.across_components.unwrap();
    let x = cs.readings[0];
    assert_eq!(reverse_coupling(inp, &x), Verdict::Match);
    assert_eq!(quantity(reverse_coupling(inp, &ChangeCoupling { minor: x.minor + 1.0, ..x })), "minor");
    // Edges 0 and 3 are parallel: their coupling is an identity, never a reading.
    assert_eq!(quantity(reverse_coupling(inp, &ChangeCoupling { e: 0, f: 3, ..x })),
        "pair admissible (not a declared identity, PC-D4)");
    // Edge 0 (0 → 1) does not change in component 1 at either step (loci 0
    // and 1 hold there), so it enters no component-1 coupling across steps.
    let none_moved = ChangeCoupling { e: 0, f: 2, over: ReadingPair::Steps { component: 1, step: 0 }, ..x };
    assert_eq!(quantity(reverse_coupling(inp, &none_moved)), "edges entered (change resolved in either reading, PC1)");
}

#[test]
fn b2_a_tampered_arrangement_reading_is_a_mismatch() {
    let (rel, seq) = worked();
    let pc = PersistenceChange::new(&EdgeSeries::from_registrations(&seq, &rel).unwrap());
    let inp = Inputs::Nodes { seq: &seq, rel: &rel };
    let xs = pc.arrangement.unwrap();
    let x = xs[0];
    assert_eq!(reverse_arrangement(inp, &x), Verdict::Match);
    let other = match x.before { AtDelta::Resolved(Sign::Positive) => AtDelta::Resolved(Sign::Negative), _ => AtDelta::Resolved(Sign::Positive) };
    assert_eq!(quantity(reverse_arrangement(inp, &ArrangementStep { before: other, ..x })), "before");
}

#[test]
fn b2_inputs_other_than_those_declared_are_a_mismatch() {
    // The same reading checked against a δ wider than the one declared.
    let (rel, seq) = worked();
    let pc = PersistenceChange::new(&EdgeSeries::from_registrations(&seq, &rel).unwrap());
    let wider: Vec<Registration> = seq.iter().map(|r| Registration::new(r.x.clone(), vec![vec![0.125; 4]; 2])).collect();
    let x = pc.relations[1];
    assert_eq!(quantity(reverse_relation(Inputs::Nodes { seq: &wider, rel: &rel }, &x)), "half_width");
}
