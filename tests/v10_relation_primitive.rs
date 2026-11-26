//! Relation as primitive (Origin, 2026-11-23, 59th): the whole relational
//! field formed from registrations of the relations themselves, with energy,
//! connection and freedom calculated together from them.
//!
//! What these tests show, and what they do not:
//!   1. The field formed at the loci is the special case of the field formed
//!      from relations: on every enumerated field, forming the relations from
//!      the loci and reading them as registered gives every form and every
//!      ground variable unchanged.
//!   2. A field registered on its relations reads a resolved turn sum, which
//!      no field formed from loci can give (there every turn sum is 0 by
//!      identity), with energy, connection and freedom all evaluated.
//!   3. A gain declared on registered relations (Z1a) enters the connection
//!      grades read from them: an independent gain the proportional grade, a
//!      gain differing between relations the identity grade.
//!
//! [IMPL] SYNTHETIC FIXTURES: dyadic values, so every sum and product is
//! exact. They establish that the forms compute as stated, nothing empirical.
//! Not evaluated here: §2.7's declared field (`DeclaredField`), which is still
//! formed from locus registrations; and the sustained grade under a declared
//! gain (test 3 reads identity and proportional only).

#[path = "support/synthetic_whole_field.rs"]
mod support;
use support::*;

use abr_kernel::ground_variables::{minor_pattern, minor_pattern_of, RelationCarrier};
use abr_kernel::persistence_change::{turn_change_sum, turn_change_sum_resolved, turn_sum, turn_sum_resolved, EdgeGains, EdgeSeries};
use abr_kernel::relational_field::RelationalField;

/// 1. The locus field is the special case. Over CASES enumerated fields, the
/// relations the locus field formed, read as registered, give the same forms
/// and the same energy, connection and freedom.
#[test]
fn the_locus_field_is_the_special_case_of_the_relation_field() {
    for_all("locus special case", 0x59_01, |r| {
        let rel = enumerated_rel(r);
        let k = r.usize(2, 3);
        let regs = r.usize(2, 4);
        let z1a = r.selects(4);
        let d = enumerated_declared(r, &rel, k, regs, 0.5, z1a, false);
        let w = whole(&rel, d);
        let carrier = RelationCarrier::from_loci(&w.carriers, &rel);
        let of_relations = RelationalField::from_relations(&rel, w.field.relations.clone(), Some(&carrier))
            .expect("the field forms from its relations");
        assert_eq!(of_relations.spatial, w.field.spatial);
        assert_eq!(of_relations.persistence, w.field.persistence);
        assert_eq!(of_relations.ground.connection, w.field.ground.connection);
        assert_eq!(of_relations.ground.freedom, w.field.ground.freedom);
        assert_eq!(of_relations.ground.v2_held_arrangement, w.field.ground.v2_held_arrangement);
        // Dyadic fixtures: every carrier difference is exact, so the nearest
        // f64 the relation carrier holds is the exact value the locus path
        // decides on. A carrier difference that rounds is not covered here.
        assert_eq!(of_relations.ground.energy, w.field.ground.energy);
        // The comparisons above share the relation path, since the locus
        // field now reads through it. The minor pattern read straight from the
        // loci (`minor_pattern`, x[s] − x[t] and the state bounds at both
        // ends) is independent of it: the two agree at every registration.
        for (t, reg) in w.field.registrations.iter().enumerate() {
            assert_eq!(minor_pattern_of(&w.field.relations, t), minor_pattern(reg, &rel));
        }
        // Formed from loci, every turn sums to zero exactly: relation reduces
        // to the loci there.
        for t in 0..w.field.relations.n_registrations() {
            for c in 0..k {
                for (e, f, g) in closed_triangles(&rel) {
                    let (v, _) = turn_sum(&w.field.relations, &rel, &[e, f, g], t, c).unwrap();
                    assert_eq!(v, 0.0, "a locus-formed turn sum is 0 by identity");
                }
            }
        }
    });
}

/// Turns of three declared relations (any direction), as (edge, sign).
fn closed_triangles(rel: &abr_kernel::operators::DeclaredRelations) -> Vec<((usize, i8), (usize, i8), (usize, i8))> {
    let mut out = Vec::new();
    let m = rel.n_edges();
    for e in 0..m { for f in 0..m { for g in 0..m {
        if e == f || f == g || e == g { continue; }
        for se in [1i8, -1] { for sf in [1i8, -1] { for sg in [1i8, -1] {
            let walk = [(e, se), (f, sf), (g, sg)];
            if abr_kernel::persistence_change::is_turn(rel, &walk) { out.push(((e, se), (f, sf), (g, sg))); }
        }}}
    }}}
    out
}

/// The declared triangle 0 → 1, 1 → 2, 0 → 2: the walk e0, e1, −e2 is closed.
fn triangle() -> abr_kernel::operators::DeclaredRelations { rel(3, vec![(0, 1), (1, 2), (0, 2)]) }
const WALK: [(usize, i8); 3] = [(0, 1), (1, 1), (2, -1)];

