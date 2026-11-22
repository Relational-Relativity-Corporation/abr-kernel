// ground_variables.rs — Metatron Dynamics, Inc. Kernel V10.
// The three ground variables, read over relational evolution at declared δ:
//   energy     ↔ tunneling     (I-T)   — V3
//   freedom    ↔ superposition (I-S)   — V2
//   connection ↔ entanglement  (I-E)   — V1
// Bounded over D. No claim beyond D.
//
// ── Three distinct variables, one relational object (Origin, 2026-11-21) ──
//
//   Energy      what is exchanged across a relation. ε = |A(c)| of the declared carrier c.
//   Freedom     change permitted: relations changing, and changing independently
//               (persistence_change.rs, PC1–PC3; Declared 2026-11-21, 7th).
//   Connection  shared structure: relations held together by a constraint, in
//               their state (identity, proportional, sustained grades) or in
//               their change (coupled-change grade, PC2).
//
// Amendment (Origin, 2026-11-21, 7th): structure is persistence; change is
// freedom. The first V2 form below (`freedom`, a minor held resolved) reads a
// persistent arrangement, which is structure; it is kept unchanged for
// declared comparison (C). The first wording of energy, "how much a relation
// changes", is replaced: ε is a difference across a relation, and its change
// over evolution is read through PC with the carrier as a component.
//
// None reduces to another. They are read from the same declared relations and
// are related as follows:
//   - per minor at δ, freedom and connection are complementary (resolved or
//     not resolved);
//   - energy is what moves minors toward or across zero; a crossing is where
//     energy exchange becomes visible in the structure;
//   - all three are held over relational evolution.
//
// ── Readings are formed over a declared sequence (Origin, 2026-11-21) ─────
//
// A frozen frame is not a relational system (O1, Df3). Every reading in this
// module is formed over a declared sequence of at least two consecutive
// registrations of the same declared relations, ordered by an observable
// property of the process (T6). A single registration is a declaration
// finding: NoProcessSequence. The per-registration readings below are
// building blocks only.
//
// ── Observable fact and calculated variable (Origin, 2026-11-21) ──────────
//
// Energy, connection and freedom are present in every registration (G1, G4).
// Nothing here tests that. When a declared topology does not let a variable
// (or a grade of connection) be calculated, the result is a finding about the
// declaration, never about the system:
//   - PRIMARY REGION: no interior locus is declared (T7). Only the sustained
//     grade of connection needs an interior; energy, freedom and the identity
//     and proportional grades of connection are readable at every region; or
//   - DOMAIN NOT FULLY DECLARED: a carrier, a component, a relation, a
//     registration, a process sequence or a finer δ is missing (R9a).
// The remedy is to return to the declaration (G4a, W5). A declared edge is
// never reversed to obtain a reading (W8); no substitute value is introduced.
// A calculated degree of zero is returned unchanged and read under W2.
//
// ── Operators ─────────────────────────────────────────────────────────────
//
// Every reading runs through the operators A (`operator_a`), B (`operator_b`)
// and the parameter-free circulation Ξ (`operator_circulation`). ρ and χ are
// retired from V10 readings (Origin, 2026-11-21). No operator formula changes in V10. Resolution is carried as
// interval half-widths: A and B are linear with coefficients ±1, so
// δ_A[e] = δ[s] + δ[t] and δ_B = B(δ_A).
//
// The frozen V9 readers in `derived_invariants.rs` are unchanged. The forms
// here supersede them for V10 readings.

use crate::persistence_change::{coupled_change_pairs, EdgeSeries, PersistenceChange};
use crate::operators::{
    circulation_half_width, operator_a, operator_b, operator_circulation, DeclaredRelations,
    EdgeField, NodeField,
};

// ═══════════════════════════════════════════════════════════════════════════
// Resolution
// ═══════════════════════════════════════════════════════════════════════════

/// Sign of a calculated value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

/// A calculated value read at declared resolution δ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtDelta<T> {
    /// The value's interval excludes zero at δ.
    Resolved(T),
    /// The interval contains zero: not observed under the declared relations
    /// at δ (W2). Not a statement of absence.
    NotResolved,
}

impl<T> AtDelta<T> {
    pub fn is_resolved(&self) -> bool {
        matches!(self, AtDelta::Resolved(_))
    }
}

/// Sign of `value` resolved at `half_width`: resolved when |value| > half_width.
pub fn sign_at_delta(value: f64, half_width: f64) -> AtDelta<Sign> {
    assert!(half_width.is_finite() && half_width >= 0.0,
        "δ must be declared, finite and non-negative");
    if value.abs() > half_width {
        AtDelta::Resolved(if value > 0.0 { Sign::Positive } else { Sign::Negative })
    } else {
        AtDelta::NotResolved
    }
}

/// Half-width of x·y over [x ± dx] and [y ± dy]: |x|·dy + |y|·dx + dx·dy.
pub fn product_half_width(x: f64, dx: f64, y: f64, dy: f64) -> f64 {
    x.abs() * dy + y.abs() * dx + dx * dy
}

// ═══════════════════════════════════════════════════════════════════════════
// Declaration gaps and findings
// ═══════════════════════════════════════════════════════════════════════════

/// The three ground variables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroundVariable {
    /// What is exchanged across a relation. Tunneling, I-T.
    Energy,
    /// Change permitted (PC1–PC3). Superposition, I-S.
    Freedom,
    /// Shared structure held by a constraint, in state or in change.
    /// Entanglement, I-E.
    Connection,
}

