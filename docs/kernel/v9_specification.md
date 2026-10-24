# Kernel V9 — Specification (First Milestone)

**Metatron Dynamics, Inc.**  
Kernel V9 specification. First milestone: executable relation definition and
conformance test contract.  
Bounded over D. No claim beyond D.

---

## Purpose

This document is the governing specification for Kernel V9 development.

The first V9 milestone is a **specification and conformance test suite**.
V8 operator forms are not rewritten in this milestone. The milestone
establishes precisely:

1. What constitutes an **executable relation** — the complete set of fields
   that must be declared through M before an operator acts.
2. How **observational provenance survives calculation** — the tracing
   requirement from observable through operator output.

Work proceeds on the `kernel-v9` branch. No change is merged to `main`
without Verifier review.

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

## 2. Executable Relation — Definition

A directed relation is **executable** when and only when all of the following
are declared through M before any operator acts:

| Field | Description | Current status in `DeclaredRelations` |
|---|---|---|
| Source locus | Node index of the relation's origin, traceable to a declared observable | PRESENT — `edges[e].0`; range enforced at construction |
| Target locus | Node index of the relation's terminus, traceable to a declared observable | PRESENT — `edges[e].1`; range enforced at construction |
| Provenance class | One of the three declared classes (Continuation, Coupling, Persistence) | PRESENT — `provenance[e]`; required at construction |
| Domain membership | Both loci projected into D through a declared M | PRESENT — `n_nodes` bound check enforced at construction |
| Direction provenance | The observable basis that establishes which end is source and which is target — not index order alone | **OPEN** — not currently stored or enforced |
| Observational source | The specific observable or physical measurement from which this relation was declared through M | **OPEN** — `EdgeProvenance` records the class, not the source |

**V9 adds** (to be defined in the next milestone, after spec review):

- `direction_basis: &'static str` — a named reference to the observable that
  establishes the direction of each edge, not derived from index order.
- `observable_source: &'static str` — a named reference to the specific
  observable or physical measurement from which the relation was declared.

These fields belong in the edge-level declaration, not in `EdgeProvenance`
(which records class, not identity). The precise representation — whether
as new fields on `DeclaredRelations`, a companion `RelationDeclaration`
structure, or an enhancement to `from_edges_with_provenance` — is a V9
implementation decision subject to Verifier review.

---

## 3. Provenance Survival Contract

Observational provenance must survive through the operator chain. The verifier
must be able to trace any operator output to the observed distinction that
originated it — **from the output type alone**, without relying on separately
retained references.

### Current state (V8)

| Output type | Carries provenance? | Verifier can trace? |
|---|---|---|
| `PrimaryEdgeField` (Δ, Σ output) | No | Only with separately retained `DeclaredRelations` |
| `EdgeField` (A, B, R output) | No | Only with separately retained `DeclaredRelations` |
| `DeclaredRelations` | Yes (class only; source OPEN) | Yes, for class — source is OPEN |

The central gap: `PrimaryEdgeField` and `EdgeField` carry no reference to the
provenance of the edges that produced them. The verifier holds the output and
a `DeclaredRelations` as a pair and consults the latter separately.

### V9 requirement

Each operator output must carry enough information to trace every entry back to
the provenance of the input edge(s) that produced it:

- `Δ(x)[e]` → provenance of edge `e`
- `Σ(Δ(x))[e]` → provenance of edge `e` and its adj⁺/adj⁻ contributing edges
- `A(x)[e]` → provenance of edge `e`
- `B(A(x))[e]` → provenance of the A output entries that contributed
- `R(B(A(x)))[e]` → provenance of the B output entries that contributed

**V9 adds** `ProvenancedOutput<T>` (working name) — a wrapper over the
existing field types that carries an edge-indexed provenance reference:

```
ProvenancedOutput<T> {
    output: T,                         // the existing PrimaryEdgeField or EdgeField
    edge_provenance: &DeclaredRelations, // the relations that produced this output
}
```

This does not change any operator formula. It is a structural addition to
the output type. Existing operator functions are wrapped; their internal
computation is unchanged.

The precise representation is subject to Verifier review before
implementation.

---

## 4. Admissibility Enforcement Contract

The following admissibility conditions are currently enforced at construction.
They must be preserved in V9:

