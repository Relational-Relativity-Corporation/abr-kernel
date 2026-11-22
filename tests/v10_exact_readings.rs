//! Kernel V10.3: exact readings (OC-NUM-1, 2026-11-23, 45th), under the
//! governing statement and the rule "decompose, do not simplify" (41st).
//!
//! Every reading is checked against an exact integer reference: node values
//! are dyadic with mixed magnitudes, scaled by 2¹² to i128, where every
//! difference, sum and product is exact. The kernel must report each value as
//! the exact value rounded once, each bound at or above its exact value, and
//! each reading as the exact comparison. Covered: PC1 (Δ), PC2(a) and PC2(b)
//! (change minors), PC3 (arrangement states), the identity and sustained
//! grades (V1), the energy floor (V3), the RelationalField forms A, B, Ξ and
//! their changes, and the reverse check.
//!
//! [IMPL] SYNTHETIC FIXTURES. They establish that the forms compute as
//! stated, nothing empirical.

#[path = "support/synthetic_feedback.rs"]
mod synthetic_feedback;
#[path = "support/synthetic_identity.rs"]
mod synthetic_identity;
use abr_kernel::ground_variables::*;
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::persistence_change::*;
use abr_kernel::relational_field::RelationalField;
use abr_kernel::reverse_check::{reverse_arrangement, reverse_coupling, reverse_relation, Inputs, Verdict};
use std::sync::Mutex;

fn rel(n: usize, edges: Vec<(usize, usize)>) -> DeclaredRelations {
    let m = edges.len();
    synthetic_feedback::declare_loops(synthetic_identity::identify(
        DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m])))
}

const CASES: u64 = 200;
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
    /// m·2^p/8 with p from `pows`.
    fn wide(&mut self, pows: &[i32]) -> f64 { let p = pows[self.usize(0, pows.len() - 1)]; self.int(-8, 8) as f64 * 2f64.powi(p) / 8.0 }
    fn delta(&mut self) -> f64 { self.int(0, 4) as f64 / 16.0 }
}
fn for_all(name: &str, prop: impl Fn(&mut Rng)) {
    for seed in 0..CASES {
        let mut r = Rng(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ 0x45);
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| prop(&mut r)));
        if caught.is_err() { panic!("property {name} failed at seed {seed}"); }
    }
}

// ── The exact reference: values scaled by 2¹², products by 2²⁴ ────────────

const S1: f64 = 4096.0;
fn to_i(x: f64) -> i128 { let y = x * S1; assert_eq!(y.fract(), 0.0); y as i128 }
fn near1(n: i128) -> f64 { n as f64 / S1 }
fn near2(n: i128) -> f64 { n as f64 / (S1 * S1) }
/// x·2^(12·level) ≥ n exactly, and the next f64 down is below n.
fn is_up(x: f64, n: i128, level: i32) -> bool {
    let scale = S1.powi(level);
    let ge = |y: f64| { let z = y * scale; if z.fract() == 0.0 { z as i128 >= n } else { z.floor() as i128 >= n } };
    ge(x) && !ge(x.next_down())
}

struct Field { rel: DeclaredRelations, seq: Vec<Registration>, x: Vec<Vec<Vec<i128>>>, d: Vec<Vec<i128>> }

/// Magnitudes up to 2⁵⁶: a difference or sum then needs more bits than an
/// f64 holds. For sums, differences and signs.
const SUMS: [i32; 3] = [0, 30, 56];
/// Magnitudes up to 2²⁵: a product of two values then needs up to 56 bits,
/// while the scaled i128 products stay below 2¹⁰⁰. For minors.
const PRODUCTS: [i32; 3] = [0, 12, 25];

fn random_field(r: &mut Rng, pows: &[i32]) -> Field {
    let n = r.usize(3, 5);
    let m = r.usize(n - 1, 6.min(n * (n - 1)));
    let mut edges: Vec<(usize, usize)> = Vec::new();
    while edges.len() < m {
        let (s, t) = (r.usize(0, n - 1), r.usize(0, n - 1));
        if s != t && !edges.contains(&(s, t)) { edges.push((s, t)); }
    }
    let rel = rel(n, edges);
    let k = 2;
    let delta: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| r.delta()).collect()).collect();
    let mut xs: Vec<Vec<Vec<f64>>> = Vec::new();
    let mut x: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| r.wide(pows)).collect()).collect();
    for _ in 0..r.usize(3, 4) {
        xs.push(x.clone());
        for c in x.iter_mut() { for v in c.iter_mut() { if r.chance() { *v += r.wide(pows); } } }
    }
    let seq = xs.iter().map(|xt| Registration::synthetic(NodeField::new(xt.clone()), delta.clone())).collect();
    let xi = xs.iter().map(|t| t.iter().map(|c| c.iter().map(|&v| to_i(v)).collect()).collect()).collect();
    let d = delta.iter().map(|c| c.iter().map(|&v| to_i(v)).collect()).collect();
    Field { rel, seq, x: xi, d }
}
impl Rng { fn chance(&mut self) -> bool { self.next() % 8 < 5 } }

