// radial.rs — Metatron Dynamics, Inc. Kernel V10.
// The radial field: the relational field on complete radial graphs, read
// step by step (Origin, 2026-11-27, the compute rebuild). Bounded over D.
//
// The complete radial graphs (109th): n elements, each with a spoke to every
// element, itself included. Each step registers one movement along one
// spoke; each registration is the radial graphs as they stand (111th). On
// this shape the definitions of the operators reduce exactly to A and the
// readback (K9):
//
//     B[x→y] = A[x→y] + Σ_z A[y→z]                       = A[x→y] + out(y)
//     Ξ[x→y] = Σ_z B[y→z] − Σ_w B[w→x]
//            = (out(y) + Σ_z out(z)) − (in(x) + n·out(x)) = out(y) + T − in(x) − n·out(x)
//
// with out(v) and in(v) the readback's halves at v (the movement out of and
// into it so far) and T = Σ_z out(z) the steps so far. These are the
// definitions evaluated on the declared shape, not approximations: each is
// tested equal to `RelationalField` at every point (`tests/v10_radial.rs`).
// Every reading is linear in the registered steps (104th): T and n·out(x)
// are sums of the steps, n is the declared shape.
//
// Kept: the order of steps (never a table per registration), the movement
// each spoke has carried (sparse: only spokes taken), and each element's
// halves. A step costs one spoke and two elements; B and Ξ at a spoke cost a
// constant; the indications at a point cost the earlier points at its
// element and how far each runs alongside.
//
// Not formed here (named): the ground readings (energy beyond the step,
// freedom's pair levels, connection's grades), which text does not read;
// they stay in `RelationalField`. Values are integers, exact.

use std::collections::HashMap;

use crate::order::Degree;

// The activity at a point, and coherence (Proposed, under trial, Origin,
// 2026-11-27, 116th: *"we should avoid any sort of probability and instead
// use coherence which is what our brains use"*). A metaphor, labelled
// (108th (4)): a neuron is an element's locus, a synapse a spoke, a firing a
// step. *Amended (Origin, 2026-11-27):* "it is synaptic activity which
// describes neural Nets ... It is information across the synapse which
// drives neural firing". The activity at the current point is the synapses
// fired at the steps behind it, within a declared reach (a scope of
// analysis, 85th). At each degree of freedom (108th), the active synapse at
// distance k agrees where the relation fired k steps behind that earlier
// point is the same relation (*was:* the element, the neuron; fenced): read at every
// distance within the reach, not stopping where the arrivals part, and never
// pooled across points (106th). The coherent degrees are those whose
// agreement no other degree's agreement strictly contains: inclusion of
// sets, never a count, a sum or a score (104th). Each continues by the
// relation registered at its point, so continuing by a coherent degree is
// continuing by an indicated one (`tests/v10_no_drift.rs`).
//
// **Fenced (Origin, 2026-11-27, the 116th amended):** *"There are no
// independent synapses everything is calculated together in the relational
// field"*. The agreement per distance and coherence by inclusion compute parts
// apart and compare them; kept as a check, never a reading.
//
// Not admitted (named): the comparison of elements at matching points is
// the 108th's comparison continued past where the arrivals part; no operator
// forms it; it waits on Origin (`tests/guard/origin_admitted.rs`).

/// The readings of one synapse (a spoke x→y) as the whole field stands at a
/// registration (the 116th amended: every radial graph calculated together,
/// one field; the synapse is a view of it, never read apart). Each is the
/// operators' definition on the complete radial graphs, reduced exactly
/// (B = A + out(y); Ξ = out(y) + T − in(x) − n·out(x)).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynapseReading {
    /// The element the synapse leaves.
    pub from: usize,
    /// The element it enters.
    pub to: usize,
    /// A: the movement the synapse has carried, as registered.
    pub a: i64,
    /// B: the synapse joined to every relation leaving where it enters.
    pub b: i64,
    /// Ξ, the passage: what follows the synapse less what precedes it.
    pub passage: i64,
    /// The readback's leaving half at `from` (K9): what has left it.
    pub leaving: i64,
    /// The readback's entering half at `to` (K9): what has entered it.
    pub entering: i64,
}

/// The most distances an activity holds: the agreement is held as one
/// machine word per degree. A limit of this implementation, not of the
/// mathematics; a wider reach is refused, never cut.
pub const MOST_REACH: usize = 64;

/// Why an activity is refused (never repaired).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivityRefusal {
    /// The point is not in the field: `through` steps read, 1 ≤ through ≤ steps.
    Through { through: usize, steps: usize },
    /// A reach wider than this implementation holds.
    Reach { reach: usize, most: usize },
}

