//! Diagnostic execution of A → B → R — IMPLEMENTATION TESTS ONLY.
//!
//! [IMPL] Every field below is a SYNTHETIC FIXTURE. Values come from existing
//! kernel fixtures: tests/common synthetic_primary_registry (chain 0→1→2,
//! values 3, 1, 2) and operators.rs open_dag / gradient_field_abr /
//! abr_two_cycle_evolution (4 nodes, x1 = [3, 2, 1, 0], x2 = [0.1, 0.5, 0.3,
//! 0.9]). They verify that the wrapper calls the unchanged operators and
//! reports diagnostics; they establish no empirical validity.
//! Values introduced only to exercise an interface path (an explicit χ₀
//! adjustment, a non-finite or zero χ₀) are marked [INTERFACE VALUE].

mod common;

use abr_kernel::diagnostic_exec::*;
use abr_kernel::operators::{
    compute_rho, operator_a, operator_b, operator_e, operator_e_v5, operator_r, DeclaredRelations,
    EdgeProvenance, NodeField, PersistenceState,
};
use common::*;

// ── Fixtures (existing kernel fixtures, carried into the wrapper's form) ──

fn observed(class: EdgeProvenance, basis: &str) -> EdgeProvenanceRecord {
    EdgeProvenanceRecord { class, support: RelationSupport::Observed, basis: Some(basis.into()) }
}

/// synthetic_primary_registry, structure syn.st.chain.
fn chain_field() -> ExperimentalField {
    let reg = synthetic_primary_registry();
    let st = reg.structure(SYN_CHAIN).unwrap().clone();
    let nodes = st.loci.iter().map(|l| {
        let locus = reg.locus(l).unwrap();
        let m = reg.mapping(&locus.m_mapping_ref).unwrap();
        NodeRecord {
            id: l.clone(),
            values: vec![Some(reg.observable(&locus.observable_ref).unwrap().reported_value)],
            measurement: MeasurementSupport::Declared(m.basis.as_ref().unwrap().statement.clone()),
        }
    }).collect();
    let edges = st.relations.iter().map(|r| {
        let rel = reg.relation(r).unwrap();
        EdgeRecord {
            id: r.clone(), source: rel.src_locus.clone(), target: rel.tgt_locus.clone(),
            provenance: observed(rel.class.clone().unwrap(), &rel.class_basis.as_ref().unwrap().statement),
        }
    }).collect();
    ExperimentalField {
        field_id: "syn.chain".into(), observation_id: "syn.obs.1".into(), process_step: 0, n_components: 1,
        nodes, edges, component_pairs: vec![],
        parameters: Some(RhoParameters::General { chi_unit: "u".into() }),
        experimental_arrangement: None, unresolved_conditions: vec![],
    }
}

/// operators.rs open_dag(): 4 nodes, edges (0,1), (0,2), (1,3), (2,3), Continuation.
/// `step`: the declared process step of this observation.
fn open_dag_field(observation_id: &str, step: u64, x: [f64; 4]) -> ExperimentalField {
    let ids = ["n0", "n1", "n2", "n3"];
    ExperimentalField {
        field_id: "syn.open_dag".into(), observation_id: observation_id.into(), process_step: step, n_components: 1,
        nodes: ids.iter().zip(x).map(|(id, v)| NodeRecord {
            id: id.to_string(), values: vec![Some(v)],
            measurement: MeasurementSupport::Declared("synthetic fixture (operators.rs gradient_field_abr / abr_two_cycle_evolution)".into()),
        }).collect(),
        edges: [(0, 1), (0, 2), (1, 3), (2, 3)].iter().map(|&(s, t)| EdgeRecord {
            id: format!("e{s}{t}"), source: ids[s].into(), target: ids[t].into(),
            provenance: observed(EdgeProvenance::Continuation, "synthetic fixture: operators.rs open_dag"),
        }).collect(),
        component_pairs: vec![],
        parameters: Some(RhoParameters::General { chi_unit: "u".into() }),
        experimental_arrangement: None, unresolved_conditions: vec![],
    }
}
const X1: [f64; 4] = [3.0, 2.0, 1.0, 0.0]; // gradient_field_abr(1, 4)
const X2: [f64; 4] = [0.1, 0.5, 0.3, 0.9]; // abr_two_cycle_evolution x2

