// persistence_change.rs — Metatron Dynamics, Inc. Kernel V10.
// Structure and freedom as persistence and change (PC1–PC4).
// Bounded over D. No claim beyond D.
//
// STATUS: DECLARED (Origin, 2026-11-21, 7th; V10_README Part 9). Drafted by
// the Generator the same day as a Proposed item (commit dffee32); adopted with
// ER-1, the restatement of V2/V8/I-SC-1 (conflict 10), the single statement of
// connection, and energy as a component.
//
// ── The principle (Origin, 2026-11-21, as stated) ─────────────────────────
//
//   Where relations permit change, freedom increases.
//   Where constraints limit change, structure increases.
//   Structure is persistence. Change is freedom.
//
// ── The object ────────────────────────────────────────────────────────────
//
// Declared edges e, components c, a declared sequence of registrations
// t = 0..n−1 ordered by an observable property of the process (T6). At each
// registration the edge field A_t[c][e] with half-width δA_t[c][e]. Change is
// read only between consecutive registrations (Df3):
//
//     Δ_s[c][e] = A_{s+1}[c][e] − A_s[c][e]          (A_p, K5)
//     δΔ_s[c][e] = δA_s[c][e] + δA_{s+1}[c][e]
//
// The prior is always the earlier registration (Df2). Nothing is read across
// non-consecutive registrations.
//
// ── Three levels, one rule ────────────────────────────────────────────────
//
// At every level the same rule applies: a reading whose change is resolved at
// δ is FREEDOM (change permitted); one whose change is not resolved is
// STRUCTURE (change held below δ by constraint: persistence).
//
//   PC1  Relation.    Does relation e change in component c over step s?
//                     |Δ_s[c][e]| > δΔ_s[c][e]  →  freedom; else structure.
//
//   PC2  Coupling.    Do two relations change together or independently?
//                     The change minor over two consecutive readings r, r′:
//                       μ(e,f) = Δ_r[e]·Δ_r′[f] − Δ_r′[e]·Δ_r[f]
//                     Not resolved → their changes are locked in proportion:
//                     one constraint holds them together (shared structure,
//                     connection). Resolved → they change independently
//                     (freedom). Two families of reading pairs:
//                       (a) two components i < j at one step s   (k ≥ 2)
//                       (b) one component at steps s, s+1         (n ≥ 3)
//                     (b) is consecutive by construction (Df3).
//
//   PC3  Arrangement. Does the relative arrangement of two relations hold?
//                     The minor on A, m_ij(e,f) (V8), read at registrations
//                     s and s+1: the same state at δ → structure; a different
//                     state → freedom (the arrangement changed).
//
//   PC4  Shares.      At each level, over the same evaluated set:
//                       F_δ = freedom / evaluated,  S_δ = structure / evaluated,
//                       F_δ + S_δ = 1.
//                     A declared summary (C), not the object.
//
// ── Derived (mathematical; tested in tests/v10_persistence_change.rs) ─────
//
//   PC-D1  Complementarity. At each level every evaluated reading is exactly
//          one of structure or freedom at δ; S_δ + F_δ = 1.
//   PC-D2  Resolution. Every half-width here is non-decreasing in δ, so a
//          reading that is freedom at δ′ is freedom at every δ ≤ δ′. Over a
//          fixed evaluated set F_δ is non-increasing in δ. Structure at a
//          coarse δ is change held below that δ, never a statement that
//          nothing changed (O1, W2).
//          CORRECTED (2026-11-21, found by tests/v10_properties.rs): this
//          holds for PC1 and PC2, which compare one value with its
//          half-width. It does not hold for PC3, which compares two states:
//          coarsening δ can turn resolved → resolved (structure) into
//          resolved → not resolved (freedom). What holds for PC3: a crossing
//          (opposite resolved signs) at δ′ is a crossing at every δ ≤ δ′.
//   PC-D3  Invariance. Offsets cancel in A and Δ. Rescaling a component
//          (values and δ together, by λ ≠ 0) leaves every reading unchanged.
//          Swapping e and f negates every minor and changes no reading.
//   PC-D4  Forced constraint. In a node-derived series, edges with the same
//          endpoints have identical A at every registration, so their
//          coupling is fixed by the declaration: an identity, not a reading
//          (V1). Such pairs are excluded and listed. Every cycle of a
//          node-derived series also satisfies Σ ±Δ[e] = 0 identically (K7);
//          only edge-registered values make that a test.
//   PC-D5  Minimum information. PC1 and PC3 need n ≥ 2; PC2(a) needs k ≥ 2;
//          PC2(b) needs n ≥ 3. Below that the result is a declaration gap.
//
// ── The three ground variables on this object (Declared 2026-11-21) ───────
//
//   Superposition (freedom, I-S): change permitted, read at PC1–PC3. This
//     restates V2 (conflict 10). The first V2 form, a minor held resolved,
//     reads a persistent arrangement: structure. It is kept unchanged in
//     ground_variables.rs as `freedom` for declared comparison (C).
//   Entanglement (connection, I-E): shared structure, relations held together
//     by a constraint, in their state (V1 grades: identity, proportional,
//     sustained) or in their change (PC2: the coupled-change grade). The
//     grades are read beside each other, never merged.
//   Tunneling (energy, I-T): what is exchanged across a relation,
//     ε[e] = |A(c)[e]| (V3), floor resolved when ε > δ_ε. Where the carrier
//     is registered at the same loci (T5) it enters the series as a component,
//     and its change is read by the same forms: PC1 on the carrier (does the
//     exchange persist or change?) and PC2(a) between the carrier and a
//     structural component (is change of exchange locked to structural change
//     or independent of it?). This states OC-SC-3 in the declared mathematics;
//     the answer comes from registered data.

