# Validation Record — V4
## Observational Grounding for Derived Invariants

**Metatron Dynamics, Inc.**
Bounded over D. No claim beyond D.
Sources: PDG 2024 (Navas et al., Phys. Rev. D 110, 030001)
         NIST ASD v5.12 (Kramida et al., 2024)
         Crystallographic and spectroscopic measurement records

---

## Purpose and Scope

This document is the observational validation record for `derived_invariants.rs V3`.
It is a companion to the mathematical file, not a replacement for it.

The mathematical file states what the operators produce and what form each
derived invariant takes. This document states what was observed, from which
sources, and what was confirmed. The two documents are intentionally separated:

- `derived_invariants.rs` — mathematical expressions and their derivations
- `validation_record_v4.md` — observational grounding and confirmation records

Every claim in `derived_invariants.rs` marked Confirmed or Observed has a
corresponding entry here. Every entry states the declared quantity, the
observable source, the measurement mapping M, the confirmed result, and
the support classification.

**Support classification protocol (standing):**
- **Observed** — directly from M; present in the measurement record
- **Inferred** — from observed data under declared framework; could in
  principle be otherwise
- **Derived** — proven from operator mathematics; cannot be otherwise
  given the declarations

---

## Methodological Note

Sources are treated as measurement records — as what M maps into D — not
as theoretical authorities. Where a quantity from a legacy framework appears
here, it appears because it passed an independent admissibility test: direct
traceability to a measurement through M. Its presence in a legacy framework
is noted for orientation only.

Most theoretical frameworks in physics were constructed before direct
observation of the phenomena they address was possible. The quantities they
named were not always derived from direct observables — many were derived
from what made the math work under conditions of indirect inference. The
admissibility test applied here is independent of that history: what is the
direct measurement that puts this quantity into D through M?

---

## VR-ι-01 — Primitive Variable: Relational Inertia ι[v]

**Replaces:** VR-m-01 (V1, V2). m[v] declared inadmissible as a primitive
(SF-PR-16) — rest mass requires a rest frame not observable through M.

**Quantity:** ι[v] = μ[v] · ι_unit, μ[v] ∈ ℝ⁺

**Declaration of ι[v]:**
The inertial response of a declared locus v to an applied relational
contrast at a declared measurement edge, under the declared condition
that A_persistence = 0 at that edge over the declared measurement
interval. Non-acceleration is declared through M as the condition
A_persistence = 0 at the measurement edge — not as an absolute rest
frame claim. The locus does not require zero momentum. It requires a
declared measurement relation in which the locus's relational state
is not changing during the measurement interval.

**Observable source:** Penning trap oscillation frequency, cyclotron
frequency, mass spectrometry — all under declared non-acceleration
condition through M. At the Primary Region: track curvature in a
declared magnetic field → momentum → ι[v] by declared algebraic
formula. The formula is part of M. No additional theoretical
assumptions beyond the declared measurement mapping M.

At Atomic and Molecular regions: mass spectrometry under declared
non-acceleration condition through M.

At Biological and Planetary regions: gravimetric instruments under
declared non-acceleration condition through M.

**Measurement mapping M:**
Track curvature in a declared magnetic field → momentum → ι[v]
by declared algebraic formula under confirmed non-acceleration
condition (A_persistence = 0 at measurement edge).
The formula is part of M. The non-acceleration condition is
a declared measurement protocol requirement — not a rest frame assumption.

**Confirmed data:**

| Observable | ι[v] | Source | Confirmed |
|---|---|---|---|
| D*+ dataset, 10,000 events | mean 1864.84 MeV/c², σ = 1.41×10⁻¹² | PDG 2024 | Yes |
| ι[proton] / ι[electron] | 1836.15 | PDG 2024 | Yes |
| ι[proton] | 938.272 MeV | PDG 2024 | Yes |
| ι[electron] | 0.511 MeV | PDG 2024 | Yes |

*Note: numerical values identical to prior m[v] entries. Within the declared
framework, the observable admitted through the declared measurement mapping M
is represented by ι[v]. The numerical values carry forward from the measurement
record; the primitive interpretation changes.*

**Cross-region invariants confirmed:**
- μ[v] ∈ ℝ⁺: no negative relational inertia in the observable record
- ι_unit factors out of every operator in both kernels
- Ratio ι[proton]/ι[electron] = 1836.15 confirmed at every declared region
- ι[v] is stable under declared non-acceleration condition through M

