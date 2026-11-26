// coherence.rs — Metatron Dynamics, Inc. Kernel V10.
// §2.10, the window and the present reading (Proposed, Generator, 2026-11-24,
// 73rd; W5, the accumulated radial graph, Origin, 74th). Bounded over D. No claim beyond D.
//
// Origin (73rd): "if you have an understanding of the energy freedom and
// connection present in a system represented by relation as primitive you can
// reliably predict the next step ... there are only so many possibilities
// which maintain coherence and the same thing should be true in any
// observable system."
//
// ── What is read ──────────────────────────────────────────────────────────
//
// The window is the field: its declared sequence of registrations (the
// braces, R10). Over it, the field's own readings are taken as held
// constraints on the next change d of each relation:
//
//   PC1 held     relation e, component c, read structure at EVERY step of the
//                window: |d[c][e]| held within its bound. Row: the unit
//                vector on c.
//   G6 balance   against a declared direction n, the held part n·Δ of
//                relation e not resolved at EVERY step: n·d[e] held within
//                its bound. Row: n.
//
// Both are linear in d, with coefficients that are declared, never
// registered (a unit vector; n, 70th), so each row is exact. Every
// constraint here involves one relation's change, so the constraint matrix is
// block-diagonal by relation, and its rank is the sum of the relations' ranks.
//
// **Freedom remaining** = k·m − rank: the number of directions of the next
// change that no held reading bounds. The rank is read through minors: the
// order of the largest minor that is not zero, each minor formed exactly
// (`Expansion`, OC-NUM-1). Minors are how V10 already reads freedom (V8, the
// minor pattern); no division and no elimination are used. Each constraint is
// reported as independent (it raises the rank of those listed before it) or
// implied (it does not); an implied constraint is reported, never dropped
// (§1.5).
//
// **Coherence** ("coherence" is Origin's word: every held reading stays held;
// not the withdrawn "coherence potential", W7). A candidate next registration
// of the relations is appended to the field's relations and every held
// constraint is read again, by the same code, at the new step. It is coherent
// when every held reading stays held. The coherent candidates are the
// anticipated set; their count is the discrete reading of freedom remaining
// (§2.8 (2d): channels). Applied to the registration that arrives, the same
// reading is the forward check: a held reading that ends is a finding about
// the declaration (a relation not in the frame acted, or the structure
// changed), never a violation (§2.8 (2c)).
//
// A candidate is a declared projection (C), never a registration. Nothing
// here is a probability. Held within the window is not held beyond it (59th:
// there is no last intersection).
//
// Not formed here (named, OC-WIN-1): constraints that couple relations or
// components through registered coefficients (PC2 coupled change, G3 phase
// lock), whose rows carry bounds and need a rank at δ; projection beyond one
// step; the quadratic held length (72nd).

use crate::exact::Expansion;
use crate::geometry::{step_held_exchange, DeclaredDirection};
use crate::ground_variables::DeclarationGap;
use crate::persistence_change::{relation_changes, EdgeSeries, Reading};
use crate::relational_field::RelationalField;

/// The largest number of components for which the rank of one relation's
/// constraints is formed (minors by Laplace expansion: k! terms at most).
/// Beyond it the rank is not evaluated, and the relation is named.
pub const RANK_COMPONENT_LIMIT: usize = 6;

/// Which held reading a constraint is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeldKind {
    /// PC1: the change of the relation in this component read structure at
    /// every step of the window.
    Relation { component: usize },
    /// G6: the held part of the relation's change against declared direction
    /// `direction` (an index into the directions given) not resolved at every
    /// step of the window.
    Balance { direction: usize },
}

/// One held reading over the window, as a constraint on the next change of
/// one relation.
#[derive(Debug, Clone, PartialEq)]
pub struct HeldConstraint {
    pub edge: usize,
    pub kind: HeldKind,
    /// Coefficients over the relation's components (declared, exact).
    pub row: Vec<f64>,
}

