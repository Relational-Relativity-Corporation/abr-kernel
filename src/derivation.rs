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
    /// K9 (Origin, 96th): the readback of relations at their loci, in two
    /// halves, Tᵀ (leaving) and Hᵀ (entering).
    Readback,
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

/// Rules in use at the code layer, derived from the operators and the
/// declaration, which no part reports by name (93rd).
pub fn rules_in_use() -> &'static [Derivation] {
    RULES_IN_USE
}

/// Legacy parts whose necessity Origin has not yet stated: named gaps
/// (94th), never filled. Every other requirement holds for them.
pub fn necessity_open() -> &'static [&'static str] {
    NECESSITY_OPEN
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

use Derivation::{Identity as Id, Legacy, Operator as Op, Registered as Reg, Rule};
use Operator::{A, B, Readback, Xi};
use Reads::{Connection as Con, Energy as En, Freedom as Fr, Relation as Rel};

// Every reading that compares pairs of relations states its scope (92nd:
// the shape of attention when taken over every pair). Recorded, not decided:
// whether these are fenced from the point of analysis is Origin's.
// The minors (Origin, 2026-11-25, 94th: reclassified as legacy). *Was
// (43rd table; 92nd register):* listed as operator readings, "multilinear
// forms of A". A minor is not A, B or Ξ.
const MINOR_PROVENANCE: &str = "exterior algebra: the 2×2 determinant (Leibniz; Cramer, 1750), the exterior product u∧v of two vectors (Grassmann, 1844), the name minor (Sylvester, 1850); summed over component planes, Σ m² = |u|²|v|² − (u·v)² (Lagrange's identity), so every minor zero is |cos| = 1 read exactly";
const MINOR_IN_RELATION: &str = "an antisymmetric bilinear form of A's output, m_ij(e, f) = A_i[e]·A_j[f] − A_j[e]·A_i[f]: one A-value from each of two relations in two components, linear in each relation and a product of the two; not A, B or Ξ, and its bound needs the product bound (93rd). Scope: every pair of relations that qualifies (resolved, or moved), whether or not B joins them; the work grows with the square of the relations, and is not counted in `work` (92nd)";

// The orthogonal split (Origin, 2026-11-26, 100th: "Proceed with the
// orthogonal split we'll see how that looks keep in mind if our construct
// reveals observable behaviors that will be our validation").
const SPLIT_PROVENANCE: &str = "Kirchhoff (1847), the currents of a network as loops and potentials; the image of a linear map and the null space of its transpose; on networks, the discrete Hodge decomposition (Eckmann, 1944); the orthogonal projection of least squares (Legendre, 1805; Gauss, 1809)";
const SPLIT_IN_RELATION: &str = "built from A and the readback only: P = A G Aᵀ, where Aᵀ is the readback's balance (what leaves less what enters, K9) and G inverts L = AᵀA over the loci, one locus of each piece held at 0; the potential part is in the image of A, the balanced part has a readback balance of zero at every locus. Not local: each relation's part depends on every relation of its piece, and the bounds read all m × m entries of P, so the work grows with the square of the relations (not counted in `work`, 92nd). Its solve over the loci needs division, computed exactly in rationals";
const SPLIT_NECESSITY: &str = "declared by Origin (100th): to test the 99th's candidate, that energy, connection and freedom are the parts of one structure; the orthogonal split is the one that depends on no choice of basis or tree (41st). Validation is the observable behaviour the construct reveals (Origin, 100th), not these synthetic tests";

// The order's reading (Origin, 2026-11-26, 102nd).
const ORDER_PROVENANCE: &str = "the continuations of a route through a registered sequence: what a suffix index or a variable-order context model computes (Weiner, 1973; Rissanen, 1983); a familiar method by the 84th and 90th, here the direct reading of the field Origin declared";
const ORDER_IN_RELATION: &str = "a reading of the order of relations: each step's relation continues from the locus the previous entered (B joins a relation to its successors; the order registers which successor followed, step by step); route length 0 is the readback at the current locus counted over the order, the summed field's star; each longer route keeps only the earlier points that also match one step further back. No operator forms it, and no weight or decay by distance enters (74th)";
const DEGREE_PROVENANCE: &str = "the comparison of two positions of a registered sequence back to their first difference: the comparison a suffix index makes (Weiner, 1973); here kept between two distinct points, nothing merged or counted (108th)";
const DEGREE_IN_RELATION: &str = "a reading of the registered order, not formed by an operator: it compares which relation stood at matching positions behind two distinct points; the relations compared are the order's registrations, the leaving relation is the readback's leaving half (K9) decomposed by point; its operator form is sought (108th (5)). *Since the 112th:* also formed from the field itself, `order::indications`: the readback (K9) of every step in order, compared at matching points behind two distinct points, equal exactly to this reading on actual text (`tests/v10_indications.rs`); whether that retires the legacy comparison is Origin's";
const DEGREE_NECESSITY: &str = "admitted by Origin (108th): the degrees of freedom of the current point read how far each earlier revolution's arrival runs alongside the current one and where it parts; no operator of the kernel yet forms the comparison";
const ORDER_NECESSITY: &str = "declared by Origin (102nd): the order is the relational field, and the possibilities at the current point are the continuations the entire input registered after the route arriving there; a sum forgets order, so no reading of the summed field can give them";

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
    ("EdgeSeries.shared_gain", Rule { of: &[A], reads: &[Fr], rule: "the shared gain bound [c][e]: enters reported change magnitudes and K7 turn sums (Z1a; I-SC-D15 (v))" }),
    ("EdgeSeries.mixed_gain", Rule { of: &[A], reads: &[Con, Fr], rule: "the independent gain bound on relations registered directly: enters every minor that mixes components (Z1a)" }),
    ("EdgeSeries.turn_sum_gain", Rule { of: &[A], reads: &[Con], rule: "the gain bound differing between relations of one component: enters identity, Ξ and K7 turn sums (I-SC-D15 (v))" }),
    ("EdgeSeries.origin", Id { what: "where the relation values came from: formed from loci, or registered directly (ER-1)" }),
    ("EdgeSeries.forced_pairs", Id { what: "pairs of relations whose coupling the declaration fixes (PC-D4): identities, not readings" }),
    // ── A, B and Ξ ────────────────────────────────────────────────────────
    ("RelationalField.spatial", Op { operators: &[A, B, Xi], reads: &[Rel], form: "A, B and Ξ at every registration" }),
    ("RelationalField.persistence", Op { operators: &[A, B, Xi], reads: &[Rel, Fr], form: "A_p = A_{t+1} − A_t, B_p = B(A_p), Ξ_p = Ξ(B_p) over every consecutive step (linear, so B_p and Ξ_p are the differences exactly)" }),
    ("Forms.a", Op { operators: &[A], reads: &[Rel], form: "A at a registration, or A_p over a step, one exact sum rounded once" }),
    ("Forms.b", Op { operators: &[A, B], reads: &[Rel], form: "B(A)[e] = A[e] + Σ_succ A[f], over the relations B joins to e only (92nd)" }),
    ("Forms.passage", Op { operators: &[A, B, Xi], reads: &[Rel, Con], form: "Ξ(B(A))[e] = Σ_succ B − Σ_pred B, over the relations B joins to e's neighbours only (92nd)" }),
    // ── The readback and the two parts (96th; 99th, Proposed) ─────────────
    ("Forms.leaving", Op { operators: &[A, Readback], reads: &[Rel, Con], form: "Tᵀg[v]: the sum of the relations leaving locus v (K9), one exact sum rounded once; a reading of relations at a locus, never a locus state" }),
    ("Forms.entering", Op { operators: &[A, Readback], reads: &[Rel, Con], form: "Hᵀg[v]: the sum of the relations entering locus v (K9); the balance Tᵀg − Hᵀg is derived from the two halves, never standing in for them" }),
    ("Forms.leaving_bound", Rule { of: &[A, Readback], reads: &[Rel, Con], rule: "the bound of what leaves: the sum of the bounds of the relations summed, the operator with its signs dropped (93rd), rounded up; the state bound widened by any differing gain at a registration, the change bound over a step" }),
    ("Forms.entering_bound", Rule { of: &[A, Readback], reads: &[Rel, Con], rule: "the bound of what enters, formed as the bound of what leaves" }),
    ("RelationalField.meeting", Reg { what: "the relations meeting at each locus, as declared: the relations each half of the readback sums (96th, condition (1)), every direction kept (condition (3))" }),
    ("Meeting.leaving", Reg { what: "the relations declared to leave each locus, in declared order" }),
    ("Meeting.entering", Reg { what: "the relations declared to enter each locus, in declared order" }),
    ("RelationalField.ways", Rule { of: &[A, Readback], reads: &[Rel, Con, En, Fr], rule: "the independent ways of the two parts of every field of relations (99th, Proposed): the potential part, the image of A, and the balanced part, the null space of the readback's balance; fixed by the declared relations alone, independent of the data, the domain and δ" }),
    ("Ways.relations", Id { what: "m, the number of declared relations" }),
    ("Ways.loci", Id { what: "n, the number of declared loci" }),
    ("Ways.pieces", Id { what: "c, the connected pieces the declared relations join the loci into; a locus no relation meets is a piece of its own" }),
    ("Ways.potential", Rule { of: &[A], reads: &[Rel, En], rule: "n − c: the independent ways of the potential part, fields explained by values at the loci (the relations of any spanning forest)" }),
    ("Ways.balanced", Rule { of: &[A, Readback], reads: &[Rel, Con], rule: "m − n + c: the independent ways of the balanced part, where what enters every locus equals what leaves (the independent turns, K7)" }),
    // ── The orthogonal split (Origin, 100th; the 99th, Proposed) ──────────
    ("TwoParts.inner_product", Reg { what: "the declared inner product the split is orthogonal under (99th, 100th): equal weight on every relation by default; any other weighting a domain's declaration" }),
    ("Equal.basis", Reg { what: "the basis of the equal-weight inner product, declared with it" }),
    ("TwoParts.spatial", Legacy { method: "the orthogonal split at every registration: the potential part A G Aᵀ g and the balanced part g − A G Aᵀ g", provenance: SPLIT_PROVENANCE, in_relation_to_operators: SPLIT_IN_RELATION, necessity: Some(SPLIT_NECESSITY) }),
    ("TwoParts.persistence", Legacy { method: "the orthogonal split over every step, of the change A_p; equal exactly to the change of the split (linear)", provenance: SPLIT_PROVENANCE, in_relation_to_operators: SPLIT_IN_RELATION, necessity: Some(SPLIT_NECESSITY) }),
    ("Split.potential", Legacy { method: "the potential part, the image of A: explained by values at the loci, closing every turn; rounded once", provenance: SPLIT_PROVENANCE, in_relation_to_operators: SPLIT_IN_RELATION, necessity: Some(SPLIT_NECESSITY) }),
    ("Split.balanced", Legacy { method: "the balanced part: at every locus what enters equals what leaves (the readback's balance zero); rounded once", provenance: SPLIT_PROVENANCE, in_relation_to_operators: SPLIT_IN_RELATION, necessity: Some(SPLIT_NECESSITY) }),
    ("Split.exact_potential", Legacy { method: "the potential part exactly, in rationals: every identity of the split holds in it", provenance: SPLIT_PROVENANCE, in_relation_to_operators: SPLIT_IN_RELATION, necessity: Some(SPLIT_NECESSITY) }),
    ("Split.exact_balanced", Legacy { method: "the balanced part exactly, in rationals", provenance: SPLIT_PROVENANCE, in_relation_to_operators: SPLIT_IN_RELATION, necessity: Some(SPLIT_NECESSITY) }),
    ("Split.potential_bound", Rule { of: &[A, Readback], reads: &[Rel, En], rule: "the bound of the potential part: Σ_f |P[e][f]|·δ_f, the projection with its signs dropped applied to the relations' bounds (93rd, 98th), exactly, rounded up" }),
    ("Split.balanced_bound", Rule { of: &[A, Readback], reads: &[Rel, Con], rule: "the bound of the balanced part: Σ_f |I[e][f] − P[e][f]|·δ_f, formed as the potential part's" }),
    ("Ratio.negative", Id { what: "the sign of an exact rational" }),
    ("Ratio.num", Id { what: "the numerator of an exact rational, in lowest terms (an integer of any size)" }),
    ("Ratio.den", Id { what: "the denominator of an exact rational, positive, in lowest terms" }),
    // ── The order (Origin, 102nd: the order is the relational field) ──────
    ("Order.edges", Reg { what: "each declared relation as the locus it leaves and the locus it enters; a self-relation is the 76th's identity spoke on the occurrence chain" }),
    ("Order.steps", Reg { what: "the order: the relation registered at each step, oldest first, each continuing from the locus the previous step entered; read left to right, every character a step, nothing cut (102nd)" }),
    ("Level.route", Id { what: "the route arriving at the current point: the last relations of the order, oldest first; empty at length 0" }),
    ("Level.occurrences", Legacy { method: "every earlier point of the order where the same route arrived, as the step that followed it there", provenance: ORDER_PROVENANCE, in_relation_to_operators: ORDER_IN_RELATION, necessity: Some(ORDER_NECESSITY) }),
    // ── The degrees of freedom of the current point (108th) ──────────────
    ("Degree.point", Id { what: "the earlier point in evolution where the order stood at the current locus: a distinct revolution (106th, 107th), never another occurrence of the same route" }),
    // *Was (to 2026-11-26):* ("Degree.leaving", Reg { what: "the relation registered at the step that left that point: the readback's leaving half at the current locus (K9), kept decomposed by point in evolution, never summed" }). Corrected: the code takes the registered step; no readback is computed, and none can be until a mapping of text onto registered values is declared.
    ("Degree.leaving", Reg { what: "the relation registered at the step that left that point (`steps[point]`), as registered; no operator forms it" }),
    ("Degree.turn", Id { what: "the degree's turn to now: the span of the order between the earlier point and the current point, the relations registered between them" }),
    ("Degree.alongside", Legacy { method: "the span the earlier arrival runs alongside the current arrival: the relations behind each point compared position by position, back to the first that differs; a comparison of two distinct points, never an identification of them", provenance: DEGREE_PROVENANCE, in_relation_to_operators: DEGREE_IN_RELATION, necessity: Some(DEGREE_NECESSITY) }),
    ("Degree.parts", Legacy { method: "where the two arrivals part: the relation there and the relation now at the first position behind the points that differs, or the record boundary", provenance: DEGREE_PROVENANCE, in_relation_to_operators: DEGREE_IN_RELATION, necessity: Some(DEGREE_NECESSITY) }),
    ("Level.continuations", Legacy { method: "each relation that followed the route in the order, with how many times: the possibilities at the current point after that route", provenance: ORDER_PROVENANCE, in_relation_to_operators: ORDER_IN_RELATION, necessity: Some(ORDER_NECESSITY) }),
    // ── The ground: energy ────────────────────────────────────────────────
    ("RelationalField.ground", Reg { what: "energy, freedom and connection, read together over the whole sequence (Df6, W11); each part below has its own derivation" }),
    // *Was (to the 103rd):* ("GroundReading.energy", Op { operators: &[A], reads: &[En], form: "V3: energy read from a declared carrier, ε = |A(carrier)|, or named as not evaluated when no carrier is declared" }),
    ("GroundReading.energy", Op { operators: &[A], reads: &[En], form: "energy read from a declared carrier as the signed exchange A(c) (103rd), or named as not evaluated when no carrier is declared; no magnitude is calculated (104th)" }),
    ("Energy.carrier", Id { what: "the declared carrier's name (R9a, V3a)" }),
    ("Energy.unit", Id { what: "the declared carrier's unit" }),
    ("Energy.signed", Op { operators: &[A], reads: &[En], form: "the signed exchange A(c)[e] at every registration and relation (103rd): energy moving across each relation against its declared direction, positive toward the entering locus; c[s] − c[t] for a carrier at the loci, the registered value for a carrier across the relations" }),
    ("SignedExchange.edge", Id { what: "the relation the movement is read on" }),
    ("SignedExchange.value", Op { operators: &[A], reads: &[En], form: "A(c)[e], linear, exact, rounded once: c[s] − c[t], or as registered across the relation; its sign is the direction of movement (103rd)" }),
    ("SignedExchange.bound", Rule { of: &[A], reads: &[En], rule: "δ_c[s] + δ_c[t], or as registered: A with its signs dropped applied to the carrier's bounds (93rd), rounded up" }),
    ("SignedExchange.side", Rule { of: &[A], reads: &[En], rule: "the side at δ (98th: δ decides only at reading), a sign against the bound (104th): toward the entering locus where A(c)[e] − bound > 0, toward the leaving locus where A(c)[e] + bound < 0, not resolved otherwise; decided exactly from the exact value and bound; the tunneling floor is the side resolved" }),
    // *Was (to the 103rd):* ("Energy.exchanges", Op { operators: &[A], reads: &[En], form: "ε at every registration and relation" }),
    // *Retired (104th):* ("Energy.exchanges", Op { operators: &[A], reads: &[En], form: "the V3 magnitude |A(c)| at every registration and relation; fenced (103rd): not a reading of energy, kept for comparison and for its floor" }),
    // *Retired (104th):* ("EnergyExchange.edge", Id { what: "the relation an exchange is read on" }),
    // *Was (to the 103rd):* ("EnergyExchange.epsilon", Op { operators: &[A], reads: &[En], form: "ε[e] = |A(c)[e]|: the magnitude of A on the carrier (absolute value of a linear operator's output)" }),
    // *Retired (104th):* ("EnergyExchange.epsilon", Op { operators: &[A], reads: &[En], form: "ε[e] = |A(c)[e]|: the magnitude of A on the carrier (absolute value of a linear operator's output); fenced (103rd): magnitude is not a variable, energy is relative" }),
    // *Retired (104th):* ("EnergyExchange.delta_epsilon", Rule { of: &[A], reads: &[En], rule: "δ_ε[e]: the carrier's bound carried through A (δ_c[s] + δ_c[t], or as registered), rounded up" }),
    // *Was (to the 103rd):* ("EnergyExchange.floor_resolved", Rule { of: &[A], reads: &[En], rule: "the tunneling floor at δ: ε > δ_ε, decided exactly" }),
    // *Retired (104th):* ("EnergyExchange.floor_resolved", Rule { of: &[A], reads: &[En], rule: "the tunneling floor at δ: the side of the signed exchange resolved (103rd), decided exactly; the same decision as ε > δ_ε" }),
    ("Energy.share_resolved", Rule { of: &[A], reads: &[En], rule: "declared projection (C, K6): the share of (registration, relation) movements whose side is resolved; the parts are reported beside it" }),
    // ── The ground: freedom (PC1–PC3) ─────────────────────────────────────
    ("GroundReading.freedom", Op { operators: &[A], reads: &[Fr], form: "freedom as change permitted: PC1 on each relation, PC2 and PC3 on pairs" }),
    ("PersistenceChange.relations", Op { operators: &[A], reads: &[Fr], form: "PC1 on every relation, component and step: the change A_p read against its bound" }),
    ("RelationChange.step", Id { what: "the step a reading is over (prior always the earlier registration, Df2)" }),
    ("RelationChange.component", Id { what: "the component a reading is in" }),
    ("RelationChange.edge", Id { what: "the relation a reading is on" }),
    ("RelationChange.change", Op { operators: &[A], reads: &[Fr], form: "A_p[c][e] = A_{t+1} − A_t, exact, rounded once" }),
    ("RelationChange.half_width", Rule { of: &[A], reads: &[Fr], rule: "the change bound, from both registrations' bounds and the gains (Z1a), rounded up" }),
    ("RelationChange.reading", Rule { of: &[A], reads: &[Fr], rule: "PC1 at δ: freedom where |A_p| exceeds its bound, structure otherwise, decided exactly" }),
    // *Was:* ("PersistenceChange.across_components", Op { operators: &[A], reads: &[Fr, Con], form: concat!("PC2(a): the 2×2 minor of two relations' changes A_p across components i < j at each step (a multilinear form, the exterior product of two changes).", " Scope: every pair of relations that qualifies (resolved, or moved), whether or not B joins them; the work grows with the square of the relations, and is not counted in `work` (92nd).") }),
    ("PersistenceChange.across_components", Legacy { method: "PC2(a): the 2×2 minor of two relations' changes A_p across components i < j at each step; it selects by δ what enters before its pairs are formed: a departure from the 98th (only relations whose change is resolved in either component enter)", provenance: MINOR_PROVENANCE, in_relation_to_operators: MINOR_IN_RELATION, necessity: None }),
    // *Was:* ("Couplings.readings", Op { operators: &[A], reads: &[Fr, Con], form: concat!("the PC2 readings, one per pair of moved relations and pair of components or steps.", " Scope: every pair of relations that qualifies (resolved, or moved), whether or not B joins them; the work grows with the square of the relations, and is not counted in `work` (92nd).") }),
    ("Couplings.readings", Legacy { method: "the PC2 readings, one minor per pair of moved relations and pair of components or steps; it selects by δ what enters before its pairs are formed: a departure from the 98th (only relations that moved at δ enter)", provenance: MINOR_PROVENANCE, in_relation_to_operators: MINOR_IN_RELATION, necessity: None }),
    ("ChangeCoupling.e", Id { what: "the first relation of a PC2 pair" }),
    ("ChangeCoupling.f", Id { what: "the second relation of a PC2 pair" }),
    ("ChangeCoupling.over", Id { what: "what a PC2 minor is taken over: two components at a step, or one component at two steps" }),
    ("Components.step", Id { what: "the step of a PC2(a) reading" }),
    ("Components.i", Id { what: "the first component of a PC2(a) reading" }),
    ("Components.j", Id { what: "the second component of a PC2(a) reading" }),
    // *Was:* ("ChangeCoupling.minor", Op { operators: &[A], reads: &[Fr, Con], form: "μ = x_e·y_f − y_e·x_f on two changes A_p, exact, rounded once" }),
    ("ChangeCoupling.minor", Legacy { method: "μ = x_e·y_f − y_e·x_f on two changes A_p, exact, rounded once", provenance: MINOR_PROVENANCE, in_relation_to_operators: MINOR_IN_RELATION, necessity: None }),
    ("ChangeCoupling.half_width", Rule { of: &[A], reads: &[Fr, Con], rule: "the minor's bound from its four entries' bounds: the product bound, legacy mathematics with its necessity (93rd; `legacy_in_use`), rounded up" }),
    ("ChangeCoupling.reading", Rule { of: &[A], reads: &[Fr, Con], rule: "PC2 at δ: freedom where the minor exceeds its bound (independent change), structure where it does not (change locked in proportion)" }),
    ("ChangeCoupling.forced", Rule { of: &[A], reads: &[Con], rule: "the reading is forced to structure by its bound (I-SC-D17 (iii)): reported apart, never read as coupled change (57th)" }),
    ("Couplings.forced_pairs", Id { what: "the pairs the declaration fixes (PC-D4), listed apart from the readings" }),
    // *Was:* ("PersistenceChange.across_steps", Op { operators: &[A], reads: &[Fr, Con], form: concat!("PC2(b): the 2×2 minor of two relations' changes over consecutive steps s, s + 1 in one component.", " Scope: every pair of relations that qualifies (resolved, or moved), whether or not B joins them; the work grows with the square of the relations, and is not counted in `work` (92nd).") }),
    ("PersistenceChange.across_steps", Legacy { method: "PC2(b): the 2×2 minor of two relations' changes over consecutive steps s, s + 1 in one component; it selects by δ what enters before its pairs are formed: a departure from the 98th (only relations whose change is resolved at either step enter)", provenance: MINOR_PROVENANCE, in_relation_to_operators: MINOR_IN_RELATION, necessity: None }),
    ("Steps.component", Id { what: "the component of a PC2(b) reading" }),
    ("Steps.step", Id { what: "the first of the two steps of a PC2(b) reading" }),
    // *Was:* ("PersistenceChange.arrangement", Op { operators: &[A], reads: &[Fr], form: concat!("PC3: the sign at δ of the minor m_ij(e, f) on A, compared at registrations s and s + 1.", " Scope: every pair of relations that qualifies (resolved, or moved), whether or not B joins them; the work grows with the square of the relations, and is not counted in `work` (92nd).") }),
    ("PersistenceChange.arrangement", Legacy { method: "PC3: the sign at δ of the minor m_ij(e, f) on A, compared at registrations s and s + 1; it selects by δ what enters before its pairs are formed: a departure from the 98th (only relations resolved at both registrations enter)", provenance: MINOR_PROVENANCE, in_relation_to_operators: MINOR_IN_RELATION, necessity: None }),
    ("ArrangementStep.step", Id { what: "the step of a PC3 reading" }),
    ("ArrangementStep.e", Id { what: "the first relation of a PC3 pair" }),
    ("ArrangementStep.f", Id { what: "the second relation of a PC3 pair" }),
    ("ArrangementStep.i", Id { what: "the first component of a PC3 minor" }),
    ("ArrangementStep.j", Id { what: "the second component of a PC3 minor" }),
    // *Was:* ("ArrangementStep.before", Rule { of: &[A], reads: &[Fr], rule: "the sign at δ of m_ij(e, f) = A_i[e]·A_j[f] − A_j[e]·A_i[f] at registration s, decided exactly" }),
    ("ArrangementStep.before", Legacy { method: "the sign at δ of m_ij(e, f) at registration s, decided exactly", provenance: MINOR_PROVENANCE, in_relation_to_operators: MINOR_IN_RELATION, necessity: None }),
    // *Was:* ("ArrangementStep.after", Rule { of: &[A], reads: &[Fr], rule: "the same sign at registration s + 1" }),
    ("ArrangementStep.after", Legacy { method: "the sign at δ of m_ij(e, f) at registration s + 1, decided exactly", provenance: MINOR_PROVENANCE, in_relation_to_operators: MINOR_IN_RELATION, necessity: None }),
    ("ArrangementStep.reading", Rule { of: &[A], reads: &[Fr], rule: "PC3: structure when the sign held over the step, freedom when it changed" }),
    // ── The ground: connection (V1, PC2) ──────────────────────────────────
    ("GroundReading.connection", Op { operators: &[A, B, Xi], reads: &[Con], form: "connection by grade: identity, proportional, sustained, coupled change, read beside each other and never merged" }),
    ("Connection.identity", Op { operators: &[A], reads: &[Con], form: concat!("the identity grade: pairs with A[e] − A[f] not resolved in any component at every registration (a difference of A between two relations, linear); it selects by δ what enters before its pairs are formed: a departure from the 98th (only relations resolved at every registration enter).", " Scope: every pair of relations that qualifies (resolved, or moved), whether or not B joins them; the work grows with the square of the relations, and is not counted in `work` (92nd).") }),
    // *Was:* ("Connection.proportional", Op { operators: &[A], reads: &[Con], form: concat!("the proportional grade: pairs whose every minor m_ij(e, f) on A is held unresolved at every registration (the exterior product of two relations, multilinear).", " Scope: every pair of relations that qualifies (resolved, or moved), whether or not B joins them; the work grows with the square of the relations, and is not counted in `work` (92nd).") }),
    ("Connection.proportional", Legacy { method: "the proportional grade: pairs whose every minor m_ij(e, f) on A is held unresolved at every registration; it selects by δ what enters before its pairs are formed: a departure from the 98th (only relations resolved at every registration enter)", provenance: MINOR_PROVENANCE, in_relation_to_operators: MINOR_IN_RELATION, necessity: None }),
    ("Connection.sustained", Op { operators: &[A, B, Xi], reads: &[Con], form: "the sustained grade: the sign at δ of Ξ(B(A))[e], held at every registration; local, over the relations B joins to e's neighbours (92nd)" }),
    ("Sustained.held_edges", Rule { of: &[A, B, Xi], reads: &[Con], rule: "relations whose sign of Ξ(B(A)) held resolved in at least one component at every registration" }),
    // *Was:* ("Sustained.reorganizing_edges", Rule { of: &[A, B, Xi], reads: &[Con], rule: "relations whose sign of Ξ(B(A)) did not hold one state" }),
    ("Sustained.reorganizing_edges", Rule { of: &[A, B, Xi], reads: &[Con], rule: "relations not held whose sign of Ξ(B(A)) at δ changed state over the sequence in some component; a relation whose passage is not resolved at any registration in any component is in neither this list nor the held one (reported nowhere: a finding for Origin, 2026-11-26)" }),
    ("Sustained.not_formable", Id { what: "relations with no neighbour in the field (IV-3): Ξ is not formable there" }),
    ("Sustained.boundary_conditioned", Id { what: "relations with exactly one side registered (A-3): Ξ there is never read as a passage of zero" }),
    ("Sustained.share_held", Rule { of: &[A, B, Xi], reads: &[Con], rule: "declared projection (C, K6): held / formable relations with both sides registered (A-3)" }),
    ("Sustained.share_held_boundary", Rule { of: &[A, B, Xi], reads: &[Con], rule: "declared projection (C): held / boundary-conditioned relations, reported apart" }),
    ("Sustained.share_held_all", Rule { of: &[A, B, Xi], reads: &[Con], rule: "declared projection (C): held / every formable relation, kept for comparison" }),
    ("Connection.coupled_change", Rule { of: &[A], reads: &[Con], rule: "the coupled-change grade (PC2): pairs whose every PC2 reading is structure, with at least one unforced (57th); every pair of moved relations (see PC2)" }),
    ("Connection.coupled_change_forced", Rule { of: &[A], reads: &[Con], rule: "pairs whose every PC2 reading was forced by the bound: reported apart, never read as coupled change (57th)" }),
    // *Was:* ("Connection.reorganizing_pairs", Rule { of: &[A], reads: &[Con], rule: "pairs whose minors reorganized over the sequence (from the minor histories); every pair of resolved relations (see the proportional grade)" }),
    ("Connection.reorganizing_pairs", Legacy { method: "pairs whose minors reorganized over the sequence (from the minor histories); it selects by δ what enters before its pairs are formed: a departure from the 98th (only relations resolved at every registration enter)", provenance: MINOR_PROVENANCE, in_relation_to_operators: MINOR_IN_RELATION, necessity: None }),
    ("Connection.evaluated_pairs", Rule { of: &[A], reads: &[Con], rule: "declared projection (C): the number of pairs of relations resolved at every registration, n(n − 1)/2" }),
    ("Connection.unresolved_edges", Rule { of: &[A], reads: &[Con], rule: "relations not resolved at δ at one or more registrations: they enter no pair, and are named" }),
    // ── V2, first form: a declared comparison only ────────────────────────
    // *Was:* ("GroundReading.v2_held_arrangement", Op { operators: &[A], reads: &[Con], form: concat!("the first V2 form, superseded for freedom (7th): pairs with a minor m_ij(e, f) on A held resolved with one sign throughout; a persistent arrangement, structure, carried as a declared comparison (C) only.", " Scope: every pair of relations that qualifies (resolved, or moved), whether or not B joins them; the work grows with the square of the relations, and is not counted in `work` (92nd).") }),
    ("GroundReading.v2_held_arrangement", Legacy { method: "the first V2 form, superseded for freedom (7th): pairs with a minor m_ij(e, f) on A held resolved with one sign throughout; a declared comparison (C) only; it selects by δ what enters before its pairs are formed: a departure from the 98th (only relations resolved at every registration enter)", provenance: MINOR_PROVENANCE, in_relation_to_operators: MINOR_IN_RELATION, necessity: None }),
    // *Was:* ("Freedom.free_pairs", Rule { of: &[A], reads: &[Con], rule: "V2 first form: the pairs with a minor held resolved; every pair of resolved relations" }),
    ("Freedom.free_pairs", Legacy { method: "V2 first form: the pairs with a minor held resolved; it selects by δ what enters before its pairs are formed: a departure from the 98th (only relations resolved at every registration enter)", provenance: MINOR_PROVENANCE, in_relation_to_operators: MINOR_IN_RELATION, necessity: None }),
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
        necessity: Some("the 41st rule, decompose, do not simplify: without it a turn sum depends on traversal order. Confirmed necessary by Origin (44th; OC-LM-1, item 1)"),
    },
    // *Was (92nd):* one entry, "interval bounds (outward rounding, absolute
    // values in product bounds)", its necessity stated by the Generator and
    // left for Origin. Split by Origin (93rd): the bounds through A, B and Ξ
    // and the gain widening are rules (`RULES_IN_USE`); only the product
    // bound is legacy.
    Derivation::Legacy {
        method: "product bounds of the minors (interval arithmetic for a product)",
        provenance: "interval arithmetic (Moore, 1966): over [x ± δx] and [y ± δy], |x·y − x₀·y₀| ≤ |x₀|·δy + |y₀|·δx + δx·δy. `exact::product_bound`",
        in_relation_to_operators: "the bound of a bilinear form of A: a 2×2 minor m_ij(e, f) = A_i[e]·A_j[f] − A_j[e]·A_i[f] multiplies two relations, and the second-order term δx·δy has no counterpart in A, B or Ξ, whose bounds are linear in δ",
        necessity: Some("the 41st rule (every part reported with its bound, never rounded down), only as long as the minors are read: the proportional grade, the V2 first form, PC2 and PC3. Declared by Origin (93rd); if those readings are fenced from the point of analysis, the necessity narrows to them"),
    },
    Derivation::Legacy {
        method: "exact rationals (arbitrary-precision integers; Euclid's algorithm; Gauss–Jordan elimination)",
        provenance: "Euclid's algorithm (Elements VII); Gauss–Jordan elimination (Gauss; Jordan, 1888); schoolbook multiplication and binary long division. `exact::Ratio`",
        in_relation_to_operators: "adds no form: it computes the orthogonal split's solve over the loci (L = AᵀA, built from A) exactly, as exact summation computes the operators' sums; each value is converted to f64 once, to the nearest or upward",
        necessity: Some("the orthogonal split (Origin, 100th), whose solve needs division that no f64 holds exactly; without it the split's identities (the parts summing to the relation, the potential part closing every turn, the balanced part balancing, orthogonality) would hold only approximately, against the 41st"),
    },
];

