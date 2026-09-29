# Kernel V8 — Complete Reference

**Metatron Dynamics, Inc.**  
Kernel V8 (2026-09-27). Operational discipline added 2026-09-29.  
Bounded over D. No claim beyond D.

This document is the single reference for any reader — human or AI — who needs
to apply, generate against, or verify work built on the kernel. It synthesizes
`lib.rs`, `operators.rs`, `derived_invariants.rs`, and `kernel_reference.md`
into one place. It introduces no declaration, operator, quantity, or
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
18. Open conditions held at Kernel V8
19. Suspended quantities (record only)
20. Legacy mathematical intrusion — classes and rule
21. Version history (summary)

---

## 1. Foundation

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

These are not hypotheses the kernel tests. They are the ground the kernel stands on. They cannot be argued with from within the framework, because the framework is built on them.

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
| **Application construction** | Declaring and executing the kernel over a particular observable system. Treats established Kernel V8 content as governing. |
| **Application verification** | Whether the application conforms to Kernel V8, the application's Origin declarations, its declared M, its declared topology, and its observable comparison criteria. Does NOT recursively re-audit Kernel V8. |

**Frozen-kernel rule (application construction and verification).** During application construction or application verification, do NOT reopen, rederive, independently justify, replace, reinterpret, or demand new observational proof for:

- established operator forms
- confirmed Part I invariant forms
- CLOSED findings
- established role constraints
- established admissibility rules
- established implementation behavior
- admitted measurement references
- or other Kernel V8 content explicitly classified as established

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
- **Before Phase 2:** declare the sequence of actual observations
  {M(o₁), …, M(oₙ)}, what one relational step is (the system's own process
  step), and whether the sequence is measured or from a declared model.
- **Before Phase 2:** declare the direction of relational evolution from an
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

Return located findings against the declared criteria (section 17). Once Verifier establishes that (1) the required application declaration exists, (2) it enters through the declared M, (3) the applicable established kernel operation was selected, (4) implementation matches that operation, and (5) no undeclared quantity entered — the mathematical provenance inquiry for that operation is complete. Proceed to observable comparison. Do not recursively investigate established Kernel V8 mathematics unless Origin explicitly requests a Kernel audit.

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

The source's legacy mathematical interpretation does NOT thereby enter Kernel V8. Do not import a Hamiltonian, probability distribution, statistical estimator, conventional force law, information-theoretic quantity, or other legacy construct merely because it appears beside the observational record.

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

---

## 9. Domain and Measurement

```
D := { x ∈ ℝⁿ | n < ∞, |x[i]| < ∞ ∀ i }
O → M → D
```

O: observables. M : O → D, declared by Origin before any operator acts.
A value in D is the numerical projection of an observable through M. The
kernel acts on M(o) only. Operator outputs are calculated results.

**M-declared operator parameters (V7.1).** No operator runs until M has
declared every value it uses. The kernel states forms only. It holds no
numerical value for any operator parameter and supplies no defaults.

| Parameter | Meaning | Status |
|---|---|---|
| ρ_base | Dimensionless ceiling on the weight given to directed imbalance of adjacent relations relative to a relation's own directed difference. Required wherever ρ is computed. | M-supplied (V7.1) |
| χ₀ | Reference in the saturating map ρ = ρ_base · χ / (χ₀ + χ). Carries the dimension of χ as declared through M. Required wherever ρ is computed. Must be finite and positive (χ₀ = 0 is not evaluable). | M-supplied (V7.1); OPEN — saturating form requires Origin declaration |
| cc[p] | Cross-topology coupling for each declared component pair p in R. Required when component pairs are declared; one value per pair. | M-supplied (V7.1) |

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
| E (Phase 1) | R(B(A(x)), ρ(A(x))) | `operator_e(f, rel, pairs, cc, rho_base, chi_0)` |
| A_persistence | E_current[e] − E_prior[e]; direction fixed | `operator_a_persistence(e_current, e_prior)` |
| B_persistence | B form over A_persistence | `operator_b_persistence(a_persistence, rel)` |
| R_persistence | R form over B_persistence; spatial component only | `operator_r_persistence(b_persistence, rel, rho_persistence)` |
| E (Phases 1+2) | Phase 1, then A_p → B_p → R_p | `operator_e_v5(f, rel, pairs, cc, persistence_state, rho_base, chi_0)` |
| ρ | ρ[i] = ρ_base · χ[i] / (χ₀ + χ[i]); χ[i] = max \|A(x)[e]\| or \|Δ(x)[e]\| over edges incident to i; edge form ρ[e] = ρ[source(e)] | `compute_rho`, `compute_rho_primary`, `compute_rho_persistence` |
| C | Declared projection: state what it preserves and discards. Not a kernel operator. | — |

### 11.2 ρ Status

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

| Constant | Value | Observable provenance |
|---|---|---|
| `HBAR` | 1.054_571_817e-34 J·s | Spectroscopic frequency-energy measurement |
| `E_UNIT` | 1.602_176_634e-19 C | Millikan oil-drop and subsequent direct measurements |
| `I_UNIT_ELECTRON` | 9.109_383_701_5e-31 kg | Penning trap oscillation frequency under declared non-acceleration |
| `H_PLANCK` | 6.626_070_15e-34 J·s | Spectroscopic frequency-energy measurement; enters D only inside M mapping hc/λ → ε[e] at Atomic Region photon edges |
| `C_DECLARED` | 2.997_924_58e8 m/s | BIPM 2019 interferometric measurement; enters D only inside M mapping hc/λ → ε[e] at Atomic Region photon edges; not a named framework variable |

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

