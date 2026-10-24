# Kernel V9 — Specification (Third Milestone)

**Metatron Dynamics, Inc.**  
Kernel V9 specification. Third milestone: Origin declaration and admission interface design.  
Bounded over D. No claim beyond D.

---

## Purpose

This document is the governing specification for Kernel V9 development.

**Milestone history:**
- Milestone 1 (commit `1a54d08`): Executable relation definition and conformance test suite.
- Milestone 2 (commit `ee46735`): RPPI established; four failure modes defined; RPPI
  conformance tests added. Verifier accepted 2026-10-24.
- **Milestone 3 (this document):** Origin declaration accepted. Declaration record /
  calculation record distinction established. Two Verifier corrections incorporated:
  enforcement belongs to V9 admission interface (not frozen V8 constructors); IRRS
  must independently recompute or verify, not merely match names and references.
  Interface design OPEN — Rust types, admission outcomes, and conformance tests are
  the next deliverable. No operator modifications.

Work proceeds on the `kernel-v9` branch. No change is merged to `main` without
Verifier review.

---

## Verifier Disposition — Milestone 3 (2026-10-24)

**Status:** Origin declaration accepted. Interface design OPEN.

**Two corrections incorporated before interface implementation:**

**Correction 1 — Record authorship must not become self-certification.**
The Generator authors calculation records as an execution responsibility. But
IRRS must independently recompute or otherwise verify the recorded operations
and their bindings. Matching names and references alone cannot establish that
the recorded calculation actually occurred. The calculation record is evidence
to be checked, not a certified account to be trusted. (Governs §3, §10.)

**Correction 2 — Enforcement belongs to the V9 admission interface, not the frozen V8 constructor.**
The enforcement of ObservableRelationDeclaration presence (§6.1) and
m_mapping_ref verifiability (§6.2) applies to the V9 admission interface.
V8 constructors are frozen per Req-5. Placing enforcement at
`from_edges_with_provenance` would contradict Req-5. (Governs §6, §7.)

**Traceable chain required by interface design:**

> Declared observable relation → declared M → projected input →
> authorized operator execution → calculation record →
> independent IRRS verification.

The interface design must establish this chain as an enforceable
computational foundation, not only as a statement of mathematical intent.

**Next Generator deliverable:** Rust types, their relationships, admission
outcomes, and conformance tests. No operator modifications. No V8 changes.

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

---

## Origin Declaration — Milestone 3 (2026-10-24)

The following seven requirements are hereby authorized by Origin as governing
constraints for Kernel V9. They supersede and incorporate all prior OPEN conditions
on relational identity and derivation. No implementation proceeds without each
requirement either closed by Origin declaration or explicitly held OPEN.

**Req-1 Relational origin:** Every admitted value traces to one or more declared
observable relations within D. A numerical value is a representation, not an
independently admitted primitive.

**Req-2 Mapping identity:** Every projection identifies the M that produced it,
the relevant input relations, and its declared domain.

**Req-3 Direction:** Direction is established by a declared relational distinction,
not inferred from index ordering.

**Req-4 Complete derivation:** Every operator output retains the identities of all
contributing values, relations, mappings, and applicable declared parameters.

**Req-5 Admission boundary:** V9 admits complete derivation records through a
separate interface. Existing V8 constructors and operators remain unchanged.

**Req-6 Independent verification:** IRRS checks the actual derivation structure
against the declarations, rather than trusting a provenance label or the Generator's
account of execution.

**Req-7 Open conditions:** Missing or unverifiable provenance remains OPEN and
cannot be admitted as established.

**Simulated observations:** Simulated observations may be admitted, provided their
simulated origin is declared. A valid simulated derivation does not become empirical
evidence merely because its provenance is complete.

**Milestone 3 scope:** The next milestone produces an interface design and
executable conformance criteria. No operator modifications.

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

This is consistent with Req-5: V8 constructors and operators are unchanged.
V9 adds a separate admission interface alongside them, not replacing them.

