//! Forward relational change and its pace, which depends on scale (Origin,
//! 2026-11-23, 60th; O1, V5; I-SC-D19, Proposed).
//!
//! Pace is counted, never timed (V5, restated 10th): for each relation e and
//! component c, Γ[e,c] is the number of declared steps with change resolved at
//! δ (PC1), read from the whole field (`PersistenceChange::evolution_counts`).
//! The scale of analysis has two declared parts: the resolution δ, and the
//! intersections (the registrations, ordered by T6). The tests read Γ from
//! whole fields and show:
//!   (i)  at fixed intersections, a finer δ never reads a slower pace:
//!        Γ under δ/2 is at least Γ under δ, at every relation and component,
//!        and on some fields it is strictly greater;
//!   (ii) at fixed δ, adding an intersection can make the pace slower or
//!        faster: a change resolved over one step can be unresolved over both
//!        of its parts, and a change that nets to nothing over one step can be
//!        resolved over both parts.
//! So pace is read at a declared scale and compared only at stated scales.
//!
//! [IMPL] SYNTHETIC FIXTURES: dyadic values, so every sum and product is
//! exact. They establish that the forms compute as stated, nothing empirical.
//! Not evaluated here: pace at the Primary Region (Part 7, OC-PACE-1).

#[path = "support/synthetic_whole_field.rs"]
mod support;
use support::*;

use std::sync::atomic::{AtomicUsize, Ordering};

fn gamma(w: &Whole) -> Vec<((usize, usize), usize)> {
    let mut g: Vec<((usize, usize), usize)> = w.freedom().evolution_counts().iter()
        .map(|x| ((x.edge, x.component), x.changed_steps)).collect();
    g.sort();
    g
}

/// (i) A finer δ never reads a slower pace, with every declared part of δ
/// (repeatability, and the Z1a offset and gain, halved together).
#[test]
fn a_finer_resolution_never_reads_a_slower_pace() {
    let strictly_faster = AtomicUsize::new(0);
    for_all("pace monotone in δ", 0x60_01, |r| {
        let rel = enumerated_rel(r);
        let k = r.usize(2, 3);
        let regs = r.usize(2, 5);
        let z1a = r.selects(4);
        let d = enumerated_declared(r, &rel, k, regs, 0.75, z1a, false);
        let half = |m: &Vec<Vec<f64>>| m.iter().map(|row| row.iter().map(|x| x / 2.0).collect()).collect::<Vec<Vec<f64>>>();
        let fine = Declared { values: d.values.clone(), delta: half(&d.delta), offset: half(&d.offset), gain: half(&d.gain) };
        let (coarse, finer) = (whole(&rel, d), whole(&rel, fine));
        let (gc, gf) = (gamma(&coarse), gamma(&finer));
        assert_eq!(gc.len(), gf.len());
        for (a, b) in gc.iter().zip(&gf) {
            assert_eq!(a.0, b.0);
            assert!(b.1 >= a.1, "relation {:?}: Γ {} at δ/2 below Γ {} at δ", a.0, b.1, a.1);
            if b.1 > a.1 { strictly_faster.fetch_add(1, Ordering::Relaxed); }
        }
    });
    // The test tells "never slower" apart from "unchanged by δ".
    assert!(strictly_faster.load(Ordering::Relaxed) > 0, "some field reads a faster pace at δ/2");
}

/// A chain 0 → 1 → 2; component 0 moves at locus 0 only, component 1 is the
/// carrier (held at 0). δ is the same everywhere.
fn chain(values0: &[f64], delta: f64) -> Whole {
    let rel = rel(3, vec![(0, 1), (1, 2)]);
    let values = values0.iter().map(|&v| vec![vec![v, 0.0, 0.0], vec![0.0; 3]]).collect();
    whole(&rel, Declared::repeatability(values, vec![vec![delta; 3]; 2]))
}

fn gamma_at(w: &Whole, e: usize, c: usize) -> usize {
    w.freedom().evolution_counts().iter().find(|x| x.edge == e && x.component == c).unwrap().changed_steps
}

/// (ii, a) Adding an intersection can make the pace slower. δ = 3/16 at every
/// locus gives the relation's change the bound 4δ = 3/4 (δ at both ends, at
/// both registrations): over one step the change 1 resolves; over its two
/// parts each change 1/2 does not.
#[test]
fn an_added_intersection_can_read_a_slower_pace() {
    let coarse = chain(&[0.0, 1.0], 0.1875);
    let fine = chain(&[0.0, 0.5, 1.0], 0.1875);
    assert_eq!(gamma_at(&coarse, 0, 0), 1);
    assert_eq!(gamma_at(&fine, 0, 0), 0);
}

/// (ii, b) Adding an intersection can make the pace faster: over one step the
/// change nets to 0; over its two parts the changes +1 and −1 each resolve
/// against 4δ = 1/2.
#[test]
fn an_added_intersection_can_read_a_faster_pace() {
    let coarse = chain(&[0.0, 0.0], 0.125);
    let fine = chain(&[0.0, 1.0, 0.0], 0.125);
    assert_eq!(gamma_at(&coarse, 0, 0), 0);
    assert_eq!(gamma_at(&fine, 0, 0), 2);
}
