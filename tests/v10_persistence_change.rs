//! Kernel V10 — structure and freedom as persistence and change (PC1–PC4).
//! PROPOSED forms. Each test checks one derived property (PC-D1 to PC-D5) or
//! one reading.
//!
//! [IMPL] SYNTHETIC FIXTURES: values are chosen to exercise the forms. They
//! establish that the forms compute as stated, nothing empirical.

use abr_kernel::ground_variables::{DeclarationGap, Registration};
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::persistence_change::*;

fn rel(n: usize, edges: Vec<(usize, usize)>) -> DeclaredRelations {
    let m = edges.len();
    DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m])
}

fn reg(data: Vec<Vec<f64>>, d: f64) -> Registration {
    let (k, n) = (data.len(), data[0].len());
    Registration::new(NodeField::new(data), vec![vec![d; n]; k])
}

/// Edge-registered series: a[t][c][e], uniform δ.
fn edge_series(a: Vec<Vec<Vec<f64>>>, d: f64) -> EdgeSeries {
    let da = a.iter().map(|t| t.iter().map(|c| vec![d; c.len()]).collect()).collect();
    EdgeSeries::from_edge_values(a, da).unwrap()
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
        Registration::new(NodeField::new(data), g.delta.clone())
    }).collect();
    assert_eq!(base, all_readings(&PersistenceChange::new(&EdgeSeries::from_registrations(&shifted, &r).unwrap())));
    // Rescale component 1 (values and δ) by a negative factor.
    let scaled: Vec<Registration> = seq.iter().map(|g| {
        let mut data = g.x.data.clone();
        let mut delta = g.delta.clone();
        data[1].iter_mut().for_each(|v| *v *= -3.0);
        delta[1].iter_mut().for_each(|d| *d *= 3.0);
        Registration::new(NodeField::new(data), delta)
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
