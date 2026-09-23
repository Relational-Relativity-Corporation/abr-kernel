// operators.rs — Metatron Dynamics, Inc. V7.1.
// Unified Relational Kernel: Primary (Δ → Σ) and ABR (A → B → R).
//
// Grounding documents (2026-07-28):
//   operators_notation_and_constraint_v10.md
//   abr_operators_plain_v9.md
//   role_separation_and_operator_application_v9.md
//   primary_operators_delta_sigma_v6.md
//   primary_region_formal_interior_v6.md
//
// ── What This File Contains ───────────────────────────────────────────────
//
// Two kernels. One declared relational structure.
//
// Primary kernel  (V6 — unchanged):
//   E_primary = Σ(Δ(x))
//   Used at the Primary Region where persistence is not confirmed and B
//   is not active. Two operators. No path structure assumed.
//
// ABR kernel  (V5 — unchanged):
//   Phase 1 (spatial):     E_spatial = R(B(A(x)), ρ(A(x)))
//   Phase 2 (persistence): A_p → B_p → R_p over edge-valued loci
//   Used when persistence is confirmed (open condition — kernel transition
//   criterion pending Origin redeclaration; see §SUSPENDED below).
//
// ── Foundation ───────────────────────────────────────────────────────────
//
// Within this framework there are no assumptions, only observations.
//
// Every quantity must be traceable to an observable through a declared
// measurement mapping M. A result that cannot be so traced is not a
// constrained result — it is a result about nothing in D.
//
// Admissibility is a provenance condition, not a formal consistency
// condition. A calculation that is formally consistent but not traceable
// to an observable through M is not inadmissible — it is simply not
// about anything in D.
//
// Numerical existence is not observational significance.
//
// ── Domain and Measurement ───────────────────────────────────────────────
//
// D := { x ∈ ℝⁿ | n < ∞, |x[i]| < ∞ ∀ i }
// M : O → D  declared by Origin before any operator acts.
// The kernel acts on M(o) only.
//
// M-declared operator parameters (V7.1):
//   No operator runs until M has declared every value it uses. The kernel
//   states forms only. It holds no numerical value for any operator
//   parameter and supplies no defaults. The values M must declare:
//     ρ_base — dimensionless ceiling on the weight given to the directed
//              imbalance of adjacent relations, relative to a relation's
//              own directed difference. Required wherever ρ is computed.
//     χ₀     — reference in the saturating map ρ = ρ_base · χ / (χ₀ + χ).
//              Carries the dimension of χ as declared through M.
//              Required wherever ρ is computed.
//     cc[p]  — cross-topology coupling for each declared component pair p
//              in R. Required when component pairs are declared; one value
//              per pair.
//   A call without these values is a declaration failure, not an operator
//   result. The function signatures enforce this: none of these values has
//   a default, and the kernel does not choose, bound, or substitute them.
//
// Sequential observation requirement (V7):
//   Phase 1 is admissible on a single declared observation M(o).
//   Phase 2 requires M to declare a sequence {M(o₁), M(o₂), ..., M(oₙ)}
//   where each oₖ is a declared observation at one declared process step
//   and the process step is the system's own declared process step —
//   not an externally imposed increment and not a model-generated value.
//   A model-generated trajectory is not an observable sequence through M
//   unless the model is itself declared as a transducer from observable
//   inputs. Cold start (E_prior = zero) is admissible on the first
//   declared step only. Treating cold start output as relational evolution
//   is inadmissible.
//
// Relational evolution direction (V7):
//   The ordering of {M(o₁), ..., M(oₙ)} has exactly one admissible
//   direction — the direction the process itself moves, traceable to
//   an observable through M. This direction is a property of the process,
//   not of the index. Index order alone is not an admissible direction
//   declaration.
//   Origin must declare the direction by identifying the observable
//   property of the process that establishes which observation is prior
//   and which is current.
//
// ── Relational Direction ─────────────────────────────────────────────────
//
// Every declared relation has exactly one admissible direction — the
// direction traceable to an observable through M. The reverse direction
// requires independent observable provenance. If it does not have it,
// it is inadmissible.
//
// Distinctness axiom (declared):
//   If both (s,t) and (t,s) are independently observed and declared,
//   they are treated as distinct relations with distinct provenance.
//   Symmetry of the declared edge-image signals inadmissible structure
//   unless each direction has independent observable provenance, in which
//   case they are distinct relations, not a symmetric pair.
//
// Ring topology is inadmissible:
//   The closing edge declares a node as its own relational predecessor.
//   Within the declared admissibility conditions, no quantity has itself
//   as a relational predecessor — such a quantity requires solving a
//   fixed-point equation to have a definite value, which is not traceable
//   to an observable through M.
//
// ── Legacy Mathematical Intrusion Warning (V7) ───────────────────────────
//
// The Verifier is required to check Generator output — including all code
// in this file and all calling code — for quantities that entered from the
// Generator's training distribution rather than from Origin's declaration.
//
// The general rule: any mathematical quantity in Generator output that was
// not declared by Origin as traceable through M before generation began is
// a provenance failure. The mathematical framework from which the quantity
// is drawn does not matter. Correctness is not the question. Provenance is.
//
// Common intrusion classes for LLM Generators:
//   Statistical: probability, expected value, variance, correlation,
//     ensemble averages, Bayesian priors. A single declared observable
//     is not an ensemble. Statistics applied without a declared ensemble
//     are inadmissible.
//   Quantum mechanical: wave functions, Hamiltonians, coupling constants,
//     decoherence rates, density matrices. Subject matter being quantum
//     mechanical does not license the formalism — Origin must declare it.
//   Classical mechanical: mass as primitive, force, potential energy,
//     trajectories, phase space entering as assumed primitives.
//   Information-theoretic: entropy, mutual information, KL divergence
//     imported as observables rather than declared projections.
//   Geometric: manifolds, curvature, metric tensors, differential forms
//     assumed as background rather than declared relational structure.
//   Any mathematical quantity not on this list is subject to the same rule.
//
// ── Antisymmetric Expression — Observational Note ────────────────────────
//
// The operator permits either zero or nonzero numerical antisymmetric
// expression; neither outcome is imposed by the mathematics. In
// information analyses mapped to observable data performed to date,
// nonzero numerical antisymmetric expression is not expected. If nonzero
// expression occurs, preserve it exactly and flag it for review. Do not
// suppress it, threshold it away, force it to zero, or automatically
// classify it as either failure or confirmation. Examine the complete
// provenance chain: observable input through M, declared relations,
// operator application, and possible undeclared mathematical intrusion.
// Its significance is determined only after that examination.
//
// R_anti = 0 and R_anti ≠ 0 are both admissible mathematical outcomes.
// R_anti ≠ 0 → REVIEW SIGNAL. Not automatically a failure, inadmissibility,
// discovery, or confirmation of a physical phenomenon.
//
// ── ρ — Provenance Audit (§8 of purge document) ──────────────────────────
//
// The following traces each element of ρ to its declared source.
// Where the chain is complete, the element is marked PRESERVE PENDING
// VERIFIER CONFIRMATION. Where the chain is incomplete, OPEN is stated.
//
// 1. rho_base
//    Code operation: scalar multiplier supplied at the call site.
//    Declared expression: ρ_base in the operator formula.
//    Dimension: none. ρ multiplies a difference of quantities that share
//      the dimension of the relation's own directed difference.
//    Value: none in the kernel. Declared by M for each analysis.
//    Status: M-SUPPLIED (V7.1).
//
// 2. χ[i] = max{ |A(x)[e]| : e incident to i }
//    Code operation: f64::max fold over absolute values of incident edges.
//    Declared expression: selection of the strongest local asymmetry.
//    Origin grounding document: operators_notation_and_constraint_v10.md
//      (referenced; Verifier must confirm χ selection appears explicitly).
//    Input provenance: computed from A(x), which is computed from M(o).
//    Absolute value: declared projection — direction is carried by R and Σ;
//      ρ is declared as scalar gain on directed difference, so direction
//      removal is part of the declared role of ρ.
//    Max selection: selection over incident locus values, not aggregation.
//      No ensemble declared or required.
//    Status: OPEN — ORIGIN DECLARATION REQUIRED for the max selection
//      itself. The admissibility of selecting the maximum over incident
//      edges (rather than any other selection rule) requires explicit
//      declaration in the grounding documents.
//
// 3. Saturating map χ/(χ₀+χ)
//    Code operation: rho_base * chi / (chi_0 + chi)
//    Declared expression: ρ[i] = ρ_base · χ[i] / (χ₀ + χ[i])
//    Input provenance: χ computed from A(x) computed from M(o); χ₀ declared
//      by M in the dimension of χ.
//    V7.1: the fixed "1" previously in the denominator was an undeclared
//      unit choice. It is removed. With χ₀ supplied by M, the map is
//      dimensionless for any declared unit.
//    Dimensional admissibility: CLOSED (V7.1).
//    The saturating form itself: OPEN — ORIGIN DECLARATION REQUIRED.
//
// Overall ρ status: values M-supplied (V7.1). Two OPEN conditions remain:
// the max selection (element 2) and the saturating form (element 3).
//
// ── B Activation Condition ───────────────────────────────────────────────
//
// B is absent from the primary kernel — not an identity operator.
// B activates when persistence is confirmed. The transition criterion
// previously referenced ρ_P — which has been suspended pending Origin
// redeclaration (see §SUSPENDED). Until redeclaration, do not introduce
// B at the Primary Region. The transition threshold remains an open condition.
//
// ── Non-Agency ───────────────────────────────────────────────────────────
//
// The operators do not act, cause, optimize, or enforce. They define
// invariant relational structure over declared relations in D.
// Whatever the workflow commits to, a person committed it.
//
// ── Ring / Torus Prohibition ─────────────────────────────────────────────
//
// A ring or torus assumes four relational behaviors none of which are
// observed in evolving systems: closed boundary, bidirectional symmetry,
// uniform degree, exact recurrence. Any ring, torus, np.roll, or
// periodic-index operation proposed for an application is a DRIFT SIGNAL.
// Refuse it and require declared relations with provenance.
//
// ── §SUSPENDED — Quantities Pending Origin Redeclaration ─────────────────
//
// The following quantities have been removed from the active kernel pending
// Origin redeclaration. They must not be reintroduced, substituted, or
// approximated without an explicit Origin declaration traceable through M.
//
// rank(Im Δ), rank(Im Σ):
//   Previously computed via SVD (nalgebra). SVD is not required to execute
//   Δ, Σ, A, B, R, or persistence. Conventional matrix rank asks about the
//   dimension of a linear vector-space span. Multiple distinct relational
//   outputs can occupy the same conventional linear span. Therefore:
//     relational differentiation ≢ rank(matrix representation).
//   The interpretation of these quantities as conventional linear-algebraic
//   matrix rank is not presently authorized as an Origin declaration.
//   Unresolved Origin question: what observable relational condition was
//   rank(Im Δ) or rank(Im Σ) originally intended to distinguish?
//   That question must be answered from the operators and observable outputs
//   before any replacement quantity is authorized.
//   No surrogate may be substituted: not count, dimensionality, component
//   count, edge count, nonzero-entry count, entropy, variance, state count,
//   or any other conventional measure.
//   Do not substitute zero.
//
// ρ_P (RhoP, RhoPUndefined, rho_p_ratio, im_delta_rank, im_sigma_rank):
//   Depended on rank(Im Σ) and propagation_capacity. Suspended with rank.
//   The component-count ceiling n_components/C_X is also suspended — it
//   was a consequence of the matrix-rank representation:
//     rank(X) ≤ k  →  ρ_P ≤ k/C_X
//   This bound must not be treated as an observed property of the relational
//   system. It must not survive automatically after removal of the
//   matrix-rank representation.
//
// detect_failure_mode, expression_condition:
//   Both depended on rank. Suspended.
//
// FailureMode::DifferentiationCollapse, FailureMode::CirculationCancellation:
//   Both depended on rank(Im Δ) or rank(Im Σ). Suspended.
//   FailureMode::RelationalIsolation depended only on declared adjacency
//   structure; replaced by relational_isolation() predicate, which is preserved.
//
// propagation_capacity():
//   Was used only to compute ρ_P. Suspended with ρ_P.
//
// cc = 0.5 in operator_r:
//   REMOVED (V7.1, Origin declaration). cc is an M-supplied value, one per
//   declared component pair. See "M-declared operator parameters" above.
//
// is_fully_connected(), has_undirected_cycle():
//   Both construct an undirected projection of the declared directed edge set.
//   Their role is diagnostic only — they do not alter operator output or
//   establish relational direction. Moved to the test module pending
//   Origin determination of whether either is required in the kernel proper.
//   They must not establish relational direction, alter operator output,
//   create reverse relations, or provide physical interpretation.
//
// Superposition invariant (I-S in derived_invariants.rs):
//   The formulation rank(Im Δ) > 1 must not remain classified as an
//   established relational definition of superposition while conventional
//   rank is suspended. See §DOWNSTREAM IMPACT in the purge report.
//
// ── Version History ──────────────────────────────────────────────────────
//
// V1–V4: See operators_v5.rs version history.
//
// V5 — Foundation statement added. Admissibility redefined as a provenance
//      condition. Declared relation primitive extended to include relational-
//      evolutionary structure (persistence edges). EdgeProvenance enum added.
//      PersistenceState declared with cold-start condition. Operator formulas
//      preserved; domain of application extended.
//
// V6 — Primary kernel added: E_primary = Σ(Δ(x)) at the Primary Region.
//      Relational direction: directional admissibility condition and
//      distinctness axiom. Ring inadmissibility derived, not asserted.
//      B absence distinguished from B = identity.
//
// V7 — Unified file: primary kernel and ABR kernel in one module.
//      DeclaredRelations unified. NodeField subsumes ObservableField.
//      Sequential observation requirement and relational evolution direction
//      added to header. Legacy mathematical intrusion warning added.
//      All V5 and V6 operator formulas preserved without modification.
//
// V7 (purge, first pass) — SVD, nalgebra, rank(Im Δ), rank(Im Σ), ρ_P,
//      RhoP, RhoPUndefined, im_delta_rank, im_sigma_rank, rho_p_ratio,
//      propagation_capacity, detect_failure_mode, expression_condition,
//      FailureMode::DifferentiationCollapse, FailureMode::CirculationCancellation,
//      frobenius_norm, frobenius_norm_spatial removed or suspended.
//      from_edges() restricted to #[cfg(test)].
//      Undirected graph projections moved to test module.
//      cc = 0.5 flagged OPEN. Antisymmetric expression overclaim corrected.
//
// V7 (purge, second pass) — Antisymmetric expression note revised to full
//      observational form per §6 of purge document. ρ provenance audit
//      added per §8. Superposition invariant downstream impact stated in
//      §SUSPENDED. "Numerical existence is not observational significance"
//      added to foundation. rho_base status corrected from
//      "PRESERVE PENDING PROVENANCE CONFIRMATION" to "OPEN — ORIGIN
//      DECLARATION REQUIRED" — the grounding document is referenced but
//      not confirmed to contain the explicit declaration. Dimensional
//      admissibility of χ in the saturating map added as OPEN condition.
//      Version history updated.
//
// V7.1 — Arbitrary values removed from operator bodies (Origin declaration,
//      2026-09-23). The fixed "1" in the saturating map is replaced by χ₀,
//      and cc = 0.5 in R is replaced by cc[p], one per declared component
//      pair. ρ_base, χ₀, and cc are M-supplied, with no kernel values and no
//      defaults. The M-declared operator parameters requirement is stated in
//      the header. Operator forms otherwise unchanged.
//
// Bounded over D. No claim beyond D.
// Metatron Dynamics, Inc. V7.1.

