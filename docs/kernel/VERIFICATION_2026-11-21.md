# Verification Package — Kernel V10, 2026-11-21

**Metatron Dynamics, Inc.** Kernel V10. Bounded over D. No claim beyond D.

Prepared by the Generator for the Verifier and Origin, **before** this work is
merged. It covers everything on branch `claude/kind-keller-is6bq1` since
`main` at `70418df` (Kernel V10.1). `V10_README.md` governs. The application
work in `abr-relational-chemistry` has its own package, `VERIFICATION_2026-11-21.md`
at that repository's root.

**Verifier criterion (KR §17):** *Can every mathematical operation be traced
continuously back to declared observables through declared operators?*
Provenance is the question, not correctness.

**Nothing here is empirical.** Every test uses synthetic fixtures or random
fields. They establish that the forms compute as declared and that the
derived results hold over the forms, nothing about any system.

---

## 1. Reproduce

```bash
git clone https://github.com/Relational-Relativity-Corporation/abr-kernel.git
cd abr-kernel && git checkout claude/kind-keller-is6bq1
cargo test                                  # expect 115 passed, 0 failed
cargo test --features v9                    # expect 207 passed, 0 failed
cargo run --example v10_relational_field    # runs to completion
```

Checked by the Generator from a clean build (`cargo clean`) on 2026-11-21: both suites pass, no
warnings, the example runs, and `bin/provenance_demo` builds with and without `v9`.

| Suite | Default | With `v9` |
|---|---|---|
| library (`src/`) | 35 | 63 |
| `bin/provenance_demo` | 3 | 3 |
| `tests/diagnostic_exec.rs` | — | 13 |
| `tests/v10_ground_variables.rs` | 18 | 19 |
| `tests/v10_persistence_change.rs` | 17 | 17 |
| `tests/v10_properties.rs` | 21 | 21 |
| `tests/v10_relational_field.rs` | 5 | 6 |
| `tests/v10_structural_consistency.rs` | 16 | 16 |
| `tests/v9_admission.rs`, `tests/v9_conformance.rs` | — | 12, 35 |
| doctests | 0 | 2 |
| **Total** | **115** | **207** |

*After Verifier review (17th): two tests added for I-SC-D6, so the totals become 117 and 209 (structural consistency 17, properties 22).*

The library count fell from 42 to 35 because the seven τ = ħ/ε tests moved behind the V9 fence (item 9).

## 2. Commits

```
dffee32 feat(v10): structure and freedom as persistence and change (PC, Proposed)
12b1862 feat(v10): declare PC; provenance chain from observable to reported number
d50c00b docs(v10): name the three variables by their form over registered data only
e365e2a docs(v10): cite the quantum-mechanical intersection as comparison, not basis
a00dfe2 docs(v10): W14 intersection not basis; audit of existing kernel text
c67be54 feat(v10): retire the tau = hbar/epsilon family from V10
6c2b6c5 feat(v10): no time in V10 - retire Phi, restate V5 and R4 over counted steps
9043033 docs(v10): W15 time is never primitive; Phi restored as declared intersection
d600e8c docs(v10): apply W14 audit items 2-7
a5a3c2b docs(v10): W14 item 6 status: application register done; V4 tables missing
b97240c feat(v10): derive structural consistency I-SC-D3 to I-SC-D8
7086f91 test(v10): property tests for PC-D and I-SC-D; correct PC-D2 for PC3
4856d4a feat(v10): derive I-SC-D9 to I-SC-D13; cycle closure on a series
(this package) docs(v10): verification package; README counts; provenance fixes
```

## 3. Items

Status labels follow `V10_README.md`. **Declared** = Origin decided (Part 9
entry). **Derived** = a mathematical result by the Generator, awaiting the
Verifier. **Correction** = an error found and corrected, original kept.