/// The held constraints on one relation and what they leave free.
#[derive(Debug, Clone, PartialEq)]
pub struct RelationFreedom {
    pub edge: usize,
    /// Indices into `WindowReading::held`, in listed order.
    pub constraints: Vec<usize>,
    /// Constraints that raise the rank of those listed before them.
    pub independent: Vec<usize>,
    /// Constraints implied by those listed before them: reported, not dropped.
    pub implied: Vec<usize>,
    /// The rank, or `None` when not evaluated (k above the limit).
    pub rank: Option<usize>,
    /// k − rank, or `None` when the rank is not evaluated.
    pub freedom_remaining: Option<usize>,
}

/// What this reading does not evaluate, named (W11).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotEvaluated {
    /// No direction declared: balance constraints (G6) not formed.
    NoDeclaredDirection,
    /// Constraints coupling relations or components through registered
    /// coefficients (PC2, G3) are not formed as constraints (OC-WIN-1).
    CouplingConstraints,
    /// The rank of this relation's constraints is not formed: k above
    /// `RANK_COMPONENT_LIMIT`.
    RankAboveLimit { edge: usize },
}

/// §2.10 over one window: the held constraints and the freedom remaining.
#[derive(Debug, Clone, PartialEq)]
pub struct WindowReading {
    /// Steps in the window (every constraint was read held at each).
    pub steps: usize,
    pub k: usize,
    pub m: usize,
    pub held: Vec<HeldConstraint>,
    pub relations: Vec<RelationFreedom>,
    /// Σ over relations of their rank, when every rank is evaluated.
    pub rank: Option<usize>,
    /// k·m − rank, when every rank is evaluated.
    pub freedom_remaining: Option<usize>,
    pub not_evaluated: Vec<NotEvaluated>,
}

/// One held constraint read again at the candidate's step.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kept {
    /// Index into `WindowReading::held`.
    pub constraint: usize,
    /// The constrained value at the new step (nearest f64) and its bound
    /// (rounded up); the reading is decided exactly.
    pub value: f64,
    pub bound: f64,
    pub kept: bool,
}

/// A candidate next registration of the relations: a declared projection (C).
#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    pub label: String,
    /// a[c][e].
    pub a: Vec<Vec<f64>>,
    /// State bound δA[c][e].
    pub da: Vec<Vec<f64>>,
    /// Repeatability part of the state bound [c][e] (Z1a).
    pub da_rep: Vec<Vec<f64>>,
    /// Unshared-gain part of the change bound into this registration [c][e].
    pub dgain: Vec<Vec<f64>>,
}

impl Candidate {
    /// A candidate whose bound is repeatability only.
    pub fn repeatability(label: &str, a: Vec<Vec<f64>>, da: Vec<Vec<f64>>) -> Self {
        let dgain = da.iter().map(|r| vec![0.0; r.len()]).collect();
        Candidate { label: label.into(), a, da_rep: da.clone(), da, dgain }
    }
}

/// The coherence of one candidate: every held constraint read at its step.
#[derive(Debug, Clone, PartialEq)]
pub struct CandidateReading {
    pub label: String,
    pub kept: Vec<Kept>,
    /// True when every held reading stays held.
    pub coherent: bool,
}

// ── Minors, exactly ─────────────────────────────────────────────────────────

/// The determinant of a square matrix of exact entries, by Laplace expansion
/// along the first row: a sum of signed products, formed exactly.
fn det(m: &[Vec<Expansion>]) -> Expansion {
    let n = m.len();
    if n == 1 { return m[0][0].clone(); }
    let mut total = Expansion::new();
    for j in 0..n {
        if m[0][j].signum() == 0 { continue; }
        let minor: Vec<Vec<Expansion>> = m[1..].iter()
            .map(|row| row.iter().enumerate().filter(|&(c, _)| c != j).map(|(_, x)| x.clone()).collect())
            .collect();
        let term = m[0][j].times(&det(&minor));
        if j % 2 == 0 { total.add_expansion(&term); } else { total.add_expansion(&term.neg()); }
    }
    total
}

/// Every r-element subset of 0..n, in order.
fn subsets(n: usize, r: usize) -> Vec<Vec<usize>> {
    fn go(start: usize, n: usize, r: usize, cur: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if cur.len() == r { out.push(cur.clone()); return; }
        for i in start..n { cur.push(i); go(i + 1, n, r, cur, out); cur.pop(); }
    }
    let mut out = Vec::new();
    go(0, n, r, &mut Vec::new(), &mut out);
    out
}

