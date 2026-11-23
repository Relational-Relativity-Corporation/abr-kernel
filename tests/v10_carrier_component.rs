//! Kernel V10.3: I-SC-D17, the carrier as a component (Proposed, Generator,
//! 2026-11-23; V10_README §2.6; OC-SC-3, §2.0 link 10).
//!
//! A carrier c registered at the declared loci (T5) enters the series as a
//! component beside the structural components. The forms that read it are
//! the declared ones; nothing is added. What follows is derived:
//!
//! (i)   One registration, one number. With no run-constant part declared on
//!       the carrier, ε[e] = |A[c][e]| and δ_ε[e] = δA[c][e]: the V3 floor is
//!       the state reading of the carrier component, at every registration.
//! (ii)  PC1 on the carrier reads the signed exchange. |ε_{s+1} − ε_s| ≤
//!       |Δ_s[c][e]|, with equality unless A[c][e] changes sign, and the two
//!       share one bound. A resolved change of ε is PC1 freedom on the
//!       carrier; PC1 freedom can occur with ε held (the exchange reverses).
//! (iii) A change minor is resolved only where each of its two readings is
//!       resolved at PC1 on at least one of its two edges (PC2(a) and
//!       PC2(b)). So wherever the structure holds on both edges, PC2(a)
//!       between the carrier and the structure reads structure: the bound
//!       forces it. "Locked exchange" is read beside PC1 on the structure.
//! (iv)  How it meets I-SC-D14 (ii). Re-declaring the carrier as
//!       c′ = a·c + b·j (a ≠ 0; R = [[a, b], [0, 1]], det R = a) multiplies
//!       every carrier–structure minor by a exactly, so at δ = 0 the locked
//!       or independent reading is unchanged. PC1 on the carrier and the V3
//!       floor are not: they belong to the declared carrier. At δ > 0, with
//!       δc′ = |a|·δc + |b|·δj, the bound never shrinks: mixing never
//!       resolves a locked reading, and can lose an independent one.
//! (v)   Locked exchange and held arrangement are independent: I-SC-D3 and
//!       D4 with components and edges exchanged.
//!
//! **Whole fields (Origin, 2026-11-23, 53rd; Df6, W11).** Every test forms a
//! whole relational field (`RelationalField::new`, with the carrier declared
//! to energy), so every variable is evaluated together, and reads its claim
//! from the parts that field reports: energy, freedom (PC1–PC3) and
//! connection. Every gap is named (`synthetic_whole_field::inclusion`). The
//! one test on edge-registered fields names what is not evaluated there.
//! *Was (48th–49th):* each claim checked on the edge series or on minors
//! computed in the test, with energy or connection left out.
//!
//! [IMPL] SYNTHETIC FIXTURES: dyadic values, so every sum and product is
//! exact. They establish that the forms compute as stated, nothing
//! empirical. OC-SC-3 itself is answered only by registered data.

#[path = "support/synthetic_whole_field.rs"]
mod wf;
use abr_kernel::exact::{exceeds, Expansion};
use abr_kernel::persistence_change::*;
use std::sync::Mutex;
use wf::{for_all, mixed, enumerated_declared, enumerated_rel, rel, whole, Declared, Whole};

fn across(w: &Whole) -> &Couplings { w.freedom().across_components.as_ref().unwrap() }

/// Carrier–structure PC2(a) readings of the field: (step, e, f, minor, half-width, reading).
fn carrier_structure(w: &Whole) -> Vec<(usize, usize, usize, f64, f64, Reading)> {
    let c = w.carrier();
    across(w).readings.iter().filter_map(|x| match x.over {
        ReadingPair::Components { step, i: 0, j } if j == c => Some((step, x.e, x.f, x.minor, x.half_width, x.reading)),
        _ => None,
    }).collect()
}

