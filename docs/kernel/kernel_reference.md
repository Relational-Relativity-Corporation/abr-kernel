# Kernel Reference

**Metatron Dynamics, Inc.** Kernel V8. Reference data. Bounded over D. No claim beyond D.

---

## Purpose and use

This document is a single reference to the kernel for any reader — a person
or an AI system — who needs to apply, generate against, or verify work built
on the kernel. It collects, in one place, the role structure, the workflow,
the operators and their Rust implementation, the derived invariants, the
verification requirements, the language discipline, and the items held open
at Kernel V8.

It introduces no declaration, operator, quantity, or admissibility rule of its
own. Every statement here restates a source file in this repository. **Where
this reference and a source file differ, the source file governs**, and the
difference is a finding to be returned to Origin.

| Topic | Source |
|---|---|
| Roles, workflow, Verification and Validation Protocol | `docs/kernel/role_separation_and_operator_application.md` |
| Operator notation and constraints | `docs/kernel/operators_notation_and_constraint.md` |
| Operator implementation | `src/operators.rs` |
| Derived invariants | `src/derived_invariants.rs` |
| Observational grounding | `docs/kernel/validation_record.md` |
| Variable sets by region | `docs/kernel/observable_variable_sets.md` |
| Verification methods and publication gate | `docs/kernel/triad-constraint-methods.md` |
| Verification procedure | `docs/process/` |

---

## 1. Foundation

**The chain.** Observable → M → numerical projection in D → declared topology
→ operators → calculated result → comparison with observables.

- D := { x ∈ ℝⁿ | n < ∞, |x[i]| < ∞ }. M : O → D is declared by Origin
  before any operator acts. The kernel acts on M(o) only.
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
  as such.
- **Declaration correspondence.** Analysis proceeds from declared observables
  without expectation of numerical outcome. Where the numerical projection
  produced by Δ or A does not preserve the observed distinction on which a
  declared relation is based, correspondence is not established; the
  declaration may be varied and evaluated again — topology first, with
  variable determination and M. The kernel does not vary the declaration.
  Topology is varied in response to comparison with observables, not to
  obtain a particular numerical result.
- **Relation.** A relation is declared through M from an observed
  distinction. Δ and A compute the numerical directed difference over a
  declared relation; the computation does not declare the relation.

---

## 2. Roles

Roles reconstruct from the constraints that must hold for the workflow to
function; they are not named into existence.

| Role | Function | Held by |
|---|---|---|
| **Origin** | Declares D, M, the relations and their provenance, and what is admissible. Disposes of findings. Holds responsibility. | The human. Not assignable to a frame; cannot be delegated without being vacated. |
| **Generator** | Proposes candidate structure within what Origin declared. | Human or AI. |
| **Verifier** | Detects divergence from declared criteria, from a frame distinct from the Generator. Returns located findings, not approval. | Human or AI, from an independent frame. |

**Constraints on the roles** (role_separation, "Constraints"):
1. No declaration, no evaluation.
2. Function defines role, not the label.
3. Description is not instantiation.
4. Responsibility stays with what has agency — the human.
5. Verification detects divergence; it does not approve.
6. Generation and verification require distinct frames.
7. Verification is collaborative; neither participant is presumed free of drift.
8. Scale does not alter declared roles.
9. Roles are declared independently of implementation.

The operators hold no agency. Whatever the workflow commits to, a person
committed it.

---

## 3. Workflow

1. **Declare** (Origin). D and M; the relations and their source; each
   relation's single admissible direction, traceable to an observable
   through M; independent provenance for each direction where both are
   declared (distinctness axiom). No relation fabricated; no boundary closed
   to supply continuation that was not declared. If no declaration is
   admissible, stop — the operators are undefined, not approximate.
   - **Before Phase 2:** declare the sequence of actual observations
     {M(o₁), …, M(oₙ)}, what one relational step is (the system's own process
     step), and whether the sequence is measured or from a declared model.
   - **Before Phase 2:** declare the direction of relational evolution from an
     observable property of the process. Index order is not a direction
     declaration.
   - **For every analysis:** M declares the operator parameters ρ_base and χ₀
     (and cc[p] for each declared component pair). The kernel holds no value
     for any of them and supplies no default.
2. **Generate** (Generator). Apply the kernel selected by the declared
   structure (section 5). State every projection with what it preserves and
   discards.
3. **Verify** (Verifier). Return located findings against the declared
   criteria (section 8).
4. **Revise** (Origin). Findings return to Origin; the pass repeats until the
   Verifier verdict is PASS as defined in `triad-constraint-methods.md`, or
   Origin declines each outstanding refinement on the record.

