//! Test support (synthetic only): declare feedback on a generated field so it
//! passes T8 by declaration, never by bypassing the check. Every edge (s, t)
//! with s > t that lies on a directed cycle is declared feedback with a basis
//! marked SYNTHETIC. The remaining edges either run s < t or lie on no cycle,
//! so the field less its feedback is acyclic. A registered record declares its
//! feedback from its own observing chain instead.

use abr_kernel::operators::DeclaredRelations;
use abr_kernel::topology::FeedbackDeclaration;

fn reaches(r: &DeclaredRelations, from: usize, to: usize) -> bool {
    let (mut seen, mut stack) = (vec![false; r.n_nodes], vec![from]);
    while let Some(v) = stack.pop() {
        if v == to { return true; }
        if seen[v] { continue; }
        seen[v] = true;
        for &e in &r.out[v] { stack.push(r.edges[e].1); }
    }
    false
}

pub fn declare_loops(r: DeclaredRelations) -> DeclaredRelations {
    let fb = (0..r.n_edges())
        .filter(|&e| { let (s, t) = r.edges[e]; s > t && reaches(&r, t, s) })
        .map(|edge| FeedbackDeclaration { edge, basis: "SYNTHETIC: generated test field".into() })
        .collect();
    r.with_feedback(fb)
}