// ═══════════════════════════════════════════════════════════════════════════
// (i) The V3 floor is the state reading of the carrier component
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn d17_i_the_energy_floor_is_the_state_reading_of_the_carrier_component() {
    // From the field: its energy exchanges, and the state of the carrier's
    // relations in the series it reads. Equal wherever no run-constant part
    // is declared on the carrier (the statement's scope). Where one is, the
    // two differ on some fields: energy takes the carrier's repeatability
    // alone. A finding for Origin (I-SC-D17 (i), 53rd).
    let counts = Mutex::new([0usize; 3]);
    for_all("d17_i", 0xD17, |r| {
        let rel = enumerated_rel(r);
        let (k, regs) = (r.usize(2, 3), r.usize(2, 4));
        let rc = r.selects(4);
        let w = whole(&rel, enumerated_declared(r, &rel, k, regs, 0.5, rc, rc));
        let (c, energy) = (w.carrier(), w.field.ground.energy.as_ref().unwrap());
        let mut g = counts.lock().unwrap();
        for t in 0..regs {
            for e in 0..rel.n_edges() {
                let x = energy.exchanges[t][e];
                let a = w.series.entry(t, c, e);
                assert_eq!(x.epsilon, a.abs().nearest(), "ε = |A[c][e]|, always");
                let state = exceeds(&a, &Expansion::of(w.series.da[t][c][e]));
                if !w.declared.carrier_run_constant() {
                    assert_eq!(x.delta_epsilon, w.series.da[t][c][e], "δ_ε = δA[c][e]");
                    assert_eq!(x.floor_resolved, state, "one reading");
                    g[x.floor_resolved as usize] += 1;
                } else if x.floor_resolved != state {
                    g[2] += 1;
                }
            }
        }
    });
    let g = counts.into_inner().unwrap();
    assert!(g[0] > 0 && g[1] > 0, "floor not resolved {}, resolved {}", g[0], g[1]);
    assert!(g[2] > 0, "with run-constant parts on the carrier, the floor and the state reading differ on some fields");
}

// ═══════════════════════════════════════════════════════════════════════════
// (ii) PC1 on the carrier reads the signed exchange
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn d17_ii_worked_the_exchange_reverses_with_its_magnitude_held() {
    // Loci 0 → 1. Carrier (component 1) at 0 then 1, then 1 then 0: A = −1
    // then +1. ε = 1 at both registrations, so the V3 degree holds; the
    // exchange has reversed, and PC1 on the carrier reads it (Δ = 2 > 1/4).
    // One edge: PC2 and connection's pair grades are not formable, and are
    // named (W11).
    let rel = rel(2, vec![(0, 1)]);
    let d = Declared::repeatability(vec![vec![vec![0.0, 0.0], vec![0.0, 1.0]], vec![vec![0.0, 0.0], vec![1.0, 0.0]]],
        vec![vec![1.0 / 16.0; 2]; 2]);
    let w = whole(&rel, d);
    let x = &w.field.ground.energy.as_ref().unwrap().exchanges;
    assert_eq!((x[0][0].epsilon, x[1][0].epsilon), (1.0, 1.0));
    assert!(x[0][0].floor_resolved && x[1][0].floor_resolved);
    assert_eq!(w.pc1(0, 1, 0), Reading::Freedom);
    assert_eq!(w.freedom().relations.iter().find(|r| r.component == 1).unwrap().change, 2.0);
    assert_eq!(w.pc1(0, 0, 0), Reading::Structure);
    assert!(!wf::inclusion(&w).is_empty(), "the gaps are named");
}

