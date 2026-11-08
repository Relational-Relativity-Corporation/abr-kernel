// diagnostic_exec.rs — Metatron Dynamics, Inc. Kernel V9.
// Thin execution wrapper for the ABR operators (A → B → R, and persistence)
// over an experimental relational field, with mandatory edge provenance and
// non-blocking diagnostics (Origin directive, 2026-11-07).
// Bounded over D. No claim beyond D.
//
// ── What this file is ─────────────────────────────────────────────────────
//
// An interface, not mathematics. Every calculated value comes from an
// unchanged public function in operators.rs:
//   spatial:      operator_a, compute_rho, operator_b, operator_r
//   persistence:  operator_a_persistence, compute_rho_persistence,
//                 operator_b_persistence, operator_r_persistence
// over DeclaredRelations::from_edges_with_provenance and NodeField::new.
// No operator is duplicated, altered, or replaced. Nothing is routed through
// admission.rs, and no AdmissionOutcome is required.
//
// ── Errors and warnings ───────────────────────────────────────────────────
//
// ERRORS stop execution before any operator runs: the field cannot be
// evaluated numerically as declared (an invalid reference, incompatible
// dimensions, a non-finite input, an undefined operation, a missing numerical
// input, an incompatible prior). Missing values are never invented.
//
// WARNINGS accompany the results and never stop execution: proposed topology,
// incomplete observational support, uncertain measurement relationships,
// experimental arrangements, and other unresolved conditions. Experimental
// uncertainty is observable information, not a reason to prohibit a
// mathematically valid computation. The kernel makes no truth judgment, applies
// no acceptance threshold, and does not optimise or learn topology.
//
// ── Observation succession ─────────────────────────────────────────────────
//
// The first declared observation is evaluated by execute_spatial alone.
// execute_with_prior pairs an observation with the actual output of the
// observation at the immediately preceding declared process step of the same
// field. Rings and tori (kernel_reference.md §8) concern relational evolution:
// the observation sequence never wraps onto itself. A cyclic arrangement of
// declared relationships (a feedback loop) is not a ring and is not flagged.
//
// ── Provenance ────────────────────────────────────────────────────────────
//
// Every edge carries an EdgeProvenanceRecord. Its `class` is the existing
// operators::EdgeProvenance (mandatory — the type has no "absent" value). Its
// `support` states, separately, whether the relation is observed or proposed;
// a proposed relation stays proposed in every output. Existence of a
// provenance record is not observational certainty.
//
// ── Parameters ────────────────────────────────────────────────────────────
//
// ρ_base and χ₀ are required wherever ρ is computed. Origin general values
// (operators.rs header, 2026-11-07): ρ_base = 0.5, χ₀ = 1 unit of the declared
// χ measurement. The field must identify the values used — either select the
// general values, stating the unit of χ, or declare different values with the
// reason. Nothing is substituted for a missing selection (MissingNumericalInput).

use crate::operators::{
    compute_rho, compute_rho_persistence, operator_a, operator_a_persistence, operator_b,
    operator_b_persistence, operator_r, operator_r_persistence, DeclaredRelations, EdgeField,
    EdgeProvenance, NodeField, PersistenceOutput,
};
use std::collections::{HashMap, HashSet};
use std::panic::{catch_unwind, AssertUnwindSafe};

/// General ρ_base (Origin directive 2026-11-07). Used only when selected.
pub const GENERAL_RHO_BASE: f64 = 0.5;
/// General χ₀, in the declared unit of χ (Origin directive 2026-11-07). Used
/// only when selected.
pub const GENERAL_CHI_0: f64 = 1.0;

// ── Input ─────────────────────────────────────────────────────────────────

/// Observational standing of a declared relation, held apart from its class.
#[derive(Clone, Debug, PartialEq)]
pub enum RelationSupport {
    /// The relation is declared from observation.
    Observed,
    /// The relation is experimentally proposed, not confirmed by observation.
    Proposed,
}

