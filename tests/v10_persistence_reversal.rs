//! Kernel V10.3: I-SC-D18, beginning, ending and reversal of the three
//! persistences (Proposed, Generator, 2026-11-23, 52nd; V10_README §2.6).
//!
//! Names (W16 (a), Origin, 52nd): **persistent imbalance**, the sign of Ξ(B(A))
//! held over the sequence (V1 sustained grade); **persistent turn sum**, the
//! state of a turn's turn sum held (K7); **persistent arrangement**,
//! the state of a minor m_ij(e, f) held (PC3, V8). Each is read per
//! registration as a sign at δ: resolved +, resolved −, or not resolved.
//!
//! (i)   **No persistence reverses without freedom.** Over a step, a reading
//!       of any of the three can go from resolved with one sign to resolved
//!       with the other only where PC1 reads freedom at some relation the
//!       reading is formed from (a non-zero coefficient of Ξ(B(·)); an edge
//!       of the walk; an entry of the minor). If every such relation holds,
//!       one field lies within the state bound of both registrations, and
//!       each reading's bound encloses it. It holds under every declared
//!       part of δ (Z1a): an offset widens the state bound and not the
//!       change bound; a node-derived per-locus gain enters the change bound
//!       as γ·(|Δx| + δ_s + δ_{s+1}) ≤ γ·(|x_s| + δ_s) + γ·(|x_{s+1}| +
//!       δ_{s+1}), its share of the two state bounds; an edge-registered gain
//!       widens a reading's bound and not PC1's.
//! (ii)  **Beginning and ending need none, at δ.**
//! (iii) **Persistent turn sum and persistent arrangement are independent.**
//!
//! **Whole fields (Origin, 53rd; Df6, W11).** The imbalance and arrangement
//! claims are read on whole relational fields, with the carrier declared,
//! from the parts the field reports: its Ξ(B(A)) at each registration
//! (`spatial[t].passage`), checked against its own sustained grade; its
//! PC1 and PC3; and with energy and every connection grade evaluated beside
//! them. Turn sum is a test only on edge-registered values (PC-D4), where the
//! kernel forms no whole field: those tests read every PC level and the
//! kernel's own turn sums there, and name what is not evaluated (option (b);
//! OC-WF-1). *Was (52nd):* Ξ(B(A)) and the closure recomputed in the test,
//! on fields with no carrier, and PC1 computed directly.
//!
//! [IMPL] SYNTHETIC FIXTURES: dyadic values, so every sum and product is
//! exact. They establish that the forms compute as stated, nothing
//! empirical.

#[path = "support/synthetic_whole_field.rs"]
mod wf;
use abr_kernel::exact::Expansion;
use abr_kernel::ground_variables::{history, sign_at_delta_exact, AtDelta, History, Sign};
use abr_kernel::operators::{passage_of_accumulation_coefficients, passage_of_accumulation_half_width, DeclaredRelations};
use abr_kernel::persistence_change::*;
use std::sync::Mutex;
use wf::{for_all, enumerated_declared, enumerated_rel, rel, whole, Declared, Seeds, Whole};

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

fn is_freedom(r: Reading) -> bool { r == Reading::Freedom }

// ═══════════════════════════════════════════════════════════════════════════
// Persistent imbalance (V1 sustained grade, K8 on B(A)), on whole fields
// ═══════════════════════════════════════════════════════════════════════════

/// The field's imbalance states: [t][c][e], from its own Ξ(B(A)) and the
/// bound Σ_succ δB + Σ_pred δB on the state bounds it reads. `None` where the
/// edge has no neighbour (IV-3).
fn imbalance_states(w: &Whole) -> Vec<Vec<Vec<Option<AtDelta<Sign>>>>> {
    let rel = &w.rel;
    w.field.spatial.iter().enumerate().map(|(t, forms)| {
        let hw = passage_of_accumulation_half_width(&w.series.da[t], rel);
        forms.passage.iter().enumerate().map(|(c, xi)| (0..rel.n_edges()).map(|e| {
            if rel.succ(e).is_empty() && rel.pred(e).is_empty() { return None; }
            Some(sign_at_delta_exact(&Expansion::of(xi[e]), &Expansion::of(hw[c][e])))
        }).collect()).collect()
    }).collect()
}

