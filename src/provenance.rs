// provenance.rs — Metatron Dynamics, Inc. Kernel V9.
// RPPI record types: Origin declaration records and Generator calculation
// records (v9_specification.md §3–§5, Req-1 through Req-7).
// Bounded over D. No claim beyond D.
//
// Foundational primitive (Kernel V9): the foundational observable primitive is a
// directed relation declared from observable information. A numerical value is not
// itself a primitive; it is the projection of an observed relational distinction
// through a declared measurement mapping M. Observed relations, measurement mappings
// M, numerical projections in D, and calculated results are held distinct.
//
// ── What this file contains ───────────────────────────────────────────────
//
// Data only. No operator, no projection, no admission decision.
//
//   Declaration records (Origin authors; v9_specification.md §3):
//     ObservableDeclaration, MMappingDeclaration, DeclaredDomain,
//     LocusDeclaration, ObservableRelationDeclaration, RelationalDeclaration,
//     collected in a DeclarationRegistry.
//   Calculation records (Generator authors at execution; §3, §5):
//     CalculationRecord inside ProvenancedOutput.
//
// Working names are the v9_specification.md names and remain subject to
// Verifier confirmation (§11). Field value types are a bounded first
// implementation of the field definitions held OPEN for Origin in §8.
//
// ── Absolute observational traceability (Origin correction, 2026-10-25) ───
//
// Every M, input, parameter, direction, classification, and kernel-selection
// claim must terminate in declared observables. The only terminus is an
// ObservableDeclaration that names an external record (ExternalRecord) and its
// declared measurement conditions. A software value, label, default, fixture
// constant, or internally consistent calculation is never a terminus.
//
// Every field below that would otherwise carry authority of its own is paired
// with an ObservationalBasis. Where the basis is not declared the field is
// None, and admission preserves OPEN: the calculation is not evaluated and not
// admitted. Absence is never a contradiction and is never REJECTED.
//
// A calculation record is evidence to be checked, not a certified account
// (v9_specification.md Correction 1). Its fields are public so that any
// account of execution can be submitted — and independently checked.

use crate::operators::{EdgeProvenance, PrimaryEdgeField};

/// Identifier of a declaration record.
pub type DeclId = String;

// ═══════════════════════════════════════════════════════════════════════════
// ── DECLARATION RECORDS (Origin) ───────────────────────────────────────────
// ═══════════════════════════════════════════════════════════════════════════

/// Epistemic origin of a declared observable.
///
/// Source: v9_specification.md Milestone 3, "Simulated observations": a
/// simulated observation may be admitted when its simulated origin is declared;
/// a valid simulated derivation does not become empirical evidence.
/// SyntheticFixture is the kernel's existing fixture class
/// (kernel_reference.md §17): it tests enforcement and implementation only.
#[derive(Clone, Debug, PartialEq)]
pub enum OriginKind {
    Measured,
    Simulated,
    SyntheticFixture,
}

/// The external record a declared observable is stated to come from.
///
/// Source: observable_provenance_and_reverse_traceability.md — a chain must
/// terminate "in something observed outside the framework, not merely in
/// another step declared inside it". The record carries the attribution; it
/// does not verify it. Verifying the attribution requires comparison with the
/// named source (provenance_demo.rs, Verifier disposition 2026-09-26).
#[derive(Clone, Debug, PartialEq)]
pub struct ExternalRecord {
    /// Named source outside the framework (instrument, dataset, publication).
    pub source: String,
    /// The specific entry within that source.
    pub entry: String,
}

/// Declared observable — the only admissible terminus of a provenance chain.
///
/// Source: Req-1; kernel_reference.md §9 (O → M → D), §7 (the observed
/// quantity enters through M, its legacy interpretation stays out); README
/// contribution discipline item 1 ("what instrument reports it, at what
/// region, under what declared conditions").
#[derive(Clone, Debug, PartialEq)]
pub struct ObservableDeclaration {
    pub id: DeclId,
    pub origin_kind: OriginKind,
    /// None: no external record is declared — not a terminus (OPEN).
    pub record: Option<ExternalRecord>,
    /// None: measurement conditions not declared (OPEN).
    pub declared_conditions: Option<String>,
    /// The value as reported by the record, in `unit`.
    pub reported_value: f64,
    pub unit: String,
}

