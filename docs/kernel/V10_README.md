> **Synchronized copy.** Source: `abr-relational-chemistry` `KERNEL_V10_README.md`, branch `claude/tender-galileo-gcquso`, commit `0058d30`. Paths beginning `sessions/` refer to that repository. The Foundation Declaration (spine) is `sessions/2026-11-14/FOUNDATION_DECLARATION_DRAFT.md` there. Edit the source and refresh this copy; do not edit this copy alone.

# Kernel V10: proposed edits and updates (reference)

**Metatron Dynamics, Inc.** Bounded over D. No claim beyond D.
Compiled 2026-11-16 from every V10 proposal, declaration and decision recorded in `abr-relational-chemistry` (branch `claude/tender-galileo-gcquso`) and `abr-kernel`. Baseline: **Kernel V9, `abr-kernel` `833ffb3`**.

This is the document we refer to from now on. It lists every V10 item from top to bottom: first the relational declaration of variables and conditions, then the formal mathematics, then traceability, code scope, what stands, conflicts and open decisions.

- **Spine.** The full text of most items is in `sessions/2026-11-14/FOUNDATION_DECLARATION_DRAFT.md` (the Foundation Declaration, FD), with four amendments of 2026-11-16. Where an item comes from somewhere else, its source is named.
- **Synchronized with the FD (2026-11-16).** Items decided or proposed in other documents were merged into the FD at their place in the chain, each keeping its status (FD "Synchronization" block). They are marked *(merged into FD 2026-11-16)*. Application-layer items (specific systems, data and field decisions) stay out of the FD under its boundary rule and are listed in Part 7.
- **Keeping them in step.** A change to a V10 item is made in the FD and in this README in the same commit. The copy in `abr-kernel` `docs/kernel/V10_README.md` is refreshed from this file.
- **Older chemistry declaration.** `FOUNDATION_DECLARATION.md` at the repository root is the earlier chemistry declaration under V9 (2026-11-10). It is not part of V10.

### Status labels

| Label | Meaning |
| --- | --- |
| **Ground** | Constitutive of observing in the frame |
| **Observed** | Present in what the human reference frame registers |
| **Derived** | Follows from earlier items |
| **Declared / Decided** | Origin has chosen and taken responsibility (date given) |
| **Proposed** | Drafted, awaiting Origin |
| **Open** | Not yet declared |
| **Withdrawn** | Declared once, then withdrawn (kept for the record) |

Each item has an ID and rests only on items above it. Read top to bottom, it goes from observation to mathematics. Read upward through "rests on", it goes from any formula back to observation.

---

## The relational declaration

Every V10 item furthers these three, and none may contradict them:

1. **Relation is the primitive** (Df1). The minimal relation registered already includes the observer's participation (Df4).
2. **Relational evolution is one-directional and continuous** (Df2, O1). It never runs backward and never stops, including while we analyse it. Looking back is analysis, not reverse evolution (Df2a).
3. **Energy, connection and freedom are observable throughout relational evolution** (G1, G4). The kernel expresses them as tunneling, entanglement and superposition. What we measure is their degree at declared resolution, and what each means in a given system is declared for its domain (R9a). An intervention or a coarse resolution bounds their degree, never their presence.

---

## The structure in one line

```
 … --> { --> --> --> --> --> } --> …
       └─ cycle of analysis ─┘
```

*(Origin, 2026-11-16: `{-->-->-->-->-->}`, "this is what we're talking about".)*

- **The arrows are relational evolution.** Every arrow points the same way. None points back, and none may be read or computed as pointing back (Df2).
- **The braces are the scope of analysis.** The observer chooses them (R10). They are not a boundary of the field, which runs on past both braces and keeps evolving while we analyse (Df3).
- **Inside the braces, reading is free.** We may read in either direction, from the last arrow, or back to an arrow's predecessor. No reading turns an arrow around (Df2a).
- **Each arrow is read with its immediate neighbours, and every arrow in one pass** (Df3: encompass, do not blur).
- **The next braces pair with this cycle's output** (K5). Arrows already accumulated are not accumulated again (scope). They may be read again (Df2a).
- **An arrow with no neighbour inside the braces** calls for wider braces (more registrations), never a reversed arrow.

---

# Part 1. Relational declaration of variables and conditions

## 1.1 The frame and the ground

