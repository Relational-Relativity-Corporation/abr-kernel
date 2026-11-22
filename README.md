# ABR Invariant Relational Kernel — Kernel V10

**Metatron Dynamics, Inc.** relationalrelativity.dev | arXiv:2601.22389

---

**The foundational observable primitive is a directed relation declared from
observable information.** A relation is observable when its two loci can be
distinguished and a direction between them can be established from what is
measured — not from what is assumed. Every quantity in D is a representation
of a relation, or a computation over relations. A numerical value is not
itself a primitive; it is the projection of an observed relational distinction
through a declared measurement mapping M.

**Canonical release: Kernel V10.** V10 is the governing documentation of the
observable relational primitive, operator constraints, provenance requirements,
admission discipline, and the three ground variables. Its canonical statement,
item by item, is `docs/kernel/V10_README.md`. Earlier version identifiers
describe historical implementations and records, not competing mathematical
authorities.

**Relation is the primitive, derived from observation.** Relational evolution
has not been observed to stop, and no registration stands alone; objects are
read from relations, not the reverse. Alongside it, three variables are present
in every observation. Their absence is never observed; what varies, and what the
kernel measures, is their degree at a declared resolution δ:

- **energy:** what is exchanged across a relation;
- **freedom:** change permitted, relations changing and changing independently;
- **connection:** shared structure, relations held together by a constraint,
  in their state or in their change.

