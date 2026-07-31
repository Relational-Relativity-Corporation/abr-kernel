# Triad Constraint Methods

**Metatron Dynamics, Inc.**
Verification methods for high-velocity artifact production.

Derived from a verification session of 2026-07-27 in which six findings were
located across two published repositories. Every method below is justified by
a specific error it would have caught. No method is included on principle
alone.

---

## Why velocity is the design constraint

Conventional research rigor assumes slow production. Peer review, replication,
and the interval between draft and publication are latency-tolerant checks:
they work because producing the artifact takes months, leaving time for the
check to engage.

At hours-per-artifact, those checks are not slow — they are absent. The
artifact is complete and published before any of them could act.

Three consequences follow, and the methods below are shaped by them.

**1. Checks must run at production speed.** A check that takes longer than the
artifact took to build will be skipped under pressure, and skipped exactly when
it matters most. Prefer mechanical checks. Where judgment is required, bound it
to a fixed-scope pass rather than an open review.

**2. Gate on publication, not on creation.** Slowing the build wastes the
velocity that makes the method valuable. Slowing the *push* costs one pass.
Build fast; publish through a gate.

**3. Verification effort must scale per artifact class, not uniformly.** At
this speed, prose is generated as fast as code and carries no test suite. In
the session that produced this document, every major finding was in a README;
the code had fifty passing tests and its errors were subtler. **Prose is the
fastest-produced and least-checked artifact class, and therefore the highest
risk.** It has been treated as commentary. It is where the claims live.

A fourth consequence bears on the cross-artifact rule below: velocity
multiplies artifacts, and an error published in one repository is cited by the
next before it surfaces. Contradiction between artifacts is not an edge case at
this speed. It is the expected failure.

---

## Status of these methods: origin is not validation

Each method below is stated with the error that motivated it. That is its
**origin**, and origin is a historical fact about why the method exists. It is
not evidence that the method reliably detects its intended class of failure in
artifacts it has not yet seen.

Two distinct claims, and this document makes only the first:

| | Claim | Evidence required |
|---|---|---|
| **Origin** | A specific observed failure motivated this method | One documented instance |
| **Validation** | The method consistently detects that class across artifacts | Repeated exercise, with hit and miss recorded |

Applying this document's own Claim Provenance rule to itself: every method
below is `argued`, not `measured`. Each is derived from a single session
against two repositories — n = 1. None has been exercised prospectively.

| Method | Origin | Validation |
|---|---|---|
| CP — Claim Provenance | Documented (n=1) | Unvalidated |
| CS — Comparison Scope | Documented (n=1) | Unvalidated |
| DI — Declaration–Implementation | Documented (n=1) | Unvalidated |
| CA — Cross-Artifact Consistency | Documented (n=2) | Unvalidated |
| IR — Invariant Range | Documented (n=1) | Unvalidated |
| AG — Adversarial Gate | Documented (n=6 findings) | Unvalidated |
| CR — Correspondence Register | Documented (n=1) | Unvalidated |

The path from argued to measured runs through prospective use: apply the gate,
record which method produced each finding, record findings that no method
caught. Accumulating that record converts this document from a well-reasoned
proposal into an empirical artifact. Until then it is a proposal, and should be
described as one.

---

## The boundary: conformance and correspondence

Two error classes, with different remedies. Confusing them produces false
confidence.

**Conformance errors** — the artifact violates a stated rule of the framework.
An undeclared quantity, a claim without provenance, an implementation that
disagrees with its declaration. These are checkable internally, mechanically,
and fast. Every method in this document addresses this class.

**Correspondence errors** — the declaration is well-formed and satisfies every
rule, but does not describe the world. In the source session, a declared edge
`Power → Voltage` carried stated provenance, correct directionality, no ring,
and passed every admissibility check. It was backwards: on RDNA 2 the DPM state
sets voltage and frequency, and power is a consequence.

> **No internal constraint catches a correspondence error.**
> Conformance checking establishes that the artifact is consistent with the
> framework. It establishes nothing about whether the framework is consistent
> with the system. Only domain knowledge from outside the declaration reaches
> that class.

This is a declared limit of the entire apparatus below, not a gap to be closed
by adding rules. Method 7 handles it by routing rather than by checking.

---

## Origin obligations

Every method in this document is executed by Generator and Verifier. None of
them binds anything unless Origin discharges two obligations. This section is
load-bearing: the six methods that follow are inert without it.

### Obligation 1 — Iterate to PASS

> **PASS** — no unresolved conformance findings remain, and every declined
> refinement has been explicitly recorded with its rationale.
>
> PASS is a statement about conformance only. It carries no claim about
> correspondence with the external system.