fn bits(f: &[Vec<f64>]) -> Vec<Vec<u64>> {
    f.iter().map(|c| c.iter().map(|v| v.to_bits()).collect()).collect()
}
fn classes(w: &[Warning]) -> Vec<(WarningClass, String)> {
    w.iter().map(|w| (w.class.clone(), w.item.clone())).collect()
}
fn has(w: &[Warning], class: WarningClass, item: &str) -> bool {
    w.iter().any(|x| x.class == class && x.item == item)
}
fn error_classes(e: &NotExecuted) -> Vec<(ErrorClass, String)> {
    e.errors.iter().map(|x| (x.class.clone(), x.item.clone())).collect()
}

/// Direct calls to the unchanged operators for the chain fixture.
fn chain_direct() -> (DeclaredRelations, NodeField) {
    let rel = DeclaredRelations::from_edges_with_provenance(3, vec![(0, 1), (1, 2)],
        vec![EdgeProvenance::Continuation, EdgeProvenance::Continuation]);
    (rel, NodeField::new(vec![vec![3.0, 1.0, 2.0]]))
}

// ── 1. A complete, well-formed field executes A → B → R ───────────────────

#[test]
fn complete_field_executes_a_b_r_with_kernel_values() {
    let x = execute_spatial(&chain_field()).expect("executes");
    let (rel, f) = chain_direct();
    let a = operator_a(&f, &rel, &[]);
    let rho = compute_rho(&a, &rel, 0.5, 1.0);
    let b = operator_b(&a, &rel);
    let r = operator_r(&b, &rel, &rho, &[]);
    assert_eq!(bits(&x.a.spatial), bits(&a.spatial));
    assert_eq!(x.rho.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), rho.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
    assert_eq!(bits(&x.b.spatial), bits(&b.spatial));
    assert_eq!(bits(&x.r.spatial), bits(&r.spatial));
    // Same as the canonical composition operator_e.
    assert_eq!(bits(&x.r.spatial), bits(&operator_e(&f, &rel, &[], &[], 0.5, 1.0).spatial));
    // Fully declared: no provenance warnings; the interior locus syn.locus.1 exists.
    assert!(!x.warnings.iter().any(|w| matches!(w.class, WarningClass::ProposedTopology
        | WarningClass::IncompleteObservationalSupport | WarningClass::UncertainMeasurement)), "{:?}", x.warnings);
    assert!(!x.warnings.iter().any(|w| w.reference == Some("kernel_reference.md §10")));
    // Only the kernel's own open condition A-3 at the chain ends.
    assert_eq!(classes(&x.warnings), vec![
        (WarningClass::UnresolvedCondition, SYN_REL_01.to_string()),
        (WarningClass::UnresolvedCondition, SYN_REL_12.to_string()),
    ]);
    assert!(x.warnings.iter().all(|w| w.reference == Some("operators.rs audit A-3")));
    assert_eq!(x.provenance.iter().map(|p| p.support.clone()).collect::<Vec<_>>(), vec![RelationSupport::Observed; 2]);
}

// ── 2. Provisional provenance executes and returns warnings ───────────────

#[test]
fn proposed_relation_executes_with_warning_and_stays_proposed() {
    let mut f = chain_field();
    f.edges[1].provenance.support = RelationSupport::Proposed;
    let x = execute_spatial(&f).expect("provisional topology is not an execution prohibition");
    assert!(has(&x.warnings, WarningClass::ProposedTopology, SYN_REL_12));
    assert_eq!(x.provenance[1].support, RelationSupport::Proposed, "never converted to observed");
    assert_eq!(x.provenance[1].class, EdgeProvenance::Continuation, "class preserved");
    let observed = execute_spatial(&chain_field()).unwrap();
    assert_eq!(bits(&x.r.spatial), bits(&observed.r.spatial), "uncertainty does not alter the mathematics");
}

