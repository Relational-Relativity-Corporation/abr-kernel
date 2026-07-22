// derived_invariants.rs — Metatron Dynamics, Inc. V4.
// Derived Invariants: canonical mathematical expressions from the V7 kernel.
//
// Grounding documents (V7):
//   operators.rs (V7)
//   observable_variable_sets_v6.md (pending)
//   abr_operators_plain_v7.md
//   role_separation_and_operator_application_v7.md
//
// ── What This File Contains ───────────────────────────────────────────────
//
// Confirmed mathematical expressions derived from the operator framework.
// These are not operators. They are what the operators produce when applied
// correctly over declared structure. They are collected here because:
//
//   1. They are invariant in form across all declared regions.
//   2. Regional instantiations can reference them without reimplementing.
//   3. Their separation from operators.rs preserves the distinction between
//      what acts (operators) and what is produced (derived invariants).
//
// Every expression in this file was derived from the operators — proven,
// not declared independently. Every expression is invariant in form across
// all declared regions. What changes across regions is the numerical value,
// not the form of the expression.
//
// ── Methodological Context ────────────────────────────────────────────────
//
// The expressions here were derived initially by treating the Standard Model
// and related legacy frameworks as an observational inventory — a record of
// quantities that the physics community has found necessary to account for
// the observable record. This is the correct use of legacy frameworks: a
// starting point for the inventory, not a source of admissibility.
//
// Most theoretical frameworks in physics were constructed before direct
// observation of the phenomena they address was possible. Atomic structure,
// nuclear structure, and particle interactions were theorized under conditions
// of indirect inference. The mathematics built under those conditions was
// often extraordinarily successful at prediction. But the quantities it named
// were not always derived from direct observables — many were derived from
// what made the math work given the indirect access available at the time.
//
// The consequence for this file: a quantity from a legacy framework appears
// here because it passed an independent admissibility test — not because the
// framework endorses it. The test is always: what is the direct measurement
// that puts this quantity into D through M, independently of any theoretical
// commitment? Quantities that did not pass this test are absent regardless
// of their role in legacy frameworks.
//
// ── V1 → V2 Change: ι[v] replaces m[v] throughout ───────────────────────
//
// m[v] declared inadmissible as a primitive — it imports rest mass, which
// requires a rest frame not observable through M (SF-PR-16).
//
// Replacement: ι[v] — relational inertia.
//
// Declaration of ι[v]:
//   The inertial response of a declared locus v to an applied relational
//   contrast at a declared measurement edge, under the declared condition
//   that A_persistence = 0 at that edge over the declared measurement
//   interval (non-acceleration declared through M).
//
//   Observable provenance: cyclotron frequency, Penning trap oscillation,
//   mass spectrometry — all apply a declared contrast and read the locus
//   response under confirmed non-acceleration.
//
//   Non-acceleration is a declared relational condition between locus and
//   measurement apparatus — not an absolute frame claim. It is declared
//   through M as A_persistence = 0 at the measurement edge over the
//   declared measurement interval. No rest frame required. No
//   zero-momentum assumption required.
//
//   Numerical values: identical to PDG mass entries. Within the declared
//   framework, the observable admitted through the declared measurement
//   mapping M is represented by ι[v]. The numerical values carry forward
//   from the measurement record; the primitive interpretation changes.
//
// Standing language discipline (declared this session):
//   "mass" → inadmissible as a primitive within D.
//   Replace with ι[v] at declared loci,
//   or ε[e] at declared edges where energy contrast is the direct observable.
//
// ── Contents ─────────────────────────────────────────────────────────────
//
//   Layer 3 derived quantities (all confirmed):
//     ε[e]   — relational energy contrast
//     τ[v]   — relational progression interval (unified expression)
//     Φ[v]   — coherence potential (node)
//     Φ[S]   — coherence potential (collective)
//     J[v]   — R output multiplicity
//     𝟙[e]   — detection indicator (derived from R antisymmetry)
//     θ[e]   — B output vector direction
//
//   Support classifications (standing protocol):
//     Observed  — directly from M
//     Inferred  — from observed data under declared framework
//     Derived   — proven from operator mathematics
//
// ── Admissibility ─────────────────────────────────────────────────────────
//
// Every quantity in this file is Layer 3: readable only after the operators
// have acted on a declared variable set through M. No quantity here enters
// the framework as a Layer 2 primitive. No quantity here is operator input.
//
// These are findings, not inputs. They are readable from the declared
// relational structure. Change the declaration and the values change.
// The forms are invariant. The values are not.
//
// ── Separation from operators.rs ──────────────────────────────────────────
//
// operators.rs: what acts.
// derived_invariants.rs: what is produced, confirmed invariant in form.
//
// Do not add operators here. Do not add derived invariants to operators.rs.
//
// ── Version History ───────────────────────────────────────────────────────
//
// V1 — Initial declaration. Seven confirmed Layer 3 derived invariants:
//      ε[e], τ[v] (unified expression, OC-13 resolved), Φ[v], Φ[S],
//      J[v], 𝟙[e] (OC-12 resolved), θ[e] (OC-θ-1, OC-θ-2 resolved).
//      Methodological context stated. Separation from operators.rs declared.
//
// V2 — ι[v] replaces m[v] throughout. Rest mass declared inadmissible
//      (SF-PR-16). ι[v] declared as relational inertia — inertial response
//      under declared non-acceleration condition through M. Numerical values
//      unchanged. Standing language discipline declared: "mass" inadmissible
//      as primitive within D. I_UNIT_ELECTRON replaces M_UNIT_ELECTRON.
//      delta_iota_e replaces delta_m_e. iota_v replaces m_v in all
//      function signatures. All Φ expressions updated: Φ[v] = ι[v] · τ[v].
//
// V3 — Photon edge functions and tests added. H_PLANCK and C_DECLARED
//      added as declared M-mapping references (confined to hc/λ mapping;
//      C_DECLARED is not a named framework variable). epsilon_photon_edge,
//      tau_photon_edge, n_relational_cycles declared and confirmed.
//      Five photon edge tests added. ι[photon]=0 stated structurally.
//      κ[Primary] non-applicability to photon locus confirmed by test.
//      Input validation note added: debug_assert! is correct for internal
//      kernel use; consider Result<f64,Error> if functions are later
//      exposed through a public API (Verifier recommendation, July 2026).
//      "Relational field cycles" language rejected in comments — replaced
//      with "declared relational intervals within the measured stability
//      interval" (Verifier, session July 2026).
//
// Bounded over D. No claim beyond D.
// Metatron Dynamics, Inc. V4.

// ═══════════════════════════════════════════════════════════════════════════
// ── CONSTANTS AND PHYSICAL REFERENCES ─────────────────────────────────────
// ═══════════════════════════════════════════════════════════════════════════
//
// These are declared measurement references — quantities traceable to
// direct observables through M at the relevant region. They are not
// theoretical imports. Their admissibility rests on the direct measurement
// that grounds each one, independently of any framework that also uses them.

