# Kernel Self-Consistency Test

**Metatron Dynamics, Inc.** Kernel V8.
Reference procedure. Bounded over D. No claim beyond D.
Declared by Origin, 2026-07-30.

---

## Purpose

Two different questions get asked interchangeably about a kernel or
application repository, and they require two different tests:

1. **Does it run, self-contained, from nothing but what's published?**
   This is mechanical. It tells you what files are missing or
   undeclared.
2. **Is it internally consistent — do the code, the notation, and the
   validation record actually agree with each other?** This is not
   mechanical. `cargo test` passing tells you the code satisfies the
   assumptions its own author encoded into the tests. It does not tell
   you those assumptions match what the notation actually declares —
   exactly the gap that let CP-01's original justification and
   `mlc_verify.rs`'s actual coverage drift apart in
   `abr-biological-binding` before either was corrected.

Conflating these two questions is itself a source of false confidence:
a repository can pass Part I perfectly and still be internally
inconsistent, and a repository can fall short of Part I for a trivial reason (a missing
data file) while its actual math is sound. Both parts are required;
neither substitutes for the other.

---

## Part I — Clean-Room Build Test

**What this catches:** files the repository silently depends on that
aren't actually published, path assumptions that only work because the
builder already has local state, and instructions that don't match
what the repository actually requires.

**Procedure:**

1. Pick a directory with nothing in it related to this project — not a
   copy, not a branch, empty.
2. `git clone` the actual public URL into that empty directory. Do not
   copy files by hand from a working local folder — the point is to
   get exactly, and only, what the repository publishes.
3. Do not consult memory of how the repository is structured. Follow
   only the README, literally, as a stranger would.
4. Run exactly the commands the README states, in the order it states
   them, with no substitutions.
5. Record the actual output at each step — not a summary, the full
   terminal output — the same discipline already established for
   `abr-biological-binding`'s verification passes today.

**Pass condition:** the repository builds, tests run, and any
documented executable produces its documented output, using nothing
but what `git clone` retrieved and nothing but what the README says to
do.

**Common findings this catches, based on gaps already found today:**
- A data file (e.g. `1MLC.pdb`) the code expects but the README never
  says to obtain, or that isn't actually committed to the repo
- A `.gitignore` gap that either excludes something needed or fails to
  exclude `target/`, bloating the repo with build artifacts that mask
  whether a clean build actually works
- A path assumed relative to a directory the instructions never
  establish ("run from the repo root" stated nowhere)
- A dependency version not pinned, so a clean build pulls something
  different than what was tested against

**When Part I does not pass:** the correction is almost never the code. It's almost always
the README, the `.gitignore`, or a missing committed file. Treat a
Part I finding as a documentation/completeness finding, not a
correctness finding.

---

## Part II — Cross-Document Consistency Audit

**What this catches:** code that runs correctly by its own internal
logic but has drifted from what the notation, role-separation
document, or validation record actually declare — the category of gap
a passing test suite cannot surface, because the tests were written
against the same assumptions as the code.

This cannot be fully mechanized the way Part I can. It requires a
Verifier pass, structured the same way today's `abr-biological-binding`
pass was structured, but aimed at the kernel itself rather than an
application repository. Use `Verification_pass_protocol.md` to order
it.

The six questions below are **required interface checks** — the
baseline every kernel verification pass must cover, not merely
suggestions. They correspond to the interfaces the kernel's own
documents declare as load-bearing (notation, code, validation record,
observable variable sets, language discipline, cross-region
invariance), so a pass that skips one of them has left a declared
interface unexamined, which `Verification_pass_protocol.md` Section 9
already treats as an incomplete chain. Additional
**repository-specific interface checks** may, and for most
application repositories should, be added on top of these six — per
`Verification_pass_protocol.md` Section 6, "repository-specific
questions may be added without modifying this protocol." The six
below are the fixed baseline that addition sits on top of, not a
menu to select from.

**Required interface checks:**

