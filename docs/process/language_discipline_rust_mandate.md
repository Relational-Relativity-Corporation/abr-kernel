# Language Discipline — Rust Mandate

**Metatron Dynamics, Inc.** Kernel V8.
Kernel doctrine. Bounded over D. No claim beyond D.
Declared by Origin, 2026-07-30.

---

## The rule

**Rust is the only acceptable language for science- and math-based
processing anywhere in the kernel.** This applies to any code that
computes a declared observable, checks a declared relation, enforces
an admissibility rule, or produces a number that will enter a
validation record or a claim.

This is not a preference stated alongside acceptable alternatives. It
is a constraint: other languages are not used for this class of work
in this kernel, full stop.

---

## Why

The kernel's entire discipline rests on one requirement: every
quantity in play must map to a stated observable, and every step
between an observable and a claim must be traceable and auditable.
That requirement is a claim about *process*, not just about the final
number — it is not satisfied by a result that happens to be correct if
the path to that result cannot be checked.

Rust serves this requirement in a way other common languages do not,
for a specific, structural reason: **Rust's type system and compiler
eliminate many common classes of silent implementation errors and
make others explicit, reducing the likelihood that incorrect
computations enter the kernel unnoticed.** A type mismatch, an
unhandled `Option::None`, an out-of-bounds access, an unchecked
numeric parse failure — these are compile-time or explicit
runtime-panic failures in Rust, not silent `NaN`s, silent wraparounds,
or a script that returns a plausible-looking wrong answer and
continues.

This matters more than it might first appear, because one failure mode
this kernel is built to guard against is not "the code crashes" — a
crash is loud, and gets noticed. That failure mode is **a computation
that produces a number, and the number is wrong, and nothing about the
program's behavior signals that**. A dynamically-typed or
exception-swallowing language makes this failure mode easy to produce
by accident. Rust makes it structurally harder to produce by accident,
because so many of the ways a wrong number gets silently returned are
closed off by the type system before the program runs at all.

**What this does not cover, stated explicitly rather than left
implicit:** Rust's compiler has no way to know whether a constant is
physically correct, whether an algorithm implements the right
mathematics, or whether a declared relation is actually grounded in an
observable rather than in plausible-sounding theory. Incorrect
constants, incorrect physics, and logically valid but substantively
wrong computations all compile and run without complaint. The original
CP-01 justification in `abr-biological-binding` (2026-07-30) is direct
evidence of this limit — it was reasoned in prose, never reached
code, and no compiler could have caught it, because the error was in
what was claimed to be true, not in how anything was implemented.
Rust closes off *one* real category of silent failure — the
implementation-level kind. It does nothing on its own for the
observable-provenance kind, which is why that discipline is stated as
its own, separate requirement (see
`observable_provenance_and_reverse_traceability.md`) rather than
assumed to follow from the language choice.

This was stated once already, earlier in this kernel's development:
*"Rust preferred over Python for kernel work because declaration can
be enforced at compile time (Python passes failures silently,
therefore not rigorous in the sense the framework requires)."* That
was a preference, and it was correct as far as it went — bounded to
implementation-level rigor, not a claim about scientific correctness.
This document upgrades it to a requirement, for the same reason and on
the same evidence, made explicit below and bounded the same way.

---

## What this looked like in practice

This mandate is not abstract. It was arrived at through direct
experience building `abr-biological-binding` (2026-07-30), where the
discipline surfaced real gaps that a looser process would have missed:

- A component-pair admissibility rule (F9/F11, notation V10) was
  declared in prose across two documents but never enforced anywhere
  in code — `operator_a` accepted any `comp_pairs` handed to it. Rust's
  type system was used to close this: `ComponentPair`'s constructors
  make it structurally impossible to build a pair without supplying
  either a grounded observable relation or an experimental pair's
  required proposed-relation and agreement-criterion statements. A
  `DerivedQuantity` computed from an experimental pair carries that
  mark forward automatically and is refused admission to a validation
  record by an explicit, checkable method — not by convention, not by
  a comment asking a future maintainer to remember.

- A PDB parser written to read fixed-column crystallographic records
  correctly handled a real formatting quirk (chain ID and residue
  number glued together with no separating space, e.g. `HOH E1601`)
  that a naive whitespace-splitting parser — trivial to write correctly
  or incorrectly in almost any language — would have silently
  misparsed. Rust didn't prevent this quirk from existing in the data;
  it made the correct handling of it an explicit, tested code path
  rather than an assumption.

- Two real bugs were caught specifically *because* Origin insisted on
  running the code on his own machine rather than trusting a sandbox's
  passing output: a Unix-only hardcoded path that failed outright on
  Windows, and a test-isolation race condition where two tests wrote
  to the same temp file concurrently. Both were caught by Rust's own
  test harness reporting a hard failure — not a quietly wrong number.

