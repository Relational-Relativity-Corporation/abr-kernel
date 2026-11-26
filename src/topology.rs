//! Kernel V10.3 — topology checks on a declared field.
//!
//! # T8: turns in a declared arrangement (10.3.0, 2026-11-22)
//!
//! T8 states: no wraparound; feedback loops admissible with provenance;
//! opposite directions of a relation not in one field without their own
//! provenance. Origin's clarification of 2026-11-07 (`kernel_reference.md`
//! §8) places ring inadmissibility on relational evolution (an observation
//! sequence that comes back onto itself, which no information can do,
//! 106th), not on a loop-shaped arrangement of independently declared
//! relations, which is admissible with provenance: a turn is shape, never a
//! system that is a loop (107th).
//! The check reads both together:
//!
//! 1. **A self-relation (s, s) is refused.** It declares a locus as its own
//!    relational predecessor, the returning edge of §8, and its A is zero by
//!    identity, so it registers nothing.
//! 2. **A directed turn is admitted only as a declared feedback loop.** At
//!    least one edge of every directed turn must be declared feedback, each
//!    with a stated basis. Equivalently: the field with its declared feedback
//!    edges removed has no directed turn. An undeclared turn is refused, and the
//!    refusal names one directed turn per strongly connected part.
//! 3. **Opposite directions** (s, t) and (t, s) form a 2-turn, so rule 2
//!    covers them: the reverse needs its own stated basis (the distinctness
//!    axiom, §8).
//!
//! A declared feedback edge must lie on a directed turn of the field (its
//! target reaches its source); otherwise it lies on no turn and is refused.
//! Admission reports, for each feedback edge, one directed turn it lies on,
//! so the provenance of every loop is in the output. Nothing is repaired: a
//! refusal returns to the declaration (W5). The observation sequence is not
//! checked here: its order is the record's (ingest).
//!
//! **Enforcement (Origin, 2026-11-22, 28th): at the reading entry points.**
//! `DeclaredRelations::from_edges_with_provenance` stays a building block and
//! does not run the check. Feedback is declared on the field with
//! `DeclaredRelations::with_feedback`, and every V10 reading entry point
//! (`RelationalField::new`, `EdgeSeries::from_registrations`, the ground
//! readings: connection, minor histories, the first V2 form, energy) refuses
//! a field that fails T8 with `DeclarationGap::T8`. An edge-valued series
//! (`EdgeSeries::from_edge_values`) declares no topology and is not checked.
//! `declare_checked` constructs, declares the feedback and checks together.

use crate::ground_variables::DeclarationGap;
use crate::operators::{DeclaredRelations, EdgeProvenance};
use std::collections::VecDeque;

/// One edge declared as feedback, with its basis (T8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeedbackDeclaration {
    pub edge: usize,
    /// Where the feedback relation is registered or declared from.
    pub basis: String,
}

/// Why a declared field was refused under T8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum T8Refusal {
    /// (s, s): a locus declared as its own predecessor.
    SelfRelation { edge: usize },
    /// A directed turn with no declared feedback edge. Edges in order of
    /// traversal along their declared direction.
    UndeclaredTurn { turn: Vec<usize> },
    /// A feedback declaration names an edge that is not declared.
    FeedbackEdgeOutOfRange { edge: usize },
    /// The same edge declared feedback twice.
    DuplicateFeedback { edge: usize },
    /// A feedback declaration with an empty basis.
    FeedbackWithoutBasis { edge: usize },
    /// A feedback edge on no directed turn.
    FeedbackNotOnTurn { edge: usize },
}

/// An admitted field: its feedback edges and, for each, one directed turn
/// it lies on (the edge itself first). *Was:* the field `closes` (107th).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct T8Admission {
    pub feedback: Vec<usize>,
    pub turns: Vec<Vec<usize>>,
}

/// Shortest directed edge path from `from` to `to` over `allowed` edges.
fn path(rel: &DeclaredRelations, from: usize, to: usize, allowed: &dyn Fn(usize) -> bool) -> Option<Vec<usize>> {
    if from == to { return Some(Vec::new()); }
    let mut via: Vec<Option<usize>> = vec![None; rel.n_nodes];
    let mut seen = vec![false; rel.n_nodes];
    seen[from] = true;
    let mut q = VecDeque::from([from]);
    while let Some(v) = q.pop_front() {
        for &e in &rel.out[v] {
            if !allowed(e) { continue; }
            let w = rel.edges[e].1;
            if seen[w] { continue; }
            seen[w] = true;
            via[w] = Some(e);
            if w == to {
                let mut p = Vec::new();
                let mut u = to;
                while u != from { let e = via[u].unwrap(); p.push(e); u = rel.edges[e].0; }
                p.reverse();
                return Some(p);
            }
            q.push_back(w);
        }
    }
    None
}