**Origin must run Generator/Verifier passes until the Verifier verdict is PASS
with no refinements outstanding.**

A verdict of "verified with minor recommendations," "provisionally verified,"
or "verified, with the following refinements" is **not** a PASS. It is an
instruction to run another pass. Refinements are findings that have been
phrased politely; a refinement declined by default is a finding suppressed by
default.

**The single exception.** Origin may terminate the loop with refinements
outstanding only by declaring each outstanding refinement unnecessary. That
declaration must be:

- **Specific** — naming the individual refinement, not the set
- **Understood** — Origin can state what the refinement asks for and what
  changes if it is not made
- **Reasoned** — carrying the ground on which it is declined
- **Recorded** — written into the artifact or its open conditions, not held
  privately

Silence is not a declaration. An unaddressed refinement that appears in no
record has been dropped, not declined.

**The exception is the failure mode of this obligation.** It exists because
some refinements are genuinely out of scope, and a loop with no termination
condition is not a process. But it is also the only route by which a finding
can leave the system without being fixed, and under time pressure it will be
the route taken. Track its use: the count of declared-unnecessary refinements
per artifact is itself a metric. A rising rate means the gate is being
processed rather than run.

**If the Verifier never returns PASS**, that is a finding about the Verifier,
not a licence to invoke the exception repeatedly. A Verifier that always
produces refinements is either operating without a termination condition or
reviewing an artifact that is not converging. Both are diagnosable; neither is
resolved by declaring the findings unnecessary.

### Obligation 2 — Read every pass in full

**Origin must read the entirety of each pass — commentary and code — and work
to understand the full context of the material as comprehensively as
possible.**

Three consequences, each of which is a hard requirement rather than a
recommendation:

**No delegation of the read.** A summary of a pass is not the pass. Origin may
not discharge this obligation by reading a Generator's synopsis of a Verifier's
findings, or a Verifier's characterization of a Generator's output. Summaries
are lossy by construction — that is what a projection is, and this document's
own Claim Provenance rule applies here. Reading a summary and acting as though
the pass was read is a provenance failure at the level of the process.

**Commentary and code are one artifact.** Reading the prose without the
implementation, or the implementation without the prose, defeats Method DI
entirely — declaration-implementation mismatch is only visible to a reader
holding both. In the source session, ρ documented per-node and implemented
per-edge was invisible from either side alone.

**Origin is the only party positioned to catch correspondence errors.** The
Generator and Verifier operate inside the declaration. Origin holds the domain
knowledge, the outside context, and the history of why each declaration was
made. A correspondence error — a well-formed declaration that is simply wrong
about the world — is unreachable from inside the loop, and Origin reading
comprehensively is the only place in this process where it can surface.

### On the rate limit

These two obligations bound how fast artifacts can be published, because Origin
cannot read faster than Origin reads.

That is not a defect of the process. It is the governor. Production velocity
was identified above as the condition generating the errors this document
exists to catch; a process whose verification scaled effortlessly with
production would not constrain anything. The binding constraint on publication
should be Origin's comprehension, and if the volume of artifacts exceeds what
Origin can read in full, the correct response is to publish fewer artifacts —
not to read less of each.

---

## Method 1 — Claim Provenance (CP)

**Rule.** The admissibility condition extends from quantities to claims. Any
statement about a system's behaviour must trace to a measurement of that
system, through a declared M, in the same way a quantity must.

A claim without a measurement behind it is a provenance failure regardless of
whether it is true.

**Caught in source session.** "These distinctions are not visible in profiler
output," published beneath a table of results computed entirely on synthetic
generated graphs. No profiler was run. The claim had no measurement behind it.

**Application.** Every declarative sentence about system behaviour carries one
of four tags, stated or evident from context:

| Tag | Meaning |
|---|---|
| `measured` | Obtained from an instrument on the declared system |
| `generated` | Computed from synthetic or constructed input |
| `declared` | A definition or a choice, not a finding |
| `argued` | Follows from stated premises; not demonstrated by this artifact |

A `generated` result may not support a `measured` claim. That single
prohibition is the highest-value rule in this document.

**Cost.** Minutes. It is a read-through, not an analysis.

---

## Method 2 — Comparison Scope Declaration (CS)

**Rule.** Before any comparative claim, declare what is being compared, what is
excluded from each side, and why the two are commensurable. The declaration
precedes the number.

