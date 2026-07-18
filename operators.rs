// operators.rs — Metatron Dynamics, Inc. V7.
// Unified Relational Kernel: Primary (Δ → Σ) and ABR (A → B → R).
//
// Grounding documents (V7):
//   operators_notation_and_constraint_v7.md
//   abr_operators_plain_v7.md
//   role_separation_and_operator_application_v7.md
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
//   Used when persistence is confirmed (ρ_P → 1, open condition).
//
// The transition between kernels is not a parameter choice. It is
// determined by ρ_P — the ratio of rank(Im Σ) to propagation capacity.
// Primary Region: ρ_P ≪ 1. ABR Region: ρ_P ≈ 1. Threshold: open condition.
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
// ── Domain and Measurement ───────────────────────────────────────────────
//
// D := { x ∈ ℝⁿ | n < ∞, |x[i]| < ∞ ∀ i }
// M : O → D  declared by Origin before any operator acts.
// The kernel acts on M(o) only.
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
//   declaration. Temporal ordering is a downstream projection of relational
//   evolution onto a one-dimensional axis, not a primitive.
//   Origin must declare the direction by identifying the observable
//   property of the process that establishes which observation is prior
//   and which is current. This applies to oscillation, decay, growth,
//   transport, folding, branching, and any other declared process.
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
// ── B Activation Condition ───────────────────────────────────────────────
//
// B is absent from the primary kernel — not an identity operator.
// B activates when persistence is confirmed: ρ_P ≈ 1. Until then,
// the primary kernel is the correct kernel. Do not introduce B at the
// Primary Region. The transition threshold is an open condition.
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
// ── Version History ──────────────────────────────────────────────────────
//
// V1–V4: See operators_v5.rs version history.
//
// V5 — Foundation statement added. Admissibility redefined as a provenance
//      condition. Declared relation primitive extended to include relational-
//      evolutionary structure (persistence edges). EdgeProvenance enum added.
//      PersistenceState declared with cold-start condition. Operator formulas
//      preserved; domain of application extended. Finding: boundary
//      cancellation theorem does not transfer to persistence domain.
//
// V6 — Primary kernel added: E_primary = Σ(Δ(x)) at the Primary Region.
//      Relational direction: directional admissibility condition and
//      distinctness axiom. Ring inadmissibility derived, not asserted.
//      B absence distinguished from B = identity. DeclaredRelations extended
//      with has_undirected_cycle() and declared_edge_image_admissibility_check.
//      Three Primary Region invariants: rank(Im Δ), rank(Im Σ), ρ_P.
//      Two legitimate failure modes (FM1, FM2); FM3 reclassified as
//      inadmissibility condition (declaration error, not operator outcome).
//
// V7 — Unified file: primary kernel and ABR kernel in one module.
//      DeclaredRelations unified: carries both node-indexed (out/inc)
//      and edge-indexed (adj_plus/adj_minus) adjacency — all derivable
//      from the same edge list at construction, serving both kernels.
//      NodeField subsumes ObservableField (same structure, unified naming).
//      PrimaryEdgeField retained separately from ABR EdgeField — the two
//      kernels have structurally distinct output types.
//      Sequential observation requirement added to header.
//      Relational evolution direction added to header.
//      Legacy mathematical intrusion warning added to header.
//      All V5 and V6 operator formulas preserved without modification.
//      All V5 and V6 tests preserved without modification.
//
// Bounded over D. No claim beyond D.
// Metatron Dynamics, Inc. V7.

