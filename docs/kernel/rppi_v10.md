# RPPI-V10 — Admission for the V10 forms

**Metatron Dynamics, Inc.** Kernel V10.3. Bounded over D. No claim beyond D.

**Status: Proposed (Generator, 2026-11-22, Part 9, 30th), for Verifier review and Origin's decision.** This document specifies admission for the V10 forms as a kernel-level statement (audit item RPPI-V10, `OPEN_CONDITIONS_AUDIT_2026-11-22.md` §E). It changes no declaration. The V9 admission interface (`v9_specification.md`, `admission.rs`, `provenanced_exec.rs`) stays behind the `v9` fence as the record.

## 1. The invariant, unchanged

The Relational Primitive Provenance Invariant (`v9_specification.md` §2, Req-1 and Req-7) holds in V10 as written:

> Every admitted mathematical variable, quantity, operation, and result must have a continuous, verifiable provenance path to an Origin-declared observable relation within D. No mathematical result can be admitted solely on numerical correctness, structural validity, or named provenance.

V10 changes how the path is laid out, not the requirement. In V10 the path **is** the provenance chain (`V10_README.md` §2.0, links 1 to 11). Admission for V10 has three parts:

1. every link of the chain is completed from the record, or the analysis stops there (§2);
2. what cannot be completed is refused, never repaired (§3);
3. every reported number can be executed back to its registrations: the reverse check, B2 (§4).

## 2. Admission is the §2.0 chain

Each link is admitted where its inputs enter. "Who holds it" says whether the kernel can check the link or only the application can, since the kernel receives values and declared relations, not instruments or files.