impl Field {
    fn a(&self, t: usize, c: usize, e: usize) -> i128 { let (s, u) = self.rel.edges[e]; self.x[t][c][s] - self.x[t][c][u] }
    fn da(&self, c: usize, e: usize) -> i128 { let (s, u) = self.rel.edges[e]; self.d[c][s] + self.d[c][u] }
    fn dlt(&self, s: usize, c: usize, e: usize) -> i128 { self.a(s + 1, c, e) - self.a(s, c, e) }
    fn ddlt(&self, c: usize, e: usize) -> i128 { 2 * self.da(c, e) }
}

fn pb(x: i128, hx: i128, y: i128, hy: i128) -> i128 { x.abs() * hy + y.abs() * hx + hx * hy }

#[test]
fn prop_pc1_is_exact() {
    let lossy = Mutex::new(0usize);
    for_all("exact PC1", |r| {
        let f = random_field(r, &SUMS);
        let s = EdgeSeries::from_registrations(&f.seq, &f.rel).unwrap();
        for x in relation_changes(&s) {
            let (d, h) = (f.dlt(x.step, x.component, x.edge), f.ddlt(x.component, x.edge));
            assert_eq!(x.change, near1(d));
            assert!(is_up(x.half_width, h, 1));
            assert_eq!(x.reading == Reading::Freedom, d.abs() > h);
            // Would Δ from the rounded A have differed?
            let naive = s.a[x.step + 1][x.component][x.edge] - s.a[x.step][x.component][x.edge];
            if naive != x.change { *lossy.lock().unwrap() += 1; }
            assert_eq!(reverse_relation(Inputs::Nodes { seq: &f.seq, rel: &f.rel }, &x), Verdict::Match);
        }
    });
    assert!(lossy.into_inner().unwrap() > 0, "Δ from rounded A never differed: the fixture does not test exactness");
}

#[test]
fn prop_pc2_is_exact() {
    let lossy = Mutex::new(0usize);
    for_all("exact PC2", |r| {
        let f = random_field(r, &PRODUCTS);
        let s = EdgeSeries::from_registrations(&f.seq, &f.rel).unwrap();
        for cs in [coupling_across_components(&s), coupling_across_steps(&s)] {
            let Ok(cs) = cs else { continue };
            for x in &cs.readings {
                let ((s0, c0), (s1, c1)) = match x.over {
                    ReadingPair::Components { step, i, j } => ((step, i), (step, j)),
                    ReadingPair::Steps { component, step } => ((step, component), (step + 1, component)),
                };
                let (xe, xf, ye, yf) = (f.dlt(s0, c0, x.e), f.dlt(s0, c0, x.f), f.dlt(s1, c1, x.e), f.dlt(s1, c1, x.f));
                let (hxe, hxf, hye, hyf) = (f.ddlt(c0, x.e), f.ddlt(c0, x.f), f.ddlt(c1, x.e), f.ddlt(c1, x.f));
                let m = xe * yf - ye * xf;
                let h = pb(xe, hxe, yf, hyf) + pb(ye, hye, xf, hxf);
                assert_eq!(x.minor, near2(m));
                assert!(is_up(x.half_width, h, 2));
                assert_eq!(x.reading == Reading::Freedom, m.abs() > h);
                assert_eq!(reverse_coupling(Inputs::Nodes { seq: &f.seq, rel: &f.rel }, x), Verdict::Match);
                // Would the f64 minor of the reported changes have differed?
                let d = |s: usize, c: usize, e: usize| f.dlt(s, c, e) as f64 / S1;
                if d(s0, c0, x.e) * d(s1, c1, x.f) - d(s1, c1, x.e) * d(s0, c0, x.f) != x.minor { *lossy.lock().unwrap() += 1; }
            }
        }
    });
    assert!(lossy.into_inner().unwrap() > 0, "the f64 minor never differed: the fixture does not test exactness");
}

