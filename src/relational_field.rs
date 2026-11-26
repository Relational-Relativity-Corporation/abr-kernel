// relational_field.rs — Metatron Dynamics, Inc. Kernel V10.
// Df6, simultaneity (Origin, 2026-11-21). Bounded over D. No claim beyond D.
//
// ── Df6 ───────────────────────────────────────────────────────────────────
//
// All operators and all ground variables occur simultaneously throughout
// relational evolution. A, B, the passage Ξ, their persistence
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
    /// Ξ, the passage.
    pub passage: Vec<Vec<f64>>,
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
    /// m − n + c: the balanced part, the independent turns.
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
    let xi_coeff: Vec<Vec<i64>> = (0..m).map(|e| crate::operators::passage_of_accumulation_coefficients(rel, e, -1)).collect();
    let (leaving, entering, leaving_bound, entering_bound) = readback(&exact_a, None, rel);
    Forms {
        a: exact_a.iter().map(|c| c.iter().map(|v| v.nearest()).collect()).collect(),
        b: (0..k).map(|c| b_coeff.iter().map(|w| combine(c, w)).collect()).collect(),
        passage: (0..k).map(|c| xi_coeff.iter().map(|w| combine(c, w)).collect()).collect(),
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
        let w = crate::operators::passage_of_accumulation_terms(rel, e, -1);
        terms(w.len());
        w
    }).collect();
    // The bound of each relation entering the readback (98th: every value
    // carries its bound): at a registration, its state bound widened by any
    // gain differing between relations (a sum across relations, as a
    // turn sum); over a step, the change bound PC1 reads.
    let exact_bound: Vec<Vec<Expansion>> = (0..k).map(|c| (0..m).map(|e| match ts {
        [(t, _)] => crate::exact::gain_widened(&series.entry(*t, c, e), &Expansion::of(series.da[*t][c][e]), series.turn_sum_gain[c][e]),
        [(_, _), (t0, _)] => series.change_bound_exact(*t0, c, e),
        _ => unreachable!("forms are read at one registration or over one step"),
    }).collect()).collect();
    let (leaving, entering, leaving_bound, entering_bound) = readback(&exact_a, Some(&exact_bound), rel);
    Forms {
        a: exact_a.iter().map(|c| c.iter().map(|v| v.nearest()).collect()).collect(),
        b: (0..k).map(|c| b_coeff.iter().map(|w| combine(c, w)).collect()).collect(),
        passage: (0..k).map(|c| xi_coeff.iter().map(|w| combine(c, w)).collect()).collect(),
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
/// special case in which every turn sum is 0 by identity.
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

/// Whole-field passage weights (I-SC-D11, Derived 2026-11-21): for every
/// declared edge f, w[f] = indeg(src f) − outdeg(tgt f). For any edge field g,
/// Σ_e Ξ(g)[e] = Σ_f w[f]·g[f], because f is a successor of every edge ending
/// at src f and a predecessor of every edge starting at tgt f. The total is
/// zero for every field exactly when every weight is zero.
pub fn passage_weights(rel: &DeclaredRelations) -> Vec<i64> {
    let indeg = |v: usize| rel.edges.iter().filter(|e| e.1 == v).count() as i64;
    let outdeg = |v: usize| rel.edges.iter().filter(|e| e.0 == v).count() as i64;
    rel.edges.iter().map(|&(s, t)| indeg(s) - outdeg(t)).collect()
}

/// Edges whose passage is **record-boundary-conditioned** (A-3, decided
/// by Origin 2026-11-22, Part 9, 20th): exactly one of the successor and
/// predecessor sums is empty. A record boundary cuts an ongoing relation
/// (Df3), so the missing side is *not registered*, not zero. The empty sum is
/// taken as 0 (so I-SC-D11's identity holds), and Ξ at such an edge is
/// reported with this flag and never read as a passage of zero. An edge
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

// ═══════════════════════════════════════════════════════════════════════════
// The orthogonal split into the two parts (Origin, 2026-11-26, 100th; 99th)
// ═══════════════════════════════════════════════════════════════════════════
//
// Every field of relations g is the sum of a potential part p = A(x), the
// image of A, and a balanced part q, where at every locus what enters equals
// what leaves (Aᵀq = 0, the readback's balance, K9). Under a declared inner
// product the two are orthogonal and the split is unique: it depends on no
// choice of basis or tree (41st). x solves L x = Aᵀg over the loci, with
// L = AᵀA (each locus with the relations meeting it), one locus of each
// connected piece held at 0; the potential part is then the same whichever
// locus is held, since A takes no difference within a constant.
//
// [IMPL] Legacy mathematics (92nd; `derivation.rs`): Kirchhoff (1847); the
// discrete Hodge decomposition (Eckmann, 1944); the orthogonal projection of
// least squares. Computed exactly in rationals (`exact::Ratio`), by
// Gauss–Jordan elimination over the loci. **Not local:** each relation's
// part depends on every relation of its piece, and the bounds read the whole
// projection, m × m entries, so its work grows with the square of the
// relations. It is not an operator reading and is not counted in `work`.

use crate::exact::Ratio;

/// The inner product the split is orthogonal under: a declaration (99th).
#[derive(Debug, Clone, PartialEq)]
pub enum InnerProduct {
    /// Equal weight on every relation: the declared default. Any other
    /// weighting is a domain's declaration, not the kernel's.
    Equal { basis: String },
}

impl InnerProduct {
    /// The declared default: equal weight on every relation.
    pub fn equal() -> Self { InnerProduct::Equal { basis: "equal weight on every relation (the declared default, 100th)".into() } }
}

/// The two parts at one registration, or over one step, [c][e].
#[derive(Debug, Clone, PartialEq)]
pub struct Split {
    /// The potential part, rounded once to the nearest f64.
    pub potential: Vec<Vec<f64>>,
    /// The balanced part, rounded once to the nearest f64.
    pub balanced: Vec<Vec<f64>>,
    /// The bound of each part: the projection with its signs dropped applied
    /// to the relations' bounds (93rd, 98th), rounded up.
    pub potential_bound: Vec<Vec<f64>>,
    pub balanced_bound: Vec<Vec<f64>>,
    /// The parts exactly: every identity of the split holds in these.
    pub exact_potential: Vec<Vec<Ratio>>,
    pub exact_balanced: Vec<Vec<Ratio>>,
}

/// The orthogonal split of a whole field into its two parts, at every
/// registration and over every step.
#[derive(Debug, Clone, PartialEq)]
pub struct TwoParts {
    pub inner_product: InnerProduct,
    pub spatial: Vec<Split>,
    pub persistence: Vec<Split>,
}

impl RelationalField {
    /// The orthogonal split of the field into its potential and balanced
    /// parts (Origin, 100th), under the declared inner product. Read from the
    /// field's own relations, at every registration and step.
    pub fn two_parts(&self, ip: &InnerProduct) -> TwoParts {
        let InnerProduct::Equal { .. } = ip;
        // The declared relations, read from the field's own record of the
        // relations meeting at each locus (`meeting`): each relation's source
        // is the locus it leaves, its target the locus it enters.
        let n = self.meeting.leaving.len();
        let mut edges = vec![(usize::MAX, usize::MAX); self.relations.n_edges()];
        for v in 0..n {
            for &e in &self.meeting.leaving[v] { edges[e].0 = v; }
            for &e in &self.meeting.entering[v] { edges[e].1 = v; }
        }
        let proj = Projection::of(n, &edges);
        let s = &self.relations;
        let (k, m) = (s.k(), s.n_edges());
        let exact_of = |ts: &[(usize, f64)]| -> (Vec<Vec<Ratio>>, Vec<Vec<Ratio>>) {
            let g = (0..k).map(|c| (0..m).map(|e| {
                ts.iter().fold(Ratio::zero(), |acc, &(t, w)| {
                    let v = Ratio::from_f64(s.a[t][c][e]).add(&Ratio::from_f64(s.a_lo[t][c][e]));
                    if w > 0.0 { acc.add(&v) } else { acc.sub(&v) }
                })
            }).collect()).collect();
            let d = (0..k).map(|c| (0..m).map(|e| Ratio::from_f64(match ts {
                [(t, _)] => crate::exact::gain_widened(&s.entry(*t, c, e), &crate::exact::Expansion::of(s.da[*t][c][e]), s.turn_sum_gain[c][e]).up(),
                [(_, _), (t0, _)] => s.change_bound_exact(*t0, c, e).up(),
                _ => unreachable!("a registration or one step"),
            })).collect()).collect();
            (g, d)
        };
        let split = |ts: &[(usize, f64)]| -> Split {
            let (g, d) = exact_of(ts);
            let mut out = Split { potential: Vec::new(), balanced: Vec::new(), potential_bound: Vec::new(), balanced_bound: Vec::new(),
                exact_potential: Vec::new(), exact_balanced: Vec::new() };
            for c in 0..k {
                let p = proj.apply(&g[c]);
                let q: Vec<Ratio> = (0..m).map(|e| g[c][e].sub(&p[e])).collect();
                let (pb, qb) = proj.bounds(&d[c]);
                out.potential.push(p.iter().map(Ratio::to_f64_nearest).collect());
                out.balanced.push(q.iter().map(Ratio::to_f64_nearest).collect());
                out.potential_bound.push(pb.iter().map(Ratio::to_f64_up).collect());
                out.balanced_bound.push(qb.iter().map(Ratio::to_f64_up).collect());
                out.exact_potential.push(p);
                out.exact_balanced.push(q);
            }
            out
        };
        let n_t = s.n_registrations();
        TwoParts {
            inner_product: ip.clone(),
            spatial: (0..n_t).map(|t| split(&[(t, 1.0)])).collect(),
            persistence: (0..n_t - 1).map(|t| split(&[(t + 1, 1.0), (t, -1.0)])).collect(),
        }
    }
}

/// The orthogonal projection onto the potential part, exactly: G, a
/// generalized inverse of L = AᵀA over the loci (one locus of each piece
/// held at 0), and P = A G Aᵀ over the relations.
struct Projection { edges: Vec<(usize, usize)>, n: usize, g: Vec<Vec<Ratio>> }

impl Projection {
    fn of(n: usize, edges: &[(usize, usize)]) -> Self {
        // L = AᵀA: the relations meeting each locus on the diagonal, minus
        // the relations joining two loci off it.
        let mut l = vec![vec![0i64; n]; n];
        for &(s, t) in edges { l[s][s] += 1; l[t][t] += 1; l[s][t] -= 1; l[t][s] -= 1; }
        // One locus of each connected piece held at 0 (the least).
        let mut root: Vec<usize> = (0..n).collect();
        fn find(p: &mut [usize], v: usize) -> usize { let mut v = v; while p[v] != v { p[v] = p[p[v]]; v = p[v]; } v }
        for &(s, t) in edges { let (a, b) = (find(&mut root, s), find(&mut root, t)); if a != b { root[a.max(b)] = a.min(b); } }
        let held: Vec<bool> = (0..n).map(|v| find(&mut root, v) == v).collect();
        let free: Vec<usize> = (0..n).filter(|&v| !held[v]).collect();
        // Gauss–Jordan on the free loci, exactly: [L_free | I] → [I | L_free⁻¹].
        let r = free.len();
        let mut a: Vec<Vec<Ratio>> = (0..r).map(|i| (0..2 * r).map(|j| {
            if j < r { Ratio::from_i64(l[free[i]][free[j]]) } else if j - r == i { Ratio::from_i64(1) } else { Ratio::zero() }
        }).collect()).collect();
        for col in 0..r {
            let piv = (col..r).find(|&i| !a[i][col].is_zero()).expect("the reduced matrix over the loci is invertible");
            a.swap(col, piv);
            let pv = a[col][col].clone();
            for j in 0..2 * r { a[col][j] = a[col][j].div(&pv); }
            for i in 0..r {
                if i == col || a[i][col].is_zero() { continue; }
                let f = a[i][col].clone();
                for j in 0..2 * r { let t = a[col][j].mul(&f); a[i][j] = a[i][j].sub(&t); }
            }
        }
        let mut g = vec![vec![Ratio::zero(); n]; n];
        for i in 0..r { for j in 0..r { g[free[i]][free[j]] = a[i][r + j].clone(); } }
        Projection { edges: edges.to_vec(), n, g }
    }
    /// The potential part of g: A G Aᵀ g, exactly.
    fn apply(&self, g: &[Ratio]) -> Vec<Ratio> {
        let mut b = vec![Ratio::zero(); self.n];
        for (e, &(s, t)) in self.edges.iter().enumerate() { b[s] = b[s].add(&g[e]); b[t] = b[t].sub(&g[e]); }
        let x: Vec<Ratio> = (0..self.n).map(|u| (0..self.n).fold(Ratio::zero(), |acc, v| if self.g[u][v].is_zero() || b[v].is_zero() { acc } else { acc.add(&self.g[u][v].mul(&b[v])) })).collect();
        self.edges.iter().map(|&(s, t)| x[s].sub(&x[t])).collect()
    }
    /// P[e][f] = (A G Aᵀ)[e][f].
    fn entry(&self, e: usize, f: usize) -> Ratio {
        let ((se, te), (sf, tf)) = (self.edges[e], self.edges[f]);
        self.g[se][sf].sub(&self.g[se][tf]).sub(&self.g[te][sf]).add(&self.g[te][tf])
    }
    /// The bounds of the two parts from the relations' bounds d: Σ_f |P[e][f]|·d[f]
    /// and Σ_f |I[e][f] − P[e][f]|·d[f] (93rd: the projection with its signs
    /// dropped, applied to the bounds).
    fn bounds(&self, d: &[Ratio]) -> (Vec<Ratio>, Vec<Ratio>) {
        let m = self.edges.len();
        let mut pb = vec![Ratio::zero(); m];
        let mut qb = vec![Ratio::zero(); m];
        for e in 0..m {
            for f in 0..m {
                if d[f].is_zero() { continue; }
                let p = self.entry(e, f);
                let q = if e == f { Ratio::from_i64(1).sub(&p) } else { p.neg() };
                pb[e] = pb[e].add(&p.abs().mul(&d[f]));
                qb[e] = qb[e].add(&q.abs().mul(&d[f]));
            }
        }
        (pb, qb)
    }
}

/// The old name, kept as a deprecated alias (Origin, 106th: Ξ is the
/// passage).
#[deprecated(note = "renamed passage_weights (Origin, 106th)")]
pub fn circulation_weights(rel: &DeclaredRelations) -> Vec<i64> { passage_weights(rel) }
