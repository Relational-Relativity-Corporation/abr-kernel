//! Every part a relational field reports has a declared derivation (Origin,
//! 2026-11-25, 92nd): the registered relation itself, an operator reading, a
//! rule applied to operator readings, or legacy mathematics with its
//! provenance, its description in relation to the operators, and its
//! necessity ("Necessity still holds"; W12 as amended, 42nd).
//!
//! Each fixture is a whole relational field: formed at the loci with the
//! carrier declared (`whole`), and from relations registered directly with
//! and without a carrier (`RelationalField::from_relations`), so every
//! variable is evaluated together and every part the field can report is
//! present. The parts are read from the field's own structure (its `Debug`
//! form), so a part added to the field without a derivation fails here.
//!
//! **What this test cannot tell (named):** whether a derivation is *true*.
//! It checks that every reported part is declared and that each declaration
//! states what the 92nd requires; the statements themselves are read and
//! decided by Origin. A part that appears only under a gap this family never
//! reaches is not seen.
//!
//! [IMPL] SYNTHETIC FIXTURES, enumerated from declared seeds (W17). They
//! establish what the field reports, nothing empirical.

#[path = "support/synthetic_whole_field.rs"]
mod support;

use std::cell::RefCell;

use abr_kernel::derivation::{check, legacy_in_use, necessity_open, of_part, register, reported_names, rules_in_use, underived, Derivation};
use abr_kernel::operators::passage_of_accumulation_terms;
use abr_kernel::ground_variables::RelationCarrier;
use abr_kernel::relational_field::RelationalField;
use support::*;

/// Every name reported by the enumerated whole fields, each once.
fn names_reported() -> Vec<String> {
    let all: RefCell<Vec<String>> = RefCell::new(Vec::new());
    let add = |debug: String| {
        let mut a = all.borrow_mut();
        for n in reported_names(&debug) { if !a.contains(&n) { a.push(n); } }
    };
    for_all("derivation: names reported", 0x92_01, |r| {
        let rel = enumerated_rel(r);
        let k = r.usize(2, 3);
        let regs = r.usize(2, 4);
        let z1a = r.selects(4);
        let w = whole(&rel, enumerated_declared(r, &rel, k, regs, 0.5, z1a, false));
        add(format!("{:?}", w.field));
        let carrier = RelationCarrier::from_loci(&w.carriers, &rel);
        let with = RelationalField::from_relations(&rel, w.field.relations.clone(), Some(&carrier)).unwrap();
        add(format!("{with:?}"));
        let without = RelationalField::from_relations(&rel, w.field.relations.clone(), None).unwrap();
        add(format!("{without:?}"));
        // An order over the declared relations (102nd): a walk from the first
        // relation, each step the first relation leaving where the last
        // entered, up to twelve steps; its possibilities are reported parts.
        let mut walk = vec![0usize];
        while walk.len() < 12 {
            let at = rel.edges[*walk.last().unwrap()].1;
            match (0..rel.n_edges()).find(|&e| rel.edges[e].0 == at) { Some(e) => walk.push(e), None => break }
        }
        let order = abr_kernel::order::Order::new(&rel, walk).unwrap();
        add(format!("{order:?}"));
        add(format!("{:?}", order.possibilities()));
        // The degrees of freedom of the current point (108th).
        add(format!("{:?}", order.degrees()));
        // The activity at a point (116th, under trial): the walk read by the
        // radial field over the declared loci, at its last point.
        let mut radial = abr_kernel::radial::RadialField::new(rel.n_nodes);
        for &e in &order.steps { radial.push(rel.edges[e].0, rel.edges[e].1).unwrap(); }
        add(format!("{:?}", radial.activity(radial.steps(), 4).unwrap()));
        // The orthogonal split read from the field (100th): its parts are
        // reported parts too.
        add(format!("{:?}", with.two_parts(&abr_kernel::relational_field::InnerProduct::equal())));
    });
    all.into_inner()
}

/// 1. Nothing the field reports is underived.
#[test]
fn every_part_the_field_reports_has_a_derivation() {
    let names = names_reported();
    assert!(names.len() > 20, "the fields report their parts: {names:?}");
    let missing = underived(&names);
    assert!(missing.is_empty(), "parts reported with no derivation: {missing:?}");
}

