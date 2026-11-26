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
    /// The readback at the loci (K9, Origin, 96th), [c][v]: what leaves
    /// locus v, Tᵀg[v] = Σ of the relations leaving v. A reading of
    /// relations at a locus, never a locus state. The relations it sums are
    /// `RelationalField::meeting`.
    pub leaving: Vec<Vec<f64>>,
    /// What enters locus v, Hᵀg[v] = Σ of the relations entering v.
    pub entering: Vec<Vec<f64>>,
    /// The bounds of the two halves, [c][v]: the sums of the bounds of the
    /// relations summed (the operator with its signs dropped, 93rd), rounded
    /// up. Empty on the locus path kept for comparison (`forms_of`).
    pub leaving_bound: Vec<Vec<f64>>,
    pub entering_bound: Vec<Vec<f64>>,
}

/// The halves of the readback (K9, 96th), [c][v], from the exact relation
/// values and their bounds: one exact sum each, rounded once (values) or up
/// (bounds). The balance, what leaves less what enters, is derived from the
/// halves and never stands in for them.
fn readback(exact: &[Vec<crate::exact::Expansion>], bound: Option<&[Vec<crate::exact::Expansion>]>, rel: &DeclaredRelations)
    -> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>) {
    use crate::exact::Expansion;
    use crate::work::terms;
    let half = |at: &[Vec<usize>], of: &[Vec<Expansion>], round_up: bool| -> Vec<Vec<f64>> {
        of.iter().map(|c| (0..rel.n_nodes).map(|v| {
            let mut sum = Expansion::new();
            terms(at[v].len());
            for &e in &at[v] { sum.add_times(&c[e], 1.0); }
            if round_up { sum.up() } else { sum.nearest() }
        }).collect()).collect()
    };
    let (lb, eb) = match bound {
        Some(b) => (half(&rel.out, b, true), half(&rel.inc, b, true)),
        None => (Vec::new(), Vec::new()),
    };
    (half(&rel.out, exact, false), half(&rel.inc, exact, false), lb, eb)
}

/// The relations meeting at each locus (K9, 96th, condition (1)): the
/// relations each half of the readback sums, in the order declared, every
/// direction kept (condition (3)).
#[derive(Debug, Clone, PartialEq)]
pub struct Meeting {
    /// leaving[v]: the relations declared to leave locus v.
    pub leaving: Vec<Vec<usize>>,
    /// entering[v]: the relations declared to enter locus v.
    pub entering: Vec<Vec<usize>>,
}

/// The independent ways of the two parts of every field of relations
/// (Proposed, Origin, 2026-11-26, 99th): fixed by the declared relations
/// alone, independent of the data, the domain and δ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ways {
    /// m, the declared relations.
    pub relations: usize,
    /// n, the declared loci.
    pub loci: usize,
    /// c, the connected pieces the relations join the loci into (a locus
    /// no relation meets is a piece of its own).
    pub pieces: usize,
    /// n − c: the potential part, fields explained by values at the loci.
    pub potential: usize,
    /// m − n + c: the balanced part, the independent cycles.
    pub balanced: usize,
}

impl Ways {
    /// The ways of the declared relations.
    pub fn of(rel: &DeclaredRelations) -> Self {
        let mut p: Vec<usize> = (0..rel.n_nodes).collect();
        fn root(p: &mut [usize], v: usize) -> usize { let mut v = v; while p[v] != v { p[v] = p[p[v]]; v = p[v]; } v }
        let mut pieces = rel.n_nodes;
        for &(s, t) in &rel.edges {
            let (a, b) = (root(&mut p, s), root(&mut p, t));
            if a != b { p[a] = b; pieces -= 1; }
        }
        let (m, n) = (rel.n_edges(), rel.n_nodes);
        Ways { relations: m, loci: n, pieces, potential: n - pieces, balanced: m + pieces - n }
    }
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
    let (leaving, entering, leaving_bound, entering_bound) = readback(&exact_a, None, rel);
    Forms {
        a: exact_a.iter().map(|c| c.iter().map(|v| v.nearest()).collect()).collect(),
        b: (0..k).map(|c| b_coeff.iter().map(|w| combine(c, w)).collect()).collect(),
        circulation: (0..k).map(|c| xi_coeff.iter().map(|w| combine(c, w)).collect()).collect(),
        leaving, entering, leaving_bound, entering_bound,
    }
}

