# Kernel V10 Migration: Redoing Repositories in V10

**Metatron Dynamics, Inc.** Kernel V10. Bounded over D. No claim beyond D.

This guide is for redoing an application repository built on Kernel V9 (or
earlier) so that it uses only Kernel V10. It restates `V10_README.md`; where
they differ, `V10_README.md` governs.

---

## 1. The V10 admissibility rule (W13)

> V10 work uses only V10 forms. A retired or superseded element enters a V10
> result only as a declared historical comparison (C), never as an input.

The rule is enforced in code by the **V9 fence**. Retired and superseded V9
elements compile only with the Cargo feature `v9`, which is off by default. A
V10 repository that calls one of them does not compile.

```toml
# A V10 repository: no features. Fenced V9 elements are unavailable.
abr-kernel = { git = "https://github.com/Relational-Relativity-Corporation/abr-kernel", branch = "main" }

# A V9 repository not yet redone: opt in visibly.
abr-kernel = { git = "https://github.com/Relational-Relativity-Corporation/abr-kernel", branch = "main", features = ["v9"] }
```

The `features = ["v9"]` line is the repository's declaration that it still
depends on V9 forms. Removing it is the test that a repository has been fully
redone: it must still compile.

---

## 2. What maps to what

| V9 element (fenced) | V10 replacement | Note |
|---|---|---|
| `operator_e`, `operator_e_v5`, Phase 1 / Phase 2 | `relational_field::RelationalField::new` | At least two consecutive registrations (Df6). No operator parameters |
| a single observation M(o) | insufficient information | `DeclarationGap::NoProcessSequence`. Declare a sequence |
| `operator_r`, R − B, `antisymmetric_term`, `operator_sigma` | `operators::operator_circulation` (Ξ) | Ξ(g)[e] = Σ_succ g − Σ_pred g, parameter-free; same sign as V9 R − B |
| `compute_rho`, `compute_rho_primary`, `compute_rho_persistence`, ρ_base, χ₀, cc[p] | none | Retired (K3). Nothing replaces them; V10 reads energy's effect from the data |
| `operator_r_persistence`, ρ-weighted persistence | A_p, B_p, Ξ_p in `RelationalField` | Exactly the differences of A, B, Ξ between registrations |
| `epsilon_e` (ε = \|Δι\|·𝟙) | `ground_variables::energy`, `energy_exchange` | ε = \|A(c)\| of a declared carrier c; connection is not a factor |
| `tunneling_floor_holds` (ε > tol) | `EnergyExchange::floor_resolved` | ε > δ_ε, with δ_ε = δ_c[s] + δ_c[t] |
| `superposition_condition`, `are_proportional` | `GroundReading::freedom` (`persistence_change`) | Freedom is change permitted, PC1–PC3 (2026-11-21, 7th). The first V10 form, `ground_variables::freedom` (minor held resolved), reads a persistent arrangement and is kept as `v2_held_arrangement` for comparison |
| `superposition_resolved` (interior present) | `ground_variables::has_interior` | Freedom no longer needs an interior |
| `detection_indicator` (𝟙[e]), `entanglement_condition`, `decoherence_condition` | `ground_variables::connection` | Graded identity / proportional / sustained; a neighbourless edge is *not formable*, never decoherent |
| `diagnostic_exec` | `RelationalField` with `GroundReading::findings` | Findings about the declaration replace warnings and errors |
| `admission`, `provenanced_exec` (RPPI over ρ-weighted Σ) | **Open** | The RPPI principle carries into V10; admission for V10 forms is still to be built |

Unchanged and available in V10: `operator_a`, `operator_delta`, `operator_b`,
`operator_a_persistence`, `operator_b_persistence`, `DeclaredRelations`,
`NodeField`, `EdgeField`, the photon edge ε function, Φ, J and the other
Part I invariants, and `provenance.rs`. *(τ = ħ/ε and its functions were retired
from V10 on 2026-11-21, 9th, and are behind the V9 fence: time is not a
variable.)*

---

## 3. Steps for one repository

1. **Record the V9 result.** Tag or copy the repository's V9 outputs before
   changing anything. They are the comparison baseline.
2. **Trace the provenance chain** (`V10_README.md` §2.0) link by link. Where a
   link cannot be completed from the record, stop and report the finding.
   Where the record registers relations (an exchange between two states, a
   transition), enter them as edge values (ER-1); construct no node values.
3. **Declare the domain (R9a).** For the system analysed, state what carries
   energy (the carrier, its unit, mapping and δ_c), which relations carry
   connection, and which components vary independently (freedom). State the
   observer's participation.
4. **Declare the sequence.** At least two consecutive registrations of the same
   relations, ordered by an observable property of the process, each with δ.
   A single snapshot is insufficient information.
5. **Remove `features = ["v9"]`** and replace each fenced call using the table
   above. The compiler lists every remaining one.
6. **Form the relational field** with `RelationalField::new` and read
   `ground` (energy, freedom, connection). `examples/v10_relational_field.rs`
   is a template.
7. **Report findings, not absences.** Every entry of `GroundReading::findings`
   is a statement about the declaration (Primary Region, or domain not fully
   declared). Return to the declaration; never reverse an edge or substitute a
   value.
8. **Compare and record.** Place the V10 result beside the V9 result as a
   declared comparison (C): what changed and why. Never overwrite the V9
   result; the transition is part of the repository's provenance.

---

## 4. What not to carry over

- No ρ, χ, ρ_base, χ₀ or cc[p] values, including the general values 0.5 and 1.
- No single-snapshot readings.
- No reading of a zero, or an unresolved value, as absence (W2).
- No V9 indicator (𝟙[e]) as a connection reading.
- No time computed from energy (τ = ħ/ε, N[v], Φ_relational, κ[region]).
- No V9 numbers as V10 inputs. They enter only as the declared comparison in
  step 7.

---

*Metatron Dynamics, Inc. Kernel V10.*