/// 2. Registered on its relations, the field reads a turn sum that no locus
/// field can give, with every variable evaluated together.
#[test]
fn a_relation_registered_field_reads_a_resolved_turn_sum_with_every_variable() {
    let rel = triangle();
    // a[t][c][e]: component 0 sums to 1 (1 + 1 − 1) at every registration;
    // component 1 changes between registrations. δ = 1/8 throughout.
    let a = vec![
        vec![vec![1.0, 1.0, 1.0], vec![2.0, -1.0, 0.5]],
        vec![vec![1.0, 1.25, 1.25], vec![2.0, -0.5, 1.0]],
        vec![vec![1.5, 1.25, 1.75], vec![1.0, -0.5, 1.5]],
    ];
    let da = vec![vec![vec![0.125; 3]; 2]; 3];
    let series = EdgeSeries::from_edge_values_synthetic(a, da).unwrap();
    let carrier = relation_carrier(vec![vec![0.5, 0.0, 0.25]; 3], vec![vec![0.125; 3]; 3]);
    let field = whole_of_relations(&rel, series, &carrier);

    // Relation: the turn sum of component 0 is 1 ± 3/8 at every registration,
    // resolved. Its change sums to zero exactly (0 ± 3/4 over each step): the
    // inconsistency is fixed over the run, and the change does not show it.
    for t in 0..3 {
        assert_eq!(turn_sum(&field.relations, &rel, &WALK, t, 0), Some((1.0, 0.375)));
        assert_eq!(turn_sum_resolved(&field.relations, &rel, &WALK, t, 0), Some(true));
    }
    for s in 0..2 {
        assert_eq!(turn_change_sum(&field.relations, &rel, &WALK, s, 0), Some((0.0, 0.75)));
        assert_eq!(turn_change_sum_resolved(&field.relations, &rel, &WALK, s, 0), Some(false));
    }
    // Energy, from the carrier registered across the relations, δ_ε = 1/8:
    // e0 resolved (0.5 > 1/8), e1 not (0 ≤ 1/8), e2 resolved (0.25 > 1/8).
    let en = field.ground.energy.as_ref().expect("energy is read");
    for t in 0..3 {
        // The floor is the side resolved (104th). *Was:* `en.exchanges[t]…floor_resolved`.
        let floors: Vec<bool> = en.signed[t].iter().map(|x| x.side != abr_kernel::ground_variables::Side::NotResolved).collect();
        assert_eq!(floors, vec![true, false, true]);
    }
    // Freedom: PC1 is read at every relation, component and step.
    let pc = field.ground.freedom.as_ref().expect("freedom is read");
    assert_eq!(pc.relations.len(), 2 * 2 * 3);
    // Connection: every grade is formed on this field (node 1 is interior,
    // two components, every relation resolved at every registration).
    let conn = field.ground.connection.as_ref().expect("connection is read");
    assert!(conn.proportional.is_ok() && conn.sustained.is_ok() && conn.coupled_change.is_ok());
    // Ξ is read from the registered relations: the forms the field reports.
    assert_eq!(field.spatial[0].a, vec![vec![1.0, 1.0, 1.0], vec![2.0, -1.0, 0.5]]);
}

/// 3a. An independent gain on registered relations enters every minor that
/// mixes components, so it enters the proportional grade. Without it the pair
/// is free (the minor 3 is resolved against 1.625); with γ = 1/2 on every
/// entry the bound is above 7, and the pair varies together at δ.
#[test]
fn an_independent_gain_enters_the_proportional_grade() {
    let rel = triangle();
    let a = vec![vec![vec![2.0, 1.0, 4.0], vec![1.0, 2.0, 4.0]]; 2];
    let da = vec![vec![vec![0.25; 3]; 2]; 2];
    let carrier = relation_carrier(vec![vec![1.0; 3]; 2], vec![vec![0.25; 3]; 2]);
    let plain = whole_of_relations(&rel, EdgeSeries::from_edge_values_synthetic(a.clone(), da.clone()).unwrap(), &carrier);
    let free = plain.ground.v2_held_arrangement.as_ref().unwrap();
    assert!(free.free_pairs.contains(&(0, 1)));
    assert!(!plain.ground.connection.as_ref().unwrap().proportional.as_ref().unwrap().contains(&(0, 1)));

    let mut gains = EdgeGains::none(2, 3);
    gains.independent = vec![vec![0.5; 3]; 2];
    let series = EdgeSeries::from_edge_values_synthetic(a, da).unwrap().with_run_constant(vec![vec![0.0; 3]; 2], gains);
    let gained = whole_of_relations(&rel, series, &carrier);
    assert!(gained.ground.connection.as_ref().unwrap().proportional.as_ref().unwrap().contains(&(0, 1)));
}

/// 3b. A gain that differs between relations enters every sum of one component
/// across relations, so it enters the identity grade. Without it e0 and e1
/// differ (1 against 0.5); with γ = 1/2 on each relation the bound is 2.25,
/// and they are not distinguished at δ.
#[test]
fn a_gain_differing_between_relations_enters_the_identity_grade() {
    let rel = triangle();
    let a = vec![vec![vec![1.0, 2.0, 4.0], vec![1.0, 2.0, -4.0]]; 2];
    let da = vec![vec![vec![0.25; 3]; 2]; 2];
    let carrier = relation_carrier(vec![vec![1.0; 3]; 2], vec![vec![0.25; 3]; 2]);
    let plain = whole_of_relations(&rel, EdgeSeries::from_edge_values_synthetic(a.clone(), da.clone()).unwrap(), &carrier);
    assert!(!plain.ground.connection.as_ref().unwrap().identity.contains(&(0, 1)));

    let mut gains = EdgeGains::none(2, 3);
    gains.edge = vec![0.5; 3];
    let series = EdgeSeries::from_edge_values_synthetic(a, da).unwrap().with_run_constant(vec![vec![0.0; 3]; 2], gains);
    let gained = whole_of_relations(&rel, series, &carrier);
    assert!(gained.ground.connection.as_ref().unwrap().identity.contains(&(0, 1)));
}