| # | Link | Admitted when | Refusal | Code | Who holds it |
|---|---|---|---|---|---|
| 1 | Observable → registration | The observing chain, window, reference, participation and δ are declared | — | — | Application (record declaration) |
| 2 | Registration → value in D | M is declared before any operator acts; not evaluated ≠ 0.0 | — | — | Application (M, units, certificates) |
| 3 | The relation | Every edge has a class (`EdgeProvenance`), endpoints within the declared loci, and is not a persistence edge; **an identity and a direction basis (G-1, adopted 31st)**. T8: no self-relation; every directed cycle carries a feedback edge with a stated basis | Construction refusal (endpoints, class); finding `DeclarationGap::T8` | `DeclaredRelations`, `topology::check_t8` | Kernel checks form, identity and T8 *(was: identity and direction basis held by the application, gap G-1; closed 31st)*. Whether the basis is true to the record: application |
| 4 | The value on the relation | Node values finite; δ finite and ≥ 0 for every component and locus; Z1a: run-constant offsets finite and ≥ 0, gains in [0, 1), the same in every registration of the run. Edge-registered values likewise per edge | Construction refusal | `Registration::new`, `with_run_constant`, `EdgeSeries::from_edge_values`, `with_run_constant` | Kernel |
| 5 | The sequence | At least two consecutive registrations of the same components and loci | Finding `NoProcessSequence`; construction refusal on mismatched shape | `check_sequence`, `EdgeSeries::from_registrations`, `RelationalField::new` | Kernel checks count and shape. **Order and consecutiveness: application** (the record's order is an observable property of the process) |
| 6–7 | Change; structure and freedom | Formed only from admitted links 3–5 | Findings: `SingleComponent`, `FewerThanTwoEdges` | `persistence_change` | Kernel |
| 8–10 | Superposition, entanglement, tunneling | As 6–7; the sustained grade needs an interior (T7); energy needs a declared carrier | Findings: `PrimaryRegion`, `NoNeighbourInField`, `NoResolvedPair`, `NoCarrierDeclared` | `ground_variables`, `relational_field` | Kernel |
| 11 | Reported number | A share or degree is a declared projection C, declared before use, with the reverse check | — | `reverse_check` (kernel share) | Kernel and application (gap G-2) |

**Two kinds of refusal**, both refusals and neither a repair:

- A **construction refusal** (an assertion) stops on a malformed declaration: a shape that does not match, a value not in D, a δ that is negative or not finite, a gain bound of 1 or more, run-constant parts that change within a run. No output is formed.
- A **finding** (`DeclarationGap`) is returned in place of a reading when the declaration is well formed but does not contain what the reading needs. It is a statement about the declaration, not about the system (Primary Region, or domain not fully declared), with the remedy it names (`DeclarationGap::remedy`).

## 3. Refusal, not repair

No admitted input is constructed, substituted or adjusted to obtain a reading.

- Nothing is filled in to complete a link (§2.0). A missing value is not evaluated, never 0.0 (R6).
- No edge is reversed to obtain a reading or break a cycle (W8); a cycle is admitted by declaring its feedback with a basis (T8), never by deleting or re-orienting an edge.
- No δ is narrowed to obtain resolution, and no declaration is changed to obtain presence (W5). A result that seems to contradict the ground returns to Origin as a statement about the declaration.
- An edge with one empty side is flagged record-boundary-conditioned, never read as zero circulation (A-3). An edge with neither side is not formable (IV-3).
- A pair the declaration fixes is listed as an identity, never as a reading (PC-D4).
- Refusals carry their remedy, and the remedy is always a further declaration, never a computation.

## 4. The reverse check (B2)

B2: from a reported number and its identity, an executable path reselects registrations, rebuilds fields, re-executes operators and returns MATCH or MISMATCH. "Can it be executed?", not "can it be narrated?".

**Identity of a reading.** Every V10 reading carries the indices that locate it in its inputs:

| Reading | Identity |
|---|---|
| PC1 (`RelationChange`) | step, component, edge |
| PC2 (`ChangeCoupling`) | edge pair (e < f); over components i < j at a step, or over steps s, s+1 in a component |
| PC3 (`ArrangementStep`) | step, edge pair, component pair |

The application maps these indices to the record's own identities: step → registration ids, component → chain, edge → edge id and direction basis.

**The three parts of the path:**

1. **Reselect (application).** From the record, by the reading's identity, reselect the registrations, the edge's loci and the δ behind it, with parsing of its own. A missing or ambiguous lookup is an error, never a choice.
2. **Rebuild and re-execute (kernel).** `reverse_check::reverse_relation`, `reverse_coupling` and `reverse_arrangement` take the declared inputs (`Inputs::Nodes` or `Inputs::Edges`) and the reported reading. They rebuild A from the inputs, re-derive the bounds under Z1a, re-execute the form and check that the reading was admissible to report: its edges entered and its pair is not a declared identity. They share no code with the forms that produced the reading.
3. **Compare.** `Verdict::Match`, or `Verdict::Mismatch` naming the first quantity that differs, with the reported and the re-derived value. The comparison is exact: the same declared inputs in the same order of arithmetic give the same number.

*Tests:* `tests/v10_reverse_check.rs`. Every PC1, PC2 and PC3 reading over 1500 random node-derived fields (half with run-constant δ parts) and 1500 edge-registered fields (half with offsets) returns MATCH. A changed value, a wider δ, a flipped reading, an out-of-range identity, a declared identity pair or an edge that never entered returns MISMATCH naming it.

*Example (application, `abr-relational-chemistry`):* the vessel ingest implements all three parts. It refuses a malformed record with a named problem per line (strict field counts, no commented lines, units, duplicates, the declared order direction, the resolved ordering step). It declares its fields as chain sets. Its `reverse_check_pc1_raw` re-reads one PC1 reading from the file text with its own parsing, sharing only the index mappings.

## 5. The three records of the RPPI, in V10

| RPPI record (`README.md`) | In V10 |
|---|---|
| **Provenance label** | `EdgeProvenance` (Continuation, Coupling). A class: describes, does not establish |
| **Declaration record** | Links 1–5: the observing chain, M, the relations with their direction basis, δ in two parts, the ordered sequence, and feedback declarations with their basis (T8). In V10, held by the application's record declaration; the kernel holds links 3–5 in form |
| **Calculation record** | The readings with their identities (§4). The independent reconstruction (IRRS) is the reverse check: reselection by the application, re-execution by the kernel's independent path |

## 6. Gaps, stated

- **G-1. Relation identity is not a kernel input.** `DeclaredRelations` carries an edge's class, not its identity or its direction basis; the V9 "missing provenance" gap (`tests/v9_conformance.rs`, RPPI-GAP) therefore persists for V10 inputs, and the identity lives in the application's record (the vessel's `edges.csv` carries an id and a direction basis per edge). *Decision for Origin:* (a) carry an identity and basis per edge in `DeclaredRelations`, refused when empty, as T8 already does for feedback; or (b) keep identity in the application and state it as the application's admission duty. **Generator's recommendation: (a)**, so the kernel can refuse a relation with no stated basis, as it refuses a feedback edge with none. **[Adopted (a) (Origin, 2026-11-22, 31st), implemented in 10.3.0: `DeclaredRelations::with_identity` declares a `topology::RelationIdentity` (id, direction basis) per edge; `topology::check_identity` refuses no identity, a count that does not match, an empty id or basis, and a repeated id (parallel edges are distinct relations); every V10 reading entry point checks identity, then T8, through `topology::admit_field`, refusing with `DeclarationGap::RelationIdentity`. Kernel fixtures declare synthetic identities through a test-only helper marked SYNTHETIC. G-1 is closed for V10 inputs; the V9 RPPI-GAP tests document the frozen V9 constructors and are unchanged.]**
- **G-2. Declared projections (link 11) have no code.** A share or degree reported as a declared projection C is declared in prose; no type records what it keeps and drops.
- **G-3. Reverse-check coverage.** The kernel share covers PC1, PC2 and PC3. The connection grades in state (identity, proportional, sustained), the energy exchange and cycle closure are not yet covered.
- **G-4. Links 1, 2 and the order of link 5 are application duties.** The kernel cannot observe an instrument, an M or a process order; it receives their results. Each application's admission states them.
- **G-5. Synthetic tests only.** Every test establishes that the forms and the check compute as stated. No registered record has yet been admitted through the full chain (OC-SC-1).

*Metatron Dynamics, Inc. Kernel V10.3.*
