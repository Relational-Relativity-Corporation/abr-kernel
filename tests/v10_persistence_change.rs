//! Kernel V10 — structure and freedom as persistence and change (PC1–PC4).
//! PROPOSED forms. Each test checks one derived property (PC-D1 to PC-D5) or
//! one reading.
//!
//! [IMPL] SYNTHETIC FIXTURES: values are chosen to exercise the forms. They
//! establish that the forms compute as stated, nothing empirical.

#[path = "support/synthetic_identity.rs"]
mod synthetic_identity;
use abr_kernel::ground_variables::{DeclarationGap, Registration};
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::persistence_change::*;

fn rel(n: usize, edges: Vec<(usize, usize)>) -> DeclaredRelations {
    let m = edges.len();
    synthetic_identity::identify(DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m]))
}

fn reg(data: Vec<Vec<f64>>, d: f64) -> Registration {
    let (k, n) = (data.len(), data[0].len());
    Registration::synthetic(NodeField::new(data), vec![vec![d; n]; k])
}

/// Edge-registered series: a[t][c][e], uniform δ.
fn edge_series(a: Vec<Vec<Vec<f64>>>, d: f64) -> EdgeSeries {
    let da = a.iter().map(|t| t.iter().map(|c| vec![d; c.len()]).collect()).collect();
    EdgeSeries::from_edge_values_synthetic(a, da).unwrap()
}

/// A chain 0 → 1 → 2 → 3, two components, four registrations. Component 0
/// rises steadily at locus 1 only; component 1 rises at locus 2 only with a
/// different pattern, so some relations change and some persist.
fn chain_series(d: f64) -> (DeclaredRelations, Vec<Registration>) {
    let r = rel(4, vec![(0, 1), (1, 2), (2, 3)]);
    let seq = (0..4).map(|t| {
        let t = t as f64;
        reg(vec![vec![5.0, 3.0 + 0.5 * t, 2.0, 0.0],
                 vec![1.0, 2.0, 4.0 + 0.3 * t * t, 1.0]], d)
    }).collect();
    (r, seq)
}

// ── PC1 ─────────────────────────────────────────────────────────────────────

#[test]
fn pc1_relation_that_moves_is_freedom_relation_that_holds_is_structure() {
    let (r, seq) = chain_series(0.01);
    let s = EdgeSeries::from_registrations(&seq, &r).unwrap();
    let rc = relation_changes(&s);
    let at = |step, c, e| rc.iter().find(|x| x.step == step && x.component == c && x.edge == e).unwrap().reading;
    // Component 0: locus 1 moves, so edges 0→1 and 1→2 change; 2→3 persists.
    assert_eq!(at(0, 0, 0), Reading::Freedom);
    assert_eq!(at(0, 0, 1), Reading::Freedom);
    assert_eq!(at(0, 0, 2), Reading::Structure);
    // Component 1: locus 2 moves; edge 0→1 persists.
    assert_eq!(at(1, 1, 0), Reading::Structure);
    assert_eq!(at(1, 1, 1), Reading::Freedom);
}

// ── PC-D1 complementarity ──────────────────────────────────────────────────

#[test]
fn pc_d1_structure_and_freedom_partition_every_level() {
    let (r, seq) = chain_series(0.01);
    let pc = PersistenceChange::new(&EdgeSeries::from_registrations(&seq, &r).unwrap());
    let check = |sh: Shares, n: usize| {
        assert_eq!(sh.evaluated(), n);
        assert!((sh.f_delta().unwrap() + sh.s_delta().unwrap() - 1.0).abs() < 1e-12);
    };
    check(pc.relation_shares(None), pc.relations.len());
    for lvl in [&pc.across_components, &pc.across_steps] {
        let c = lvl.as_ref().unwrap();
        check(Shares::of(c.readings.iter().map(|x| &x.reading)), c.readings.len());
    }
    let a = pc.arrangement.as_ref().unwrap();
    check(Shares::of(a.iter().map(|x| &x.reading)), a.len());
}