| ID | Statement | Status | V10 change |
| --- | --- | --- | --- |
| **H** | The human reference frame: the constant and only available reference within D. Every M is declared relative to it (M_H : O → D). Every instrument, model or AI frame is declared, read and compared within H. H enters the topology through declared observer edges | Ground; Proposed (`2b22e00`, `cc9ba31`) | New in kernel text. Open: H's own window (T_W, δ_W); OC-F-1 (H as a shared frame) |
| **O1** | Non-stop progression: relational evolution has not been observed to stop. Change is progression, not replacement; a relation can persist while evolution runs through it | Observed (Origin, 2026-11-14) | New |
| **O2** | Complexity: no registration stands alone | Observed (KR §1) | Restated |
| **O3** | Bounded in extent: each declared record, window and domain is finite | Observed (KR §1) | Restated |
| **O4** | Bounded in resolution: each observing chain has a smallest difference it registers | Observed | New |
| **Df1** | **Relation is the primitive.** The minimal observable is a directed relation declared from observable information. Objects are read from relations, not the reverse | Derived (O1, O2) | Restated as derived |
| **Df2** | **One direction of evolution.** Relational evolution has exactly one admissible direction, traced to an observable property of the process (not an index, row order or label). It does not return. **Forbidden:** any assumption, computation or interpretation in which evolution runs both ways: (s, t) and (t, s) read as one relation evolving both ways; a later output used as the prior of an earlier one; a later registration read as an earlier state restored; a dynamics assumed symmetric under reversal unless that symmetry is registered; an edge direction chosen for convenience, index order or a non-zero reading | Derived; forbidden list Declared (Origin, 2026-11-16, 4th amendment) | Forbidden list new |
| **Df2a** | **Direction of analysis: free.** The observer may read backward or forward: reading what precedes (Σ_pred), reading a relation from either end (−A, same relation), past records, earlier cycles, reverse traceability, any computation order that does not change the result. Each backward reading keeps every relation's declared direction and states that it is a reading. It is not a licence to count the same evolution twice (that is scope, Df3) | Declared (Origin, 2026-11-16, 4th amendment) | **New** |
| **Df3** | **No registered beginning or end.** Any first observation or boundary is a record boundary cut into an ongoing relation. No cold start. **Operator clause, "encompass, do not blur":** three things are held distinct: the ongoing field, the cycle of analysis, and the consecutive observations inside it. *Within a cycle:* cover every relation in one pass (no mid-series restart); read each relation with the observations consecutive to it (no sums of non-consecutive observations). *Between cycles:* pair only with the previous cycle's output; do not re-accumulate what it carries; re-reading is admissible | Derived; operator clause Declared (Origin, 2026-11-16, 1st–4th amendments) | **New operator clause** |
| **Df4** | **Observation is a relation.** Each registration intersects observable and observer. Participation kinds: receive, probe (supplies energy), prepare (sets conditions) | Derived; kinds Proposed (RS-5) | New |
| **Df5** | **Bounded language.** Quantifiers range over declared finite domains. The admissible negative is *not observed under the declared relations* | Derived (Origin, 2026-11-14) | New |
| **G1** | **Energy, connection and freedom** are present in each registration. Absence is not an available observation. They are observational constants, not axioms; **their degree, at declared resolution, is the variable** | Ground; Proposed (`2b22e00`; RS-3) | New |
| **G1-K** | **The kernel expresses them as tunneling (I-T), entanglement (I-E) and superposition (I-S)**: energy ↔ tunneling, connection ↔ entanglement, freedom ↔ superposition (§1.2) | Origin declaration, 2026-11-12 (`sessions/2026-11-12/KERNEL_AMENDMENT_reference_frame_resolution.md` item 3); PROPOSED pending Verifier *(merged into FD 2026-11-16)* | Must be added to FD G1: the FD never names the three invariants |
| **G1-OC** | OC-T-3, OC-S-3, OC-E-3 (necessity statements about the *absence* of tunneling, superposition, entanglement) are **dissolved**: absence within H is not observed, so it is not available to derive from | Origin declaration, 2026-11-12 (same file) *(merged into FD 2026-11-16)* | OC-T-1 (derivation of ε[e] > 0) stays open |
| **G2** | **Antisymmetric circulation** is an observational constant; its degree is the variable. A calculated zero is a statement about the declared relations at δ. Antisymmetry is not reversing evolution, not a fabricated reverse registration, not a requirement that every quantity be antisymmetric | Ground; Proposed (`c2ea957`; RS-4) | New |
| **G3** | **Presence confirms; degree carries the finding.** Yes/no presence results confirm the ground and do not test it | Derived | New |
| **G4** | **Each observable system contains each declared variable:** the interior locus and the full path A → B → R, energy, connection, freedom (G1) and circulation (G2). We do not propose them; we find them | Ground (Origin, 2026-11-15; `sessions/2026-11-15/SYSTEM_EXPANSION_DRAFT.md`, X-a) *(merged into FD 2026-11-16)* | New |
| **G4a** | **A subject without an interior locus** is either analysed in more detail (finer δ, undeclared loci) or expanded (move the record boundary outward along registered relations). The interim reading is *interior not yet declared*, never "no interior" or "proposed interior". The same holds for any declared variable not yet shown | Declared (Origin, 2026-11-15, X-a) *(merged into FD 2026-11-16)* | New. G4b ("registered through successor") Withdrawn |

## 1.2 The variables of the ground: energy, connection, freedom, circulation

These are the variables every analysis reads. Each is present by ground (G1, G2, G4). What is measured is its **degree at δ**. The kernel functions are in `abr-kernel` `src/derived_invariants.rs` and are unchanged in V10. Their formal forms are in Part 2 (V1–V3, K4).

| Ground variable | Kernel invariant | Kernel reading | Graded reading at δ |
| --- | --- | --- | --- |
| **Energy**: how much a relation changes | **Tunneling, I-T** | ε[e] = \|Δι[e]\|·𝟙[e] (at photon edges hc/λ); `tunneling_floor_holds` (ε > 0), `tunneling_tau_bounded` (τ = ħ/ε finite) | ε / δ_ε (V3; δ_ε Proposed) |
| **Connection**: how much holds together as it changes | **Entanglement, I-E** | 𝟙[e] = 1 iff the antisymmetric term of R over B(A(x)) is non-zero; `entanglement_condition`; decoherence = 𝟙 = 0 and ε = 0 | 𝟙_δ (sign resolved at δ); C_δ form Open (V1) |
| **Freedom**: how many independent ways it varies | **Superposition, I-S** | at least two edges with non-proportional Δ vectors across components; `superposition_condition`; `superposition_resolved` = interior present | F_δ (V2); proposed d(e,f) (IV-2) |
| **Circulation**: imbalance between what follows and what precedes | (R − B) | (R − B)[e] = ρ[src]·(Σ_succ B − Σ_pred B) | sign is connection; magnitude only once ρ choices are declared (V4) |

**Each is declared per domain (R9a).** What energy, connection and freedom are depends on the relational system analysed, and is stated in its domain declaration before any operator acts. That declaration establishes D.

**Reading rule for all four:** a calculated zero, or a reading not resolved at δ, is a statement about the declared relations and their resolution, never a statement of absence (W2). The kernel's boolean forms use the implementation tolerance δ_C; the finding is the graded form at δ (G3, and 2026-11-16 finding A-2: the I-S boolean depends on tol).

## 1.3 Registration and measurement

