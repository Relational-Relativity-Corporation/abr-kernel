# Validation Record — Kernel V8
## Observational Grounding for Derived Invariants

**Metatron Dynamics, Inc.** Kernel V8.
Bounded over D. No claim beyond D.
Sources: PDG 2024 (Navas et al., Phys. Rev. D 110, 030001)
         NIST ASD v5.12 (Kramida et al., 2024)
         Crystallographic and spectroscopic measurement records

---

## Purpose and Scope

This document is the observational validation record for `derived_invariants.rs` (Kernel V8).
It is a companion to the mathematical file, not a replacement for it.

The mathematical file states what the operators produce and what form each
derived invariant takes. This document states what was observed, from which
sources, and what was confirmed. The two documents are intentionally separated:

- `derived_invariants.rs` — mathematical expressions and their derivations
- `validation_record.md` — observational grounding and confirmation records

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

*Scale-conditional Observed subcategories* (per the observational resolution
principle in `operators_notation_and_constraint.md`):
- **Observed — locus-resolved** (ℛ_M = 1, 𝓜_M = 1): the constituent loci and
  their directed relations are independently declarable through M.
- **Observed — expression-observed, mechanism-unresolved** (ℛ_M = 1,
  𝓜_M = 0): the observable record contains a traceable expression of the
  condition through M; its internal mechanism is not yet resolvable by M at
  the available scale.

*Downstream projection:* a quantity computed from observed values by a
declared reduction (for example a fit) is labeled a downstream projection. It
is a calculated result, not itself an observation, and states what it
preserves and discards.

*Kernel V8:* values entered here from M are numerical projections of
observables into D. Operator outputs compared against them are calculated
results. Where a record entry compares the two, the comparison is the
correspondence being recorded.

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
(SF-PR-16) — its definition requires a frame with no declared relational
motion, which does not exist in D.

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
Scaling from observable record (downstream projection — log-log fit over
8 declared composite loci; preserves the exponent of the fitted relation,
discards per-locus residuals): δι ~ ι[composite]^0.5057.
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

**Confirmed:** 22 transitions confirmed across Primary and Atomic regions.
See V4 for full table. Unchanged from V4.

**Support classification:** Observed (22 transitions). Derived (𝟙[e] from R antisymmetry).

---

## VR-τ-01 — Relational Progression Interval τ[v]

**Quantity:** τ[v] = ℏ / |A_persistence[e_coupling] · cos(θ[e_v, e_coupling])|

**Four special cases confirmed:** See V4 for full table. Unchanged from V4.

**Support classification:** Derived. Observed (26 loci confirmed).

---

## VR-Φ-01 — Coherence Potential Φ[v]

**Quantity:** Φ[v] = ι[v] · τ[v]

**Confirmed across 29 declared loci, 78 decades.** See V4 for full table.
Unchanged from V4.

**Support classification:** Observed (29 loci, 78 decades).
Formal derivation: OC-Φ-1 pending.

---

## VR-ΦS-01 — Collective Coherence Potential Φ[S]

**Quantity:** Φ[S] = ι[S] · τ[S], where ι[S] = Σᵢ ι[vᵢ]

**Six confirmed accumulation transitions.** See V4 for full table.
Unchanged from V4.

**Support classification:** Observed (six transitions confirmed).
Formal derivation: OC-CA-1 pending.

---

## VR-𝟙-01 — Detection Indicator 𝟙[e]

**Quantity:** 𝟙[e] = 1 iff R(A(x))[e]_antisymmetric ≠ 0

**Confirmed:** Selection rules at 7 declared transition types. Unchanged from V4.

**Support classification:** Derived (from R operator). Observed (selection rules).

---

## VR-θ-01 — B Output Vector Direction θ[e]

**Quantity:** Equilibrium condition Σᵢ v[eᵢ] = 0 at each declared locus.

**Confirmed:** 180°, 120°, 109.47°. Unchanged from V4.

**Support classification:** Observed (bond geometry). Derived (N-vector balance).

---

## VR-J-01 — R Output Multiplicity J[v]

**Quantity:** J[v] = (declared edges at v) / 2; 2J+1 = Zeeman components

