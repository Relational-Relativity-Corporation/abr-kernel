//! Every derivation is true (Origin, 2026-11-26: *"finish the unit tests
//! until we have a comprehensive suite"*).
//!
//! The derivation register (92nd) states, for every part a field reports,
//! how it is formed. Until now a test checked only that a statement exists.
//! Here each part the register claims as **an operator reading**, **a
//! registration** or **a rule** is recomputed from its definition, exactly,
//! by a reference written in this file from the definitions (A = x[s] − x[t]
//! or as registered; B[e] = A[e] + Σ_succ A; Ξ[e] = Σ_succ B − Σ_pred B; the
//! readback's halves Σ of the relations leaving or entering a locus; A_p =
//! A_{t+1} − A_t; bounds the same operators with their signs dropped), and
//! must equal what the field reports. A registration must equal the input
//! it was registered from. The field's own code is never the reference.
//!
//! **Completeness:** every register entry that claims an operator, a
//! registration or a rule is in `FORMED` (recomputed here) or in
//! `NOT_RECOMPUTED` with its reason; an operator reading or a registration
//! may never be in `NOT_RECOMPUTED`. A new part that claims an operator
//! without a recomputation fails, whatever it is called (the failure of
//! `Degree.leaving`, 2026-11-26, which claimed the readback).
//!
//! **Cannot tell (named):** the references are the definitions as written
//! here; a definition wrong in the register and wrong the same way here
//! passes. Legacy parts are held by the guard (admitted by name), not here.
//! The fixtures declare repeatability only (no run-constant parts, Z1a), so
//! the gains are zero and the bounds are the sums of δ; the gain widening
//! is tested in `v10_z1a_a3.rs`.
//!
//! [IMPL] SYNTHETIC FIXTURES, whole fields enumerated from declared seeds
//! (W17), dyadic values, so every reference is exact.

#[path = "support/synthetic_whole_field.rs"]
mod support;

use std::cell::RefCell;

use abr_kernel::derivation::{of_part, register, Derivation};
use abr_kernel::exact::{exceeds, Expansion};
use abr_kernel::ground_variables::{ObservedRange, RelationCarrier, Side};
use abr_kernel::operators::DeclaredRelations;
use abr_kernel::order::Order;
use abr_kernel::persistence_change::Reading;
use abr_kernel::relational_field::{InnerProduct, RelationalField};
use support::*;

/// The parts recomputed in this file.
const FORMED: &[&str] = &[
    // Registrations: equal to the input.
    "RelationalField.registrations", "Registration.x", "NodeField.data", "Registration.delta", "Registration.offset",
    "Registration.locus_gain", "Registration.shared_gain", "Registration.range", "Synthetic.basis",
    "RelationalField.relations", "RelationalField.meeting", "Meeting.leaving", "Meeting.entering",
    "RelationalField.ground", "TwoParts.inner_product", "Equal.basis", "Order.edges", "Order.steps", "Degree.leaving",
    // Operator readings.
    "EdgeSeries.a", "EdgeSeries.a_lo", "RelationalField.spatial", "RelationalField.persistence", "Forms.a", "Forms.b",
    "Forms.passage", "Forms.leaving", "Forms.entering", "PersistenceChange.relations", "RelationChange.change",
    "GroundReading.energy", "Energy.signed", "SignedExchange.value", "GroundReading.freedom", "GroundReading.connection",
    "Connection.identity", "Connection.sustained",
    // Rules (the framework reading the operators' output).
    "EdgeSeries.da", "EdgeSeries.da_rep", "EdgeSeries.dgain", "EdgeSeries.shared_gain", "EdgeSeries.mixed_gain",
    "EdgeSeries.turn_sum_gain", "Forms.leaving_bound", "Forms.entering_bound", "RelationChange.half_width",
    "RelationChange.reading", "SignedExchange.bound", "SignedExchange.side", "Energy.share_resolved",
    "Sustained.held_edges", "Sustained.reorganizing_edges", "Sustained.share_held", "Sustained.share_held_all",
    "Sustained.share_held_boundary", "Connection.evaluated_pairs", "Connection.unresolved_edges",
    "RelationalField.ways", "Ways.potential", "Ways.balanced",
];

