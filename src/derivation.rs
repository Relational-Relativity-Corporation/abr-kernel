// derivation.rs — Metatron Dynamics, Inc. Kernel V10.
// The derivation of every part a relational field reports (Origin,
// 2026-11-25, 92nd). Bounded over D. No claim beyond D.
//
// Origin (92nd): "legacy math is permitted provided that its provenance is
// clear and that it is described in relation to the operators"; "Necessity
// still holds" (W12 as amended, 42nd). Every part a `RelationalField`
// reports, named as the field's own structure names it, is listed here with
// how it is derived: the registered relation itself, an operator reading, a
// reading rule or declared projection applied to operator output, or legacy
// mathematics with its provenance, its description in relation to A, B and
// Ξ, and its necessity. `tests/v10_derivation.rs` fails on any reported part
// not listed, and on any legacy entry missing one of the three.

/// The kernel's operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    /// A: the relation as registered (or the incidence difference x[s] − x[t]).
    A,
    /// B: a relation accumulated with its declared successors.
    B,
    /// Ξ: B over successors less B over predecessors.
    Xi,
}

/// The primitive and the three variables (71st).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reads {
    Relation,
    Energy,
    Connection,
    Freedom,
}

/// How a part is derived.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Derivation {
    /// The registered relations, or a declaration the field carries with
    /// them (identity, bound, order, provenance): relation, the primitive.
    Registered { what: &'static str },
    /// The identity of a reported part (which relation, component, step):
    /// every part is reported with its identity (§1.5).
    Identity { what: &'static str },
    /// A linear or multilinear form of the operators' output.
    Operator { operators: &'static [Operator], reads: &'static [Reads], form: &'static str },
    /// A reading rule (at δ) or a declared projection (C, K6: shares and
    /// counts) applied to operator readings; not itself an operator.
    Rule { of: &'static [Operator], reads: &'static [Reads], rule: &'static str },
    /// Legacy mathematics (W12 as amended, 42nd, 92nd).
    Legacy {
        method: &'static str,
        provenance: &'static str,
        in_relation_to_operators: &'static str,
        necessity: Option<&'static str>,
    },
}

/// Every part a relational field reports, by the name its structure gives
/// it, with its derivation.
pub fn register() -> &'static [(&'static str, Derivation)] {
    REGISTER
}

/// Legacy methods in use at the code layer, which no part reports by name
/// (Part 7, OC-LM-1).
pub fn legacy_in_use() -> &'static [Derivation] {
    LEGACY_IN_USE
}

/// The derivation of a reported part, if one is declared.
pub fn of_part(name: &str) -> Option<&'static Derivation> {
    REGISTER.iter().find(|(n, _)| *n == name).map(|(_, d)| d)
}

/// Why a derivation does not meet the 92nd, or `Ok`.
pub fn check(d: &Derivation) -> Result<(), &'static str> {
    let stated = |s: &str| !s.trim().is_empty();
    match d {
        Derivation::Registered { what } | Derivation::Identity { what } =>
            if stated(what) { Ok(()) } else { Err("what it is, not stated") },
        Derivation::Operator { operators, reads, form } =>
            if operators.is_empty() { Err("an operator reading names no operator") }
            else if reads.is_empty() { Err("an operator reading names nothing it reads") }
            else if !stated(form) { Err("its form, not stated") } else { Ok(()) },
        Derivation::Rule { of, reads, rule } =>
            if of.is_empty() { Err("a rule names no operator it applies to") }
            else if reads.is_empty() { Err("a rule names nothing it reads") }
            else if !stated(rule) { Err("the rule, not stated") } else { Ok(()) },
        Derivation::Legacy { method, provenance, in_relation_to_operators, necessity } =>
            if !stated(method) { Err("legacy: the method, not named") }
            else if !stated(provenance) { Err("legacy: provenance not clear") }
            else if !stated(in_relation_to_operators) { Err("legacy: not described in relation to the operators") }
            else if !necessity.is_some_and(stated) { Err("legacy: necessity not identified") }
            else { Ok(()) },
    }
}

