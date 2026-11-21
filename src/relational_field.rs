// relational_field.rs — Metatron Dynamics, Inc. Kernel V10.
// Df6, simultaneity (Origin, 2026-11-21). Bounded over D. No claim beyond D.
//
// ── Df6 ───────────────────────────────────────────────────────────────────
//
// All operators and all ground variables occur simultaneously throughout
// relational evolution. A → B → R, the persistence forms A_p → B_p → R_p, and
// energy, freedom and connection are aspects of one ongoing relational
// evolution, not stages. The composition order R(B(A(x))) and the sequence of
// registrations are orders of analysis (Df2a). The observer intersects the
// system (R10). The only limit on a reading is declared information: too few
// registrations, too few components, too coarse a δ, or no declared carrier.
//
// Consequences in this module:
//
//   1. There are no phases. The spatial and persistence operators are both
//      evaluated on every relational field, at every registration and every
//      consecutive step.
//
//   2. A relational field is formed from a declared sequence of at least two
//      consecutive registrations of the same declared relations, ordered by
//      an observable property of the process (T6). One registration is
//      insufficient information: nothing about relational evolution can
//      terminate in one observation. The operators still compute on a single
//      registration (`operators.rs`), but the result is a building block, not
//      a relational field.
//
//   3. `RelationalField::new` evaluates everything together: the spatial
//      operators at each registration, persistence over each consecutive step
//      (the prior is always the earlier actual output, Df2), and energy,
//      freedom and connection over the whole sequence. None is available
//      without the others.
//
// Two registrations are the minimum information, not a guarantee of enough.
// Telling reorganization from an unresolved fluctuation may need more
// registrations or a finer δ; under W2 that remains a statement about the
// declaration.
//
// The operator formulas are unchanged. This module calls `operator_e` and
// `operator_e_v5` from `operators.rs` and the readers in `ground_variables.rs`.

use crate::ground_variables::{ground_reading, DeclarationGap, EnergyCarrier, GroundReading, Registration};
use crate::operators::{
    operator_e, operator_e_v5, DeclaredRelations, EdgeField, PersistenceOutput, PersistenceState,
};

/// Operator parameters declared by M for this field (no defaults; V7.1).
#[derive(Debug, Clone, PartialEq)]
pub struct OperatorParameters {
    pub rho_base: f64,
    pub chi_0: f64,
    /// Declared component pairs.
    pub pairs: Vec<(usize, usize)>,
    /// cc[p], one per declared component pair.
    pub cc: Vec<f64>,
}

/// A relational field: one declared intersection with an ongoing relational
/// evolution, evaluated as a whole.
#[derive(Debug, Clone)]
pub struct RelationalField {
    /// The declared sequence of registrations.
    pub registrations: Vec<Registration>,
    /// E = R(B(A(x)), ρ(A(x))) at each registration.
    pub spatial: Vec<EdgeField>,
    /// A_p → B_p → R_p over each consecutive step; `persistence[t]` pairs
    /// registration t (prior) with registration t + 1 (current).
    pub persistence: Vec<PersistenceOutput>,
    /// Energy, freedom and connection over the whole sequence.
    pub ground: GroundReading,
}

impl RelationalField {
    /// Form the relational field from a declared sequence. Fewer than two
    /// registrations is insufficient information: `NoProcessSequence`.
    pub fn new(
        rel: &DeclaredRelations,
        registrations: Vec<Registration>,
        params: &OperatorParameters,
        carriers: Option<&[EnergyCarrier]>,
    ) -> Result<Self, DeclarationGap> {
        if registrations.len() < 2 {
            return Err(DeclarationGap::NoProcessSequence);
        }
        let k = registrations[0].x.k;
        assert!(registrations.iter().all(|r| r.x.k == k),
            "every registration must declare the same components");
        assert!(registrations.iter().all(|r| r.x.n == rel.n_nodes),
            "every registration must cover the declared loci");

        let OperatorParameters { rho_base, chi_0, pairs, cc } = params;
        let mut spatial: Vec<EdgeField> = Vec::with_capacity(registrations.len());
        let mut persistence = Vec::with_capacity(registrations.len() - 1);
        spatial.push(operator_e(&registrations[0].x, rel, pairs, cc, *rho_base, *chi_0));
        for r in &registrations[1..] {
            // The prior is the earlier actual output (Df2); never a substitute.
            let prior = PersistenceState::from_prior(spatial.last().unwrap().clone());
            let (e, p) = operator_e_v5(&r.x, rel, pairs, cc, &prior, *rho_base, *chi_0);
            spatial.push(e);
            persistence.push(p);
        }
        let ground = ground_reading(&registrations, rel, carriers);
        Ok(RelationalField { registrations, spatial, persistence, ground })
    }

    /// Number of consecutive steps in the field.
    pub fn n_steps(&self) -> usize {
        self.persistence.len()
    }
}
