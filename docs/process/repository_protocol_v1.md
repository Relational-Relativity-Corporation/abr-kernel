# Repository Protocol — Metatron Dynamics, Inc.
## Declared workflow for all ABR/ABRCE repositories
### Version 1.1 — Declared August 10, 2026 (Verifier corrections applied)

Bounded over D. No claim beyond D.

---

## Purpose

This document declares the required workflow for creating, verifying, and publishing any Metatron Dynamics repository. It applies without exception to all new repositories and to updates of existing repositories that introduce functional code changes.

The protocol enforces the four-stage Verification and Validation requirement from the V7 kernel (operators.rs) — Declaration Verification, Mathematical Verification, Observable Validation, and Repeatability — at the repository level.

---

## Step 1 — Architecture (Python scaffold)

A Python script creates the repository structure. The scaffold script is the only Python in the repository.

**What the scaffold script does:**
- Creates directory structure
- Writes all source files (Rust, documentation, configuration)
- Initializes git and makes the initial commit

**What the scaffold script does not do:**
- Contain any functional code
- Implement any declared process or operator
- Make any claim about D

**File naming convention:** `create_<repo-name>.py`

**Execution:** `python create_<repo-name>.py` from the GitHub_Repos root directory. The script creates the repo as a subdirectory.

---

## Step 2 — Implementation (Rust)

All functional code is written in Rust. No exceptions.

**What belongs in Rust:**
- Domain declaration (`src/declaration.rs`) — all declared constants with provenance comments pointing to `docs/grounding/GROUNDING.md`
- Types (`src/types.rs`) — mass-conserving structs, state enums; correctness properties enforced by the type system and constructors, not by runtime checks alone
- Process models (`src/shredder.rs`, `src/sorter.rs`, etc.) — one file per declared process stage
- Operators — ABR A, B, R kernel when invoked
- Control logic — alert generation, state transitions
- Economics — derived from declared process outputs
- Dashboard (`src/dashboard.rs`) — declared operator signal layer; typed struct, not a reporting utility
- Test suite (`src/lib.rs` `#[cfg(test)]`) — all tests unconditional; no conditional assertions that can pass without exercising the declared path

**What the type system must enforce:**
- Mass conservation at every process boundary (constructor returns `Result`, rejects violations)
- Exhaustive state handling (enums require pattern matching on all arms)
- Composition integrity (sum-to-1.0 enforced at construction)

**Simulation binary:** `experiments/simulate.rs` — uses the declared dashboard layer; not a standalone reporting script.

---

## Step 3 — Verification (ZIP to cold Verifier)

Once the repository builds and all tests pass in the build environment:

1. Run `cargo test` — all tests must pass
2. Run `cargo run --bin simulate` (or equivalent) — simulation must complete without panic
3. Produce a ZIP archive excluding the `target/` build directory
4. Send the ZIP to a **cold Verifier instance** — ChatGPT with no prior context about the repository

The Verifier prompt must specify:
- The seven verification categories from the V7 protocol
- The simulation results for reference (test count, key output figures)
- A request for file-by-file source review
- An explicit request for PASS / PASS WITH CONDITIONS / NOT PASS verdict

The Verifier reviews against the V7 kernel framework. All findings must be resolved before proceeding to Step 4. If the Verifier returns NOT PASS, the Generator addresses all findings and returns to Step 3 — a new ZIP to a new cold Verifier instance.

**The Verifier prompt and the Verifier response are both retained** in the session record as part of the verification audit trail.

**Status after Step 3:** The repository holds status **VERIFIER PASS** — not VERIFIED. Verifier PASS confirms structural and mathematical admissibility of the source and declared test results. It does not confirm execution, because the Verifier may not have access to the Cargo toolchain. The distinction matters: VERIFIER PASS is a necessary but not sufficient condition for VERIFIED status.

---

## Step 4 — Origin Pass (execution on declared hardware)

After the Verifier issues PASS or PASS WITH CONDITIONS (with all conditions resolved):

The Origin (Robin Macomber) executes the repository on the declared local hardware:

```powershell
cd "C:\Users\Robin Macomber\Documents\Metatron_Dynamics\GitHub_Repos\<repo-name>"
cargo test
cargo run --bin simulate
```

**Required outcomes:**
- `cargo test`: all tests pass, zero failures
- `cargo run --bin simulate`: completes without panic, output matches Verifier reference figures

This step closes the **Repeatability** requirement from Stage 4 of the V7 Verification and Validation Protocol. Origin is a declared role that participated in the repository workflow; this is not independent execution. Origin execution on a separately declared hardware environment (Windows, local Cargo toolchain) confirms reproducibility across the two build and execution environments used in the protocol — the build environment (Linux, Cargo) and the Origin environment (Windows, Cargo).

---

## Publication Gate

**Status after Step 4:** The repository holds status **VERIFIED** — Verifier PASS (Step 3) plus Origin execution confirmed (Step 4). VERIFIED is the only status that licenses publication and citation in commercial or investor materials.

Remote repository creation and push occur **only after** Steps 1–4 are complete:

```powershell
# Create remote at github.com/Relational-Relativity-Corporation/<repo-name>
git remote add origin https://github.com/Relational-Relativity-Corporation/<repo-name>.git
git push -u origin main
```

A repository that has not completed all four steps is not published. A repository that is published without completing all four steps is marked as pre-protocol in its README until the protocol is retroactively completed.

---

## Open Conditions at Publication

Every repository is published with declared open conditions in `docs/OPEN_CONDITIONS.md`. Open conditions are not blockers to publication — they are declarations of what requires real-world measurement or external data to close. The protocol does not require open conditions to be closed before publication. It requires them to be declared.

---

## Retroactive Application

Existing repositories published before this protocol was declared are subject to retroactive review. The review sequence:

1. Identify repositories that were published without a completed Generator/Verifier pass
2. Review each at a ratio of approximately one legacy review per five new builds
3. Repositories that fail retroactive review are marked pre-protocol in their README
4. Full protocol pass is completed before the repository is cited in commercial or investor materials

---

## Relationship to V7 Kernel

This protocol operationalizes the four-stage Verification and Validation Protocol from operators.rs V7:

| V7 Stage | Protocol Step |
|---|---|
| Stage 1 — Declaration Verification | Step 3 (Verifier Category 1: M Declaration Integrity) |
| Stage 2 — Mathematical Verification | Step 3 (Verifier Categories 2–4: mass balance, process models, control logic) |
| Stage 3 — Observable Validation | Step 3 (Verifier Category 3: grounding document review) + open conditions |
| Stage 4 — Repeatability | Step 4 (Origin execution on declared hardware) |

The Verifier Criterion from V7 applies to all source files: *"Can I trace every mathematical operation continuously back to declared observables through declared operators?"* This question is the primary test applied in Step 3.

---

*Metatron Dynamics, Inc.*
*Bounded over D. No claim beyond D.*