**Caught in source session.** A 93% code-reduction claim against a production
profiling tool. Both line counts were accurate. The comparison was not: one
side carried multi-architecture counter tables, dashboards, a database layer,
roofline analysis, CLI and packaging; the other implemented six loci and six
edges. Writing the scope declaration first would have made the mismatch visible
before the claim was made.

**Application.** A comparative claim ships with three lines:

```
Compared:     <the specific layer or function on each side>
Excluded:     <what each side carries that the other does not>
Commensurable because: <the argument, or the claim does not ship>
```

If the third line cannot be written, the comparison is not available. Narrow
the scope until it can be.

**Cost.** Minutes, and it usually improves the claim — a narrow comparison that
holds is worth more than a broad one that does not.

---

## Method 3 — Declaration–Implementation Correspondence (DI)

**Rule.** A docstring is a declaration. Where implementation and declaration
disagree, the artifact has an undeclared modelling decision, and that is a
provenance failure independent of whether the code is correct.

**Caught in source session.** ρ documented as a per-node quantity, implemented
as one value per edge taken from the source node only. The code was
self-consistent; the target node's contribution was silently discarded. The
discarding may well be right — it was never declared, so it was never checked.

**Application.** For each operator, three questions: does the docstring state
the same indexing the code returns; does it state the same accumulation depth
the code performs; does every discard in the code appear in the docstring's
declared discards. Any "no" is a finding.

**Cost.** One pass per operator, mechanical.

---

## Method 4 — Cross-Artifact Consistency (CA)

**Rule.** Two artifacts citing the same grounding documents must not describe
the same object differently. Where they do, at least one is wrong, and both are
publicly reachable.

**Caught in source session.** Two findings of this class. One repository stated
that relational analysis and profiling are "complementary and non-overlapping";
a second stated it was a code reduction *of* the profiler — non-overlapping
outputs cannot be reductions of each other. Separately, one repository
described B as accumulating recursively along full continuation chains in
reverse topological order while the shared implementation summed immediate
successors only.

Both were reachable within two clicks of the same organization page.

**Application.** Before publishing artifact N, grep artifacts 1..N−1 for the
operator names, the framing claims, and the grounding-document citations they
share. Contradiction is a blocking finding, not a note.

**Cost.** Grows with portfolio size, which is precisely why it should be
automated early. At forty-eight repositories this is no longer a manual check.

---

## Method 5 — Invariant Range Declaration (IR)

**Rule.** Every declared invariant states its expected range at the point of
definition. A computed value outside that range is a blocking finding, not an
observation.

**Caught in source session.** ρ_P is documented as ≪ 1 in the Primary Region
and ≈ 1 at ABR activation. On a live run it returned 2.667 — the numerator
counts nonzero entries across all edges while the denominator counts only edges
with successors, so the ratio is not bounded by 1 at all. Present in the code
for as long as the code existed. Never surfaced, because nothing declared what
the value was supposed to do.

**Application.** Each invariant carries `expected_range` in its declaration.
A test asserts it. Out-of-range halts and reports; it does not warn and
continue.

**Cost.** One line per invariant, once.

---

## Method 6 — Adversarial Gate (AG)

**Rule.** Before any artifact becomes public, one pass whose sole instruction
is to find what is wrong, conducted with no context indicating that the work is
expected to succeed.

This is a **gate**, not a review. It is required, it is time-boxed, and its
output is a findings list rather than an assessment.

**Why it is separate from the other methods.** Methods 1–5 are conformance
checks and can be automated. Method 6 is the only one that can locate an error
nobody thought to write a rule against, and it is therefore the only method
here with a chance — not a guarantee — of touching a correspondence error, if
the reviewing party happens to hold the relevant domain knowledge.

**A limit that must be stated.** In the source session, an AI instance in build
mode endorsed the artifacts; the same class of instance under adversarial
instruction located six findings in an hour. Changing review stance and
instructions coincided with a substantially different findings set.

Stance is the hypothesis for that difference. The session did not isolate it as
the explanatory variable — instruction, context, and the artifacts' own
maturity all differed between the two conditions — and it remains a
methodological hypothesis requiring repeated observation. Tagged `argued`,
n = 1.

The operational consequence stands regardless of whether the hypothesis
validates, because it is a precaution rather than a conclusion: run the gate
cold. No prior context, no framing about hoped-for outcomes, no history of the
build. An assistant that helped build something is not an independent check on
it, however confident it sounds — and the cost of running cold is low enough
that it does not need the causal claim to justify it.

**Cost.** One session. In the source case, six findings across two published
repositories in approximately one hour.

---

## Method 7 — Correspondence Register (CR)