// ═══════════════════════════════════════════════════════════════════════════
// ── SHARED FOUNDATION ──────────────────────────────────────────────────────
// ═══════════════════════════════════════════════════════════════════════════

// ── Edge Provenance ───────────────────────────────────────────────────────
//
// Provenance is an admissibility condition, not an annotation.
// Every declared edge carries one of three provenances.
// A relation with no declared provenance is fabricated within D.

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeProvenance {
    /// Within-family spatial continuation — declared from observable or
    /// device geometry. Open boundary: no wraparound.
    Continuation,
    /// Cross-family spatial coupling — declared from physical coupling
    /// mechanism (e.g. evanescent backscattering, scaffold connectivity).
    Coupling,
    /// Relational-evolutionary — connects E_prior[e] to E_current[e]
    /// across one declared process step.
    /// Represented in PersistenceState, not in the spatial edge list,
    /// because its loci are edge-valued, not node-valued.
    Persistence,
}

// ── Declared Relational Structure ─────────────────────────────────────────
//
// Unified for both kernels.
//
// Node-indexed adjacency (out / inc):
//   out[i] = edge indices leaving node i     — used by ABR B and R operators
//   inc[i] = edge indices entering node i    — used by ABR B and R operators
//   succ(e) = out[edges[e].1]               — forward continuation of edge e
//   pred(e) = inc[edges[e].0]               — backward continuation of edge e
//
// Edge-indexed adjacency (adj_plus / adj_minus):
//   adj_plus[e]  = edges f where source(f) = target(e)  — used by primary Σ
//   adj_minus[e] = edges f where target(f) = source(e)  — used by primary Σ
//
// adj_plus[e] and succ(e) are identical sets. They are stored separately
// because they serve different access patterns in the two kernels.
//
// Persistence edges are represented in PersistenceState, not here.
// Their loci are edge-valued, not node-valued — adding them to this
// node-indexed structure would fabricate a node identity for them.

#[derive(Clone, Debug)]
pub struct DeclaredRelations {
    pub n_nodes: usize,
    pub edges: Vec<(usize, usize)>,       // spatial edge e = (src, tgt)
    pub provenance: Vec<EdgeProvenance>,  // provenance[e] — one per spatial edge
    pub out: Vec<Vec<usize>>,             // out[i]: edges leaving node i     (ABR)
    pub inc: Vec<Vec<usize>>,             // inc[i]: edges entering node i    (ABR)
    pub adj_plus: Vec<Vec<usize>>,        // adj_plus[e]: edges continuing forward from e (Primary)
    pub adj_minus: Vec<Vec<usize>>,       // adj_minus[e]: edges arriving into source of e (Primary)
}

impl DeclaredRelations {
    /// Production constructor. Requires explicit provenance for every edge.
    /// Provenance is an admissibility condition — it cannot be manufactured
    /// by a convenience constructor. See test_rel_from_edges() in #[cfg(test)]
    /// for the synthetic-only constructor used in mathematical verification tests.
    pub fn from_edges_with_provenance(
        n_nodes: usize,
        edges: Vec<(usize, usize)>,
        provenance: Vec<EdgeProvenance>,
    ) -> Self {
        assert_eq!(edges.len(), provenance.len(),
            "provenance must have one entry per declared edge");
        assert!(edges.iter().all(|&(s, t)| s < n_nodes && t < n_nodes),
            "all edge endpoints must be within declared node range");
        assert!(provenance.iter().all(|p| *p != EdgeProvenance::Persistence),
            "persistence edges are not declared in the spatial edge list; \
             use PersistenceState instead");

        let n_edges = edges.len();

        // Node-indexed adjacency (ABR)
        let mut out = vec![Vec::new(); n_nodes];
        let mut inc = vec![Vec::new(); n_nodes];
        for (e, &(s, t)) in edges.iter().enumerate() {
            out[s].push(e);
            inc[t].push(e);
        }

        // Edge-indexed adjacency (Primary)
        let mut adj_plus  = vec![Vec::new(); n_edges];
        let mut adj_minus = vec![Vec::new(); n_edges];
        for (e, &(_, et)) in edges.iter().enumerate() {
            for (f, &(fs, _)) in edges.iter().enumerate() {
                if fs == et { adj_plus[e].push(f); }
            }
        }
        for (e, &(es, _)) in edges.iter().enumerate() {
            for (f, &(_, ft)) in edges.iter().enumerate() {
                if ft == es { adj_minus[e].push(f); }
            }
        }

        DeclaredRelations { n_nodes, edges, provenance, out, inc, adj_plus, adj_minus }
    }

    #[inline] pub fn n_edges(&self) -> usize { self.edges.len() }

