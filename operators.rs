// operators.rs — Metatron Dynamics V4 — Invariant Relational Kernel: ABR
//
// Kernel:  E(x, ρ) = R(B(A(x)), ρ(A(x)))      [A → B → R → E]
// Domain:  D := { x ∈ ℝⁿ | n < ∞, |x[i]| < ∞ ∀ i }.  All quantifiers
//          bounded over D; no claim beyond D.
//
// The kernel acts over declared relations, never over a primitive
// topology. The operators require declared relations (adjacency +
// continuation) to act over, but never supply them: no periodic default,
// no built-in ring, torus, or dimension. Adjacency is supplied through M
// from observables and/or declared explicitly by Origin — two distinct
// provenance routes — per the admissibility conditions of the topology
// paper §16. ("Topology" survives in the papers as a governed umbrella
// term for a declared relation bundle; it is not a type and not a
// primitive. The kernel type is `DeclaredRelations`.)
//
// RING / TORUS ARE PROOF CONSTRUCTS, NOT APPLICATION CONSTRUCTS.
// The Object Error theorems are established on the ring (object_error.md
// §8.2); that is its only role. A ring or torus asserts four relational
// behaviors, none observed in evolving systems: closed boundary,
// bidirectional symmetry, uniform degree, and exact recurrence
// (topology paper §12). Reaching for one declares into D a structure that
// M never produced from observables — a C → O → D category error, not a
// shortcut. A ring, torus, `np.roll`, or any periodic-index operation
// proposed for an application is a DRIFT SIGNAL, not a declaration: refuse
// it and require declared relations with provenance. That it executes is
// not a reason to accept it (topology paper §16.6: execution is not the
// question). This file constructs no ring anywhere, including in tests.
//
// Non-agency: the operators do not act, cause, optimize, or enforce. They
// define invariant relational structure over the declared relations in D.
//
// Pre-A constraint: no transformation may precede A if it alters pairwise
// differences, unless declared within M. Admissible before A: x + c
// (uniform shift) and x / s (declared unit scale). All else inadmissible
// unless declared with preserved/discarded invariants.
//
// C is not a kernel operator in V4: it is a declared application-layer
// projection (preserved/discarded invariants stated per use). σ² and Γ
// are declared diagnostics at the verification layer. None are defined
// here. (object_error.md §8.7; invariant-taxonomy.md.)

// ---- Declared relational structure --------------------------------------
// The kernel's input is declared relations, not a geometric topology:
// distinguishable elements, declared adjacency (edges), declared
// continuation. `DeclaredRelations` names this bundle; admissibility sits
// on the relations themselves (topology paper §16), not on a geometric
// primitive. Continuation is derived, not inferred: succ(e) = edges
// leaving tgt(e); pred(e) = edges entering src(e). Irregular degree,
// terminals, and branch points are admissible. No index arithmetic; open
// boundaries not wrapped.

#[derive(Clone, Debug)]
pub struct DeclaredRelations {
    pub n_nodes: usize,
    pub edges: Vec<(usize, usize)>, // edge e = (src, tgt) — a declared relation
    pub out: Vec<Vec<usize>>,       // out[i]: edges leaving node i (derived)
    pub inc: Vec<Vec<usize>>,       // inc[i]: edges entering node i (derived)
}

impl DeclaredRelations {
    pub fn from_edges(n_nodes: usize, edges: Vec<(usize, usize)>) -> Self {
        assert!(edges.iter().all(|&(s, t)| s < n_nodes && t < n_nodes));
        let mut out = vec![Vec::new(); n_nodes];
        let mut inc = vec![Vec::new(); n_nodes];
        for (e, &(s, t)) in edges.iter().enumerate() {
            out[s].push(e);
            inc[t].push(e);
        }
        DeclaredRelations { n_nodes, edges, out, inc }
    }
    #[inline] pub fn n_edges(&self) -> usize { self.edges.len() }
    #[inline] fn succ(&self, e: usize) -> &[usize] { &self.out[self.edges[e].1] }
    #[inline] fn pred(&self, e: usize) -> &[usize] { &self.inc[self.edges[e].0] }
}

// ---- Fields -------------------------------------------------------------
// One kernel. k components per node (scalar = k = 1). Dimensionality is a
// property of the declared relations, not of the field type.

#[derive(Clone, Debug)]
pub struct NodeField {
    pub data: Vec<Vec<f64>>, // data[c][i]
    pub k: usize,
    pub n: usize,
}

impl NodeField {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        let k = data.len();
        assert!(k > 0);
        let n = data[0].len();
        assert!(data.iter().all(|c| c.len() == n));
        assert!(data.iter().all(|c| c.iter().all(|v| v.is_finite()))); // ∈ D
        NodeField { data, k, n }
    }
}

#[derive(Clone, Debug)]
pub struct EdgeField {
    pub spatial: Vec<Vec<f64>>,          // spatial[c][e]
    pub comp: Vec<Vec<f64>>,             // comp[p][i]
    pub comp_pairs: Vec<(usize, usize)>, // declared component relations
    pub k: usize,
}

// ---- Operators (A, B, R, E) ---------------------------------------------

/// A — relational gradient extraction. NodeField → EdgeField over the
/// declared spatial relations and the declared component pairs.
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