/// The states above are the field's own sustained grade: an edge is held
/// exactly when some component's sign is held resolved, and reorganizing
/// exactly when it is not held and some component's state changes.
fn agrees_with_the_sustained_grade(w: &Whole, st: &[Vec<Vec<Option<AtDelta<Sign>>>>]) {
    let Ok(sus) = &w.connection().sustained else { return };
    for e in 0..w.rel.n_edges() {
        if st[0][0][e].is_none() { assert!(sus.not_formable.contains(&e)); continue; }
        let hs: Vec<History> = (0..w.declared.k()).map(|c| history(&st.iter().map(|t| t[c][e].unwrap()).collect::<Vec<_>>())).collect();
        let held = hs.iter().any(|h| matches!(h, History::HeldResolved(_)));
        let reorg = !held && hs.iter().any(|h| matches!(h, History::Reorganizing { .. }));
        assert_eq!(sus.held_edges.contains(&e), held, "held, as the field reads it");
        assert_eq!(sus.reorganizing_edges.contains(&e), reorg, "reorganizing, as the field reads it");
    }
}

#[test]
fn d18_persistent_imbalance_reverses_only_with_freedom_among_its_relations() {
    let tally = Mutex::new(Tally::default());
    for_all("d18_imbalance", 0xD18, |r| {
        let rel = enumerated_rel(r);
        let (k, regs) = (r.usize(2, 3), r.usize(2, 4));
        let z = r.selects(4);
        let w = whole(&rel, enumerated_declared(r, &rel, k, regs, 0.25, z, z));
        let st = imbalance_states(&w);
        agrees_with_the_sustained_grade(&w, &st);
        let mut g = tally.lock().unwrap();
        for s in 0..regs - 1 {
            for c in 0..k {
                for e in 0..rel.n_edges() {
                    let (Some(before), Some(after)) = (st[s][c][e], st[s + 1][c][e]) else { continue };
                    let wc = passage_of_accumulation_coefficients(&rel, e, -1);
                    let moved: Vec<bool> = wc.iter().enumerate().filter(|(_, &q)| q != 0).map(|(f, _)| is_freedom(w.pc1(s, c, f))).collect();
                    g.add(before, after, &moved);
                }
            }
        }
    });
    tally.into_inner().unwrap().require("persistent imbalance");
}

#[test]
fn d18_worked_an_imbalance_ends_with_every_relation_held() {
    // Chain 0 → 1 → 2, a structural component and the carrier, δ = 1/16 at
    // each locus. Ξ(B(A)) at edge 0 is A[e1] (its one successor; no
    // predecessor, so the edge is boundary-conditioned, A-3), bound
    // δA[e1] = 1/8. In the structure, locus 2 moves by 1/4, exactly δΔ, so
    // PC1 holds at every relation. A[e1] goes from 3/8 (resolved) to 1/8 (not
    // resolved): the persistence ends with nothing resolved to change. The
    // carrier holds throughout; energy is read beside it.
    let values = vec![vec![vec![0.0, 0.0, -0.375], vec![1.0, 0.0, 0.0]], vec![vec![0.0, 0.0, -0.125], vec![1.0, 0.0, 0.0]]];
    let w = whole(&rel(3, vec![(0, 1), (1, 2)]), Declared::repeatability(values, vec![vec![1.0 / 16.0; 3]; 2]));
    let st = imbalance_states(&w);
    agrees_with_the_sustained_grade(&w, &st);
    assert_eq!(passage_of_accumulation_coefficients(&w.rel, 0, -1), vec![0, 1]);
    assert_eq!((st[0][0][0], st[1][0][0]), (Some(AtDelta::Resolved(Sign::Positive)), Some(AtDelta::NotResolved)));
    assert_eq!((w.pc1(0, 0, 0), w.pc1(0, 0, 1)), (Reading::Structure, Reading::Structure));
    assert!(w.connection().sustained.as_ref().unwrap().boundary_conditioned.contains(&0));
    assert!(w.field.ground.energy.as_ref().unwrap().share_resolved > 0.0);
}