    /// Forward continuation of edge e — edges starting where e ends.
    /// Used by ABR B and R operators. Equivalent to adj_plus[e].
    #[inline] pub fn succ(&self, e: usize) -> &[usize] { &self.out[self.edges[e].1] }

    /// Backward continuation of edge e — edges ending where e starts.
    /// Used by ABR R operator. Equivalent to adj_minus[e].
    #[inline] pub fn pred(&self, e: usize) -> &[usize] { &self.inc[self.edges[e].0] }
}

// ── Node Field (unified) ──────────────────────────────────────────────────
//
// Observable field over declared nodes. Input to both kernels.
// Formerly NodeField (ABR) and ObservableField (Primary) — same structure.
// Unified here under NodeField. n_components = k; n_nodes = n.

#[derive(Clone, Debug)]
pub struct NodeField {
    pub data: Vec<Vec<f64>>,   // data[c][i] — component c, node i
    pub k: usize,              // number of components (n_components)
    pub n: usize,              // number of nodes
}

impl NodeField {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        let k = data.len();
        assert!(k > 0, "at least one component required");
        let n = data[0].len();
        assert!(data.iter().all(|c| c.len() == n),
            "all components must have the same node count");
        assert!(data.iter().all(|c| c.iter().all(|v| v.is_finite())),
            "all observable values must be finite (∈ D)");
        NodeField { data, k, n }
    }

    /// n_components alias — synonym for k.
    #[inline] pub fn n_components(&self) -> usize { self.k }
    /// n_nodes alias — synonym for n.
    #[inline] pub fn n_nodes(&self) -> usize { self.n }
}

// ═══════════════════════════════════════════════════════════════════════════
// ── PRIMARY KERNEL: Δ → Σ ──────────────────────────────────────────────────
// ═══════════════════════════════════════════════════════════════════════════
//
// E_primary = Σ(Δ(x))
//
// Used at the Primary Region — where persistence is not yet confirmed
// and B is not active. Two operators. One composition. No path structure
// assumed. No history carried.
//
// Δ is A under a declared renaming — formula identical.
// Σ is R applied directly to Δ(x) without prior B accumulation.
//
// B is absent — not an identity operator that happens to do nothing.
// It is simply not invoked here.

// ── Primary Edge Field ────────────────────────────────────────────────────
//
// Output of primary kernel operators. Structurally simpler than ABR EdgeField:
// no component pairs, no spatial/comp split. field[c][e] only.

#[derive(Clone, Debug)]
pub struct PrimaryEdgeField {
    pub field: Vec<Vec<f64>>,  // field[c][e]
    pub n_components: usize,
    pub n_edges: usize,
}

impl PrimaryEdgeField {
    pub fn new(field: Vec<Vec<f64>>, n_edges: usize) -> Self {
        let n_components = field.len();
        assert!(n_components > 0, "at least one component required");
        assert!(field.iter().all(|c| c.len() == n_edges),
            "all components must have length n_edges");
        assert!(field.iter().all(|c| c.iter().all(|v| v.is_finite())),
            "all field values must be finite (∈ D)");
        PrimaryEdgeField { field, n_components, n_edges }
    }
}

// ── Operator Δ ────────────────────────────────────────────────────────────
//
// Δ(x)[e] = x[s] − x[t]  for each declared edge e = (s, t).
//
// Directed difference of the observable field across each declared relation.
// Direction determined by the declared observable — not by convention.
//
// Formula identical to A. Role: irreducible primitive of the primary kernel.
//
// Constraint: directed difference only. No relation and no direction may
// be added that the declaration did not trace to an observable through M.
//
// AUDIT (§22):
//   Code operation: x.data[c][s] - x.data[c][t]
//   Declared expression: Δ(x)[e] = x[s] − x[t]
//   Origin source: primary_operators_delta_sigma_v6.md
//   Input provenance: NodeField constructed from M(o)
//   Role: DECLARED — PRESERVE
//   Status: DECLARED — PRESERVE

pub fn operator_delta(x: &NodeField, rel: &DeclaredRelations) -> PrimaryEdgeField {
    assert_eq!(x.n, rel.n_nodes,
        "observable field and declared relations must have the same node count");
    let field = (0..x.k)
        .map(|c| rel.edges.iter()
            .map(|&(s, t)| x.data[c][s] - x.data[c][t])
            .collect::<Vec<f64>>())
        .collect::<Vec<Vec<f64>>>();
    PrimaryEdgeField::new(field, rel.n_edges())
}

// ── ρ (Primary) ───────────────────────────────────────────────────────────
//
// EDGE FORM of ρ — the node-form quantity evaluated at the source locus.
//
// ρ[e] = ρ_base · χ[s] / (χ₀ + χ[s]),  s = source(e)
// χ[s] = max{ |Δ(x)[e']| : e' incident to s }
//
// Derived per NODE from Δ(x); no aggregation beyond the node. Returned per
// EDGE by evaluating the node quantity at source(e).
// Preserves: the source locus strength. Discards: the target locus strength.
// The asymmetry is declared, consistent with the single admissible direction
// of the relation.
// ρ ∈ [0, ρ_base) in both forms.
//
// STATUS — PRESERVE PENDING PROVENANCE CONFIRMATION.
// See ρ provenance audit in the file header (§8 of purge document).
// ρ_base and χ₀ are M-supplied (V7.1). Two OPEN conditions remain: max
// selection declaration and the saturating form.
//
// AUDIT (§22):
//   rho_base:
//     Code operation: scalar multiplier at call site
//     Declared expression: ρ_base
//     Status: M-SUPPLIED (V7.1) — no kernel value
//   χ[s] = max|Δ[e']|:
//     Code operation: f64::max fold over c[e].abs() for incident edges
//     Declared expression: χ[s] = max{ |Δ(x)[e']| : e' incident to s }
//     Origin source: operators_notation_and_constraint_v10.md (referenced)
//     Input provenance: OPEN — max selection rule requires explicit declaration
//     Status: OPEN — ORIGIN DECLARATION REQUIRED
//   χ/(χ₀+χ):
//     Code operation: chi / (chi_0 + chi)
//     Declared expression: χ/(χ₀+χ) saturating map, χ₀ M-supplied
//     Dimensional admissibility: CLOSED (V7.1)
//     Saturating form: OPEN — ORIGIN DECLARATION REQUIRED

// ── M-Declaration Requirement ─────────────────────────────────────────────
//
// Structural checks only. They admit any value M declares that the formula
// can evaluate, and they do not choose, bound, or default any value.
//   χ₀ > 0: at χ₀ = 0 the map is 0/0 wherever χ = 0; at χ₀ < 0 it is
//   singular at χ = −χ₀. Neither is evaluable.

pub fn require_rho_declaration(rho_base: f64, chi_0: f64) {
    assert!(rho_base.is_finite(),
        "ρ_base must be declared by M as a finite value");
    assert!(chi_0.is_finite() && chi_0 > 0.0,
        "χ₀ must be declared by M as a finite positive value in the dimension of χ");
}

pub fn require_cc_declaration(cc: &[f64], pairs: &[(usize, usize)]) {
    assert_eq!(cc.len(), pairs.len(),
        "cc must be declared by M for every declared component pair");
    assert!(cc.iter().all(|v| v.is_finite()),
        "cc values declared by M must be finite");
}

pub fn compute_rho_primary(
    delta_field: &PrimaryEdgeField,
    rel: &DeclaredRelations,
    rho_base: f64,
    chi_0: f64,
) -> Vec<f64> {
    require_rho_declaration(rho_base, chi_0);
    let mut node_incident: Vec<Vec<usize>> = vec![Vec::new(); rel.n_nodes];
    for (e, &(s, t)) in rel.edges.iter().enumerate() {
        node_incident[s].push(e);
        node_incident[t].push(e);
    }
    rel.edges.iter().map(|&(s, _)| {
        let chi = node_incident[s].iter()
            .flat_map(|&e| delta_field.field.iter().map(move |c| c[e].abs()))
            .fold(0.0_f64, f64::max);
        rho_base * chi / (chi_0 + chi)
    }).collect()
}

// ── Operator Σ ────────────────────────────────────────────────────────────
//
// Σ(g)[e] = g[e] + ρ[e] · (Σ_{adj⁺(e)} g[e'] − Σ_{adj⁻(e)} g[e'])
//
// Σ is R applied directly to Δ(x) without prior B accumulation.
// Formula preserved exactly from V5 R. Input condition changed.
//
// adj⁺(e): declared edges whose source = target of e.
// adj⁻(e): declared edges whose target = source of e.
// Adjacency sums are over immediate neighbors only.
//
// The sums inside the operator instantiate the declared local relational
// recurrence — they are not global analytical aggregation.
//
// Antisymmetric expression observational note applies here. See file header.
//
// Constraint: immediate adjacency only. No path accumulation.
//
// AUDIT (§22):
//   Pass-through g[e]:
//     Code operation: delta_field.field[c][e]
//     Declared expression: g[e] in Σ formula
//     Origin source: primary_operators_delta_sigma_v6.md
//     Status: DECLARED — PRESERVE
//   Adjacency sums:
//     Code operation: .iter().map(|&f| ...).sum()
//     Declared expression: Σ_{adj⁺} and Σ_{adj⁻} in Σ formula
//     Origin source: primary_operators_delta_sigma_v6.md
//     Note: sums are part of the declared local relational recurrence,
//       not global analytical aggregation.
//     Status: DECLARED — PRESERVE
//   ρ[e] as scalar gain:
//     Status: OPEN (inherits from ρ audit above)

pub fn operator_sigma(
    delta_field: &PrimaryEdgeField,
    rel: &DeclaredRelations,
    rho_base: f64,
    chi_0: f64,
) -> PrimaryEdgeField {
    assert_eq!(delta_field.n_edges, rel.n_edges(),
        "delta field and declared relations must have the same edge count");
    let rho = compute_rho_primary(delta_field, rel, rho_base, chi_0);
    let field = (0..delta_field.n_components).map(|c| {
        (0..rel.n_edges()).map(|e| {
            let forward: f64 = rel.adj_plus[e].iter()
                .map(|&f| delta_field.field[c][f]).sum();
            let backward: f64 = rel.adj_minus[e].iter()
                .map(|&p| delta_field.field[c][p]).sum();
            delta_field.field[c][e] + rho[e] * (forward - backward)
        }).collect()
    }).collect();
    PrimaryEdgeField::new(field, rel.n_edges())
}

