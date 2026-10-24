# Kernel V9 — Specification (Second Milestone)

**Metatron Dynamics, Inc.**  
Kernel V9 specification. Second milestone: Relational Primitive Provenance Invariant.  
Bounded over D. No claim beyond D.

---

## Purpose

This document is the governing specification for Kernel V9 development.

The **second V9 milestone** establishes the Relational Primitive Provenance
Invariant (RPPI) — the governing requirement that every admitted mathematical
variable, quantity, operation, and result must have a continuous, verifiable
provenance path to an Origin-declared observable relation within D.

The first milestone (specification and conformance test suite, commit `1a54d08`)
documented the executable relation definition and the provenance survival gap.
This milestone names the invariant, distinguishes provenance identity from
provenance derivation, defines the four RPPI failure modes, and establishes the
IRRS rejection requirement.

Work proceeds on the `kernel-v9` branch. No change is merged to `main` without
Verifier review.

---

## Verifier Disposition — Milestone 2 (2026-10-24)

**Status:** Accepted. Specification and gap documentation complete.

**RPPI enforcement:** OPEN. No operator changes authorized by this review.

**Architectural principle governing Milestone 3:**

> Every admitted result must be traceable through every contributing mathematical
> input to declared observable relations. A complete provenance record establishes
> derivation; a provenance label merely describes it.

**Vocabulary established by this disposition:**

| Term | Definition | Example |
|---|---|---|
| **Provenance label** | Describes a category of relation. Does not constitute a derivation path. Does not establish that any admitted result traces to a declared observable. | `EdgeProvenance::Continuation` — names a class; not a record. |
| **Provenance record** | Complete enough that every admitted result can be traced through every contributing mathematical input to a declared observable relation. Establishes derivation; does not merely name it. | `ObservableRelationDeclaration` with `observable_source`, `direction_basis`, `m_mapping_ref` — the target of Milestone 3. |

This distinction guards against undeclared abstractions entering the mathematical
treatment without prohibiting useful, simple numerical representations. A variable
need not be complex; its representation can be simple, provided its derivation from
the observable relational primitive is established through a complete record, not
asserted through a label.

**Next milestone:** Origin declaration of the relational identity and derivation
requirements, followed by a separate V9 admission interface design. See §10 for
the updated milestone order.

---

## 1. V8 Preservation Contract

The following are **frozen**. They must not be modified in any V9 milestone:

- Operator formulas: `operator_delta`, `operator_sigma`, `operator_a`,
  `operator_b`, `operator_r`, and all persistence variants.
- `DeclaredRelations` field layout and constructor behavior.
- `EdgeProvenance` variants (Continuation, Coupling, Persistence) and their
  admissibility semantics.
- `NodeField` finite-projection enforcement.
- `PersistenceState::from_prior` — no cold-start constructor added.

Conformance: all `[V8-BASELINE]` tests in `tests/v9_conformance.rs` must
pass without modification to operator code.

---

## 2. Relational Primitive Provenance Invariant (RPPI)

**The governing requirement:**

> Every admitted mathematical variable, quantity, operation, and result must
> have a continuous, verifiable provenance path to an Origin-declared observable
> relation within D.

A variable need not itself be complex. Its mathematical representation can be
simple, provided its derivation from the observable relational primitive is
established.

**Acceptance criterion:**

> No mathematical result can be admitted solely on numerical correctness,
> structural validity, or named provenance. Its derivation must terminate in
> declared observable relations.

This invariant is constitutive, not supplementary. The foundational primitive is
"a directed relation declared from observable information." Provenance is not an
annotation applied after construction — it is what constitutes membership in D.
A relation without provenance identity is not a declared relation; a variable
without provenance derivation is a number, not a projection through M.

**Two distinct requirements:**

| Requirement | Question answered | What it is NOT |
|---|---|---|
| **Provenance identity** | Which observable relation was declared, through which M, with which direction basis? | An `EdgeProvenance` class (Continuation, Coupling, Persistence) is NOT a provenance identity. It is a provenance label — it describes a category; it does not identify a declared observable. |
| **Provenance derivation** | How does this variable or result trace back to that identity, through every contributing mathematical input and through M and authorized operators only? | Numerical correctness is NOT derivation. Structural validity is NOT derivation. A provenance label is NOT derivation — it describes but does not establish. |

Both requirements must be satisfied for admission. A result that satisfies only
one is inadmissible.

**The record/label boundary (Verifier disposition, 2026-10-24):**

