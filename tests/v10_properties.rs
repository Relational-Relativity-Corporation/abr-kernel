//! Kernel V10 — property tests for PC-D1 to PC-D5 and I-SC-D1 to I-SC-D8.
//!
//! Each property is checked against many randomly generated fields: random
//! topologies, components, registration counts, values and δ. The generator
//! is seeded and self-contained (no external dependency, per Cargo.toml), so
//! every case is reproducible from the seed printed on failure.
//!
//! Values are multiples of 1/8 and δ multiples of 1/16, within small ranges,
//! so every sum, difference and product here is exact in f64. Boundary
//! comparisons (|x| > δ) are therefore exact, and no property can pass or
//! fail by rounding.
//!
//! [IMPL] SYNTHETIC: these establish that the mathematics holds as stated
//! over the forms, nothing empirical.

#[path = "support/synthetic_feedback.rs"]
mod synthetic_feedback;
#[path = "support/synthetic_identity.rs"]
mod synthetic_identity;
use abr_kernel::ground_variables::*;
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::persistence_change::*;
use abr_kernel::relational_field::RelationalField;

const CASES: u64 = 2000;

use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

/// Counts how often a conditional property was actually exercised, so no
/// property passes vacuously.
fn hit(c: &AtomicUsize) { c.fetch_add(1, Relaxed); }
fn exercised(c: &AtomicUsize, name: &str) {
    let n = c.load(Relaxed);
    assert!(n > 0, "{name}: condition never occurred, property untested");
    eprintln!("{name}: exercised {n} times");
}

// ── Seeded generator (SplitMix64) ───────────────────────────────────────────

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn int(&mut self, lo: i64, hi: i64) -> i64 { lo + (self.next() % (hi - lo + 1) as u64) as i64 }
    fn usize(&mut self, lo: usize, hi: usize) -> usize { self.int(lo as i64, hi as i64) as usize }
    /// A multiple of 1/8 in [lo, hi].
    fn eighth(&mut self, lo: f64, hi: f64) -> f64 { self.int((lo * 8.0) as i64, (hi * 8.0) as i64) as f64 / 8.0 }
    /// A δ: a multiple of 1/16 in [0, hi].
    fn delta(&mut self, hi: f64) -> f64 { self.int(0, (hi * 16.0) as i64) as f64 / 16.0 }
    fn chance(&mut self, p_in_8: u64) -> bool { self.next() % 8 < p_in_8 }
}

/// Run `prop` on CASES seeds; report the seed on failure.
fn for_all(name: &str, prop: impl Fn(&mut Rng)) {
    for seed in 0..CASES {
        let mut r = Rng(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ 0xA5A5);
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| prop(&mut r)));
        if caught.is_err() { panic!("property {name} failed at seed {seed}"); }
    }
}

// ── Random declarations ─────────────────────────────────────────────────────

fn random_rel(r: &mut Rng, allow_parallel: bool) -> DeclaredRelations {
    let n = r.usize(3, 7);
    // Without parallel edges there are at most n(n − 1) distinct directed edges.
    let cap = if allow_parallel { 9 } else { 9.min(n * (n - 1)) };
    let m = r.usize(2, cap);
    let mut edges: Vec<(usize, usize)> = Vec::new();
    while edges.len() < m {
        let (s, t) = (r.usize(0, n - 1), r.usize(0, n - 1));
        if s == t { continue; }
        if !allow_parallel && edges.contains(&(s, t)) { continue; }
        edges.push((s, t));
    }
    synthetic_feedback::declare_loops(synthetic_identity::identify(DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m])))
}

/// A sequence of node-valued registrations: a random walk per locus and
/// component, with frequent zero steps so persistence occurs.
fn random_seq(r: &mut Rng, n: usize, k: (usize, usize), regs: (usize, usize), d_hi: f64) -> Vec<Registration> {
    let (k, regs) = (r.usize(k.0, k.1), r.usize(regs.0, regs.1));
    let mut x: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| r.eighth(-4.0, 4.0)).collect()).collect();
    let delta: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| r.delta(d_hi)).collect()).collect();
    let mut out = Vec::new();
    for _ in 0..regs {
        out.push(Registration::synthetic(NodeField::new(x.clone()), delta.clone()));
        for c in x.iter_mut() {
            for v in c.iter_mut() {
                if r.chance(5) { *v += r.eighth(-2.0, 2.0); }
            }
        }
    }
    out
}

