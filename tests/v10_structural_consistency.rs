//! Kernel V10 — derived structural consistency, I-SC-D3 to I-SC-D8.
//! Mathematical results (Derived), stated in `derived_invariants.rs` Part II.
//! Each test checks one result on the V10 forms.
//!
//! [IMPL] SYNTHETIC FIXTURES: values are chosen to exercise each result. They
//! establish that the results hold for the forms as declared, nothing
//! empirical.

use abr_kernel::ground_variables::{
    minor, minor_change_bound, minor_half_width, sign_at_delta, step_preserves_sign, AtDelta, Registration,
};
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::persistence_change::*;
use abr_kernel::relational_field::RelationalField;

fn rel(n: usize, edges: Vec<(usize, usize)>) -> DeclaredRelations {
    let m = edges.len();
    DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m])
}

fn reg(data: Vec<Vec<f64>>, d: f64) -> Registration {
    let (k, n) = (data.len(), data[0].len());
    Registration::new(NodeField::new(data), vec![vec![d; n]; k])
}

/// Edge-registered series a[t][c][e], uniform δ.
fn edges(a: Vec<Vec<Vec<f64>>>, d: f64) -> EdgeSeries {
    let da = a.iter().map(|t| t.iter().map(|c| vec![d; c.len()]).collect()).collect();
    EdgeSeries::from_edge_values(a, da).unwrap()
}

fn all_couplings(s: &EdgeSeries) -> Vec<ChangeCoupling> {
    let mut v = coupling_across_components(s).unwrap().readings;
    v.extend(coupling_across_steps(s).unwrap().readings);
    v
}

// ── I-SC-D3: proportional state with a fixed ratio locks change ────────────
//
// If A[f] = λ·A[e] in every component at registrations s and s+1, then
// Δ[f] = λ·Δ[e], so every change minor of (e, f) is exactly zero, in both
// PC2 families. Connection in state with a fixed ratio implies connection in
// change.

#[test]
fn sc_d3_fixed_ratio_in_state_locks_change() {
    let lam = -2.5;
    let e: [[f64; 2]; 4] = [[1.0, 3.0], [2.0, 2.0], [4.0, -1.0], [3.5, 0.5]];
    let a: Vec<Vec<Vec<f64>>> = e.iter().map(|x| vec![vec![x[0], lam * x[0]], vec![x[1], lam * x[1]]]).collect();
    let c = all_couplings(&edges(a, 0.01));
    assert!(!c.is_empty());
    for x in &c {
        assert_eq!(x.minor, 0.0, "change minor must vanish exactly");
        assert_eq!(x.reading, Reading::Structure);
    }
}

// Corollary: a ratio that changes between registrations keeps the pair
// proportional in state at each registration, yet their change is free:
// μ = (λ' − λ)·(Δ_i[e]·A_j[e] − Δ_j[e]·A_i[e]).

#[test]
fn sc_d3_changing_ratio_is_connected_in_state_and_free_in_change() {
    let e0 = [1.0, 2.0];
    let e1 = [3.0, 1.0];
    let (l0, l1) = (2.0, 5.0);
    let a = vec![
        vec![vec![e0[0], l0 * e0[0]], vec![e0[1], l0 * e0[1]]],
        vec![vec![e1[0], l1 * e1[0]], vec![e1[1], l1 * e1[1]]],
    ];
    let s = edges(a, 0.01);
    // Proportional in state at each registration: the state minor is zero.
    for t in 0..2 {
        assert_eq!(minor(&s.a[t], 0, 1, 0, 1), 0.0);
    }
    // Free in change, with the predicted value.
    let c = coupling_across_components(&s).unwrap().readings;
    assert_eq!(c.len(), 1);
    let predicted = (l1 - l0) * ((e1[0] - e0[0]) * e0[1] - (e1[1] - e0[1]) * e0[0]);
    assert!((c[0].minor - predicted).abs() < 1e-12);
    assert_eq!(c[0].reading, Reading::Freedom);
}

// ── I-SC-D4: an offset locks change without proportionality in state ──────
//
// If A[f] = λ·A[e] + c with c fixed over the step, then Δ[f] = λ·Δ[e]: the
// change minors vanish, while the state minors equal λ·0 + (A_i[e]·c_j −
// A_j[e]·c_i) and are generally resolved. The coupled-change grade reads a
// connection that the proportional grade of V1 does not.

