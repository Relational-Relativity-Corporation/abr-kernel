//! §2.10 (Proposed, 73rd): the window and the present reading. Held readings
//! over the window (the whole field) as constraints on the next change of each
//! relation; the freedom they leave, k·m − rank, the rank read through
//! minors exactly; the coherence of a candidate next registration (every held
//! reading stays held); the anticipated set (the coherent candidates).
//!
//! What is shown: (1) on every enumerated whole field, the held constraints
//! are exactly the field's own PC1 readings held at every step and the G6
//! balances held at every step, and the freedom remaining is k·m less the
//! rank, computed independently by integer elimination, never k·m less the
//! count of constraints (implied constraints occur and are reported);
//! (2) a wider window holds no more than a narrower one, and the freedom it
//! leaves is no smaller; (3) a candidate is coherent exactly when every held
//! constraint, read at the appended step with its bound, stays held,
//! including changes within the bound that are not zero; (4) a declared field
//! worked by hand: the anticipated set, the readings each other candidate
//! breaks, and the forward check on an arrival that ends a held balance.
//!
//! [IMPL] SYNTHETIC FIXTURES: dyadic values, so every sum and product is
//! exact. They establish that the forms compute as stated, nothing empirical:
//! not that any system's next step is anticipated by them.
//! Not evaluated (named gaps): constraints coupling relations or components
//! through registered coefficients (PC2 coupled change, G3 phase lock;
//! OC-WIN-1); projection beyond one step; energy is not part of this reading
//! (the carrier registered on these fields is read by the field, not
//! constrained here); the quadratic held length (72nd).

#[path = "support/synthetic_whole_field.rs"]
mod support;
use support::*;

use abr_kernel::coherence::{anticipated, coherence, window_reading, Candidate, HeldKind, NotEvaluated, WindowReading};
use abr_kernel::geometry::{step_held_exchange, DeclaredDirection};
use abr_kernel::persistence_change::{EdgeSeries, Reading};
use abr_kernel::relational_field::RelationalField;

const M: usize = 3;

fn triangle() -> abr_kernel::operators::DeclaredRelations { rel(3, vec![(0, 1), (1, 2), (0, 2)]) }

fn field_of(a: &[Vec<Vec<f64>>], d: f64) -> RelationalField {
    let (n, k) = (a.len(), a[0].len());
    let series = EdgeSeries::from_edge_values_synthetic(a.to_vec(), vec![vec![vec![d; M]; k]; n]).unwrap();
    whole_of_relations(&triangle(), series, &relation_carrier(vec![vec![1.0; M]; n], vec![vec![0.25; M]; n]))
}

fn dir(n: Vec<f64>) -> DeclaredDirection { DeclaredDirection::new(n, "SYNTHETIC: direction declared for the test") }

/// An enumerated field on the triangle: per relation, a declared mode of change
/// (held; balance held against n0 with components moving; moving; mixed), the
/// directions declared (none, n0, or n0 and n1).
struct Enumerated { a: Vec<Vec<Vec<f64>>>, d: f64, dirs: Vec<DeclaredDirection>, k: usize }

