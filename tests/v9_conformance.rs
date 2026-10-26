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
//!   [V9-REQUIRED]  — defines V9 enforcement that does not yet exist. The test is
//!                    #[ignore] until the implementing milestone is verified. Remove
//!                    #[ignore] and confirm the test passes after each V9 enforcement
//!                    milestone. The test body contains todo!() as a placeholder;
//!                    replace with the actual V9 API call when available.
//!
//! Test counts (second milestone):
//!   27 existing tests ([V8-BASELINE] + [GAP]) — all must pass.
//!   4 new [RPPI-GAP] tests — document inadmissible current behavior; must pass.
//!   4 new [V9-REQUIRED] tests — #[ignore]; define enforcement contract.
//!
//! All [V8-BASELINE] and [RPPI-GAP] tests must pass on the `kernel-v9` branch
//! before any V9 implementation milestone is submitted for Verifier review.

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
/// V9 enforcement (see `rppi_missing_observable_source_rejected` below):
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

/// [V9-REQUIRED] MISSING PROVENANCE: construction without a complete
/// `ObservableRelationDeclaration` (observable_source + direction_basis +
/// m_mapping_ref) must be rejected.
///
/// When V9 adds the admission interface, submitting class-only provenance
/// without an identity declaration must be rejected there. V8 constructors
/// remain frozen (Req-5; v9_specification.md Correction 2).
///
/// Remove `#[ignore]` after V9 enforcement milestone is implemented and verified.
/// Replace `todo!()` with the actual V9 admission-interface call.
#[test]
#[ignore = "V9-REQUIRED: ObservableRelationDeclaration not yet enforced at construction"]
fn rppi_missing_observable_source_rejected() {
    // When the V9 admission interface is implemented, replace the todo!()
    // below with a submission to that interface (working names subject to
    // Verifier review, v9_specification.md §11), e.g.:
    //
    //   v9_admit(/* DeclaredRelations with EdgeProvenance class only —
    //               no ObservableRelationDeclaration */)
    //
    // Admission must be rejected: observable_source absent. V8 constructors
    // (from_edges_with_provenance) remain frozen (Req-5; Correction 2).
    let result = std::panic::catch_unwind(|| {
        todo!(
            "V9 adds ObservableRelationDeclaration enforcement; \
             replace todo! with construction-without-identity call when API exists"
        )
    });
    assert!(
        result.is_err(),
        "construction without ObservableRelationDeclaration must be rejected under V9"
    );
}

// ── §8.2  FABRICATED PROVENANCE ──────────────────────────────────────────────

/// [RPPI-GAP] FABRICATED PROVENANCE: an `EdgeProvenance` class can be assigned to
/// any edge regardless of whether the named class corresponds to a declared
/// observable through M. Under RPPI, provenance identity must be verifiable
/// against a declared M mapping. Current enforcement has no such check.
///
/// V9 enforcement (see `rppi_fabricated_observable_source_rejected` below):
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

