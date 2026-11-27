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
}