/// Random edge-registered series a[t][c][e] with δ.
fn random_edges(r: &mut Rng, m: (usize, usize), k: (usize, usize), regs: (usize, usize), d_hi: f64) -> EdgeSeries {
    let (m, k, regs) = (r.usize(m.0, m.1), r.usize(k.0, k.1), r.usize(regs.0, regs.1));
    let mut a: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| r.eighth(-4.0, 4.0)).collect()).collect();
    let da: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| r.delta(d_hi)).collect()).collect();
    let (mut aa, mut dd) = (Vec::new(), Vec::new());
    for _ in 0..regs {
        aa.push(a.clone());
        dd.push(da.clone());
        for c in a.iter_mut() {
            for v in c.iter_mut() {
                if r.chance(5) { *v += r.eighth(-2.0, 2.0); }
            }
        }
    }
    EdgeSeries::from_edge_values_synthetic(aa, dd).unwrap()
}

fn widen(seq: &[Registration], r: &mut Rng) -> Vec<Registration> {
    // δ' ≥ δ at every value; the same for every registration.
    let k = seq[0].x.k;
    let n = seq[0].x.n;
    let extra: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| r.delta(0.25)).collect()).collect();
    seq.iter().map(|g| {
        let delta = g.delta.iter().zip(&extra).map(|(d, x)| d.iter().zip(x).map(|(a, b)| a + b).collect()).collect();
        Registration::synthetic(g.x.clone(), delta)
    }).collect()
}

// ═══════════════════════════════════════════════════════════════════════════
// PC-D1 to PC-D5
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn prop_pc_d1_shares_partition_every_level() {
    for_all("PC-D1", |r| {
        let rel = random_rel(r, true);
        let seq = random_seq(r, rel.n_nodes, (1, 3), (2, 6), 0.5);
        let pc = PersistenceChange::new(&EdgeSeries::from_registrations(&seq, &rel).unwrap());
        let sh = pc.relation_shares(None);
        assert_eq!(sh.evaluated(), pc.relations.len());
        for lvl in [&pc.across_components, &pc.across_steps] {
            if let Ok(c) = lvl {
                assert_eq!(Shares::of(c.readings.iter().map(|x| &x.reading)).evaluated(), c.readings.len());
            }
        }
        if let (Some(f), Some(s)) = (sh.f_delta(), sh.s_delta()) {
            assert!((f + s - 1.0).abs() < 1e-12);
        }
    });
}

#[test]
fn prop_pc_d2_coarser_delta_never_adds_freedom_pc1_pc2() {
    static N1: AtomicUsize = AtomicUsize::new(0);
    static N2: AtomicUsize = AtomicUsize::new(0);
    for_all("PC-D2 (PC1, PC2)", |r| {
        let rel = random_rel(r, false);
        let seq = random_seq(r, rel.n_nodes, (1, 3), (2, 6), 0.5);
        let wide = widen(&seq, r);
        let fine = PersistenceChange::new(&EdgeSeries::from_registrations(&seq, &rel).unwrap());
        let coarse = PersistenceChange::new(&EdgeSeries::from_registrations(&wide, &rel).unwrap());
        for (f, c) in fine.relations.iter().zip(&coarse.relations) {
            if c.reading == Reading::Freedom { assert_eq!(f.reading, Reading::Freedom); hit(&N1); }
        }
        for (fl, cl) in [(&fine.across_components, &coarse.across_components), (&fine.across_steps, &coarse.across_steps)] {
            if let (Ok(fl), Ok(cl)) = (fl, cl) {
                for c in cl.readings.iter().filter(|x| x.reading == Reading::Freedom) {
                    if let Some(f) = fl.readings.iter().find(|x| x.e == c.e && x.f == c.f && x.over == c.over) {
                        assert_eq!(f.reading, Reading::Freedom);
                        hit(&N2);
                    }
                }
            }
        }
    });
    exercised(&N1, "PC-D2 PC1 coarse freedom");
    exercised(&N2, "PC-D2 PC2 coarse freedom");
}

#[test]
fn prop_pc_d2_crossings_are_monotone_in_delta() {
    static N: AtomicUsize = AtomicUsize::new(0);
    // The PC3 statement that does hold: a crossing (opposite resolved signs)
    // at δ' is a crossing at every δ ≤ δ'.
    for_all("PC-D2 (PC3 crossings)", |r| {
        let rel = random_rel(r, false);
        let seq = random_seq(r, rel.n_nodes, (2, 3), (2, 6), 0.5);
        let wide = widen(&seq, r);
        let fine = arrangement_steps(&EdgeSeries::from_registrations(&seq, &rel).unwrap()).unwrap();
        let coarse = arrangement_steps(&EdgeSeries::from_registrations(&wide, &rel).unwrap()).unwrap();
        for c in &coarse {
            if let (AtDelta::Resolved(p), AtDelta::Resolved(q)) = (c.before, c.after) {
                if p != q {
                    let f = fine.iter().find(|x| (x.step, x.e, x.f, x.i, x.j) == (c.step, c.e, c.f, c.i, c.j))
                        .expect("edges resolved at δ' are resolved at δ");
                    assert_eq!((f.before, f.after), (c.before, c.after));
                    hit(&N);
                }
            }
        }
    });
    exercised(&N, "PC-D2 PC3 crossings");
}

