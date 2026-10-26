// provenanced_exec.rs — Metatron Dynamics, Inc. Kernel V9.
// Generator execution path: executes a declared calculation through the frozen
// Kernel V8 constructors and functions and authors its calculation record
// (v9_specification.md §3: "Generated at computation time ... The Generator is
// the author").
// Bounded over D. No claim beyond D.
//
// This file makes no admission decision. What it produces is evidence to be
// checked by admission.rs, not a certified account (Correction 1). It
// introduces no operator, formula, parameter, or default: every value it uses
// is read from the declaration registry, and every calculation is a call to an
// unchanged function in operators.rs or derived_invariants.rs.
//
// Execution requires the declarations it reads. Where one is absent the
// Generator cannot execute and returns ExecError; it does not substitute.

use crate::derived_invariants::epsilon_photon_edge;
use crate::operators::{
    operator_delta, operator_sigma, DeclaredRelations, EdgeProvenance, NodeField,
};
use crate::provenance::*;

/// Why the Generator could not execute a declared calculation.
#[derive(Clone, Debug, PartialEq)]
pub enum ExecError {
    NotDeclared { kind: &'static str, id: DeclId },
    NotExecutable { reason: String },
}

fn not_declared(kind: &'static str, id: &str) -> ExecError {
    ExecError::NotDeclared { kind, id: id.to_string() }
}

fn project(reg: &DeclarationRegistry, obs: &ObservableDeclaration, m: &MMappingDeclaration)
    -> Result<f64, ExecError>
{
    match &m.kind {
        MMappingKind::Identity => Ok(obs.reported_value),
        MMappingKind::DeclaredUnitScale { scale_ref } => {
            let s = reg.observable(scale_ref).ok_or_else(|| not_declared("observable", scale_ref))?;
            Ok(obs.reported_value * s.reported_value)
        }
        MMappingKind::KernelDeclaredFunction { function: KernelFunction::EpsilonPhotonEdge, .. } => {
            Ok(epsilon_photon_edge(obs.reported_value))
        }
        MMappingKind::Other { name } => Err(ExecError::NotExecutable {
            reason: format!("M kind '{name}' is outside the supported subset"),
        }),
    }
}

fn source_of(obs: &ObservableDeclaration) -> String {
    obs.record.as_ref().map(|r| r.source.clone()).unwrap_or_default()
}

fn sorted(mut v: Vec<DeclId>) -> Vec<DeclId> {
    v.sort();
    v.dedup();
    v
}

/// Execute a declared Primary-kernel calculation: Δ, and Σ where ρ_base and χ₀
/// are declared through M.
pub fn execute_primary(reg: &DeclarationRegistry, structure_id: &str)
    -> Result<ProvenancedOutput, ExecError>
{
    let st = reg.structure(structure_id).ok_or_else(|| not_declared("structure", structure_id))?;
    if st.kernel_selection != KernelSelection::Primary {
        return Err(ExecError::NotExecutable { reason: "kernel selection is not Primary".into() });
    }

    // Inputs: each declared locus projected through its declared M.
    let mut inputs = Vec::new();
    let mut values = Vec::new();
    for lid in &st.loci {
        let l = reg.locus(lid).ok_or_else(|| not_declared("locus", lid))?;
        let obs = reg.observable(&l.observable_ref).ok_or_else(|| not_declared("observable", &l.observable_ref))?;
        let m = reg.mapping(&l.m_mapping_ref).ok_or_else(|| not_declared("mapping", &l.m_mapping_ref))?;
        let v = project(reg, obs, m)?;
        inputs.push(InputRecord {
            binding_ref: lid.clone(),
            observable_ref: obs.id.clone(),
            claimed_source: source_of(obs),
            observed_value: obs.reported_value,
            m_mapping_ref: m.id.clone(),
            projected_value: v,
        });
        values.push(v);
    }

    // Relations through the frozen constructor.
    let mut edges = Vec::new();
    let mut classes = Vec::new();
    for rid in &st.relations {
        let r = reg.relation(rid).ok_or_else(|| not_declared("relation", rid))?;
        let s = st.loci.iter().position(|x| *x == r.src_locus)
            .ok_or_else(|| not_declared("locus in structure", &r.src_locus))?;
        let t = st.loci.iter().position(|x| *x == r.tgt_locus)
            .ok_or_else(|| not_declared("locus in structure", &r.tgt_locus))?;
        edges.push((s, t));
        classes.push(r.class.clone().ok_or_else(|| not_declared("relation class", rid))?);
    }
    if classes.iter().any(|c| *c == EdgeProvenance::Persistence) || values.is_empty() {
        return Err(ExecError::NotExecutable { reason: "not a spatial Phase 1 declaration".into() });
    }
    let rel = DeclaredRelations::from_edges_with_provenance(st.loci.len(), edges.clone(), classes.clone());
    let x = NodeField::new(vec![values]);

    // Frozen Δ.
    let delta = operator_delta(&x, &rel);
    let mut path = vec![Step::Delta];

    // Frozen Σ only where both parameters are declared through M.
    let params = match (&st.rho_base_ref, &st.chi_0_ref) {
        (Some(rb), Some(c0)) => {
            let rbo = reg.observable(rb).ok_or_else(|| not_declared("observable", rb))?;
            let c0o = reg.observable(c0).ok_or_else(|| not_declared("observable", c0))?;
            Some((rbo, c0o))
        }
        _ => None,
    };
    let sigma = params.map(|(rb, c0)| {
        path.push(Step::Sigma);
        operator_sigma(&delta, &rel, rb.reported_value, c0.reported_value)
    });
    let mut parameters = Vec::new();
    if let Some((rb, c0)) = params {
        parameters.push(ParameterRecord { name: "rho_base".into(), observable_ref: rb.id.clone(), value: rb.reported_value });
        parameters.push(ParameterRecord { name: "chi_0".into(), observable_ref: c0.id.clone(), value: c0.reported_value });
    }

    // Relation traversals, including what Σ read.
    let mut relations = Vec::new();
    for (e, rid) in st.relations.iter().enumerate() {
        let (s, t) = edges[e];
        let (ap, am, inc) = if sigma.is_some() {
            let inc: Vec<usize> = (0..edges.len())
                .filter(|&f| edges[f].0 == s || edges[f].1 == s).collect();
            (rel.adj_plus[e].clone(), rel.adj_minus[e].clone(), inc)
        } else {
            (vec![], vec![], vec![])
        };
        relations.push(RelationRecord {
            relation_ref: rid.clone(), src_index: s, tgt_index: t, class: classes[e].clone(),
            adj_plus: ap, adj_minus: am, rho_incident: inc,
        });
    }

    // Output entries and their contributing declarations.
    let edge_ids = |list: &[usize]| -> Vec<DeclId> {
        list.iter().flat_map(|&f| {
            let (s, t) = edges[f];
            vec![st.relations[f].clone(), st.loci[s].clone(), st.loci[t].clone()]
        }).collect()
    };
    let mut outputs = Vec::new();
    for (e, rid) in st.relations.iter().enumerate() {
        outputs.push(OutputEntry {
            step: Step::Delta, relation_ref: rid.clone(), component: 0,
            value: delta.field[0][e], contributing: sorted(edge_ids(&[e])),
        });
    }
    if let Some(sig) = &sigma {
        for (e, rid) in st.relations.iter().enumerate() {
            let r = &relations[e];
            let mut c = edge_ids(&[e]);
            c.extend(edge_ids(&r.adj_plus));
            c.extend(edge_ids(&r.adj_minus));
            c.extend(edge_ids(&r.rho_incident));
            c.extend(parameters.iter().map(|p| p.observable_ref.clone()));
            outputs.push(OutputEntry {
                step: Step::Sigma, relation_ref: rid.clone(), component: 0,
                value: sig.field[0][e], contributing: sorted(c),
            });
        }
    }

    Ok(ProvenancedOutput {
        output: CalculatedOutput::Primary { delta, sigma },
        record: CalculationRecord {
            calculation: Calculation::Primary { structure_ref: structure_id.to_string() },
            inputs, relations, path, parameters, outputs,
        },
    })
}

/// Execute a declared M projection attached to one declared relation
/// (e.g. ε[e] = hc/λ at a declared photon edge).
pub fn execute_edge_m_projection(reg: &DeclarationRegistry, relation_id: &str)
    -> Result<ProvenancedOutput, ExecError>
{
    let r = reg.relation(relation_id).ok_or_else(|| not_declared("relation", relation_id))?;
    let obs = reg.observable(&r.observable_source).ok_or_else(|| not_declared("observable", &r.observable_source))?;
    let m = reg.mapping(&r.m_mapping_ref).ok_or_else(|| not_declared("mapping", &r.m_mapping_ref))?;
    let v = project(reg, obs, m)?;
    let mut contributing = vec![r.id.clone(), obs.id.clone(), m.id.clone()];
    if let MMappingKind::KernelDeclaredFunction { references, .. } = &m.kind {
        contributing.extend(references.iter().cloned());
    }
    Ok(ProvenancedOutput {
        output: CalculatedOutput::EdgeQuantity { value: v },
        record: CalculationRecord {
            calculation: Calculation::EdgeMProjection { relation_ref: relation_id.to_string() },
            inputs: vec![InputRecord {
                binding_ref: r.id.clone(),
                observable_ref: obs.id.clone(),
                claimed_source: source_of(obs),
                observed_value: obs.reported_value,
                m_mapping_ref: m.id.clone(),
                projected_value: v,
            }],
            relations: vec![],
            path: vec![Step::MProjection],
            parameters: vec![],
            outputs: vec![OutputEntry {
                step: Step::MProjection, relation_ref: r.id.clone(), component: 0,
                value: v, contributing: sorted(contributing),
            }],
        },
    })
}