/// The parts a structure reports, read from its `Debug` form: every field of
/// every struct in it, named `Struct.field` by the struct (or struct-like enum
/// variant) that holds it, outside quoted text; each once, in order of first
/// appearance.
pub fn reported_names(debug: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let mut holders: Vec<String> = Vec::new();
    let mut last_ident = String::new();
    let b = debug.as_bytes();
    let (mut i, mut in_str) = (0usize, false);
    while i < b.len() {
        let c = b[i];
        if in_str {
            if c == b'\\' { i += 2; continue; }
            if c == b'"' { in_str = false; }
            i += 1;
            continue;
        }
        match c {
            b'"' => { in_str = true; i += 1; }
            b'{' => { holders.push(std::mem::take(&mut last_ident)); i += 1; }
            b'}' => { holders.pop(); i += 1; }
            _ if c.is_ascii_alphabetic() || c == b'_' => {
                let start = i;
                while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') { i += 1; }
                let ident = &debug[start..i];
                let before = debug[..start].trim_end();
                if debug[i..].starts_with(": ") && (before.ends_with('{') || before.ends_with(',')) {
                    let n = format!("{}.{}", holders.last().map(String::as_str).unwrap_or("?"), ident);
                    if !names.contains(&n) { names.push(n); }
                } else {
                    last_ident = ident.to_string();
                }
            }
            _ => { i += 1; }
        }
    }
    names
}

/// Reported names with no declared derivation.
pub fn underived(names: &[String]) -> Vec<String> {
    names.iter().filter(|n| of_part(n).is_none()).cloned().collect()
}

use Derivation::{Identity as Id, Operator as Op, Registered as Reg, Rule};
use Operator::{A, B, Xi};
use Reads::{Connection as Con, Energy as En, Freedom as Fr, Relation as Rel};