/// Observational basis and declared correspondence for a declaration element
/// that is not itself an observable.
///
/// Source: kernel_reference.md §8 ("declaration with preserved and discarded
/// invariants stated"); role_separation_and_operator_application.md Step 1
/// (every declared relation traceable to an observable through M).
#[derive(Clone, Debug, PartialEq)]
pub struct ObservationalBasis {
    /// ObservableDeclarations this element is declared from. Must be non-empty.
    pub observables: Vec<DeclId>,
    /// The declared correspondence: what is observed, and what the element
    /// preserves and discards. Must be non-empty.
    pub statement: String,
}

/// Kernel-declared function used inside an M mapping.
///
/// Source: derived_invariants.rs — "The formula hc/λ is part of M at the
/// Atomic Region." The function is called unchanged.
#[derive(Clone, Debug, PartialEq)]
pub enum KernelFunction {
    /// derived_invariants::epsilon_photon_edge — ε[e] = hc/λ.
    EpsilonPhotonEdge,
}

/// Kind of a declared measurement mapping M.
///
/// SUPPORTED SUBSET in this implementation — not the canonical M vocabulary
/// (Verifier correction 3). Identity and DeclaredUnitScale are the
/// transformations kernel_reference.md §8 admits before A or Δ;
/// KernelDeclaredFunction admits an M expression the kernel itself declares
/// as part of M. Every kind still requires an observational basis and a
/// declared correspondence. Any other declared kind is `Other` and is OPEN.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum MMappingKind {
    /// The reported value enters D unchanged in the declared unit.
    Identity,
    /// The reported value is multiplied by a declared scale. The scale is
    /// itself a declared observable (`scale_ref`) — never a bare number.
    DeclaredUnitScale { scale_ref: DeclId },
    /// A kernel-declared function that is part of M. Each declared
    /// measurement reference the function uses is a declared observable.
    KernelDeclaredFunction { function: KernelFunction, references: Vec<DeclId> },
    /// A declared kind outside the supported subset. Not evaluated (OPEN).
    Other { name: String },
}

/// Declared measurement mapping M : O → D for one admitted observable family.
///
/// Source: Req-2; kernel_reference.md §8, §9.
#[derive(Clone, Debug, PartialEq)]
pub struct MMappingDeclaration {
    pub id: DeclId,
    pub kind: MMappingKind,
    /// Unit of the observable as reported.
    pub input_unit: String,
    /// Unit of the projection in D.
    pub output_unit: String,
    /// None: observational basis / correspondence not declared (OPEN).
    pub basis: Option<ObservationalBasis>,
}

/// Declared domain — the subset of D into which an observable is admitted.
///
/// Source: Req-2 (declared_domain); role_separation_and_operator_application.md,
/// M Regional Instantiation Prerequisite (b) observable extent from the
/// domain, and (d) observations outside the extent are not admitted into
/// operator evaluation.
#[derive(Clone, Debug, PartialEq)]
pub struct DeclaredDomain {
    pub id: DeclId,
    /// Extent of the projection in D, inclusive.
    pub floor: f64,
    pub ceiling: f64,
    pub unit: String,
    /// None: the extent's observational basis is not declared (OPEN).
    pub basis: Option<ObservationalBasis>,
}

/// NodeField declaration record for one locus.
///
/// Source: v9_specification.md §4 and §8, "NodeField declaration record":
/// every NodeField value carries the M that projected it, the observable it
/// represents, and the locus at which it was declared.
#[derive(Clone, Debug, PartialEq)]
pub struct LocusDeclaration {
    pub id: DeclId,
    pub observable_ref: DeclId,
    pub m_mapping_ref: DeclId,
    pub domain_ref: DeclId,
}