// ── PC2 ─────────────────────────────────────────────────────────────────────

#[test]
fn pc2_locked_change_is_structure_independent_change_is_freedom() {
    // Three edges, one component, registered directly. Edge 1 changes as
    // exactly twice edge 0 at every step (locked); edge 2 changes on its own.
    let a: Vec<Vec<Vec<f64>>> = [0.0, 1.0, 3.0, 4.0].iter().zip([0.0, 5.0, 1.0, 6.0]).map(|(&u, w)| {
        vec![vec![10.0 + u, 20.0 + 2.0 * u, 7.0 + w]]
    }).collect();
    let c = coupling_across_steps(&edge_series(a, 0.01)).unwrap();
    let get = |e, f| c.readings.iter().filter(|x| x.e == e && x.f == f).map(|x| x.reading).collect::<Vec<_>>();
    assert!(get(0, 1).iter().all(|r| *r == Reading::Structure), "locked change holds them together");
    assert!(get(0, 2).iter().all(|r| *r == Reading::Freedom), "independent change is freedom");
}

#[test]
fn pc2_steps_family_reads_freedom_with_one_component() {
    // A single component is enough for PC2(b); PC2(a) is a declaration gap.
    let a = (0..3).map(|t| vec![vec![t as f64, (t * t) as f64]]).collect();
    let s = edge_series(a, 0.01);
    assert_eq!(coupling_across_components(&s).unwrap_err(), DeclarationGap::SingleComponent);
    assert!(!coupling_across_steps(&s).unwrap().readings.is_empty());
}

#[test]
fn pc2_only_consecutive_steps_are_paired() {
    let a = (0..5).map(|t| vec![vec![t as f64, (t * t) as f64]]).collect();
    let c = coupling_across_steps(&edge_series(a, 0.01)).unwrap();
    // Four steps give exactly three consecutive pairs (s, s+1).
    let steps: Vec<usize> = c.readings.iter().map(|x| match x.over {
        ReadingPair::Steps { step, .. } => step, _ => unreachable!(),
    }).collect();
    assert_eq!(steps, vec![0, 1, 2]);
}

#[test]
fn pc2_persisting_relations_enter_no_coupling() {
    // Edge 1 never changes: it is structure at PC1 and pairs with nothing.
    let a = (0..3).map(|t| vec![vec![t as f64, 4.0, (t * t) as f64]]).collect();
    let c = coupling_across_steps(&edge_series(a, 0.01)).unwrap();
    assert!(c.readings.iter().all(|x| x.e != 1 && x.f != 1));
}

// ── PC3 ─────────────────────────────────────────────────────────────────────

#[test]
fn pc3_arrangement_change_is_freedom() {
    // Two edges, two components. At t = 2 edge 1's components swap order,
    // so the minor on A changes sign between t = 1 and t = 2.
    let a = vec![
        vec![vec![1.0, 1.0], vec![0.0, 2.0]],
        vec![vec![1.0, 1.0], vec![0.0, 2.1]],
        vec![vec![1.0, 2.0], vec![0.0, -1.0]],
    ];
    let steps = arrangement_steps(&edge_series(a, 0.01)).unwrap();
    assert_eq!(steps[0].reading, Reading::Structure);
    assert_eq!(steps[1].reading, Reading::Freedom);
}

// ── PC-D2 resolution ───────────────────────────────────────────────────────