// Every reading that compares pairs of relations states its scope (92nd:
// the shape of attention when taken over every pair). Recorded, not decided:
// whether these are fenced from the point of analysis is Origin's.
static REGISTER: &[(&str, Derivation)] = &[
    // ── The relational field and its registrations ────────────────────────
    ("RelationalField.registrations", Reg { what: "the declared registrations at the loci, when the relations were formed from loci (empty when registered directly); kept as registered" }),
    ("Registration.range", Reg { what: "the observed range of the registering chain (OC-RANGE-1, 47th), declared with the registration" }),
    ("Synthetic.basis", Reg { what: "the basis of a synthetic observed range: a test fixture of the forms, not an observation (47th)" }),
    ("Registration.x", Reg { what: "the registered values at the loci (a NodeField)" }),
    ("NodeField.data", Reg { what: "registered value data[c][i] at component c and locus i" }),
    ("NodeField.k", Id { what: "the number of declared components" }),
    ("NodeField.n", Id { what: "the number of declared loci" }),
    ("Registration.delta", Reg { what: "the repeatability bound δ[c][i] declared with each registered value" }),
    ("Registration.offset", Reg { what: "the declared run-constant offset bound (Z1a), state forms only" }),
    ("Registration.locus_gain", Reg { what: "the declared run-constant gain bound not shared between loci (Z1a)" }),
    ("Registration.shared_gain", Reg { what: "the declared run-constant gain bound shared by a component's loci (Z1a)" }),
    ("RelationalField.relations", Reg { what: "the registrations of the relations: the primitive every form is read from (59th)" }),
    ("EdgeSeries.a", Op { operators: &[A], reads: &[Rel], form: "A[t][c][e]: the relation as registered, or x[s] − x[t] from the loci (K1), the nearest f64 to the exact value" }),
    ("EdgeSeries.a_lo", Op { operators: &[A], reads: &[Rel], form: "the exact remainder of A: the relation value is a + a_lo exactly (OC-NUM-1, 45th)" }),
    ("EdgeSeries.da", Rule { of: &[A], reads: &[Rel], rule: "the state bound δA of each relation: δ[s] + δ[t] from the loci, or as registered, with the run-constant parts (Z1a); rounded up, never down (41st)" }),
    ("EdgeSeries.da_rep", Rule { of: &[A], reads: &[Rel], rule: "the repeatability part of δA (Z1a)" }),
    ("EdgeSeries.dgain", Rule { of: &[A], reads: &[Fr], rule: "the unshared-gain part of the change bound: γ_s·|Δx_s| + γ_t·|Δx_t| (Z1a)" }),
    ("EdgeSeries.shared_gain", Rule { of: &[A], reads: &[Fr], rule: "the shared gain bound [c][e]: enters reported change magnitudes and K7 closures (Z1a; I-SC-D15 (v))" }),
    ("EdgeSeries.mixed_gain", Rule { of: &[A], reads: &[Con, Fr], rule: "the independent gain bound on relations registered directly: enters every minor that mixes components (Z1a)" }),
    ("EdgeSeries.closure_gain", Rule { of: &[A], reads: &[Con], rule: "the gain bound differing between relations of one component: enters identity, Ξ and K7 closures (I-SC-D15 (v))" }),
    ("EdgeSeries.origin", Id { what: "where the relation values came from: formed from loci, or registered directly (ER-1)" }),
    ("EdgeSeries.forced_pairs", Id { what: "pairs of relations whose coupling the declaration fixes (PC-D4): identities, not readings" }),
    // ── A, B and Ξ ────────────────────────────────────────────────────────
    ("RelationalField.spatial", Op { operators: &[A, B, Xi], reads: &[Rel], form: "A, B and Ξ at every registration" }),
    ("RelationalField.persistence", Op { operators: &[A, B, Xi], reads: &[Rel, Fr], form: "A_p = A_{t+1} − A_t, B_p = B(A_p), Ξ_p = Ξ(B_p) over every consecutive step (linear, so B_p and Ξ_p are the differences exactly)" }),
    ("Forms.a", Op { operators: &[A], reads: &[Rel], form: "A at a registration, or A_p over a step, one exact sum rounded once" }),
    ("Forms.b", Op { operators: &[A, B], reads: &[Rel], form: "B(A)[e] = A[e] + Σ_succ A[f], over the relations B joins to e only (92nd)" }),
    ("Forms.circulation", Op { operators: &[A, B, Xi], reads: &[Rel, Con], form: "Ξ(B(A))[e] = Σ_succ B − Σ_pred B, over the relations B joins to e's neighbours only (92nd)" }),
    // ── The ground: energy ────────────────────────────────────────────────
    ("RelationalField.ground", Reg { what: "energy, freedom and connection, read together over the whole sequence (Df6, W11); each part below has its own derivation" }),
    ("GroundReading.energy", Op { operators: &[A], reads: &[En], form: "V3: energy read from a declared carrier, ε = |A(carrier)|, or named as not evaluated when no carrier is declared" }),
    ("Energy.carrier", Id { what: "the declared carrier's name (R9a, V3a)" }),
    ("Energy.unit", Id { what: "the declared carrier's unit" }),
    ("Energy.exchanges", Op { operators: &[A], reads: &[En], form: "ε at every registration and relation" }),
    ("EnergyExchange.edge", Id { what: "the relation an exchange is read on" }),
    ("EnergyExchange.epsilon", Op { operators: &[A], reads: &[En], form: "ε[e] = |A(c)[e]|: the magnitude of A on the carrier (absolute value of a linear operator's output)" }),
    ("EnergyExchange.delta_epsilon", Rule { of: &[A], reads: &[En], rule: "δ_ε[e]: the carrier's bound carried through A (δ_c[s] + δ_c[t], or as registered), rounded up" }),
    ("EnergyExchange.floor_resolved", Rule { of: &[A], reads: &[En], rule: "the tunneling floor at δ: ε > δ_ε, decided exactly" }),
    ("Energy.share_resolved", Rule { of: &[A], reads: &[En], rule: "declared projection (C, K6): the share of (registration, relation) readings with the floor resolved; the parts are reported beside it" }),
    // ── The ground: freedom (PC1–PC3) ─────────────────────────────────────
    ("GroundReading.freedom", Op { operators: &[A], reads: &[Fr], form: "freedom as change permitted: PC1 on each relation, PC2 and PC3 on pairs" }),
    ("PersistenceChange.relations", Op { operators: &[A], reads: &[Fr], form: "PC1 on every relation, component and step: the change A_p read against its bound" }),
    ("RelationChange.step", Id { what: "the step a reading is over (prior always the earlier registration, Df2)" }),
    ("RelationChange.component", Id { what: "the component a reading is in" }),
    ("RelationChange.edge", Id { what: "the relation a reading is on" }),
    ("RelationChange.change", Op { operators: &[A], reads: &[Fr], form: "A_p[c][e] = A_{t+1} − A_t, exact, rounded once" }),
    ("RelationChange.half_width", Rule { of: &[A], reads: &[Fr], rule: "the change bound, from both registrations' bounds and the gains (Z1a), rounded up" }),
    ("RelationChange.reading", Rule { of: &[A], reads: &[Fr], rule: "PC1 at δ: freedom where |A_p| exceeds its bound, structure otherwise, decided exactly" }),
    ("PersistenceChange.across_components", Op { operators: &[A], reads: &[Fr, Con], form: concat!("PC2(a): the 2×2 minor of two relations' changes A_p across components i < j at each step (a multilinear form, the exterior product of two changes).", " Scope: every pair of relations that qualifies (resolved, or moved), whether or not B joins them; the work grows with the square of the relations, and is not counted in `work` (92nd).") }),
    ("Couplings.readings", Op { operators: &[A], reads: &[Fr, Con], form: concat!("the PC2 readings, one per pair of moved relations and pair of components or steps.", " Scope: every pair of relations that qualifies (resolved, or moved), whether or not B joins them; the work grows with the square of the relations, and is not counted in `work` (92nd).") }),
    ("ChangeCoupling.e", Id { what: "the first relation of a PC2 pair" }),
    ("ChangeCoupling.f", Id { what: "the second relation of a PC2 pair" }),
    ("ChangeCoupling.over", Id { what: "what a PC2 minor is taken over: two components at a step, or one component at two steps" }),
    ("Components.step", Id { what: "the step of a PC2(a) reading" }),
    ("Components.i", Id { what: "the first component of a PC2(a) reading" }),
    ("Components.j", Id { what: "the second component of a PC2(a) reading" }),
    ("ChangeCoupling.minor", Op { operators: &[A], reads: &[Fr, Con], form: "μ = x_e·y_f − y_e·x_f on two changes A_p, exact, rounded once" }),
    ("ChangeCoupling.half_width", Rule { of: &[A], reads: &[Fr, Con], rule: "the minor's bound from its four entries' bounds (interval product bounds), rounded up" }),
    ("ChangeCoupling.reading", Rule { of: &[A], reads: &[Fr, Con], rule: "PC2 at δ: freedom where the minor exceeds its bound (independent change), structure where it does not (change locked in proportion)" }),
    ("ChangeCoupling.forced", Rule { of: &[A], reads: &[Con], rule: "the reading is forced to structure by its bound (I-SC-D17 (iii)): reported apart, never read as coupled change (57th)" }),
    ("Couplings.forced_pairs", Id { what: "the pairs the declaration fixes (PC-D4), listed apart from the readings" }),
    ("PersistenceChange.across_steps", Op { operators: &[A], reads: &[Fr, Con], form: concat!("PC2(b): the 2×2 minor of two relations' changes over consecutive steps s, s + 1 in one component.", " Scope: every pair of relations that qualifies (resolved, or moved), whether or not B joins them; the work grows with the square of the relations, and is not counted in `work` (92nd).") }),
    ("Steps.component", Id { what: "the component of a PC2(b) reading" }),
    ("Steps.step", Id { what: "the first of the two steps of a PC2(b) reading" }),
    ("PersistenceChange.arrangement", Op { operators: &[A], reads: &[Fr], form: concat!("PC3: the sign at δ of the minor m_ij(e, f) on A, compared at registrations s and s + 1.", " Scope: every pair of relations that qualifies (resolved, or moved), whether or not B joins them; the work grows with the square of the relations, and is not counted in `work` (92nd).") }),
    ("ArrangementStep.step", Id { what: "the step of a PC3 reading" }),
    ("ArrangementStep.e", Id { what: "the first relation of a PC3 pair" }),
    ("ArrangementStep.f", Id { what: "the second relation of a PC3 pair" }),
    ("ArrangementStep.i", Id { what: "the first component of a PC3 minor" }),
    ("ArrangementStep.j", Id { what: "the second component of a PC3 minor" }),
    ("ArrangementStep.before", Rule { of: &[A], reads: &[Fr], rule: "the sign at δ of m_ij(e, f) = A_i[e]·A_j[f] − A_j[e]·A_i[f] at registration s, decided exactly" }),
    ("ArrangementStep.after", Rule { of: &[A], reads: &[Fr], rule: "the same sign at registration s + 1" }),
    ("ArrangementStep.reading", Rule { of: &[A], reads: &[Fr], rule: "PC3: structure when the sign held over the step, freedom when it changed" }),
    // ── The ground: connection (V1, PC2) ──────────────────────────────────
    ("GroundReading.connection", Op { operators: &[A, B, Xi], reads: &[Con], form: "connection by grade: identity, proportional, sustained, coupled change, read beside each other and never merged" }),
    ("Connection.identity", Op { operators: &[A], reads: &[Con], form: concat!("the identity grade: pairs with A[e] − A[f] not resolved in any component at every registration (a difference of A between two relations, linear).", " Scope: every pair of relations that qualifies (resolved, or moved), whether or not B joins them; the work grows with the square of the relations, and is not counted in `work` (92nd).") }),
    ("Connection.proportional", Op { operators: &[A], reads: &[Con], form: concat!("the proportional grade: pairs whose every minor m_ij(e, f) on A is held unresolved at every registration (the exterior product of two relations, multilinear).", " Scope: every pair of relations that qualifies (resolved, or moved), whether or not B joins them; the work grows with the square of the relations, and is not counted in `work` (92nd).") }),
    ("Connection.sustained", Op { operators: &[A, B, Xi], reads: &[Con], form: "the sustained grade: the sign at δ of Ξ(B(A))[e], held at every registration; local, over the relations B joins to e's neighbours (92nd)" }),
    ("Sustained.held_edges", Rule { of: &[A, B, Xi], reads: &[Con], rule: "relations whose sign of Ξ(B(A)) held resolved in at least one component at every registration" }),
    ("Sustained.reorganizing_edges", Rule { of: &[A, B, Xi], reads: &[Con], rule: "relations whose sign of Ξ(B(A)) did not hold one state" }),
    ("Sustained.not_formable", Id { what: "relations with no neighbour in the field (IV-3): Ξ is not formable there" }),
    ("Sustained.boundary_conditioned", Id { what: "relations with exactly one side registered (A-3): Ξ there is never read as a circulation of zero" }),
    ("Sustained.share_held", Rule { of: &[A, B, Xi], reads: &[Con], rule: "declared projection (C, K6): held / formable relations with both sides registered (A-3)" }),
    ("Sustained.share_held_boundary", Rule { of: &[A, B, Xi], reads: &[Con], rule: "declared projection (C): held / boundary-conditioned relations, reported apart" }),
    ("Sustained.share_held_all", Rule { of: &[A, B, Xi], reads: &[Con], rule: "declared projection (C): held / every formable relation, kept for comparison" }),
    ("Connection.coupled_change", Rule { of: &[A], reads: &[Con], rule: "the coupled-change grade (PC2): pairs whose every PC2 reading is structure, with at least one unforced (57th); every pair of moved relations (see PC2)" }),
    ("Connection.coupled_change_forced", Rule { of: &[A], reads: &[Con], rule: "pairs whose every PC2 reading was forced by the bound: reported apart, never read as coupled change (57th)" }),
    ("Connection.reorganizing_pairs", Rule { of: &[A], reads: &[Con], rule: "pairs whose minors reorganized over the sequence (from the minor histories); every pair of resolved relations (see the proportional grade)" }),
    ("Connection.evaluated_pairs", Rule { of: &[A], reads: &[Con], rule: "declared projection (C): the number of pairs of relations resolved at every registration, n(n − 1)/2" }),
    ("Connection.unresolved_edges", Rule { of: &[A], reads: &[Con], rule: "relations not resolved at δ at one or more registrations: they enter no pair, and are named" }),
    // ── V2, first form: a declared comparison only ────────────────────────
    ("GroundReading.v2_held_arrangement", Op { operators: &[A], reads: &[Con], form: concat!("the first V2 form, superseded for freedom (7th): pairs with a minor m_ij(e, f) on A held resolved with one sign throughout; a persistent arrangement, structure, carried as a declared comparison (C) only.", " Scope: every pair of relations that qualifies (resolved, or moved), whether or not B joins them; the work grows with the square of the relations, and is not counted in `work` (92nd).") }),
    ("Freedom.free_pairs", Rule { of: &[A], reads: &[Con], rule: "V2 first form: the pairs with a minor held resolved; every pair of resolved relations" }),
    ("Freedom.evaluated_pairs", Rule { of: &[A], reads: &[Con], rule: "declared projection (C): the number of pairs evaluated by the V2 first form" }),
    ("Freedom.unresolved_edges", Rule { of: &[A], reads: &[Con], rule: "relations not resolved at every registration, entering no V2 pair" }),
    ("Freedom.f_delta", Rule { of: &[A], reads: &[Con], rule: "declared projection (C): free / evaluated pairs of the V2 first form" }),
];