/// Rules not recomputed here, each with its reason. No operator reading and
/// no registration may be listed.
const NOT_RECOMPUTED: &[(&str, &str)] = &[
    ("ChangeCoupling.half_width", "a rule over the legacy minors (94th): the product bound of a minor; tested in v10_exact_readings"),
    ("ChangeCoupling.reading", "a rule over the legacy minors (94th)"),
    ("ChangeCoupling.forced", "a rule over the legacy minors (94th)"),
    ("ArrangementStep.reading", "a rule over the legacy minors (94th): PC3"),
    ("Connection.coupled_change", "a rule over the legacy minors (94th): PC2"),
    ("Connection.coupled_change_forced", "a rule over the legacy minors (94th): PC2"),
    ("Freedom.evaluated_pairs", "a rule over the legacy V2 first form (minors, 94th)"),
    ("Freedom.unresolved_edges", "a rule over the legacy V2 first form (minors, 94th)"),
    ("Freedom.f_delta", "a rule over the legacy V2 first form (minors, 94th)"),
    ("Split.potential_bound", "a rule over the legacy orthogonal split (100th): tested in v10_orthogonal_split"),
    ("Split.balanced_bound", "a rule over the legacy orthogonal split (100th): tested in v10_orthogonal_split"),
];

// ── The references, from the definitions ─────────────────────────────────

/// A[c][e] at registration t, exactly: x[s] − x[t] from the declared values.
fn a_ref(d: &Declared, rel: &DeclaredRelations, t: usize, c: usize, e: usize) -> Expansion {
    let (s, u) = rel.edges[e];
    Expansion::diff(d.values[t][c][s], d.values[t][c][u])
}

/// δA[c][e], exactly: δ[s] + δ[t] (repeatability only).
fn da_ref(d: &Declared, rel: &DeclaredRelations, c: usize, e: usize) -> Expansion {
    let (s, u) = rel.edges[e];
    Expansion::of(d.delta[c][s]).plus(&Expansion::of(d.delta[c][u]))
}

/// The successors and predecessors of a relation, from the declared edges:
/// what leaves the locus e enters; what enters the locus e leaves.
fn succ(rel: &DeclaredRelations, e: usize) -> Vec<usize> { (0..rel.n_edges()).filter(|&f| rel.edges[f].0 == rel.edges[e].1).collect() }
fn pred(rel: &DeclaredRelations, e: usize) -> Vec<usize> { (0..rel.n_edges()).filter(|&f| rel.edges[f].1 == rel.edges[e].0).collect() }

/// B[e] = A[e] + Σ_succ A, of any relation field g.
fn b_ref(rel: &DeclaredRelations, g: &dyn Fn(usize) -> Expansion, e: usize) -> Expansion {
    let mut v = g(e);
    for f in succ(rel, e) { v = v.plus(&g(f)); }
    v
}

/// Ξ[e] = Σ_succ B − Σ_pred B; with `drop_signs` the bound's form, every
/// term added (|Ξ|∘|B|, 93rd).
fn xi_ref(rel: &DeclaredRelations, g: &dyn Fn(usize) -> Expansion, e: usize, drop_signs: bool) -> Expansion {
    let mut v = Expansion::new();
    for f in succ(rel, e) { v = v.plus(&b_ref(rel, g, f)); }
    for p in pred(rel, e) { let b = b_ref(rel, g, p); v = if drop_signs { v.plus(&b) } else { v.minus(&b) }; }
    v
}

/// The readback's halves at locus v: Σ of the relations leaving (or
/// entering) it.
fn half_ref(rel: &DeclaredRelations, g: &dyn Fn(usize) -> Expansion, v: usize, leaving: bool) -> Expansion {
    let mut h = Expansion::new();
    for e in 0..rel.n_edges() { if (if leaving { rel.edges[e].0 } else { rel.edges[e].1 }) == v { h = h.plus(&g(e)); } }
    h
}

