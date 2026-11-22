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
//!       δc′ = |a|·δc + |b|·δj, PC2(a) readings move.
//! (v)   Locked exchange and held arrangement are independent: I-SC-D3 and
//!       D4 with components and edges exchanged, since every minor is the
//!       determinant of a 2×2 block. c = λ·j + k (k fixed) locks every
//!       change minor while the state minor k_e·j_f − j_e·k_f reorganizes;
//!       c_t = λ_t·j_t (λ moving) holds every state minor at 0 while the
//!       change minors resolve.
//!
//! [IMPL] SYNTHETIC FIXTURES: dyadic values, so every sum and product is
//! exact. They establish that the forms compute as stated, nothing
//! empirical. OC-SC-3 itself (how a registered carrier's change meets a
//! registered structure's) is answered only by registered data.

#[path = "support/synthetic_feedback.rs"]
mod synthetic_feedback;
#[path = "support/synthetic_identity.rs"]
mod synthetic_identity;
use abr_kernel::exact::{exceeds, Expansion};
use abr_kernel::ground_variables::{energy_exchange, EnergyCarrier, ObservedRange, Registration};
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::persistence_change::*;

fn rel(n: usize, edges: Vec<(usize, usize)>) -> DeclaredRelations {
    let m = edges.len();
    synthetic_feedback::declare_loops(synthetic_identity::identify(
        DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m])))
}

/// One registration: data[c][v] with δ[c][v].
fn reg(data: Vec<Vec<f64>>, delta: Vec<Vec<f64>>) -> Registration {
    Registration::synthetic(NodeField::new(data), delta)
}

fn carrier(values: Vec<f64>, delta: Vec<f64>) -> EnergyCarrier {
    EnergyCarrier { range: ObservedRange::synthetic(), name: "SYNTHETIC carrier".into(),
        unit: "synthetic".into(), values, delta }
}

fn pc1(pc: &PersistenceChange, step: usize, c: usize, e: usize) -> Reading {
    pc.relations.iter().find(|x| x.step == step && x.component == c && x.edge == e).unwrap().reading
}

fn across(pc: &PersistenceChange) -> &Couplings { pc.across_components.as_ref().unwrap() }

// ── Self-contained generator (SplitMix64), as in v10_properties ────────────

const CASES: u64 = 300;

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
    fn eighth(&mut self, lo: f64, hi: f64) -> f64 { self.int((lo * 8.0) as i64, (hi * 8.0) as i64) as f64 / 8.0 }
    fn delta(&mut self, hi: f64) -> f64 { self.int(0, (hi * 16.0) as i64) as f64 / 16.0 }
    fn chance(&mut self, p_in_8: u64) -> bool { self.next() % 8 < p_in_8 }
}

fn for_all(name: &str, prop: impl Fn(&mut Rng)) {
    for seed in 0..CASES {
        let mut r = Rng(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ 0xD17);
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| prop(&mut r)));
        if caught.is_err() { panic!("property {name} failed at seed {seed}"); }
    }
}

fn random_rel(r: &mut Rng) -> DeclaredRelations {
    let n = r.usize(3, 5);
    let m = r.usize(n - 1, 7.min(n * (n - 1) / 2));
    let mut edges: Vec<(usize, usize)> = Vec::new();
    while edges.len() < m {
        let (s, t) = (r.usize(0, n - 1), r.usize(0, n - 1));
        if s != t && !edges.contains(&(s, t)) && !edges.contains(&(t, s)) { edges.push((s, t)); }
    }
    rel(n, edges)
}

/// A node-valued sequence: k components over n loci and `regs` registrations.
/// Component 0 is structural; the last is the carrier. Values move by small
/// dyadic steps, so some relations hold and some change.
struct Field { values: Vec<Vec<Vec<f64>>>, delta: Vec<Vec<Vec<f64>>> }

fn random_field(r: &mut Rng, n: usize, k: usize, regs: usize, d_hi: f64) -> Field {
    let mut values = vec![(0..k).map(|_| (0..n).map(|_| r.eighth(-3.0, 3.0)).collect::<Vec<_>>()).collect::<Vec<_>>()];
    for _ in 1..regs {
        let prev = values.last().unwrap().clone();
        values.push(prev.iter().map(|row| row.iter().map(|&x| if r.chance(4) { x + r.eighth(-1.0, 1.0) } else { x }).collect()).collect());
    }
    let d: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| r.delta(d_hi)).collect()).collect();
    let delta = (0..regs).map(|_| d.clone()).collect();
    Field { values, delta }
}