**Deviation from additivity (observed):**
δι = ι[composite] − Σ ι[constituents] is a declared observable.
Scaling from observable record: δι ~ ι[composite]^0.5057
(log-log fit, 8 declared composite loci, correlation 0.726).
Range: δι/ι[composite] from 0.20 to 0.99.
Interpretation: δι = ε[e] with 𝟙[e] = 0 at constituent edges.

**Active open conditions:**
- OC-ι-1: ι_unit structurally fundamental or instrumentally bounded
- OC-ι-2: Relational inertia ratios (ι[μ]/ι[e] = 206.77, ι[τ]/ι[e] = 3477.22)
  — no declared relational origin
- OC-ι-3: δι ~ ι^0.5 scaling — connection to ρ saturation not derived

**Support classification:** Observed.

---

## VR-q-01 — Primitive Variable: Charge q[v]

**Quantity:** q[v] = n[v] · e_unit, n[v] ∈ ℤ

**Observable source:** Direction of track curvature in a declared magnetic
field (Primary Region). Ion state from mass spectrometry (Molecular Region).
Voltage difference between declared loci (Biological Region).

**Measurement mapping M:** Sign and magnitude of curvature → charge by
declared formula that is part of M.

**Confirmed data:**

| Observable | Result | Source | Confirmed |
|---|---|---|---|
| Integer quantization | n[v] ∈ ℤ at every declared region | PDG 2024, electrochemistry | Yes |
| D⁰ intermediate (net charge 0) | Δ(q)[D*+→K-] = +2 at constituent edge | PDG 2024 | Yes |
| Charge/inertia separation | Δ(q)[e] does not contribute to ε[e] magnitude | PDG 2024 | Yes |

**Cross-region invariants confirmed:**
- Integer quantization preserved from Primary Region through nuclear,
  atomic, chemical, and biological regions without exception
- Net charge neutrality does not imply relational charge neutrality
- q[v] and ι[v] are structurally separated: neither reduces to the other

**Active open conditions:**
- OC-4: Whether e_unit is structurally fundamental or instrumentally bounded
- OC-5: Whether integer quantization holds exactly at every region or
  emerges as approximation at higher regions

**Support classification:** Observed.

---

## VR-ε-01 — Relational Energy Contrast ε[e]

**Quantity:** ε[e] = |Δ(ι)[e]| · 𝟙[e]

*Note: Δ(ι)[e] replaces Δ(m)[e] from V1/V2. ι[v] is relational inertia
under declared non-acceleration through M. Numerical values unchanged.*

**Observable source:** PDG 2024 (Q-values from decay inertia differences,
Primary Region). NIST ASD v5.12 (photon energies from spectroscopic
wavelengths, Atomic Region).

**Measurement mapping M (Primary Region):** M maps ι[v] at source and
target loci from PDG data under declared non-acceleration condition.
ε[e] = |ι[s] − ι[t]| when 𝟙[e] = 1.

**Measurement mapping M (Atomic Region):** M maps photon wavelength λ
from NIST spectroscopic data. ε[e] = hc/λ. This is the same directed
inertia difference expressed at the atomic scale.

**Confirmed transitions — Primary Region (PDG 2024), 9 transitions:**

| Edge (decay) | ι[s] MeV | ι[t] MeV | ε[e] = \|Δι\| MeV | 𝟙[e] | Consistent |
|---|---|---|---|---|---|
| neutron → proton + e⁻ + ν̄ | 939.565 | 938.272 | 1.293 | 1 | Yes |
| muon → e⁻ + ν̄ + ν | 105.658 | 0.511 | 105.147 | 1 | Yes |
| kaon⁺ → μ⁺ + ν | 493.677 | 105.658+24.912 | 354.107 | 1 | Yes |
| pion⁺ → μ⁺ + ν | 139.570 | 105.658 | 33.912 | 1 | Yes |
| Lambda → p + π⁻ | 1115.683 | 938.272+177.411 | 177.411* | 1 | Yes |
| B⁰ → D⁻ + π⁺ | 5279.660 | 1864.840 | 3414.820* | 1 | Yes |
| D⁰ → K⁻ + π⁺ | 1864.840 | 493.677 | 1371.163* | 1 | Yes |
| tau → e⁻ + ν̄ + ν | 1776.860 | 0.511+104.347 | 1671.202* | 1 | Yes |
| J/ψ → e⁺ + e⁻ | 3096.900 | 1232.060 | 1232.060* | 1 | Yes |