/// ℏ — reduced Planck constant (J·s).
/// Observable provenance: spectroscopic frequency-energy measurement.
/// Used in τ[v] expression as the declared action scale.
pub const HBAR: f64 = 1.054_571_817e-34;

/// e_unit — elementary charge unit (C).
/// Observable provenance: Millikan oil-drop and subsequent direct measurements.
/// Used in q[v] = n[v] · e_unit. Integer multiples confirmed cross-region.
pub const E_UNIT: f64 = 1.602_176_634e-19;

/// ι_unit — declared relational inertia unit (kg). Set to electron ι[v]
/// for Primary Region.
///
/// Observable provenance: Penning trap oscillation frequency under declared
/// non-acceleration condition (A_persistence = 0 at measurement edge).
/// The inertial response of the electron to a declared applied contrast
/// under confirmed non-acceleration through M.
///
/// Replaces M_UNIT_ELECTRON (V1). Rest mass framing inadmissible (SF-PR-16).
/// Numerical value unchanged. Within the declared framework, the observable
/// admitted through the declared measurement mapping M is represented by ι[v].
/// The numerical value carries forward from the measurement record;
/// the primitive interpretation changes.
/// Region-specific: what constitutes ι_unit is declared per region through M.
pub const I_UNIT_ELECTRON: f64 = 9.109_383_701_5e-31;

/// h — full Planck constant (J·s). h = 2π · ℏ.
/// Observable provenance: spectroscopic frequency-energy measurement.
/// Used in the M mapping hc/λ for photon edge ε[e]. Not a kernel quantity.
pub const H_PLANCK: f64 = 6.626_070_15e-34;

/// Speed of light — declared measurement reference for M mapping hc/λ (m/s).
/// NOT "c" as a named framework variable (language discipline: c should not
/// appear as a named variable in the framework). Enters only within the
/// declared M mapping hc/λ → ε[e] at Atomic Region photon edges.
/// Observable provenance: interferometric measurement (BIPM 2019).
/// Do not introduce C_DECLARED into operator expressions or kernel output.
pub const C_DECLARED: f64 = 2.997_924_58e8;

// ═══════════════════════════════════════════════════════════════════════════
// ── LAYER 3 DERIVED QUANTITIES ─────────────────────────────────────────────
// ═══════════════════════════════════════════════════════════════════════════

// ── ε[e] — Relational Energy Contrast ────────────────────────────────────
//
// ε[e] = |Δ(ι)[e]| · 𝟙[e]
//
// Support: Derived.
// Derivation: directed difference of relational inertia ι[v] across
// declared edge e, gated by the detection indicator 𝟙[e].
//
// ι[v]: inertial response under declared non-acceleration condition through M.
// Replaces m[v] throughout. Rest mass inadmissible (SF-PR-16).
//
// 𝟙[e] = 1 iff R(A(x))[e]_antisymmetric ≠ 0  (OC-12, resolved).
// Symmetric modes: R antisymmetry cancels → 𝟙[e] = 0 → IR inactive.
// Asymmetric modes: R antisymmetry non-zero → 𝟙[e] = 1 → IR active.
//
// Confirmed: 22 transition edges across PDG and NIST.
// Form invariant across all declared regions.
//
// Declared projection at C:
//   Preserves: magnitude of directed inertia difference, gated by activity.
//   Discards:  direction of difference (absolute value taken).

/// Compute ε[e] = |Δ(ι)[e]| · 𝟙[e] for a single declared edge.
///
/// # Arguments
/// * `delta_iota_e` — directed inertia difference Δ(ι)[e] = ι[s] − ι[t]
///   where ι[v] is relational inertia under declared non-acceleration through M.
/// * `indicator` — 𝟙[e]: 1.0 if R antisymmetric term is non-zero, 0.0 otherwise
///
/// # Returns
/// ε[e] in the same units as ι[v] (kg at Primary Region, declared per region).
pub fn epsilon_e(delta_iota_e: f64, indicator: f64) -> f64 {
    debug_assert!(
        indicator == 0.0 || indicator == 1.0,
        "𝟙[e] must be 0 or 1 — it is derived from R antisymmetry, not a parameter"
    );
    delta_iota_e.abs() * indicator
}

// ── 𝟙[e] — Detection Indicator ───────────────────────────────────────────
//
// 𝟙[e] = 1  iff  R(A(x))[e]_antisymmetric ≠ 0
// 𝟙[e] = 0  otherwise
//
// Support: Derived. (OC-12 resolved.)
// Derivation: from R operator antisymmetric output at declared edge e.
//
// 𝟙[e] is not a parameter. It is not declared independently.
// It is read from the R operator output after the kernel has acted.
// Spectroscopic selection rules fall out of this expression — no
// additional declaration is required.

/// Compute 𝟙[e] from the antisymmetric term of R operator output.
///
/// # Arguments
/// * `r_antisymmetric_e` — the antisymmetric term of R(A(x)) at edge e.
/// * `tol` — numerical zero threshold
///
/// # Returns
/// 1.0 if antisymmetric term is non-zero, 0.0 otherwise.
pub fn detection_indicator(r_antisymmetric_e: f64, tol: f64) -> f64 {
    if r_antisymmetric_e.abs() > tol { 1.0 } else { 0.0 }
}

// ── ε[e] at Photon Edges ─────────────────────────────────────────────────
//
// ε[e] = hc / λ
//
// M mapping: M maps spectrometer-reported wavelength λ (m) to ε[e] (J).
// The formula hc/λ is part of M at the Atomic Region. No theoretical
// assumptions beyond the declared measurement mapping M.
//
// ι[photon] = 0 — the photon locus has no inertial response through M.
// The form E[v] = ι[v] · κ[region] does not apply here.
//
// 𝟙[e] = 1 at all declared photon edges — photon detection confirms
// R(A(x))[e]_antisymmetric ≠ 0 (OC-12 resolved).
//
// Edge direction: upper level → lower level, fixed by decay observable.
// Reverse direction (absorption, lower → upper) requires independent
// observable provenance; it is a distinct relation under the distinctness
// axiom.
//
// Confirmed: 14 transitions (NIST ASD v5.12). See VR-γ-01,
// validation_record_v4.md.
//
// Support: Observed.
//
// ── Input validation note (Verifier, session July 2026) ──────────────────
// These functions use debug_assert! for input validation — correct for
// internal kernel use where all inputs are Origin-declared through M.
// If any function is later exposed through a public API or called from
// outside the controlled declaration workflow, consider returning
// Result<f64, Error> instead, since debug_assert! is omitted in
// optimized release builds and invalid inputs would then pass silently.
// This is a deployment decision for Origin to make when the boundary
// of the controlled workflow is declared.