/// Grades of connection, strongest first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionGrade {
    /// The same relation at different loci: A[e] − A[f] not resolved,
    /// held through evolution.
    Identity,
    /// Relations that vary together: every minor m_ij(e,f) not resolved,
    /// held through evolution.
    Proportional,
    /// Persistent circulation: the sign of Ξ(B(A(x)))[e] resolved and the
    /// same through evolution. Needs an interior locus.
    Sustained,
    /// Relations whose change is locked in proportion wherever it is read
    /// (PC2). Declared 2026-11-21 (7th).
    CoupledChange,
}

/// Why a reading cannot be calculated over the declaration.
/// Each gap is a statement about the declaration, not about the system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclarationGap {
    /// Insufficient information: fewer than two consecutive registrations are
    /// declared. A single registration is not a relational field; a frozen
    /// frame is not a relational system (O1, Df3, Df6).
    NoProcessSequence,
    /// No interior locus is declared (T7). Applies to the sustained grade only.
    PrimaryRegion,
    /// Only one component is declared (T5). Freedom and the proportional
    /// grade are read across components and need at least two.
    SingleComponent,
    /// Fewer than two declared edges.
    FewerThanTwoEdges,
    /// Fewer than two edges are resolved at δ in every registration.
    NoResolvedPair,
    /// No edge has a successor or arriving relation in the field (IV-3);
    /// sustained connection is not formable at any edge.
    NoNeighbourInField,
    /// No energy carrier is declared in M for this domain (R9a, V3a).
    NoCarrierDeclared,
}

impl DeclarationGap {
    /// Which of the two findings this gap is.
    pub fn finding(&self) -> FindingKind {
        match self {
            DeclarationGap::PrimaryRegion => FindingKind::PrimaryRegion,
            _ => FindingKind::DomainNotFullyDeclared,
        }
    }

    /// What to return to in the declaration.
    pub fn remedy(&self) -> &'static str {
        match self {
            DeclarationGap::NoProcessSequence =>
                "insufficient information: declare a sequence of at least two consecutive \
                 registrations of the same relations, ordered by an observable property of \
                 the process (T6, Df6)",
            DeclarationGap::PrimaryRegion =>
                "no interior locus declared: read sustained connection at finer δ, \
                 or read the identity and proportional grades, which need no interior",
            DeclarationGap::SingleComponent =>
                "declare a second component over the same loci (T5)",
            DeclarationGap::FewerThanTwoEdges =>
                "declare the registered relations of the domain",
            DeclarationGap::NoResolvedPair =>
                "declare a finer δ or add registrations so at least two edges are \
                 resolved throughout the sequence",
            DeclarationGap::NoNeighbourInField =>
                "widen the cycle of analysis or declare the registered relations \
                 continuing from or arriving at these edges; never reverse an edge",
            DeclarationGap::NoCarrierDeclared =>
                "declare the energy carrier, its unit, mapping and δ in M (R9a)",
        }
    }
}

/// The kinds of finding a non-calculable or zero reading yields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingKind {
    /// No interior locus declared: Primary Region.
    PrimaryRegion,
    /// The domain declaration is incomplete.
    DomainNotFullyDeclared,
    /// Calculated, but the degree is zero at δ (W2). Return to the
    /// declaration (W5).
    NotResolvedAtDelta,
}

/// A finding about the declaration for one ground variable or grade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub variable: GroundVariable,
    /// The connection grade, for findings about connection.
    pub grade: Option<ConnectionGrade>,
    pub kind: FindingKind,
    /// The gap, when the reading could not be calculated.
    pub gap: Option<DeclarationGap>,
}

// ═══════════════════════════════════════════════════════════════════════════
// The declared sequence
// ═══════════════════════════════════════════════════════════════════════════

/// One registration: the declared node field and its δ per component and node.
///
/// **δ in two parts (Z1a, Declared 2026-11-22; corrected and confirmed by
/// Origin, Part 9, 23rd).** `delta` is the *repeatability* bound: how far a
/// repeated registration of the same state can differ. It enters every form.
/// The *run-constant* parts are what stays fixed over one run of one
/// observing chain, declared beside it and the same in every registration of
/// the run (`with_run_constant`):
///
/// - `offset[c][i]`: a bound on an additive offset at locus i. It cancels in
///   every change (x′ + o − (x + o) = x′ − x), so it enters the **state
///   forms only** (A, the minors on A, identity, Ξ). An offset common to every
///   locus of a component also cancels in A and need not be declared.
/// - `locus_gain[c][i]`: a bound γ_i on the part of the gain at locus i that
///   is **not shared** with the other loci of the component. It enters the
///   state bound as γ_i·|x_i| and the change bound as γ_s·|Δx_s| + γ_t·|Δx_t|
///   for a relation (s, t).
/// - `shared_gain[c]`: a bound γ on the gain common to every locus of the
///   component. A shared gain scales each relation and each change by the same
///   factor 1 + γ′, |γ′| ≤ γ, so it leaves **every classification unchanged**
///   and enters only reported change magnitudes, as γ·|Δ|
///   (`EdgeSeries::change_magnitude`).
///
/// **The gain-sign condition (the Verifier's qualifier, made explicit in
/// 10.3.0).** The statements above hold because a declared gain is a factor
/// 1 + γ′ with 0 ≤ γ < 1: strictly positive, and the repeatability bound is
/// declared in the registered unit, so it scales with the value. A gain bound
/// γ ≥ 1 would admit a factor ≤ 0 and is refused. A *negative* scale (a
/// reversed polarity, a reversed sense of a chain) is not a gain: it is a
/// declared mapping (W14). Under it every resolution at δ, every held state and
/// every crossing is unchanged, and the reported first-order signs (A, Δ, Ξ)
/// reverse; a minor's sign reverses with the product of the two components'
/// scale signs (property test `pc_d3_negative_scale_reverses_signs_only`).
///
/// `Registration::new` declares the repeatability alone, with every
/// run-constant part zero, so a record declared before 10.3.0 reads as before.
#[derive(Debug, Clone)]
pub struct Registration {
    pub x: NodeField,
    /// Repeatability δ[c][i], same shape as `x.data`.
    pub delta: Vec<Vec<f64>>,
    /// Run-constant offset bound [c][i] (state forms only).
    pub offset: Vec<Vec<f64>>,
    /// Run-constant gain bound not shared between loci [c][i], 0 ≤ γ < 1.
    pub locus_gain: Vec<Vec<f64>>,
    /// Run-constant gain bound shared by every locus of a component [c], 0 ≤ γ < 1.
    pub shared_gain: Vec<f64>,
}