**Confirmed 29/52 cases.** Unchanged from V4.

**Support classification:** Derived. Observed (Zeeman components).

---

## VR-γ-01 — Relational Energy Contrast ε[e] at Photon Edges

**14 declared Atomic Region transitions confirmed.** See V4 for full table.
Unchanged from V4.

**Support classification:** Observed.

---

## VR-γ-τ-01 — Relational Progression Interval τ[v] at Photon Edges

**12 values confirmed.** See V4 for full table. Unchanged from V4.

**Support classification:** Derived. Observed (ε[e] values from NIST ASD v5.12).

---

## VR-γ-N-01 — Relational Cycle Count N[v] at Photon-Emitting Loci

**H n=3 confirmed:** N = 4.58×10⁶, log₁₀ = 6.66. Unchanged from V4.

**Support classification:** Derived. Observed (τ_stability from NIST ASD v5.12).

---

## VR-T-01 — Tunneling Invariant (I-T)

**Quantity:** At every declared transition edge where a relational energy
contrast is established through M, ε[e] > 0 — the directed inertia contrast
Δ(ι)[e] is nonzero at that edge.

**Derived consequences:**
- ε[e] > 0 at every such edge (nonzero floor)
- τ[v] < ∞ at every such edge (bounded above)
- 𝟙[e] = 1 at every such edge

**Observable provenance (scale-conditional):** Expression-observed at the
Primary Region (ℛ_M = 1, 𝓜_M = 0) — nonzero transition rates at declared
edges are in the observable record through M where the primary kernel alone,
acting on unresolved mechanism, predicts zero contrast; the mechanism is not
yet resolvable by M at this scale. Locus-resolved at larger scales
(ℛ_M = 1, 𝓜_M = 1). What the established literature calls tunneling is what
M reports where ε[e] > 0 at a declared transition edge and 𝓜_M = 0.

**Confirmed consequences:**

| Observable | System | Result | Source |
|---|---|---|---|
| Transition rate at a declared transition edge | Semiconductor tunnel junction | Nonzero where the primary kernel alone predicts zero | Published measurement record |
| Field emission current | Metal surface under applied field | Nonzero below classical barrier | Fowler-Nordheim confirmed |
| Alpha decay rate | Po-210, Ra-226 (representative) | Geiger-Nuttall relation confirmed | PDG 2024 |

Each observation records a nonzero rate at a declared transition edge where
the primary kernel alone, acting on unresolved mechanism, predicts zero. The
tunneling invariant is the declared relational structure compared against
these records.

**Scale and domain invariance:** ε[e] > 0 at every declared transition edge
regardless of region. No declared transition edge has been confirmed through
M with ε[e] = 0.

**Framework-level necessity note:** The claim that absence of tunneling
would preclude energy transfer is a framework-level observation — not yet
formally derived from the kernel (OC-T-3).

**Active open conditions:** OC-T-1, OC-T-2, OC-T-3

**Support classification:** Derived (from ε[e], τ[v], 𝟙[e]).
Observed — expression-observed, mechanism-unresolved (Primary Region);
locus-resolved (Atomic Region and larger).

---

## VR-S-01 — Superposition Invariant (I-S)

**Quantity (Origin declaration, 2026-09-23):** The superposition condition
is the condition in which the Δ output field carries relational
distinguishability before B accumulation resolves it: there exist declared
edges e and f such that Δ(x)[e] and Δ(x)[f] are not proportional — there is
no scalar k with Δ(x)[e][c] = k · Δ(x)[f][c] for every component c. The
condition is read from the calculated Δ output over declared projections
through M. No matrix construction, no decomposition, no threshold.

**Resolution:** structural — the condition resolves when M establishes at
least one interior locus (full-operator admissibility condition,
`operators_notation_and_constraint.md`), not at a numerical threshold.

**Previous formulation (retired):** rank(Im Δ) > 1 with ρ_P ≪ 1. rank(Im Δ),
rank(Im Σ), and ρ_P are retired; the Origin question of what observable
relational condition rank(Im Δ) > 1 was intended to distinguish is answered
by relational distinguishability.