// ═══════════════════════════════════════════════════════════════════════════
// Persistent arrangement (PC3, V8), on whole fields
// ═══════════════════════════════════════════════════════════════════════════

fn tally_arrangement(pc: &PersistenceChange, g: &mut Tally) {
    let pc1 = |st: usize, c: usize, e: usize| pc.relations.iter().find(|x| x.step == st && x.component == c && x.edge == e).unwrap().reading;
    let Ok(arr) = &pc.arrangement else { return };
    for x in arr {
        let moved: Vec<bool> = [(x.i, x.e), (x.j, x.e), (x.i, x.f), (x.j, x.f)].iter().map(|&(c, e)| is_freedom(pc1(x.step, c, e))).collect();
        g.add(x.before, x.after, &moved);
    }
}

#[test]
fn d18_persistent_arrangement_reverses_only_with_freedom_at_its_entries() {
    // The field's own PC3 states and PC1 readings. A PC3 crossing is the
    // reversal.
    let tally = Mutex::new(Tally::default());
    for_all("d18_arrangement", 0xD18, |r| {
        let rel = enumerated_rel(r);
        let (k, regs) = (r.usize(2, 3), r.usize(2, 4));
        let z = r.selects(4);
        let w = whole(&rel, enumerated_declared(r, &rel, k, regs, 0.25, z, z));
        tally_arrangement(w.freedom(), &mut tally.lock().unwrap());
    });
    tally.into_inner().unwrap().require("persistent arrangement");
}

// ═══════════════════════════════════════════════════════════════════════════
// Edge-registered series (option (b), 53rd): persistent turn sum, and the
// arrangement with gains of every kind
// ═══════════════════════════════════════════════════════════════════════════

/// What is not evaluated on an edge-registered series, and why (W11;
/// whole-field evaluation there is OC-WF-1).
const NOT_EVALUATED_ON_EDGE_REGISTERED: [(&str, &str); 3] = [
    ("energy (V3)", "the carrier is declared on loci; an edge-registered series has none"),
    ("connection: identity, proportional, sustained (persistent imbalance)", "read on node-derived fields only (I-SC-D15 (v))"),
    ("the relational field (A, B, Ξ and their persistence forms)", "RelationalField is formed from node values (OC-WF-1)"),
];

/// Edge-registered series with declared offsets and, on some, gains of every
/// kind. Every PC level is read on it (`PersistenceChange::new`).
fn edge_series(r: &mut Seeds, k: usize, m: usize, regs: usize) -> EdgeSeries {
    let mut a = vec![(0..k).map(|_| (0..m).map(|_| r.eighth(-3.0, 3.0)).collect::<Vec<_>>()).collect::<Vec<_>>()];
    for _ in 1..regs {
        let p = a.last().unwrap().clone();
        a.push(p.iter().map(|row| row.iter().map(|&x| if r.selects(4) { x + r.eighth(-1.0, 1.0) } else { x }).collect()).collect());
    }
    let rep: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| r.delta(0.25)).collect()).collect();
    let da = (0..regs).map(|_| rep.clone()).collect();
    let offset: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| if r.selects(3) { r.delta(0.25) } else { 0.0 }).collect()).collect();
    let g = |r: &mut Seeds| if r.selects(2) { 1.0 / 16.0 } else { 0.0 };
    let gains = if r.selects(4) {
        EdgeGains { component: (0..k).map(|_| g(r)).collect(), edge: (0..m).map(|_| g(r)).collect(),
            independent: (0..k).map(|_| (0..m).map(|_| g(r)).collect()).collect() }
    } else { EdgeGains::none(k, m) };
    EdgeSeries::from_edge_values_synthetic(a, da).unwrap().with_run_constant(offset, gains)
}