use crate::ground_variables::{product_half_width, sign_at_delta, AtDelta, DeclarationGap, Registration};
use crate::operators::{operator_a, DeclaredRelations};

/// The reading at δ, at every level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reading {
    /// Change not resolved at δ: held by constraint (persistence).
    Structure,
    /// Change resolved at δ: permitted.
    Freedom,
}

fn read(value: f64, half_width: f64) -> Reading {
    if sign_at_delta(value, half_width).is_resolved() { Reading::Freedom } else { Reading::Structure }
}

/// Where the edge values came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeOrigin {
    /// A[e] = x[s] − x[t] from node-valued registrations.
    NodeDerived,
    /// A registered value per declared edge (R3, ratio form; ER-1).
    EdgeRegistered,
}

/// A declared sequence of edge fields with half-widths.
///
/// δ in two parts (Z1a; see `Registration`): `da` is the **state** bound
/// (repeatability + offset + unshared gain on the value), used by the state
/// forms (PC3). A change Δ = a[s+1] − a[s] is bounded by the repeatability of
/// both registrations plus the unshared gain on each end's own change
/// (`dgain`); offsets cancel in every change and do not enter. A shared gain
/// changes no classification and enters only `change_magnitude`.
///
/// **Gains on edge-registered values (corrected after the Verifier's 10.3.0
/// review, 2026-11-22, 34th).** A gain factor common to every edge of a
/// component, or common to every component of an edge, multiplies each minor
/// by one positive factor ((1+γ_i)(1+γ_j), or (1+γ_e)(1+γ_f)) and changes no
/// classification: it is *shared* and enters `change_magnitude` only. An
/// *independent* gain on one (component, edge) entry does not: in
/// m = Δ_i[e]·Δ_j[f] − Δ_j[e]·Δ_i[f] it scales one term and not the other,
/// so it can move a minor across zero. It enters the bound of every minor
/// that mixes components (PC2(a), PC3), as γ·|value| on that entry
/// (`mixed_gain`). It is not added to PC1 or PC2(b): a factor on one entry
/// scales Δ there by one positive factor (PC1), and over one component the
/// same entries carry the same factor at both steps (PC2(b)), so neither
/// classification changes.
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeSeries {
    /// a[t][c][e]
    pub a: Vec<Vec<Vec<f64>>>,
    /// State bound δA[t][c][e].
    pub da: Vec<Vec<Vec<f64>>>,
    /// Repeatability part of δA[t][c][e] (Z1a).
    pub da_rep: Vec<Vec<Vec<f64>>>,
    /// Unshared-gain part of the change bound [step][c][e]: γ_s·|Δx_s| + γ_t·|Δx_t| (Z1a).
    pub dgain: Vec<Vec<Vec<f64>>>,
    /// Shared gain bound [c][e] (a component factor times an edge factor):
    /// enters reported change magnitudes only (Z1a).
    pub shared_gain: Vec<Vec<f64>>,
    /// Independent gain bound [c][e] on edge-registered values: enters the
    /// bound of every minor that mixes components (PC2(a), PC3) and reported
    /// magnitudes. Zero for node-derived series, whose unshared gains are
    /// already in `da` and `dgain`.
    pub mixed_gain: Vec<Vec<f64>>,
    pub origin: EdgeOrigin,
    /// Edge pairs whose coupling the declaration fixes (PC-D4).
    pub forced_pairs: Vec<(usize, usize)>,
}

