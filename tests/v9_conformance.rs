//! Kernel V9 Conformance Test Suite — Third Milestone
//!
//! Organization follows the V9 development contract (v9_specification.md):
//!
//!   §1  V8 preservation — baseline operator behavior is frozen
//!   §2  Executable relation — what is enforced at construction
//!   §3  Provenance survival — the current gap, documented as [GAP]
//!   §4  Admissibility enforcement — conditions currently enforced
//!   §5  Open condition preservation — no cold-start substitute
//!   §6  V8 operator continuity — full chains produce correct finite output
//!   §7  IRRS integration — tracing gap documented as [GAP]
//!   §8  Relational Primitive Provenance Invariant — four failure modes
//!
//! Tags:
//!   [V8-BASELINE]  — confirms behavior that must not change in any V9 milestone.
//!                    A V9 code change that fails a V8-BASELINE test violates the
//!                    V8 preservation contract.
//!   [GAP]          — documents a gap that V9 closes. The test passes today;
//!                    it captures the current state so the gap is explicit,
//!                    not silent.
//!   [RPPI-GAP]     — documents a gap relative to the Relational Primitive Provenance
//!                    Invariant (v9_specification.md §2). The current behavior is
//!                    inadmissible under RPPI; the test records it explicitly so the
//!                    gap is named, not silent. All [RPPI-GAP] tests must pass.
//!   [V9-REQUIRED]  — V9 enforcement through the admission interface
//!                    (src/admission.rs). Each test constructs a declared context,
//!                    executes or reconstructs the canonical calculation, invokes
//!                    admission, and asserts the exact OPEN or REJECTED outcome and
//!                    its evidence. Bounded scope: see src/admission.rs.
//!
//! Test counts:
//!   27 existing tests ([V8-BASELINE] + [GAP]) — all must pass.
//!   4 [RPPI-GAP] tests — document that the frozen V8 constructors still accept
//!     class-only provenance (they are unchanged, Req-5); must pass.
//!   4 [V9-REQUIRED] tests — executable since the V9 admission interface
//!     (src/admission.rs): each constructs one invalid case and asserts the exact
//!     OPEN or REJECTED outcome and its evidence. None is ignored and none
//!     has a placeholder body.
//!     Renamed to state their outcomes (Origin correction 2026-10-25: absence
//!     is OPEN; only an established contradiction is REJECTED).
//!
//! All [V8-BASELINE] and [RPPI-GAP] tests must pass on the `kernel-v9` branch
//! before any V9 implementation milestone is submitted for Verifier review.

mod common;

use abr_kernel::admission::{admit, AdmissionOutcome, BasisOf, DeclKind, Link, OpenCondition, RppiViolation};
use abr_kernel::derived_invariants::epsilon_photon_edge;
use abr_kernel::provenance::CalculatedOutput;
use abr_kernel::provenanced_exec::{execute_edge_m_projection, execute_primary};
use common::*;

use abr_kernel::operators::{
    compute_rho, compute_rho_primary, declared_edge_image_admissibility_check,
    operator_a, operator_delta, operator_e, operator_e_primary,
    relational_isolation, require_rho_declaration, DeclaredRelations,
    EdgeProvenance, NodeField, PersistenceState,
};

// ── Fixtures ──────────────────────────────────────────────────────────────────

/// Single edge (0→1). Minimal declared relation.
fn rel_single_edge() -> DeclaredRelations {
    DeclaredRelations::from_edges_with_provenance(
        2,
        vec![(0, 1)],
        vec![EdgeProvenance::Continuation],
    )
}

/// Two-edge chain (0→1→2). Three nodes, two continuation edges.
fn rel_three_node_chain() -> DeclaredRelations {
    DeclaredRelations::from_edges_with_provenance(
        3,
        vec![(0, 1), (1, 2)],
        vec![EdgeProvenance::Continuation, EdgeProvenance::Continuation],
    )
}

