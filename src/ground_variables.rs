// ground_variables.rs — Metatron Dynamics, Inc. Kernel V10.
// The three ground variables read at declared resolution δ:
//   freedom    ↔ superposition (I-S)   — V2
//   connection ↔ entanglement  (I-E)   — V1
//   energy     ↔ tunneling     (I-T)   — V3
// Bounded over D. No claim beyond D.
//
// ── Observable fact and calculated variable (Origin, 2026-11-21) ──────────
//
// Two things are held distinct:
//
//   1. The OBSERVABLE FACT. Energy, connection and freedom are present in
//      every registration (G1, G4). Their absence is not an available
//      observation. Nothing in this module tests, confirms or denies it.
//
//   2. The CALCULATED VARIABLE. Over a declared topology, at declared
//      resolution δ, the operators yield a degree for each variable. This
//      module computes those degrees.
//
// When a declared topology does not let one of the three be calculated, the
// result is a statement about the DECLARATION, never about the system. It
// is one of two findings:
//
//   - PRIMARY REGION: no interior locus is declared (T7). Δ → Σ applies and
//     the full path A → B → R is not formable at this resolution; or
//   - DOMAIN NOT FULLY DECLARED: the domain declaration (R9a) is
//     incomplete: a carrier, a component, a relation, a registration or a
//     finer δ has not been declared.
//
// The remedy in both cases is to return to the declaration (W5, G4a): analyse
// in more detail, or widen the record boundary along registered relations. A
// declared edge is never reversed to obtain a reading (W8), and no
// substitute value is introduced.
//
// A degree of zero at δ is a calculated result, returned unchanged. It is
// read under W2 as "not resolved under the declared relations at δ" and
// reported as a finding of the same kind, never as absence.
//
// ── Operators ─────────────────────────────────────────────────────────────
//
// Every reading below runs through the unchanged operators: A (`operator_a`)
// and B (`operator_b`). No operator formula changes in V10. Resolution is
// propagated as interval half-widths: A and B are linear with coefficients
// ±1, so their half-widths are A and B applied with absolute coefficients
// (δ_A[e] = δ[s] + δ[t]; δ_B = B(δ_A)).
//
// The frozen V9 readers in `derived_invariants.rs`
// (`superposition_condition`, `entanglement_condition`, `epsilon_e`,
// `tunneling_floor_holds`) are unchanged. The forms here supersede them for
// V10 readings.

use crate::operators::{operator_a, operator_b, DeclaredRelations, EdgeField, NodeField};

// ═══════════════════════════════════════════════════════════════════════════
// Resolution and readings
// ═══════════════════════════════════════════════════════════════════════════

/// Sign of a calculated value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

/// A calculated value read at declared resolution δ.
#[derive(Debug, Clone, PartialEq)]
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

/// Sign of `value` resolved at half-width `half_width`: resolved when
/// |value| > half_width. An exact 0.0 with half-width 0.0 has no sign and is
/// not resolved.
pub fn sign_at_delta(value: f64, half_width: f64) -> AtDelta<Sign> {
    assert!(half_width.is_finite() && half_width >= 0.0,
        "δ must be declared, finite and non-negative");
    if value.abs() > half_width {
        AtDelta::Resolved(if value > 0.0 { Sign::Positive } else { Sign::Negative })
    } else {
        AtDelta::NotResolved
    }
}

/// Half-width of x·y over the intervals [x ± dx] and [y ± dy].
/// Conservative bound: |x|·dy + |y|·dx + dx·dy.
pub fn product_half_width(x: f64, dx: f64, y: f64, dy: f64) -> f64 {
    x.abs() * dy + y.abs() * dx + dx * dy
}

// ═══════════════════════════════════════════════════════════════════════════
// Declaration gaps and findings
// ═══════════════════════════════════════════════════════════════════════════

/// The three ground variables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroundVariable {
    /// Freedom: how many independent ways a relation varies. Superposition, I-S.
    Freedom,
    /// Connection: how much holds together as it changes. Entanglement, I-E.
    Connection,
    /// Energy: how much a relation changes. Tunneling, I-T.
    Energy,
}