/// The rank of exact rows: the order of the largest minor that is not zero.
pub fn rank_by_minors(rows: &[Vec<f64>]) -> usize {
    if rows.is_empty() { return 0; }
    let cols = rows[0].len();
    let x: Vec<Vec<Expansion>> = rows.iter().map(|r| r.iter().map(|&v| Expansion::of(v)).collect()).collect();
    for r in (1..=rows.len().min(cols)).rev() {
        for rs in subsets(rows.len(), r) {
            for cs in subsets(cols, r) {
                let m: Vec<Vec<Expansion>> = rs.iter().map(|&i| cs.iter().map(|&j| x[i][j].clone()).collect()).collect();
                if det(&m).signum() != 0 { return r; }
            }
        }
    }
    0
}

// ── The window ─────────────────────────────────────────────────────────────

/// Read the held constraints over the whole field (the window) and the
/// freedom they leave. `directions` are the declared directions for G6
/// balances; none declared is named.
pub fn window_reading(field: &RelationalField, directions: &[DeclaredDirection]) -> Result<WindowReading, DeclarationGap> {
    let s = &field.relations;
    let (k, m, steps) = (s.k(), s.n_edges(), s.n_steps());
    let mut held = Vec::new();

    // PC1 held at every step.
    let pc1 = relation_changes(s);
    for e in 0..m {
        for c in 0..k {
            if pc1.iter().filter(|r| r.edge == e && r.component == c).all(|r| r.reading == Reading::Structure) {
                let mut row = vec![0.0; k];
                row[c] = 1.0;
                held.push(HeldConstraint { edge: e, kind: HeldKind::Relation { component: c }, row });
            }
        }
    }
    // G6 balance held at every step, for each declared direction.
    for (di, n) in directions.iter().enumerate() {
        let per_step: Vec<_> = (0..steps).map(|st| step_held_exchange(s, st, n)).collect::<Result<_, _>>()?;
        for e in 0..m {
            if per_step.iter().all(|xs| !xs[e].held.reading.is_resolved()) {
                held.push(HeldConstraint { edge: e, kind: HeldKind::Balance { direction: di }, row: n.n.clone() });
            }
        }
    }

    let mut not_evaluated = vec![NotEvaluated::CouplingConstraints];
    if directions.is_empty() { not_evaluated.insert(0, NotEvaluated::NoDeclaredDirection); }

    let mut relations = Vec::new();
    for e in 0..m {
        let constraints: Vec<usize> = (0..held.len()).filter(|&i| held[i].edge == e).collect();
        if k > RANK_COMPONENT_LIMIT {
            not_evaluated.push(NotEvaluated::RankAboveLimit { edge: e });
            relations.push(RelationFreedom { edge: e, constraints, independent: vec![], implied: vec![], rank: None, freedom_remaining: None });
            continue;
        }
        let (mut independent, mut implied, mut rows): (Vec<usize>, Vec<usize>, Vec<Vec<f64>>) = (vec![], vec![], vec![]);
        for &i in &constraints {
            let before = rank_by_minors(&rows);
            rows.push(held[i].row.clone());
            if rank_by_minors(&rows) > before { independent.push(i); } else { rows.pop(); implied.push(i); }
        }
        let rank = independent.len();
        relations.push(RelationFreedom { edge: e, constraints, independent, implied, rank: Some(rank), freedom_remaining: Some(k - rank) });
    }
    let rank = relations.iter().map(|r| r.rank).sum::<Option<usize>>();
    let freedom_remaining = rank.map(|r| k * m - r);
    Ok(WindowReading { steps, k, m, held, relations, rank, freedom_remaining, not_evaluated })
}