#[test]
fn prop_pc3_states_are_exact() {
    let states = Mutex::new(0usize);
    for_all("exact PC3", |r| {
        let f = random_field(r, &PRODUCTS);
        let s = EdgeSeries::from_registrations(&f.seq, &f.rel).unwrap();
        let Ok(arr) = arrangement_steps(&s) else { return };
        let state = |t: usize, e: usize, g: usize, i: usize, j: usize| {
            let m = f.a(t, i, e) * f.a(t, j, g) - f.a(t, j, e) * f.a(t, i, g);
            let h = pb(f.a(t, i, e), f.da(i, e), f.a(t, j, g), f.da(j, g)) + pb(f.a(t, j, e), f.da(j, e), f.a(t, i, g), f.da(i, g));
            if m.abs() > h { AtDelta::Resolved(if m > 0 { Sign::Positive } else { Sign::Negative }) } else { AtDelta::NotResolved }
        };
        for x in &arr {
            assert_eq!(x.before, state(x.step, x.e, x.f, x.i, x.j));
            assert_eq!(x.after, state(x.step + 1, x.e, x.f, x.i, x.j));
            assert_eq!(reverse_arrangement(Inputs::Nodes { seq: &f.seq, rel: &f.rel }, x), Verdict::Match);
            *states.lock().unwrap() += 1;
        }
    });
    assert!(states.into_inner().unwrap() > 0);
}

#[test]
fn prop_connection_grades_and_energy_are_exact() {
    let checked = Mutex::new(0usize);
    for_all("exact V1, V3", |r| {
        let f = random_field(r, &SUMS);
        let n_t = f.seq.len();
        let m = f.rel.n_edges();
        let resolved = |t: usize, e: usize| (0..2).any(|c| f.a(t, c, e).abs() > f.da(c, e));
        let held: Vec<usize> = (0..m).filter(|&e| (0..n_t).all(|t| resolved(t, e))).collect();
        // Identity: A[e] − A[f] within δA[e] + δA[f] in every component and registration.
        let mut ident = Vec::new();
        for (i, &e) in held.iter().enumerate() {
            for &g in &held[i + 1..] {
                if (0..n_t).all(|t| (0..2).all(|c| (f.a(t, c, e) - f.a(t, c, g)).abs() <= f.da(c, e) + f.da(c, g))) { ident.push((e, g)); }
            }
        }
        let conn = connection(&f.seq, &f.rel).unwrap();
        assert_eq!(conn.identity, ident);
        assert_eq!(conn.unresolved_edges, (0..m).filter(|e| !held.contains(e)).collect::<Vec<_>>());
        // Sustained: the sign of Ξ(B(A)) at δ, exactly, at every registration.
        if let Ok(sus) = &conn.sustained {
            let b = |t: usize, c: usize, g: usize| f.a(t, c, g) + f.rel.succ(g).iter().map(|&h| f.a(t, c, h)).sum::<i128>();
            let db = |c: usize, g: usize| f.da(c, g) + f.rel.succ(g).iter().map(|&h| f.da(c, h)).sum::<i128>();
            for e in 0..m {
                if f.rel.succ(e).is_empty() && f.rel.pred(e).is_empty() { continue; }
                let sign = |t: usize, c: usize| {
                    let v = f.rel.succ(e).iter().map(|&g| b(t, c, g)).sum::<i128>() - f.rel.pred(e).iter().map(|&g| b(t, c, g)).sum::<i128>();
                    let h = f.rel.succ(e).iter().chain(f.rel.pred(e)).map(|&g| db(c, g)).sum::<i128>();
                    if v.abs() > h { v.signum() } else { 0 }
                };
                let held_c = (0..2).any(|c| { let s0 = sign(0, c); s0 != 0 && (0..n_t).all(|t| sign(t, c) == s0) });
                assert_eq!(sus.held_edges.contains(&e), held_c, "sustained grade at edge {e}");
            }
        }
        // Energy: the first component as a carrier.
        let carrier = EnergyCarrier { range: abr_kernel::ground_variables::ObservedRange::synthetic(), name: "SYNTHETIC".into(), unit: "u".into(),
            values: f.seq[0].x.data[0].clone(), delta: f.seq[0].delta[0].clone() };
        for x in energy_exchange(&carrier, &f.rel) {
            let (s, u) = f.rel.edges[x.edge];
            let (d, h) = (f.x[0][0][s] - f.x[0][0][u], f.d[0][s] + f.d[0][u]);
            assert_eq!(x.epsilon, near1(d.abs()));
            assert!(is_up(x.delta_epsilon, h, 1));
            assert_eq!(x.floor_resolved, d.abs() > h);
        }
        *checked.lock().unwrap() += 1;
    });
    assert!(checked.into_inner().unwrap() > 0);
}