/// Strongly connected parts over `allowed` edges (Tarjan, iterative).
fn strong_parts(rel: &DeclaredRelations, allowed: &dyn Fn(usize) -> bool) -> Vec<Vec<usize>> {
    let n = rel.n_nodes;
    let (mut index, mut low, mut on) = (vec![usize::MAX; n], vec![0; n], vec![false; n]);
    let (mut next, mut stack, mut parts) = (0, Vec::new(), Vec::new());
    for root in 0..n {
        if index[root] != usize::MAX { continue; }
        let mut work: Vec<(usize, usize)> = vec![(root, 0)];
        index[root] = next; low[root] = next; next += 1; stack.push(root); on[root] = true;
        while let Some(&mut (v, ref mut i)) = work.last_mut() {
            if *i < rel.out[v].len() {
                let e = rel.out[v][*i];
                *i += 1;
                if !allowed(e) { continue; }
                let w = rel.edges[e].1;
                if index[w] == usize::MAX {
                    index[w] = next; low[w] = next; next += 1; stack.push(w); on[w] = true;
                    work.push((w, 0));
                } else if on[w] {
                    low[v] = low[v].min(index[w]);
                }
            } else {
                work.pop();
                if let Some(&(u, _)) = work.last() { low[u] = low[u].min(low[v]); }
                if low[v] == index[v] {
                    let mut part = Vec::new();
                    loop { let w = stack.pop().unwrap(); on[w] = false; part.push(w); if w == v { break; } }
                    part.sort();
                    parts.push(part);
                }
            }
        }
    }
    parts.sort();
    parts
}

/// The T8 check of a declared field with its feedback declarations.
pub fn check_t8(rel: &DeclaredRelations, feedback: &[FeedbackDeclaration]) -> Result<T8Admission, Vec<T8Refusal>> {
    let mut refusals = Vec::new();
    for (e, &(s, t)) in rel.edges.iter().enumerate() {
        if s == t { refusals.push(T8Refusal::SelfRelation { edge: e }); }
    }
    let mut fb: Vec<usize> = Vec::new();
    for d in feedback {
        if d.edge >= rel.n_edges() { refusals.push(T8Refusal::FeedbackEdgeOutOfRange { edge: d.edge }); continue; }
        if fb.contains(&d.edge) { refusals.push(T8Refusal::DuplicateFeedback { edge: d.edge }); continue; }
        if d.basis.trim().is_empty() { refusals.push(T8Refusal::FeedbackWithoutBasis { edge: d.edge }); }
        fb.push(d.edge);
    }
    let not_self = |e: usize| rel.edges[e].0 != rel.edges[e].1;
    let mut turns = Vec::new();
    for &f in &fb {
        let (s, t) = rel.edges[f];
        if s == t { continue; }
        match path(rel, t, s, &|e| not_self(e)) {
            Some(p) => { let mut c = vec![f]; c.extend(p); turns.push(c); }
            None => refusals.push(T8Refusal::FeedbackNotOnTurn { edge: f }),
        }
    }
    let residual = |e: usize| not_self(e) && !fb.contains(&e);
    for part in strong_parts(rel, &residual) {
        if part.len() < 2 { continue; }
        // One directed turn inside the part: an edge within it, turning back.
        let inside = |e: usize| residual(e) && part.contains(&rel.edges[e].0) && part.contains(&rel.edges[e].1);
        let e0 = (0..rel.n_edges()).find(|&e| inside(e)).unwrap();
        let mut turn = vec![e0];
        turn.extend(path(rel, rel.edges[e0].1, rel.edges[e0].0, &inside).unwrap());
        refusals.push(T8Refusal::UndeclaredTurn { turn });
    }
    if refusals.is_empty() {
        let mut feedback = fb.clone();
        let mut order: Vec<usize> = (0..feedback.len()).collect();
        order.sort_by_key(|&i| feedback[i]);
        turns = order.iter().map(|&i| turns[i].clone()).collect();
        feedback.sort();
        Ok(T8Admission { feedback, turns })
    } else {
        Err(refusals)
    }
}

/// Construct a declared field and check it under T8 together.
pub fn declare_checked(
    n_nodes: usize, edges: Vec<(usize, usize)>, provenance: Vec<EdgeProvenance>, feedback: &[FeedbackDeclaration],
) -> Result<(DeclaredRelations, T8Admission), Vec<T8Refusal>> {
    let rel = DeclaredRelations::from_edges_with_provenance(n_nodes, edges, provenance).with_feedback(feedback.to_vec());
    let admitted = check_t8(&rel, feedback)?;
    Ok((rel, admitted))
}