1. **Notation → code.** For every operator declared in
   `operators_notation_and_constraint.md` (Δ, Σ, A, B, R, and any
   others), does `operators.rs` implement exactly what's declared —
   same admissibility conditions, same directional discipline (D-1,
   D-2), no silently added behavior the notation doesn't state?

2. **Code → validation record.** For every function in
   `derived_invariants.rs` claiming a support classification
   (Observed / Derived / Inferred), does `validation_record.md`
   actually state the observable source and measurement mapping M for
   that classification? A function with no corresponding validation
   record entry is an unaudited claim sitting in working code.

3. **Validation record → observable variable sets.** Does every
   observable cited in `validation_record.md` actually appear in
   `observable_variable_sets.md`'s declared primitive set at the
   region in question? An observable used but never declared at that
   region is an undeclared import — the exact pattern CP-01's
   resonance-theory justification was.

4. **Language discipline → actual usage.** Do `operators.rs` and
   `derived_invariants.rs` avoid every term the notation declares
   inadmissible (see the language disciplines declared in the kernel documents and code headers
   list — "violation" → "asymmetry", "flavor" inadmissible, `c` not a
   named variable, etc.)? This is checkable by direct text search and
   should be, rather than assumed.

5. **Open conditions → resolution status.** For every OC-ε or OC-E
   item declared open in the notation or
   `cross_region_energy_expression.md`, is its status (still open /
   resolved / superseded) accurately reflected in the current
   document, or has an OC been silently resolved in code without the
   declaring document being updated to say so?

6. **Cross-region invariance.** The README states operators are
   "domain and scale invariant... What changes across regions is the
   declared measurement mapping M, not the operators or their
   constraints." Does any region-specific code path in `operators.rs`
   or `derived_invariants.rs` actually violate this — i.e., does any
   function branch on region in a way that changes operator behavior
   rather than just M?

**Output:** follow `Verification_pass_protocol.md` Section 12 exactly
— located findings, cleared findings, items not verifiable, evidence
additionally required. This is not a gate; it's the same
disciplined findings-return process already used today.

---

## When to run each part

**Part I (clean-room build):** before any public push, and after any
change to `Cargo.toml`, `.gitignore`, or file layout. Cheap enough to
run often.

**Part II (cross-document audit):** before any version bump to a
canonical document (the V7 → V8 transition, for instance), and any
time a correction like CP-01's is made — a correction to one document
is exactly the situation where drift into an adjacent, uncorrected
document becomes likely. Expensive enough that it should be scoped
deliberately, per the protocol, rather than run reflexively.

---

## Relationship to existing kernel discipline

This document does not introduce new admissibility rules. It states
*when* and *how* to check that the rules already declared in
`operators_notation_and_constraint.md`, `role_separation_and_operator_
application.md`, and `Verification_pass_protocol.md` are actually
being satisfied in practice, across the whole kernel rather than one
document or one file at a time — consistent with
`Verification_pass_protocol.md` Section 9's principle that verification
of continuity, not isolated correctness, is the objective.

---

## Kernel V8 synchronization (2026-09-26)

**Wording.** Finding-based wording replaces "failure" for Part I outcomes. The language-disciplines reference previously pointed to `/areas/v7-framework.md`, a path outside this repository; it now points to the kernel documents and code headers.

**Additions to the required interface checks under Kernel V8:**

- **Check 1 (notation → code) also covers the declared-information rule.** No operator is evaluated with a substitute value for an input not declared through M. For example, persistence is not evaluated against a zero prior at the first declared observation.
- **Check 4 (language discipline) also covers the Kernel V8 vocabulary:**
  - values in D are numerical projections through M, not observables;
  - "not evaluated", not "absent", for operators and quantities;
  - "provenance not observed or incomplete", not "provenance failure";
  - DRIFT SIGNAL means legacy mathematics inserted into or before the operators, or otherwise undeclared.

This document ran for the V7 → Kernel V8 transition, as its "when to run" section anticipates.