impl Registration {
    pub fn new(x: NodeField, delta: Vec<Vec<f64>>) -> Self {
        assert_eq!(delta.len(), x.k, "δ must be declared for every component");
        assert!(delta.iter().all(|d| d.len() == x.n), "δ must be declared for every node");
        assert!(delta.iter().flatten().all(|d| d.is_finite() && *d >= 0.0),
            "δ must be finite and non-negative");
        let zero = vec![vec![0.0; x.n]; x.k];
        let k = x.k;
        Registration { x, delta, offset: zero.clone(), locus_gain: zero, shared_gain: vec![0.0; k] }
    }

    /// Declare the run-constant parts of δ (Z1a). Same shapes as `delta`;
    /// `shared_gain` per component. Offsets finite and non-negative; gains in
    /// [0, 1) (the gain-sign condition).
    pub fn with_run_constant(mut self, offset: Vec<Vec<f64>>, locus_gain: Vec<Vec<f64>>, shared_gain: Vec<f64>) -> Self {
        let (k, n) = (self.x.k, self.x.n);
        assert!(offset.len() == k && offset.iter().all(|o| o.len() == n),
            "an offset must be declared for every component and node");
        assert!(locus_gain.len() == k && locus_gain.iter().all(|g| g.len() == n),
            "a locus gain must be declared for every component and node");
        assert_eq!(shared_gain.len(), k, "a shared gain must be declared for every component");
        assert!(offset.iter().flatten().all(|o| o.is_finite() && *o >= 0.0),
            "an offset bound must be finite and non-negative");
        assert!(locus_gain.iter().flatten().chain(&shared_gain).all(|g| g.is_finite() && *g >= 0.0 && *g < 1.0),
            "a gain bound must lie in [0, 1): a declared gain is strictly positive (Z1a)");
        self.offset = offset;
        self.locus_gain = locus_gain;
        self.shared_gain = shared_gain;
        self
    }

    /// The state bound at locus i of component c: repeatability, offset and
    /// the unshared gain on the registered value (Z1a). The shared gain does
    /// not enter: it changes no classification.
    pub fn state_delta(&self, c: usize, i: usize) -> f64 {
        self.delta[c][i] + self.offset[c][i] + self.locus_gain[c][i] * self.x.data[c][i].abs()
    }

    /// True when the run-constant parts are the same as `other`'s.
    pub fn same_run_constant(&self, other: &Registration) -> bool {
        self.offset == other.offset && self.locus_gain == other.locus_gain && self.shared_gain == other.shared_gain
    }
}

fn check_sequence(seq: &[Registration], rel: &DeclaredRelations) -> Result<(), DeclarationGap> {
    if seq.len() < 2 {
        return Err(DeclarationGap::NoProcessSequence);
    }
    let k = seq[0].x.k;
    assert!(seq.iter().all(|r| r.x.k == k),
        "every registration must declare the same components");
    assert!(seq.iter().all(|r| r.x.n == rel.n_nodes),
        "every registration must cover the declared loci");
    assert!(seq.iter().all(|r| r.same_run_constant(&seq[0])),
        "the run-constant parts of δ must be the same in every registration of the run (Z1a)");
    Ok(())
}

/// A(x) and δ_A for one registration. δ_A is the **state** bound (Z1a):
/// repeatability, offset and unshared gain at both ends.
fn a_with_half_width(r: &Registration, rel: &DeclaredRelations) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let a = operator_a(&r.x, rel, &[]).spatial;
    let da = (0..r.x.k)
        .map(|c| rel.edges.iter().map(|&(s, t)| r.state_delta(c, s) + r.state_delta(c, t)).collect())
        .collect();
    (a, da)
}

/// True when at least one declared locus is the target of one edge and the
/// source of another (T7).
pub fn has_interior(rel: &DeclaredRelations) -> bool {
    (0..rel.n_edges()).any(|e| !rel.succ(e).is_empty())
}

