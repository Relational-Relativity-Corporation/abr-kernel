# ABR Invariant Relational Kernel — V7

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

All definitions bounded over D := { x ∈ ℝⁿ | n < ∞, |x[i]| < ∞ }.
No claim beyond D.

---

## Contents

**`operators_notation_and_constraint.md`**
Formal operator definitions and constraints. The notation says what the
operators are. The constraints say what may not be added. V13.

**`role_separation_and_operator_application.md`**
Role separation protocol and application workflow. Declares the minimum
role structure required for the kernel to function correctly. V8.2.

**`abr_operators_plain.md`**
Plain language statement of each operator constraint. No mathematics
required. The reader remains Origin. V8.4.

**`operators.rs`**
Reference implementation in Rust. Declared relations only. No ring,
no torus, no undeclared topology. V7.

**`derived_invariants.rs`**
Canonical implementation of Layer 3 derived quantities: ε[e], τ[v],
Φ[v], 𝟙[e], J[v], N[v], and photon edge functions. These are the
quantities the operators produce; their forms are invariant across
all declared regions. Every function has a declared observable source
and support classification. V4.3.

**`validation_record.md`**
Observational grounding for every confirmed quantity in
`derived_invariants.rs`. States what was observed, from which source,
through which measurement mapping M, and with what support classification.
V8.

**`observable_variable_sets.md`**
Declares the primitive variable set at each region (Primary through
Planetary), the Layer 3 derived outputs, architectural principles, and
empirical findings established from the observable record. V7.4.

**`cross_region_energy_expression.md`**
Derives E[v] = ι[v] · κ[region] from the isolated-locus limit of the
unified τ expression. Distinguishes Φ_stability from Φ_relational.
Confirms κ[Primary] = c² = 8.988×10¹⁶ J/kg as internal consistency
within M. States open conditions OC-E-1 through OC-E-5. V4.2.

The seven documents above (`docs/kernel/`) declare what the kernel
*is* — the mathematics, operators, and observable record. The
documents below (`docs/process/`) declare how that content is
*checked* — a distinct concern, added 2026-07-30, that does not alter
any of the mathematics above.

**`docs/process/Verification_pass_protocol.md`**
Governs how Origin orders an independent verification pass over a
declared artifact — scope, evidence, reading order, and output format.
Does not redefine roles; governs preparation of a verification
assignment within the workflow `role_separation_and_operator_
application.md` already declares.

**`docs/process/observable_provenance_and_reverse_traceability.md`**
States the kernel's foundational epistemic requirement: no
mathematical construct is a source of truth by virtue of internal
consistency alone; a chain must terminate in a declared observable,
and that chain must be reconstructible in both directions —
executable, not merely documented.

**`docs/process/kernel_self_consistency_test.md`**
Two-part test: a clean-room build test (does the repository run from
nothing but what's published) and a cross-document consistency audit
(do the notation, code, and validation record actually agree with each
other, checked interface by interface rather than document by
document).

**`docs/process/language_discipline_rust_mandate.md`**
Declares Rust as the only acceptable language for science- and
math-bearing processing in this kernel, and states why — the language
mechanism through which the provenance discipline above is actually
enforceable rather than aspirational.

**`bin/provenance_demo.rs`**
Minimal, real worked example of the provenance requirement, built
against one already-documented kernel quantity (the H-alpha photon
edge, NIST ASD v5.12) rather than an invented case. Demonstrates a
value and its provenance as a single reconstructible object, checked
in both directions.

**How the four process documents relate.** They are not four
independent policies that happen to agree — they are four distinct
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

This is the same invariance discipline the kernel's mathematics
already claims about itself — what changes is the *application*
(implementation language, audit procedure, execution ordering); what
does not change, at any layer, is the underlying principle that
mathematical claims require observable termination. The governance
layer is invariant in the same sense the operators are.

**In words** (restates the diagram above; if the two ever disagree,
the diagram is the source of truth and this paragraph is stale):
Observable Provenance and Reverse Traceability states the foundational
principle — mathematical claims require observable termination and
reconstructible provenance. Language Discipline — Rust Mandate states
one implementation mechanism intended to support that principle. The
Kernel Self-Consistency Test states how to check that both the
principle and the implementation discipline are actually being
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