/// The pieces the declared relations join the loci into.
fn pieces_ref(rel: &DeclaredRelations) -> usize {
    let mut p: Vec<usize> = (0..rel.n_nodes).collect();
    fn root(p: &mut Vec<usize>, x: usize) -> usize { let mut r = x; while p[r] != r { r = p[r]; } p[x] = r; r }
    for &(s, t) in &rel.edges { let (a, b) = (root(&mut p, s), root(&mut p, t)); if a != b { p[a] = b; } }
    (0..rel.n_nodes).filter(|&x| root(&mut p, x) == x).count()
}

fn eq(v: f64, x: &Expansion) -> bool { v == x.nearest() }
fn up(v: f64, x: &Expansion) -> bool { v == x.up() }

/// A whole field at the loci, repeatability only, its carrier the last
/// component.
fn field(r: &mut Seeds) -> Whole {
    let rel = enumerated_rel(r);
    let k = r.usize(2, 3);
    let regs = r.usize(2, 4);
    whole(&rel, enumerated_declared(r, &rel, k, regs, 0.5, false, false))
}

// ── The recomputations ───────────────────────────────────────────────────

/// Registrations equal the input they were registered from.
#[test]
fn registrations_equal_their_input() {
    for_all("formation: registrations", 0xF0_01, |r| {
        let w = field(r);
        let (d, f, rel) = (&w.declared, &w.field, &w.rel);
        assert_eq!(f.registrations.len(), d.values.len());
        for (t, g) in f.registrations.iter().enumerate() {
            assert_eq!(g.x.data, d.values[t], "NodeField.data");
            assert_eq!((g.x.k, g.x.n), (d.k(), rel.n_nodes));
            assert_eq!(g.delta, d.delta, "Registration.delta");
            assert!(g.offset.iter().flatten().all(|&x| x == 0.0) && g.locus_gain.iter().flatten().all(|&x| x == 0.0) && g.shared_gain.iter().all(|&x| x == 0.0), "none declared");
            assert_eq!(g.range, ObservedRange::synthetic(), "Registration.range, Synthetic.basis");
        }
        for v in 0..rel.n_nodes {
            let l: Vec<usize> = (0..rel.n_edges()).filter(|&e| rel.edges[e].0 == v).collect();
            let n: Vec<usize> = (0..rel.n_edges()).filter(|&e| rel.edges[e].1 == v).collect();
            assert_eq!(f.meeting.leaving[v], l, "Meeting.leaving");
            assert_eq!(f.meeting.entering[v], n, "Meeting.entering");
        }
        // The relations registered directly are the series given.
        let g = RelationalField::from_relations(rel, f.relations.clone(), None).unwrap();
        assert_eq!(g.relations.a, f.relations.a, "RelationalField.relations, as registered");
        assert!(g.registrations.is_empty(), "no locus value constructed");
        assert!(f.ground.freedom.is_ok() && f.ground.connection.is_ok() && f.ground.energy.is_ok(), "RelationalField.ground: all three read");
        let ip = InnerProduct::equal();
        assert_eq!(f.two_parts(&ip).inner_product, ip, "TwoParts.inner_product, Equal.basis: as declared");
    });
}

/// The order and its degrees register the steps as given; the relation that
/// left a degree's point is the step registered there.
#[test]
fn the_order_registers_its_steps() {
    for_all("formation: the order", 0xF0_02, |r| {
        let rel = enumerated_rel(r);
        let mut steps = vec![r.usize(0, rel.n_edges() - 1)];
        let len = r.usize(3, 30);
        while steps.len() < len {
            let at = rel.edges[*steps.last().unwrap()].1;
            let out: Vec<usize> = (0..rel.n_edges()).filter(|&e| rel.edges[e].0 == at).collect();
            if out.is_empty() { break; }
            steps.push(out[r.usize(0, out.len() - 1)]);
        }
        let o = Order::new(&rel, steps.clone()).unwrap();
        assert_eq!(o.edges, rel.edges, "Order.edges");
        assert_eq!(o.steps, steps, "Order.steps");
        for d in o.degrees() { assert_eq!(d.leaving, steps[d.point], "Degree.leaving: the step registered at its point"); }
    });
}

