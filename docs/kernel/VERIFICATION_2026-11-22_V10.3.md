# Verification Package — Kernel V10.3, 2026-11-22

**Metatron Dynamics, Inc.** Kernel V10.3. Bounded over D. No claim beyond D.

Prepared by the Generator for the Verifier and Origin, **before** this work is merged. It covers everything on branch `claude/kind-keller-is6bq1` since `main` at `c1228a2` (Kernel V10.2 with the second-pass dispositions), up to the release commit `717ca2a`, plus the commit that adds this file. `V10_README.md` governs. The earlier passes are in `VERIFICATION_2026-11-21.md`. The application change is in §9.

**Verifier criterion (KR §17):** *Can every mathematical operation be traced continuously back to declared observables through declared operators?* Provenance is the question, not correctness.

**Nothing here is empirical.** Every test uses synthetic fixtures or random fields. They establish that the forms and checks compute as declared and that the derived results hold over the forms, nothing about any system. No registered record has yet been read.

---

## 1. Reproduce

```bash
git clone https://github.com/Relational-Relativity-Corporation/abr-kernel.git
cd abr-kernel && git checkout claude/kind-keller-is6bq1
cargo test                                  # expect 186 passed, 0 failed (179 before §12, 183 before §13)
cargo test --features v9                    # expect 278 passed, 0 failed (271 before §12, 275 before §13)
cargo run --example v10_relational_field    # runs to completion
cargo build --all-targets --features v9     # expect no warnings
```

Reproduced by the Generator from a fresh clone with no build cache at `717ca2a`: 179 and 271 passed, no warnings.

## 2. Commits

