// geometry.rs — Metatron Dynamics, Inc. Kernel V10.3. Bounded over D. No claim beyond D.
//
// §2.9, the geometry of relations (Proposed, Generator, 2026-11-23, 69th;
// Origin: *"We clearly need to establish this relationship... I'm
// particularly interested in oscillation or frequency and its relationship to
// geometry"*).
//
// A relation at registration t is a vector over its components, A_t[e]. In
// each pair of components (i, j), a 2-plane, two vectors u and v have two
// bilinear readings, both linear algebra:
//
//   area   u_i·v_j − u_j·v_i     the signed area they span (the sine side)
//   inner  u_i·v_i + u_j·v_j     their inner product (the cosine side)
//
// Each is formed exactly from the registered relations and read at δ. The
// angle itself (atan2 of the two) is not computed: it needs a function outside
// linear algebra (W12), so it is a declared projection beside the parts, and
// the parts are what is reported (§1.5, decompose, do not simplify).
//
//   G1  the angle between two relations e, f at one registration
//       (area = the minor m_ij(e, f) of V8; zero area is shared direction)
//   G2  the step angle of one relation: its states at s and s + 1. The area's
//       sign is the sense in which the relation's direction turns over the
//       step; turning by a constant step angle is oscillation, and frequency
//       is the step angle per step, counted in steps (W15)
//   G3  the relative phase of two relations over a step: the drift
//       area(G1_s, G1_{s+1}) of the pair (inner, area) itself. Held: the
//       relative angle is kept (phase-locked; I-SC-D6). Resolved: it drifts
//       (beating), and its sign is the sense of the drift
//   G4  (in the text) frequency is read at a declared scale: a step equal to a
//       period reads held (I-SC-D19 (ii))
//   G6  (Proposed, 70th) a relation against a declared direction n over its
//       components: the held part n·u (the inner product, term by term, one
//       exact sum: the balance, §2.8 item 2a) and the exchange per plane,
//       n_i·u_j − n_j·u_i (the area of n and u). Over a step, read on the
//       change Δ = A_{s+1} − A_s: a held part held at δ is a balance held
//       (connection as shared balance, item 2b), and a resolved exchange is
//       carrier moved between components, its sign the sense (toward j
//       positive). Nothing is dropped: for every component c,
//       |n|²·u_c = (n·u)·n_c + Σ_j n_j·(n_j·u_c − n_c·u_j), exactly
//
// Words (W16): "rotation" and "oscillation" are labels beside the object, the
// angle between consecutive states of a relation.
//
// Component scale: an inner product adds products of different components,
// and the area's magnitude changes under a scale that differs between
// components (I-SC-D15). Every reading is of the components as declared; a
// different declared component scale is a different reading. Gains (Z1a): the
// area across two relations takes the independent gain (as the minors, PC3);
// the step area takes none (each entry's run-constant factor is common to both
// of its terms); every inner product takes every declared gain, since its two
// terms can carry different factors. Bounds are never below the exact bound.

use crate::exact::{gain_widened, product_bound, Expansion};
use crate::ground_variables::{sign_at_delta_exact, AtDelta, DeclarationGap, Sign};
use crate::persistence_change::{combined_gain, EdgeSeries};

/// One bilinear reading: the value (nearest f64 to the exact value), its
/// bound (rounded up from the exact bound), and its reading at δ, decided
/// exactly.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlaneValue {
    pub value: f64,
    pub bound: f64,
    pub reading: AtDelta<Sign>,
}

impl PlaneValue {
    fn of(v: &Expansion, h: &Expansion) -> Self {
        PlaneValue { value: v.nearest(), bound: h.up(), reading: sign_at_delta_exact(v, h) }
    }
}

/// G1 at registration t: relations e < f in the component plane (i, j).
#[derive(Debug, Clone, PartialEq)]
pub struct Angle { pub registration: usize, pub e: usize, pub f: usize, pub i: usize, pub j: usize, pub area: PlaneValue, pub inner: PlaneValue }

/// G2 over step s: relation e in the plane (i, j), its state at s against s + 1.
#[derive(Debug, Clone, PartialEq)]
pub struct StepAngle { pub step: usize, pub e: usize, pub i: usize, pub j: usize, pub area: PlaneValue, pub inner: PlaneValue }

/// G3 over step s: the drift of the relative angle of e < f in the plane (i, j).
#[derive(Debug, Clone, PartialEq)]
pub struct RelativePhase { pub step: usize, pub e: usize, pub f: usize, pub i: usize, pub j: usize, pub drift: PlaneValue }

/// An entry with its bound widened by a gain bound (h + γ·(|v| + h)).
fn entry(s: &EdgeSeries, t: usize, c: usize, e: usize, gain: f64) -> (Expansion, Expansion) {
    let v = s.entry(t, c, e);
    let h = gain_widened(&v, &Expansion::of(s.da[t][c][e]), gain);
    (v, h)
}