#[test]
fn pc_d2_coarser_delta_never_adds_freedom() {
    let (r, seq_fine) = chain_series(0.01);
    let (_, seq_coarse) = chain_series(0.2);
    let fine = PersistenceChange::new(&EdgeSeries::from_registrations(&seq_fine, &r).unwrap());
    let coarse = PersistenceChange::new(&EdgeSeries::from_registrations(&seq_coarse, &r).unwrap());
    // PC1: same evaluated set; every freedom at coarse δ is freedom at fine δ.
    for (f, c) in fine.relations.iter().zip(&coarse.relations) {
        if c.reading == Reading::Freedom { assert_eq!(f.reading, Reading::Freedom); }
    }
    assert!(coarse.relation_shares(None).freedom <= fine.relation_shares(None).freedom);
    // PC2: for every pair evaluated at both δ, the same implication holds.
    let (fc, cc) = (fine.across_steps.unwrap(), coarse.across_steps.unwrap());
    for c in cc.readings.iter().filter(|x| x.reading == Reading::Freedom) {
        let f = fc.readings.iter().find(|x| x.e == c.e && x.f == c.f && x.over == c.over).unwrap();
        assert_eq!(f.reading, Reading::Freedom);
    }
}

#[test]
fn pc_d2_structure_at_very_coarse_delta_is_a_statement_about_delta() {
    // At a δ larger than every change, every relation reads structure. The
    // values still changed: F_δ = 0 is read under W2, never as "no change".
    let (r, seq) = chain_series(100.0);
    let s = EdgeSeries::from_registrations(&seq, &r).unwrap();
    let pc = PersistenceChange::new(&s);
    assert_eq!(pc.relation_shares(None).f_delta(), Some(0.0));
    assert!(pc.relations.iter().any(|x| x.change != 0.0));
}

// ── PC-D3 invariance ───────────────────────────────────────────────────────

fn all_readings(pc: &PersistenceChange) -> Vec<Reading> {
    let mut v: Vec<Reading> = pc.relations.iter().map(|x| x.reading).collect();
    for lvl in [&pc.across_components, &pc.across_steps] {
        v.extend(lvl.as_ref().unwrap().readings.iter().map(|x| x.reading));
    }
    v.extend(pc.arrangement.as_ref().unwrap().iter().map(|x| x.reading));
    v
}

#[test]
fn pc_d3_offsets_and_rescaling_change_no_reading() {
    let (r, seq) = chain_series(0.01);
    let base = all_readings(&PersistenceChange::new(&EdgeSeries::from_registrations(&seq, &r).unwrap()));
    // Offset every component at every locus.
    let shifted: Vec<Registration> = seq.iter().map(|g| {
        let data = g.x.data.iter().map(|c| c.iter().map(|v| v + 37.0).collect()).collect();
        Registration::synthetic(NodeField::new(data), g.delta.clone())
    }).collect();
    assert_eq!(base, all_readings(&PersistenceChange::new(&EdgeSeries::from_registrations(&shifted, &r).unwrap())));
    // Rescale component 1 (values and δ) by a negative factor.
    let scaled: Vec<Registration> = seq.iter().map(|g| {
        let mut data = g.x.data.clone();
        let mut delta = g.delta.clone();
        data[1].iter_mut().for_each(|v| *v *= -3.0);
        delta[1].iter_mut().for_each(|d| *d *= 3.0);
        Registration::synthetic(NodeField::new(data), delta)
    }).collect();
    assert_eq!(base, all_readings(&PersistenceChange::new(&EdgeSeries::from_registrations(&scaled, &r).unwrap())));
}

#[test]
fn pc_d3_swapping_edges_negates_minors_and_keeps_readings() {
    let a: Vec<Vec<Vec<f64>>> = (0..3).map(|t| { let t = t as f64; vec![vec![t, t * t + 1.0]] }).collect();
    let swapped: Vec<Vec<Vec<f64>>> = a.iter().map(|r| vec![vec![r[0][1], r[0][0]]]).collect();
    let (x, y) = (coupling_across_steps(&edge_series(a, 0.01)).unwrap(),
                  coupling_across_steps(&edge_series(swapped, 0.01)).unwrap());
    for (p, q) in x.readings.iter().zip(&y.readings) {
        assert!((p.minor + q.minor).abs() < 1e-12);
        assert_eq!(p.reading, q.reading);
    }
}