fn enumerated(r: &mut Seeds) -> Enumerated {
    let k = r.usize(2, 4);
    let regs = r.usize(3, 5);
    let d = r.delta(0.25);
    let mut n0: Vec<f64> = (0..k).map(|_| r.int(-3, 3) as f64).collect();
    let star = r.usize(0, k - 1);
    n0[star] = if r.selects(4) { 1.0 } else { -1.0 };
    let n1: Vec<f64> = { let mut v: Vec<f64> = (0..k).map(|_| r.int(-2, 2) as f64).collect(); if v.iter().all(|&x| x == 0.0) { v[0] = 1.0; } v };
    let modes: Vec<usize> = (0..M).map(|_| r.usize(0, 3)).collect();
    let mut a = vec![(0..k).map(|_| (0..M).map(|_| r.eighth(-3.0, 3.0)).collect::<Vec<f64>>()).collect::<Vec<_>>()];
    for _ in 1..regs {
        let prev = a.last().unwrap().clone();
        let mut next = prev.clone();
        for e in 0..M {
            let mut delta: Vec<f64> = (0..k).map(|_| match modes[e] {
                0 => r.int(-1, 1) as f64 * d / 2.0,                   // held, within the bound
                3 if r.selects(4) => 0.0,
                _ => { let v = r.int(1, 2) as f64; if r.selects(4) { v } else { -v } }
            }).collect();
            if modes[e] == 1 {
                // The balance against n0 held exactly: n0·Δ = 0, with n0[star] = ±1.
                let rest: f64 = (0..k).filter(|&c| c != star).map(|c| n0[c] * delta[c]).sum();
                delta[star] = -rest / n0[star];
            }
            for c in 0..k { next[c][e] = prev[c][e] + delta[c]; }
        }
        a.push(next);
    }
    let dirs = match r.usize(0, 2) { 0 => vec![], 1 => vec![dir(n0)], _ => vec![dir(n0), dir(n1)] };
    Enumerated { a, d, dirs, k }
}

/// Rank of integer rows by fraction-free elimination (independent of the
/// kernel's minors).
fn int_rank(mut rows: Vec<Vec<i128>>) -> usize {
    let cols = rows.first().map_or(0, |r| r.len());
    let mut rank = 0;
    for c in 0..cols {
        let Some(p) = (rank..rows.len()).find(|&i| rows[i][c] != 0) else { continue };
        rows.swap(rank, p);
        for i in 0..rows.len() {
            if i != rank && rows[i][c] != 0 {
                let (a, b) = (rows[rank][c], rows[i][c]);
                for j in 0..cols { rows[i][j] = rows[i][j] * a - rows[rank][j] * b; }
            }
        }
        rank += 1;
    }
    rank
}

/// The held set read from the field's own parts: PC1 from the field's freedom,
/// G6 from the held part on each step.
fn held_from_parts(f: &RelationalField, dirs: &[DeclaredDirection]) -> Vec<(usize, HeldKind)> {
    let pc = f.ground.freedom.as_ref().expect("freedom is read");
    let (k, steps) = (f.relations.k(), f.relations.n_steps());
    let mut out = Vec::new();
    for e in 0..M {
        for c in 0..k {
            if (0..steps).all(|s| pc.relations.iter().find(|x| x.step == s && x.edge == e && x.component == c).unwrap().reading == Reading::Structure) {
                out.push((e, HeldKind::Relation { component: c }));
            }
        }
    }
    for (di, n) in dirs.iter().enumerate() {
        for e in 0..M {
            if (0..steps).all(|s| !step_held_exchange(&f.relations, s, n).unwrap()[e].held.reading.is_resolved()) {
                out.push((e, HeldKind::Balance { direction: di }));
            }
        }
    }
    out
}

fn held_set(w: &WindowReading) -> Vec<(usize, HeldKind)> { w.held.iter().map(|h| (h.edge, h.kind)).collect() }