/// Two edges, mixed provenance: Continuation then Coupling.
fn rel_mixed_provenance() -> DeclaredRelations {
    DeclaredRelations::from_edges_with_provenance(
        3,
        vec![(0, 1), (1, 2)],
        vec![EdgeProvenance::Continuation, EdgeProvenance::Coupling],
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// §1. V8 PRESERVATION
// ═══════════════════════════════════════════════════════════════════════════

/// [V8-BASELINE] Δ(x)[e] = x[source] − x[target]. Exact arithmetic.
#[test]
fn v8_delta_directed_difference() {
    let rel = rel_single_edge(); // edge (0,1)
    let x = NodeField::new(vec![vec![3.0, 1.0]]);
    let delta = operator_delta(&x, &rel);
    assert!(
        (delta.field[0][0] - 2.0).abs() < 1e-12,
        "Δ[(0,1)] = x[0] - x[1] = 3.0 - 1.0 = 2.0"
    );
}

/// [V8-BASELINE] Direction is a property of the declared relation, not a convention.
/// Declaring (0,1) and (1,0) on the same field produces exact negations.
#[test]
fn v8_delta_direction_is_structural() {
    let rel_fwd = DeclaredRelations::from_edges_with_provenance(
        2,
        vec![(0, 1)],
        vec![EdgeProvenance::Continuation],
    );
    let rel_rev = DeclaredRelations::from_edges_with_provenance(
        2,
        vec![(1, 0)],
        vec![EdgeProvenance::Continuation],
    );
    let x = NodeField::new(vec![vec![3.0, 1.0]]);
    let d_fwd = operator_delta(&x, &rel_fwd);
    let d_rev = operator_delta(&x, &rel_rev);
    assert!(
        (d_fwd.field[0][0] + d_rev.field[0][0]).abs() < 1e-12,
        "forward and reverse declarations produce exact negations: {} + {} ≠ 0",
        d_fwd.field[0][0],
        d_rev.field[0][0]
    );
}

/// [V8-BASELINE] For an isolated edge (no adjacency), Σ(Δ(x))[e] = Δ(x)[e].
/// The antisymmetric accumulation term is zero when adj⁺ and adj⁻ are empty.
#[test]
fn v8_sigma_equals_delta_for_isolated_edge() {
    let rel = rel_single_edge();
    let x = NodeField::new(vec![vec![1.0, 0.0]]);
    let (delta, sigma) = operator_e_primary(&x, &rel, 0.5, 1.0);
    assert!(
        (sigma.field[0][0] - delta.field[0][0]).abs() < 1e-12,
        "Σ must equal Δ for an isolated edge: sigma={} delta={}",
        sigma.field[0][0],
        delta.field[0][0]
    );
}

/// [V8-BASELINE] A single isolated edge has empty adj⁺ and adj⁻ — relational isolation.
#[test]
fn v8_relational_isolation_single_edge() {
    let rel = rel_single_edge();
    assert!(
        relational_isolation(&rel),
        "a single edge with no adjacency must satisfy relational isolation"
    );
}

/// [V8-BASELINE] A chain of two edges is not relationally isolated — edge 0's adj⁺
/// contains edge 1.
#[test]
fn v8_relational_isolation_false_for_chain() {
    let rel = rel_three_node_chain();
    assert!(
        !relational_isolation(&rel),
        "a two-edge chain must not satisfy relational isolation"
    );
}

/// [V8-BASELINE] Δ output for a two-edge chain: [x[0]-x[1], x[1]-x[2]].
#[test]
fn v8_delta_two_edge_chain() {
    let rel = rel_three_node_chain();
    let x = NodeField::new(vec![vec![4.0, 1.0, 3.0]]);
    let delta = operator_delta(&x, &rel);
    // edge (0,1): 4.0 - 1.0 = 3.0
    // edge (1,2): 1.0 - 3.0 = -2.0
    assert!((delta.field[0][0] - 3.0).abs() < 1e-12, "Δ[(0,1)] must be 3.0");
    assert!((delta.field[0][1] + 2.0).abs() < 1e-12, "Δ[(1,2)] must be -2.0");
}

/// [V8-BASELINE] declared_edge_image_admissibility_check: a chain with non-symmetric
/// differences is admissible.
#[test]
fn v8_edge_image_admissibility_asymmetric_chain() {
    let rel = rel_three_node_chain();
    let x = NodeField::new(vec![vec![4.0, 1.0, 3.0]]);
    let delta = operator_delta(&x, &rel);
    let (admissible, witness) = declared_edge_image_admissibility_check(&delta, 1e-9);
    assert!(
        admissible,
        "non-symmetric delta output must be admissible; witness: {:?}",
        witness
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// §2. EXECUTABLE RELATION
// ═══════════════════════════════════════════════════════════════════════════

/// [V8-BASELINE] Provenance count must match edge count.
/// from_edges_with_provenance panics if they differ.
#[test]
fn executable_relation_requires_provenance_per_edge() {
    let result = std::panic::catch_unwind(|| {
        DeclaredRelations::from_edges_with_provenance(
            2,
            vec![(0, 1)],
            vec![], // no provenance declared — must be rejected
        )
    });
    assert!(
        result.is_err(),
        "construction with mismatched provenance count must be rejected"
    );
}

/// [V8-BASELINE] All edge endpoints must be within the declared node range.
#[test]
fn executable_relation_rejects_out_of_range_node() {
    let result = std::panic::catch_unwind(|| {
        DeclaredRelations::from_edges_with_provenance(
            2,
            vec![(0, 5)], // node 5 not in [0, 2)
            vec![EdgeProvenance::Continuation],
        )
    });
    assert!(
        result.is_err(),
        "edge to undeclared node must be rejected"
    );
}

/// [V8-BASELINE] Persistence edges are not spatial — they have edge-valued loci.
/// Admitting them in the spatial list would fabricate a node identity for them.
#[test]
fn executable_relation_rejects_persistence_in_spatial_list() {
    let result = std::panic::catch_unwind(|| {
        DeclaredRelations::from_edges_with_provenance(
            2,
            vec![(0, 1)],
            vec![EdgeProvenance::Persistence], // must be rejected in spatial list
        )
    });
    assert!(
        result.is_err(),
        "Persistence provenance in spatial edge list must be rejected"
    );
}

/// [V8-BASELINE] Continuation and Coupling are admissible spatial provenances.
#[test]
fn executable_relation_admits_continuation_and_coupling() {
    let rel = rel_mixed_provenance();
    assert_eq!(rel.n_edges(), 2);
    assert_eq!(rel.provenance[0], EdgeProvenance::Continuation);
    assert_eq!(rel.provenance[1], EdgeProvenance::Coupling);
}

/// [V8-BASELINE] NodeField::new enforces finiteness — all projections through M must be finite.
#[test]
fn executable_relation_node_field_rejects_nan() {
    let result = std::panic::catch_unwind(|| {
        NodeField::new(vec![vec![1.0, f64::NAN]])
    });
    assert!(result.is_err(), "NaN must not enter D");
}

/// [V8-BASELINE] NodeField::new enforces finiteness — infinity is not in D.
#[test]
fn executable_relation_node_field_rejects_infinity() {
    let result = std::panic::catch_unwind(|| {
        NodeField::new(vec![vec![1.0, f64::INFINITY]])
    });
    assert!(result.is_err(), "infinite value must not enter D");
}

// ═══════════════════════════════════════════════════════════════════════════
// §3. PROVENANCE SURVIVAL
// ═══════════════════════════════════════════════════════════════════════════

/// [GAP] PrimaryEdgeField carries no provenance reference.
///
/// The verifier cannot trace a Δ output back to its EdgeProvenance from the
/// output type alone. Tracing requires separately consulting DeclaredRelations.
///
/// This test documents the gap. V9 closes it by adding ProvenancedOutput<T>
/// (v9_specification.md §3).
///
/// Current state: the test passes because it only asserts that the output value
/// is finite. The provenance trace is performed by consulting `rel` separately —
/// not from `delta` itself.
#[test]
fn provenance_survival_gap_primary_output_carries_no_provenance() {
    let rel = rel_three_node_chain();
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0]]);
    let delta = operator_delta(&x, &rel);

    // GAP: delta carries no reference to rel.provenance.
    // The verifier must hold (delta, rel) as a pair and consult rel separately.
    //
    // The line below traces edge 0's provenance — but it reads from rel, not delta.
    let edge_0_result = delta.field[0][0];
    let edge_0_provenance = &rel.provenance[0]; // must consult rel separately

    assert!(edge_0_result.is_finite());
    assert_eq!(*edge_0_provenance, EdgeProvenance::Continuation);
    // If delta carried provenance, this would be: delta.provenance[0]
    // That field does not exist — the gap is explicit here.
}

/// [GAP] EdgeField (A, B, R output) carries no provenance reference.
///
/// Same gap as PrimaryEdgeField, for the ABR kernel path.
#[test]
fn provenance_survival_gap_abr_output_carries_no_provenance() {
    let rel = rel_three_node_chain();
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0]]);
    let pairs: &[(usize, usize)] = &[];
    let cc: &[f64] = &[];
    let r = operator_e(&x, &rel, pairs, cc, 0.5, 1.0);

    // GAP: r.spatial carries no reference to rel.provenance.
    // The verifier must hold (r, rel) as a pair.
    let edge_0_spatial = r.spatial[0][0];
    let edge_0_provenance = &rel.provenance[0]; // must consult rel separately

    assert!(edge_0_spatial.is_finite());
    assert_eq!(*edge_0_provenance, EdgeProvenance::Continuation);
    // If r carried provenance, this would be: r.provenance[0]
    // That field does not exist.
}