/// 2. Every declared derivation states what the 92nd requires: an operator
/// reading names its operators and what it reads; legacy mathematics names
/// its method, its provenance, its description in relation to the operators,
/// and its necessity.
#[test]
fn every_derivation_states_what_the_92nd_requires() {
    for (name, d) in register() {
        // A legacy part whose necessity Origin has not yet stated is a named
        // gap (94th), listed in `necessity_open`; every other requirement
        // still holds for it.
        let open = necessity_open().contains(name);
        match (check(d), d) {
            (Ok(()), _) => assert!(!open, "{name} is listed as open but its necessity is stated"),
            (Err(_), Derivation::Legacy { method, provenance, in_relation_to_operators, necessity: None }) if open => {
                assert!(!method.is_empty() && !provenance.is_empty() && !in_relation_to_operators.is_empty(), "{name}");
            }
            (Err(why), _) => panic!("{name}: {why}"),
        }
    }
    for name in necessity_open() {
        assert!(matches!(of_part(name), Some(Derivation::Legacy { necessity: None, .. })), "{name} listed as open is not a legacy part without its necessity");
    }
    assert!(!legacy_in_use().is_empty(), "the legacy methods in use at the code layer are listed (OC-LM-1)");
    for d in legacy_in_use() {
        assert!(matches!(d, Derivation::Legacy { .. }), "a method in use at the code layer is legacy mathematics: {d:?}");
        if let Err(why) = check(d) { panic!("{d:?}: {why}"); }
    }
}

/// 3. The register lists each part once, and only parts a field reports.
#[test]
fn the_register_lists_each_reported_part_once() {
    let names = names_reported();
    let reg = register();
    for (n, (name, _)) in reg.iter().enumerate() {
        assert!(!reg[..n].iter().any(|(m, _)| m == name), "{name} listed twice");
        assert!(names.iter().any(|x| x == name), "{name} is listed but no field reports it");
    }
}