A **provenance record** establishes derivation: it is complete enough that every
admitted result can be traced through every contributing mathematical input to
declared observable relations. A **provenance label** merely describes a category.
`EdgeProvenance` is a label. `ObservableRelationDeclaration` must be a record.

---

## 3. Provenance Identity — ObservableRelationDeclaration

An admitted observable relation is identified by an `ObservableRelationDeclaration`
(working name — subject to Verifier review before implementation). This is the
formal record of an Origin-declared observable relation within D.

Under V9, every edge in `DeclaredRelations` carries one.

**Required fields:**

| Field | Description | Status |
|---|---|---|
| `observable_source` | A named reference to the specific observable or physical measurement from which this relation was declared through M. Not a class — a specific named declaration (e.g., "H-alpha spectroscopic edge, loci i=3 → i=4, declared 2026-09-26", not "Continuation"). | **OPEN** — returned to Origin for declaration |
| `direction_basis` | A named reference to the observable basis that establishes which end is source and which is target. Not index order. | **OPEN** — returned to Origin for declaration |
| `m_mapping_ref` | A reference to the declared M mapping that projects this observable into D. Required for fabrication detection: an `observable_source` that cannot be verified against a declared M is inadmissible. | **OPEN** — returned to Origin for declaration |

These three fields together constitute the provenance identity of the relation.

**Without an `ObservableRelationDeclaration`:** a relation carries a class but
not an identity. It is not an admitted observable relation — it is a fabrication
within D. Its loci have no standing. Any variable derived from it has no
derivation path to a declared observable.

**Current state:** `EdgeProvenance` records the class only. The three fields
above do not exist in the current API. Their precise representation — struct
definition, field types, constructor changes — is an implementation decision
returned to Origin as a declaration requirement (§7, OPEN Conditions).

---

## 4. Provenance Derivation Contract

Provenance derivation is the chain from a variable, quantity, or result back
to its provenance identity, through every contributing mathematical input,
through M and authorized operators only.

**Every contributing mathematical input** means the derivation path must be
complete. For an operator output `Δ(x)[e]`, traceability requires:
- The provenance identity of edge `e` (which observable relation was declared), and
- The declared M-mapping origin of each NodeField value (`x[src]` and `x[tgt]`)
  that was the numerical input to the operator.

It is not sufficient to record edge provenance if the NodeField values that
were operated on are themselves ungrounded — numbers admitted without a declared
M-mapping origin. The derivation path must terminate in declared observable
relations at every input, not only at the edge declaration.

**A derivation path is continuous** when every step is either:
- A declared M mapping (O → D), or
- An authorized operator application (Δ, Σ, A, B, R) acting on values that
  are themselves already grounded through declared M mappings.

**A derivation path is broken** when it passes through any value whose origin
is asserted rather than traced. This includes:
- Operator outputs that carry no reference to the provenance of their inputs
  (the current state — documented in §5.3).
- NodeField values declared by numerical assignment without a declared M-mapping
  origin (currently: `NodeField::new` enforces finiteness only, not M-mapping
  origin — an OPEN condition, see §7).
- Results composed from inputs of mixed relational origin without a record
  of which origin contributed each part.

### Current state (V8)

| Output type | Carries provenance identity? | Carries derivation reference? |
|---|---|---|
| `PrimaryEdgeField` (Δ, Σ output) | No | No |
| `EdgeField` (A, B, R output) | No | No |
| `DeclaredRelations` | Class only (identity: OPEN) | N/A — this is the declared source |

No operator output can currently satisfy the derivation requirement from its
type alone. The verifier bridges the gap by retaining `(DeclaredRelations,
output)` as a pair — but this is external coupling, not derivation carried in
the output.

### V9 requirement

Each operator output must carry a derivation reference enabling tracing of
every output entry back to the provenance record of the edge(s) and the
M-mapping origin of every input value that produced it:

| Output | Derivation requirement |
|---|---|
| `Δ(x)[e]` | Provenance record of edge `e`; declared M-mapping origin of `x[src]` and `x[tgt]` |
| `Σ(Δ(x))[e]` | Provenance record of edge `e` and its adj⁺/adj⁻ contributing edges; M-mapping origins of all contributing `x` values |
| `A(x)[e]` | Provenance record of edge `e`; declared M-mapping origin of each `x[i]` that contributed |
| `B(A(x))[e]` | Provenance records and M-mapping origins of all A-output entries that contributed |
| `R(B(A(x)))[e]` | Provenance records and M-mapping origins of all B-output entries that contributed |

**V9 adds** `ProvenancedOutput<T>` (working name) — a wrapper over the existing
field types that carries an edge-indexed reference to the `ObservableRelationDeclaration`
of each contributing edge:

