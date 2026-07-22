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
operators are. The constraints say what may not be added. V7.

**`role_separation_and_operator_application.md`**
Role separation protocol and application workflow. Declares the minimum
role structure required for the kernel to function correctly. V7.

**`abr_operators_plain.md`**
Plain language statement of each operator constraint. No mathematics
required. The reader remains Origin. V7.

**`operators.rs`**
Reference implementation in Rust. Declared relations only. No ring,
no torus, no undeclared topology. V7.

**`derived_invariants.rs`**
Canonical implementation of Layer 3 derived quantities: ε[e], τ[v],
Φ[v], 𝟙[e], J[v], N[v], and photon edge functions. These are the
quantities the operators produce; their forms are invariant across
all declared regions. Every function has a declared observable source
and support classification. V4.

**`validation_record.md`**
Observational grounding for every confirmed quantity in
`derived_invariants.rs`. States what was observed, from which source,
through which measurement mapping M, and with what support classification.
V5.

**`observable_variable_sets.md`**
Declares the primitive variable set at each region (Primary through
Planetary), the Layer 3 derived outputs, architectural principles, and
empirical findings established from the observable record. V7.

**`cross_region_energy_expression.md`**
Derives E[v] = ι[v] · κ[region] from the isolated-locus limit of the
unified τ expression. Distinguishes Φ_stability from Φ_relational.
Confirms κ[Primary] = c² = 8.988×10¹⁶ J/kg as internal consistency
within M. States open conditions OC-E-1 through OC-E-5. V4.

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
through M. A quantity that is declared but not traceable to an observable
through M is not inadmissible — it is not about anything in D.

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

V7. Updates from V4:

- derived_invariants.rs updated V3 → V4: four new invariants added —
  I-T (Tunneling), I-S (Superposition), I-E (Entanglement), I-RE
  (Relational Evolution); joint necessity preamble; 9 new open conditions
- validation_record.md updated V4 → V5: VR-T-01, VR-S-01, VR-E-01,
  VR-RE-01 added; 12 new open conditions; companion file reference updated
- operators_notation_and_constraint.md: Primary Region
  invariants section extended with I-T, I-S, I-E, I-RE cross-references;
  joint necessity observation stated
- abr_operators_plain.md: new plain language section on
  Primary Region phenomena — tunneling, superposition, entanglement,
  relational evolution; epistemic status explicit throughout
- cross_region_energy_expression.md updated V3 → V4: I-RE cross-reference
  added; κ[region] monotonic decrease stated; confirmation language
  discipline applied — four categories: observed measurement confirmed,
  internal consistency confirmed, derived identity confirmed, framework
  consequence pending derivation

**V7 changes (carried forward):**
- Sequential observation requirement added (operators_notation_and_constraint.md)
- Relational evolution direction as distinct named constraint added
- Primary kernel E_primary = Σ(Δ(x)) declared alongside ABR kernel
- ι[v] replaces m[v] throughout — rest mass declared inadmissible (SF-PR-16)
- derived_invariants.rs V3 — photon edge functions, N[v], five new tests
- validation_record.md V4 — VR-γ-01, VR-γ-τ-01, VR-γ-N-01
- observable_variable_sets.md V7 — EF-14 through EF-17
- cross_region_energy_expression.md V3 — κ[Primary] = c² stated
  explicitly; photon edge section; OC-E-5 elevated
- Verifier language discipline: "relational field cycles" inadmissible;
  replaced with "declared relational intervals over the measured
  stability interval"

---

*Metatron Dynamics, Inc. — Delaware C-Corp #10551645*
*Bounded over D. No claim beyond D.*