fn all_gains(s: &EdgeSeries, c: usize, e: usize) -> f64 { combined_gain(s.shared_gain[c][e], s.mixed_gain[c][e]) }

/// area = u_i·v_j − u_j·v_i and inner = u_i·v_i + u_j·v_j, exactly, with
/// their product bounds. `u` and `v` are (value, bound) in components i, j:
/// for the area, `ua`, `va`; for the inner product, `ub`, `vb`.
type Pair = [(Expansion, Expansion); 2];
fn area_of(u: &Pair, v: &Pair) -> (Expansion, Expansion) {
    let val = u[0].0.times(&v[1].0).minus(&u[1].0.times(&v[0].0));
    let h = product_bound(&u[0].0, &u[0].1, &v[1].0, &v[1].1).plus(&product_bound(&u[1].0, &u[1].1, &v[0].0, &v[0].1));
    (val, h)
}
fn inner_of(u: &Pair, v: &Pair) -> (Expansion, Expansion) {
    let val = u[0].0.times(&v[0].0).plus(&u[1].0.times(&v[1].0));
    let h = product_bound(&u[0].0, &u[0].1, &v[0].0, &v[0].1).plus(&product_bound(&u[1].0, &u[1].1, &v[1].0, &v[1].1));
    (val, h)
}

fn planes(k: usize) -> Vec<(usize, usize)> {
    (0..k).flat_map(|i| ((i + 1)..k).map(move |j| (i, j))).collect()
}

fn need(s: &EdgeSeries, edges: usize) -> Result<(), DeclarationGap> {
    if s.k() < 2 { return Err(DeclarationGap::SingleComponent); }
    if s.n_edges() < edges { return Err(DeclarationGap::FewerThanTwoEdges); }
    Ok(())
}

fn g1(s: &EdgeSeries, t: usize, e: usize, f: usize, i: usize, j: usize) -> ((Expansion, Expansion), (Expansion, Expansion)) {
    let m = |c, x| entry(s, t, c, x, s.mixed_gain[c][x]);
    let g = |c, x| entry(s, t, c, x, all_gains(s, c, x));
    (area_of(&[m(i, e), m(j, e)], &[m(i, f), m(j, f)]), inner_of(&[g(i, e), g(j, e)], &[g(i, f), g(j, f)]))
}

/// G1 at registration t, for every pair of relations and every component plane.
pub fn angles(s: &EdgeSeries, t: usize) -> Result<Vec<Angle>, DeclarationGap> {
    need(s, 2)?;
    let mut out = Vec::new();
    for e in 0..s.n_edges() { for f in (e + 1)..s.n_edges() { for &(i, j) in &planes(s.k()) {
        let ((av, ah), (iv, ih)) = g1(s, t, e, f, i, j);
        out.push(Angle { registration: t, e, f, i, j, area: PlaneValue::of(&av, &ah), inner: PlaneValue::of(&iv, &ih) });
    }}}
    Ok(out)
}

/// G2 over step s (registration s to s + 1), for every relation and plane.
pub fn step_angles(s: &EdgeSeries, step: usize) -> Result<Vec<StepAngle>, DeclarationGap> {
    need(s, 1)?;
    let mut out = Vec::new();
    for e in 0..s.n_edges() { for &(i, j) in &planes(s.k()) {
        let a = |t, c| entry(s, t, c, e, 0.0);
        let g = |t, c| entry(s, t, c, e, all_gains(s, c, e));
        let (av, ah) = area_of(&[a(step, i), a(step, j)], &[a(step + 1, i), a(step + 1, j)]);
        let (iv, ih) = inner_of(&[g(step, i), g(step, j)], &[g(step + 1, i), g(step + 1, j)]);
        out.push(StepAngle { step, e, i, j, area: PlaneValue::of(&av, &ah), inner: PlaneValue::of(&iv, &ih) });
    }}
    Ok(out)
}

/// G3 over step s: drift = inner_s·area_{s+1} − area_s·inner_{s+1}, the area
/// spanned by the pair (inner, area) of G1 at s and at s + 1, exactly, with
/// its product bound. Held: the relative angle of e and f is kept at δ.
pub fn relative_phases(s: &EdgeSeries, step: usize) -> Result<Vec<RelativePhase>, DeclarationGap> {
    need(s, 2)?;
    let mut out = Vec::new();
    for e in 0..s.n_edges() { for f in (e + 1)..s.n_edges() { for &(i, j) in &planes(s.k()) {
        let (a0, i0) = g1(s, step, e, f, i, j);
        let (a1, i1) = g1(s, step + 1, e, f, i, j);
        let (dv, dh) = area_of(&[i0, a0], &[i1, a1]);
        out.push(RelativePhase { step, e, f, i, j, drift: PlaneValue::of(&dv, &dh) });
    }}}
    Ok(out)
}