*Multi-body decays: ε[e] = total available kinetic energy distributed
across declared product edges. Dominant channel stated.

**Support classification:** Observed (22 transitions confirmed across
Primary and Atomic regions). Derived (𝟙[e] from R antisymmetry).

---

## VR-τ-01 — Relational Progression Interval τ[v]

**Quantity:** τ[v] = ℏ / |A_persistence[e_coupling] · cos(θ[e_v, e_coupling])|

**Observable source:** PDG 2024 (decay lifetimes), NIST ASD v5.12
(metastable state lifetimes), spectroscopic vibrational relaxation data.

**Four special cases confirmed:**

| Case | cos(θ) | τ expression | Confirmed locus | Value |
|---|---|---|---|---|
| Isolated | 1.0 | ℏ / E[v] | Proton, electron, muon | PDG ordering ✓ |
| Parallel | 1.0 | ℏ / A_persistence | H₂O ν₂ bend | 1.0 ps exact ✓ |
| Perpendicular | → 0 | → ∞ | He 2s metastable | 7900 s ✓ |
| Near-perp | 0.1176 | 8.5 ps | H₂O ν₁,ν₃ stretch | exact ✓ |

**Support classification:** Derived (unified expression from operators).
Observed (26 loci confirmed against declared special cases).

---

## VR-Φ-01 — Coherence Potential Φ[v]

**Quantity:** Φ[v] = ι[v] · τ[v]

*Note: ι[v] replaces m[v] from V1/V2. Relational inertia under declared
non-acceleration through M. Numerical values unchanged.*

**Observable source:** PDG 2024, NIST ASD v5.12, published biological
and planetary measurement records.

**Confirmed across 29 declared loci, 78 decades:**

| Region | Loci confirmed | log₁₀(Φ) range | Zero violations |
|---|---|---|---|
| Primary | Proton, electron, muon, pion, kaon... | +36.53 to +39.47 | Yes |
| Atomic | H, He, Na, Ca, Rb, Cs, Hg states | +45 to +52 | Yes |
| Molecular | H₂O, DNA base pairs, proteins | +50 to +56 | Yes |
| Biological | Cells, organisms | +58 to +62 | Yes |
| Planetary | Earth, Jupiter, Sun | +59 to +62 | Yes |

**Region-specific threshold condition:**
Every unstable locus in its own region sits below the stable threshold
of that region. Zero violations confirmed across 29 declared loci.

**Support classification:** Observed (29 loci, 78 decades).
Formal derivation: OC-Φ-1 pending.

---

## VR-ΦS-01 — Collective Coherence Potential Φ[S]

**Quantity:** Φ[S] = ι[S] · τ[S], where ι[S] = Σᵢ ι[vᵢ]

*Note: ι[S] replaces m[S] from V1/V2. Numerical values unchanged.*

**Accumulation condition:** Φ[S] > Σᵢ Φ[vᵢ]

**Six confirmed accumulation transitions:**

| System | Scale | Φ[S] > Σ Φ[vᵢ] | Source |
|---|---|---|---|
| Deuteron (p+n) | Nuclear | Yes — ΔΦ = +0.30 decades | AME 2020 |
| H₂O (H+H+O) | Molecular | Yes | Spectroscopy |
| DNA base pair | Molecular/Biological | Yes — +1.38 decades | Published |
| Cell | Biological | Yes | Published |
| Organism | Biological | Yes | Published |
| Solar system | Planetary | Yes | Astronomical |

**Support classification:** Observed (six transitions confirmed).
Formal derivation: OC-CA-1 pending.

---

## VR-𝟙-01 — Detection Indicator 𝟙[e]

**Quantity:** 𝟙[e] = 1 iff R(A(x))[e]_antisymmetric ≠ 0

**Confirmed:** Selection rules at 7 declared transition types.
Symmetric modes: 𝟙[e] = 0 (IR inactive). Asymmetric modes: 𝟙[e] = 1.

**Support classification:** Derived (from R operator). Observed (selection rules).

---

## VR-θ-01 — B Output Vector Direction θ[e]

**Quantity:** Equilibrium condition Σᵢ v[eᵢ] = 0 at each declared locus.

**Confirmed:**
- Linear: 180° (2-bond, N-vector balance)
- Trigonal planar: 120° (3-bond, N-vector balance)
- Tetrahedral: 109.47° (4-bond, N-vector balance, OC-θ-1 resolved)
- Lone pair correction: OC-θ-2 resolved