Conformance: all `[V8-BASELINE]` tests in `tests/v9_conformance.rs` must
pass without modification to operator code.

---

## 2. Relational Primitive Provenance Invariant (RPPI)

**The governing requirement:**

> Every admitted mathematical variable, quantity, operation, and result must
> have a continuous, verifiable provenance path to an Origin-declared observable
> relation within D. (Req-1)

A variable need not itself be complex. Its mathematical representation can be
simple, provided its derivation from the observable relational primitive is
established.

**Acceptance criterion:**

> No mathematical result can be admitted solely on numerical correctness,
> structural validity, or named provenance. Its derivation must terminate in
> declared observable relations. (Req-7)

This invariant is constitutive, not supplementary. The foundational primitive is
"a directed relation declared from observable information." Provenance is not an
annotation applied after construction — it is what constitutes membership in D.
A relation without provenance identity is not a declared relation; a variable
without provenance derivation is a number, not a projection through M.

**Two distinct requirements:**

| Requirement | Question answered | What it is NOT |
|---|---|---|
| **Provenance identity** | Which observable relation was declared, through which M (Req-2), with which direction basis (Req-3)? | An `EdgeProvenance` class (Continuation, Coupling, Persistence) is NOT a provenance identity. It is a provenance label — it describes a category; it does not identify a declared observable. |
| **Provenance derivation** | How does this variable or result trace back to that identity, through every contributing mathematical input (Req-4), through M and authorized operators only? | Numerical correctness is NOT derivation. Structural validity is NOT derivation. A provenance label is NOT derivation — it describes but does not establish. |

Both requirements must be satisfied for admission. A result that satisfies only
one is inadmissible (Req-7).

**The record/label boundary (Verifier disposition, 2026-10-24):**

A **provenance record** establishes derivation: it is complete enough that every
admitted result can be traced through every contributing mathematical input to
declared observable relations. A **provenance label** merely describes a category.
`EdgeProvenance` is a label. `ObservableRelationDeclaration` must be a record.

---

## 3. Declaration Record and Calculation Record

Origin declares two distinct record types (Req-4, Req-6). They have distinct
authors, distinct contents, and distinct verification roles.

**Declaration record:** Identifies what Origin admitted as observable and how
it was mapped. Records the observable, the M that mapped it, the relevant
input relations, and the declared domain. Set at construction time, before
any operator runs. Origin is the author.

**Calculation record:** Records what the operators actually used and produced.
Records the actual input values used, the relations traversed, the operator
parameters applied (ρ_base, χ₀, cc), and the outputs computed. Generated at
computation time by the operators. The Generator is the author.

| Property | Declaration record | Calculation record |
|---|---|---|
| Author | Origin | Generator (operator execution) |
| Set at | Construction time, before operators run | Computation time, during operator execution |
| Contents | Observable identity, M identity, input relations, declared domain | Actual input values, relations used, parameters applied, outputs produced |
| Verifiable against | Origin's stated declarations | Declaration records — IRRS checks (Req-6) |
| Current V8 analog | `DeclaredRelations` (partially — class, not identity) | **Absent** — this is the structural gap |

**What independent verification requires (Req-6):**

IRRS does not accept the Generator's summary of what was computed. It
independently recomputes or verifies the recorded operations and their bindings
against declaration records directly. Matching names and references alone does not
establish that the recorded calculation occurred. Specific checks:
- Every input value in the calculation record must trace to a declaration record
  (Req-1, Req-2) — verified against the declared observable, not just named.
- The direction used in computation must match the declared direction basis (Req-3)
  — verified against the declaration record, not asserted in the calculation record.
- Parameters used (ρ_base, χ₀, cc) must match the M-declared values — verified
  against Origin's declared parameters, not against what the Generator recorded.
- If any input is unverifiable against a declaration record, the result is OPEN (Req-7).