#[test]
fn d17_ii_a_resolved_change_of_exchange_is_pc1_freedom_not_conversely() {
    // ε and δ_ε from the field's energy; PC1 from the field's freedom.
    let counts = Mutex::new((0usize, 0usize));
    for_all("d17_ii", 0xD17, |r| {
        let rel = enumerated_rel(r);
        let regs = r.usize(2, 4);
        let rc = r.selects(4);
        let w = whole(&rel, enumerated_declared(r, &rel, 2, regs, 0.5, rc, rc));
        let (c, x) = (w.carrier(), &w.field.ground.energy.as_ref().unwrap().exchanges);
        let mut g = counts.lock().unwrap();
        for st in 0..regs - 1 {
            for e in 0..rel.n_edges() {
                let d_eps = Expansion::diff(x[st + 1][e].epsilon, x[st][e].epsilon);
                let d = w.series.change_exact(st, c, e);
                // |Δε| ≤ |Δ| exactly, on every field.
                assert!(d.abs().minus(&d_eps.abs()).signum() >= 0);
                if w.declared.carrier_run_constant() { continue; }
                // One bound: δΔ = δε_s + δε_{s+1}.
                let h = Expansion::of(x[st][e].delta_epsilon).plus(&Expansion::of(x[st + 1][e].delta_epsilon));
                assert_eq!(w.series.change_bound_exact(st, c, e).minus(&h).signum(), 0);
                let resolved = exceeds(&d_eps, &h);
                if resolved { assert_eq!(w.pc1(st, c, e), Reading::Freedom); g.0 += 1; }
                if w.pc1(st, c, e) == Reading::Freedom && !resolved { g.1 += 1; }
            }
        }
    });
    let (both, pc1_only) = counts.into_inner().unwrap();
    assert!(both > 0 && pc1_only > 0, "ε resolved {both}, PC1 alone {pc1_only}");
}

// ═══════════════════════════════════════════════════════════════════════════
// (iii) A change minor needs change in both readings
// ═══════════════════════════════════════════════════════════════════════════

/// Structure holding at the first steps of the carrier fixture of
/// `energy_carrier_as_a_component_reads_through_the_same_forms`.
fn locked_fixture() -> Whole {
    let structural = [0.0, 1.0, 2.0, 2.0];
    let c = [0.0, 3.0, 6.0, 8.0];
    let values = (0..4).map(|t| vec![vec![4.0, 2.0 + structural[t], 0.0], vec![9.0, 5.0 + c[t], 1.0]]).collect();
    whole(&rel(3, vec![(0, 1), (1, 2)]), Declared::repeatability(values, vec![vec![1.0 / 128.0; 3]; 2]))
}

#[test]
fn d17_iii_worked_energy_flows_while_structure_holds_reads_locked() {
    // At step 2 the structure holds on both edges and the carrier's exchange
    // changes by (2, −2). PC2(a) reads structure: the minor is 0 exactly, Δ_j
    // being 0. It is the ratio-0 case, forced by the bound; PC1 on the
    // structure tells it apart. Read from the whole field.
    let w = locked_fixture();
    let at2: Vec<_> = carrier_structure(&w).into_iter().filter(|x| x.0 == 2).collect();
    assert_eq!(at2.len(), 1);
    assert_eq!((at2[0].3, at2[0].5), (0.0, Reading::Structure));
    assert_eq!((w.pc1(2, 0, 0), w.pc1(2, 0, 1)), (Reading::Structure, Reading::Structure));
    assert_eq!((w.pc1(2, 1, 0), w.pc1(2, 1, 1)), (Reading::Freedom, Reading::Freedom));
    assert_eq!((w.pc1(1, 0, 0), w.pc1(1, 1, 0)), (Reading::Freedom, Reading::Freedom));
    // Energy is read beside it: the exchange's floor at every registration.
    assert!(w.field.ground.energy.as_ref().unwrap().share_resolved > 0.0);
}

