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

use crate::ground_variables::{ground_reading, ground_reading_of_relations, DeclarationGap, EnergyCarrier, GroundReading, RelationCarrier, Registration};
use crate::operators::{DeclaredRelations, NodeField};
use crate::persistence_change::EdgeSeries;

/// A, B and Ξ at one registration: spatial[c][e] per component and edge.
#[derive(Debug, Clone, PartialEq)]
pub struct Forms {
    pub a: Vec<Vec<f64>>,
    pub b: Vec<Vec<f64>>,
    /// Ξ, the circulation.
    pub circulation: Vec<Vec<f64>>,
}

/// A, B and Ξ of Σ_r w_r·A(x_r), each from the registered node values in one
/// exact sum, rounded once (OC-NUM-1, 45th). One registration with w = 1
/// gives the forms at that registration; (current, +1), (prior, −1) gives the
/// persistence forms over the step. *Was:* A rounded, A_p as the difference
/// of rounded A, and B and Ξ from those (41st: Ξ from rounded A).
///
/// *Retired from the whole field (Origin, 2026-11-23, 59th: relation is
/// primitive).* The forms are now read from the registered relations
/// (`forms_of_relations`). Kept to show, in this module's test, that the
/// locus path gives the same forms exactly.
#[cfg_attr(not(test), allow(dead_code))]
fn forms_of(xs: &[(&NodeField, f64)], rel: &DeclaredRelations) -> Forms {
    use crate::exact::Expansion;
    let k = xs[0].0.k;
    let exact_a: Vec<Vec<Expansion>> = (0..k).map(|c| rel.edges.iter().map(|&(s, t)| {
        let mut v = Expansion::new();
        for (x, w) in xs { v.add_product(*w, x.data[c][s]); v.add_product(-*w, x.data[c][t]); }
        v
    }).collect()).collect();
    let combine = |c: usize, coeff: &[i64]| {
        let mut v = Expansion::new();
        for (f, &kf) in coeff.iter().enumerate() { if kf != 0 { v.add_times(&exact_a[c][f], kf as f64); } }
        v.nearest()
    };
    let m = rel.n_edges();
    let b_coeff: Vec<Vec<i64>> = (0..m).map(|e| {
        let mut w = vec![0i64; m];
        w[e] += 1;
        for &f in rel.succ(e) { w[f] += 1; }
        w
    }).collect();
    let xi_coeff: Vec<Vec<i64>> = (0..m).map(|e| crate::operators::circulation_of_accumulation_coefficients(rel, e, -1)).collect();
    Forms {
        a: exact_a.iter().map(|c| c.iter().map(|v| v.nearest()).collect()).collect(),
        b: (0..k).map(|c| b_coeff.iter().map(|w| combine(c, w)).collect()).collect(),
        circulation: (0..k).map(|c| xi_coeff.iter().map(|w| combine(c, w)).collect()).collect(),
    }
}

/// A, B and Ξ of Σ_t w_t·A_t, read from the registered relations in one
/// exact sum each, rounded once (relation as primitive, 59th). (t, 1) gives
/// the forms at registration t; (t + 1, 1), (t, −1) the persistence forms
/// over step t. For relations formed from loci this equals `forms_of` exactly.
fn forms_of_relations(series: &EdgeSeries, ts: &[(usize, f64)], rel: &DeclaredRelations) -> Forms {
    use crate::exact::Expansion;
    let k = series.k();
    let exact_a: Vec<Vec<Expansion>> = (0..k).map(|c| (0..rel.n_edges()).map(|e| {
        let mut v = Expansion::new();
        for &(t, w) in ts { v.add_times(&series.entry(t, c, e), w); }
        v
    }).collect()).collect();
    let combine = |c: usize, coeff: &[i64]| {
        let mut v = Expansion::new();
        for (f, &kf) in coeff.iter().enumerate() { if kf != 0 { v.add_times(&exact_a[c][f], kf as f64); } }
        v.nearest()
    };
    let m = rel.n_edges();
    let b_coeff: Vec<Vec<i64>> = (0..m).map(|e| {
        let mut w = vec![0i64; m];
        w[e] += 1;
        for &f in rel.succ(e) { w[f] += 1; }
        w
    }).collect();
    let xi_coeff: Vec<Vec<i64>> = (0..m).map(|e| crate::operators::circulation_of_accumulation_coefficients(rel, e, -1)).collect();
    Forms {
        a: exact_a.iter().map(|c| c.iter().map(|v| v.nearest()).collect()).collect(),
        b: (0..k).map(|c| b_coeff.iter().map(|w| combine(c, w)).collect()).collect(),
        circulation: (0..k).map(|c| xi_coeff.iter().map(|w| combine(c, w)).collect()).collect(),
    }
}