#[test]
fn pc3_freedom_is_not_monotone_in_delta() {
    // Counterexample to extending PC-D2 to PC3. One minor: 1.0 at s, 0.5 at
    // s+1. At a fine δ both states are resolved positive (structure). At a
    // coarser δ the second state is not resolved, so the state changes
    // (freedom). PC3 compares two states, so coarsening can create freedom.
    let a = vec![
        vec![vec![1.0, 0.0], vec![0.0, 1.0]],
        vec![vec![1.0, 0.0], vec![0.0, 0.5]],
    ];
    let with = |d: f64| {
        let da = a.iter().map(|t| t.iter().map(|c| vec![d; c.len()]).collect()).collect();
        arrangement_steps(&EdgeSeries::from_edge_values_synthetic(a.clone(), da).unwrap()).unwrap()[0].reading
    };
    assert_eq!(with(0.0625), Reading::Structure);
    assert_eq!(with(0.25), Reading::Freedom);
}

#[test]
fn prop_pc_d3_offsets_and_rescaling_change_no_reading() {
    for_all("PC-D3", |r| {
        let rel = random_rel(r, false);
        let k = r.usize(1, 3);
        let seq = random_seq(r, rel.n_nodes, (k, k), (2, 5), 0.5);
        let readings = |seq: &[Registration]| {
            let pc = PersistenceChange::new(&EdgeSeries::from_registrations(seq, &rel).unwrap());
            let mut v: Vec<Reading> = pc.relations.iter().map(|x| x.reading).collect();
            for lvl in [&pc.across_components, &pc.across_steps] {
                if let Ok(c) = lvl { v.extend(c.readings.iter().map(|x| x.reading)); }
            }
            if let Ok(a) = &pc.arrangement { v.extend(a.iter().map(|x| x.reading)); }
            v
        };
        let base = readings(&seq);
        // Offset one component at every locus and registration (exact).
        let (c, off) = (r.usize(0, k - 1), r.eighth(-8.0, 8.0));
        let shifted: Vec<Registration> = seq.iter().map(|g| {
            let mut d = g.x.data.clone();
            d[c].iter_mut().for_each(|v| *v += off);
            Registration::synthetic(NodeField::new(d), g.delta.clone())
        }).collect();
        assert_eq!(base, readings(&shifted));
        // Rescale one component by ±2^p (exact), δ by 2^p.
        let p = r.int(-2, 3) as i32;
        let lam = 2f64.powi(p) * if r.chance(4) { -1.0 } else { 1.0 };
        let scaled: Vec<Registration> = seq.iter().map(|g| {
            let (mut d, mut dl) = (g.x.data.clone(), g.delta.clone());
            d[c].iter_mut().for_each(|v| *v *= lam);
            dl[c].iter_mut().for_each(|v| *v *= lam.abs());
            Registration::synthetic(NodeField::new(d), dl)
        }).collect();
        assert_eq!(base, readings(&scaled));
    });
}

#[test]
fn prop_pc_d3_swapping_edges_negates_change_minors() {
    for_all("PC-D3 (edge swap)", |r| {
        let k = r.usize(1, 3);
        let s = random_edges(r, (2, 2), (k, k), (3, 5), 0.5);
        let swapped = EdgeSeries::from_edge_values_synthetic(
            s.a.iter().map(|t| t.iter().map(|c| vec![c[1], c[0]]).collect()).collect(),
            s.da.iter().map(|t| t.iter().map(|c| vec![c[1], c[0]]).collect()).collect()).unwrap();
        let (x, y) = (coupling_across_steps(&s).unwrap(), coupling_across_steps(&swapped).unwrap());
        assert_eq!(x.readings.len(), y.readings.len());
        for (p, q) in x.readings.iter().zip(&y.readings) {
            assert_eq!(p.minor, -q.minor);
            assert_eq!(p.reading, q.reading);
        }
    });
}

