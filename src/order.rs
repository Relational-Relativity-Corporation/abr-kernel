// order.rs — Metatron Dynamics, Inc. Kernel V10.
// The order is the relational field (Origin, 2026-11-26, 102nd).
// Bounded over D. No claim beyond D.
//
// Origin (102nd): "the order is the relational field the order of
// information read from left to right including all character spaces and
// punctuations". An order is the registered sequence of relations, one per
// step, each continuing from the locus the previous step entered: one
// direction, nothing cut (Df2, Df3, the 85th).
//
// The possibilities at the current point (the locus the last step entered)
// are the relations the entire order registered after the route arriving
// there, at every length of route the order carries: route length 0 is the
// bare current locus (what a summed field gives); each longer route keeps
// only the earlier occurrences that also match one step further back. Each
// level reports its route, every occurrence and each continuation's count.
// No weight or decay by distance (74th): every earlier step counts exactly,
// through the route it belongs to. The choice among the possibilities is not
// made here (102nd (3), open).
//
// [IMPL] Legacy mathematics, stated (92nd; `derivation.rs`): reading the
// continuations of a route is what a suffix index or a variable-order
// context model computes (Weiner, 1973; Rissanen, 1983). Here it is the
// direct reading of the field Origin declared. Work: one pass over the order
// for route length 0, then each longer route filters the occurrences of the
// shorter, so the work grows with the order and the occurrences, never with
// the square of the order.

use crate::operators::DeclaredRelations;

/// Why an order is refused (never repaired).
#[derive(Debug, Clone, PartialEq)]
pub enum OrderRefusal {
    /// No step: an order with no current point.
    Empty,
    /// A step names a relation that is not declared.
    RelationOutOfRange { step: usize, relation: usize },
    /// A step does not continue from the locus the previous step entered.
    Breaks { step: usize },
    /// A declared relation lacks its identity or basis (RPPI-V10 G-1).
    Identity(Vec<crate::topology::IdentityRefusal>),
}

/// A registered order of relations over declared relations.
#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    /// Each relation as (the locus it leaves, the locus it enters).
    pub edges: Vec<(usize, usize)>,
    /// The order: the relation registered at each step, oldest first.
    pub steps: Vec<usize>,
}

/// The possibilities after a route of one length.
#[derive(Debug, Clone, PartialEq)]
pub struct Level {
    /// The route arriving at the current point: its relations, oldest first
    /// (empty at length 0).
    pub route: Vec<usize>,
    /// Every earlier point where the same route arrived, as the step that
    /// followed it there.
    pub occurrences: Vec<usize>,
    /// Each relation that followed the route, with how many times, in
    /// increasing order of relation.
    pub continuations: Vec<(usize, usize)>,
}

impl Order {
    /// Admit an order over the declared relations. Every relation must carry
    /// its identity and basis. T8 is not applied: it governs forming B and Ξ
    /// over turns, and this reading forms neither; a self-relation (a
    /// doubled character) is the 76th's identity spoke on the occurrence
    /// chain, and the order is that chain, so it is kept, never cut.
    pub fn new(rel: &DeclaredRelations, steps: Vec<usize>) -> Result<Self, OrderRefusal> {
        crate::topology::check_identity(rel).map_err(OrderRefusal::Identity)?;
        if steps.is_empty() { return Err(OrderRefusal::Empty); }
        for (i, &e) in steps.iter().enumerate() {
            if e >= rel.n_edges() { return Err(OrderRefusal::RelationOutOfRange { step: i, relation: e }); }
            if i > 0 && rel.edges[steps[i - 1]].1 != rel.edges[e].0 { return Err(OrderRefusal::Breaks { step: i }); }
        }
        Ok(Order { edges: rel.edges.clone(), steps })
    }

    /// The locus at point j of the order (point 0 is where the first step
    /// leaves; point j is where step j − 1 enters).
    fn locus(&self, j: usize) -> usize {
        if j == 0 { self.edges[self.steps[0]].0 } else { self.edges[self.steps[j - 1]].1 }
    }

    /// The possibilities at the current point, at every length of route the
    /// order carries, shortest first; it ends at the first length no earlier
    /// point shares.
    pub fn possibilities(&self) -> Vec<Level> {
        let n = self.steps.len();
        let here = self.locus(n);
        // Route length 0: every earlier point at the current locus.
        let mut occ: Vec<usize> = (0..n).filter(|&j| self.locus(j) == here).collect();
        let mut levels = Vec::new();
        let mut len = 0;
        while !occ.is_empty() {
            let mut counts: Vec<(usize, usize)> = Vec::new();
            for &j in &occ {
                let e = self.steps[j];
                match counts.iter_mut().find(|x| x.0 == e) { Some(x) => x.1 += 1, None => counts.push((e, 1)) }
            }
            counts.sort_unstable();
            levels.push(Level { route: self.steps[n - len..].to_vec(), occurrences: occ.clone(), continuations: counts });
            // One step further back: keep the points whose route also matches there.
            if len == n { break; }
            let back = self.steps[n - 1 - len];
            occ.retain(|&j| j > len && self.steps[j - 1 - len] == back);
            len += 1;
        }
        levels
    }
}