#[test]
fn d17_iii_the_coupled_change_grade_is_not_met_by_forced_readings() {
    // Two registrations, two components. The structure holds everywhere; the
    // carrier's exchange changes by (1, 5) on the two edges. PC2(a) is the
    // only family (n = 2), and its one reading is forced to structure by
    // (iii). *Decided (Origin, 57th: option 2):* the reading is flagged
    // forced, the coupled-change grade does not hold the pair, and the pair
    // is reported apart. *Was (48th, "case for Origin"):* the grade read the
    // pair as connected in change; the fenced grade still does.
    let values = vec![vec![vec![0.0, 1.0, 3.0], vec![0.0, 0.0, 0.0]], vec![vec![0.0, 1.0, 3.0], vec![0.0, -1.0, -6.0]]];
    let w = whole(&rel(3, vec![(0, 1), (1, 2)]), Declared::repeatability(values, vec![vec![1.0 / 64.0; 3]; 2]));
    assert_eq!((w.pc1(0, 0, 0), w.pc1(0, 0, 1)), (Reading::Structure, Reading::Structure));
    assert_eq!(w.series.change(0).0[1], vec![1.0, 5.0]);
    assert!(w.freedom().across_steps.is_err());
    assert_eq!(across(&w).readings.len(), 1);
    assert_eq!(across(&w).readings[0].reading, Reading::Structure);
    assert!(across(&w).readings[0].forced);
    assert!(w.connection().coupled_change.as_ref().unwrap().is_empty());
    assert_eq!(w.connection().coupled_change_forced.as_ref().unwrap(), &vec![(0, 1)]);
    assert_eq!(coupled_change_pairs_including_forced(w.freedom()).unwrap(), vec![(0, 1)], "the fenced grade, for comparison");
}

/// The two readings (step, component) a change minor is formed over.
fn readings_of(over: ReadingPair) -> [(usize, usize); 2] {
    match over {
        ReadingPair::Components { step, i, j } => [(step, i), (step, j)],
        ReadingPair::Steps { component, step } => [(step, component), (step + 1, component)],
    }
}

/// (iii) over one PersistenceChange: tally resolved, forced and locked
/// readings, asserting the statement on each.
fn tally_iii(pc: &PersistenceChange, g: &mut [usize; 3]) -> Vec<(usize, usize)> {
    let pc1 = |st: usize, c: usize, e: usize| pc.relations.iter().find(|x| x.step == st && x.component == c && x.edge == e).unwrap().reading;
    let mut forced_pairs = Vec::new();
    for fam in [&pc.across_components, &pc.across_steps] {
        let Ok(fam) = fam else { continue };
        for x in &fam.readings {
            let moved = |(st, c): (usize, usize)| pc1(st, c, x.e) == Reading::Freedom || pc1(st, c, x.f) == Reading::Freedom;
            let [p, q] = readings_of(x.over);
            assert_eq!(x.forced, !(moved(p) && moved(q)), "the kernel's forced flag is (iii)'s condition");
            if x.reading == Reading::Freedom {
                assert!(moved(p) && moved(q), "a resolved minor needs change in both readings");
                g[0] += 1;
            } else if !(moved(p) && moved(q)) {
                g[1] += 1;
                forced_pairs.push((x.e, x.f));
            } else {
                g[2] += 1;
            }
        }
    }
    forced_pairs
}

#[test]
fn d17_iii_a_change_minor_resolves_only_where_each_reading_changes() {
    // Over whole fields, with Z1a parts (offsets, per-locus gains) on some:
    // every resolved change minor has, in each of its readings, PC1 freedom
    // on e or on f. Also counted: coupled-change pairs of the field whose
    // every reading is forced (the finding for Origin, in whole fields).
    let counts = Mutex::new(([0usize; 3], 0usize));
    for_all("d17_iii", 0xD17, |r| {
        let rel = enumerated_rel(r);
        let (k, regs) = (r.usize(2, 3), r.usize(3, 4));
        let z = r.selects(4);
        let w = whole(&rel, enumerated_declared(r, &rel, k, regs, 0.25, z, z));
        let mut g = counts.lock().unwrap();
        let forced = tally_iii(w.freedom(), &mut g.0);
        let all_forced = |p: &(usize, usize)| {
            let n = [&w.freedom().across_components, &w.freedom().across_steps].iter()
                .filter_map(|f| f.as_ref().ok()).flat_map(|f| f.readings.iter()).filter(|x| (x.e, x.f) == *p).count();
            n > 0 && forced.iter().filter(|q| *q == p).count() == n
        };
        // The decided grade (57th): no coupled pair is met by forced readings
        // alone; those pairs are reported apart, and the fenced grade (7th)
        // still counts them.
        if let (Ok(cc), Ok(fo)) = (&w.connection().coupled_change, &w.connection().coupled_change_forced) {
            assert!(cc.iter().all(|p| !all_forced(p)));
            assert!(fo.iter().all(|p| all_forced(p)));
            let old = coupled_change_pairs_including_forced(w.freedom()).unwrap();
            g.1 += old.iter().filter(|p| all_forced(p)).count();
            assert_eq!(old.iter().filter(|p| all_forced(p)).count(), fo.iter().filter(|p| old.contains(p)).count());
        }
    });
    let ([resolved, forced, locked], vacuous) = counts.into_inner().unwrap();
    assert!(resolved > 0 && forced > 0 && locked > 0, "resolved {resolved}, forced {forced}, locked {locked}");
    assert!(vacuous > 0, "pairs the fenced grade met by forced readings alone occur in whole fields, and are reported apart");
}

