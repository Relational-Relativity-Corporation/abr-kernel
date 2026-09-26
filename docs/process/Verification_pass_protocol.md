# Verification Pass Ordering Protocol

**Metatron Dynamics, Inc.** Kernel V8.\
Reference procedure. Bounded over D. No claim beyond D.

## Purpose

This document defines how Origin orders an independent verification pass
over a declared artifact.

It does **not** redefine the roles declared in *Role Separation and
Operator Application*. Origin, Generator, and Verifier remain exactly as
declared there. This document governs the preparation of a verification
pass so that the Verifier receives a complete, bounded, and reproducible
assignment.

A verification order is itself a declaration by Origin.

This protocol defines the organization of a verification pass. It does
not replace, supersede, or redefine the verification methods declared
elsewhere in the kernel. Those methods define what verification
activities are performed. This protocol defines how a verification pass
is declared, bounded, supplied with evidence, and reported.

------------------------------------------------------------------------

# 1. Preconditions

Before ordering a verification pass, Origin declares:

-   the bounded domain D;
-   the observable mapping M;
-   the artifact under review;
-   the revision or commit being reviewed;
-   the purpose of the pass.

Verification is inadmissible if any required declaration is not supplied,
incomplete, internally inconsistent, or otherwise insufficient to
identify the declared artifact and its observable provenance. There is
no partial admissibility arising from partial declaration.

------------------------------------------------------------------------

# 2. Role Declaration

Origin declares the intended roles for the session.

At minimum:

-   Origin
-   Generator
-   Verifier

The Verifier shall operate from a frame distinct from the Generator for
the artifact under review.

This protocol does not define the lifetime of role declarations. Role
declaration, role separation, and any alternation of roles are governed
by **Role Separation and Operator Application**. This protocol governs
only the preparation and execution of an individual verification pass
within that declared workflow.

------------------------------------------------------------------------

# 3. Scope

Origin declares exactly what the Verifier is expected to examine.

The scope should identify:

-   included artifacts;
-   excluded artifacts;
-   declared assumptions;
-   declared limits.

The Verifier reports findings only within the declared scope.

------------------------------------------------------------------------

# 4. Evidence

Origin declares the evidence available for verification.

Examples include:

-   declarations;
-   source code;
-   specifications;
-   datasets;
-   execution logs;
-   validation records;
-   published measurements;
-   generated artifacts.

If evidence is unavailable, Origin states this explicitly.

The Verifier shall distinguish between:

-   evidence supplied,
-   evidence not supplied,
-   evidence required but unavailable.

------------------------------------------------------------------------

# 5. Reading Order

Origin declares the order in which artifacts should be examined whenever
that order is important.

If later artifacts depend upon earlier declarations, the dependency
order shall be stated explicitly.

When no ordering dependency exists, artifacts may be reviewed
independently.

------------------------------------------------------------------------

# 6. Verification Questions

Origin declares the specific questions to be answered.

Questions should identify interfaces rather than isolated documents
whenever possible.

Examples include:

-   notation → declaration
-   declaration → implementation
-   implementation → execution
-   execution → validation record
-   validation record → published claim

Repository-specific questions may be added without modifying this
protocol.

------------------------------------------------------------------------

# 7. Evidence Discipline

Every conclusion returned by the Verifier shall be classified as one of:

**Verified**

Supported directly by the supplied evidence.

**Inference**

Reasonably follows from the supplied evidence but is not explicitly
established.

**Not Verifiable**

Cannot be established from the supplied evidence.

The Verifier shall not silently convert inference into verification.

------------------------------------------------------------------------

# 8. Located Findings

Verification returns findings, not approval.

Every finding should identify, whenever possible:

-   artifact;
-   section;
-   file;
-   line;
-   declaration;
-   implementation;
-   execution output;
-   or other observable location.

A statement that no finding was located is itself a valid finding when
supported by examination.

------------------------------------------------------------------------

# 9. Cross-Artifact Consistency

Where a claim spans multiple artifacts, verification is incomplete until
each link in the provenance chain has been examined.

Typical chains include:

-   Declaration → Mathematics
-   Declaration → Implementation
-   Implementation → Execution
-   Execution → Validation Record
-   Validation Record → Published Claim

The objective is verification of continuity, not isolated correctness.

Verification of a provenance chain is complete only when every declared
interface in that chain has been examined. A chain containing one
unexamined interface is not considered fully verified, even if every
examined artifact is individually consistent.

------------------------------------------------------------------------

# 10. Missing Evidence

If required evidence is not supplied, the Verifier shall identify precisely
what is missing.

Evidence not supplied establishes nothing about correctness.

The pass may still return verified findings for the material that was
supplied.

------------------------------------------------------------------------

# 11. Out-of-Scope Matters

Origin should explicitly declare subjects that are not part of the
current pass.

Examples include:

-   theoretical correctness of the framework;
-   scientific interpretation;
-   engineering style;
-   performance optimization;
-   future work.

Declaring exclusions prevents ambiguity regarding the intended scope of
verification.

------------------------------------------------------------------------

# 12. Output

A verification report should contain:

1.  The declared question.
2.  The answer.
3.  Supporting evidence.
4.  Located findings.
5.  Cleared findings.
6.  Items not verifiable from supplied evidence.
7.  Evidence additionally required, if any.

Verification concludes with findings returned to Origin.

It does not approve, certify, endorse, or publish the artifact.

Disposition remains with Origin.

------------------------------------------------------------------------

# 13. Pass Completion

Completion of a verification pass returns the located findings to Origin
for disposition.

Origin may:

-   revise the artifact;
-   decline individual findings with recorded rationale;
-   request an additional verification pass;
-   or terminate verification according to the governing workflow.

Completion of a verification pass is not equivalent to approval,
publication, certification, validation, or acceptance. Those
dispositions remain governed by the declared workflow and remain the
responsibility of Origin.

------------------------------------------------------------------------

# Principle

A verification pass is not an opinion about an artifact.

It is a bounded examination of declared claims against declared
evidence, performed from a frame distinct from the Generator, returning
located findings whose provenance is traceable to the supplied
materials.

This protocol establishes procedure only. It introduces no mathematical
declarations, no admissibility criteria, and no role definitions beyond
those already declared elsewhere in the kernel.

---

## Kernel V8 synchronization (2026-09-26)

Wording aligned with the Kernel V8 vocabulary: "absent" / "absence" of declarations and evidence restated as "not supplied". No procedure changed.
