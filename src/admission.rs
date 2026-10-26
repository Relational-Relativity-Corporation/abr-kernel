// admission.rs — Metatron Dynamics, Inc. Kernel V9.
// V9 admission interface: independent reconstruction of a declared calculation
// from Origin declaration records, and the admission decision for a submitted
// calculation record (v9_specification.md §3, §6, §7, §10; Req-5, Req-6, Req-7).
// Bounded over D. No claim beyond D.
//
// Foundational primitive (Kernel V9): the foundational observable primitive is a
// directed relation declared from observable information. A numerical value is not
// itself a primitive; it is the projection of an observed relational distinction
// through a declared measurement mapping M. Observed relations, measurement mappings
// M, numerical projections in D, and calculated results are held distinct.
//
// ── Independence ──────────────────────────────────────────────────────────
//
// This module never calls provenanced_exec.rs and never reads a V8 structure
// carried in a submitted record. It reconstructs every input, traversal, and
// output from the DeclarationRegistry alone, through the unchanged frozen
// functions, and then compares. The calculation record is evidence to be
// checked (Correction 1). V8 constructors are not modified (Req-5,
// Correction 2).
//
// This is reconstruction within the kernel crate. It is NOT independent IRRS
// verification from a distinct frame (v9_specification.md §11 milestone 7):
// it shares the operator code it reconstructs with, and a record forged
// consistently in every field over two declarations that yield identical
// computation cannot be distinguished by reconstruction alone.
//
// ── Outcomes (Origin correction, 2026-10-25, rules 6 and 7) ────────────────
//
//   OPEN      — a required declaration or observational basis is absent or
//               unverifiable, or the calculation is outside supported scope.
//               Not evaluated; not admitted. Never reported as 0.0. Never
//               converted into REJECTED.
//   REJECTED  — only when a specific contradiction is established between the
//               record and the authoritative declarations or the independent
//               reconstruction. Every violation carries both sides.
//   ADMITTED  — every chain is resolved and terminates in declared external
//               observables, and the reconstruction reproduces the record.
//               Admission is a provenance-conformance statement. It is not a
//               correspondence claim (kernel_reference.md §16.3) and, for
//               Synthetic or Simulated termini, not empirical evidence.
//
// Evaluation order: resolve everything (OPEN stops here) → reconstruct →
// resolve the record's references (OPEN) → contradictions (REJECTED) →
// ADMITTED.
//
// ── Comparison rule ───────────────────────────────────────────────────────
//
// Exact f64 bit equality is used only where the reconstruction executes the
// same declared calculation, through the same frozen function, in the same
// order, on bit-identical inputs. In the supported scope that holds for every
// compared value: the M projections (Identity, DeclaredUnitScale,
// epsilon_photon_edge), operator_delta, and operator_sigma are deterministic
// single-path functions of their declared inputs. Numerical agreement is
// necessary for admission and never sufficient: identity and observational
// termination are established first.
//
// ── Supported scope (reported explicitly) ─────────────────────────────────
//
//   Supported:   Primary-kernel Δ over one declared observation (Phase 1);
//                Primary Σ only where ρ_base and χ₀ are declared through M;
//                declared M projection for one edge quantity through a
//                kernel-declared function (ε[e] = hc/λ at a photon edge);
//                M kinds Identity, DeclaredUnitScale, KernelDeclaredFunction.
//   OPEN:        ABR A → B → R; component pairs and cc; persistence (Phase 2);
//                other Layer 3 quantities; other M kinds; external IRRS.

use crate::derived_invariants::{epsilon_photon_edge, C_DECLARED, H_PLANCK};
use crate::operators::{
    operator_delta, operator_sigma, DeclaredRelations, EdgeProvenance, NodeField,
    PrimaryEdgeField,
};
use crate::provenance::*;

pub const COMPARISON_RULE: &str =
    "exact f64 bit equality: same declared calculation, same frozen function, \
     same execution order, bit-identical declared inputs";

// ── Outcome types ─────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub enum DeclKind {
    Structure,
    Locus,
    Observable,
    Mapping,
    Domain,
    Relation,
}