/// Why a ground variable cannot be calculated over the declared topology.
/// Each gap is a statement about the declaration, not about the system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclarationGap {
    /// No interior locus is declared anywhere in the field (T7): the subject
    /// is read at the Primary Region, Δ → Σ.
    PrimaryRegion,
    /// Edge has no successor and no arriving relation in the declared field
    /// (IV-3). Connection is not formable at this edge.
    NoNeighbourInField { edge: usize },
    /// Only one component is declared (T5). Freedom is read across
    /// components and needs at least two.
    SingleComponent,
    /// Fewer than two declared edges.
    FewerThanTwoEdges,
    /// Fewer than two edges are resolved at δ, so no pair can be read.
    NoResolvedPair,
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
            DeclarationGap::PrimaryRegion =>
                "no interior locus declared: read at the Primary Region (Δ → Σ), \
                 or declare the interior at finer δ",
            DeclarationGap::NoNeighbourInField { .. } =>
                "widen the cycle of analysis or declare the registered relations \
                 continuing from or arriving at this edge; never reverse an edge",
            DeclarationGap::SingleComponent =>
                "declare a second component over the same loci (T5), or read \
                 freedom at a region where components are registered",
            DeclarationGap::FewerThanTwoEdges =>
                "declare the registered relations of the domain",
            DeclarationGap::NoResolvedPair =>
                "declare a finer δ or add registrations so at least two edges \
                 are resolved",
            DeclarationGap::NoCarrierDeclared =>
                "declare the energy carrier, its unit, mapping and δ in M (R9a)",
        }
    }
}

/// The two kinds of finding a non-calculable or unresolved variable yields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingKind {
    /// No interior locus declared: Primary Region.
    PrimaryRegion,
    /// The domain declaration is incomplete.
    DomainNotFullyDeclared,
    /// Calculated, but the degree is zero at δ (W2). Return to the
    /// declaration (W5): finer δ, more registrations, or a wider boundary.
    NotResolvedAtDelta,
}

/// A finding about the declaration for one ground variable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub variable: GroundVariable,
    pub kind: FindingKind,
    /// The gap, when the variable could not be calculated.
    pub gap: Option<DeclarationGap>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Shared: A and its half-widths
// ═══════════════════════════════════════════════════════════════════════════

/// δ declared per component and node: `delta[c][i]`, same shape as `x.data`.
fn check_delta_shape(x: &NodeField, delta: &[Vec<f64>]) {
    assert_eq!(delta.len(), x.k, "δ must be declared for every component");
    assert!(delta.iter().all(|d| d.len() == x.n), "δ must be declared for every node");
    assert!(delta.iter().flatten().all(|d| d.is_finite() && *d >= 0.0),
        "δ must be finite and non-negative");
}

/// A(x) and its half-widths δ_A[c][e] = δ[c][s] + δ[c][t].
fn a_with_half_width(
    x: &NodeField,
    delta: &[Vec<f64>],
    rel: &DeclaredRelations,
) -> (EdgeField, Vec<Vec<f64>>) {
    check_delta_shape(x, delta);
    let a = operator_a(x, rel, &[]);
    let da = (0..x.k)
        .map(|c| rel.edges.iter().map(|&(s, t)| delta[c][s] + delta[c][t]).collect())
        .collect();
    (a, da)
}

/// True when at least one declared locus is the target of one edge and the
/// source of another (T7).
pub fn has_interior(rel: &DeclaredRelations) -> bool {
    (0..rel.n_edges()).any(|e| !rel.succ(e).is_empty())
}

// ═══════════════════════════════════════════════════════════════════════════
// FREEDOM — superposition (V2; Origin, 2026-11-21)
// ═══════════════════════════════════════════════════════════════════════════
//
// Freedom is how many independent ways the declared relations vary. Two edges
// e and f vary in independent ways when their A vectors across components are
// not proportional.
//
// Form (2×2 minors). For components i < j:
//
//     m_ij(e, f) = A[i][e]·A[j][f] − A[j][e]·A[i][f]
//
// A[e] and A[f] are proportional exactly when every m_ij is zero. The pair is
// DISTINGUISHABLE AT δ when the sign of at least one m_ij is resolved at δ.
// With two components there is one minor; this is the V2 contrast.
//
// Properties, by construction:
//   - pairwise only: no rank, no matrix decomposition;
//   - symmetric in e and f (m_ij(f, e) = −m_ij(e, f)), so the reading does
//     not depend on edge order (closes the V8 HOLD on `are_proportional`);
//   - one resolution in one unit: the minor's interval, from δ_A;
//   - any number of components (closes OC-V-2).
//
// An edge whose A vector is not resolved at δ in any component is reported
// separately and enters no pair (V2).
//
// F_δ = distinguishable pairs / evaluated pairs.

