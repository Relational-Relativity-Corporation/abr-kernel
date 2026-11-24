//! §2.9 G6 (Proposed, 70th): a relation against a declared direction n over
//! its components. The held part n·u (inner product, term by term, one exact
//! sum: the balance) and the exchange per plane n_i·u_j − n_j·u_i (the area of
//! n and u). Read from a whole field (`field.relations`), with every variable
//! evaluated together.
//!
//! What is shown: (1) nothing is dropped: on every enumerated whole field, at
//! every registration and over every step, |n|²·u_c = (n·u)·n_c +
//! Σ_j n_j·(n_j·u_c − n_c·u_j) exactly, and the held part is exactly the sum
//! of its terms; (2) a change with the total held and carrier moved between
//! components reads the held part held at δ and the exchange resolved, with
//! the sense of the move; a change with no exchange reads the reverse; (3) the
//! exchange against n is minus G1's area against a relation registered equal
//! to n, so zero exchange is shared direction with n (the proportional grade).
//!
//! [IMPL] SYNTHETIC FIXTURES: dyadic values, so every sum and product is
//! exact. They establish that the forms compute as stated, nothing empirical.
//! Not evaluated (named gaps): G6 is not part of the whole field's report
//! (OC-GEO-1 (ii)), so it is not compared with the field's connection grades
//! (proportional, coupled change) or with the field's energy reading; that a
//! relation's components are carriers of energy is a declaration of the
//! application, not checked here; the quadratic form (held length, |u|² held,
//! 69th) is not evaluated; readings under a re-declared n or component scale
//! give different values by design.

#[path = "support/synthetic_whole_field.rs"]
mod support;
use support::*;

use abr_kernel::geometry::{angles, held_exchange, step_held_exchange, DeclaredDirection, HeldAndExchange};
use abr_kernel::ground_variables::{AtDelta, Sign};
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::relational_field::RelationalField;

fn triangle() -> abr_kernel::operators::DeclaredRelations { rel(3, vec![(0, 1), (1, 2), (0, 2)]) }

/// An edge-registered whole field on the triangle: a[t][c][e], δ for every entry.
fn field_of(a: Vec<Vec<Vec<f64>>>, d: f64) -> RelationalField {
    let (n, k) = (a.len(), a[0].len());
    let series = EdgeSeries::from_edge_values_synthetic(a, vec![vec![vec![d; 3]; k]; n]).unwrap();
    whole_of_relations(&triangle(), series, &relation_carrier(vec![vec![1.0; 3]; n], vec![vec![0.25; 3]; n]))
}

fn ex(x: &HeldAndExchange, i: usize, j: usize) -> f64 {
    x.exchange.iter().find(|p| p.0 == i && p.1 == j).unwrap().2.value
}

/// 1. Nothing is dropped. Enumerated over CASES seeded whole fields.
#[test]
fn held_part_and_exchange_account_for_every_component() {
    for_all("G6 decomposition", 0x70_01, |r| {
        let k = r.usize(2, 4);
        let regs = r.usize(2, 3);
        let a: Vec<Vec<Vec<f64>>> = (0..regs).map(|_| (0..k).map(|_| (0..3).map(|_| r.eighth(-3.0, 3.0)).collect()).collect()).collect();
        let mut n: Vec<f64> = (0..k).map(|_| r.int(-3, 3) as f64).collect();
        if n.iter().all(|&x| x == 0.0) { n[0] = 1.0; }
        let dir = DeclaredDirection::new(n.clone(), "SYNTHETIC: direction declared for the test");
        let f = field_of(a.clone(), r.delta(0.25));
        let n2: f64 = n.iter().map(|x| x * x).sum();
        let check = |x: &HeldAndExchange, u: &dyn Fn(usize) -> f64| {
            assert_eq!(x.held.value, x.terms.iter().map(|t| t.value).sum::<f64>());
            for c in 0..k {
                assert_eq!(x.terms[c].value, n[c] * u(c));
                let across: f64 = (0..k).filter(|&j| j != c)
                    .map(|j| n[j] * if j < c { ex(x, j, c) } else { -ex(x, c, j) }).sum();
                assert_eq!(n2 * u(c), x.held.value * n[c] + across);
            }
        };
        for t in 0..regs {
            for x in held_exchange(&f.relations, t, &dir).unwrap() { check(&x, &|c| a[t][c][x.e]); }
        }
        for s in 0..regs - 1 {
            for x in step_held_exchange(&f.relations, s, &dir).unwrap() { check(&x, &|c| a[s + 1][c][x.e] - a[s][c][x.e]); }
        }
    });
}

