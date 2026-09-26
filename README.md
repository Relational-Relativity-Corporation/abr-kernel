# ABR Invariant Relational Kernel — Kernel V8

**Metatron Dynamics, Inc.** relationalrelativity.dev | arXiv:2601.22389

---

This repository is the canonical declaration of the ABR/ABRCE kernel:
operator constraints, role separation protocol, reference implementation,
derived invariants, observational validation record, observable variable
sets, and cross-region energy expression.

The kernel is domain and scale invariant. The operators produce the same
classes of output — ε[e], τ[v], Φ[v], 𝟙[e], θ[e], J[v] — at every
declared region from Primary through Planetary. What changes across regions
is the declared measurement mapping M, not the operators or their constraints.

Observable → M → numerical projection in D → declared topology → operators
→ calculated result → comparison with observables. A value in D is the
numerical projection of an observable through M, not the observable itself.
Where information an operator requires has not been declared through M, that
operator is not evaluated, and no substitute value or classification is
introduced.

All definitions bounded over D := { x ∈ ℝⁿ | n < ∞, |x[i]| < ∞ }.
No claim beyond D.

---

## Building and testing

Requires a Rust toolchain (`cargo`). No external dependencies.

```
git clone https://github.com/Relational-Relativity-Corporation/abr-kernel.git
cd abr-kernel
cargo test
cargo run --bin provenance_demo
```

`cargo test` runs 61 library tests (`metatron_kernel_v8`) and 3
`provenance_demo` tests. `cargo run --bin provenance_demo` prints the
H-alpha photon edge reconstruction described below.

---

## Contents

Every file in this repository carries the kernel release number,
**Kernel V8**. Each document's own earlier revision numbers are kept in its
history section, labeled as document-local.

### `docs/kernel/` — what the kernel is

**`kernel_reference.md`** — start here
A single reference for people and AI systems using the kernel: roles and
workflow, the foundation (O → M → D, declared information, declaration
correspondence), every operator with its form and Rust function, the derived
invariants, the language discipline, the verification requirements, and the
items open at Kernel V8. It restates the files below and adds nothing of its
own; where it and a source file differ, the source file governs.

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

**`cross_region_energy_expression.md`**
Derives E[v] = ι[v] · κ[region] from the isolated-locus limit of the
unified τ expression. Distinguishes Φ_stability from Φ_relational.
States κ[Primary] = c² = 8.988×10¹⁶ J/kg as internal consistency
within M. States open conditions OC-E-1 through OC-E-5.

**`triad-constraint-methods.md`**
The seven verification methods (CP, CS, DI, CA, IR, AG, CR), the two
Origin obligations, and the publication gate.

### `src/` and `bin/` — reference implementation

**`src/operators.rs`**
Reference implementation in Rust. Primary kernel (Δ → Σ) and ABR kernel
(A → B → R). Declared relations only. No ring, no torus, no undeclared
topology. Operator parameters (ρ_base, χ₀, cc) are supplied by M with no
kernel defaults.

**`src/derived_invariants.rs`**
Canonical implementation of Layer 3 derived quantities: ε[e], τ[v],
Φ[v], 𝟙[e], J[v], N[v], and photon edge functions. These are the
quantities the operators produce; their forms are invariant across
all declared regions. Every function has a declared observable source
and support classification.

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

## Version

### Kernel V8 (2026-09-26)

Kernel V8 is one release across every file: code, kernel documents, process
documents, and this README. The crate is `metatron_kernel_v8`, package
version 8.0.0.

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
- A-11 — ι[photon] = 0: measured as 0.0, or not declared through M.
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
  V4: I-RE cross-reference added; κ[region] monotonic decrease stated;
  confirmation language discipline applied

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
- ι[v] replaces m[v] throughout — rest mass declared inadmissible (SF-PR-16)
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