/// The minor m_ij(e, f) over the edge field a[c][e].
pub fn freedom_minor(a: &[Vec<f64>], e: usize, f: usize, i: usize, j: usize) -> f64 {
    a[i][e] * a[j][f] - a[j][e] * a[i][f]
}

/// Half-width of m_ij(e, f) from the half-widths da[c][e].
pub fn freedom_minor_half_width(
    a: &[Vec<f64>], da: &[Vec<f64>], e: usize, f: usize, i: usize, j: usize,
) -> f64 {
    product_half_width(a[i][e], da[i][e], a[j][f], da[j][f])
        + product_half_width(a[j][e], da[j][e], a[i][f], da[i][f])
}

/// True when edge e's A vector is resolved at δ in at least one component.
pub fn edge_resolved(a: &[Vec<f64>], da: &[Vec<f64>], e: usize) -> bool {
    (0..a.len()).any(|c| a[c][e].abs() > da[c][e])
}

/// Reading of one edge pair.
/// `Resolved(true)`: distinguishable at δ. `Resolved(false)`: proportional at
/// δ (every minor's interval contains zero). `NotResolved`: one of the two
/// edges is not resolved at δ.
pub fn freedom_pair(
    a: &[Vec<f64>], da: &[Vec<f64>], e: usize, f: usize,
) -> Result<AtDelta<bool>, DeclarationGap> {
    let k = a.len();
    if k < 2 {
        return Err(DeclarationGap::SingleComponent);
    }
    if !edge_resolved(a, da, e) || !edge_resolved(a, da, f) {
        return Ok(AtDelta::NotResolved);
    }
    for i in 0..k {
        for j in (i + 1)..k {
            let m = freedom_minor(a, e, f, i, j);
            let hw = freedom_minor_half_width(a, da, e, f, i, j);
            if sign_at_delta(m, hw).is_resolved() {
                return Ok(AtDelta::Resolved(true));
            }
        }
    }
    Ok(AtDelta::Resolved(false))
}

/// Degree of freedom over a declared field at δ.
#[derive(Debug, Clone, PartialEq)]
pub struct FreedomDegree {
    /// Pairs (e, f), e < f, distinguishable at δ.
    pub distinguishable_pairs: Vec<(usize, usize)>,
    /// Number of pairs of resolved edges evaluated.
    pub evaluated_pairs: usize,
    /// Edges not resolved at δ in any component (reported separately).
    pub unresolved_edges: Vec<usize>,
    /// F_δ = distinguishable / evaluated.
    pub f_delta: f64,
}