**Structure is persistence; change is freedom** (Origin, 2026-11-21). Every
reading compares a relation's change between consecutive registrations with its
resolution: held below δ is structure, resolved is freedom. The kernel expresses
the three variables as tunneling, superposition and entanglement. These are names
defined by their forms over registered data; where they meet the same terms in
quantum mechanics, that is a cited intersection, not a basis (W14). The full
chain from observable to reported number is `docs/kernel/V10_README.md` §2.0.
*(The first wording, kept in V10_README §1.2: energy "how much a relation
changes", freedom "how many independent ways relations vary", connection "what
holds together through relational evolution".)*

**All operators and all variables occur simultaneously** (Df6). The composition
order and the sequence of observations are orders of analysis. A relational
field is formed from at least two consecutive registrations; one registration
is insufficient information.

This repository is the sole canonical declaration of the ABR/ABRCE kernel:
operator constraints, role separation protocol, reference implementation,
derived invariants, observational validation record, observable variable
sets, and cross-region energy expression.

The kernel is domain and scale invariant. The operators produce the same
classes of output — ε[e], τ[v], Φ[v], 𝟙[e], θ[e], J[v] — at every
declared region from Primary through Planetary. What changes across regions
is the declared measurement mapping M, not the operators or their constraints.

Declared observable relation → M → numerical projection in D → declared
topology → operators → calculated result → comparison with observables.
A value in D is the numerical projection of an observable through M,
not the observable itself. Where information an operator requires has not
been declared through M, that operator is not evaluated, and no substitute
value or classification is introduced.

All definitions bounded over D := { x ∈ ℝⁿ | n < ∞, |x[i]| < ∞ }.
No claim beyond D.

---

## Relational Primitive Provenance Invariant (RPPI)

Every admitted value, operation, and result must have a continuous,
verifiable provenance path to an Origin-declared observable relation within D.
Three things must be held distinct:

**Provenance label** — describes a category of relation (e.g.,
`EdgeProvenance::Continuation`). Names a class. Does not identify a declared
observable. Does not constitute a derivation path. A label describes; it does
not establish.

**Declaration record** — identifies what was admitted as an observable
relation: through which measurement mapping M, with which direction basis,
and into which declared domain. Set at construction time by Origin. This is
provenance identity. Without it, a relation carries a class but not an
identity; any value derived from it carries no declaration to check against.

**Calculation record** — records what the operators actually used and produced:
the input values, the relations traversed, the parameters applied (ρ_base,
χ₀, cc), and the outputs computed. Generated at computation time. The
Independent Relational Reconstruction System (IRRS) independently recomputes
or verifies the recorded operations and their bindings against declaration
records. Matching names and references alone is insufficient; the Generator's
account of execution is evidence to be checked, not a certification to be
accepted.

A result is admissible only when its derivation can be traced, through every
contributing mathematical input, to declared observable relations. Numerical
correctness, structural validity, and a provenance label are not substitutes
for a verifiable derivation path.

---

## Building and testing

Requires a Rust toolchain (`cargo`). No external dependencies.

```
git clone https://github.com/Relational-Relativity-Corporation/abr-kernel.git
cd abr-kernel
cargo test                                  # Kernel V10 only (V9 fenced)
cargo test --features v9                    # V10 and the V9 record
cargo run --example v10_relational_field    # V10 end-to-end template
cargo run --bin provenance_demo
```

**The V9 fence.** Retired and superseded V9 elements (the ρ-weighted
operators, the superseded readers, and the V9 execution paths `diagnostic_exec`,
`provenanced_exec` and `admission`) compile only with the `v9` feature, which is
off by default. V10 work cannot use them by accident: a call does not compile.
V9 work opts in visibly with `features = ["v9"]`. The code stays where it was,
with its history. See `docs/kernel/v10_migration.md`.

`cargo test` (V10 only) runs 121 tests:
- 35 library;
- 3 `provenance_demo`;
- 18 ground variables;
- 20 persistence and change (PC);
- 23 property tests (`tests/v10_properties.rs`, 2000 seeded random fields each);
- 5 relational field;
- 17 structural consistency (I-SC-D3 to I-SC-D13).

Building without the `v9` feature is itself a check that no V10 module depends on a fenced element.

`cargo test --features v9` runs all 213 tests:
- 63 library;
- 3 `provenance_demo`;
- 13 diagnostic execution (`tests/diagnostic_exec.rs`);
- 19 ground variables;
- 20 persistence and change;
- 23 property tests;
- 6 relational field;
- 17 structural consistency;
- 12 V9 admission (`tests/v9_admission.rs`);
- 35 V9 conformance (`tests/v9_conformance.rs`);
- 2 `compile_fail` doctests.

*(Counts at the V10.2 release, package `10.2.0`, 2026-11-22. At the V10.1
release they were 68 and 153.)* The four
`[V9-REQUIRED]` conformance tests exercise the bounded V9 admission interface
(see Kernel V9 — RPPI Enforcement Status below). The two V10 tests that compare
against V9 forms run only here, as declared historical comparisons.

`cargo run --bin provenance_demo` prints the H-alpha photon edge reconstruction
described below.

**Verification (2026-11-21).** A clean-room build of the release commit (fresh
clone, no cache) compiles with no warnings and passes all 153 tests. An
end-to-end run through the public API alone (a synthetic four-locus field with
two interior loci, two components and five registrations) formed the relational
field with no operator parameters, classified every minor as held or
reorganizing, read all three ground variables, and returned the expected
declaration findings for a single registration and for the Primary Region. The
first registration was checked by hand. These are synthetic fixtures: they
confirm that the forms compute as declared and establish no empirical validity.
The invariant tests against registered data (I-SC, OC-SC-1 to OC-SC-4) are next.

**Verification (2026-11-21 and 22), V10.2.** The Verifier's first pass found
three kernel findings, all accepted and dispositioned; the second-pass brief is
in `docs/kernel/VERIFICATION_2026-11-21.md`. The release was merged before that
pass by Origin's decision; its findings will be dispositioned on `main`. Every
count above was reproduced from a clean build. All tests are synthetic or random
fields: they establish that the forms compute as declared and that the derived
results hold over the forms, nothing empirical.
---

## Contents

**Kernel V10 is the canonical documentation release.** Historical V9, V8 and V7
identifiers remain only where needed to describe prior implementations,
release history, or explicitly frozen source. Document-local revision numbers
are historical identifiers, not alternate canonical kernel versions.
The Rust package `abr-kernel` (library crate `abr_kernel`) is version
`10.2.0`, the Kernel V10.2 release. Its operator formulas are unchanged from the
frozen V8 implementation (`8.0.0`). V10 adds the `ground_variables`,
`relational_field` and `persistence_change` modules and the parameter-free
circulation Ξ.

**Kernel only.** This repository holds kernel items. Work on specific systems
(the water vessel record, its ingest pipeline, the source-class register, the
items moved from the kernel's open conditions) is in
`abr-relational-chemistry`, which depends on this package. A kernel text names a
specific system only inside a marked *Example (application)* block.

### `docs/kernel/` — what the kernel is

**`v10_migration.md`** — redoing a repository in Kernel V10
The V10 admissibility rule (W13), the V9 fence, a V9 → V10 mapping of every
fenced element, the steps for one repository, and what not to carry over.

**`V10_README.md`** — the canonical statement of Kernel V10
Every V10 item by ID, from the relational declaration of variables and
conditions to the formal mathematics, traceability, code scope, conflicts and
open decisions. Includes the provenance chain (§2.0), Df6 (simultaneity), the
three ground variables, structure and freedom as persistence and change (§2.5),
the derived structural consistency results (§2.6), the reading rules W1–W15, and
the amendment log (Part 9).

**`OPEN_CONDITIONS_AUDIT_2026-11-22.md`** — every open condition classed
(closed, moved to the application, open for registered data, decided, or
Generator work), with Origin's decisions of 2026-11-22.

**`w14_audit.md`** — the audit of quantum-mechanical constructs against W14
(intersection, not basis), with each item's disposition.

**`VERIFICATION_2026-11-21.md`** — the verification package: reproduction,
every item with its status and what to check, the Verifier's findings and their
dispositions, and the second-pass brief.

**`kernel_reference.md`** — start here
A single reference for people and AI systems using the kernel: roles and
workflow, the foundation (O → M → D, declared information, declaration
correspondence), every operator with its form and Rust function, the derived
invariants, the language discipline, the verification requirements, the
open conditions held since Kernel V8, and the status of RPPI enforcement.
It restates the files below and adds nothing of its own; where it and a
source file differ, the source file governs.

**`operators_notation_and_constraint.md`**
Formal operator definitions and constraints. The notation says what the
operators are. The constraints say what may not be added.

**`role_separation_and_operator_application.md`**
Role separation protocol and application workflow. Declares the minimum
role structure required for the kernel to function correctly, and the
four-stage Verification and Validation Protocol (Stages 1–4, Verifier
Criterion).

**`abr_operators_plain.md`**
Plain language statement of each operator constraint. No mathematics
required. The reader remains Origin.

**`validation_record.md`**
Observational grounding for every confirmed quantity in
`derived_invariants.rs`. States what was observed, from which source,
through which measurement mapping M, and with what support classification.

**`observable_variable_sets.md`**
Declares the primitive variable set at each region (Primary through
Planetary), the Layer 3 derived outputs, architectural principles, and
empirical findings established from the observable record.

**`cross_region_energy_expression.md`** (record)
Derives E[v] = ι[v] · κ[region] from the isolated-locus limit of the
unified τ expression. *Amended 2026-11-21:* τ = ħ/ε, Φ_relational and κ[region]
are retired from V10 (κ[Primary] = c² is the unit conversion returning itself);
Φ_stability is a declared intersection, not a kernel reading. Kept as the
record.

**`triad-constraint-methods.md`**
The seven verification methods (CP, CS, DI, CA, IR, AG, CR), the two
Origin obligations, and the publication gate.

### `src/` and `bin/` — reference implementation

**`src/ground_variables.rs`** (Kernel V10)
Energy, freedom and connection read over a declared sequence at declared δ,
through the unchanged operators. `GroundReading::freedom` is the PC reading
(change permitted). The first V2 form (a minor held resolved: a persistent
arrangement, structure) is kept as `v2_held_arrangement` for declared
comparison. Connection is graded identity, proportional, sustained and coupled
change, read side by side. A reading that cannot be calculated is a finding
about the declaration (Primary Region, or domain not fully declared), never a
statement of absence. See `kernel_reference.md` §21.

**`src/persistence_change.rs`** (Kernel V10.2)
Structure and freedom as persistence and change, at three levels: the relation
(PC1), the coupling of change between relations (PC2), and the arrangement
(PC3), with shares (PC4). Works on node-derived or edge-registered series (ER-1).
Also:
- the step profile;
- relation transitions (freedom, or persistent structure, becomes evident);
- the counted rate of relational evolution (V5);
- cycle closure in state and on change (K7 / HB-1).

The derived properties PC-D1 to PC-D5 and I-SC-D1 to I-SC-D13 are tested by
worked examples and by property tests over 2000 random fields
(`tests/v10_properties.rs`).

**`examples/v10_relational_field.rs`** (Kernel V10)
End-to-end template using only the public V10 API: declare relations, a
sequence of registrations with δ and an energy carrier; form the relational
field; read the three ground variables and their findings.

**`src/relational_field.rs`** (Kernel V10, Df6)
`RelationalField::new` forms a relational field from at least two consecutive
registrations and evaluates A, B, the circulation Ξ, their persistence and the
ground variables together, in one call, with **no operator parameters**. ρ and
χ (ρ_base, χ₀, the max selection, the saturating form) and cc[p] are retired
from V10; the ρ-weighted V9 functions in `operators.rs` remain for V9 callers.
`circulation_weights` gives the whole-field circulation identity (I-SC-D11).

**`src/operators.rs`**
Reference implementation in Rust. Primary kernel (Δ → Σ) and ABR kernel
(A → B → R). Declared relations only. No ring, no torus (relational evolution
never wraps onto itself), no undeclared topology. Operator parameters (ρ_base,
χ₀, cc) are identified by M for each evaluation; Origin general values are
ρ_base = 0.5 and χ₀ = 1 unit of χ, never silently substituted. *(ρ, χ and cc
are retired from V10; these apply to V9 callers only, behind the V9 fence.)*

**`src/diagnostic_exec.rs`** (V9, behind the fence)
Thin execution wrapper for experimental relational fields (Origin directive,
2026-11-07). Runs A → B → R and persistence through the unchanged operators,
independent of the admission interface. Edge provenance is mandatory and keeps
observed and proposed relations distinct; experimental uncertainty is returned as
warnings with the results; only conditions that prevent numerical evaluation are
errors. Persistence pairs only consecutive declared process steps.
See `kernel_reference.md` §11.7.

**`src/derived_invariants.rs`**
Canonical implementation of Layer 3 derived quantities: ε[e], τ[v],
Φ[v], 𝟙[e], J[v], N[v], and photon edge functions. Every function has a
declared observable source and support classification. *Since 2026-11-21:* the
τ = ħ/ε family and `HBAR` are retired from V10 (behind the V9 fence); Φ is a
declared intersection with a time projection (W15); hc/λ's output is Projected,
and V10 photon-edge readings use the registered wavenumber (W14); Part II holds
the I-SC derivations.

**`bin/provenance_demo.rs`**
Minimal worked example of the provenance requirement, built against one
already-documented kernel quantity (the H-alpha photon edge, NIST ASD
v5.12). Given a record, the result can be reconstructed from its recorded
input and identified calculation; the record also carries the declared
source attribution for that input. Verifying the attribution itself
requires comparison with the named source.

### `docs/process/` — how the kernel is checked

These documents, added 2026-07-30, declare how the content above is
checked. They do not alter the mathematics.

**`docs/process/observable_provenance_and_reverse_traceability.md`**
No mathematical construct enters the kernel record as established by
virtue of internal consistency alone; a chain must terminate in a
declared observable, and that chain must be reconstructible in both
directions — executable, not merely documented.

**`docs/process/language_discipline_rust_mandate.md`**
Declares Rust as the only acceptable language for science- and
math-bearing processing in this kernel, and states why.

**`docs/process/kernel_self_consistency_test.md`**
Two-part test: a clean-room build test (does the repository run from
nothing but what's published) and a cross-document consistency audit
(six required interface checks between notation, code, validation record,
variable sets, language discipline, and cross-region invariance).

**`docs/process/Verification_pass_protocol.md`**
Governs how Origin orders an independent verification pass over a
declared artifact — scope, evidence, reading order, and output format.

**`docs/process/repository_protocol_v1.md`**
The required workflow for creating, verifying, and publishing any
Metatron Dynamics repository. It applies the four stages from
`role_separation_and_operator_application.md` and the seven methods from
`triad-constraint-methods.md` at repository level.

**How the process documents relate.** Four of them are distinct
layers, each an application of the one beneath it:

```
Observable Provenance and Reverse Traceability   (the fixed principle)
        ↓
Language Discipline — Rust Mandate               (one implementation
                                                    mechanism for it)
        ↓
Kernel Self-Consistency Test                     (how to check the
                                                    principle and the
                                                    mandate are both
                                                    actually honored)
        ↓
Verification Pass Ordering Protocol              (how any single
                                                    check — including
                                                    a self-consistency
                                                    pass — is organized
                                                    and run)
```

The Repository Protocol applies all four at repository level, from
scaffold through Verifier pass and Origin execution to publication.

What changes across these layers is the *application* (implementation
language, audit procedure, execution ordering); what does not change, at
any layer, is the requirement that mathematical claims terminate in a
declared observable.

**In words** (restates the diagram above; if the two ever disagree,
the diagram is the reference and this paragraph is stale):
Observable Provenance and Reverse Traceability states the foundational
requirement — mathematical claims require observable termination and
reconstructible provenance. Language Discipline — Rust Mandate states
one implementation mechanism intended to support that requirement. The
Kernel Self-Consistency Test states how to check that both the
requirement and the implementation discipline are actually being
honored in a given repository. The Verification Pass Ordering Protocol
states how any individual verification assignment is organized and
reported, regardless of what is being verified.

---

## Contribution discipline

**Development branch.** `main` is the sole internal development branch of the
canonical kernel. No change is made to `main` without Verifier review and Origin
authorization (`docs/process/repository_protocol_v1.md`).

This is a living document. Contributions are welcome provided they
satisfy the declared admissibility condition:

**Every quantity in a contribution must be traceable to an observable
through a declared measurement mapping M.**

Concretely, a contribution must state:

1. **The observable** — what instrument reports it, at what region,
   under what declared conditions.
2. **The measurement mapping M** — the declared formula or procedure
   that maps the instrument output into D.
3. **The operator path** — which operator or combination of operators
   produces the quantity from the declared input through M.
4. **The support classification** — Observed, Derived, or Inferred,
   per the protocol in `validation_record.md`.
5. **The open conditions** — what remains unresolved, stated explicitly.

Declaration is not a substitute for traceability to an observable
through M. A declared quantity lacking observable traceability does
not enter the kernel as an established quantity, because it is not
about anything in D.

Contributions that import quantities from legacy theoretical frameworks
without independent declaration through M are not admissible. The
presence of a quantity in an established framework is noted for
orientation only — it is not itself an admissibility argument.

Pull requests should include updates to `validation_record.md` and
`observable_variable_sets.md` for any new confirmed quantities.

---

## Scope

The kernel is bounded over D. It makes no claim beyond D.

The operators are domain and scale invariant over every declared region
confirmed in `observable_variable_sets.md`. Application to a new region
requires declaring the measurement mapping M at that region — what the
instruments report, what constitutes a locus, what constitutes a declared
relation, and what establishes the direction of relational evolution.

The `derived_invariants.rs` functions are the shared computational
foundation. Application repositories draw from this file rather than
reimplementing locally, so that updates propagate from a single
declared source.

---

## Kernel V10 — Status at V10.2 (2026-11-22)

**Declared and implemented:**
- Df6 (simultaneity) and the three ground variables;
- the provenance chain (§2.0);
- structure and freedom as persistence and change (PC1–PC4), with the coupled-change grade of connection;
- edge-valued registration (ER-1);
- W13 (the V9 fence), W14 (intersection, not basis), W15 (time is never primitive);
- the τ = ħ/ε retirement;
- relation transitions.

**Derived:** I-SC-D1 to I-SC-D13, PC-D1 to PC-D5 (PC-D2 and I-SC-D6 corrected
after testing and review), with worked examples and property tests.

**Declared, not yet implemented (next, 10.3.0):**
- Z1a (δ in two parts: repeatability and run-constant);
- the A-3 boundary flag;
- the renames (E6; Κ = connection density; σ → N_δ);
- the audit's Generator work (OC-SC-4, RPPI-V10 admission, the T8 acyclicity check, K7 enumeration).

**Open for registered data:** OC-SC-1 to OC-SC-3, OC-T-1, OC-T-2, OC-RE-1, and
relational inertia (P-a to P-f). No registered record has yet been read.

Item-by-item status: `docs/kernel/V10_README.md` Parts 6, 7 and 9, and
`docs/kernel/OPEN_CONDITIONS_AUDIT_2026-11-22.md`. RPPI enforcement status below
is unchanged from V9. RPPI admission for the V10 forms is Generator work.

## Kernel V9 — RPPI Enforcement Status

Kernel V9 established the relational primitive and its provenance
requirements, carried unchanged into V10. Enforcement of the Relational Primitive Provenance
Invariant remains under development; canonical status does not imply that
all V9 admission interfaces are implemented. The RPPI and its enforcement
architecture are specified in `docs/kernel/v9_specification.md`.

**Completed milestones:**

- **Milestone 1** — Executable relation definition; conformance suite in
  `tests/v9_conformance.rs` with 31 active tests and 4 `#[ignore]` tests
  pending V9 enforcement milestones. Active tests establish V8 baseline
  behavior and document the provenance survival gap.
- **Milestone 2** — RPPI established. Four failure modes documented:
  missing provenance (no identity), fabricated provenance (unverifiable
  against declared M), disconnected provenance (no derivation path through
  operator output), misattributed provenance (wrong relation attributed).
  Verifier accepted 2026-10-24.
- **Milestone 3** — Origin declaration of seven relational identity and
  derivation requirements (Req-1 through Req-7). Declaration record /
  calculation record distinction established. Two Verifier corrections
  incorporated: enforcement belongs to the V9 admission interface rather than
  frozen V8 constructors (Req-5); IRRS must independently recompute or verify
  recorded operations against declaration records — self-certification excluded.
  Verifier accepted 2026-10-24.

**OPEN — V9 enforcement not yet implemented:**

- `ObservableRelationDeclaration`: declaration record for each edge —
  observable source, direction basis, M mapping reference, declared domain.
  Returned to Origin for field definition.
- NodeField declaration record: M-mapping origin of each projected value.
- `ProvenancedOutput<T>`: calculation record wrapper carrying the identities
  of all contributing values, relations, mappings, and declared parameters.
- V9 admission interface: separate from V8 constructors; independently
  recomputes or verifies calculation records against declaration records.

V8 constructors and operators are unchanged (Req-5). All `[V8-BASELINE]` tests
pass without modification to operator code.

**Bounded first implementation (subject to Verifier review).**
`src/provenance.rs` (declaration and calculation records), `src/provenanced_exec.rs`
(Generator execution through the frozen functions), `src/admission.rs` (independent
reconstruction and admission). The four RPPI failure modes are enforced within the
supported scope stated in `kernel_reference.md` §18: absent or unverifiable
observational basis → OPEN; established contradiction → REJECTED. ABR, persistence,
other Layer 3 quantities, other M kinds, and independent IRRS verification from a
distinct frame remain OPEN.

---

## Version and historical implementation record

**Current canonical documentation: Kernel V10.** The historical notes below
preserve the provenance of earlier implementations; they do not designate
V9 or V8 as the current canonical mathematical reference. The frozen V8
operator implementation is an implementation baseline within the V9 and V10
development record, not a second source of mathematical authority.

### Kernel V10.2 package release (2026-11-22)

- Package `abr-kernel` `10.1.0` → `10.2.0` (Origin decision, V10_README Part 9, 20th and 21st).
- **Declared and implemented:**
  - structure is persistence, change is freedom (PC1–PC4), with V2 restated and the first V2 form kept for comparison;
  - the provenance chain (§2.0);
  - edge-valued registration (ER-1);
  - the coupled-change grade;
  - energy as a component.
- **Reading rules:** W14 (intersection, not basis); W15 (time is never primitive); W9 and W10 declared.
- **Retired from V10, behind the V9 fence:** the τ = ħ/ε family (`HBAR`, `tau_photon_edge`, `n_relational_cycles`, `tau_v`, `tau_v_isolated`, `tunneling_tau_bounded`). Φ stays as a declared intersection.
- **New public API:**
  - `persistence_change` (`EdgeSeries`, `PersistenceChange`, `relation_changes`, `coupling_across_components`, `coupling_across_steps`, `arrangement_steps`, `coupled_change_pairs`, `step_profile`, `relation_transitions`, `evolution_counts`, `cycle_closure`, `cycle_change_closure`);
  - `relational_field::circulation_weights`;
  - `GroundReading::freedom` now the PC reading, with `v2_held_arrangement`;
  - `ConnectionGrade::CoupledChange`.
- **Derived:** I-SC-D3 to I-SC-D13; property tests over 2000 random fields each.
- **Audits:** the W14 audit (quantum constructs; §13.1 constants classed); the open-conditions audit (all items decided or dispositioned).
- **Tests:** 121 (default), 213 (`--features v9`). Three existing tests changed their expected findings under PC. Each change is stated in the test and in the verification package.
- **Merged to `main` with every commit kept,** before the Verifier's second pass, by Origin's decision.
- **Not yet implemented:** Z1a, the A-3 flag, the renames (10.3.0).

### Kernel V10.1 package release: the V9 fence (2026-11-21)

- Package `abr-kernel` `10.0.0` → `10.1.0`. Cargo feature `v9`, off by
  default, now gates the retired and superseded V9 elements. **Breaking for
  V9 consumers:** a repository that calls a fenced element must add
  `features = ["v9"]` or be redone in V10 (`docs/kernel/v10_migration.md`).
- V10 admissibility rule declared (W13): V10 work uses only V10 forms; a
  retired element enters a V10 result only as a declared historical
  comparison.
- `examples/v10_relational_field.rs` added (end-to-end template).
- No code removed and no V9 formula changed; all 153 tests pass with
  `--features v9`, and 68 pass without it.

### Kernel V10 package release (2026-11-21)

- Package `abr-kernel` version `9.0.0` → `10.0.0` (Origin decision). Package
  name and library crate name `abr_kernel` unchanged.
- No operator formula, V9 function signature, return type, parameter, or
  existing test expectation changed. New public modules: `ground_variables`,
  `relational_field`.
- Df6 (simultaneity) declared; the Phase 1 / Phase 2 rule amended in
  `kernel_reference.md`, `operators_notation_and_constraint.md`,
  `role_separation_and_operator_application.md` and `abr_operators_plain.md`,
  with the V9 text retained as record.
- Release designation synchronized across code comments and documents.
- ρ and χ (with ρ_base, χ₀, the max selection, the saturating form) and
  cc[p] retired from V10 readings; parameter-free circulation Ξ added
  (`operator_circulation`). OC-K-1 dissolved.
- Tests: 153 pass (see Building and testing).

### Kernel V9 package release (2026-10-25)

- Package `abr-kernel` version `8.0.0` → `9.0.0` (Origin decision). Package
  name and library crate name `abr_kernel` unchanged.
- No operator formula, function signature, return type, parameter, or test
  expectation changed. The major version marks the canonical Kernel V9
  release; it does not indicate an API break.
- Consumers that depend on `branch = "main"` without a version
  requirement build unchanged.
- Tests: 61 library tests, 3 `provenance_demo` tests, 31 active V9
  conformance tests pass; 4 `[V9-REQUIRED]` tests remain `#[ignore]`
  (RPPI enforcement not implemented; see `kernel_reference.md` §18).

### Historical Kernel V8 implementation (2026-09-26)

Kernel V8 is one release across every file: code, kernel documents, process
documents, and this README. The package is `abr-kernel` (library crate `abr_kernel`), package
version 8.0.0. Renamed from `metatron_kernel_v8` on 2026-10-01; no
operator formula changed.

**Declarations carried into every file:**
- Observable → M → numerical projection in D → operators → calculated
  result → comparison with observables. No numerical result is itself an
  observable.
- Declared information: where information an operator requires has not
  been declared through M for the calculation, the operator is not
  evaluated. No substitute value or classification is introduced. A 0.0
  produced over declared information is a calculated result.
- Declaration correspondence: where a declared relation's numerical
  projection does not preserve the observed distinction it is based on,
  the declaration is varied and evaluated again — topology first, with
  variables and M. The kernel does not vary the declaration.
- Persistence requires a declared pair of sequential observations. At the
  first declared observation only the spatial kernel is evaluated.

**Code changes:**
- `PersistenceState::cold_start`, `is_cold_start`, and `EdgeField::zero`
  removed. `PersistenceState` is constructed only from the kernel output of
  a completed declared observation. No operator formula changed.
- Crate renamed from `metatron_kernel_v7` to `metatron_kernel_v8`.
  Repositories that depend on `metatron_kernel_v7` continue to build against
  V7; each moves to V8 when it is rebuilt and reviewed.
- Tests: 61 library tests and 3 `provenance_demo` tests.

**Declarations of 2026-09-23 now written into the documents.** The V7
release notes below describe document revisions from 2026-09-23 (listed as
operators_notation V13, role_separation V8.2, abr_operators_plain V8.4,
validation_record V8, observable_variable_sets V7.4,
cross_region_energy_expression V4.2). Those revisions were implemented in
`derived_invariants.rs` V4.3 and `operators.rs` V7.1 but were not written
into the documents at the time. Kernel V8 writes them in: I-S as relational
distinguishability of the Δ output; the full-operator admissibility
condition replacing ρ_P; rank(Im Δ), rank(Im Σ), and ρ_P retired; OC-S-1
closed; scale-conditional observational status; downstream projections
labeled. Each document's history entry lists which passages were
reconstructed.

**New document.** `docs/kernel/kernel_reference.md` — a single reference to
the kernel for people and AI systems, restating the declarations in the other
files.

**Correction carried forward.** The correlation statistic (0.726) in
`validation_record.md` is removed, per a 2026-07-28 Verifier finding that
was not previously published.

**Open and held at Kernel V8** (recorded in the files where they arise):
- A-3 — empty adjacency in Σ and R: whether sums over an empty declared
  set are defined over the set of declared adjacent relations. Not
  classified.
- 𝟙[e] interpretation audit, and ε[e] downstream of it.
- ~~A-11 — ι[photon] = 0~~ **CLOSED (2026-09-27):** ι is not declared
  through M at the photon locus. The legacy "photon rest mass = 0" imports
  a definition requiring a frame with no declared relational motion —
  inadmissible in D (SF-PR-16). Under the declared-information rule, ι is
  not evaluated at photon edges. ε[e] = hc/λ stands on its own M mapping.
- `are_proportional` in `derived_invariants.rs`: result depends on edge
  order when a Δ vector is 0.0 in every component; tolerance used in two
  units.
- Part II "absence" wording audit (I-E decoherence, joint necessity,
  photon J[v] interpretation).
- R² = 0.527 at the δι fit in `observable_variable_sets.md` (same class of
  statistic as the removed correlation).
- J[v] wording on what M observes.
- OPEN conditions on ρ: the max selection rule and the saturating form.

---

**Update (2026-09-27):**

- A-11 CLOSED across all files: ι at the photon locus was an import of
  the legacy quantity "photon rest mass = 0," whose definition requires a
  frame with no declared relational motion — inadmissible in D (SF-PR-16).
  Under the declared-information rule, ι is not evaluated at photon edges.
  ε[e] = hc/λ stands on its own M mapping.
- Test `photon_edge_kappa_non_applicable` renamed to
  `photon_edge_iota_not_declared` and rewritten structurally — no
  manufactured 0.0.
- "Rest mass" language purged from all documents; replaced with "m[v] as
  primitive inadmissible (SF-PR-16)" and the structural reason: its
  definition requires a frame with no declared relational motion, which
  does not exist in D.
- I-RE passages in `abr_operators_plain.md` and
  `operators_notation_and_constraint.md` corrected: the 78-decade Φ span
  is the observable expression of the τ_stability ordering; τ_relational
  is a distinct quantity whose ordering is not established by the
  stability data alone. I-RE remains open derivation (OC-RE-1).
- `validation_record.md`: "not directly observable" replaced with
  scale-conditional language (expression-observed, mechanism-unresolved).
- 61 library tests + 3 provenance_demo tests: all pass.

---

### V7 release notes (historical)

The notes below are the V7 release notes as published before Kernel V8.
Their document version numbers are document-local, and the 2026-09-23
entries describe revisions that were written into the documents only in
Kernel V8 (see above). Test counts are as recorded at the time.

**V7 (mathematical content). Process and provenance discipline added
2026-07-30 — content unchanged.**

The mathematics — operators, derived invariants, validation record,
observable variable sets — remains V7 as declared below. Nothing added
on 2026-07-30 alters `operators.rs`, `derived_invariants.rs`, or any
VR- or OC- entry. What was added is a governance and verification
layer around the existing content: `docs/process/`. See that
directory's contents for what changed and why. The mathematical
content is not claimed complete by this addition — only that this
addition does not change it.

V7. Updates from V4:

- derived_invariants.rs V4.3 (current):
  I-S redeclared — relational distinguishability declared as the observable
  condition the superposition invariant expresses: the Δ output field carries
  relational distinguishability when at least two declared edges produce
  directed differences that are not proportional. Directly observable from
  operator output; no matrix construction, no SVD, no threshold.
  superposition_condition() reinstated: pairwise proportionality check,
  O(n²·k). superposition_resolved() reinstated: structural interior locus
  criterion (full-operator admissibility condition), not numerical threshold.
  rank(Im Δ), rank(Im Σ), ρ_P not reinstated — unnecessary representation
  layers over a condition directly readable from the output. OC-S-1 closed.
  I-T, I-E, constants block: legacy physics terms replaced with
  framework-native declarations; legacy correspondence stated as what M
  observes, not as operator primitives. zeeman_components() renamed
  r_output_states() — R output state count is the framework-native quantity;
  Zeeman components named as what M observes at locus resolution.
  Joint necessity preamble: scale-conditional ℛ_M/𝓜_M framing throughout.
  55 tests pass.

- derived_invariants.rs V4.2 (Verifier PASS — superseded by V4.3):
  Photon edge direction revised — each photon edge declared from its own
  independent observation through M. I-S suspended (rank(Im Δ) > 1 and
  ρ_P not authorized); observable record preserved; superposition_condition()
  and superposition_resolved() commented out; OC-S-1/2/3 reopened.
  R_anti → 𝟙[e] → I-T/I-E chain audited — chain intact.
  Historical V3→V4 I-S entry marked HISTORICAL — SUSPENDED BY V4.2.

- derived_invariants.rs updated V3 → V4: four new invariants added —
  I-T (Tunneling), I-S (Superposition), I-E (Entanglement), I-RE
  (Relational Evolution); joint necessity preamble; 9 new open conditions
- validation_record.md V8 (current): support classification protocol
  extended with scale-conditional Observed subcategories (locus-resolved;
  expression-observed, mechanism-unresolved) and downstream projection label.
  VR-T-01 redeclared in framework-native terms. VR-S-01 redeclared with
  relational distinguishability condition; OC-S-1 closed. VR-E-01 updated
  with scale-conditional observational status. VR-RE-01 updated.
  δι log-log fit labeled as downstream projection.
- validation_record.md V5 → V7: VR-T-01, VR-S-01, VR-E-01, VR-RE-01
  added (V5); synchronization passes (V6, V7) — scale-conditional
  classifications, legacy physics removed, companion refs updated
- operators_notation_and_constraint.md: Primary Region
  invariants section extended with I-T, I-S, I-E, I-RE cross-references;
  joint necessity observation stated
- abr_operators_plain.md: new plain language section on
  Primary Region phenomena — tunneling, superposition, entanglement,
  relational evolution; epistemic status explicit throughout
- cross_region_energy_expression.md V4.2: companion file references
  updated to current versions. Mathematical content unchanged from V4.
  V4: I-RE cross-reference added; κ[region] monotonic decrease stated
  as proposed (pending derivation OC-RE-1); confirmation language
  discipline applied

**Kernel purge and synchronization pass (2026-09-23):**

- operators_notation_and_constraint.md V13: observational resolution
  principle declared (ℛ_M, 𝓜_M — scale-conditional observational status);
  full-operator admissibility condition declared (replaces ρ_P — A→B→R
  admissible when M establishes interior locus t with independent incoming
  and continuing edges); rank(Im Δ), rank(Im Σ), ρ_P resolved and retired
  (unnecessary representation layers — relational distinguishability is the
  directly readable condition); relational distinguishability named as
  condition on Im Δ; I-S redeclared; I-T/I-E scale-conditional status;
  classical/quantum moved to labeled interpretive note — canonical derivation
  stands without those categories; OC-S-1 closed; OC-CQ-1 opened.
- role_separation_and_operator_application.md V8.2: ρ_P → 1 kernel
  boundary and n_components/propagation-capacity ceiling retired; replaced
  with full-operator admissibility condition (structural, not numerical).
- abr_operators_plain.md V8.4: I-S plain language updated; Three Primary
  Region quantities section replaced with full-operator admissibility;
  scale-conditional observational language throughout.
- observable_variable_sets.md V7.4: downstream projections labeled.
- Cargo.toml: nalgebra dependency removed (SVD/matrix machinery purged
  from operators.rs). Cargo.lock regenerated — root package only, no
  external dependencies. 55 tests pass.

**V7 changes (carried forward):**
- Sequential observation requirement added (operators_notation_and_constraint.md)
- Relational evolution direction as distinct named constraint added
- Primary kernel E_primary = Σ(Δ(x)) declared alongside ABR kernel
- ι[v] replaces m[v] throughout — m[v] as primitive declared inadmissible (SF-PR-16)
- derived_invariants.rs V3 — photon edge functions, N[v], five new tests
- validation_record.md V4 — VR-γ-01, VR-γ-τ-01, VR-γ-N-01
- observable_variable_sets.md V7.4: downstream projections labeled
  (δι log-log fit, EF-3); companion references updated; cos(θ) circularity
  in near-perpendicular τ case labeled as internal consistency.
  V7: EF-14 through EF-17 (photon edges)
- cross_region_energy_expression.md V3 — κ[Primary] = c² stated
  explicitly; photon edge section; OC-E-5 elevated
- Verifier language discipline: "relational field cycles" inadmissible;
  replaced with "declared relational intervals over the measured
  stability interval"

---

*Metatron Dynamics, Inc. — Delaware C-Corp #10551645*
*Bounded over D. No claim beyond D.*