**Observable provenance (scale-conditional):** Expression-observed at the
Primary Region (ℛ_M = 1, 𝓜_M = 0) — distributed directed contrast across
multiple declared edges is in the observable record through M; the mechanism
by which multiple non-proportional directed contrasts are simultaneously
operative is not yet resolvable by M at this scale. Locus-resolved at larger
scales (ℛ_M = 1, 𝓜_M = 1).

**Observable record:**

| Observable | System | Result | Source |
|---|---|---|---|
| Interference pattern | Double-slit (electrons, photons) | Consistent with multiple non-proportional directed contrasts operative before B accumulation | Published measurement record |
| Measurement outcome distribution | Atomic state preparation | Consistent with multiple non-proportional directed contrasts operative before B accumulation | Published measurement record |

Each observation is recorded after resolution. The comparison recorded here
is between these observations and the relational distinguishability
condition declared above.

**Framework-level necessity note:** The claim that the absence of relational
distinguishability throughout a declared history precludes self-organization
within D is a framework-level observation — not yet formally derived from the
kernel (OC-S-3).

**Implementation hold:** `are_proportional` in `derived_invariants.rs` is
held — its result depends on edge order when a Δ vector is 0.0 in every
component, and its tolerance is used in two units. No code change is
authorized.

**Active open conditions:** OC-S-2, OC-S-3. OC-S-1 closed (2026-09-23).

**Support classification:** Declared condition (Origin, 2026-09-23), read
from the calculated Δ output. Observed — expression-observed,
mechanism-unresolved (Primary Region); locus-resolved (larger scales).

---

## VR-E-01 — Entanglement Invariant (I-E)

**Quantity:** Declared edge e=(s,t) with 𝟙[e]=1 establishes a relational
constraint between loci s and t preserved under the operators regardless
of spatial separation within D.

**Decoherence:** 𝟙[e]=0 and ε[e]=0 — the admissible statement of
decoherence within D. Removing the declared relation removes the
correlation. This is the admissible test of the invariant through M.

**Observable provenance (scale-conditional):** Expression-observed at the
Primary Region (ℛ_M = 1, 𝓜_M = 0) — correlated measurement outcomes at s
and t are in the observable record through M; the internal mechanism
constituting the edge relation is not yet resolvable by M at this scale.
Locus-resolved at larger scales (ℛ_M = 1, 𝓜_M = 1).

**Confirmed consequences:**

| Observable | System | Result | Source |
|---|---|---|---|
| Correlated measurement outcomes | EPR-type particle pairs | Bell inequality violations — correlation exceeds what is admissible without a declared edge | Published measurement record |
| Molecular bond directionality | 22 transition edges | 𝟙[e]=1 confirmed, directed output at each | PDG 2024, NIST ASD v5.12 (VR-γ-01) |

No observation directly observes the edge relation. Each observes a
correlation that is inadmissible without it.

**Scale and domain invariance:** 𝟙[e]=1 condition identical regardless
of region. At larger scales expressed through structural relations
(molecular bonds, gravitational coupling, systemic dependency) rather
than particle correlations — operator statement identical.

**Framework-level necessity note:** The claim that 𝟙[e]=0 everywhere
precludes collective organization is a framework-level observation —
not yet formally derived from the kernel (OC-E-3).

**Active open conditions:** OC-E-1, OC-E-2, OC-E-3

**Support classification:** Derived (from 𝟙[e], R antisymmetric output, ε[e]).
Observed (consequences only — correlated outcomes and bond directionality,
Primary and Atomic regions).

---

## VR-RE-01 — Relational Evolution Invariant (I-RE)

**Quantity:** Relational evolution rate is monotonically decreasing from
the Primary Region outward. The Primary Region is the fastest-evolving
declared region in D.

**Formal expression:** τ[v] at the Primary Region produces the smallest
declared relational progression intervals in D. At every larger declared
region, τ[v] is greater — relational progression is slower.

**Observable provenance:** Relational evolution rate is not directly
observable. What is observable through M are its consequences — τ[v]
values across declared regions and Φ[v] scaling across 78 decades.
These are C projections of the underlying relational evolution rate
onto scalar interval observables.

**Confirmed consequences:**