// ═══════════════════════════════════════════════════════════════════════════
// Relation identity (RPPI-V10 G-1, adopted by Origin 2026-11-22, 31st)
// ═══════════════════════════════════════════════════════════════════════════
//
// A provenance label (`EdgeProvenance`) names a class; it does not identify
// a declared observable relation (RPPI, `v9_specification.md` §2). In V10
// every declared edge carries its identity and its direction basis: the id
// by which the record names the relation, and the observable property of the
// process from which its direction was declared (Df2, T2). A field whose
// edges do not each carry a non-empty, distinct id and a non-empty basis is
// refused at every V10 reading entry point. Parallel edges are distinct
// relations and carry distinct ids (§8, distinctness axiom).

/// The identity and direction basis of one declared relation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationIdentity {
    /// The record's id for the relation.
    pub id: String,
    /// The observable property of the process the direction is declared from.
    pub basis: String,
}

/// Why a field's relation identities were refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityRefusal {
    /// No identity declared for the field's edges.
    NotDeclared,
    /// Identities declared for a different number of edges.
    CountMismatch { declared: usize, edges: usize },
    EmptyId { edge: usize },
    EmptyBasis { edge: usize },
    /// Two edges carry the same id: `edge` repeats the id of `first`.
    DuplicateId { edge: usize, first: usize },
}

/// The identity check of a declared field.
pub fn check_identity(rel: &DeclaredRelations) -> Result<(), Vec<IdentityRefusal>> {
    if rel.identity.is_empty() { return Err(vec![IdentityRefusal::NotDeclared]); }
    if rel.identity.len() != rel.n_edges() {
        return Err(vec![IdentityRefusal::CountMismatch { declared: rel.identity.len(), edges: rel.n_edges() }]);
    }
    let mut out = Vec::new();
    for (e, r) in rel.identity.iter().enumerate() {
        if r.id.trim().is_empty() { out.push(IdentityRefusal::EmptyId { edge: e }); }
        else if let Some(first) = rel.identity[..e].iter().position(|q| q.id.trim() == r.id.trim()) {
            out.push(IdentityRefusal::DuplicateId { edge: e, first });
        }
        if r.basis.trim().is_empty() { out.push(IdentityRefusal::EmptyBasis { edge: e }); }
    }
    if out.is_empty() { Ok(()) } else { Err(out) }
}

/// Admission of a declared field at a V10 reading entry point: relation
/// identity (RPPI-V10 G-1), then T8.
pub fn admit_field(rel: &DeclaredRelations) -> Result<(), DeclarationGap> {
    check_identity(rel).map_err(DeclarationGap::RelationIdentity)?;
    t8_gap(rel)
}

/// T8 at a reading entry point: the field with its own declared feedback.
pub fn t8_gap(rel: &DeclaredRelations) -> Result<(), DeclarationGap> {
    check_t8(rel, &rel.feedback).map(|_| ()).map_err(DeclarationGap::T8)
}

// ═══════════════════════════════════════════════════════════════════════════
// K7: routes and a turn basis from a stated spanning tree (10.3.0)
// ═══════════════════════════════════════════════════════════════════════════
//
// K7 (Declared, Origin 2026-11-16; general form HB-1, 2026-11-17): a route
// π = (e₁, …, e_m) is a directed sequence of declared edges; for two routes
// with the same source and target, A_π = B_π₁(g) − B_π₂(g), which is the
// turn sum of π₁ followed by π₂ traversed against its direction. The
// independent turn sums are a turn basis from a **stated** spanning tree:
// each edge not in the tree (a chord) forms one fundamental turn with the
// tree path between its ends. There are m − n + c of them (m edges, n loci,
// c connected parts). Traversal and sign are orders of analysis (Df2a,
// OC-K-3a): reversing a turn negates its turn sum and changes no reading.
//
// Every turn's signed edge count is the sum, over chords, of the
// chord's net count times its fundamental turn, so its turn sum is that
// integer combination of the basis turn sums, exactly (`express_in_basis`).
// The tree is declared, not chosen by the kernel: a different stated tree
// gives a different basis of the same turn space.

/// A directed route: edges in order, each starting where the last ended.
pub type Route = Vec<usize>;

/// Routes from one locus to another, up to a stated limit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteEnumeration {
    pub routes: Vec<Route>,
    /// False when the limit was reached before every route was listed.
    pub complete: bool,
}