// ═══════════════════════════════════════════════════════════════════════════
// §4. ADMISSIBILITY ENFORCEMENT
// ═══════════════════════════════════════════════════════════════════════════

/// [V8-BASELINE] require_rho_declaration enforces M-supplied values.
/// ρ_base non-finite → rejected.
#[test]
fn admissibility_rho_rejects_non_finite_base() {
    let result = std::panic::catch_unwind(|| {
        require_rho_declaration(f64::NAN, 1.0)
    });
    assert!(result.is_err(), "non-finite ρ_base must be rejected");
}

/// [V8-BASELINE] χ₀ = 0 produces 0/0 at χ = 0 — structurally inadmissible.
#[test]
fn admissibility_rho_rejects_zero_chi_0() {
    let result = std::panic::catch_unwind(|| {
        require_rho_declaration(0.5, 0.0)
    });
    assert!(result.is_err(), "χ₀ = 0 must be rejected");
}

/// [V8-BASELINE] χ₀ negative produces a singularity at χ = −χ₀ — inadmissible.
#[test]
fn admissibility_rho_rejects_negative_chi_0() {
    let result = std::panic::catch_unwind(|| {
        require_rho_declaration(0.5, -1.0)
    });
    assert!(result.is_err(), "negative χ₀ must be rejected");
}

/// [V8-BASELINE] Valid ρ_base and χ₀ are accepted.
#[test]
fn admissibility_rho_accepts_valid_declaration() {
    let result = std::panic::catch_unwind(|| {
        require_rho_declaration(0.5, 1.0)
    });
    assert!(result.is_ok(), "valid ρ_base and χ₀ must be accepted");
}