// ── PC-D4 forced constraint ────────────────────────────────────────────────

#[test]
fn pc_d4_parallel_node_derived_edges_are_identities_not_readings() {
    // Two declared edges 0 → 1 over node values: identical A by construction.
    let r = rel(3, vec![(0, 1), (0, 1), (1, 2)]);
    let seq: Vec<Registration> = (0..3).map(|t| {
        let t = t as f64;
        reg(vec![vec![t * t, 0.0, -t], vec![2.0 * t, 1.0, t * t]], 0.01)
    }).collect();
    let s = EdgeSeries::from_registrations(&seq, &r).unwrap();
    assert_eq!(s.forced_pairs, vec![(0, 1)]);
    let pc = PersistenceChange::new(&s);
    for lvl in [&pc.across_components, &pc.across_steps] {
        assert!(lvl.as_ref().unwrap().readings.iter().all(|x| (x.e, x.f) != (0, 1)));
    }
    assert!(pc.arrangement.unwrap().iter().all(|x| (x.e, x.f) != (0, 1)));
}

#[test]
fn pc_d4_parallel_edge_registered_relations_are_read() {
    // The same endpoints, registered independently: a genuine reading.
    let a: Vec<Vec<Vec<f64>>> = (0..3).map(|t| { let t = t as f64; vec![vec![t, 3.0 * t * t]] }).collect();
    let s = edge_series(a, 0.01);
    assert!(s.forced_pairs.is_empty());
    assert!(coupling_across_steps(&s).unwrap().readings.iter().any(|x| (x.e, x.f) == (0, 1)));
}

// ── PC-D5 minimum information ──────────────────────────────────────────────

#[test]
fn pc_d5_declaration_gaps() {
    let (r, seq) = chain_series(0.01);
    assert_eq!(EdgeSeries::from_registrations(&seq[..1], &r).unwrap_err(), DeclarationGap::NoProcessSequence);
    let two = EdgeSeries::from_registrations(&seq[..2], &r).unwrap();
    assert_eq!(coupling_across_steps(&two).unwrap_err(), DeclarationGap::NoProcessSequence);
    assert!(coupling_across_components(&two).is_ok());
    assert!(arrangement_steps(&two).is_ok());
    assert_eq!(relation_changes(&two).len(), 2 * 3);
}

// ── Energy as a component (Declared 2026-11-21, 7th) ───────────────────────

#[test]
fn energy_carrier_as_a_component_reads_through_the_same_forms() {
    // Loci 0 → 1 → 2. Component 0: a structural quantity. Component 1: the
    // energy carrier registered at the same loci (T5). Over the first steps
    // the carrier's exchange changes in proportion to the structural change
    // (locked); at the last step the structure holds while energy still flows.
    let r = rel(3, vec![(0, 1), (1, 2)]);
    let structural = [0.0, 1.0, 2.0, 2.0];
    let carrier = [0.0, 3.0, 6.0, 8.0];
    let seq: Vec<Registration> = (0..4).map(|t| reg(vec![
        vec![4.0, 2.0 + structural[t], 0.0],
        vec![9.0, 5.0 + carrier[t], 1.0],
    ], 0.01)).collect();
    let s = EdgeSeries::from_registrations(&seq, &r).unwrap();
    let pc = PersistenceChange::new(&s);
    let at = |step, c, e| pc.relations.iter()
        .find(|x| x.step == step && x.component == c && x.edge == e).unwrap().reading;
    // PC1 on the carrier: the exchange changes at every step.
    assert!((0..3).all(|st| at(st, 1, 0) == Reading::Freedom));
    // PC1 on the structure: change at steps 0 and 1, persistence at step 2.
    assert_eq!(at(1, 0, 0), Reading::Freedom);
    assert_eq!(at(2, 0, 0), Reading::Structure);
    // PC2(a) between the carrier and the structural component: at each step
    // both edges change by (u, −u) in both components, so their change is
    // locked: structure; the coupling is read, not assumed.
    let c = pc.across_components.as_ref().unwrap();
    assert!(c.readings.iter().filter(|x| matches!(x.over, ReadingPair::Components { step: 0, .. }))
        .all(|x| x.reading == Reading::Structure));
    // The exchange itself is still read as V3 energy on the carrier.
    let carriers: Vec<_> = seq.iter().map(|g| abr_kernel::ground_variables::EnergyCarrier { range: abr_kernel::ground_variables::ObservedRange::synthetic(),
        name: "carrier".into(), unit: "J".into(), values: g.x.data[1].clone(), delta: vec![0.01; 3],
    }).collect();
    let e = abr_kernel::ground_variables::energy(Some(&carriers), 4, &r).unwrap();
    assert!(e.share_resolved > 0.0);
}

