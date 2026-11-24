//! Counts of registered events admitted at δ = 0 (68th; `ObservedRange::Counts`),
//! read in a whole field formed from relations registered directly (59th).
//!
//! What is shown: a field whose relations are exact counts is formed, and on
//! it PC1 reads the one count that changed over a step as freedom and every
//! other as structure, at δ = 0; a non-integer, a negative value, or a value
//! beyond 2⁵³ at δ = 0 is refused; the human range still refuses a non-zero
//! value at δ = 0.
//!
//! [IMPL] SYNTHETIC FIXTURES of the forms; the counts are declared for the test.
//! Not evaluated (named gaps): energy (no carrier declared, `NoCarrierDeclared`);
//! connection grades are formed by the field but not read here.

#[path = "support/synthetic_whole_field.rs"]
mod support;
use support::rel;

use abr_kernel::ground_variables::{DeclarationGap, ObservedRange};
use abr_kernel::persistence_change::{EdgeSeries, Reading};
use abr_kernel::relational_field::RelationalField;

fn counts() -> ObservedRange { ObservedRange::counts("SYNTHETIC: events counted for the test") }

#[test]
fn a_field_of_counts_reads_the_count_that_changed() {
    // Three relations, two components (declared witnesses, input), counts at δ = 0.
    let a = vec![
        vec![vec![2.0, 1.0, 0.0], vec![0.0, 3.0, 5.0]],
        vec![vec![2.0, 1.0, 0.0], vec![0.0, 4.0, 5.0]],
    ];
    let da = vec![vec![vec![0.0; 3]; 2]; 2];
    let series = EdgeSeries::from_edge_values_in(a, da, counts()).unwrap();
    let field = RelationalField::from_relations(&rel(3, vec![(0, 1), (1, 2), (0, 2)]), series, None).unwrap();
    assert_eq!(field.ground.energy.as_ref().err(), Some(&DeclarationGap::NoCarrierDeclared), "energy named as not evaluated");
    let pc = field.ground.freedom.as_ref().unwrap();
    let freed: Vec<(usize, usize)> = pc.relations.iter().filter(|r| r.reading == Reading::Freedom).map(|r| (r.component, r.edge)).collect();
    assert_eq!(freed, vec![(1, 1)], "only the input count of relation 1 changed");
    assert_eq!(pc.relations.iter().filter(|r| r.reading == Reading::Structure).count(), 5);
}

#[test]
fn counts_admit_only_exact_non_negative_integers_at_zero_bound() {
    let c = counts();
    assert!(c.admits(0.0, 0.0) && c.admits(7.0, 0.0) && c.admits(9_007_199_254_740_992.0, 0.0));
    assert!(!c.admits(0.5, 0.0) && !c.admits(-1.0, 0.0) && !c.admits(9_007_199_254_740_994.0, 0.0));
    assert!(c.admits(3.0, 1.0), "with δ > 0, the human range");
    assert!(!ObservedRange::Human.admits(7.0, 0.0), "the human range is unchanged");
}

#[test]
#[should_panic(expected = "outside the observed range")]
fn a_fractional_count_is_refused() {
    let _ = EdgeSeries::from_edge_values_in(vec![vec![vec![0.5]], vec![vec![1.0]]], vec![vec![vec![0.0]]; 2], counts());
}