/// Rules at the code layer derived from the operators and the declaration
/// (Origin, 2026-11-25, 93rd).
static RULES_IN_USE: &[Derivation] = &[
    Derivation::Rule {
        of: &[A, B, Xi],
        reads: &[Rel, En, Con, Fr],
        rule: "bounds through A, B and Ξ: each is an integer combination of relations, Σ w_f·g_f, and its bound is Σ |w_f|·δ_f, the operator with the signs of its coefficients dropped, applied to the bounds (the triangle inequality). δA = δ[s] + δ[t]; the change bound δ_t + δ_{t+1}; the bound of Ξ(B(A)) is |Ξ|∘|B| applied to δA, term by term, so it equals the operator with the signs dropped wherever no relation enters with opposite signs, and is wider where one cancels in the value (never below; `tests/v10_derivation.rs`, 5). Rounded up, never down (41st)",
    },
    Derivation::Rule {
        of: &[A],
        reads: &[Rel, Con, Fr],
        rule: "gain widening (Z1a): a declared run-constant gain γ carried into a bound, h + γ·(|v| + h), exactly, rounded up. `exact::gain_widened`. The gain is declared, never fitted",
    },
];

/// The minor readings (94th): their necessity is Origin's to state, reading
/// by reading. The Generator's expectation, not a declaration: PC3 and W2's
/// rank have the strongest case, the every-pair proportional grade the
/// weakest.
static NECESSITY_OPEN: &[&str] = &[
    "PersistenceChange.across_components", "PersistenceChange.across_steps", "Couplings.readings",
    "ChangeCoupling.minor", "PersistenceChange.arrangement", "ArrangementStep.before", "ArrangementStep.after",
    "Connection.proportional", "Connection.reorganizing_pairs", "GroundReading.v2_held_arrangement", "Freedom.free_pairs",
];
