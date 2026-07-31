// lib.rs — Metatron Dynamics, Inc.
// Crate root for the ABR relational kernel.
// Bounded over D. No claim beyond D.
//
// Grounding documents:
//   docs/kernel/operators_notation_and_constraint.md
//   docs/kernel/abr_operators_plain.md
//   docs/kernel/role_separation_and_operator_application.md
//   docs/kernel/observable_variable_sets.md
//
// Process and provenance discipline (added 2026-07-30, content above
// unchanged by it):
//   docs/process/observable_provenance_and_reverse_traceability.md
//   docs/process/language_discipline_rust_mandate.md
//   docs/process/kernel_self_consistency_test.md
//   docs/process/Verification_pass_protocol.md

pub mod operators;
pub mod derived_invariants;