/// What is not evaluated on an edge-registered series, and why (W11; option
/// (b), Origin 53rd; whole-field evaluation there is OC-WF-1).
const NOT_EVALUATED_ON_EDGE_REGISTERED: [(&str, &str); 3] = [
    ("energy (V3)", "the carrier is declared on loci; an edge-registered series has none"),
    ("connection: identity, proportional, sustained", "read on node-derived fields only (I-SC-D15 (v))"),
    ("the relational field (A, B, Ξ and their persistence forms)", "RelationalField is formed from node values (OC-WF-1)"),
];

#[test]
fn d17_iii_on_edge_registered_series_with_independent_gains() {
    // Independent gains widen PC2(a)'s bounds and never PC1's; they exist on
    // edge-registered series only. Every PC level is read there, or named;
    // what is not evaluated is listed above.
    assert_eq!(NOT_EVALUATED_ON_EDGE_REGISTERED.len(), 3);
    let counts = Mutex::new([0usize; 3]);
    for_all("d17_iii_edge", 0xD17, |r| {
        let rel = enumerated_rel(r);
        let (k, regs) = (r.usize(2, 3), r.usize(3, 4));
        let w = whole(&rel, enumerated_declared(r, &rel, k, regs, 0.25, false, false));
        let m = rel.n_edges();
        let gains = EdgeGains { component: vec![0.0; k], edge: vec![0.0; m],
            independent: (0..k).map(|_| (0..m).map(|_| if r.selects(3) { 1.0 / 16.0 } else { 0.0 }).collect()).collect() };
        let s = EdgeSeries::from_edge_values_synthetic(w.series.a.clone(), w.series.da.clone()).unwrap()
            .with_run_constant(vec![vec![0.0; m]; k], gains);
        let pc = PersistenceChange::new(&s);
        assert!(pc.gaps().len() <= 3);
        tally_iii(&pc, &mut counts.lock().unwrap());
    });
    let g = counts.into_inner().unwrap();
    assert!(g.iter().all(|&n| n > 0), "resolved, forced, locked: {g:?}");
}

