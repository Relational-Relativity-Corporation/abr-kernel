// relational_field.rs — Metatron Dynamics, Inc. Kernel V10.
// Df6, simultaneity (Origin, 2026-11-21). Bounded over D. No claim beyond D.
//
// ── Df6 ───────────────────────────────────────────────────────────────────
//
// All operators and all ground variables occur simultaneously throughout
// relational evolution. A, B, the circulation Ξ, their persistence
// differences, and energy, freedom and connection are aspects of one ongoing
// relational evolution, not stages. The composition order and the sequence of
// registrations are orders of analysis (Df2a). The observer intersects the
// system (R10). The only limit on a reading is declared information: too few
// registrations, too few components, too coarse a δ, or no declared carrier.
//
// Consequences in this module:
//
//   1. There are no phases. The spatial forms and their persistence are
//      evaluated together on every relational field, at every registration
//      and every consecutive step.
//
//   2. A relational field is formed from a declared sequence of at least two
//      consecutive registrations of the same declared relations, ordered by
//      an observable property of the process (T6). One registration is
//      insufficient information: nothing about relational evolution can
//      terminate in one observation.
//
//   3. `RelationalField::new` evaluates everything together, in one call.
//
// ── No operator parameters (Origin, 2026-11-21) ───────────────────────────
//
// ρ and χ (ρ_base, χ₀, the max selection, the saturating form) and the
// component coupling cc[p] are retired from V10. A relational field is formed
// from the declared relations and registrations alone; nothing is weighted by
// a value without an observable correspondence. At each registration t:
//
//     A_t = A(x_t)        B_t = B(A_t)        Ξ_t = Ξ(B_t)
//
// and over each consecutive step (prior always the earlier registration, Df2):
//
//     A_p = A_{t+1} − A_t      B_p = B(A_p)      Ξ_p = Ξ(B_p)
//
// Because A, B and Ξ are linear, B_p = B_{t+1} − B_t and Ξ_p = Ξ_{t+1} − Ξ_t
// exactly. The V9 nonlinearity came entirely from ρ. Energy's effect on the
// structure is now read from the data (the minor pattern), not modelled.
//
// Two registrations are the minimum information, not a guarantee of enough.

use crate::ground_variables::{ground_reading, DeclarationGap, EnergyCarrier, GroundReading, Registration};
use crate::operators::{operator_a, operator_b, operator_circulation, DeclaredRelations, EdgeField};

/// A, B and Ξ at one registration: spatial[c][e] per component and edge.
#[derive(Debug, Clone, PartialEq)]
pub struct Forms {
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    /// Ξ, the circulation.
    pub circulation: Vec<Vec<f64>>,
}

fn forms_of(a: Vec<Vec<f64>>, rel: &DeclaredRelations) -> Forms {
    let k = a.len();
    let b = operator_b(&EdgeField { spatial: a.clone(), comp: vec![], comp_pairs: vec![], k }, rel).spatial;
    let circulation = operator_circulation(&b, rel);
    Forms { a, b, circulation }
}

/// A relational field: one declared intersection with an ongoing relational
/// evolution, evaluated as a whole.
#[derive(Debug, Clone)]
pub struct RelationalField {
    /// The declared sequence of registrations.
    pub registrations: Vec<Registration>,
    /// A, B, Ξ at each registration.
    pub spatial: Vec<Forms>,
    /// A_p, B_p, Ξ_p over each consecutive step; `persistence[t]` pairs
    /// registration t (prior) with registration t + 1 (current).
    pub persistence: Vec<Forms>,
    /// Energy, freedom and connection over the whole sequence.
    pub ground: GroundReading,
}

impl RelationalField {
    /// Form the relational field from a declared sequence. Fewer than two
    /// registrations is insufficient information: `NoProcessSequence`.
    pub fn new(
        rel: &DeclaredRelations,
        registrations: Vec<Registration>,
        carriers: Option<&[EnergyCarrier]>,
    ) -> Result<Self, DeclarationGap> {
        if registrations.len() < 2 {
            return Err(DeclarationGap::NoProcessSequence);
        }
        crate::topology::t8_gap(rel)?;
        let k = registrations[0].x.k;
        assert!(registrations.iter().all(|r| r.x.k == k),
            "every registration must declare the same components");
        assert!(registrations.iter().all(|r| r.x.n == rel.n_nodes),
            "every registration must cover the declared loci");

        let spatial: Vec<Forms> = registrations.iter()
            .map(|r| forms_of(operator_a(&r.x, rel, &[]).spatial, rel))
            .collect();
        let persistence: Vec<Forms> = spatial.windows(2).map(|w| {
            // The prior is the earlier registration (Df2); never a substitute.
            let a_p = (0..k).map(|c| {
                w[1].a[c].iter().zip(&w[0].a[c]).map(|(cur, pri)| cur - pri).collect()
            }).collect();
            forms_of(a_p, rel)
        }).collect();
        let ground = ground_reading(&registrations, rel, carriers);
        Ok(RelationalField { registrations, spatial, persistence, ground })
    }

    /// Number of consecutive steps in the field.
    pub fn n_steps(&self) -> usize {
        self.persistence.len()
    }
}

/// Whole-field circulation weights (I-SC-D11, Derived 2026-11-21): for every
/// declared edge f, w[f] = indeg(src f) − outdeg(tgt f). For any edge field g,
/// Σ_e Ξ(g)[e] = Σ_f w[f]·g[f], because f is a successor of every edge ending
/// at src f and a predecessor of every edge starting at tgt f. The total is
/// zero for every field exactly when every weight is zero.
pub fn circulation_weights(rel: &DeclaredRelations) -> Vec<i64> {
    let indeg = |v: usize| rel.edges.iter().filter(|e| e.1 == v).count() as i64;
    let outdeg = |v: usize| rel.edges.iter().filter(|e| e.0 == v).count() as i64;
    rel.edges.iter().map(|&(s, t)| indeg(s) - outdeg(t)).collect()
}

/// Edges whose circulation is **record-boundary-conditioned** (A-3, decided
/// by Origin 2026-11-22, Part 9, 20th): exactly one of the successor and
/// predecessor sums is empty. A record boundary cuts an ongoing relation
/// (Df3), so the missing side is *not registered*, not zero. The empty sum is
/// taken as 0 (so I-SC-D11's identity holds), and Ξ at such an edge is
/// reported with this flag and never read as a circulation of zero. An edge
/// with both sides empty is not formable (IV-3) and is not listed here.
pub fn boundary_conditioned_edges(rel: &DeclaredRelations) -> Vec<usize> {
    (0..rel.n_edges()).filter(|&e| rel.succ(e).is_empty() != rel.pred(e).is_empty()).collect()
}
