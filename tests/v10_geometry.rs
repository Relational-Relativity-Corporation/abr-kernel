//! §2.9, the geometry of relations (Proposed, 69th): G1 the angle between
//! two relations, G2 the step angle of one relation (oscillation), G3 the
//! relative phase of two relations over a step. Each read from a whole field
//! (`field.relations`), with every variable evaluated together.
//!
//! What is shown: (1) G1's area is the V8 minor, value and reading, on every
//! enumerated whole field formed from loci; (2) a relation turning a quarter
//! per step reads a resolved step area of one sense at every step, and the
//! same relation registered once per full turn reads held (G4, I-SC-D19 (ii));
//! (3) two relations turning together keep their relative angle (held), and
//! one turning against a fixed one drifts (resolved); (4) on every enumerated
//! field whose second registration is the first turned a quarter in the
//! plane, every relative phase is held with drift exactly 0, and every inner
//! product of a relation with its next state is exactly 0.
//!
//! [IMPL] SYNTHETIC FIXTURES: dyadic values, so every sum and product is
//! exact. They establish that the forms compute as stated, nothing empirical.
//! Not evaluated: G4 beyond the worked case; readings under a re-declared
//! component scale (I-SC-D15), which give different values by design.

#[path = "support/synthetic_whole_field.rs"]
mod support;
use support::*;

use abr_kernel::geometry::{angles, relative_phases, step_angles};
use abr_kernel::ground_variables::{minor_pattern, AtDelta, Sign};
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::relational_field::RelationalField;

/// 1. G1's area is the V8 minor on whole fields formed from loci.
#[test]
fn the_area_between_two_relations_is_the_minor() {
    for_all("G1 area is the minor", 0x69_01, |r| {
        let rel = enumerated_rel(r);
        let k = r.usize(3, 4);
        let regs = r.usize(2, 3);
        let z1a = r.selects(4);
        let d = enumerated_declared(r, &rel, k, regs, 0.5, z1a, false);
        let w = whole(&rel, d);
        for (t, reg) in w.field.registrations.iter().enumerate() {
            let g = angles(&w.field.relations, t).unwrap();
            let mp = minor_pattern(reg, &rel).unwrap();
            for &(e, f, i, j, reading) in &mp.minors {
                let a = g.iter().find(|x| x.e == e && x.f == f && x.i == i && x.j == j).unwrap();
                assert_eq!(a.area.reading, reading);
            }
        }
    });
}

fn triangle() -> abr_kernel::operators::DeclaredRelations { rel(3, vec![(0, 1), (1, 2), (0, 2)]) }

/// An edge-registered whole field: a[t] = [[component 0 per edge], [component 1 per edge]].
fn field_of(a: Vec<[[f64; 3]; 2]>, d: f64) -> RelationalField {
    let n = a.len();
    let a: Vec<Vec<Vec<f64>>> = a.iter().map(|t| t.iter().map(|c| c.to_vec()).collect()).collect();
    let series = EdgeSeries::from_edge_values_synthetic(a, vec![vec![vec![d; 3]; 2]; n]).unwrap();
    whole_of_relations(&triangle(), series, &relation_carrier(vec![vec![1.0; 3]; n], vec![vec![0.25; 3]; n]))
}

/// 2. Oscillation, and the scale it is read at. Relation e0 turns a quarter
/// per step: (1, 0), (0, 1), (−1, 0), (0, −1), (1, 0). e1 and e2 are fixed.
#[test]
fn a_turning_relation_reads_one_sense_per_step_and_held_at_its_period() {
    let e0 = [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0), (1.0, 0.0)];
    let fine = field_of(e0.iter().map(|&(x, y)| [[x, 2.0, 0.0], [y, 0.0, 2.0]]).collect(), 0.0625);
    for s in 0..4 {
        let g = step_angles(&fine.relations, s).unwrap();
        let x = g.iter().find(|x| x.e == 0).unwrap();
        assert_eq!(x.area.value, 1.0);
        assert_eq!(x.area.reading, AtDelta::Resolved(Sign::Positive));
        assert_eq!(x.inner.value, 0.0);
        assert_eq!(x.inner.reading, AtDelta::NotResolved);
        // The fixed relations do not turn.
        for f in [1, 2] { assert_eq!(g.iter().find(|x| x.e == f).unwrap().area.value, 0.0); }
    }
    // The same relation registered once per full turn: held.
    let coarse = field_of(vec![[[1.0, 2.0, 0.0], [0.0, 0.0, 2.0]]; 2], 0.0625);
    let x = step_angles(&coarse.relations, 0).unwrap().into_iter().find(|x| x.e == 0).unwrap();
    assert_eq!(x.area.reading, AtDelta::NotResolved);
    assert_eq!(x.inner.reading, AtDelta::Resolved(Sign::Positive));
}

/// 3. Phase-locked and drifting. e0 and e1 turn a quarter per step together,
/// a quarter apart; e2 is fixed at (1, 1).
#[test]
fn relations_turning_together_hold_their_relative_angle_and_a_fixed_one_drifts() {
    let e0 = [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0)];
    let e1 = [(0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)];
    let f = field_of((0..3).map(|t| [[e0[t].0, e1[t].0, 1.0], [e0[t].1, e1[t].1, 1.0]]).collect(), 0.0625);
    for s in 0..2 {
        let g = relative_phases(&f.relations, s).unwrap();
        let p = |e: usize, f: usize| g.iter().find(|x| x.e == e && x.f == f).unwrap().drift;
        assert_eq!(p(0, 1).value, 0.0);
        assert_eq!(p(0, 1).reading, AtDelta::NotResolved);
        assert!(p(0, 2).reading.is_resolved(), "a relation turning against a fixed one drifts");
    }
}

/// 4. A quarter turn of every relation in the plane keeps every relative
/// angle exactly, and makes every relation's inner product with its next
/// state exactly 0. Enumerated over CASES seeded fields.
#[test]
fn a_common_quarter_turn_holds_every_relative_phase() {
    for_all("common quarter turn", 0x69_04, |r| {
        let t0: [[f64; 3]; 2] = [[r.eighth(-3.0, 3.0), r.eighth(-3.0, 3.0), r.eighth(-3.0, 3.0)],
                                 [r.eighth(-3.0, 3.0), r.eighth(-3.0, 3.0), r.eighth(-3.0, 3.0)]];
        let t1 = [[-t0[1][0], -t0[1][1], -t0[1][2]], t0[0]];
        let f = field_of(vec![t0, t1], r.delta(0.25));
        for p in relative_phases(&f.relations, 0).unwrap() {
            assert_eq!(p.drift.value, 0.0);
            assert_eq!(p.drift.reading, AtDelta::NotResolved);
        }
        for x in step_angles(&f.relations, 0).unwrap() {
            assert_eq!(x.inner.value, 0.0);
            assert!(x.area.value >= 0.0);
            if x.area.reading.is_resolved() { assert_eq!(x.area.reading, AtDelta::Resolved(Sign::Positive)); }
        }
    });
}