// ── Antisymmetric Term (Primary) ──────────────────────────────────────────
//
// Extracts the antisymmetric term of Σ separately from the pass-through.
//   antisymmetric_term[e] = ρ[e] · (adj⁺ sum − adj⁻ sum)
//
// Declared projection:
//   Preserves: directed adjacency contribution at each declared edge.
//   Discards:  pass-through term g[e].
//
// Antisymmetric expression observational note applies. See file header.
// Zero and nonzero values are both admissible mathematical outcomes.
// Neither is imposed. Neither is suppressed.
//
// AUDIT (§22): Status: DECLARED — PRESERVE (formula); OPEN (ρ component).

pub fn antisymmetric_term(
    delta_field: &PrimaryEdgeField,
    rel: &DeclaredRelations,
    rho_base: f64,
    chi_0: f64,
) -> PrimaryEdgeField {
    assert_eq!(delta_field.n_edges, rel.n_edges());
    let rho = compute_rho_primary(delta_field, rel, rho_base, chi_0);
    let field = (0..delta_field.n_components).map(|c| {
        (0..rel.n_edges()).map(|e| {
            let forward: f64 = rel.adj_plus[e].iter()
                .map(|&f| delta_field.field[c][f]).sum();
            let backward: f64 = rel.adj_minus[e].iter()
                .map(|&p| delta_field.field[c][p]).sum();
            rho[e] * (forward - backward)
        }).collect()
    }).collect();
    PrimaryEdgeField::new(field, rel.n_edges())
}

// ── Primary Kernel ────────────────────────────────────────────────────────
//
// E_primary(x) = Σ(Δ(x))
// Returns (delta_field, sigma_field). Both preserved for analysis.
//
// AUDIT (§22): Status: DECLARED — PRESERVE (composition of Δ and Σ).

pub fn operator_e_primary(
    x: &NodeField,
    rel: &DeclaredRelations,
    rho_base: f64,
    chi_0: f64,
) -> (PrimaryEdgeField, PrimaryEdgeField) {
    let delta = operator_delta(x, rel);
    let sigma = operator_sigma(&delta, rel, rho_base, chi_0);
    (delta, sigma)
}

// ── Relational Isolation Check ────────────────────────────────────────────
//
// All declared edges are isolated — every edge has empty adj⁺ and adj⁻.
// When this holds, the antisymmetric term of Σ is structurally zero for
// any observable field. This is a structural property of the declared
// adjacency, not a numerical test.
//
// AUDIT (§22):
//   Code operation: .all(|(p, m)| p.is_empty() && m.is_empty())
//   Declared expression: structural adjacency predicate
//   Origin source: primary_operators_delta_sigma_v6.md (FM2)
//   Status: DECLARED — PRESERVE

pub fn relational_isolation(rel: &DeclaredRelations) -> bool {
    rel.adj_plus.iter().zip(rel.adj_minus.iter())
        .all(|(p, m)| p.is_empty() && m.is_empty())
}

// ── Declared Edge-Image Admissibility Check ───────────────────────────────
//
// An admissible declared edge-image is ASYMMETRIC: no declared edge vector
// has its negation as another declared edge vector.
// Returns (is_admissible, inadmissibility_witness).
//
// IMPLEMENTATION-ONLY NUMERICAL TOLERANCE: `tol` guards finite-precision
// arithmetic in the negation comparison. It is not a property of the
// observed system and must not be reported as one. It is separate from
// any measurement-provenance tolerance.
//
// AUDIT (§22):
//   Status: DECLARED — PRESERVE. Negation test follows from the distinctness
//   axiom in the relational direction declaration.

pub fn declared_edge_image_admissibility_check(
    delta_field: &PrimaryEdgeField,
    tol: f64,
) -> (bool, Option<Vec<f64>>) {
    if delta_field.n_edges == 0 { return (true, None); }
    let rows: Vec<Vec<f64>> = (0..delta_field.n_edges)
        .map(|e| (0..delta_field.n_components)
            .map(|c| delta_field.field[c][e]).collect())
        .collect();
    for row in &rows {
        let neg: Vec<f64> = row.iter().map(|v| -v).collect();
        let found = rows.iter().any(|other| {
            other.iter().zip(neg.iter()).all(|(a, b)| (a - b).abs() < tol)
        });
        if found {
            return (false, Some(row.clone()));
        }
    }
    (true, None)
}

// ═══════════════════════════════════════════════════════════════════════════
// ── ABR KERNEL: A → B → R ──────────────────────────────────────────────────
// ═══════════════════════════════════════════════════════════════════════════
//
// Phase 1 (spatial):     E_spatial = R(B(A(x)), ρ(A(x)))
// Phase 2 (persistence): A_p → B_p → R_p over edge-valued loci
//
// All formulas preserved exactly from V5.

// ── ABR Edge Field ────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct EdgeField {
    pub spatial: Vec<Vec<f64>>,          // spatial[c][e]
    pub comp: Vec<Vec<f64>>,             // comp[p][i]
    pub comp_pairs: Vec<(usize, usize)>, // declared component relations
    pub k: usize,
}

impl EdgeField {
    /// Zero EdgeField — declared cold-start prior state.
    /// All spatial values = 0.0. Represents no relational history.
    /// Admissible on the first declared step only.
    pub fn zero(k: usize, n_edges: usize, comp_pairs: &[(usize, usize)], n_nodes: usize) -> Self {
        EdgeField {
            spatial: vec![vec![0.0; n_edges]; k],
            comp: vec![vec![0.0; n_nodes]; comp_pairs.len()],
            comp_pairs: comp_pairs.to_vec(),
            k,
        }
    }
}

// ── Persistence State ─────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct PersistenceState {
    pub e_prior: EdgeField,
    pub is_cold_start: bool,
}

impl PersistenceState {
    pub fn cold_start(k: usize, n_edges: usize, comp_pairs: &[(usize, usize)], n_nodes: usize) -> Self {
        PersistenceState {
            e_prior: EdgeField::zero(k, n_edges, comp_pairs, n_nodes),
            is_cold_start: true,
        }
    }

    /// Prior state from a completed declared process step's kernel output.
    /// Must come from an actual observation — not a model, parameter value,
    /// or static configuration.
    pub fn from_prior(e_prior: EdgeField) -> Self {
        PersistenceState { e_prior, is_cold_start: false }
    }
}

// ── Persistence Output ────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct PersistenceOutput {
    pub a_persistence: Vec<Vec<f64>>,  // [c][e] — persistence directed difference
    pub b_persistence: Vec<Vec<f64>>,  // [c][e] — accumulated
    pub r_persistence: Vec<Vec<f64>>,  // [c][e] — relational evolution field
}

// ── Operator A ────────────────────────────────────────────────────────────
//
// A(x)[e] = x[s] − x[t] over each declared spatial edge.
// NodeField → EdgeField.
// Constraint: directed difference only. No relation added beyond declared.
//
// AUDIT (§22): Status: DECLARED — PRESERVE.

pub fn operator_a(f: &NodeField, rel: &DeclaredRelations, pairs: &[(usize, usize)]) -> EdgeField {
    let spatial = (0..f.k)
        .map(|c| rel.edges.iter().map(|&(s, d)| f.data[c][s] - f.data[c][d]).collect())
        .collect();
    let comp = pairs
        .iter()
        .map(|&(a, b)| (0..f.n).map(|i| f.data[a][i] - f.data[b][i]).collect())
        .collect();
    EdgeField { spatial, comp, comp_pairs: pairs.to_vec(), k: f.k }
}

// ── Operator B ────────────────────────────────────────────────────────────
//
// B(g)[e] = g[e] + Σ_{f ∈ succ(e)} g[f], same direction only.
// Terminal edges accumulate nothing. No wraparound. B is absent from
// the primary kernel — it activates when persistence is confirmed.
//
// AUDIT (§22): Status: DECLARED — PRESERVE.

pub fn operator_b(g: &EdgeField, rel: &DeclaredRelations) -> EdgeField {
    let spatial = g.spatial.iter().map(|s| {
        (0..rel.n_edges())
            .map(|e| s[e] + rel.succ(e).iter().map(|&f| s[f]).sum::<f64>())
            .collect()
    }).collect();
    let comp = g.comp.iter().map(|c| {
        (0..rel.n_nodes)
            .map(|i| c[i] + rel.out[i].iter().map(|&e| c[rel.edges[e].1]).sum::<f64>())
            .collect()
    }).collect();
    EdgeField { spatial, comp, comp_pairs: g.comp_pairs.clone(), k: g.k }
}

// ── ρ (ABR) ───────────────────────────────────────────────────────────────
//
// ρ[i] = rho_base × χ[i] / (χ₀ + χ[i]),  ρ_base and χ₀ M-supplied
// χ[i] = selection over declared asymmetries incident on i.
// Selection, not statistical reduction — no ensemble declared or required.
// Preserves: magnitude of the single strongest declared asymmetry at i.
// Discards:  direction (absolute value) and all weaker incident asymmetries.
//
// STATUS — values M-supplied (V7.1). See ρ provenance audit in the file
// header. Two OPEN conditions remain: max selection and saturating form.
//
// AUDIT (§22): ρ_base, χ₀: M-SUPPLIED. Max selection, saturating form:
//   OPEN — ORIGIN DECLARATION REQUIRED.

pub fn compute_rho(a: &EdgeField, rel: &DeclaredRelations, rho_base: f64, chi_0: f64) -> Vec<f64> {
    require_rho_declaration(rho_base, chi_0);
    (0..rel.n_nodes).map(|i| {
        let mut chi = 0.0_f64;
        for &e in rel.out[i].iter().chain(rel.inc[i].iter()) {
            for c in &a.spatial { chi = chi.max(c[e].abs()); }
        }
        for c in &a.comp { chi = chi.max(c[i].abs()); }
        rho_base * chi / (chi_0 + chi)
    }).collect()
}