| ID | Statement | Status |
| --- | --- | --- |
| **R1** | Observable: information within the human reference frame | Declared (Origin, 2026-11-14) |
| **R2** | Registration: an observable recorded through a declared observing chain; a distinction between a and b, through window W, against a declared reference | Proposed |
| **R3** | Registration descriptor ⟦a : b⟧_W = r ± δ, with 13 fields (identifier, relation, observable, chain and reference, window, participation, environment, observer steps, form, δ, status, M mapping, source). Forms: ratio (enters as the edge value), ordering (sign only), value against a reference (value in D through M). δ is carried as interval bounds; quadrature is not used | Proposed (RD); symbol to be chosen. OC-RD-1, OC-RD-2 Open *(merged into FD 2026-11-16)* |
| **R4** | Observer window W: extent T_W, resolution δ_W, σ_W = T_W / δ_W | Proposed (RS-2a) |
| **R5** | Environment: the energy of the surroundings relative to the relation, with its source, including what the observer prepared or supplied | Proposed (RS-5) |
| **R6** | M : O → D, declared by Origin before any operator acts. D := { x ∈ ℝⁿ \| n < ∞, \|x[i]\| < ∞ }. Not evaluated ≠ evaluated with result 0.0 | Declared (KR §1, §9) |
| **R7** | Value classes: Registered; Registered with declared offset; Projected (model; comparison only); Withheld; Provider-declared. Letters to be renamed so "D" is not the domain | Proposed |
| **R8** | Record ledger: each quantity with its class, extent, δ, σ and role. Outside the ledger is not observed under the declared relations | Proposed |
| **R9** | Regional M: system and boundary, chain, primitive variables, references, δ layers, window, participation, ledger, operator parameters. Apparatus becomes a locus or edge only by explicit declaration | Proposed |
| **R9a** | **Domain declaration.** The domain is the full scope of the relational system under analysis; D is its mathematical establishment. Before any operator acts, the domain declaration states what each ground variable is **in that system**: **energy** (what carries change; unit, mapping, δ_ε), **connection** (which relations carry continuation; loci and direction traced to an observable; δ for the sign of R − B), **freedom** (which components vary independently over common loci; why one field; δ for proportionality). It sets D's structure: loci and relations (connection), components with n = loci × components (freedom), what each value carries (energy). Meaning depends on the system, not only the region (two atomic systems: H/D level network carries energy in hν; the 2S–6P count field has no energy carrier). The Generator drafts it from the evidence; Origin declares it; committed before execution. An undeclared variable is *not evaluated* and named; one that cannot be formed is *not formable in this domain*. **Participation:** for each variable the declaration also states how the observer participates (prepare, probe, receive) and how that bounds it: freedom constrained by a prepared environment, energy supplied or confined, connection built by the observing chain. Readings in a prepared domain (e.g. a Penning trap) are of the system with its intervention; comparisons with natural domains (e.g. flare neutron capture, received only) state each side's participation first. An intervention bounds degree, never presence or direction | Declared (Origin, 2026-11-16, fifth amendment) — **new** |
| **R10** | **Cycle of analysis:** the observer's declared intersection with the ongoing field, chosen for comprehensibility (resolvable at δ, carryable σ), declared before execution. Its output is the next cycle's prior. A result describes the field *as intersected in its cycle*. Modes: **tracking** (cycles advance with arriving registrations) and **record analysis** (a past record; no claim of present alignment) | Declared (Origin, 2026-11-16) — **new** |

## 1.4 Variable register

The full register is `sessions/2026-11-14/VARIABLE_REGISTER_DRAFT.md` (DRAFT, Generator, 2026-11-14). Marks: ✔ defined, ◐ partial, ✖ missing, ⚠ inconsistent. Its row IDs collide with FD IDs (G, E, D) and with data labels. Renumbering is Proposed (§8).

| Group | Rows (mark) |
| --- | --- |
| **A. Frame and domain** | A1 H ◐ · A2 Observables ✖ · A3 Registration ✖ (in kernel) · A4 M ◐ · A5 D ◐ (structure of n addressed by R9a) · A6 Value class ⚠ · A7 Ledger ✔ · A8 Record boundary ✔ · A9 Region ◐ · A10 ℛ_M, 𝓜_M ✔ · A11 Participation ◐ · A12 Environment ✖ · A13 Descriptor ✖ |
| **B. Topology** | B1 Locus ◐ · B2 Edge ✔ · B3 Direction ✔ · B4 Provenance class ✔ · B5 Support ✔ (*superseded by T3 amended*) · B6 succ/pred/adj ◐ (A-3) · B7 Interior ✔ · B8 Component ✖ · B9 Component pair ◐ · B10 Process step ✔ · B11 Observation/field ◐ |
| **C. Operator parameters** | C1 ρ ◐ · C2 χ ⚠ (code adds component-pair asymmetries; text does not) · C3 ρ_base ◐ · C4 χ₀ ◐ · C5 cc[p] ◐ · C6 tol = δ_C ✔ |
| **D. Operators and outputs** | D1 A, Δ ✔ · D2 B ⚠ (called a vector in OVS; scalar in kernel and code) · D3 R ✔ · D4 Σ ✔ · D5 E ✔ · D6 A_p, B_p, R_p ✔ · D7 (R − B) ✔ · D8 C projection ⚠ (σ² in examples) |
| **E. Resolution and window** | E1 δ_N, δ_M, δ_C ◐ · E2 σ ◐ · E3 E_δ ✔ · E4 W ✖ (in kernel) |
| **F. Graded degrees** | F1 𝟙 ✔ · F2 𝟙_δ ✔ · F3 C_δ ✖ (three readings) · F4 F_δ ◐ (> 2 components) · F5 ε/δ_ε ✖ · F6 observer circulation ◐ |
| **G. Derived invariants** | G1 ι ◐ · G2 q ✔ · G3 variable 3 ✖ · G4 ε ✔ (HOLD on A-3) · G5 τ ◐ · G6 τ_stability, N ◐ · G7 θ, θ̂ ⚠ (m_e, Z_eff, electronegativity not declared through M) · G8 Φ ✔ · G9 J ✔ · G10 E[v] = ι·κ ◐ (EF-17) · **G11 I-T, I-S, I-E ✔** · G12 I-RE ✖ (OC-RE-1) |
| **H. Candidates** | H1 Γ_W, Λ_W, Λ̂_W ◐ · H2 Κ ◐ (OC-RS-4) · H3 ΔQ ◐ |

**Proposed variables from later drafts:**
- **Relational inertia ι** (`sessions/2026-11-15/RELATIONAL_PHYSICS_DRAFT.md`, P1; Proposed; *merged into FD Part VI as P1, 2026-11-16*). Mass is replaced by relational inertia, defined relative to surrounding relations by behaviour and contribution (Origin, 2026-11-15).
  - P1.1 ⟦ι[a] : ι[b]⟧ = ⟦ω[b] : ω[a]⟧ · ⟦q[a] : q[b]⟧;
  - P1.2 δι[S] = Σᵢ ι[vᵢ] − ι[S];
  - P1.3 ⟦δι[S] : ε_formation⟧ through a declared cross-carrier mapping (mapping Open);
  - P1.4 at the Primary Region, inertia read at the edge, x = (ε[e], q[v]);
  - P1.5 language: "mass defect" → δι, "binding energy" → ε_formation, "rest mass" → ι under A_p = 0.
  - Decisions P-a to P-f Open.
- **Isotope edge** as a relation kind (*merged into FD T3, 2026-11-16*; ED-a, `sessions/2026-11-15/ELECTRON_DEUTERON_EXPANSION_DRAFT.md`; Proposed). Related: IV-1.
- **Electron**: a locus within the ion, not a separate system; its theory input is a mark on the value in M (Origin, 2026-11-15; `TRAP_RELATIONAL_ANALYSIS_DRAFT.md`). *Application layer: kept out of the FD (boundary rule).*

## 1.5 Conditions

### Topology