/// A, B, Ξ and the readback's halves at every registration, and over every
/// step; the change of each relation; their bounds.
#[test]
fn the_operators_are_formed_as_stated() {
    for_all("formation: A, B, Xi, the readback", 0xF0_03, |r| {
        let w = field(r);
        let (d, f, rel) = (&w.declared, &w.field, &w.rel);
        let (regs, k, m) = (d.values.len(), d.k(), rel.n_edges());
        assert_eq!((f.spatial.len(), f.persistence.len()), (regs, regs - 1), "RelationalField.spatial, .persistence");
        for c in 0..k {
            let dav = |e: usize| da_ref(d, rel, c, e);
            for e in 0..m {
                assert!(up(f.relations.da[0][c][e], &dav(e)) && f.relations.da_rep[0][c][e] == f.relations.da[0][c][e], "EdgeSeries.da, .da_rep");
                assert!(f.relations.dgain[0][c][e] == 0.0 && f.relations.shared_gain[c][e] == 0.0 && f.relations.mixed_gain[c][e] == 0.0 && f.relations.turn_sum_gain[c][e] == 0.0, "the gains: none declared");
            }
            for t in 0..regs {
                let a = |e: usize| a_ref(d, rel, t, c, e);
                let s = &f.spatial[t];
                for e in 0..m {
                    assert!(eq(f.relations.a[t][c][e], &a(e)), "EdgeSeries.a");
                    assert_eq!(Expansion::of(f.relations.a[t][c][e]).plus(&Expansion::of(f.relations.a_lo[t][c][e])).minus(&a(e)).signum(), 0, "EdgeSeries.a_lo: a + a_lo is A exactly");
                    assert!(eq(s.a[c][e], &a(e)), "Forms.a, registration {t}");
                    assert!(eq(s.b[c][e], &b_ref(rel, &a, e)), "Forms.b: A[e] + Σ_succ A, registration {t}");
                    assert!(eq(s.passage[c][e], &xi_ref(rel, &a, e, false)), "Forms.passage: Σ_succ B − Σ_pred B, registration {t}");
                }
                for v in 0..rel.n_nodes {
                    assert!(eq(s.leaving[c][v], &half_ref(rel, &a, v, true)), "Forms.leaving");
                    assert!(eq(s.entering[c][v], &half_ref(rel, &a, v, false)), "Forms.entering");
                    assert!(up(s.leaving_bound[c][v], &half_ref(rel, &dav, v, true)), "Forms.leaving_bound");
                    assert!(up(s.entering_bound[c][v], &half_ref(rel, &dav, v, false)), "Forms.entering_bound");
                }
            }
            for st in 0..regs - 1 {
                let ap = |e: usize| a_ref(d, rel, st + 1, c, e).minus(&a_ref(d, rel, st, c, e));
                let p = &f.persistence[st];
                for e in 0..m {
                    assert!(eq(p.a[c][e], &ap(e)), "A_p");
                    assert!(eq(p.b[c][e], &b_ref(rel, &ap, e)), "B_p = B(A_p)");
                    assert!(eq(p.passage[c][e], &xi_ref(rel, &ap, e, false)), "Ξ_p = Ξ(B(A_p))");
                    // PC1: the change, its bound, its reading.
                    let x = w.freedom().relations.iter().find(|x| x.step == st && x.component == c && x.edge == e).expect("PersistenceChange.relations: every step, component, relation");
                    let h = dav(e).plus(&dav(e));
                    assert!(eq(x.change, &ap(e)), "RelationChange.change");
                    assert!(up(x.half_width, &h), "RelationChange.half_width: both registrations' bounds");
                    assert_eq!(x.reading == Reading::Freedom, exceeds(&ap(e), &h), "RelationChange.reading");
                }
                for v in 0..rel.n_nodes {
                    assert!(eq(p.leaving[c][v], &half_ref(rel, &ap, v, true)), "the readback of the change, leaving");
                    assert!(eq(p.entering[c][v], &half_ref(rel, &ap, v, false)), "the readback of the change, entering");
                }
            }
        }
        assert_eq!(w.freedom().relations.len(), (regs - 1) * k * m, "PersistenceChange.relations: each once");
        assert!(f.ground.freedom.is_ok(), "GroundReading.freedom");
    });
}

