# Kernel V10 — Complete Reference

**Metatron Dynamics, Inc.**  
Kernel V10 (2026-11-21). Ground variables, simultaneity and the relational field.  
Kernel V9 (2026-10-24). Foundational primitive established.  
Bounded over D. No claim beyond D.

This document is the single reference for any reader — human or AI — who needs
to apply, generate against, or verify work built on the kernel. It synthesizes
`lib.rs`, `operators.rs`, `derived_invariants.rs`, `ground_variables.rs`,
`relational_field.rs`, and the kernel documents into one place. The canonical
statement of every Kernel V10 item, by ID, is `V10_README.md`. It introduces no declaration, operator, quantity, or
admissibility rule of its own. Where this document and a source file differ,
the source file governs, and the difference is a finding to be returned to
Origin.

---

## Contents

1. Foundation
2. Observational order and governing constants
3. Roles
4. Task scope — kernel audit vs. application
5. Workflow
6. Language discipline
7. Observable record vs. legacy interpretation
8. Relational structure and topology rules
9. Domain and measurement
10. Kernel selection — full-operator admissibility condition
11. Operators (Primary and ABR kernels)
12. Rust data structures
13. Derived invariants — Part I (confirmed)
14. Declared measurement references
15. Derived invariants — Part II (open derivation)
16. Experimental execution discipline
17. Verification
18. Open conditions (held since Kernel V8; RPPI enforcement status)
19. Suspended quantities (record only)
20. Legacy mathematical intrusion — classes and rule
21. Kernel V10 — ground variables and the relational field
22. Version history (summary)

---

## 1. Foundation

### The Foundational Primitive

This framework introduces one primitive that has not previously existed in
mathematics:

**A DIRECTED RELATION DECLARED FROM OBSERVABLE INFORMATION.**

Every operator, every invariant, every admissibility condition in this
framework exists because of this primitive and cannot be used without it.

The primitive is not derived. It is the starting point. It arises from the
distillation of human experience into three irreducible characteristics:

> **Kernel V10 restatement (Df1).** V10 states the same primitive as *derived
> from observation*: relational evolution has not been observed to stop (O1),
> and no registration stands alone (O2); hence relation is the primitive, and
> objects are read from relations, not the reverse. "Not derived" above means
> not derived from any prior mathematical object; V10 makes explicit that it
> is derived from what is observed. The three characteristics below are those
> observations. See `V10_README.md` §1.1 and §21 below.

**1. COMPLEXITY**  
Observable systems are complex. They are not reducible to a single governing
variable or a universal parameter set. The structure of a hydrogen atom, a
magnetospheric current sheet, a biochemical oscillator, and a social network
are incommensurable. Complexity makes domain-specific declaration structurally
unavoidable — not a limitation to be overcome, but the correct description of
what is actually there.

**2. CHANGE AS THE ONLY OBSERVABLE CONSTANT**  
What is consistently observable across all domains is not a stable object, but
change. The directed relation captures change at its most elementary form: a
declared difference between two loci, with direction. Nothing in this framework
assumes a stable substrate beneath the change. Change is what is declared.
Change is what the operators act on.

**3. BOUNDED DOMAIN OF ANALYSIS**  
Every declaration is bounded. D contains only what has been declared through M
from observable evidence. There is no unbounded extension, no inference beyond
the declared boundary, no silent default. The boundary is not a deficiency — it
is what makes the result correspond to what was actually observed rather than to
a model of it.

These three characteristics — complexity, change, and bounded domain — jointly
determine that the correct primitive is a DIRECTED RELATION, not a point, not a
set, not an object with properties. Standard mathematics constructs relations
from objects. This framework takes the directed relation itself as primitive and
derives everything else from it.

**The operators (A, B, R, Δ, Σ) are not independent mathematical objects.**
They are the forms that act on declared directed relations. They cannot be
applied to anything that was not declared through M, and they cannot produce
output that carries meaning beyond the declared boundary.

**The operators exist because of the primitive. The primitive does not exist
because of the operators.**

---

### The Chain

**The chain.**

```
Observable → M → numerical projection in D → declared topology
  → operators → calculated result → comparison with observables
```

- **D** := { x ∈ ℝⁿ | n < ∞, |x[i]| < ∞ ∀ i }
- **M : O → D** is declared by Origin before any operator acts. The kernel
  acts on M(o) only.
- A value in D is the numerical projection of an observable through M, not
  the observable itself. No numerical result is itself an observable.
- **Admissibility is a provenance condition.** Every quantity must be
  traceable to an observable through a declared M. A calculation that is
  formally consistent but not so traceable is not about anything in D.
- **Declared information.** The kernel makes no declaration about what exists
  or does not exist. Where information required by an operator has not been
  declared through M for the calculation, that operator is not evaluated. No
  substitute value or classification is introduced. A 0.0 produced by an
  operator over declared information is a calculated result and is returned
  as such. **Not evaluated ≠ evaluated with result 0.0.**
- **Declaration correspondence.** Analysis proceeds from declared observables
  without expectation of numerical outcome. Where the numerical projection
  produced by Δ or A does not preserve the observed distinction on which a
  declared relation is based, correspondence is not established; the
  declaration may be varied and evaluated again — topology first, with
  variable determination and M. The kernel does not vary the declaration.
  Topology is varied in response to comparison with observables, not to
  obtain a particular numerical result.
- **Relation.** A relation is declared through M from an observed distinction.
  Δ and A compute the numerical directed difference over a declared relation;
  the computation does not declare the relation.
- **Numerical existence is not observational significance.**
- **The operators hold no agency.** Whatever the workflow commits to, a person
  committed it.

---

## 2. Observational Order and Governing Constants

**Origin declaration (2026-09-29).**

The framework begins with human observation within a bounded domain.

**Change is the observable constant. Relation is primitive within the bounded declared domain.**

These are not hypotheses the kernel tests. They are the ground the kernel stands on. They cannot be argued with from within the framework, because the framework is built on them. The full account of why these are the correct governing constants — tracing through complexity, change as the only observable constant, and bounded domain — is in Section 1.

The governing order is:

```
human observation → bounded domain → observable distinction / change
  → declared relation → M → numerical projection in D → declared topology
  → applicable operators → calculated result → comparison with observables
```

Consequences:

- The operators do not create the observable relation. The relation is declared through M from an observed distinction. Δ and A calculate directed difference over the declared relation.
- Where sufficient relational information is declared through M to instantiate the full relational operator path, relational evolution is evaluated through the full established operator structure.
- The Primary kernel (Δ → Σ) applies when the observable record does not provide sufficient relational information to instantiate the full path. It is not the default; it is the exception — the condition of limited information.
- Change is the observable constant and relation is primitive within the bounded declared domain. Where sufficient relational information establishes the full-operator admissibility condition, the complete A→B→R path applies. R is therefore present in every full-path evaluation and is not optional. See section 10.
- When a mathematically conformant application does not correspond with observables, the established kernel mathematics is not thereby reopened. Return the correspondence failure to Origin for examination of the bounded application declaration and observable comparison. A separate implementation-conformance failure is a code finding, not an operator failure.

**Do not convert these statements into a philosophical argument.** Apply them operationally: declare from observation, execute the applicable operators unchanged, compare with observation, return divergence.

---

## 3. Roles

Roles reconstruct from the constraints that must hold for the workflow to
function; they are not named into existence.

| Role | Function | Held by |
|---|---|---|
| **Origin** | Declares D, M, the relations and their provenance, and what is admissible. Disposes of findings. Holds responsibility. | The human. Not assignable to a frame; cannot be delegated without being vacated. |
| **Generator** | Proposes candidate structure within what Origin declared. | Human or AI. |
| **Verifier** | Detects divergence from declared criteria, from a frame distinct from the Generator. Returns located findings, not approval. | Human or AI, from an independent frame. |

**Constraints on the roles:**

1. No declaration, no evaluation.
2. Function defines role, not the label.
3. Description is not instantiation.
4. Responsibility stays with what has agency — the human.
5. Verification detects divergence; it does not approve.
6. Generation and verification require distinct frames.
7. Verification is collaborative; neither participant is presumed free of drift.
8. Scale does not alter declared roles.
9. Roles are declared independently of implementation.

---

## 4. Task Scope — Kernel Audit vs. Application

Before beginning work, classify the active task. These scopes must not be silently crossed.

| Scope | What it may examine |
|---|---|
| **Kernel audit** | Operator derivation; invariant derivation; kernel declarations; implementation correspondence; internal kernel consistency; OPEN/HOLD conditions; source-level contradictions. |
| **Application construction** | Declaring and executing the kernel over a particular observable system. Treats established Kernel V10 content as governing. |
| **Application verification** | Whether the application conforms to Kernel V10, the application's Origin declarations, its declared M, its declared topology, and its observable comparison criteria. Does NOT recursively re-audit Kernel V10. |

**Frozen-kernel rule (application construction and verification).** During application construction or application verification, do NOT reopen, rederive, independently justify, replace, reinterpret, or demand new observational proof for:

- established operator forms
- confirmed Part I invariant forms
- CLOSED findings
- established role constraints
- established admissibility rules
- established implementation behavior
- admitted measurement references
- or other Kernel V10 content explicitly classified as established

Only Origin may intentionally escalate an application task into a Kernel audit.

**Confirmed invariant form vs. system-specific instantiation.** A confirmed invariant form does not require rederivation for every system in which it is evaluated. Separate the established form from the system-specific instantiation of that invariant. If the declarations necessary to instantiate a confirmed invariant are unavailable for a particular system, the invariant is not evaluated for that system. Failure to instantiate in one system does not reopen the derivation of the invariant. "No declaration, no evaluation" does not mean rederive every upstream kernel statement before every evaluation.

**OPEN and HOLD conditions are local.** An OPEN or HOLD condition applies only to the proposition it explicitly identifies. It is not permission to reopen neighboring established mathematics. If the active application does not require resolution of the open condition, preserve its status and continue. If execution actually reaches a point that cannot be evaluated because of that open condition, stop there and return the located condition to Origin. Do not expand it.

---

## 5. Workflow

**Step 1 — Declare (Origin)**

- Declare D and M; the relations and their source; each relation's single
  admissible direction, traceable to an observable through M; independent
  provenance for each direction where both are declared (distinctness axiom).