/// B — local relational accumulation along declared continuation. Same
/// direction only; no cross-axis coupling. Terminal edges accumulate
/// nothing (no wraparound). Branch-point combination: sum (declared open).
pub fn operator_b(g: &EdgeField, rel: &DeclaredRelations) -> EdgeField {
    let spatial = g.spatial.iter().map(|s| {
        (0..rel.n_edges())
            .map(|e| s[e] + rel.succ(e).iter().map(|&f| s[f]).sum::<f64>())
            .collect()
    }).collect();
    // Component edges accumulate along downstream nodes (direction declared open).
    let comp = g.comp.iter().map(|c| {
        (0..rel.n_nodes)
            .map(|i| c[i] + rel.out[i].iter().map(|&e| c[rel.edges[e].1]).sum::<f64>())
            .collect()
    }).collect();
    EdgeField { spatial, comp, comp_pairs: g.comp_pairs.clone(), k: g.k }
}

/// ρ — per-node circulation strength from the local relational gradient
/// over incident edges; no aggregation beyond the node. Single ρ
/// (a ρ_spatial / ρ_component split is a declared open condition).
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

/// R — antisymmetric circulation over declared continuation.
/// Spatial: successor − predecessor asymmetry, stated over the declared
/// relations and over them only. This is the topology-general form; it
/// supersedes the earlier fixed-grid cross-axis coupling. The ring/torus
/// reductions are proof-construct identities and live in the proof papers,
/// not here (see header note). Verification on specific graph classes is
/// open. Cross-topology: spatial edges receive component-edge asymmetry;
/// component edges receive spatial-edge asymmetry. All coupling is local,
/// antisymmetric, additive.
pub fn operator_r(bg: &EdgeField, rel: &DeclaredRelations, rho: &[f64]) -> EdgeField {
    let k = bg.k;
    let pairs = &bg.comp_pairs;
    let rho_e: Vec<f64> = rel.edges.iter().map(|&(s, _)| rho[s]).collect(); // ρ at src (declared)

    // Spatial antisymmetric circulation.
    let mut spatial: Vec<Vec<f64>> = (0..k).map(|c| {
        (0..rel.n_edges()).map(|e| {
            let fwd: f64 = rel.succ(e).iter().map(|&f| bg.spatial[c][f]).sum();
            let bwd: f64 = rel.pred(e).iter().map(|&p| bg.spatial[c][p]).sum();
            bg.spatial[c][e] + rho_e[e] * (fwd - bwd)
        }).collect()
    }).collect();

    // Spatial edges receive component-edge asymmetry across the edge.
    let cc = 0.5;
    for (p, &(a, b)) in pairs.iter().enumerate() {
        for (e, &(s, d)) in rel.edges.iter().enumerate() {
            let asym = bg.comp[p][d] - bg.comp[p][s];
            spatial[a][e] += rho_e[e] * cc * asym;
            spatial[b][e] -= rho_e[e] * cc * asym;
        }
    }

    // Component edges receive spatial-edge asymmetry (representative:
    // net outgoing spatial edge at the node — declared open).
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

/// E — kernel composition: E(x, ρ) = R(B(A(x)), ρ(A(x))).
pub fn operator_e(f: &NodeField, rel: &DeclaredRelations, pairs: &[(usize, usize)], rho_base: f64) -> EdgeField {
    let a = operator_a(f, rel, pairs);
    let rho = compute_rho(&a, rel, rho_base);
    let b = operator_b(&a, rel);
    operator_r(&b, rel, &rho)
}

// ---- Minimal structural tests -------------------------------------------
// Relations are declared explicitly even here — no constructor hides them.
// Every structure below is open and irregular by choice. No ring is
// constructed: a ring would seed the inadmissible construct the header
// forbids (terminal→initial wraparound has no provenance, §16.3), and a
// test input is read as an example to imitate.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_boundary_is_not_wrapped() {
        let rel = DeclaredRelations::from_edges(3, vec![(0, 1), (1, 2)]); // path: last edge has no successor
        assert!(rel.succ(rel.n_edges() - 1).is_empty());
    }

    #[test]
    fn terminal_node_has_no_outgoing_edges() {
        // Fan-out / fan-in DAG: node 0 has out-degree 2, node 3 in-degree 2,
        // node 3 is terminal (out-degree 0). Irregular degree, open
        // boundary — the shape of a real declared structure, not a ring.
        let rel = DeclaredRelations::from_edges(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]);
        assert!(rel.out[3].is_empty());           // terminal: nothing leaves
        assert_eq!(rel.out[0].len(), 2);          // fan-out
        assert_eq!(rel.inc[3].len(), 2);          // fan-in
    }

    #[test]
    fn identical_components_zero_comp_edges() {
        // Property under test is independent of the declared structure.
        // Declared over the same open, irregular-degree DAG — not a ring.
        let rel = DeclaredRelations::from_edges(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]);
        let v: Vec<f64> = (0..4).map(|i| i as f64).collect();
        let f = NodeField::new(vec![v.clone(), v]);
        let e = operator_e(&f, &rel, &[(0, 1)], 0.3);
        assert!(e.comp[0].iter().all(|&x| x.abs() < 1e-9));
    }
}