| ID | Statement | Status |
| --- | --- | --- |
| **T1** | Locus: a declared endpoint of declared relations; node-valued (spatial) or edge-valued (persistence) | Declared |
| **T2** | Edge e = (s, t), one admissible direction traced to an observable. The reverse needs independent provenance; if both are observed they are distinct relations. **Two ways to go "the other way", kept apart:** reading e from t is analysis (−A, same relation, Df2a); declaring (t, s) asserts a second relation (Df2) | Declared; 4th-amendment clause new |
| **T3** | Each edge carries a provenance class: Continuation, Coupling, Persistence. **Support (amended): Registered or Not yet registered.** A not-yet-registered edge is a gap (G4a applies). No edge is held as a proposal, and none is inferred from another registration. A model enters only as a Projected comparison | Declared; **support amended (Origin, 2026-11-15, X-a)** *(merged into FD 2026-11-16)* |
| **T4** | succ(e) = edges whose source is e's target; pred(e) = edges whose target is e's source. Immediate neighbours only. Empty sum evaluates to 0.0 (definition Open, A-3) | Declared |
| **T5** | Component: values per locus registered through distinct parts of one observing chain over the same loci. Declaring components is not neutral (they share ρ) | Proposed (Origin, 2026-11-14) |
| **T6** | Process step and sequence {M(o₁), …, M(oₙ)}, ordered by an observable property of the process | Declared |
| **T7** | Interior locus t = target(e) = source(f). Interior present → A → B → R; otherwise Δ → Σ. **Restated:** Δ → Σ applies where the *observer's resolution* does not yet declare the interior; the interior is present, not yet declared at that δ. No Primary Region mathematics added | Declared; **restatement Declared (Origin, 2026-11-15, X-b)** *(merged into FD 2026-11-16)* |
| **T8** | No wraparound; feedback loops admissible with provenance; opposite directions of an arrangement relation not in one field; no difference-altering transform before A or Δ unless declared in M. **Acyclicity is checked and refused if violated** (V10 code) | Declared |

### Direction declarations in use

| ID | Rule | Status |
| --- | --- | --- |
| **EN-a** | Coupling edges run **faster → slower** (relational evolution is one-directional regardless of inputs or conditions) | Decided (Origin, 2026-11-16; `TRAP_NETWORK_E_VS_N_DECLARATION.md`) *(merged into FD 2026-11-16)* |
| **HD-a** | Capture edges run target → product (⁴He diamond) | Generator's declaration; Origin may amend *(merged into FD 2026-11-16)*. Check against EN-a (§6) |
| OC-K-3a | Order of the two routes in a K7 closure | Open; proposed as an order of analysis (Df2a) |

### Reading rules

| ID | Rule | Status |
| --- | --- | --- |
| **W1** | Presence confirms; degree carries the finding | Derived |
| **W2** | Not resolved at δ = *not observed under the declared relations at δ*. No statement of absence | Proposed |
| **W3** | Each variable is reported as the system's contribution and the observer's participation, where the record separates them | Proposed |
| **W4** | Structure that follows the observing arrangement is read as structure of the intersection first | Proposed |
| **W5** | A result that seems to contradict the ground is returned to Origin as a statement about the declaration, M, topology, δ or chain. It is not repaired by changing a declaration to obtain presence | Proposed |
| **W6** | Statistical, geometric or external quantities enter only as a declared projection C, declared before use | Proposed; statistics as declared C approved (Origin, 2026-11-14) |
| **W7** | **"Potential"** is reserved for projected exchange under declared conditions. Energy a system holds is **"energy held in connection"**. "Coherence potential" is withdrawn | Declared (Origin, 2026-11-14) *(merged into FD 2026-11-16)* |
| **W8** | **Never reverse a declared edge to obtain a reading;** the remedy is a wider intersection (Df2, Df2a; 2026-11-16) | Declared with the 4th amendment |
| **W9** | **Classification of structure:** *system structure* closes across chains at every mapping order; *chain structure* is a divergence that persists across mapping orders; *mapping structure* is a divergence that moves with mapping order | Proposed (drafted Nov 15; to be declared as a reading rule) *(merged into FD 2026-11-16)* |
| **W10** | **Relational invariant:** a structure that holds across independent observing chains (W4), at every mapping order where it can be formed, and across consecutive cycles (R10). Persistence across a mapping order only by construction (a factor that cancels) is not a test at that order | Proposed (Nov 16 handoff) *(merged into FD 2026-11-16)* |
| **W11** | **Rule of inclusion:** each declared variable is read for each system; where one is not declared, the missing declaration is named | Origin, 2026-11-15 (`PRIMAL_COMPARISON_DRAFT.md`); PC-a Open *(merged into FD 2026-11-16)* |
| **W12** | **Reference frame of comparison.** Results are stated as observed within H, bounded within D. The standard frame of comparison is abstract, not observed; a relational result is not judged against it, and no outside-validation statement is made. Legacy mathematics is not discarded: it enters as declared mappings (the ladder) and its values as Projected or Provider-declared comparison quantities | Declared (Origin, 2026-11-16) |

### Method (carried from the handoffs)

- Declare before running; commit the declaration first; disclose what is already known.
- Node-derived fields make closure an identity; only independently registered edges test closure.
- Registrations are not mappings. Radii, adjusted constants and composition edges are Projected. Never compare against an adjusted value as if it were a registration.
- Provider intervals are an interim δ (M3 corners), not Z3. Seek repeated registrations.
- Check the numerics: a central value outside its own interval means the solver, not the data.
- Kernel text names a specific system only inside a marked **Example (application)** block (boundary rule, Origin 2026-11-14).

---

# Part 2. Formal mathematical definitions

Operators record how they are grounded: **forced** (no alternative), **motivated** (Origin chose the form), **declared** (value supplied by M). **No operator formula changes from V9 to V10.** V10 changes scope, execution and wording, and adds K7.

## 2.1 Operators