/// A relational field: one declared intersection with an ongoing relational
/// evolution, evaluated as a whole.
///
/// *Amended (Origin, 2026-11-23, 59th): relation is primitive.* The field is
/// formed from a registration of the relations (`relations`), and every form
/// and every ground variable is read from it. Registered directly
/// (`from_relations`, ER-1), it is the general case. Registered at the loci
/// (`new`), the relations are formed as x[s] − x[t] first; that is the
/// special case in which every closure is 0 by identity.
#[derive(Debug, Clone)]
pub struct RelationalField {
    /// The declared sequence of registrations at the loci, when the relations
    /// were formed from loci. Empty when the relations were registered
    /// directly: no locus value is constructed (see `locus_registered`).
    pub registrations: Vec<Registration>,
    /// The registrations of the relations: the primitive the field is read
    /// from (59th).
    pub relations: EdgeSeries,
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
        crate::topology::admit_field(rel)?;
        let k = registrations[0].x.k;
        assert!(registrations.iter().all(|r| r.x.k == k),
            "every registration must declare the same components");
        assert!(registrations.iter().all(|r| r.x.n == rel.n_nodes),
            "every registration must cover the declared loci");

        // Relation first (59th): the relations are formed from the loci, and
        // every form is read from them. *Was:* the forms read from x directly
        // (`forms_of`), with the same values exactly.
        let relations = EdgeSeries::from_registrations(&registrations, rel)?;
        let (spatial, persistence) = forms_over(&relations, rel);
        let ground = ground_reading(&registrations, rel, carriers);
        Ok(RelationalField { registrations, relations, spatial, persistence, ground })
    }

    /// Form the relational field from a declared sequence of registrations of
    /// the relations themselves (relation as primitive, Origin, 2026-11-23,
    /// 59th; ER-1). Energy is read from a carrier registered across the
    /// relations (`RelationCarrier`); with none declared, energy is not
    /// evaluated (`NoCarrierDeclared`). Every form, and energy, connection and
    /// freedom, are evaluated together, as in `new`.
    pub fn from_relations(
        rel: &DeclaredRelations,
        relations: EdgeSeries,
        carrier: Option<&RelationCarrier>,
    ) -> Result<Self, DeclarationGap> {
        if relations.n_registrations() < 2 {
            return Err(DeclarationGap::NoProcessSequence);
        }
        crate::topology::admit_field(rel)?;
        assert_eq!(relations.n_edges(), rel.n_edges(), "the series must register every declared relation");
        let (spatial, persistence) = forms_over(&relations, rel);
        let ground = ground_reading_of_relations(&relations, rel, carrier);
        Ok(RelationalField { registrations: Vec::new(), relations, spatial, persistence, ground })
    }

    /// True when the relations were formed from registrations at the loci.
    pub fn locus_registered(&self) -> bool {
        !self.registrations.is_empty()
    }

    /// Number of consecutive steps in the field.
    pub fn n_steps(&self) -> usize {
        self.persistence.len()
    }
}

/// The forms at every registration, and over every consecutive step (the
/// prior always the earlier registration, Df2; never a substitute).
fn forms_over(series: &EdgeSeries, rel: &DeclaredRelations) -> (Vec<Forms>, Vec<Forms>) {
    let n_t = series.n_registrations();
    let spatial = (0..n_t).map(|t| forms_of_relations(series, &[(t, 1.0)], rel)).collect();
    let persistence = (0..n_t - 1).map(|t| forms_of_relations(series, &[(t + 1, 1.0), (t, -1.0)], rel)).collect();
    (spatial, persistence)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operators::NodeField;

    /// The locus path and the relation path give the same forms exactly
    /// (59th). Enumerated: x[c][i] = (3i + 5c + t)² mod 11 − 5 on a declared
    /// chain with a branch, at three registrations.
    #[test]
    fn forms_from_relations_equal_forms_from_loci() {
        use crate::operators::EdgeProvenance;
        use crate::topology::RelationIdentity;
        let edges = vec![(0, 1), (1, 2), (2, 3), (1, 3)];
        let ids = (0..edges.len()).map(|e| RelationIdentity { id: format!("SYNTHETIC-e{e}"),
            basis: "SYNTHETIC: direction declared for the test".into() }).collect();
        let rel = DeclaredRelations::from_edges_with_provenance(4, edges, vec![EdgeProvenance::Continuation; 4]).with_identity(ids);
        let regs: Vec<Registration> = (0..3).map(|t| {
            let data: Vec<Vec<f64>> = (0..2).map(|c| (0..4).map(|i| (((3 * i + 5 * c + t) * (3 * i + 5 * c + t)) % 11) as f64 - 5.0).collect()).collect();
            Registration::synthetic(NodeField::new(data), vec![vec![0.5; 4]; 2])
        }).collect();
        let field = RelationalField::new(&rel, regs.clone(), None).unwrap();
        for (t, r) in regs.iter().enumerate() {
            assert_eq!(field.spatial[t], forms_of(&[(&r.x, 1.0)], &rel));
        }
        for t in 0..2 {
            assert_eq!(field.persistence[t], forms_of(&[(&regs[t + 1].x, 1.0), (&regs[t].x, -1.0)], &rel));
        }
    }
}