impl EdgeSeries {
    /// From node-valued registrations through A, with δA[e] = δ[s] + δ[t]
    /// (state bounds, Z1a), and the change bound from the repeatability and
    /// the unshared gain on each end's change.
    pub fn from_registrations(seq: &[Registration], rel: &DeclaredRelations) -> Result<Self, DeclarationGap> {
        if seq.len() < 2 { return Err(DeclarationGap::NoProcessSequence); }
        crate::topology::admit_field(rel)?;
        let k = seq[0].x.k;
        assert!(seq.iter().all(|r| r.x.k == k), "every registration must declare the same components");
        assert!(seq.iter().all(|r| r.x.n == rel.n_nodes), "every registration must cover the declared loci");
        if rel.n_edges() < 1 { return Err(DeclarationGap::FewerThanTwoEdges); }
        assert!(seq.iter().all(|r| r.same_run_constant(&seq[0])),
            "the run-constant parts of δ must be the same in every registration of the run (Z1a)");
        let a = seq.iter().map(|r| operator_a(&r.x, rel, &[]).spatial).collect();
        let da = seq.iter().map(|r| (0..k).map(|c| {
            rel.edges.iter().map(|&(s, t)| r.state_delta(c, s) + r.state_delta(c, t)).collect()
        }).collect()).collect();
        let da_rep = seq.iter().map(|r| (0..k).map(|c| {
            rel.edges.iter().map(|&(s, t)| r.delta[c][s] + r.delta[c][t]).collect()
        }).collect()).collect();
        let g = &seq[0].locus_gain;
        let dgain = seq.windows(2).map(|w| (0..k).map(|c| {
            let dx = |i: usize| (w[1].x.data[c][i] - w[0].x.data[c][i]).abs();
            rel.edges.iter().map(|&(s, t)| g[c][s] * dx(s) + g[c][t] * dx(t)).collect()
        }).collect()).collect();
        let shared_gain = (0..k).map(|c| vec![seq[0].shared_gain[c]; rel.n_edges()]).collect();
        let mut forced_pairs = Vec::new();
        for e in 0..rel.n_edges() {
            for f in (e + 1)..rel.n_edges() {
                if rel.edges[e] == rel.edges[f] { forced_pairs.push((e, f)); }
            }
        }
        let mixed_gain = vec![vec![0.0; rel.n_edges()]; k];
        Ok(EdgeSeries { a, da, da_rep, dgain, shared_gain, mixed_gain, origin: EdgeOrigin::NodeDerived, forced_pairs })
    }

    /// From a registered value per declared edge and component, with δ per
    /// edge: the edge-valued registration (ER-1, Declared 2026-11-21). The
    /// relation enters as registered; no node value is constructed.
    pub fn from_edge_values(a: Vec<Vec<Vec<f64>>>, da: Vec<Vec<Vec<f64>>>) -> Result<Self, DeclarationGap> {
        if a.len() < 2 { return Err(DeclarationGap::NoProcessSequence); }
        assert_eq!(a.len(), da.len(), "δ must be declared for every registration");
        let (k, m) = (a[0].len(), a[0].first().map_or(0, |c| c.len()));
        assert!(k > 0, "at least one component required");
        for (at, dt) in a.iter().zip(&da) {
            assert!(at.len() == k && dt.len() == k, "every registration must declare the same components");
            assert!(at.iter().chain(dt).all(|c| c.len() == m), "every registration must cover the declared edges");
            assert!(at.iter().flatten().all(|v| v.is_finite()), "values must be finite (∈ D)");
            assert!(dt.iter().flatten().all(|d| d.is_finite() && *d >= 0.0), "δ must be finite and non-negative");
        }
        if m < 1 { return Err(DeclarationGap::FewerThanTwoEdges); }
        let steps = a.len() - 1;
        Ok(EdgeSeries {
            dgain: vec![vec![vec![0.0; m]; k]; steps],
            shared_gain: vec![vec![0.0; m]; k],
            mixed_gain: vec![vec![0.0; m]; k],
            da_rep: da.clone(),
            a, da, origin: EdgeOrigin::EdgeRegistered, forced_pairs: Vec::new(),
        })
    }

    /// Declare run-constant parts for an edge-valued series (Z1a): an offset
    /// bound per component and edge, entering the state bound only, and the
    /// gain bounds in three parts (`EdgeGains`). Every gain bound in [0, 1).
    ///
    /// *Was (10.3.0 before the Verifier's review):* one gain per component and
    /// edge, treated as shared by its relation and excluded from every
    /// classification. That holds only for gains common to a component or to
    /// an edge: an independent gain can change a minor that mixes components.
    pub fn with_run_constant(mut self, offset: Vec<Vec<f64>>, gains: EdgeGains) -> Self {
        let (k, m) = (self.k(), self.n_edges());
        assert!(offset.len() == k && offset.iter().all(|o| o.len() == m),
            "an offset must be declared for every component and edge");
        assert!(gains.component.len() == k && gains.edge.len() == m
            && gains.independent.len() == k && gains.independent.iter().all(|g| g.len() == m),
            "gains must be declared per component, per edge, and per component and edge");
        assert!(offset.iter().flatten().all(|o| o.is_finite() && *o >= 0.0),
            "an offset bound must be finite and non-negative");
        assert!(gains.component.iter().chain(&gains.edge).chain(gains.independent.iter().flatten())
            .all(|g| g.is_finite() && *g >= 0.0 && *g < 1.0),
            "a gain bound must lie in [0, 1): a declared gain is strictly positive (Z1a)");
        for t in 0..self.n_registrations() {
            for c in 0..k { for e in 0..m { self.da[t][c][e] = self.da_rep[t][c][e] + offset[c][e]; } }
        }
        self.shared_gain = (0..k).map(|c| (0..m)
            .map(|e| (1.0 + gains.component[c]) * (1.0 + gains.edge[e]) - 1.0).collect()).collect();
        self.mixed_gain = gains.independent;
        self
    }

