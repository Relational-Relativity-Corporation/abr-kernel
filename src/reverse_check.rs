//! Kernel V10.3 — the reverse check (B2), kernel side (RPPI-V10).
//!
//! B2: "from a reported number and its identity, an executable path
//! reselects registrations, rebuilds fields, re-executes operators and
//! returns MATCH or MISMATCH." Reselecting registrations from a record is
//! the application's part (it holds the record and its identities). This
//! module is the kernel's part: given the declared inputs the reading was
//! formed from and the reading's identity (step, component, edges), it
//! rebuilds A from the inputs, re-derives the bounds under Z1a, re-executes
//! the form and compares, **without calling the code that produced the
//! reading** (`EdgeSeries`, `operator_a`, `persistence_change`'s forms). It
//! also checks that the reading was admissible to report: that its edges
//! entered and that the pair is not a declared identity (PC-D4).
//!
//! Comparison is exact: the same declared inputs and the same order of
//! arithmetic give the same floating-point result. A MISMATCH names the first
//! quantity that differs. Nothing is repaired.

use crate::ground_variables::{AtDelta, Registration, Sign};
use crate::operators::DeclaredRelations;
use crate::persistence_change::{ArrangementStep, ChangeCoupling, Reading, ReadingPair, RelationChange};

/// The outcome of a reverse check.
#[derive(Debug, Clone, PartialEq)]
pub enum Verdict {
    Match,
    /// The first quantity that differs, with the reported and the re-derived value.
    Mismatch { quantity: &'static str, reported: String, rederived: String },
}

fn mismatch<T: std::fmt::Debug>(q: &'static str, reported: T, rederived: T) -> Verdict {
    Verdict::Mismatch { quantity: q, reported: format!("{reported:?}"), rederived: format!("{rederived:?}") }
}

/// The declared inputs a reading was formed from.
#[derive(Debug, Clone, Copy)]
pub enum Inputs<'a> {
    /// Node-valued registrations over declared relations (link 4, node-derived).
    Nodes { seq: &'a [Registration], rel: &'a DeclaredRelations },
    /// Edge-registered values (ER-1): a[t][c][e], repeatability δ[t][c][e],
    /// the run-constant offset bound [c][e] and the independent gain bound
    /// [c][e] (0 where none was declared; shared gains enter no reading).
    Edges { a: &'a [Vec<Vec<f64>>], delta: &'a [Vec<Vec<f64>>], offset: &'a [Vec<f64>], independent_gain: &'a [Vec<f64>] },
}

