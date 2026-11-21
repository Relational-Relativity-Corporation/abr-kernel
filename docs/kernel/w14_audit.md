# W14 Audit: Intersection, Not Basis

**Metatron Dynamics, Inc.** Kernel V10. Bounded over D. No claim beyond D.
Audit of 2026-11-21 (Generator), against W14 (Declared, Origin 2026-11-21).
`V10_README.md` governs. **No audited item has been changed.** Each finding
carries a recommendation for Origin.

> **W14.** A result may cite where it intersects an abstraction (a construct,
> model or fitted value) as a declared comparison (C). No assumption, input,
> edge, value or reading is based on a construct. Where a mapping through a
> construct is needed to place a registration in D, it is declared in M as a
> mapping with its δ, and its output is classed Projected, not Registered (R7).
> A value is classed by how it was obtained (registered, defined by
> convention, or fitted), never by the authority that publishes it.

**Scope.** Every occurrence in `src/` and `docs/kernel/` of quantum-mechanical
terms and constructs: *quantum*, ħ, h, `HBAR`, `H_PLANCK`, `C_DECLARED`, hc/λ,
hν, wave function, Hamiltonian, decoherence. About 100 matches, grouped below
by what they do.

**Classes.**

| Class | Meaning | Action |
|---|---|---|
| **A. Intersection** | Cites where a kernel reading meets a term of the construct | Keep |
| **B. Basis** | An assumption, value or reading rests on the construct | Origin decides |
| **C. Mislabelled provenance** | A defined or fitted value is labelled as a measurement or as Observed | Correct the label |
| **D. Prohibition** | Text that already forbids importing the construct | Keep (consistent with W14) |

---

## B. Basis: assumptions resting on a construct

### B1. τ = ħ/ε and everything built on it

The relation τ = ħ/ε is a construct. It supplies a time from an energy through
a constant of the construct, and no registration in the kernel's record
registers that time. Everything computed from it inherits the assumption.

| Where | Item |
|---|---|
| `src/derived_invariants.rs` 565–645 | `tau_photon_edge`, `tau_v`, `tau_v_isolated`, `n_relational_cycles` (N[v] = τ_stability·ε/ħ) |
| `src/derived_invariants.rs` 585–592, 616–625 | "Confirmed" cases: H n=3 N[v] = 4.58×10⁶; "H₂O ν₂ bend: 1.0 ps exact"; "He 2s metastable: 7900 s"; "H₂O ν₁,ν₃ … exact" |
| `docs/kernel/cross_region_energy_expression.md` 20–30, 58–66 | τ_relational = ℏ/E[v]; Φ_relational = ι·τ_relational, "action quantum per unit relational field magnitude" |
| `V10_README.md` V5, V7 and §2.2 | V5 "τ from ħ/ε where ε is declared"; V7 Φ built on τ; §2.2 `tau_photon_edge`, `tunneling_tau_bounded` (τ = ħ/ε) |
| `docs/kernel/kernel_reference.md` 803–822 | The same functions restated |
| `docs/kernel/observable_variable_sets.md` 1314 | τ[v] = ℏ/ε[e] at 12 edges; N[v] at H n=3 |

These functions are in the **default V10 build**: the V9 fence does not cover
them, and the migration guide lists them as available.