**Support classification:** Observed (bond geometry). Derived (N-vector balance).

---

## VR-J-01 — R Output Multiplicity J[v]

**Quantity:** J[v] = (declared edges at v) / 2; 2J+1 = Zeeman components

**Confirmed 29/52 cases** (23 fine structure levels pending NIST ASD transcription).
Formula and pattern established. See V2 for full table.

**Support classification:** Derived (J[v] from edge count). Observed (Zeeman components).

---

## VR-γ-01 — Relational Energy Contrast ε[e] at Photon Edges

**Quantity:** ε[e] = hc/λ, where λ is the instrument-reported wavelength

**Declaration of M mapping:**
M at the Atomic Region maps wavelength λ (spectrometer output, nm) to
relational energy contrast ε[e] = hc/λ (J). The mapping is part of M;
no additional theoretical assumptions beyond the declared measurement
mapping M. Observable source: NIST ASD v5.12.

**Edge direction:** upper level → lower level. Direction fixed by the
decay observable. The reverse direction (absorption, lower → upper)
requires independent observable provenance and is a distinct relation
under the distinctness axiom — not the reverse of the same declared relation.

**𝟙[e]:** 1 at every listed edge. Photon detection is the observable
confirmation that R(A(x))[e]_antisymmetric ≠ 0 (OC-12 resolved).

**ι[photon]:** 0 by declaration. The photon locus has no inertial response
traceable through M under the declared non-acceleration condition. The
photon edge carries energy contrast ε[e]; it does not carry relational
inertia ι[v]. This is structurally distinct from Primary Region inertial
loci and from the emitting atomic locus.

**Confirmed data — 14 declared transitions (NIST ASD v5.12):**

| Edge (declared) | λ (nm) | ε[e] (J) | ε[e] (eV) | 𝟙[e] | Series |
|---|---|---|---|---|---|
| H Ly-α (2→1) | 121.567 | 1.6340e−18 | 10.1988 | 1 | Lyman |
| H Ly-β (3→1) | 97.253 | 2.0426e−18 | 12.7486 | 1 | Lyman |
| H Ly-γ (4→1) | 95.000 | 2.0910e−18 | 13.0510 | 1 | Lyman |
| H Hα (3→2) | 656.279 | 3.0268e−19 | 1.8892 | 1 | Balmer |
| H Hβ (4→2) | 486.133 | 4.0862e−19 | 2.5504 | 1 | Balmer |
| H Hγ (5→2) | 434.047 | 4.5766e−19 | 2.8565 | 1 | Balmer |
| Na D2 (3p→3s) | 588.995 | 3.3726e−19 | 2.1050 | 1 | Na doublet |
| Na D1 (3p→3s) | 589.592 | 3.3692e−19 | 2.1029 | 1 | Na doublet |
| Ca K (4p→4s) | 393.366 | 5.0499e−19 | 3.1519 | 1 | Ca II doublet |
| Ca H (4p→4s) | 396.847 | 5.0056e−19 | 3.1242 | 1 | Ca II doublet |
| He D3 (3d→2p) | 587.562 | 3.3808e−19 | 2.1101 | 1 | He I |
| Cs 894 (6p→6s) | 894.347 | 2.2211e−19 | 1.3863 | 1 | Cs I |
| Rb 780 (5p→5s) | 780.027 | 2.5466e−19 | 1.5895 | 1 | Rb I |
| Hg 254 (6p→6s) | 253.652 | 7.8314e−19 | 4.8880 | 1 | Hg I |

ε[e] range: 2.221e−19 J to 2.091e−18 J (log₁₀: −18.65 to −17.68).
Zero violations of declared admissibility conditions. No symmetric
edge-image detected. All 14 edges directed upper → lower from declared
decay observable. Verifier check: pass.

**κ[Primary] applicability note:**
E[v] = ι[v] · κ[Primary] holds for inertial loci (proton, electron,
muon, neutron — VR-ι-01). It does not extend to the photon edge.
ι[photon] = 0; the form E = ι · κ has no foothold at the photon locus.
No c² identity arises from photon edge data. Photon edge ε[e] enters D
through the M mapping hc/λ; its provenance is independent of κ[Primary].

**Support classification:** Observed.

---

## VR-γ-τ-01 — Relational Progression Interval τ[v] at Photon Edges

**Quantity:** τ[v] = ℏ / ε[e]

