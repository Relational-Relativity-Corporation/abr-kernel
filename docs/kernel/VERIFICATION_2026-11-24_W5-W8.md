# Verification Order and Package — Kernel V10, 2026-11-24 (§2.10 W5–W8; the character field read by the kernel)

**Metatron Dynamics, Inc.** Kernel V10. Bounded over D. No claim beyond D.

Ordered by Origin under `docs/process/Verification_pass_protocol.md`; prepared by the Generator at Origin's request (2026-11-24). `V10_README.md` governs. **Verification returns findings, not approval** (protocol §8, §12).

**Nothing here is empirical.** The tests use enumerated synthetic fixtures. They establish that the forms compute as declared and that the stated properties hold over them, nothing about any language or system.

---

## 1. Preconditions (protocol §1)

| | Declared |
| --- | --- |
| **Bounded domain D** | Registered English text, as character progressions, read by the Kernel V10 relational field |
| **Observable mapping M** | Every character of the text registered as a locus, nothing cut. Every ordered pair x → y registered as a relation (the primitive), with identity and basis, and feedback declared on every relation that lies on a directed cycle (76th). x → x as an identity spoke beside the field (76th, option A). Components kept apart: (0) input, (1) produced, (2) declared registry witnesses. Exact counts at δ = 0 (68th, `ObservedRange::Counts`). Registrations: cumulative counts at the last steps |
| **Artifacts** | See §3 |
| **Revisions** | `abr-kernel` **`ecf52c7`** (branch `claude/eager-dijkstra-5slcq5`). `abr-relational-ai` **`1e82fe3`** (branch `claude/eager-dijkstra-5slcq5`) |
| **Purpose** | (i) Whether the character field and its tests implement relation as primitive, with energy (tunneling), connection (entanglement) and freedom (superposition) calculated by the kernel's linear, composed operators, all together. (ii) Whether the records report the work and its results honestly |

## 2. Roles (protocol §2)

**Origin**: orders the pass and decides. **Generator**: the author of everything under review, this session. **Verifier**: the external frame Origin has engaged. The Generator does not verify its own work.

## 3. Scope (protocol §3)

**Included:**

| Repository @ commit | Artifact | What it is |
| --- | --- | --- |
| `abr-kernel` @ `ecf52c7` | `src/ground_variables.rs`: `ObservedRange::Counts`, `counts()`, `admits()` | The 68th's admission of exact counts at δ = 0, implemented (it had been in the text only) |
| `abr-kernel` @ `ecf52c7` | `tests/v10_counts.rs` | Its tests (3) |
| `abr-kernel` @ `ecf52c7` | `src/coherence.rs`, `tests/v10_coherence.rs` | §2.10 W1–W5 (Proposed), with tests (8) |
| `abr-kernel` @ `ecf52c7` | `docs/kernel/V10_README.md`: §2.10 (W1–W8), Part 9 entries 73rd–90th, OC-WIN-1, OC-RANGE-1 note | Declarations and the amendment log |
| `abr-relational-ai` @ `1e82fe3` | `text-anticipation/src/afield.rs` | The character field, declared and registered for the kernel; `KernelReport` |
| `abr-relational-ai` @ `1e82fe3` | `text-anticipation/tests/kernel_use.rs` | The 10 tests of the kernel's use (90th) |
| `abr-relational-ai` @ `1e82fe3` | `text-anticipation/README.md` | The record of the application work and its results |

**Excluded:** the generating code in `text-anticipation` (`src/route.rs`, `src/subject.rs`, `src/connect.rs`, `src/words.rs`, `src/kinds.rs`, `src/kfield.rs`, `src/bin/*`), except for whether the records disclose that it does not call the kernel (Question 4). All other kernel code is unchanged since 10.3.0.

**Declared assumptions:** the kernel at `f603a16` (which `text-anticipation/Cargo.toml` pins) and at `ecf52c7` have identical source code. Since `f603a16`, only `docs/` changed. This can be checked with `git diff --stat f603a16 ecf52c7 -- src tests`.

**Declared limits:** the energy carrier for text is not declared (Origin to decide), so energy is tested with a synthetic carrier and, without one, as a named gap. No chooser yet reads `KernelReport`. Orders of relation above the alphabet are not built.

## 4. Evidence (protocol §4)

**Supplied:** both repositories at the commits above; the test suites; the result files in `abr-relational-ai/text-anticipation/results/`; this package.

**Not supplied:** the three Project Gutenberg books (*Hamlet*, *Pride and Prejudice*, *On the Origin of Species*) used in the application results. They were supplied by Origin and are not stored in either repository. Their SHA-256 hashes are in `text-anticipation/results/route-*.md`. The tests under review do not use them.

**Access:** `abr-kernel` is public. `abr-relational-ai` is **private**: the Verifier needs read access to it.

## 5. Reproduce

```bash
git clone https://github.com/Relational-Relativity-Corporation/abr-kernel.git
cd abr-kernel && git checkout ecf52c7
cargo test                      # expect 288 passed, 0 failed, no warnings
cargo test --features v9        # expect 380 passed, 0 failed
git diff --stat f603a16 ecf52c7 -- src tests   # expect no output (source identical)
cd ..

git clone https://github.com/Relational-Relativity-Corporation/abr-relational-ai.git
cd abr-relational-ai && git checkout 1e82fe3 && cd text-anticipation
cargo test --release            # expect: tests/kernel_use.rs 10, tests/route.rs 11,
                                #         tests/anticipation.rs 6, tests/subject.rs 2; 0 failed
```