/// True when edge e's A vector is resolved at δ in at least one component.
pub fn edge_resolved(a: &[Vec<f64>], da: &[Vec<f64>], e: usize) -> bool {
    (0..a.len()).any(|c| a[c][e].abs() > da[c][e])
}

/// How a reading behaves over the declared sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum History {
    /// Not resolved at every registration.
    HeldUnresolved,
    /// Resolved with the same sign at every registration.
    HeldResolved(Sign),
    /// Did not hold one state: the structure reorganized. `crossed` is true
    /// when two registrations resolve opposite signs.
    Reorganizing { crossed: bool },
}

/// Classify a sequence of readings at δ.
pub fn history(readings: &[AtDelta<Sign>]) -> History {
    let first = readings[0];
    if readings.iter().all(|r| *r == first) {
        return match first {
            AtDelta::NotResolved => History::HeldUnresolved,
            AtDelta::Resolved(s) => History::HeldResolved(s),
        };
    }
    let pos = readings.contains(&AtDelta::Resolved(Sign::Positive));
    let neg = readings.contains(&AtDelta::Resolved(Sign::Negative));
    History::Reorganizing { crossed: pos && neg }
}

// ═══════════════════════════════════════════════════════════════════════════
// The minor pattern P(F)
// ═══════════════════════════════════════════════════════════════════════════
//
// For edges e, f and components i < j:
//
//     m_ij(e, f) = A[i][e]·A[j][f] − A[j][e]·A[i][f]
//
// A[e] and A[f] are proportional exactly when every m_ij is zero. Properties
// by construction: pairwise only (no rank); swapping e and f negates every
// minor together; positive rescaling of a component preserves every sign and
// any non-zero rescaling preserves which minors are zero; offsets in x cancel
// in A. The pattern depends on the components declared through M (R9a).

/// The minor m_ij(e, f) over an edge field a[c][e].
pub fn minor(a: &[Vec<f64>], e: usize, f: usize, i: usize, j: usize) -> f64 {
    a[i][e] * a[j][f] - a[j][e] * a[i][f]
}

/// Half-width of m_ij(e, f) from the half-widths da[c][e].
pub fn minor_half_width(a: &[Vec<f64>], da: &[Vec<f64>], e: usize, f: usize, i: usize, j: usize) -> f64 {
    product_half_width(a[i][e], da[i][e], a[j][f], da[j][f])
        + product_half_width(a[j][e], da[j][e], a[i][f], da[i][f])
}

/// Bound on the change of m_ij(e, f) over one step, from the prior edge field
/// `a` and the persistence difference `a_p` = A_current − A_prior (Derived):
/// |Δm| ≤ |a_ie·Δa_jf| + |Δa_ie·a_jf| + |Δa_ie·Δa_jf|
///      + |a_je·Δa_if| + |Δa_je·a_if| + |Δa_je·Δa_if|.
pub fn minor_change_bound(a: &[Vec<f64>], a_p: &[Vec<f64>], e: usize, f: usize, i: usize, j: usize) -> f64 {
    let t = |x: f64, dx: f64, y: f64, dy: f64| (x * dy).abs() + (dx * y).abs() + (dx * dy).abs();
    t(a[i][e], a_p[i][e], a[j][f], a_p[j][f]) + t(a[j][e], a_p[j][e], a[i][f], a_p[i][f])
}

/// Persistence condition (Derived): a minor cannot change sign over a step
/// whose change bound is smaller than its prior magnitude, because a sign
/// changes only by passing through zero.
pub fn step_preserves_sign(prior_minor: f64, change_bound: f64) -> bool {
    change_bound < prior_minor.abs()
}

/// Every minor of one registration, at δ.
#[derive(Debug, Clone, PartialEq)]
pub struct MinorPattern {
    /// (e, f, i, j, reading) for e < f over edges resolved at δ, i < j.
    pub minors: Vec<(usize, usize, usize, usize, AtDelta<Sign>)>,
    /// Edges not resolved at δ in any component (enter no pair).
    pub unresolved_edges: Vec<usize>,
}

impl MinorPattern {
    /// κ_δ = resolved minors / evaluated minors: a declared summary (C) of the
    /// pattern, toward 0 at the connection-dominant end and toward 1 at the
    /// freedom-dominant end. `None` when no minor is evaluated.
    pub fn kappa(&self) -> Option<f64> {
        if self.minors.is_empty() { return None; }
        let resolved = self.minors.iter().filter(|m| m.4.is_resolved()).count();
        Some(resolved as f64 / self.minors.len() as f64)
    }
}

/// The minor pattern P(F) of one registration. A building block: a single
/// registration is not a relational reading.
pub fn minor_pattern(r: &Registration, rel: &DeclaredRelations) -> Result<MinorPattern, DeclarationGap> {
    if r.x.k < 2 { return Err(DeclarationGap::SingleComponent); }
    if rel.n_edges() < 2 { return Err(DeclarationGap::FewerThanTwoEdges); }
    let (a, da) = a_with_half_width(r, rel);
    let (resolved, unresolved): (Vec<usize>, Vec<usize>) =
        (0..rel.n_edges()).partition(|&e| edge_resolved(&a, &da, e));
    let mut minors = Vec::new();
    for (n, &e) in resolved.iter().enumerate() {
        for &f in &resolved[n + 1..] {
            for i in 0..r.x.k {
                for j in (i + 1)..r.x.k {
                    let m = minor(&a, e, f, i, j);
                    minors.push((e, f, i, j, sign_at_delta(m, minor_half_width(&a, &da, e, f, i, j))));
                }
            }
        }
    }
    Ok(MinorPattern { minors, unresolved_edges: unresolved })
}