| Region | τ[v] range (representative) | Φ[v] log₁₀ range | Ordering confirmed |
|---|---|---|---|
| Primary | ~10⁻²⁵ to 10⁻²¹ s | +36.53 to +39.47 | Yes — fastest |
| Atomic | ~10⁻¹⁶ to 10⁻¹⁵ s | +45 to +52 | Yes |
| Molecular | ~10⁻¹² to 10⁻⁹ s | +50 to +56 | Yes |
| Biological | ~10⁻³ to 10² s | +58 to +62 | Yes |
| Planetary | ~10⁶ to 10¹⁰ s | +59 to +62 | Yes — slowest confirmed |

Monotonic ordering confirmed: no declared region violates τ[Primary] < τ[all larger regions].
Zero violations across 29 declared loci spanning 78 decades (VR-Φ-01).

**Consequence for I-T, I-S, I-E:** Tunneling, superposition, and
entanglement are most active and most visible as distinct phenomena at
the Primary Region precisely because τ[v] is smallest there. At larger
scales the same conditions are present but τ[v] is large enough relative
to observation windows that consequences appear as stable structure rather
than active evolution.

**Consequence for accumulated structure:** Persistent structure at larger
scales is Primary Region relational evolution accumulated into confirmed
persistence through B activation. The 78-decade Φ[v] span is the
observable expression of this accumulation.

**Framework-level necessity note:** The claim that every larger-scale
structure is formally downstream of Primary Region relational evolution
is a framework-level observation — not yet formally derived from the
kernel (OC-RE-1).

**Active open conditions:** OC-RE-1, OC-RE-2, OC-RE-3

**Support classification:** Derived (from τ[v], Φ[v], ι[v], region ordering).
Observed (consequences only — τ[v] and Φ[v] scaling across declared
regions, 78-decade confirmation, VR-Φ-01).

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
| OC-γ-1 | κ[Atomic] from photon edges — requires paired ι[emitting locus] declaration | Open | VR-γ-01 |
| OC-γ-2 | N[v] extension across all NIST ASD loci with declared τ_stability | Open program | VR-γ-N-01 |
| OC-T-1 | Tunneling floor magnitude — formal expression not yet derived | Open | VR-T-01 |
| OC-T-2 | Tunneling at scale — confirm beyond Primary and Atomic regions | Open | VR-T-01 |
| OC-T-3 | Tunneling necessity — absence precludes energy transfer (framework-level) | Open | VR-T-01 |
| OC-S-1 | Kernel boundary criterion — CLOSED 2026-09-23: the boundary is the full-operator admissibility condition, not a numerical threshold | Closed | VR-S-01 |
| OC-S-2 | Superposition at scale — confirm relational distinguishability of the Δ output beyond the Primary Region | Open | VR-S-01 |
| OC-S-3 | Superposition necessity — absence of relational distinguishability throughout a declared history precludes self-organization (framework-level) | Open | VR-S-01 |
| OC-E-1 | Entanglement preservation — derive 𝟙[e]=1 preserved under operator evolution | Open | VR-E-01 |
| OC-E-2 | Decoherence rate — derive rate at which 𝟙[e]→0 | Open | VR-E-01 |
| OC-E-3 | Entanglement necessity — 𝟙[e]=0 everywhere precludes collective organization (framework-level) | Open | VR-E-01 |
| OC-RE-1 | Downstream derivation — B activation at larger scales is accumulated Primary Region evolution | Open | VR-RE-01 |
| OC-RE-2 | τ[v] lower bound — no declared region has τ[v] smaller than Primary Region | Open | VR-RE-01 |
| OC-RE-3 | Evolution rate expression by domain — biological and computational regions | Open | VR-RE-01 |

---

## What Remains for Data Entry

**VR-J-01:** 23 additional fine structure levels to complete the 52/52 table.
**VR-γ-N-01:** N[v] extension across additional NIST ASD loci (OC-γ-2).
**VR-T-01:** Formal confirmation entries when OC-T-1 and OC-T-2 are resolved.
**VR-S-01:** Scale confirmation entries when OC-S-2 is resolved.
**VR-E-01:** Formal preservation derivation when OC-E-1 is resolved.
**VR-RE-01:** Domain expression entries when OC-RE-3 is resolved.