/// The relations with the candidate appended as one further registration.
fn appended(s: &EdgeSeries, c: &Candidate) -> EdgeSeries {
    let (k, m) = (s.k(), s.n_edges());
    assert!(c.a.len() == k && c.a.iter().all(|r| r.len() == m), "a candidate registers every declared relation in every component");
    assert!([&c.da, &c.da_rep, &c.dgain].iter().all(|b| b.len() == k && b.iter().all(|r| r.len() == m && r.iter().all(|&x| x >= 0.0 && x.is_finite()))),
        "a candidate declares a finite, non-negative bound for every entry");
    assert!(c.a.iter().chain(&c.da).chain(&c.da_rep).chain(&c.dgain).flatten().all(|&v| v.is_finite() && crate::exact::in_exact_range(v)),
        "a candidate is within the range of exact arithmetic (OC-NUM-1)");
    let mut x = s.clone();
    x.a.push(c.a.clone());
    x.a_lo.push(vec![vec![0.0; m]; k]);
    x.da.push(c.da.clone());
    x.da_rep.push(c.da_rep.clone());
    x.dgain.push(c.dgain.clone());
    x
}

/// The coherence of each candidate next registration against the window:
/// every held constraint read again, by the same code, at the appended step.
/// Applied to the registration that arrives, this is the forward check.
pub fn coherence(field: &RelationalField, window: &WindowReading, directions: &[DeclaredDirection],
                 candidates: &[Candidate]) -> Result<Vec<CandidateReading>, DeclarationGap> {
    let mut out = Vec::new();
    for cand in candidates {
        let x = appended(&field.relations, cand);
        let st = x.n_steps() - 1;
        let pc1 = relation_changes(&x);
        let g6: Vec<_> = directions.iter().map(|n| step_held_exchange(&x, st, n)).collect::<Result<_, _>>()?;
        let kept: Vec<Kept> = window.held.iter().enumerate().map(|(i, h)| match h.kind {
            HeldKind::Relation { component } => {
                let r = pc1.iter().find(|r| r.step == st && r.edge == h.edge && r.component == component).expect("read at the new step");
                Kept { constraint: i, value: r.change, bound: r.half_width, kept: r.reading == Reading::Structure }
            }
            HeldKind::Balance { direction } => {
                let v = &g6[direction][h.edge].held;
                Kept { constraint: i, value: v.value, bound: v.bound, kept: !v.reading.is_resolved() }
            }
        }).collect();
        let coherent = kept.iter().all(|k| k.kept);
        out.push(CandidateReading { label: cand.label.clone(), kept, coherent });
    }
    Ok(out)
}

/// The labels of the coherent candidates: the anticipated set. Its size is the
/// discrete reading of freedom remaining among the declared candidates.
pub fn anticipated(readings: &[CandidateReading]) -> Vec<&str> {
    readings.iter().filter(|r| r.coherent).map(|r| r.label.as_str()).collect()
}

// ═══════════════════════════════════════════════════════════════════════════
// W5: the accumulated radial graph of every element (Origin, 2026-11-24, 74th)
// ═══════════════════════════════════════════════════════════════════════════
//
// Origin (74th): "a radial graph around each of the elements of a system as
// information is input to the elements the graph behavior changes degrees of
// freedom energy and connection shape the relationships as relational
// progression advances. The window bearing on the point of prediction is
// informed by all of the information which precedes it."
//
// The element is a locus; its radial graph is its star: every relation
// incident on it, leaving it (Out) or arriving at it (In), read inside the
// whole field. Through registration t (steps 0 … t − 1), each spoke carries
// what has accumulated, from the record boundary (Df3: a boundary cut into an
// ongoing relation, not a beginning) to t:
//
//   freedom and structure  per component, the count of steps the spoke's
//                          change read freedom and read structure (PC1), and
//                          its current run: the reading at step t − 1 and the
//                          number of consecutive steps ending there with it;
//   energy                 the exact sum of the exchanged energy ε over
//                          registrations 0 … t (V3, each ε as V3 reports it),
//                          its bound summed and rounded up, and the count of
//                          registrations with the floor resolved;
//   connection             for each pair of spokes, the count of PC2 readings
//                          (both families) within steps 0 … t − 1 that read
//                          coupled (structure, not forced), independent
//                          (freedom), and forced (reported apart, 57th).
//
// Every input updates every spoke of every graph, in one pass (Df3). Nothing
// is removed and nothing is weighted: there is no decay and no learning rate
// (operator parameters are retired from V10). Recency is read as structure,
// the current run, beside the full counts. W1 is the strictest selection from
// W5: the spoke components held over every step.
//
// For a discrete system the spokes are channels (§2.8 (2d)): `ChannelRecord`
// counts each channel taken from each element, a count of registered events
// admitted at δ = 0 (68th), with the step it was last taken.

