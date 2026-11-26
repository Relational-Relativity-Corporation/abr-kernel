//! ORIGIN'S FILE. What Origin has admitted, read by `tests/guard_operators.rs`.
//!
//! The Generator (Claude) may not change this file: a PreToolUse hook in
//! `.claude/settings.json` refuses any edit to `tests/guard/` and `.claude/`.
//! Origin changes it directly, with the Part 9 entry that admits each line.
//!
//! Written by the Generator on 2026-11-26 from the register as it stood at
//! `9d6e4ae`, for Origin to confirm or strike. Every line names the Part 9
//! entry it rests on; a line with no entry of Origin's is not admitted.

/// Legacy mathematics Origin has admitted, by the part it reports (or, for a
/// method in use at the code layer, the first words of its method), with the
/// Part 9 entry that admits it and whether it is a reading or only a check.
pub const ADMITTED_LEGACY: &[(&str, &str, Use)] = &[
    // The orthogonal split (100th).
    ("TwoParts.spatial", "100th", Use::Reading),
    ("TwoParts.persistence", "100th", Use::Reading),
    ("Split.potential", "100th", Use::Reading),
    ("Split.balanced", "100th", Use::Reading),
    ("Split.exact_potential", "100th", Use::Reading),
    ("Split.exact_balanced", "100th", Use::Reading),
    // The 2×2 minors: legacy (94th), necessity open reading by reading.
    ("PersistenceChange.across_components", "94th", Use::Reading),
    ("PersistenceChange.across_steps", "94th", Use::Reading),
    ("Couplings.readings", "94th", Use::Reading),
    ("ChangeCoupling.minor", "94th", Use::Reading),
    ("PersistenceChange.arrangement", "94th", Use::Reading),
    ("ArrangementStep.before", "94th", Use::Reading),
    ("ArrangementStep.after", "94th", Use::Reading),
    ("Connection.proportional", "94th", Use::Reading),
    ("Connection.reorganizing_pairs", "94th", Use::Reading),
    ("GroundReading.v2_held_arrangement", "94th", Use::Reading),
    ("Freedom.free_pairs", "94th", Use::Reading),
    // Methods in use at the code layer.
    ("exact summation", "44th", Use::Reading),
    ("product bounds", "93rd", Use::Reading),
    ("exact rationals", "100th", Use::Reading),
];

/// Whether admitted legacy mathematics may stand as a reading, or only as a
/// check of readings the operators form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Use { Reading, Check }

/// Legacy parts in the code that Origin has NOT admitted: named gaps, each
/// with what it waits on. The guard reports them every run and no
/// application may read them as a result. Origin strikes a line to make the
/// guard fail until the part is fenced or formed by the operators.
pub const NOT_ADMITTED: &[(&str, &str)] = &[
    ("Level.occurrences", "the 103rd: no operator forms it (the suffix match of the 102nd)"),
    ("Level.continuations", "the 103rd: no operator forms it (the suffix match of the 102nd)"),
];

/// The public structures as they stood at `9d6e4ae`, before this guard. A
/// public structure not in this list must have every field registered in
/// `src/derivation.rs`, so a new reading cannot enter undeclared.
pub const STRUCTS_BEFORE_THE_GUARD: &[&str] = &[
    "Terminus", "ReverseTrace", "ReconstructedInput", "ReconstructedEntry", "AdmissionEvidence",
    "HeldConstraint", "RelationFreedom", "WindowReading", "Kept", "Candidate", "CandidateReading", "Run",
    "AccumulatedEnergy", "Spoke", "SpokePair", "RadialGraph", "ChannelRecord", "DeclaredProjection",
    "EdgeProvenanceRecord", "NodeRecord", "EdgeRecord", "ComponentPair", "ExperimentalField", "Warning",
    "DiagnosticError", "NotExecuted", "ParameterUse", "SpatialExecution", "PersistenceExecution",
    "EnergyTransfer", "FreedomDeclaration", "ConnectionDeclaration", "DomainDeclaration", "Read",
    "TransferReading", "DeclaredEnergy", "EnergyEntry", "FreedomEntry", "ConnectionEntry", "GroundRecord",
    "DeclaredField", "Boundary", "PlaneValue", "Angle", "DeclaredDirection", "HeldAndExchange", "Finding",
    "MinorPattern", "MinorHistories", "EnergyCarrier", "RelationCarrier", "DeclaredRelations",
    "PrimaryEdgeField", "EdgeField", "PersistenceState", "PersistenceOutput", "EdgeGains", "Shares",
    "EvolutionCount", "StepProfile", "RelationTransition", "ExternalRecord", "ObservableDeclaration",
    "ObservationalBasis", "MMappingDeclaration", "DeclaredDomain", "LocusDeclaration",
    "ObservableRelationDeclaration", "RelationalDeclaration", "DeclarationRegistry", "InputRecord",
    "RelationRecord", "ParameterRecord", "OutputEntry", "CalculationRecord", "ProvenancedOutput",
    "FeedbackDeclaration", "T8Admission", "RelationIdentity", "RouteEnumeration", "CycleBasis",
];

/// Familiar methods (CLAUDE.md, 84th, 90th): a name in the code (outside
/// comments and quoted text) containing any of these fails the guard.
pub const FAMILIAR_METHODS: &[&str] = &[
    "suffix", "ngram", "n_gram", "bigram", "trigram", "cooccur", "co_occur", "cosine", "markov",
    "softmax", "tfidf", "tf_idf", "embedding", "context_model", "bag_of", "perplex", "logit",
    "sigmoid", "attention", "similarity", "debruijn", "de_bruijn",
];

/// Places Origin has allowed one of those names, as (file, name). None yet.
pub const FAMILIAR_ALLOWED: &[(&str, &str)] = &[];