- `from_edges_with_provenance` panics if provenance count ≠ edge count.
- `from_edges_with_provenance` panics if any node index ≥ `n_nodes`.
- `from_edges_with_provenance` panics if any spatial edge carries
  `EdgeProvenance::Persistence` (persistence loci are edge-valued, not node-valued).
- `NodeField::new` panics if any value is non-finite (not in D).
- `require_rho_declaration` panics if `ρ_base` is non-finite or `χ₀` ≤ 0.

**V9 adds** enforcement for:

- Direction provenance: a relation without a declared direction basis is
  inadmissible. Currently not enforced — index order can stand in for
  direction without rejection.
- Observational source: a relation without a declared observable source is
  inadmissible. Currently not enforced — `EdgeProvenance` class is accepted
  without a source name.

---

## 5. Open Condition Preservation Contract

OPEN conditions must propagate explicitly. No assumed value may be substituted.

The following OPEN conditions are carried from V8:

| Condition | Status | Location |
|---|---|---|
| ρ max-selection declaration | OPEN — Origin declaration required | `operators.rs`, ρ provenance audit §2 |
| ρ saturating form declaration | OPEN — Origin declaration required | `operators.rs`, ρ provenance audit §3 |
| A-3 empty adjacency: Σ, R, R_persistence sums over empty declared sets evaluate to 0.0 | OPEN — Origin disposition pending | `operators.rs`, Kernel V8 notes |

V9 adds OPEN conditions (to be confirmed by Origin):

| Condition | Added in | Notes |
|---|---|---|
| Direction provenance requirement | V9 specification §4 | Each declared edge requires a named direction basis |
| Observational source requirement | V9 specification §4 | Each declared edge requires a named observable source |

---

## 6. Conformance Test Contract

The test suite in `tests/v9_conformance.rs` must demonstrate:

| Category | Tag | What is tested |
|---|---|---|
| V8 baseline | `[V8-BASELINE]` | Existing operator behavior unchanged. Any operator code change that fails a baseline test violates the V8 preservation contract. |
| Executable relation | `[V8-BASELINE]` | Provenance required at construction; out-of-range nodes rejected; Persistence excluded from spatial list; finite projection enforced. |
| Provenance survival gap | `[GAP]` | Documents that `PrimaryEdgeField` and `EdgeField` carry no provenance reference. The test shows the verifier must hold `DeclaredRelations` separately. Not a failure — a documented gap that V9 closes. |
| Open condition | `[V8-BASELINE]` | `PersistenceState` requires a real prior — no cold-start constructor. |
| V8 continuity | `[V8-BASELINE]` | Full Δ→Σ and A→B→R chains produce finite output on valid declarations. |
| IRRS tracing gap | `[GAP]` | Documents that tracing from operator output to observable provenance requires externally retaining `DeclaredRelations`. The verifier can trace, but only as a pair (output, rel). |

---

## 7. IRRS Integration Contract

The verifier must be able to trace results to relational observations
**without relying on the Generator's assertions**. This requires:

1. The output type carries sufficient information to identify the provenance
   of every contributing edge — addressed by V9 addition in §3.
2. The provenance class is supplemented by the observational source — addressed
   by V9 addition in §2.
3. The direction of each edge is grounded in a declared observable basis, not
   index order — addressed by V9 addition in §2 and §4.

**Current state**: the verifier can trace results by retaining the
`DeclaredRelations` used to produce them alongside the operator output. The
`[GAP]` tests in `tests/v9_conformance.rs` document this explicitly.

**V9 requirement**: the output type alone is sufficient for tracing. The verifier
does not need to separately retain `DeclaredRelations`.

---

## 8. Development Constraints

- All V9 work is on the `kernel-v9` branch.
- No V9 addition merges to `main` without Verifier review.
- V8 operator forms are not changed.
- OPEN conditions are not closed by the Generator; they require Origin declaration.
- `ProvenancedOutput` and the direction-basis / observable-source fields are
  working names. The Verifier confirms or revises the names before implementation.
- The order of milestones after the first:
    1. (This document) Specification + conformance test suite.
    2. Origin declaration for direction provenance and observable source fields.
    3. Implementation of `ProvenancedOutput` wrapper.
    4. Implementation of direction-basis and observable-source enforcement.
    5. IRRS integration verification.

---

*The operators hold no agency. Responsibility stays with Origin.*  
*Bounded over D. No claim beyond D.*  
**Metatron Dynamics, Inc. Kernel V9 — First Milestone Specification.**