    pub fn n_registrations(&self) -> usize { self.a.len() }
    pub fn n_steps(&self) -> usize { self.a.len() - 1 }
    pub fn k(&self) -> usize { self.a[0].len() }
    pub fn n_edges(&self) -> usize { self.a[0][0].len() }

    /// Δ_s and δΔ_s, [c][e], over step s (registration s to s+1).
    pub fn change(&self, s: usize) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let d = (0..self.k()).map(|c| (0..self.n_edges())
            .map(|e| self.a[s + 1][c][e] - self.a[s][c][e]).collect()).collect();
        let dd = (0..self.k()).map(|c| (0..self.n_edges())
            .map(|e| self.da_rep[s + 1][c][e] + self.da_rep[s][c][e] + self.dgain[s][c][e]).collect()).collect();
        (d, dd)
    }

    /// The bound on a reported change magnitude over step s, [c][e]: the
    /// change bound plus the shared gain on the change, δΔ + γ·|Δ| (Z1a).
    /// Classifications use `change`; this is for reporting |Δ| only.
    pub fn change_magnitude(&self, s: usize) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let (d, dd) = self.change(s);
        let dm = (0..self.k()).map(|c| (0..self.n_edges())
            .map(|e| {
                let g = (1.0 + self.shared_gain[c][e]) * (1.0 + self.mixed_gain[c][e]) - 1.0;
                dd[c][e] + g * d[c][e].abs()
            }).collect()).collect();
        (d, dm)
    }

    /// Bounds for a minor that mixes components: the bound on each entry
    /// widened by its independent gain on the entry's value, δ + γ·|v|.
    fn mixed(&self, v: &[Vec<f64>], dv: &[Vec<f64>]) -> Vec<Vec<f64>> {
        (0..self.k()).map(|c| (0..self.n_edges())
            .map(|e| dv[c][e] + self.mixed_gain[c][e] * v[c][e].abs()).collect()).collect()
    }

    fn is_forced(&self, e: usize, f: usize) -> bool { self.forced_pairs.contains(&(e, f)) }
}

/// Gain bounds on edge-registered values, in three parts (Z1a; corrected
/// after the Verifier's 10.3.0 review). Each bound in [0, 1).
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeGains {
    /// A gain common to every edge of component c: shared.
    pub component: Vec<f64>,
    /// A gain common to every component of edge e: shared.
    pub edge: Vec<f64>,
    /// A gain on one (component, edge) entry alone: enters the bounds of
    /// minors that mix components.
    pub independent: Vec<Vec<f64>>,
}

impl EdgeGains {
    /// No gain declared, for k components and m edges.
    pub fn none(k: usize, m: usize) -> Self {
        EdgeGains { component: vec![0.0; k], edge: vec![0.0; m], independent: vec![vec![0.0; m]; k] }
    }
}

/// Counts at one level over one evaluated set (PC4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Shares {
    pub structure: usize,
    pub freedom: usize,
}

impl Shares {
    pub fn of<'a>(readings: impl IntoIterator<Item = &'a Reading>) -> Self {
        let mut s = Shares::default();
        for r in readings {
            match r { Reading::Structure => s.structure += 1, Reading::Freedom => s.freedom += 1 }
        }
        s
    }
    pub fn evaluated(&self) -> usize { self.structure + self.freedom }
    /// F_δ. `None` when nothing is evaluated.
    pub fn f_delta(&self) -> Option<f64> {
        (self.evaluated() > 0).then(|| self.freedom as f64 / self.evaluated() as f64)
    }
    /// S_δ = 1 − F_δ over the same set.
    pub fn s_delta(&self) -> Option<f64> {
        (self.evaluated() > 0).then(|| self.structure as f64 / self.evaluated() as f64)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PC1 — Relation
// ═══════════════════════════════════════════════════════════════════════════

/// One relation, one component, one step.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RelationChange {
    pub step: usize,
    pub component: usize,
    pub edge: usize,
    /// Δ_s[c][e]; the degree is |change| / half_width.
    pub change: f64,
    pub half_width: f64,
    pub reading: Reading,
}