fn series_of(f: &Field, rel: &DeclaredRelations) -> (Vec<Registration>, EdgeSeries) {
    let seq: Vec<Registration> = f.values.iter().zip(&f.delta).map(|(v, d)| reg(v.clone(), d.clone())).collect();
    let s = EdgeSeries::from_registrations(&seq, rel).unwrap();
    (seq, s)
}

// ═══════════════════════════════════════════════════════════════════════════
// (i) The V3 floor is the state reading of the carrier component
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn d17_i_the_energy_floor_is_the_state_reading_of_the_carrier_component() {
    let (mut floors, mut held) = (0, 0);
    let counts = std::sync::Mutex::new((0usize, 0usize));
    for_all("d17_i", |r| {
        let rel = random_rel(r);
        let (k, regs) = (r.usize(2, 3), r.usize(2, 4));
        let f = random_field(r, rel.n_nodes, k, regs, 0.5);
        let (_, s) = series_of(&f, &rel);
        let c = k - 1;
        for t in 0..regs {
            let x = energy_exchange(&carrier(f.values[t][c].clone(), f.delta[t][c].clone()), &rel);
            for e in 0..rel.n_edges() {
                let a = s.entry(t, c, e);
                assert_eq!(x[e].epsilon, a.abs().nearest(), "ε = |A[c][e]|");
                assert_eq!(x[e].delta_epsilon, s.da[t][c][e], "δ_ε = δA[c][e]");
                assert_eq!(x[e].floor_resolved, exceeds(&a, &Expansion::of(s.da[t][c][e])), "one reading");
                let mut g = counts.lock().unwrap();
                if x[e].floor_resolved { g.0 += 1 } else { g.1 += 1 }
            }
        }
    });
    let g = counts.into_inner().unwrap();
    floors += g.0; held += g.1;
    // Both outcomes occur, so the equality is tested on each side.
    assert!(floors > 0 && held > 0, "floor resolved {floors}, not resolved {held}");
}

// ═══════════════════════════════════════════════════════════════════════════
// (ii) PC1 on the carrier reads the signed exchange
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn d17_ii_worked_the_exchange_reverses_with_its_magnitude_held() {
    // Loci 0 → 1. Carrier (component 1) at 0 then 1, then 1 then 0: A = −1
    // then +1. ε = 1 at both registrations, so the V3 degree holds; the
    // exchange has reversed, and PC1 on the carrier reads it (Δ = 2 > 1/4).
    let rel = rel(2, vec![(0, 1)]);
    let d = vec![vec![1.0 / 16.0; 2]; 2];
    let seq = vec![reg(vec![vec![0.0, 0.0], vec![0.0, 1.0]], d.clone()),
                   reg(vec![vec![0.0, 0.0], vec![1.0, 0.0]], d.clone())];
    let s = EdgeSeries::from_registrations(&seq, &rel).unwrap();
    let pc = PersistenceChange::new(&s);
    let e0 = energy_exchange(&carrier(vec![0.0, 1.0], d[1].clone()), &rel)[0];
    let e1 = energy_exchange(&carrier(vec![1.0, 0.0], d[1].clone()), &rel)[0];
    assert_eq!((e0.epsilon, e1.epsilon), (1.0, 1.0));
    assert!(e0.floor_resolved && e1.floor_resolved);
    assert_eq!(pc1(&pc, 0, 1, 0), Reading::Freedom);
    assert_eq!(pc.relations.iter().find(|x| x.component == 1).unwrap().change, 2.0);
    // The structural component did not move.
    assert_eq!(pc1(&pc, 0, 0, 0), Reading::Structure);
}