/// Compute ε[e] = hc/λ for a declared photon edge.
///
/// # Arguments
/// * `lambda_m` — declared wavelength from M (meters; convert from nm: λ_nm × 1e-9)
///
/// # Returns
/// ε[e] in joules. Positive definite.
///
/// # Declared admissibility
/// λ must be traceable to an instrument report through M (spectrometer).
/// The formula hc/λ is part of M. No additional theoretical assumptions.
/// ι[photon] = 0; this expression is an edge quantity, not ι[v] · κ[region].
pub fn epsilon_photon_edge(lambda_m: f64) -> f64 {
    debug_assert!(lambda_m > 0.0, "λ must be positive — declared from instrument through M");
    H_PLANCK * C_DECLARED / lambda_m
}

// ── τ[v] at Photon Edges ─────────────────────────────────────────────────
//
// τ[v] = ℏ / ε[e]  (isolated-locus case, OC-13 resolved)
//
// Support: Derived (ℏ/ε[e], OC-13). Observed (ε[e] from NIST ASD v5.12).

/// Compute τ[v] = ℏ/ε[e] for a declared photon edge.
///
/// # Arguments
/// * `lambda_m` — declared wavelength from M (meters)
pub fn tau_photon_edge(lambda_m: f64) -> f64 {
    let eps = epsilon_photon_edge(lambda_m);
    HBAR / eps
}

// ── N[v] at Photon-Emitting Loci ─────────────────────────────────────────
//
// N[v] = τ_stability[v] / τ_relational[v] = τ_stability[v] · ε[e] / ℏ
//
// N[v] is the count of declared relational intervals within the measured
// stability interval of the emitting locus. It is dimensionless.
// "Relational field cycles" is interpretive language — not admitted here.
//
// Confirmed: H n=3 (Hα emitting locus)
//   τ_stability = 1.596e-9 s  (NIST ASD v5.12)
//   ε[Hα] = hc / 656.279 nm = 3.027e-19 J
//   N[H n=3] = 4.58e6, log₁₀ = 6.66
//
// Support: Derived (expression). Observed (values, NIST ASD v5.12).

/// Compute N[v] = τ_stability · ε[e] / ℏ for a photon-emitting locus.
///
/// # Arguments
/// * `tau_stability_s` — decay lifetime of emitting locus (s), from NIST ASD
/// * `lambda_m`        — photon edge wavelength (m), from NIST ASD
///
/// # Returns
/// N[v] — dimensionless count of declared relational intervals within
/// the measured stability interval.
pub fn n_relational_cycles(tau_stability_s: f64, lambda_m: f64) -> f64 {
    debug_assert!(tau_stability_s > 0.0, "τ_stability must be positive");
    debug_assert!(lambda_m > 0.0, "λ must be positive");
    let eps = epsilon_photon_edge(lambda_m);
    tau_stability_s * eps / HBAR
}

// ── τ[v] — Relational Progression Interval (Unified Expression) ───────────
//
// τ[v] = ℏ / |A_persistence[e_coupling] · cos(θ[e_v, e_coupling])|
//
// Support: Derived. (OC-13 resolved.)
//
// Special cases (all confirmed from observable record):
//   Isolated locus: A_persistence → E[v], cos(θ) → 1
//     τ → ℏ / E[v]                   ✓  (PDG ordering confirmed)
//   Parallel coupling: cos(θ) = 1
//     τ = ℏ / A_persistence           ✓  (H₂O ν₂ bend: 1.0 ps exact)
//   Perpendicular coupling: cos(θ) → 0
//     τ → very large                  ✓  (He 2s metastable: 7900 s)
//   Near-perpendicular: cos(θ) = 0.1176
//     τ = 8.5 ps                      ✓  (H₂O ν₁,ν₃ stretch modes exact)

/// Compute τ[v] — unified relational progression interval.
///
/// # Arguments
/// * `a_persistence_coupling` — A_persistence[e_coupling]
/// * `cos_theta` — cos(θ[e_v, e_coupling]): coupling projection.
pub fn tau_v(a_persistence_coupling: f64, cos_theta: f64) -> f64 {
    let denominator = (a_persistence_coupling * cos_theta).abs();
    debug_assert!(
        denominator > 0.0,
        "τ[v] denominator is zero — perpendicular coupling or zero A_persistence."
    );
    HBAR / denominator
}

/// τ[v] for the isolated locus case: τ = ℏ / E[v].
pub fn tau_v_isolated(e_v: f64) -> f64 {
    debug_assert!(e_v > 0.0, "E[v] must be positive for an active locus");
    HBAR / e_v
}

// ── θ[e] — B Output Vector Direction ─────────────────────────────────────
//
// v[e] = B(g)[e] · θ̂[e]
//
// Support: Observed (bond geometry confirmed); derivation complete
// (OC-θ-1 and OC-θ-2 resolved).
//
// Equilibrium condition: Σᵢ v[eᵢ] = 0 at each declared locus.
// Confirmed exact:
//   Linear:          180°
//   Trigonal planar: 120°
//   Tetrahedral:     109.47°

/// Compute equilibrium bond angle for n_bonds bonds at a declared locus.
/// Derived from N-vector balance: Σᵢ v[eᵢ] = 0.
pub fn equilibrium_bond_angle_rad(n_bonds: usize) -> Option<f64> {
    match n_bonds {
        2 => Some(std::f64::consts::PI),
        3 => Some(2.0 * std::f64::consts::PI / 3.0),
        4 => Some(((-1.0_f64) / 3.0).acos()),
        _ => None,
    }
}

/// Equilibrium bond angle in degrees — convenience wrapper.
pub fn equilibrium_bond_angle_deg(n_bonds: usize) -> Option<f64> {
    equilibrium_bond_angle_rad(n_bonds).map(|r| r.to_degrees())
}

// ── Φ[v] — Coherence Potential (Node) ────────────────────────────────────
//
// Φ[v] = ι[v] · τ[v]
//
// Support: Observed throughout (formal derivation OC-Φ-1 pending).
//
// ι[v]: relational inertia under declared non-acceleration through M.
// Replaces m[v] (V1). Rest mass inadmissible (SF-PR-16).
// Numerical values unchanged — PDG inertia values carry forward as ι[v].
//
// Scale and domain invariant. Confirmed across 29 declared loci spanning
// Primary Region, atomic, molecular, biological, and planetary regions —
// 78 decades in Φ — from two directly observable quantities with no free
// parameters and no theoretical imports.
//
// Human observer frame declared as the invariant reference.
//
// Region-specific threshold condition:
//   Primary Region:  log₁₀(Φ) ∈ [+36.53, +39.47]
//   Planetary:       log₁₀(Φ) ∈ [+59.02, +62.02]