/// Legacy mathematics in use at the code layer (Part 7, OC-LM-1; 92nd).
static LEGACY_IN_USE: &[Derivation] = &[
    Derivation::Legacy {
        method: "exact summation (floating-point expansions)",
        provenance: "Shewchuk's expansions (1997); the method of Python's math.fsum; two-sum and fused multiply-add. `src/exact.rs`",
        in_relation_to_operators: "adds no form, input or construct: it computes the sums A, B and Ξ (and the minors and bounds) are defined by, without loss, so a sum does not depend on the order of its terms (Df2a); each value is rounded once",
        necessity: Some("the 41st rule, decompose, do not simplify: without it a closure depends on traversal order. Confirmed necessary by Origin (44th; OC-LM-1, item 1)"),
    },
    Derivation::Legacy {
        method: "interval bounds (outward rounding, absolute values in product bounds)",
        provenance: "interval arithmetic (Moore, 1966), as used throughout the readings at δ: |a|·δb + |b|·δa + δa·δb for a product, rounded up. `src/exact.rs` (product_bound, gain_widened)",
        in_relation_to_operators: "not an operator: it carries each registered bound through A, B, Ξ and the minors, so every reading is decided against the bound of its own operator output; the absolute values are on bounds and on ε = |A(c)| only",
        necessity: Some("the 41st rule: every part is reported with its bound, and bounds are never rounded down. Stated by the Generator from that rule (92nd); for Origin to confirm, as exact summation was at the 44th"),
    },
];
