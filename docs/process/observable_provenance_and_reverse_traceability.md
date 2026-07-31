# Observable Provenance and Reverse Traceability

**Metatron Dynamics, Inc.**
Reference procedure. Bounded over D. No claim beyond D.
Declared by Origin, 2026-07-30.

---

## Foundational principle — truth requires observable relationship

**No mathematical construct may be treated as a source of truth by
virtue of its internal properties alone.** Consistency, elegance,
symmetry, generality, or predictive success measured entirely within
the mathematics itself — none of these substitute for an established
relationship to an observable. A construct may be proposed and
explored. It does not become admissible as true by being internally
well-formed. It becomes admissible only when a chain from the
construct terminates in something observed outside the framework, not
merely in another step declared inside it.

This is the deepest reason reverse traceability matters, and it is
worth stating as its own requirement rather than leaving it implied by
the mechanics below. A framework could in principle be scrupulously
documented, provenance-complete at every internal step, and still be
describing nothing — if every step's provenance points only to
another step inside the same framework, and the chain never actually
exits the mathematics to touch an observable. Internal consistency,
however rigorously maintained, does not by itself rule this out. Only
a chain that demonstrably terminates in an observable does.

Two consequences follow directly, and both are necessary — the second
is what keeps the first from becoming so strict that it forecloses
discovery:

1. **No mathematical proposition enters the kernel's record as
   established without provenance** — a stated observable, a stated
   operator sequence, and a checkable path back. This is not limited
   to named Layer 3 invariants. It covers any numeric or mathematical
   quantity doing work in the kernel — intermediate operator outputs,
   coefficients, and constants (a bare threshold such as the 3.5 Å
   hydrogen-bond cutoff used in `abr-biological-binding`,
   2026-07-30, is exactly this class of thing, and required the same
   provenance treatment after the fact that this principle asks for
   from the start).

2. **A candidate relationship may still be proposed and tested before
   it is established.** Exploration must remain possible, or the
   framework could only ever confirm what is already known. A proposed
   relationship does not need to be true to be proposed — but it must
   be *traceable to an observable* (so it is testable at all, not
   free-floating theory) and *plausible*, meaning grounded in
   something already declared elsewhere in the kernel or in an
   independently established fact — not invented after the fact to
   explain the one data point in front of you. Plausibility earns a
   proposition the right to be tested. It never earns the right to
   skip the test. The original CP-01 justification in
   `abr-biological-binding` (resonance delocalization) was plausible
   by this standard — real, independently-established chemistry. Its
   failure was not implausibility; it was using plausibility as a
   substitute for measurement instead of a reason to go measure.

---

## The claim being tested

Origin has stated the kernel's actual claim precisely:

> The kernel is establishing a traceable correspondence between three
> layers — observable declarations, operator application, derived
> invariants — and the path must also work in reverse. That
> bidirectionality is what prevents the mathematics from becoming
> self-referential.

This is a stronger and more specific claim than "the code is
internally consistent," and it needs its own test, distinct from both
the clean-room build test and the cross-document consistency audit
(`kernel_self_consistency_test.md`, 2026-07-30). Those two check
whether documents and code agree with each other. This one checks
whether the forward computation actually has a working inverse — not
asserted, executed.

**The concrete failure mode this guards against:** a derived invariant
sitting in a validation record, correct by every document cross-check,
that nonetheless cannot be traced back to the specific observable and
operator sequence that produced it except by a person remembering how
it was built. A number that only a person can trace has become
self-referential in exactly the sense Origin means — its provenance
depends on human recollection rather than executable kernel state.

---

## Worked example (forward direction), from today's session

`abr-biological-binding`'s `mlc_verify.rs` (2026-07-30) already
implements one small, real instance of this pattern, for the simplest
possible invariant: a Euclidean distance between two declared loci.

**Forward:**