/// Declaration record for one declared relation (edge).
///
/// Source: v9_specification.md §4 (observable_source, direction_basis,
/// m_mapping_ref, declared_domain); §2 (an EdgeProvenance class is a label,
/// "not a provenance identity").
#[derive(Clone, Debug, PartialEq)]
pub struct ObservableRelationDeclaration {
    pub id: DeclId,
    /// Source locus (declared direction: source → target).
    pub src_locus: DeclId,
    pub tgt_locus: DeclId,
    /// The observation from which the relation is declared through M.
    pub observable_source: DeclId,
    pub m_mapping_ref: DeclId,
    pub declared_domain: DeclId,
    /// Req-3. None: direction basis not declared (OPEN).
    pub direction_basis: Option<ObservationalBasis>,
    /// Frozen V8 class (Continuation / Coupling). None: no class is declared
    /// for this relation (OPEN) — no software default stands in for it.
    pub class: Option<EdgeProvenance>,
    /// The class is a label; its basis must be declared. None: OPEN.
    pub class_basis: Option<ObservationalBasis>,
}

/// Kernel selection declared by Origin.
///
/// Source: kernel_reference.md §10 — the full-operator admissibility
/// condition is structural; "M establishes it; the kernel does not compute it
/// from DeclaredRelations".
#[derive(Clone, Debug, PartialEq)]
pub enum KernelSelection {
    /// Interior locus not established through M: Δ → Σ; B not evaluated.
    Primary,
    /// Interior locus established through M: A → B → R. Outside the supported
    /// scope of this implementation (OPEN).
    Abr,
}

/// Declared relational structure for one calculation.
///
/// Source: v9_specification.md §3 (declaration record: input relations,
/// declared domain); kernel_reference.md §9 (ρ_base, χ₀ M-supplied, no
/// defaults), §10 (kernel selection).
#[derive(Clone, Debug, PartialEq)]
pub struct RelationalDeclaration {
    pub id: DeclId,
    /// Declared loci; position in this list is the node index in D.
    pub loci: Vec<DeclId>,
    /// Declared relations; position in this list is the edge index.
    pub relations: Vec<DeclId>,
    pub kernel_selection: KernelSelection,
    /// None: the selection's observational basis is not declared (OPEN).
    pub selection_basis: Option<ObservationalBasis>,
    /// Declared observables carrying ρ_base and χ₀. None: not declared
    /// through M — operators that require them are not evaluated.
    pub rho_base_ref: Option<DeclId>,
    pub chi_0_ref: Option<DeclId>,
}

/// The Origin declaration registry — the authoritative declarations.
#[derive(Clone, Debug, Default)]
pub struct DeclarationRegistry {
    pub observables: Vec<ObservableDeclaration>,
    pub mappings: Vec<MMappingDeclaration>,
    pub domains: Vec<DeclaredDomain>,
    pub loci: Vec<LocusDeclaration>,
    pub relations: Vec<ObservableRelationDeclaration>,
    pub structures: Vec<RelationalDeclaration>,
}