```
ProvenancedOutput<T> {
    output: T,                                      // PrimaryEdgeField or EdgeField (unchanged)
    declarations: &ObservableRelationDeclaration,   // identity of each contributing edge
}
```

This does not change any operator formula. Existing operator functions are
wrapped; their internal computation is unchanged. The precise representation is
subject to Verifier review.

---

## 5. RPPI Failure Modes

Four failure modes violate the RPPI. Each requires detection and rejection.
The V8 boundary separating relational treatment from instrumentation and
independent validation is preserved: these are structural enforcement additions,
not new operator mathematics.

### 5.1 Missing Provenance

A relation or variable has no `ObservableRelationDeclaration`. The provenance
identity is absent.

- **Current state (V8):** `EdgeProvenance::Continuation` is accepted without an
  `observable_source`. Construction succeeds. The relation carries a class but not
  an identity.
- **V9 enforcement:** `from_edges_with_provenance` must reject when
  `ObservableRelationDeclaration` is absent for any edge. A class label alone is
  insufficient for admission.
- **Conformance:** `rppi_missing_observable_source_gap` (documents current state,
  `[RPPI-GAP]`); `rppi_missing_observable_source_rejected` (V9 contract, `[V9-REQUIRED]`,
  `#[ignore]` until enforcement implemented).

### 5.2 Fabricated Provenance

A relation carries an `ObservableRelationDeclaration` whose `observable_source`
or `m_mapping_ref` names an observable or M that was never declared by Origin.
The identity is asserted, not traced.

- **Current state (V8):** `EdgeProvenance` class is an enum variant. Any edge can
  carry any variant. There is no mechanism to verify that the class corresponds to
  an observable declared through M. Fabrication cannot be detected.
- **V9 enforcement:** Construction must verify that `m_mapping_ref` names an M
  declared by Origin. An `observable_source` that cannot be verified against a
  declared M mapping must be rejected.
- **Conformance:** `rppi_fabricated_provenance_gap` (documents current state,
  `[RPPI-GAP]`); `rppi_fabricated_observable_source_rejected` (V9 contract,
  `[V9-REQUIRED]`, `#[ignore]` until M-mapping verification implemented).

### 5.3 Disconnected Provenance

A result exists and is numerically correct, but its derivation path to the
declared observable relation that produced it is broken. Numerical correctness
and structural validity are satisfied; relational grounding is not.

This is the RPPI framing of the provenance survival gap documented in §4.
Disconnected provenance is not just a structural limitation — it is an RPPI
violation. A result that cannot establish its own derivation path does not satisfy
the invariant and must not be admitted to IRRS.

- **Current state (V8):** All operator outputs are disconnected in this sense.
  `PrimaryEdgeField` and `EdgeField` carry no derivation reference.
- **V9 enforcement:** IRRS must reject any result that does not carry a continuous
  derivation path through `ProvenancedOutput<T>`. Numerical correctness without a
  derivation path does not constitute admission.
- **Conformance:** `rppi_disconnected_provenance_gap` (documents current state,
  `[RPPI-GAP]`); `rppi_disconnected_provenance_rejected_by_irrs` (V9 contract,
  `[V9-REQUIRED]`, `#[ignore]` until ProvenancedOutput implemented).

### 5.4 Misattributed Provenance

A result carries a derivation record that attributes it to a declared observable
relation other than the one that actually produced it. The attribution is present,
internally consistent, and wrong.

- **Current state (V8):** No attribution mechanism exists. Misattribution cannot
  be detected because attribution cannot occur. The structural gap precedes the
  attribution gap: before a result can be attributed incorrectly, it must be
  possible to attribute it at all.
- **V9 enforcement:** `ProvenancedOutput<T>` carries a binding reference to the
  `DeclaredRelations` that produced it. IRRS verifies this binding during admission.
  A result presented with a substituted or exchanged declaration must be rejected.
- **Conformance:** `rppi_misattributed_provenance_gap` (documents current state,
  `[RPPI-GAP]`); `rppi_misattributed_provenance_rejected_by_irrs` (V9 contract,
  `[V9-REQUIRED]`, `#[ignore]` until attribution verification implemented).

---

## 6. Admissibility Enforcement Contract

The following admissibility conditions are currently enforced at construction.
They must be preserved in V9:

- `from_edges_with_provenance` panics if provenance count ≠ edge count.
- `from_edges_with_provenance` panics if any node index ≥ `n_nodes`.
- `from_edges_with_provenance` panics if any spatial edge carries
  `EdgeProvenance::Persistence` (persistence loci are edge-valued, not node-valued).
