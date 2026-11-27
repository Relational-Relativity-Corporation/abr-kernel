// lib.rs — Metatron Dynamics, Inc. Kernel V10.
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
// Kernel V10 synchronization (2026-11-21): release designation updated to
//   Kernel V10. Comment-only change in this file; no function changed.
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

// ── V9 fence (Origin, 2026-11-21) ────────────────────────────────────────
// Retired and superseded V9 elements compile only with the `v9` feature,
// which is off by default: the ρ-weighted operators (compute_rho*,
// operator_sigma, antisymmetric_term, operator_e_primary, operator_r,
// operator_e, operator_r_persistence, operator_e_v5, require_rho_declaration,
// require_cc_declaration), the superseded readers (epsilon_e,
// detection_indicator, superposition_condition, superposition_resolved,
// entanglement_condition, decoherence_condition, tunneling_floor_holds), and
// the V9 execution paths built on them (diagnostic_exec, provenanced_exec,
// admission). V10 work cannot use them by accident: a call does not compile.
// V9 work opts in visibly with `features = ["v9"]`. The code stays where it
// was, with its history. See docs/kernel/v10_migration.md.
pub mod operators;
pub mod derived_invariants;
pub mod exact;

// Kernel V9 RPPI admission interface (v9_specification.md §3–§10). Added
// alongside the frozen V8 modules; operators.rs and derived_invariants.rs are
// unchanged (Req-5). Bounded first implementation — supported scope is stated
// in admission.rs.
pub mod provenance;
#[cfg(feature = "v9")]
pub mod provenanced_exec;
#[cfg(feature = "v9")]
pub mod admission;

// Diagnostic execution of the ABR operators over experimental relational
// fields (Origin directive, 2026-11-07). Thin wrapper: calls the unchanged
// operators.rs functions; mandatory edge provenance; non-blocking warnings,
// blocking errors. Independent of admission.rs.
#[cfg(feature = "v9")]
pub mod diagnostic_exec;

// Kernel V10 ground variables (Origin, 2026-11-21): energy (tunneling),
// freedom (superposition) and connection (entanglement): three distinct
// variables read from one relational object over a declared sequence of
// registrations at declared δ, through the unchanged operators. A reading
// that cannot be calculated is a finding about the declaration (Primary
// Region, or domain not fully declared), never a statement of absence.
pub mod ground_variables;

// Kernel V10 Df6, simultaneity (Origin, 2026-11-21): all operators and all
// ground variables occur simultaneously; the composition order and the
// sequence of registrations are orders of analysis. A relational field is
// formed from at least two consecutive registrations and evaluates A, B, the
// passage Ξ, their persistence and the ground variables together, with no
// operator parameters (ρ, χ and cc[p] retired from V10). One registration is
// insufficient information. No phases.
pub mod relational_field;

pub mod persistence_change;

// Kernel V10.3 — topology checks on a declared field: T8 (self-relations
// refused; a directed turn admitted only as a declared feedback loop with a
// stated basis) and, with K7, route enumeration and a turn basis.
pub mod topology;

// Kernel V10.3 — RPPI-V10, the kernel's share of the reverse check (B2):
// from the declared inputs and a reported reading's identity, rebuild the
// fields and re-execute the form independently of the code that produced
// it, returning MATCH or MISMATCH (`docs/kernel/rppi_v10.md`).
pub mod reverse_check;

// Kernel V10.3 — §2.7, the declared ground (Origin, 2026-11-23, 57th): the
// domain declaration (R9a) mapping energy, freedom and connection in the
// system; the declared field formed from it; the ground record with all three
// variables for every relation, component and step.
pub mod domain;

// §2.9, the geometry of relations (Proposed, 69th): angle, step angle,
// relative phase, read exactly from the registered relations.
pub mod geometry;

// §2.10, the window and the present reading (Proposed, 73rd): held readings
// over the window as constraints on the next change, the freedom they leave
// (rank by minors, exactly), and the coherence of a candidate next step.
pub mod coherence;

// The work of forming the operator readings, counted exactly (Origin,
// 2026-11-25, 92nd): it grows with the relations and their neighbours.
pub mod work;

// The derivation of every part a relational field reports: an operator
// reading, or legacy mathematics with its provenance, its description in
// relation to the operators and its necessity (Origin, 2026-11-25, 92nd).
pub mod derivation;

// The order is the relational field (Origin, 2026-11-26, 102nd): the
// possibilities at the current point after every route arriving there.
pub mod order;
// The radial field: the relational field on complete radial graphs, read step
// by step (Origin, 2026-11-27, the compute rebuild).
pub mod radial;