impl DeclarationRegistry {
    pub fn observable(&self, id: &str) -> Option<&ObservableDeclaration> {
        self.observables.iter().find(|d| d.id == id)
    }
    pub fn mapping(&self, id: &str) -> Option<&MMappingDeclaration> {
        self.mappings.iter().find(|d| d.id == id)
    }
    pub fn domain(&self, id: &str) -> Option<&DeclaredDomain> {
        self.domains.iter().find(|d| d.id == id)
    }
    pub fn locus(&self, id: &str) -> Option<&LocusDeclaration> {
        self.loci.iter().find(|d| d.id == id)
    }
    pub fn relation(&self, id: &str) -> Option<&ObservableRelationDeclaration> {
        self.relations.iter().find(|d| d.id == id)
    }
    pub fn structure(&self, id: &str) -> Option<&RelationalDeclaration> {
        self.structures.iter().find(|d| d.id == id)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ── CALCULATION RECORDS (Generator) ────────────────────────────────────────
// ═══════════════════════════════════════════════════════════════════════════

/// Which declared calculation the record claims to account for.
#[derive(Clone, Debug, PartialEq)]
pub enum Calculation {
    /// Primary kernel over a RelationalDeclaration: Δ, and Σ where ρ_base and
    /// χ₀ are declared through M.
    Primary { structure_ref: DeclId },
    /// A declared M projection attached to one declared relation — the
    /// edge quantity produced by the relation's M from its observable_source
    /// (e.g. ε[e] = hc/λ at a photon edge). Not an operator calculation.
    EdgeMProjection { relation_ref: DeclId },
}

/// One recorded input: what was projected into D, from what, through which M.
///
/// For Calculation::Primary, `binding_ref` is a LocusDeclaration id. For
/// Calculation::EdgeMProjection, `binding_ref` is the relation id and the
/// observable is the relation's observable_source.
#[derive(Clone, Debug, PartialEq)]
pub struct InputRecord {
    pub binding_ref: DeclId,
    pub observable_ref: DeclId,
    /// The source the record claims for that observable.
    pub claimed_source: String,
    /// The observable value the record claims was used.
    pub observed_value: f64,
    pub m_mapping_ref: DeclId,
    /// The projection into D the record claims was produced.
    pub projected_value: f64,
}

/// One recorded relation traversal.
#[derive(Clone, Debug, PartialEq)]
pub struct RelationRecord {
    pub relation_ref: DeclId,
    /// Node indices used for (source, target).
    pub src_index: usize,
    pub tgt_index: usize,
    pub class: EdgeProvenance,
    /// Edge indices Σ read as adj⁺ and adj⁻ for this edge (empty if Σ was
    /// not evaluated).
    pub adj_plus: Vec<usize>,
    pub adj_minus: Vec<usize>,
    /// Edge indices incident to the source locus, over which χ[src(e)] is
    /// selected for ρ[e] (operators.rs compute_rho_primary). Empty if Σ was
    /// not evaluated.
    pub rho_incident: Vec<usize>,
}

/// One operator or M step in the recorded path.
#[derive(Clone, Debug, PartialEq)]
pub enum Step {
    MProjection,
    Delta,
    Sigma,
    A,
    B,
    R,
}

/// One recorded parameter.
#[derive(Clone, Debug, PartialEq)]
pub struct ParameterRecord {
    /// "rho_base" or "chi_0".
    pub name: String,
    pub observable_ref: DeclId,
    pub value: f64,
}

/// One recorded output entry and the declarations that contributed to it.
#[derive(Clone, Debug, PartialEq)]
pub struct OutputEntry {
    pub step: Step,
    pub relation_ref: DeclId,
    pub component: usize,
    pub value: f64,
    pub contributing: Vec<DeclId>,
}

/// The Generator's calculation record (v9_specification.md §3, §5).
#[derive(Clone, Debug, PartialEq)]
pub struct CalculationRecord {
    pub calculation: Calculation,
    pub inputs: Vec<InputRecord>,
    pub relations: Vec<RelationRecord>,
    pub path: Vec<Step>,
    pub parameters: Vec<ParameterRecord>,
    pub outputs: Vec<OutputEntry>,
}

/// The calculated result itself, as produced by the frozen functions.
#[derive(Clone, Debug)]
pub enum CalculatedOutput {
    Primary { delta: PrimaryEdgeField, sigma: Option<PrimaryEdgeField> },
    EdgeQuantity { value: f64 },
}

/// A calculated result carried together with its calculation record
/// (v9_specification.md §5, working name ProvenancedOutput<T>). This
/// implementation uses one closed output type in place of the type parameter.
/// Only a ProvenancedOutput can be submitted for admission; a bare
/// PrimaryEdgeField or EdgeField cannot.
#[derive(Clone, Debug)]
pub struct ProvenancedOutput {
    pub output: CalculatedOutput,
    pub record: CalculationRecord,
}
