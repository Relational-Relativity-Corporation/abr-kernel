//! Signed energy (Origin, 2026-11-26, 103rd; 97th (1)).
//!
//! Origin: *"energy is present and moving through the system and further
//! moves the system through relational evolution"*; *"magnitude really can't
//! be its own variable because energy is relative so magnitude is only
//! determinant to the degree to which the energy moves through the system and
//! participates in relational evolution"*.
//!
//! The variable: at every registration t and relation e (leaving locus s,
//! entering locus t), the signed exchange A(c)_t[e], the movement of energy
//! across the relation against its declared direction:
//!   value  A(c)[e], exact, rounded once: c[s] − c[t] for a carrier at the
//!          loci, the registered value for a carrier across the relations;
//!   bound  δ_c[s] + δ_c[t], or as registered, rounded up (93rd);
//!   side   read at δ (98th): toward the entering locus (positive), toward
//!          the leaving locus (negative), or not resolved.
//! Magnitude is not a variable: |A(c)| is fenced as a reading of energy; the
//! tunneling floor is the side resolved.
//!
//! Every fixture is a whole relational field (53rd; Df6, W11): energy,
//! freedom and connection are evaluated together (`whole` asserts the rule
//! of inclusion), and each claim is read from the parts the field reports.
//! **Not read here (named gaps):** freedom and connection are formed but no
//! claim reads them; energy's movement through B, Ξ, the readback and the
//! step change is step two (103rd), not yet built.
//!
//! [IMPL] SYNTHETIC FIXTURES, enumerated from declared seeds (W17): dyadic
//! values, so every difference is exact. They establish that the reading is
//! formed as stated, nothing empirical.

#[path = "support/synthetic_whole_field.rs"]
mod support;

use std::cell::Cell;

use abr_kernel::derivation::{of_part, reported_names, Derivation};
use abr_kernel::exact::{exceeds, Expansion};
use abr_kernel::ground_variables::{Energy, ObservedRange, RelationCarrier, Side};
use abr_kernel::operators::DeclaredRelations;
use abr_kernel::relational_field::RelationalField;
use support::*;

/// The side the 103rd declares, decided exactly from the exact value and
/// bound.
fn side_of(v: &Expansion, h: &Expansion) -> Side {
    if !exceeds(v, h) { Side::NotResolved } else if v.signum() > 0 { Side::Entering } else { Side::Leaving }
}

fn energy_of(field: &RelationalField) -> &Energy {
    field.ground.energy.as_ref().expect("a carrier is declared, so energy is read")
}

/// A whole field formed at the loci, with the carrier as its last component.
fn whole_at_loci(r: &mut Seeds) -> Whole {
    let rel = enumerated_rel(r);
    let k = r.usize(2, 3);
    let regs = r.usize(2, 4);
    let z1a = r.selects(3);
    whole(&rel, enumerated_declared(r, &rel, k, regs, 0.5, z1a, false))
}

/// A carrier registered across the relations: values and bounds per
/// relation and registration, on the dyadic grid.
fn carrier_across(r: &mut Seeds, rel: &DeclaredRelations, regs: usize) -> RelationCarrier {
    let m = rel.n_edges();
    RelationCarrier {
        name: "SYNTHETIC carrier across the relations".into(), unit: "synthetic".into(),
        a: (0..regs).map(|_| (0..m).map(|_| r.eighth(-2.0, 2.0)).collect()).collect(),
        da: (0..regs).map(|_| (0..m).map(|_| r.delta(0.5)).collect()).collect(),
        range: ObservedRange::synthetic(),
    }
}

/// A whole field of relations registered directly, with a carrier across
/// them.
fn field_of_relations(w: &Whole, carrier: &RelationCarrier) -> RelationalField {
    let f = RelationalField::from_relations(&w.rel, w.field.relations.clone(), Some(carrier)).expect("the relational field forms");
    inclusion_of(&f);
    f
}