| Commit | Part 9 | What |
|---|---|---|
| `e16b289` | 24th | Z1a δ in two parts, with the gain-sign condition; the A-3 record-boundary flag |
| `b2e8174` | 25th | Renames E6, E7, OC-Z-1 (text and comments, originals kept beside) |
| `709169c` | 26th | I-SC-D14: the held minor pattern and the sustained circulation are independent grades; OC-SC-4 closed |
| `9c7e326` | 27th | T8 check (`topology`); the T8 row's claim corrected |
| `0797685` | 28th | T8 enforced at the V10 reading entry points (Origin's decision) |
| `0f260f4` | 29th | K7 route enumeration and a cycle basis from a stated spanning tree |
| `beef76f` | 30th | RPPI-V10 admission spec (Proposed) and the kernel share of the reverse check |
| `1c5efda` | 31st | Relation identity is a kernel input (RPPI-V10 G-1, Origin adopted (a)) |
| `717ca2a` | 32nd | Release: package `10.3.0` (Origin decision) |

## 3. Items

| Item | Code | Tests | What to check |
|---|---|---|---|
| **Z1a**, δ in two parts | `Registration::with_run_constant`, `state_delta`; `EdgeSeries` fields `da` (state), `da_rep`, `dgain`, `shared_gain`; `EdgeSeries::change`, `change_magnitude`, `with_run_constant` | `tests/v10_z1a_a3.rs` (10 Z1a tests, including 1 property test) | Offsets enter state forms only; the unshared gain enters the state bound as γ_i·\|x_i\| and the change bound as γ_s·\|Δx_s\| + γ_t·\|Δx_t\|; the shared gain enters only `change_magnitude`; `Registration::new` reads as before |
| **Gain-sign condition** (the Verifier's qualifier) | gain bounds refused unless in [0, 1) | `pc_d3_negative_scale_reverses_signs_only` (300 random fields, vacuity counters) | A negative scale is a declared mapping: resolutions, held states and crossings unchanged; first-order signs reverse; a minor reverses with the product of its components' scale signs |
| **A-3** | `relational_field::boundary_conditioned_edges`; `Sustained.boundary_conditioned` | `tests/v10_z1a_a3.rs` (3) | Exactly one empty side flagged; both empty not formable and not flagged |
| **Renames** | docs and comments | — | §6 |
| **I-SC-D14** | none (a derivation on existing forms) | `tests/v10_grade_independence.rs` (6 worked, 1 property over 2000 fields) | All four combinations on one field at one δ, through `minor_histories` and the sustained grade; the two laws exactly |
| **T8** | `topology::check_t8`, `declare_checked`, `t8_gap`; `DeclaredRelations::with_feedback` | `tests/v10_topology_t8.rs` (14; property against Kahn's algorithm over 3000 fields) | Self-relation refused; every directed cycle needs a declared feedback edge with a basis; refusals name a cycle per strongly connected part; admission names a cycle each feedback edge closes |
| **T8 enforcement** | `admit_field` called by `RelationalField::new`, `EdgeSeries::from_registrations`, `check_sequence` (connection grades, minor histories, the first V2 form), `energy` | `tests/v10_topology_t8.rs` `entry_points` (4) | Every entry point refuses; a declared loop is read; edge-valued series are not checked (no topology declared) |
| **K7** | `topology::routes`, `is_route`, `two_route_cycle`, `cycle_basis`, `express_in_basis`, `basis_closures` | `tests/v10_topology_k7.rs` (10 worked, 2 property: routes against brute force over 1500 fields; basis size and exact chord combination over 1500 fields) | Basis size m − n + c; a stated tree that is not a spanning tree refused; any closed walk's closure is its chord-coefficient combination of the basis closures |
| **RPPI-V10** | `docs/kernel/rppi_v10.md` (Proposed); `reverse_check` | `tests/v10_reverse_check.rs` (2 property over 1500 fields each, 4 tampering) | The spec's link table against the code; the reverse check shares no code with the forms; MATCH on every reading, MISMATCH on every tampering |
| **Relation identity** (G-1) | `topology::RelationIdentity`, `IdentityRefusal`, `check_identity`, `admit_field`; `DeclaredRelations::with_identity`; `DeclarationGap::RelationIdentity` | `tests/v10_relation_identity.rs` (5) | Refused: none declared, count mismatch, empty id, empty basis, repeated id; checked before T8 |
| **Release** | `Cargo.toml` `10.3.0` | — | README release notes state the release is breaking for V10 callers |

## 4. Existing tests: what changed

**No existing reading changed.** Every test that passed at `c1228a2` passes with the same expected values.

Fixtures changed so that they declare what 10.3.0 requires, each by declaration, never by bypassing a check:

- **Synthetic identity.** Every field built in a V10 test file is wrapped in `synthetic_identity::identify` (`tests/support/synthetic_identity.rs`): ids `SYNTHETIC-e{index}`, basis marked SYNTHETIC.
- **Synthetic feedback.** The random generators (`random_rel` in `v10_properties`, `v10_z1a_a3`, `v10_grade_independence`, `v10_reverse_check`, `v10_topology_k7`; `random_cycle` in `v10_properties`) wrap their fields in `synthetic_feedback::declare_loops` (`tests/support/synthetic_feedback.rs`): every edge from a higher to a lower index that lies on a directed cycle is declared feedback, basis marked SYNTHETIC. The rest of the field is then acyclic.
- **The A-3 fixture** declares its closing edge 3 → 1 as feedback explicitly.
- **`t8_declare_checked_carries_its_feedback_onto_the_field`** now first asserts that the field is refused for identity, then declares it.
- **The example** `v10_relational_field` declares ids and a SYNTHETIC basis.

## 5. Derivations to check by hand

1. **Z1a.** For y = (1 + γ_i)·x + o_i at locus i: A = y_s − y_t carries o_s − o_t (state bound: o_s + o_t) and γ_s·x_s − γ_t·x_t (state bound: γ_s·|x_s| + γ_t·|x_t|). Δ over a step cancels o and carries γ_s·Δx_s − γ_t·Δx_t (change bound: γ_s·|Δx_s| + γ_t·|Δx_t|). A gain common to every locus scales A and Δ by one positive factor, so no |·| > δ comparison made in registered units changes; its bound enters a reported |Δ| as γ·|Δ|. *The state bound's unshared-gain term is the Generator's extension of the confirmed correction, which stated the change bound; check it.*
2. **Gain-sign condition.** A gain bound γ < 1 keeps 1 + γ′ > 0. Under x → a·x with δ → |a|·δ, every |·| > δ comparison is unchanged; first-order forms scale by a; a minor over components i, j by a_i·a_j.
3. **I-SC-D14.** (i) m_ij(e, f) under A[·][e] → λ_e·A[·][e] is λ_e·λ_f·m_ij(e, f). (ii) With A′ = R·A over components, m′ = det R · m and Ξ(B(A′)) = R·Ξ(B(A)) by linearity of B and Ξ within a component. Then the worked field: Ξ(B(A_1)) = (5, 4, −1, −7), Ξ(B(A_2)) = (5, 4, −2, −7) on the path 0 → … → 4.
4. **T8.** Every directed cycle contains a declared feedback edge ⇔ the field less its feedback edges has no directed cycle. A feedback edge (s, t) lies on a directed cycle ⇔ t reaches s.
5. **K7.** For a spanning forest T, each chord c's fundamental cycle contains c once and no other chord. A closed walk's signed edge-count vector lies in the cycle space, which the fundamental cycles span, so it equals Σ_c n_c·(cycle of c), with n_c the walk's net count of c. Closure is linear in the signed edge counts, so the walk's closure is Σ_c n_c·closure(c), exactly on dyadic values.
6. **Reverse check exactness.** `reverse_check` repeats each form's order of floating-point operations: (δ_s + δ_t) at s+1, plus (δ_s + δ_t) at s, plus the gain term, for the change bound; the same products and sums for the minors. Equal inputs therefore give bit-equal results. Check the order against `persistence_change.rs`.

## 6. Provenance of the text

A deleted-lines check over every document changed since `c1228a2` (51 removed lines) finds no original content lost. The only characters deleted from documents are:

- the decided renames ("coherence potential" → "coherence density"; σ → N_δ), each with *was:* beside it;
- superseded test counts in `README.md`, with the earlier counts recorded beside;
- "OC-SC-1 to OC-SC-4" → "OC-SC-1 to OC-SC-3" in `kernel_reference.md`, with *was:* beside;
- a bold marker in the V6 row, where "Κ, connected relations per locus" became "Κ, connection density (connected relations per locus)".

Every other amendment is inserted beside the text it amends. The V10.2 status list in `README.md` is kept as written under the V10.3 status. Code changes in `src/` (16 removed lines) are the Z1a and identity changes listed in §3; no function was removed.

## 7. Corrections and choices to confirm

**Corrected:** the T8 row said acyclicity "is checked and refused if violated (V10 code)". No check existed before 10.3.0. The correction is beside the row.

**The Generator's choices, for the Verifier and Origin to confirm:**

1. **T8 read with Origin's clarification of 2026-11-07** (`kernel_reference.md` §8): a cyclic arrangement is admitted as a declared feedback loop with a basis, rather than every cyclic field being refused. Opposite directions are treated as a 2-cycle needing their own basis.
2. **A-3 flagged edges stay in the sustained grade's held and reorganizing lists**, with the flag beside them, rather than being removed from the grade. *[Refined at the Verifier's recommendation, §13: they stay in the per-edge lists, and the headline share counts only edges with both sides registered.]*
3. **Z1a's unshared gain enters the state bound** (§5, item 1), as well as the change bound the correction stated. *[Both bounds corrected to the scaling model, §13.]*
4. **Z1a for edge-registered values:** one gain per edge is treated as shared by that relation *[Found wrong by the Verifier; corrected, §12]* (it changes no classification), since a relation registered as one value carries one gain at both its ends.
5. **Identity is checked before T8**, so a field lacking both is refused for identity first.
6. **Enforcement placement:** the constructor `from_edges_with_provenance` stays a building block and checks nothing; the reading entry points check (Origin's decision for T8, applied to identity as well).

## 8. Breaking for V10 callers (Origin decided to keep 10.3.0)

Every field read through a V10 entry point must declare an identity and direction basis for each edge, and feedback for each directed cycle, or the reading is refused. `DeclarationGap` has two new variants (`T8`, `RelationIdentity`), and `Registration`, `EdgeSeries`, `Sustained` and `DeclaredRelations` have new public fields. A record declared only with `Registration::new` reads its δ as before. The V9 code behind the fence does not pass through the V10 entry points and is unaffected.

## 9. The application change (`abr-relational-chemistry`)

Branch `claude/kind-keller-is6bq1`, one commit since `main` `c0357fe`: the vessel ingest's `relations(field)` declares each relation's id and direction basis from `edges.csv`, and a new test checks that every field passes kernel admission. The ingest already refused an empty basis itself (`Problem::NoDirectionBasis`).

**It builds only against kernel 10.3.0.** Its `Cargo.lock` pins `abr-kernel` `main` at `663964a` (package 10.2.0). Reproduce with both repositories cloned side by side, both on `claude/kind-keller-is6bq1`. The patch path must be absolute, and the lock must be moved to the patched 10.3.0 first. Otherwise cargo leaves the patch unused, with a warning, and builds against the locked 10.2.0, which fails to compile:

```bash
cd abr-relational-chemistry/water/vessel/ingest
P="patch.'https://github.com/Relational-Relativity-Corporation/abr-kernel'.abr-kernel.path='$(realpath ../../../../abr-kernel)'"
cargo update -p abr-kernel --config "$P"
cargo test --config "$P"                     # expect 43 passed
git checkout Cargo.lock                      # restore the pinned lock
```

Reproduced by the Generator after the version bump: 43 passed, and the `profile` example runs.

**Merge order:** kernel first; then, on the application branch, `cargo update -p abr-kernel` to the new kernel `main`, and merge the application.

The application's vessel field has no directed cycle (heater → water → wall → air), so T8 refuses nothing there.

## 10. Open, and not verified by this package

- **RPPI-V10 gaps** (`rppi_v10.md` §6): G-2, declared projections C have no code; G-3, the reverse check covers PC1 to PC3 only (not the state connection grades, energy or cycle closure); G-4, links 1 and 2 and the order of the sequence are application duties; G-5, synthetic tests only.
- **The "no-split" test** listed in V10_README Part 4 is not written.
- **Route enumeration** is limited by a stated count and can be incomplete; `complete` reports it.
- **I-SC-D14** is shown for the grades as the code summarizes them; the worked cases are built so that each "held" case holds every reading and each "reorganizing" case has crossings, so the result does not rest on the summary rule.
- **Registered data:** OC-SC-1 to OC-SC-3, OC-T-1, OC-T-2, OC-RE-1, P-a to P-f. No registered record has been read.

## 11. For Origin after verification

1. Disposition of the Verifier's findings, and of the six choices in §7.
2. Status of `rppi_v10.md` (Proposed → Declared, or amended).
3. The merge of both branches, in the order of §9, with every commit kept.

## 12. Verifier's review (2026-11-22) and disposition

**Reproduced by the Verifier:** kernel `717ca2a`, 179 and 271 passed, the example ran, no warnings; chemistry `9762324`, 43 passed and the profile example ran with the documented patch. The committed application lockfile still targets 10.2.0, so the kernel-first merge order and the lockfile update are required.

**Finding (blocker), accepted: independent positive gains can change a PC2(a) classification on edge-registered data, and the same holds for PC3 state minors.** `EdgeSeries::with_run_constant` took a gain per component and edge and excluded it from every classification. The invariance holds only when both terms of a minor get the same factor: a gain common to a whole component, or to a whole edge. In m = Δ_i[e]·Δ_j[f] − Δ_j[e]·Δ_i[f], a factor on Δ_i[e] alone scales one term. The Verifier's case: changes (1, 1) and (1, 1) at zero repeatability read structure; 1.125 on one entry made the kernel read freedom. The Generator's earlier argument ("a per-edge factor preserves every minor's sign") assumed the factor was the same in every component.

**Disposition (both remedies the Verifier named, each where it applies):**

1. **Enforce the condition for shared gains.** `EdgeGains { component, edge, independent }`. The `component` and `edge` parts are the only shared forms; their combined factor (1 + γ_c)(1 + γ_e) enters `change_magnitude` only.
2. **Carry independent gains into the affected minor bounds.** The `independent` part enters the bound of each entry inside every minor that mixes components, PC2(a) and the PC3 state minors, as γ·|value|. It does not enter PC1 or PC2(b): a factor on one entry scales Δ there by one positive factor (PC1), and over one component the same entries carry the same factor at both steps (PC2(b)). It does not change which edges enter a pair (that is PC1's reading).
3. **Node-derived series are unaffected.** Their unshared (per-locus) gains were already in the state and change bounds, and a shared gain is per component, which scales whole components.
4. **The reverse check mirrors the correction** (`Inputs::Edges` takes the independent gain) and still matches exactly over random fields with independent gains.

**Tests added** (`tests/v10_z1a_a3.rs`, 4):
- the Verifier's case: with the gain declared on the entry, both the unscaled and the scaled values read structure, and PC1 is unchanged;
- a factor common to a whole component, or a whole edge, leaves the minor at 0;
- property over 300 random fields: shared factors, with δ scaled, change no reading;
- property over 300 random fields: independent factors within the declared bound never reverse a resolved PC2(a) or PC3 minor, PC1 and PC2(b) are unchanged, and without the declaration some resolved minors do reverse (exercised).

**Tests now: 183 (default), 275 (`--features v9`)**, no warnings.

**A-3 (checked by the Verifier):** exactly one empty side is flagged and both empty is not formable. Flagged edges still count in the sustained grade's held and reorganizing lists, as disclosed (§7, choice 2). That remains Origin's decision on how the summary is read.

**Review this state:** kernel `claude/kind-keller-is6bq1` at the commit that adds this section; chemistry unchanged at `9762324` (it declares no gains, and its 43 tests pass against this kernel).

## 13. Verifier's second review (2026-11-22) and disposition

**Reproduced by the Verifier at `54e5728`:** 183 and 275 passed, no warnings; chemistry `9762324`, 43 passed and the profile example ran against the revised kernel with the local patch. The original Z1a classification finding (§12) is fixed.

**Finding, accepted: the gain bounds did not follow the stated scaling model.** Under the model, a gain scales the registered value together with its repeatability bound. An entry with value v and bound h therefore lies in (1 + γ′)(v ± h), |γ′| ≤ γ, at distance at most γ·|v| + (1 + γ)·h = h + γ·(|v| + h) from v. `change_magnitude` added γ·|Δ| and left out γ·δΔ. The Verifier's case: Δ = 1, δΔ = 1/8, γ = 1/8 reported an upper endpoint of 1.25, while the model permits 1.125 × 1.125 = 1.265625.

**Disposition.** Every gain term now uses h + γ·(|v| + h):

| Bound | Was | Now |
|---|---|---|
| `change_magnitude` (shared and independent gains) | δΔ + γ·\|Δ\| | δΔ + γ·(\|Δ\| + δΔ) |
| Independent-gain widening, PC2(a) and PC3 (edge-registered) | h + γ·\|v\| | h + γ·(\|v\| + h) |
| Node-derived per-locus gain, state bound | δ + o + γ·\|x\| | δ + o + γ·(\|x\| + δ) |
| Node-derived per-locus gain, change bound, per end | γ·\|Δx\| | γ·(\|Δx\| + δ_s + δ_{s+1}) |

The Verifier named the first two. The Generator found the last two in the same review: the same omission in the node-derived gain. The reverse check mirrors all four and still matches exactly. In the PC3 widening, h includes the offset bound, so the widening is conservative there (an offset is not scaled by the gain).

**Tests** (`tests/v10_z1a_a3.rs`):
- the Verifier's case: the upper endpoint is 1.265625;
- `prop_z1a_every_gain_bound_covers_the_scaling_model`: every corner (1 ± γ)(v ± h) lies within the bound, for magnitudes, the node state bound and the node change bound, over 300 random fields. **It fails under the old formula**, which was checked by reverting it;
- `prop_z1a_mixing_minors_hold_under_gain_and_repeatability`: over 300 random fields, with true values (1 + γ′)(v + ε), every PC2(a) and PC3 minor resolved under the declared bounds keeps its sign. **It also passes under the old widening**, because the omitted γ·h term is small beside the interval product's own cross terms. The new widening's sufficiency therefore rests on the interval argument (an interval product over entries each within v ± H bounds the minor), not on this test;
- three hand-computed tests updated to the corrected values, with the earlier values noted beside.

**A-3 summary, the Verifier's recommendation, applied (Origin to confirm).** Flagged edges stay in the per-edge held and reorganizing lists. `Sustained.share_held`, the headline, now counts only formable edges with both sides registered, and is `None` when there are none. The flagged edges are reported apart (`share_held_boundary`). The earlier headline over all formable edges is kept as `share_held_all`. One existing test changed: `sustained_edge_with_no_neighbour_is_not_formable` asserted `share_held == 1.0` on a chain whose two formable edges are both boundary-conditioned. It now asserts `share_held_all == 1.0`, `share_held == None` and `share_held_boundary == Some(1.0)`, with the old assertion noted.

**Tests now: 186 (default), 278 (`--features v9`)**, no warnings. Chemistry `9762324`: 43 passed and the profile example ran against this kernel. The chemistry branch uses none of the changed bounds or fields.

**Review this state:** kernel `claude/kind-keller-is6bq1` at the commit that adds this section; chemistry unchanged at `9762324`.

## 14. Sign-off and merge (2026-11-22)

**The Verifier confirmed** the state at `3d8781c` (§13). **Origin directed the merge of both repositories**, with the A-3 summary as applied in §13. Merge order as in §9: the kernel to `main` first; then the application's `Cargo.lock` moved to the new kernel `main`, its tests rerun, and the application merged. Every commit kept.


## 15. After the merge: post-merge batch 1, for the Verifier (2026-11-23)

Not part of the signed-off state (§14). **This is a stopping point.** Origin chose derived invariants first, in the Generator's order. The first two are done, and the second rests on code the first corrects, so they are verified together before the next family (the circulation identities, items 3 and 4) begins.

**Reproduce:** kernel branch `claude/vibrant-tesla-2fp7ve` at the commit that adds this section. `cargo test` gives 197 passed and `cargo test --features v9` gives 289, with no warnings. The application at `main` `94a89c1` passes 43 against this kernel with a local patch (run `cargo update -p abr-kernel` under the patch first; check that the local kernel compiles). It declares no gains and uses no closures.

**Items** (V10_README §2.6 and Part 9, 37th and 38th):

| # | Item | Kind | Code change | Tests |
| --- | --- | --- | --- | --- |
| 1 | **I-SC-D15**, scale invariance stated whole | Derivation (Proposed) | none | `tests/v10_scale_invariance.rs` |
| 2 | **K7 closures carry per-edge and per-entry gains** (I-SC-D15 (vi)) | **Correction of 10.3.0 code** (Proposed; Origin to confirm) | `EdgeSeries::closure_gain`; `cycle_closure`, `cycle_change_closure` | same file; the two closure tests failed on the 10.3.0 bound |
| 3 | **I-SC-D10 qualified**: its implication holds for δ as repeatability alone | Qualifier beside the text | none (comment) | `isc_d10_with_an_offset_…` |
| 4 | **I-SC-D16**, closure over a cycle basis | Derivation (Proposed) | none | `tests/v10_closure_basis.rs` |

**Item 2, the finding.** On edge-registered values, a declared per-edge gain (`EdgeGains::edge`) or per-entry gain (`EdgeGains::independent`) did not reach K7 closures (`cycle_closure`, `cycle_change_closure`, and so `basis_closures`). A closure sums one component across edges, so a factor that differs between edges moves it. The §12 disposition ("the shared parts enter `change_magnitude` only") holds for PC1, PC2 and PC3 only.

Example: on a triangle with values (1, 1, 2), a gain of 1/8 declared on the third edge permits 2.25 there. That gives a closure of −0.25, which 10.3.0 read against a bound of 0. **Correction:** each closure term's bound is h + γ·(|v| + h), with γ = (1 + γ_e)(1 + γ_ce) − 1. A per-component gain does not enter. With no gains declared, nothing changes. The reverse check does not cover closures (RPPI G-3), so nothing there mirrors it.

**To check by hand:**
1. The D15 table, cell by cell. The rule in (i), that one factor per term is equivalent to invariance on every field. The per-registration column (iii) and the sign rule (iv).
2. The closure widening's interval argument (D15 (vi)). Inner sum Σ s_e (1 + γ′_e)(1 + γ′_ce)(v_e + ε_e), with the component factor taken out as a positive common factor.
3. The D10 qualifier: with an offset declared, both state closures can hold while the change closure is resolved.
4. D16 (i): the tree-edge argument for c_w = Σ n_i·c_{C_i}.
5. D16 (ii): the diamond.
6. D16 (iii): the difference-constraint equivalence, including the step from closed walks to simple cycles.

**What the tests can and cannot tell apart:**
- The D15 table test checks every cell both ways: a ✓ cell must hold on all 300 fields, and a ✗ cell must change at least one.
- The identity and sustained-grade E and X cells are not reachable in code (those grades read node-derived fields only). They rest on the arithmetic and D14(i).
- The energy row rests on (i) and a single-factor test.
- D16 (iii) was run with a control: the wrong statement ("every basis closure holds") in place of "every simple cycle holds". The control failed.
- D16 (ii) and the tree dependence are existence claims. The property test only requires them to occur, on near-node fields. The diamond is the worked case.

**For Origin after the Verifier:**
- declare or amend I-SC-D15 and I-SC-D16;
- confirm the closure correction, and whether it ships as 10.3.1; *[superseded (Origin, 40th): versioning frozen at the pre-release `10.3.0`]*
- whether the kernel should gain an all-cycles closure test (D16 (iii), by shortest paths).

The `V10_README.md` header still reads package `10.2.0`, which has been stale since 10.3.0.


## 16. Verifier's disposition of batch 1 (2026-11-23) and response

**Reproduced by the Verifier at `a0bf7ec`:** 197 (default), 289 (`v9`), and 43 ingest tests against the locally patched kernel, with no warnings. Both gain tests failed with the old closure bounds restored. The D16 control failed when "every basis closure holds" replaced "every simple cycle holds".

| # | Disposition | Response |
| --- | --- | --- |
| 1 | **I-SC-D15: amend before declaring.** Scope the rule to the listed forms; say that classifications are preserved in passing cells while signed outputs transform as specified; word the gain example on the nominal (1, 1, 2) and the permitted (1, 1, 2.25) triangle. The D10 qualifier was verified (±1/8 against 3/16 in state; −1/4 against 0 on change) | Amended beside the text (V10_README §2.6, D15 (i) and the finding) |
| 2a | **K7 correction: mathematics confirmed.** Factoring out the positive component gain gives the conservative per-term bound h + γ(\|v\| + h) | — |
| 2b | **Finding: a small gain rounds away.** (1 + γ_e)(1 + γ_ce) − 1 with γ = 2⁻⁵⁴ on magnitude 2⁵⁴ gave −0.5 ± 0, not −0.5 ± 1 | Fixed: `combined_gain(a, b) = a + b + a·b`. The Generator found the same form in the shared gain and in `change_magnitude`, and fixed both. Regression test fails on `a0bf7ec` |
| 2c | **Finding: `cycle_closure` indexes before validating the walk**, so an open walk with an out-of-range registration panics | Fixed: walk checked first, also in `cycle_change_closure`, where the order predates 10.3.0. Regression test fails on `a0bf7ec` |
| 2d | **Compatibility:** the public field `closure_gain` breaks downstream struct literals and exhaustive patterns. Record an explicit decision before choosing the release number | **Owed by Origin** (options in the Generator's report). **[Decided (Origin, 40th): versioning frozen at the pre-release `10.3.0`. The field stays; the break is recorded against `10.3.0`; no 10.3.1.]** |
| 3 | **I-SC-D16: declare, with the interval scope explicit** (one state or one change; no joint calibration witness across registrations). Approve a separate all-cycles check (Bellman–Ford; report a resolved cycle; state the boundary comparisons) | Scope added beside (iii). The check awaits Origin's approval |

**Tests now: 199 (default), 291 (`--features v9`)**, no warnings. The application at `94a89c1` passes 43 against this kernel.


## 17. Batch 2: exact sums, for the Verifier (2026-11-23)

**Why.** Origin declared (Part 9, 41st): *decompose, do not simplify.* Applied to the arithmetic: a K7 closure was a left-to-right f64 sum, so the cycle over (2⁵⁴, −0.5, 2⁵⁴) read −0.5 from one starting locus and 0 from the other two. Traversal is an order of analysis (Df2a), so its order must not change a reading.

**What changed.** `src/exact.rs` keeps sums exactly as non-overlapping partials, as in Shewchuk's method behind Python's `math.fsum`, with products made exact through a fused multiply-add. A value is rounded once to the nearest f64; a bound is rounded up to the smallest f64 at or above it. Applied to `cycle_closure` and `cycle_change_closure` (the change taken from the two registrations, not the rounded Δ), `operator_b` (spatial), `operator_circulation` and `circulation_half_width`. The new `circulation_of_accumulation` and its half-width compute Ξ(B(A)) from A in one exact sum; the sustained grade and `RelationalField` read them.

**To check:**
1. The guarantee in `exact.rs`: with fl(v) the value rounded to nearest and H the bound rounded up, a resolution |fl(v)| > H implies |v| > h exactly, since rounding is monotone and H ≥ h.
2. The port of `fsum`'s final rounding, including the half-way case.
3. `up`, which returns the smallest f64 at or above the exact sum, decided from the sign of the residual expansion.
4. The coefficients of Ξ(B(·)).

**Tests** (`tests/v10_exact_sums.rs`, 5). Every check is against an exact i128 reference, on dyadic values from 2⁰ to 2⁵⁴:
- sums and integer combinations;
- closures from every starting locus, with declared gains: value, minimal upper bound, and resolution read only where it holds exactly;
- Ξ(B(A)) and its bound.

All five fail when the sums are switched back to sequential f64 addition, and every earlier test passes unchanged.

**Not covered:** the rounding points in OC-NUM-1. **Reproduce:** 204 (default), 296 (`--features v9`), no warnings; the application at `94a89c1` passes 43 against this kernel.


## 18. Batch 2, completed: every reading exact (2026-11-23, for the Verifier)

Batch 2 is §17, the exact sums, together with this section. Origin decided (Part 9, 44th): the "⇐" direction of D16 (iii) is fenced as a declared intersection; exact summation is confirmed as necessary; the Bellman–Ford check is dropped. The governing statement (43rd) heads V10_README.

**What changed (OC-NUM-1, 45th).**
- The relation value A = x[s] − x[t] is held exactly: `EdgeSeries::a_lo` sits beside the rounded `a`, from two-sum.
- Every change, minor, identity difference, Ξ(B(A)), closure and `RelationalField` form is computed exactly from those values.
- Stored bounds are rounded up from their exact values: state, repeatability, gain terms, `combined_gain`, δ_ε.
- Bounds formed from them are exact sums, rounded up when reported.
- Readings are decided exactly: `read_exact`, `sign_at_delta_exact`, `ExactField`.
- The reverse check is rewritten exactly, apart from the forms.
- In the application (`water/vessel/ingest`), both reverse checks are re-derived exactly through `pc1_exact`.

**To check:**
1. The guarantee: a resolution is read only where the exact value exceeds its exact bound. The only possible loss is a value between the exact bound and the bound built from stored parts rounded up.
2. `Expansion::times` and `product_bound`.
3. That `exact_field` and `EdgeSeries::from_registrations` store the same state bound, and the reverse check mirrors it.
4. That `RelationalField`'s persistence forms are formed from the two registrations' x, not from rounded A.

**What the tests can and cannot tell apart** (`tests/v10_exact_readings.rs`, against an exact i128 reference):
- PC1, PC2, the V1 identity and sustained grades, the energy floor, and the forms all fail on the earlier arithmetic.
- The PC3 property passes on both; the worked minor 2⁻⁶⁰ (exact positive, f64 zero) is what separates them.
- One earlier assertion changed: `combined_gain(2⁻⁵⁴, 2⁻⁵⁴)` is now rounded up.

**Merge order:** the application uses the kernel's `exact` module, so the kernel merges first, then the application's lockfile moves.

**Reproduce:** kernel `claude/vibrant-tesla-2fp7ve`: 210 (default), 302 (`v9`), no warnings. Application `claude/vibrant-tesla-2fp7ve` against it with a local patch: 43, no warnings.


## 19. Verifier's batch-2 disposition (2026-11-23) and response

**Reproduced by the Verifier at `80828f4` / `ee0cf14`:** 210, 302 and 43, no warnings. Hand checks passed: the rounding and products within range, the `fsum` correction, the residual-based `up`, `times`, `product_bound`, the Ξ(B(·)) coefficients, the shared state bound, the persistence forms and `pc1_exact`.

| # | Finding | Response |
| --- | --- | --- |
| 1 | A false PC3 resolution: values near 2⁻¹⁰⁷⁴ made a product's residual underflow, and finite-input admission accepted the record | Every product of non-zero factors below 2⁻⁹⁶⁹, or not finite, is refused (`Expansion::add_product`). V10 admission refuses values, bounds, offsets and gains outside 0 ∪ [2⁻²⁰⁰, 2²⁰⁰]. The Verifier's record is refused |
| 2 | `[MAX, MAX, −MAX]` gave NaN, `[MAX, −MAX, MAX]` gave MAX | A sum that overflows is refused (`Expansion::add`). Outside the admitted range a sum may be refused in one order and computed in another, but it is never wrong |
| 3 | The sustained grade rounded its final bound before deciding; the K7 pair gives only the one-sided guarantee | The sustained bound is kept exact through the decision. K7 gains `cycle_closure_resolved` and `cycle_change_closure_resolved` for the exact verdict; the pairs are documented as one-sided |

**Origin's point on range** (Part 9, 46th; OC-RANGE-1): the 2⁻²⁰⁰ to 2²⁰⁰ limit is an arithmetic backstop, not the observed range. The observed range is to be declared and established by comparison to the human reference frame; that decision is open.

**Tests:** `tests/v10_exact_range.rs` (11): each finding as a regression; admission at the range's edges; and a property that no reading of a field drawn from the whole admitted range is refused. **221 (default), 313 (`v9`)**, no warnings. The application at `ee0cf14` passes 43 against this kernel.

---

*Metatron Dynamics, Inc. Kernel V10.3.*