- No relation fabricated; no boundary closed to supply continuation that was
  not declared.
- If no declaration is admissible, stop — the operators are undefined, not
  approximate.
- **Before forming a relational field** (V9: "before Phase 2"): declare the sequence of actual observations
  {M(o₁), …, M(oₙ)}, what one relational step is (the system's own process
  step), and whether the sequence is measured or from a declared model.
- **Before forming a relational field** (V9: "before Phase 2"): declare the direction of relational evolution from an
  observable property of the process. Index order is not a direction
  declaration.
- **For every analysis:** M declares the operator parameters ρ_base and χ₀
  (and cc[p] for each declared component pair). The kernel holds no value
  for any of them and supplies no default.

**Step 2 — Check declaration completeness, then execute (Generator)**

Declaration completeness is checked before execution. Declaration correspondence is tested through execution and comparison with observables. These are distinct steps.

- **Completeness:** are all inputs required by the applicable operator path declared? This is not the question of whether the declaration has already been proven to correspond correctly to the system. An experimental declaration is permitted to be wrong.
- If a required declaration is genuinely missing: stop at that exact missing declaration and return it to Origin. Do not substitute, do not search for increasingly remote reasons why the application might fail, and do not begin auditing the kernel.
- If all required declarations exist: execute the applicable kernel (section 10) unchanged. State every projection with what it preserves and discards.
- Do not delay execution because Generator or Verifier is uncertain whether the declared inputs will produce correspondence.

**Step 3 — Verify (Verifier)**

Return located findings against the declared criteria (section 17). Once Verifier establishes that (1) the required application declaration exists, (2) it enters through the declared M, (3) the applicable established kernel operation was selected, (4) implementation matches that operation, and (5) no undeclared quantity entered — the mathematical provenance inquiry for that operation is complete. Proceed to observable comparison. Do not recursively investigate established Kernel V10 mathematics unless Origin explicitly requests a Kernel audit.

**Step 4 — Revise (Origin)**

Findings return to Origin; the pass repeats until the Verifier verdict is
PASS as defined in `triad-constraint-methods.md`, or Origin declines each
outstanding refinement on the record.

---

## 6. Language Discipline

| Use | Do not use |
|---|---|
| numerical projection through M; value in D | calling a value in D "the observable" |
| not evaluated; not declared through M | "absent" for an operator or quantity; 0.0 as a stand-in |
| provenance not observed or incomplete | "provenance failure" |
| operator is not evaluated (missing M-declared values) | "declaration failure" |
| ι[v] relational inertia | "mass" as a primitive (SF-PR-16) |
| directed difference | "gradient" |
| correspondence with observables | "truth" / "true" as a claim of the calculation |
| declared relations with provenance | rings, tori, periodic indices, wraparound |

**DRIFT SIGNAL:** legacy mathematics inserted into or before the operators,
or otherwise undeclared.

**Support classifications:**
- **Observed** — from M
- **Inferred** — from observed data under the declared framework
- **Derived** — proven from operator mathematics
- **Downstream projection** — a quantity computed from observed values by a
  declared reduction

**Observational status (scale-conditional):**
- **Locus-resolved** (ℛ_M = 1, 𝓜_M = 1): constituent loci and relations
  are independently declarable through M.
- **Expression-observed, mechanism-unresolved** (ℛ_M = 1, 𝓜_M = 0): the
  observable record contains a traceable expression through M; the mechanism
  is not yet resolvable at that scale.
- "Not directly observable" is not used as a universal classification.

---

## 7. Observable Record vs. Legacy Mathematical Interpretation

An observable record is not rendered inadmissible merely because the publication, discipline, instrument documentation, or dataset containing it also uses legacy mathematics.

**Separate what was observed or measured from the mathematical framework historically used to interpret that observation.**

Observable quantities that may enter the framework through the declared M include: temperature, wavelength, frequency, charge, voltage, timing, occupation, position, energy transition, instrument response, and any other directly reported quantity.

The source's legacy mathematical interpretation does NOT thereby enter Kernel V10. Do not import a Hamiltonian, probability distribution, statistical estimator, conventional force law, information-theoretic quantity, or other legacy construct merely because it appears beside the observational record.

Likewise, do not reject usable observational data solely because such mathematics appears in its source.

The observable quantity enters through M. The legacy interpretation stays out.

---

## 8. Relational Structure and Topology Rules

**Direction.** Every declared relation has exactly one admissible direction —
the direction traceable to an observable through M. The reverse requires
independent observable provenance; without it, it is inadmissible.

**Distinctness axiom.** If both (s,t) and (t,s) are independently observed
and declared, they are distinct relations, not a symmetric pair.

**Rings and tori are inadmissible.** A closing edge declares a node as its
own relational predecessor. Any ring, torus, `np.roll`, or periodic-index
operation is a DRIFT SIGNAL. Refuse it and require declared relations with
provenance.

*Clarification (Origin, 2026-11-07).* Ring and torus inadmissibility concerns
relational evolution: an observation sequence or process step that closes onto
itself (wraparound, periodic indices, `np.roll`, a successor declared as its own
predecessor in the sequence of observations). It does not concern a ring-shaped
arrangement of declared relationships. A cyclic set of independently declared
relations — for example a feedback loop between declared loci — is a declared
relationship, admissible with provenance like any other.

*Kernel V10.3 (2026-11-22): checked by `topology::check_t8` (T8). A
self-relation is refused; every directed cycle must carry at least one edge
declared feedback with a stated basis; opposite directions form a 2-cycle and
need their own basis. Enforced at every V10 reading entry point
(`DeclarationGap::T8`; Origin, 2026-11-22). See `V10_README.md` T8.*

**Before Δ / Before A.** No transformation that alters pairwise differences
precedes A or Δ unless declared within M. Admissible: uniform shift, declared
unit scale. All else requires declaration with preserved and discarded
invariants stated.

**Edge provenance** — every declared edge carries one of three provenances.
A relation with no declared provenance is fabricated within D:

| Provenance | Meaning |
|---|---|
| `Continuation` | Within-family spatial continuation — declared from observable or device geometry. Open boundary: no wraparound. |
| `Coupling` | Cross-family spatial coupling — declared from physical coupling mechanism. |
| `Persistence` | Relational-evolutionary — connects E_prior[e] to E_current[e] across one declared process step. Represented in `PersistenceState`, not in the spatial edge list. |

**Sequential observation requirement (Phase 2).** Phase 1 is admissible on a
single declared observation M(o). Phase 2 requires M to declare a sequence
{M(o₁), M(o₂), …, M(oₙ)} where each oₖ is a declared observation at one
declared process step and the process step is the system's own declared process
step — not an externally imposed increment and not a model-generated value.
A model-generated trajectory is not an observable sequence through M unless
the model is itself declared as a transducer from observable inputs.

**Persistence:** requires a declared pair of sequential observations. At the
first declared observation, only the spatial kernel is evaluated. A
`PersistenceState` can be constructed only from the kernel output of a
completed declared observation (`PersistenceState::from_prior`). No prior
value is supplied where no prior observation was declared.

**Kernel V10 amendment — Df6, simultaneity (Origin, 2026-11-21).** All operators and all ground variables occur simultaneously throughout relational evolution; A → B → R, the persistence forms and energy, freedom and connection are aspects of one ongoing relational evolution, not stages. The composition order and the sequence of registrations are orders of analysis (Df2a). There are no phases. A **relational field** is formed from a declared sequence of at least two consecutive registrations of the same declared relations; the spatial and persistence operators are both evaluated on it, at every registration and every consecutive step, together with the ground variables (`src/relational_field.rs`, `RelationalField::new`). One registration is insufficient information: the operators still compute on it, and their outputs are calculated results, but they are building blocks, not a relational field, and are not cited as a relational reading. The only limit on a reading is declared information. Operator formulas are unchanged. The Phase 1 / Phase 2 text above is retained as the V9 record. See `V10_README.md` Df6.

---

## 9. Domain and Measurement

```
D := { x ∈ ℝⁿ | n < ∞, |x[i]| < ∞ ∀ i }
O → M → D
```

O: observables. M : O → D, declared by Origin before any operator acts.
A value in D is the numerical projection of an observable through M. The
kernel acts on M(o) only. Operator outputs are calculated results.

**M-declared operator parameters (V7.1; Origin general values, 2026-11-07).**
Origin establishes general reference values **ρ_base = 0.5** and **χ₀ = 1**. These
are the standard values to record in M when their numerical scale is appropriate.
χ₀ = 1 is one unit of the declared χ measurement, not a universal physical unit
(for a pixel-domain χ, 1 pixel). Where a mathematical relation or ratio exhibits a
numerical disparity requiring a different scale, Origin may explicitly declare
different values through M, with the reason and units recorded. No operator runs
until M identifies the values used for that evaluation. The kernel does not
silently substitute the general values for a missing M; the operator functions
hold no parameter values. (Source: `operators.rs` header, `77c65ae`.)

| Parameter | Meaning | Status |
|---|---|---|
| ρ_base | Dimensionless ceiling on the weight given to directed imbalance of adjacent relations relative to a relation's own directed difference. Required wherever ρ is computed. | M-supplied (V7.1); Origin general value 0.5 |
| χ₀ | Reference in the saturating map ρ = ρ_base · χ / (χ₀ + χ). Carries the dimension of χ as declared through M. Required wherever ρ is computed. Must be finite and positive (χ₀ = 0 is not evaluable). | M-supplied (V7.1); Origin general value 1 unit of χ; OPEN — saturating form requires Origin declaration |
| cc[p] | Cross-topology coupling for each declared component pair p in R. Required when component pairs are declared; one value per pair. | M-supplied (V7.1); no general value |

Where these values are not declared by M, the operator is not evaluated and
there is no operator result. The kernel does not choose, bound, or substitute
them.

---

## 10. Kernel Selection — Full-Operator Admissibility Condition

A → B → R is admissible when M establishes at least one **interior locus** t:
t = target(e) = source(f) for independently declared edges e and f, each with
provenance through M. The minimal admissible interior is s → t → u.

The condition is structural, not a numerical threshold. M establishes it; the
kernel does not compute it from `DeclaredRelations`. This replaces the retired
ρ_P criterion.

| Condition | Kernel applied |
|---|---|
| Interior not established (M can declare the relation but cannot resolve its constituent loci) | **Primary kernel** Δ → Σ. B is not evaluated — it is not an identity operator evaluated with no effect. |
| Interior established | **ABR kernel** A → B → R |