/// One degree of freedom of the current point and how the active neurons
/// agree with it (116th, under trial). Distances are identities in the
/// order: distance k is the active neuron at the point k behind the current
/// one, matched with the point k behind the degree's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Agreement {
    /// The earlier point in evolution where the text stood at the current element.
    pub point: usize,
    /// The relation that left it there, as registered (x·n + y).
    pub leaving: usize,
    /// The distances at which the synapse fired behind the earlier point is
    /// the synapse fired behind the current point (the same relation), in
    /// increasing order. *Was:* the element (the neuron), fenced.
    pub agrees: Vec<usize>,
    /// The distances at which they are different elements.
    pub differs: Vec<usize>,
    /// The distances behind the record boundary of either point.
    pub beyond: Vec<usize>,
}

/// **Fenced (the 116th amended):** a check, never a reading.
///
/// The activity at a point: every degree of freedom with its agreement, and
/// the coherent degrees (116th, under trial).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Activity {
    /// The current point: where step `point − 1` entered.
    pub point: usize,
    /// The declared reach: how many points behind the current point are
    /// active. A scope of analysis (85th), never a boundary of the field.
    pub reach: usize,
    /// Every degree of the current point, in order of evolution, none merged.
    pub agreements: Vec<Agreement>,
    /// The coherent degrees, as positions in `agreements`, in order: those
    /// whose agreement no other degree's agreement strictly contains.
    pub coherent: Vec<usize>,
}

/// Why a step is refused (never repaired).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RadialRefusal {
    /// An element not declared.
    NotDeclared { element: usize },
    /// The step does not leave where the last step entered (the order is
    /// read in order: one direction, nothing cut, 85th).
    Breaks { leaves: usize, last_entered: usize },
}

/// The relational field on complete radial graphs over `n` declared
/// elements, registered one step at a time.
#[derive(Debug, Clone, PartialEq)]
pub struct RadialField {
    n: usize,
    /// The order: each step as (the element left, the element entered).
    steps: Vec<(usize, usize)>,
    /// The movement each spoke has carried; spokes never taken are absent
    /// (their A is 0).
    a: HashMap<(usize, usize), i64>,
    /// The readback's halves at each element, the field as it stands.
    out: Vec<i64>,
    into: Vec<i64>,
}

impl RadialField {
    /// A field over `n` declared elements, before any step.
    pub fn new(n: usize) -> Self {
        RadialField { n, steps: Vec::new(), a: HashMap::new(), out: vec![0; n], into: vec![0; n] }
    }

    /// Declare one more element (its radial graph, a spoke to every element);
    /// returns its index. Text's alphabet is declared in advance (109th); an
    /// open vocabulary is declared as it is observed.
    pub fn declare(&mut self) -> usize {
        self.out.push(0);
        self.into.push(0);
        self.n += 1;
        self.n - 1
    }

    /// Register a step: one movement from `x` to `y`.
    pub fn push(&mut self, x: usize, y: usize) -> Result<(), RadialRefusal> {
        for v in [x, y] { if v >= self.n { return Err(RadialRefusal::NotDeclared { element: v }); } }
        if let Some(&(_, last)) = self.steps.last() {
            if x != last { return Err(RadialRefusal::Breaks { leaves: x, last_entered: last }); }
        }
        self.steps.push((x, y));
        *self.a.entry((x, y)).or_insert(0) += 1;
        self.out[x] += 1;
        self.into[y] += 1;
        Ok(())
    }

    /// The elements declared.
    pub fn n(&self) -> usize { self.n }
    /// The steps registered, T.
    pub fn steps(&self) -> usize { self.steps.len() }
    /// Step p of the order, as (the element left, the element entered): the
    /// readback of the change between registrations, in order.
    pub fn step(&self, p: usize) -> (usize, usize) { self.steps[p] }
    /// A[x→y]: the movement the spoke has carried.
    pub fn a(&self, x: usize, y: usize) -> i64 { *self.a.get(&(x, y)).unwrap_or(&0) }
    /// The readback at v, leaving: the movement out of it so far, out(v).
    pub fn leaving(&self, v: usize) -> i64 { self.out[v] }
    /// The readback at v, entering: the movement into it so far, in(v).
    pub fn entering(&self, v: usize) -> i64 { self.into[v] }
    /// B[x→y] = A[x→y] + out(y): the spoke and what leaves where it leads.
    pub fn b(&self, x: usize, y: usize) -> i64 { self.a(x, y) + self.out[y] }
    /// Ξ[x→y] = out(y) + T − in(x) − n·out(x): what follows the spoke less
    /// what precedes it, the passage.
    pub fn xi(&self, x: usize, y: usize) -> i64 {
        self.out[y] + self.steps.len() as i64 - self.into[x] - self.n as i64 * self.out[x]
    }