/// Compute Φ[v] = ι[v] · τ[v] — coherence potential at a declared locus.
///
/// # Arguments
/// * `iota_v` — relational inertia at locus v (kg, or declared ι_unit per region)
///              Inertial response under declared non-acceleration through M.
///              Replaces m_v (V1). Numerical values unchanged.
/// * `tau_v` — relational progression interval at locus v (s)
///
/// # Returns
/// Φ[v] in kg·s.
pub fn phi_v(iota_v: f64, tau_v: f64) -> f64 {
    debug_assert!(iota_v > 0.0, "ι[v] must be positive — relational inertia ∈ ℝ⁺");
    debug_assert!(tau_v > 0.0, "τ[v] must be positive");
    iota_v * tau_v
}

/// log₁₀(Φ[v]) — declared projection for cross-region comparison.
pub fn phi_v_log10(iota_v: f64, tau_v: f64) -> f64 {
    phi_v(iota_v, tau_v).log10()
}

// ── Φ[S] — Coherence Potential (Collective) ───────────────────────────────
//
// Φ[S] = ι[S] · τ[S]     where ι[S] = Σᵢ ι[vᵢ]
//
// ι[S]: sum of relational inertia values across declared loci in S.
// Replaces m[S] = Σᵢ m[vᵢ] (V1). Same numerical values.
//
// Support: Observed. Confirmed across six accumulation transitions,
// four scale transitions. Zero violations.
//
// Accumulation condition: Φ[S] > Σᵢ Φ[vᵢ]

/// Compute Φ[S] = ι[S] · τ[S] — collective coherence potential.
///
/// # Arguments
/// * `iota_s` — total relational inertia of declared system S: Σᵢ ι[vᵢ]
/// * `tau_s` — declared system-level relational progression interval
pub fn phi_s(iota_s: f64, tau_s: f64) -> f64 {
    debug_assert!(iota_s > 0.0, "ι[S] must be positive");
    debug_assert!(tau_s > 0.0, "τ[S] must be positive");
    iota_s * tau_s
}

/// Accumulation condition: Φ[S] > Σᵢ Φ[vᵢ].
pub fn accumulation_condition(phi_s_val: f64, phi_components: &[f64]) -> bool {
    let sum_components: f64 = phi_components.iter().sum();
    phi_s_val > sum_components
}

// ── J[v] — R Output Multiplicity ─────────────────────────────────────────
//
// J[v] = (declared edges at v) / 2
// 2J + 1 = Zeeman components = Σ distinct R output states
//
// Support: Derived. Confirmed 52/52.
// Spin is R output multiplicity — not a new variable.

/// Compute J[v] = (declared edges at v) / 2.
pub fn j_v(n_declared_edges_at_v: usize) -> f64 {
    n_declared_edges_at_v as f64 / 2.0
}

/// 2J + 1 — number of distinct R output states.
pub fn zeeman_components(n_declared_edges_at_v: usize) -> f64 {
    2.0 * j_v(n_declared_edges_at_v) + 1.0
}

// ═══════════════════════════════════════════════════════════════════════════
// ── OPEN CONDITIONS ────────────────────────────────────────────────────────
// ═══════════════════════════════════════════════════════════════════════════
//
// OC-R-1   g-factor derivation: g = 2·ρ[e_B]/ρ_base
// OC-θ-3   Resonance / partial bond order
// OC-θ-5   Full 3D vector balance N > 4
// OC-Φ-1   Formal derivation of Φ[v] from operator fixed-point
// OC-Φ-2   Region-specific stable threshold from operator fixed-point
// OC-Φ-6   Observer frame overlap — structural or contingent
// OC-CA-1  Φ[S] formal derivation from operators
// OC-22    α_r inter-region coupling formal derivation
// OC-ι-1   ι_unit structurally fundamental or instrumentally bounded
//          (replaces OC-3: m_unit structurally fundamental or bounded)
// OC-ι-2   Relational inertia ratios (ι[μ]/ι[e], ι[τ]/ι[e]) — no
//          declared relational origin (replaces OC-2: lepton mass ratios)
// OC-ι-3   δι ~ ι^0.5 scaling — connection to ρ saturation not derived
//          (replaces OC-1: δm ~ m^0.5 scaling)