use crate::persistence_change::ReadingPair;

/// Whether a spoke leaves the element or arrives at it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation { Out, In }

/// The reading at the latest step and the number of consecutive steps ending
/// there with the same reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Run { pub reading: Reading, pub length: usize }

/// One component of one spoke, accumulated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpokeComponent {
    pub component: usize,
    pub held_steps: usize,
    pub changed_steps: usize,
    /// `None` through registration 0: no step yet.
    pub run: Option<Run>,
}

/// Energy exchanged along a spoke, accumulated over registrations 0 … t.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AccumulatedEnergy {
    pub registrations: usize,
    /// Σ ε, one exact sum of the reported ε, rounded once.
    pub sum: f64,
    /// Σ δ_ε, rounded up.
    pub bound: f64,
    /// Registrations with the floor ε > δ_ε resolved.
    pub floor_resolved: usize,
}

/// One spoke of an element's radial graph.
#[derive(Debug, Clone, PartialEq)]
pub struct Spoke {
    pub edge: usize,
    pub orientation: Orientation,
    pub components: Vec<SpokeComponent>,
    /// `Err` with the gap when energy is not read on the field.
    pub energy: Result<AccumulatedEnergy, DeclarationGap>,
}

/// Connection between two spokes of one element, accumulated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpokePair {
    pub e: usize,
    pub f: usize,
    pub coupled: usize,
    pub independent: usize,
    /// Forced by the bound (57th): reported apart, not read as coupled.
    pub forced: usize,
}

/// The radial graph of one element through registration `through`.
#[derive(Debug, Clone, PartialEq)]
pub struct RadialGraph {
    pub locus: usize,
    pub through: usize,
    /// Steps accumulated: `through`.
    pub steps: usize,
    pub spokes: Vec<Spoke>,
    /// `Err` with the gap when PC2 cannot be formed on the field.
    pub pairs: Result<Vec<SpokePair>, DeclarationGap>,
}

fn pair_step(p: ReadingPair) -> usize {
    // The latest step a PC2 reading uses.
    match p { ReadingPair::Components { step, .. } => step, ReadingPair::Steps { step, .. } => step + 1 }
}

/// The radial graph of every element through registration `through`, from the
/// whole field's own readings (PC1 and PC2 from its freedom, ε from its
/// energy). Needs the relations' loci: the declared relations of the field.
pub fn radial_graphs(field: &RelationalField, rel: &crate::operators::DeclaredRelations, through: usize)
    -> Result<Vec<RadialGraph>, DeclarationGap>
{
    use crate::exact::Expansion;
    let s = &field.relations;
    assert!(through < s.n_registrations(), "through a registration of the field");
    assert_eq!(rel.n_edges(), s.n_edges(), "the declared relations of the field");
    let pc = field.ground.freedom.as_ref().map_err(|g| g.clone())?;
    let k = s.k();

    let spoke = |e: usize, orientation: Orientation| -> Spoke {
        let components = (0..k).map(|c| {
            let rs: Vec<Reading> = (0..through).map(|st| pc.relations.iter()
                .find(|r| r.step == st && r.edge == e && r.component == c).expect("PC1 at every step").reading).collect();
            let held_steps = rs.iter().filter(|&&r| r == Reading::Structure).count();
            let run = rs.last().map(|&last| Run { reading: last, length: rs.iter().rev().take_while(|&&r| r == last).count() });
            SpokeComponent { component: c, held_steps, changed_steps: rs.len() - held_steps, run }
        }).collect();
        let energy = field.ground.energy.as_ref().map_err(|g| g.clone()).map(|en| {
            let (mut v, mut h, mut resolved) = (Expansion::new(), Expansion::new(), 0);
            // Reads the fenced magnitude ε (Origin, 103rd: magnitude is not a
            // variable). Named, not changed: whether the accumulated energy
            // becomes the signed sum Σ A(c) is for Origin (step two).
            for t in 0..=through {
                let x = &en.exchanges[t][e];
                v.add(x.epsilon);
                h.add(x.delta_epsilon);
                if x.floor_resolved { resolved += 1; }
            }
            AccumulatedEnergy { registrations: through + 1, sum: v.nearest(), bound: h.up(), floor_resolved: resolved }
        });
        Spoke { edge: e, orientation, components, energy }
    };

    let couplings: Result<Vec<_>, DeclarationGap> = match (&pc.across_components, &pc.across_steps) {
        (Err(a), Err(_)) => Err(a.clone()),
        (a, b) => Ok(a.iter().chain(b.iter()).flat_map(|x| x.readings.iter().copied()).collect()),
    };

    Ok((0..rel.n_nodes).map(|v| {
        let mut spokes = Vec::new();
        for (e, &(src, tgt)) in rel.edges.iter().enumerate() {
            if src == v { spokes.push(spoke(e, Orientation::Out)); }
            if tgt == v { spokes.push(spoke(e, Orientation::In)); }
        }
        let pairs = couplings.as_ref().map_err(|g| g.clone()).map(|cs| {
            let mut edges: Vec<usize> = spokes.iter().map(|x| x.edge).collect();
            edges.sort();
            edges.dedup();
            let mut out = Vec::new();
            for (i, &e) in edges.iter().enumerate() {
                for &f in &edges[i + 1..] {
                    let (mut coupled, mut independent, mut forced) = (0, 0, 0);
                    for r in cs.iter().filter(|r| r.e == e && r.f == f && pair_step(r.over) < through) {
                        if r.forced { forced += 1; }
                        else if r.reading == Reading::Structure { coupled += 1; }
                        else { independent += 1; }
                    }
                    out.push(SpokePair { e, f, coupled, independent, forced });
                }
            }
            out
        });
        RadialGraph { locus: v, through, steps: through, spokes, pairs }
    }).collect())
}