#[test]
fn prop_relational_field_forms_are_exact() {
    let lossy = Mutex::new(0usize);
    for_all("exact forms", |r| {
        let f = random_field(r, &SUMS);
        let rf = RelationalField::new(&f.rel, f.seq.clone(), None).unwrap();
        let m = f.rel.n_edges();
        let check = |forms: &abr_kernel::relational_field::Forms, val: &dyn Fn(usize, usize) -> i128| {
            let b = |c: usize, g: usize| val(c, g) + f.rel.succ(g).iter().map(|&h| val(c, h)).sum::<i128>();
            for c in 0..2 {
                for e in 0..m {
                    assert_eq!(forms.a[c][e], near1(val(c, e)));
                    assert_eq!(forms.b[c][e], near1(b(c, e)));
                    let xi = f.rel.succ(e).iter().map(|&g| b(c, g)).sum::<i128>() - f.rel.pred(e).iter().map(|&g| b(c, g)).sum::<i128>();
                    assert_eq!(forms.circulation[c][e], near1(xi));
                }
            }
        };
        for t in 0..f.seq.len() { check(&rf.spatial[t], &|c, e| f.a(t, c, e)); }
        for s in 0..rf.n_steps() {
            check(&rf.persistence[s], &|c, e| f.dlt(s, c, e));
            if (0..2).any(|c| (0..m).any(|e| rf.spatial[s + 1].a[c][e] - rf.spatial[s].a[c][e] != rf.persistence[s].a[c][e])) {
                *lossy.lock().unwrap() += 1;
            }
        }
    });
    assert!(lossy.into_inner().unwrap() > 0, "A_p from rounded A never differed: the fixture does not test exactness");
}

/// The PC3 property above passes on the earlier arithmetic too: on random
/// fields no exact minor and its f64 counterpart fall on different sides of
/// their bound. This case does. With A[0][e] = A[1][f] = 1 + 2⁻³⁰,
/// A[1][e] = 1 and A[0][f] = 1 + 2⁻²⁹, the exact minor is
/// (1 + 2⁻³⁰)² − (1 + 2⁻²⁹) = 2⁻⁶⁰. In f64, (1 + 2⁻³⁰)² rounds to 1 + 2⁻²⁹
/// and the minor reads 0. At zero repeatability the exact minor is resolved
/// positive; the earlier arithmetic read it as not resolved.
#[test]
fn a_minor_of_2_to_the_minus_60_is_resolved() {
    let u = 1.0 + 2f64.powi(-30);
    let v = 1.0 + 2f64.powi(-29);
    assert_eq!(u * u, v, "the case depends on (1 + 2⁻³⁰)² rounding to 1 + 2⁻²⁹");
    // Loci 0, 1, 2; e = 0 → 1, f = 0 → 2; A = x[s] − x[t], so x[1] = −A[e], x[2] = −A[f].
    let r = rel(3, vec![(0, 1), (0, 2)]);
    let x = NodeField::new(vec![vec![0.0, -u, -v], vec![0.0, -1.0, -u]]);
    let reg = Registration::synthetic(x, vec![vec![0.0; 3]; 2]);
    let p = minor_pattern(&reg, &r).unwrap();
    assert_eq!(p.minors, vec![(0, 1, 0, 1, AtDelta::Resolved(Sign::Positive))]);
    let s = EdgeSeries::from_registrations(&[reg.clone(), reg.clone()], &r).unwrap();
    let a = arrangement_steps(&s).unwrap();
    assert_eq!((a[0].before, a[0].after), (AtDelta::Resolved(Sign::Positive), AtDelta::Resolved(Sign::Positive)));
    assert_eq!(reverse_arrangement(Inputs::Nodes { seq: &[reg.clone(), reg], rel: &r }, &a[0]), Verdict::Match);
}
