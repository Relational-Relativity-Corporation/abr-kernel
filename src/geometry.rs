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
