//! The Planck step at the Primary Region (Origin, 2026-11-23, 60th, 62nd,
//! 63rd): a declared projection, not a reading.
//!
//! What is checked: the projection agrees with the CODATA 2018 listed value
//! t_P = 5.391 247(60) × 10⁻⁴⁴ s within its stated uncertainty; it is
//! classed Projected; and the finest registered time resolution does not
//! resolve it, so it cannot enter as a registration (OC-RANGE-1).
//!
//! Not evaluated, by the rule that admits it (W15 as amended, 63rd): no
//! relational field is formed, because the Planck step enters no form and no
//! reading is based on it. That is why this test forms no whole field.

use abr_kernel::derived_invariants::planck_step;
use abr_kernel::ground_variables::{sign_at_delta, AtDelta};

#[test]
fn the_planck_step_agrees_with_the_listed_value_within_its_uncertainty() {
    let t = planck_step();
    let (listed, listed_u) = (5.391_247e-44, 0.000_060e-44);
    assert!((t.value - listed).abs() <= t.half_width + listed_u);
    assert!(t.half_width > 0.0 && t.half_width < listed_u);
    assert!(t.class.starts_with("Projected"));
    assert_eq!(t.unit, "s");
}

/// Not resolvable by any registration in the human frame: at the finest
/// registered time resolution (about 10⁻¹⁸ s, attosecond metrology), the
/// Planck step reads as not resolved, about 10²⁶ times below δ. A registration
/// that resolved it would need a declared δ below 5.4 × 10⁻⁴⁴ s, which no
/// observing chain has. So it enters only as a label, never as a registration.
#[test]
fn the_planck_step_is_not_resolved_by_any_registered_resolution() {
    let t = planck_step();
    let finest_registered = 1e-18;
    assert_eq!(sign_at_delta(t.value, finest_registered), AtDelta::NotResolved);
    assert!(finest_registered / t.value > 1e25);
}