- `NodeField::new` panics if any value is non-finite (not in D).
- `require_rho_declaration` panics if `ρ_base` is non-finite or `χ₀` ≤ 0.

**V9 adds enforcement for (per RPPI §5):**

| Failure mode | Enforcement point | Enforcement action |
|---|---|---|
| Missing (§5.1) | `from_edges_with_provenance` | Reject when `ObservableRelationDeclaration` absent for any edge |
| Fabricated (§5.2) | `from_edges_with_provenance` | Reject when `observable_source` or `m_mapping_ref` unverifiable against declared M |
| Disconnected (§5.3) | IRRS admission function | Reject any result not wrapped in `ProvenancedOutput<T>` |
| Misattributed (§5.4) | IRRS admission function | Reject `ProvenancedOutput<T>` whose derivation binding does not match producing `DeclaredRelations` |

---

## 7. Open Condition Preservation Contract

OPEN conditions must propagate explicitly. No assumed value may be substituted.

The following OPEN conditions are carried from V8:

| Condition | Status | Location |
|---|---|---|
| ρ max-selection declaration | OPEN — Origin declaration required | `operators.rs`, ρ provenance audit §2 |
| ρ saturating form declaration | OPEN — Origin declaration required | `operators.rs`, ρ provenance audit §3 |
| A-3 empty adjacency: Σ, R, R_persistence sums over empty declared sets evaluate to 0.0 | OPEN — Origin disposition pending | `operators.rs`, Kernel V8 notes |

V9 adds OPEN conditions from the RPPI and Verifier disposition:

| Condition | Added in | Notes |
|---|---|---|
| `observable_source` field definition | V9 spec §3 | Name, type, admissibility rules, and constructor requirements |
| `direction_basis` field definition | V9 spec §3 | Name, type, admissibility rules, and what constitutes a valid direction declaration |
| `m_mapping_ref` field definition | V9 spec §3 | How an M mapping is named and verified against Origin declarations |
| `ObservableRelationDeclaration` structure | V9 spec §3 | Full struct definition: name, fields, constructor, invariants, V8 interface integration |
| `ProvenancedOutput<T>` structure | V9 spec §4 | Full wrapper definition: name, fields, constructor, binding invariants |
| NodeField provenance declaration | V9 spec §4, Verifier disposition | How each NodeField value declares its declared M-mapping origin. Currently `NodeField::new` enforces finiteness only. Under the complete RPPI, the M-mapping origin of each value must also be declared — not just that the value is finite, but that it is a projection through a named M from a declared observable. |
| V9 admission interface definition | Verifier disposition | The IRRS admission function design: what it accepts, what it verifies, what it returns or rejects. Separate from ProvenancedOutput<T>; designed before implementation. |

None of these is closed by the Generator. They require Origin declaration.

---

## 8. Conformance Test Contract

The test suite in `tests/v9_conformance.rs` must demonstrate:

| Category | Tag | What is tested |
|---|---|---|
| V8 baseline | `[V8-BASELINE]` | Existing operator behavior unchanged. Any operator code change that fails a baseline test violates the V8 preservation contract. |
| Executable relation | `[V8-BASELINE]` | Provenance required at construction; out-of-range nodes rejected; Persistence excluded from spatial list; finite projection enforced. |
| Provenance survival gap | `[GAP]` | Documents that `PrimaryEdgeField` and `EdgeField` carry no provenance reference. Not a failure — a documented gap that V9 closes. |
| Open condition | `[V8-BASELINE]` | `PersistenceState` requires a real prior — no cold-start constructor. |
| V8 continuity | `[V8-BASELINE]` | Full Δ→Σ and A→B→R chains produce finite output on valid declarations. |
| IRRS tracing gap | `[GAP]` | Documents that tracing from operator output to observable provenance requires externally retaining `DeclaredRelations`. |
| RPPI — missing provenance | `[RPPI-GAP]` | Class-only provenance accepted without observable identity. Documents inadmissible current behavior. |
| RPPI — missing provenance | `[V9-REQUIRED]` | Construction without `ObservableRelationDeclaration` must be rejected. `#[ignore]` until enforcement implemented. |
| RPPI — fabricated provenance | `[RPPI-GAP]` | Enum variant accepted without M-mapping verification; fabrication undetectable. |
| RPPI — fabricated provenance | `[V9-REQUIRED]` | Unverifiable `observable_source` must be rejected at construction. `#[ignore]` until M-mapping verification implemented. |
| RPPI — disconnected provenance | `[RPPI-GAP]` | Operator result is finite and numerically correct; carries no derivation path to its declared observable. |
| RPPI — disconnected provenance | `[V9-REQUIRED]` | IRRS must reject results without continuous derivation path. `#[ignore]` until `ProvenancedOutput<T>` implemented. |
| RPPI — misattributed provenance | `[RPPI-GAP]` | No attribution mechanism exists; substituting one relation's output for another's is undetectable. |
| RPPI — misattributed provenance | `[V9-REQUIRED]` | IRRS must reject `ProvenancedOutput<T>` whose binding is wrong. `#[ignore]` until attribution verification implemented. |

