//! Test support (synthetic only): whole relational fields (Df6, W11).
//!
//! Every fixture is formed through `RelationalField::new`, so A, B, Ξ, their
//! persistence forms, energy, freedom (PC1–PC3) and connection (identity,
//! proportional, sustained, coupled change) are evaluated together, in one
//! call, on one declared sequence. A claim is then checked on that evaluated
//! field, from the parts it reports. `inclusion` asserts the rule of
//! inclusion (W11): every declared variable is read, and every one that
//! cannot be is named as a declaration gap.
//!
//! The layout: components 0 … k − 2 are structural; component k − 1 is the
//! energy carrier, registered at the same loci (T5) and declared to energy
//! as an `EnergyCarrier` from the same values and repeatability. Run-constant
//! parts (Z1a) are declared on structural components on some fields, and on
//! the carrier on some fields when asked for.
//!
//! [IMPL] SYNTHETIC FIXTURES: dyadic values, so every sum and product is
//! exact. They establish that the forms compute as stated, nothing empirical.
#![allow(dead_code)]

use abr_kernel::ground_variables::{
    Connection, DeclarationGap, EnergyCarrier, Finding, ObservedRange, Registration,
};
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::persistence_change::{EdgeSeries, PersistenceChange};
use abr_kernel::relational_field::RelationalField;

#[path = "synthetic_feedback.rs"]
mod synthetic_feedback;
#[path = "synthetic_identity.rs"]
mod synthetic_identity;

pub fn rel(n: usize, edges: Vec<(usize, usize)>) -> DeclaredRelations {
    let m = edges.len();
    synthetic_feedback::declare_loops(synthetic_identity::identify(
        DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m])))
}

// ── Self-contained generator (SplitMix64), as in v10_properties ────────────

pub const CASES: u64 = 300;

pub struct Rng(pub u64);
impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    pub fn int(&mut self, lo: i64, hi: i64) -> i64 { lo + (self.next() % (hi - lo + 1) as u64) as i64 }
    pub fn usize(&mut self, lo: usize, hi: usize) -> usize { self.int(lo as i64, hi as i64) as usize }
    pub fn eighth(&mut self, lo: f64, hi: f64) -> f64 { self.int((lo * 8.0) as i64, (hi * 8.0) as i64) as f64 / 8.0 }
    pub fn delta(&mut self, hi: f64) -> f64 { self.int(0, (hi * 16.0) as i64) as f64 / 16.0 }
    pub fn chance(&mut self, p_in_8: u64) -> bool { self.next() % 8 < p_in_8 }
}

pub fn for_all(name: &str, salt: u64, prop: impl Fn(&mut Rng)) {
    for seed in 0..CASES {
        let mut r = Rng(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ salt);
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| prop(&mut r)));
        if caught.is_err() { panic!("property {name} failed at seed {seed}"); }
    }
}

pub fn random_rel(r: &mut Rng) -> DeclaredRelations {
    let n = r.usize(3, 5);
    let m = r.usize(n, 7.min(n * (n - 1) / 2));
    let mut edges: Vec<(usize, usize)> = Vec::new();
    while edges.len() < m {
        let (s, t) = (r.usize(0, n - 1), r.usize(0, n - 1));
        if s != t && !edges.contains(&(s, t)) && !edges.contains(&(t, s)) { edges.push((s, t)); }
    }
    rel(n, edges)
}

/// The declaration of a node-valued sequence: values[t][c][v], repeatability
/// δ[c][v], and the run-constant parts offset[c][v], gain[c][v] (Z1a).
#[derive(Clone)]
pub struct Declared {
    pub values: Vec<Vec<Vec<f64>>>,
    pub delta: Vec<Vec<f64>>,
    pub offset: Vec<Vec<f64>>,
    pub gain: Vec<Vec<f64>>,
}

impl Declared {
    pub fn k(&self) -> usize { self.delta.len() }
    pub fn carrier(&self) -> usize { self.k() - 1 }
    /// True when a run-constant part is declared on the carrier.
    pub fn carrier_run_constant(&self) -> bool {
        let c = self.carrier();
        self.offset[c].iter().chain(&self.gain[c]).any(|&x| x != 0.0)
    }
    /// True when any run-constant part is declared.
    pub fn any_run_constant(&self) -> bool {
        self.offset.iter().chain(&self.gain).flatten().any(|&x| x != 0.0)
    }
    /// Repeatability only, everywhere.
    pub fn repeatability(values: Vec<Vec<Vec<f64>>>, delta: Vec<Vec<f64>>) -> Self {
        let z = delta.iter().map(|d| vec![0.0; d.len()]).collect::<Vec<_>>();
        Declared { values, delta, offset: z.clone(), gain: z }
    }
}