/// Energy: the signed exchange on the carrier, its bound and side, and the
/// share of sides resolved; on both paths.
#[test]
fn energy_is_formed_as_stated() {
    for_all("formation: energy", 0xF0_04, |r| {
        let w = field(r);
        let (d, rel) = (&w.declared, &w.rel);
        let c = d.carrier();
        let check = |en: &abr_kernel::ground_variables::Energy, val: &dyn Fn(usize, usize) -> Expansion, bnd: &dyn Fn(usize, usize) -> Expansion| {
            let mut resolved = 0usize;
            for (t, row) in en.signed.iter().enumerate() {
                assert_eq!(row.len(), rel.n_edges(), "Energy.signed");
                for (e, x) in row.iter().enumerate() {
                    let (v, h) = (val(t, e), bnd(t, e));
                    assert!(eq(x.value, &v), "SignedExchange.value: A(c)");
                    assert!(up(x.bound, &h), "SignedExchange.bound");
                    let side = if v.minus(&h).signum() > 0 { Side::Entering } else if v.plus(&h).signum() < 0 { Side::Leaving } else { Side::NotResolved };
                    assert_eq!(x.side, side, "SignedExchange.side: a sign against the bound");
                    if side != Side::NotResolved { resolved += 1; }
                }
            }
            assert_eq!(en.share_resolved, resolved as f64 / (en.signed.len() * rel.n_edges()) as f64, "Energy.share_resolved");
        };
        check(w.field.ground.energy.as_ref().expect("GroundReading.energy"), &|t, e| a_ref(d, rel, t, c, e), &|_, e| da_ref(d, rel, c, e));
        let regs = d.values.len();
        let carrier = RelationCarrier { name: "SYNTHETIC".into(), unit: "u".into(), range: ObservedRange::synthetic(),
            a: (0..regs).map(|_| (0..rel.n_edges()).map(|_| r.eighth(-2.0, 2.0)).collect()).collect(),
            da: (0..regs).map(|_| (0..rel.n_edges()).map(|_| r.delta(0.5)).collect()).collect() };
        let g = RelationalField::from_relations(rel, w.field.relations.clone(), Some(&carrier)).unwrap();
        check(g.ground.energy.as_ref().unwrap(), &|t, e| Expansion::of(carrier.a[t][e]), &|t, e| Expansion::of(carrier.da[t][e]));
    });
}

