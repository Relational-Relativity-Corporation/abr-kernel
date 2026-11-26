// domain.rs — Metatron Dynamics, Inc. Kernel V10.3. Bounded over D. No claim beyond D.
//
// §2.7, the declared ground (Declared, Origin, 2026-11-23, 57th; drafted 56th).
//
// (1) The domain declaration (R9a carried into the calculation). Before any
//     operator acts, the declaration maps what energy, freedom and
//     connection are in the system, so the operators have them to run on:
//       energy      its expressions: held at loci (a carrier component,
//                   declared once), transferred across a relation (ER-1),
//                   the ordering of the sequence (T6); each with unit, δ,
//                   observed range and participation;
//       freedom     every component of the field with its role, why the
//                   components are one field (T5), the prepared constraints
//                   it is read against (I-SC-1 as amended, 49th);
//       connection  the relations carrying continuation (the declared
//                   relations, with provenance, identity and feedback), the
//                   constraints declared to hold structure.
//     Guard: the declaration maps where and what; it never states a result
//     (G1, G3). Refusal, not repair: a declaration that does not match its
//     registrations is refused.
//
// (2) The ground record. For every declared relation e, component c and step
//     s, all three variables, each active, noted as structure, or not
//     evaluated with its reason. None is absent. Shares stay projections.
//
// (3) Stages of relational evolution are read from the records (I-SC-D18
//     boundaries); the band location is the application's, from its declared
//     parameters.
//
// What this module adds to the 10.3.0 reading: energy read from its declared
// expressions, with every part of δ (the carrier is declared once: it
// answers I-SC-D17's finding that it was declared twice, 53rd); energy
// transferred across a relation (edge-registered energy, OC-WF-1 for energy);
// held energy at loci reported; and the ground record. `RelationalField::new`
// is unchanged for its callers.

use crate::exact::{exceeds, Expansion};
use crate::ground_variables::{AtDelta, DeclarationGap, ObservedRange, Registration, Sign};
use crate::operators::DeclaredRelations;
use crate::persistence_change::{EdgeSeries, PersistenceChange, Reading, ReadingPair};
use crate::relational_field::RelationalField;

// ═══════════════════════════════════════════════════════════════════════════
// (1) The declaration
// ═══════════════════════════════════════════════════════════════════════════

/// How the observer participates (Df4, R9a).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Participation { Prepared, Probed, Received }

/// How energy participates in this system (R9a: supplied, removed, confined
/// or received).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnergyParticipation { Supplied, Removed, Confined, Received }

/// Where a transfer is registered.
#[derive(Debug, Clone, PartialEq)]
pub enum TransferSite {
    /// Across a declared relation of the field.
    Edge(usize),
    /// Into a declared locus from a source outside the field (for example a
    /// supply meter): the source is named, not a locus.
    Entering { locus: usize, source: String },
    /// Out of a declared locus into a sink outside the field (for example a
    /// metered cooler): the sink is named, not a locus (58th).
    Leaving { locus: usize, sink: String },
}

/// Energy transferred across a relation, registered per step (ER-1). The
/// value over step s is what the observing chain registers as transferred
/// between registrations s and s + 1.
#[derive(Debug, Clone, PartialEq)]
pub struct EnergyTransfer {
    pub name: String,
    pub unit: String,
    pub site: TransferSite,
    /// values[s], one per step.
    pub values: Vec<f64>,
    /// Repeatability δ[s], one per step.
    pub delta: Vec<f64>,
    /// A run-constant gain bound (Z1a), for example a calibration factor. A
    /// gain shared by every value scales value and bound together, so it
    /// changes no classification (I-SC-D15); it enters reported magnitudes.
    pub gain: f64,
    /// A declared mapping to another unit, cited as an intersection (W14),
    /// for example "J per pulse from the meter's certificate".
    pub mapping: Option<String>,
    pub participation: EnergyParticipation,
    pub range: ObservedRange,
}