Kernel V8 HOLD: depends on the 𝟙[e] audit (A-3 open condition). No change.

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

Kernel V8 AUDIT PENDING: determine whether 0.0 always means an evaluated R
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

### 13.9 Φ[v] — Coherence Potential (Node)

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

### 13.10 Φ[S] — Coherence Potential (Collective)

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
| Φ[v] coherence potential | ι[v] · τ[v] | `phi_v`, `phi_v_log10` |
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

Open conditions: OC-T-1, OC-T-2, OC-T-3 (see section 18).

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

Open conditions: OC-S-2, OC-S-3 (OC-S-1 CLOSED).

```rust
pub fn superposition_condition(delta_field: &[Vec<f64>], tol: f64) -> bool {
    // tol is IMPLEMENTATION-ONLY NUMERICAL TOLERANCE
    // Returns true if at least one pair of declared edges has non-proportional
    // directed differences. O(n²·k). No SVD. No threshold.
    ...
}
pub fn superposition_resolved(has_interior: bool) -> bool { has_interior }
```

Kernel V8 HOLD on `are_proportional`: edge-order dependent when one Δ vector
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

Open conditions: OC-E-1, OC-E-2, OC-E-3.

```rust
pub fn entanglement_condition(indicator: f64) -> bool { indicator == 1.0 }
pub fn decoherence_condition(indicator: f64, epsilon_e: f64, tol: f64) -> bool {
    // tol is IMPLEMENTATION-ONLY NUMERICAL TOLERANCE
    indicator == 0.0 && epsilon_e.abs() <= tol
}
```

### 15.5 I-RE — Relational Evolution Rate Invariant

Relational evolution rate decreases monotonically from the Primary Region
outward. Primary Region is the fastest-evolving declared region in D. Φ[v]
78-decade span is the observable expression of this invariant. Larger-scale
structure is downstream accumulation of Primary Region relational evolution.

Open condition: OC-RE-1.

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

## 18. Open Conditions Held at Kernel V8

| Item | Where | Status |
|---|---|---|
| A-3 — empty adjacency in Σ, R, R_persistence | `operators.rs` | OPEN — not classified |
| 𝟙[e] interpretation audit; ε[e] downstream | `derived_invariants.rs` | HOLD |
| A-11 — ι at photon locus not declared through M | `derived_invariants.rs` | CLOSED — legacy import (SF-PR-16); declared-information rule applies |
| `are_proportional` edge-order dependence and two-unit tolerance | `derived_invariants.rs` | HOLD |
| ρ max-selection rule | `operators.rs` | OPEN — Origin declaration required |
| ρ saturating form | `operators.rs` | OPEN — Origin declaration required |
| Part II "absence" wording | I-E, joint necessity, photon J[v] | Separate audit |
| R² = 0.527 at the δι fit | `observable_variable_sets.md` | HOLD |
| J[v] wording on what M observes | `derived_invariants.rs` | Origin wording decision |
| OC-T-1 | Tunneling floor magnitude: formal expression not yet derived | OPEN |
| OC-T-2 | Tunneling at scale: confirm beyond Primary and Atomic regions | OPEN |
| OC-T-3 | Tunneling necessity: formally derive that absence precludes energy transfer | OPEN |
| OC-S-1 | Kernel boundary criterion | CLOSED (2026-09-23: structural interior locus criterion) |
| OC-S-2 | Relational distinguishability beyond Primary Region | OPEN |
| OC-S-3 | Formally derive absence precludes self-organization | OPEN |
| OC-E-1 | 𝟙[e] = 1 preservation under operator evolution | OPEN |
| OC-E-2 | Decoherence rate derivation | OPEN |
| OC-E-3 | Entanglement necessity | OPEN |
| OC-RE-1 | Formally derive monotonic decrease of relational evolution rate | OPEN |
| OC-R-1 | g-factor derivation: g = 2·ρ[e_B]/ρ_base | OPEN |
| OC-θ-3 | Resonance / partial bond order | OPEN |
| OC-θ-5 | Full 3D vector balance N > 4 | OPEN |
| OC-Φ-1 | Formal derivation of Φ[v] from operator fixed-point | OPEN |
| OC-Φ-2 | Region-specific stable threshold from operator fixed-point | OPEN |
| OC-Φ-6 | Observer frame overlap — structural or contingent | OPEN |
| OC-CA-1 | Φ[S] formal derivation from operators | OPEN |
| OC-22 | α_r inter-region coupling formal derivation | OPEN |
| OC-ι-1 | ι_unit structurally fundamental or instrumentally bounded | OPEN |
| OC-ι-2 | Relational inertia ratios (ι[μ]/ι[e], ι[τ]/ι[e]) — no declared relational origin | OPEN |
| OC-ι-3 | δι ~ ι^0.5 scaling — connection to ρ saturation not derived | OPEN |

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

## 21. Version History (Summary)

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

---

*The operators hold no agency. Responsibility stays with Origin.*  
*Bounded over D. No claim beyond D.*  
**Metatron Dynamics, Inc. Kernel V8.**