/// Freedom over the declared node field x with resolution δ[c][i].
pub fn freedom_degree(
    x: &NodeField, delta: &[Vec<f64>], rel: &DeclaredRelations,
) -> Result<FreedomDegree, DeclarationGap> {
    if x.k < 2 {
        return Err(DeclarationGap::SingleComponent);
    }
    if rel.n_edges() < 2 {
        return Err(DeclarationGap::FewerThanTwoEdges);
    }
    let (a, da) = a_with_half_width(x, delta, rel);
    let a = &a.spatial;
    let (resolved, unresolved): (Vec<usize>, Vec<usize>) =
        (0..rel.n_edges()).partition(|&e| edge_resolved(a, &da, e));
    if resolved.len() < 2 {
        return Err(DeclarationGap::NoResolvedPair);
    }
    let mut distinguishable = Vec::new();
    let mut evaluated = 0;
    for (n, &e) in resolved.iter().enumerate() {
        for &f in &resolved[n + 1..] {
            evaluated += 1;
            if freedom_pair(a, &da, e, f)? == AtDelta::Resolved(true) {
                distinguishable.push((e, f));
            }
        }
    }
    let f_delta = distinguishable.len() as f64 / evaluated as f64;
    Ok(FreedomDegree {
        distinguishable_pairs: distinguishable,
        evaluated_pairs: evaluated,
        unresolved_edges: unresolved,
        f_delta,
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// CONNECTION — entanglement (V1; IV-3 Origin, 2026-11-21)
// ═══════════════════════════════════════════════════════════════════════════
//
// Connection is how much holds together as the relations change. It is read
// from the circulation term of R over g = B(A(x)):
//
//     (R − B)[e] = ρ[src(e)] · (Σ_{succ(e)} g − Σ_{pred(e)} g)
//
// Since ρ > 0, the sign of (R − B)[e] is the sign of the bracket and does not
// depend on the open ρ choices (OC-K-1). 𝟙_δ[e] holds when that sign is
// resolved at δ. The bracket's half-width is Σ_{succ} δ_B + Σ_{pred} δ_B with
// δ_B = B(δ_A).
//
// Formability (IV-3, declared 2026-11-21): if e has no successor and no
// arriving relation in the declared field, the bracket is zero by topology
// alone. That is a gap in the declaration, NoNeighbourInField, not
// decoherence. If no edge in the field has a successor, no interior locus is
// declared: PrimaryRegion.
//
// Scope: the spatial circulation term. The cross-component term scaled by
// cc[p] is not part of V1.
//
// C_δ, connection preserved across a process sequence, remains OPEN
// (OC-V-1).

/// Connection reading at edge e, one entry per component.
#[derive(Debug, Clone, PartialEq)]
pub struct ConnectionAtEdge {
    pub edge: usize,
    /// Σ_succ B − Σ_pred B per component.
    pub bracket: Vec<f64>,
    /// Half-width of the bracket per component.
    pub half_width: Vec<f64>,
    /// Sign of (R − B)[e] at δ per component.
    pub sign: Vec<AtDelta<Sign>>,
}

impl ConnectionAtEdge {
    /// 𝟙_δ[e]: the sign is resolved in at least one component.
    pub fn connected(&self) -> bool {
        self.sign.iter().any(|s| s.is_resolved())
    }
}

/// B(A(x)) and its half-widths δ_B = B(δ_A), through the unchanged operator_b.
fn b_with_half_width(
    x: &NodeField, delta: &[Vec<f64>], rel: &DeclaredRelations,
) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let (a, da) = a_with_half_width(x, delta, rel);
    let b = operator_b(&a, rel).spatial;
    let da_field = EdgeField { spatial: da, comp: vec![], comp_pairs: vec![], k: x.k };
    let db = operator_b(&da_field, rel).spatial;
    (b, db)
}

fn connection_from_b(
    b: &[Vec<f64>], db: &[Vec<f64>], rel: &DeclaredRelations, e: usize,
) -> Result<ConnectionAtEdge, DeclarationGap> {
    if !has_interior(rel) {
        return Err(DeclarationGap::PrimaryRegion);
    }
    let (succ, pred) = (rel.succ(e), rel.pred(e));
    if succ.is_empty() && pred.is_empty() {
        return Err(DeclarationGap::NoNeighbourInField { edge: e });
    }
    let k = b.len();
    let mut bracket = Vec::with_capacity(k);
    let mut half_width = Vec::with_capacity(k);
    let mut sign = Vec::with_capacity(k);
    for c in 0..k {
        let fwd: f64 = succ.iter().map(|&f| b[c][f]).sum();
        // Backward READING of arriving relations (Df2a); none is reversed.
        let bwd: f64 = pred.iter().map(|&p| b[c][p]).sum();
        let hw: f64 = succ.iter().chain(pred).map(|&f| db[c][f]).sum();
        bracket.push(fwd - bwd);
        half_width.push(hw);
        sign.push(sign_at_delta(fwd - bwd, hw));
    }
    Ok(ConnectionAtEdge { edge: e, bracket, half_width, sign })
}

/// Connection at edge e over the declared node field x with resolution δ.
pub fn connection_at_edge(
    x: &NodeField, delta: &[Vec<f64>], rel: &DeclaredRelations, e: usize,
) -> Result<ConnectionAtEdge, DeclarationGap> {
    let (b, db) = b_with_half_width(x, delta, rel);
    connection_from_b(&b, &db, rel, e)
}

/// Degree of connection over a declared field at δ.
#[derive(Debug, Clone, PartialEq)]
pub struct ConnectionDegree {
    /// Readings at every edge where connection is formable.
    pub edges: Vec<ConnectionAtEdge>,
    /// Edges where connection is not formable in this field (IV-3).
    pub not_formable: Vec<usize>,
    /// Share of formable edges with 𝟙_δ = 1.
    pub share_connected: f64,
}

/// Connection over every declared edge, in one pass (Df3).
pub fn connection_degree(
    x: &NodeField, delta: &[Vec<f64>], rel: &DeclaredRelations,
) -> Result<ConnectionDegree, DeclarationGap> {
    if !has_interior(rel) {
        return Err(DeclarationGap::PrimaryRegion);
    }
    let (b, db) = b_with_half_width(x, delta, rel);
    let mut edges = Vec::new();
    let mut not_formable = Vec::new();
    for e in 0..rel.n_edges() {
        match connection_from_b(&b, &db, rel, e) {
            Ok(r) => edges.push(r),
            Err(DeclarationGap::NoNeighbourInField { edge }) => not_formable.push(edge),
            Err(g) => return Err(g),
        }
    }
    let connected = edges.iter().filter(|r| r.connected()).count();
    let share_connected = connected as f64 / edges.len() as f64;
    Ok(ConnectionDegree { edges, not_formable, share_connected })
}

// ═══════════════════════════════════════════════════════════════════════════
// ENERGY — tunneling (V3; ENERGY_V3 form adopted, Origin, 2026-11-21)
// ═══════════════════════════════════════════════════════════════════════════
//
// Energy is how much a relation changes. A carrier c is declared in M for the
// domain (R9a, V3a): what carries change, its unit, mapping and δ_c.
//
//     held       E[v] = c[v]                   (the carrier at a locus)
//     exchanged  ε[e] = |A(c)[e]| = |c[s] − c[t]|
//     δ_ε[e]     = δ_c[s] + δ_c[t]
//
// I-T graded: the tunneling floor holds at δ when ε[e] > δ_ε[e]; the degree
// is ε / δ_ε. ε is in the carrier's declared unit.
//
// The connection indicator is NOT a factor of ε. Energy and connection are
// read beside each other. Their joint reading is the connected exchange
// ε·𝟙_δ (V3b), formed only when both are resolved.
//
// Where no carrier is declared, energy is not evaluated: NoCarrierDeclared.
//
// Supersedes for V10 readings: `epsilon_e` (ε = |Δι|·𝟙, KR §13.3), which is
// unchanged for existing callers.

/// An energy carrier declared in M for one domain.
#[derive(Debug, Clone, PartialEq)]
pub struct EnergyCarrier {
    /// What carries change in this domain.
    pub name: String,
    /// Declared unit of the carrier.
    pub unit: String,
    /// Carrier value at each declared locus, projected through M.
    pub values: Vec<f64>,
    /// Half-width δ_c at each locus.
    pub delta: Vec<f64>,
}

/// Exchanged energy at one edge.
#[derive(Debug, Clone, PartialEq)]
pub struct EnergyExchange {
    pub edge: usize,
    /// ε[e] = |c[s] − c[t]|, in the carrier's unit.
    pub epsilon: f64,
    /// δ_ε[e] = δ_c[s] + δ_c[t].
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

fn check_carrier(carrier: &EnergyCarrier, rel: &DeclaredRelations) {
    assert_eq!(carrier.values.len(), rel.n_nodes, "carrier must be declared at every locus");
    assert_eq!(carrier.delta.len(), rel.n_nodes, "δ_c must be declared at every locus");
    assert!(carrier.values.iter().all(|v| v.is_finite()), "carrier values must be in D");
    assert!(carrier.delta.iter().all(|d| d.is_finite() && *d >= 0.0),
        "δ_c must be finite and non-negative");
}

/// Exchanged energy at edge e.
pub fn energy_exchange(
    carrier: Option<&EnergyCarrier>, rel: &DeclaredRelations, e: usize,
) -> Result<EnergyExchange, DeclarationGap> {
    let c = carrier.ok_or(DeclarationGap::NoCarrierDeclared)?;
    check_carrier(c, rel);
    let (s, t) = rel.edges[e];
    let x = NodeField::new(vec![c.values.clone()]);
    let a = operator_a(&x, rel, &[]);
    let epsilon = a.spatial[0][e].abs();
    let delta_epsilon = c.delta[s] + c.delta[t];
    Ok(EnergyExchange { edge: e, epsilon, delta_epsilon, floor_resolved: epsilon > delta_epsilon })
}

/// Half-width of a mapped carrier f over [c ± δ_c] (E4): the larger of
/// |f(c + δ) − f(c)| and |f(c) − f(c − δ)|. Valid when f is monotone over the
/// interval; the mapping and its monotonicity are declared in M.
pub fn mapped_half_width(f: impl Fn(f64) -> f64, c: f64, delta_c: f64) -> f64 {
    let fc = f(c);
    (f(c + delta_c) - fc).abs().max((fc - f(c - delta_c)).abs())
}

/// Connected exchange ε·𝟙_δ (V3b). Formed only when the tunneling floor and
/// the connection sign are both resolved at δ.
pub fn connected_exchange(energy: &EnergyExchange, connection: &ConnectionAtEdge) -> AtDelta<f64> {
    assert_eq!(energy.edge, connection.edge, "energy and connection must be read at the same edge");
    if energy.floor_resolved && connection.connected() {
        AtDelta::Resolved(energy.epsilon)
    } else {
        AtDelta::NotResolved
    }
}

/// Degree of energy over every declared edge.
#[derive(Debug, Clone, PartialEq)]
pub struct EnergyDegree {
    pub carrier: String,
    pub unit: String,
    pub edges: Vec<EnergyExchange>,
    /// Share of edges with the tunneling floor resolved at δ.
    pub share_resolved: f64,
}

/// Energy over every declared edge, in one pass.
pub fn energy_degree(
    carrier: Option<&EnergyCarrier>, rel: &DeclaredRelations,
) -> Result<EnergyDegree, DeclarationGap> {
    let c = carrier.ok_or(DeclarationGap::NoCarrierDeclared)?;
    if rel.n_edges() == 0 {
        return Err(DeclarationGap::FewerThanTwoEdges);
    }
    let edges: Vec<EnergyExchange> = (0..rel.n_edges())
        .map(|e| energy_exchange(Some(c), rel, e))
        .collect::<Result<_, _>>()?;
    let resolved = edges.iter().filter(|x| x.floor_resolved).count();
    Ok(EnergyDegree {
        carrier: c.name.clone(),
        unit: c.unit.clone(),
        share_resolved: resolved as f64 / edges.len() as f64,
        edges,
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// All three, read together (G4, W11)
// ═══════════════════════════════════════════════════════════════════════════

/// The three ground variables over one declared field. Each is calculated or
/// names its declaration gap. They are read beside each other and never
/// combined into one number (V6).
#[derive(Debug, Clone, PartialEq)]
pub struct GroundReading {
    pub freedom: Result<FreedomDegree, DeclarationGap>,
    pub connection: Result<ConnectionDegree, DeclarationGap>,
    pub energy: Result<EnergyDegree, DeclarationGap>,
}

/// Read all three ground variables over one declared field (rule of
/// inclusion, W11).
pub fn ground_reading(
    x: &NodeField,
    delta: &[Vec<f64>],
    rel: &DeclaredRelations,
    carrier: Option<&EnergyCarrier>,
) -> GroundReading {
    GroundReading {
        freedom: freedom_degree(x, delta, rel),
        connection: connection_degree(x, delta, rel),
        energy: energy_degree(carrier, rel),
    }
}

impl GroundReading {
    /// Findings about the declaration: every variable that could not be
    /// calculated, and every one whose degree is zero at δ.
    pub fn findings(&self) -> Vec<Finding> {
        let mut out = Vec::new();
        let mut push = |variable, r: Result<f64, &DeclarationGap>| match r {
            Err(g) => out.push(Finding { variable, kind: g.finding(), gap: Some(g.clone()) }),
            Ok(d) if d == 0.0 => out.push(Finding {
                variable, kind: FindingKind::NotResolvedAtDelta, gap: None,
            }),
            Ok(_) => {}
        };
        push(GroundVariable::Freedom, self.freedom.as_ref().map(|f| f.f_delta));
        push(GroundVariable::Connection, self.connection.as_ref().map(|c| c.share_connected));
        push(GroundVariable::Energy, self.energy.as_ref().map(|e| e.share_resolved));
        out
    }

    /// True when all three are calculated with a non-zero degree at δ.
    pub fn complete(&self) -> bool {
        self.findings().is_empty()
    }
}