/// One expression of energy in the system (§2.7 (1)).
#[derive(Debug, Clone, PartialEq)]
pub enum EnergyExpression {
    /// Held at loci: the carrier is a component of the registrations, so it
    /// is declared once; every part of its δ (Z1a) enters its readings.
    Held { component: usize, name: String, unit: String, participation: EnergyParticipation },
    /// Transferred across a relation, or entering a locus.
    Transferred(EnergyTransfer),
    /// The ordering of the sequence (T6): energy supplied orders the
    /// registrations. Never a value in a form.
    Ordering { basis: String },
}

/// The role of a component in the field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentRole {
    /// Structural: read for freedom and connection.
    Structural,
    /// The carrier of a `Held` energy expression; read for freedom and
    /// connection too (I-SC-D17).
    EnergyHeld,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FreedomDeclaration {
    /// One per component of the registrations, in order: its name and role.
    pub components: Vec<(String, ComponentRole)>,
    /// Why the components are one field (T5).
    pub basis: String,
    /// The prepared constraints the field is read against, declared and held.
    pub prepared_constraints: Vec<String>,
    pub participation: Participation,
}

/// A constraint declared to hold structure.
#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    /// Registered as declared relations (for example a wall).
    Relations { name: String, edges: Vec<usize> },
    /// A prepared condition of the run (for example a sealed volume or a
    /// regulated pressure).
    Prepared { name: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConnectionDeclaration {
    /// Why these relations carry continuation (their provenance, identity and
    /// direction are on the declared relations, G-1, T8).
    pub basis: String,
    pub constraints: Vec<Constraint>,
    pub participation: Participation,
}

/// The domain declaration (R9a; §2.7 (1)).
#[derive(Debug, Clone, PartialEq)]
pub struct DomainDeclaration {
    /// Every expression of energy declared. Empty: energy is not evaluated,
    /// and named (W11).
    pub energy: Vec<EnergyExpression>,
    pub freedom: FreedomDeclaration,
    pub connection: ConnectionDeclaration,
}

/// Why a declaration is refused (refusal, not repair).
#[derive(Debug, Clone, PartialEq)]
pub enum DomainRefusal {
    /// The field itself does not form.
    Field(DeclarationGap),
    /// The components are not declared one for one.
    Components { declared: usize, registered: usize },
    /// A held expression names a component that is not declared `EnergyHeld`,
    /// or an `EnergyHeld` component has no held expression.
    HeldCarrier(usize),
    /// A transfer does not match the sequence or the field.
    Transfer(String),
    /// A constraint names a relation that is not declared.
    Constraint(String),
}

// ═══════════════════════════════════════════════════════════════════════════
// Energy readings from the declaration
// ═══════════════════════════════════════════════════════════════════════════

/// A value read against its bound at δ, decided exactly.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Read { pub value: f64, pub bound: f64, pub resolved: bool }

fn read(v: &Expansion, h: &Expansion) -> Read { Read { value: v.nearest(), bound: h.up(), resolved: exceeds(v, h) } }

/// Held energy over the sequence, from one carrier component.
#[derive(Debug, Clone, PartialEq)]
pub struct HeldEnergy {
    pub component: usize,
    pub name: String,
    pub unit: String,
    /// at_loci[t][v]: E[v] = c[v] with its state bound (V3, held). Reported:
    /// present and held, never read as zero (G1).
    pub at_loci: Vec<Vec<Read>>,
    /// exchange[t][e]: the signed exchange A[c][e] against its state bound,
    /// every part of δ included; resolved is the floor, the side resolved
    /// (I-SC-D17 (i); 103rd). *Was:* ε = |A[c][e]| (retired, 104th).
    pub exchange: Vec<Vec<Read>>,
}

/// One transfer over the sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct TransferReading {
    pub name: String,
    pub unit: String,
    pub site: TransferSite,
    /// per_step[s]: the transfer against its repeatability.
    pub per_step: Vec<Read>,
    /// change[s]: the change of transfer from step s to s + 1 against
    /// δ_s + δ_{s+1} (PC1 on the transfer). Needs three registrations.
    pub change: Vec<Read>,
    /// Bound on each reported magnitude, with the declared gain:
    /// δ + γ·(|v| + δ) (Z1a).
    pub magnitude_bound: Vec<f64>,
    pub mapping: Option<String>,
    pub participation: EnergyParticipation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclaredEnergy {
    pub held: Vec<HeldEnergy>,
    pub transfers: Vec<TransferReading>,
    pub ordering: Vec<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// (2) The ground record
// ═══════════════════════════════════════════════════════════════════════════

/// The state of one variable in one record (§2.7 (2)).
#[derive(Debug, Clone, PartialEq)]
pub enum State {
    /// Resolved change or exchange at δ.
    Active,
    /// Held at δ: present, its change or exchange not resolved.
    NotedAsStructure,
    /// Not evaluated, with the reason named (W11).
    NotEvaluated(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnergyEntry {
    /// Held carriers: (component, exchange at s, exchange at s + 1, PC1 on
    /// the carrier over s).
    pub held: Vec<(usize, Read, Read, Reading)>,
    /// Transfers registered across this relation over s: (index, reading,
    /// change of transfer over s where formable).
    pub transfer: Vec<(usize, Read, Option<Read>)>,
    /// Transfers entering this relation's source locus over s: context, at
    /// the locus, not at the relation.
    pub entering_at_source: Vec<(usize, Read)>,
    /// Transfers leaving this relation's target locus over s, into a sink
    /// outside the field: context, at the locus (58th).
    pub leaving_at_target: Vec<(usize, Read)>,
    pub state: State,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FreedomEntry {
    pub pc1: Reading,
    /// PC2 readings involving (e, c) over step s: (freedom, structure, of which forced).
    pub pc2: (usize, usize, usize),
    /// PC3 readings involving (e, c) over step s: (freedom, structure).
    pub pc3: (usize, usize),
    pub state: State,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConnectionEntry {
    /// The imbalance Ξ(B(A))[e] in c at s and s + 1 (`None`: no neighbour, IV-3).
    pub imbalance: Option<(AtDelta<Sign>, AtDelta<Sign>)>,
    /// Sequence grades this relation takes part in.
    pub identity: bool,
    pub proportional: bool,
    pub coupled_change: bool,
    pub coupled_change_forced: bool,
    pub sustained_held: bool,
    pub boundary_conditioned: bool,
    /// Constraints declared to hold structure that name this relation.
    pub constraints: Vec<String>,
    pub state: State,
}

/// All three variables for relation e, component c, step s.
#[derive(Debug, Clone, PartialEq)]
pub struct GroundRecord {
    pub edge: usize,
    pub component: usize,
    pub step: usize,
    pub energy: EnergyEntry,
    pub freedom: FreedomEntry,
    pub connection: ConnectionEntry,
}

/// A relational field formed from a domain declaration: every variable
/// evaluated together, with the ground record.
#[derive(Debug, Clone)]
pub struct DeclaredField {
    pub declaration: DomainDeclaration,
    /// The declared relations the field is formed over.
    pub relations: DeclaredRelations,
    /// The relational field. Its `ground.energy` is formed with no
    /// `EnergyCarrier`; in a declared field energy is `energy`, from the
    /// declaration, and the field's NoCarrierDeclared finding does not apply.
    pub field: RelationalField,
    pub series: EdgeSeries,
    pub energy: DeclaredEnergy,
    pub records: Vec<GroundRecord>,
}

impl DeclaredField {
    /// Form the field from its declaration. Refused when the declaration does
    /// not match the registrations.
    pub fn new(rel: &DeclaredRelations, registrations: Vec<Registration>, declaration: DomainDeclaration)
        -> Result<Self, DomainRefusal>
    {
        let field = RelationalField::new(rel, registrations, None).map_err(DomainRefusal::Field)?;
        let series = EdgeSeries::from_registrations(&field.registrations, rel).map_err(DomainRefusal::Field)?;
        let (k, n_t, m) = (series.k(), series.n_registrations(), rel.n_edges());
        validate(&declaration, rel, k, n_t)?;
        let energy = read_energy(&declaration, &field, &series, rel);
        let records = records(&declaration, &field, &series, &energy, rel);
        debug_assert_eq!(records.len(), m * k * (n_t - 1));
        Ok(DeclaredField { declaration, relations: rel.clone(), field, series, energy, records })
    }

    /// The record for (e, c, s).
    pub fn record(&self, e: usize, c: usize, s: usize) -> &GroundRecord {
        // Records are listed by step, then component, then edge.
        let (k, m) = (self.series.k(), self.relations.n_edges());
        let r = &self.records[(s * k + c) * m + e];
        debug_assert!(r.edge == e && r.component == c && r.step == s);
        r
    }

    /// Energy not evaluated anywhere: true when no expression is declared.
    pub fn energy_not_declared(&self) -> bool { self.declaration.energy.is_empty() }
}

fn validate(d: &DomainDeclaration, rel: &DeclaredRelations, k: usize, n_t: usize) -> Result<(), DomainRefusal> {
    if d.freedom.components.len() != k {
        return Err(DomainRefusal::Components { declared: d.freedom.components.len(), registered: k });
    }
    let held: Vec<usize> = d.energy.iter().filter_map(|x| match x { EnergyExpression::Held { component, .. } => Some(*component), _ => None }).collect();
    for &c in &held {
        if c >= k || d.freedom.components[c].1 != ComponentRole::EnergyHeld { return Err(DomainRefusal::HeldCarrier(c)); }
    }
    for (c, (_, role)) in d.freedom.components.iter().enumerate() {
        if *role == ComponentRole::EnergyHeld && !held.contains(&c) { return Err(DomainRefusal::HeldCarrier(c)); }
    }
    for x in &d.energy {
        if let EnergyExpression::Transferred(t) = x {
            let bad = |why: &str| Err(DomainRefusal::Transfer(format!("{}: {why}", t.name)));
            if t.values.len() != n_t - 1 || t.delta.len() != n_t - 1 { return bad("one value and one δ per step"); }
            if !t.values.iter().all(|v| v.is_finite()) { return bad("values must be in D"); }
            if !t.delta.iter().all(|d| d.is_finite() && *d >= 0.0) { return bad("δ must be finite and non-negative"); }
            if !(t.gain.is_finite() && t.gain >= 0.0 && t.gain < 1.0) { return bad("a gain bound lies in [0, 1) (Z1a)"); }
            if !t.values.iter().chain(&t.delta).chain(std::iter::once(&t.gain)).all(|v| crate::exact::in_exact_range(*v)) {
                return bad("outside the range of exact arithmetic (OC-NUM-1)");
            }
            if !t.values.iter().zip(&t.delta).all(|(v, d)| t.range.admits(*v, *d)) { return bad("outside the observed range (OC-RANGE-1)"); }
            match &t.site {
                TransferSite::Edge(e) if *e >= rel.n_edges() => return bad("not a declared relation"),
                TransferSite::Entering { locus, .. } | TransferSite::Leaving { locus, .. } if *locus >= rel.n_nodes => return bad("not a declared locus"),
                _ => {}
            }
        }
    }
    for c in &d.connection.constraints {
        if let Constraint::Relations { name, edges } = c {
            if edges.iter().any(|&e| e >= rel.n_edges()) { return Err(DomainRefusal::Constraint(name.clone())); }
        }
    }
    Ok(())
}

fn read_energy(d: &DomainDeclaration, field: &RelationalField, s: &EdgeSeries, rel: &DeclaredRelations) -> DeclaredEnergy {
    let n_t = s.n_registrations();
    let mut out = DeclaredEnergy { held: Vec::new(), transfers: Vec::new(), ordering: Vec::new() };
    for x in &d.energy {
        match x {
            EnergyExpression::Held { component, name, unit, .. } => {
                let c = *component;
                let at_loci = field.registrations.iter().map(|r| (0..rel.n_nodes).map(|v|
                    read(&Expansion::of(r.x.data[c][v]), &r.state_bound(c, v))).collect()).collect();
                let exchange = (0..n_t).map(|t| (0..rel.n_edges()).map(|e| {
                    // The signed exchange A(c)[e] (Origin, 103rd, 104th: no
                    // magnitude). *Was:* `value: a.abs().nearest()`.
                    let a = s.entry(t, c, e);
                    read(&a, &Expansion::of(s.da[t][c][e]))
                }).collect()).collect();
                out.held.push(HeldEnergy { component: c, name: name.clone(), unit: unit.clone(), at_loci, exchange });
            }
            EnergyExpression::Transferred(t) => {
                let per_step: Vec<Read> = t.values.iter().zip(&t.delta).map(|(v, h)| read(&Expansion::of(*v), &Expansion::of(*h))).collect();
                let change = (1..t.values.len()).map(|i| read(&Expansion::diff(t.values[i], t.values[i - 1]),
                    &Expansion::of(t.delta[i]).plus(&Expansion::of(t.delta[i - 1])))).collect();
                let magnitude_bound = t.values.iter().zip(&t.delta)
                    .map(|(v, h)| crate::exact::gain_widened(&Expansion::of(*v), &Expansion::of(*h), t.gain).up()).collect();
                out.transfers.push(TransferReading { name: t.name.clone(), unit: t.unit.clone(), site: t.site.clone(),
                    per_step, change, magnitude_bound, mapping: t.mapping.clone(), participation: t.participation });
            }
            EnergyExpression::Ordering { basis } => out.ordering.push(basis.clone()),
        }
    }
    out
}

fn records(d: &DomainDeclaration, field: &RelationalField, s: &EdgeSeries, en: &DeclaredEnergy, rel: &DeclaredRelations) -> Vec<GroundRecord> {
    let (k, n_t, m) = (s.k(), s.n_registrations(), rel.n_edges());
    let pc: &PersistenceChange = field.ground.freedom.as_ref().expect("freedom forms with the field");
    let conn = field.ground.connection.as_ref().ok();
    let imb: Vec<Vec<Option<Vec<AtDelta<Sign>>>>> = field.registrations.iter()
        .map(|r| crate::ground_variables::passage_signs(r, rel)).collect();
    // Indexed once (no reading changes): `relation_changes` lists step, then
    // component, then edge, so (st, c, e) is at st·k·m + c·m + e.
    let ix = |st: usize, c: usize, e: usize| (st * k + c) * m + e;
    debug_assert!(pc.relations.iter().enumerate().all(|(n, x)| ix(x.step, x.component, x.edge) == n));
    let pc1 = |st: usize, c: usize, e: usize| pc.relations[ix(st, c, e)].reading;
    let mut pc2_at = vec![(0usize, 0usize, 0usize); (n_t - 1) * k * m];
    for fam in [&pc.across_components, &pc.across_steps] {
        let Ok(fam) = fam else { continue };
        for x in &fam.readings {
            let at: Vec<(usize, usize)> = match x.over {
                ReadingPair::Components { step, i, j } => vec![(step, i), (step, j)],
                ReadingPair::Steps { component, step } => vec![(step, component)],
            };
            for (st, c) in at {
                for e in [x.e, x.f] {
                    let v = &mut pc2_at[ix(st, c, e)];
                    if x.reading == Reading::Freedom { v.0 += 1 } else { v.1 += 1; if x.forced { v.2 += 1 } }
                }
            }
        }
    }
    let mut pc3_at = vec![(0usize, 0usize); (n_t - 1) * k * m];
    if let Ok(arr) = &pc.arrangement {
        for x in arr {
            for c in [x.i, x.j] {
                for e in [x.e, x.f] {
                    let v = &mut pc3_at[ix(x.step, c, e)];
                    if x.reading == Reading::Freedom { v.0 += 1 } else { v.1 += 1 }
                }
            }
        }
    }
    let in_pair = |p: &(usize, usize), e: usize| p.0 == e || p.1 == e;
    let mut out = Vec::with_capacity(m * k * (n_t - 1));
    for st in 0..n_t - 1 {
        for c in 0..k {
            for e in 0..m {
                // ── Energy ─────────────────────────────────────────────────
                let held: Vec<(usize, Read, Read, Reading)> = en.held.iter()
                    .map(|h| (h.component, h.exchange[st][e], h.exchange[st + 1][e], pc1(st, h.component, e))).collect();
                let transfer: Vec<(usize, Read, Option<Read>)> = en.transfers.iter().enumerate()
                    .filter(|(_, t)| t.site == TransferSite::Edge(e))
                    .map(|(i, t)| (i, t.per_step[st], t.change.get(st).copied())).collect();
                let entering_at_source: Vec<(usize, Read)> = en.transfers.iter().enumerate()
                    .filter(|(_, t)| matches!(&t.site, TransferSite::Entering { locus, .. } if *locus == rel.edges[e].0))
                    .map(|(i, t)| (i, t.per_step[st])).collect();
                let leaving_at_target: Vec<(usize, Read)> = en.transfers.iter().enumerate()
                    .filter(|(_, t)| matches!(&t.site, TransferSite::Leaving { locus, .. } if *locus == rel.edges[e].1))
                    .map(|(i, t)| (i, t.per_step[st])).collect();
                let energy_state = if held.is_empty() && transfer.is_empty() {
                    State::NotEvaluated(if d.energy.is_empty() {
                        "no energy expression declared (R9a, W11)".into()
                    } else {
                        "energy not registered at this relation; present by ground (G1)".into()
                    })
                } else if held.iter().any(|h| h.1.resolved || h.2.resolved || h.3 == Reading::Freedom)
                    || transfer.iter().any(|t| t.1.resolved || t.2.map_or(false, |x| x.resolved)) {
                    State::Active
                } else { State::NotedAsStructure };
                // ── Freedom ────────────────────────────────────────────────
                let (pc2, pc3) = (pc2_at[ix(st, c, e)], pc3_at[ix(st, c, e)]);
                let p1 = pc1(st, c, e);
                let freedom_state = if p1 == Reading::Freedom || pc2.0 > 0 || pc3.0 > 0 { State::Active } else { State::NotedAsStructure };
                // ── Connection ─────────────────────────────────────────────
                let imbalance = match (&imb[st][e], &imb[st + 1][e]) {
                    (Some(a), Some(b)) => Some((a[c], b[c])),
                    _ => None,
                };
                let grade = |r: Option<&Result<Vec<(usize, usize)>, DeclarationGap>>| r.and_then(|x| x.as_ref().ok())
                    .map_or(false, |v| v.iter().any(|p| in_pair(p, e)));
                let identity = conn.map_or(false, |x| x.identity.iter().any(|p| in_pair(p, e)));
                let proportional = grade(conn.map(|x| &x.proportional));
                let coupled_change = grade(conn.map(|x| &x.coupled_change));
                let coupled_change_forced = grade(conn.map(|x| &x.coupled_change_forced));
                let sus = conn.and_then(|x| x.sustained.as_ref().ok());
                let sustained_held = sus.map_or(false, |x| x.held_edges.contains(&e));
                let boundary_conditioned = sus.map_or(false, |x| x.boundary_conditioned.contains(&e));
                let reorganizing = conn.map_or(false, |x| x.reorganizing_pairs.iter().any(|p| in_pair(p, e)));
                let constraints = d.connection.constraints.iter().filter_map(|k| match k {
                    Constraint::Relations { name, edges } if edges.contains(&e) => Some(name.clone()),
                    _ => None,
                }).collect();
                let connection_state = match (&field.ground.connection, imbalance) {
                    (Err(g), None) => State::NotEvaluated(format!("{g:?}: {}", g.remedy())),
                    _ if imbalance.map_or(false, |(a, b)| a != b) || reorganizing => State::Active,
                    _ => State::NotedAsStructure,
                };
                out.push(GroundRecord {
                    edge: e, component: c, step: st,
                    energy: EnergyEntry { held, transfer, entering_at_source, leaving_at_target, state: energy_state },
                    freedom: FreedomEntry { pc1: p1, pc2, pc3, state: freedom_state },
                    connection: ConnectionEntry { imbalance, identity, proportional, coupled_change, coupled_change_forced,
                        sustained_held, boundary_conditioned, constraints, state: connection_state },
                });
            }
        }
    }
    out
}

// ═══════════════════════════════════════════════════════════════════════════
// (3) Stages of relational evolution: their boundaries
// ═══════════════════════════════════════════════════════════════════════════
//
// A stage is a stretch of the sequence over which a declared set of
// persistences holds; its boundaries are where one begins, ends or reverses
// (I-SC-D18). Listed here for persistent imbalance and persistent
// arrangement. Persistent turn sum is 0 by identity on node-derived fields
// (PC-D4) and is read on edge-registered series. Locating a stage as a band
// (where boundaries fall between resolved changes) uses parameters the
// application declares; it is not done here.

/// Which persistence a boundary belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Persistence {
    /// Ξ(B(A))[edge] in `component`.
    Imbalance { edge: usize, component: usize },
    /// m_ij(e, f).
    Arrangement { e: usize, f: usize, i: usize, j: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryKind { Began, Ended, Reversed }

/// A step where a persistence begins, ends or reverses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Boundary {
    pub step: usize,
    pub persistence: Persistence,
    pub kind: BoundaryKind,
    pub before: AtDelta<Sign>,
    pub after: AtDelta<Sign>,
    /// PC1 reads freedom at some relation the reading is formed from. Always
    /// true for `Reversed` (I-SC-D18 (i)); either way for `Began` and `Ended`
    /// (I-SC-D18 (ii)).
    pub freedom_among_its_relations: bool,
}

fn kind(a: AtDelta<Sign>, b: AtDelta<Sign>) -> Option<BoundaryKind> {
    match (a, b) {
        (AtDelta::NotResolved, AtDelta::Resolved(_)) => Some(BoundaryKind::Began),
        (AtDelta::Resolved(_), AtDelta::NotResolved) => Some(BoundaryKind::Ended),
        (AtDelta::Resolved(x), AtDelta::Resolved(y)) if x != y => Some(BoundaryKind::Reversed),
        _ => None,
    }
}

/// Every boundary of persistent imbalance and persistent arrangement in the
/// declared field, in step order.
pub fn persistence_boundaries(df: &DeclaredField) -> Vec<Boundary> {
    let rel_edges = df.relations.n_edges();
    let pc = df.field.ground.freedom.as_ref().expect("freedom forms with the field");
    let free = |st: usize, c: usize, e: usize| df.record(e, c, st).freedom.pc1 == Reading::Freedom;
    let mut out = Vec::new();
    for r in &df.records {
        let Some((a, b)) = r.connection.imbalance else { continue };
        let Some(kind) = kind(a, b) else { continue };
        let w = crate::operators::passage_of_accumulation_coefficients(&df.relations, r.edge, -1);
        let moved = (0..rel_edges).any(|f| w[f] != 0 && free(r.step, r.component, f));
        out.push(Boundary { step: r.step, persistence: Persistence::Imbalance { edge: r.edge, component: r.component },
            kind, before: a, after: b, freedom_among_its_relations: moved });
    }
    if let Ok(arr) = &pc.arrangement {
        for x in arr {
            let Some(kind) = kind(x.before, x.after) else { continue };
            let moved = [(x.i, x.e), (x.j, x.e), (x.i, x.f), (x.j, x.f)].iter().any(|&(c, e)| free(x.step, c, e));
            out.push(Boundary { step: x.step, persistence: Persistence::Arrangement { e: x.e, f: x.f, i: x.i, j: x.j },
                kind, before: x.before, after: x.after, freedom_among_its_relations: moved });
        }
    }
    out.sort_by_key(|b| b.step);
    out
}