// ═══════════════════════════════════════════════════════════════════════════
// (iv) Re-declaring the carrier within its plane with the structure (D14 (ii))
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn d17_iv_exactly_the_carrier_structure_minor_is_a_property_of_their_plane() {
    // δ = 0. Two whole fields, the carrier as declared and re-declared. Every
    // carrier–structure minor is multiplied by det R = a, so the locked or
    // independent reading is unchanged; PC1 on the carrier and the V3 floor
    // change on some fields when b ≠ 0. The structure's readings do not.
    let moved = Mutex::new((0usize, 0usize));
    for_all("d17_iv_exact", 0xD17, |r| {
        let rel = enumerated_rel(r);
        let regs = r.usize(2, 4);
        let d = enumerated_declared(r, &rel, 2, regs, 0.0, false, false);
        let (a, b) = (*[1.0, -1.0, 2.0, 0.5, -3.0].get(r.usize(0, 4)).unwrap(), r.eighth(-2.0, 2.0));
        let (wf_, wg) = (whole(&rel, d.clone()), whole(&rel, mixed(&d, a, b)));
        let (xf, xg) = (carrier_structure(&wf_), carrier_structure(&wg));
        assert_eq!(xf.len(), xg.len(), "the same pairs enter");
        for (u, v) in xf.iter().zip(&xg) {
            assert_eq!((u.0, u.1, u.2), (v.0, v.1, v.2));
            assert_eq!(v.3, a * u.3, "μ′ = det R · μ");
            assert_eq!(u.5, v.5, "locked or independent, unchanged");
        }
        let pc1s = |w: &Whole, c| w.freedom().relations.iter().filter(|x| x.component == c).map(|x| x.reading).collect::<Vec<_>>();
        assert_eq!(pc1s(&wf_, 0), pc1s(&wg, 0));
        let mut m = moved.lock().unwrap();
        if pc1s(&wf_, 1) != pc1s(&wg, 1) { m.0 += 1 }
        let floors = |w: &Whole| w.field.ground.energy.as_ref().unwrap().exchanges.iter()
            .map(|t| t.iter().map(|x| x.floor_resolved).collect::<Vec<_>>()).collect::<Vec<_>>();
        if floors(&wf_) != floors(&wg) { m.1 += 1 }
    });
    let (pc1_moved, floor_moved) = moved.into_inner().unwrap();
    assert!(pc1_moved > 0 && floor_moved > 0, "PC1 moved {pc1_moved}, floor moved {floor_moved}");
}

#[test]
fn d17_iv_at_delta_a_carrier_mixed_with_structure_never_gains_resolution() {
    // δ > 0, the re-declared carrier's δ propagated as an interval,
    // δc′ = |a|·δc + |b|·δj, on whole fields. b = 0 is a per-component scale
    // (I-SC-D15, kind C): no reading moves. For b ≠ 0 the minor is a·μ
    // exactly and its bound is at least |a| times the original:
    //   hw′ − |a|·hw ≥ 2|b|·δΔj[e]·δΔj[f] ≥ 0.
    // So a locked reading is never resolved by mixing; an independent one can
    // be lost. No pair enters that did not enter before; some can leave.
    //
    // *Generator's first statement (2026-11-23), refuted by this test before
    // commit:* "at δ > 0 PC2(a) readings move in both directions". On 300
    // fields: 40 moved freedom → structure, none structure → freedom.
    let counts = Mutex::new((0usize, 0usize, 0usize));
    for_all("d17_iv_delta", 0xD17, |r| {
        let rel = enumerated_rel(r);
        let regs = r.usize(2, 4);
        let d = enumerated_declared(r, &rel, 2, regs, 0.25, false, false);
        let wf_ = whole(&rel, d.clone());
        let a = *[1.0, -1.0, 2.0, 0.5].get(r.usize(0, 3)).unwrap();
        let ws = whole(&rel, mixed(&d, a, 0.0));
        let readings = |w: &Whole| carrier_structure(w).iter().map(|x| (x.0, x.1, x.2, x.5)).collect::<Vec<_>>();
        assert_eq!(readings(&wf_), readings(&ws), "a per-component scale moves no reading");
        let b = r.eighth(-2.0, 2.0);
        let wm = whole(&rel, mixed(&d, a, b));
        let (xf, xm) = (carrier_structure(&wf_), carrier_structure(&wm));
        let mut g = counts.lock().unwrap();
        for v in &xm {
            let u = xf.iter().find(|u| (u.0, u.1, u.2) == (v.0, v.1, v.2)).expect("no new pair enters");
            assert_eq!(v.3, a * u.3, "μ′ = a·μ");
            assert!(v.4 >= a.abs() * u.4, "the bound does not shrink");
            match (u.5, v.5) {
                (Reading::Structure, Reading::Freedom) => panic!("mixing resolved a locked reading"),
                (Reading::Freedom, Reading::Structure) => g.0 += 1,
                _ => {}
            }
        }
        if xm.len() < xf.len() { g.1 += 1 }
        if b != 0.0 && xm.iter().any(|v| v.5 == Reading::Freedom) { g.2 += 1 }
    });
    let (lost, left, kept) = counts.into_inner().unwrap();
    assert!(lost > 0 && left > 0 && kept > 0, "lost {lost}, pairs left {left}, kept {kept}");
}