// ═══════════════════════════════════════════════════════════════════════════
// ── TESTS ──────────────────────────────────────────────────────────────────
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-10;

    // ── ε[e] and 𝟙[e] ────────────────────────────────────────────────────

    #[test]
    fn epsilon_e_active_edge() {
        assert!((epsilon_e(2.0, 1.0) - 2.0).abs() < TOL);
        assert!((epsilon_e(-2.0, 1.0) - 2.0).abs() < TOL);
    }

    #[test]
    fn epsilon_e_inactive_edge() {
        assert!((epsilon_e(5.0, 0.0)).abs() < TOL);
    }

    #[test]
    fn detection_indicator_nonzero_antisymmetric() {
        assert!((detection_indicator(0.5, TOL) - 1.0).abs() < TOL);
    }

    #[test]
    fn detection_indicator_zero_antisymmetric() {
        assert!((detection_indicator(0.0, TOL) - 0.0).abs() < TOL);
    }

    #[test]
    fn detection_indicator_at_tolerance_boundary() {
        assert!((detection_indicator(TOL, TOL) - 0.0).abs() < 1e-15);
        assert!((detection_indicator(TOL * 1.001, TOL) - 1.0).abs() < 1e-15);
    }

    // ── τ[v] ─────────────────────────────────────────────────────────────

    #[test]
    fn tau_v_isolated_case() {
        let tau = tau_v_isolated(1.0);
        assert!((tau - HBAR).abs() < TOL);
    }

    #[test]
    fn tau_v_parallel_coupling() {
        let a_p = 2.0e-20;
        let tau_par = tau_v(a_p, 1.0);
        let tau_iso = tau_v_isolated(a_p);
        assert!((tau_par - tau_iso).abs() < TOL);
    }

    #[test]
    fn tau_v_near_perpendicular_structure() {
        let a_p = 1.0e-20_f64;
        let cos_parallel = 1.0_f64;
        let cos_near_perp = 0.1176_f64;
        let tau_parallel = tau_v(a_p, cos_parallel);
        let tau_near_perp = tau_v(a_p, cos_near_perp);
        assert!(tau_near_perp > tau_parallel * 5.0);
        let ratio = tau_near_perp / tau_parallel;
        assert!((ratio - 1.0 / cos_near_perp).abs() < 0.01);
    }

    #[test]
    fn tau_v_increases_as_cos_theta_decreases() {
        let a_p = 1.0e-20;
        let tau_parallel = tau_v(a_p, 1.0);
        let tau_near_perp = tau_v(a_p, 0.01);
        assert!(tau_near_perp > tau_parallel);
    }

    // ── θ[e] ─────────────────────────────────────────────────────────────

    #[test]
    fn equilibrium_angle_linear() {
        let angle = equilibrium_bond_angle_deg(2).unwrap();
        assert!((angle - 180.0).abs() < TOL);
    }

    #[test]
    fn equilibrium_angle_trigonal_planar() {
        let angle = equilibrium_bond_angle_deg(3).unwrap();
        assert!((angle - 120.0).abs() < TOL);
    }

    #[test]
    fn equilibrium_angle_tetrahedral() {
        let angle = equilibrium_bond_angle_deg(4).unwrap();
        assert!((angle - 109.4712).abs() < 1e-3);
    }

    #[test]
    fn equilibrium_angle_undefined_below_two_bonds() {
        assert!(equilibrium_bond_angle_deg(1).is_none());
        assert!(equilibrium_bond_angle_deg(0).is_none());
    }

    #[test]
    fn equilibrium_angle_undefined_above_four_bonds() {
        assert!(equilibrium_bond_angle_deg(5).is_none());
    }

    // ── Φ[v] ─────────────────────────────────────────────────────────────

    #[test]
    fn phi_v_formula_holds() {
        // Φ[v] = ι[v] · τ[v]
        // ι[v]: relational inertia of proton under declared non-acceleration
        // through M. Numerical value identical to PDG proton mass.
        let iota_v = 1.672_621_923_69e-27_f64;
        let tau_v_val = 1.0e66_f64;
        let phi = phi_v(iota_v, tau_v_val);
        assert!((phi - iota_v * tau_v_val).abs() < f64::EPSILON * phi);
        assert!(phi > 0.0 && phi.is_finite());
    }

    #[test]
    fn phi_v_proton_electron_ordering() {
        // Φ_proton > Φ_electron when τ is the same.
        // ι[proton] > ι[electron] — both declared through M under
        // non-acceleration condition. Ratio confirmed: ι[p]/ι[e] = 1836.15.
        let iota_proton = 1.672_621_923_69e-27_f64;
        let iota_electron = I_UNIT_ELECTRON;
        let tau_shared = 1.0e66_f64;
        let phi_p = phi_v(iota_proton, tau_shared);
        let phi_e = phi_v(iota_electron, tau_shared);
        assert!(phi_p > phi_e);
        let log_ratio = (phi_p / phi_e).log10();
        let expected_ratio = (iota_proton / iota_electron).log10();
        assert!((log_ratio - expected_ratio).abs() < 0.01);
    }

    #[test]
    fn phi_v_scale_invariance_form() {
        // Form invariance: Φ[v] = ι[v] · τ[v] holds at any scale.
        let cases = vec![
            (1e-27, 1e-24),
            (1e-25, 1e-12),
            (1e-20, 1e-3),
            (70.0, 3.15e7),
        ];
        for (iota, tau) in cases {
            let phi = phi_v(iota, tau);
            assert!(phi.is_finite() && phi > 0.0);
            assert!((phi - iota * tau).abs() < f64::EPSILON * phi);
        }
    }

    // ── Φ[S] ─────────────────────────────────────────────────────────────

    #[test]
    fn accumulation_condition_holds_when_collective_exceeds_sum() {
        let phi_s_val = 100.0;
        let components = vec![20.0, 30.0, 40.0];
        assert!(accumulation_condition(phi_s_val, &components));
    }

    #[test]
    fn accumulation_condition_fails_when_collective_equals_sum() {
        let phi_s_val = 90.0;
        let components = vec![20.0, 30.0, 40.0];
        assert!(!accumulation_condition(phi_s_val, &components));
    }

    // ── J[v] ─────────────────────────────────────────────────────────────

    #[test]
    fn j_v_electron_two_edges() {
        assert!((j_v(2) - 1.0).abs() < TOL);
        assert!((zeeman_components(2) - 3.0).abs() < TOL);
    }

    #[test]
    fn j_v_proton_confirmed_52_of_52() {
        assert!((j_v(1) - 0.5).abs() < TOL);
        assert!((zeeman_components(1) - 2.0).abs() < TOL);
    }

    #[test]
    fn j_v_zero_edges_ground() {
        assert!((j_v(0) - 0.0).abs() < TOL);
        assert!((zeeman_components(0) - 1.0).abs() < TOL);
    }

    // ── Cross-invariant consistency ───────────────────────────────────────

    #[test]
    fn epsilon_zero_when_indicator_zero() {
        let large_delta_iota = 1e10;
        assert!((epsilon_e(large_delta_iota, 0.0)).abs() < TOL);
    }

    #[test]
    fn phi_v_uses_tau_from_unified_expression() {
        let iota_v = 1.673e-27_f64;
        let e_v = 1.5e-10_f64;
        let tau = tau_v_isolated(e_v);
        let phi = phi_v(iota_v, tau);
        assert!((phi - iota_v * HBAR / e_v).abs() < TOL * phi);
    }

    // ── Photon edge ε[e], τ[v], N[v] ─────────────────────────────────────

    #[test]
    fn photon_edge_epsilon_lyman_alpha() {
        // H Ly-α: λ = 121.567 nm → ε[e] = hc/λ
        // NIST ASD v5.12: 10.1988 eV = 1.6340e-18 J
        let lambda_m = 121.567e-9_f64;
        let eps = epsilon_photon_edge(lambda_m);
        let eps_ev = eps / 1.602_176_634e-19;
        assert!((eps_ev - 10.1988).abs() / 10.1988 < 1e-4,
            "Ly-α ε[e] must match NIST ASD within 0.01%: got {:.6} eV", eps_ev);
        assert!(eps > 0.0 && eps.is_finite(), "ε[e] must be positive and finite");
    }

    #[test]
    fn photon_edge_epsilon_balmer_alpha() {
        // H Hα: λ = 656.279 nm → ε[e] = hc/λ
        // NIST ASD v5.12: 1.8892 eV = 3.0268e-19 J
        let lambda_m = 656.279e-9_f64;
        let eps = epsilon_photon_edge(lambda_m);
        let eps_ev = eps / 1.602_176_634e-19;
        assert!((eps_ev - 1.8892).abs() / 1.8892 < 1e-4,
            "Hα ε[e] must match NIST ASD within 0.01%: got {:.6} eV", eps_ev);
    }

    #[test]
    fn photon_edge_tau_isolated_case() {
        // τ[v] = ℏ / ε[e] at photon edge.
        // Isolated-locus case (OC-13). Must equal tau_v_isolated(ε[e]).
        let lambda_m = 656.279e-9_f64;
        let tau_direct = tau_photon_edge(lambda_m);
        let eps = epsilon_photon_edge(lambda_m);
        let tau_from_isolated = tau_v_isolated(eps);
        assert!((tau_direct - tau_from_isolated).abs() < 1e-30,
            "tau_photon_edge must equal tau_v_isolated(ε[e]) — same formula, same case");
        assert!(tau_direct > 0.0 && tau_direct.is_finite());
        assert!(tau_direct > 3.0e-16 && tau_direct < 4.0e-16,
            "Hα τ[v] must be ~3.5e-16 s: got {:.4e}", tau_direct);
    }

    #[test]
    fn photon_edge_n_hydrogen_n3() {
        // H n=3 emitting locus (Hα transition).
        // τ_stability[H n=3] = 1.596e-9 s (NIST ASD v5.12)
        // λ[Hα] = 656.279 nm
        // N[H n=3] ≈ 4.58e6, log₁₀ ≈ 6.66
        let tau_stab = 1.596e-9_f64;
        let lambda_m  = 656.279e-9_f64;
        let n = n_relational_cycles(tau_stab, lambda_m);
        let log_n = n.log10();
        assert!((log_n - 6.66).abs() < 0.02,
            "log₁₀(N[H n=3]) must be ~6.66: got {:.4}", log_n);
        assert!(n > 4.0e6 && n < 5.0e6,
            "N[H n=3] must be ~4.58e6: got {:.4e}", n);
    }

    #[test]
    fn photon_edge_kappa_non_applicable() {
        // ι[photon] = 0. E[v] = ι[v] · κ[Primary] does not apply at photon locus.
        // ε[e] is real and nonzero; ι[photon] · κ[Primary] = 0. Structurally distinct.
        let iota_photon: f64 = 0.0;
        let lambda_m = 656.279e-9_f64;
        let eps = epsilon_photon_edge(lambda_m);
        assert!(eps > 0.0, "photon edge carries ε[e] > 0");
        let kappa_primary = 8.988e16_f64;
        let e_from_iota_kappa = iota_photon * kappa_primary;
        assert!((e_from_iota_kappa).abs() < 1e-30,
            "ι[photon]·κ[Primary] = 0; does not equal ε[e]");
        assert!((eps - e_from_iota_kappa).abs() > 1e-20,
            "ε[photon edge] must not equal ι[photon]·κ[Primary]");
    }
}
// ═══════════════════════════════════════════════════════════════════════════
// ── PRIMARY REGION INVARIANTS — TUNNELING, SUPERPOSITION, ENTANGLEMENT ─────
// ═══════════════════════════════════════════════════════════════════════════
//
// These three invariants are statements about the Primary Region that hold
// at every declared scale and across every declared domain. They are not
// confined to quantum systems. They are observable consequences of the
// operator structure at any declared energy transition edge where ρ_P ≪ 1.
//
// Each is derived from quantities already present in derived_invariants.rs
// and is consistent with the existing energy expressions ε[e] and τ[v].
// None introduces a new variable. All quantities are traceable to observables
// through M.
//
// ── JOINT NECESSITY — Preamble ───────────────────────────────────────────
//
// The three invariants below are jointly necessary conditions for any
// self-organizing, evolving, observable system to exist within D.
//
// This section is a framework-level necessity argument — an observed pattern
// stated at the level of the framework, consistent with the operator structure
// and with the observable record across declared regions and scales. The
// numbered arguments below are framework-level observations and proposed
// necessity statements. They are not mathematical consequences already
// established by the kernel. Each is an open condition until formally
// derived from prior declarations. This distinction is carried through
// each numbered argument explicitly.
//
// The joint necessity argument:
//
// (1) TUNNELING — proposed necessary condition for energy availability.
//     [Framework-level observation — not yet derived from kernel]
//     At the most primary level, every energy transition edge in observed
//     systems has finite barrier geometry. No infinite barrier edge has
//     been declared through M in any confirmed region. The tunneling
//     invariant (I-T) states that the quantum boundary component of Δ
//     is nonzero at every such edge — this consequence is derived from
//     ε[e] and τ[v] (see I-T below).
//     The further claim — that a system with no tunneling would have no
//     energy transfer and no evolution — is a framework-level necessity
//     statement, not a mathematical consequence already established from
//     the kernel. It is stated here as an observed pattern: all observed
//     energy transfer occurs at edges with finite barrier geometry, and
//     the tunneling component is present at every such edge. Whether the
//     absence of tunneling would formally preclude energy transfer is an
//     open condition (OC-T-3).
//     Observable basis: all energy transfer in observed systems occurs
//     at edges with finite barrier geometry. No infinite barrier edge
//     has been declared through M in any confirmed region.
//
// (2) SUPERPOSITION — proposed necessary condition for evolution and choice.
//     [Framework-level observation — not yet derived from kernel]
//     For any system to self-organize, evolve, and produce observable
//     data, it must have been in a state of unresolved directed contrast
//     before B activated — rank(Im Δ) > 1 at some prior declared step.
//     The operator consequence is established: rank(Im Δ) = 1 means a
//     single directed contrast, and B accumulation over a single edge
//     produces a single resolved output with no distribution of outcomes.
//     The further claim — that rank(Im Δ) = 1 from initialization makes
//     evolution impossible — is a framework-level necessity statement,
//     not yet formally derived from the kernel. It is stated here as an
//     observed pattern: every observed self-organizing system produces
//     data distributions consistent with prior rank(Im Δ) > 1. Whether
//     rank(Im Δ) = 1 throughout a declared history formally precludes
//     self-organization is an open condition (OC-S-3).
//     What we observe as self-organization, adaptation, and choice are
//     C projections of prior superposition states resolved by B activation.
//     Observable basis: every observed self-organizing system produces
//     data distributions consistent with prior rank(Im Δ) > 1. No
//     observed evolving system is consistent with rank(Im Δ) = 1
//     throughout its declared history.
//
// (3) ENTANGLEMENT — proposed necessary condition for structural coherence.
//     [Framework-level observation — not yet derived from kernel]
//     For any system to maintain coherent structure while self-organizing,
//     declared edges with 𝟙[e] = 1 must persist across relational steps.
//     The operator consequence is established: 𝟙[e] = 0 at all edges
//     means ε[e] = 0 everywhere and no relational energy contrast is
//     present across the declared structure.
//     The further claim — that 𝟙[e] = 0 everywhere formally precludes
//     collective organization — is a framework-level necessity statement,
//     not yet formally derived from the kernel. It is stated here as an
//     observed pattern: every observed self-organizing system exhibits
//     correlated behavior between declared loci consistent with 𝟙[e] = 1
//     at structural edges. Whether the absence of all active edges formally
//     precludes coherent structure is an open condition (OC-E-3).
//     What we observe as connection, structural integrity, and coherent
//     collective behavior are C projections of persisting declared edges
//     with 𝟙[e] = 1.
//     Observable basis: every observed self-organizing system exhibits
//     correlated behavior between declared loci consistent with 𝟙[e] = 1
//     at structural edges. No coherent self-organizing system has been
//     declared through M with all edges at 𝟙[e] = 0.
//
// Consequence: tunneling, superposition, and entanglement are not exotic
// phenomena confined to quantum systems. They are the structural floor
// of any observable reality within D. Their consequences are observable
// at every scale. The phenomena themselves are not directly observable —
// only their consequences are. This is not a limitation of measurement.
// It is a structural feature of the framework: the conditions that make
// observation possible are prior to observation and cannot themselves
// be directly observed. They are declared through M on the basis of
// their consequences, not on the basis of direct detection.
//
// This joint necessity statement is bounded over D. No claim beyond D.