impl Inputs<'_> {
    fn k(&self) -> usize {
        match self { Inputs::Nodes { seq, .. } => seq[0].x.k, Inputs::Edges { a, .. } => a[0].len() }
    }
    fn m(&self) -> usize {
        match self { Inputs::Nodes { rel, .. } => rel.edges.len(), Inputs::Edges { a, .. } => a[0][0].len() }
    }
    fn regs(&self) -> usize {
        match self { Inputs::Nodes { seq, .. } => seq.len(), Inputs::Edges { a, .. } => a.len() }
    }
    /// A at (t, c, e), rebuilt from the inputs.
    // Exact re-derivation (OC-NUM-1, 45th), written apart from the forms:
    // values exact and compared after rounding once, bounds exact and
    // compared after rounding up, readings decided exactly. *Was:* f64 in
    // sequence, mirroring the forms' former arithmetic.
    fn a(&self, t: usize, c: usize, e: usize) -> X {
        match self {
            Inputs::Nodes { seq, rel } => { let (s, u) = rel.edges[e]; X::diff(seq[t].x.data[c][s], seq[t].x.data[c][u]) }
            Inputs::Edges { a, .. } => X::of(a[t][c][e]),
        }
    }
    /// The state bound as the forms store it: each locus's exact bound, the
    /// two ends' sum rounded up.
    fn state_hw(&self, t: usize, c: usize, e: usize) -> X {
        match self {
            Inputs::Nodes { seq, rel } => {
                let (s, u) = rel.edges[e];
                let r = &seq[t];
                let at = |i: usize| {
                    let mut h = X::of(r.delta[c][i]);
                    h.add(r.offset[c][i]);
                    h.add_product(r.locus_gain[c][i], r.x.data[c][i].abs());
                    h.add_product(r.locus_gain[c][i], r.delta[c][i]);
                    h
                };
                X::of(at(s).plus(&at(u)).up())
            }
            Inputs::Edges { delta, offset, .. } => X::of(sum_up([delta[t][c][e], offset[c][e]])),
        }
    }
    fn change(&self, s: usize, c: usize, e: usize) -> (X, X) {
        let d = self.a(s + 1, c, e).minus(&self.a(s, c, e));
        let hw = match self {
            Inputs::Nodes { seq, rel } => {
                let (p, q) = rel.edges[e];
                let (r0, r1) = (&seq[s], &seq[s + 1]);
                // The stored parts, each rounded up as the forms store them.
                let rep = |r: &Registration| sum_up([r.delta[c][p], r.delta[c][q]]);
                let g = &r0.locus_gain[c];
                let dx = |i: usize| X::diff(r1.x.data[c][i], r0.x.data[c][i]).abs().plus(&X::of(r0.delta[c][i])).plus(&X::of(r1.delta[c][i]));
                let gain = dx(p).times_f64(g[p]).plus(&dx(q).times_f64(g[q])).up();
                let mut h = X::of(rep(r1)); h.add(rep(r0)); h.add(gain);
                h
            }
            Inputs::Edges { delta, .. } => { let mut h = X::of(delta[s + 1][c][e]); h.add(delta[s][c][e]); h }
        };
        (d, hw)
    }
    fn mixing(&self, c: usize, e: usize, v: &X, hw: &X) -> X {
        match self { Inputs::Nodes { .. } => hw.clone(), Inputs::Edges { independent_gain, .. } => gain_widened(v, hw, independent_gain[c][e]) }
    }
    fn forced(&self, e: usize, f: usize) -> bool {
        match self { Inputs::Nodes { rel, .. } => rel.edges[e] == rel.edges[f], Inputs::Edges { .. } => false }
    }
}

use crate::exact::{exceeds, gain_widened, product_bound, sum_up, Expansion as X};

fn at_delta(v: &X, hw: &X) -> AtDelta<Sign> {
    if exceeds(v, hw) { AtDelta::Resolved(if v.signum() > 0 { Sign::Positive } else { Sign::Negative }) } else { AtDelta::NotResolved }
}

fn reading_of(v: &X, hw: &X) -> Reading { if exceeds(v, hw) { Reading::Freedom } else { Reading::Structure } }

fn identity_in_range(inp: &Inputs, step_last: usize, comps: &[usize], edges: &[usize]) -> Option<Verdict> {
    if step_last >= inp.regs() { return Some(mismatch("step", step_last, inp.regs() - 1)); }
    if let Some(&c) = comps.iter().find(|&&c| c >= inp.k()) { return Some(mismatch("component", c, inp.k() - 1)); }
    if let Some(&e) = edges.iter().find(|&&e| e >= inp.m()) { return Some(mismatch("edge", e, inp.m() - 1)); }
    None
}

/// Reverse check of one PC1 reading.
pub fn reverse_relation(inp: Inputs, x: &RelationChange) -> Verdict {
    if let Some(v) = identity_in_range(&inp, x.step + 1, &[x.component], &[x.edge]) { return v; }
    let (d, hw) = inp.change(x.step, x.component, x.edge);
    if d.nearest() != x.change { return mismatch("change", x.change, d.nearest()); }
    if hw.up() != x.half_width { return mismatch("half_width", x.half_width, hw.up()); }
    let r = reading_of(&d, &hw);
    if r != x.reading { return mismatch("reading", x.reading, r); }
    Verdict::Match
}