#[test]
fn d17_ii_a_resolved_change_of_exchange_is_pc1_freedom_not_conversely() {
    let counts = std::sync::Mutex::new((0usize, 0usize));
    for_all("d17_ii", |r| {
        let rel = random_rel(r);
        let regs = r.usize(2, 4);
        let f = random_field(r, rel.n_nodes, 2, regs, 0.5);
        let (_, s) = series_of(&f, &rel);
        let pc = PersistenceChange::new(&s);
        for st in 0..s.n_steps() {
            for e in 0..rel.n_edges() {
                let (a0, a1) = (s.entry(st, 1, e), s.entry(st + 1, 1, e));
                let d_eps = a1.abs().minus(&a0.abs());
                let d = s.change_exact(st, 1, e);
                let h = s.change_bound_exact(st, 1, e);
                // |Δε| ≤ |Δ| exactly; equal unless the sign of A changes.
                assert!(d.abs().minus(&d_eps.abs()).signum() >= 0);
                if a0.signum() * a1.signum() >= 0 { assert_eq!(d.abs().minus(&d_eps.abs()).signum(), 0); }
                // One bound: δΔ = δε_s + δε_{s+1} (no run-constant part declared).
                assert_eq!(h.up(), Expansion::of(s.da[st][1][e]).plus(&Expansion::of(s.da[st + 1][1][e])).up());
                if exceeds(&d_eps, &h) { assert_eq!(pc1(&pc, st, 1, e), Reading::Freedom); }
                let mut g = counts.lock().unwrap();
                if exceeds(&d_eps, &h) { g.0 += 1 }
                if pc1(&pc, st, 1, e) == Reading::Freedom && !exceeds(&d_eps, &h) { g.1 += 1 }
            }
        }
    });
    let (both, pc1_only) = counts.into_inner().unwrap();
    // The converse fails on some fields: PC1 freedom with the change of ε held.
    assert!(both > 0 && pc1_only > 0, "ε resolved {both}, PC1 alone {pc1_only}");
}

// ═══════════════════════════════════════════════════════════════════════════
// (iii) A change minor needs change in both readings
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn d17_iii_worked_energy_flows_while_structure_holds_reads_locked() {
    // The fixture of `energy_carrier_as_a_component_reads_through_the_same_forms`
    // (v10_persistence_change.rs). At step 2 the structure holds on both
    // edges and the carrier's exchange changes by (2, −2). PC2(a) reads
    // structure: the minor is 0 exactly, Δ_j being 0. It is the ratio-0 case,
    // forced by the bound; PC1 on the structure tells it apart.
    let rel = rel(3, vec![(0, 1), (1, 2)]);
    let structural = [0.0, 1.0, 2.0, 2.0];
    let c = [0.0, 3.0, 6.0, 8.0];
    let d = vec![vec![1.0 / 128.0; 3]; 2];
    let seq: Vec<Registration> = (0..4).map(|t| reg(vec![
        vec![4.0, 2.0 + structural[t], 0.0],
        vec![9.0, 5.0 + c[t], 1.0],
    ], d.clone())).collect();
    let s = EdgeSeries::from_registrations(&seq, &rel).unwrap();
    let pc = PersistenceChange::new(&s);
    let at2: Vec<_> = across(&pc).readings.iter()
        .filter(|x| matches!(x.over, ReadingPair::Components { step: 2, .. })).collect();
    assert_eq!(at2.len(), 1);
    assert_eq!((at2[0].minor, at2[0].reading), (0.0, Reading::Structure));
    assert_eq!((pc1(&pc, 2, 0, 0), pc1(&pc, 2, 0, 1)), (Reading::Structure, Reading::Structure));
    assert_eq!((pc1(&pc, 2, 1, 0), pc1(&pc, 2, 1, 1)), (Reading::Freedom, Reading::Freedom));
    // At step 1 both move; the reading there is locked with both changing.
    assert_eq!((pc1(&pc, 1, 0, 0), pc1(&pc, 1, 1, 0)), (Reading::Freedom, Reading::Freedom));
}

#[test]
fn d17_iii_case_for_origin_the_coupled_change_grade_met_by_a_held_reading() {
    // Two registrations, two components. The structure holds everywhere; the
    // carrier's exchange changes by (1, 5) on the two edges, so its changes
    // are not in any common proportion with anything that moves. PC2(a) is
    // the only family (n = 2), and its one reading is forced to structure by
    // (iii). The coupled-change grade (V1, `coupled_change_pairs`) as it
    // stands reads the pair as connected in change. Recorded as it is, for
    // Origin (I-SC-D17, the 48th): this test states the current behaviour,
    // and changes if the grade is amended.
    let rel = rel(3, vec![(0, 1), (1, 2)]);
    let d = vec![vec![1.0 / 64.0; 3]; 2];
    let seq = vec![reg(vec![vec![0.0, 1.0, 3.0], vec![0.0, 0.0, 0.0]], d.clone()),
                   reg(vec![vec![0.0, 1.0, 3.0], vec![0.0, -1.0, -6.0]], d.clone())];
    let s = EdgeSeries::from_registrations(&seq, &rel).unwrap();
    let pc = PersistenceChange::new(&s);
    assert_eq!((pc1(&pc, 0, 0, 0), pc1(&pc, 0, 0, 1)), (Reading::Structure, Reading::Structure));
    assert_eq!(s.change(0).0[1], vec![1.0, 5.0]);
    assert!(pc.across_steps.is_err());
    assert_eq!(across(&pc).readings.len(), 1);
    assert_eq!(across(&pc).readings[0].reading, Reading::Structure);
    assert_eq!(coupled_change_pairs(&pc).unwrap(), vec![(0, 1)]);
}