**R is not optional.** Once the full-operator admissibility condition is satisfied, R must be applied. R must not be removed, bypassed, replaced, or withheld because its output is difficult to interpret, unexpected, zero, or nonzero, because antisymmetric expression appears, because the Generator expects another result, because the Verifier questions whether the result is physically familiar, or because legacy mathematics describes the system differently. Kernel selection occurs from the declared structural condition, not from the desired or expected numerical result. Preserve the result exactly and compare it with observation. Zero and nonzero antisymmetric expression are both admissible mathematical outcomes (see section 11.4).

**Primary Region clarification (Origin, 2026-09-29).** The Primary Region is the exceptional observational condition in which there is insufficient relational information to instantiate the full A→B→R path. Do not infer additional Primary Region mathematics from this statement. The Primary Δ→Σ implementation and its declared admissibility conditions are preserved exactly.

**Experimental fields (Origin directive, 2026-11-07).** Kernel selection remains as
stated above and is unchanged for declared applications. For an experimental
field executed through `diagnostic_exec` (§11.7), the A → B → R path is evaluated
as requested; where the declared topology contains no interior locus, the result
carries an `UnresolvedCondition` warning citing this section. The kernel does not
reject an otherwise computable field because its topology is provisional; the
correspondence of the result with observation is determined outside the kernel.

---

## 11. Operators

### 11.1 Operator Forms

| Operator | Form | Rust function |
|---|---|---|
| Δ | Δ(x)[e] = x[s] − x[t], each declared edge e = (s, t) | `operator_delta(x, rel)` |
| Σ | Σ(g)[e] = g[e] + ρ[e]·(Σ_{adj⁺(e)} g − Σ_{adj⁻(e)} g), g = Δ(x); immediate neighbours only | `operator_sigma(delta, rel, rho_base, chi_0)` |
| E_primary | Σ(Δ(x)) | `operator_e_primary(x, rel, rho_base, chi_0)` → returns (Δ, Σ) |
| A | A(x)[e] = x[s] − x[t] (formula identical to Δ); component pairs (a,b): x[a][i] − x[b][i] | `operator_a(f, rel, pairs)` |
| B | B(g)[e] = g[e] + Σ_{f ∈ succ(e)} g[f]; immediate successors only; terminal edges accumulate nothing | `operator_b(g, rel)` |
| R | R(g)[e] = g[e] + ρ[src(e)]·(Σ_{succ(e)} g − Σ_{pred(e)} g); cross-topology term scaled by cc[p] | `operator_r(bg, rel, rho, cc)` |
| E (spatial; V9 "Phase 1") | R(B(A(x)), ρ(A(x))) | `operator_e(f, rel, pairs, cc, rho_base, chi_0)` |
| A_persistence | E_current[e] − E_prior[e]; direction fixed | `operator_a_persistence(e_current, e_prior)` |
| B_persistence | B form over A_persistence | `operator_b_persistence(a_persistence, rel)` |
| R_persistence | R form over B_persistence; spatial component only | `operator_r_persistence(b_persistence, rel, rho_persistence)` |
| E (spatial and persistence; V9 "Phases 1+2") | spatial, with A_p → B_p → R_p over each consecutive step; evaluated together in a relational field (Df6) | `operator_e_v5(f, rel, pairs, cc, persistence_state, rho_base, chi_0)` |
| ρ | ρ[i] = ρ_base · χ[i] / (χ₀ + χ[i]); χ[i] = max \|A(x)[e]\| or \|Δ(x)[e]\| over edges incident to i; edge form ρ[e] = ρ[source(e)] | `compute_rho`, `compute_rho_primary`, `compute_rho_persistence` |
| Ξ (V10) | Ξ(g)[e] = Σ_succ(e) g − Σ_pred(e) g, g = B(A(x)) or Δ(x); parameter-free | `operator_circulation(g, rel)`, `circulation_half_width(dg, rel)` |
| C | Declared projection: state what it preserves and discards. Not a kernel operator. | — |

### 11.2 ρ Status

**Kernel V10 (Origin, 2026-11-21): ρ and χ are retired from V10 readings**, with
ρ_base, χ₀, the max selection, the saturating form and cc[p]. They imposed a
fixed model of how local magnitude of change (an energy-like quantity read with
no declared carrier) modulates circulation; V10 declares energy with its own
carrier and reads its effect on structure from the data (§21). V10 circulation
is the parameter-free Ξ (§11.1). OC-K-1 is dissolved for V10. The V9 status
below is retained for V9 callers of the unchanged ρ-weighted functions.

- ρ_base, χ₀: **M-supplied (V7.1)** — no kernel value.
- χ[i] = max selection: **OPEN** — Origin declaration required for the max
  selection rule itself.
- Saturating form χ/(χ₀+χ): **OPEN** — Origin declaration required.

### 11.3 M-Declaration Enforcement

```rust
pub fn require_rho_declaration(rho_base: f64, chi_0: f64) {
    assert!(rho_base.is_finite(), "ρ_base must be declared by M as a finite value");
    assert!(chi_0.is_finite() && chi_0 > 0.0,
        "χ₀ must be declared by M as a finite positive value in the dimension of χ");
}

pub fn require_cc_declaration(cc: &[f64], pairs: &[(usize, usize)]) {
    assert_eq!(cc.len(), pairs.len(),
        "cc must be declared by M for every declared component pair");
    assert!(cc.iter().all(|v| v.is_finite()), "cc values declared by M must be finite");
}
```

### 11.4 Antisymmetric Expression — Observational Rule

The operator permits either zero or nonzero numerical antisymmetric expression;
neither outcome is imposed by the mathematics.

- **R_anti = 0 and R_anti ≠ 0 are both admissible mathematical outcomes.**
- **R_anti ≠ 0 → REVIEW SIGNAL.** Not automatically a failure, inadmissibility,
  discovery, or confirmation of a physical phenomenon.
- If nonzero expression occurs: preserve it exactly and examine the complete
  provenance chain. Do not suppress it, threshold it away, force it to zero,
  or automatically classify it.

### 11.5 Structural Checks