// ── V5 restated: rate of relational evolution as counted steps ─────────────

#[test]
fn v5_rate_is_counted_steps_never_time() {
    // Edge 0 changes at steps 0 and 2; edge 1 never changes. Four
    // registrations, three steps. No clock enters.
    let a = vec![
        vec![vec![1.0, 5.0]], vec![vec![2.0, 5.0]], vec![vec![2.0, 5.0]], vec![vec![4.0, 5.0]],
    ];
    let pc = PersistenceChange::new(&edge_series(a, 0.01));
    let c = pc.evolution_counts();
    let e0 = c.iter().find(|x| x.edge == 0).unwrap();
    let e1 = c.iter().find(|x| x.edge == 1).unwrap();
    assert_eq!((e0.changed_steps, e0.steps), (2, 3));
    assert_eq!((e1.changed_steps, e1.steps), (0, 3));
    assert!((e0.share() - 2.0 / 3.0).abs() < 1e-12);
    // Γ summed over relations equals PC1's freedom count (one rate, one reading).
    let total: usize = c.iter().map(|x| x.changed_steps).sum();
    assert_eq!(total, pc.relation_shares(None).freedom);
}

// ── W14: the photon-edge carrier (2026-11-21, 12th) ────────────────────────

#[test]
fn photon_carrier_wavenumber_and_hc_over_lambda_read_identically() {
    use abr_kernel::derived_invariants::{C_DECLARED, H_PLANCK};
    use abr_kernel::ground_variables::{signed_exchanges, mapped_half_width, EnergyCarrier};
    // Registered wavelengths (m) at three loci over four registrations, with
    // δ_λ. hc/λ is a construct mapping; the wavenumber 1/λ is the registered
    // quantity in another unit. Every V10 reading must agree.
    let r = rel(3, vec![(0, 1), (1, 2)]);
    let lam = [[121.6e-9, 102.6e-9, 97.3e-9], [121.6e-9, 102.0e-9, 97.3e-9],
               [121.5e-9, 101.0e-9, 97.0e-9], [121.5e-9, 101.0e-9, 96.0e-9]];
    let d_lam = 0.05e-9;
    let hc = H_PLANCK * C_DECLARED;
    let carrier = |scale: f64, t: usize| EnergyCarrier { range: abr_kernel::ground_variables::ObservedRange::synthetic(),
        name: "photon edge".into(), unit: if scale == 1.0 { "1/m".into() } else { "J (Projected)".into() },
        values: lam[t].iter().map(|l| scale / l).collect(),
        delta: lam[t].iter().map(|&l| mapped_half_width(|x| scale / x, l, d_lam)).collect(),
    };
    for t in 0..4 {
        // The sides agree (104th). *Was:* the floors and the degrees ε/δ_ε
        // (the degree retired with the magnitude).
        let (k, e) = (signed_exchanges(&carrier(1.0, t), &r), signed_exchanges(&carrier(hc, t), &r));
        for (a, b) in k.iter().zip(&e) {
            assert_eq!(a.side, b.side);
        }
    }
    // The carrier as a component: PC readings agree.
    let series = |scale: f64| {
        let seq: Vec<Registration> = (0..4).map(|t| {
            let c = carrier(scale, t);
            Registration::synthetic(NodeField::new(vec![c.values.clone()]), vec![c.delta.clone()])
        }).collect();
        PersistenceChange::new(&EdgeSeries::from_registrations(&seq, &r).unwrap())
    };
    let (k, e) = (series(1.0), series(hc));
    let readings = |p: &PersistenceChange| p.relations.iter().map(|x| x.reading).collect::<Vec<_>>();
    assert_eq!(readings(&k), readings(&e));
    // Not vacuous: both structure and freedom occur.
    assert!(readings(&k).contains(&Reading::Freedom) && readings(&k).contains(&Reading::Structure));
}