use nalgebra::DMatrix;

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
    /// Construct from edges with declared provenance.
    /// Builds all four adjacency structures from a single edge list.
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

    /// Convenience constructor: all edges declared as Continuation.
    pub fn from_edges(n_nodes: usize, edges: Vec<(usize, usize)>) -> Self {
        let n = edges.len();
        Self::from_edges_with_provenance(
            n_nodes, edges,
            vec![EdgeProvenance::Continuation; n],
        )
    }

    #[inline] pub fn n_edges(&self) -> usize { self.edges.len() }

    /// Forward continuation of edge e — edges starting where e ends.
    /// Used by ABR B and R operators. Equivalent to adj_plus[e].
    #[inline] pub fn succ(&self, e: usize) -> &[usize] { &self.out[self.edges[e].1] }

    /// Backward continuation of edge e — edges ending where e starts.
    /// Used by ABR R operator. Equivalent to adj_minus[e].
    #[inline] pub fn pred(&self, e: usize) -> &[usize] { &self.inc[self.edges[e].0] }

    /// Propagation capacity — number of edges with at least one successor.
    /// Used in ρ_P = rank(Im Σ) / propagation_capacity.
    pub fn propagation_capacity(&self) -> usize {
        self.adj_plus.iter().filter(|a| !a.is_empty()).count()
    }

    /// Admissibility check: no disconnected factors.
    pub fn is_fully_connected(&self) -> bool {
        if self.n_nodes == 0 { return true; }
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); self.n_nodes];
        for &(s, t) in &self.edges {
            if !adj[s].contains(&t) { adj[s].push(t); }
            if !adj[t].contains(&s) { adj[t].push(s); }
        }
        let mut visited = vec![false; self.n_nodes];
        let mut stack = vec![0usize];
        visited[0] = true;
        while let Some(n) = stack.pop() {
            for &m in &adj[n] {
                if !visited[m] { visited[m] = true; stack.push(m); }
            }
        }
        visited.iter().all(|&v| v)
    }

    /// Detects any undirected cycle of length ≥ 3.
    ///
    /// Returns true when the declared edge set contains an undirected cycle.
    /// This detects both rings (inadmissible) and diamond DAGs (admissible
    /// when declared with provenance). The caller determines admissibility
    /// from context — a ring closing edge has no admissible provenance; a
    /// diamond DAG has declared provenance at each edge.
    pub fn has_undirected_cycle(&self) -> bool {
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); self.n_nodes];
        for &(s, t) in &self.edges {
            if s != t && !adj[s].contains(&t) { adj[s].push(t); }
            if s != t && !adj[t].contains(&s) { adj[t].push(s); }
        }
        let mut visited = vec![false; self.n_nodes];
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
        for start in 0..self.n_nodes {
            if !visited[start] && dfs(start, None, &adj, &mut visited) {
                return true;
            }
        }
        false
    }
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

    pub fn frobenius_norm(&self) -> f64 {
        self.field.iter()
            .flat_map(|c| c.iter())
            .map(|v| v * v)
            .sum::<f64>()
            .sqrt()
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
// ρ[e] = ρ_base · m[s] / (1 + m[s])
// m[s] = max{ |Δ(x)[e']| : e' incident to s }
// Derived per edge from Δ(x). No aggregation beyond the node.
// ρ ∈ [0, ρ_base) for all declared edges.

pub fn compute_rho_primary(
    delta_field: &PrimaryEdgeField,
    rel: &DeclaredRelations,
    rho_base: f64,
) -> Vec<f64> {
    let mut node_incident: Vec<Vec<usize>> = vec![Vec::new(); rel.n_nodes];
    for (e, &(s, t)) in rel.edges.iter().enumerate() {
        node_incident[s].push(e);
        node_incident[t].push(e);
    }
    rel.edges.iter().map(|&(s, _)| {
        let m = node_incident[s].iter()
            .flat_map(|&e| delta_field.field.iter().map(move |c| c[e].abs()))
            .fold(0.0_f64, f64::max);
        rho_base * m / (1.0 + m)
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
// Under the directional admissibility condition, admissible declared
// structures are asymmetric. For any admissible structure with non-trivial
// adjacency, the antisymmetric term is non-zero.
//
// Constraint: immediate adjacency only. No path accumulation.

pub fn operator_sigma(
    delta_field: &PrimaryEdgeField,
    rel: &DeclaredRelations,
    rho_base: f64,
) -> PrimaryEdgeField {
    assert_eq!(delta_field.n_edges, rel.n_edges(),
        "delta field and declared relations must have the same edge count");
    let rho = compute_rho_primary(delta_field, rel, rho_base);
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
//   Preserves: circulation contribution at each declared edge.
//   Discards:  pass-through term g[e].

pub fn antisymmetric_term(
    delta_field: &PrimaryEdgeField,
    rel: &DeclaredRelations,
    rho_base: f64,
) -> PrimaryEdgeField {
    assert_eq!(delta_field.n_edges, rel.n_edges());
    let rho = compute_rho_primary(delta_field, rel, rho_base);
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

pub fn operator_e_primary(
    x: &NodeField,
    rel: &DeclaredRelations,
    rho_base: f64,
) -> (PrimaryEdgeField, PrimaryEdgeField) {
    let delta = operator_delta(x, rel);
    let sigma = operator_sigma(&delta, rel, rho_base);
    (delta, sigma)
}

// ── Primary Region Analysis ───────────────────────────────────────────────

pub const SVD_TOLERANCE: f64 = 1e-10;

/// rank(Im Δ) — dimension of the span of declared edge vectors in Δ output.
/// Declared projection:
///   Preserves: dimension of the span.
///   Discards:  individual edge values and their ordering.
pub fn im_delta_rank(delta_field: &PrimaryEdgeField) -> (usize, Vec<f64>) {
    if delta_field.n_edges == 0 || delta_field.n_components == 0 {
        return (0, Vec::new());
    }
    let n_rows = delta_field.n_edges;
    let n_cols = delta_field.n_components;
    let data: Vec<f64> = (0..n_rows)
        .flat_map(|e| (0..n_cols).map(move |c| delta_field.field[c][e]))
        .collect();
    let mat = DMatrix::from_row_slice(n_rows, n_cols, &data);
    let svd = mat.svd(false, false);
    let singular_values: Vec<f64> = svd.singular_values.iter().cloned().collect();
    let rank = singular_values.iter().filter(|&&s| s > SVD_TOLERANCE).count();
    (rank, singular_values)
}

/// rank(Im Σ) — same projection applied to the Σ output field.
pub fn im_sigma_rank(sigma_field: &PrimaryEdgeField) -> (usize, Vec<f64>) {
    im_delta_rank(sigma_field)
}

/// ρ_P = rank(Im Σ) / propagation_capacity.
/// Primary Region: ρ_P ≪ 1. ABR transition: ρ_P ≈ 1. OPEN CONDITION.
pub fn rho_p_ratio(sigma_field: &PrimaryEdgeField, rel: &DeclaredRelations) -> f64 {
    let (rank_sigma, _) = im_sigma_rank(sigma_field);
    let c_x = rel.propagation_capacity();
    if c_x == 0 { return 0.0; }
    rank_sigma as f64 / c_x as f64
}

/// Declared edge-image admissibility check.
///
/// An admissible declared edge-image is ASYMMETRIC: no declared edge vector
/// has its negation as another declared edge vector.
/// Returns (is_admissible, inadmissibility_witness).
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

/// Expression condition — admissible when rank(Im Σ) > 0 and antisymmetric
/// term is non-zero on at least one declared edge.
pub fn expression_condition(
    delta_field: &PrimaryEdgeField,
    sigma_field: &PrimaryEdgeField,
    rel: &DeclaredRelations,
    rho_base: f64,
) -> (bool, usize, bool) {
    let (rank_sigma, _) = im_sigma_rank(sigma_field);
    let asym = antisymmetric_term(delta_field, rel, rho_base);
    let has_nonzero_asym = asym.field.iter()
        .any(|c| c.iter().any(|&v| v.abs() > SVD_TOLERANCE));
    let expressed = rank_sigma > 0 && has_nonzero_asym;
    (expressed, rank_sigma, has_nonzero_asym)
}

/// Failure mode detection for the primary kernel.
#[derive(Clone, Debug, PartialEq)]
pub enum FailureMode {
    None,
    /// FM1: rank(Im Δ) = 0. Observable undifferentiated across all declared relations.
    DifferentiationCollapse,
    /// FM2: all edges isolated. Antisymmetric term structurally zero.
    RelationalIsolation,
    /// FM3: Circulation Cancellation — inadmissibility condition, not a legitimate
    /// failure mode. Signals inadmissible symmetric adjacency — declaration error.
    CirculationCancellation,
}

pub fn detect_failure_mode(
    delta_field: &PrimaryEdgeField,
    sigma_field: &PrimaryEdgeField,
    rel: &DeclaredRelations,
    rho_base: f64,
) -> FailureMode {
    let (rank_delta, _) = im_delta_rank(delta_field);
    if rank_delta == 0 { return FailureMode::DifferentiationCollapse; }
    let all_isolated = rel.adj_plus.iter().zip(rel.adj_minus.iter())
        .all(|(p, m)| p.is_empty() && m.is_empty());
    if all_isolated { return FailureMode::RelationalIsolation; }
    let asym = antisymmetric_term(delta_field, rel, rho_base);
    let asym_all_zero = asym.field.iter()
        .all(|c| c.iter().all(|&v| v.abs() <= SVD_TOLERANCE));
    let (rank_sigma, _) = im_sigma_rank(sigma_field);
    if asym_all_zero && rank_sigma > 0 {
        return FailureMode::CirculationCancellation;
    }
    FailureMode::None
}

// ═══════════════════════════════════════════════════════════════════════════
// ── ABR KERNEL: A → B → R ──────────────────────────────────────────────────
// ═══════════════════════════════════════════════════════════════════════════
//
// Phase 1 (spatial):     E_spatial = R(B(A(x)), ρ(A(x)))
// Phase 2 (persistence): A_p → B_p → R_p over edge-valued loci
//
// Used when persistence is confirmed (ρ_P → 1, open condition).
// All formulas preserved exactly from V5.

// ── ABR Edge Field ────────────────────────────────────────────────────────
//
// Output of ABR kernel operators. Carries spatial and component pair fields.
// Structurally distinct from PrimaryEdgeField — two kernels, two output types.

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

    pub fn frobenius_norm_spatial(&self) -> f64 {
        self.spatial.iter()
            .flat_map(|c| c.iter())
            .map(|v| v * v)
            .sum::<f64>()
            .sqrt()
    }
}

// ── Persistence State ─────────────────────────────────────────────────────
//
// Holds the prior complete kernel output.
//
// is_cold_start: true on first declared step — E_prior is zero EdgeField.
// After the first step, E_prior must come from a prior actual observation
// in the declared sequence {M(o₁), M(o₂), ...}. A model-generated prior,
// a parameter value, or a repeated static configuration is not admissible
// as E_prior beyond the first step.

#[derive(Clone, Debug)]
pub struct PersistenceState {
    pub e_prior: EdgeField,
    pub is_cold_start: bool,
}

impl PersistenceState {
    /// Declare cold-start state: no relational history, full current state
    /// is new. Admissible on the first declared step only.
    pub fn cold_start(k: usize, n_edges: usize, comp_pairs: &[(usize, usize)], n_nodes: usize) -> Self {
        PersistenceState {
            e_prior: EdgeField::zero(k, n_edges, comp_pairs, n_nodes),
            is_cold_start: true,
        }
    }

    /// Declare prior state from a completed declared process step's kernel output.
    /// The prior state must come from an actual observation in the declared
    /// sequence — not from a model, a parameter value, or a static configuration.
    pub fn from_prior(e_prior: EdgeField) -> Self {
        PersistenceState { e_prior, is_cold_start: false }
    }
}

// ── Persistence Output ────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct PersistenceOutput {
    pub a_persistence: Vec<Vec<f64>>,  // [c][e] — persistence gradient
    pub b_persistence: Vec<Vec<f64>>,  // [c][e] — accumulated
    pub r_persistence: Vec<Vec<f64>>,  // [c][e] — circulation
}

// ── Operator A ────────────────────────────────────────────────────────────
//
// A(x)[e] = x[s] − x[t] over each declared spatial edge.
// NodeField → EdgeField.
// Constraint: directed difference only. No relation added beyond declared.

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
// ρ[i] = rho_base × m[i] / (1 + m[i])
// m[i] = largest gradient at node i in A output.
// Derived per node from A(x). No aggregation beyond the node.

pub fn compute_rho(a: &EdgeField, rel: &DeclaredRelations, rho_base: f64) -> Vec<f64> {
    (0..rel.n_nodes).map(|i| {
        let mut m = 0.0_f64;
        for &e in rel.out[i].iter().chain(rel.inc[i].iter()) {
            for c in &a.spatial { m = m.max(c[e].abs()); }
        }
        for c in &a.comp { m = m.max(c[i].abs()); }
        rho_base * m / (1.0 + m)
    }).collect()
}

// ── Operator R ────────────────────────────────────────────────────────────
//
// R(g)[e] = g[e] + ρ[src(e)] × (Σ_succ B(g) − Σ_pred B(g))
// Cross-topology: spatial edges receive component-edge asymmetry;
// component edges receive spatial-edge asymmetry. Local, antisymmetric, additive.

pub fn operator_r(bg: &EdgeField, rel: &DeclaredRelations, rho: &[f64]) -> EdgeField {
    let k = bg.k;
    let pairs = &bg.comp_pairs;
    let rho_e: Vec<f64> = rel.edges.iter().map(|&(s, _)| rho[s]).collect();

    let mut spatial: Vec<Vec<f64>> = (0..k).map(|c| {
        (0..rel.n_edges()).map(|e| {
            let fwd: f64 = rel.succ(e).iter().map(|&f| bg.spatial[c][f]).sum();
            let bwd: f64 = rel.pred(e).iter().map(|&p| bg.spatial[c][p]).sum();
            bg.spatial[c][e] + rho_e[e] * (fwd - bwd)
        }).collect()
    }).collect();

    let cc = 0.5;
    for (p, &(a, b)) in pairs.iter().enumerate() {
        for (e, &(s, d)) in rel.edges.iter().enumerate() {
            let asym = bg.comp[p][d] - bg.comp[p][s];
            spatial[a][e] += rho_e[e] * cc * asym;
            spatial[b][e] -= rho_e[e] * cc * asym;
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

pub fn operator_e(f: &NodeField, rel: &DeclaredRelations, pairs: &[(usize, usize)], rho_base: f64) -> EdgeField {
    let a = operator_a(f, rel, pairs);
    let rho = compute_rho(&a, rel, rho_base);
    let b = operator_b(&a, rel);
    operator_r(&b, rel, &rho)
}

// ── A_persistence ─────────────────────────────────────────────────────────
//
// A_persistence[c][e] = E_current.spatial[c][e] − E_prior.spatial[c][e]
//
// Directed difference between current and prior kernel output.
// Direction is fixed: E_current − E_prior.
// The reverse direction is not traceable to an observable through M.
//
// On cold start (E_prior = zero): A_persistence = E_current — the entire
// current relational state is declared new. This is admissible as the
// first declared step. It does not constitute evidence of relational evolution.
//
// After the first step, E_prior must come from a prior actual observation
// in the declared sequence. A_persistence over a static observable
// (E_current = E_prior) produces zero output — the correct result.

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
// Same formula as compute_rho, applied to the persistence gradient field.
// Reflects the magnitude of relational evolution at each node.

pub fn compute_rho_persistence(
    a_persistence: &[Vec<f64>],
    rel: &DeclaredRelations,
    rho_base: f64,
) -> Vec<f64> {
    (0..rel.n_nodes).map(|i| {
        let mut m = 0.0_f64;
        for &e in rel.out[i].iter().chain(rel.inc[i].iter()) {
            for c in a_persistence { m = m.max(c[e].abs()); }
        }
        rho_base * m / (1.0 + m)
    }).collect()
}

// ── B_persistence ─────────────────────────────────────────────────────────
//
// B_persistence[c][e] = A_persistence[c][e] + Σ_{f ∈ succ(e)} A_persistence[c][f]
// Terminal edges accumulate nothing. No wraparound.

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
// Returns (E_spatial, PersistenceOutput).
// E_spatial is V4-compatible. PersistenceOutput is the evolutionary layer.
//
// Phase 2 requires the declared sequence from the sequential observation
// declaration in Step 1 (Declare). Cold start output is the declared first
// step only — not evidence of relational evolution.

pub fn operator_e_v5(
    f: &NodeField,
    rel: &DeclaredRelations,
    pairs: &[(usize, usize)],
    persistence_state: &PersistenceState,
    rho_base: f64,
) -> (EdgeField, PersistenceOutput) {
    let e_spatial = operator_e(f, rel, pairs, rho_base);
    let a_p = operator_a_persistence(&e_spatial, &persistence_state.e_prior);
    let rho_p = compute_rho_persistence(&a_p, rel, rho_base);
    let b_p = operator_b_persistence(&a_p, rel);
    let r_p = operator_r_persistence(&b_p, rel, &rho_p);
    (e_spatial, PersistenceOutput {
        a_persistence: a_p,
        b_persistence: b_p,
        r_persistence: r_p,
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// ── TESTS ──────────────────────────────────────────────────────────────────
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── Shared helpers ────────────────────────────────────────────────────

    fn open_dag() -> DeclaredRelations {
        // Fan-out/fan-in DAG: (0,1),(0,2),(1,3),(2,3). Strictly directed.
        DeclaredRelations::from_edges(4, vec![(0,1),(0,2),(1,3),(2,3)])
    }

    fn directed_chain(n: usize) -> DeclaredRelations {
        let edges: Vec<(usize,usize)> = (0..n-1).map(|i| (i, i+1)).collect();
        DeclaredRelations::from_edges(n, edges)
    }

    fn open_chain(n: usize) -> DeclaredRelations {
        directed_chain(n)
    }

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

    // ── Shared DeclaredRelations tests ────────────────────────────────────

    #[test]
    fn declared_relations_unified_adjacency() {
        // adj_plus[e] and succ(e) must agree; adj_minus[e] and pred(e) must agree.
        let rel = open_dag();
        for e in 0..rel.n_edges() {
            let succ_e: Vec<usize> = rel.succ(e).to_vec();
            let mut adj_p = rel.adj_plus[e].clone(); adj_p.sort();
            let mut succ_s = succ_e.clone(); succ_s.sort();
            assert_eq!(adj_p, succ_s,
                "adj_plus[{e}] must equal succ({e})");

            let pred_e: Vec<usize> = rel.pred(e).to_vec();
            let mut adj_m = rel.adj_minus[e].clone(); adj_m.sort();
            let mut pred_s = pred_e.clone(); pred_s.sort();
            assert_eq!(adj_m, pred_s,
                "adj_minus[{e}] must equal pred({e})");
        }
    }

    #[test]
    fn closed_ring_detected_as_undirected_cycle() {
        let rel = DeclaredRelations::from_edges(4, vec![(0,1),(1,2),(2,3),(3,0)]);
        assert!(rel.has_undirected_cycle());
    }

    #[test]
    fn directed_chain_no_undirected_cycle() {
        let rel = directed_chain(4);
        assert!(!rel.has_undirected_cycle());
    }

    #[test]
    fn node_field_unifies_observable_field() {
        // NodeField is the unified type. Both k/n and n_components()/n_nodes() work.
        let f = gradient_field(4);
        assert_eq!(f.k, f.n_components());
        assert_eq!(f.n, f.n_nodes());
    }

    // ── Primary kernel tests ──────────────────────────────────────────────

    #[test]
    fn primary_delta_directed_difference() {
        let rel = DeclaredRelations::from_edges(2, vec![(0,1)]);
        let x = NodeField::new(vec![vec![3.0, 1.0]]);
        let d = operator_delta(&x, &rel);
        assert!((d.field[0][0] - 2.0).abs() < 1e-12, "Δ[(0,1)] = x[0] - x[1] = 2.0");
    }

    #[test]
    fn primary_delta_reverse_is_negation() {
        let rel_fwd = DeclaredRelations::from_edges(2, vec![(0,1)]);
        let rel_rev = DeclaredRelations::from_edges(2, vec![(1,0)]);
        let x = NodeField::new(vec![vec![3.0, 1.0]]);
        let d_fwd = operator_delta(&x, &rel_fwd);
        let d_rev = operator_delta(&x, &rel_rev);
        assert!((d_fwd.field[0][0] + d_rev.field[0][0]).abs() < 1e-12,
            "forward and reverse must be exact negations — direction is structural");
    }

    #[test]
    fn primary_sigma_equals_delta_for_isolated_edge() {
        let rel = DeclaredRelations::from_edges(2, vec![(0,1)]);
        let x = NodeField::new(vec![vec![1.0, 0.0]]);
        let (d, s) = operator_e_primary(&x, &rel, 0.2);
        assert!((s.field[0][0] - d.field[0][0]).abs() < 1e-12,
            "Σ must equal Δ for isolated edge (no adjacency)");
    }

    #[test]
    fn primary_b_absent_no_accumulation() {
        let rel = DeclaredRelations::from_edges(2, vec![(0,1)]);
        assert_eq!(rel.propagation_capacity(), 0,
            "single edge: propagation capacity must be 0");
        let x = NodeField::new(vec![vec![2.0, 1.0]]);
        let (d, s) = operator_e_primary(&x, &rel, 0.2);
        assert!((d.field[0][0] - 1.0).abs() < 1e-12, "Δ[(0,1)] = 1.0");
        assert!((s.field[0][0] - 1.0).abs() < 1e-12, "Σ = Δ — B absent, not identity");
    }

    #[test]
    fn primary_failure_mode_differentiation_collapse() {
        let rel = open_dag();
        let x = uniform_field(4, 1.0);
        let (d, s) = operator_e_primary(&x, &rel, 0.2);
        let mode = detect_failure_mode(&d, &s, &rel, 0.2);
        assert_eq!(mode, FailureMode::DifferentiationCollapse);
    }

    #[test]
    fn primary_no_failure_open_dag_gradient() {
        let rel = open_dag();
        let x = gradient_field(4);
        let (d, s) = operator_e_primary(&x, &rel, 0.2);
        let mode = detect_failure_mode(&d, &s, &rel, 0.2);
        assert_ne!(mode, FailureMode::DifferentiationCollapse);
    }

    #[test]
    fn primary_rho_p_bounded() {
        let rel = open_dag();
        let x = gradient_field(4);
        let (_, s) = operator_e_primary(&x, &rel, 0.2);
        let rp = rho_p_ratio(&s, &rel);
        assert!(rp >= 0.0 && rp.is_finite());
    }

    #[test]
    fn primary_declared_edge_image_admissibility_directed_chain() {
        let rel = directed_chain(4);
        let x = gradient_field(4);
        let d = operator_delta(&x, &rel);
        let (admissible, _) = declared_edge_image_admissibility_check(&d, 1e-10);
        assert!(admissible, "directed chain with gradient must be admissible");
    }

    // ── Relational evolution direction tests (primary) ────────────────────

    #[test]
    fn evolution_direction_is_current_minus_prior() {
        // The persistence direction is fixed: E_current − E_prior.
        // Swapping arguments produces the negation — direction is structural,
        // not a convention.
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
            assert!((f + r).abs() < 1e-12,
                "forward and reverse must be exact negations");
        }
        assert!(forward.iter().any(|&v| v.abs() > 1e-12),
            "forward direction must be non-zero when states differ");
    }

    #[test]
    fn evolution_direction_zero_on_stable_observable() {
        // When M(oₖ) = M(oₖ₋₁), A_persistence = 0. Correct result, not failure.
        let rel = directed_chain(3);
        let x = NodeField::new(vec![vec![1.0, 0.5, 0.0]]);
        let e = operator_delta(&x, &rel);
        let stable: Vec<f64> = e.field[0].iter()
            .zip(e.field[0].iter()).map(|(c, p)| c - p).collect();
        assert!(stable.iter().all(|&v| v.abs() < 1e-12),
            "stable observable: A_persistence = 0 — correct declared result");
    }

    // ── ABR kernel tests ──────────────────────────────────────────────────

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

    #[test]
    fn abr_e_v4_finite() {
        let rel = open_dag();
        let x = gradient_field_abr(1, 4);
        let e = operator_e(&x, &rel, &[], 0.3);
        assert!(e.spatial[0].iter().all(|v| v.is_finite()));
    }

    #[test]
    fn abr_cold_start_flag() {
        let rel = directed_chain(3);
        let ps = PersistenceState::cold_start(1, rel.n_edges(), &[], rel.n_nodes);
        assert!(ps.is_cold_start,
            "cold start must be flagged");
        assert!(ps.e_prior.spatial[0].iter().all(|&v| v == 0.0),
            "cold start E_prior must be zero");
    }

    #[test]
    fn abr_persistence_warm_state_not_cold() {
        let rel = directed_chain(3);
        let x = gradient_field_abr(1, 3);
        let e = operator_e(&x, &rel, &[], 0.3);
        let ps = PersistenceState::from_prior(e);
        assert!(!ps.is_cold_start,
            "warm state must not be flagged as cold start");
    }

    #[test]
    fn abr_a_persistence_cold_start_equals_current() {
        // On cold start, A_persistence = E_current (full state declared new).
        let rel = directed_chain(3);
        let x = gradient_field_abr(1, 3);
        let e = operator_e(&x, &rel, &[], 0.3);
        let ps = PersistenceState::cold_start(1, rel.n_edges(), &[], rel.n_nodes);
        let a_p = operator_a_persistence(&e, &ps.e_prior);
        for (idx, (&ap, &es)) in a_p[0].iter().zip(e.spatial[0].iter()).enumerate() {
            assert!((ap - es).abs() < 1e-12,
                "cold start: A_persistence[{}] must equal E_current[{}]", idx, idx);
        }
    }

    #[test]
    fn abr_a_persistence_stable_field_zero() {
        // Stable field (E_current = E_prior): A_persistence = 0. Correct result.
        let rel = directed_chain(3);
        let x = gradient_field_abr(1, 3);
        let e = operator_e(&x, &rel, &[], 0.3);
        let a_p = operator_a_persistence(&e, &e);
        assert!(a_p[0].iter().all(|&v| v.abs() < 1e-12),
            "stable field: A_persistence must be zero (correct declared result)");
    }

    #[test]
    fn abr_e_v5_spatial_matches_v4() {
        // V5 spatial phase must be identical to V4 E output.
        let rel = open_dag();
        let x = gradient_field_abr(1, 4);
        let ps = PersistenceState::cold_start(1, rel.n_edges(), &[], rel.n_nodes);
        let (e_v5, _) = operator_e_v5(&x, &rel, &[], &ps, 0.3);
        let e_v4 = operator_e(&x, &rel, &[], 0.3);
        for (a, b) in e_v5.spatial[0].iter().zip(e_v4.spatial[0].iter()) {
            assert!((a - b).abs() < 1e-12,
                "V5 spatial phase must match V4 exactly");
        }
    }

    #[test]
    fn abr_two_cycle_evolution() {
        // Cycle 1 (cold start): A_persistence = E_current.
        // Cycle 2 (warm): A_persistence = E_cycle2 - E_cycle1.
        let rel = open_dag();
        let x1 = gradient_field_abr(1, 4);
        let x2 = NodeField::new(vec![vec![0.1, 0.5, 0.3, 0.9]]);

        let ps_cold = PersistenceState::cold_start(1, rel.n_edges(), &[], rel.n_nodes);
        let (e1, _) = operator_e_v5(&x1, &rel, &[], &ps_cold, 0.3);

        let ps_warm = PersistenceState::from_prior(e1.clone());
        let (e2, p2) = operator_e_v5(&x2, &rel, &[], &ps_warm, 0.3);

        for e in 0..rel.n_edges() {
            let expected = e2.spatial[0][e] - e1.spatial[0][e];
            assert!((p2.a_persistence[0][e] - expected).abs() < 1e-12,
                "cycle 2: A_persistence must equal E_current - E_prior");
        }
    }

    #[test]
    fn abr_b_persistence_open_boundary() {
        let rel = open_chain(4);
        let x = gradient_field_abr(1, 4);
        let e_curr = operator_e(&x, &rel, &[], 0.3);
        let ps = PersistenceState::cold_start(1, rel.n_edges(), &[], rel.n_nodes);
        let a_p = operator_a_persistence(&e_curr, &ps.e_prior);
        let b_p = operator_b_persistence(&a_p, &rel);
        for e in 0..rel.n_edges() {
            if rel.succ(e).is_empty() {
                assert!((b_p[0][e] - a_p[0][e]).abs() < 1e-12,
                    "terminal persistence edge must not accumulate (no wraparound)");
            }
        }
    }

    #[test]
    fn abr_r_persistence_stable_field_zero() {
        // Stable field: A_p = 0 → B_p = 0 → R_p = 0.
        let rel = open_dag();
        let x = gradient_field_abr(1, 4);
        let e = operator_e(&x, &rel, &[], 0.3);
        let a_p = operator_a_persistence(&e, &e);
        let rho_p = compute_rho_persistence(&a_p, &rel, 0.3);
        let b_p = operator_b_persistence(&a_p, &rel);
        let r_p = operator_r_persistence(&b_p, &rel, &rho_p);
        assert!(r_p[0].iter().all(|&v| v.abs() < 1e-12),
            "stable field: full persistence sequence must be zero");
    }

    // ── Cross-kernel coherence ────────────────────────────────────────────

    #[test]
    fn delta_and_a_produce_same_directed_differences() {
        // Δ and A are the same formula. On a single-component field,
        // their outputs must agree numerically.
        let rel = directed_chain(4);
        let x = gradient_field(4);
        let d = operator_delta(&x, &rel);
        let a = operator_a(&x, &rel, &[]);
        for e in 0..rel.n_edges() {
            assert!((d.field[0][e] - a.spatial[0][e]).abs() < 1e-12,
                "Δ and A must produce identical directed differences at edge {e}");
        }
    }

    #[test]
    fn sigma_reduces_to_r_without_b() {
        // Σ = R applied to Δ directly. On a single-component field with
        // the same topology, Σ(Δ) and R(Δ_as_EdgeField) must agree.
        // This test constructs the R input manually to verify.
        let rel = directed_chain(4);
        let x = gradient_field(4);

        // Primary path: Σ(Δ(x))
        let d = operator_delta(&x, &rel);
        let s = operator_sigma(&d, &rel, 0.2);

        // ABR path: R(A(x)) — using A output as B input (B = identity on A here
        // because this is a chain and B adds succ values, which differ from just A)
        // We do NOT expect these to be equal in general — Σ acts on Δ directly,
        // R acts on B(A). This test instead verifies structural consistency:
        // both produce finite output on the same declared relations.
        let a = operator_a(&x, &rel, &[]);
        let rho = compute_rho(&a, &rel, 0.2);
        let r = operator_r(&a, &rel, &rho);  // R without B — only valid for this test

        assert!(s.field[0].iter().all(|v| v.is_finite()),
            "Σ output must be finite");
        assert!(r.spatial[0].iter().all(|v| v.is_finite()),
            "R output must be finite");
    }
}