/// 1. The signed value is A(c) exactly, its bound the operator with its
/// signs dropped, its side read at δ: at the loci, c[s] − c[t] with
/// δ_c[s] + δ_c[t]; across the relations, the registered value and bound.
#[test]
fn one_the_signed_exchange_is_a_of_the_carrier() {
    for_all("signed energy: at the loci", 0x103_01, |r| {
        let w = whole_at_loci(r);
        let en = energy_of(&w.field);
        assert_eq!(en.signed.len(), w.carriers.len(), "one reading per registration");
        for (t, c) in w.carriers.iter().enumerate() {
            assert_eq!(en.signed[t].len(), w.rel.n_edges(), "one reading per relation");
            for (e, &(s, u)) in w.rel.edges.iter().enumerate() {
                let x = &en.signed[t][e];
                let v = Expansion::diff(c.values[s], c.values[u]);
                let h = Expansion::of(c.delta[s]).plus(&Expansion::of(c.delta[u]));
                assert_eq!(x.edge, e);
                assert_eq!(x.value, v.nearest(), "registration {t}, relation {e}: A(c) = c[s] − c[t], rounded once");
                assert_eq!(x.bound, h.up(), "registration {t}, relation {e}: δ_c[s] + δ_c[t], rounded up");
                assert_eq!(x.side, side_of(&v, &h), "registration {t}, relation {e}");
            }
        }
    });
    for_all("signed energy: across the relations", 0x103_02, |r| {
        let w = whole_at_loci(r);
        let carrier = carrier_across(r, &w.rel, w.carriers.len());
        let f = field_of_relations(&w, &carrier);
        let en = energy_of(&f);
        for t in 0..carrier.a.len() {
            for e in 0..w.rel.n_edges() {
                let x = &en.signed[t][e];
                let (v, h) = (Expansion::of(carrier.a[t][e]), Expansion::of(carrier.da[t][e]));
                assert_eq!(x.value, carrier.a[t][e], "the registered movement, unchanged");
                assert_eq!(x.bound, carrier.da[t][e]);
                assert_eq!(x.side, side_of(&v, &h));
            }
        }
    });
}

/// 2. Reversing a relation's declared direction reverses its side and its
/// value, and leaves its bound and every other relation's reading alone.
/// Under |A(c)| nothing would change: this test tells the two apart.
#[test]
fn two_reversing_a_relation_reverses_its_side() {
    let flipped = Cell::new(0usize);
    for_all("signed energy: reversed direction", 0x103_03, |r| {
        let w = whole_at_loci(r);
        let e0 = r.usize(0, w.rel.n_edges() - 1);
        let mut edges = w.rel.edges.clone();
        edges[e0] = (edges[e0].1, edges[e0].0);
        let back = whole(&rel(w.rel.n_nodes, edges), w.declared.clone());
        let (a, b) = (energy_of(&w.field), energy_of(&back.field));
        for t in 0..a.signed.len() {
            for e in 0..w.rel.n_edges() {
                let (x, y) = (&a.signed[t][e], &b.signed[t][e]);
                assert_eq!(x.bound, y.bound);
                if e == e0 {
                    assert_eq!(y.value, -x.value, "registration {t}: the movement reverses with the direction");
                    let want = match x.side { Side::Entering => Side::Leaving, Side::Leaving => Side::Entering, Side::NotResolved => Side::NotResolved };
                    assert_eq!(y.side, want, "registration {t}");
                    if x.side != Side::NotResolved { flipped.set(flipped.get() + 1); }
                } else {
                    assert_eq!(x, y, "registration {t}, relation {e}: untouched");
                }
            }
        }
    });
    assert!(flipped.get() > 0, "the family reaches resolved sides that reverse");
}

/// 3. Two carriers that differ only in sign: a magnitude cannot tell them
/// apart; the signed exchange reads opposite sides. *Was (103rd):* the
/// magnitude read from the field's fenced ε; retired (104th), so the test
/// takes |·| itself, in the framework.
#[test]
fn three_carriers_differing_only_in_sign_read_opposite_sides() {
    let opposite = Cell::new(0usize);
    for_all("signed energy: opposite carriers", 0x103_04, |r| {
        let w = whole_at_loci(r);
        let c = carrier_across(r, &w.rel, w.carriers.len());
        let neg = RelationCarrier { a: c.a.iter().map(|row| row.iter().map(|v| -v).collect()).collect(), ..c.clone() };
        let (f, g) = (field_of_relations(&w, &c), field_of_relations(&w, &neg));
        let (a, b) = (energy_of(&f), energy_of(&g));
        for t in 0..c.a.len() {
            for e in 0..w.rel.n_edges() {
                let (x, y) = (&a.signed[t][e], &b.signed[t][e]);
                assert_eq!(x.value.abs(), y.value.abs(), "a magnitude is the same for both");
                assert_eq!(y.value, -x.value);
                match (x.side, y.side) {
                    (Side::Entering, Side::Leaving) | (Side::Leaving, Side::Entering) => opposite.set(opposite.get() + 1),
                    (Side::NotResolved, Side::NotResolved) => {}
                    other => panic!("registration {t}, relation {e}: {other:?}"),
                }
            }
        }
    });
    assert!(opposite.get() > 0, "the family reaches movements the magnitude cannot tell apart");
}