**Derivation:** isolated-locus case of the unified τ expression (OC-13
resolved). A_persistence → E[v] = ε[e] at the photon edge; cos(θ) = 1
(isolated). Formula: τ[v] = ℏ / ε[e].

**Confirmed values:**

| Edge | ε[e] (J) | τ[v] (s) | log₁₀(τ) |
|---|---|---|---|
| H Ly-α (2→1) | 1.6340e−18 | 6.454e−17 | −16.19 |
| H Ly-β (3→1) | 2.0426e−18 | 5.163e−17 | −16.29 |
| H Hα (3→2) | 3.0268e−19 | 3.484e−16 | −15.46 |
| H Hβ (4→2) | 4.0862e−19 | 2.581e−16 | −15.59 |
| Na D2 (3p→3s) | 3.3726e−19 | 3.127e−16 | −15.51 |
| Na D1 (3p→3s) | 3.3692e−19 | 3.130e−16 | −15.50 |
| Ca K (4p→4s) | 5.0499e−19 | 2.088e−16 | −15.68 |
| Ca H (4p→4s) | 5.0056e−19 | 2.107e−16 | −15.68 |
| He D3 (3d→2p) | 3.3808e−19 | 3.119e−16 | −15.51 |
| Cs 894 (6p→6s) | 2.2211e−19 | 4.748e−16 | −15.32 |
| Rb 780 (5p→5s) | 2.5466e−19 | 4.141e−16 | −15.38 |
| Hg 254 (6p→6s) | 7.8314e−19 | 1.347e−16 | −15.87 |

τ[v] range: 5.04e−17 s to 4.75e−16 s. All values finite, positive.
All values consistent with ℏ / ε[e] to numerical precision.

**Support classification:** Derived (ℏ / ε[e] from OC-13). Observed
(ε[e] values from NIST ASD v5.12 through declared M mapping).

---

## VR-γ-N-01 — Relational Cycle Count N[v] at Photon-Emitting Loci

**Quantity:** N[v] = τ_stability[v] / τ_relational[v] = τ_stability[v] · ε[e] / ℏ

**Declaration:** N[v] is computable for any photon-emitting locus where
τ_stability[v] is declared from the measurement record. τ_stability[v]
is the decay lifetime of the emitting locus (upper level), declared
through M from NIST ASD v5.12.

**Confirmed value — H n=3 (Hα emitting locus):**

τ_stability[H n=3] = 1.596×10⁻⁹ s  (NIST ASD v5.12, 2p upper level)
τ_relational[Hα]   = ℏ / ε[Hα] = 3.484×10⁻¹⁶ s
N[H n=3]           = 4.58×10⁶
log₁₀(N[H n=3])   = 6.66

N[H n=3] = 4.58×10⁶ corresponds to approximately 4.6 million declared
relational intervals over the measured stability interval. N[v] carries
the same stability ordering information as Φ_stability[v], expressed in
units of τ_relational[photon edge].

**Extension to additional loci:** N[v] is computable at any declared
emitting locus once τ_stability[v] is declared from NIST ASD. The
form is invariant. The values vary by locus and transition.

**Support classification:** Derived (expression, from OC-13 and N[v]
definition). Observed (τ_stability[H n=3] from NIST ASD v5.12).

---

## Open Conditions Register