/// Every simple directed route (no locus repeated) from `from` to `to`,
/// listing at most `limit`, sorted shortest first and then by edge order. The
/// count of routes can grow combinatorially, so the limit is stated by the
/// caller and `complete` reports whether the list is exhaustive. When it is
/// not, which routes are listed follows the depth-first order of the declared
/// edges, not their length. Parallel edges are distinct routes of length one.
pub fn routes(rel: &DeclaredRelations, from: usize, to: usize, limit: usize) -> RouteEnumeration {
    assert!(from < rel.n_nodes && to < rel.n_nodes, "route ends must be declared loci");
    let mut found: Vec<Route> = Vec::new();
    let mut complete = true;
    if from != to {
        let mut on = vec![false; rel.n_nodes];
        let mut route = Vec::new();
        fn walk(rel: &DeclaredRelations, v: usize, to: usize, on: &mut Vec<bool>, route: &mut Vec<usize>,
                found: &mut Vec<Route>, limit: usize, complete: &mut bool) {
            if v == to { if found.len() < limit { found.push(route.clone()); } else { *complete = false; } return; }
            on[v] = true;
            for &e in &rel.out[v] {
                if !*complete { break; }
                let w = rel.edges[e].1;
                if on[w] { continue; }
                route.push(e);
                walk(rel, w, to, on, route, found, limit, complete);
                route.pop();
            }
            on[v] = false;
        }
        walk(rel, from, to, &mut on, &mut route, &mut found, limit, &mut complete);
    }
    found.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    RouteEnumeration { routes: found, complete }
}

/// True when `route` is a non-empty directed sequence of declared edges.
pub fn is_route(rel: &DeclaredRelations, route: &[usize]) -> bool {
    !route.is_empty() && route.iter().all(|&e| e < rel.n_edges())
        && route.windows(2).all(|w| rel.edges[w[0]].1 == rel.edges[w[1]].0)
}

/// The turn of a K7 two-route turn sum: π₁ along its direction, then
/// π₂ against it. `None` unless both are routes with the same source and
/// target. Its turn sum is B_π₁(g) − B_π₂(g).
pub fn two_route_turn(rel: &DeclaredRelations, p1: &[usize], p2: &[usize]) -> Option<Vec<(usize, i8)>> {
    if !is_route(rel, p1) || !is_route(rel, p2) { return None; }
    let ends = |p: &[usize]| (rel.edges[p[0]].0, rel.edges[*p.last().unwrap()].1);
    if ends(p1) != ends(p2) { return None; }
    let mut c: Vec<(usize, i8)> = p1.iter().map(|&e| (e, 1)).collect();
    c.extend(p2.iter().rev().map(|&e| (e, -1)));
    Some(c)
}

/// Why a stated spanning tree was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeRefusal {
    EdgeOutOfRange { edge: usize },
    DuplicateEdge { edge: usize },
    /// The stated edges close an (undirected) turn: not a tree.
    FormsTurn { edge: usize },
    /// Loci joined by declared edges but not by the stated tree.
    DoesNotSpan { loci: (usize, usize) },
}

/// A turn basis from a stated spanning tree: one fundamental turn per chord.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnBasis {
    pub tree: Vec<usize>,
    /// Edges not in the tree, ascending; `turns[i]` is formed by `chords[i]`.
    pub chords: Vec<usize>,
    /// Signed turns, each starting with its chord along its direction.
    pub turns: Vec<Vec<(usize, i8)>>,
}

fn find(p: &mut [usize], v: usize) -> usize { let mut v = v; while p[v] != v { p[v] = p[p[v]]; v = p[v]; } v }