/// Reverse check of one PC2 reading, (a) across components or (b) across steps.
pub fn reverse_coupling(inp: Inputs, x: &ChangeCoupling) -> Verdict {
    let ((s0, c0), (s1, c1), last) = match x.over {
        ReadingPair::Components { step, i, j } => ((step, i), (step, j), step + 1),
        ReadingPair::Steps { component, step } => ((step, component), (step + 1, component), step + 2),
    };
    if let Some(v) = identity_in_range(&inp, last, &[c0, c1], &[x.e, x.f]) { return v; }
    if x.e >= x.f { return mismatch("pair order (e < f)", (x.e, x.f), (x.e.min(x.f), x.e.max(x.f))); }
    if let ReadingPair::Components { i, j, .. } = x.over {
        if i >= j { return mismatch("component order (i < j)", (i, j), (i.min(j), i.max(j))); }
    }
    if inp.forced(x.e, x.f) { return mismatch("pair admissible (not a declared identity, PC-D4)", true, false); }
    let (xe, xf) = (inp.change(s0, c0, x.e), inp.change(s0, c0, x.f));
    let (ye, yf) = (inp.change(s1, c1, x.e), inp.change(s1, c1, x.f));
    let moved = |p: &(X, X), q: &(X, X)| exceeds(&p.0, &p.1) || exceeds(&q.0, &q.1);
    if !moved(&xe, &ye) || !moved(&xf, &yf) {
        return mismatch("edges entered (change resolved in either reading, PC1)", true, false);
    }
    let minor = xe.0.times(&yf.0).minus(&ye.0.times(&xf.0));
    // A minor across components carries the independent gains; across steps
    // (one component) it does not.
    let mixes = matches!(x.over, ReadingPair::Components { .. });
    let h = |c: usize, e: usize, p: &(X, X)| if mixes { inp.mixing(c, e, &p.0, &p.1) } else { p.1.clone() };
    let (hxe, hxf, hye, hyf) = (h(c0, x.e, &xe), h(c0, x.f, &xf), h(c1, x.e, &ye), h(c1, x.f, &yf));
    let hw = product_bound(&xe.0, &hxe, &yf.0, &hyf).plus(&product_bound(&ye.0, &hye, &xf.0, &hxf));
    if minor.nearest() != x.minor { return mismatch("minor", x.minor, minor.nearest()); }
    if hw.up() != x.half_width { return mismatch("half_width", x.half_width, hw.up()); }
    let r = reading_of(&minor, &hw);
    if r != x.reading { return mismatch("reading", x.reading, r); }
    Verdict::Match
}

/// Reverse check of one PC3 reading.
pub fn reverse_arrangement(inp: Inputs, x: &ArrangementStep) -> Verdict {
    if let Some(v) = identity_in_range(&inp, x.step + 1, &[x.i, x.j], &[x.e, x.f]) { return v; }
    if x.e >= x.f || x.i >= x.j { return mismatch("index order (e < f, i < j)", (x.e, x.f, x.i, x.j), (x.e, x.f, x.i, x.j)); }
    if inp.forced(x.e, x.f) { return mismatch("pair admissible (not a declared identity, PC-D4)", true, false); }
    let resolved = |t: usize, e: usize| (0..inp.k()).any(|c| exceeds(&inp.a(t, c, e), &inp.state_hw(t, c, e)));
    for &e in &[x.e, x.f] {
        if !(resolved(x.step, e) && resolved(x.step + 1, e)) {
            return mismatch("edge entered (A resolved at both registrations)", true, false);
        }
    }
    let state = |t: usize| {
        let a = |c: usize, e: usize| inp.a(t, c, e);
        let h = |c: usize, e: usize| inp.mixing(c, e, &inp.a(t, c, e), &inp.state_hw(t, c, e));
        let (i, j, e, f) = (x.i, x.j, x.e, x.f);
        let m = a(i, e).times(&a(j, f)).minus(&a(j, e).times(&a(i, f)));
        let hw = product_bound(&a(i, e), &h(i, e), &a(j, f), &h(j, f)).plus(&product_bound(&a(j, e), &h(j, e), &a(i, f), &h(i, f)));
        at_delta(&m, &hw)
    };
    let (b, a) = (state(x.step), state(x.step + 1));
    if b != x.before { return mismatch("before", x.before, b); }
    if a != x.after { return mismatch("after", x.after, a); }
    let r = if b == a { Reading::Structure } else { Reading::Freedom };
    if r != x.reading { return mismatch("reading", x.reading, r); }
    Verdict::Match
}