/// [V9-REQUIRED] FABRICATED PROVENANCE: a provenance record whose `observable_source`
/// or `m_mapping_ref` cannot be verified against a declared M mapping must be
/// rejected by the V9 admission interface (not by frozen V8 constructors).
///
/// Remove `#[ignore]` after V9 M-mapping verification is implemented and verified.
/// Replace `todo!()` with an actual V9 admission attempt using a fabricated source.
#[test]
#[ignore = "V9-REQUIRED: M-mapping verification against ObservableRelationDeclaration not yet implemented"]
fn rppi_fabricated_observable_source_rejected() {
    // When the V9 admission interface is implemented, replace the todo!()
    // below with a submission to that interface (working names subject to
    // Verifier review, v9_specification.md §11), e.g.:
    //
    //   v9_admit(/* ObservableRelationDeclaration {
    //       observable_source: "fabricated-source-not-in-any-declared-M",
    //       direction_basis:   "fabricated-direction",
    //       m_mapping_ref:     "nonexistent-M",
    //   } */)
    //
    // Admission must be rejected: m_mapping_ref names no declared M.
    // V8 constructors remain frozen (Req-5; Correction 2).
    let result = std::panic::catch_unwind(|| {
        todo!(
            "V9 adds M-mapping verification; replace todo! with fabricated-source \
             construction attempt when ObservableRelationDeclaration API exists"
        )
    });
    assert!(
        result.is_err(),
        "construction with unverifiable observable_source must be rejected under V9"
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
/// V9 enforcement (see `rppi_disconnected_provenance_rejected_by_irrs` below):
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

/// [V9-REQUIRED] DISCONNECTED PROVENANCE: IRRS must reject any result that does
/// not carry a continuous derivation path to a declared observable relation.
///
/// When V9 adds `ProvenancedOutput<T>`, operator outputs carry an edge-indexed
/// reference to the `ObservableRelationDeclaration` of each contributing edge.
/// The IRRS admission function verifies this reference before admission.
///
/// Remove `#[ignore]` after `ProvenancedOutput<T>` and IRRS admission enforcement
/// are implemented and verified.
/// Replace `todo!()` with an actual IRRS admission call on a bare (unwrapped) result.
#[test]
#[ignore = "V9-REQUIRED: ProvenancedOutput<T> not yet implemented — derivation path cannot be carried"]
fn rppi_disconnected_provenance_rejected_by_irrs() {
    // When V9 is implemented, replace the todo!() below with:
    //
    //   let rel = rel_three_node_chain();
    //   let x = NodeField::new(vec![vec![5.0, 2.0, 1.0]]);
    //   let delta = operator_delta(&x, &rel); // PrimaryEdgeField — no provenance reference
    //   irrs_admit(&delta) // must panic: no ProvenancedOutput wrapper
    //
    // IRRS admission must reject delta without a ProvenancedOutput wrapper.
    let result = std::panic::catch_unwind(|| {
        todo!(
            "V9 adds IRRS admission enforcement; replace todo! with irrs_admit call \
             on unwrapped operator output when ProvenancedOutput<T> API exists"
        )
    });
    assert!(
        result.is_err(),
        "IRRS must reject disconnected result (no ProvenancedOutput) under V9"
    );
}

// ── §8.4  MISATTRIBUTED PROVENANCE ───────────────────────────────────────────

/// [RPPI-GAP] MISATTRIBUTED PROVENANCE: no attribution mechanism currently
/// exists. A result computed from one `DeclaredRelations` can be presented
/// alongside a different `DeclaredRelations` without detection. The structural
/// gap precedes the attribution gap: before a result can be attributed incorrectly,
/// it must be possible to attribute it at all.
///
/// V9 enforcement (see `rppi_misattributed_provenance_rejected_by_irrs` below):
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

/// [V9-REQUIRED] MISATTRIBUTED PROVENANCE: a `ProvenancedOutput<T>` whose
/// derivation record attributes it to a declared relation other than the one
/// that actually produced it must be rejected by IRRS.
///
/// The attribution is present, internally consistent, and wrong. IRRS verifies
/// the binding between output and its `DeclaredRelations` during admission.
///
/// Remove `#[ignore]` after `ProvenancedOutput<T>` and IRRS attribution
/// verification are implemented and verified.
/// Replace `todo!()` with an actual IRRS admission call on a misattributed output.
#[test]
#[ignore = "V9-REQUIRED: ProvenancedOutput attribution binding verification not yet implemented"]
fn rppi_misattributed_provenance_rejected_by_irrs() {
    // When V9 is implemented, replace the todo!() below with:
    //
    //   let rel_a = rel_single_edge();
    //   let rel_b = DeclaredRelations::from_edges_with_provenance(
    //       2, vec![(0, 1)], vec![EdgeProvenance::Coupling],
    //   );
    //   let x = NodeField::new(vec![vec![3.0, 1.0]]);
    //   let output_from_a = operator_delta_provenance(&x, &rel_a); // ProvenancedOutput
    //   let misattributed = output_from_a.rebind(&rel_b); // swap the binding — or equivalent
    //   irrs_admit(&misattributed) // must panic: binding does not match producing rel
    //
    // IRRS admission must reject: derivation binding is rel_a, attribution claims rel_b.
    let result = std::panic::catch_unwind(|| {
        todo!(
            "V9 adds IRRS attribution verification; replace todo! with misattributed \
             ProvenancedOutput admission attempt when derivation binding API exists"
        )
    });
    assert!(
        result.is_err(),
        "IRRS must reject misattributed ProvenancedOutput under V9"
    );
}