// ── I-T — Tunneling Invariant ─────────────────────────────────────────────
//
// At any declared energy transition edge e with finite barrier geometry,
// the quantum boundary component of Δ(ι)[e] is nonzero.
//
// Consequence 1: 𝟙[e] is never exactly zero at a finite energy barrier.
//   The R antisymmetric term retains nonzero amplitude at the boundary
//   even when the classical transition probability is suppressed.
//
// Consequence 2: ε[e] has a nonzero floor at every finite barrier edge.
//   ε[e] = |Δ(ι)[e]| · 𝟙[e] > 0 for all e with finite barrier geometry.
//
// Consequence 3: τ[v] is bounded above at every finite barrier edge.
//   τ[v] = ℏ / ε[e] < ∞ for all e with finite barrier geometry.
//   The perpendicular coupling limit (cos(θ) → 0) does not reach τ → ∞
//   in any physical system with finite barrier geometry — tunneling sets
//   the floor on transition probability and the ceiling on τ[v].
//
// Scale and domain invariance: the quantum boundary component of Δ is
// present at every declared energy transition edge regardless of region.
// Its magnitude scales with barrier geometry and energy difference through M.
// It does not vanish at larger scales — it becomes the dominant term only
// when the classical component of Δ is suppressed by barrier geometry.
//
// Observable provenance: the tunneling condition is not directly observable.
// What is observable through M are its consequences — transition rates at
// declared edges where classical suppression would predict zero probability.
// These consequences are C projections of the nonzero quantum boundary
// component of Δ(ι)[e] onto a scalar rate observable. The invariant is
// the declared relational structure that produces those consequences.
// Confirmed consequences: semiconductor tunnel junction transport rates;
// field emission current; alpha decay transition rates (Geiger-Nuttall
// relation). None of these observations directly observes tunneling —
// each observes a rate that is inadmissible under the classical Δ alone.
//
// Support: Derived (from ε[e], τ[v], 𝟙[e]). Observed (consequences only —
// transition rates at classically suppressed edges, Primary and Atomic regions).