// ── Operator R ────────────────────────────────────────────────────────────
//
// R(g)[e] = g[e] + ρ[src(e)] × (Σ_succ B(g) − Σ_pred B(g))
// Cross-topology: spatial edges receive component-edge asymmetry;
// component edges receive spatial-edge asymmetry. Local, antisymmetric, additive.
//
// Antisymmetric expression observational note applies. See file header.
// R_anti = 0 and R_anti ≠ 0 are both admissible mathematical outcomes.
// R_anti ≠ 0 → REVIEW SIGNAL.
//
// cc[p]: M-supplied, one value per declared component pair (V7.1).
//   With no component pairs declared, cc is empty and the cross-topology
//   term does not execute.
//
// AUDIT (§22):
//   Spatial pass-through and directed sum: DECLARED — PRESERVE
//   ρ[src(e)] as scalar gain: values M-supplied; see ρ audit
//   cc[p] cross-topology term: M-SUPPLIED (V7.1) — no kernel value

pub fn operator_r(bg: &EdgeField, rel: &DeclaredRelations, rho: &[f64], cc: &[f64]) -> EdgeField {
    let k = bg.k;
    let pairs = &bg.comp_pairs;
    require_cc_declaration(cc, pairs);
    let rho_e: Vec<f64> = rel.edges.iter().map(|&(s, _)| rho[s]).collect();

    let mut spatial: Vec<Vec<f64>> = (0..k).map(|c| {
        (0..rel.n_edges()).map(|e| {
            let fwd: f64 = rel.succ(e).iter().map(|&f| bg.spatial[c][f]).sum();
            let bwd: f64 = rel.pred(e).iter().map(|&p| bg.spatial[c][p]).sum();
            bg.spatial[c][e] + rho_e[e] * (fwd - bwd)
        }).collect()
    }).collect();

    // cc[p] declared by M. Executes only when component pairs are declared.
    for (p, &(a, b)) in pairs.iter().enumerate() {
        for (e, &(s, d)) in rel.edges.iter().enumerate() {
            let asym = bg.comp[p][d] - bg.comp[p][s];
            spatial[a][e] += rho_e[e] * cc[p] * asym;
            spatial[b][e] -= rho_e[e] * cc[p] * asym;
        }
    }

    let mut comp = bg.comp.clone();
    for (p, &(a, b)) in pairs.iter().enumerate() {
        for i in 0..rel.n_nodes {
            let net_a: f64 = rel.out[i].iter().map(|&e| bg.spatial[a][e]).sum();
            let net_b: f64 = rel.out[i].iter().map(|&e| bg.spatial[b][e]).sum();
            comp[p][i] += rho[i] * (net_a - net_b);
        }
    }
    EdgeField { spatial, comp, comp_pairs: pairs.clone(), k }
}

// ── Operator E (V4 spatial kernel, unchanged) ─────────────────────────────
//
// E(x, ρ) = R(B(A(x)), ρ(A(x))). Spatial kernel only.
//
// AUDIT (§22): Status: DECLARED — PRESERVE (composition).

pub fn operator_e(
    f: &NodeField,
    rel: &DeclaredRelations,
    pairs: &[(usize, usize)],
    cc: &[f64],
    rho_base: f64,
    chi_0: f64,
) -> EdgeField {
    let a = operator_a(f, rel, pairs);
    let rho = compute_rho(&a, rel, rho_base, chi_0);
    let b = operator_b(&a, rel);
    operator_r(&b, rel, &rho, cc)
}

// ── A_persistence ─────────────────────────────────────────────────────────
//
// A_persistence[c][e] = E_current.spatial[c][e] − E_prior.spatial[c][e]
//
// Directed difference between current and prior kernel output.
// Direction is fixed: E_current − E_prior.
// The reverse direction is not traceable to an observable through M.
//
// On cold start (E_prior = zero): A_persistence = E_current.
// This is admissible as the first declared step only. It does not
// constitute evidence of relational evolution.
//
// A_persistence over a stable observable (E_current = E_prior) produces
// zero — the correct declared result.
//
// AUDIT (§22): Status: DECLARED — PRESERVE.

pub fn operator_a_persistence(
    e_current: &EdgeField,
    e_prior: &EdgeField,
) -> Vec<Vec<f64>> {
    assert_eq!(e_current.k, e_prior.k,
        "E_current and E_prior must have the same number of components");
    assert!(e_current.spatial.iter().zip(e_prior.spatial.iter())
        .all(|(c, p)| c.len() == p.len()),
        "E_current and E_prior must have the same edge count");
    e_current.spatial.iter()
        .zip(e_prior.spatial.iter())
        .map(|(curr, prev)| {
            curr.iter().zip(prev.iter())
                .map(|(c, p)| c - p)
                .collect()
        })
        .collect()
}

// ── ρ_persistence ─────────────────────────────────────────────────────────
//
// Same formula as compute_rho, applied to the persistence directed difference.
// ρ_base and χ₀ M-supplied (V7.1). Max selection and saturating form OPEN
// (inherits from ρ audit).

pub fn compute_rho_persistence(
    a_persistence: &[Vec<f64>],
    rel: &DeclaredRelations,
    rho_base: f64,
    chi_0: f64,
) -> Vec<f64> {
    require_rho_declaration(rho_base, chi_0);
    (0..rel.n_nodes).map(|i| {
        let mut chi = 0.0_f64;
        for &e in rel.out[i].iter().chain(rel.inc[i].iter()) {
            for c in a_persistence { chi = chi.max(c[e].abs()); }
        }
        rho_base * chi / (chi_0 + chi)
    }).collect()
}

// ── B_persistence ─────────────────────────────────────────────────────────
//
// B_persistence[c][e] = A_persistence[c][e] + Σ_{f ∈ succ(e)} A_persistence[c][f]
// Terminal edges accumulate nothing. No wraparound.
//
// AUDIT (§22): Status: DECLARED — PRESERVE.

pub fn operator_b_persistence(
    a_persistence: &[Vec<f64>],
    rel: &DeclaredRelations,
) -> Vec<Vec<f64>> {
    a_persistence.iter().map(|c| {
        (0..rel.n_edges())
            .map(|e| c[e] + rel.succ(e).iter().map(|&f| c[f]).sum::<f64>())
            .collect()
    }).collect()
}

// ── R_persistence ─────────────────────────────────────────────────────────
//
// R_persistence[c][e] = B_persistence[c][e] + ρ_p[src(e)] × (fwd − bwd)
// Spatial component only — no component pairs in the persistence domain.
// Antisymmetric expression observational note applies. See file header.
//
// AUDIT (§22): Status: DECLARED — PRESERVE (formula); OPEN (ρ component).

pub fn operator_r_persistence(
    b_persistence: &[Vec<f64>],
    rel: &DeclaredRelations,
    rho_persistence: &[f64],
) -> Vec<Vec<f64>> {
    let rho_e: Vec<f64> = rel.edges.iter()
        .map(|&(s, _)| rho_persistence[s])
        .collect();
    b_persistence.iter().map(|c| {
        (0..rel.n_edges()).map(|e| {
            let fwd: f64 = rel.succ(e).iter().map(|&f| c[f]).sum();
            let bwd: f64 = rel.pred(e).iter().map(|&p| c[p]).sum();
            c[e] + rho_e[e] * (fwd - bwd)
        }).collect()
    }).collect()
}

// ── Operator E V5 (full kernel: spatial + persistence) ────────────────────
//
// Phase 1 (spatial): E_spatial = R(B(A(x)), ρ(A(x)))  [V4, unchanged]
// Phase 2 (persistence): A_p → B_p → R_p over edge-valued loci [V5]
//
// Cold start output is the declared first step only — not evidence of
// relational evolution.
//
// AUDIT (§22): Status: DECLARED — PRESERVE (composition).