**Test counts:**
- 27 existing tests (`[V8-BASELINE]` and `[GAP]`) — all must pass.
- 4 new `[RPPI-GAP]` tests — document inadmissible current behavior; must pass.
- 4 new `[V9-REQUIRED]` tests — define V9 enforcement contract; `#[ignore]` until
  implementing milestones are verified. Remove `#[ignore]` and confirm all 35 tests
  pass after each V9 enforcement milestone.

---

## 9. IRRS Integration Contract

The Verifier must be able to trace results to relational observations without
relying on the Generator's assertions. This requires:

1. The output type carries sufficient information to identify the provenance
   identity of every contributing edge — addressed by `ProvenancedOutput<T>` (§4).
2. The provenance class is supplemented by the full `ObservableRelationDeclaration`
   (observable source, direction basis, M-mapping reference) — addressed by §3.
3. The direction of each edge is grounded in a declared observable basis, not
   index order — addressed by §3 (`direction_basis` field).

**Current state:** The verifier can trace results by retaining `(DeclaredRelations,
output)` as a pair. The `[GAP]` tests in `tests/v9_conformance.rs` document this
explicitly.

**V9 requirement:** The output type alone is sufficient for tracing. The IRRS
does not retain `DeclaredRelations` separately.

**V9 rejection requirement (per RPPI §5):**

The IRRS must reject admission of any result that:
- Cannot be traced to an `ObservableRelationDeclaration` — missing or disconnected
  provenance (§5.1, §5.3).
- Carries an `observable_source` that cannot be verified against a declared M
  mapping — fabricated provenance (§5.2).
- Carries a derivation record that does not bind to the `DeclaredRelations` that
  produced it — misattributed provenance (§5.4).

The IRRS does not admit results based on numerical correctness, structural
validity, or provenance class alone. Derivation must terminate in declared
observable relations.

---

## 10. Development Constraints

- All V9 work is on the `kernel-v9` branch.
- No V9 addition merges to `main` without Verifier review.
- V8 operator forms are not changed.
- OPEN conditions are not closed by the Generator; they require Origin declaration.
- `ObservableRelationDeclaration`, `ProvenancedOutput`, `observable_source`,
  `direction_basis`, and `m_mapping_ref` are working names. The Verifier confirms
  or revises names before implementation.
- `[V9-REQUIRED]` tests are `#[ignore]` until the implementing milestone is
  verified. Remove `#[ignore]` and confirm all 35 tests pass after each V9
  enforcement milestone.
- The order of milestones:
    1. (Milestone 1, commit `1a54d08`) Specification + conformance test suite.
    2. (Milestone 2, commit `ee46735`) RPPI established; four failure modes defined;
       RPPI conformance tests added. Verifier disposition accepted 2026-10-24.
    3. **(Milestone 3 — next)** Origin declaration of relational identity and
       derivation requirements, and V9 admission interface design. Governed by:
       *Every admitted result must be traceable through every contributing
       mathematical input to declared observable relations.* Specifically:
       - Origin declares `ObservableRelationDeclaration` fields: `observable_source`,
         `direction_basis`, `m_mapping_ref` — as a complete provenance record, not
         a provenance label.
       - Origin declares NodeField provenance: how each NodeField value records
         its declared M-mapping origin (not only finiteness).
       - Origin declares the V9 admission interface: what it accepts, verifies,
         and returns or rejects — designed before any implementation.
    4. Implementation of `ObservableRelationDeclaration` enforcement at construction
       (closes RPPI failure modes: missing, fabricated).
    5. Implementation of `ProvenancedOutput<T>` wrapper with full through-every-input
       derivation record (closes RPPI failure modes: disconnected, misattributed).
    6. IRRS rejection enforcement for all four failure modes.
    7. IRRS integration verification.

---

*The operators hold no agency. Responsibility stays with Origin.*  
*Bounded over D. No claim beyond D.*  
**Metatron Dynamics, Inc. Kernel V9 — Second Milestone Specification.**