// ═══════════════════════════════════════════════════════════════════════════
// G6: held part and exchange against a declared direction (Proposed,
// Generator, 2026-11-24, 70th; Origin: "Let's go back to this at the primary
// region because that's where we were seeing consequences of energy and
// geometry")
// ═══════════════════════════════════════════════════════════════════════════
//
// The direction n is declared, never registered: it states how each
// component counts toward the balance, with its basis (for components in one
// unit counting equally, n = (1, …, 1)). It enters exactly, with no bound.
// Gains (Z1a): the held part and the exchange both add terms from different
// components, which can carry different run-constant factors, so every entry
// takes every declared gain (as the inner product of G1, G2).

/// A declared direction over the components, with its basis (G6).
#[derive(Debug, Clone, PartialEq)]
pub struct DeclaredDirection {
    pub n: Vec<f64>,
    pub basis: String,
}

impl DeclaredDirection {
    pub fn new(n: Vec<f64>, basis: &str) -> Self {
        assert!(!basis.trim().is_empty(), "a declared direction states its basis");
        assert!(n.iter().all(|x| x.is_finite() && crate::exact::in_exact_range(*x)),
            "a declared direction is finite and within the range of exact arithmetic (OC-NUM-1)");
        assert!(n.iter().any(|&x| x != 0.0), "a declared direction is not zero");
        DeclaredDirection { n, basis: basis.into() }
    }
}

/// G6 for relation e: the held part and the exchange against n, at one
/// registration (`registration: Some(t)`) or on the change over a step
/// (`step: Some(s)`).
#[derive(Debug, Clone, PartialEq)]
pub struct HeldAndExchange {
    pub registration: Option<usize>,
    pub step: Option<usize>,
    pub e: usize,
    /// n_c·u_c for every component c, in component order.
    pub terms: Vec<PlaneValue>,
    /// Σ_c n_c·u_c, one exact sum with the sum of the term bounds.
    pub held: PlaneValue,
    /// (i, j, n_i·u_j − n_j·u_i) for every plane i < j.
    pub exchange: Vec<(usize, usize, PlaneValue)>,
}

fn held_and_exchange(u: &[(Expansion, Expansion)], n: &[f64]) -> (Vec<PlaneValue>, PlaneValue, Vec<(usize, usize, PlaneValue)>) {
    let zero = Expansion::new();
    let nx: Vec<Expansion> = n.iter().map(|&x| Expansion::of(x)).collect();
    let term = |c: usize| (nx[c].times(&u[c].0), product_bound(&nx[c], &zero, &u[c].0, &u[c].1));
    let mut hv = Expansion::new();
    let mut hh = Expansion::new();
    let mut terms = Vec::new();
    for c in 0..u.len() {
        let (v, h) = term(c);
        hv.add_expansion(&v);
        hh.add_expansion(&h);
        terms.push(PlaneValue::of(&v, &h));
    }
    let exchange = planes(u.len()).into_iter().map(|(i, j)| {
        let v = nx[i].times(&u[j].0).minus(&nx[j].times(&u[i].0));
        let h = product_bound(&nx[i], &zero, &u[j].0, &u[j].1).plus(&product_bound(&nx[j], &zero, &u[i].0, &u[i].1));
        (i, j, PlaneValue::of(&v, &h))
    }).collect();
    (terms, PlaneValue::of(&hv, &hh), exchange)
}

fn check_direction(s: &EdgeSeries, n: &DeclaredDirection) -> Result<(), DeclarationGap> {
    need(s, 1)?;
    assert_eq!(n.n.len(), s.k(), "the direction is declared over every component");
    Ok(())
}

/// G6 at registration t, for every relation.
pub fn held_exchange(s: &EdgeSeries, t: usize, n: &DeclaredDirection) -> Result<Vec<HeldAndExchange>, DeclarationGap> {
    check_direction(s, n)?;
    Ok((0..s.n_edges()).map(|e| {
        let u: Vec<_> = (0..s.k()).map(|c| entry(s, t, c, e, all_gains(s, c, e))).collect();
        let (terms, held, exchange) = held_and_exchange(&u, &n.n);
        HeldAndExchange { registration: Some(t), step: None, e, terms, held, exchange }
    }).collect())
}

/// G6 on the change over step s (registration s to s + 1), for every
/// relation: Δ_c = A_{s+1} − A_s exactly, with the sum of the two bounds.
pub fn step_held_exchange(s: &EdgeSeries, step: usize, n: &DeclaredDirection) -> Result<Vec<HeldAndExchange>, DeclarationGap> {
    check_direction(s, n)?;
    Ok((0..s.n_edges()).map(|e| {
        let u: Vec<_> = (0..s.k()).map(|c| {
            let (v0, h0) = entry(s, step, c, e, all_gains(s, c, e));
            let (v1, h1) = entry(s, step + 1, c, e, all_gains(s, c, e));
            (v1.minus(&v0), h1.plus(&h0))
        }).collect();
        let (terms, held, exchange) = held_and_exchange(&u, &n.n);
        HeldAndExchange { registration: None, step: Some(step), e, terms, held, exchange }
    }).collect())
}