| ID | Mathematics | Grounded | V10 change |
| --- | --- | --- | --- |
| **K1 A** | A(x)[e] = x[s] − x[t] for each declared e = (s, t) and component. Persistence: A_p[e] = E_current[e] − E_prior[e], prior always the earlier output. Δ = A under its primary-kernel name | Forced (Df1) | Prior direction stated (Df2) |
| **K2 B** | B(g)[e] = g[e] + Σ_{f ∈ succ(e)} g[f]. Consecutive (immediate successors). Terminal edge accumulates nothing. Scalar per component. Not evaluated in the primary path. Accumulates in the declared direction only: an arriving relation is never added in | Motivated (O2, Df2); consecutive form grounded in Df3 | Evaluated for every relation in one pass. **Withdrawn:** whole-range B = g[e] + Σ_{down(e)} g[f] |
| **K3 ρ** | ρ[i] = ρ_base · χ[i] / (χ₀ + χ[i]); ρ[e] = ρ[src(e)]. χ[i] = max \|A(x)[e]\| over declared edges incident to i and over each component (max-selection Declared, Origin 2026-11-14); with component pairs, χ also takes \|comp[p][i]\|. ρ_base, χ₀ from M; general values 0.5 and 1 | Declared; saturating form Open (OC-K-1) | Unchanged. Text to state the component-pair term (VR C2) |
| **K4 R** | R(g)[e] = g[e] + ρ[src(e)] · (Σ_{succ(e)} g − Σ_{pred(e)} g), g = B(A(x)). Component pairs p = (a, b): the asymmetry g_comp[p][dst(e)] − g_comp[p][src(e)], scaled by ρ[src(e)]·cc[p], is added to a and subtracted from b; comp[p][i] + ρ[i]·(Σ_{out(i)} g_a − Σ_{out(i)} g_b). Σ = R on Δ(x) without B. **Σ_pred is the operator reading backward** (Df2a); every pred edge keeps its direction | Motivated (G2) | One pass; consecutive. **Withdrawn:** whole-range Σ_down, Σ_up |
| **K5 E** | E(x) = R(B(A(x)), ρ(A(x))). Persistence A_p → B_p → R_p, spatial component only, ρ_p from A_p. **One-step pairing:** each cycle pairs only with the immediately preceding cycle's actual output; never a later output as an earlier prior. Within a cycle, consecutive and encompassing | Declared | One-step pairing Declared (2026-11-16); OC-K-4 closed |
| **K6 C** | Declared projection: a reduction of operator output to fewer numbers (share, bound, variance), declared before use, stating what it keeps and drops. Not a kernel operator | Declared | Variance under C (Origin, 2026-11-14) |
| **K7** | **Route accumulation and closure.** Route π = (e₁, …, e_m), a directed sequence within the cycle's field. B_π(g) = Σ_{e ∈ π} g[e]. For two routes with the same source and target: A_π(π₁, π₂) = B_π₁(g) − B_π₂(g). Parallel edges are routes of length one. Holds at δ when the interval of A_π contains 0. On node-derived fields (g = A(x)) closure is 0 **by identity**. Needed because R − B is blind to route disagreement | Declared (Origin, 2026-11-16) | **New.** Direction OC-K-3a Open. **Cycle closure (HB-1, Origin 2026-11-17):** for any cycle, A_cyc = Σ s_e g[e] (s_e = +1 along, −1 against the declared direction; traversal is an order of analysis, Df2a); holds at δ when its interval contains 0; independent set = a cycle basis from a stated spanning tree; two-route K7 is the one-source, one-sink case |

## 2.2 Kernel invariant functions (unchanged; `derived_invariants.rs`)

| Function | Definition |
| --- | --- |
| `detection_indicator` | 𝟙[e] = 1 if \|antisymmetric term of R at e\| > tol, else 0. 𝟙 is read from R output; it is not a parameter |
| `epsilon_e` | ε[e] = \|Δι[e]\| · 𝟙[e] (units of ι, declared per region) |
| `epsilon_photon_edge`, `tau_photon_edge` | ε = hc/λ; τ = ħ/ε (Regional M constants) |
| `tunneling_floor_holds` / `tunneling_tau_bounded` | **I-T:** ε[e] > tol / τ = ħ/ε finite and > 0 |
| `entanglement_condition` / `decoherence_condition` | **I-E:** 𝟙[e] = 1 / 𝟙[e] = 0 and \|ε[e]\| ≤ tol |
| `superposition_condition` | **I-S:** true when at least one edge pair has non-proportional Δ vectors across components. One component: false by construction (any two scalars are proportional) |
| `superposition_resolved` | true when an interior locus is present (structural, no threshold) |

HOLD items carried from V8: `are_proportional` is edge-order dependent when a Δ vector is all-zero, and uses tol both absolutely and relatively; the 𝟙[e] audit follows A-3.

## 2.3 Resolution

| ID | Definition | Status |
| --- | --- | --- |
| **Z1** | δ declared per component and layer: δ_N (system), δ_M (chain and data), δ_C (code tolerance, implementation-only). A distinction is reported at the coarsest layer it passes through | Proposed |
| **Z2** | σ = extent / δ (resolvable distinctions in the extent) | Proposed; OC-Z-1 ("extent", renaming σ) |
| **Z3** | Resolved at δ: the same classification in each registration of an observed ensemble E_δ (repeated registrations, declared order, windows of K). A seeded ensemble is projected (a comparison only) | Proposed (`fe0bf03`; observed ensembles approved 2026-11-14) |
| *Interim* | M3: corners of provider-stated ±u intervals, labelled "not Z3" | Approved per field (EN-e) |

## 2.4 Degrees (graded forms)