**What this is not:**

The Generator does not certify its own execution. A calculation record produced by
the Generator is evidence to be checked against declaration records — not a certified
account to be accepted as-is. A provenance label ("this was produced by a Continuation
operator") is not an alternative to independent verification. A calculation record
that cannot be independently recomputed or verified fails Req-6 regardless of its
internal consistency. Labels describe; declaration records establish identity;
independently verified calculation records establish derivation.

**Simulated observations:**

A simulated observation may have a complete declaration record (its simulated
origin is declared) and a complete calculation record (its derivation traces
correctly). Completeness does not change epistemic status: a simulated derivation
does not become empirical evidence. IRRS admits it as a valid computation over a
declared simulated observable — not as an observation of D.

---

## 4. Provenance Identity — ObservableRelationDeclaration

An admitted observable relation is identified by an `ObservableRelationDeclaration`
(working name — subject to Verifier review before implementation). This is the
**declaration record** for a single declared edge: the formal record of what
Origin admitted as an observable relation within D.

Under V9, every edge in `DeclaredRelations` carries one.

**Required fields (each corresponds to an Origin declaration requirement):**

| Field | Requirement | Description | Status |
|---|---|---|---|
| `observable_source` | Req-1, Req-2 | The specific named observable from which this relation was declared through M. Not a class. A specific declaration: names the observable and the date of admission. | **OPEN** — returned to Origin for field definition |
| `direction_basis` | Req-3 | The declared relational distinction that establishes which end is source and which is target. Not index order. | **OPEN** — returned to Origin for field definition |
| `m_mapping_ref` | Req-2 | The declared M mapping (O → D) that projected this observable into D. Required for Req-6: IRRS checks the actual mapping used against this declaration. | **OPEN** — returned to Origin for field definition |
| `declared_domain` | Req-2 | The declared domain of this projection — the subset of D into which this observable was admitted. | **OPEN** — returned to Origin for field definition |

These fields together constitute the declaration record for one declared edge. Without
them, a relation carries a class but not an identity. Its loci have no standing.
Any variable derived from it has no declaration record to check against.

**NodeField declaration record (Req-1, Req-2):**

Every value in `NodeField` is a projection through M of an observable at a locus.
Under the RPPI, each value must also carry a declaration record: the M that projected
it, the observable it represents, and the locus at which it was declared.

Currently `NodeField::new` enforces finiteness only. Under V9, a declared `NodeField`
value is not merely a finite number — it is a declared projection through M. The
declaration record for each value is an OPEN condition (see §8).

**Current state:** `EdgeProvenance` records the class only. The `ObservableRelationDeclaration`
fields do not exist in the current API. Their precise representation is returned to
Origin as declaration requirements (§8, OPEN Conditions).

---

## 5. Provenance Derivation Contract

Provenance derivation is the chain from a variable, quantity, or result back
to its provenance identity, through every contributing mathematical input,
through M and authorized operators only (Req-4).

**Every contributing mathematical input (Req-4):**

Traceability is not limited to edge provenance. For an operator output `Δ(x)[e]`,
a complete derivation record includes:
- The declaration record of edge `e` (which observable relation was declared), and
- The declaration record of each NodeField value (`x[src]`, `x[tgt]`) — which M
  projected them, from which observable, at which locus.

Edge provenance alone is insufficient if the NodeField values are ungrounded.
The derivation must trace every contributing input.

**A derivation path is continuous** when every step is either:
- A declared M mapping (O → D) with a declaration record, or
- An authorized operator application (Δ, Σ, A, B, R) acting on values that
  themselves have declaration records — and when the resulting calculation record
  retains those declaration records.

**A derivation path is broken** when it passes through any value whose origin
is asserted rather than traced (Req-7). This includes:
- Operator outputs that carry no calculation record (the current state — §6.3).
- NodeField values admitted without a declaration record (currently: `NodeField::new`
  enforces finiteness only — OPEN condition, §8).
- Results composed from inputs of mixed relational origin without a record
  of which origin contributed each part.

### Current state (V8)

| Output type | Carries declaration record? | Carries calculation record? | Independent verification possible? |
|---|---|---|---|
| `PrimaryEdgeField` (Δ, Σ output) | No | No | Only with externally retained `DeclaredRelations` |
| `EdgeField` (A, B, R output) | No | No | Only with externally retained `DeclaredRelations` |
| `DeclaredRelations` | Partial (class only; identity OPEN) | N/A — this is the declared source | Partial |
| `NodeField` | No | N/A — this is an input | Not from type alone |

### V9 requirement

Each operator output must carry a **calculation record** that retains all of the
following (Req-4), enabling independent verification against declaration records (Req-6):

| Output | Calculation record must retain |
|---|---|
| `Δ(x)[e]` | Declaration record of edge `e`; declaration records of `x[src]` and `x[tgt]` |
| `Σ(Δ(x))[e]` | Declaration records of edge `e` and adj⁺/adj⁻ contributing edges; declaration records of all contributing `x` values; declared parameters (ρ_base, χ₀) |
| `A(x)[e]` | Declaration record of edge `e`; declaration records of each `x[i]` that contributed |
| `B(A(x))[e]` | Calculation records of all A-output entries that contributed |
| `R(B(A(x)))[e]` | Calculation records of all B-output entries that contributed; declared parameters (ρ, cc) |

**V9 adds** `ProvenancedOutput<T>` (working name) — a wrapper over the existing field
types that carries the full calculation record for each output entry:

```
ProvenancedOutput<T> {
    output: T,                // the existing PrimaryEdgeField or EdgeField (unchanged)
    calculation_record: ...,  // identities of all contributing values, relations,
                              // mappings, and declared parameters (Req-4)
}
```

This does not change any operator formula (Req-5). The precise structure of
`calculation_record` is the subject of Milestone 3 interface design. Returned to
Verifier review before implementation.

---

## 6. RPPI Failure Modes

Four failure modes violate the RPPI. Each requires detection and rejection (Req-7).
The V8 boundary is preserved: these are structural enforcement additions,
not new operator mathematics (Req-5).

### 6.1 Missing Provenance

A relation or variable has no declaration record. The provenance identity is absent.

- **Current state (V8):** `EdgeProvenance::Continuation` is accepted without an
  `ObservableRelationDeclaration`. The relation carries a label but not a record.
- **V9 enforcement (Req-1, Req-2, Req-7):** The V9 admission interface must
  reject any admitted relation that lacks an `ObservableRelationDeclaration` for
  any edge. V8 constructors are frozen (Req-5); this enforcement belongs to the
  separate V9 admission interface, not to `from_edges_with_provenance`. A label
  alone is insufficient.
- **Conformance:** `rppi_missing_observable_source_gap` (`[RPPI-GAP]`);
  `rppi_missing_observable_source_rejected` (`[V9-REQUIRED]`, `#[ignore]`).

### 6.2 Fabricated Provenance

A relation carries an `ObservableRelationDeclaration` whose `observable_source`
or `m_mapping_ref` cannot be verified against Origin's declarations. The identity
is asserted, not traced.

- **Current state (V8):** No M-mapping verification exists. Any class label is accepted.
- **V9 enforcement (Req-2, Req-6, Req-7):** The V9 admission interface must
  verify `m_mapping_ref` against Origin's declared M mappings. V8 constructors
  are frozen (Req-5); this enforcement belongs to the separate V9 admission
  interface. An unverifiable source must be rejected.
- **Conformance:** `rppi_fabricated_provenance_gap` (`[RPPI-GAP]`);
  `rppi_fabricated_observable_source_rejected` (`[V9-REQUIRED]`, `#[ignore]`).

### 6.3 Disconnected Provenance

A result exists and is numerically correct, but carries no calculation record. Its
derivation path to the declared observable relation that produced it is broken.
Numerical correctness and structural validity are satisfied; relational grounding
in the output type is not.

This is the RPPI framing of the provenance survival gap. Under Req-4 and Req-6,
a result without a calculation record cannot be independently verified and must
not be admitted.

- **Current state (V8):** All operator outputs are disconnected. `PrimaryEdgeField`
  and `EdgeField` carry no calculation record.
- **V9 enforcement (Req-4, Req-5, Req-6):** IRRS must reject any result not wrapped
  in `ProvenancedOutput<T>` with a complete calculation record.
- **Conformance:** `rppi_disconnected_provenance_gap` (`[RPPI-GAP]`);
  `rppi_disconnected_provenance_rejected_by_irrs` (`[V9-REQUIRED]`, `#[ignore]`).

### 6.4 Misattributed Provenance

A result carries a calculation record that attributes it to a declared relation
other than the one that actually produced it. The attribution is present,
internally consistent, and wrong.

- **Current state (V8):** No calculation record exists. Misattribution cannot occur
  because attribution cannot occur. The structural gap precedes the attribution gap.
- **V9 enforcement (Req-4, Req-6):** `ProvenancedOutput<T>` carries a binding to the
  `DeclaredRelations` that produced it. IRRS verifies the binding. A substituted
  declaration must be rejected.
- **Conformance:** `rppi_misattributed_provenance_gap` (`[RPPI-GAP]`);
  `rppi_misattributed_provenance_rejected_by_irrs` (`[V9-REQUIRED]`, `#[ignore]`).

---

## 7. Admissibility Enforcement Contract

The following admissibility conditions are currently enforced at construction.
They must be preserved in V9 (Req-5):

- `from_edges_with_provenance` panics if provenance count ≠ edge count.
- `from_edges_with_provenance` panics if any node index ≥ `n_nodes`.
- `from_edges_with_provenance` panics if any spatial edge carries
  `EdgeProvenance::Persistence`.
- `NodeField::new` panics if any value is non-finite (not in D).
- `require_rho_declaration` panics if `ρ_base` is non-finite or `χ₀` ≤ 0.

**V9 adds enforcement (per RPPI §6 and Origin declaration):**

| Failure mode | Enforcement point | Req | Enforcement action |
|---|---|---|---|
| Missing (§6.1) | V9 admission interface | Req-1, 2, 7 | Reject when `ObservableRelationDeclaration` absent for any edge |
| Fabricated (§6.2) | V9 admission interface | Req-2, 6, 7 | Reject when `m_mapping_ref` unverifiable against declared M |
| Disconnected (§6.3) | V9 admission interface | Req-4, 5, 6 | Reject any result not wrapped in `ProvenancedOutput<T>` |
| Misattributed (§6.4) | V9 admission interface | Req-4, 6 | Reject `ProvenancedOutput<T>` whose calculation record binding does not match the producing `DeclaredRelations` |

---

## 8. Open Condition Preservation Contract

OPEN conditions must propagate explicitly. No assumed value may be substituted (Req-7).

**Carried from V8:**

| Condition | Status |
|---|---|
| ρ max-selection declaration | OPEN — Origin declaration required |
| ρ saturating form declaration | OPEN — Origin declaration required |
| A-3 empty adjacency: Σ, R, R_persistence sums over empty declared sets evaluate to 0.0 | OPEN — Origin disposition pending |

**Added by RPPI and Origin declaration:**

| Condition | Req | Notes |
|---|---|---|
| `observable_source` field definition | Req-1, 2 | Name, type, admissibility rules, constructor requirements. What constitutes a valid named observable source. |
| `direction_basis` field definition | Req-3 | Name, type, admissibility rules. What constitutes a declared relational distinction establishing direction. |
| `m_mapping_ref` field definition | Req-2 | How an M mapping is named, stored, and verified against Origin's declared M set. |
| `declared_domain` field definition | Req-2 | What the declared domain of a projection records and how it is checked at construction. |
| `ObservableRelationDeclaration` structure | Req-1, 2, 3 | Full struct definition: name, fields, constructor, invariants, V8 interface integration (how it extends `DeclaredRelations` without modifying it). |
| NodeField declaration record | Req-1, 2 | How each value in `NodeField` carries a declaration record. Currently enforces finiteness only. The M-mapping origin of each value must also be declared — but the form of that declaration (new type, companion struct, extended constructor) is OPEN. |
| `ProvenancedOutput<T>` structure | Req-4, 5 | Full wrapper definition including the complete `calculation_record` type (all contributing values, relations, mappings, declared parameters). |
| `calculation_record` type | Req-4, 6 | The internal structure of the calculation record: how it records contributing values, which identities it carries, how IRRS traverses it. |
| V9 admission interface | Req-5, 6 | The separate interface through which V9 admits complete derivation records. What it accepts, what it verifies, what it returns or rejects. Designed in Milestone 3. |

None of these is closed by the Generator. They require Origin declaration.

---

## 9. Conformance Test Contract

The test suite in `tests/v9_conformance.rs` must demonstrate:

| Category | Tag | What is tested |
|---|---|---|
| V8 baseline | `[V8-BASELINE]` | Existing operator behavior unchanged (Req-5). Any code change that fails a baseline test violates V8 preservation. |
| Executable relation | `[V8-BASELINE]` | Provenance required at construction; out-of-range nodes rejected; Persistence excluded from spatial list; finite projection enforced. |
| Provenance survival gap | `[GAP]` | `PrimaryEdgeField` and `EdgeField` carry no calculation record. |
| Open condition | `[V8-BASELINE]` | `PersistenceState` requires a real prior — no cold-start constructor. |
| V8 continuity | `[V8-BASELINE]` | Full Δ→Σ and A→B→R chains produce finite output on valid declarations. |
| IRRS tracing gap | `[GAP]` | Tracing from operator output requires externally retaining `DeclaredRelations`. |
| RPPI — missing provenance | `[RPPI-GAP]` | Class-only provenance accepted without declaration record (Req-1, 2). |
| RPPI — missing provenance | `[V9-REQUIRED]` | Construction without `ObservableRelationDeclaration` must be rejected. `#[ignore]`. |
| RPPI — fabricated provenance | `[RPPI-GAP]` | Asserted class accepted without M-mapping verification (Req-6). |
| RPPI — fabricated provenance | `[V9-REQUIRED]` | Unverifiable `m_mapping_ref` must be rejected. `#[ignore]`. |
| RPPI — disconnected provenance | `[RPPI-GAP]` | Operator result is finite; carries no calculation record (Req-4). |
| RPPI — disconnected provenance | `[V9-REQUIRED]` | V9 admission interface rejects results without calculation record. `#[ignore]`. |
| RPPI — misattributed provenance | `[RPPI-GAP]` | No calculation record exists; substituting one relation's output for another's is undetectable. |
| RPPI — misattributed provenance | `[V9-REQUIRED]` | V9 admission interface rejects wrong derivation binding. `#[ignore]`. |

**Test counts:**
- 27 existing tests (`[V8-BASELINE]` and `[GAP]`) — all must pass.
- 4 new `[RPPI-GAP]` tests — must pass.
- 4 new `[V9-REQUIRED]` tests — `#[ignore]` until implementing milestones are verified.

---

## 10. IRRS Integration Contract

The Verifier must be able to trace results to relational observations without
relying on the Generator's assertions (Req-6). This requires:

1. **Declaration records for every admitted edge** — addressed by `ObservableRelationDeclaration`
   (§4), covering M identity (Req-2), direction basis (Req-3), and observable source (Req-1).
2. **Declaration records for every admitted value** — addressed by NodeField declaration
   record (§4, §8), covering M-mapping origin of each projected value (Req-1, Req-2).
3. **Calculation records for every operator output** — addressed by `ProvenancedOutput<T>`
   (§5), covering all contributing values, relations, mappings, and declared parameters (Req-4).
4. **A separate V9 admission interface** that checks calculation records against declaration
   records independently of the Generator's account (Req-5, Req-6).

**Traceable chain (Verifier disposition, 2026-10-24):**

> Declared observable relation → declared M → projected input →
> authorized operator execution → calculation record →
> independent IRRS verification.

This chain must be enforceable at every link, not only stated as intent.

**Current state:** The verifier can trace by retaining `(DeclaredRelations, output)` as
a pair. The `[GAP]` tests in `tests/v9_conformance.rs` document this explicitly.

**V9 requirement:** The output type alone is sufficient for tracing. The IRRS does not
retain `DeclaredRelations` separately.

**Independent verification requirement (Req-6):**

IRRS independently recomputes or verifies the recorded operations and their bindings.
It does not accept the Generator's calculation record as a certified account of
execution. A calculation record is evidence to be verified against declaration records
— matching names and references is not sufficient to establish that the recorded
calculation occurred.

**IRRS rejection requirement:**

The V9 admission interface must reject any result where (Req-7):
- A declaration record is absent or unverifiable (§6.1 missing, §6.2 fabricated).
- A calculation record is absent (§6.3 disconnected).
- A calculation record's binding does not match the producing `DeclaredRelations`
  (§6.4 misattributed).
- A calculation record cannot be independently recomputed or verified against
  declaration records — even if internally consistent.

The IRRS does not admit results based on numerical correctness, structural validity,
or provenance labels alone. Derivation must terminate in declared observable relations,
and the derivation structure must survive independent verification.

---

## 11. Development Constraints

- All V9 work is on the `kernel-v9` branch.
- No V9 addition merges to `main` without Verifier review.
- V8 operator forms are not changed (Req-5).
- OPEN conditions are not closed by the Generator; they require Origin declaration.
- Working names (`ObservableRelationDeclaration`, `ProvenancedOutput`, `calculation_record`,
  `observable_source`, `direction_basis`, `m_mapping_ref`, `declared_domain`) are subject
  to Verifier confirmation before implementation.
- `[V9-REQUIRED]` tests are `#[ignore]` until the implementing milestone is verified.
  Remove `#[ignore]` and confirm all 35 tests pass after each V9 enforcement milestone.
- The order of milestones:
    1. (Milestone 1, commit `1a54d08`) Specification + conformance test suite.
    2. (Milestone 2, commit `ee46735`) RPPI established; four failure modes defined;
       RPPI conformance tests added.
    3. (Milestone 3, commits `ab6d4f0`, `e118199`, this document) Origin declaration
       accepted. Declaration record / calculation record distinction established.
       Two Verifier corrections: RPPI failure mode enforcement (§6.1, §6.2) belongs
       to V9 admission interface, not frozen V8 constructors; IRRS must independently
       recompute or verify, not match names and references (self-certification excluded).
       **Next:** Interface design and executable conformance criteria for:
       - `ObservableRelationDeclaration` (declaration record for edges)
       - NodeField declaration record
       - `ProvenancedOutput<T>` and `calculation_record` type
       - V9 admission interface (separate from V8 constructors)
    4. Implementation of `ObservableRelationDeclaration` enforcement at construction
       (closes RPPI failure modes: missing §6.1, fabricated §6.2).
    5. Implementation of `ProvenancedOutput<T>` with full `calculation_record`
       (closes RPPI failure modes: disconnected §6.3, misattributed §6.4).
    6. Implementation of V9 admission interface with independent verification.
    7. IRRS integration verification.

---

*The operators hold no agency. Responsibility stays with Origin.*  
*Bounded over D. No claim beyond D.*  
**Metatron Dynamics, Inc. Kernel V9 — Third Milestone Specification.**