/// 1. Freedom remaining is k·m less the rank, never less the count.
#[test]
fn freedom_remaining_is_components_less_rank() {
    let implied_seen = std::sync::atomic::AtomicUsize::new(0);
    for_all("freedom remaining", 0x73_01, |r| {
        let x = enumerated(r);
        let f = field_of(&x.a, x.d);
        let w = window_reading(&f, &x.dirs).unwrap();
        let mut got = held_set(&w); got.sort_by_key(|h| format!("{h:?}"));
        let mut want = held_from_parts(&f, &x.dirs); want.sort_by_key(|h| format!("{h:?}"));
        assert_eq!(got, want, "the held constraints are the field's own held readings");
        let mut rank = 0;
        for rf in &w.relations {
            let rows: Vec<Vec<i128>> = rf.constraints.iter().map(|&i| w.held[i].row.iter().map(|&v| v as i128).collect()).collect();
            let rk = int_rank(rows);
            assert_eq!(rf.rank, Some(rk), "the rank of each relation's constraints");
            assert_eq!(rf.independent.len() + rf.implied.len(), rf.constraints.len(), "every constraint is reported");
            assert_eq!(rf.freedom_remaining, Some(x.k - rk));
            rank += rk;
            if !rf.implied.is_empty() { implied_seen.fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
        }
        assert_eq!(w.freedom_remaining, Some(x.k * M - rank));
        assert!(w.not_evaluated.contains(&NotEvaluated::CouplingConstraints));
        assert_eq!(x.dirs.is_empty(), w.not_evaluated.contains(&NotEvaluated::NoDeclaredDirection));
    });
    // The test can tell rank from count: implied constraints occur.
    assert!(implied_seen.into_inner() > 0, "no implied constraint enumerated: rank and count not told apart");
}

/// 2. A wider window holds no more, and leaves no less freedom.
#[test]
fn a_wider_window_holds_no_more() {
    let strictly = std::sync::atomic::AtomicUsize::new(0);
    for_all("wider window", 0x73_02, |r| {
        let x = enumerated(r);
        let wide = window_reading(&field_of(&x.a, x.d), &x.dirs).unwrap();
        let narrow = window_reading(&field_of(&x.a[..x.a.len() - 1], x.d), &x.dirs).unwrap();
        let n = held_set(&narrow);
        for h in held_set(&wide) { assert!(n.contains(&h), "held over the wider window, so held over the narrower"); }
        assert!(wide.freedom_remaining.unwrap() >= narrow.freedom_remaining.unwrap());
        if wide.held.len() < narrow.held.len() { strictly.fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
    });
    assert!(strictly.into_inner() > 0, "no field where the wider window holds fewer: the test cannot tell");
}

/// 3. A candidate is coherent exactly when every held constraint stays held at
/// the appended step, read with its bound.
#[test]
fn coherent_exactly_when_every_held_reading_stays_held() {
    let within_nonzero = std::sync::atomic::AtomicUsize::new(0);
    for_all("coherence", 0x73_03, |r| {
        let x = enumerated(r);
        let f = field_of(&x.a, x.d);
        let w = window_reading(&f, &x.dirs).unwrap();
        let last = x.a.last().unwrap().clone();
        let cands: Vec<Candidate> = (0..6).map(|i| {
            let delta: Vec<Vec<f64>> = (0..x.k).map(|_| (0..M).map(|_| match r.usize(0, 3) {
                0 => 0.0,
                1 => r.int(-1, 1) as f64 * x.d,          // within the bound 2δ
                _ => r.int(-2, 2) as f64,
            }).collect()).collect();
            let a = (0..x.k).map(|c| (0..M).map(|e| last[c][e] + delta[c][e]).collect()).collect();
            Candidate::repeatability(&format!("candidate {i}"), a, vec![vec![x.d; M]; x.k])
        }).collect();
        let readings = coherence(&f, &w, &x.dirs, &cands).unwrap();
        for (cand, rd) in cands.iter().zip(&readings) {
            let delta = |c: usize, e: usize| cand.a[c][e] - last[c][e];
            let expect: Vec<bool> = w.held.iter().map(|h| match h.kind {
                HeldKind::Relation { component } => delta(component, h.edge).abs() <= 2.0 * x.d,
                HeldKind::Balance { direction } => {
                    let n = &x.dirs[direction].n;
                    let v: f64 = (0..x.k).map(|c| n[c] * delta(c, h.edge)).sum();
                    let b: f64 = (0..x.k).map(|c| n[c].abs() * 2.0 * x.d).sum();
                    v.abs() <= b
                }
            }).collect();
            assert_eq!(rd.kept.iter().map(|k| k.kept).collect::<Vec<_>>(), expect);
            assert_eq!(rd.coherent, expect.iter().all(|&k| k));
            let nonzero_within = w.held.iter().zip(&expect).any(|(h, &k)| k && match h.kind {
                HeldKind::Relation { component } => delta(component, h.edge) != 0.0,
                HeldKind::Balance { .. } => false,
            });
            if rd.coherent && nonzero_within { within_nonzero.fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
        }
        assert_eq!(anticipated(&readings).len(), readings.iter().filter(|x| x.coherent).count());
    });
    assert!(within_nonzero.into_inner() > 0, "no coherent candidate with a change within the bound: exact zero not told apart");
}

/// 4. Worked by hand. Two components, n = (1, 1). Relation 0 held in both
/// components; relation 1 moving with its balance held, (+1, −1) each step;
/// relation 2 moving freely. δ = 1/8 on every entry.
#[test]
fn a_declared_field_anticipates_the_coherent_continuations() {
    // a[t][c][e]
    let a = vec![
        vec![vec![1.0, 0.0, 0.0], vec![2.0, 0.0, 0.0]],
        vec![vec![1.0, 1.0, 1.0], vec![2.0, -1.0, 2.0]],
        vec![vec![1.0, 2.0, -1.0], vec![2.0, -2.0, 3.0]],
    ];
    let f = field_of(&a, 0.125);
    let dirs = [dir(vec![1.0, 1.0])];
    let w = window_reading(&f, &dirs).unwrap();
    assert_eq!(held_set(&w), vec![
        (0, HeldKind::Relation { component: 0 }), (0, HeldKind::Relation { component: 1 }),
        (0, HeldKind::Balance { direction: 0 }), (1, HeldKind::Balance { direction: 0 }),
    ]);
    // Relation 0: three held readings, rank 2, the balance implied; relation 1:
    // rank 1; relation 2: none. Freedom remaining 0 + 1 + 2 = 3 of 6.
    assert_eq!(w.relations[0].implied, vec![2]);
    assert_eq!(w.relations.iter().map(|r| r.freedom_remaining.unwrap()).collect::<Vec<_>>(), vec![0, 1, 2]);
    assert_eq!(w.freedom_remaining, Some(3));

    let cand = |label: &str, a: Vec<Vec<f64>>| Candidate::repeatability(label, a, vec![vec![0.125; M]; 2]);
    let cands = [
        cand("continue", vec![vec![1.0, 3.0, 5.0], vec![2.0, -3.0, -7.0]]),   // relation 2 free to go anywhere
        cand("exchange reversed", vec![vec![1.0, 1.0, 0.0], vec![2.0, -1.0, 0.0]]),
        cand("balance broken", vec![vec![1.0, 3.0, 0.0], vec![2.0, -1.0, 0.0]]),
        cand("held relation moves", vec![vec![2.0, 3.0, 0.0], vec![1.0, -3.0, 0.0]]),
    ];
    let rd = coherence(&f, &w, &dirs, &cands).unwrap();
    assert_eq!(anticipated(&rd), vec!["continue", "exchange reversed"]);
    let broken = |i: usize| rd[i].kept.iter().filter(|k| !k.kept).map(|k| k.constraint).collect::<Vec<_>>();
    assert_eq!(broken(2), vec![3], "the balance of relation 1 ends: n·Δ = 2 against 1/2");
    assert_eq!((rd[2].kept[3].value, rd[2].kept[3].bound), (2.0, 0.5));
    assert_eq!(broken(3), vec![0, 1], "relation 0 moves in both components; its balance stays held");

    // The forward check: the arrival is read the same way. An arrival that ends
    // a held balance is a finding about the declaration, reported by constraint.
    let arrival = coherence(&f, &w, &dirs, &cands[2..3]).unwrap();
    assert!(!arrival[0].coherent);
}