/// The diamond 0→1, 1→2, 0→2, 0→3, 3→2 and its three simple turns.
fn diamond() -> (DeclaredRelations, Vec<Vec<(usize, i8)>>) {
    (rel(4, vec![(0, 1), (1, 2), (0, 2), (0, 3), (3, 2)]),
     vec![vec![(0, 1), (1, 1), (2, -1)], vec![(3, 1), (4, 1), (2, -1)], vec![(0, 1), (1, 1), (4, -1), (3, -1)]])
}

/// The kernel's turn sum state: its exact verdict, with the sign of its value.
fn turn_sum_state(s: &EdgeSeries, rel: &DeclaredRelations, walk: &[(usize, i8)], t: usize, c: usize) -> AtDelta<Sign> {
    if turn_sum_resolved(s, rel, walk, t, c).unwrap() {
        AtDelta::Resolved(if turn_sum(s, rel, walk, t, c).unwrap().0 > 0.0 { Sign::Positive } else { Sign::Negative })
    } else { AtDelta::NotResolved }
}

#[test]
fn d18_persistent_closure_reverses_only_with_freedom_on_its_walk() {
    assert_eq!(NOT_EVALUATED_ON_EDGE_REGISTERED.len(), 3);
    let (rel, walks) = diamond();
    let tally = Mutex::new(Tally::default());
    for_all("d18_closure", 0xD18, |r| {
        let (k, regs) = (r.usize(2, 3), r.usize(2, 4));
        let s = edge_series(r, k, rel.n_edges(), regs);
        let pc = PersistenceChange::new(&s);
        assert!(pc.gaps().len() <= 3, "every PC level is read or named");
        let pc1 = |st: usize, c: usize, e: usize| pc.relations.iter().find(|x| x.step == st && x.component == c && x.edge == e).unwrap().reading;
        let mut g = tally.lock().unwrap();
        for st in 0..regs - 1 {
            for c in 0..k {
                for w in &walks {
                    let moved: Vec<bool> = w.iter().map(|&(e, _)| is_freedom(pc1(st, c, e))).collect();
                    g.add(turn_sum_state(&s, &rel, w, st, c), turn_sum_state(&s, &rel, w, st + 1, c), &moved);
                }
            }
        }
    });
    tally.into_inner().unwrap().require("persistent turn sum");
}

#[test]
fn d18_persistent_arrangement_on_edge_registered_series_with_gains() {
    let tally = Mutex::new(Tally::default());
    for_all("d18_arrangement_edge", 0xD18, |r| {
        let rel = enumerated_rel(r);
        let (k, regs) = (r.usize(2, 3), r.usize(2, 4));
        tally_arrangement(&PersistenceChange::new(&edge_series(r, k, rel.n_edges(), regs)), &mut tally.lock().unwrap());
    });
    tally.into_inner().unwrap().require("persistent arrangement, edge-registered");
}

#[test]
fn d18_iii_closure_and_arrangement_persist_independently() {
    // Over one step: turn sum persistent when every walk's state, in every
    // component, is the same at both registrations; arrangement persistent
    // when every PC3 reading is structure (with at least one evaluated).
    let (rel, walks) = diamond();
    let seen = Mutex::new([[0usize; 2]; 2]);
    for_all("d18_iii", 0xD18, |r| {
        let k = r.usize(2, 3);
        let s = edge_series(r, k, rel.n_edges(), 2);
        let pc = PersistenceChange::new(&s);
        let arr = pc.arrangement.as_ref().unwrap();
        if arr.is_empty() { return; }
        let held_arr = arr.iter().all(|x| x.reading == Reading::Structure);
        let held_clo = (0..k).all(|c| walks.iter().all(|w| turn_sum_state(&s, &rel, w, 0, c) == turn_sum_state(&s, &rel, w, 1, c)));
        seen.lock().unwrap()[held_clo as usize][held_arr as usize] += 1;
    });
    let g = seen.into_inner().unwrap();
    assert!(g.iter().flatten().all(|&n| n > 0), "turn sum × arrangement (changes/persists): {g:?}");
}