#[test]
fn d17_iv_the_one_way_result_belongs_to_the_bound_not_the_field() {
    // The one-way result above is about resolution, not about structure and
    // freedom. Over the sequence, relations move from structure to freedom
    // and back (PC1, step to step). Under the re-declaration, PC1 on the
    // carrier (a different quantity) and PC3 (two states compared, PC-D2 as
    // corrected) move both ways. Only the carrier–structure PC2(a) reading
    // moves one way (the test above). Whole fields throughout.
    let c = Mutex::new([0usize; 6]);
    for_all("d17_iv_both_ways", 0xD17, |r| {
        let rel = enumerated_rel(r);
        let regs = r.usize(3, 5);
        let d = enumerated_declared(r, &rel, 2, regs, 0.25, false, false);
        let (wf_, wm) = (whole(&rel, d.clone()), whole(&rel, mixed(&d, 1.0, r.eighth(-2.0, 2.0))));
        let (pf, pm) = (wf_.freedom(), wm.freedom());
        let mut g = c.lock().unwrap();
        let mut tally = |u: Reading, v: Reading, k: usize| match (u, v) {
            (Reading::Structure, Reading::Freedom) => g[k] += 1,
            (Reading::Freedom, Reading::Structure) => g[k + 1] += 1,
            _ => {}
        };
        for x in &pf.relations {
            if let Some(y) = pf.relations.iter().find(|y| y.step == x.step + 1 && y.component == x.component && y.edge == x.edge) {
                tally(x.reading, y.reading, 0);
            }
        }
        for (u, v) in pf.relations.iter().zip(&pm.relations) {
            if u.component == 1 { tally(u.reading, v.reading, 2); }
        }
        let (af, am) = (pf.arrangement.as_ref().unwrap(), pm.arrangement.as_ref().unwrap());
        for u in af {
            if let Some(v) = am.iter().find(|v| (v.step, v.e, v.f, v.i, v.j) == (u.step, u.e, u.f, u.i, u.j)) {
                tally(u.reading, v.reading, 4);
            }
        }
    });
    let g = c.into_inner().unwrap();
    assert!(g.iter().all(|&n| n > 0), "over the sequence {:?}, PC1 on the carrier {:?}, PC3 {:?}", &g[0..2], &g[2..4], &g[4..6]);
}

// ═══════════════════════════════════════════════════════════════════════════
// (v) Locked exchange and held arrangement are independent
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn d17_v_worked_locked_exchange_with_the_arrangement_reorganizing() {
    // c = 2·j + k with k fixed: every change of the carrier is twice the
    // structure's, so PC2(a) reads locked at every pair; the state minor is
    // k_e·j_f − j_e·k_f, and it crosses zero as j moves (PC3 freedom). The
    // field's connection reads the pair as coupled change and not
    // proportional.
    let k = [0.0, 1.0, 0.0];
    let js = [[0.0, 1.0, 3.0], [0.0, -1.0, -3.0]];
    let values = js.iter().map(|j| vec![j.to_vec(), j.iter().zip(&k).map(|(&j, &k)| 2.0 * j + k).collect()]).collect();
    let w = whole(&rel(3, vec![(0, 1), (1, 2)]), Declared::repeatability(values, vec![vec![1.0 / 64.0; 3]; 2]));
    let x = carrier_structure(&w);
    assert!(!x.is_empty() && x.iter().all(|u| u.3 == 0.0 && u.5 == Reading::Structure));
    let arr = w.freedom().arrangement.as_ref().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0].reading, Reading::Freedom);
    assert_ne!(arr[0].before, arr[0].after);
    assert_eq!(w.connection().coupled_change.as_ref().unwrap(), &vec![(0, 1)]);
    assert!(w.connection().proportional.as_ref().unwrap().is_empty());
}