#[test]
fn sc_d4_offset_locks_change_without_proportional_state() {
    let (lam, c) = (1.5, [4.0, -3.0]);
    let e: [[f64; 2]; 3] = [[1.0, 2.0], [2.0, 2.5], [2.5, 4.0]];
    let a: Vec<Vec<Vec<f64>>> = e.iter()
        .map(|x| vec![vec![x[0], lam * x[0] + c[0]], vec![x[1], lam * x[1] + c[1]]]).collect();
    let s = edges(a, 0.01);
    // Not proportional in state: the state minor is resolved at every registration.
    for t in 0..3 {
        let m = minor(&s.a[t], 0, 1, 0, 1);
        assert!(sign_at_delta(m, minor_half_width(&s.a[t], &s.da[t], 0, 1, 0, 1)).is_resolved());
    }
    // Locked in change: the coupled-change grade holds the pair.
    let pc = PersistenceChange::new(&s);
    assert_eq!(coupled_change_pairs(&pc).unwrap(), vec![(0, 1)]);
}

// ── I-SC-D5: circulation over a step is exactly additive ───────────────────
//
// Ξ is linear in B and B is linear in A, so Ξ_{s+1} = Ξ_s + Ξ(B(Δ_s)) exactly.
// Persistence condition for the sustained grade (the circulation counterpart
// of I-SC-D1): the sign of Ξ[c][e] cannot change over a step where
// |Ξ(B(Δ))[c][e]| < |Ξ_s[c][e]|, because a sign changes only through zero.

fn evolving_field() -> RelationalField {
    // 0 → 1 → 2 → 3 with 1 → 3: interior loci 1 and 2.
    let r = rel(4, vec![(0, 1), (1, 2), (2, 3), (1, 3)]);
    let seq = (0..5).map(|t| {
        let t = t as f64;
        reg(vec![vec![4.0, 2.0 + 0.4 * t, 1.0 - 0.3 * t * t, 0.0],
                 vec![0.0, 1.0, 3.0 - 1.5 * t, 0.5 + 0.2 * t]], 0.01)
    }).collect();
    RelationalField::new(&r, seq, None).unwrap()
}

#[test]
fn sc_d5_circulation_is_additive_over_a_step() {
    let f = evolving_field();
    for s in 0..f.n_steps() {
        for (c, row) in f.spatial[s + 1].circulation.iter().enumerate() {
            for (e, xi_next) in row.iter().enumerate() {
                let sum = f.spatial[s].circulation[c][e] + f.persistence[s].circulation[c][e];
                assert!((xi_next - sum).abs() < 1e-12);
            }
        }
    }
}

#[test]
fn sc_d5_circulation_sign_persists_when_its_change_is_smaller() {
    let f = evolving_field();
    let (mut checked, mut crossed) = (0, 0);
    for s in 0..f.n_steps() {
        for c in 0..f.spatial[s].circulation.len() {
            for e in 0..f.spatial[s].circulation[c].len() {
                let (now, change) = (f.spatial[s].circulation[c][e], f.persistence[s].circulation[c][e]);
                let next = f.spatial[s + 1].circulation[c][e];
                if step_preserves_sign(now, change.abs()) {
                    assert_eq!(now.signum(), next.signum());
                    checked += 1;
                } else if now.signum() != next.signum() && next != 0.0 {
                    crossed += 1;
                }
            }
        }
    }
    assert!(checked > 0, "the condition must be exercised");
    assert!(crossed > 0, "the fixture must also contain a sign change where it fails");
}

// ── I-SC-D6: uniform positive rescaling holds every sign ───────────────────
//
// If A_{s+1} = κ·A_s with κ > 0 at every edge and component, every minor
// scales by κ² and every circulation by κ, so no sign changes: the
// arrangement (PC3) and the sustained grade are both held, while every
// non-zero relation changes (PC1 freedom). Change without reorganization.