#[test]
fn feedback_loop_executes_and_missing_interior_is_reported() {
    // A declared feedback loop n0 → n1 → n3 → n0 is a relationship, not a ring. kernel_reference.md §8
    // "rings and tori" concern relational evolution (wraparound / periodic indices), not cyclic relationships.
    let mut f = open_dag_field("o1", 0, X1);
    f.edges.push(EdgeRecord { id: "e30".into(), source: "n3".into(), target: "n0".into(),
        provenance: EdgeProvenanceRecord { class: EdgeProvenance::Continuation, support: RelationSupport::Proposed, basis: None } });
    let x = execute_spatial(&f).expect("executes");
    assert!(!x.warnings.iter().any(|w| w.reference == Some("kernel_reference.md §8")), "no ring warning for a declared loop");
    assert!(has(&x.warnings, WarningClass::ProposedTopology, "e30"));
    // Single edge: no interior locus (kernel_reference.md §10) — reported, not refused.
    let mut g = chain_field();
    g.edges.truncate(1);
    let y = execute_spatial(&g).expect("executes");
    assert!(y.warnings.iter().any(|w| w.reference == Some("kernel_reference.md §10")));
}

// ── 3. Incomplete observational support executes and returns warnings ─────

#[test]
fn incomplete_support_and_uncertain_measurement_execute_with_warnings() {
    let mut f = chain_field();
    f.edges[0].provenance.basis = None;
    f.nodes[0].measurement = MeasurementSupport::NotDeclared;
    f.nodes[2].measurement = MeasurementSupport::Uncertain("synthetic: M relationship uncertain".into());
    f.experimental_arrangement = Some("synthetic: experimental variable arrangement".into());
    f.unresolved_conditions = vec!["synthetic: unresolved condition".into()];
    let x = execute_spatial(&f).expect("executes");
    assert!(has(&x.warnings, WarningClass::IncompleteObservationalSupport, SYN_REL_01));
    assert!(has(&x.warnings, WarningClass::IncompleteObservationalSupport, "syn.locus.0"));
    assert!(has(&x.warnings, WarningClass::UncertainMeasurement, "syn.locus.2"));
    assert!(has(&x.warnings, WarningClass::ExperimentalArrangement, "syn.chain"));
    assert!(has(&x.warnings, WarningClass::UnresolvedCondition, "syn.chain"));
    assert_eq!(x.provenance[0].basis, None, "absent basis stays absent");
    assert_eq!(bits(&x.r.spatial), bits(&execute_spatial(&chain_field()).unwrap().r.spatial));
}

// ── 4. Invalid node reference: error, nothing executed ─────────────────────

#[test]
fn invalid_node_reference_is_an_error() {
    let mut f = chain_field();
    f.edges[1].target = "syn.locus.9".into();
    let e = execute_spatial(&f).unwrap_err();
    assert_eq!(error_classes(&e), vec![(ErrorClass::InvalidNodeReference, SYN_REL_12.to_string())]);
}

#[test]
fn structural_errors_are_reported_together() {
    let mut f = chain_field();
    f.nodes[1].id = "syn.locus.0".into();                       // duplicate node id
    f.nodes[2].values.push(Some(2.0));                           // 2 values for 1 component
    f.edges[0].provenance.class = EdgeProvenance::Persistence;   // not a spatial class
    f.component_pairs = vec![ComponentPair { a: 0, b: 1, cc: None }]; // index 1 ∉ 0..1, cc missing
    let e = execute_spatial(&f).unwrap_err();
    let got = error_classes(&e);
    for want in [ErrorClass::DuplicateIdentifier, ErrorClass::IncompatibleDimensions, ErrorClass::MissingNumericalInput] {
        assert!(got.iter().any(|(c, _)| *c == want), "{want:?} missing from {got:?}");
    }
}