#[test]
fn d17_v_worked_held_arrangement_with_independent_exchange() {
    // c_t = λ_t·j_t with λ moving from 1 to 2: every state minor is 0 (the
    // field's proportional grade holds the pair), while the change minor is
    // (λ′ − λ)·(Δ_j[e]·j_f − Δ_j[f]·j_e) ≠ 0: independent exchange.
    let js = [[0.0, 1.0, 3.0], [0.0, 2.0, 3.0]];
    let values = js.iter().zip([1.0, 2.0]).map(|(j, l)| vec![j.to_vec(), j.iter().map(|&j| l * j).collect()]).collect();
    let w = whole(&rel(3, vec![(0, 1), (1, 2)]), Declared::repeatability(values, vec![vec![1.0 / 64.0; 3]; 2]));
    let arr = w.freedom().arrangement.as_ref().unwrap();
    assert!(!arr.is_empty() && arr.iter().all(|x| x.reading == Reading::Structure && x.before == x.after));
    assert!(carrier_structure(&w).iter().any(|u| u.3 != 0.0 && u.5 == Reading::Freedom));
    assert_eq!(w.connection().proportional.as_ref().unwrap(), &vec![(0, 1)]);
    assert!(w.connection().coupled_change.as_ref().unwrap().is_empty());
}

#[test]
fn d17_v_d3_and_d4_with_components_and_edges_exchanged() {
    // δ = 0, whole fields. c_t = λ·j_t + k ⇒ every carrier–structure change
    // minor in the field is 0 and every PC3 state is that of
    // k_e·j_f − j_e·k_f; c_t = λ_t·j_t ⇒ every PC3 state is not resolved
    // (the minor is 0).
    use abr_kernel::ground_variables::{sign_at_delta_exact, AtDelta};
    for_all("d17_v", 0xD17, |r| {
        let rel = enumerated_rel(r);
        let regs = r.usize(2, 4);
        let base = enumerated_declared(r, &rel, 1, regs, 0.0, false, false);
        let n = rel.n_nodes;
        let (l, k): (f64, Vec<f64>) = (r.eighth(-2.0, 2.0), (0..n).map(|_| r.eighth(-2.0, 2.0)).collect());
        let ka: Vec<f64> = rel.edges.iter().map(|&(s, t)| k[s] - k[t]).collect();
        let values: Vec<Vec<Vec<f64>>> = base.values.iter().map(|t| vec![t[0].clone(),
            t[0].iter().zip(&k).map(|(&j, &k)| l * j + k).collect()]).collect();
        let w = whole(&rel, Declared::repeatability(values, vec![vec![0.0; n]; 2]));
        if let Ok(cs) = &w.freedom().across_components {
            assert!(cs.readings.iter().all(|x| x.minor == 0.0 && x.reading == Reading::Structure), "locked exchange");
        }
        if let Ok(arr) = &w.freedom().arrangement {
            for x in arr {
                for (t, st) in [(x.step, x.before), (x.step + 1, x.after)] {
                    let (je, jf) = (w.series.entry(t, 0, x.e), w.series.entry(t, 0, x.f));
                    let want = je.times(&Expansion::of(ka[x.f])).minus(&Expansion::of(ka[x.e]).times(&jf));
                    assert_eq!(st, sign_at_delta_exact(&want, &Expansion::new()), "k_e·j_f − j_e·k_f");
                }
            }
        }
        let lam: Vec<f64> = (0..regs).map(|_| r.eighth(-2.0, 2.0)).collect();
        let values: Vec<Vec<Vec<f64>>> = base.values.iter().zip(&lam).map(|(t, &l)| vec![t[0].clone(),
            t[0].iter().map(|&j| l * j).collect()]).collect();
        let w = whole(&rel, Declared::repeatability(values, vec![vec![0.0; n]; 2]));
        if let Ok(arr) = &w.freedom().arrangement {
            assert!(arr.iter().all(|x| x.before == AtDelta::NotResolved && x.after == AtDelta::NotResolved), "held proportional");
        }
    });
}