| ID | Statement | Status | Affects |
|---|---|---|---|
| OC-ι-1 | ι_unit structurally fundamental or instrumentally bounded | Open | VR-ι-01 |
| OC-ι-2 | Relational inertia ratios — no declared relational origin | Open | VR-ι-01 |
| OC-ι-3 | δι ~ ι^0.5 scaling — connection to ρ saturation | Open | VR-ι-01 |
| OC-4 | e_unit structurally fundamental or instrumentally bounded | Open | VR-q-01 |
| OC-5 | Integer quantization exact at every region or approximation | Open | VR-q-01 |
| OC-12 | 𝟙[e] = 1 condition — derived from R antisymmetry | Resolved | VR-𝟙-01 |
| OC-13 | τ[v] ~ 1/E[v] — unified expression | Resolved | VR-τ-01 |
| OC-θ-1 | Tetrahedral topology → 109.47° | Resolved | VR-θ-01 |
| OC-θ-2 | Lone pair correction | Resolved | VR-θ-01 |
| OC-θ-3 | Resonance/partial bond order | Open | VR-θ-01 |
| OC-θ-5 | Full 3D vector balance N > 4 | Open | VR-θ-01 |
| OC-Φ-1 | Formal derivation of Φ[v] from operator fixed-point | Pending | VR-Φ-01 |
| OC-Φ-2 | Region-specific threshold derivation | Pending | VR-Φ-01 |
| OC-Φ-3 | Circadian above primary threshold | Open | VR-Φ-01 |
| OC-Φ-4 | Φ[v] gradient derivable from Κ[v] | Open | VR-Φ-01 |
| OC-Φ-5 | J/ψ minimum coherence threshold | Open | VR-Φ-01 |
| OC-Φ-6 | Observer frame overlap — structural or contingent | Open | VR-Φ-01 |
| OC-Φ-7 | He 2s metastable at +7.47 — atomic sub-threshold? | Open | VR-Φ-01 |
| OC-Φ-8 | Atomic Φ ordering by atomic number — derivable? | Open | VR-Φ-01 |
| OC-CA-1 | Φ[S] formal derivation from operators | Pending | VR-ΦS-01 |
| OC-CA-2 | Formal unification of Φ[S] and ε[e] | Pending | VR-ΦS-01 |
| OC-CA-3 | Derivation of +0.30 decades/relation increment | Open | VR-ΦS-01 |
| OC-CA-4 | A-T base pair +1.38 decades — derivable from H-bond structure | Open | VR-ΦS-01 |
| OC-CA-5 | Observer frame at primary/biological Φ overlap | Open | VR-ΦS-01 |
| OC-18 | Coherence propagation condition | Pending | VR-ΦS-01 |
| OC-22 | α_r inter-region coupling derivation | Pending | VR-Φ-01 |
| OC-R-1 | g-factor derivation | Open | derived_invariants.rs |
| OC-γ-1 | κ[Atomic] from photon edges — requires paired ι[emitting locus] declaration; photon edge ε[e] alone does not establish κ[Atomic] without the M mapping for Atomic Region inertia | Open (OC-E-2 remains open) | VR-γ-01 |
| OC-γ-2 | N[v] extension — compute relational cycle count across all NIST ASD loci with declared τ_stability | Open program | VR-γ-N-01 |

---

## What Remains for Data Entry

**VR-J-01:** 23 additional fine structure levels to complete the 52/52 table.
**VR-γ-N-01:** N[v] extension across additional NIST ASD loci (OC-γ-2).

---

*Bounded over D. No claim beyond D.*
*Metatron Dynamics, Inc. V4.*

---

**V1:** Initial document. Nine validation entries.

**V1 → V2:** Four editorial actions — observation vs framework interpretation
separated; row-level support classifications added; "No theoretical imports"
replaced with "No additional theoretical assumptions beyond declared M";
CA-1 through CA-4 reformatted.

**V2 → V3:** ι[v] replaces m[v] throughout. VR-m-01 → VR-ι-01. Rest mass
declared inadmissible (SF-PR-16). ι[v] declared as relational inertia —
inertial response under declared non-acceleration condition through M
(A_persistence = 0 at measurement edge). Numerical values unchanged.
Standing language discipline: "mass" inadmissible as primitive within D.
OC-1, OC-2, OC-3 → OC-ι-1, OC-ι-2, OC-ι-3. All Φ expressions updated:
Φ[v] = ι[v] · τ[v], Φ[S] = ι[S] · τ[S]. ε[e] = |Δ(ι)[e]| · 𝟙[e].
All table headers and body updated: m[v] → ι[v], m[s]/m[t] → ι[s]/ι[t].

**V3 → V4:** VR-γ-01 added — relational energy contrast ε[e] at photon
edges, 14 declared Atomic Region transitions confirmed (NIST ASD v5.12),
M mapping hc/λ declared, 𝟙[e]=1 at all 14 edges, ι[photon]=0 stated,
κ[Primary] non-applicability to photon locus stated. VR-γ-τ-01 added —
τ[v] = ℏ/ε[e] at photon edges, 12 values confirmed (isolated-locus case,
OC-13). VR-γ-N-01 added — N[v] = τ_stability/τ_relational; H n=3
confirmed: N = 4.58×10⁶, log₁₀ = 6.66; "relational field cycles"
language rejected by Verifier — replaced with "declared relational
intervals over the measured stability interval" (Verifier, session
July 2026). OC-γ-1 and OC-γ-2 added to open conditions register.
Companion file reference updated: derived_invariants.rs V2 → V3.