#[test]
fn sc_d6_uniform_rescaling_holds_every_sign() {
    let r = rel(4, vec![(0, 1), (1, 2), (2, 3), (1, 3)]);
    let base = vec![vec![4.0, 2.0, 1.0, 0.0], vec![0.0, 1.0, 3.0, 0.5]];
    let kappa = 1.7;
    let scaled: Vec<Vec<f64>> = base.iter().map(|c| c.iter().map(|v| kappa * v).collect()).collect();
    let seq = vec![reg(base, 0.0), reg(scaled, 0.0)];
    let f = RelationalField::new(&r, seq.clone(), None).unwrap();
    for (c, row) in f.spatial[0].circulation.iter().enumerate() {
        for (e, &xi) in row.iter().enumerate() {
            assert!((f.spatial[1].circulation[c][e] - kappa * xi).abs() < 1e-12);
        }
    }
    let s = EdgeSeries::from_registrations(&seq, &r).unwrap();
    for x in arrangement_steps(&s).unwrap() {
        assert_eq!(x.reading, Reading::Structure, "no arrangement changes under uniform rescaling");
    }
    let m0 = minor(&s.a[0], 0, 1, 0, 1);
    assert!((minor(&s.a[1], 0, 1, 0, 1) - kappa * kappa * m0).abs() < 1e-12);
    assert!(relation_changes(&s).iter().any(|x| x.reading == Reading::Freedom));
}

// ── I-SC-D7: the declared sequence is part of the reading ──────────────────
//
// Compare two declarations of one record: registrations (s, s+1, s+2), and
// the same record with s+1 omitted. Half-widths: δΔ_whole = δA_s + δA_{s+2};
// δΔ_1 + δΔ_2 = δΔ_whole + 2·δA_{s+1}.
//   (a) Both halves resolved with the same sign ⇒ the whole is resolved with
//       that sign: |Δ_1 + Δ_2| = |Δ_1| + |Δ_2| > δΔ_1 + δΔ_2 ≥ δΔ_whole.
//   (b) The whole can be resolved while neither half is (δ accumulates at
//       the middle registration).
//   (c) Halves resolved with opposite signs can cancel to an unresolved whole.
// So F_δ is not monotone under omitting registrations: which registrations
// are declared is part of the reading (T6, Df6).

fn pc1(values: &[f64], d: f64) -> Vec<Reading> {
    let a = values.iter().map(|&v| vec![vec![v]]).collect();
    relation_changes(&edges(a, d)).iter().map(|x| x.reading).collect()
}

#[test]
fn sc_d7_same_sign_halves_resolve_the_whole() {
    assert_eq!(pc1(&[0.0, 1.0, 2.0], 0.4), vec![Reading::Freedom, Reading::Freedom]);
    assert_eq!(pc1(&[0.0, 2.0], 0.4), vec![Reading::Freedom]);
}

#[test]
fn sc_d7_whole_resolved_while_neither_half_is() {
    assert_eq!(pc1(&[0.0, 0.7, 1.4], 0.4), vec![Reading::Structure, Reading::Structure]);
    assert_eq!(pc1(&[0.0, 1.4], 0.4), vec![Reading::Freedom]);
}

#[test]
fn sc_d7_opposite_halves_cancel_to_an_unresolved_whole() {
    assert_eq!(pc1(&[0.0, 2.0, 0.1], 0.4), vec![Reading::Freedom, Reading::Freedom]);
    assert_eq!(pc1(&[0.0, 0.1], 0.4), vec![Reading::Structure]);
}

// ── I-SC-D8: a crossing needs change larger than both resolutions ──────────
//
// If the minor m_ij(e,f) is resolved with opposite signs at s and s+1 (a
// crossing at PC3), then |m_{s+1} − m_s| > hw_s + hw_{s+1}, and |Δm| is
// bounded by `minor_change_bound`, so the bound exceeds hw_s + hw_{s+1}. A
// crossing is therefore never produced by changes that are small against the
// declared resolution of the arrangement.

#[test]
fn sc_d8_a_crossing_needs_change_larger_than_both_resolutions() {
    let f = evolving_field();
    let s = EdgeSeries::from_registrations(&f.registrations, &DeclaredRelations::from_edges_with_provenance(
        4, vec![(0, 1), (1, 2), (2, 3), (1, 3)], vec![EdgeProvenance::Continuation; 4])).unwrap();
    let mut crossings = 0;
    for x in arrangement_steps(&s).unwrap() {
        if let (AtDelta::Resolved(p), AtDelta::Resolved(q)) = (x.before, x.after) {
            if p != q {
                crossings += 1;
                let (d, _) = s.change(x.step);
                let bound = minor_change_bound(&s.a[x.step], &d, x.e, x.f, x.i, x.j);
                let hw = minor_half_width(&s.a[x.step], &s.da[x.step], x.e, x.f, x.i, x.j)
                    + minor_half_width(&s.a[x.step + 1], &s.da[x.step + 1], x.e, x.f, x.i, x.j);
                assert!(bound > hw);
            }
        }
    }
    assert!(crossings > 0, "the fixture must contain a crossing");
}