| # | Item | Status | Where | Tests | What to check |
|---|---|---|---|---|---|
| 1 | **PC1–PC4**: structure is persistence, change is freedom, at three levels (relation, coupling of change, arrangement), with shares | Declared (7th) | `src/persistence_change.rs`; V10_README §2.5 | `v10_persistence_change.rs`; `v10_properties.rs` | That every reading is formed from Δ between consecutive registrations, prior always earlier; that PC2 admits only relations whose change is resolved |
| 2 | **PC-D1 to PC-D5** (complementarity, resolution, invariance, forced constraint, minimum information) | Derived | §2.5 | `prop_pc_d1…d5`, plus fixtures | PC-D2 as **corrected** (item 13) |
| 3 | **ER-1**, edge-valued registration | Declared (7th) | `EdgeSeries::from_edge_values` | throughout `v10_properties.rs` | That no node value is constructed for an edge-registered series; `forced_pairs` empty there |
| 4 | **Conflict 10**: V2 restated; `GroundReading::freedom` is the PC reading; first V2 form kept as `v2_held_arrangement` (declared comparison) | Declared (7th) | `src/ground_variables.rs`; V2, V8 rows; §1.2 | `v10_ground_variables.rs` | **Two existing tests changed meaning**; see §4 |
| 5 | **Coupled-change grade** of connection | Declared (7th) | `ConnectionGrade::CoupledChange`, `coupled_change_pairs` | `pc_d4…`, `sc_d4…` | That connection in state (V1) and in change (PC2) are read side by side, never merged |
| 6 | **Energy as a component** | Declared (7th) | §2.0 link 10 | `energy_carrier_as_a_component…` | That OC-SC-3 is stated in the mathematics and left to registered data |
| 7 | **§2.0 the provenance chain** (11 links) | Declared (7th) | V10_README §2.0 | — | That each link names its items and code; the "Names" note is an intersection, not a basis |
| 8 | **W14** intersection, not basis; audit of existing text; audit items 2–7 applied | Declared (8th, 12th) | V10_README W14; `w14_audit.md` | `photon_carrier_wavenumber_and_hc_over_lambda_read_identically`; `prop_energy_readings_invariant_under_carrier_rescaling` | §13.1 class column; hc/λ output Projected; Bell and J[v] relabelled as intersections |
| 9 | **τ = ħ/ε retired** from V10: `HBAR`, `tau_photon_edge`, `n_relational_cycles`, `tau_v`, `tau_v_isolated`, `tunneling_tau_bounded` behind the V9 fence | Declared (9th) | `src/derived_invariants.rs` | 7 tests now `v9` only | Code kept in place; records marked historical comparison |
| 10 | **W15** time is never primitive; **Φ** restored as a declared intersection (C) | Declared (11th) | V10_README W15, V7 | Φ tests run by default again | **The 10th amendment overstated the rule and the 11th corrected it.** Check the final state is consistent: Φ out of the fence, described as an intersection, no invariant based on it, I-RE's basis withdrawn |
| 11 | **V5 and R4 restated** over counted steps | Declared (10th, 11th) | `PersistenceChange::evolution_counts` | `v5_rate_is_counted_steps_never_time` | ΣΓ equals PC1's freedom count; no clock enters |
| 12 | **I-SC-D3 to I-SC-D13** (*I-SC-D6 corrected after Verifier review: resolution conditions stated*): fixed-ratio and offset coupling; additive circulation and its persistence condition; change without reorganization; dependence on the declared registrations; crossing bound; transitivity exactly but not at δ; closure on change; total circulation; locality to components; bounds on Γ | Derived (13th, 15th) | `derived_invariants.rs` Part II; V10_README §2.6 | `v10_structural_consistency.rs` (16); `v10_properties.rs` | The proofs in §5 by hand |
| 13 | **Property tests**, and the **PC-D2 correction** they found | Derived; Correction (14th) | `tests/v10_properties.rs` | 21 tests, 2000 fields each | That monotonicity in δ is claimed for PC1 and PC2 only; for PC3 only crossings (`pc3_freedom_is_not_monotone_in_delta`) |
| 14 | **New code for K7 / HB-1** on a series: `is_closed_walk`, `cycle_closure`, `cycle_change_closure`; `circulation_weights` | Derived (15th) | `persistence_change.rs`, `relational_field.rs` | `sc_d10`, `sc_d11`, properties | That traversal is an order of analysis: no edge is reversed |

## 4. Existing tests whose meaning changed

Each change is stated in the test itself. The Verifier should judge each. *(The first version of this package listed two of the three; corrected after Verifier review.)*

| Test | Before | After | Why |
|---|---|---|---|
| `proportional_grade_holds_while_values_change` | Freedom read zero (the first V2 form) | First V2 form still zero, as `v2_held_arrangement`; PC freedom > 0 | Under the 7th amendment the relations *changed*: proportional in state (structure) while changing (freedom) |
| `ground_reading_complete_over_a_sequence` | Two registrations, complete | Three registrations, complete; with two, one finding (coupling across steps needs three) | PC2(b) needs n ≥ 3; with two registrations that is a declaration finding |
| `at_the_primary_region_only_sustained_connection_is_a_gap` *(omitted from the first version of this package; added after Verifier review)* | Freedom f_δ = 1.0 (the first V2 form); findings: the sustained-grade gap and connection zero at δ | The first V2 value checked through `v2_held_arrangement`; PC freedom > 0; findings gain `NoProcessSequence` for freedom, before the two previous findings | Two registrations: coupling across steps (PC2(b)) is a declaration finding under the PC rule |

No other assertion was weakened. Tests moved behind the V9 fence (item 9) are unchanged and still run with `v9`.

## 5. Derivations to check by hand

The proofs are in `derived_invariants.rs` Part II and V10_README §2.6. These are the ones whose arguments are not just linearity:

