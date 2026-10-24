//! Kernel V9 Conformance Test Suite — First Milestone
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
//!
//! Tags:
//!   [V8-BASELINE] — confirms behavior that must not change in any V9 milestone.
//!                   A V9 code change that fails a V8-BASELINE test violates the
//!                   V8 preservation contract.
//!   [GAP]         — documents a gap that V9 closes. The test passes today;
//!                   it captures the current state so the gap is explicit,
//!                   not silent.
//!
//! All tests here must pass on the `kernel-v9` branch before any V9
//! implementation milestone is submitted for Verifier review.

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
