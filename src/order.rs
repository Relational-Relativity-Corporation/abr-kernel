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

/// A degree of freedom of the current point (Origin, 2026-11-26, 108th): an
/// earlier point where the order stood at the current locus, a distinct
/// revolution of the helix (106th, 107th), never another occurrence of the
/// same route. Every part is an identity in the order; none is a count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Degree {
    /// The earlier point in evolution (point j: where step j − 1 entered).
    pub point: usize,
    /// The relation that left it there, `steps[point]`, as registered. No
    /// operator forms it. *Was:* described as the readback's leaving half
    /// (K9), which the code does not compute (corrected, 2026-11-26).
    pub leaving: usize,
    /// Its turn to now: the span of the order `point .. n`, the relations
    /// registered between it and the current point.
    pub turn: (usize, usize),
    /// The span its arrival runs alongside the current arrival: `steps[a..b]`
    /// (b = point) equals, relation by relation, the last b − a steps.
    pub alongside: (usize, usize),
    /// Where the two arrivals part: (the relation there, the relation now);
    /// `None` where the earlier arrival reaches the record boundary.
    pub parts: Option<(usize, usize)>,
}

impl Order {
    /// The degrees of freedom of the current point (108th): every earlier
    /// point at the current locus, in order of evolution, each kept
    /// distinct; nothing is merged and nothing is counted.
    pub fn degrees(&self) -> Vec<Degree> {
        let n = self.steps.len();
        let here = self.locus(n);
        (0..n).filter(|&j| self.locus(j) == here).map(|j| {
            let mut k = 0;
            while k < j && self.steps[j - 1 - k] == self.steps[n - 1 - k] { k += 1; }
            let parts = if k == j { None } else { Some((self.steps[j - 1 - k], self.steps[n - 1 - k])) };
            Degree { point: j, leaving: self.steps[j], turn: (j, n), alongside: (j - k, j), parts }
        }).collect()
    }

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
    /// point shares. **Fenced as a check (Origin, 108th):** it counts the
    /// earlier points at the same route as one, which reads repetition
    /// (106th); the reading is `degrees`, which keeps every point distinct.
    /// Kept to confirm the degrees (`tests/v10_degrees.rs`, 4).
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