/// The fundamental turns of a stated spanning tree (or forest, one tree per
/// connected part). Refused, not repaired, when the stated edges are not a
/// spanning tree of the declared field.
pub fn turn_basis(rel: &DeclaredRelations, tree: &[usize]) -> Result<TurnBasis, Vec<TreeRefusal>> {
    let mut refusals = Vec::new();
    let mut parent: Vec<usize> = (0..rel.n_nodes).collect();
    let mut t_edges: Vec<usize> = Vec::new();
    for &e in tree {
        if e >= rel.n_edges() { refusals.push(TreeRefusal::EdgeOutOfRange { edge: e }); continue; }
        if t_edges.contains(&e) { refusals.push(TreeRefusal::DuplicateEdge { edge: e }); continue; }
        let (s, t) = rel.edges[e];
        let (a, b) = (find(&mut parent, s), find(&mut parent, t));
        if a == b { refusals.push(TreeRefusal::FormsTurn { edge: e }); continue; }
        parent[a] = b;
        t_edges.push(e);
    }
    for e in 0..rel.n_edges() {
        let (s, t) = rel.edges[e];
        if find(&mut parent, s) != find(&mut parent, t) {
            let pair = (s.min(t), s.max(t));
            if !refusals.contains(&TreeRefusal::DoesNotSpan { loci: pair }) {
                refusals.push(TreeRefusal::DoesNotSpan { loci: pair });
            }
        }
    }
    if !refusals.is_empty() { return Err(refusals); }
    // Tree adjacency, undirected, with the sign of travelling each edge.
    let mut adj: Vec<Vec<(usize, usize, i8)>> = vec![Vec::new(); rel.n_nodes];
    for &e in &t_edges {
        let (s, t) = rel.edges[e];
        adj[s].push((t, e, 1));
        adj[t].push((s, e, -1));
    }
    let tree_path = |from: usize, to: usize| -> Vec<(usize, i8)> {
        let mut via: Vec<Option<(usize, usize, i8)>> = vec![None; rel.n_nodes];
        let mut seen = vec![false; rel.n_nodes];
        seen[from] = true;
        let mut q = VecDeque::from([from]);
        while let Some(v) = q.pop_front() {
            if v == to { break; }
            for &(w, e, sg) in &adj[v] {
                if !seen[w] { seen[w] = true; via[w] = Some((v, e, sg)); q.push_back(w); }
            }
        }
        let mut p = Vec::new();
        let mut u = to;
        while u != from { let (v, e, sg) = via[u].unwrap(); p.push((e, sg)); u = v; }
        p.reverse();
        p
    };
    let chords: Vec<usize> = (0..rel.n_edges()).filter(|e| !t_edges.contains(e)).collect();
    let turns = chords.iter().map(|&e| {
        let (s, t) = rel.edges[e];
        let mut c = vec![(e, 1i8)];
        c.extend(tree_path(t, s));
        c
    }).collect();
    let mut tree_sorted = t_edges;
    tree_sorted.sort();
    Ok(TurnBasis { tree: tree_sorted, chords, turns })
}

/// The coefficients of a turn in the basis: for each chord, its net
/// signed count in the walk. The walk's turn sum equals Σ coefficient ×
/// fundamental-turn sum, exactly. `None` when `walk` is not a turn.
pub fn express_in_basis(rel: &DeclaredRelations, basis: &TurnBasis, walk: &[(usize, i8)]) -> Option<Vec<i64>> {
    if !crate::persistence_change::is_turn(rel, walk) { return None; }
    Some(basis.chords.iter().map(|&c| walk.iter().filter(|w| w.0 == c).map(|w| w.1 as i64).sum()).collect())
}

/// The turn sum of every fundamental turn of `basis` at registration t in
/// component c, (value, half-width) each, in chord order. Each holds at δ
/// when |value| ≤ half-width. On a node-derived series every value is 0 by
/// identity; on edge-registered values it is a test (K7).
pub fn basis_turn_sums(series: &crate::persistence_change::EdgeSeries, rel: &DeclaredRelations, basis: &TurnBasis,
                      t: usize, c: usize) -> Vec<(f64, f64)> {
    basis.turns.iter().map(|cy| crate::persistence_change::turn_sum(series, rel, cy, t, c).unwrap()).collect()
}

// ── The old names, kept as deprecated aliases (Origin, 2026-11-26, 107th):
// a cycle is a turn, one revolution of the helix a route traces as
// evolution carries it forward; a closure is the turn sum.
#[deprecated(note = "renamed TurnBasis (Origin, 107th)")]
pub type CycleBasis = TurnBasis;
#[deprecated(note = "renamed turn_basis (Origin, 107th)")]
pub fn cycle_basis(rel: &DeclaredRelations, tree: &[usize]) -> Result<TurnBasis, Vec<TreeRefusal>> { turn_basis(rel, tree) }
#[deprecated(note = "renamed two_route_turn (Origin, 107th)")]
pub fn two_route_cycle(rel: &DeclaredRelations, p1: &[usize], p2: &[usize]) -> Option<Vec<(usize, i8)>> { two_route_turn(rel, p1, p2) }
#[deprecated(note = "renamed basis_turn_sums (Origin, 107th)")]
pub fn basis_closures(series: &crate::persistence_change::EdgeSeries, rel: &DeclaredRelations, basis: &TurnBasis, t: usize, c: usize) -> Vec<(f64, f64)> { basis_turn_sums(series, rel, basis, t, c) }