// ── 5. Missing numerical inputs: errors, never invented ───────────────────

#[test]
fn missing_numerical_inputs_are_errors_not_defaults() {
    let mut f = chain_field();
    f.parameters = None;
    let e = execute_spatial(&f).unwrap_err();
    assert_eq!(error_classes(&e), vec![(ErrorClass::MissingNumericalInput, "parameters".to_string())]);

    let mut f = chain_field();
    f.nodes[1].values = vec![None];
    let e = execute_spatial(&f).unwrap_err();
    assert_eq!(error_classes(&e), vec![(ErrorClass::MissingNumericalInput, "syn.locus.1".to_string())]);

    let mut f = chain_field();
    f.parameters = Some(RhoParameters::General { chi_unit: " ".into() });
    assert_eq!(execute_spatial(&f).unwrap_err().errors[0].class, ErrorClass::MissingNumericalInput);

    let mut f = chain_field();
    f.nodes[0].values = vec![Some(f64::NAN)];
    assert_eq!(execute_spatial(&f).unwrap_err().errors[0].class, ErrorClass::NonFiniteInput);
}

// ── 6. General values respected when explicitly selected ──────────────────

#[test]
fn general_values_are_used_only_when_selected() {
    assert_eq!((GENERAL_RHO_BASE, GENERAL_CHI_0), (0.5, 1.0));
    let x = execute_spatial(&chain_field()).unwrap();
    assert_eq!(x.parameters, ParameterUse { selection: "general", rho_base: 0.5, chi_0: 1.0, chi_unit: "u".into(), adjustment_basis: None });
    let (rel, f) = chain_direct();
    let want = compute_rho(&operator_a(&f, &rel, &[]), &rel, 0.5, 1.0);
    assert_eq!(x.rho.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), want.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
}

// ── 7. Explicitly declared numerical-scale adjustment ──────────────────────

#[test]
fn declared_scale_adjustment_is_supported_and_recorded() {
    let mut f = chain_field();
    // [INTERFACE VALUE] χ₀ = 1000 in milli-u: the same reference as 1 u, restated for a rescaled χ unit.
    f.parameters = Some(RhoParameters::Declared { rho_base: 0.5, chi_0: 1000.0, chi_unit: "milli-u".into(),
        adjustment_basis: "synthetic: χ reported in milli-u".into() });
    let x = execute_spatial(&f).unwrap();
    assert_eq!(x.parameters.selection, "declared");
    assert_eq!(x.parameters.adjustment_basis.as_deref(), Some("synthetic: χ reported in milli-u"));
    let (rel, nf) = chain_direct();
    let want = compute_rho(&operator_a(&nf, &rel, &[]), &rel, 0.5, 1000.0);
    assert_eq!(x.rho.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), want.iter().map(|v| v.to_bits()).collect::<Vec<_>>());

    // [INTERFACE VALUE] χ₀ = 0 and NaN: not evaluable — errors, no substitution.
    f.parameters = Some(RhoParameters::Declared { rho_base: 0.5, chi_0: 0.0, chi_unit: "u".into(), adjustment_basis: "x".into() });
    assert_eq!(execute_spatial(&f).unwrap_err().errors[0].class, ErrorClass::UndefinedOperation);
    f.parameters = Some(RhoParameters::Declared { rho_base: 0.5, chi_0: f64::NAN, chi_unit: "u".into(), adjustment_basis: "x".into() });
    assert_eq!(execute_spatial(&f).unwrap_err().errors[0].class, ErrorClass::NonFiniteInput);
    // An adjustment without a stated basis still executes, with a warning.
    f.parameters = Some(RhoParameters::Declared { rho_base: 0.5, chi_0: 1000.0, chi_unit: "milli-u".into(), adjustment_basis: "".into() });
    assert!(has(&execute_spatial(&f).unwrap().warnings, WarningClass::IncompleteObservationalSupport, "parameters"));
}