**Recommendation:** retire τ = ħ/ε from V10 readings and fence the functions
behind the `v9` feature, as was done for ρ and χ (W13). Keep every number they
produced as a declared historical comparison. Where a time is needed, take it
from registrations: steps with A_p resolved at δ_W (V5's own second branch).

### B2. ε = hc/λ (E = hν)

The mapping turns a registered wavelength into an energy through the
construct. The kernel's own list of inadmissible imports contains it:
`observable_variable_sets.md` 218 lists "Frequency as energy via E = hν —
requires quantum import". Yet EF-14 (line 1115) records "**[Observed]** ε[e] =
hc/λ confirmed at 14 … photon edge transitions". The output of a construct
mapping is labelled Observed. That is an internal contradiction.

| Where | Item |
|---|---|
| `src/derived_invariants.rs` 158–160, 193, 505–563, 1016–1085 | `epsilon_photon_edge`, its comments and tests |
| `src/operators.rs` 1890–1952 | Operator-chain test fixtures built from hc/λ, labelled `[OBS]` |
| `src/provenance.rs` 118–122, 297 | hc/λ given as the example of a kernel-declared function |
| `src/admission.rs`, `src/provenanced_exec.rs` (V9, fenced) | The same mapping in the V9 admission path |
| `V10_README.md` V3, R9a, §2.2 | "at photon edges hc/λ in Regional M"; R9a example "H/D level network carries energy in hν" |
| `docs/kernel/kernel_reference.md` 770–781, 906, 921–923, 1204, 1217 | Restated |
| `docs/kernel/observable_variable_sets.md` 1115–1140 | EF-14 "[Observed]" |
| `docs/kernel/cross_region_energy_expression.md` 336 | Restated |

**Observation that settles most of it.** hc is one constant factor. Every V10
reading of a carrier (the sign and resolution of each change, every minor, the
degree ε/δ_ε) is unchanged when the carrier and its δ are multiplied by the
same constant (PC-D3). So **hc/λ contributes nothing to any V10 reading**:
the registered wavenumber 1/λ, with its own δ, gives every reading
identically. The construct would matter only when energies from different
carriers are compared on one scale, and that is a cross-carrier mapping
(P1.3, Open) that has to be declared on its own.

**Recommendation:** declare the registered wavenumber (or wavelength) as the
carrier at photon edges, in its registered unit. Keep hc/λ only as a declared
mapping whose outputs are Projected, for comparison. Relabel EF-14 and the
`[OBS]` fixtures accordingly.

### B3. "Quantum boundary component" of Δ(ι)[e]

The I-T text once asserted a mechanism that was never registered. It was
replaced in the I-T statement (`derived_invariants.rs` 230, 243), but
**OC-T-1** (line 1582) still asks to "derive the quantum boundary component of
Δ(ι)[e] as a function of declared barrier geometry". That is a live open
condition resting on the construct. Line 1691 is the V4 history entry; it is
record and stays.

**Recommendation:** restate OC-T-1 on V3 (the exchange floor ε > δ_ε over a
registered carrier), or withdraw it.

### B4. Support status taken from a construct

| Where | Text | Why it is basis |
|---|---|---|
| `src/derived_invariants.rs` 1539–1548 (I-E, V9 reader comment) | "Confirmed consequences: EPR-type measurement correlations (Bell inequality violations …)" | The registered data are coincidence counts. A "violation" is computed against a bound derived from a model. Support classified through that bound rests on the construct |
| `src/derived_invariants.rs` 762–768; `observable_variable_sets.md` 954 | J[v] "Confirmed 29 of 52" | The comparison targets are spin quantum numbers, values assigned by the construct. Comparing with them is an intersection; calling the match a confirmation is basis |

**Recommendation:** relabel both as cited intersections (C), with the
registered quantity named (coincidence counts; the observed line multiplicity
or splitting). Remove "Confirmed".

---

## C. Mislabelled provenance

### C1. `kernel_reference.md` §13.1, declared measurement references

The provenance column labels defined and fitted values as measurements:

| Constant | Labelled as | How the value was obtained |
|---|---|---|
| `H_PLANCK` 6.626 070 15e−34 J·s | "Spectroscopic frequency-energy measurement" | **Exact by definition** in the SI since 2019 |
| `HBAR` | "Spectroscopic frequency-energy measurement" | h/2π: **defined** |
| `C_DECLARED` 2.997 924 58e8 m/s | "BIPM 2019 interferometric measurement" | **Exact by definition** since 1983 (it defines the metre) |
| `E_UNIT` 1.602 176 634e−19 C | "Millikan oil-drop and subsequent direct measurements" | **Exact by definition** in the SI since 2019 |
| `I_UNIT_ELECTRON` 9.109 383 701 5e−31 kg | "Penning trap oscillation frequency under declared non-acceleration" | The CODATA 2018 **adjusted value**: a least-squares fit across many experiments, not one registration |

**Recommendation:** correct the column under R7. The first four are
*defined by convention*; the electron value is *fitted (Provider-declared)*.
These are factual corrections, but the text is Origin's, so it is left
unchanged here.

### C2. Source class of database values (application layer)

NIST ASD wavelengths are listed as **Observed** or **Ritz**. Ritz values are
calculated from fitted level energies. Many listed lifetimes and transition
probabilities are calculated, not measured. EF-14 to EF-16, the τ_stability
values and the J[v] comparisons all draw on NIST ASD and PDG.

**Recommendation:** check each value's source class in the application
repositories before it enters any V10 result. Kernel action: none, beyond
W14's classing rule.

---

## A. Intersections (keep)

| Where | Text |
|---|---|
| `V10_README.md` §2.0 | Names note: the intersection with quantum terms cited as C; nothing based on it |
| `src/derived_invariants.rs` 1109–1113, 1216–1217; `kernel_reference.md` 948–949 | "What the established literature calls quantum phenomena are these conditions …" |
| `src/derived_invariants.rs` 768 | J[v]: "What the established literature calls spin quantum number" (the naming only; see B4 for the confirmation) |
| `src/lib.rs` 61; `v10_migration.md` 47; `validation_record.md` 393–394, 524; `kernel_reference.md` 1010–1020, 1170 | "Decoherence" as the V9 reader's name, behind the V9 fence or in the record |

**One intersection that overreaches.** `operators_notation_and_constraint.md`
80–90 says the framework "derives a unification result" and that classical and
quantum "are not two regimes of nature". The intersection is admissible. A
statement about nature is a claim beyond D, and "derives" overstates what is a
reading of the construct's domain.

**Recommendation:** restate it as a cited intersection bounded over D, with
the V9 text kept beside the amendment.

**One superseded definition.** `abr_operators_plain.md` 260 defines
entanglement by 𝟙[e] (V9). It is now V1 plus coupled change.

**Recommendation:** add an amendment pointer to §2.0, keeping the text.

## D. Prohibitions already consistent with W14 (keep)

`src/operators.rs` 200–201; `role_separation_and_operator_application.md` 79;
`kernel_reference.md` 314, 1264; `observable_variable_sets.md` 19, 212–218;
`cross_region_energy_expression.md` 272. Validation targets in
`role_separation_and_operator_application.md` 127 (PDG, NIST ASD, provider
calibration data) remain comparison targets, classed Provider-declared under
R7.

---

## Decisions for Origin

| # | Item | Recommendation |
|---|---|---|
| 1 | B1, τ = ħ/ε family | **Done 2026-11-21 (9th):** retired entirely from V10 under the time prohibition; fenced behind `v9`; Φ_relational and κ[region] retired as identities |
| 2 | B2, hc/λ | **Done (12th):** wavenumber carrier; hc/λ a declared mapping, output Projected, cited as intersection; readings shown identical by test; EF-14, VR-γ-01, `[OBS]` fixtures relabelled |
| 3 | B3, OC-T-1 | **Done (12th):** restated on V3, no mechanism posited; first statement kept |
| 4 | B4, "Confirmed" via Bell bound and spin numbers | **Done (12th):** cited intersections; registered quantities named (coincidence counts; resolved Zeeman components) |
| 5 | C1, §13.1 provenance | **Done (12th):** class column added beside the original (defined / fitted); same classes in code comments |
| 6 | C2, database source classes | **Kernel part done (12th):** classing required before citing (migration guide step 3; VR-γ-01, VR-Φ-01 notes). **Open:** the per-value check in the application repositories |
| 7 | A, unification claim and superseded entanglement text | **Done (12th):** read as a bounded intersection; amendment pointer added |

---

*Metatron Dynamics, Inc. Kernel V10.*