```
Declared observable (M_declaration_1MLC.md, HB-02)
  GLU50(B)-OE2, ARG45(E)-NE
        ↓
Operator application (implicit: Euclidean distance in 3-space)
        ↓
Derived invariant: 2.73 Å
```

**Reverse**, as actually implemented in `run_check()`:

```
Derived invariant: 2.73 Å (declared value)
        ↑
Operator sequence: distance_to() computed from two atom coordinates
        ↑
Declared observables: (chain 'B', resSeq 50, "GLU", "OE2") and
                       (chain 'E', resSeq 45, "ARG", "NE"),
                       looked up by exact key in the parsed structure
```

The reverse path is not narrated — it is executed. Given only the
declared relation's identifying key (which atoms, which declared
distance), the program independently retrieves the actual coordinates
and recomputes the value, and reports whether it matches. This is
exactly the shape of the reverse arrow Origin is describing, just for
the simplest possible invariant.

**What this example does not yet test:** anything involving an actual
kernel operator (Δ, Σ, A, B, R) or a Layer 3 derived invariant (ε[e],
τ[v], Φ[v]). A raw distance requires no operator application at all —
it is the observable itself, not something an operator produced from
it. The real test of reverse traceability has not yet been attempted
against the kernel's actual operator layer.

---

## What the reverse test looks like for a real kernel invariant

Using the actual signatures present in `derived_invariants.rs` and
`operators.rs` (confirmed present, 2026-07-30):

```rust
pub fn epsilon_e(delta_iota_e: f64, indicator: f64) -> f64
pub fn tau_v(a_persistence_coupling: f64, cos_theta: f64) -> f64
pub fn phi_v(iota_v: f64, tau_v: f64) -> f64
pub fn operator_a(f: &NodeField, rel: &DeclaredRelations, pairs: &[(usize, usize)]) -> EdgeField
pub fn operator_r(bg: &EdgeField, rel: &DeclaredRelations, rho: &[f64]) -> EdgeField
```

These are pure functions: given the same declared inputs, they always
produce the same output, with no hidden state. That is the property
that makes reverse traceability tractable at all — a function with
internal mutable state or hidden dependencies cannot be reliably
inverted, because the same output could have come from different
histories.

**Forward, generalized:**

```
Declared observable set (per M, at some region)
        ↓
NodeField / DeclaredRelations construction
        ↓
operator_a / operator_r / operator_sigma (per role_separation's
  declared operator path)
        ↓
epsilon_e / tau_v / phi_v (Layer 3 derived invariant)
```

**Reverse, generalized — what must exist and currently does not:**

A bare invariant value alone is not, strictly, invertible — ε[e] =
0.0034 could legitimately occur on more than one edge. What must be
reconstructible is the invariant *together with its declared
identity* — region, observable identifiers, edge or node identifier.
Given the invariant and its identity, there is currently no function
that answers: *which* `NodeField`, *which* `DeclaredRelations`, and
*which* operator sequence produced this number. The forward functions
exist and are tested (58/58 passing, per `test_log.txt`). No
corresponding reverse lookup exists.

This is not a defect in the current kernel — reverse lookup was never
declared as a requirement until Origin stated it explicitly today. It
is, however, the concrete gap between "internally consistent" and
"bidirectionally traceable," and it is currently open.

**Reverse traceability is not mathematical inversion.** It does not
require solving Δ, Σ, A, or R backward as equations. It requires
reconstructing a derived invariant's declared provenance from
provenance recorded at computation time — bookkeeping, not algebra.
The distinction matters because it keeps the requirement achievable:
the kernel needs to *record* what went in, not *derive* what must have
gone in from the output alone.

---

## Kernel principle — provenance-carrying quantities

**Provenance is part of the value. Not metadata, not documentation,
not a separately-maintained validation record — part of the computed
object itself.** This is the pattern `DerivedQuantity` already
implements in `abr-biological-binding`'s `component_pair.rs`: it is
not possible to obtain a bare, provenance-free number from that type.
The value and its history are the same object.