// ═══════════════════════════════════════════════════════════════════════════
// Minor histories over the sequence
// ═══════════════════════════════════════════════════════════════════════════

/// The history of every minor over the declared sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct MinorHistories {
    /// (e, f, i, j, history) over pairs of edges resolved at every registration.
    pub minors: Vec<(usize, usize, usize, usize, History)>,
    /// Edges not resolved at δ at one or more registrations.
    pub unresolved_edges: Vec<usize>,
    /// κ_δ at each registration (declared summary, C).
    pub kappa: Vec<Option<f64>>,
}

/// Minor histories over a declared sequence of at least two registrations.
pub fn minor_histories(seq: &[Registration], rel: &DeclaredRelations) -> Result<MinorHistories, DeclarationGap> {
    check_sequence(seq, rel)?;
    let patterns: Vec<MinorPattern> =
        seq.iter().map(|r| minor_pattern(r, rel)).collect::<Result<_, _>>()?;
    let fields: Vec<_> = seq.iter().map(|r| a_with_half_width(r, rel)).collect();
    let unresolved: Vec<usize> = (0..rel.n_edges())
        .filter(|&e| fields.iter().any(|(a, da)| !edge_resolved(a, da, e)))
        .collect();
    let held: Vec<usize> = (0..rel.n_edges()).filter(|e| !unresolved.contains(e)).collect();
    if held.len() < 2 { return Err(DeclarationGap::NoResolvedPair); }
    let k = seq[0].x.k;
    let mut minors = Vec::new();
    for (n, &e) in held.iter().enumerate() {
        for &f in &held[n + 1..] {
            for i in 0..k {
                for j in (i + 1)..k {
                    let readings: Vec<AtDelta<Sign>> = fields.iter().map(|(a, da)| {
                        sign_at_delta(minor(a, e, f, i, j), minor_half_width(a, da, e, f, i, j))
                    }).collect();
                    minors.push((e, f, i, j, history(&readings)));
                }
            }
        }
    }
    Ok(MinorHistories {
        minors,
        unresolved_edges: unresolved,
        kappa: patterns.iter().map(|p| p.kappa()).collect(),
    })
}

/// How one edge pair behaves over the sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairHistory {
    /// At least one minor held resolved: independent variation persists.
    Free,
    /// Every minor held unresolved: the relations vary together throughout.
    Proportional,
    /// Neither: the pair's structure reorganized over the sequence.
    Reorganizing,
}

fn pair_histories(h: &MinorHistories) -> Vec<((usize, usize), PairHistory)> {
    let mut pairs: Vec<(usize, usize)> = h.minors.iter().map(|m| (m.0, m.1)).collect();
    pairs.dedup();
    pairs.into_iter().map(|(e, f)| {
        let ms: Vec<History> = h.minors.iter()
            .filter(|m| m.0 == e && m.1 == f).map(|m| m.4).collect();
        let ph = if ms.iter().any(|m| matches!(m, History::HeldResolved(_))) {
            PairHistory::Free
        } else if ms.iter().all(|m| *m == History::HeldUnresolved) {
            PairHistory::Proportional
        } else {
            PairHistory::Reorganizing
        };
        ((e, f), ph)
    }).collect()
}

// ═══════════════════════════════════════════════════════════════════════════
// V2, FIRST FORM — superseded for freedom (Origin, 2026-11-21, 7th)
// ═══════════════════════════════════════════════════════════════════════════
//
// A pair of relations is free over the sequence when at least one of its
// minors is resolved with the same sign at every registration: independent
// variation persists. F_δ = free pairs / evaluated pairs.
//
// Amendment: under structure = persistence, a held state is structure. This
// form therefore reads a persistent independent arrangement, not freedom.
// Kept unchanged, carried in GroundReading as `v2_held_arrangement` for
// declared comparison (C). V10 freedom: persistence_change.rs (PC1–PC3).

/// Freedom over a declared sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct Freedom {
    pub free_pairs: Vec<(usize, usize)>,
    pub evaluated_pairs: usize,
    pub unresolved_edges: Vec<usize>,
    /// F_δ = free / evaluated.
    pub f_delta: f64,
}