/// The declaration element whose observational basis is required.
#[derive(Clone, Debug, PartialEq)]
pub enum BasisOf {
    Mapping(DeclId),
    Domain(DeclId),
    Direction(DeclId),
    Class(DeclId),
    KernelSelection(DeclId),
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpenCondition {
    /// A declaration referenced by another declaration is not in the registry.
    MissingDeclaration { kind: DeclKind, id: DeclId },
    /// The submitted record cites an id the registry does not contain.
    Unverifiable { kind: DeclKind, id: DeclId },
    /// The element has no declared observational basis / correspondence.
    MissingObservationalBasis { of: BasisOf },
    /// The observable names no external record: not a terminus.
    NoExternalTerminus { observable: DeclId },
    /// The observable's measurement conditions are not declared.
    MissingMeasurementConditions { observable: DeclId },
    /// A declared M kind outside the supported subset.
    UnsupportedMapping { mapping: DeclId, kind: String },
    /// A declared calculation outside the supported scope.
    UnsupportedScope { what: String },
    /// Origin declarations that cannot be evaluated together, returned to Origin.
    DeclarationInconsistent { detail: String },
    /// Prerequisite (d): a projection outside the declared extent is not
    /// admitted into operator evaluation.
    OutsideDeclaredExtent { binding: DeclId, value: f64, floor: f64, ceiling: f64 },
    /// An operator not evaluated because its declared inputs are absent.
    NotEvaluated { step: Step, missing: Vec<String> },
}

/// Where a recorded derivation contradicts the reconstructed one.
#[derive(Clone, Debug, PartialEq)]
pub enum Link {
    InputBinding,
    Traversal,
    OutputDerivation,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RppiViolation {
    /// §6.2: a claimed declaration content contradicts the authoritative
    /// declaration with the same id.
    Fabricated { id: DeclId, field: String, declared: String, recorded: String },
    /// §6.4: the result is attributed to a declared relation other than the one
    /// that produced it.
    Misattributed { entry: String, claimed: DeclId, reconstructed: DeclId },
    /// kernel_reference.md §10: the recorded path is not the path the declared
    /// kernel selection admits.
    KernelSelection { declared: String, recorded_path: Vec<Step> },
    /// §6.3: the recorded derivation contradicts the independent reconstruction
    /// at a located link.
    Disconnected { link: Link, entry: String, reconstructed: String, recorded: String },
}

#[derive(Clone, Debug, PartialEq)]
pub enum ObservationalStatus {
    Measured,
    Simulated,
    Synthetic,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Terminus {
    pub observable: DeclId,
    pub origin_kind: OriginKind,
    pub record: ExternalRecord,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReverseTrace {
    pub entry: String,
    pub termini: Vec<Terminus>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReconstructedInput {
    pub binding: DeclId,
    pub observable: DeclId,
    pub m_mapping: DeclId,
    pub projected: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReconstructedEntry {
    pub step: Step,
    pub relation: DeclId,
    pub component: usize,
    pub value: f64,
    pub contributing: Vec<DeclId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmissionEvidence {
    pub observational_status: ObservationalStatus,
    pub inputs: Vec<ReconstructedInput>,
    pub entries: Vec<ReconstructedEntry>,
    pub reverse_traces: Vec<ReverseTrace>,
    /// Operators of the declared kernel that were not evaluated (e.g. Σ
    /// without declared ρ_base, χ₀). Not admitted; not 0.0.
    pub not_evaluated: Vec<OpenCondition>,
    pub comparison_rule: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AdmissionOutcome {
    Admitted(AdmissionEvidence),
    Rejected(RppiViolation),
    Open(Vec<OpenCondition>),
}

// ── Resolution (registry only) ────────────────────────────────────────────

struct Resolver<'a> {
    reg: &'a DeclarationRegistry,
    open: Vec<OpenCondition>,
}

impl<'a> Resolver<'a> {
    fn push(&mut self, c: OpenCondition) {
        if !self.open.contains(&c) {
            self.open.push(c);
        }
    }

    fn observable(&mut self, id: &str) -> Option<&'a ObservableDeclaration> {
        let Some(o) = self.reg.observable(id) else {
            self.push(OpenCondition::MissingDeclaration { kind: DeclKind::Observable, id: id.into() });
            return None;
        };
        match &o.record {
            Some(r) if !r.source.trim().is_empty() && !r.entry.trim().is_empty() => {}
            _ => self.push(OpenCondition::NoExternalTerminus { observable: id.into() }),
        }
        match &o.declared_conditions {
            Some(c) if !c.trim().is_empty() => {}
            _ => self.push(OpenCondition::MissingMeasurementConditions { observable: id.into() }),
        }
        if !o.reported_value.is_finite() {
            self.push(OpenCondition::DeclarationInconsistent {
                detail: format!("observable {id}: reported value not finite (not in D)"),
            });
        }
        Some(o)
    }

    fn basis(&mut self, b: &Option<ObservationalBasis>, of: BasisOf) {
        match b {
            Some(b) if !b.observables.is_empty() && !b.statement.trim().is_empty() => {
                for o in &b.observables {
                    self.observable(o);
                }
            }
            _ => self.push(OpenCondition::MissingObservationalBasis { of }),
        }
    }

    fn mapping(&mut self, id: &str, input_unit: Option<&str>) -> Option<&'a MMappingDeclaration> {
        let Some(m) = self.reg.mapping(id) else {
            self.push(OpenCondition::MissingDeclaration { kind: DeclKind::Mapping, id: id.into() });
            return None;
        };
        self.basis(&m.basis, BasisOf::Mapping(id.into()));
        if let Some(u) = input_unit {
            if u != m.input_unit {
                self.push(OpenCondition::DeclarationInconsistent {
                    detail: format!("mapping {id}: input unit '{}' but observable unit '{u}'", m.input_unit),
                });
            }
        }
        match &m.kind {
            MMappingKind::Identity => {}
            MMappingKind::DeclaredUnitScale { scale_ref } => {
                self.observable(scale_ref);
            }
            MMappingKind::KernelDeclaredFunction { function: KernelFunction::EpsilonPhotonEdge, references } => {
                let mut values = Vec::new();
                for r in references {
                    if let Some(o) = self.observable(r) {
                        values.push(o.reported_value.to_bits());
                    }
                }
                // The function uses the frozen declared measurement references
                // H_PLANCK and C_DECLARED (kernel_reference.md §13.1). The
                // declared references must be exactly those, as declared values.
                let mut want = vec![H_PLANCK.to_bits(), C_DECLARED.to_bits()];
                want.sort();
                values.sort();
                if values.len() == references.len() && values != want {
                    self.push(OpenCondition::DeclarationInconsistent {
                        detail: format!(
                            "mapping {id}: declared references do not correspond to the \
                             measurement references H_PLANCK and C_DECLARED used by \
                             epsilon_photon_edge"),
                    });
                }
            }
            MMappingKind::Other { name } => self.push(OpenCondition::UnsupportedMapping {
                mapping: id.into(), kind: name.clone(),
            }),
        }
        Some(m)
    }

    fn domain(&mut self, id: &str, unit: Option<&str>) -> Option<&'a DeclaredDomain> {
        let Some(d) = self.reg.domain(id) else {
            self.push(OpenCondition::MissingDeclaration { kind: DeclKind::Domain, id: id.into() });
            return None;
        };
        self.basis(&d.basis, BasisOf::Domain(id.into()));
        if !(d.floor.is_finite() && d.ceiling.is_finite() && d.floor <= d.ceiling) {
            self.push(OpenCondition::DeclarationInconsistent {
                detail: format!("domain {id}: extent is not a finite interval"),
            });
        }
        if let Some(u) = unit {
            if u != d.unit {
                self.push(OpenCondition::DeclarationInconsistent {
                    detail: format!("domain {id}: unit '{}' but projection unit '{u}'", d.unit),
                });
            }
        }
        Some(d)
    }

    fn relation(&mut self, id: &str) -> Option<&'a ObservableRelationDeclaration> {
        let Some(r) = self.reg.relation(id) else {
            self.push(OpenCondition::MissingDeclaration { kind: DeclKind::Relation, id: id.into() });
            return None;
        };
        let obs_unit = self.observable(&r.observable_source).map(|o| o.unit.clone());
        let out_unit = self.mapping(&r.m_mapping_ref, obs_unit.as_deref()).map(|m| m.output_unit.clone());
        self.domain(&r.declared_domain, out_unit.as_deref());
        self.basis(&r.direction_basis, BasisOf::Direction(id.into()));
        match &r.class {
            Some(_) => self.basis(&r.class_basis, BasisOf::Class(id.into())),
            None => self.push(OpenCondition::MissingObservationalBasis { of: BasisOf::Class(id.into()) }),
        }
        if r.class == Some(EdgeProvenance::Persistence) {
            self.push(OpenCondition::DeclarationInconsistent {
                detail: format!("relation {id}: Persistence is not a spatial relation (use PersistenceState)"),
            });
        }
        Some(r)
    }
}

// ── Reconstruction (registry only, frozen functions) ──────────────────────

fn project(reg: &DeclarationRegistry, value: f64, m: &MMappingDeclaration) -> f64 {
    match &m.kind {
        MMappingKind::Identity => value,
        MMappingKind::DeclaredUnitScale { scale_ref } => {
            value * reg.observable(scale_ref).expect("resolved").reported_value
        }
        MMappingKind::KernelDeclaredFunction { function: KernelFunction::EpsilonPhotonEdge, .. } => {
            epsilon_photon_edge(value)
        }
        MMappingKind::Other { .. } => unreachable!("Other is OPEN before reconstruction"),
    }
}

fn sorted(mut v: Vec<DeclId>) -> Vec<DeclId> {
    v.sort();
    v.dedup();
    v
}

/// Observable ids reached from a declared M mapping (its basis and references).
fn mapping_observables(reg: &DeclarationRegistry, id: &str) -> Vec<DeclId> {
    let m = reg.mapping(id).expect("resolved");
    let mut v = m.basis.as_ref().map(|b| b.observables.clone()).unwrap_or_default();
    match &m.kind {
        MMappingKind::DeclaredUnitScale { scale_ref } => v.push(scale_ref.clone()),
        MMappingKind::KernelDeclaredFunction { references, .. } => v.extend(references.iter().cloned()),
        _ => {}
    }
    v
}

fn domain_observables(reg: &DeclarationRegistry, id: &str) -> Vec<DeclId> {
    reg.domain(id).expect("resolved").basis.as_ref().map(|b| b.observables.clone()).unwrap_or_default()
}

fn locus_observables(reg: &DeclarationRegistry, id: &str) -> Vec<DeclId> {
    let l = reg.locus(id).expect("resolved");
    let mut v = vec![l.observable_ref.clone()];
    v.extend(mapping_observables(reg, &l.m_mapping_ref));
    v.extend(domain_observables(reg, &l.domain_ref));
    v
}

fn relation_observables(reg: &DeclarationRegistry, id: &str) -> Vec<DeclId> {
    let r = reg.relation(id).expect("resolved");
    let mut v = vec![r.observable_source.clone()];
    v.extend(mapping_observables(reg, &r.m_mapping_ref));
    v.extend(domain_observables(reg, &r.declared_domain));
    for b in [&r.direction_basis, &r.class_basis] {
        v.extend(b.as_ref().map(|b| b.observables.clone()).unwrap_or_default());
    }
    v
}

fn termini(reg: &DeclarationRegistry, ids: Vec<DeclId>) -> Vec<Terminus> {
    sorted(ids).into_iter().map(|id| {
        let o = reg.observable(&id).expect("resolved");
        Terminus { observable: id, origin_kind: o.origin_kind.clone(), record: o.record.clone().expect("resolved") }
    }).collect()
}

struct Reconstruction {
    inputs: Vec<ReconstructedInput>,
    relation_ids: Vec<DeclId>,
    edges: Vec<(usize, usize)>,
    classes: Vec<EdgeProvenance>,
    adjacency: Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>,
    path: Vec<Step>,
    parameters: Vec<ParameterRecord>,
    entries: Vec<ReconstructedEntry>,
    reverse_traces: Vec<ReverseTrace>,
    not_evaluated: Vec<OpenCondition>,
    delta: Option<PrimaryEdgeField>,
    sigma: Option<PrimaryEdgeField>,
    edge_quantity: Option<f64>,
}

/// Resolve and reconstruct a declared Primary calculation from the registry.
fn reconstruct_primary(reg: &DeclarationRegistry, sid: &str) -> Result<Reconstruction, Vec<OpenCondition>> {
    let mut rs = Resolver { reg, open: vec![] };
    let Some(st) = reg.structure(sid) else {
        return Err(vec![OpenCondition::Unverifiable { kind: DeclKind::Structure, id: sid.into() }]);
    };
    if st.kernel_selection == KernelSelection::Abr {
        return Err(vec![OpenCondition::UnsupportedScope {
            what: "ABR kernel A → B → R (with component pairs and cc)".into(),
        }]);
    }
    rs.basis(&st.selection_basis, BasisOf::KernelSelection(sid.into()));
    if st.loci.is_empty() || st.relations.is_empty() {
        rs.push(OpenCondition::DeclarationInconsistent { detail: format!("structure {sid}: no declared loci or relations") });
    }

    // Loci → observable → M → domain.
    let mut d_unit: Option<String> = None;
    for lid in &st.loci {
        let Some(l) = reg.locus(lid) else {
            rs.push(OpenCondition::MissingDeclaration { kind: DeclKind::Locus, id: lid.clone() });
            continue;
        };
        let obs_unit = rs.observable(&l.observable_ref).map(|o| o.unit.clone());
        let out = rs.mapping(&l.m_mapping_ref, obs_unit.as_deref()).map(|m| m.output_unit.clone());
        rs.domain(&l.domain_ref, out.as_deref());
        if let Some(u) = out {
            match &d_unit {
                None => d_unit = Some(u),
                Some(prev) if *prev != u => rs.push(OpenCondition::DeclarationInconsistent {
                    detail: format!("structure {sid}: loci projected into different units ('{prev}', '{u}') in one component"),
                }),
                _ => {}
            }
        }
    }
    // Relations, endpoints within the declared structure.
    for rid in &st.relations {
        if let Some(r) = rs.relation(rid) {
            for l in [&r.src_locus, &r.tgt_locus] {
                if !st.loci.contains(l) {
                    rs.push(OpenCondition::DeclarationInconsistent {
                        detail: format!("relation {rid}: locus {l} is not declared in structure {sid}"),
                    });
                }
            }
        }
    }
    // Parameters: both declared through M, or Σ is not evaluated.
    let params = match (&st.rho_base_ref, &st.chi_0_ref) {
        (Some(rb), Some(c0)) => {
            rs.observable(rb);
            rs.observable(c0);
            Some((rb.clone(), c0.clone()))
        }
        _ => None,
    };
    if !rs.open.is_empty() {
        return Err(rs.open);
    }

    // Inputs projected through declared M; extent prerequisite (d).
    let mut inputs = Vec::new();
    let mut values = Vec::new();
    let mut open = Vec::new();
    for lid in &st.loci {
        let l = reg.locus(lid).expect("resolved");
        let o = reg.observable(&l.observable_ref).expect("resolved");
        let m = reg.mapping(&l.m_mapping_ref).expect("resolved");
        let d = reg.domain(&l.domain_ref).expect("resolved");
        let v = project(reg, o.reported_value, m);
        if !v.is_finite() || v < d.floor || v > d.ceiling {
            open.push(OpenCondition::OutsideDeclaredExtent { binding: lid.clone(), value: v, floor: d.floor, ceiling: d.ceiling });
        }
        inputs.push(ReconstructedInput { binding: lid.clone(), observable: o.id.clone(), m_mapping: m.id.clone(), projected: v });
        values.push(v);
    }
    if !open.is_empty() {
        return Err(open);
    }

    // Frozen constructor and operators.
    let mut edges = Vec::new();
    let mut classes = Vec::new();
    for rid in &st.relations {
        let r = reg.relation(rid).expect("resolved");
        let s = st.loci.iter().position(|x| *x == r.src_locus).expect("checked");
        let t = st.loci.iter().position(|x| *x == r.tgt_locus).expect("checked");
        edges.push((s, t));
        classes.push(r.class.clone().expect("resolved"));
    }
    let rel = DeclaredRelations::from_edges_with_provenance(st.loci.len(), edges.clone(), classes.clone());
    let x = NodeField::new(vec![values]);
    let delta = operator_delta(&x, &rel);
    let mut path = vec![Step::Delta];
    let mut parameters = Vec::new();
    let mut not_evaluated = Vec::new();
    let sigma = match &params {
        Some((rb, c0)) => {
            let rbv = reg.observable(rb).expect("resolved").reported_value;
            let c0v = reg.observable(c0).expect("resolved").reported_value;
            if !(c0v > 0.0) {
                // require_rho_declaration would panic; χ₀ = 0 or negative is not evaluable.
                return Err(vec![OpenCondition::DeclarationInconsistent {
                    detail: format!("χ₀ declared as {c0v}: not evaluable (must be finite and positive)"),
                }]);
            }
            path.push(Step::Sigma);
            parameters.push(ParameterRecord { name: "rho_base".into(), observable_ref: rb.clone(), value: rbv });
            parameters.push(ParameterRecord { name: "chi_0".into(), observable_ref: c0.clone(), value: c0v });
            Some(operator_sigma(&delta, &rel, rbv, c0v))
        }
        None => {
            let mut missing = Vec::new();
            if st.rho_base_ref.is_none() { missing.push("rho_base".to_string()); }
            if st.chi_0_ref.is_none() { missing.push("chi_0".to_string()); }
            not_evaluated.push(OpenCondition::NotEvaluated { step: Step::Sigma, missing });
            None
        }
    };

    // Traversal as read by the evaluated operators.
    let adjacency: Vec<(Vec<usize>, Vec<usize>, Vec<usize>)> = (0..edges.len()).map(|e| {
        if sigma.is_some() {
            let s = edges[e].0;
            let inc = (0..edges.len()).filter(|&f| edges[f].0 == s || edges[f].1 == s).collect();
            (rel.adj_plus[e].clone(), rel.adj_minus[e].clone(), inc)
        } else {
            (vec![], vec![], vec![])
        }
    }).collect();

    // Entries, contributing declarations, and reverse traces.
    let edge_ids = |list: &[usize]| -> Vec<DeclId> {
        list.iter().flat_map(|&f| {
            let (s, t) = edges[f];
            vec![st.relations[f].clone(), st.loci[s].clone(), st.loci[t].clone()]
        }).collect()
    };
    let edge_obs = |list: &[usize]| -> Vec<DeclId> {
        list.iter().flat_map(|&f| {
            let (s, t) = edges[f];
            let mut v = relation_observables(reg, &st.relations[f]);
            v.extend(locus_observables(reg, &st.loci[s]));
            v.extend(locus_observables(reg, &st.loci[t]));
            v
        }).collect()
    };
    let selection_obs = st.selection_basis.as_ref().map(|b| b.observables.clone()).unwrap_or_default();
    let mut entries = Vec::new();
    let mut reverse_traces = Vec::new();
    for e in 0..edges.len() {
        entries.push(ReconstructedEntry {
            step: Step::Delta, relation: st.relations[e].clone(), component: 0,
            value: delta.field[0][e], contributing: sorted(edge_ids(&[e])),
        });
        let mut obs = edge_obs(&[e]);
        obs.extend(selection_obs.iter().cloned());
        reverse_traces.push(ReverseTrace { entry: format!("Delta[{}]", st.relations[e]), termini: termini(reg, obs) });
    }
    if let Some(sig) = &sigma {
        for e in 0..edges.len() {
            let (ap, am, inc) = &adjacency[e];
            let mut c = edge_ids(&[e]);
            c.extend(edge_ids(ap));
            c.extend(edge_ids(am));
            c.extend(edge_ids(inc));
            c.extend(parameters.iter().map(|p| p.observable_ref.clone()));
            entries.push(ReconstructedEntry {
                step: Step::Sigma, relation: st.relations[e].clone(), component: 0,
                value: sig.field[0][e], contributing: sorted(c),
            });
            let mut obs = edge_obs(&[e]);
            obs.extend(edge_obs(ap));
            obs.extend(edge_obs(am));
            obs.extend(edge_obs(inc));
            obs.extend(parameters.iter().map(|p| p.observable_ref.clone()));
            obs.extend(selection_obs.iter().cloned());
            reverse_traces.push(ReverseTrace { entry: format!("Sigma[{}]", st.relations[e]), termini: termini(reg, obs) });
        }
    }

    Ok(Reconstruction {
        inputs, relation_ids: st.relations.clone(), edges, classes, adjacency, path, parameters,
        entries, reverse_traces, not_evaluated, delta: Some(delta), sigma, edge_quantity: None,
    })
}

/// Resolve and reconstruct a declared edge M projection from the registry.
fn reconstruct_edge_m(reg: &DeclarationRegistry, rid: &str) -> Result<Reconstruction, Vec<OpenCondition>> {
    let mut rs = Resolver { reg, open: vec![] };
    if reg.relation(rid).is_none() {
        return Err(vec![OpenCondition::Unverifiable { kind: DeclKind::Relation, id: rid.into() }]);
    }
    rs.relation(rid);
    if !rs.open.is_empty() {
        return Err(rs.open);
    }
    let r = reg.relation(rid).expect("resolved");
    let o = reg.observable(&r.observable_source).expect("resolved");
    let m = reg.mapping(&r.m_mapping_ref).expect("resolved");
    let d = reg.domain(&r.declared_domain).expect("resolved");
    let v = project(reg, o.reported_value, m);
    if !v.is_finite() || v < d.floor || v > d.ceiling {
        return Err(vec![OpenCondition::OutsideDeclaredExtent { binding: rid.into(), value: v, floor: d.floor, ceiling: d.ceiling }]);
    }
    let mut contributing = vec![r.id.clone(), o.id.clone(), m.id.clone()];
    if let MMappingKind::KernelDeclaredFunction { references, .. } = &m.kind {
        contributing.extend(references.iter().cloned());
    }
    Ok(Reconstruction {
        inputs: vec![ReconstructedInput { binding: rid.into(), observable: o.id.clone(), m_mapping: m.id.clone(), projected: v }],
        relation_ids: vec![], edges: vec![], classes: vec![], adjacency: vec![],
        path: vec![Step::MProjection], parameters: vec![],
        entries: vec![ReconstructedEntry {
            step: Step::MProjection, relation: rid.into(), component: 0, value: v, contributing: sorted(contributing),
        }],
        reverse_traces: vec![ReverseTrace { entry: format!("MProjection[{rid}]"), termini: termini(reg, relation_observables(reg, rid)) }],
        not_evaluated: vec![], delta: None, sigma: None, edge_quantity: Some(v),
    })
}

// ── Admission ─────────────────────────────────────────────────────────────

fn bits_eq(a: f64, b: f64) -> bool {
    a.to_bits() == b.to_bits()
}

fn disconnected(link: Link, entry: impl Into<String>, reconstructed: impl Into<String>, recorded: impl Into<String>)
    -> AdmissionOutcome
{
    AdmissionOutcome::Rejected(RppiViolation::Disconnected {
        link, entry: entry.into(), reconstructed: reconstructed.into(), recorded: recorded.into(),
    })
}

/// Every id the record cites must exist in the registry; otherwise the claim
/// cannot be verified and remains OPEN.
fn record_references(rec: &CalculationRecord, reg: &DeclarationRegistry) -> Vec<OpenCondition> {
    let mut open = Vec::new();
    let mut need = |kind: DeclKind, id: &str, found: bool| {
        let c = OpenCondition::Unverifiable { kind, id: id.into() };
        if !found && !open.contains(&c) {
            open.push(c);
        }
    };
    for i in &rec.inputs {
        let is_locus = reg.locus(&i.binding_ref).is_some();
        let is_rel = reg.relation(&i.binding_ref).is_some();
        need(DeclKind::Locus, &i.binding_ref, is_locus || is_rel);
        need(DeclKind::Observable, &i.observable_ref, reg.observable(&i.observable_ref).is_some());
        need(DeclKind::Mapping, &i.m_mapping_ref, reg.mapping(&i.m_mapping_ref).is_some());
    }
    for r in &rec.relations {
        need(DeclKind::Relation, &r.relation_ref, reg.relation(&r.relation_ref).is_some());
    }
    for p in &rec.parameters {
        need(DeclKind::Observable, &p.observable_ref, reg.observable(&p.observable_ref).is_some());
    }
    for o in &rec.outputs {
        need(DeclKind::Relation, &o.relation_ref, reg.relation(&o.relation_ref).is_some());
    }
    open
}

fn status(traces: &[ReverseTrace]) -> ObservationalStatus {
    let kinds: Vec<&OriginKind> = traces.iter().flat_map(|t| t.termini.iter().map(|x| &x.origin_kind)).collect();
    if kinds.iter().any(|k| **k == OriginKind::SyntheticFixture) {
        ObservationalStatus::Synthetic
    } else if kinds.iter().any(|k| **k == OriginKind::Simulated) {
        ObservationalStatus::Simulated
    } else {
        ObservationalStatus::Measured
    }
}

/// V9 admission interface. Only a ProvenancedOutput can be submitted:
///
/// ```compile_fail
/// use abr_kernel::admission::admit;
/// use abr_kernel::provenance::DeclarationRegistry;
/// use abr_kernel::operators::{operator_delta, DeclaredRelations, EdgeProvenance, NodeField};
/// let rel = DeclaredRelations::from_edges_with_provenance(2, vec![(0, 1)], vec![EdgeProvenance::Continuation]);
/// let bare = operator_delta(&NodeField::new(vec![vec![3.0, 1.0]]), &rel);
/// // A bare operator output carries no calculation record: it cannot be submitted.
/// let _ = admit(&bare, &DeclarationRegistry::default());
/// ```
pub fn admit(submitted: &ProvenancedOutput, reg: &DeclarationRegistry) -> AdmissionOutcome {
    let rec = &submitted.record;

    // 1–2. Resolve and reconstruct from the registry alone.
    let recon = match &rec.calculation {
        Calculation::Primary { structure_ref } => reconstruct_primary(reg, structure_ref),
        Calculation::EdgeMProjection { relation_ref } => reconstruct_edge_m(reg, relation_ref),
    };
    let recon = match recon {
        Ok(r) => r,
        Err(open) => return AdmissionOutcome::Open(open),
    };

    // 3. The record's own references.
    let open = record_references(rec, reg);
    if !open.is_empty() {
        return AdmissionOutcome::Open(open);
    }

    // 4a. Fabricated: claimed declaration content vs the authoritative declaration.
    for i in &rec.inputs {
        let o = reg.observable(&i.observable_ref).expect("checked");
        let declared_source = o.record.as_ref().map(|r| r.source.clone()).unwrap_or_default();
        if i.claimed_source != declared_source {
            return AdmissionOutcome::Rejected(RppiViolation::Fabricated {
                id: o.id.clone(), field: "source".into(), declared: declared_source, recorded: i.claimed_source.clone(),
            });
        }
        if !bits_eq(i.observed_value, o.reported_value) {
            return AdmissionOutcome::Rejected(RppiViolation::Fabricated {
                id: o.id.clone(), field: "reported_value".into(),
                declared: format!("{:e}", o.reported_value), recorded: format!("{:e}", i.observed_value),
            });
        }
    }
    for p in &rec.parameters {
        let o = reg.observable(&p.observable_ref).expect("checked");
        if !bits_eq(p.value, o.reported_value) {
            return AdmissionOutcome::Rejected(RppiViolation::Fabricated {
                id: o.id.clone(), field: format!("parameter {}", p.name),
                declared: format!("{:e}", o.reported_value), recorded: format!("{:e}", p.value),
            });
        }
    }

    // 4b. Misattributed: a claimed relation that is not the producing relation.
    for (e, r) in rec.relations.iter().enumerate() {
        if let Some(expected) = recon.relation_ids.get(e) {
            let traversal_matches_producer = recon.edges[e] == (r.src_index, r.tgt_index) && recon.classes[e] == r.class;
            if r.relation_ref != *expected && traversal_matches_producer {
                return AdmissionOutcome::Rejected(RppiViolation::Misattributed {
                    entry: format!("relation[{e}]"), claimed: r.relation_ref.clone(), reconstructed: expected.clone(),
                });
            }
        }
    }
    for (k, out) in rec.outputs.iter().enumerate() {
        if let Some(ent) = recon.entries.iter().find(|x| x.step == out.step && x.component == out.component
            && bits_eq(x.value, out.value) && x.relation != out.relation_ref)
        {
            // Same step, same value, but attributed to a different declared relation.
            let unique_by_value = recon.entries.iter()
                .filter(|x| x.step == out.step && bits_eq(x.value, out.value)).count() == 1;
            let claimed_is_produced = recon.entries.iter().any(|x| x.step == out.step && x.relation == out.relation_ref);
            if unique_by_value && !claimed_is_produced {
                return AdmissionOutcome::Rejected(RppiViolation::Misattributed {
                    entry: format!("output[{k}]"), claimed: out.relation_ref.clone(), reconstructed: ent.relation.clone(),
                });
            }
        }
    }

    // 4c. Kernel selection: the Primary kernel admits Δ and Σ only.
    if let Calculation::Primary { .. } = rec.calculation {
        if rec.path.iter().any(|s| matches!(s, Step::A | Step::B | Step::R | Step::MProjection)) {
            return AdmissionOutcome::Rejected(RppiViolation::KernelSelection {
                declared: "Primary (Δ → Σ; B not evaluated)".into(), recorded_path: rec.path.clone(),
            });
        }
    }

    // 4d. Disconnected — input bindings.
    for ri in &recon.inputs {
        let Some(i) = rec.inputs.iter().find(|i| i.binding_ref == ri.binding) else {
            return disconnected(Link::InputBinding, &ri.binding, format!("{} via {}", ri.observable, ri.m_mapping), "no recorded input");
        };
        if i.observable_ref != ri.observable || i.m_mapping_ref != ri.m_mapping {
            return disconnected(Link::InputBinding, &ri.binding,
                format!("{} via {}", ri.observable, ri.m_mapping), format!("{} via {}", i.observable_ref, i.m_mapping_ref));
        }
        if !bits_eq(i.projected_value, ri.projected) {
            return disconnected(Link::InputBinding, &ri.binding, format!("{:e}", ri.projected), format!("{:e}", i.projected_value));
        }
    }
    if rec.inputs.len() != recon.inputs.len() {
        return disconnected(Link::InputBinding, "inputs", format!("{} declared inputs", recon.inputs.len()),
            format!("{} recorded inputs", rec.inputs.len()));
    }

    // 4d. Disconnected — traversal.
    if rec.path != recon.path {
        return disconnected(Link::Traversal, "path", format!("{:?}", recon.path), format!("{:?}", rec.path));
    }
    if rec.relations.len() != recon.edges.len() {
        return disconnected(Link::Traversal, "relations", format!("{} declared relations", recon.edges.len()),
            format!("{} recorded relations", rec.relations.len()));
    }
    for (e, r) in rec.relations.iter().enumerate() {
        let (ap, am, inc) = &recon.adjacency[e];
        let reconstructed = format!("{} ({}→{}) {:?} adj+{:?} adj-{:?} inc{:?}",
            recon.relation_ids[e], recon.edges[e].0, recon.edges[e].1, recon.classes[e], ap, am, inc);
        let recorded = format!("{} ({}→{}) {:?} adj+{:?} adj-{:?} inc{:?}",
            r.relation_ref, r.src_index, r.tgt_index, r.class, r.adj_plus, r.adj_minus, r.rho_incident);
        if reconstructed != recorded {
            return disconnected(Link::Traversal, format!("relation[{e}]"), reconstructed, recorded);
        }
    }
    if rec.parameters != recon.parameters {
        return disconnected(Link::Traversal, "parameters", format!("{:?}", recon.parameters), format!("{:?}", rec.parameters));
    }

    // 4d. Disconnected — output derivation (record and the carried output).
    if rec.outputs.len() != recon.entries.len() {
        return disconnected(Link::OutputDerivation, "outputs", format!("{} reconstructed entries", recon.entries.len()),
            format!("{} recorded entries", rec.outputs.len()));
    }
    for ent in &recon.entries {
        let name = format!("{:?}[{}]", ent.step, ent.relation);
        let Some(out) = rec.outputs.iter().find(|o| o.step == ent.step && o.relation_ref == ent.relation && o.component == ent.component) else {
            return disconnected(Link::OutputDerivation, &name, format!("{:e}", ent.value), "no recorded entry");
        };
        if !bits_eq(out.value, ent.value) {
            return disconnected(Link::OutputDerivation, &name, format!("{:e}", ent.value), format!("{:e}", out.value));
        }
        if sorted(out.contributing.clone()) != ent.contributing {
            return disconnected(Link::OutputDerivation, &name, format!("{:?}", ent.contributing), format!("{:?}", out.contributing));
        }
    }
    let carried_matches = match (&submitted.output, &recon) {
        (CalculatedOutput::Primary { delta, sigma }, r) if r.delta.is_some() => {
            let same = |a: &PrimaryEdgeField, b: &PrimaryEdgeField| {
                a.field.len() == b.field.len() && a.field.iter().zip(&b.field).all(|(x, y)| {
                    x.len() == y.len() && x.iter().zip(y).all(|(p, q)| bits_eq(*p, *q))
                })
            };
            same(delta, r.delta.as_ref().unwrap()) && match (sigma, &r.sigma) {
                (Some(s), Some(t)) => same(s, t),
                (None, None) => true,
                _ => false,
            }
        }
        (CalculatedOutput::EdgeQuantity { value }, r) if r.edge_quantity.is_some() => bits_eq(*value, r.edge_quantity.unwrap()),
        _ => false,
    };
    if !carried_matches {
        return disconnected(Link::OutputDerivation, "carried output",
            "output of the reconstructed calculation", "carried output differs from the reconstruction");
    }

    // 5. Admitted.
    AdmissionOutcome::Admitted(AdmissionEvidence {
        observational_status: status(&recon.reverse_traces),
        inputs: recon.inputs,
        entries: recon.entries,
        reverse_traces: recon.reverse_traces,
        not_evaluated: recon.not_evaluated,
        comparison_rule: COMPARISON_RULE,
    })
}