/// Connection: the identity grade and the sustained grade, recomputed from
/// their definitions; the shares and counts read from them.
#[test]
fn connection_is_formed_as_stated() {
    let reached = RefCell::new((0usize, 0usize));
    for_all("formation: connection", 0xF0_05, |r| {
        let w = field(r);
        let (d, rel, con) = (&w.declared, &w.rel, w.connection());
        let (regs, k, m) = (d.values.len(), d.k(), rel.n_edges());
        // Relations resolved in some component at every registration.
        let resolved = |e: usize, t: usize| (0..k).any(|c| exceeds(&a_ref(d, rel, t, c, e), &da_ref(d, rel, c, e)));
        let unresolved: Vec<usize> = (0..m).filter(|&e| (0..regs).any(|t| !resolved(e, t))).collect();
        let held: Vec<usize> = (0..m).filter(|e| !unresolved.contains(e)).collect();
        assert_eq!(con.unresolved_edges, unresolved, "Connection.unresolved_edges");
        assert_eq!(con.evaluated_pairs, held.len() * held.len().saturating_sub(1) / 2, "Connection.evaluated_pairs");
        let mut identity = Vec::new();
        for (n, &e) in held.iter().enumerate() {
            for &f in &held[n + 1..] {
                let same = (0..regs).all(|t| (0..k).all(|c| !exceeds(&a_ref(d, rel, t, c, e).minus(&a_ref(d, rel, t, c, f)), &da_ref(d, rel, c, e).plus(&da_ref(d, rel, c, f)))));
                if same { identity.push((e, f)); }
            }
        }
        assert_eq!(con.identity, identity, "Connection.identity: A[e] − A[f] not resolved throughout");
        if !identity.is_empty() { reached.borrow_mut().0 += 1; }
        // The sustained grade: the sign of Ξ(B(A)) held resolved in some
        // component at every registration.
        // No locus both entered and left: the grade is a named gap (IV-3).
        let interior = (0..rel.n_nodes).any(|v| rel.edges.iter().any(|x| x.1 == v) && rel.edges.iter().any(|x| x.0 == v));
        let s = match &con.sustained {
            Ok(s) => { assert!(interior, "read only with an interior locus"); s }
            Err(g) => { assert!(!interior, "Connection.sustained: a gap ({g:?}) on a field with an interior locus"); assert!(w.field.ground.connection.is_ok()); return; }
        };
        let not_formable: Vec<usize> = (0..m).filter(|&e| succ(rel, e).is_empty() && pred(rel, e).is_empty()).collect();
        let boundary: Vec<usize> = (0..m).filter(|&e| !not_formable.contains(&e) && (succ(rel, e).is_empty() != pred(rel, e).is_empty())).collect();
        let sign = |t: usize, c: usize, e: usize| -> i8 {
            let a = |f: usize| a_ref(d, rel, t, c, f);
            let dav = |f: usize| da_ref(d, rel, c, f);
            let (v, h) = (xi_ref(rel, &a, e, false), xi_ref(rel, &dav, e, true));
            if v.minus(&h).signum() > 0 { 1 } else if v.plus(&h).signum() < 0 { -1 } else { 0 }
        };
        let held_s: Vec<usize> = (0..m).filter(|e| !not_formable.contains(e))
            .filter(|&e| (0..k).any(|c| { let s0 = sign(0, c, e); s0 != 0 && (0..regs).all(|t| sign(t, c, e) == s0) })).collect();
        // Reorganizing: not held, and the sign changed state in some
        // component. A relation unresolved throughout in every component is
        // in neither list (a finding, 2026-11-26: reported nowhere).
        let reorg: Vec<usize> = (0..m).filter(|e| !not_formable.contains(e) && !held_s.contains(e))
            .filter(|&e| (0..k).any(|c| (0..regs).any(|t| sign(t, c, e) != sign(0, c, e)))).collect();
        assert_eq!(s.not_formable, not_formable, "Sustained.not_formable");
        assert_eq!(s.boundary_conditioned, boundary, "Sustained.boundary_conditioned");
        assert_eq!(s.held_edges, held_s, "Sustained.held_edges");
        assert_eq!(s.reorganizing_edges, reorg, "Sustained.reorganizing_edges");
        let share = |num: usize, den: usize| if den == 0 { None } else { Some(num as f64 / den as f64) };
        let both: Vec<usize> = (0..m).filter(|e| !not_formable.contains(e) && !boundary.contains(e)).collect();
        assert_eq!(s.share_held, share(held_s.iter().filter(|e| both.contains(e)).count(), both.len()), "Sustained.share_held");
        assert_eq!(s.share_held_boundary, share(held_s.iter().filter(|e| boundary.contains(e)).count(), boundary.len()), "Sustained.share_held_boundary");
        let formable = m - not_formable.len();
        assert!(formable > 0, "a sustained grade is read only where some relation is formable");
        assert_eq!(s.share_held_all, held_s.len() as f64 / formable as f64, "Sustained.share_held_all");
        if !held_s.is_empty() { reached.borrow_mut().1 += 1; }
        assert!(w.field.ground.connection.is_ok(), "GroundReading.connection");
    });
    let (i, s) = reached.into_inner();
    assert!(i > 0 && s > 0, "the family reaches identity pairs ({i}) and held passages ({s})");
}