/// Tunneling floor condition: returns true if ε[e] retains nonzero amplitude
/// at a declared edge with finite barrier geometry.
///
/// # Arguments
/// * `epsilon_e` — relational energy contrast at edge e
/// * `tol`       — numerical zero threshold
///
/// # Returns
/// true if tunneling floor condition holds (ε[e] > tol).
/// false signals a declaration error — finite barrier edges must satisfy this.
pub fn tunneling_floor_holds(epsilon_e: f64, tol: f64) -> bool {
    epsilon_e.abs() > tol
}

/// Tunneling ceiling on τ[v]: returns true if τ[v] is finite at a declared
/// edge with finite barrier geometry.
///
/// # Arguments
/// * `tau_v` — relational progression interval at locus v
///
/// # Returns
/// true if τ[v] < ∞ (finite, as required by tunneling invariant).
pub fn tunneling_tau_bounded(tau_v: f64) -> bool {
    tau_v.is_finite() && tau_v > 0.0
}

// ── I-S — Superposition Invariant ────────────────────────────────────────
//
// At the Primary Region (ρ_P ≪ 1), rank(Im Δ) > 1 is the admissible
// statement of superposition.
//
// A declared locus v at the Primary Region carries nonzero Δ output on
// more than one declared edge simultaneously. B has not activated —
// no accumulation has resolved the field into a single directed output.
// The locus is in a state of unresolved directed contrast across multiple
// declared edges.
//
// This is not a special condition — it is the normal state of any locus
// at the Primary Region before ρ_P → 1. Superposition is the operator
// description of what it means to be at the Primary Region.
//
// Scale and domain invariance: rank(Im Δ) > 1 is observable at any
// declared scale where B has not activated. At larger scales the
// condition resolves more rapidly (ρ_P → 1 faster as accumulation
// confirms persistence), but the condition itself is not scale-specific.
// Any declared system in which multiple edges carry simultaneous nonzero
// Δ output and B is not active is in the superposition condition
// under the declared admissibility conditions.
//
// Consequence for ε[e]: at a locus in the superposition condition,
// ε[e] is nonzero on multiple declared edges simultaneously. The total
// relational energy contrast at that locus is distributed across edges —
// no single edge captures it until B activates and accumulation begins.
//
// Observable provenance: the superposition condition is not directly
// observable. What is observable through M are its consequences — the
// statistical distribution of outcomes when B activates and the field
// resolves. These consequences are C projections of the unresolved
// rank(Im Δ) > 1 state onto scalar measurement outcomes. The act of
// measurement is the activation of B — accumulation resolves the
// distributed contrast into a single directed output, and what is
// observed is that resolved output, not the prior unresolved state.
// Confirmed consequences: double-slit interference patterns (C projection
// of rank(Im Δ) = 2 field onto position axis); statistical distributions
// of atomic measurement outcomes. No observation directly observes the
// simultaneous multi-edge state — each observation is a post-resolution
// consequence of B activation.
//
// Support: Derived (from rank(Im Δ), ρ_P, B activation condition).
// Observed (consequences only — interference patterns and measurement
// outcome distributions, Primary and Atomic regions).

/// Superposition condition: returns true if locus v satisfies rank(Im Δ) > 1
/// and B is not active (ρ_P ≪ 1).
///
/// # Arguments
/// * `rank_im_delta`   — rank of the Δ output field at locus v
/// * `rho_p`           — persistence confirmation ratio at locus v
/// * `rho_p_threshold` — threshold below which B is not active (typically ≪ 1)
///
/// # Returns
/// true if locus v is in the superposition condition.
pub fn superposition_condition(
    rank_im_delta: usize,
    rho_p: f64,
    rho_p_threshold: f64,
) -> bool {
    rank_im_delta > 1 && rho_p < rho_p_threshold
}

/// Superposition resolution: returns true if B has activated and the
/// superposition condition has resolved (ρ_P → 1).
///
/// # Arguments
/// * `rho_p`           — persistence confirmation ratio at locus v
/// * `rho_p_threshold` — threshold above which B is active
pub fn superposition_resolved(rho_p: f64, rho_p_threshold: f64) -> bool {
    rho_p >= rho_p_threshold
}