The Generator checked these on 2026-11-24.

**The deliberate breaks** (the Generator's claim: each fails at least one test in `tests/kernel_use.rs`). Make one at a time, run `cargo test --release --test kernel_use`, and restore before the next.

*In the application* (`text-anticipation/src/afield.rs`):

| Break | Change | Expected failing test |
| --- | --- | --- |
| Provenance merged into the input | in `declare`, count every transition in `INPUT` instead of `PRODUCED` or `INPUT` | `produced_relations_are_registered_with_their_provenance` |
| The progression cut at "." | skip every pair with "." as either character | `relation_is_the_primitive` |
| B computed in the application | in `KernelReport::of`, take `b` from `spatial[last].a` | `the_report_is_the_kernels_own` |

*In the kernel.* Point the application at a local checkout of `abr-kernel` by appending to `text-anticipation/Cargo.toml`:

```toml
[patch."https://github.com/Relational-Relativity-Corporation/abr-kernel"]
abr-kernel = { path = "../../abr-kernel" }
```

| Break | Change in `abr-kernel` | Expected failing test |
| --- | --- | --- |
| B without its successors | `src/relational_field.rs`, `forms_of_relations`: drop `for &f in rel.succ(e) { w[f] += 1; }` | `b_is_built_from_a_and_xi_from_b` |
| PC1 on the wrong side of its bound | `src/persistence_change.rs`, `read_exact`: freedom when **not** exceeding the bound (and non-zero) | `freedom_is_change_of_relation` |
| Identity at one registration instead of every one | `src/ground_variables.rs`, `identity_pairs`: `fields.iter().any(` instead of `.all(` | `connection_is_read_from_the_relations` |
| ε without its absolute value | `src/ground_variables.rs`, `energy_of_relations`: `epsilon: v.nearest()` instead of `v.abs().nearest()` | `energy_is_read_from_the_declared_carrier` |

Restore afterwards: `git checkout -- src` in `abr-kernel`, remove the patch, and `git checkout -- Cargo.lock` in `abr-relational-ai`.

## 6. Reading order (protocol §5)

1. `abr-kernel` `docs/kernel/V10_README.md`: the governing statement; then §2.10 (W1–W8); then Part 9, the 73rd to the 90th.
2. `abr-kernel` `src/ground_variables.rs` (`ObservedRange`) and `tests/v10_counts.rs`, against the 68th (§2.8 item 2).
3. `abr-kernel` `src/coherence.rs` and `tests/v10_coherence.rs`, against §2.10 W1–W5.
4. `abr-relational-ai` `text-anticipation/src/afield.rs`, against the mapping M above and the 85th.
5. `abr-relational-ai` `text-anticipation/tests/kernel_use.rs`, against the 90th.
6. `abr-relational-ai` `text-anticipation/README.md` and `results/`, against Part 9.

## 7. Verification questions (protocol §6)

1. **Declaration → implementation (the character field).** Does `afield.rs` implement M as declared: every character a locus, nothing cut, every pair a relation with identity and basis, feedback on cycles, identity spokes beside, components apart, counts at δ = 0? Does it compute any reading itself, or only declare and register?
2. **Declaration → tests (comprehensive without being overly restrictive).** Do the 10 tests in `kernel_use.rs` establish what each claims: relation as primitive; linearity of A, B, Ξ; B from A and Ξ from B; freedom, connection and energy as the kernel defines them from the relations; all four in one formation (Df6); provenance; one direction? Is anything claimed but not tested, tested only trivially, or over-restrictive (fixing implementation detail rather than a property)? Do the fixtures exercise the cases claimed (the "can tell" assertions)?
3. **Implementation → execution.** Do the suites pass as stated, and does each deliberate break in §5 fail the test stated?
4. **Declaration → implementation (kernel).** Does `ObservedRange::Counts` implement the 68th exactly: non-negative integers to 2⁵³ admitted at δ = 0, the human range for δ > 0, the human range itself unchanged?
5. **Validation record → claim.** Do Part 9 entries 73rd–90th and `text-anticipation/README.md` report the work accurately: what was built, what was measured, what was decided by Origin and what was taken as decided "for correction", and above all that **the generating code (`route.rs`, `subject.rs`, `both`, `steer`, `split` and the others) never called the kernel** (stated in the 84th and the 90th)? Is any result presented as more than it is?

## 8. Evidence discipline (protocol §7)

Each conclusion is classified **Verified**, **Inference** or **Not Verifiable**. Findings are located by repository, file and line (§8).

## 9. Out of scope (protocol §11)

The theoretical correctness of the framework; whether generated text is coherent; the scientific interpretation of the application results; performance (the kernel's speed on large fields is a known open item); engineering style; future work (the chooser, the energy carrier, orders of relation).

## 10. Output (protocol §12)

1. The declared question. 2. The answer. 3. Supporting evidence. 4. Located findings. 5. Cleared findings. 6. Items not verifiable from supplied evidence. 7. Evidence additionally required.

Return the report to Origin.