/// A random declaration over `rel`: k components (the last the carrier),
/// `regs` registrations, values moving by small dyadic steps.
pub fn random_declared(r: &mut Rng, rel: &DeclaredRelations, k: usize, regs: usize, d_hi: f64,
                       z1a: bool, z1a_on_carrier: bool) -> Declared {
    let n = rel.n_nodes;
    let mut values = vec![(0..k).map(|_| (0..n).map(|_| r.eighth(-3.0, 3.0)).collect::<Vec<_>>()).collect::<Vec<_>>()];
    for _ in 1..regs {
        let prev = values.last().unwrap().clone();
        values.push(prev.iter().map(|row| row.iter().map(|&x| if r.chance(4) { x + r.eighth(-1.0, 1.0) } else { x }).collect()).collect());
    }
    let delta = (0..k).map(|_| (0..n).map(|_| r.delta(d_hi)).collect()).collect();
    let on = |c: usize| z1a && (c + 1 < k || z1a_on_carrier);
    let offset = (0..k).map(|c| (0..n).map(|_| if on(c) && r.chance(3) { r.delta(0.25) } else { 0.0 }).collect()).collect();
    let gain = (0..k).map(|c| (0..n).map(|_| if on(c) && r.chance(3) { 1.0 / 16.0 } else { 0.0 }).collect()).collect();
    Declared { values, delta, offset, gain }
}

/// A whole relational field and the parts its claims are read from.
pub struct Whole {
    pub rel: DeclaredRelations,
    pub declared: Declared,
    pub carriers: Vec<EnergyCarrier>,
    pub field: RelationalField,
    /// The edge series the field's freedom is read from
    /// (`ground_reading` forms it from the same registrations); its state
    /// bounds give each reading's resolution.
    pub series: EdgeSeries,
}

impl Whole {
    pub fn freedom(&self) -> &PersistenceChange { self.field.ground.freedom.as_ref().expect("freedom is read") }
    pub fn connection(&self) -> &Connection { self.field.ground.connection.as_ref().expect("connection is read") }
    pub fn carrier(&self) -> usize { self.declared.carrier() }
    /// PC1 at (step, component, edge), from the field's own freedom.
    pub fn pc1(&self, step: usize, c: usize, e: usize) -> abr_kernel::persistence_change::Reading {
        self.freedom().relations.iter().find(|x| x.step == step && x.component == c && x.edge == e).unwrap().reading
    }
}

/// Form the whole field: registrations with every declared part, the carrier
/// declared to energy from the same values and repeatability, and
/// `RelationalField::new` evaluating everything together.
pub fn whole(rel: &DeclaredRelations, d: Declared) -> Whole {
    let (k, c) = (d.k(), d.carrier());
    let seq: Vec<Registration> = d.values.iter().map(|v| {
        let g = Registration::synthetic(NodeField::new(v.clone()), d.delta.clone());
        if d.any_run_constant() { g.with_run_constant(d.offset.clone(), d.gain.clone(), vec![0.0; k]) } else { g }
    }).collect();
    let carriers: Vec<EnergyCarrier> = d.values.iter().map(|v| EnergyCarrier {
        range: ObservedRange::synthetic(), name: "SYNTHETIC carrier".into(), unit: "synthetic".into(),
        values: v[c].clone(), delta: d.delta[c].clone(),
    }).collect();
    let field = RelationalField::new(rel, seq, Some(&carriers)).expect("the relational field forms");
    let series = EdgeSeries::from_registrations(&field.registrations, rel).unwrap();
    // The series is the one the field reads: its relation values are the
    // field's spatial A at every registration.
    for t in 0..field.registrations.len() { assert_eq!(series.a[t], field.spatial[t].a); }
    let w = Whole { rel: rel.clone(), declared: d, carriers, field, series };
    inclusion(&w);
    w
}

/// The rule of inclusion (W11): every declared variable is read, or named as
/// a gap. Energy, freedom and connection are each formed; every level or
/// grade that could not be formed appears among the field's findings with its
/// gap. Returns the gaps.
pub fn inclusion(w: &Whole) -> Vec<DeclarationGap> {
    let g = &w.field.ground;
    let findings: Vec<Finding> = g.findings();
    let named = |gap: &DeclarationGap| findings.iter().any(|f| f.gap.as_ref() == Some(gap));
    let mut gaps = Vec::new();
    assert!(g.energy.is_ok(), "the carrier is declared, so energy is read");
    let pc = g.freedom.as_ref().expect("freedom is read");
    for r in [pc.across_components.as_ref().err(), pc.across_steps.as_ref().err(), pc.arrangement.as_ref().err()].into_iter().flatten() {
        assert!(named(r), "a PC level not formed is named: {r:?}");
        gaps.push(r.clone());
    }
    match &g.connection {
        Err(e) => { assert!(named(e)); gaps.push(e.clone()); }
        Ok(c) => {
            for r in [c.proportional.as_ref().err(), c.sustained.as_ref().err(), c.coupled_change.as_ref().err()].into_iter().flatten() {
                assert!(named(r), "a connection grade not formed is named: {r:?}");
                gaps.push(r.clone());
            }
        }
    }
    gaps
}

/// The same declaration with the carrier re-declared as a·c + b·j (j the
/// first structural component), δ propagated as |a|·δc + |b|·δj.
/// Repeatability only: run-constant parts are not propagated.
pub fn mixed(d: &Declared, a: f64, b: f64) -> Declared {
    assert!(!d.any_run_constant(), "mixing is declared on repeatability only");
    let c = d.carrier();
    let values = d.values.iter().map(|t| {
        let mut t = t.clone();
        t[c] = t[c].iter().zip(&t[0]).map(|(&x, &j)| a * x + b * j).collect();
        t
    }).collect();
    let mut delta = d.delta.clone();
    delta[c] = d.delta[c].iter().zip(&d.delta[0]).map(|(&x, &j)| a.abs() * x + b.abs() * j).collect();
    Declared::repeatability(values, delta)
}