/// PC1 over every relation, component and step.
pub fn relation_changes(series: &EdgeSeries) -> Vec<RelationChange> {
    let mut out = Vec::new();
    for s in 0..series.n_steps() {
        let (d, dd) = series.change(s);
        for c in 0..series.k() {
            for e in 0..series.n_edges() {
                out.push(RelationChange {
                    step: s, component: c, edge: e,
                    change: d[c][e], half_width: dd[c][e], reading: read(d[c][e], dd[c][e]),
                });
            }
        }
    }
    out
}

/// V5 restated: steps over which one relation's change is resolved at δ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvolutionCount {
    pub edge: usize,
    pub component: usize,
    /// Γ: declared steps with change resolved at δ.
    pub changed_steps: usize,
    /// Declared steps in the cycle of analysis (registrations − 1).
    pub steps: usize,
}

impl EvolutionCount {
    /// Λ̂ = Γ / steps, in [0, 1]. A declared summary (C).
    pub fn share(&self) -> f64 { self.changed_steps as f64 / self.steps as f64 }
}

// ═══════════════════════════════════════════════════════════════════════════
// PC2 — Coupling of change
// ═══════════════════════════════════════════════════════════════════════════

/// The two consecutive readings a change minor is formed over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadingPair {
    /// (a) Components i < j at step s.
    Components { step: usize, i: usize, j: usize },
    /// (b) One component at steps s and s+1.
    Steps { component: usize, step: usize },
}

/// The change minor μ(e, f) over one reading pair.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChangeCoupling {
    pub e: usize,
    pub f: usize,
    pub over: ReadingPair,
    pub minor: f64,
    pub half_width: f64,
    pub reading: Reading,
}

/// PC2 readings, with the pairs the declaration fixes listed apart.
#[derive(Debug, Clone, PartialEq)]
pub struct Couplings {
    pub readings: Vec<ChangeCoupling>,
    /// Identities, not readings (PC-D4).
    pub forced_pairs: Vec<(usize, usize)>,
}

/// μ over two readings (x, dx) and (y, dy), each indexed by edge. Edges whose
/// change is not resolved in either reading persisted (PC1) and enter no pair.
/// `hx`, `hy` are the bounds used in the minor (widened by independent gains
/// when the minor mixes components); entry uses the change bounds `x.1`, `y.1`.
fn couple(series: &EdgeSeries, over: ReadingPair, x: (&[f64], &[f64]), y: (&[f64], &[f64]),
          hx: &[f64], hy: &[f64], out: &mut Vec<ChangeCoupling>) {
    let moved: Vec<usize> = (0..series.n_edges())
        .filter(|&e| x.0[e].abs() > x.1[e] || y.0[e].abs() > y.1[e]).collect();
    for (n, &e) in moved.iter().enumerate() {
        for &f in &moved[n + 1..] {
            if series.is_forced(e, f) { continue; }
            let minor = x.0[e] * y.0[f] - y.0[e] * x.0[f];
            let hw = product_half_width(x.0[e], hx[e], y.0[f], hy[f])
                + product_half_width(y.0[e], hy[e], x.0[f], hx[f]);
            out.push(ChangeCoupling { e, f, over, minor, half_width: hw, reading: read(minor, hw) });
        }
    }
}

/// PC2(a): change minors across components i < j at each step. Needs k ≥ 2.
pub fn coupling_across_components(series: &EdgeSeries) -> Result<Couplings, DeclarationGap> {
    if series.k() < 2 { return Err(DeclarationGap::SingleComponent); }
    if series.n_edges() < 2 { return Err(DeclarationGap::FewerThanTwoEdges); }
    let mut readings = Vec::new();
    for s in 0..series.n_steps() {
        let (d, dd) = series.change(s);
        let hm = series.mixed(&d, &dd);
        for i in 0..series.k() {
            for j in (i + 1)..series.k() {
                couple(series, ReadingPair::Components { step: s, i, j },
                    (&d[i], &dd[i]), (&d[j], &dd[j]), &hm[i], &hm[j], &mut readings);
            }
        }
    }
    Ok(Couplings { readings, forced_pairs: series.forced_pairs.clone() })
}

/// PC2(b): change minors across consecutive steps s, s+1 in each component.
/// Needs at least three registrations.
pub fn coupling_across_steps(series: &EdgeSeries) -> Result<Couplings, DeclarationGap> {
    if series.n_registrations() < 3 { return Err(DeclarationGap::NoProcessSequence); }
    if series.n_edges() < 2 { return Err(DeclarationGap::FewerThanTwoEdges); }
    let mut readings = Vec::new();
    for s in 0..series.n_steps() - 1 {
        let (d0, dd0) = series.change(s);
        let (d1, dd1) = series.change(s + 1);
        for c in 0..series.k() {
            couple(series, ReadingPair::Steps { component: c, step: s },
                (&d0[c], &dd0[c]), (&d1[c], &dd1[c]), &dd0[c], &dd1[c], &mut readings);
        }
    }
    Ok(Couplings { readings, forced_pairs: series.forced_pairs.clone() })
}