/// A, B and Ξ of Σ_t w_t·A_t, read from the registered relations in one
/// exact sum each, rounded once (relation as primitive, 59th). (t, 1) gives
/// the forms at registration t; (t + 1, 1), (t, −1) the persistence forms
/// over step t. For relations formed from loci this equals `forms_of` exactly.
fn forms_of_relations(series: &EdgeSeries, ts: &[(usize, f64)], rel: &DeclaredRelations) -> Forms {
    use crate::exact::Expansion;
    use crate::work::terms;
    let k = series.k();
    let exact_a: Vec<Vec<Expansion>> = (0..k).map(|c| (0..rel.n_edges()).map(|e| {
        let mut v = Expansion::new();
        terms(ts.len());
        for &(t, w) in ts { v.add_times(&series.entry(t, c, e), w); }
        v
    }).collect()).collect();
    // Each relation's B and Ξ visit only the relations B joins to it (92nd).
    // *Was:* a coefficient vector over every relation for every e, so the
    // work grew with the square of the relations; the nonzero coefficients,
    // and so every exact sum and every value, are the same.
    let combine = |c: usize, coeff: &[(usize, i64)]| {
        let mut v = Expansion::new();
        terms(coeff.len());
        for &(f, kf) in coeff { v.add_times(&exact_a[c][f], kf as f64); }
        v.nearest()
    };
    let m = rel.n_edges();
    let b_coeff: Vec<Vec<(usize, i64)>> = (0..m).map(|e| {
        let w = crate::operators::accumulation_terms(rel, e);
        terms(w.len());
        w
    }).collect();
    let xi_coeff: Vec<Vec<(usize, i64)>> = (0..m).map(|e| {
        let w = crate::operators::circulation_of_accumulation_terms(rel, e, -1);
        terms(w.len());
        w
    }).collect();
    // The bound of each relation entering the readback (98th: every value
    // carries its bound): at a registration, its state bound widened by any
    // gain differing between relations (a sum across relations, as a
    // closure); over a step, the change bound PC1 reads.
    let exact_bound: Vec<Vec<Expansion>> = (0..k).map(|c| (0..m).map(|e| match ts {
        [(t, _)] => crate::exact::gain_widened(&series.entry(*t, c, e), &Expansion::of(series.da[*t][c][e]), series.closure_gain[c][e]),
        [(_, _), (t0, _)] => series.change_bound_exact(*t0, c, e),
        _ => unreachable!("forms are read at one registration or over one step"),
    }).collect()).collect();
    let (leaving, entering, leaving_bound, entering_bound) = readback(&exact_a, Some(&exact_bound), rel);
    Forms {
        a: exact_a.iter().map(|c| c.iter().map(|v| v.nearest()).collect()).collect(),
        b: (0..k).map(|c| b_coeff.iter().map(|w| combine(c, w)).collect()).collect(),
        circulation: (0..k).map(|c| xi_coeff.iter().map(|w| combine(c, w)).collect()).collect(),
        leaving, entering, leaving_bound, entering_bound,
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
    /// The relations meeting at each locus, which the readback sums (K9).
    pub meeting: Meeting,
    /// The ways of the two parts of every field of relations (99th, Proposed).
    pub ways: Ways,
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
        Ok(RelationalField { registrations, relations, spatial, persistence, ground, meeting: meeting_of(rel), ways: Ways::of(rel) })
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
        Ok(RelationalField { registrations: Vec::new(), relations, spatial, persistence, ground, meeting: meeting_of(rel), ways: Ways::of(rel) })
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

fn meeting_of(rel: &DeclaredRelations) -> Meeting {
    Meeting { leaving: rel.out.clone(), entering: rel.inc.clone() }
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
        // The locus path carries no bounds (`forms_of`), so the bounds are
        // set aside before comparing; every value is compared. *Was:* the
        // whole `Forms` compared, before the readback (99th) gave it bounds.
        let values = |f: &Forms| Forms { leaving_bound: Vec::new(), entering_bound: Vec::new(), ..f.clone() };
        for (t, r) in regs.iter().enumerate() {
            assert_eq!(values(&field.spatial[t]), forms_of(&[(&r.x, 1.0)], &rel));
        }
        for t in 0..2 {
            assert_eq!(values(&field.persistence[t]), forms_of(&[(&regs[t + 1].x, 1.0), (&regs[t].x, -1.0)], &rel));
        }
    }
}