// ── Step profile (2026-11-21) ───────────────────────────────────────────────

#[test]
fn step_profile_partitions_every_level_by_step() {
    let (r, seq) = chain_series(0.01);
    let pc = PersistenceChange::new(&EdgeSeries::from_registrations(&seq, &r).unwrap());
    let prof = pc.step_profile();
    assert_eq!(prof.len(), 3);
    let sum = |f: &dyn Fn(&StepProfile) -> Option<Shares>| prof.iter().filter_map(|p| f(p))
        .fold((0, 0), |(a, b), s| (a + s.structure, b + s.freedom));
    let total = |r: &[Reading]| (r.iter().filter(|x| **x == Reading::Structure).count(), r.iter().filter(|x| **x == Reading::Freedom).count());
    assert_eq!(sum(&|p| Some(p.relations)), total(&pc.relations.iter().map(|x| x.reading).collect::<Vec<_>>()));
    assert_eq!(sum(&|p| p.across_components), total(&pc.across_components.as_ref().unwrap().readings.iter().map(|x| x.reading).collect::<Vec<_>>()));
    assert_eq!(sum(&|p| p.across_steps), total(&pc.across_steps.as_ref().unwrap().readings.iter().map(|x| x.reading).collect::<Vec<_>>()));
    assert_eq!(sum(&|p| p.arrangement), total(&pc.arrangement.as_ref().unwrap().iter().map(|x| x.reading).collect::<Vec<_>>()));
}

// ── Relation transitions (Origin, 2026-11-21) ──────────────────────────────

#[test]
fn transitions_are_turns_of_a_held_reading_in_either_direction() {
    // One relation: held structure, then held freedom, a one-step
    // fluctuation, then held structure again.
    //   values: 0 0 0 0 1 2 3 4 4 5 5 5 5
    //   PC1:     S S S F F F F S F S S S      (δ small)
    let v = [0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0, 5.0, 5.0];
    let a: Vec<Vec<Vec<f64>>> = v.iter().map(|&x| vec![vec![x]]).collect();
    let pc = PersistenceChange::new(&edge_series(a, 0.01));
    let t = pc.relation_transitions(3);
    assert_eq!(t.len(), 2);
    assert_eq!((t[0].step, t[0].to), (3, Reading::Freedom));   // freedom becomes evident
    assert_eq!((t[1].step, t[1].to), (9, Reading::Structure)); // persistent structure becomes evident
    // The one-step structure at step 7 and the freedom at step 8 are not transitions at min_hold 3.
    assert_eq!(pc.relation_transitions(1).len(), 4);
}

#[test]
fn a_fluctuation_back_to_the_held_state_is_not_a_transition() {
    // S S S F S S S: the single freedom does not hold, and the structure after
    // it is the same held state as before.
    let v = [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let a: Vec<Vec<Vec<f64>>> = v.iter().map(|&x| vec![vec![x]]).collect();
    let pc = PersistenceChange::new(&edge_series(a, 0.01));
    assert!(pc.relation_transitions(2).is_empty());
}