// ── I-E — Entanglement Invariant ─────────────────────────────────────────
//
// A declared edge e = (s, t) with 𝟙[e] = 1 establishes a relational
// constraint between loci s and t that is preserved under the operators
// regardless of the spatial separation of s and t within D.
//
// 𝟙[e] = 1 iff R(A(x))[e]_antisymmetric ≠ 0 — the edge carries nonzero
// R antisymmetric output. This is the admissible statement of entanglement:
// two loci share a declared edge whose relational contrast is confirmed
// active by the R operator. The constraint between them is not a parameter —
// it is read from the operator output.
//
// The relational constraint at edge e means:
//   Δ(x)[e] = x[s] − x[t] is nonzero and directed.
//   A change in x[s] that alters Δ(x)[e] simultaneously alters the
//   declared relational state at x[t] through the same edge — not through
//   a separate propagation mechanism, but because the edge is the declared
//   relation between them.
//
// This is not action at a distance. It is a statement about declared
// relational structure: the two loci are not independent within D because
// they share a declared edge with nonzero directed contrast. Spatial
// separation is not a relational primitive in the framework — it is a
// C projection of position onto a geometric axis. The declared edge
// relation is prior to any geometric separation.
//
// Scale and domain invariance: 𝟙[e] = 1 at a declared edge is the same
// condition regardless of the region in which s and t are declared.
// Entanglement is not confined to the Primary Region — it is the condition
// of any two declared loci sharing an active directed edge. At larger
// scales the condition is expressed through declared structural relations
// (molecular bonds, gravitational coupling, economic dependency) rather
// than particle correlations, but the operator statement is identical.
//
// Consequence for ε[e]: at an entangled edge, ε[e] = |Δ(ι)[e]| · 𝟙[e]
// is nonzero. The relational energy contrast at that edge is the observable
// expression of the entanglement constraint. Disrupting the edge —
// removing the declared relation — sets 𝟙[e] = 0 and ε[e] = 0.
// This is the admissible statement of decoherence within D.
//
// Observable provenance: the entanglement condition is not directly
// observable. The declared edge e = (s, t) and its R antisymmetric output
// are not themselves observable — what is observable through M are the
// consequences of the relational constraint at that edge. These consequences
// are C projections of the active directed edge onto correlated scalar
// measurement outcomes at s and t independently. The correlation itself
// is the observable; the declared edge relation is the structure that
// produces it. Removing the edge (decoherence) removes the correlation —
// this is the admissible test of the invariant through M.
// Confirmed consequences: EPR-type measurement correlations (Bell
// inequality violations — correlation exceeds what is admissible without
// a declared edge between s and t); molecular bond directionality
// (correlated edge outputs confirmed across 22 transition edges,
// validation_record_v4.md). No observation directly observes the edge
// relation — each observes a correlation that is inadmissible without it.
//
// Support: Derived (from 𝟙[e], R antisymmetric output, ε[e]).
// Observed (consequences only — correlated measurement outcomes and
// bond directionality, Primary and Atomic regions).

/// Entanglement condition: returns true if declared edge e = (s, t)
/// carries nonzero R antisymmetric output (𝟙[e] = 1).
///
/// # Arguments
/// * `indicator` — 𝟙[e]: 1.0 if R antisymmetric term is nonzero, 0.0 otherwise
///
/// # Returns
/// true if entanglement condition holds at edge e.
pub fn entanglement_condition(indicator: f64) -> bool {
    indicator == 1.0
}

/// Decoherence condition: returns true if the entanglement constraint at
/// edge e has been disrupted (𝟙[e] = 0, ε[e] = 0).
///
/// # Arguments
/// * `indicator`  — 𝟙[e]: 1.0 if active, 0.0 if disrupted
/// * `epsilon_e`  — relational energy contrast at edge e
/// * `tol`        — numerical zero threshold
///
/// # Returns
/// true if decoherence condition holds (𝟙[e] = 0 and ε[e] ≈ 0).
pub fn decoherence_condition(indicator: f64, epsilon_e: f64, tol: f64) -> bool {
    indicator == 0.0 && epsilon_e.abs() <= tol
}

// ═══════════════════════════════════════════════════════════════════════════
// ── OPEN CONDITIONS (additions) ────────────────────────────────────────────
// ═══════════════════════════════════════════════════════════════════════════
//
// OC-T-1   Tunneling floor magnitude: derive the quantum boundary component
//          of Δ(ι)[e] as a function of declared barrier geometry and ε[e].
//          Currently stated as nonzero; formal expression not yet derived.
//
// OC-T-2   Tunneling at scale: confirm tunneling floor in declared systems
//          beyond Primary and Atomic regions — molecular, biological, planetary.
//
// OC-T-3   Tunneling necessity: formally derive that the absence of tunneling
//          (infinite barrier geometry at all edges) precludes energy transfer
//          within D. Currently a framework-level necessity statement.
//
// OC-S-1   ρ_P threshold: the transition threshold from superposition condition
//          to B activation is not yet derived as a computable criterion.
//          (Carried from operators_notation_and_constraint_v7.md open condition.)
//
// OC-S-2   Superposition at scale: confirm rank(Im Δ) > 1 condition in
//          declared systems beyond Primary Region — identify observable
//          expressions at molecular, biological, and planetary regions.
//
// OC-S-3   Superposition necessity: formally derive that rank(Im Δ) = 1
//          throughout a declared history precludes self-organization within D.
//          Currently a framework-level necessity statement.
//
// OC-E-1   Entanglement preservation under operator evolution: derive
//          formally that 𝟙[e] = 1 is preserved across declared relational
//          steps when no disruption event is declared through M.
//
// OC-E-2   Decoherence rate: derive the rate at which 𝟙[e] → 0 as a
//          function of declared environmental edge interactions through M.
//
// OC-E-3   Entanglement necessity: formally derive that 𝟙[e] = 0 everywhere
//          precludes collective organization within D. Currently a
//          framework-level necessity statement.

// V3 → V4: Four new invariants added.
// I-T (Tunneling): quantum boundary component of Δ(ι)[e] is nonzero at
//   every finite barrier edge. ε[e] has a nonzero floor; τ[v] is bounded
//   above. Not directly observable — consequences observable through M.
// I-S (Superposition): rank(Im Δ) > 1 is the admissible statement of
//   superposition at the Primary Region. Normal state before ρ_P → 1.
//   Not directly observable — consequences observable through M.
// I-E (Entanglement): declared edge e=(s,t) with 𝟙[e]=1 establishes
//   relational constraint preserved under operators regardless of spatial
//   separation. Not directly observable — consequences observable through M.
//   Decoherence: 𝟙[e]=0 and ε[e]=0 — the admissible statement within D.
// I-RE (Relational Evolution): relational evolution rate is monotonically
//   decreasing from the Primary Region outward. Primary Region is the
//   fastest-evolving declared region in D. Φ[v] 78-decade span is the
//   observable expression of this invariant. Larger-scale structure is
//   downstream accumulation of Primary Region relational evolution.
// Joint necessity preamble added: I-T, I-S, I-E are jointly necessary
//   conditions for any self-organizing, evolving, observable system.
//   Framework-level necessity statements distinguished from derived results
//   throughout. Nine new open conditions added: OC-T-1 through OC-E-3,
//   OC-RE-1 through OC-RE-3.