---

## 4. Relational structure

- **Direction.** Every declared relation has exactly one admissible
  direction — the direction traceable to an observable through M. The
  reverse requires independent observable provenance; without it, it is
  inadmissible. If both (s,t) and (t,s) are independently observed and
  declared, they are distinct relations, not a symmetric pair.
- **Rings and tori are inadmissible.** A closing edge declares a node as its
  own relational predecessor. Any ring, torus, `np.roll`, or periodic-index
  operation is a DRIFT SIGNAL.
- **DRIFT SIGNAL:** legacy mathematics inserted into or before the operators,
  or otherwise undeclared.
- **Before Δ / Before A.** No transformation that alters pairwise differences
  precedes A or Δ unless declared within M. Admissible: uniform shift,
  declared unit scale. All else requires declaration with preserved and
  discarded invariants stated.
- **Edge provenance** (`EdgeProvenance`): `Continuation` (within-family
  spatial continuation), `Coupling` (cross-family spatial coupling),
  `Persistence` (relational-evolutionary; held in `PersistenceState`, never in
  the spatial edge list).

---

## 5. Operators

### Kernel selection — the full-operator admissibility condition

A → B → R is admissible when M establishes at least one **interior locus** t:
t = target(e) = source(f) for independently declared edges e and f, each with
provenance through M. The minimal admissible interior is s → t → u.

- **Interior not established** (M can declare the relation but cannot
  resolve its constituent loci): the **primary kernel** Δ → Σ applies. B is
  not evaluated — it is not an identity operator evaluated with no effect.
- **Interior established:** the **ABR kernel** A → B → R applies.

The condition is structural, not a numerical threshold. M establishes it; the
kernel does not compute it from `DeclaredRelations`. (It replaces the retired
ρ_P criterion.)

### Operator forms

| Operator | Form | Rust |
|---|---|---|
| Δ | Δ(x)[e] = x[s] − x[t], each declared edge e = (s, t) | `operator_delta(x, rel)` |
| Σ | Σ(g)[e] = g[e] + ρ[e]·( Σ_{adj⁺(e)} g − Σ_{adj⁻(e)} g ), g = Δ(x); immediate neighbours only | `operator_sigma(delta, rel, rho_base, chi_0)` |
| E_primary | Σ(Δ(x)) | `operator_e_primary(x, rel, rho_base, chi_0)` → (Δ, Σ) |
| A | A(x)[e] = x[s] − x[t] (formula identical to Δ); component pairs (a,b): x[a][i] − x[b][i] | `operator_a(f, rel, pairs)` |
| B | B(g)[e] = g[e] + Σ_{f ∈ succ(e)} g[f]; immediate successors only; terminal edges accumulate nothing | `operator_b(g, rel)` |
| R | R(g)[e] = g[e] + ρ[src(e)]·( Σ_{succ(e)} g − Σ_{pred(e)} g ); cross-topology term scaled by cc[p] | `operator_r(bg, rel, rho, cc)` |
| E (Phase 1) | R(B(A(x)), ρ(A(x))) | `operator_e(f, rel, pairs, cc, rho_base, chi_0)` |
| A_persistence | E_current[e] − E_prior[e]; direction fixed | `operator_a_persistence(e_current, e_prior)` |
| B_persistence, R_persistence | B and R forms over A_persistence | `operator_b_persistence`, `operator_r_persistence` |
| E (Phases 1 + 2) | Phase 1, then A_p → B_p → R_p | `operator_e_v5(f, rel, pairs, cc, persistence_state, rho_base, chi_0)` |
| ρ | ρ[i] = ρ_base · χ[i] / (χ₀ + χ[i]); χ[i] = max \|A(x)[e]\| or \|Δ(x)[e]\| over edges incident to i; edge form ρ[e] = ρ[source(e)] | `compute_rho`, `compute_rho_primary`, `compute_rho_persistence` |
| C | Declared projection: state what it preserves and discards. Not a kernel operator. | — |

**M-declared parameters.** `require_rho_declaration(rho_base, chi_0)` and
`require_cc_declaration(cc, pairs)` enforce that ρ_base is finite, χ₀ is
finite and positive, and one finite cc value is supplied per declared
component pair. Without them the operator is not evaluated.

### Persistence

Persistence requires a declared pair of sequential observations. At the first
declared observation, only the spatial kernel is evaluated. No prior value is
supplied. A `PersistenceState` can be constructed only from the kernel output
of a completed declared observation (`PersistenceState::from_prior`).