Stated as a rule: **every derived invariant shall carry sufficient
provenance to permit independent reconstruction of its declared
observable inputs and operator sequence.**

A note on how to read this rule, since a cold Verifier pass on this
document raised it explicitly and the two readings are not quite the
same: this could be stated as a new mathematical admissibility
criterion (an invariant without provenance is not a valid kernel
object) or as a verification requirement (the mathematics is
unchanged; what's new is that admission to the record now requires
demonstrating provenance). This document does not resolve which
framing is correct — that is Origin's call, not a Generator's or a
Verifier's, consistent with `Verification_pass_protocol.md`'s
disposition rule. What both framings agree on, and what is not in
question: a derived invariant lacking executable provenance does not
enter the kernel's record as an established claim, regardless of
which name is given to the rule that keeps it out.

---

## Minimum requirement for a reverse-traceable invariant

For any derived invariant type (ε[e], τ[v], Φ[v], and any added
later), the following must be constructible, not merely narratable:

1. **A provenance record accompanying the value itself** — not stored
   separately in a validation record document that must be manually
   cross-referenced, but structurally attached to the computed value,
   the way `DerivedQuantity` in `abr-biological-binding`'s
   `component_pair.rs` carries its contributing `ComponentPair`s
   forward automatically rather than returning a bare `f64`.

2. **The provenance record must identify:** the specific declared
   observable(s) consumed, the specific measurement mapping M applied,
   and the specific operator (or operator sequence) invoked — by name
   and by the actual argument values passed, not by general
   description.

3. **A reverse check must be executable, not asserted:** given the
   provenance record, an independent function must be able to
   recompute the invariant from the recorded observables and operator
   sequence, and confirm it matches the stored value — the same
   MATCH/MISMATCH pattern `mlc_verify.rs` already uses for raw
   distances, extended to cover an actual operator application rather
   than a bare geometric computation.

This is a real, non-trivial addition to `metatron_kernel_v7` — it
likely means every Layer 3 function's return type changes from a bare
`f64` to a struct carrying both the value and its provenance, mirroring
the `DerivedQuantity` pattern already built and tested in
`abr-biological-binding` today. That pattern is offered here as the
concrete starting point, since it already exists, compiles, and is
tested — not as a hypothetical design.

---

## Generalization criterion — how to know if one example is enough

Origin's question was whether one or two worked examples are
sufficient to generalize, given the operators' declared invariance
across regions. The answer this document proposes: **the test of
generalization is not "does the pattern look similar," but "does the
identical reverse-lookup code work unmodified at a second region."**

Concretely: once reverse traceability is built for one Layer 3
invariant at one region (Primary, most likely, given
`operators_notation_and_constraint.md`'s primary derivations), the
generalization claim is tested — not assumed — by applying the exact
same reverse-lookup mechanism to a different invariant, or the same
invariant at a different declared region (per
`observable_variable_sets.md`'s region list, Primary through
Planetary), with zero changes to the reverse-lookup logic itself, only
to the declared M for that region.

If that holds without modification, the invariance claim in the README
— "what changes across regions is the declared measurement mapping M,
not the operators or their constraints" — is directly supported by
working code, not merely stated. If it does not hold without
modification, that is itself a located finding worth recording:
either the invariance claim needs qualification, or the reverse-lookup
implementation smuggled in a region-specific assumption it shouldn't
have.

---

## Relationship to existing kernel discipline

Whether this specification introduces a new mathematical admissibility
criterion or a new verification requirement against unchanged
mathematics is Origin's disposition to make, per the note above — this
document deliberately states both readings rather than choosing one.
What is not in question either way: it operationalizes a claim Origin
has already made about what the kernel is for — bidirectional
traceability between observable, operator, and invariant, and no
source of truth outside that relationship — into something buildable
and testable, following the same discipline already applied today to
`abr-biological-binding`: build the smallest real example first,
verify it actually executes, and only then generalize the pattern into
kernel doctrine.