/// The ways of the two parts: n − c and m − n + c, from the declared
/// relations alone.
#[test]
fn the_ways_are_formed_as_stated() {
    for_all("formation: ways", 0xF0_06, |r| {
        let w = field(r);
        let (ways, rel) = (&w.field.ways, &w.rel);
        let c = pieces_ref(rel);
        assert_eq!((ways.relations, ways.loci, ways.pieces), (rel.n_edges(), rel.n_nodes, c));
        assert_eq!(ways.potential, rel.n_nodes - c, "Ways.potential");
        assert_eq!(ways.balanced, rel.n_edges() + c - rel.n_nodes, "Ways.balanced");
    });
}

// ── Completeness, and that the references can tell ──────────────────────

/// The register entries a list does not account for: every part claiming an
/// operator, a registration or a rule must be formed or listed with a
/// reason, and an operator reading or a registration may not be listed.
fn unaccounted(reg: &[(&str, &str)], formed: &[&str], not_recomputed: &[(&str, &str)]) -> Vec<String> {
    let mut why = Vec::new();
    for (name, kind) in reg {
        let listed = not_recomputed.iter().any(|x| x.0 == *name);
        match *kind {
            "operator" | "registered" if listed => why.push(format!("{name}: a {kind} part may not go without its recomputation")),
            "operator" | "registered" | "rule" if !formed.contains(name) && !listed => why.push(format!("{name}: claims a {kind} formation and is not recomputed")),
            _ => {}
        }
    }
    for f in formed { if !reg.iter().any(|x| x.0 == *f) { why.push(format!("{f}: recomputed, but not in the register")); } }
    why
}

fn kind(d: &Derivation) -> &'static str {
    match d {
        Derivation::Operator { .. } => "operator", Derivation::Registered { .. } => "registered", Derivation::Rule { .. } => "rule",
        Derivation::Identity { .. } => "identity", Derivation::Legacy { .. } => "legacy",
    }
}

#[test]
fn every_claimed_formation_is_recomputed() {
    let reg: Vec<(&str, &str)> = register().iter().map(|(n, d)| (*n, kind(d))).collect();
    let why = unaccounted(&reg, FORMED, NOT_RECOMPUTED);
    assert!(why.is_empty(), "\n{}\n", why.join("\n"));
    for (n, _) in NOT_RECOMPUTED { assert!(matches!(of_part(n), Some(Derivation::Rule { .. })), "{n} is listed as a rule not recomputed"); }
}

#[test]
fn the_completeness_check_can_fail() {
    let reg = [("A.x", "operator"), ("A.y", "registered"), ("A.z", "rule")];
    assert!(unaccounted(&reg, &["A.x", "A.y", "A.z"], &[]).is_empty());
    assert_eq!(unaccounted(&reg, &["A.y", "A.z"], &[]).len(), 1, "an operator reading not recomputed");
    assert_eq!(unaccounted(&reg, &["A.y"], &[("A.x", "why"), ("A.z", "why")]).len(), 1, "an operator listed as not recomputed");
    assert_eq!(unaccounted(&reg, &["A.x", "A.y", "A.z", "A.w"], &[]).len(), 1, "a stale entry");
}

/// The references tell the operators apart: on the family, B with
/// predecessors in place of successors, and Ξ with the signs dropped,
/// differ from the definitions somewhere, so a field computing either would
/// fail above.
#[test]
fn the_references_tell_the_operators_apart() {
    let differs = RefCell::new((0usize, 0usize));
    for_all("formation: the references can tell", 0xF0_07, |r| {
        let w = field(r);
        let (d, rel) = (&w.declared, &w.rel);
        let a = |e: usize| a_ref(d, rel, 0, 0, e);
        for e in 0..rel.n_edges() {
            let mut wrong_b = a(e);
            for p in pred(rel, e) { wrong_b = wrong_b.plus(&a(p)); }
            if wrong_b.minus(&b_ref(rel, &a, e)).signum() != 0 { differs.borrow_mut().0 += 1; }
            if xi_ref(rel, &a, e, true).minus(&xi_ref(rel, &a, e, false)).signum() != 0 { differs.borrow_mut().1 += 1; }
        }
    });
    let (b, x) = differs.into_inner();
    assert!(b > 0 && x > 0, "B by predecessors differs {b} times; Ξ with signs dropped differs {x} times");
}