/// Mandatory provenance of one declared edge.
#[derive(Clone, Debug, PartialEq)]
pub struct EdgeProvenanceRecord {
    /// Existing kernel class (Continuation | Coupling). Persistence is not a
    /// spatial class and is an error here (it lives in the prior).
    pub class: EdgeProvenance,
    pub support: RelationSupport,
    /// Statement of, or reference to, the observational support. None or
    /// empty: incomplete observational support (warning).
    pub basis: Option<String>,
}

/// How a node's values are related to observation (the M side).
#[derive(Clone, Debug, PartialEq)]
pub enum MeasurementSupport {
    /// Measurement relationship declared (statement or reference).
    Declared(String),
    /// Measurement relationship declared but uncertain (note).
    Uncertain(String),
    /// No measurement relationship stated.
    NotDeclared,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodeRecord {
    pub id: String,
    /// One value per component. None = not measured / not supplied (error:
    /// never invented).
    pub values: Vec<Option<f64>>,
    pub measurement: MeasurementSupport,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EdgeRecord {
    pub id: String,
    /// Node ids. Direction: source → target, as declared.
    pub source: String,
    pub target: String,
    pub provenance: EdgeProvenanceRecord,
}

/// A declared component pair (a, b) with its cc value.
#[derive(Clone, Debug, PartialEq)]
pub struct ComponentPair {
    pub a: usize,
    pub b: usize,
    /// None: cc not supplied (error — never invented).
    pub cc: Option<f64>,
}

/// Explicit ρ parameter selection. There is no default.
#[derive(Clone, Debug, PartialEq)]
pub enum RhoParameters {
    /// ρ_base = GENERAL_RHO_BASE, χ₀ = GENERAL_CHI_0 in `chi_unit`.
    General { chi_unit: String },
    /// Explicitly declared adjustment, e.g. for numerical disparity in a
    /// relation or ratio.
    Declared { rho_base: f64, chi_0: f64, chi_unit: String, adjustment_basis: String },
}

/// One experimental relational field at one observation.
#[derive(Clone, Debug, PartialEq)]
pub struct ExperimentalField {
    pub field_id: String,
    /// Identity of the observation these values come from. A persistence
    /// prior must come from a different (earlier) observation.
    pub observation_id: String,
    /// The system's own declared process step at which this observation was
    /// made (kernel_reference.md §8, sequential observation requirement).
    /// Persistence pairs only consecutive steps: prior step + 1 = current step.
    pub process_step: u64,
    pub n_components: usize,
    pub nodes: Vec<NodeRecord>,
    pub edges: Vec<EdgeRecord>,
    pub component_pairs: Vec<ComponentPair>,
    pub parameters: Option<RhoParameters>,
    /// Some(note): the variable arrangement is experimental (warning).
    pub experimental_arrangement: Option<String>,
    /// Further scientifically unresolved conditions, carried as warnings.
    pub unresolved_conditions: Vec<String>,
}

// ── Diagnostics ───────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub enum WarningClass {
    ProposedTopology,
    IncompleteObservationalSupport,
    UncertainMeasurement,
    ExperimentalArrangement,
    UnresolvedCondition,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Warning {
    pub class: WarningClass,
    /// The node, edge, field, or parameter the warning concerns.
    pub item: String,
    pub detail: String,
    /// Canonical source of the condition, where there is one.
    pub reference: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorClass {
    InvalidNodeReference,
    DuplicateIdentifier,
    IncompatibleDimensions,
    NonFiniteInput,
    UndefinedOperation,
    MissingNumericalInput,
    IncompatiblePrior,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiagnosticError {
    pub class: ErrorClass,
    pub item: String,
    pub detail: String,
}

/// Execution did not take place. Every error found is listed; warnings found
/// before stopping are kept.
#[derive(Clone, Debug, PartialEq)]
pub struct NotExecuted {
    pub errors: Vec<DiagnosticError>,
    pub warnings: Vec<Warning>,
}

// ── Output ────────────────────────────────────────────────────────────────

/// The ρ parameters actually used, and how they were selected.
#[derive(Clone, Debug, PartialEq)]
pub struct ParameterUse {
    /// "general" or "declared".
    pub selection: &'static str,
    pub rho_base: f64,
    pub chi_0: f64,
    pub chi_unit: String,
    pub adjustment_basis: Option<String>,
}

/// Result of first-observation (spatial) execution: E = R(B(A(x)), ρ(A(x))).
/// Constructed only by `execute_spatial`, so a value of this type is always
/// the actual kernel output of an executed observation.
#[derive(Clone, Debug)]
pub struct SpatialExecution {
    pub field_id: String,
    pub observation_id: String,
    pub process_step: u64,
    pub node_ids: Vec<String>,
    pub edge_ids: Vec<String>,
    /// (source id, target id) per edge, as declared.
    pub edge_endpoints: Vec<(String, String)>,
    /// Provenance per edge, preserved unchanged from the input.
    pub provenance: Vec<EdgeProvenanceRecord>,
    pub component_pairs: Vec<(usize, usize)>,
    pub cc: Vec<f64>,
    pub parameters: ParameterUse,
    /// Operator outputs, in order: A, ρ, B, R. `r` is E for this observation.
    pub a: EdgeField,
    pub rho: Vec<f64>,
    pub b: EdgeField,
    pub r: EdgeField,
    pub warnings: Vec<Warning>,
    rel: DeclaredRelations,
}

impl SpatialExecution {
    /// The DeclaredRelations the operators ran over.
    pub fn relations(&self) -> &DeclaredRelations {
        &self.rel
    }
}

/// Result of subsequent-observation execution: the spatial kernel for the
/// current observation, then A_p → ρ_p → B_p → R_p against the actual prior.
#[derive(Clone, Debug)]
pub struct PersistenceExecution {
    pub current: SpatialExecution,
    pub prior_observation_id: String,
    pub rho_persistence: Vec<f64>,
    pub persistence: PersistenceOutput,
    /// Current-observation warnings plus persistence warnings.
    pub warnings: Vec<Warning>,
}

// ── Validation ────────────────────────────────────────────────────────────

fn err(class: ErrorClass, item: impl Into<String>, detail: impl Into<String>) -> DiagnosticError {
    DiagnosticError { class, item: item.into(), detail: detail.into() }
}

fn warn(class: WarningClass, item: impl Into<String>, detail: impl Into<String>, reference: Option<&'static str>) -> Warning {
    Warning { class, item: item.into(), detail: detail.into(), reference }
}

struct Checked {
    data: Vec<Vec<f64>>,
    edges: Vec<(usize, usize)>,
    classes: Vec<EdgeProvenance>,
    pairs: Vec<(usize, usize)>,
    cc: Vec<f64>,
    params: ParameterUse,
}

fn check(field: &ExperimentalField, errors: &mut Vec<DiagnosticError>, warnings: &mut Vec<Warning>) -> Option<Checked> {
    let k = field.n_components;
    if k == 0 {
        errors.push(err(ErrorClass::IncompatibleDimensions, &field.field_id, "n_components is 0: at least one component required"));
    }
    if field.nodes.is_empty() {
        errors.push(err(ErrorClass::IncompatibleDimensions, &field.field_id, "no nodes declared"));
    }
    if field.edges.is_empty() {
        errors.push(err(ErrorClass::IncompatibleDimensions, &field.field_id, "no relations declared: A has nothing to evaluate"));
    }

    // Nodes: identity, dimensions, values, measurement support.
    let mut node_index = HashMap::new();
    for (i, n) in field.nodes.iter().enumerate() {
        if node_index.insert(n.id.clone(), i).is_some() {
            errors.push(err(ErrorClass::DuplicateIdentifier, &n.id, "node id declared more than once"));
        }
        if n.values.len() != k {
            errors.push(err(ErrorClass::IncompatibleDimensions, &n.id,
                format!("{} values supplied for {k} components", n.values.len())));
        }
        for (c, v) in n.values.iter().enumerate() {
            match v {
                None => errors.push(err(ErrorClass::MissingNumericalInput, &n.id, format!("component {c}: value not supplied"))),
                Some(x) if !x.is_finite() => errors.push(err(ErrorClass::NonFiniteInput, &n.id, format!("component {c}: {x} is not finite (∉ D)"))),
                _ => {}
            }
        }
        match &n.measurement {
            MeasurementSupport::Declared(s) if !s.trim().is_empty() => {}
            MeasurementSupport::Declared(_) | MeasurementSupport::NotDeclared => warnings.push(warn(
                WarningClass::IncompleteObservationalSupport, &n.id, "no measurement relationship (M) stated for this node", None)),
            MeasurementSupport::Uncertain(note) => warnings.push(warn(WarningClass::UncertainMeasurement, &n.id, note.clone(), None)),
        }
    }

    // Edges: identity, references, provenance class, support.
    let mut edge_ids = HashSet::new();
    let mut edges = Vec::new();
    let mut classes = Vec::new();
    for e in &field.edges {
        if !edge_ids.insert(e.id.clone()) {
            errors.push(err(ErrorClass::DuplicateIdentifier, &e.id, "edge id declared more than once"));
        }
        let s = node_index.get(&e.source).copied();
        let t = node_index.get(&e.target).copied();
        if s.is_none() {
            errors.push(err(ErrorClass::InvalidNodeReference, &e.id, format!("source '{}' is not a declared node", e.source)));
        }
        if t.is_none() {
            errors.push(err(ErrorClass::InvalidNodeReference, &e.id, format!("target '{}' is not a declared node", e.target)));
        }
        if e.provenance.class == EdgeProvenance::Persistence {
            errors.push(err(ErrorClass::IncompatibleDimensions, &e.id,
                "class Persistence connects edge-valued loci (E_prior → E_current); it is not a spatial edge between nodes \
                 — supply the prior observation instead (kernel_reference.md §8)"));
        }
        if let (Some(s), Some(t)) = (s, t) {
            edges.push((s, t));
            classes.push(e.provenance.class.clone());
        }
        if e.provenance.support == RelationSupport::Proposed {
            warnings.push(warn(WarningClass::ProposedTopology, &e.id,
                format!("{} → {} is experimentally proposed, not confirmed by observation", e.source, e.target), None));
        }
        if e.provenance.basis.as_deref().map(|b| b.trim().is_empty()).unwrap_or(true) {
            warnings.push(warn(WarningClass::IncompleteObservationalSupport, &e.id,
                "provenance class declared without a stated observational basis", None));
        }
    }

    // Component pairs and cc.
    let mut pairs = Vec::new();
    let mut cc = Vec::new();
    for (p, pair) in field.component_pairs.iter().enumerate() {
        let item = format!("component_pair[{p}] ({}, {})", pair.a, pair.b);
        if pair.a >= k || pair.b >= k {
            errors.push(err(ErrorClass::IncompatibleDimensions, &item, format!("component index outside 0..{k}")));
        }
        match pair.cc {
            None => errors.push(err(ErrorClass::MissingNumericalInput, &item, "cc not supplied for this declared component pair")),
            Some(v) if !v.is_finite() => errors.push(err(ErrorClass::NonFiniteInput, &item, format!("cc = {v} is not finite"))),
            Some(v) => cc.push(v),
        }
        pairs.push((pair.a, pair.b));
    }

    // ρ parameters: explicit selection only.
    let params = match &field.parameters {
        None => {
            errors.push(err(ErrorClass::MissingNumericalInput, "parameters",
                "ρ_base and χ₀ are required to compute ρ; select the general values or declare them (no default)"));
            None
        }
        Some(RhoParameters::General { chi_unit }) => {
            if chi_unit.trim().is_empty() {
                errors.push(err(ErrorClass::MissingNumericalInput, "parameters.chi_unit",
                    "χ₀ = 1 is stated in the declared unit of χ; the unit is not declared"));
            }
            Some(ParameterUse { selection: "general", rho_base: GENERAL_RHO_BASE, chi_0: GENERAL_CHI_0,
                                chi_unit: chi_unit.clone(), adjustment_basis: None })
        }
        Some(RhoParameters::Declared { rho_base, chi_0, chi_unit, adjustment_basis }) => {
            if !rho_base.is_finite() {
                errors.push(err(ErrorClass::NonFiniteInput, "parameters.rho_base", format!("{rho_base} is not finite")));
            }
            if !chi_0.is_finite() {
                errors.push(err(ErrorClass::NonFiniteInput, "parameters.chi_0", format!("{chi_0} is not finite")));
            } else if *chi_0 <= 0.0 {
                errors.push(err(ErrorClass::UndefinedOperation, "parameters.chi_0",
                    format!("χ₀ = {chi_0}: χ/(χ₀+χ) is not evaluable for χ₀ ≤ 0 (kernel_reference.md §9)")));
            }
            if chi_unit.trim().is_empty() {
                errors.push(err(ErrorClass::MissingNumericalInput, "parameters.chi_unit", "unit of χ (and χ₀) not declared"));
            }
            if adjustment_basis.trim().is_empty() {
                warnings.push(warn(WarningClass::IncompleteObservationalSupport, "parameters",
                    "declared ρ_base/χ₀ adjustment has no stated basis", None));
            }
            Some(ParameterUse { selection: "declared", rho_base: *rho_base, chi_0: *chi_0, chi_unit: chi_unit.clone(),
                                adjustment_basis: Some(adjustment_basis.clone()) })
        }
    };

    // Field-level experimental conditions.
    if let Some(note) = &field.experimental_arrangement {
        warnings.push(warn(WarningClass::ExperimentalArrangement, &field.field_id, note.clone(), None));
    }
    for note in &field.unresolved_conditions {
        warnings.push(warn(WarningClass::UnresolvedCondition, &field.field_id, note.clone(), None));
    }

    if !errors.is_empty() {
        return None;
    }
    let data = (0..k).map(|c| field.nodes.iter().map(|n| n.values[c].expect("checked")).collect()).collect();
    Some(Checked { data, edges, classes, pairs, cc, params: params.expect("checked") })
}

/// Structural observations about the declared topology that the canonical
/// reference treats as conditions; reported, never used to stop execution.
fn topology_warnings(field: &ExperimentalField, c: &Checked, warnings: &mut Vec<Warning>) {
    let ids: Vec<&str> = field.nodes.iter().map(|x| x.id.as_str()).collect();

    // Interior locus (kernel_reference.md §10): t = target(e) = source(f).
    let has_in: HashSet<usize> = c.edges.iter().map(|&(_, t)| t).collect();
    let interior: Vec<&str> = c.edges.iter().map(|&(s, _)| s).filter(|s| has_in.contains(s))
        .collect::<HashSet<_>>().into_iter().map(|i| ids[i]).collect();
    if interior.is_empty() {
        warnings.push(warn(WarningClass::UnresolvedCondition, &field.field_id,
            "no interior locus (t = target(e) = source(f)) in the declared topology; the full-operator admissibility \
             condition names the Primary kernel for this case. A → B → R executed as requested.",
            Some("kernel_reference.md §10")));
    }

    // Empty adjacency (audit A-3): an empty succ / pred sum evaluates to 0.0.
    for (e, rec) in field.edges.iter().enumerate() {
        let (s, t) = c.edges[e];
        let succ_empty = !c.edges.iter().any(|&(fs, _)| fs == t);
        let pred_empty = !c.edges.iter().any(|&(_, ft)| ft == s);
        if succ_empty || pred_empty {
            let which = match (succ_empty, pred_empty) { (true, true) => "succ and pred", (true, false) => "succ", _ => "pred" };
            warnings.push(warn(WarningClass::UnresolvedCondition, &rec.id,
                format!("{which} empty: the sum over the empty set is evaluated as 0.0 in B/R; whether it is defined over \
                         the declared adjacent relations is OPEN"),
                Some("operators.rs audit A-3")));
        }
    }
}

fn finite(f: &[Vec<f64>]) -> bool {
    f.iter().all(|c| c.iter().all(|v| v.is_finite()))
}

fn panic_text(p: Box<dyn std::any::Any + Send>) -> String {
    p.downcast_ref::<String>().cloned()
        .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_else(|| "non-string panic payload".into())
}

// ── Execution ─────────────────────────────────────────────────────────────

/// First-observation execution: A → ρ → B → R over the declared field.
pub fn execute_spatial(field: &ExperimentalField) -> Result<SpatialExecution, NotExecuted> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let Some(c) = check(field, &mut errors, &mut warnings) else {
        return Err(NotExecuted { errors, warnings });
    };
    topology_warnings(field, &c, &mut warnings);

    // The unchanged kernel constructors and operators. The checks above remove
    // every condition they assert on; the guard reports anything else verbatim.
    let ran = catch_unwind(AssertUnwindSafe(|| {
        let rel = DeclaredRelations::from_edges_with_provenance(field.nodes.len(), c.edges.clone(), c.classes.clone());
        let x = NodeField::new(c.data.clone());
        let a = operator_a(&x, &rel, &c.pairs);
        let rho = compute_rho(&a, &rel, c.params.rho_base, c.params.chi_0);
        let b = operator_b(&a, &rel);
        let r = operator_r(&b, &rel, &rho, &c.cc);
        (rel, a, rho, b, r)
    }));
    let (rel, a, rho, b, r) = match ran {
        Ok(x) => x,
        Err(p) => return Err(NotExecuted {
            errors: vec![err(ErrorClass::UndefinedOperation, &field.field_id, format!("kernel assertion: {}", panic_text(p)))],
            warnings,
        }),
    };
    for (name, ok) in [("A", finite(&a.spatial) && finite(&a.comp)), ("ρ", rho.iter().all(|v| v.is_finite())),
                       ("B", finite(&b.spatial) && finite(&b.comp)), ("R", finite(&r.spatial) && finite(&r.comp))] {
        if !ok {
            return Err(NotExecuted {
                errors: vec![err(ErrorClass::UndefinedOperation, name, "operator output is not finite (∉ D) for these finite inputs")],
                warnings,
            });
        }
    }

    Ok(SpatialExecution {
        field_id: field.field_id.clone(),
        observation_id: field.observation_id.clone(),
        process_step: field.process_step,
        node_ids: field.nodes.iter().map(|n| n.id.clone()).collect(),
        edge_ids: field.edges.iter().map(|e| e.id.clone()).collect(),
        edge_endpoints: field.edges.iter().map(|e| (e.source.clone(), e.target.clone())).collect(),
        provenance: field.edges.iter().map(|e| e.provenance.clone()).collect(),
        component_pairs: c.pairs,
        cc: c.cc,
        parameters: c.params,
        a, rho, b, r,
        warnings,
        rel,
    })
}

/// Subsequent-observation execution: the spatial kernel for `field`, then the
/// persistence operators against `prior`, the actual output of an earlier
/// executed observation of the same declared field.
///
/// A prior cannot be assembled by hand — only `execute_spatial` produces one:
///
/// ```compile_fail
/// use abr_kernel::diagnostic_exec::SpatialExecution;
/// let fabricated = SpatialExecution { field_id: String::new(), observation_id: String::new(), process_step: 0,
///     node_ids: vec![], edge_ids: vec![], edge_endpoints: vec![], provenance: vec![],
///     component_pairs: vec![], cc: vec![], parameters: todo!(), a: todo!(), rho: vec![],
///     b: todo!(), r: todo!(), warnings: vec![], rel: todo!() };
/// ```
pub fn execute_with_prior(field: &ExperimentalField, prior: &SpatialExecution) -> Result<PersistenceExecution, NotExecuted> {
    // Prior compatibility, checked before anything runs.
    let mut errors = Vec::new();
    let pe = |d: String| err(ErrorClass::IncompatiblePrior, &prior.observation_id, d);
    if prior.observation_id == field.observation_id {
        errors.push(pe(format!("prior and current are the same observation '{}'; persistence requires a declared pair \
                                of sequential observations (kernel_reference.md §8)", field.observation_id)));
    }
    match prior.process_step.checked_add(1) {
        Some(next) if next == field.process_step => {}
        _ => errors.push(pe(format!(
            "prior is declared at process step {}, current at step {}: persistence pairs an observation only with the              immediately preceding declared process step — no reversal, repetition, gap, or wraparound of the              observation sequence (kernel_reference.md §8)", prior.process_step, field.process_step))),
    }
    if prior.field_id != field.field_id {
        errors.push(pe(format!("prior is from field '{}', current is '{}'", prior.field_id, field.field_id)));
    }
    let node_ids: Vec<String> = field.nodes.iter().map(|n| n.id.clone()).collect();
    if prior.node_ids != node_ids {
        errors.push(pe("declared nodes differ from the prior's".into()));
    }
    let edges: Vec<(String, String, String)> = field.edges.iter().map(|e| (e.id.clone(), e.source.clone(), e.target.clone())).collect();
    let prior_edges: Vec<(String, String, String)> = prior.edge_ids.iter().zip(&prior.edge_endpoints)
        .map(|(i, (s, t))| (i.clone(), s.clone(), t.clone())).collect();
    if prior_edges != edges {
        errors.push(pe("declared edges (id, source, target) differ from the prior's: E_prior[e] and E_current[e] would not be the same relation".into()));
    }
    if prior.r.k != field.n_components {
        errors.push(pe(format!("prior has {} components, current declares {}", prior.r.k, field.n_components)));
    }
    let pairs: Vec<(usize, usize)> = field.component_pairs.iter().map(|p| (p.a, p.b)).collect();
    if prior.component_pairs != pairs {
        errors.push(pe("declared component pairs differ from the prior's".into()));
    }
    if !errors.is_empty() {
        return Err(NotExecuted { errors, warnings: vec![] });
    }

    let current = execute_spatial(field)?;
    let mut warnings = current.warnings.clone();
    if prior.parameters.rho_base.to_bits() != current.parameters.rho_base.to_bits()
        || prior.parameters.chi_0.to_bits() != current.parameters.chi_0.to_bits()
    {
        warnings.push(warn(WarningClass::UnresolvedCondition, "parameters",
            format!("ρ_base/χ₀ differ between prior ({}, {}) and current ({}, {}) observation",
                    prior.parameters.rho_base, prior.parameters.chi_0, current.parameters.rho_base, current.parameters.chi_0), None));
    }
    for (e, (p, q)) in prior.provenance.iter().zip(&current.provenance).enumerate() {
        if p != q {
            warnings.push(warn(WarningClass::UnresolvedCondition, &current.edge_ids[e],
                "edge provenance record differs between prior and current observation", None));
        }
    }

    let rel = current.relations();
    let ran = catch_unwind(AssertUnwindSafe(|| {
        let a_p = operator_a_persistence(&current.r, &prior.r);
        let rho_p = compute_rho_persistence(&a_p, rel, current.parameters.rho_base, current.parameters.chi_0);
        let b_p = operator_b_persistence(&a_p, rel);
        let r_p = operator_r_persistence(&b_p, rel, &rho_p);
        (rho_p, PersistenceOutput { a_persistence: a_p, b_persistence: b_p, r_persistence: r_p })
    }));
    let (rho_persistence, persistence) = match ran {
        Ok(x) => x,
        Err(p) => return Err(NotExecuted {
            errors: vec![err(ErrorClass::UndefinedOperation, &field.field_id, format!("kernel assertion: {}", panic_text(p)))],
            warnings,
        }),
    };
    if !(finite(&persistence.a_persistence) && finite(&persistence.b_persistence) && finite(&persistence.r_persistence)
         && rho_persistence.iter().all(|v| v.is_finite()))
    {
        return Err(NotExecuted {
            errors: vec![err(ErrorClass::UndefinedOperation, "persistence", "persistence output is not finite (∉ D)")],
            warnings,
        });
    }
    Ok(PersistenceExecution { prior_observation_id: prior.observation_id.clone(), current, rho_persistence, persistence, warnings })
}