// ── 8. Persistence only with a compatible actual prior ─────────────────────

#[test]
fn persistence_executes_with_actual_prior_and_matches_operator_e_v5() {
    let first = execute_spatial(&open_dag_field("o1", 0, X1)).unwrap();
    let second = execute_with_prior(&open_dag_field("o2", 1, X2), &first).expect("executes");

    let rel = DeclaredRelations::from_edges_with_provenance(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)],
        vec![EdgeProvenance::Continuation; 4]);
    let x1 = NodeField::new(vec![X1.to_vec()]);
    let x2 = NodeField::new(vec![X2.to_vec()]);
    let e1 = operator_e(&x1, &rel, &[], &[], 0.5, 1.0);
    let (e2, p) = operator_e_v5(&x2, &rel, &[], &[], &PersistenceState::from_prior(e1.clone()), 0.5, 1.0);
    assert_eq!(bits(&first.r.spatial), bits(&e1.spatial));
    assert_eq!(bits(&second.current.r.spatial), bits(&e2.spatial));
    assert_eq!(bits(&second.persistence.a_persistence), bits(&p.a_persistence));
    assert_eq!(bits(&second.persistence.b_persistence), bits(&p.b_persistence));
    assert_eq!(bits(&second.persistence.r_persistence), bits(&p.r_persistence));
    assert_eq!(second.prior_observation_id, "o1");
    // The sequence continues: step 2 against the actual output of step 1.
    let third = execute_with_prior(&open_dag_field("o3", 2, X1), &second.current).expect("executes");
    assert_eq!(third.prior_observation_id, "o2");
}

#[test]
fn observation_succession_is_enforced() {
    let first = execute_spatial(&open_dag_field("o1", 0, X1)).unwrap();
    let second = execute_spatial(&open_dag_field("o2", 1, X2)).unwrap();
    let succession = |r: Result<PersistenceExecution, NotExecuted>| {
        let e = r.unwrap_err();
        assert!(e.errors.iter().any(|x| x.class == ErrorClass::IncompatiblePrior && x.detail.contains("immediately preceding")), "{e:?}");
    };
    // Reversed (wraparound): the later observation as prior of the earlier one.
    succession(execute_with_prior(&open_dag_field("o1", 0, X1), &second));
    // Gap: step 0 as prior of step 2.
    succession(execute_with_prior(&open_dag_field("o3", 2, X2), &first));
    // Repetition: two observations declared at the same step.
    succession(execute_with_prior(&open_dag_field("o2", 0, X2), &first));
}

#[test]
fn incompatible_prior_is_an_error() {
    let first = execute_spatial(&open_dag_field("o1", 0, X1)).unwrap();
    // Same observation twice is not a sequential pair.
    let e = execute_with_prior(&open_dag_field("o1", 0, X1), &first).unwrap_err();
    assert_eq!(e.errors[0].class, ErrorClass::IncompatiblePrior);
    // Different declared edges: E_prior[e] and E_current[e] are not the same relation.
    let mut g = open_dag_field("o2", 1, X2);
    g.edges.swap(0, 1);
    assert_eq!(execute_with_prior(&g, &first).unwrap_err().errors[0].class, ErrorClass::IncompatiblePrior);
    // A prior from another field.
    let other = execute_spatial(&chain_field()).unwrap();
    assert!(execute_with_prior(&open_dag_field("o2", 1, X2), &other).unwrap_err().errors.iter()
        .all(|e| e.class == ErrorClass::IncompatiblePrior));
}

#[test]
fn first_observation_has_no_persistence_output() {
    // execute_spatial returns only A, ρ, B, R — there is no persistence field to fill.
    let x = execute_spatial(&open_dag_field("o1", 0, X1)).unwrap();
    assert_eq!(x.r.spatial[0].len(), 4);
    assert_eq!(x.observation_id, "o1");
}