None of this is unique to Rust in principle — a sufficiently careful
implementation in another language could avoid all of the above. The
point of the mandate is that Rust makes the careful path the *default*
path, and makes the careless path require active effort to reach,
rather than the reverse.

---

## Retroactive scope

This mandate is declared going forward from 2026-07-30. It does not
retroactively invalidate work already published under prior practice.
Several repositories built before this date used Python or other
languages for science- and math-bearing processing (for example,
`abr-gpu-amd-demo`, run on Python 3.10.11). Those repositories are not
struck from the record by this document.

They are, however, now out of compliance with kernel doctrine, and
should be tracked as such rather than left in ambiguous status. Origin's
existing plan to review the repository backlog methodically (see
`abr-repositories.md`) is the appropriate mechanism for deciding, per
repository, whether to port the processing code to Rust, retire the
repository, or explicitly except it with recorded rationale. This
document does not itself disposition any specific prior repository —
that disposition remains Origin's, made repository by repository.

## Why this is enforceable now and was not equally practical before

This mandate depends on more than discipline — it depends on a specific
infrastructure change worth stating explicitly rather than leaving
implicit, since a future reader auditing why this rule wasn't declared
earlier should have the real answer rather than assume the discipline
itself is new.

Prior to mid-2026, verifying that Rust code actually compiled and
passed its own tests required a full round trip through Origin's own
machine for every iteration — every fix, every typo, every candidate
correction. That is a real cost, and it made a hard Rust-only mandate
a heavier practical burden than the discipline alone would suggest,
because "Generator, please make sure this Rust code is exercised
before I approve of it, this cannot pass through with error, then I
review it" carried a much higher latency and friction cost per
iteration.

The Generator side of this session had, and used, a sandboxed
execution environment capable of installing a Rust toolchain,
compiling real crates, running their test suites, and iterating on
failures — before any of it reached Origin for the machine-level
confirmation that actually matters. This is *why* the CP-01 negative
check, the CP-02 geometric-premise check, and the threshold-provenance
fix could each be built, compiled, and pre-checked for gross errors in
the same turn they were conceived, rather than costing a full
round-trip cycle each. It does not replace Origin's own execution —
every actual confirmation in this session's record came from Origin's
machine, not the sandbox — but it substantially lowers the cost of
getting to that confirmation with fewer wasted round trips.

Stated plainly: this mandate is practical today in a way it might not
have been equally practical to hold to strictly some months ago,
because the tooling available to enforce it in near-real-time did not
previously exist in this form. The discipline was always correct; the
infrastructure to apply it without excessive friction is comparatively
new.

---

## Scope

This mandate covers:
- Any code implementing a declared operator (A, R, Σ, ρ, and future
  operators)
- Any code computing a distance, invariant, or derived quantity that
  will be compared against a declaration or enter a validation record
- Any code enforcing an admissibility rule (component-pair grounding,
  topology declarations, directional distinctness)
- Any parser reading external data (crystallographic structures,
  detector output, sensor traces) that will feed a declared M

This mandate does not cover:
- Documentation, prose, or declaration authoring (markdown, plain
  text)
- Build tooling, CI configuration, or repository scaffolding scripts
  where no science- or math-bearing computation occurs
- One-off, throwaway exploratory scratch work that never enters the
  record as a claim — with the caveat that anything promoted from
  scratch work into a repository or a declaration must be rebuilt in
  Rust before that promotion, not carried over as-is

---

## Relationship to existing kernel discipline

This document sits alongside, and does not replace, the existing
admissibility rules in `operators_notation_and_constraint.md` and
`role_separation_and_operator_application.md`. Those documents govern
*what* may be declared and computed. This document governs *what
language that computation is written in*, on the grounds that the
"how" is itself part of the provenance chain the kernel requires —
consistent with `Verification_pass_protocol.md` Section 9's principle
that verification of continuity, not isolated correctness, is the
objective.

It is also continuous with, not novel relative to, the kernel's
existing published contribution discipline. `abr-kernel`'s own README
already states: *"Declaration is not a substitute for traceability to
an observable through M. A quantity that is declared but not traceable
to an observable through M is not... about anything in D."* The
CP-01 correction in `abr-biological-binding` (2026-07-30) was, in
substance, exactly this rule — already declared, already canonical —
applied for the first time against a biological dataset rather than a
physics one. This document does not introduce a new principle. It
extends an existing one to cover implementation language as well as
declared content.

---

## Kernel V8 synchronization (2026-09-26)

**Scope of vocabulary.** "Failure" in this document refers to implementation-level errors in software: silent `NaN`s, unhandled parse errors, test-harness failures. Kernel V8's vocabulary for operator results does not apply to these; it governs how operator results and declarations are described. Neither this vocabulary nor the rule itself changed.

**Kernel crate.** The kernel crate is `metatron_kernel_v8` from Kernel V8. It remains Rust only, with no external dependencies.