/// The checks can tell: a reported part with no derivation is found; legacy
/// mathematics missing its provenance, its relation to the operators or its
/// necessity is refused; an operator reading naming no operator is refused.
#[test]
fn the_checks_can_tell() {
    let names = reported_names(r#"Field { relations: S { a: [1.0] }, not_a_declared_part: 3, basis: "x: y" }"#);
    assert_eq!(names, vec!["Field.relations", "S.a", "Field.not_a_declared_part", "Field.basis"],
        "each part named by the struct that holds it; names inside quoted text are not read");
    assert!(underived(&names).contains(&"Field.not_a_declared_part".to_string()));
    let legacy = |provenance, rel, necessity| Derivation::Legacy {
        method: "a method", provenance, in_relation_to_operators: rel, necessity };
    assert!(check(&legacy("a source", "from A", Some("stated"))).is_ok());
    assert!(check(&legacy("", "from A", Some("stated"))).is_err(), "provenance");
    assert!(check(&legacy("a source", "", Some("stated"))).is_err(), "relation to the operators");
    assert!(check(&legacy("a source", "from A", None)).is_err(), "necessity");
    assert!(check(&Derivation::Operator { operators: &[], reads: &[abr_kernel::derivation::Reads::Relation], form: "f" }).is_err());
}

/// 4. The bounds, as split by Origin (93rd): the only legacy mathematics in
/// the bounds is the product bound of the minors; the bounds through A, B and
/// Ξ and the gain widening are rules derived from the operators and the
/// declaration. The legacy methods in use are exactly exact summation and the
/// product bound, and the product bound's necessity names the minors it
/// exists for.
#[test]
fn the_bounds_are_split_as_the_93rd_declares() {
    let methods: Vec<&str> = legacy_in_use().iter().map(|d| match d {
        Derivation::Legacy { method, .. } => *method, _ => "not legacy" }).collect();
    // *Amended (100th):* a third method, exact rationals, for the orthogonal
    // split. *Was:* `assert_eq!(methods.len(), 2, ...)`.
    assert_eq!(methods.len(), 3, "legacy methods in use: {methods:?}");
    assert!(methods[2].starts_with("exact rationals"), "{methods:?}");
    assert!(methods[0].starts_with("exact summation"), "{methods:?}");
    assert!(methods[1].starts_with("product bounds"), "{methods:?}");
    match &legacy_in_use()[1] {
        Derivation::Legacy { necessity: Some(n), .. } => assert!(n.contains("minors"), "the necessity names the minors: {n}"),
        d => panic!("the product bound is legacy with its necessity: {d:?}"),
    }
    let rules: Vec<&str> = rules_in_use().iter().map(|d| match d {
        Derivation::Rule { rule, .. } => *rule, _ => "not a rule" }).collect();
    assert_eq!(rules.len(), 2, "rules in use: {rules:?}");
    assert!(rules[0].starts_with("bounds through A, B and Ξ"), "{rules:?}");
    assert!(rules[1].starts_with("gain widening"), "{rules:?}");
    for d in rules_in_use() { if let Err(why) = check(d) { panic!("{d:?}: {why}"); } }
}

/// 5. The bound of Ξ(B(A)) is |Ξ|∘|B| applied to δ: on every declared field
/// of the family, its coefficient on each relation is never below the
/// magnitude of Ξ(B)'s own coefficient, and exceeds it by an even number, by
/// twice the times that relation enters with opposite signs and cancels in
/// the value. Equal where no relation enters both ways (93rd).
#[test]
fn the_bound_of_xi_is_its_operators_with_the_signs_dropped() {
    let (equal, wider) = (RefCell::new(0usize), RefCell::new(0usize));
    for_all("derivation: the bound of Xi", 0x93_01, |r| {
        let rel = enumerated_rel(r);
        for e in 0..rel.n_edges() {
            let value = passage_of_accumulation_terms(&rel, e, -1);
            let bound = passage_of_accumulation_terms(&rel, e, 1);
            let at = |t: &[(usize, i64)], f: usize| t.iter().find(|x| x.0 == f).map_or(0, |x| x.1);
            for f in 0..rel.n_edges() {
                let (v, b) = (at(&value, f), at(&bound, f));
                assert!(b >= v.abs(), "relation {f} in Ξ[{e}]: bound {b} below |{v}|");
                assert_eq!((b - v.abs()) % 2, 0, "relation {f} in Ξ[{e}]");
                if b == v.abs() { *equal.borrow_mut() += 1 } else { *wider.borrow_mut() += 1 }
            }
        }
    });
    assert!(*equal.borrow() > 0, "the bound equals the operator with the signs dropped somewhere");
    assert!(*wider.borrow() > 0, "and is wider where a relation cancels in the value: the family reaches both");
}

/// The parts read through 2×2 minors (94th).
const MINOR_READINGS: [&str; 11] = [
    "PersistenceChange.across_components", "PersistenceChange.across_steps", "Couplings.readings",
    "ChangeCoupling.minor", "PersistenceChange.arrangement", "ArrangementStep.before", "ArrangementStep.after",
    "Connection.proportional", "Connection.reorganizing_pairs", "GroundReading.v2_held_arrangement", "Freedom.free_pairs",
];

/// 6. The minors are legacy mathematics (Origin, 94th): every part read
/// through a 2×2 minor is listed as legacy, with the exterior algebra as its
/// provenance and its relation to A stated; until Origin states its
/// necessity, it is named open, and the open list is exactly these parts.
#[test]
fn the_minors_are_legacy_mathematics() {
    for name in MINOR_READINGS {
        match of_part(name) {
            Some(Derivation::Legacy { provenance, in_relation_to_operators, .. }) => {
                assert!(provenance.contains("Grassmann"), "{name}: {provenance}");
                assert!(in_relation_to_operators.contains("bilinear form of A"), "{name}: {in_relation_to_operators}");
            }
            d => panic!("{name} is read through a minor and is not listed as legacy: {d:?}"),
        }
    }
    let mut open: Vec<&str> = necessity_open().to_vec();
    // *Amended (116th):* and the activity's comparison, under trial, not
    // admitted, its necessity Origin's to state. *Was:* the minors alone.
    let mut want: Vec<&str> = MINOR_READINGS.to_vec();
    want.extend(["Agreement.agrees", "Agreement.differs"]);
    open.sort();
    want.sort();
    assert_eq!(open, want, "the parts whose necessity is open are exactly the minor readings and the activity's comparison");
}

/// The parts formed only from relations selected by δ (resolved, or moved)
/// before their pairs are formed (98th).
const SELECTED_BY_DELTA: [&str; 9] = [
    "Connection.identity", "PersistenceChange.across_components", "PersistenceChange.across_steps",
    "Couplings.readings", "PersistenceChange.arrangement", "Connection.proportional",
    "Connection.reorganizing_pairs", "GroundReading.v2_held_arrangement", "Freedom.free_pairs",
];

/// 7. δ decides only at reading (Origin, 98th): where a reading selects by δ
/// what enters before it forms its parts, its derivation names the
/// departure; and A, B, Ξ, which select nothing by δ, name none.
#[test]
fn a_selection_by_delta_is_named_as_a_departure() {
    const MARK: &str = "selects by δ what enters before its pairs are formed: a departure from the 98th";
    let text = |d: &Derivation| match d {
        Derivation::Operator { form, .. } => form.to_string(),
        Derivation::Legacy { method, .. } => method.to_string(),
        Derivation::Rule { rule, .. } => rule.to_string(),
        Derivation::Registered { what } | Derivation::Identity { what } => what.to_string(),
    };
    for name in SELECTED_BY_DELTA {
        let d = of_part(name).unwrap_or_else(|| panic!("{name} is registered"));
        assert!(text(d).contains(MARK), "{name} does not name its selection by δ: {}", text(d));
    }
    // *Was:* the list ended with "EnergyExchange.epsilon" (retired, 104th).
    for name in ["Forms.a", "Forms.b", "Forms.passage", "RelationalField.persistence", "SignedExchange.value"] {
        assert!(!text(of_part(name).unwrap()).contains(MARK), "{name} selects nothing by δ");
    }
}