/// The two readings (step, component) a change minor is formed over.
fn readings_of(over: ReadingPair) -> [(usize, usize); 2] {
    match over {
        ReadingPair::Components { step, i, j } => [(step, i), (step, j)],
        ReadingPair::Steps { component, step } => [(step, component), (step + 1, component)],
    }
}

#[test]
fn d17_iii_a_change_minor_resolves_only_where_each_reading_changes() {
    // Over node-derived fields and edge-registered fields with independent
    // gains (which widen PC2(a)'s bounds, never PC1's): every resolved change
    // minor has, in each of its readings, PC1 freedom on e or on f.
    let counts = std::sync::Mutex::new((0usize, 0usize, 0usize));
    for_all("d17_iii", |r| {
        let rel = random_rel(r);
        let (k, regs) = (r.usize(2, 3), r.usize(3, 4));
        let f = random_field(r, rel.n_nodes, k, regs, 0.25);
        let (_, node) = series_of(&f, &rel);
        let m = rel.n_edges();
        let edge = {
            let a = node.a.clone();
            let da = node.da.clone();
            let gains = EdgeGains { component: vec![0.0; k], edge: vec![0.0; m],
                independent: (0..k).map(|_| (0..m).map(|_| if r.chance(3) { 1.0 / 16.0 } else { 0.0 }).collect()).collect() };
            EdgeSeries::from_edge_values_synthetic(a, da).unwrap().with_run_constant(vec![vec![0.0; m]; k], gains)
        };
        for s in [&node, &edge] {
            let pc = PersistenceChange::new(s);
            let mut g = counts.lock().unwrap();
            for fam in [&pc.across_components, &pc.across_steps] {
                let Ok(fam) = fam else { continue };
                for x in &fam.readings {
                    let moved = |(st, c): (usize, usize)|
                        pc1(&pc, st, c, x.e) == Reading::Freedom || pc1(&pc, st, c, x.f) == Reading::Freedom;
                    let [p, q] = readings_of(x.over);
                    if x.reading == Reading::Freedom {
                        assert!(moved(p) && moved(q), "a resolved minor needs change in both readings");
                        g.0 += 1;
                    } else if !(moved(p) && moved(q)) {
                        g.1 += 1; // entered, and forced to structure by the bound
                    } else {
                        g.2 += 1; // both readings change, and the change is locked
                    }
                }
            }
        }
    });
    let (resolved, forced, locked) = counts.into_inner().unwrap();
    // The statement has content: the kernel enters pairs where one reading is
    // held on both edges (its filter is "moved in either reading"), and every
    // one of them reads structure.
    assert!(resolved > 0 && forced > 0 && locked > 0, "resolved {resolved}, forced {forced}, locked {locked}");
}

// ═══════════════════════════════════════════════════════════════════════════
// (iv) Re-declaring the carrier within its plane with the structure (D14 (ii))
// ═══════════════════════════════════════════════════════════════════════════

/// The field with its carrier re-declared as a·c + b·j, δ by |a|·δc + |b|·δj.
fn mixed(f: &Field, a: f64, b: f64) -> Field {
    let c = f.values[0].len() - 1;
    let values = f.values.iter().map(|t| {
        let mut t = t.clone();
        t[c] = t[c].iter().zip(&t[0]).map(|(&x, &j)| a * x + b * j).collect();
        t
    }).collect();
    let delta = f.delta.iter().map(|t| {
        let mut t = t.clone();
        t[c] = t[c].iter().zip(&t[0]).map(|(&x, &j)| a.abs() * x + b.abs() * j).collect();
        t
    }).collect();
    Field { values, delta }
}