| ID | Variable | Mathematics | Status |
| --- | --- | --- | --- |
| **V1** | **Connection (entanglement)** | 𝟙_δ[e] = 1 when (R − B)[e] = ρ[src]·(Σ_succ B − Σ_pred B) is non-zero with sign resolved at δ (sign independent of open ρ choices, needs only ρ > 0). At δ_C this is 𝟙[e]. C_δ[e] (preservation over a process sequence): three readings (Ca, Cb, Creg) reported until one is declared. **Proposed (IV-3):** an edge with no continuing and no arriving relation in the intersected field has R − B = 0 by topology; it is reported as *connection not formable in this field* (scope), not as decoherence | 𝟙 Declared; graded Proposed; C_δ Open (OC-V-1) |
| **V2** | **Freedom (superposition)** | Edges not resolved at δ reported separately. Pair (e, f) distinguishable at δ when non-proportionality across components is resolved; with two components the contrast A[e][0]·A[f][1] − A[e][1]·A[f][0] has resolved sign. F_δ = share of distinguishable pairs. Pairwise only, no rank. **Proposed alternative (IV-2):** d(e,f) = r_f / r_e − 1 with r_e = A[e][1] / A[e][0], read with its interval and \|d\|/δ | Proposed; > 2 components Open (OC-V-2); IV-2 Open |
| **V3** | **Energy (tunneling)** | **FD form:** ε[e] = \|A(ι)[e]\| · 𝟙[e]; at photon edges hc/λ in Regional M; graded ε/δ_ε. Where no carrier is declared, energy is not evaluated. **Proposed replacement (ENERGY_V3, E1):** held E[v] = M_c(v); exchanged ε[e] = \|A(c)[e]\| (carrier c declared in M; 𝟙 moves to V3b "connected exchange" ε·𝟙_δ); graded ε/δ_ε with δ_ε[e] = δ_c[s] + δ_c[t] at the coarsest layer, or for a mapped carrier f the larger half-width of f over [c ± δ_c] (E4, closes OC-V-3) | FD form Declared (KR §13.3); replacement Proposed *(conflict, Part 6)* |
| **V3a** | Energy declaration fields | Eight M fields and a carrier register in Regional M | Proposed (E2) *(merged into FD 2026-11-16)* |
| **V3b** | Energy variables | B(A(c)) carried exchange; (R − B)(A(c)) energy circulation; ε·𝟙_δ connected exchange; F_δ over components of A(c); A_p(ε); Φ; ΔΚ/ΔQ and the other ratios. Energy circulation is a route to an observable correspondence for ρ (OC-K-1) | Proposed (E3) *(merged into FD 2026-11-16)* |
| **V4** | **Circulation** | Degree read from (R − B)[e]; its resolved sign is connection. Magnitude reported only once ρ_base, χ₀, max-selection and saturating form are declared. Reported on two sides: system side, and observer side (forward evolution at the observer's pace, from registered observer steps) | Proposed; observer measure Open (OC-V-4) |
| **V5** | **Rate of relational evolution** (candidate) | Γ_W[S] = T_W / τ[S]; Λ_W = Γ_W / T_W; Λ̂_W = Γ_W / σ_W, 0 ≤ Γ_W ≤ σ_W. τ from ħ/ε where ε is declared, else from steps with A_p resolved at δ_W | Proposed (RS-2a); OC-RE-1 |
| **V6** | **Κ, connected relations per locus** (candidate) | Κ_ℓ counted independently of ι; read beside C_δ, F_δ and supplied energy ΔQ, conjoined only as ratios ΔΚ/ΔQ, ΔC_δ/ΔQ, ΔF_δ/ΔQ, ΔF_δ/ΔC_δ. No energy partition assumed | Proposed (RS-3a); name **"connection density"** to confirm (E7); criterion Open (OC-RS-4) |
| **V7** | **Φ, coherence density** | Φ[v] = ι[v] · τ[v]; Φ_stability = ι·τ_stability; collective Φ[S] = ι[S]·τ[S], ι[S] = Σ ι[vᵢ]; accumulation condition Φ[S] > Σ Φ[vᵢ] (KR §13.10). Φ_relational = ħ/κ[region] goes to Regional M (name E8) | Name Declared (Origin, 2026-11-14); Φ_stability Observed (29 loci); *(merged into FD 2026-11-16)* |

---

# Part 3. Traceability

| ID | Statement |
| --- | --- |
| **B1** | Forward chain: observable → registration (descriptor) → M → value in D → declared relation → operator (function, commit) → graded form → declared C → reported number. Each link examined or reported as unexamined |
| **B2** | Reverse check: from a reported number and its identity, an executable path reselects registrations, rebuilds fields, re-executes operators and returns MATCH or MISMATCH. "Can it be executed?", not "can it be narrated?" (looking back: Df2a) |
| **B3** | Example (application): hydrogen 2S–6P, G1A, Ch0, 0–6 MHz: 𝟙_δ holds on 1,576 of 2,295 window-edges (68.7%); the reverse check returned MATCH for this and 297 other numbers |
| **Part IX** | Any document or code using V10 identifies by ID how it carries R1–R10, T1–T8, K1–K7, V1–V7. Undeclared correspondence is stated as *not evaluated*. The Relational Math Check (RMC) runs before every Verifier pass. RMC-7, RMC-8, RMC-9 proposed as new checks (`RELATIONAL_SYSTEM_DECLARATION.md`) *(merged into FD 2026-11-16)* |

---

# Part 4. Kernel code scope (`abr-kernel`)

**Applied so far:** `abr-kernel` branch `claude/tender-galileo-gcquso`, commit `757262a` (on `833ffb3`): direction of evolution vs direction of analysis, with the structure figure. Doc and comments only, two new tests, 128 tests pass. **Not merged to `main`.** (The Nov 16 handoff had named `claude/eloquent-allen-v4te7h` for the V10 code cycle; one branch is to be chosen.)

| Item | V9 (`833ffb3`) | V10 | State |
| --- | --- | --- | --- |
| `operator_b`, `operator_r`, `operator_sigma`, `antisymmetric_term`, persistence forms | Consecutive | Unchanged in form; comments cite Df3 | To do |
| Execution wrappers (`diagnostic_exec.rs`) | One field, one prior | One pass over the whole intersected field, no splitting; each execution states its cycle of analysis (R10) and its prior | To do |
| `DeclaredRelations` | out, inc, adj_plus, adj_minus | Acyclicity check (refuse if violated); route enumeration for K7 | To do |
| **New `operator_route_closure` (K7)** | — | Routes between loci joined by two or more routes; B_π, A_π per pair with route order recorded; node-derived inputs flagged | To do |
| `compute_rho*`, `derived_invariants.rs` readers | — | Unchanged | — |
| `admission.rs` adjacency use | — | Review for one-pass, no-split execution | To do |
| Tests | Consecutive | Kept; add K7 tests and a no-split test | Direction tests added (`757262a`) |
| Kernel text: direction of evolution vs analysis | One direction only | Paragraph, figure, comments at pred(e), Σ_pred, A_persistence; B wording | **Done (`757262a`)** |
| **Layer boundary** (`V10_BOUNDARY_AUDIT.md`) | Mixed | Classify code and docs as K (kernel), RM (regional M), A (application), P (process). New docs: `foundation_chain.md`, `variable_register.md`, `layer_boundary.md`, `v10_specification.md`; `docs/regional_m/` (4 files); `docs/applications/` (3 files). Logic unchanged; comments in scope. Batch 1 patch: `sessions/2026-11-14/v10-boundary_batch1.patch` | Boundary endorsed in substance (Origin, 2026-11-14); layout approval pending |
| **Five inconsistencies** before the rewrite | — | B as vector (VR D2); θ lone-pair inputs (G7); χ component-pair term (C2); value classes (R7); symbols (§8) | Open |
| **W7 rename list ("batch 2")** | "Coherence Potential" in `kernel_reference.md`, `cross_region_energy_expression.md`, `observable_variable_sets.md`, `validation_record.md`, `derived_invariants.rs` comments | → "Coherence Density" (Φ) and "energy held in connection" | Proposed (E6) |

---

# Part 5. What stands and what is withdrawn

**Stands under V10 (computed with the consecutive form; no re-run needed for the operator form):**
- Hydrogen 2S–6P traced chain: 298 MATCH (2026-11-14), and the B3 example.
- Trap ratio closure: ⟦d : p⟧ closes on three paths and two laboratories at ~10⁻¹¹; the H₂⁺ binding term is load-bearing. α against ¹²C⁶⁺ differs between MPIK and UW by 6.5 combined standard uncertainties (TC-e).
- Isotope-shift ladder rung 1: 99.997% of the H–D 1S–2S shift; residual −18.95 MHz.
- Trap network E vs N; ⁴He diamond consecutive signs (+ on f1–f3, − on f4–f5).
- Three-variable re-read (2026-11-16): superposition resolved between {2S→1S} and {2S→n} on the H/D field at 239–517 δ, across chains (candidate invariant; mapping order and cycles untested); first tunneling reading (⁴He diamond); connection not formable on the ³He and t relations (scope, not direction).

**Read under R9a (notes added 2026-11-16, after the fact; numbers unchanged).** Every analysis so far ran before R9a and had no domain declaration. Each findings record now opens with a note stating its domain and the observer's participation:
- **prepared and probed:** hydrogen 2S–6P, trap closures, trap network, H/D level network;
- **mixed domains:** the ⁴He diamond (retroactive domain declaration: `sessions/2026-11-16/HE4_DIAMOND_DOMAIN_DECLARATION_RETRO.md`), the isotope ladder (atomic and trap domains joined by the rung-1 mapping), and the n → d step;
- **no observing relation:** the earlier chemistry tests (synthetic or provider data), where R9a cannot be met.

The trap readings (one component, observer-built connections) and the ⁴He readings (a composition, not a formation process) are statements about prepared domains. The candidate H/D invariant has held across three preparations but not yet across participation (W10). Its domain is drafted for Origin in `sessions/2026-11-16/HD_LEVEL_NETWORK_DOMAIN_DECLARATION.md`, to be declared before IV-4.

**Withdrawn or lapsed:**
- Whole-range B and R (1st amendment of 2026-11-16), with OC-K-2, EN-g's second form, EN-F4, HD-F1, HD-F2, HD-F5, HV-a, HV-d. Their readings are kept as labelled "whole-range (withdrawn)".
- G4b ("registered through successor").
- "Coherence potential".
- Kernel amendment of 2026-11-12, item 1 ("H not a locus", amended by `cc9ba31`), item 5 (seeded ensembles → observed), item 6 (connection is R over B(A(x))).
- Traceability amendments A3 and A5 (withdrawn in favour of P1.2, Generator).

---

# Part 6. Conflicts to resolve before adoption

| # | Conflict | Where | Proposed resolution |
| --- | --- | --- | --- |
| 1 | **Energy form** (both forms now in FD V3, the replacement marked Proposed). FD V3: ε = \|A(ι)\|·𝟙. ENERGY_V3: ε = \|A(c)\|, with 𝟙 moved to "connected exchange". RS-3a: "kept unchanged". ENERGY_V3 finding F1: ε comes out in kg, not J, under KR §13.3 | FD V3; `ENERGY_V3_DRAFT.md`; RS-3a | Origin to decide E1–E4 |
| 2 | **Naming of Κ and Φ.** RS-3a and older text call Κ "coherence density"; the Nov 14 decision gives it to Φ. FD now has V7 Φ and marks Κ → "connection density" to confirm | FD V6; Nov 14 handoff; `ENERGY_V3_DRAFT.md` | Confirm E7 |
| 3 | ~~Edge support~~ | FD T3; X-a | **Resolved 2026-11-16:** merged into FD T3. VR B5 to be updated with the register |
| 4 | ~~T7 restatement, G4, G4a~~ | X-a, X-b | **Resolved 2026-11-16:** merged into FD |
| 5 | ~~The three variables not named in FD~~ | FD G1, V1–V3 | **Resolved 2026-11-16:** FD G1 now states the kernel expression (I-T, I-E, I-S) and the dissolution of OC-T-3, OC-S-3, OC-E-3 |
| 6 | **Direction rules.** EN-a (faster → slower, Decided) and HD-a (target → product, Generator's) are now both in FD T2 and may give different rules for related edges | Nov 16 declarations | Origin to confirm one rule, traced to an observable |
| 7 | **ID collisions.** VR G1–G12, E1–E4, D1–D8 collide with FD G1–G4, ENERGY_V3 E1–E8, delay loci D1–D16 and data labels D15–D21 | VR | Renumber VR with a prefix (e.g. VR-A1) |
| 8 | **Kernel branch.** V10 code cycle named on `claude/eloquent-allen-v4te7h` (Nov 16 handoff); direction patch is on `claude/tender-galileo-gcquso` | handoff; kernel patch | Choose one branch |
| 9 | **χ component-pair term.** Code includes \|comp[p][i]\| in χ; the kernel text does not say so | VR C2 | Stated in FD K3 (2026-11-16); kernel text still to state it (V10 code cycle) |

---

# Part 7. Open decisions awaiting Origin

**Kernel (affects V10 text or code):**
- **OC-K-3a** K7 route order (proposed: order of analysis, stated per closure).
- **OC-D-1** whether a backward reading's direction is recorded per output or per run.
- **OC-K-1** observable correspondence of ρ, ρ_base, χ₀, cc; saturating form.
- **A-3** empty-sum semantics.
- **OC-V-1** C_δ form (also RMC 5.2). **OC-V-2** F_δ beyond two components. **OC-V-3** δ_ε (E4). **OC-V-4** observer circulation measure.
- **OC-Z-1** "extent" in σ; rename σ. **OC-F-1** H as a shared frame.
- **OC-RS-1 to 5**, **OC-RE-1**, **OC-E-1**, **OC-OBS-3**, **OC-RES-1 to 4**, **OC-RD-1, 2**, **OC-T-1**.
- **E1–E8** energy V3, V3a, V3b, δ_ε, F1 re-read, W7 renames, Κ name, Φ_relational name.
- **P-a to P-f** relational inertia P1.1–P1.4, the OVS inference, ratio-registration direction (answered by Tr-d, open).
- **Tr-a to Tr-f** trap observing chain in Regional M; five-class split; reference locus; two kinds of inertia registration; invariance theorem as a mapping.
- **IV-1** H and D as components or as loci with isotope edges. **IV-2** d(e,f) as the graded freedom form. **IV-3** 𝟙 = 0 by topology reported as decoherence or "connection not formable".
- **DD-1** whether a composition across domains with different participation is one field, or a declared comparison between domains.
- **W9** declare the system / chain / mapping classification. **W10** the relational-invariant definition.
- **Boundary audit** layout approval (next steps 1–5); the five inconsistencies.
- **Symbols** (§8); RS open list (rate variable, RS-3a, Κ criterion, RD fields and symbol, participation kinds, RS-1–6 approval, RS-4 for Σ); RMC 5.6, 5.7.

**Application and data (do not change the kernel):**
- Complex-system area: water rovibrational network or AME input network (atomic-region alternative: H/D/He line networks).
- **DH-1 to DH-5** the H/D level network's domain declaration (energy, connection, freedom; the 2S identification as a mapping; components or isotope edges; rung-1 trap inputs as a cross-domain mapping; a received-only domain as the W10 test). To be declared before IV-4.
- **IV-4** ladder rungs 2–4 per edge on the H/D field (the test of the candidate invariant). **IV-5** CODATA correlations. **IV-6** registered relation at ³He⁺. **IV-7** second cycle.
- **FL-T1 to T6** ladder rungs 2–4. **IL-a to d**. **TC-a to g** (TC-d, TC-g: trap cycle-data request). **HD-F3, HD-F4** (capture-photon and Q-value registrations; IAEA hosts). **EN-F1, EN-F2** (to record).
- **ED-a to g** electron–deuteron (ED-a isotope edge; ED-b bound electron as observing relation, for OC-V-4).
- **FL-a to k**, **FG-a to k** flare fields. **PC-a to d** primal comparison (PC: replace the non-peer-reviewed MS source).

---

# Part 8. Symbols and identifiers to settle

- One meaning each for **E** (operator, E[v], E_δ, E_prior), **D** (domain, value class, delay loci), **σ** (ratio, deviation), **C** (projection, C_δ, component pairs), **R** (operator, value class, runs), **ρ** (strength, Spearman), **Σ** (operator, sum).
- Registration descriptor symbol ⟦a : b⟧ (R3) to be chosen.
- **Domain and D** are related, not colliding: *domain* is the full scope of the relational system; *D* is its mathematical establishment (R6, R9a). "D" as a value-class letter and as delay-locus labels still collides.
- Value-class letters R, R+, P, W, D to be renamed (R7).
- Variable register IDs to be prefixed (§6, conflict 7).

---

# Part 9. Amendment log

| Date | Decision | Effect |
| --- | --- | --- |
| 2026-11-12 | Ground of relational science; G1 expressed as I-T, I-E, I-S; OC-T-3, OC-S-3, OC-E-3 dissolved; reference frame and resolution | H, G1, G1-K, Z1–Z3, R7, W2 |
| 2026-11-13 | Relational system declaration RS-1 to RS-6, RD | Df1–Df4, R3–R5, V4–V6 |
| 2026-11-14 | Foundation Declaration drafted; O1, Df5, R1, T5 declared; ρ max-selection; statistics as declared C; "potential" rule; Φ named coherence density; V10 layer boundary endorsed | Parts I–XII |
| 2026-11-15 | G4, G4a; T3 support amended; T7 restated; G4b withdrawn; mass → relational inertia; electron as locus in ion; rule of inclusion | G4, G4a, T3, T7, W11 |
| 2026-11-16 (1st) | Whole-range operators | **Withdrawn** by the 3rd |
| 2026-11-16 (2nd) | Cycles of analysis and the ongoing field | R10; OC-K-4 closed |
| 2026-11-16 (3rd) | Encompass consecutive observations; do not blur | Df3 operator clause; consecutive K2, K4 |
| 2026-11-16 | EN-a faster → slower; EN-a to EN-g approved; K7 adopted | EN-a, K7 |
| 2026-11-16 (4th) | One direction of evolution; any direction of analysis; structure figure | Df2, Df2a, T2, K4, K5, W8; kernel `757262a` |
| 2026-11-17 | HB-1: cycle closure, the signed sum around any cycle, adopted as K7's general form | K7 |
| 2026-11-16 | W12: the reference frame of comparison is abstract in the standard frame; results stated as observed within D; legacy mathematics as declared mappings | W12 |
| 2026-11-16 (5th) | Domain of the relational system, established through energy, connection and freedom; participation in the domain (intervention bounds degree, never presence or direction) | R9a; R6, Df4, R5, G4, W11, Part XI |
| 2026-11-16 | Synchronization (Origin: "merge and synchronize"): decided and proposed items from other documents merged into the FD; this README aligned | FD G1, G4, G4a, R3, T2, T3, T7, K3, V1–V3b, P1, V6, V7, W7–W11, Parts IX–XII |

---

# Part 10. Sources

| Source | Content |
| --- | --- |
| `sessions/2026-11-14/FOUNDATION_DECLARATION_DRAFT.md` | Spine (Parts I–XII, four amendments) |
| `sessions/2026-11-14/VARIABLE_REGISTER_DRAFT.md` | Variable register |
| `sessions/2026-11-14/V10_BOUNDARY_AUDIT.md`, `v10-boundary_batch1.patch` | Layer boundary and layout |
| `sessions/2026-11-14/RELATIONAL_MATH_CHECK_REPORT.md` | RMC findings, §5 items |
| `sessions/2026-11-12/KERNEL_AMENDMENT_reference_frame_resolution.md` | Ground; I-T, I-E, I-S; resolution |
| `sessions/2026-11-13/RELATIONAL_SYSTEM_DECLARATION.md` | RS-1 to RS-6, RD, RMC-7 to 9 |
| `sessions/2026-11-15/SYSTEM_EXPANSION_DRAFT.md` | G4, G4a, T3, T7, X-a to X-d |
| `sessions/2026-11-15/ENERGY_V3_DRAFT.md` | V3, V3a, V3b, V6, V7, W7, E1–E8 |
| `sessions/2026-11-15/RELATIONAL_PHYSICS_DRAFT.md` | Relational inertia P1, P-a to P-f |
| `sessions/2026-11-15/TRAP_RELATIONAL_ANALYSIS_DRAFT.md`, `PRIMAL_COMPARISON_DRAFT.md`, `ELECTRON_DEUTERON_EXPANSION_DRAFT.md` | Tr, PC, ED items |
| `sessions/2026-11-16/TRAP_NETWORK_E_VS_N_DECLARATION.md` and findings; `HE4_DIAMOND_*`; `HYDROGEN_V10_RERUN_FINDINGS.md`; `THREE_VARIABLES_REREAD_*` | EN, HD, HV, IV items and results |
| `sessions/2026-11-16/kernel_patch/` | Kernel text patch (applied at `757262a`) |
| `abr-kernel` `833ffb3`: `docs/kernel/operators_notation_and_constraint.md`, `src/operators.rs`, `src/derived_invariants.rs` | V9 baseline |

On adoption, merged sources point here or to the FD for their content. Their history remains in their commits.