#[test]
fn prop_pc_d4_parallel_node_edges_are_identities() {
    static N: AtomicUsize = AtomicUsize::new(0);
    for_all("PC-D4", |r| {
        let rel = random_rel(r, true);
        let seq = random_seq(r, rel.n_nodes, (2, 3), (3, 5), 0.5);
        let s = EdgeSeries::from_registrations(&seq, &rel).unwrap();
        let expected: Vec<(usize, usize)> = (0..rel.n_edges())
            .flat_map(|e| ((e + 1)..rel.n_edges()).map(move |f| (e, f)))
            .filter(|&(e, f)| rel.edges[e] == rel.edges[f]).collect();
        assert_eq!(s.forced_pairs, expected);
        if !expected.is_empty() { hit(&N); }
        let pc = PersistenceChange::new(&s);
        for lvl in [&pc.across_components, &pc.across_steps] {
            if let Ok(c) = lvl { assert!(c.readings.iter().all(|x| !expected.contains(&(x.e, x.f)))); }
        }
        if let Ok(a) = &pc.arrangement { assert!(a.iter().all(|x| !expected.contains(&(x.e, x.f)))); }
    });
    exercised(&N, "PC-D4 parallel edges");
}

#[test]
fn prop_pc_d5_minimum_information() {
    for_all("PC-D5", |r| {
        let rel = random_rel(r, false);
        let two = random_seq(r, rel.n_nodes, (1, 1), (2, 2), 0.5);
        assert_eq!(EdgeSeries::from_registrations(&two[..1], &rel).unwrap_err(), DeclarationGap::NoProcessSequence);
        let s = EdgeSeries::from_registrations(&two, &rel).unwrap();
        assert_eq!(coupling_across_steps(&s).unwrap_err(), DeclarationGap::NoProcessSequence);
        assert_eq!(coupling_across_components(&s).unwrap_err(), DeclarationGap::SingleComponent);
        assert_eq!(arrangement_steps(&s).unwrap_err(), DeclarationGap::SingleComponent);
        assert_eq!(relation_changes(&s).len(), rel.n_edges());
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// I-SC-D1 to I-SC-D8
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn prop_isc_d1_change_bound_is_valid_and_sign_persists() {
    static N: AtomicUsize = AtomicUsize::new(0);
    for_all("I-SC-D1", |r| {
        let s = random_edges(r, (2, 5), (2, 3), (2, 2), 0.0);
        let (d, _) = s.change(0);
        let (m, k) = (s.n_edges(), s.k());
        for e in 0..m { for f in (e + 1)..m { for i in 0..k { for j in (i + 1)..k {
            let (m0, m1) = (minor(&s.a[0], e, f, i, j), minor(&s.a[1], e, f, i, j));
            let bound = minor_change_bound(&s.a[0], &d, e, f, i, j);
            assert!((m1 - m0).abs() <= bound, "the change bound must bound |Δm|");
            if step_preserves_sign(m0, bound) { assert_eq!(m0.signum(), m1.signum()); hit(&N); }
        }}}}
    });
    exercised(&N, "I-SC-D1 sign preserved");
}

#[test]
fn prop_isc_d2_freedom_and_connection_partition_each_minor() {
    for_all("I-SC-D2", |r| {
        let s = random_edges(r, (2, 5), (2, 3), (2, 2), 0.5);
        for x in arrangement_steps(&s).unwrap() {
            // Each state is exactly one of resolved (freedom) or not (connection).
            for st in [x.before, x.after] {
                assert!(matches!(st, AtDelta::Resolved(_)) ^ matches!(st, AtDelta::NotResolved));
            }
        }
    });
}

#[test]
fn prop_isc_d3_d4_fixed_ratio_or_offset_locks_change() {
    for_all("I-SC-D3/D4", |r| {
        let (k, regs) = (r.usize(2, 3), r.usize(3, 5));
        let lam = r.eighth(-3.0, 3.0);
        let off: Vec<f64> = (0..k).map(|_| if r.chance(4) { r.eighth(-4.0, 4.0) } else { 0.0 }).collect();
        let mut e: Vec<f64> = (0..k).map(|_| r.eighth(-4.0, 4.0)).collect();
        let mut a = Vec::new();
        for _ in 0..regs {
            a.push((0..k).map(|c| vec![e[c], lam * e[c] + off[c]]).collect::<Vec<_>>());
            for v in e.iter_mut() { *v += r.eighth(-2.0, 2.0); }
        }
        let da = a.iter().map(|t| t.iter().map(|c| vec![0.0625; c.len()]).collect()).collect();
        let s = EdgeSeries::from_edge_values_synthetic(a, da).unwrap();
        for lvl in [coupling_across_components(&s), coupling_across_steps(&s)] {
            for x in lvl.unwrap().readings {
                assert_eq!(x.minor, 0.0);
                assert_eq!(x.reading, Reading::Structure);
            }
        }
    });
}

#[test]
fn prop_isc_d5_circulation_additive_and_sign_persists() {
    static N: AtomicUsize = AtomicUsize::new(0);
    for_all("I-SC-D5", |r| {
        let rel = random_rel(r, false);
        let seq = random_seq(r, rel.n_nodes, (1, 3), (2, 5), 0.0);
        let f = RelationalField::new(&rel, seq, None).unwrap();
        for s in 0..f.n_steps() {
            for c in 0..f.spatial[s].circulation.len() {
                for e in 0..rel.n_edges() {
                    let (now, ch) = (f.spatial[s].circulation[c][e], f.persistence[s].circulation[c][e]);
                    let next = f.spatial[s + 1].circulation[c][e];
                    assert_eq!(next, now + ch);
                    if step_preserves_sign(now, ch.abs()) && ch != 0.0 { assert_eq!(now.signum(), next.signum()); hit(&N); }
                }
            }
        }
    });
    exercised(&N, "I-SC-D5 circulation sign preserved under change");
}

#[test]
fn prop_isc_d6_uniform_rescaling_holds_every_sign() {
    for_all("I-SC-D6", |r| {
        let rel = random_rel(r, false);
        let k = r.usize(2, 3);
        let base: Vec<Vec<f64>> = (0..k).map(|_| (0..rel.n_nodes).map(|_| r.eighth(-4.0, 4.0)).collect()).collect();
        let kappa = 2f64.powi(r.int(-2, 3) as i32);
        let scaled: Vec<Vec<f64>> = base.iter().map(|c| c.iter().map(|v| kappa * v).collect()).collect();
        let zero = vec![vec![0.0; rel.n_nodes]; k];
        let seq = vec![Registration::synthetic(NodeField::new(base), zero.clone()),
                       Registration::synthetic(NodeField::new(scaled), zero)];
        let f = RelationalField::new(&rel, seq.clone(), None).unwrap();
        for c in 0..k { for e in 0..rel.n_edges() {
            assert_eq!(f.spatial[1].circulation[c][e], kappa * f.spatial[0].circulation[c][e]);
        }}
        let s = EdgeSeries::from_registrations(&seq, &rel).unwrap();
        for x in arrangement_steps(&s).unwrap() { assert_eq!(x.reading, Reading::Structure); }
    });
}

#[test]
fn prop_isc_d7a_same_sign_halves_resolve_the_whole() {
    for_all("I-SC-D7(a)", |r| {
        let v = [r.eighth(-4.0, 4.0), 0.0, 0.0];
        let sign = if r.chance(4) { 1.0 } else { -1.0 };
        let (d0, d1, d2) = (r.delta(0.5), r.delta(0.5), r.delta(0.5));
        let h1 = sign * (d0 + d1 + 0.125 * r.int(1, 16) as f64);
        let h2 = sign * (d1 + d2 + 0.125 * r.int(1, 16) as f64);
        let v = [v[0], v[0] + h1, v[0] + h1 + h2];
        let whole = v[2] - v[0];
        // Both halves are resolved with one sign by construction.
        assert!(h1.abs() > d0 + d1 && h2.abs() > d1 + d2);
        assert!(whole.abs() > d0 + d2 && whole.signum() == sign);
        // And the forms agree.
        let series = |vals: &[f64], ds: &[f64]| EdgeSeries::from_edge_values_synthetic(
            vals.iter().map(|&x| vec![vec![x]]).collect(), ds.iter().map(|&d| vec![vec![d]]).collect()).unwrap();
        let halves = relation_changes(&series(&v, &[d0, d1, d2]));
        let coarse = relation_changes(&series(&[v[0], v[2]], &[d0, d2]));
        assert!(halves.iter().all(|x| x.reading == Reading::Freedom));
        assert_eq!(coarse[0].reading, Reading::Freedom);
    });
}

#[test]
fn prop_isc_d8_crossing_needs_change_above_both_resolutions() {
    static N: AtomicUsize = AtomicUsize::new(0);
    for_all("I-SC-D8", |r| {
        let s = random_edges(r, (2, 5), (2, 3), (2, 5), 0.5);
        for x in arrangement_steps(&s).unwrap() {
            if let (AtDelta::Resolved(p), AtDelta::Resolved(q)) = (x.before, x.after) {
                if p != q {
                    let (d, _) = s.change(x.step);
                    let bound = minor_change_bound(&s.a[x.step], &d, x.e, x.f, x.i, x.j);
                    let hw = minor_half_width(&s.a[x.step], &s.da[x.step], x.e, x.f, x.i, x.j)
                        + minor_half_width(&s.a[x.step + 1], &s.da[x.step + 1], x.e, x.f, x.i, x.j);
                    assert!(bound > hw);
                    hit(&N);
                }
            }
        }
    });
    exercised(&N, "I-SC-D8 crossings");
}

#[test]
fn prop_energy_readings_invariant_under_carrier_rescaling() {
    // The W14 result behind the wavenumber carrier: multiplying a carrier and
    // its δ by one constant changes no energy reading.
    for_all("V3 rescaling", |r| {
        let rel = random_rel(r, false);
        let vals: Vec<f64> = (0..rel.n_nodes).map(|_| r.eighth(-4.0, 4.0)).collect();
        let del: Vec<f64> = (0..rel.n_nodes).map(|_| r.delta(0.5)).collect();
        let kappa = 2f64.powi(r.int(-3, 6) as i32);
        let c = |s: f64| EnergyCarrier { range: abr_kernel::ground_variables::ObservedRange::synthetic(), name: "c".into(), unit: "u".into(),
            values: vals.iter().map(|v| s * v).collect(), delta: del.iter().map(|d| s * d).collect() };
        let (x, y) = (energy_exchange(&c(1.0), &rel), energy_exchange(&c(kappa), &rel));
        for (p, q) in x.iter().zip(&y) {
            assert_eq!(p.floor_resolved, q.floor_resolved);
            assert_eq!(p.degree(), q.degree());
        }
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// I-SC-D9 to I-SC-D13
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn prop_isc_d9_coupled_change_is_transitive_exactly() {
    static N: AtomicUsize = AtomicUsize::new(0);
    for_all("I-SC-D9", |r| {
        // f arbitrary and non-zero in change; e = α·f, g = β·f in change.
        let (al, be) = (r.eighth(-3.0, 3.0), r.eighth(-3.0, 3.0));
        let regs = r.usize(3, 5);
        let mut f = r.eighth(-4.0, 4.0);
        let mut a = Vec::new();
        for _ in 0..regs {
            a.push(vec![vec![al * f, f, be * f]]);
            f += r.eighth(-2.0, 2.0);
        }
        let s = EdgeSeries::from_edge_values_synthetic(a.clone(), a.iter().map(|t| vec![vec![0.0; 3]; t.len()]).collect()).unwrap();
        for x in coupling_across_steps(&s).unwrap().readings {
            assert_eq!(x.minor, 0.0);
            if (x.e, x.f) == (0, 2) { hit(&N); }
        }
    });
    exercised(&N, "I-SC-D9 e–g pair evaluated");
}

/// A random closed walk v0 → v1 → … → v0 over L ≥ 3 distinct loci, each step
/// declared as an edge in either direction.
fn random_cycle(r: &mut Rng) -> (DeclaredRelations, Vec<(usize, i8)>) {
    let l = r.usize(3, 6);
    let mut edges = Vec::new();
    let mut cycle = Vec::new();
    for i in 0..l {
        let (u, v) = (i, (i + 1) % l);
        if r.chance(4) { edges.push((u, v)); cycle.push((i, 1)); } else { edges.push((v, u)); cycle.push((i, -1)); }
    }
    let m = edges.len();
    (synthetic_feedback::declare_loops(synthetic_identity::identify(DeclaredRelations::from_edges_with_provenance(l, edges, vec![EdgeProvenance::Continuation; m]))), cycle)
}

#[test]
fn prop_isc_d10_closure_on_change() {
    static N: AtomicUsize = AtomicUsize::new(0);
    for_all("I-SC-D10", |r| {
        let (rel, cycle) = random_cycle(r);
        assert!(is_closed_walk(&rel, &cycle));
        let s = random_edges(r, (rel.n_edges(), rel.n_edges()), (1, 1), (2, 4), 0.5);
        for st in 0..s.n_steps() {
            let (c0, h0) = cycle_closure(&s, &rel, &cycle, st, 0).unwrap();
            let (c1, h1) = cycle_closure(&s, &rel, &cycle, st + 1, 0).unwrap();
            let (dc, dh) = cycle_change_closure(&s, &rel, &cycle, st, 0).unwrap();
            assert_eq!(dc, c1 - c0);
            assert_eq!(dh, h0 + h1);
            if c0.abs() <= h0 && c1.abs() <= h1 { assert!(dc.abs() <= dh); hit(&N); }
        }
        // Node-derived: 0 by identity.
        let seq = random_seq(r, rel.n_nodes, (1, 1), (2, 3), 0.5);
        let n = EdgeSeries::from_registrations(&seq, &rel).unwrap();
        for t in 0..n.n_registrations() { assert_eq!(cycle_closure(&n, &rel, &cycle, t, 0).unwrap().0, 0.0); }
    });
    exercised(&N, "I-SC-D10 closure held at both registrations");
}

#[test]
fn prop_isc_d11_total_circulation_is_the_weighted_sum() {
    use abr_kernel::operators::operator_circulation;
    use abr_kernel::relational_field::circulation_weights;
    for_all("I-SC-D11", |r| {
        let rel = random_rel(r, true);
        let w = circulation_weights(&rel);
        let k = r.usize(1, 3);
        let g: Vec<Vec<f64>> = (0..k).map(|_| (0..rel.n_edges()).map(|_| r.eighth(-4.0, 4.0)).collect()).collect();
        let xi = operator_circulation(&g, &rel);
        for c in 0..k {
            let total: f64 = xi[c].iter().sum();
            let weighted: f64 = g[c].iter().zip(&w).map(|(v, &wi)| v * wi as f64).sum();
            assert_eq!(total, weighted);
        }
    });
}

#[test]
fn prop_isc_d12_readings_are_local_to_their_components() {
    for_all("I-SC-D12", |r| {
        let rel = random_rel(r, false);
        let seq = random_seq(r, rel.n_nodes, (2, 3), (3, 5), 0.5);
        let k = seq[0].x.k;
        let drop = r.usize(0, k - 1);
        let kept: Vec<usize> = (0..k).filter(|&c| c != drop).collect();
        let reduced_seq: Vec<Registration> = seq.iter().map(|g| Registration::synthetic(
            NodeField::new(kept.iter().map(|&c| g.x.data[c].clone()).collect()),
            kept.iter().map(|&c| g.delta[c].clone()).collect())).collect();
        let full = PersistenceChange::new(&EdgeSeries::from_registrations(&seq, &rel).unwrap());
        let red = PersistenceChange::new(&EdgeSeries::from_registrations(&reduced_seq, &rel).unwrap());
        // PC1: identical for kept components.
        for x in &red.relations {
            let y = full.relations.iter().find(|y| y.step == x.step && y.edge == x.edge && y.component == kept[x.component]).unwrap();
            assert_eq!(x.reading, y.reading);
        }
        // PC2(b): identical per kept component.
        if let (Ok(fr), Ok(rr)) = (&full.across_steps, &red.across_steps) {
            for x in &rr.readings {
                if let ReadingPair::Steps { component, step } = x.over {
                    let over = ReadingPair::Steps { component: kept[component], step };
                    let y = fr.readings.iter().find(|y| (y.e, y.f) == (x.e, x.f) && y.over == over).unwrap();
                    assert_eq!(x.reading, y.reading);
                }
            }
        }
        // PC2(a): identical per kept component pair.
        if let (Ok(fr), Ok(rr)) = (&full.across_components, &red.across_components) {
            for x in &rr.readings {
                if let ReadingPair::Components { step, i, j } = x.over {
                    let over = ReadingPair::Components { step, i: kept[i], j: kept[j] };
                    let y = fr.readings.iter().find(|y| (y.e, y.f) == (x.e, x.f) && y.over == over).unwrap();
                    assert_eq!((x.minor, x.reading), (y.minor, y.reading));
                }
            }
        }
        // PC3: every reduced reading is a full reading, unchanged; the set can only shrink.
        if let (Ok(fa), Ok(ra)) = (&full.arrangement, &red.arrangement) {
            for x in ra {
                let y = fa.iter().find(|y| (y.step, y.e, y.f, y.i, y.j) == (x.step, x.e, x.f, kept[x.i], kept[x.j])).unwrap();
                assert_eq!(x.reading, y.reading);
            }
        }
    });
}

#[test]
fn prop_isc_d13_gamma_under_coarsening_is_bounded() {
    static N: AtomicUsize = AtomicUsize::new(0);
    for_all("I-SC-D13", |r| {
        let s = random_edges(r, (1, 3), (1, 2), (3, 7), 0.5);
        let n = s.n_registrations();
        // Keep the first and last registration and a random subset between.
        let mut idx = vec![0];
        for t in 1..n - 1 { if r.chance(4) { idx.push(t); } }
        idx.push(n - 1);
        let sub = EdgeSeries::from_edge_values_synthetic(idx.iter().map(|&t| s.a[t].clone()).collect(),
                                               idx.iter().map(|&t| s.da[t].clone()).collect()).unwrap();
        let fine = relation_changes(&s);
        let coarse = PersistenceChange::new(&sub).evolution_counts();
        for x in &coarse {
            let groups = idx.windows(2).map(|w| (w[0], w[1]));
            let mut one_sign = 0;
            for (lo, hi) in groups {
                let steps: Vec<&RelationChange> = fine.iter()
                    .filter(|y| y.edge == x.edge && y.component == x.component && y.step >= lo && y.step < hi).collect();
                let all_pos = steps.iter().all(|y| y.reading == Reading::Freedom && y.change > 0.0);
                let all_neg = steps.iter().all(|y| y.reading == Reading::Freedom && y.change < 0.0);
                if all_pos || all_neg { one_sign += 1; }
                if (all_pos || all_neg) && hi - lo > 1 { hit(&N); }
            }
            assert!(one_sign <= x.changed_steps && x.changed_steps <= idx.len() - 1);
        }
    });
    exercised(&N, "I-SC-D13 merged one-sign group");
}

// ── I-SC-D6 corrected: resolution conditions under uniform rescaling ───────
//
// A_{s+1} = κ·A_s, κ > 0. (i) PC1 freedom iff |κ − 1|·|A| > δA_s + δA_{s+1}.
// (ii) δ scaled with the values: every PC3 state is held. (iii) δ fixed:
// κ ≥ 1 holds every resolved state (unresolved may resolve); κ ≤ 1 holds
// every unresolved state (resolved may unresolve). (iv) Never a crossing.

#[test]
fn prop_isc_d6_resolution_conditions_under_rescaling() {
    static UP: AtomicUsize = AtomicUsize::new(0);
    static DOWN: AtomicUsize = AtomicUsize::new(0);
    static PC1_STRUCT: AtomicUsize = AtomicUsize::new(0);
    for_all("I-SC-D6 resolution", |r| {
        let (m, k) = (r.usize(2, 5), r.usize(2, 3));
        let a0: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| r.eighth(-4.0, 4.0)).collect()).collect();
        let d0: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| r.delta(0.5)).collect()).collect();
        let kappa = 2f64.powi(r.int(-2, 2) as i32);
        let a1: Vec<Vec<f64>> = a0.iter().map(|c| c.iter().map(|v| kappa * v).collect()).collect();
        let scaled_delta = r.chance(4);
        let d1: Vec<Vec<f64>> = if scaled_delta {
            d0.iter().map(|c| c.iter().map(|v| kappa * v).collect()).collect()
        } else { d0.clone() };
        let s = EdgeSeries::from_edge_values_synthetic(vec![a0.clone(), a1], vec![d0.clone(), d1.clone()]).unwrap();
        // (i)
        for x in relation_changes(&s) {
            let cond = (kappa - 1.0).abs() * a0[x.component][x.edge].abs()
                > d0[x.component][x.edge] + d1[x.component][x.edge];
            assert_eq!(x.reading == Reading::Freedom, cond);
            if !cond && a0[x.component][x.edge] != 0.0 && kappa != 1.0 { hit(&PC1_STRUCT); }
        }
        // (ii)–(iv)
        for x in arrangement_steps(&s).unwrap() {
            if let (AtDelta::Resolved(p), AtDelta::Resolved(q)) = (x.before, x.after) { assert_eq!(p, q); }
            if scaled_delta { assert_eq!(x.before, x.after); continue; }
            if kappa >= 1.0 && x.before.is_resolved() { assert_eq!(x.before, x.after); }
            if kappa <= 1.0 && !x.before.is_resolved() { assert_eq!(x.before, x.after); }
            if kappa > 1.0 && !x.before.is_resolved() && x.after.is_resolved() { hit(&UP); }
            if kappa < 1.0 && x.before.is_resolved() && !x.after.is_resolved() { hit(&DOWN); }
        }
    });
    exercised(&PC1_STRUCT, "I-SC-D6 relation changed exactly but unresolved at δ");
    exercised(&UP, "I-SC-D6 unresolved state resolves under κ > 1, fixed δ");
    exercised(&DOWN, "I-SC-D6 resolved state unresolves under κ < 1, fixed δ");
}

// ── Relation transitions (Origin, 2026-11-21) ──────────────────────────────
// Per relation: transitions alternate direction; each new state holds at
// least min_hold steps; with min_hold = 1 they are exactly the reading changes.

#[test]
fn prop_relation_transitions_alternate_and_hold() {
    static N: AtomicUsize = AtomicUsize::new(0);
    for_all("relation transitions", |r| {
        let s = random_edges(r, (1, 4), (1, 2), (3, 9), 0.5);
        let pc = PersistenceChange::new(&s);
        let hold = r.usize(1, 3);
        let t = pc.relation_transitions(hold);
        for e in 0..s.n_edges() { for c in 0..s.k() {
            let mine: Vec<_> = t.iter().filter(|x| x.edge == e && x.component == c).collect();
            for w in mine.windows(2) { assert_ne!(w[0].to, w[1].to); assert!(w[0].step < w[1].step); }
            assert!(mine.iter().all(|x| x.held_for >= hold));
            if !mine.is_empty() { hit(&N); }
            let readings: Vec<Reading> = pc.relations.iter().filter(|x| x.edge == e && x.component == c).map(|x| x.reading).collect();
            let changes = readings.windows(2).filter(|w| w[0] != w[1]).count();
            assert_eq!(pc.relation_transitions(1).iter().filter(|x| x.edge == e && x.component == c).count(), changes);
        }}
    });
    exercised(&N, "relation transitions found");
}