---

*Bounded over D. No claim beyond D.*
*Metatron Dynamics, Inc. Kernel V8.*

---

**V6 → Kernel V8 changes (2026-09-26):** *(a) Declarations of 2026-09-23 not previously written into this document (reconstructed from operators_notation_and_constraint.md, derived_invariants.rs, and the repository README):* VR-S-01 redeclared with the relational distinguishability condition (no longer suspended); OC-S-1 closed, OC-S-2 and OC-S-3 restated; VR-T-01 restated in framework-native terms (ε[e] > 0 at declared transition edges; "quantum boundary component", "finite barrier", and "classically suppressed" removed from the operative statement); VR-T-01 and VR-E-01 observable provenance restated scale-conditionally in place of "not directly observable"; support classification protocol extended with scale-conditional Observed subcategories and the downstream projection label; the δι log-log fit labeled a downstream projection. *(b) Correction carried forward:* the correlation statistic (0.726) at the deviation-from-additivity entry is removed. A 2026-07-28 Verifier pass (finding F6) removed it because correlation is inadmissible absent an ensemble declared by Origin through M, and no ensemble was declared at that entry; that correction was recorded in a separate local copy of this document and was not published until this revision. *(c) Kernel V8 declarations:* values from M described as numerical projections into D; comparisons described as correspondence between observations and calculated results. Held: `are_proportional` implementation (noted at VR-S-01). Not changed: VR-E-01 decoherence wording on 𝟙[e] = 0 (Part II audit, separate). Companion references updated to derived_invariants.rs (Kernel V8). Kernel release numbering (Origin declaration): this document's own numbering ends at V6.

**Historical document-local revision history — predates unified Kernel V8 release numbering.** The entries below record this document's own revisions; their version numbers are document-local, not kernel release versions.

**V1:** Initial document. Nine validation entries.

**V1 → V2:** Four editorial actions — observation vs framework interpretation
separated; row-level support classifications added; "No theoretical imports"
replaced with "No additional theoretical assumptions beyond declared M";
CA-1 through CA-4 reformatted.

**V2 → V3:** ι[v] replaces m[v] throughout. VR-m-01 → VR-ι-01. m[v] as
primitive declared inadmissible (SF-PR-16). ι[v] declared as relational inertia.
Numerical values unchanged. OC-1, OC-2, OC-3 → OC-ι-1, OC-ι-2, OC-ι-3.

**V3 → V4:** VR-γ-01, VR-γ-τ-01, VR-γ-N-01 added. Photon edge functions
confirmed. OC-γ-1 and OC-γ-2 added. Companion file updated to V3.

**V4 → V5:** Four new invariant validation entries added corresponding to
derived_invariants.rs V4. VR-T-01 (Tunneling): consequences observed at
declared transition edges; expression-observed, mechanism-unresolved at
Primary Region; three confirmed consequence types. VR-S-01 (Superposition):
consequences observed as interference patterns and outcome distributions;
expression-observed, mechanism-unresolved at Primary Region. VR-E-01
(Entanglement): consequences observed as Bell inequality
violations and bond directionality; expression-observed,
mechanism-unresolved at Primary Region;
decoherence as admissible test through M. VR-RE-01 (Relational Evolution):
monotonic τ[v] ordering confirmed across 29 loci spanning 78 decades;
Primary Region confirmed as fastest-evolving declared region; 78-decade Φ[v]
span cited as observable expression. Twelve new open conditions added:
OC-T-1 through OC-T-3, OC-S-1 through OC-S-3, OC-E-1 through OC-E-3,
OC-RE-1 through OC-RE-3. Companion file updated: derived_invariants.rs V3 → V4.

**V5 → V6:** VR-S-01 suspended — rank(Im Δ) and ρ_P suspended in
`operators.rs` V7 (purge) and `derived_invariants.rs V4.2 (FROZEN)`.
Observable record preserved; mathematical interpretation suspended;
OC-S-1, OC-S-2, OC-S-3 reopened. Purpose section and version history
companion file reference updated to `derived_invariants.rs V4.2 (FROZEN)`.
Version updated to V6.