/// 4. The tunneling floor is the side resolved: its share is the share of
/// resolved sides, on both paths. **Cannot tell (named):** it holds under a
/// magnitude reading too (resolved is the same decision either way).
/// *Was (103rd):* also checked the fenced floor against the side; the floor
/// retired with the magnitude (104th).
#[test]
fn four_the_floor_is_the_side_resolved() {
    let check = |en: &Energy| {
        let mut resolved = 0usize;
        let mut total = 0usize;
        for x in en.signed.iter().flatten() {
            total += 1;
            if x.side != Side::NotResolved { resolved += 1; }
        }
        assert_eq!(en.share_resolved, resolved as f64 / total as f64);
    };
    for_all("signed energy: the floor", 0x103_05, |r| {
        let w = whole_at_loci(r);
        check(energy_of(&w.field));
        let c = carrier_across(r, &w.rel, w.carriers.len());
        check(energy_of(&field_of_relations(&w, &c)));
    });
}

/// 5. A movement not resolved at δ is reported as not resolved, with its
/// value and bound as formed, never filled with zero.
#[test]
fn five_a_side_not_resolved_keeps_its_value_and_bound() {
    let nonzero = Cell::new(0usize);
    for_all("signed energy: not resolved", 0x103_06, |r| {
        let w = whole_at_loci(r);
        for (t, c) in w.carriers.iter().enumerate() {
            for (e, &(s, u)) in w.rel.edges.iter().enumerate() {
                let x = &energy_of(&w.field).signed[t][e];
                if x.side != Side::NotResolved { continue; }
                assert_eq!(x.value, Expansion::diff(c.values[s], c.values[u]).nearest(), "the value as formed");
                assert!(x.value.abs() <= x.bound, "not resolved: within its bound");
                if x.value != 0.0 { nonzero.set(nonzero.get() + 1); }
            }
        }
    });
    assert!(nonzero.get() > 0, "the family reaches unresolved movements that are not zero");
}

/// 6. Nothing the field reports as energy is a magnitude on its own: the
/// field reports the signed exchange, every part of it derived from A, and
/// any part that reads |A(c)| is fenced (103rd).
#[test]
fn six_no_reading_of_energy_is_a_magnitude_on_its_own() {
    let mut r = Seeds(0x103_07);
    let w = whole_at_loci(&mut r);
    let names = reported_names(&format!("{:?}", energy_of(&w.field)));
    for part in ["Energy.signed", "SignedExchange.edge", "SignedExchange.value", "SignedExchange.bound", "SignedExchange.side"] {
        assert!(names.iter().any(|n| n == part), "the field reports {part}: {names:?}");
    }
    match of_part("Energy.signed") {
        Some(Derivation::Operator { form, .. }) => assert!(form.contains("A(c)"), "{form}"),
        d => panic!("Energy.signed is an operator reading of A on the carrier: {d:?}"),
    }
    for n in &names {
        let d = of_part(n).unwrap_or_else(|| panic!("{n} has a derivation"));
        let text = match d {
            Derivation::Operator { form, .. } => form.to_string(),
            Derivation::Rule { rule, .. } => rule.to_string(),
            Derivation::Registered { what } | Derivation::Identity { what } => what.to_string(),
            Derivation::Legacy { method, .. } => method.to_string(),
        };
        let magnitude = text.contains("|A(c)") || text.contains("magnitude") || text.contains("ε/δ");
        assert!(!magnitude || text.contains("fenced (103rd)"), "{n} reads a magnitude of energy and is not fenced: {text}");
    }
}
