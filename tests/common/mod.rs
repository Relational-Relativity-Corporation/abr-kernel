//! Shared declaration fixtures for the Kernel V9 RPPI tests.
//!
//! Two kinds, kept strictly apart:
//!
//! 1. `halpha_canonical_registry` — ONLY what the canonical repository declares
//!    for the H-alpha photon edge (derived_invariants.rs, provenance_demo.rs,
//!    kernel_reference.md §13.1). Text is quoted verbatim. Every declaration
//!    the repository does not contain is left absent; nothing is supplied.
//!
//! 2. `synthetic_*` — SYNTHETIC FIXTURES (kernel_reference.md §17). Every
//!    observable is `OriginKind::SyntheticFixture`, names a synthetic record,
//!    and uses synthetic values. They test enforcement behaviour only and
//!    cannot establish empirical observational validity.
#![allow(dead_code)]

use abr_kernel::derived_invariants::{C_DECLARED, H_PLANCK};
use abr_kernel::operators::EdgeProvenance;
use abr_kernel::provenance::*;

pub fn basis(observables: &[&str], statement: &str) -> Option<ObservationalBasis> {
    Some(ObservationalBasis {
        observables: observables.iter().map(|s| s.to_string()).collect(),
        statement: statement.to_string(),
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// H-alpha photon edge — canonical declarations only
// ═══════════════════════════════════════════════════════════════════════════

pub const HA_LAMBDA: &str = "obs.nist_asd_v5_12.h_n3_n2.lambda";
pub const HA_REF_H: &str = "ref.kernel_13_1.h_planck";
pub const HA_REF_C: &str = "ref.kernel_13_1.c_declared";
pub const HA_M: &str = "m.atomic.photon_edge.hc_over_lambda";
pub const HA_DOMAIN: &str = "domain.atomic.photon_edge";
pub const HA_EDGE: &str = "rel.photon_edge.h_alpha";

pub fn halpha_canonical_registry() -> DeclarationRegistry {
    let mut reg = DeclarationRegistry::default();
    // λ: provenance_demo.rs — "NIST ASD v5.12, H n=3->2 transition
    // (Balmer-alpha / H-alpha)", 656.279e-9 m. The canonical repository does
    // not declare the measurement conditions (e.g. air or vacuum wavelength).
    reg.observables.push(ObservableDeclaration {
        id: HA_LAMBDA.into(),
        origin_kind: OriginKind::Measured,
        record: Some(ExternalRecord {
            source: "NIST ASD v5.12".into(),
            entry: "H n=3->2 transition (Balmer-alpha / H-alpha)".into(),
        }),
        declared_conditions: None,
        reported_value: 656.279e-9,
        unit: "m".into(),
    });
    // H_PLANCK, C_DECLARED: kernel_reference.md §13.1 declared measurement
    // references, with the observable provenance stated there. Measurement
    // conditions are not declared in the canonical repository.
    reg.observables.push(ObservableDeclaration {
        id: HA_REF_H.into(),
        origin_kind: OriginKind::Measured,
        record: Some(ExternalRecord {
            source: "Spectroscopic frequency-energy measurement".into(),
            entry: "H_PLANCK, declared measurement reference (kernel_reference.md §13.1)".into(),
        }),
        declared_conditions: None,
        reported_value: H_PLANCK,
        unit: "J·s".into(),
    });
    reg.observables.push(ObservableDeclaration {
        id: HA_REF_C.into(),
        origin_kind: OriginKind::Measured,
        record: Some(ExternalRecord {
            source: "BIPM 2019 interferometric measurement".into(),
            entry: "C_DECLARED, declared measurement reference (kernel_reference.md §13.1)".into(),
        }),
        declared_conditions: None,
        reported_value: C_DECLARED,
        unit: "m/s".into(),
    });
    // M: derived_invariants.rs, verbatim.
    reg.mappings.push(MMappingDeclaration {
        id: HA_M.into(),
        kind: MMappingKind::KernelDeclaredFunction {
            function: KernelFunction::EpsilonPhotonEdge,
            references: vec![HA_REF_H.into(), HA_REF_C.into()],
        },
        input_unit: "m".into(),
        output_unit: "J".into(),
        basis: basis(&[HA_LAMBDA],
            "M maps spectrometer-reported wavelength λ (m) to ε[e] (J). The formula hc/λ is \
             part of M at the Atomic Region. No theoretical assumptions beyond the declared \
             measurement mapping M."),
    });
    // No declared domain (observable extent) for photon-edge ε exists in the
    // canonical repository: HA_DOMAIN is referenced and absent.
    //
    // Relation: direction declared from the emission observable (verbatim).
    // Endpoint names from the canonical text. No EdgeProvenance class is
    // declared for photon edges in the canonical repository.
    reg.relations.push(ObservableRelationDeclaration {
        id: HA_EDGE.into(),
        src_locus: "H n=3 (emitting locus)".into(),
        tgt_locus: "detection locus".into(),
        observable_source: HA_LAMBDA.into(),
        m_mapping_ref: HA_M.into(),
        declared_domain: HA_DOMAIN.into(),
        direction_basis: basis(&[HA_LAMBDA],
            "Edge direction: declared from each observation through M. The emission \
             observable (spectrometer-reported wavelength) establishes the direction of \
             this edge."),
        class: None,
        class_basis: None,
    });
    reg
}

// ═══════════════════════════════════════════════════════════════════════════
// SYNTHETIC FIXTURES — enforcement behaviour only
// ═══════════════════════════════════════════════════════════════════════════

pub const SYN_SOURCE: &str = "SYNTHETIC FIXTURE — not an observation";

pub fn syn_obs(id: &str, value: f64, unit: &str) -> ObservableDeclaration {
    ObservableDeclaration {
        id: id.into(),
        origin_kind: OriginKind::SyntheticFixture,
        record: Some(ExternalRecord {
            source: SYN_SOURCE.into(),
            entry: format!("tests/common/mod.rs fixture {id}"),
        }),
        declared_conditions: Some("synthetic fixture: no measurement conditions exist; \
                                   tests enforcement behaviour only".into()),
        reported_value: value,
        unit: unit.into(),
    }
}

pub const SYN_BASIS: &str = "syn.obs.basis";
pub const SYN_M: &str = "syn.m.identity";
pub const SYN_DOMAIN: &str = "syn.domain";
pub const SYN_REL_01: &str = "syn.rel.01";
pub const SYN_REL_12: &str = "syn.rel.12";
pub const SYN_REL_01_COUPLING: &str = "syn.rel.01.coupling";
pub const SYN_DELTA: &str = "syn.st.delta";
pub const SYN_CHAIN: &str = "syn.st.chain";

fn syn_relation(id: &str, src: &str, tgt: &str, class: EdgeProvenance) -> ObservableRelationDeclaration {
    ObservableRelationDeclaration {
        id: id.into(),
        src_locus: src.into(),
        tgt_locus: tgt.into(),
        observable_source: SYN_BASIS.into(),
        m_mapping_ref: SYN_M.into(),
        declared_domain: SYN_DOMAIN.into(),
        direction_basis: basis(&[SYN_BASIS], "synthetic: direction declared for enforcement testing"),
        class: Some(class),
        class_basis: basis(&[SYN_BASIS], "synthetic: class declared for enforcement testing"),
    }
}

/// Synthetic Primary-kernel registry.
///
/// `syn.st.delta`: loci 0, 1; relation 0→1; no ρ_base, χ₀ (Σ not evaluated).
/// `syn.st.chain`: loci 0, 1, 2; relations 0→1, 1→2; ρ_base, χ₀ declared
/// (synthetic) so Σ is evaluated. A second relation over 0→1 with a different
/// class (`syn.rel.01.coupling`) is declared for the misattribution case.
pub fn synthetic_primary_registry() -> DeclarationRegistry {
    let mut reg = DeclarationRegistry::default();
    for (id, v) in [("syn.obs.a", 3.0), ("syn.obs.b", 1.0), ("syn.obs.c", 2.0), (SYN_BASIS, 0.0)] {
        reg.observables.push(syn_obs(id, v, "u"));
    }
    reg.observables.push(syn_obs("syn.obs.rho_base", 0.5, "dimensionless"));
    reg.observables.push(syn_obs("syn.obs.chi_0", 1.0, "u"));
    reg.mappings.push(MMappingDeclaration {
        id: SYN_M.into(),
        kind: MMappingKind::Identity,
        input_unit: "u".into(),
        output_unit: "u".into(),
        basis: basis(&[SYN_BASIS], "synthetic: the reported value enters D unchanged in unit u"),
    });
    reg.domains.push(DeclaredDomain {
        id: SYN_DOMAIN.into(),
        floor: -10.0,
        ceiling: 10.0,
        unit: "u".into(),
        basis: basis(&[SYN_BASIS], "synthetic: extent declared for enforcement testing"),
    });
    for (l, o) in [("syn.locus.0", "syn.obs.a"), ("syn.locus.1", "syn.obs.b"), ("syn.locus.2", "syn.obs.c")] {
        reg.loci.push(LocusDeclaration {
            id: l.into(), observable_ref: o.into(), m_mapping_ref: SYN_M.into(), domain_ref: SYN_DOMAIN.into(),
        });
    }
    reg.relations.push(syn_relation(SYN_REL_01, "syn.locus.0", "syn.locus.1", EdgeProvenance::Continuation));
    reg.relations.push(syn_relation(SYN_REL_12, "syn.locus.1", "syn.locus.2", EdgeProvenance::Continuation));
    reg.relations.push(syn_relation(SYN_REL_01_COUPLING, "syn.locus.0", "syn.locus.1", EdgeProvenance::Coupling));
    reg.structures.push(RelationalDeclaration {
        id: SYN_DELTA.into(),
        loci: vec!["syn.locus.0".into(), "syn.locus.1".into()],
        relations: vec![SYN_REL_01.into()],
        kernel_selection: KernelSelection::Primary,
        selection_basis: basis(&[SYN_BASIS], "synthetic: M declared not to resolve an interior locus"),
        rho_base_ref: None,
        chi_0_ref: None,
    });
    reg.structures.push(RelationalDeclaration {
        id: SYN_CHAIN.into(),
        loci: vec!["syn.locus.0".into(), "syn.locus.1".into(), "syn.locus.2".into()],
        relations: vec![SYN_REL_01.into(), SYN_REL_12.into()],
        kernel_selection: KernelSelection::Primary,
        selection_basis: basis(&[SYN_BASIS], "synthetic: M declared not to resolve an interior locus"),
        rho_base_ref: Some("syn.obs.rho_base".into()),
        chi_0_ref: Some("syn.obs.chi_0".into()),
    });
    reg
}

pub const SYN_PHOTON_EDGE: &str = "syn.rel.photon_edge";

/// Synthetic photon-edge registry. λ is a synthetic value (500e-9), not a
/// measured line. The two measurement references are synthetic declarations
/// whose declared values are the frozen H_PLANCK and C_DECLARED: they exercise
/// the reference correspondence check and establish nothing empirical.
pub fn synthetic_photon_registry() -> DeclarationRegistry {
    let mut reg = DeclarationRegistry::default();
    reg.observables.push(syn_obs("syn.obs.lambda", 500e-9, "m"));
    reg.observables.push(syn_obs("syn.ref.h", H_PLANCK, "J·s"));
    reg.observables.push(syn_obs("syn.ref.c", C_DECLARED, "m/s"));
    reg.mappings.push(MMappingDeclaration {
        id: "syn.m.hc_over_lambda".into(),
        kind: MMappingKind::KernelDeclaredFunction {
            function: KernelFunction::EpsilonPhotonEdge,
            references: vec!["syn.ref.h".into(), "syn.ref.c".into()],
        },
        input_unit: "m".into(),
        output_unit: "J".into(),
        basis: basis(&["syn.obs.lambda"], "synthetic: λ (m) projected to ε[e] (J) by hc/λ"),
    });
    reg.domains.push(DeclaredDomain {
        id: "syn.domain.photon".into(),
        floor: 0.0,
        ceiling: 1e-17,
        unit: "J".into(),
        basis: basis(&["syn.obs.lambda"], "synthetic: extent declared for enforcement testing"),
    });
    reg.relations.push(ObservableRelationDeclaration {
        id: SYN_PHOTON_EDGE.into(),
        src_locus: "syn emitting locus".into(),
        tgt_locus: "syn detection locus".into(),
        observable_source: "syn.obs.lambda".into(),
        m_mapping_ref: "syn.m.hc_over_lambda".into(),
        declared_domain: "syn.domain.photon".into(),
        direction_basis: basis(&["syn.obs.lambda"], "synthetic: direction declared for enforcement testing"),
        class: Some(EdgeProvenance::Coupling),
        class_basis: basis(&["syn.obs.lambda"], "synthetic: class declared for enforcement testing"),
    });
    reg
}
