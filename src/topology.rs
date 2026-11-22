//! Kernel V10.3 — topology checks on a declared field.
//!
//! # T8: cycles in a declared arrangement (10.3.0, 2026-11-22)
//!
//! T8 states: no wraparound; feedback loops admissible with provenance;
//! opposite directions of a relation not in one field without their own
//! provenance. Origin's clarification of 2026-11-07 (`kernel_reference.md`
//! §8) places ring inadmissibility on relational evolution (an observation
//! sequence that closes onto itself), not on a cyclic arrangement of
//! independently declared relations, which is admissible with provenance.
//! The check reads both together:
//!
//! 1. **A self-relation (s, s) is refused.** It declares a locus as its own
//!    relational predecessor, the closing edge of §8, and its A is zero by
//!    identity, so it registers nothing.
//! 2. **A directed cycle is admitted only as a declared feedback loop.** At
//!    least one edge of every directed cycle must be declared feedback, each
//!    with a stated basis. Equivalently: the field with its declared feedback
//!    edges removed is acyclic. An undeclared cycle is refused, and the
//!    refusal names one directed cycle per strongly connected part.
//! 3. **Opposite directions** (s, t) and (t, s) form a 2-cycle, so rule 2
//!    covers them: the reverse needs its own stated basis (the distinctness
//!    axiom, §8).
//!
//! A declared feedback edge must lie on a directed cycle of the field (its
//! target reaches its source); otherwise it closes nothing and is refused.
//! Admission reports, for each feedback edge, one directed cycle it closes,
//! so the provenance of every loop is in the output. Nothing is repaired: a
//! refusal returns to the declaration (W5). The observation sequence is not
//! checked here: its order is the record's (ingest).
//!
//! `DeclaredRelations::from_edges_with_provenance` does not run this check;
//! `declare_checked` constructs and checks together.

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
    /// A directed cycle with no declared feedback edge. Edges in order of
    /// traversal along their declared direction.
    UndeclaredCycle { cycle: Vec<usize> },
    /// A feedback declaration names an edge that is not declared.
    FeedbackEdgeOutOfRange { edge: usize },
    /// The same edge declared feedback twice.
    DuplicateFeedback { edge: usize },
    /// A feedback declaration with an empty basis.
    FeedbackWithoutBasis { edge: usize },
    /// A feedback edge on no directed cycle: it closes nothing.
    FeedbackNotOnCycle { edge: usize },
}

/// An admitted field: its feedback edges and, for each, one directed cycle
/// it closes (the edge itself first).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct T8Admission {
    pub feedback: Vec<usize>,
    pub closes: Vec<Vec<usize>>,
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
    let mut closes = Vec::new();
    for &f in &fb {
        let (s, t) = rel.edges[f];
        if s == t { continue; }
        match path(rel, t, s, &|e| not_self(e)) {
            Some(p) => { let mut c = vec![f]; c.extend(p); closes.push(c); }
            None => refusals.push(T8Refusal::FeedbackNotOnCycle { edge: f }),
        }
    }
    let residual = |e: usize| not_self(e) && !fb.contains(&e);
    for part in strong_parts(rel, &residual) {
        if part.len() < 2 { continue; }
        // One directed cycle inside the part: an edge within it, closed back.
        let inside = |e: usize| residual(e) && part.contains(&rel.edges[e].0) && part.contains(&rel.edges[e].1);
        let e0 = (0..rel.n_edges()).find(|&e| inside(e)).unwrap();
        let mut cycle = vec![e0];
        cycle.extend(path(rel, rel.edges[e0].1, rel.edges[e0].0, &inside).unwrap());
        refusals.push(T8Refusal::UndeclaredCycle { cycle });
    }
    if refusals.is_empty() {
        let mut feedback = fb.clone();
        let mut order: Vec<usize> = (0..feedback.len()).collect();
        order.sort_by_key(|&i| feedback[i]);
        closes = order.iter().map(|&i| closes[i].clone()).collect();
        feedback.sort();
        Ok(T8Admission { feedback, closes })
    } else {
        Err(refusals)
    }
}

/// Construct a declared field and check it under T8 together.
pub fn declare_checked(
    n_nodes: usize, edges: Vec<(usize, usize)>, provenance: Vec<EdgeProvenance>, feedback: &[FeedbackDeclaration],
) -> Result<(DeclaredRelations, T8Admission), Vec<T8Refusal>> {
    let rel = DeclaredRelations::from_edges_with_provenance(n_nodes, edges, provenance);
    let admitted = check_t8(&rel, feedback)?;
    Ok((rel, admitted))
}