    /// The element at point j of the order (point 0 where step 0 leaves).
    fn locus(&self, j: usize) -> usize { if j == 0 { self.steps[0].0 } else { self.steps[j - 1].1 } }

    /// The indications at the point after `through` steps (112th): every
    /// earlier point at the current element, its continuation, its turn to
    /// now, how far its arrival runs alongside, and where the two part.
    /// Relations are numbered x·n + y, as on the declared radial graphs.
    pub fn indications(&self, through: usize) -> Vec<Degree> {
        let here = self.locus(through);
        let spoke = |s: (usize, usize)| s.0 * self.n + s.1;
        (0..through).filter(|&j| self.locus(j) == here).map(|j| {
            let mut k = 0;
            while k < j && self.steps[j - 1 - k] == self.steps[through - 1 - k] { k += 1; }
            let parts = if k == j { None } else { Some((spoke(self.steps[j - 1 - k]), spoke(self.steps[through - 1 - k]))) };
            Degree { point: j, leaving: spoke(self.steps[j]), turn: (j, through), alongside: (j - k, j), parts }
        }).collect()
    }

    /// The readings of the synapse x→y as the whole field stands now.
    pub fn synapse(&self, x: usize, y: usize) -> SynapseReading {
        SynapseReading { from: x, to: y, a: self.a(x, y), b: self.b(x, y), passage: self.xi(x, y), leaving: self.out[x], entering: self.into[y] }
    }

    /// The activity at the point after `through` steps, within `reach`
    /// (116th, under trial).
    ///
    /// *Amended (Origin, 2026-11-27):* *"it is synaptic activity which
    /// describes neural Nets ... It is information across the synapse which
    /// drives neural firing"*. The active synapse at distance k is the
    /// relation fired k steps behind the point; it agrees where the relation
    /// fired k steps behind the earlier point is the same relation.
    pub fn activity(&self, through: usize, reach: usize) -> Result<Activity, ActivityRefusal> {
        self.read_activity(through, reach, &|j, k| self.steps[j - k] == self.steps[through - k])
    }

    /// **Fenced (Origin, 2026-11-27):** the first reading of the 116th, the
    /// elements (the neurons) at matching points compared; kept as a check,
    /// never a reading: *"It is information across the synapse which drives
    /// neural firing"*.
    pub fn element_activity(&self, through: usize, reach: usize) -> Result<Activity, ActivityRefusal> {
        self.read_activity(through, reach, &|j, k| self.locus(j - k) == self.locus(through - k))
    }

    /// The activity, with the comparison at distance k behind the earlier
    /// point j and the current point (1 ≤ k ≤ j).
    fn read_activity(&self, through: usize, reach: usize, same: &dyn Fn(usize, usize) -> bool) -> Result<Activity, ActivityRefusal> {
        if through == 0 || through > self.steps.len() { return Err(ActivityRefusal::Through { through, steps: self.steps.len() }); }
        if reach > MOST_REACH { return Err(ActivityRefusal::Reach { reach, most: MOST_REACH }); }
        let here = self.locus(through);
        let mut agreements = Vec::new();
        let mut held: Vec<u64> = Vec::new();
        for j in (0..through).filter(|&j| self.locus(j) == here) {
            let (mut agrees, mut differs, mut beyond, mut word) = (Vec::new(), Vec::new(), Vec::new(), 0u64);
            for k in 1..=reach {
                if k > j { beyond.push(k); }
                else if same(j, k) { agrees.push(k); word |= 1 << (k - 1); }
                else { differs.push(k); }
            }
            let (x, y) = self.steps[j];
            agreements.push(Agreement { point: j, leaving: x * self.n + y, agrees, differs, beyond });
            held.push(word);
        }
        // Inclusion: a degree is coherent unless another's agreement strictly
        // contains its own. Distinct agreements are compared widest first; an
        // agreement inside another lies inside one already kept.
        let mut distinct: Vec<u64> = held.clone();
        distinct.sort_unstable_by(|a, b| b.count_ones().cmp(&a.count_ones()).then(a.cmp(b)));
        distinct.dedup();
        let mut kept: Vec<u64> = Vec::new();
        for w in distinct { if !kept.iter().any(|&m| m & w == w) { kept.push(w); } }
        let coherent = (0..held.len()).filter(|&i| kept.contains(&held[i])).collect();
        Ok(Activity { point: through, reach, agreements, coherent })
    }
}