fn carrier_structure(pc: &PersistenceChange, c: usize) -> Vec<(usize, usize, usize, f64, Reading)> {
    across(pc).readings.iter().filter_map(|x| match x.over {
        ReadingPair::Components { step, i: 0, j } if j == c => Some((step, x.e, x.f, x.minor, x.reading)),
        _ => None,
    }).collect()
}

#[test]
fn d17_iv_exactly_the_carrier_structure_minor_is_a_property_of_their_plane() {
    // δ = 0. Every carrier–structure minor is multiplied by det R = a, so the
    // locked or independent reading is unchanged; PC1 on the carrier and the
    // V3 floor change on some fields when b ≠ 0.
    let moved = std::sync::Mutex::new((0usize, 0usize));
    for_all("d17_iv_exact", |r| {
        let rel = random_rel(r);
        let regs = r.usize(2, 4);
        let f = random_field(r, rel.n_nodes, 2, regs, 0.0);
        let (a, b) = (*[1.0, -1.0, 2.0, 0.5, -3.0].get(r.usize(0, 4)).unwrap(), r.eighth(-2.0, 2.0));
        let g = mixed(&f, a, b);
        let (pf, pg) = (PersistenceChange::new(&series_of(&f, &rel).1), PersistenceChange::new(&series_of(&g, &rel).1));
        let (xf, xg) = (carrier_structure(&pf, 1), carrier_structure(&pg, 1));
        assert_eq!(xf.len(), xg.len(), "the same pairs enter");
        for (u, v) in xf.iter().zip(&xg) {
            assert_eq!((u.0, u.1, u.2), (v.0, v.1, v.2));
            assert_eq!(v.3, a * u.3, "μ′ = det R · μ");
            assert_eq!(u.4, v.4, "locked or independent, unchanged");
        }
        // The structural component is not re-declared: its PC1 is unchanged.
        let pc1s = |p: &PersistenceChange, c| p.relations.iter().filter(|x| x.component == c).map(|x| x.reading).collect::<Vec<_>>();
        assert_eq!(pc1s(&pf, 0), pc1s(&pg, 0));
        let mut m = moved.lock().unwrap();
        if pc1s(&pf, 1) != pc1s(&pg, 1) { m.0 += 1 }
        let floors = |h: &Field| (0..regs).map(|t| energy_exchange(&carrier(h.values[t][1].clone(), h.delta[t][1].clone()), &rel)
            .iter().map(|x| x.floor_resolved).collect::<Vec<_>>()).collect::<Vec<_>>();
        if floors(&f) != floors(&g) { m.1 += 1 }
    });
    let (pc1_moved, floor_moved) = moved.into_inner().unwrap();
    assert!(pc1_moved > 0 && floor_moved > 0, "PC1 moved {pc1_moved}, floor moved {floor_moved}");
}