// ═══════════════════════════════════════════════════════════════════════════
// §5. OPEN CONDITION PRESERVATION
// ═══════════════════════════════════════════════════════════════════════════

/// [V8-BASELINE] PersistenceState requires a prior observation.
/// No cold-start constructor exists (audit A-1 CLOSED in Kernel V8).
/// The only constructor is from_prior(e_prior: EdgeField), which takes a
/// kernel output from a completed declared observation.
///
/// This test constructs a PersistenceState from a real ABR output to confirm
/// the from_prior pathway is intact — and implicitly confirms that no
/// cold_start or default constructor exists.
#[test]
fn open_condition_persistence_requires_prior_observation() {
    let rel = rel_single_edge();
    let x = NodeField::new(vec![vec![1.0, 0.0]]);
    let pairs: &[(usize, usize)] = &[];
    let cc: &[f64] = &[];
    let e_spatial = operator_e(&x, &rel, pairs, cc, 0.5, 1.0);
    let state = PersistenceState::from_prior(e_spatial);
    // from_prior accepted a real operator output — CONFIRMED.
    // cold_start is absent: the type cannot be constructed without a prior.
    assert_eq!(state.e_prior.k, 1, "prior state carries operator output");
}

// ═══════════════════════════════════════════════════════════════════════════
// §6. V8 OPERATOR CONTINUITY
// ═══════════════════════════════════════════════════════════════════════════

/// [V8-BASELINE] Primary kernel Δ→Σ produces finite output.
#[test]
fn v8_continuity_primary_chain_finite() {
    let rel = rel_three_node_chain();
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0]]);
    let (delta, sigma) = operator_e_primary(&x, &rel, 0.5, 1.0);
    for &v in delta.field[0].iter() {
        assert!(v.is_finite(), "Δ output must be finite: {v}");
    }
    for &v in sigma.field[0].iter() {
        assert!(v.is_finite(), "Σ output must be finite: {v}");
    }
}

/// [V8-BASELINE] ABR kernel A→B→R produces finite output.
#[test]
fn v8_continuity_abr_chain_finite() {
    let rel = rel_three_node_chain();
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0]]);
    let pairs: &[(usize, usize)] = &[];
    let cc: &[f64] = &[];
    let r = operator_e(&x, &rel, pairs, cc, 0.5, 1.0);
    for &v in r.spatial[0].iter() {
        assert!(v.is_finite(), "R output must be finite: {v}");
    }
}

/// [V8-BASELINE] ρ is in [0, ρ_base) for any declared NodeField.
#[test]
fn v8_continuity_rho_bounds() {
    let rho_base = 0.7_f64;
    let chi_0 = 1.0_f64;
    let rel = rel_three_node_chain();
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0]]);
    let delta = operator_delta(&x, &rel);
    let rho = compute_rho_primary(&delta, &rel, rho_base, chi_0);
    for &r in &rho {
        assert!(r >= 0.0, "ρ must be ≥ 0, got {r}");
        assert!(r < rho_base, "ρ must be < ρ_base ({rho_base}), got {r}");
    }
}

/// [V8-BASELINE] A→B→R ρ is in [0, ρ_base).
#[test]
fn v8_continuity_abr_rho_bounds() {
    let rho_base = 0.7_f64;
    let chi_0 = 1.0_f64;
    let rel = rel_three_node_chain();
    let x = NodeField::new(vec![vec![3.0, 1.0, 2.0]]);
    let pairs: &[(usize, usize)] = &[];
    let a = operator_a(&x, &rel, pairs);
    let rho = compute_rho(&a, &rel, rho_base, chi_0);
    for &r in &rho {
        assert!(r >= 0.0, "ABR ρ must be ≥ 0, got {r}");
        assert!(r < rho_base, "ABR ρ must be < ρ_base ({rho_base}), got {r}");
    }
}