- **I-SC-D7(a) and I-SC-D13 (lower bound).** δΔ over a merged group is δA_first + δA_last, at most the sum of the steps' δΔ, because every middle δA appears twice in that sum. Same-sign resolved steps give |ΣΔ| = Σ|Δ| > ΣδΔ ≥ δΔ_merged.
- **I-SC-D8.** Opposite resolved signs at s and s+1 give |Δm| > hw_s + hw_{s+1}. `minor_change_bound` bounds |Δm| by the triangle inequality over the expansion of Δm.
- **I-SC-D9.** Exactness needs the middle relation's change to be non-zero. PC2's entry rule (resolved in at least one reading) guarantees that.
- **I-SC-D10.** Closure at δ at both registrations implies closure on the change by the triangle inequality; the converse fails, as shown by `sc_d10`.
- **I-SC-D11.** Edge f is a successor of every edge ending at src f, and a predecessor of every edge starting at tgt f.
- **PC-D2 corrected.** PC1 and PC2 compare one value with a half-width that is non-decreasing in δ. PC3 compares two states, each of which can move from resolved to unresolved as δ grows. A crossing needs both states resolved, and resolved at δ′ implies resolved, with the same sign, at δ ≤ δ′.

## 6. Provenance of the text (amendments beside, never deleted)

The Generator checked every line deleted against `main`, mechanically, for
whether its text survives in the added lines:

- **Kept as amendments.** In the canonical text, amendments are appended to the original row or paragraph.
- **Lapses found and fixed in this package.** The three §1.2 rows (energy, freedom, connection), the release line and one migration-guide row had been *replaced*.
  - *As first written here:* "Each original is now restored beside its amendment, taken exactly from `main`."
  - *Corrected after Verifier review:* that overstates it. The three §1.2 rows and the migration row retain their original substantive cells verbatim, with a label added inside each row. The release line now reads `10.1.0` and records the former `10.0.0` in a note.
- **Rewritten, not deleted.** The other changed lines were renumbered or reflowed with their text kept: migration steps, open-condition rows, the OC-SC-3 comment.
- **Code and tests** are recorded by the git history and the commit messages: the `GroundReading` field and the test changes in §4.
- **`README.md`.** The test counts and package version were updated. The V10.1 counts (68 and 153) are kept as a note.

**Superseded or retired code is fenced or kept, never deleted.** V2's first form stays as `ground_variables::freedom`. The τ family is behind `v9`. Φ was fenced and then unfenced; the code was never removed.

## 7. Open, and not verified by this package

- **No registered data.** OC-SC-1 to OC-SC-4 (OC-SC-4 partly derived) and the I-SC-1 to I-SC-3 claims await the vessel record. Its declaration is still a draft, in `abr-relational-chemistry`.
- **`validation_record_v4.md` is missing** from the repository and its history. The tables behind VR-γ-01, VR-Φ-01, VR-J-01, VR-ε-01 and VR-τ-01 cannot be classed or checked until it is recovered.
- **Still open:** G1-K (pending Verifier); A-3 (empty sums); the acyclicity check (T8); K7 route enumeration and cycle basis; RPPI-V10; and the rest of V10_README Part 7.
- **Version:** the manifest is already `10.1.0`. The open decision is whether to bump from there for the merge (Origin's decision).
- **Stale:** the landing page in `.github` (not attached to this session).

## 8. For Origin after verification

1. The Verifier's disposition of items 1–14 and of §4.
2. Whether to merge `claude/kind-keller-is6bq1` into `main` with every commit kept (as for PR #3).
3. Whether the package version changes for the merge.

*Metatron Dynamics, Inc. Kernel V10.*

## 9. Verifier review (2026-11-21) and disposition

| Finding | Disposition |
|---|---|
| I-SC-D6 needs explicit resolution conditions; "every relation changes" fails at δ (example: 0.1 → 0.2 with δ = 0.1 reads structure) | **Accepted.** Corrected beside the original in Part II and §2.6: PC1 freedom iff \|κ − 1\|·\|A\| > δA_s + δA_{s+1}; PC3 states held only under the stated δ conditions; no crossing; circulation claimed for exact signs only. The Verifier's example is now a fixture, and the conditions a property test |
| A third test changed its expected findings and was missing from §4 | **Accepted.** Added to §4 |
| The package overstated verbatim restoration; the remaining version decision is whether to bump from `10.1.0` | **Accepted.** §6 and §7 corrected beside the first wording; Part 9 entry 17 |
| The chemistry ingest accepts inputs that can change readings without a finding | Handled in the `abr-relational-chemistry` package |

Both packages stay in review, per the Verifier, until these dispositions are checked.

## 10. Added after the review began (not yet reviewed)

| Commit | Item | Tests |
|---|---|---|
| `f811e6d` | `PersistenceChange::step_profile`: structure and freedom counts per step at every level, a declared summary (C) for following readings along an energy order (Part 9 entry 18) | `step_profile_partitions_every_level_by_step`. Default build 118 tests, with `v9` 210 |