#[test]
fn d17_iv_at_delta_a_carrier_mixed_with_structure_never_gains_resolution() {
    // δ > 0, the re-declared carrier's δ propagated as an interval,
    // δc′ = |a|·δc + |b|·δj. b = 0 is a per-component scale (I-SC-D15, kind
    // C): no reading moves. For b ≠ 0 the minor is a·μ exactly and its bound
    // is at least |a| times the original:
    //   hw′ − |a|·hw ≥ 2|a|·|b/a|·δΔj[e]·δΔj[f] ≥ 0,
    // since |Δc′| ≥ |a|·|Δc| − |b|·|Δj| on each edge and the bound gains
    // |b|·(|Δj[f]|·δΔj[e] + |Δj[e]|·δΔj[f] + 2·δΔj[e]·δΔj[f]). So a locked
    // reading is never resolved by mixing; an independent one can be lost.
    // No pair enters that did not enter before (a held relation stays held:
    // |Δc′| ≤ |a|·δΔc + |b|·δΔj = δΔc′); some pairs can leave.
    //
    // *Generator's first statement (2026-11-23), refuted by this test before
    // commit:* "at δ > 0 PC2(a) readings move in both directions". On 300
    // fields: 40 moved freedom → structure, none structure → freedom.
    let counts = std::sync::Mutex::new((0usize, 0usize, 0usize));
    for_all("d17_iv_delta", |r| {
        let rel = random_rel(r);
        let regs = r.usize(2, 4);
        let f = random_field(r, rel.n_nodes, 2, regs, 0.25);
        let sf = series_of(&f, &rel).1;
        let pf = PersistenceChange::new(&sf);
        let a = *[1.0, -1.0, 2.0, 0.5].get(r.usize(0, 3)).unwrap();
        let ps = PersistenceChange::new(&series_of(&mixed(&f, a, 0.0), &rel).1);
        let readings = |p: &PersistenceChange| carrier_structure(p, 1).iter().map(|x| (x.0, x.1, x.2, x.4)).collect::<Vec<_>>();
        assert_eq!(readings(&pf), readings(&ps), "a per-component scale moves no reading");
        let b = r.eighth(-2.0, 2.0);
        let pm = PersistenceChange::new(&series_of(&mixed(&f, a, b), &rel).1);
        let (xf, xm) = (carrier_structure(&pf, 1), carrier_structure(&pm, 1));
        let bounds = |p: &PersistenceChange| across(p).readings.iter().filter(|x| matches!(x.over, ReadingPair::Components { i: 0, j: 1, .. }))
            .map(|x| x.half_width).collect::<Vec<_>>();
        let (hf, hm) = (bounds(&pf), bounds(&pm));
        let mut g = counts.lock().unwrap();
        for (n, v) in xm.iter().enumerate() {
            let u = xf.iter().position(|u| (u.0, u.1, u.2) == (v.0, v.1, v.2)).expect("no new pair enters");
            assert_eq!(v.3, a * xf[u].3, "μ′ = a·μ");
            // Reported bounds are rounded up once; dyadic fixtures make them exact.
            assert!(hm[n] >= a.abs() * hf[u], "the bound does not shrink");
            match (xf[u].4, v.4) {
                (Reading::Structure, Reading::Freedom) => panic!("mixing resolved a locked reading"),
                (Reading::Freedom, Reading::Structure) => g.0 += 1,
                _ => {}
            }
        }
        if xm.len() < xf.len() { g.1 += 1 }
        if b != 0.0 && xm.iter().any(|v| v.4 == Reading::Freedom) { g.2 += 1 }
    });
    let (lost, left, kept) = counts.into_inner().unwrap();
    // Each side has content: resolution is lost on some fields, pairs leave
    // on some, and on some an independent reading survives the mixing.
    assert!(lost > 0 && left > 0 && kept > 0, "lost {lost}, pairs left {left}, kept {kept}");
}

#[test]
fn d17_iv_the_one_way_result_belongs_to_the_bound_not_the_field() {
    // The one-way result above is about resolution, not about structure and
    // freedom. Over the sequence, relations move from structure to freedom
    // and back (PC1, step to step, one field). Under the re-declaration,
    // PC1 on the carrier (a different quantity) and PC3 (two states compared,
    // PC-D2 as corrected) move both ways. Only the carrier–structure PC2(a)
    // reading, whose value the mixing leaves fixed and whose bound it widens,
    // moves one way (the test above).
    let c = std::sync::Mutex::new([0usize; 6]);
    for_all("d17_iv_both_ways", |r| {
        let rel = random_rel(r);
        let regs = r.usize(3, 5);
        let f = random_field(r, rel.n_nodes, 2, regs, 0.25);
        let pf = PersistenceChange::new(&series_of(&f, &rel).1);
        let pm = PersistenceChange::new(&series_of(&mixed(&f, 1.0, r.eighth(-2.0, 2.0)), &rel).1);
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
    // k_e·j_f − j_e·k_f, and it crosses zero as j moves (PC3 freedom).
    let rel = rel(3, vec![(0, 1), (1, 2)]);
    let k = [0.0, 1.0, 0.0];
    let js = [[0.0, 1.0, 3.0], [0.0, -1.0, -3.0]];
    let d = vec![vec![1.0 / 64.0; 3]; 2];
    let seq: Vec<Registration> = js.iter().map(|j| reg(vec![j.to_vec(),
        j.iter().zip(&k).map(|(&j, &k)| 2.0 * j + k).collect()], d.clone())).collect();
    let s = EdgeSeries::from_registrations(&seq, &rel).unwrap();
    let pc = PersistenceChange::new(&s);
    let x = carrier_structure(&pc, 1);
    assert!(!x.is_empty() && x.iter().all(|u| u.3 == 0.0 && u.4 == Reading::Structure));
    let arr = pc.arrangement.as_ref().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0].reading, Reading::Freedom);
    assert_ne!(arr[0].before, arr[0].after);
}