/// [V8-BASELINE] A→B→R with zero field produces zero spatial output.
/// Δ(0) = 0; A(0) = 0; ρ(0) = 0; B(0) = 0; R(0) = 0.
#[test]
fn v8_continuity_abr_zero_field_produces_zero_output() {
    let rel = rel_three_node_chain();
    let x = NodeField::new(vec![vec![0.0, 0.0, 0.0]]);
    let pairs: &[(usize, usize)] = &[];
    let cc: &[f64] = &[];
    let r = operator_e(&x, &rel, pairs, cc, 0.5, 1.0);
    for &v in r.spatial[0].iter() {
        assert!(
            v.abs() < 1e-12,
            "R(B(A(0))) must be 0.0 everywhere, got {v}"
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// §7. IRRS INTEGRATION
// ═══════════════════════════════════════════════════════════════════════════

/// [GAP] Tracing from operator output to observable provenance requires
/// externally retaining DeclaredRelations.
///
/// The verifier CAN trace — but only as a (output, rel) pair. The output
/// type alone is not sufficient. V9 closes this gap with ProvenancedOutput<T>.
///
/// This test demonstrates the full trace path as it exists today, making the
/// reliance on external rel explicit.
#[test]
fn irrs_integration_gap_tracing_requires_external_rel() {
    let rel = rel_three_node_chain();
    let x = NodeField::new(vec![vec![4.0, 1.0, 3.0]]);
    let (delta, _sigma) = operator_e_primary(&x, &rel, 0.5, 1.0);

    // The verifier traces edge 1 (the (1,2) continuation):
    let edge_idx = 1;
    let computed_result = delta.field[0][edge_idx];

    // Trace via rel (EXTERNAL — not from delta):
    let (src, tgt) = rel.edges[edge_idx];
    let provenance_class = &rel.provenance[edge_idx];
    let declared_src_value = x.data[0][src];
    let declared_tgt_value = x.data[0][tgt];

    // The declared difference matches the computed result:
    assert!(
        (computed_result - (declared_src_value - declared_tgt_value)).abs() < 1e-12,
        "computed result must equal x[src] - x[tgt]"
    );
    assert_eq!(
        *provenance_class,
        EdgeProvenance::Continuation,
        "edge {edge_idx} provenance must be Continuation"
    );

    // GAP: none of (src, tgt, provenance_class) came from delta.
    // They came from rel, which was retained separately.
    // If delta carried ProvenancedOutput, these would be:
    //   delta.provenance_of(edge_idx).src
    //   delta.provenance_of(edge_idx).tgt
    //   delta.provenance_of(edge_idx).class
    // Those methods do not exist — the gap is explicit.
}

/// [GAP] The same tracing gap exists for ABR kernel output.
#[test]
fn irrs_integration_gap_abr_tracing_requires_external_rel() {
    let rel = rel_three_node_chain();
    let x = NodeField::new(vec![vec![4.0, 1.0, 3.0]]);
    let pairs: &[(usize, usize)] = &[];
    let cc: &[f64] = &[];
    let r = operator_e(&x, &rel, pairs, cc, 0.5, 1.0);

    let edge_idx = 0;
    let computed_spatial = r.spatial[0][edge_idx];
    let provenance_class = &rel.provenance[edge_idx]; // EXTERNAL — not from r

    assert!(computed_spatial.is_finite());
    assert_eq!(*provenance_class, EdgeProvenance::Continuation);
    // GAP: r.spatial carries no provenance reference.
    // Tracing requires (r, rel) as a pair held externally.
}

// ═══════════════════════════════════════════════════════════════════════════
// §8. RELATIONAL PRIMITIVE PROVENANCE INVARIANT (RPPI)
// ═══════════════════════════════════════════════════════════════════════════
//
// Governing requirement (v9_specification.md §2):
//   Every admitted mathematical variable, quantity, operation, and result must
//   have a continuous, verifiable provenance path to an Origin-declared
//   observable relation within D.
//
// Acceptance criterion:
//   No mathematical result can be admitted solely on numerical correctness,
//   structural validity, or named provenance. Its derivation must terminate
//   in declared observable relations.
//
// The four RPPI failure modes (v9_specification.md §6):
//
//   MISSING      — no ObservableRelationDeclaration; provenance identity absent.
//                  A class label (EdgeProvenance::Continuation) is not an identity.
//
//   FABRICATED   — ObservableRelationDeclaration present but observable_source not
//                  traceable to a declared M mapping; identity asserted, not traced.
//
//   DISCONNECTED — provenance identity present and valid; derivation path from
//                  result to declared observable relation broken; numerical result
//                  exists without relational grounding in the output type.
//
//   MISATTRIBUTED — derivation record attributes result to a declared relation
//                   other than the one that actually produced it; attribution is
//                   present, complete, and wrong.

// ── §8.1  MISSING PROVENANCE ─────────────────────────────────────────────────

/// [RPPI-GAP] MISSING PROVENANCE: a relation with class-only `EdgeProvenance`
/// is accepted without an `ObservableRelationDeclaration`. Under RPPI, a class
/// label is not a provenance identity. This is inadmissible; the current API
/// does not enforce it.
///
/// V9 enforcement (see `rppi_missing_provenance_not_admitted` below):
/// the V9 admission interface must reject when `ObservableRelationDeclaration`
/// is absent for any edge. V8 constructors remain frozen (Req-5;
/// v9_specification.md Correction 2).
#[test]
fn rppi_missing_observable_source_gap() {
    // EdgeProvenance::Continuation — a class label, not a declaration.
    // No observable_source. No direction_basis. No m_mapping_ref.
    let result = std::panic::catch_unwind(|| {
        DeclaredRelations::from_edges_with_provenance(
            2,
            vec![(0, 1)],
            vec![EdgeProvenance::Continuation],
        )
    });
    // RPPI-GAP: accepted without observable identity.
    // V9 enforcement: must be rejected when ObservableRelationDeclaration is absent.
    assert!(
        result.is_ok(),
        "RPPI-GAP: class-only provenance currently accepted — no observable_source enforced yet"
    );
}

/// [V9-REQUIRED] MISSING PROVENANCE — enforced (v9_specification.md §6.1; Req-1,
/// Req-2, Req-7).
///
/// An input without a complete, verifiable observational provenance chain
/// remains OPEN and is not admitted. Absence is not a contradiction, so it is
/// never REJECTED (Origin correction 2026-10-25, rules 6–7).
///
/// Case 1 — the canonical H-alpha photon edge, with exactly the declarations
/// the canonical repository holds (tests/common: quoted verbatim, nothing
/// supplied). The Generator can execute the frozen ε[e] = hc/λ; executing it
/// establishes nothing. Admission reports each declaration that is absent.
/// Case 2 — a declared structure references a relation with no declaration.
/// Case 3 — an observable names no external record: not a terminus.
#[test]
fn rppi_missing_provenance_not_admitted() {
    // Case 1.
    let reg = halpha_canonical_registry();
    let po = execute_edge_m_projection(&reg, HA_EDGE).expect("frozen ε[e] = hc/λ executes");
    assert_eq!(po.record.outputs[0].value.to_bits(), epsilon_photon_edge(656.279e-9).to_bits());
    assert_eq!(
        admit(&po, &reg),
        AdmissionOutcome::Open(vec![
            OpenCondition::MissingMeasurementConditions { observable: HA_LAMBDA.into() },
            OpenCondition::MissingMeasurementConditions { observable: HA_REF_H.into() },
            OpenCondition::MissingMeasurementConditions { observable: HA_REF_C.into() },
            OpenCondition::MissingDeclaration { kind: DeclKind::Domain, id: HA_DOMAIN.into() },
            OpenCondition::MissingObservationalBasis { of: BasisOf::Class(HA_EDGE.into()) },
        ]),
        "canonical H-alpha chain is incomplete: OPEN, not admitted"
    );

    // Case 2 (synthetic): the record is a correct execution; the authoritative
    // registry lacks the relation's declaration record.
    let full = synthetic_primary_registry();
    let po = execute_primary(&full, SYN_DELTA).unwrap();
    let mut reg = full.clone();
    reg.relations.retain(|r| r.id != SYN_REL_01);
    assert_eq!(
        admit(&po, &reg),
        AdmissionOutcome::Open(vec![OpenCondition::MissingDeclaration {
            kind: DeclKind::Relation, id: SYN_REL_01.into(),
        }])
    );

    // Case 3 (synthetic): an observable with no external record.
    let mut reg = full.clone();
    reg.observables.iter_mut().find(|o| o.id == "syn.obs.a").unwrap().record = None;
    assert_eq!(
        admit(&po, &reg),
        AdmissionOutcome::Open(vec![OpenCondition::NoExternalTerminus { observable: "syn.obs.a".into() }])
    );
}

// ── §8.2  FABRICATED PROVENANCE ──────────────────────────────────────────────

/// [RPPI-GAP] FABRICATED PROVENANCE: an `EdgeProvenance` class can be assigned to
/// any edge regardless of whether the named class corresponds to a declared
/// observable through M. Under RPPI, provenance identity must be verifiable
/// against a declared M mapping. Current enforcement has no such check.
///
/// V9 enforcement (see `rppi_fabricated_provenance_rejected` below):
/// the V9 admission interface must verify that `m_mapping_ref` names an M
/// declared by Origin. V8 constructors remain frozen (Req-5).
#[test]
fn rppi_fabricated_provenance_gap() {
    // A Coupling edge is declared, but no cross-family observable has been
    // declared through M for this relation. Under the current API this is
    // accepted: the class is asserted, not verified against a declared M.
    let result = std::panic::catch_unwind(|| {
        DeclaredRelations::from_edges_with_provenance(
            2,
            vec![(0, 1)],
            vec![EdgeProvenance::Coupling], // asserted — no M-mapping verification
        )
    });
    // RPPI-GAP: accepted without M-mapping verification.
    // The class name is structurally valid but its provenance identity is
    // fabricated — it traces to no declared observable in D.
    assert!(
        result.is_ok(),
        "RPPI-GAP: asserted class provenance currently accepted — no M-mapping check exists"
    );
}

/// [V9-REQUIRED] FABRICATED PROVENANCE — enforced (v9_specification.md §6.2;
/// Req-2, Req-6, Req-7).
///
/// A demonstrated contradiction between a claimed declaration and the
/// authoritative declaration with the same id is REJECTED, with both sides as
/// evidence. A claim citing a declaration that is not available cannot be
/// verified and remains OPEN. Synthetic fixtures; enforcement behaviour only.
#[test]
fn rppi_fabricated_provenance_rejected() {
    let reg = synthetic_primary_registry();
    let po = execute_primary(&reg, SYN_DELTA).unwrap();
    assert!(matches!(admit(&po, &reg), AdmissionOutcome::Admitted(_)), "control: unaltered record admitted");

    // Claimed observable source contradicts the declared external record.
    let mut forged = po.clone();
    forged.record.inputs[0].claimed_source = "NIST ASD v5.12".into();
    assert_eq!(
        admit(&forged, &reg),
        AdmissionOutcome::Rejected(RppiViolation::Fabricated {
            id: "syn.obs.a".into(), field: "source".into(),
            declared: SYN_SOURCE.into(), recorded: "NIST ASD v5.12".into(),
        })
    );

    // Claimed observed value contradicts the declared reported value.
    let mut forged = po.clone();
    forged.record.inputs[1].observed_value = 1.5;
    assert_eq!(
        admit(&forged, &reg),
        AdmissionOutcome::Rejected(RppiViolation::Fabricated {
            id: "syn.obs.b".into(), field: "reported_value".into(),
            declared: "1e0".into(), recorded: "1.5e0".into(),
        })
    );

    // Claimed parameter value contradicts the declared parameter observable.
    let po_chain = execute_primary(&reg, SYN_CHAIN).unwrap();
    let mut forged = po_chain.clone();
    forged.record.parameters[0].value = 0.25;
    assert_eq!(
        admit(&forged, &reg),
        AdmissionOutcome::Rejected(RppiViolation::Fabricated {
            id: "syn.obs.rho_base".into(), field: "parameter rho_base".into(),
            declared: "5e-1".into(), recorded: "2.5e-1".into(),
        })
    );

    // Unverifiable, not contradicted: the cited observable is not declared.
    let mut forged = po.clone();
    forged.record.inputs[0].observable_ref = "obs.not_declared".into();
    assert_eq!(
        admit(&forged, &reg),
        AdmissionOutcome::Open(vec![OpenCondition::Unverifiable {
            kind: DeclKind::Observable, id: "obs.not_declared".into(),
        }])
    );
}

// ── §8.3  DISCONNECTED PROVENANCE ────────────────────────────────────────────

/// [RPPI-GAP] DISCONNECTED PROVENANCE: an operator result exists and is
/// numerically correct, but its derivation path to the declared observable
/// relation that produced it is broken. Numerical correctness and structural
/// validity are satisfied; relational grounding in the output type is not.
///
/// This is the precise RPPI framing of the provenance survival gap documented in
/// §3 and §7. All current operator outputs are disconnected in this sense.
///
/// V9 enforcement (see `rppi_disconnected_derivation_rejected` below):
/// IRRS must reject any result that does not carry a continuous derivation path
/// through `ProvenancedOutput<T>`.
#[test]
fn rppi_disconnected_provenance_gap() {
    let rel = rel_three_node_chain();
    let x = NodeField::new(vec![vec![5.0, 2.0, 1.0]]);
    let delta = operator_delta(&x, &rel);

    // Numerically correct and structurally valid:
    assert!(delta.field[0][0].is_finite()); // x[0] - x[1] = 3.0
    assert!(delta.field[0][1].is_finite()); // x[1] - x[2] = 1.0

    // RPPI-GAP: delta carries no derivation reference to rel.
    // The result is DISCONNECTED from its declared observable relation.
    //
    // Under RPPI, admission requires a continuous derivation path. delta cannot
    // provide that from its own type. The verifier can bridge this gap externally
    // by retaining (delta, rel) — but external coupling is not derivation carried
    // in the output, and it does not satisfy the RPPI.
    //
    // Explicit record of the gap:
    let can_trace_derivation_from_delta_alone: bool = false;
    assert!(
        !can_trace_derivation_from_delta_alone,
        "RPPI-GAP: delta carries no derivation reference — provenance is disconnected"
    );
}

/// [V9-REQUIRED] DISCONNECTED DERIVATION — enforced (v9_specification.md §6.3;
/// Req-4, Req-6).
///
/// A calculation record that contradicts the independently reconstructed
/// derivation is REJECTED at the located link. Admission rebuilds inputs,
/// traversal, and outputs from the declarations alone (it never reads the
/// record's account or the V8 structures it carries) and compares. The links
/// below are identity and traversal contradictions, not merely numerical
/// disagreement. Synthetic fixtures (Primary Δ and Σ, ρ_base and χ₀ declared
/// synthetically); enforcement behaviour only.
#[test]
fn rppi_disconnected_derivation_rejected() {
    let reg = synthetic_primary_registry();
    let po = execute_primary(&reg, SYN_CHAIN).unwrap();
    assert!(matches!(admit(&po, &reg), AdmissionOutcome::Admitted(_)), "control: unaltered record admitted");

    // Input binding: locus 0 is bound to a different declared observation. Its
    // source and value are that observation's own, so nothing is fabricated;
    // the binding contradicts the locus declaration.
    let mut forged = po.clone();
    forged.record.inputs[0].observable_ref = "syn.obs.c".into();
    forged.record.inputs[0].observed_value = 2.0;
    forged.record.inputs[0].projected_value = 2.0;
    assert_eq!(
        admit(&forged, &reg),
        AdmissionOutcome::Rejected(RppiViolation::Disconnected {
            link: Link::InputBinding, entry: "syn.locus.0".into(),
            reconstructed: "syn.obs.a via syn.m.identity".into(),
            recorded: "syn.obs.c via syn.m.identity".into(),
        })
    );

    // Traversal: the record claims Σ read no successor at edge 0; the declared
    // relations give adj⁺(0) = {1}.
    let mut forged = po.clone();
    forged.record.relations[0].adj_plus = vec![];
    match admit(&forged, &reg) {
        AdmissionOutcome::Rejected(RppiViolation::Disconnected { link: Link::Traversal, entry, reconstructed, recorded }) => {
            assert_eq!(entry, "relation[0]");
            assert!(reconstructed.contains("adj+[1]") && recorded.contains("adj+[]"), "{reconstructed} / {recorded}");
        }
        other => panic!("expected Disconnected(Traversal), got {other:?}"),
    }

    // Output derivation: the record's derivation of Σ[syn.rel.01] omits the
    // declared parameters ρ_base and χ₀ the reconstructed Σ used.
    let mut forged = po.clone();
    let k = forged.record.outputs.iter()
        .position(|o| o.relation_ref == SYN_REL_01 && format!("{:?}", o.step) == "Sigma").unwrap();
    forged.record.outputs[k].contributing.retain(|d| d != "syn.obs.rho_base" && d != "syn.obs.chi_0");
    match admit(&forged, &reg) {
        AdmissionOutcome::Rejected(RppiViolation::Disconnected { link: Link::OutputDerivation, entry, reconstructed, recorded }) => {
            assert_eq!(entry, format!("Sigma[{SYN_REL_01}]"));
            assert!(reconstructed.contains("syn.obs.rho_base") && !recorded.contains("syn.obs.rho_base"));
        }
        other => panic!("expected Disconnected(OutputDerivation), got {other:?}"),
    }

    // Output derivation: the carried V8 output was altered after execution
    // while the record still describes the declared calculation.
    let mut forged = po.clone();
    if let CalculatedOutput::Primary { delta, .. } = &mut forged.output {
        delta.field[0][0] += 1.0;
    }
    assert!(matches!(
        admit(&forged, &reg),
        AdmissionOutcome::Rejected(RppiViolation::Disconnected { link: Link::OutputDerivation, ref entry, .. })
            if entry == "carried output"
    ));
}

// ── §8.4  MISATTRIBUTED PROVENANCE ───────────────────────────────────────────

/// [RPPI-GAP] MISATTRIBUTED PROVENANCE: no attribution mechanism currently
/// exists. A result computed from one `DeclaredRelations` can be presented
/// alongside a different `DeclaredRelations` without detection. The structural
/// gap precedes the attribution gap: before a result can be attributed incorrectly,
/// it must be possible to attribute it at all.
///
/// V9 enforcement (see `rppi_misattributed_provenance_rejected` below):
/// `ProvenancedOutput<T>` carries a binding reference. IRRS verifies the binding
/// during admission. A result presented with a substituted declaration must panic.
#[test]
fn rppi_misattributed_provenance_gap() {
    let rel_a = rel_single_edge(); // edge (0→1), Continuation
    let rel_b = DeclaredRelations::from_edges_with_provenance(
        2,
        vec![(0, 1)],
        vec![EdgeProvenance::Coupling], // different class — a distinct declared relation
    );
    let x = NodeField::new(vec![vec![3.0, 1.0]]);
    let delta_from_a = operator_delta(&x, &rel_a);

    // delta_from_a was produced using rel_a.
    // Nothing in delta_from_a's type prevents it from being presented as if it
    // were produced from rel_b. The derivation record does not exist; the
    // attribution gap precedes the misattribution gap.
    let _wrong_class = &rel_b.provenance[0]; // consulting the wrong relation — undetectable

    // The result from rel_a is numerically indistinguishable from a result
    // attributed to rel_b:
    assert!(
        delta_from_a.field[0][0].is_finite(),
        "RPPI-GAP: result from rel_a indistinguishable from a result attributed to rel_b — \
         substitution is undetectable without a derivation binding"
    );
}

/// [V9-REQUIRED] MISATTRIBUTED PROVENANCE — enforced (v9_specification.md §6.4;
/// Req-4, Req-6).
///
/// The output is produced through declared relation syn.rel.01 (0→1,
/// Continuation). The record attributes it to syn.rel.01.coupling — a declared
/// relation over the same loci with a different class. The numerical values are
/// identical; the attribution contradicts the reconstructed producing relation
/// and is REJECTED. Synthetic fixtures; enforcement behaviour only.
#[test]
fn rppi_misattributed_provenance_rejected() {
    let reg = synthetic_primary_registry();
    let po = execute_primary(&reg, SYN_DELTA).unwrap();
    assert!(matches!(admit(&po, &reg), AdmissionOutcome::Admitted(_)), "control: unaltered record admitted");

    let mut forged = po.clone();
    forged.record.relations[0].relation_ref = SYN_REL_01_COUPLING.into();
    forged.record.outputs[0].relation_ref = SYN_REL_01_COUPLING.into();
    for c in forged.record.outputs[0].contributing.iter_mut() {
        if c == SYN_REL_01 {
            *c = SYN_REL_01_COUPLING.into();
        }
    }
    assert_eq!(forged.record.outputs[0].value.to_bits(), po.record.outputs[0].value.to_bits(),
        "numerical output unchanged");
    assert_eq!(
        admit(&forged, &reg),
        AdmissionOutcome::Rejected(RppiViolation::Misattributed {
            entry: "relation[0]".into(),
            claimed: SYN_REL_01_COUPLING.into(),
            reconstructed: SYN_REL_01.into(),
        })
    );
}