// ═══════════════════════════════════════════════════════════════════════════
// PC3 — Arrangement
// ═══════════════════════════════════════════════════════════════════════════

/// The minor m_ij(e, f) on A, compared at registrations s and s+1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArrangementStep {
    pub step: usize,
    pub e: usize,
    pub f: usize,
    pub i: usize,
    pub j: usize,
    pub before: AtDelta<crate::ground_variables::Sign>,
    pub after: AtDelta<crate::ground_variables::Sign>,
    /// Structure when `before == after`, freedom otherwise.
    pub reading: Reading,
}

/// PC3 over every step. An edge enters a step when its A is resolved at δ in
/// at least one component at both registrations. Needs k ≥ 2.
pub fn arrangement_steps(series: &EdgeSeries) -> Result<Vec<ArrangementStep>, DeclarationGap> {
    if series.k() < 2 { return Err(DeclarationGap::SingleComponent); }
    if series.n_edges() < 2 { return Err(DeclarationGap::FewerThanTwoEdges); }
    let resolved = |t: usize, e: usize| (0..series.k()).any(|c| series.a[t][c][e].abs() > series.da[t][c][e]);
    let widened: Vec<Vec<Vec<f64>>> = (0..series.n_registrations())
        .map(|t| series.mixed(&series.a[t], &series.da[t])).collect();
    let state = |t: usize, e: usize, f: usize, i: usize, j: usize| {
        let (a, da) = (&series.a[t], &widened[t]);
        let m = a[i][e] * a[j][f] - a[j][e] * a[i][f];
        let hw = product_half_width(a[i][e], da[i][e], a[j][f], da[j][f])
            + product_half_width(a[j][e], da[j][e], a[i][f], da[i][f]);
        sign_at_delta(m, hw)
    };
    let mut out = Vec::new();
    for s in 0..series.n_steps() {
        let held: Vec<usize> = (0..series.n_edges()).filter(|&e| resolved(s, e) && resolved(s + 1, e)).collect();
        for (n, &e) in held.iter().enumerate() {
            for &f in &held[n + 1..] {
                if series.is_forced(e, f) { continue; }
                for i in 0..series.k() {
                    for j in (i + 1)..series.k() {
                        let (before, after) = (state(s, e, f, i, j), state(s + 1, e, f, i, j));
                        let reading = if before == after { Reading::Structure } else { Reading::Freedom };
                        out.push(ArrangementStep { step: s, e, f, i, j, before, after, reading });
                    }
                }
            }
        }
    }
    Ok(out)
}

// ═══════════════════════════════════════════════════════════════════════════
// PC4 — Shares at every level
// ═══════════════════════════════════════════════════════════════════════════

/// Structure and freedom at every level that the declaration lets be formed.
/// A level that cannot be formed carries its declaration gap.
#[derive(Debug, Clone, PartialEq)]
pub struct PersistenceChange {
    pub relations: Vec<RelationChange>,
    pub across_components: Result<Couplings, DeclarationGap>,
    pub across_steps: Result<Couplings, DeclarationGap>,
    pub arrangement: Result<Vec<ArrangementStep>, DeclarationGap>,
}

impl PersistenceChange {
    pub fn new(series: &EdgeSeries) -> Self {
        PersistenceChange {
            relations: relation_changes(series),
            across_components: coupling_across_components(series),
            across_steps: coupling_across_steps(series),
            arrangement: arrangement_steps(series),
        }
    }

    /// F_δ of PC1 over every relation, component and step: the summary of
    /// superposition (freedom) for the sequence. A declared summary (C).
    pub fn f_delta(&self) -> Option<f64> {
        self.relation_shares(None).f_delta()
    }

    /// Every gap among the levels that could not be formed, without repeats.
    pub fn gaps(&self) -> Vec<DeclarationGap> {
        let mut out: Vec<DeclarationGap> = Vec::new();
        let errs = [self.across_components.as_ref().err(), self.across_steps.as_ref().err(),
                    self.arrangement.as_ref().err()];
        for g in errs.into_iter().flatten() {
            if !out.contains(g) { out.push(g.clone()); }
        }
        out
    }