/// Channels taken from each element (§2.8 (2d)): counts of registered events,
/// admitted at δ = 0 (68th), each with the step it was last taken. Every
/// event is kept; nothing is removed or weighted.
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelRecord {
    pub basis: String,
    pub n_channels: usize,
    /// (element, channel) for every registered event, in order.
    pub events: Vec<(usize, usize)>,
    counts: Vec<Vec<usize>>,
    last: Vec<Vec<Option<usize>>>,
}

impl ChannelRecord {
    pub fn new(n_elements: usize, n_channels: usize, basis: &str) -> Self {
        assert!(!basis.trim().is_empty(), "the channels are declared with their basis");
        ChannelRecord { basis: basis.into(), n_channels, events: Vec::new(),
            counts: vec![vec![0; n_channels]; n_elements], last: vec![vec![None; n_channels]; n_elements] }
    }
    /// Register one event: from `element`, channel `channel` taken.
    pub fn register(&mut self, element: usize, channel: usize) {
        assert!(element < self.counts.len() && channel < self.n_channels, "a declared element and channel");
        self.last[element][channel] = Some(self.events.len());
        self.counts[element][channel] += 1;
        self.events.push((element, channel));
    }
    /// The element's radial graph over channels: (channel, count, last event index).
    pub fn graph(&self, element: usize) -> Vec<(usize, usize, Option<usize>)> {
        (0..self.n_channels).map(|c| (c, self.counts[element][c], self.last[element][c])).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_by_minors_on_declared_rows() {
        assert_eq!(rank_by_minors(&[]), 0);
        assert_eq!(rank_by_minors(&[vec![0.0, 0.0]]), 0);
        assert_eq!(rank_by_minors(&[vec![1.0, 0.0], vec![0.0, 1.0], vec![1.0, 1.0]]), 2);
        assert_eq!(rank_by_minors(&[vec![1.0, 2.0, 3.0], vec![2.0, 4.0, 6.0]]), 1);
        assert_eq!(rank_by_minors(&[vec![1.0, 1.0, 0.0], vec![0.0, 1.0, 1.0], vec![1.0, 0.0, -1.0]]), 2);
        assert_eq!(rank_by_minors(&[vec![1.0, 1.0, 0.0], vec![0.0, 1.0, 1.0], vec![1.0, 0.0, 1.0]]), 3);
        // Exact: a row differing from a multiple only in the last bit is independent.
        let eps = f64::EPSILON;
        assert_eq!(rank_by_minors(&[vec![1.0, 1.0], vec![1.0, 1.0 + eps]]), 2);
    }
}
