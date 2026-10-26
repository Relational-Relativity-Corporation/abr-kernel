//! Kernel V9 admission — admissible calculations and OPEN scope.
//!
//! Companion to tests/v9_conformance.rs §8 (the four RPPI failure modes).
//! Tags:
//!   [V9-ADMIT]  a correctly declared calculation is admitted only after
//!               resolution, independent reconstruction, and reverse trace;
//!               values are checked against direct calls to the frozen
//!               functions. All admitted cases here are SYNTHETIC FIXTURES and
//!               are admitted as ObservationalStatus::Synthetic — they show that
//!               enforcement does not reject a correct declaration; they
//!               establish no empirical validity.
//!   [V9-OPEN]   absent observational basis, unsupported scope, unsupported M
//!               kind, extent, or parameters: OPEN, never REJECTED, never
//!               admitted.
//!   [V9-REJECT] kernel-selection contradiction.

mod common;

use abr_kernel::admission::*;
use abr_kernel::derived_invariants::epsilon_photon_edge;
use abr_kernel::operators::{operator_delta, operator_sigma, DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::provenance::*;
use abr_kernel::provenanced_exec::{execute_edge_m_projection, execute_primary};
use common::*;

fn admitted(o: AdmissionOutcome) -> AdmissionEvidence {
    match o {
        AdmissionOutcome::Admitted(e) => e,
        other => panic!("expected Admitted, got {other:?}"),
    }
}

fn termini_ids(e: &AdmissionEvidence, k: usize) -> Vec<String> {
    e.reverse_traces[k].termini.iter().map(|t| t.observable.clone()).collect()
}

// ── [V9-ADMIT] ────────────────────────────────────────────────────────────

/// [V9-ADMIT] Primary Δ over a declared relation; Σ not evaluated because
/// ρ_base and χ₀ are not declared through M.
#[test]
fn admit_primary_delta_sigma_not_evaluated() {
    let reg = synthetic_primary_registry();
    let po = execute_primary(&reg, SYN_DELTA).unwrap();
    let e = admitted(admit(&po, &reg));

    // Independent check against the frozen operator called directly.
    let rel = DeclaredRelations::from_edges_with_provenance(2, vec![(0, 1)], vec![EdgeProvenance::Continuation]);
    let direct = operator_delta(&NodeField::new(vec![vec![3.0, 1.0]]), &rel);
    assert_eq!(e.entries.len(), 1);
    assert_eq!(e.entries[0].value.to_bits(), direct.field[0][0].to_bits());

    assert_eq!(e.not_evaluated, vec![OpenCondition::NotEvaluated {
        step: Step::Sigma, missing: vec!["rho_base".into(), "chi_0".into()],
    }]);
    assert_eq!(e.observational_status, ObservationalStatus::Synthetic);
    assert_eq!(termini_ids(&e, 0), vec!["syn.obs.a", "syn.obs.b", SYN_BASIS]);
    assert!(e.reverse_traces[0].termini.iter().all(|t| t.origin_kind == OriginKind::SyntheticFixture
        && t.record.source == SYN_SOURCE));
    assert_eq!(e.comparison_rule, COMPARISON_RULE);
}

/// [V9-ADMIT] Primary Δ → Σ with ρ_base and χ₀ declared (synthetically).
#[test]
fn admit_primary_delta_and_sigma() {
    let reg = synthetic_primary_registry();
    let po = execute_primary(&reg, SYN_CHAIN).unwrap();
    let e = admitted(admit(&po, &reg));

    let rel = DeclaredRelations::from_edges_with_provenance(
        3, vec![(0, 1), (1, 2)], vec![EdgeProvenance::Continuation, EdgeProvenance::Continuation]);
    let d = operator_delta(&NodeField::new(vec![vec![3.0, 1.0, 2.0]]), &rel);
    let s = operator_sigma(&d, &rel, 0.5, 1.0);
    let got = |step: Step, r: &str| e.entries.iter().find(|x| x.step == step && x.relation == r).unwrap().value.to_bits();
    assert_eq!(got(Step::Delta, SYN_REL_01), d.field[0][0].to_bits());
    assert_eq!(got(Step::Delta, SYN_REL_12), d.field[0][1].to_bits());
    assert_eq!(got(Step::Sigma, SYN_REL_01), s.field[0][0].to_bits());
    assert_eq!(got(Step::Sigma, SYN_REL_12), s.field[0][1].to_bits());
    assert!(e.not_evaluated.is_empty());

    // Σ[syn.rel.01] reverse-traces through its successor and the parameters.
    let k = e.reverse_traces.iter().position(|t| t.entry == format!("Sigma[{SYN_REL_01}]")).unwrap();
    let ids = termini_ids(&e, k);
    for want in ["syn.obs.a", "syn.obs.b", "syn.obs.c", "syn.obs.rho_base", "syn.obs.chi_0"] {
        assert!(ids.contains(&want.to_string()), "{want} missing from {ids:?}");
    }
}

/// [V9-ADMIT] Edge M projection through the frozen ε[e] = hc/λ (synthetic λ).
#[test]
fn admit_photon_edge_m_projection_synthetic() {
    let reg = synthetic_photon_registry();
    let po = execute_edge_m_projection(&reg, SYN_PHOTON_EDGE).unwrap();
    let e = admitted(admit(&po, &reg));
    assert_eq!(e.entries[0].value.to_bits(), epsilon_photon_edge(500e-9).to_bits());
    assert_eq!(e.entries[0].step, Step::MProjection);
    assert_eq!(termini_ids(&e, 0), vec!["syn.obs.lambda", "syn.ref.c", "syn.ref.h"]);
    assert_eq!(e.observational_status, ObservationalStatus::Synthetic);
}

/// [V9-ADMIT] Independence: a record written directly — not produced by
/// provenanced_exec — is admitted when, and only because, it agrees with the
/// independent reconstruction.
#[test]
fn admit_record_not_authored_by_exec() {
    let reg = synthetic_primary_registry();
    let rel = DeclaredRelations::from_edges_with_provenance(2, vec![(0, 1)], vec![EdgeProvenance::Continuation]);
    let delta = operator_delta(&NodeField::new(vec![vec![3.0, 1.0]]), &rel);
    let v = delta.field[0][0];
    let input = |l: &str, o: &str, x: f64| InputRecord {
        binding_ref: l.into(), observable_ref: o.into(), claimed_source: SYN_SOURCE.into(),
        observed_value: x, m_mapping_ref: SYN_M.into(), projected_value: x,
    };
    let po = ProvenancedOutput {
        output: CalculatedOutput::Primary { delta, sigma: None },
        record: CalculationRecord {
            calculation: Calculation::Primary { structure_ref: SYN_DELTA.into() },
            inputs: vec![input("syn.locus.0", "syn.obs.a", 3.0), input("syn.locus.1", "syn.obs.b", 1.0)],
            relations: vec![RelationRecord {
                relation_ref: SYN_REL_01.into(), src_index: 0, tgt_index: 1,
                class: EdgeProvenance::Continuation, adj_plus: vec![], adj_minus: vec![], rho_incident: vec![],
            }],
            path: vec![Step::Delta],
            parameters: vec![],
            outputs: vec![OutputEntry {
                step: Step::Delta, relation_ref: SYN_REL_01.into(), component: 0, value: v,
                contributing: vec!["syn.locus.0".into(), "syn.locus.1".into(), SYN_REL_01.into()],
            }],
        },
    };
    assert!(matches!(admit(&po, &reg), AdmissionOutcome::Admitted(_)));
}

// ── [V9-OPEN] ─────────────────────────────────────────────────────────────

/// [V9-OPEN] Each declaration element without its observational basis is
/// OPEN — M, domain extent, direction, class, kernel selection.
#[test]
fn open_when_any_observational_basis_is_absent() {
    let full = synthetic_primary_registry();
    let po = execute_primary(&full, SYN_DELTA).unwrap();
    let cases: Vec<(Box<dyn Fn(&mut DeclarationRegistry)>, OpenCondition)> = vec![
        (Box::new(|r| r.mappings[0].basis = None),
         OpenCondition::MissingObservationalBasis { of: BasisOf::Mapping(SYN_M.into()) }),
        (Box::new(|r| r.domains[0].basis = None),
         OpenCondition::MissingObservationalBasis { of: BasisOf::Domain(SYN_DOMAIN.into()) }),
        (Box::new(|r| r.relations.iter_mut().find(|x| x.id == SYN_REL_01).unwrap().direction_basis = None),
         OpenCondition::MissingObservationalBasis { of: BasisOf::Direction(SYN_REL_01.into()) }),
        (Box::new(|r| r.relations.iter_mut().find(|x| x.id == SYN_REL_01).unwrap().class_basis = None),
         OpenCondition::MissingObservationalBasis { of: BasisOf::Class(SYN_REL_01.into()) }),
        (Box::new(|r| r.relations.iter_mut().find(|x| x.id == SYN_REL_01).unwrap().class = None),
         OpenCondition::MissingObservationalBasis { of: BasisOf::Class(SYN_REL_01.into()) }),
        (Box::new(|r| r.structures.iter_mut().find(|x| x.id == SYN_DELTA).unwrap().selection_basis = None),
         OpenCondition::MissingObservationalBasis { of: BasisOf::KernelSelection(SYN_DELTA.into()) }),
        (Box::new(|r| r.mappings[0].basis = Some(ObservationalBasis { observables: vec![], statement: "no observable".into() })),
         OpenCondition::MissingObservationalBasis { of: BasisOf::Mapping(SYN_M.into()) }),
    ];
    for (mutate, want) in cases {
        let mut reg = full.clone();
        mutate(&mut reg);
        assert_eq!(admit(&po, &reg), AdmissionOutcome::Open(vec![want]));
    }
}

/// [V9-OPEN] A declared scale factor must itself be a declared observable.
#[test]
fn open_when_unit_scale_is_not_a_declared_observable() {
    let mut reg = synthetic_primary_registry();
    reg.mappings[0].kind = MMappingKind::DeclaredUnitScale { scale_ref: "syn.obs.scale_undeclared".into() };
    let po = execute_primary(&synthetic_primary_registry(), SYN_DELTA).unwrap();
    assert_eq!(admit(&po, &reg), AdmissionOutcome::Open(vec![OpenCondition::MissingDeclaration {
        kind: DeclKind::Observable, id: "syn.obs.scale_undeclared".into(),
    }]));
}

/// [V9-OPEN] Outside the supported scope or M subset: OPEN, not REJECTED.
#[test]
fn open_for_unsupported_scope_and_mapping() {
    let mut reg = synthetic_primary_registry();
    let po = execute_primary(&reg, SYN_DELTA).unwrap();
    reg.structures.iter_mut().find(|x| x.id == SYN_DELTA).unwrap().kernel_selection = KernelSelection::Abr;
    assert!(matches!(admit(&po, &reg), AdmissionOutcome::Open(ref v)
        if matches!(v.as_slice(), [OpenCondition::UnsupportedScope { .. }])));

    let mut reg = synthetic_primary_registry();
    reg.mappings[0].kind = MMappingKind::Other { name: "declared but unsupported".into() };
    assert_eq!(admit(&po, &reg), AdmissionOutcome::Open(vec![OpenCondition::UnsupportedMapping {
        mapping: SYN_M.into(), kind: "declared but unsupported".into(),
    }]));
}

/// [V9-OPEN] Prerequisite (d): a projection outside the declared extent is not
/// admitted into operator evaluation.
#[test]
fn open_outside_declared_extent() {
    let mut reg = synthetic_primary_registry();
    let po = execute_primary(&reg, SYN_DELTA).unwrap();
    reg.domains[0].ceiling = 2.0;
    assert_eq!(admit(&po, &reg), AdmissionOutcome::Open(vec![OpenCondition::OutsideDeclaredExtent {
        binding: "syn.locus.0".into(), value: 3.0, floor: -10.0, ceiling: 2.0,
    }]));
}

/// [V9-OPEN] One parameter declared: Σ is not evaluated and the missing
/// parameter is named; Δ is still admitted.
#[test]
fn sigma_not_evaluated_when_one_parameter_is_undeclared() {
    let mut reg = synthetic_primary_registry();
    reg.structures.iter_mut().find(|x| x.id == SYN_CHAIN).unwrap().chi_0_ref = None;
    let po = execute_primary(&reg, SYN_CHAIN).unwrap();
    let e = admitted(admit(&po, &reg));
    assert!(e.entries.iter().all(|x| x.step == Step::Delta));
    assert_eq!(e.not_evaluated, vec![OpenCondition::NotEvaluated { step: Step::Sigma, missing: vec!["chi_0".into()] }]);
}

/// [V9-OPEN] χ₀ declared as 0 is not evaluable: OPEN, returned to Origin.
#[test]
fn open_when_declared_chi_0_is_not_evaluable() {
    let mut reg = synthetic_primary_registry();
    let po = execute_primary(&reg, SYN_CHAIN).unwrap();
    reg.observables.iter_mut().find(|o| o.id == "syn.obs.chi_0").unwrap().reported_value = 0.0;
    assert!(matches!(admit(&po, &reg), AdmissionOutcome::Open(ref v)
        if matches!(v.as_slice(), [OpenCondition::DeclarationInconsistent { .. }])));
}

/// [V9-OPEN] The photon-edge M must declare exactly the measurement references
/// the frozen function uses.
#[test]
fn open_when_photon_references_do_not_correspond() {
    let mut reg = synthetic_photon_registry();
    let po = execute_edge_m_projection(&reg, SYN_PHOTON_EDGE).unwrap();
    reg.observables.iter_mut().find(|o| o.id == "syn.ref.c").unwrap().reported_value = 3.0e8;
    assert!(matches!(admit(&po, &reg), AdmissionOutcome::Open(ref v)
        if matches!(v.as_slice(), [OpenCondition::DeclarationInconsistent { .. }])));
}

// ── [V9-REJECT] ───────────────────────────────────────────────────────────

/// [V9-REJECT] kernel_reference.md §10: B is not evaluated in the Primary
/// kernel. A record claiming a B step under a Primary declaration contradicts
/// the declared kernel selection.
#[test]
fn reject_operator_path_outside_declared_kernel_selection() {
    let reg = synthetic_primary_registry();
    let mut po = execute_primary(&reg, SYN_DELTA).unwrap();
    po.record.path = vec![Step::Delta, Step::B];
    assert_eq!(admit(&po, &reg), AdmissionOutcome::Rejected(RppiViolation::KernelSelection {
        declared: "Primary (Δ → Σ; B not evaluated)".into(), recorded_path: vec![Step::Delta, Step::B],
    }));
}