**Rule.** Every declaration that asserts something about the external world —
causal direction, physical mechanism, hardware behaviour, domain structure —
is entered in a register with its source, and marked `verified` or `unverified`.

Artifacts publish with unverified entries **visible**, not withheld.

**Why this method exists.** It does not check anything. Correspondence cannot
be checked internally. What the register does is make the unreachable class
*visible* rather than invisible, so that its size is known and it can be routed
to someone who can settle it.

**Caught in source session.** The `Power → Voltage` edge would have been
entered as an unverified causal assertion about RDNA 2 power management. Under
this method it publishes flagged, and the first hardware engineer who reads it
corrects it in one line instead of quietly distrusting the whole graph.

**Application.** One table per artifact:

```
| Declared assertion | Domain | Source | Status |
```

`Status` is `verified` only when an outside source confirms it — a datasheet,
a specification, a domain expert. "It seemed right when declared" is
`unverified`, and there is no shame in a register full of them. There is
considerable exposure in a register that pretends to be empty.

**Cost.** Ongoing, small. Highest-value method in the document for external
credibility, because it converts an unknown liability into a stated open
condition.

---

## The publication gate

Nothing becomes public without passing all seven. Estimated total: one to two
hours against an artifact built in hours.

```
[ ] O1  Verifier verdict is PASS with no refinements outstanding, or every
        outstanding refinement is specifically, understandably, reasonably
        and recordedly declared unnecessary by Origin
[ ] O2  Origin has read every pass in full — commentary and code, no
        summaries substituted
[ ] CP  Every behavioural claim tagged; no generated result supports a
        measured claim
[ ] CS  Every comparative claim carries scope, exclusions, commensurability
[ ] DI  Every operator's docstring matches its implementation's indexing,
        depth and discards
[ ] CA  No contradiction with any previously published artifact sharing
        grounding documents
[ ] IR  Every invariant declares a range; every range is asserted in test
[ ] AG  Cold adversarial pass run; findings resolved or declared open
[ ] CR  Correspondence register attached; unverified entries visible
```

O1 and O2 are discharged by Origin and by no one else. The remaining seven may
be executed by Generator or Verifier; these two may not be delegated, and an
artifact on which they have not been discharged has not passed the gate
regardless of how many other boxes are ticked.

> **Passing the gate establishes conformance to this methodology. It does not
> establish correctness with respect to the external system.**
>
> A fully compliant artifact may still be wrong about the hardware, the market,
> or the physics. The correspondence register records what remains outside
> reach; it does not close it. Treating a passed gate as a correctness result
> would reproduce, at the level of the methodology, exactly the overreach the
> methodology exists to prevent.

Findings that are not resolved are **declared as open conditions**, with stable
identifiers, in the artifact itself. A declared open condition is not a
weakness in the publication. It is the mechanism by which the publication
remains honest at speed.

---

## Testing the methodology itself

The methods above are subject to the same discipline as any other declaration.

**Every method must fire.** A rule that never produces a finding is either
unnecessary or unenforced. Record which method produced each finding. Review
the tally quarterly. A method with no hits in six months is removed or
rewritten — not retained for completeness.

**Record the misses, not only the hits.** A findings ledger showing only what
the gate caught measures activity, not reliability. The quantity that matters
is what the gate *missed* — errors found later, by a reader, a client, or a
subsequent session, that a method should have caught and did not. Every miss is
either a method that failed to fire, or a class no method covers. Both are
findings about the methodology.

This ledger is what moves each method from Origin to Validation. Without it,
the status table above never changes, and the methodology stays a proposal
indefinitely.

**Every method must be justified by an error.** New methods enter only when an
actual failure demonstrates the gap. Methods added on principle produce
ceremony, and ceremony consumes the velocity that makes this approach worth
having.

**The false-confidence risk is the real one.** A framework that reports
compliance with itself can feel verified while remaining wrong about the world.
The correspondence register exists to keep that gap in view. If the register is
ever empty, that is a finding about the register.

---

## Declared limits

This apparatus reaches conformance errors. It does not reach correspondence
errors, and no extension of it will.

Three things reach correspondence, none of which is a rule:

1. Domain knowledge from outside the declaration
2. Reproducibility — fixed seeds, no special hardware, clone-and-verify — which
   makes independent checking *available* to anyone who cares to run it
3. At least one technical reader whose reasoning does not route through the
   framework

The third has no substitute. Additional Triads sharing the same kernel,
declarations and framing share the same blind spots by construction. Producing
more instances of one's own method is not independence, and the resemblance is
close enough to be dangerous.

---

**Metatron Dynamics, Inc.**
*Bounded over D. No claim beyond D.*