pub fn freedom(seq: &[Registration], rel: &DeclaredRelations) -> Result<Freedom, DeclarationGap> {
    let h = minor_histories(seq, rel)?;
    let pairs = pair_histories(&h);
    let free_pairs: Vec<(usize, usize)> =
        pairs.iter().filter(|p| p.1 == PairHistory::Free).map(|p| p.0).collect();
    Ok(Freedom {
        f_delta: free_pairs.len() as f64 / pairs.len() as f64,
        evaluated_pairs: pairs.len(),
        free_pairs,
        unresolved_edges: h.unresolved_edges,
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// CONNECTION — entanglement (V1)
// ═══════════════════════════════════════════════════════════════════════════
//
// Connection is shared relational structure persisting through relational
// evolution. It is graded, strongest first:
//
//   IDENTITY      A[e] − A[f] not resolved in any component at every
//                 registration: the same relation at different loci.
//                 Formable with one component; needs no interior.
//   PROPORTIONAL  every minor m_ij(e, f) not resolved at every registration:
//                 the relations vary together. Needs two components; needs no
//                 interior.
//   SUSTAINED     sign of the circulation Ξ(B(A(x)))[e] = Σ_succ B − Σ_pred B
//                 resolved and the same at every registration: persistent
//                 circulation. Parameter-free (`operator_circulation`); ρ and χ
//                 are retired from V10 (2026-11-21). Needs an interior locus;
//                 an edge with no neighbour is not formable (IV-3).
//
// The grades are read beside each other and not merged into one number.
// Shared structure has observational content only between independently
// registered relations: a zero forced by the declaration is an identity, not
// a reading (Method; W10).

/// Identity grade: pairs whose difference is not resolved throughout.
fn identity_pairs(seq: &[Registration], rel: &DeclaredRelations, held: &[usize]) -> Vec<(usize, usize)> {
    let fields: Vec<_> = seq.iter().map(|r| a_with_half_width(r, rel)).collect();
    let mut out = Vec::new();
    for (n, &e) in held.iter().enumerate() {
        for &f in &held[n + 1..] {
            let same = fields.iter().all(|(a, da)| {
                (0..a.len()).all(|c| (a[c][e] - a[c][f]).abs() <= da[c][e] + da[c][f])
            });
            if same { out.push((e, f)); }
        }
    }
    out
}

/// Sustained grade at one registration: sign of Ξ(B(A(x)))[e] at δ per
/// component, through the parameter-free circulation operator. `None` where
/// the edge has no neighbour in the field (IV-3).
fn circulation_signs(r: &Registration, rel: &DeclaredRelations) -> Vec<Option<Vec<AtDelta<Sign>>>> {
    let (a, da) = a_with_half_width(r, rel);
    let k = r.x.k;
    let b = operator_b(&EdgeField { spatial: a, comp: vec![], comp_pairs: vec![], k }, rel).spatial;
    let db = operator_b(&EdgeField { spatial: da, comp: vec![], comp_pairs: vec![], k }, rel).spatial;
    let c = operator_circulation(&b, rel);
    let dc = circulation_half_width(&db, rel);
    (0..rel.n_edges()).map(|e| {
        if rel.succ(e).is_empty() && rel.pred(e).is_empty() { return None; }
        Some((0..k).map(|ci| sign_at_delta(c[ci][e], dc[ci][e])).collect())
    }).collect()
}

/// Sustained connection over the sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct Sustained {
    /// Edges whose circulation sign is held resolved in at least one component.
    pub held_edges: Vec<usize>,
    /// Edges whose circulation reorganized.
    pub reorganizing_edges: Vec<usize>,
    /// Edges with no neighbour in the field (IV-3).
    pub not_formable: Vec<usize>,
    /// Formable edges with exactly one empty side, successor or predecessor
    /// (A-3): their circulation is record-boundary-conditioned. They are read
    /// and listed in `held_edges` or `reorganizing_edges` as usual, and
    /// flagged here; a Ξ at one of them is never read as a circulation of
    /// zero, since the missing side is not registered.
    pub boundary_conditioned: Vec<usize>,
    /// held / formable edges.
    pub share_held: f64,
}

fn sustained(seq: &[Registration], rel: &DeclaredRelations) -> Result<Sustained, DeclarationGap> {
    if !has_interior(rel) { return Err(DeclarationGap::PrimaryRegion); }
    let signs: Vec<_> = seq.iter().map(|r| circulation_signs(r, rel)).collect();
    let k = seq[0].x.k;
    let (mut held, mut reorg, mut nf) = (Vec::new(), Vec::new(), Vec::new());
    for e in 0..rel.n_edges() {
        if signs[0][e].is_none() { nf.push(e); continue; }
        let hs: Vec<History> = (0..k).map(|c| {
            let rs: Vec<AtDelta<Sign>> = signs.iter().map(|s| s[e].as_ref().unwrap()[c]).collect();
            history(&rs)
        }).collect();
        if hs.iter().any(|h| matches!(h, History::HeldResolved(_))) {
            held.push(e);
        } else if hs.iter().any(|h| matches!(h, History::Reorganizing { .. })) {
            reorg.push(e);
        }
    }
    let formable = rel.n_edges() - nf.len();
    if formable == 0 { return Err(DeclarationGap::NoNeighbourInField); }
    Ok(Sustained {
        share_held: held.len() as f64 / formable as f64,
        held_edges: held,
        reorganizing_edges: reorg,
        not_formable: nf,
        boundary_conditioned: (0..rel.n_edges())
            .filter(|&e| rel.succ(e).is_empty() != rel.pred(e).is_empty()).collect(),
    })
}

/// Connection over a declared sequence, by grade.
#[derive(Debug, Clone, PartialEq)]
pub struct Connection {
    /// Identity grade: pairs of relations that are the same relation at
    /// different loci throughout.
    pub identity: Vec<(usize, usize)>,
    /// Proportional grade: pairs whose minors are all held unresolved.
    pub proportional: Result<Vec<(usize, usize)>, DeclarationGap>,
    /// Sustained grade: persistent circulation.
    pub sustained: Result<Sustained, DeclarationGap>,
    /// Coupled-change grade: pairs whose change is locked in proportion (PC2).
    pub coupled_change: Result<Vec<(usize, usize)>, DeclarationGap>,
    /// Pairs whose shared structure reorganized (from the minor histories).
    pub reorganizing_pairs: Vec<(usize, usize)>,
    /// Number of pairs of edges resolved at every registration.
    pub evaluated_pairs: usize,
    pub unresolved_edges: Vec<usize>,
}

pub fn connection(seq: &[Registration], rel: &DeclaredRelations) -> Result<Connection, DeclarationGap> {
    check_sequence(seq, rel)?;
    if rel.n_edges() < 2 { return Err(DeclarationGap::FewerThanTwoEdges); }
    let fields: Vec<_> = seq.iter().map(|r| a_with_half_width(r, rel)).collect();
    let unresolved: Vec<usize> = (0..rel.n_edges())
        .filter(|&e| fields.iter().any(|(a, da)| !edge_resolved(a, da, e)))
        .collect();
    let held: Vec<usize> = (0..rel.n_edges()).filter(|e| !unresolved.contains(e)).collect();
    let n = held.len();
    let identity = identity_pairs(seq, rel, &held);
    let (proportional, reorganizing_pairs) = match minor_histories(seq, rel) {
        Ok(h) => {
            let ps = pair_histories(&h);
            (Ok(ps.iter().filter(|p| p.1 == PairHistory::Proportional).map(|p| p.0).collect()),
             ps.iter().filter(|p| p.1 == PairHistory::Reorganizing).map(|p| p.0).collect())
        }
        Err(g) => (Err(g), Vec::new()),
    };
    let coupled_change = EdgeSeries::from_registrations(seq, rel)
        .and_then(|s| coupled_change_pairs(&PersistenceChange::new(&s)));
    Ok(Connection {
        identity,
        proportional,
        sustained: sustained(seq, rel),
        coupled_change,
        reorganizing_pairs,
        evaluated_pairs: n * n.saturating_sub(1) / 2,
        unresolved_edges: unresolved,
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// ENERGY — tunneling (V3)
// ═══════════════════════════════════════════════════════════════════════════
//
// A carrier c is declared in M for the domain (R9a, V3a): what carries change,
// its unit, mapping and δ_c.
//
//     held       E[v] = c[v]
//     exchanged  ε[e] = |A(c)[e]| = |c[s] − c[t]|,   δ_ε[e] = δ_c[s] + δ_c[t]
//
// I-T graded: the tunneling floor holds at δ when ε > δ_ε; degree ε/δ_ε.
// Energy has its own carrier: it flows whether or not anything reorganizes,
// and connection is not a factor of it. Where no carrier is declared, energy
// is not evaluated. Supersedes `epsilon_e` (ε = |Δι|·𝟙) for V10 readings.

/// An energy carrier declared in M, at one registration.
#[derive(Debug, Clone, PartialEq)]
pub struct EnergyCarrier {
    pub name: String,
    pub unit: String,
    /// Carrier value at each declared locus, projected through M.
    pub values: Vec<f64>,
    /// δ_c at each locus.
    pub delta: Vec<f64>,
}

/// Exchanged energy at one edge and registration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnergyExchange {
    pub edge: usize,
    pub epsilon: f64,
    pub delta_epsilon: f64,
    /// Tunneling floor at δ: ε > δ_ε.
    pub floor_resolved: bool,
}

impl EnergyExchange {
    /// Graded degree ε / δ_ε. `None` when δ_ε is zero.
    pub fn degree(&self) -> Option<f64> {
        if self.delta_epsilon > 0.0 { Some(self.epsilon / self.delta_epsilon) } else { None }
    }
}

/// Exchanged energy at every edge of one registration. A building block.
pub fn energy_exchange(c: &EnergyCarrier, rel: &DeclaredRelations) -> Vec<EnergyExchange> {
    assert_eq!(c.values.len(), rel.n_nodes, "carrier must be declared at every locus");
    assert_eq!(c.delta.len(), rel.n_nodes, "δ_c must be declared at every locus");
    assert!(c.values.iter().all(|v| v.is_finite()), "carrier values must be in D");
    assert!(c.delta.iter().all(|d| d.is_finite() && *d >= 0.0), "δ_c must be finite and non-negative");
    let a = operator_a(&NodeField::new(vec![c.values.clone()]), rel, &[]).spatial;
    rel.edges.iter().enumerate().map(|(e, &(s, t))| {
        let epsilon = a[0][e].abs();
        let delta_epsilon = c.delta[s] + c.delta[t];
        EnergyExchange { edge: e, epsilon, delta_epsilon, floor_resolved: epsilon > delta_epsilon }
    }).collect()
}

/// Half-width of a mapped carrier f over [c ± δ_c] (E4): the larger of
/// |f(c + δ) − f(c)| and |f(c) − f(c − δ)|; f monotone over the interval,
/// declared in M.
pub fn mapped_half_width(f: impl Fn(f64) -> f64, c: f64, delta_c: f64) -> f64 {
    let fc = f(c);
    (f(c + delta_c) - fc).abs().max((fc - f(c - delta_c)).abs())
}

/// Energy over a declared sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct Energy {
    pub carrier: String,
    pub unit: String,
    /// exchanges[t][e] at registration t.
    pub exchanges: Vec<Vec<EnergyExchange>>,
    /// Share of (registration, edge) readings with the floor resolved.
    pub share_resolved: f64,
}

/// Energy over the sequence; `carriers[t]` is the carrier at registration t.
pub fn energy(carriers: Option<&[EnergyCarrier]>, n_registrations: usize, rel: &DeclaredRelations)
    -> Result<Energy, DeclarationGap>
{
    if n_registrations < 2 { return Err(DeclarationGap::NoProcessSequence); }
    let cs = carriers.ok_or(DeclarationGap::NoCarrierDeclared)?;
    assert_eq!(cs.len(), n_registrations, "the carrier must be declared at every registration");
    if rel.n_edges() == 0 { return Err(DeclarationGap::FewerThanTwoEdges); }
    let exchanges: Vec<Vec<EnergyExchange>> = cs.iter().map(|c| energy_exchange(c, rel)).collect();
    let total = exchanges.len() * rel.n_edges();
    let resolved = exchanges.iter().flatten().filter(|x| x.floor_resolved).count();
    Ok(Energy {
        carrier: cs[0].name.clone(),
        unit: cs[0].unit.clone(),
        share_resolved: resolved as f64 / total as f64,
        exchanges,
    })
}

/// Connected exchange (V3b): edges whose circulation is held (sustained grade)
/// and whose tunneling floor is resolved at every registration, with ε at
/// each registration.
pub fn connected_exchange(energy: &Energy, connection: &Connection) -> Vec<(usize, Vec<f64>)> {
    let held = match &connection.sustained { Ok(s) => &s.held_edges, Err(_) => return Vec::new() };
    held.iter().filter_map(|&e| {
        let xs: Vec<&EnergyExchange> = energy.exchanges.iter().map(|t| &t[e]).collect();
        if xs.iter().all(|x| x.floor_resolved) {
            Some((e, xs.iter().map(|x| x.epsilon).collect()))
        } else { None }
    }).collect()
}

// ═══════════════════════════════════════════════════════════════════════════
// All three, read together (G4, W11)
// ═══════════════════════════════════════════════════════════════════════════

/// The three ground variables over one declared sequence. Read beside each
/// other; never combined into one number (V6).
#[derive(Debug, Clone, PartialEq)]
pub struct GroundReading {
    pub energy: Result<Energy, DeclarationGap>,
    /// Freedom as change permitted (PC1–PC3).
    pub freedom: Result<PersistenceChange, DeclarationGap>,
    pub connection: Result<Connection, DeclarationGap>,
    /// The first V2 form (a minor held resolved): a persistent arrangement,
    /// structure under the 7th amendment. Declared comparison (C) only.
    pub v2_held_arrangement: Result<Freedom, DeclarationGap>,
}

/// Read all three over one declared sequence (rule of inclusion, W11).
pub fn ground_reading(
    seq: &[Registration], rel: &DeclaredRelations, carriers: Option<&[EnergyCarrier]>,
) -> GroundReading {
    GroundReading {
        energy: energy(carriers, seq.len(), rel),
        freedom: EdgeSeries::from_registrations(seq, rel).map(|s| PersistenceChange::new(&s)),
        connection: connection(seq, rel),
        v2_held_arrangement: freedom(seq, rel),
    }
}

impl GroundReading {
    /// Findings about the declaration: every reading that could not be
    /// calculated, and every one whose degree is zero at δ.
    pub fn findings(&self) -> Vec<Finding> {
        let mut out = Vec::new();
        let gap = |variable, grade, g: &DeclarationGap|
            Finding { variable, grade, kind: g.finding(), gap: Some(g.clone()) };
        let zero = |variable, grade|
            Finding { variable, grade, kind: FindingKind::NotResolvedAtDelta, gap: None };

        match &self.energy {
            Err(g) => out.push(gap(GroundVariable::Energy, None, g)),
            Ok(e) if e.share_resolved == 0.0 => out.push(zero(GroundVariable::Energy, None)),
            _ => {}
        }
        match &self.freedom {
            Err(g) => out.push(gap(GroundVariable::Freedom, None, g)),
            Ok(pc) => {
                for g in pc.gaps() { out.push(gap(GroundVariable::Freedom, None, &g)); }
                if pc.f_delta() == Some(0.0) { out.push(zero(GroundVariable::Freedom, None)); }
            }
        }
        match &self.connection {
            Err(g) => out.push(gap(GroundVariable::Connection, None, g)),
            Ok(c) => {
                let p = Some(ConnectionGrade::Proportional);
                let s = Some(ConnectionGrade::Sustained);
                if let Err(g) = &c.proportional { out.push(gap(GroundVariable::Connection, p, g)); }
                if let Err(g) = &c.sustained { out.push(gap(GroundVariable::Connection, s, g)); }
                if let Err(g) = &c.coupled_change {
                    out.push(gap(GroundVariable::Connection, Some(ConnectionGrade::CoupledChange), g));
                }
                let any = !c.identity.is_empty()
                    || c.coupled_change.as_ref().map_or(false, |v| !v.is_empty())
                    || c.proportional.as_ref().map_or(false, |v| !v.is_empty())
                    || c.sustained.as_ref().map_or(false, |v| !v.held_edges.is_empty());
                if !any { out.push(zero(GroundVariable::Connection, None)); }
            }
        }
        out
    }

    /// True when every variable and every grade is calculated and each
    /// variable has a non-zero degree at δ.
    pub fn complete(&self) -> bool {
        self.findings().is_empty()
    }
}