```rust
use metatron_kernel_v8::operators::*;

// Declared through M before any operator acts (the kernel supplies no values):
//   rel       — DeclaredRelations::from_edges_with_provenance(n, edges, provenance)
//   x1, x2    — NodeField::new(...) for two sequential declared observations
//   pairs, cc — declared component pairs and one cc value per pair
//   rho_base, chi_0 — declared by M for this analysis

// First declared observation: spatial kernel only.
let e1 = operator_e(&x1, &rel, &pairs, &cc, rho_base, chi_0);

// Second declared observation: persistence over the declared pair (E1, E2).
let prior = PersistenceState::from_prior(e1);
let (e2, persistence) = operator_e_v5(&x2, &rel, &pairs, &cc, &prior, rho_base, chi_0);
// persistence.a_persistence, .b_persistence, .r_persistence
```

### Structural checks

| Check | Rust |
|---|---|
| Relational isolation (every edge has empty adj⁺ and adj⁻; Σ's antisymmetric term is then structurally zero) | `relational_isolation(rel)` |
| Declared edge-image admissibility (no edge vector is the negation of another) | `declared_edge_image_admissibility_check(delta, tol)` |
| Antisymmetric term of Σ, extracted separately | `antisymmetric_term(delta, rel, rho_base, chi_0)` |

**Antisymmetric expression.** R_anti = 0 and R_anti ≠ 0 are both admissible
outcomes. R_anti ≠ 0 is a review signal — preserved exactly and examined
through the full provenance chain. It is not suppressed, thresholded, or
automatically classified as failure or confirmation.

**Tolerances.** Every `tol` argument is an IMPLEMENTATION-ONLY NUMERICAL
TOLERANCE guarding finite-precision arithmetic. It is not a property of the
observed system and is never reported as one.

---

## 6. Derived invariants (`src/derived_invariants.rs`)

Layer 3 quantities are calculated results, readable only after the operators
act. A Layer 3 result is compared with observables through M; its numerical
existence alone does not establish correspondence. "Derived" states how a
result was obtained; it does not make the result an observable.

### Part I — confirmed derived invariants

| Quantity | Form | Rust |
|---|---|---|
| ε[e] relational energy contrast | \|Δ(ι)[e]\| · 𝟙[e] | `epsilon_e(delta_iota_e, indicator)` |
| 𝟙[e] detection indicator | 1 iff R(A(x))[e] antisymmetric term ≠ 0 | `detection_indicator(r_anti, tol)` |
| ε[e] at photon edges | hc/λ (part of M at the Atomic Region; stands on its own M mapping — ι is not declared through M at the photon locus, so E[v] = ι[v]·κ[region] is not evaluated there) | `epsilon_photon_edge(lambda_m)` |
| τ[v] at photon edges | ℏ / ε[e] | `tau_photon_edge(lambda_m)` |
| N[v] at photon-emitting loci | τ_stability · ε[e] / ℏ | `n_relational_cycles(tau_stability_s, lambda_m)` |
| τ[v] unified | ℏ / \|A_persistence[e_coupling] · cos θ\| | `tau_v(a_p, cos_theta)`; isolated case `tau_v_isolated(e_v)` |
| θ[e] equilibrium bond angle | from Σᵢ v[eᵢ] = 0 (2, 3, 4 bonds) | `equilibrium_bond_angle_rad`, `_deg` |
| Φ[v] coherence potential | ι[v] · τ[v] | `phi_v`, `phi_v_log10` |
| Φ[S] collective | ι[S] · τ[S]; accumulation Φ[S] > Σᵢ Φ[vᵢ] | `phi_s`, `accumulation_condition` |
| J[v] R output multiplicity | (declared edges at v) / 2; states 2J+1 | `j_v`, `r_output_states` |

### Part II — proposed framework-level invariants (open derivation)

A Part II statement may not be cited as a proven consequence of the operators
and may not support a Part I derivation.

| Invariant | Statement | Rust | Open |
|---|---|---|---|
| I-T tunneling | ε[e] > 0 at every declared transition edge where M establishes a relational energy contrast | `tunneling_floor_holds`, `tunneling_tau_bounded` | OC-T-1–3 |
| I-S superposition | Relational distinguishability: some declared edges e, f have Δ(x)[e] and Δ(x)[f] not proportional; resolves structurally when M establishes an interior locus | `superposition_condition`, `superposition_resolved` | OC-S-2, OC-S-3 |
| I-E entanglement | Declared edge with 𝟙[e] = 1 establishes a relational constraint preserved under the operators | `entanglement_condition`, `decoherence_condition` | OC-E-1–3 |
| I-RE relational evolution | Relational evolution rate decreases monotonically from the Primary Region outward | — | OC-RE-1 |

### Measurement references

`HBAR`, `H_PLANCK`, `C_DECLARED`, `E_UNIT`, `I_UNIT_ELECTRON` are declared
measurement references. `H_PLANCK` and `C_DECLARED` enter D only inside the M
mapping hc/λ → ε[e] at Atomic Region photon edges; neither appears in any
operator formula.

### Observational status (scale-conditional)

- **Locus-resolved** (ℛ_M = 1, 𝓜_M = 1): constituent loci and relations are
  independently declarable through M.
- **Expression-observed, mechanism-unresolved** (ℛ_M = 1, 𝓜_M = 0): the
  observable record contains a traceable expression through M; the mechanism
  is not yet resolvable at that scale.
- "Not directly observable" is not used as a universal classification.

Support classifications: **Observed** (from M), **Inferred** (from observed
data under the declared framework), **Derived** (from operator mathematics).
A quantity computed from observed values by a declared reduction is a
**downstream projection**.

---

## 7. Language discipline

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

**Legacy mathematical intrusion.** Any mathematical quantity in Generator
output that was not declared by Origin as traceable through M before
generation began has provenance not observed or incomplete — whatever
framework it comes from and whether or not it computes correctly. Common
classes for AI Generators:

- **Statistical:** probability, expected value, variance, correlation,
  ensemble averages, Bayesian priors. These need a declared ensemble.
- **Quantum mechanical:** wave functions, Hamiltonians, density matrices.
- **Classical mechanical:** mass, force, potential energy, trajectories,
  phase space as primitives.
- **Information-theoretic:** entropy, mutual information, KL divergence.
- **Geometric:** manifolds, curvature, metric tensors assumed as background.

---

## 8. Verification

**Verifier criterion:** *"Can I trace every mathematical operation
continuously back to declared observables through declared operators?"*
Correctness is not the question. Provenance is.

**Four stages** (role_separation, "Verification and Validation Protocol"):

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

**Seven verification methods** (`triad-constraint-methods.md`): Claim
Provenance (CP), Comparison Scope Declaration (CS), Declaration–Implementation
Correspondence (DI), Cross-Artifact Consistency (CA), Invariant Range
Declaration (IR), Adversarial Gate (AG), Correspondence Register (CR).

**Procedure:** `docs/process/Verification_pass_protocol.md` (ordering a pass;
Verified / Inference / Not Verifiable); `docs/process/kernel_self_consistency_test.md`
(clean-room build and six required interface checks);
`docs/process/repository_protocol_v1.md` (repository workflow and publication).

---

## 9. Open and held at Kernel V8

| Item | Where | Status |
|---|---|---|
| A-3 — empty adjacency in Σ, R, R_persistence | `operators.rs` | OPEN — not classified |
| 𝟙[e] interpretation audit; ε[e] downstream | `derived_invariants.rs` | HOLD |
| A-11 — ι at photon locus not declared through M | `derived_invariants.rs`, `cross_region_energy_expression.md` | CLOSED — legacy import (SF-PR-16); declared-information rule applies |
| `are_proportional` edge-order dependence and two-unit tolerance | `derived_invariants.rs` | HOLD |
| ρ max-selection rule; saturating form | `operators.rs`, notation | OPEN — Origin declaration required |
| Part II "absence" wording | I-E, joint necessity, photon J[v] | Separate audit |
| R² = 0.527 at the δι fit | `observable_variable_sets.md` | HOLD |
| J[v] wording on what M observes | `derived_invariants.rs` | Origin wording decision |

---

## 10. Reading order

1. This reference.
2. `docs/kernel/role_separation_and_operator_application.md`
3. `docs/kernel/operators_notation_and_constraint.md`
4. `src/operators.rs`, then `src/derived_invariants.rs`
5. `docs/kernel/validation_record.md` and `observable_variable_sets.md`
6. `docs/kernel/triad-constraint-methods.md` and `docs/process/`

*The operators hold no agency. Responsibility stays with Origin.*
*Bounded over D. No claim beyond D.*

---

**Kernel V8 (2026-09-26):** Document created at Origin's request as a single
reference for people and AI systems. It restates existing declarations from
the files listed under "Purpose and use" and introduces no declaration of its
own.

**Kernel V8 update (2026-09-27):** A-11 closed. ι at the photon locus was an
import of the legacy quantity "photon rest mass = 0," whose definition requires
a frame with no declared relational motion — inadmissible in D for the same
structural reason that retired m[v] as a primitive (SF-PR-16). Under the
declared-information rule, ι is not evaluated at photon edges. ε[e] = hc/λ
stands on its own M mapping. Photon edge ε[e] entry in Part I table updated.