    /// Rate of relational evolution (V5, restated 2026-11-21, 10th): for each
    /// relation and component, the number of declared steps over which its
    /// change is resolved at δ, out of all declared steps. The count is the
    /// reading. Time is never primitive: a clock duration of the same steps
    /// may be cited beside it only as a declared intersection (W15).
    pub fn evolution_counts(&self) -> Vec<EvolutionCount> {
        let steps = self.relations.iter().map(|r| r.step + 1).max().unwrap_or(0);
        let mut out: Vec<EvolutionCount> = Vec::new();
        for r in &self.relations {
            let i = match out.iter().position(|x| x.edge == r.edge && x.component == r.component) {
                Some(i) => i,
                None => { out.push(EvolutionCount { edge: r.edge, component: r.component, changed_steps: 0, steps }); out.len() - 1 }
            };
            if r.reading == Reading::Freedom { out[i].changed_steps += 1; }
        }
        out
    }

    /// PC1 shares at step s, or over all steps when `None`.
    pub fn relation_shares(&self, step: Option<usize>) -> Shares {
        Shares::of(self.relations.iter().filter(|r| step.map_or(true, |s| r.step == s)).map(|r| &r.reading))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Connection in change — the coupled-change grade
// ═══════════════════════════════════════════════════════════════════════════

/// Pairs whose change is locked in proportion at every reading pair where the
/// pair is evaluated (PC2, either family), and evaluated at least once: shared
/// structure in change. Pairs fixed by the declaration are excluded (PC-D4).
/// A gap only when neither family can be formed.
pub fn coupled_change_pairs(pc: &PersistenceChange) -> Result<Vec<(usize, usize)>, DeclarationGap> {
    let families: Vec<&Couplings> =
        [&pc.across_components, &pc.across_steps].into_iter().filter_map(|f| f.as_ref().ok()).collect();
    if families.is_empty() {
        return Err(pc.across_components.as_ref().err().cloned().unwrap_or(DeclarationGap::SingleComponent));
    }
    let mut pairs: Vec<(usize, usize)> =
        families.iter().flat_map(|f| f.readings.iter().map(|x| (x.e, x.f))).collect();
    pairs.sort();
    pairs.dedup();
    Ok(pairs.into_iter().filter(|&(e, f)| families.iter()
        .flat_map(|c| c.readings.iter())
        .filter(|x| (x.e, x.f) == (e, f))
        .all(|x| x.reading == Reading::Structure)).collect())
}

// ═══════════════════════════════════════════════════════════════════════════
// Closure around a cycle (K7 / HB-1 on the series; I-SC-D10)
// ═══════════════════════════════════════════════════════════════════════════
//
// A cycle is a closed walk over declared edges, each with a sign: +1 when
// traversed along its declared direction, −1 against it. Traversal is an
// order of analysis (Df2a); no edge is reversed. The closure at registration
// t is A_cyc = Σ s_e·A_t[c][e], half-width Σ δA_t[c][e]; it holds at δ when
// |A_cyc| ≤ its half-width. On a node-derived series it is 0 by identity
// (PC-D4); only edge-registered values make it a test.
//
// I-SC-D10 (Derived): the closure of the change Σ s_e·Δ_s[c][e] equals
// A_cyc(s+1) − A_cyc(s), with half-width Σ δΔ_s[c][e]. If closure holds at δ
// at both registrations, it holds on the change. The converse fails: an
// inconsistency fixed over the step (a calibration offset around the cycle)
// cancels in the change. Persistent chain structure (W9) is read in state;
// change closure reads only what changed.

/// True when `cycle` (edge, sign) is a closed walk over the declared edges.
pub fn is_closed_walk(rel: &DeclaredRelations, cycle: &[(usize, i8)]) -> bool {
    if cycle.is_empty() { return false; }
    let ends = |&(e, sg): &(usize, i8)| {
        let (s, t) = rel.edges[e];
        if sg >= 0 { (s, t) } else { (t, s) }
    };
    if cycle.iter().any(|&(e, sg)| e >= rel.n_edges() || (sg != 1 && sg != -1)) { return false; }
    let start = ends(&cycle[0]).0;
    let mut at = start;
    for step in cycle {
        let (from, to) = ends(step);
        if from != at { return false; }
        at = to;
    }
    at == start
}

/// Closure value and half-width of `cycle` in component c of field (a, da).
fn closure_of(a: &[Vec<f64>], da: &[Vec<f64>], cycle: &[(usize, i8)], c: usize) -> (f64, f64) {
    cycle.iter().fold((0.0, 0.0), |(v, h), &(e, sg)| (v + sg as f64 * a[c][e], h + da[c][e]))
}

/// Closure at registration t. `None` when `cycle` is not a closed walk.
pub fn cycle_closure(series: &EdgeSeries, rel: &DeclaredRelations, cycle: &[(usize, i8)], t: usize, c: usize)
    -> Option<(f64, f64)>
{
    is_closed_walk(rel, cycle).then(|| closure_of(&series.a[t], &series.da[t], cycle, c))
}

/// Closure of the change over step s. `None` when `cycle` is not a closed walk.
pub fn cycle_change_closure(series: &EdgeSeries, rel: &DeclaredRelations, cycle: &[(usize, i8)], s: usize, c: usize)
    -> Option<(f64, f64)>
{
    let (d, dd) = series.change(s);
    is_closed_walk(rel, cycle).then(|| closure_of(&d, &dd, cycle, c))
}

// ═══════════════════════════════════════════════════════════════════════════
// Step profile (2026-11-21): the readings along the declared order
// ═══════════════════════════════════════════════════════════════════════════
//
// For each step of the declared sequence, the structure and freedom counts at
// every level formed at that step. Read along an energy order, this is where
// phase changes are expected to show (persistence on a plateau,
// reorganization at its ends). A declared summary (C), not the object; it
// adds no reading. PC2(b) readings pair steps s and s+1 and are counted at s.

/// Structure and freedom at one step.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StepProfile {
    pub step: usize,
    /// PC1: relations and components over this step.
    pub relations: Shares,
    /// PC2(a): change coupling across components at this step.
    pub across_components: Option<Shares>,
    /// PC2(b): change coupling across steps (this step, the next).
    pub across_steps: Option<Shares>,
    /// PC3: arrangement held or changed over this step.
    pub arrangement: Option<Shares>,
}

impl PersistenceChange {
    /// The step profile. A level that cannot be formed is `None` at every step
    /// (its gap is in `gaps()`).
    pub fn step_profile(&self) -> Vec<StepProfile> {
        let steps = self.relations.iter().map(|r| r.step + 1).max().unwrap_or(0);
        let at = |c: &Couplings, s: usize| Shares::of(c.readings.iter().filter(|x| match x.over {
            ReadingPair::Components { step, .. } | ReadingPair::Steps { step, .. } => step == s,
        }).map(|x| &x.reading));
        (0..steps).map(|s| StepProfile {
            step: s,
            relations: self.relation_shares(Some(s)),
            across_components: self.across_components.as_ref().ok().map(|c| at(c, s)),
            across_steps: self.across_steps.as_ref().ok().map(|c| at(c, s)),
            arrangement: self.arrangement.as_ref().ok()
                .map(|a| Shares::of(a.iter().filter(|x| x.step == s).map(|x| &x.reading))),
        }).collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Relation transitions (Origin, 2026-11-21)
// ═══════════════════════════════════════════════════════════════════════════
//
// "The readings locate a transition when the degree of freedom becomes
// evident, or the persistent structure conversely." Per relation and
// component, along the declared order:
//   - freedom becomes evident: the PC1 reading turns to freedom (change
//     resolved at δ) after a held run of structure;
//   - persistent structure becomes evident: the reading turns to structure
//     after a held run of freedom.
// A state is held when it lasts at least `min_hold` consecutive steps (a
// declared parameter: a one-step fluctuation is not persistence). Each new
// held state is compared with the last held state, so a fluctuation that
// returns to the earlier state is not a transition. No threshold on any
// share is involved: the reading is per relation, at its own δ.

/// One relation's reading turns and holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelationTransition {
    pub edge: usize,
    pub component: usize,
    /// The first step of the new held state.
    pub step: usize,
    /// Freedom: freedom becomes evident. Structure: persistent structure
    /// becomes evident.
    pub to: Reading,
    /// How many steps the new state holds.
    pub held_for: usize,
}

impl PersistenceChange {
    /// Every relation transition, ordered by step.
    pub fn relation_transitions(&self, min_hold: usize) -> Vec<RelationTransition> {
        assert!(min_hold >= 1, "a held state lasts at least one step");
        let mut keys: Vec<(usize, usize)> = self.relations.iter().map(|r| (r.edge, r.component)).collect();
        keys.sort();
        keys.dedup();
        let mut out = Vec::new();
        for (edge, component) in keys {
            let mut seq: Vec<&RelationChange> =
                self.relations.iter().filter(|r| r.edge == edge && r.component == component).collect();
            seq.sort_by_key(|r| r.step);
            // Runs of one reading: (first step, reading, length).
            let mut runs: Vec<(usize, Reading, usize)> = Vec::new();
            for r in seq {
                match runs.last_mut() {
                    Some(last) if last.1 == r.reading => last.2 += 1,
                    _ => runs.push((r.step, r.reading, 1)),
                }
            }
            let mut held: Option<Reading> = None;
            for (step, reading, len) in runs {
                if len < min_hold { continue; }
                if let Some(h) = held {
                    if h != reading { out.push(RelationTransition { edge, component, step, to: reading, held_for: len }); }
                }
                held = Some(reading);
            }
        }
        out.sort_by_key(|t| (t.step, t.edge, t.component));
        out
    }
}