| Check | Rust |
|---|---|
| Relational isolation (every edge has empty adj⁺ and adj⁻; Σ's antisymmetric term is then structurally zero) | `relational_isolation(rel)` |
| Declared edge-image admissibility (no edge vector is the negation of another) | `declared_edge_image_admissibility_check(delta, tol)` |
| Antisymmetric term of Σ, extracted separately | `antisymmetric_term(delta, rel, rho_base, chi_0)` |

**Tolerances.** Every `tol` argument is an IMPLEMENTATION-ONLY NUMERICAL
TOLERANCE guarding finite-precision arithmetic. It is not a property of the
observed system and is never reported as one.

### 11.6 Persistence — Usage Pattern

At the first declared observation, evaluate spatial kernel only:

```rust
let e1 = operator_e(&x1, &rel, &pairs, &cc, rho_base, chi_0);
```

At the second and subsequent declared observations:

```rust
let prior = PersistenceState::from_prior(e1);
let (e2, persistence) = operator_e_v5(&x2, &rel, &pairs, &cc, &prior, rho_base, chi_0);
// persistence.a_persistence, .b_persistence, .r_persistence
```

`PersistenceState::from_prior` requires actual observation output — not a
model, parameter value, or static configuration.

**Kernel V10 (Df6).** Use `RelationalField::new(&rel, registrations, &params, carriers)` to form a relational field: it calls `operator_e` at the first registration and `operator_e_v5` with the earlier actual output as prior at each later one, and reads the ground variables over the whole sequence, in one call. Fewer than two registrations returns `DeclarationGap::NoProcessSequence` (insufficient information). The pattern above remains the building block it calls.

### 11.7 Diagnostic Execution — Experimental Fields (`diagnostic_exec.rs`)

Origin directive, 2026-11-07: *Provenance is mandatory. Experimental uncertainty
is observable information, not a reason to prohibit mathematically valid
computation.* `src/diagnostic_exec.rs` is a thin execution wrapper. It calls the
unchanged functions of §11.1 (`operator_a`, `compute_rho`, `operator_b`,
`operator_r`; `operator_a_persistence`, `compute_rho_persistence`,
`operator_b_persistence`, `operator_r_persistence`) and is independent of the
admission interface. It introduces no operator form and no acceptance threshold.

| Entry point | Use |
|---|---|
| `execute_spatial(&ExperimentalField)` | First declared observation: A → ρ → B → R. Returns every operator output, the per-edge provenance, the parameters used, and warnings. |
| `execute_with_prior(&ExperimentalField, &SpatialExecution)` | Subsequent observation: the spatial kernel, then A_p → ρ_p → B_p → R_p against the prior's R. |

**Provenance.** Each edge carries an `EdgeProvenanceRecord`: the existing
`EdgeProvenance` class (mandatory), its observational `support` (`Observed` or
`Proposed`), and its basis. A proposed relation remains proposed in every output.
A provenance record's existence is not observational certainty.

**Parameters.** The field identifies the ρ values used: the Origin general values
(§9) with the unit of χ stated, or explicitly declared values with the reason.
No selection is an error; nothing is substituted.

**Observation succession.** Each field declares its observation identity and the
system's own declared process step (§8). A `SpatialExecution` can be produced only
by execution, never assembled. Persistence pairs an observation only with the
actual output at the **immediately preceding declared process step** of the same
field, with identical declared nodes, edges, components, and component pairs.
Reversal, repetition, gaps, and wraparound of the observation sequence are errors.

**Warnings — execution continues; carried with the results:**

| Class | Raised for |
|---|---|
| `ProposedTopology` | a relation whose support is `Proposed` |
| `IncompleteObservationalSupport` | an edge with no stated basis; a node with no stated measurement relationship; a declared ρ adjustment with no stated reason |
| `UncertainMeasurement` | a node whose measurement relationship is declared uncertain |
| `ExperimentalArrangement` | an experimental variable arrangement declared for the field |
| `UnresolvedCondition` | conditions declared by the field; no interior locus (§10); an empty succ/pred sum evaluated as 0.0 (A-3, §18); ρ parameters or edge provenance differing from the prior |

**Errors — no operator is evaluated; every error found is listed:**

| Class | Raised for |
|---|---|
| `InvalidNodeReference` | an edge endpoint that is not a declared node |
| `DuplicateIdentifier` | a node or edge identifier declared twice |
| `IncompatibleDimensions` | value count ≠ component count; component pair outside the components; no nodes or no relations; class `Persistence` on a spatial edge (§8) |
| `NonFiniteInput` | a non-finite node value, cc, ρ_base, or χ₀ |
| `UndefinedOperation` | χ₀ ≤ 0; a non-finite operator output; a kernel assertion |
| `MissingNumericalInput` | no ρ parameter selection; a node value not supplied; cc not supplied for a declared pair; the unit of χ not stated |
| `IncompatiblePrior` | a prior that is not the immediately preceding declared step of the same field and structure |

Results are calculated results. Their correspondence with observation is
determined by the surrounding experimental system, not by the kernel.

---

## 12. Rust Data Structures

### 12.1 DeclaredRelations

Production constructor — requires explicit provenance for every edge:

```rust
DeclaredRelations::from_edges_with_provenance(
    n_nodes: usize,
    edges: Vec<(usize, usize)>,
    provenance: Vec<EdgeProvenance>,
) -> Self
```

- `edges[e] = (src, tgt)` — spatial edge e
- `provenance[e]` — one `EdgeProvenance` per spatial edge; `Persistence`
  edges are not declared here (use `PersistenceState`)
- `out[i]` — edge indices leaving node i (ABR B and R)
- `inc[i]` — edge indices entering node i (ABR B and R)
- `adj_plus[e]` = `succ(e)` — edges starting where e ends (Primary Σ, ABR B/R)
- `adj_minus[e]` = `pred(e)` — edges ending where e starts (Primary Σ, ABR B/R)

`adj_plus[e]` and `succ(e)` are the same set, stored separately for access
pattern reasons.

### 12.2 NodeField

Numerical projection through M over declared nodes. Input to both kernels.

```rust
NodeField::new(data: Vec<Vec<f64>>) -> Self
// data[c][i] — component c, node i
// k = data.len()   (n_components)
// n = data[0].len() (n_nodes)
// All values must be finite (∈ D)
```

### 12.3 PrimaryEdgeField

Output of primary kernel operators. `field[c][e]`.

### 12.4 EdgeField (ABR)

```rust
EdgeField {
    spatial: Vec<Vec<f64>>,          // spatial[c][e]
    comp: Vec<Vec<f64>>,             // comp[p][i]
    comp_pairs: Vec<(usize, usize)>, // declared component relations
    k: usize,
}
```

### 12.5 PersistenceState

```rust
PersistenceState { e_prior: EdgeField }
impl PersistenceState {
    pub fn from_prior(e_prior: EdgeField) -> Self { ... }
}
```

No constructor without a declared prior. `EdgeField::zero`,
`PersistenceState::cold_start`, and `is_cold_start` were removed at Kernel V8.

### 12.6 PersistenceOutput

```rust
PersistenceOutput {
    a_persistence: Vec<Vec<f64>>,  // [c][e]
    b_persistence: Vec<Vec<f64>>,  // [c][e]
    r_persistence: Vec<Vec<f64>>,  // [c][e]
}
```

### 12.7 EdgeProvenance

```rust
pub enum EdgeProvenance {
    Continuation,
    Coupling,
    Persistence,  // not used in spatial edge list — use PersistenceState
}
```

---

## 13. Derived Invariants — Part I (Confirmed)

**Layer 3 quantities.** Readable only after the operators act. A Layer 3 result
is compared with observables through M; its numerical existence alone does not
establish correspondence.

**Separation rule:** `operators.rs` = what acts. `derived_invariants.rs` = what
is produced, confirmed invariant in form. Do not add operators to
`derived_invariants.rs`. Do not add derived invariants to `operators.rs`.

Nothing in Part I depends on any statement in Part II.

### 13.1 Declared Measurement References

*Amended (Origin, 2026-11-21, 9th):* `HBAR` and every τ = ħ/ε form below are
retired from V10 and behind the V9 fence. The provenance column is under W14
audit C1 (several values are defined or fitted, not measured).

| Constant | Value | Observable provenance | Class (W14, 2026-11-21) |
|---|---|---|---|
| `HBAR` | 1.054_571_817e-34 J·s | Spectroscopic frequency-energy measurement | Defined by convention: h/2π, with h exact in the SI since 2019. Retired from V10 (9th) |
| `E_UNIT` | 1.602_176_634e-19 C | Millikan oil-drop and subsequent direct measurements | Defined by convention: exact in the SI since 2019 |
| `I_UNIT_ELECTRON` | 9.109_383_701_5e-31 kg | Penning trap oscillation frequency under declared non-acceleration | Fitted: the CODATA 2018 least-squares adjusted value (Provider-declared), not one registration |
| `H_PLANCK` | 6.626_070_15e-34 J·s | Spectroscopic frequency-energy measurement; enters D only inside M mapping hc/λ → ε[e] at Atomic Region photon edges | Defined by convention: exact in the SI since 2019. Its mapping output is Projected |
| `C_DECLARED` | 2.997_924_58e8 m/s | BIPM 2019 interferometric measurement; enters D only inside M mapping hc/λ → ε[e] at Atomic Region photon edges; not a named framework variable | Defined by convention: exact since 1983 (it defines the metre). Its mapping output is Projected |

`H_PLANCK` and `C_DECLARED` do not appear in any operator formula. Do not
introduce `C_DECLARED` into operator expressions or kernel output.

### 13.2 ι[v] — Relational Inertia

Replaces m[v] throughout. m[v] declared inadmissible as a primitive — its
definition requires a frame with no declared relational motion, which does not
exist in D (SF-PR-16).

**Declaration of ι[v]:** the inertial response of a declared locus v to an
applied relational contrast at a declared measurement edge, under the declared
condition that A_persistence = 0 at that edge over the declared measurement
interval (non-acceleration declared through M).

Observable provenance: cyclotron frequency, Penning trap oscillation, mass
spectrometry. Numerical values: identical to PDG mass entries. The primitive
interpretation changes; the numerical values carry forward.

Standing language discipline: "mass" → inadmissible as a primitive within D.
Replace with ι[v] at declared loci, or ε[e] at declared edges where energy
contrast is the direct observable.

### 13.3 ε[e] — Relational Energy Contrast

```
ε[e] = |Δ(ι)[e]| · 𝟙[e]
```

Support: Derived. Confirmed: 22 transition edges across PDG and NIST.

```rust
pub fn epsilon_e(delta_iota_e: f64, indicator: f64) -> f64 {
    // indicator must be 0.0 or 1.0
    delta_iota_e.abs() * indicator
}
```

HOLD (since Kernel V8): depends on the 𝟙[e] audit (A-3 open condition). No change.

### 13.4 𝟙[e] — Detection Indicator

```
𝟙[e] = 1  iff  R(A(x))[e]_antisymmetric ≠ 0
𝟙[e] = 0  otherwise
```

Support: Derived (OC-12 resolved). 𝟙[e] is not a parameter. It is read from
the R operator output after the kernel has acted.

```rust
pub fn detection_indicator(r_antisymmetric_e: f64, tol: f64) -> f64 {
    if r_antisymmetric_e.abs() > tol { 1.0 } else { 0.0 }
}
// tol is IMPLEMENTATION-ONLY NUMERICAL TOLERANCE
```

AUDIT PENDING (since Kernel V8): determine whether 0.0 always means an evaluated R
output whose antisymmetric term is 0.0, or is ever used where no relation was
declared through M. Follows Origin determination of empty-adjacency semantics
(audit A-3).

### 13.5 ε[e] at Photon Edges

```
ε[e] = hc / λ
```

M mapping: M maps spectrometer-reported wavelength λ (m) to ε[e] (J).
The formula hc/λ is part of M at the Atomic Region.

ι at the photon locus: **not declared through M (A-11 CLOSED).** Under the
declared-information rule, ι is not evaluated at photon edges — not
evaluated-and-equal-to-zero. E[v] = ι[v] · κ[region] is not evaluated at
photon edges. ε[e] = hc/λ stands on its own M mapping.

Confirmed: 14 transitions (NIST ASD v5.12).

```rust
pub fn epsilon_photon_edge(lambda_m: f64) -> f64 {
    H_PLANCK * C_DECLARED / lambda_m
    // lambda_m in meters; declared from instrument through M
}
```

### 13.6 τ[v] — Relational Progression Interval (Unified)

```
τ[v] = ℏ / |A_persistence[e_coupling] · cos θ[e_v, e_coupling]|
```

Support: Derived (OC-13 resolved). Special cases confirmed from observable record:

| Case | Expression | Confirmed |
|---|---|---|
| Isolated locus | τ = ℏ / E[v] | PDG ordering confirmed |
| Parallel coupling (cos θ = 1) | τ = ℏ / A_persistence | H₂O ν₂ bend: 1.0 ps exact |
| Perpendicular coupling (cos θ → 0) | τ → very large | He 2s metastable: 7900 s |
| Near-perpendicular (cos θ = 0.1176) | τ = 8.5 ps | H₂O ν₁,ν₃ stretch modes exact |

```rust
pub fn tau_v(a_persistence_coupling: f64, cos_theta: f64) -> f64 {
    HBAR / (a_persistence_coupling * cos_theta).abs()
}
pub fn tau_v_isolated(e_v: f64) -> f64 { HBAR / e_v }
pub fn tau_photon_edge(lambda_m: f64) -> f64 {
    HBAR / epsilon_photon_edge(lambda_m)
}
```

### 13.7 N[v] at Photon-Emitting Loci

```
N[v] = τ_stability[v] · ε[e] / ℏ
```

Count of declared relational intervals within the measured stability interval.
Dimensionless. Confirmed: H n=3 (Hα) → N ≈ 4.58e6.

```rust
pub fn n_relational_cycles(tau_stability_s: f64, lambda_m: f64) -> f64 {
    tau_stability_s * epsilon_photon_edge(lambda_m) / HBAR
}
```

### 13.8 θ[e] — B Output Vector Direction

```
v[e] = B(g)[e] · θ̂[e]
Equilibrium condition: Σᵢ v[eᵢ] = 0 at each declared locus
```

Confirmed exact:

| Bonds | Angle |
|---|---|
| 2 | 180° |
| 3 | 120° |
| 4 | 109.47° |

```rust
pub fn equilibrium_bond_angle_rad(n_bonds: usize) -> Option<f64> { ... }
pub fn equilibrium_bond_angle_deg(n_bonds: usize) -> Option<f64> { ... }
// returns None for n_bonds outside {2, 3, 4}
```

### 13.9 Φ[v] — Coherence Density (Node) *(was: Coherence Potential; renamed 10.3.0, E6, W7)*

```
Φ[v] = ι[v] · τ[v]
```

Support: Observed throughout (formal derivation OC-Φ-1 pending). Confirmed
across 29 declared loci spanning 78 decades in Φ.

Region-specific threshold condition:
- Primary Region: log₁₀(Φ) ∈ [+36.53, +39.47]
- Planetary: log₁₀(Φ) ∈ [+59.02, +62.02]

```rust
pub fn phi_v(iota_v: f64, tau_v: f64) -> f64 { iota_v * tau_v }
pub fn phi_v_log10(iota_v: f64, tau_v: f64) -> f64 { phi_v(iota_v, tau_v).log10() }
```

### 13.10 Φ[S] — Coherence Density (Collective) *(was: Coherence Potential; renamed 10.3.0, E6, W7)*

```
Φ[S] = ι[S] · τ[S]  where  ι[S] = Σᵢ ι[vᵢ]
Accumulation condition: Φ[S] > Σᵢ Φ[vᵢ]
```

Support: Observed. Confirmed across six accumulation transitions, four scale
transitions. Zero violations.

```rust
pub fn phi_s(iota_s: f64, tau_s: f64) -> f64 { iota_s * tau_s }
pub fn accumulation_condition(phi_s_val: f64, phi_components: &[f64]) -> bool {
    phi_s_val > phi_components.iter().sum::<f64>()
}
```

### 13.11 J[v] — R Output Multiplicity

```
J[v] = (declared edges at v) / 2
R output states = 2J[v] + 1
```

What the established literature calls spin is R output multiplicity. What the
established literature calls Zeeman components is what the framework computes
as 2J[v] + 1 distinct R output states.

Support: Derived. Confirmed 29/52 (VR-J-01 — completion in progress).

```rust
pub fn j_v(n_declared_edges_at_v: usize) -> f64 { n_declared_edges_at_v as f64 / 2.0 }
pub fn r_output_states(n_declared_edges_at_v: usize) -> f64 { 2.0 * j_v(n_declared_edges_at_v) + 1.0 }
```

### 13.12 Summary Table — Part I

| Quantity | Form | Rust |
|---|---|---|
| ε[e] relational energy contrast | \|Δ(ι)[e]\| · 𝟙[e] | `epsilon_e(delta_iota_e, indicator)` |
| 𝟙[e] detection indicator | 1 iff R(A(x))[e] antisymmetric term ≠ 0 | `detection_indicator(r_anti, tol)` |
| ε[e] at photon edges | hc/λ (part of M at Atomic Region; ι not declared at photon locus) | `epsilon_photon_edge(lambda_m)` |
| τ[v] at photon edges | ℏ / ε[e] | `tau_photon_edge(lambda_m)` |
| N[v] at photon-emitting loci | τ_stability · ε[e] / ℏ | `n_relational_cycles(tau_stability_s, lambda_m)` |
| τ[v] unified | ℏ / \|A_persistence[e_coupling] · cos θ\| | `tau_v(a_p, cos_theta)`; isolated case `tau_v_isolated(e_v)` |
| θ[e] equilibrium bond angle | from Σᵢ v[eᵢ] = 0 (2, 3, 4 bonds) | `equilibrium_bond_angle_rad`, `_deg` |
| Φ[v] coherence density *(was: coherence potential; renamed 10.3.0, E6, W7)* | ι[v] · τ[v] | `phi_v`, `phi_v_log10` |
| Φ[S] collective | ι[S] · τ[S]; accumulation Φ[S] > Σᵢ Φ[vᵢ] | `phi_s`, `accumulation_condition` |
| J[v] R output multiplicity | (declared edges at v) / 2; states 2J+1 | `j_v`, `r_output_states` |

---

## 14. Declared Measurement References

(See also section 13.1 for constant values.)

`HBAR`, `H_PLANCK`, `C_DECLARED`, `E_UNIT`, `I_UNIT_ELECTRON` are declared
measurement references. `H_PLANCK` and `C_DECLARED` enter D only inside the M
mapping hc/λ → ε[e] at Atomic Region photon edges; neither appears in any
operator formula.

---

## 15. Derived Invariants — Part II (Open Derivation)

**EPISTEMIC STATUS.** The statements in this Part are NOT derived from the
kernel. They are framework-level necessity statements: consistent with the
operator structure and with the observable record, and awaiting derivation.
Each carries a named open condition. Nothing in Part I depends on anything here.
A Part II statement may not be cited as a proven consequence of the operators.
Movement from Part II to Part I requires the open condition to be closed by
derivation, not by additional agreement. The functions in this Part compute
declared conditions; computing correctly is not derivation.

### 15.1 Joint Necessity — Preamble

I-T, I-S, and I-E are jointly necessary conditions for any self-organizing,
evolving, observable system to exist within D. This is a framework-level
necessity argument, not a mathematical consequence already established by the
kernel.

These conditions are observable at every scale where M operates. What changes
across scales is not whether the phenomena are observable but what M can resolve
about their internal mechanism (𝓜_M). What the established literature calls
quantum phenomena are these conditions expression-observed at the Primary Region.

### 15.2 I-T — Tunneling Invariant

At every declared transition edge where a relational energy contrast is
established through M, ε[e] > 0. Consequences: 𝟙[e] > 0 at every such edge;
ε[e] has a nonzero floor; τ[v] = ℏ / ε[e] is bounded above.

What the established literature calls tunneling is what M observes at Primary
Region transition edges where ε[e] > 0 and 𝓜_M = 0.

Open conditions: OC-T-1, OC-T-2 (see section 18). *(OC-T-3 dissolved, G1-OC 2026-11-12; listed here as open until 2026-11-22.)*

```rust
pub fn tunneling_floor_holds(epsilon_e: f64, tol: f64) -> bool { epsilon_e.abs() > tol }
pub fn tunneling_tau_bounded(tau_v: f64) -> bool { tau_v.is_finite() && tau_v > 0.0 }
```

### 15.3 I-S — Superposition Invariant

**DECLARED — Origin declaration, 2026-09-23.**

The superposition condition is the condition in which the Δ output field carries
relational distinguishability — at least two declared edges produce directed
differences that are not proportional — before B accumulation resolves them.

Relational distinguishability: there exist declared edges e and f such that
Δ(x)[e] and Δ(x)[f] are not proportional — no scalar k such that
Δ(x)[e][c] = k · Δ(x)[f][c] for every component c.

Superposition resolution: structural condition — M independently establishes
at least one interior locus t (full-operator admissibility condition). Not a
numerical threshold.

Observable basis: expression-observed at Primary Region (ℛ_M = 1, 𝓜_M = 0);
locus-resolved at larger scales (ℛ_M = 1, 𝓜_M = 1).

Open conditions: none. *(OC-S-1 closed 2026-09-23; OC-S-3 dissolved, G1-OC 2026-11-12; OC-S-2 closed 2026-11-22. Listed here as "OC-S-2, OC-S-3" until 2026-11-22.)*

```rust
pub fn superposition_condition(delta_field: &[Vec<f64>], tol: f64) -> bool {
    // tol is IMPLEMENTATION-ONLY NUMERICAL TOLERANCE
    // Returns true if at least one pair of declared edges has non-proportional
    // directed differences. O(n²·k). No SVD. No threshold.
    ...
}
pub fn superposition_resolved(has_interior: bool) -> bool { has_interior }
```

HOLD (since Kernel V8) on `are_proportional`: edge-order dependent when one Δ vector
is 0.0 in every component. No code change authorized.

### 15.4 I-E — Entanglement Invariant

A declared edge e = (s, t) with 𝟙[e] = 1 establishes a relational constraint
between loci s and t preserved under the operators regardless of spatial
separation. The constraint is not a parameter — it is read from R operator output.

This is not action at a distance. It is a statement about declared relational
structure.

Decoherence: 𝟙[e] = 0 and ε[e] = 0 — the admissible statement within D.
Disrupting the edge removes the correlation.

Observable basis: EPR-type measurement correlations; molecular bond
directionality (confirmed across 22 transition edges, validation_record_v4.md).
*Amended (W14, 2026-11-21):* the EPR-type entry is a cited intersection: the registered quantity is the coincidence counts between detectors at s and t over declared settings. A Bell "violation" compares a combination of those counts with a bound derived from a model; it is a cited intersection (C), not a confirmation.

Open conditions: none. *(OC-E-3 dissolved, G1-OC 2026-11-12; OC-E-1, OC-E-2 closed 2026-11-22. Listed here as open until 2026-11-22.)*

```rust
pub fn entanglement_condition(indicator: f64) -> bool { indicator == 1.0 }
pub fn decoherence_condition(indicator: f64, epsilon_e: f64, tol: f64) -> bool {
    // tol is IMPLEMENTATION-ONLY NUMERICAL TOLERANCE
    indicator == 0.0 && epsilon_e.abs() <= tol
}
```

### 15.5 I-RE — Relational Evolution Rate Invariant

*Amended (Origin, 2026-11-21, 10th):* Φ is retired from V10 (it takes a
lifetime as input; time is not a variable), so the 78-decade span is no longer
an observable basis for I-RE. *(11th, W15: Φ is restored as a declared
intersection with a time projection; it is still not a basis for I-RE.)* A rate of relational evolution is now counted:
steps with change resolved at δ out of the declared steps (V10_README V5).
OC-RE-1 stays open. The text below is kept as the record.

Relational evolution rate decreases monotonically from the Primary Region
outward. Primary Region is the fastest-evolving declared region in D. Φ[v]
78-decade span is the observable expression of this invariant. Larger-scale
structure is downstream accumulation of Primary Region relational evolution.

Open condition: OC-RE-1.

### 15.6 I-SC — Structural Consistency (Kernel V10)

Object: the minor pattern P(F) over a declared sequence (§21.3). Derived:
I-SC-D1, a minor cannot change sign over a step whose change bound is smaller
than its magnitude (|Δm_ij| < |m_ij|); I-SC-D2, per minor at δ freedom and
connection are complementary. Proposed: I-SC-1, along registrations of one
declared system ordered by increasing held energy, κ_δ does not decrease across
phase boundaries (the condensate-to-plasma axis); I-SC-2, at no registered state
with at least two components is κ_δ exactly 0 or 1 (G1); I-SC-3, identity-grade
connection persists across levels of combination until a reorganization is
registered.

*Amended (Origin, 2026-11-21, 7th):* structure is persistence; change is
freedom (§21.6). I-SC-1 is restated on F_δ of PC1: along registrations ordered
by increasing held energy, the share of relations whose change is resolved at δ
does not decrease across phase boundaries. κ_δ measures how distinct an
arrangement is, not freedom. The first statement is kept above.

Derived (2026-11-21, Generator): I-SC-D3 to I-SC-D13, relating proportional
state, coupled change, circulation, cycle closure, components and the
declared sequence
(`V10_README.md` §2.6, `tests/v10_structural_consistency.rs`). OC-SC-4 partly
derived. **[10.3.0 (2026-11-22, 26th): I-SC-D14 derived, the held minor pattern
and the sustained circulation are independent grades, all four combinations
realized (`tests/v10_grade_independence.rs`). OC-SC-4 closed.]**

Open conditions: OC-SC-1 to OC-SC-3 (`derived_invariants.rs` Part II) *(was: OC-SC-1 to OC-SC-4; OC-SC-4 closed by I-SC-D14, 10.3.0)*.

---

## 16. Experimental Execution Discipline

### 16.1 Execution order

Experimental application follows this order and no other:

**Declare → Execute unchanged → Preserve output → Compare → Return correspondence/divergence to Origin**

It does NOT follow:

**Declare → interrogate until AI believes declaration is correct → modify declaration → execute**

The Generator does not determine whether Origin's declaration is likely to succeed before executing it. The Verifier does not require the declaration to produce the expected result before accepting it as an experimental declaration. Neither participant may tune the declaration to obtain correspondence.

### 16.2 Dual trace

Every experimental repository should, where applicable, preserve enough information for human and AI inspection of two independent traces.

**Mathematical trace:** Origin declaration → M → D → declared topology → kernel selection → operator inputs → intermediate operator outputs → calculated result.

**Observational trace:** observable source → measured/reported quantity → M mapping → independent observable comparison target.

These traces must remain distinguishable. Do not silently combine the calculated result with the observational target. A final numerical result without this trace is insufficient for independent experimental verification.

### 16.3 Correspondence classification

A result may be:

- mathematically conformant and observationally correspondent
- mathematically conformant and observationally divergent
- mathematically nonconformant
- not evaluable because a required declaration is missing
- unresolved because the observable comparison is insufficient

Do not collapse these into a generic PASS/FAIL.

### 16.4 Divergence is data

When calculated output and observable comparison diverge, preserve the divergence.

Do NOT: alter inputs to eliminate it; change topology to obtain the desired result; add undeclared edges; infer reverse relations; introduce undeclared mathematics; fit or tune parameters; suppress unexpected R output; threshold away unexpected expression except where an implementation-only tolerance is explicitly authorized; reinterpret the observation to manufacture agreement; or replace an observation with a legacy mathematical prediction.

Return the divergence to Origin. Where possible identify: declared input → operator location → calculated result → observable comparison target → divergence.

The divergence may indicate a problem in: the application declaration; M; topology; data completeness; implementation; correspondence assumption; observable comparison record; or an explicitly encountered OPEN kernel condition. Do not determine which explanation is correct without evidence.

### 16.5 No silent repair

Generator and Verifier must not silently repair an experimental declaration. If something appears wrong: preserve the supplied declaration; determine whether it is complete enough for evaluation; if complete, execute it; preserve the result; compare with observation; return any divergence to Origin. Only Origin changes the declaration.

---

## 17. Verification

**Verifier criterion:** *"Can I trace every mathematical operation continuously
back to declared observables through declared operators?"* Correctness is not
the question. Provenance is.

**Four stages:**

| Stage | Confirms |
|---|---|
| 1 — Declaration Verification | D declared and bounded; M declared; variables and relations declared before use; support classifications assigned; no undeclared quantities |
| 2 — Mathematical Verification | Every result follows from declared operators; no imported constructs; dimensional consistency; implementation matches declared mathematics |
| 3 — Observable Validation | Calculated results agree with independent observables (e.g. PDG, NIST ASD); no free parameters, fitting, or tuning |
| 4 — Repeatability | Independent investigators reproduce every result; code produces identical results |

**Verification and validation** partition the error classes: verification
reaches conformance errors; validation reaches correspondence errors. A PASS
under the publication gate is a conformance statement and carries no
correspondence claim.

**Seven verification methods** (`triad-constraint-methods.md`): Claim Provenance
(CP), Comparison Scope Declaration (CS), Declaration–Implementation
Correspondence (DI), Cross-Artifact Consistency (CA), Invariant Range
Declaration (IR), Adversarial Gate (AG), Correspondence Register (CR).

**Test classification key (for Rust tests):**

| Tag | Meaning |
|---|---|
| [IMPL] | Implementation verification: code executes declared formula without error |
| [MATH] | Mathematical verification: declared mathematical property of the operator formulas |
| [REPR] | Forward reproducibility/determinism: two independent forward applications agree |
| [OBS] | Observable validation: drives operator chain from declared M-mapped projection, checks against declared measurement source |
| [CORR] | Correspondence test: checks operator output against a declared value from the validation record |

**Synthetic test fixtures** assign `EdgeProvenance::Continuation` to all edges
and treat Origin-declared inputs as fixtures for mathematical/implementation
verification only. Synthetic test provenance is not empirical provenance.

---

## 18. Open Conditions (Held Since Kernel V8) and RPPI Enforcement Status

| Item | Where | Status |
|---|---|---|
| A-3 — empty adjacency in Σ, R, R_persistence | `operators.rs` | **Decided 2026-11-22: 0, flagged record-boundary-conditioned; never read as zero circulation** *(was: OPEN — not classified. Operators unchanged; `diagnostic_exec` reports each affected edge as an `UnresolvedCondition` warning (§11.7))* |
| 𝟙[e] interpretation audit; ε[e] downstream | `derived_invariants.rs` | HOLD |
| A-11 — ι at photon locus not declared through M | `derived_invariants.rs` | CLOSED — legacy import (SF-PR-16); declared-information rule applies |
| `are_proportional` edge-order dependence and two-unit tolerance | `derived_invariants.rs` | HOLD for the V9 function, which is unchanged. **Not reached by V10 readings:** V2 uses the minor form, symmetric in edge order with one resolution in one unit (§21.4) |
| ρ max-selection rule | `operators.rs` | Declared 2026-11-14; **retired from V10 with ρ (2026-11-21)** |
| ρ saturating form | `operators.rs` | **Dissolved for V10: ρ retired (2026-11-21).** OPEN only for V9 callers |
| Part II "absence" wording | I-E, joint necessity, photon J[v] | Separate audit |
| R² = 0.527 at the δι fit | `observable_variable_sets.md` | HOLD |
| J[v] wording on what M observes | `derived_invariants.rs` | Origin wording decision |
| OC-T-1 | Tunneling floor magnitude: formal expression not yet derived. *Restated (W14, 2026-11-21):* where, and at what δ, the exchange floor ε > δ_ε is resolved over a registered carrier (V3); read from registered data, no mechanism posited | OPEN |
| OC-T-2 | Tunneling at scale: confirm beyond Primary and Atomic regions | OPEN |
| OC-T-3 | Tunneling necessity: formally derive that absence precludes energy transfer | **Dissolved (G1-OC, 2026-11-12); register corrected 2026-11-22** *(was: OPEN)* |
| OC-S-1 | Kernel boundary criterion | CLOSED (2026-09-23: structural interior locus criterion) |
| OC-S-2 | Relational distinguishability beyond Primary Region | **Closed 2026-11-22: freedom readable at every region (PC)** *(was: OPEN)* |
| OC-S-3 | Formally derive absence precludes self-organization | **Dissolved (G1-OC, 2026-11-12); register corrected 2026-11-22** *(was: OPEN)* |
| OC-E-1 | 𝟙[e] = 1 preservation under operator evolution | **Closed 2026-11-22: 𝟙 fenced; V10 persistence conditions (PC, I-SC-D5)** *(was: OPEN)* |
| OC-E-2 | Decoherence rate derivation | **Closed 2026-11-22: a rate in time of a fenced reader (W15)** *(was: OPEN)* |
| OC-E-3 | Entanglement necessity | **Dissolved (G1-OC, 2026-11-12); register corrected 2026-11-22** *(was: OPEN)* |
| OC-RE-1 | Formally derive monotonic decrease of relational evolution rate | **Open for registered data (counted Γ across regions); Φ basis withdrawn** *(was: OPEN)* |
| OC-R-1 | g-factor derivation: g = 2·ρ[e_B]/ρ_base | **Closed 2026-11-22: ρ retired** *(was: OPEN)* |
| OC-θ-3 | Resonance / partial bond order | **Moved to the application 2026-11-22** *(was: OPEN)* |
| OC-θ-5 | Full 3D vector balance N > 4 | **Moved to the application 2026-11-22** *(was: OPEN)* |
| OC-Φ-1 | Formal derivation of Φ[v] from operator fixed-point | **Closed 2026-11-22: Φ is a declared intersection (W15)** *(was: OPEN)* |
| OC-Φ-2 | Region-specific stable threshold from operator fixed-point | **Closed 2026-11-22: Φ is a declared intersection (W15)** *(was: OPEN)* |
| OC-Φ-6 | Observer frame overlap — structural or contingent | **Decided 2026-11-22 with OC-F-1: each record declares its observer** *(was: OPEN)* |
| OC-CA-1 | Φ[S] formal derivation from operators | **Closed 2026-11-22: Φ is a declared intersection (W15)** *(was: OPEN)* |
| OC-22 | α_r inter-region coupling formal derivation | **Moved to the application 2026-11-22** *(was: OPEN)* |
| OC-ι-1 | ι_unit structurally fundamental or instrumentally bounded | **Open (Proposed); inputs classed under W14** *(was: OPEN)* |
| OC-ι-2 | Relational inertia ratios (ι[μ]/ι[e], ι[τ]/ι[e]) — no declared relational origin | **Open (Proposed); inputs classed under W14** *(was: OPEN)* |
| OC-ι-3 | δι ~ ι^0.5 scaling — connection to ρ saturation not derived | **Closed 2026-11-22: ρ retired; the scaling is a fit (W14)** *(was: OPEN)* |

**Kernel V9 RPPI enforcement — bounded first implementation** (`v9_specification.md`
§6–§10; `src/provenance.rs`, `src/provenanced_exec.rs`, `src/admission.rs`). Subject to
Verifier review and Origin authorization.

Every M, input, parameter, direction, classification, and kernel-selection claim must
terminate in a declared observable naming an external record. Absent or unverifiable
basis → OPEN (not evaluated, not admitted). Only an established contradiction →
REJECTED. Admission is a provenance-conformance statement, not a correspondence claim;
results whose termini are synthetic or simulated are admitted only as such.

| RPPI failure mode | Enforcement (V9 admission interface, `src/admission.rs`; V8 constructors unchanged) | Test | Status |
|---|---|---|---|
| Missing provenance (§6.1) | Any absent declaration, observational basis, external record, or measurement conditions → **OPEN**, not admitted (Origin correction 2026-10-25, rule 6) | `rppi_missing_provenance_not_admitted` | ENFORCED — bounded scope |
| Fabricated provenance (§6.2) | Claimed declaration content contradicting the authoritative declaration → **REJECTED**; a cited declaration that is not available → **OPEN** (rule 7) | `rppi_fabricated_provenance_rejected` | ENFORCED — bounded scope |
| Disconnected provenance (§6.3) | Recorded input binding, traversal, or output derivation contradicting the independent reconstruction → **REJECTED** at the located link; a bare operator output cannot be submitted (compile-time) | `rppi_disconnected_derivation_rejected` | ENFORCED — bounded scope |
| Misattributed provenance (§6.4) | Attribution to a declared relation other than the reconstructed producing relation → **REJECTED**, also at identical numerical output | `rppi_misattributed_provenance_rejected` | ENFORCED — bounded scope |

**Supported scope:** Primary-kernel Δ over one declared observation (Phase 1); Primary Σ
only where ρ_base and χ₀ are declared through M; one declared edge M projection through
a kernel-declared function (ε[e] = hc/λ); M kinds Identity, DeclaredUnitScale,
KernelDeclaredFunction (a supported subset, not the canonical M vocabulary).

**Still OPEN (separate completion requirements):** ABR A → B → R, component pairs and cc;
persistence (Phase 2); other Layer 3 quantities; other M kinds; independent IRRS
verification from a distinct frame (milestone 7) — `admission.rs` reconstructs within
the kernel crate and shares its operator code; a record forged consistently in every
field over two declarations with identical computation is not distinguishable by
reconstruction alone. Field value types remain a bounded first implementation of the
definitions held for Origin in `v9_specification.md` §8.

**H-alpha photon edge (canonical declarations): OPEN.** With exactly the declarations the
repository holds, admission reports: measurement conditions not declared for λ
(including air or vacuum wavelength), H_PLANCK, and C_DECLARED; no declared domain
extent for photon-edge ε[e]; no declared EdgeProvenance class or class basis for the
photon edge. VR-γ-01 refers to a V4 table not held in this repository. These are
returned to Origin; none is supplied.

---

## 19. Suspended Quantities (Record Only)

The following quantities are retired. They must not be reintroduced,
substituted, or approximated without an explicit Origin declaration traceable
through M.

| Quantity | Reason | Resolution |
|---|---|---|
| rank(Im Δ), rank(Im Σ) | SVD not required to execute operators; conventional matrix rank ≢ relational differentiation | Replaced by relational distinguishability condition (I-S, V4.3) |
| ρ_P (RhoP, rho_p_ratio, im_delta_rank, im_sigma_rank) | Depended on rank(Im Σ) and propagation_capacity | Retired; kernel boundary is full-operator admissibility condition |
| detect_failure_mode, expression_condition | Depended on rank | Retired |
| FailureMode::DifferentiationCollapse, FailureMode::CirculationCancellation | Depended on rank | Retired; FailureMode::RelationalIsolation replaced by `relational_isolation()` |
| propagation_capacity | Used only to compute ρ_P | Retired |
| frobenius_norm, frobenius_norm_spatial | SVD dependency | Removed |
| cc = 0.5 (fixed in operator_r) | Undeclared unit choice (V7.1) | Replaced by M-supplied cc[p], one per component pair |
| m[v] (rest mass as primitive) | Requires frame with no declared relational motion — inadmissible (SF-PR-16) | Replaced by ι[v] relational inertia; numerical values unchanged |
| Rank formulation of I-S | rank(Im Δ) > 1 suspended | Replaced by relational distinguishability declaration |

No surrogate may be substituted for any suspended quantity: not count,
dimensionality, component count, edge count, nonzero-entry count, entropy,
variance, state count, or any other conventional measure.

---

## 20. Legacy Mathematical Intrusion — Classes and Rule

**The rule:** any mathematical quantity in Generator output that was not
declared by Origin as traceable through M before generation began has provenance
not observed or incomplete — whatever framework it comes from and whether or not
it computes correctly. Correctness is not the question. Provenance is.

The Verifier is required to check Generator output — including all code —
for quantities that entered from the Generator's training distribution rather
than from Origin's declaration.

**Common intrusion classes for AI Generators:**

| Class | Examples |
|---|---|
| Statistical | Probability, expected value, variance, correlation, ensemble averages, Bayesian priors. A single declared observable is not an ensemble. |
| Quantum mechanical | Wave functions, Hamiltonians, coupling constants, decoherence rates, density matrices. Subject matter being quantum mechanical does not license the formalism. |
| Classical mechanical | Mass as primitive, force, potential energy, trajectories, phase space entering as assumed primitives. |
| Information-theoretic | Entropy, mutual information, KL divergence imported as observables rather than declared projections. |
| Geometric | Manifolds, curvature, metric tensors, differential forms assumed as background rather than declared relational structure. |

Any mathematical quantity not on this list is subject to the same rule.

**Prefixed quantities** (e.g., Spearman, Pearson, Fourier, Lagrangian): flag
before using — these require Origin declaration through M before appearing in
Generator output.

---

## 21. Kernel V10 — Ground Variables and the Relational Field

This section restates `V10_README.md` (§1.1, §1.2, V1–V3, V8, Df6) and
`ground_variables.rs` / `relational_field.rs`. Where they differ, those govern.

### 21.1 Simultaneity (Df6)

All operators and all ground variables occur simultaneously throughout
relational evolution. A → B → R, the persistence forms, and energy, freedom and
connection are aspects of one ongoing relational evolution, not stages. The
composition order and the sequence of registrations are orders of analysis
(Df2a). There are no phases. The only limit on a reading is declared
information.

A **relational field** is formed from a declared sequence of at least two
consecutive registrations of the same declared relations, ordered by an
observable property of the process. `RelationalField::new` evaluates, in one
call, A, B and the circulation Ξ at each registration, A_p, B_p and Ξ_p over
each step (prior always the earlier registration), and the ground variables over
the whole sequence. It takes **no operator parameters**: ρ, χ and cc[p] are
retired from V10 (§11.2). A, B and Ξ are linear, so B_p and Ξ_p are exactly the
differences of B and Ξ.
One registration is insufficient information (`DeclarationGap::NoProcessSequence`):
its operator outputs are calculated results and building blocks, not a
relational field. Two registrations are the minimum information, not a
guarantee of enough.

### 21.2 Observable fact and calculated variable

Energy, connection and freedom are present in every registration (G1, G4);
absence is not an available observation, and no calculation tests it. The
calculated variable is the degree the operators yield over a declared topology
and sequence at declared δ. A reading that cannot be calculated is a finding
about the declaration, never about the system:

| Finding | When |
|---|---|
| **Primary Region** | No interior locus is declared (T7). Applies only to the sustained grade of connection |
| **Domain not fully declared** | A carrier, component, relation, registration, process sequence or finer δ is missing (R9a) |
| **Not resolved at δ** | Calculated, with degree zero at δ (W2) |

The remedy is to return to the declaration (G4a, W5); a declared edge is never
reversed to obtain a reading (W8), and no substitute value is introduced.

### 21.3 One relational object: the minor pattern P(F)

For declared edges e, f and components i < j:

```
m_ij(e, f) = A[i][e]·A[j][f] − A[j][e]·A[i][f]
```

with half-width |A[i][e]|·δ_A[j][f] + |A[j][f]|·δ_A[i][e] + δ_A[i][e]·δ_A[j][f]
plus the same bound for the second product, δ_A[e] = δ[s] + δ[t]. A[e] and A[f]
are proportional exactly when every m_ij is zero. Over the declared sequence
each minor does one of three things:

| m_ij(e, f) over the sequence | Reading |
|---|---|
| stays not resolved | connection: shared structure persisting |
| stays resolved with one sign | freedom: independent variation persisting |
| does neither | reorganization: the structure changes |

κ_δ = resolved / evaluated minors is a declared summary (C), not the object.
The pattern is unchanged by edge order, positive component rescaling and
offsets; it depends on the components declared through M.

### 21.4 The three ground variables

| Variable | Invariant | What it reads | Form, held over the sequence | Function |
|---|---|---|---|---|
| **Energy** | I-T | how much a relation changes | ε[e] = \|A(c)[e]\| of the declared carrier c; δ_ε[e] = δ_c[s] + δ_c[t]; floor resolved when ε > δ_ε; degree ε/δ_ε | `energy`, `energy_exchange` *Amended 2026-11-21 (7th): energy is what is exchanged across a relation; its change is read through PC with the carrier as a component (§21.6).* |
| **Freedom** | I-S | how many independent ways relations vary | a pair is free when at least one minor is resolved with one sign throughout; F_δ = free / evaluated pairs | `freedom` *Amended 2026-11-21 (7th): freedom is change permitted, PC1–PC3; the form in this row reads a persistent arrangement, structure, kept as `v2_held_arrangement` (§21.6).* |
| **Connection** | I-E | what holds together through evolution | **identity**: A[e] − A[f] not resolved throughout; **proportional**: every minor not resolved throughout; **sustained**: sign of the circulation Ξ[e] = Σ_succ B − Σ_pred B resolved and the same throughout | `connection` *Amended 2026-11-21 (7th): also the coupled-change grade, a persistent ratio of changes (PC2) (§21.6).* |

Energy has its own carrier, flows whether or not anything reorganizes, and is
not multiplied by connection; the connected exchange (V3b) reads ε at edges
with held circulation and a resolved floor. Freedom and connection are
complementary per minor at δ; energy moves minors toward or across zero. Only
the sustained grade of connection needs an interior; energy, freedom and the
identity and proportional grades are readable at every region, Primary
included. A zero forced by the declaration is an identity, not a reading.

The V9 readers (`epsilon_e`, `superposition_condition`, `entanglement_condition`,
`tunneling_floor_holds`) are unchanged and are behind the V9 fence; V10 readings
use the forms above.

### 21.5 V10 admissibility and the V9 fence (W13)

V10 work uses only V10 forms. A retired or superseded element enters a V10
result only as a declared historical comparison (C), never as an input. The
retired and superseded V9 elements (the ρ-weighted operators, the superseded
readers, and `diagnostic_exec`, `provenanced_exec`, `admission`) compile only
with the Cargo feature `v9`, off by default. A V10 call to one does not
compile; V9 work opts in visibly with `features = ["v9"]`. Redoing a repository:
`docs/kernel/v10_migration.md`; template: `examples/v10_relational_field.rs`.

### 21.6 Structure is persistence; change is freedom (Origin, 2026-11-21, 7th)

Restates `V10_README.md` §2.0 and §2.5; where they differ, that file governs.

Change is read between consecutive registrations, Δ = A_later − A_earlier
(A_p). Change not resolved at δ is **structure** (persistence: change held by
constraint); change resolved is **freedom** (change permitted). Three levels:
the relation (PC1), the coupling of change between two relations (PC2: locked
in proportion → shared structure; independent → freedom), and the arrangement
of two relations (PC3). Shares F_δ + S_δ = 1 at each level.

- **Superposition (freedom):** change permitted, PC1–PC3. This restates V2;
  the first V2 form (a minor held resolved) reads a persistent arrangement,
  structure, and is kept as a declared comparison.
- **Entanglement (connection):** shared structure held by a constraint, in
  state (identity, proportional, sustained) or in change (coupled change).
- **Tunneling (energy):** what is exchanged across a relation, ε = |A(c)|; a
  carrier registered at the same loci is a component, read by PC1 and PC2(a).
- **Edge-valued registration (ER-1):** a relation registered as a relation
  enters as its value; node values only where the record registers states.

The full chain from observable to reported number: `V10_README.md` §2.0.
Code: `src/persistence_change.rs`.

---

## 22. Version History (Summary)

| Version | Key change |
|---|---|
| V5 | Foundation statement; admissibility as provenance condition; persistence edges; EdgeProvenance; PersistenceState with cold-start |
| V6 | Primary kernel E_primary = Σ(Δ(x)); ring inadmissibility derived |
| V7 | Unified file; NodeField subsumes ObservableField; sequential observation and evolution direction requirements; legacy intrusion warning |
| V7 (purge) | SVD, nalgebra, rank(Im Δ/Σ), ρ_P, FailureMode::DifferentiationCollapse/CirculationCancellation, frobenius_norm removed or suspended; cc = 0.5 flagged |
| V7.1 | Arbitrary values removed from operator bodies; ρ_base, χ₀, cc M-supplied; no kernel defaults |
| Kernel V8 (2026-09-26) | O→M→D statement; relation declared through M from observed distinction; declared-information rule; Declaration Correspondence; not-evaluated vs 0.0; persistence revised (A-1 CLOSED: PersistenceState::cold_start removed); DRIFT SIGNAL defined; vocabulary aligned |
| Kernel V8 (2026-09-27) | A-11 CLOSED: ι at photon locus not declared through M; legacy import SF-PR-16; ε[e] = hc/λ stands on own M mapping |
| derived_invariants V2 | m[v] → ι[v] throughout; SF-PR-16 |
| derived_invariants V3 | Photon edge functions; H_PLANCK, C_DECLARED as declared M-mapping references |
| derived_invariants V4 | I-T, I-S, I-E, I-RE added; joint necessity preamble |
| derived_invariants V4.2 | I-S suspended; rank/ρ_P suspended; R_anti→𝟙[e]→I-T/I-E audit — chain intact |
| derived_invariants V4.3 | I-S redeclared as relational distinguishability; OC-S-1 CLOSED; superposition_condition reinstated (no SVD) |
| derived_invariants V4.3.1 | Grounding reference updated to operators.rs V7.1 |
| Kernel V8 (derived_invariants) | Wording synchronized; A-11 closed; photon_edge_kappa_non_applicable rewritten; 𝟙[e] audit HOLD; are_proportional HOLD |
| Operational discipline (2026-09-29) | Sections 2, 4, 7, 16 added (Origin declaration): observational order and governing constants; task scope and frozen-kernel rule; observable record vs. legacy interpretation; experimental execution discipline. Section 5 (Workflow Step 2) revised: declaration-completeness vs. correspondence distinction added; execute-or-stop-exactly rule added. Section 10 (Kernel Selection) revised: R-is-not-optional rule added; Primary Region clarification added. No operator form, invariant form, Rust function, or source file content changed. |
| Kernel V9 synchronization (2026-10-25) | Kernel V9 established as the sole canonical release across all files (Origin directive, README `e54fcec`). Release designations updated; foundational-primitive statement added to the crate root, `operators.rs`, and governing kernel documents; RPPI enforcement status register added to §18. No operator form, invariant form, Rust function, or test expectation changed. |
| Kernel V9 (2026-10-24) | Foundational primitive established as Section 1 preamble: "a directed relation declared from observable information" — a primitive not previously existing in mathematics. Three roots stated: complexity, change as the only observable constant, bounded domain of analysis. Operator dependence on the primitive stated explicitly: operators exist because of the primitive and cannot be applied to anything not declared through M. Section 2 updated to reference Section 1 for the full account. No operator form, invariant form, Rust function, or source file content changed. |
| Kernel V9 diagnostic execution (2026-11-07) | Origin directive. `src/diagnostic_exec.rs` added (§11.7): thin A → B → R and persistence execution wrapper over the unchanged operators for experimental fields; mandatory edge provenance with observed/proposed support; non-blocking warnings and blocking errors; observation succession by declared process step. §8 ring/torus clarification (relational evolution, not cyclic relationships); §9 synchronized with the Origin general values ρ_base = 0.5, χ₀ = 1 (`operators.rs` header, `77c65ae`); §10 experimental-field note; §18 A-3 row annotated. No operator form, parameter handling inside an operator, or existing test expectation changed. |
| Kernel V10 Df6 (2026-11-21) | Origin declaration: all operators and ground variables occur simultaneously; composition order and registration sequence are orders of analysis; no phases. A relational field needs at least two consecutive registrations; one registration is insufficient information. `src/relational_field.rs` added (§11.6 note). §5, §8, §11.1 wording amended with the V9 text retained. No operator form or existing test expectation changed. |
| Kernel V10 release (2026-11-21) | Package `abr-kernel` `9.0.0` → `10.0.0` (Origin decision). Title and release designation updated. §1 Df1 restatement; §15.6 I-SC; §18 `are_proportional` and ρ max-selection rows updated; §21 added (ground variables, minor pattern, relational field). `src/ground_variables.rs`, `src/relational_field.rs`. No operator formula, V9 function signature or existing test expectation changed. |
| Kernel V10 ρ retirement (2026-11-21) | Origin declaration: ρ, χ, ρ_base, χ₀, the max selection, the saturating form and cc[p] retired from V10 readings; OC-K-1 dissolved. Parameter-free circulation Ξ added (`operator_circulation`). `RelationalField::new` takes no operator parameters. V9 ρ-weighted functions unchanged. |
| Kernel V10 PC (2026-11-21, 7th) | Origin declaration: structure is persistence, change is freedom (PC1–PC4); V2 restated, V8 re-read, I-SC-1 restated; coupled-change grade of connection; energy as a component; edge-valued registration ER-1; provenance chain (V10_README §2.0). `src/persistence_change.rs`. First V2 form kept as declared comparison |
| Kernel V10.2 release (2026-11-22) | Package `10.2.0`: PC and the provenance chain, W14, W15, τ = ħ/ε retired, Φ as a declared intersection, I-SC-D3 to I-SC-D13, step profile, relation transitions, the open-conditions decisions (V10_README Part 9, 20th and 21st). Merged to `main` with every commit kept |
| Kernel V10 W14 audit applied (2026-11-21, 12th) | hc/λ output Projected, wavenumber carrier; OC-T-1 restated; Bell and J[v] matches as cited intersections; §13.1 classes column; database source classing required; classical/quantum passage bounded; entanglement pointer. Original text kept |
| Kernel V10 W15 (2026-11-21, 11th) | Origin: time is never primitive; a registered time may enter as a declared intersection, described as such. Φ restored as a declared intersection (out of the V9 fence); τ = ħ/ε stays retired; the 10th's "no time" wording corrected |
| Kernel V10 no time (2026-11-21, 10th) | Φ = ι·τ retired from V10 (`phi_v`, `phi_v_log10`, `phi_s`, `accumulation_condition` behind the V9 fence); V5 restated as counted steps (`evolution_counts`); R4 window restated as a count of registrations; I-RE basis withdrawn |
| Kernel V10 τ retirement (2026-11-21, 9th) | Origin declaration: the τ = ħ/ε family is retired from V10 (time is a downstream projection, not a variable; no registration relates energy to an interval of evolution). `HBAR`, `tau_photon_edge`, `n_relational_cycles`, `tau_v`, `tau_v_isolated`, `tunneling_tau_bounded` behind the V9 fence. Φ_relational and κ[region] retired as identities. §13 text kept as record |
| Kernel V10.1 V9 fence (2026-11-21) | Origin declaration W13: V10 work uses only V10 forms. Cargo feature `v9`, off by default, gates the retired and superseded V9 elements (§21.5). Package `10.1.0`. Migration guide and end-to-end template added. No code removed. |

---

*The operators hold no agency. Responsibility stays with Origin.*  
*Bounded over D. No claim beyond D.*  
**Metatron Dynamics, Inc. Kernel V10.**
