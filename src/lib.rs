// lib.rs — Metatron Dynamics, Inc. Kernel V9.
// Crate root for the ABR relational kernel.
// Bounded over D. No claim beyond D.
//
// Foundational primitive (Kernel V9): the foundational observable primitive is a
// directed relation declared from observable information. A numerical value is not
// itself a primitive; it is the projection of an observed relational distinction
// through a declared measurement mapping M. Observed relations, measurement mappings
// M, numerical projections in D, and calculated results are held distinct.
// See docs/kernel/kernel_reference.md §1.
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
//
// Kernel V9 synchronization (2026-10-25): release designation updated; no code changed.

// Operator requirement (V7.1, 2026-09-23):
//   No operator runs without a declared M. The kernel states operator forms
//   only and holds no parameter values. ρ_base, χ₀, and cc (one per declared
//   component pair) are supplied by M for each analysis, with no defaults.
//   See operators.rs, "M-declared operator parameters".

// Observables, M, and D (Kernel V8, Origin declaration, 2026-09-26):
//   O → M → D. M : O → D is declared by Origin before any operator acts.
//   A value in D is the numerical projection of an observable through M.
//   Relations, variables, and topology are declared through M from
//   observed distinctions.
//   Operators act only on declared numerical projections in D and return
//   their results unchanged. Results are compared with the observables
//   from which the declaration was constructed. Where correspondence is not
//   established, the declaration may be varied by Origin and evaluated
//   again. The kernel does not vary the declaration.
//   The kernel makes no declaration about what exists or does not exist.
//   Where information required by an operator has not been declared
//   through M for the calculation, that operator is not evaluated. No
//   substitute value or classification is introduced. A 0.0 produced by
//   an operator over declared information is a calculated result and is
//   returned as such.
//   No numerical result is itself an observable. D holds projections
//   admitted through M. Bounded over D. No claim beyond D.

pub mod operators;
pub mod derived_invariants;