pub fn operator_e_v5(
    f: &NodeField,
    rel: &DeclaredRelations,
    pairs: &[(usize, usize)],
    cc: &[f64],
    persistence_state: &PersistenceState,
    rho_base: f64,
    chi_0: f64,
) -> (EdgeField, PersistenceOutput) {
    let e_spatial = operator_e(f, rel, pairs, cc, rho_base, chi_0);
    let a_p = operator_a_persistence(&e_spatial, &persistence_state.e_prior);
    let rho_persistence = compute_rho_persistence(&a_p, rel, rho_base, chi_0);
    let b_p = operator_b_persistence(&a_p, rel);
    let r_p = operator_r_persistence(&b_p, rel, &rho_persistence);
    (e_spatial, PersistenceOutput {
        a_persistence: a_p,
        b_persistence: b_p,
        r_persistence: r_p,
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// ── TESTS ──────────────────────────────────────────────════════════════════
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── Synthetic test fixture declaration ────────────────────────────────
    //
    // Observable values, relational direction, relational provenance, and
    // measurement mapping M supplied to synthetic fixtures below are treated
    // as Origin-declared inputs for purposes of operator implementation
    // verification. These tests do not independently establish physical
    // provenance. They verify what the declared mathematics produces once
    // admissible information crosses the declared M boundary.
    //
    // Synthetic fixtures assign EdgeProvenance::Continuation to all edges.
    // This is an assumed Origin-derived fixture used for
    // mathematical/implementation verification only. Synthetic test
    // provenance is not empirical provenance.
    //
    // Test classification key (per §23 of purge document):
    //   [IMPL]  — implementation verification: tests that the code executes
    //             the declared formula without error.
    //   [MATH]  — mathematical verification: tests a declared mathematical
    //             property of the operator formulas.
    //   [REPR]  — forward reproducibility / determinism: two independent
    //             forward applications of the same declared chain from the
    //             same declared inputs must agree. Establishes determinism,
    //             not provenance.
    //   [OBS]   — observable validation: drives the operator chain from a
    //             declared M-mapped observable value and checks the result
    //             against a declared measurement source.
    //   [CORR]  — correspondence test: checks operator output against a
    //             declared value from the validation record.

    // Fixture declaration of χ₀ for synthetic tests. It reproduces the
    // pre-V7.1 behavior of these fixtures so their expected values carry
    // over unchanged. It is a test-fixture declaration, not a kernel value.
    const FIXTURE_CHI_0: f64 = 1.0;

    fn test_rel_from_edges(n_nodes: usize, edges: Vec<(usize, usize)>) -> DeclaredRelations {
        let n = edges.len();
        DeclaredRelations::from_edges_with_provenance(
            n_nodes, edges,
            vec![EdgeProvenance::Continuation; n],
        )
    }

    fn open_dag() -> DeclaredRelations {
        test_rel_from_edges(4, vec![(0,1),(0,2),(1,3),(2,3)])
    }

    fn directed_chain(n: usize) -> DeclaredRelations {
        let edges: Vec<(usize,usize)> = (0..n-1).map(|i| (i, i+1)).collect();
        test_rel_from_edges(n, edges)
    }

    fn open_chain(n: usize) -> DeclaredRelations { directed_chain(n) }

    fn gradient_field(n: usize) -> NodeField {
        NodeField::new(vec![(0..n).map(|i| (n - 1 - i) as f64).collect()])
    }

    fn uniform_field(n: usize, v: f64) -> NodeField {
        NodeField::new(vec![vec![v; n]])
    }

    fn gradient_field_abr(k: usize, n: usize) -> NodeField {
        let data = (0..k)
            .map(|c| (0..n).map(|i| ((n-1-i) * (c+1)) as f64).collect())
            .collect();
        NodeField::new(data)
    }

    // ── Undirected graph diagnostics (test-only, DIAGNOSTIC ONLY) ─────────
    //
    // These functions construct an undirected projection of the declared
    // directed edge set for ring-detection diagnostics only.
    // They do not alter operator output, establish relational direction,
    // create reverse relations, or provide physical interpretation.
    // Classification: DIAGNOSTIC ONLY — ISOLATE.

    fn has_undirected_cycle(rel: &DeclaredRelations) -> bool {
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); rel.n_nodes];
        for &(s, t) in &rel.edges {
            if s != t && !adj[s].contains(&t) { adj[s].push(t); }
            if s != t && !adj[t].contains(&s) { adj[t].push(s); }
        }
        let mut visited = vec![false; rel.n_nodes];
        fn dfs(
            n: usize, parent: Option<usize>,
            adj: &[Vec<usize>], vis: &mut Vec<bool>,
        ) -> bool {
            vis[n] = true;
            for &m in &adj[n] {
                if Some(m) == parent { continue; }
                if vis[m] { return true; }
                if dfs(m, Some(n), adj, vis) { return true; }
            }
            false
        }
        for start in 0..rel.n_nodes {
            if !visited[start] && dfs(start, None, &adj, &mut visited) {
                return true;
            }
        }
        false
    }

    // ── DeclaredRelations tests ───────────────────────────────────────────

    // [MATH] adj_plus[e] and succ(e) are the same set; adj_minus[e] and pred(e)
    // are the same set. Stored separately for access-pattern reasons.
    #[test]
    fn declared_relations_unified_adjacency() {
        let rel = open_dag();
        for e in 0..rel.n_edges() {
            let mut adj_p = rel.adj_plus[e].clone(); adj_p.sort();
            let mut succ_s = rel.succ(e).to_vec(); succ_s.sort();
            assert_eq!(adj_p, succ_s, "adj_plus[{e}] must equal succ({e})");
            let mut adj_m = rel.adj_minus[e].clone(); adj_m.sort();
            let mut pred_s = rel.pred(e).to_vec(); pred_s.sort();
            assert_eq!(adj_m, pred_s, "adj_minus[{e}] must equal pred({e})");
        }
    }

    // [IMPL] Ring detection diagnostic — synthetic fixture only.
    #[test]
    fn closed_ring_detected_as_undirected_cycle() {
        let rel = test_rel_from_edges(4, vec![(0,1),(1,2),(2,3),(3,0)]);
        assert!(has_undirected_cycle(&rel));
    }

    // [MATH] Open directed chain has no undirected cycle.
    #[test]
    fn directed_chain_no_undirected_cycle() {
        let rel = directed_chain(4);
        assert!(!has_undirected_cycle(&rel));
    }

    // [IMPL] NodeField k/n accessors agree.
    #[test]
    fn node_field_unifies_observable_field() {
        let f = gradient_field(4);
        assert_eq!(f.k, f.n_components());
        assert_eq!(f.n, f.n_nodes());
    }

    // ── Primary kernel tests ──────────────────────────────────────────────

    // [MATH] Δ[(0,1)] = x[0] − x[1]. Exact arithmetic.
    #[test]
    fn primary_delta_directed_difference() {
        let rel = test_rel_from_edges(2, vec![(0,1)]);
        let x = NodeField::new(vec![vec![3.0, 1.0]]);
        let d = operator_delta(&x, &rel);
        // IMPLEMENTATION-ONLY NUMERICAL TOLERANCE: 1e-12
        assert!((d.field[0][0] - 2.0).abs() < 1e-12, "Δ[(0,1)] = x[0] - x[1] = 2.0");
    }

    // [MATH] Declaring (0,1) and (1,0) on the same field produces exact negations.
    // Direction is structural, not a convention.
    #[test]
    fn primary_delta_reverse_is_negation() {
        let rel_fwd = test_rel_from_edges(2, vec![(0,1)]);
        let rel_rev = test_rel_from_edges(2, vec![(1,0)]);
        let x = NodeField::new(vec![vec![3.0, 1.0]]);
        let d_fwd = operator_delta(&x, &rel_fwd);
        let d_rev = operator_delta(&x, &rel_rev);
        assert!((d_fwd.field[0][0] + d_rev.field[0][0]).abs() < 1e-12,
            "forward and reverse must be exact negations — direction is structural");
    }

    // [MATH] For an isolated edge (no adjacency), Σ = Δ exactly.
    #[test]
    fn primary_sigma_equals_delta_for_isolated_edge() {
        let rel = test_rel_from_edges(2, vec![(0,1)]);
        let x = NodeField::new(vec![vec![1.0, 0.0]]);
        let (d, s) = operator_e_primary(&x, &rel, 0.2, FIXTURE_CHI_0);
        assert!((s.field[0][0] - d.field[0][0]).abs() < 1e-12,
            "Σ must equal Δ for isolated edge (no adjacency)");
    }

    // [MATH] B is absent from the primary kernel. Single edge: adj⁺ = adj⁻ = ∅.
    // relational_isolation() = true. Σ = Δ.
    #[test]
    fn primary_b_absent_no_accumulation() {
        let rel = test_rel_from_edges(2, vec![(0,1)]);
        assert!(relational_isolation(&rel), "single edge: must be relationally isolated");
        let x = NodeField::new(vec![vec![2.0, 1.0]]);
        let (d, s) = operator_e_primary(&x, &rel, 0.2, FIXTURE_CHI_0);
        assert!((d.field[0][0] - 1.0).abs() < 1e-12, "Δ[(0,1)] = 1.0");
        assert!((s.field[0][0] - 1.0).abs() < 1e-12, "Σ = Δ — B absent, not identity");
    }

    // [MATH] Uniform field: Δ = 0 everywhere regardless of topology.
    // The antisymmetric expression is zero as a field effect, not a structural one.
    // This is not a failure mode — it is the declared mathematical result.
    #[test]
    fn primary_delta_zero_on_uniform_field() {
        let rel = open_dag();
        let x = uniform_field(4, 1.0);
        let (d, _s) = operator_e_primary(&x, &rel, 0.2, FIXTURE_CHI_0);
        assert!(d.field[0].iter().all(|&v| v == 0.0),
            "uniform field: all Δ values must be zero");
    }

    // [MATH] Non-uniform field: Δ is not uniformly zero.
    #[test]
    fn primary_delta_nonzero_on_gradient_field() {
        let rel = open_dag();
        let x = gradient_field(4);
        let (d, _s) = operator_e_primary(&x, &rel, 0.2, FIXTURE_CHI_0);
        assert!(d.field[0].iter().any(|&v| v != 0.0),
            "gradient field: at least one Δ value must be nonzero");
    }

    // [MATH] Directed chain with gradient: declared edge-image is admissible.
    // IMPLEMENTATION-ONLY NUMERICAL TOLERANCE: 1e-10 in negation comparison.
    #[test]
    fn primary_declared_edge_image_admissibility_directed_chain() {
        let rel = directed_chain(4);
        let x = gradient_field(4);
        let d = operator_delta(&x, &rel);
        let (admissible, _) = declared_edge_image_admissibility_check(&d, 1e-10);
        assert!(admissible, "directed chain with gradient must be admissible");
    }

    // [MATH] Persistence direction is fixed: E_current − E_prior.
    // Swapping arguments produces the exact negation — direction is structural.
    #[test]
    fn evolution_direction_is_current_minus_prior() {
        let rel = directed_chain(3);
        let x_prior   = NodeField::new(vec![vec![1.0, 0.5, 0.0]]);
        let x_current = NodeField::new(vec![vec![2.0, 1.0, 0.5]]);
        let e_prior   = operator_delta(&x_prior,   &rel);
        let e_current = operator_delta(&x_current, &rel);
        let forward: Vec<f64> = e_current.field[0].iter()
            .zip(e_prior.field[0].iter()).map(|(c, p)| c - p).collect();
        let reverse: Vec<f64> = e_prior.field[0].iter()
            .zip(e_current.field[0].iter()).map(|(p, c)| p - c).collect();
        for (f, r) in forward.iter().zip(reverse.iter()) {
            assert!((f + r).abs() < 1e-12, "forward and reverse must be exact negations");
        }
        assert!(forward.iter().any(|&v| v.abs() > 1e-12),
            "forward direction must be non-zero when states differ");
    }

    // [MATH] Stable observable: A_persistence = 0. Correct declared result.
    #[test]
    fn evolution_direction_zero_on_stable_observable() {
        let rel = directed_chain(3);
        let x = NodeField::new(vec![vec![1.0, 0.5, 0.0]]);
        let e = operator_delta(&x, &rel);
        let stable: Vec<f64> = e.field[0].iter()
            .zip(e.field[0].iter()).map(|(c, p)| c - p).collect();
        assert!(stable.iter().all(|&v| v.abs() < 1e-12),
            "stable observable: A_persistence = 0 — correct declared result");
    }

    // ── ABR kernel tests ──────────────────────────────────────────────────

    // [MATH] A[e] = x[s] − x[t] for each declared edge.
    #[test]
    fn abr_a_directed_difference() {
        let rel = directed_chain(3);
        let x = gradient_field_abr(1, 3);
        let a = operator_a(&x, &rel, &[]);
        for (e, &(s, t)) in rel.edges.iter().enumerate() {
            let expected = x.data[0][s] - x.data[0][t];
            assert!((a.spatial[0][e] - expected).abs() < 1e-12,
                "A[{}] must equal x[{}] - x[{}]", e, s, t);
        }
    }

    // [MATH] Terminal edge of chain: B = A (no accumulation, no wraparound).
    #[test]
    fn abr_b_terminal_no_accumulation() {
        let rel = directed_chain(4);
        let x = gradient_field_abr(1, 4);
        let a = operator_a(&x, &rel, &[]);
        let b = operator_b(&a, &rel);
        let last = rel.n_edges() - 1;
        assert!(rel.succ(last).is_empty(), "last edge must have no successor");
        assert!((b.spatial[0][last] - a.spatial[0][last]).abs() < 1e-12,
            "terminal edge: B must equal A (no accumulation)");
    }

    // [IMPL] E V4 produces finite output on declared DAG.
    // Note: this test exercises the cc = 0.5 path only if pairs is non-empty.
    // With empty pairs, the cross-topology block does not execute.
    #[test]
    fn abr_e_v4_finite() {
        let rel = open_dag();
        let x = gradient_field_abr(1, 4);
        let e = operator_e(&x, &rel, &[], &[], 0.3, FIXTURE_CHI_0);
        // Empty pairs — cc = 0.5 path not exercised.
        assert!(e.spatial[0].iter().all(|v| v.is_finite()));
    }

    // [IMPL] cold_start sets is_cold_start and e_prior to zero.
    #[test]
    fn abr_cold_start_flag() {
        let rel = directed_chain(3);
        let ps = PersistenceState::cold_start(1, rel.n_edges(), &[], rel.n_nodes);
        assert!(ps.is_cold_start, "cold start must be flagged");
        assert!(ps.e_prior.spatial[0].iter().all(|&v| v == 0.0),
            "cold start E_prior must be zero");
    }

    // [IMPL] from_prior does not set is_cold_start.
    #[test]
    fn abr_persistence_warm_state_not_cold() {
        let rel = directed_chain(3);
        let x = gradient_field_abr(1, 3);
        let e = operator_e(&x, &rel, &[], &[], 0.3, FIXTURE_CHI_0);
        let ps = PersistenceState::from_prior(e);
        assert!(!ps.is_cold_start, "warm state must not be flagged as cold start");
    }

    // [MATH] Cold start: A_persistence = E_current (E_prior = zero).
    #[test]
    fn abr_a_persistence_cold_start_equals_current() {
        let rel = directed_chain(3);
        let x = gradient_field_abr(1, 3);
        let e = operator_e(&x, &rel, &[], &[], 0.3, FIXTURE_CHI_0);
        let ps = PersistenceState::cold_start(1, rel.n_edges(), &[], rel.n_nodes);
        let a_p = operator_a_persistence(&e, &ps.e_prior);
        for (idx, (&ap, &es)) in a_p[0].iter().zip(e.spatial[0].iter()).enumerate() {
            assert!((ap - es).abs() < 1e-12,
                "cold start: A_persistence[{}] must equal E_current[{}]", idx, idx);
        }
    }

    // [MATH] Stable field: A_persistence = 0. Correct declared result.
    #[test]
    fn abr_a_persistence_stable_field_zero() {
        let rel = directed_chain(3);
        let x = gradient_field_abr(1, 3);
        let e = operator_e(&x, &rel, &[], &[], 0.3, FIXTURE_CHI_0);
        let a_p = operator_a_persistence(&e, &e);
        assert!(a_p[0].iter().all(|&v| v.abs() < 1e-12),
            "stable field: A_persistence must be zero (correct declared result)");
    }

    // [MATH] V5 spatial phase is identical to V4 E output.
    #[test]
    fn abr_e_v5_spatial_matches_v4() {
        let rel = open_dag();
        let x = gradient_field_abr(1, 4);
        let ps = PersistenceState::cold_start(1, rel.n_edges(), &[], rel.n_nodes);
        let (e_v5, _) = operator_e_v5(&x, &rel, &[], &[], &ps, 0.3, FIXTURE_CHI_0);
        let e_v4 = operator_e(&x, &rel, &[], &[], 0.3, FIXTURE_CHI_0);
        for (a, b) in e_v5.spatial[0].iter().zip(e_v4.spatial[0].iter()) {
            assert!((a - b).abs() < 1e-12, "V5 spatial phase must match V4 exactly");
        }
    }

    // [MATH] Two-step evolution: A_persistence[step 2] = E_step2 − E_step1.
    #[test]
    fn abr_two_cycle_evolution() {
        let rel = open_dag();
        let x1 = gradient_field_abr(1, 4);
        let x2 = NodeField::new(vec![vec![0.1, 0.5, 0.3, 0.9]]);
        let ps_cold = PersistenceState::cold_start(1, rel.n_edges(), &[], rel.n_nodes);
        let (e1, _) = operator_e_v5(&x1, &rel, &[], &[], &ps_cold, 0.3, FIXTURE_CHI_0);
        let ps_warm = PersistenceState::from_prior(e1.clone());
        let (e2, p2) = operator_e_v5(&x2, &rel, &[], &[], &ps_warm, 0.3, FIXTURE_CHI_0);
        for e in 0..rel.n_edges() {
            let expected = e2.spatial[0][e] - e1.spatial[0][e];
            assert!((p2.a_persistence[0][e] - expected).abs() < 1e-12,
                "cycle 2: A_persistence must equal E_current - E_prior");
        }
    }

    // [MATH] Terminal persistence edge does not accumulate (no wraparound).
    #[test]
    fn abr_b_persistence_open_boundary() {
        let rel = open_chain(4);
        let x = gradient_field_abr(1, 4);
        let e_curr = operator_e(&x, &rel, &[], &[], 0.3, FIXTURE_CHI_0);
        let ps = PersistenceState::cold_start(1, rel.n_edges(), &[], rel.n_nodes);
        let a_p = operator_a_persistence(&e_curr, &ps.e_prior);
        let b_p = operator_b_persistence(&a_p, &rel);
        for e in 0..rel.n_edges() {
            if rel.succ(e).is_empty() {
                assert!((b_p[0][e] - a_p[0][e]).abs() < 1e-12,
                    "terminal persistence edge must not accumulate");
            }
        }
    }

    // [MATH] Stable field: A_p = 0 → B_p = 0 → R_p = 0.
    // Zero antisymmetric expression here is the correct declared result
    // for a stable observable. It is not a failure.
    #[test]
    fn abr_r_persistence_stable_field_zero() {
        let rel = open_dag();
        let x = gradient_field_abr(1, 4);
        let e = operator_e(&x, &rel, &[], &[], 0.3, FIXTURE_CHI_0);
        let a_p = operator_a_persistence(&e, &e);
        let rho_persistence = compute_rho_persistence(&a_p, &rel, 0.3, FIXTURE_CHI_0);
        let b_p = operator_b_persistence(&a_p, &rel);
        let r_p = operator_r_persistence(&b_p, &rel, &rho_persistence);
        assert!(r_p[0].iter().all(|&v| v.abs() < 1e-12),
            "stable field: full persistence sequence must be zero");
    }

    // ── Cross-kernel coherence ────────────────────────────────────────────

    // [MATH] Δ and A implement the same formula. On a single-component field,
    // their outputs must agree at every edge.
    #[test]
    fn delta_and_a_produce_same_directed_differences() {
        let rel = directed_chain(4);
        let x = gradient_field(4);
        let d = operator_delta(&x, &rel);
        let a = operator_a(&x, &rel, &[]);
        for e in 0..rel.n_edges() {
            assert!((d.field[0][e] - a.spatial[0][e]).abs() < 1e-12,
                "Δ and A must produce identical directed differences at edge {e}");
        }
    }

    // [IMPL] Σ and R both produce finite output on declared relations.
    // Outputs are not expected to be numerically equal — Σ acts on Δ directly;
    // R acts on B(A). No claim about antisymmetric expression is made here.
    // Note: R with empty pairs does not exercise the cc path.
    #[test]
    fn sigma_and_r_both_finite_on_declared_relations() {
        let rel = directed_chain(4);
        let x = gradient_field(4);
        let d = operator_delta(&x, &rel);
        let s = operator_sigma(&d, &rel, 0.2, FIXTURE_CHI_0);
        let a = operator_a(&x, &rel, &[]);
        let rho = compute_rho(&a, &rel, 0.2, FIXTURE_CHI_0);
        let r = operator_r(&a, &rel, &rho, &[]);  // empty pairs — cc path not exercised
        assert!(s.field[0].iter().all(|v| v.is_finite()), "Σ output must be finite");
        assert!(r.spatial[0].iter().all(|v| v.is_finite()), "R output must be finite");
    }

    // ── Observable validation (Category A) ───────────────────────────────
    //
    // These tests drive the operator chain from declared M-mapped observable
    // values and check results against declared measurement sources.
    //
    // Fixture declaration: observable values and relational direction are
    // treated as Origin-declared inputs for operator implementation
    // verification. They do not independently establish physical provenance.
    //
    // 𝟙[e] = 1 is supplied as an assumed fixture. These tests do not verify
    // R → 𝟙 → ε. The upstream R derivation of 𝟙[e] is outside test scope.
    // The gap is left visible per §14 of the purge document.

    // [OBS] Directed difference of declared relational inertia values.
    // Source: PDG 2024, Penning trap under declared non-acceleration (A_p = 0).
    // ι[proton] = 1.672_621_923_69e-27 kg
    // ι[electron] = 9.109_383_701_5e-31 kg
    // Edge direction: proton (source) → electron (target), declared from
    // the decay observable: the heavier locus is the source.
    #[test]
    fn obs_delta_iota_proton_electron_primary_region() {
        const IOTA_PROTON:   f64 = 1.672_621_923_69e-27;
        const IOTA_ELECTRON: f64 = 9.109_383_701_5e-31;
        let rel = test_rel_from_edges(2, vec![(0, 1)]); // source=proton, target=electron
        let x = NodeField::new(vec![vec![IOTA_PROTON, IOTA_ELECTRON]]);
        let d = operator_delta(&x, &rel);
        let delta_e = d.field[0][0]; // Δ(ι)[e] = ι[proton] − ι[electron]
        let expected = IOTA_PROTON - IOTA_ELECTRON;
        // IMPLEMENTATION-ONLY NUMERICAL TOLERANCE: 1e-37
        assert!((delta_e - expected).abs() < 1e-37,
            "Δ(ι)[proton→electron] must equal ι[proton] − ι[electron]");
        assert!(delta_e > 0.0, "directed difference must be positive (proton is heavier)");
        // ε[e] = |Δ(ι)[e]| · 𝟙[e]; 𝟙[e] = 1 assumed as fixture.
        // This does not verify R → 𝟙 — gap left visible.
        let epsilon = delta_e.abs() * 1.0; // 𝟙[e] = 1 (assumed fixture)
        assert!((epsilon - expected.abs()).abs() < 1e-37,
            "ε[e] = |Δ(ι)[e]| with 𝟙[e]=1 (fixture) must equal |ι[proton] − ι[electron]|");
    }

    // [OBS] D*+ → D0 decay edge.
    // Source: PDG 2024, invariant mass from detector-measured momentum through M.
    // Confirmed at 10,000 declared events, mean 1864.84 MeV/c², σ = 1.41e-12.
    // Values in MeV/c² (dimensionless relative to declared ι_unit at Primary Region).
    // Edge direction: D*+ (source) → D0 (target), declared from the decay observable.
    #[test]
    fn obs_delta_iota_d_star_decay_primary_region() {
        const IOTA_D_STAR: f64 = 2010.26; // MeV/c² — PDG 2024
        const IOTA_D0:     f64 = 1864.84; // MeV/c² — PDG 2024
        let rel = test_rel_from_edges(2, vec![(0, 1)]); // D*+ → D0
        let x = NodeField::new(vec![vec![IOTA_D_STAR, IOTA_D0]]);
        let d = operator_delta(&x, &rel);
        let delta_e = d.field[0][0];
        let expected = IOTA_D_STAR - IOTA_D0; // 145.42 MeV/c²
        // IMPLEMENTATION-ONLY NUMERICAL TOLERANCE: relative 1e-4 (PDG quoted precision)
        assert!((delta_e - expected).abs() / expected < 1e-4,
            "Δ(ι)[D*+→D0] must match PDG value within quoted precision: \
             got {:.4} expected {:.4}", delta_e, expected);
        assert!(delta_e > 0.0, "directed difference must be positive (D*+ is heavier)");
    }

    // [REPR] Forward reproducibility: two independent forward applications of
    // operator_delta from the same declared NodeField must agree exactly.
    // Establishes determinism. Does not establish provenance.
    #[test]
    fn repr_delta_two_applications_agree() {
        const IOTA_PROTON:   f64 = 1.672_621_923_69e-27;
        const IOTA_ELECTRON: f64 = 9.109_383_701_5e-31;
        let rel = test_rel_from_edges(2, vec![(0, 1)]);
        let x = NodeField::new(vec![vec![IOTA_PROTON, IOTA_ELECTRON]]);
        // First application
        let d1 = operator_delta(&x, &rel);
        // Second application — independently constructed from the same declared inputs
        let d2 = operator_delta(&x, &rel);
        assert_eq!(d1.field[0][0], d2.field[0][0],
            "two forward applications of operator_delta from identical declared \
             inputs must produce identical output (determinism)");
    }

    // [OBS] Operator chain from declared M mapping hc/λ (Ly-α).
    // Source: NIST ASD v5.12, spectrometer-reported λ = 121.567 nm.
    // M mapping: ε[e] = hc/λ (declared M at Atomic Region photon edges).
    // The NodeField carries ε[Ly-α] at node 0 (emitting locus) and 0.0
    // at node 1 (detection locus). Edge direction: emitter → detector,
    // declared from the emission observable.
    // 𝟙[e] = 1 is assumed as a fixture. R → 𝟙 derivation is outside scope.
    #[test]
    fn obs_operator_chain_lyman_alpha() {
        const H_PLANCK: f64 = 6.626_070_15e-34;
        const C_DECLARED: f64 = 2.997_924_58e8;
        const LAMBDA_LY_ALPHA: f64 = 121.567e-9; // NIST ASD v5.12
        let eps_ly_alpha = H_PLANCK * C_DECLARED / LAMBDA_LY_ALPHA;
        // Declare NodeField with ε[e] at emitting locus (node 0), 0.0 at detector (node 1)
        let rel = test_rel_from_edges(2, vec![(0, 1)]); // emitter → detector
        let x = NodeField::new(vec![vec![eps_ly_alpha, 0.0]]);
        let d = operator_delta(&x, &rel);
        let delta_e = d.field[0][0]; // = ε[Ly-α] − 0.0 = ε[Ly-α]
        // IMPLEMENTATION-ONLY NUMERICAL TOLERANCE: relative 1e-4 (NIST ASD precision)
        assert!((delta_e - eps_ly_alpha).abs() / eps_ly_alpha < 1e-4,
            "operator_delta on Ly-α ε[e] field must equal ε[Ly-α]: \
             got {:.6e} expected {:.6e}", delta_e, eps_ly_alpha);
        // Check ε[e] = |Δ(ι)[e]| · 𝟙[e] with 𝟙[e] = 1 (fixture):
        let epsilon_from_chain = delta_e.abs() * 1.0;
        let eps_ev = epsilon_from_chain / 1.602_176_634e-19;
        assert!((eps_ev - 10.1988).abs() / 10.1988 < 1e-3,
            "ε[Ly-α] from operator chain must match NIST ASD within 0.1%: \
             got {:.6} eV", eps_ev);
    }

    // [OBS] Same chain for Hα (λ = 656.279 nm, NIST ASD v5.12).
    // Demonstrates that the identical operator chain works at a second declared
    // observable with only the M input changing.
    #[test]
    fn obs_operator_chain_balmer_alpha() {
        const H_PLANCK: f64 = 6.626_070_15e-34;
        const C_DECLARED: f64 = 2.997_924_58e8;
        const LAMBDA_H_ALPHA: f64 = 656.279e-9; // NIST ASD v5.12
        let eps_h_alpha = H_PLANCK * C_DECLARED / LAMBDA_H_ALPHA;
        let rel = test_rel_from_edges(2, vec![(0, 1)]);
        let x = NodeField::new(vec![vec![eps_h_alpha, 0.0]]);
        let d = operator_delta(&x, &rel);
        let delta_e = d.field[0][0];
        assert!((delta_e - eps_h_alpha).abs() / eps_h_alpha < 1e-4,
            "operator_delta on Hα ε[e] field must equal ε[Hα]: \
             got {:.6e} expected {:.6e}", delta_e, eps_h_alpha);
        let eps_ev = delta_e.abs() / 1.602_176_634e-19;
        assert!((eps_ev - 1.8892).abs() / 1.8892 < 1e-3,
            "ε[Hα] from operator chain must match NIST ASD within 0.1%: \
             got {:.6} eV", eps_ev);
    }

    // [REPR] Two independent forward applications from identical declared
    // observable inputs (Hα) must agree exactly.
    #[test]
    fn repr_operator_chain_two_applications_agree() {
        const H_PLANCK: f64 = 6.626_070_15e-34;
        const C_DECLARED: f64 = 2.997_924_58e8;
        const LAMBDA_H_ALPHA: f64 = 656.279e-9;
        let eps = H_PLANCK * C_DECLARED / LAMBDA_H_ALPHA;
        let rel = test_rel_from_edges(2, vec![(0, 1)]);
        let x = NodeField::new(vec![vec![eps, 0.0]]);
        let d1 = operator_delta(&x, &rel);
        let d2 = operator_delta(&x, &rel);
        assert_eq!(d1.field[0][0], d2.field[0][0],
            "two forward applications from identical M inputs must produce \
             identical output (determinism)");
        // Also verify: direct formula agrees with operator output.
        // This tests that operator_delta does not smuggle in any quantity
        // beyond the declared x[s] - x[t].
        let direct = x.data[0][0] - x.data[0][1];
        assert_eq!(d1.field[0][0], direct,
            "operator_delta output must equal x[s] - x[t] exactly — \
             no undeclared quantity may enter");
    }

    // ── V7.1 M-declaration requirement ───────────────────────────────────

    // [IMPL] χ₀ must be declared by M; zero is not evaluable.
    #[test]
    #[should_panic(expected = "χ₀ must be declared by M")]
    fn chi_0_undeclared_zero_is_rejected() {
        let rel = directed_chain(4);
        let x = gradient_field(4);
        let _ = operator_e_primary(&x, &rel, 0.2, 0.0);
    }

    // [IMPL] cc must be declared by M for every declared component pair.
    #[test]
    #[should_panic(expected = "cc must be declared by M")]
    fn cc_missing_for_declared_pair_is_rejected() {
        let rel = directed_chain(4);
        let x = NodeField::new(vec![vec![1.0, 2.0, 4.0, 7.0], vec![0.5, 1.0, 1.5, 3.0]]);
        let _ = operator_e(&x, &rel, &[(0, 1)], &[], 0.3, FIXTURE_CHI_0);
    }

    // [MATH] Unit invariance: expressing the same observation in a different
    // unit (x and χ₀ rescaled together) rescales Σ by the same factor exactly.
    // ρ is unchanged, so no unit choice enters the relational output.
    #[test]
    fn sigma_invariant_under_declared_unit_change() {
        let rel = directed_chain(4);
        let v = vec![1.0, 2.0, 4.0, 7.0];
        let scale = 1000.0;
        let x1 = NodeField::new(vec![v.clone()]);
        let x2 = NodeField::new(vec![v.iter().map(|a| a * scale).collect()]);
        let (_, s1) = operator_e_primary(&x1, &rel, 0.3, 1.0);
        let (_, s2) = operator_e_primary(&x2, &rel, 0.3, 1.0 * scale);
        for e in 0..rel.n_edges() {
            let expected = s1.field[0][e] * scale;
            assert!((s2.field[0][e] - expected).abs() <= 1e-9 * expected.abs().max(1.0),
                "Σ must rescale exactly with a declared unit change at edge {e}");
        }
    }
}