#[test]
fn d17_v_worked_held_arrangement_with_independent_exchange() {
    // c_t = λ_t·j_t with λ moving from 1 to 2: every state minor is 0 (the
    // proportional grade holds at both registrations), while the change
    // minor is (λ′ − λ)·(Δ_j[e]·j_f − Δ_j[f]·j_e) ≠ 0: independent exchange.
    let rel = rel(3, vec![(0, 1), (1, 2)]);
    let js = [[0.0, 1.0, 3.0], [0.0, 2.0, 3.0]];
    let lambda = [1.0, 2.0];
    let d = vec![vec![1.0 / 64.0; 3]; 2];
    let seq: Vec<Registration> = js.iter().zip(lambda).map(|(j, l)| reg(vec![j.to_vec(),
        j.iter().map(|&j| l * j).collect()], d.clone())).collect();
    let s = EdgeSeries::from_registrations(&seq, &rel).unwrap();
    let pc = PersistenceChange::new(&s);
    let arr = pc.arrangement.as_ref().unwrap();
    assert!(!arr.is_empty() && arr.iter().all(|x| x.reading == Reading::Structure && x.before == x.after));
    let x = carrier_structure(&pc, 1);
    assert!(x.iter().any(|u| u.3 != 0.0 && u.4 == Reading::Freedom));
}

#[test]
fn d17_v_d3_and_d4_with_components_and_edges_exchanged() {
    // Exactly, over random fields: c_t = λ·j_t + k ⇒ every carrier–structure
    // change minor is 0 and every state minor is k_e·j_f − j_e·k_f (edge
    // values of k); c_t = λ_t·j_t ⇒ every state minor is 0.
    for_all("d17_v", |r| {
        let rel = random_rel(r);
        let regs = r.usize(2, 4);
        let f = random_field(r, rel.n_nodes, 1, regs, 0.0);
        let (l, k): (f64, Vec<f64>) = (r.eighth(-2.0, 2.0), (0..rel.n_nodes).map(|_| r.eighth(-2.0, 2.0)).collect());
        let ka: Vec<f64> = rel.edges.iter().map(|&(s, t)| k[s] - k[t]).collect();
        let d0 = vec![vec![0.0; rel.n_nodes]; 2];
        let seq: Vec<Registration> = f.values.iter().map(|t| reg(vec![t[0].clone(),
            t[0].iter().zip(&k).map(|(&j, &k)| l * j + k).collect()], d0.clone())).collect();
        let s = EdgeSeries::from_registrations(&seq, &rel).unwrap();
        for st in 0..s.n_steps() {
            for e in 0..rel.n_edges() {
                for g in 0..rel.n_edges() {
                    let mu = s.change_exact(st, 1, e).times(&s.change_exact(st, 0, g))
                        .minus(&s.change_exact(st, 0, e).times(&s.change_exact(st, 1, g)));
                    assert_eq!(mu.signum(), 0, "locked exchange");
                }
            }
        }
        for t in 0..regs {
            for e in 0..rel.n_edges() {
                for g in 0..rel.n_edges() {
                    let m = s.entry(t, 0, e).times(&s.entry(t, 1, g)).minus(&s.entry(t, 1, e).times(&s.entry(t, 0, g)));
                    let want = s.entry(t, 0, e).times(&Expansion::of(ka[g])).minus(&Expansion::of(ka[e]).times(&s.entry(t, 0, g)));
                    assert_eq!(m.minus(&want).signum(), 0, "state minor k_e·j_f − j_e·k_f, up to orientation");
                }
            }
        }
        let lam: Vec<f64> = (0..regs).map(|_| r.eighth(-2.0, 2.0)).collect();
        let seq: Vec<Registration> = f.values.iter().zip(&lam).map(|(t, &l)| reg(vec![t[0].clone(),
            t[0].iter().map(|&j| l * j).collect()], d0.clone())).collect();
        let s = EdgeSeries::from_registrations(&seq, &rel).unwrap();
        for t in 0..regs {
            for e in 0..rel.n_edges() {
                for g in 0..rel.n_edges() {
                    let m = s.entry(t, 0, e).times(&s.entry(t, 1, g)).minus(&s.entry(t, 1, e).times(&s.entry(t, 0, g)));
                    assert_eq!(m.signum(), 0, "held proportional");
                }
            }
        }
    });
}