/// 2. Exchange with the total held, and change with no exchange. n = (1, 1).
/// e0 moves one unit from component 0 to component 1 per step; e1 grows
/// along n; e2 moves one unit from component 1 to component 0 per step.
#[test]
fn a_held_total_with_carrier_moved_reads_held_and_exchange_resolved() {
    let e0 = [(3.0, 1.0), (2.0, 2.0), (1.0, 3.0)];
    let e1 = [(1.0, 1.0), (2.0, 2.0), (3.0, 3.0)];
    let e2 = [(1.0, 3.0), (2.0, 2.0), (3.0, 1.0)];
    let a = (0..3).map(|t| vec![vec![e0[t].0, e1[t].0, e2[t].0], vec![e0[t].1, e1[t].1, e2[t].1]]).collect();
    let f = field_of(a, 0.0625);
    let n = DeclaredDirection::new(vec![1.0, 1.0], "SYNTHETIC: components in one unit, counting equally");
    for s in 0..2 {
        let g = step_held_exchange(&f.relations, s, &n).unwrap();
        let (x0, x1, x2) = (&g[0], &g[1], &g[2]);
        // e0: total held, carrier moved toward component 1.
        assert_eq!(x0.held.value, 0.0);
        assert_eq!(x0.held.reading, AtDelta::NotResolved);
        assert_eq!(ex(x0, 0, 1), 2.0);
        assert_eq!(x0.exchange[0].2.reading, AtDelta::Resolved(Sign::Positive));
        // e1: total grows, no exchange.
        assert_eq!(x1.held.value, 2.0);
        assert_eq!(x1.held.reading, AtDelta::Resolved(Sign::Positive));
        assert_eq!(ex(x1, 0, 1), 0.0);
        assert_eq!(x1.exchange[0].2.reading, AtDelta::NotResolved);
        // e2: total held, carrier moved toward component 0.
        assert_eq!(x2.held.reading, AtDelta::NotResolved);
        assert_eq!(x2.exchange[0].2.reading, AtDelta::Resolved(Sign::Negative));
    }
    // At a registration: e0 at t = 0 is (3, 1): held 4, exchange 1·1 − 1·3 = −2.
    let x = &held_exchange(&f.relations, 0, &n).unwrap()[0];
    assert_eq!((x.held.value, ex(x, 0, 1)), (4.0, -2.0));
}

/// 3. The exchange against n is minus G1's area against a relation
/// registered equal to n. Enumerated over CASES seeded whole fields; e2 is
/// registered equal to n at every registration.
#[test]
fn the_exchange_against_n_is_the_angle_to_a_relation_along_n() {
    for_all("G6 is G1 against n", 0x70_03, |r| {
        let n = [r.int(-3, 3) as f64, r.int(1, 3) as f64];
        let regs = r.usize(2, 3);
        let a: Vec<Vec<Vec<f64>>> = (0..regs).map(|_| (0..2).map(|c| vec![r.eighth(-3.0, 3.0), r.eighth(-3.0, 3.0), n[c]]).collect()).collect();
        let f = field_of(a, r.delta(0.25));
        let dir = DeclaredDirection::new(n.to_vec(), "SYNTHETIC: direction declared for the test");
        for t in 0..regs {
            let g6 = held_exchange(&f.relations, t, &dir).unwrap();
            let g1 = angles(&f.relations, t).unwrap();
            for e in 0..2 {
                let area = g1.iter().find(|x| x.e == e && x.f == 2 && x.i == 0 && x.j == 1).unwrap().area;
                assert_eq!(area.value, -ex(&g6[e], 0, 1));
                if let (AtDelta::Resolved(p), AtDelta::Resolved(q)) = (area.reading, g6[e].exchange[0].2.reading) {
                    assert_ne!(p, q, "opposite senses where both resolve");
                }
            }
        }
    });
}
