//! The operators on text's radial graphs (Origin, 2026-11-27, 109th, 110th;
//! the trial registration agreed: *"yes"*).
//!
//! The mapping (109th): every element of the alphabet a locus; each locus's
//! radial graph a spoke to every element, itself included, declared with the
//! basis "the order of reading"; the input calculated across all the radial
//! graphs by the operators. **The trial registration:** one registration per
//! step of the input, +1 on the spoke the text took and 0 on every other
//! spoke. **The trial carrier (the Generator's, for Origin):** energy is
//! what causes the system to progress (109th (4)), so the carrier is the
//! step itself, the movement registered across the spoke taken. Characters
//! are registered exactly: every bound is zero.
//!
//! What the operators give, each checked against its definition on
//! *" a car cras"* (five elements, 25 spokes, ten steps):
//!   A        the spoke taken;
//!   readback out of the element left, into the element entered;
//!   A_p      +1 on the next spoke, −1 on the last (0 where a spoke repeats);
//!   B        the spoke taken, and every spoke entering where it starts;
//!   Ξ        what follows less what precedes, by its definition;
//!   energy   the signed exchange toward the element entered; its net inflow
//!            accumulated over the steps.
//!
//! **What this cannot tell (named):** the operators already compute these
//! forms; the tests fail only where the kernel departs from a definition, and
//! were not shown to fail against a missing feature, because none was added.
//! Connection's grades and freedom's pair levels are formed or named, not
//! read; the subject (110th) is not yet read.
//!
//! [IMPL] SYNTHETIC: text as the metaphorical basis (108th (4)).

#[path = "support/synthetic_whole_field.rs"]
mod support;

use abr_kernel::exact::Expansion;
use abr_kernel::ground_variables::{ObservedRange, RelationCarrier, Side};
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance};
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::relational_field::RelationalField;
use abr_kernel::topology::{FeedbackDeclaration, RelationIdentity};

/// Text's field under the trial registration.
struct TextField { chars: Vec<char>, rel: DeclaredRelations, steps: Vec<usize>, field: RelationalField }

fn text_field(text: &str) -> TextField {
    let cs: Vec<char> = text.chars().collect();
    let mut chars: Vec<char> = Vec::new();
    for &c in &cs { if !chars.contains(&c) { chars.push(c); } }
    let n = chars.len();
    let at = |c: char| chars.iter().position(|&x| x == c).unwrap();
    // The complete radial graphs: spoke (s, t) is relation s·n + t.
    let edges: Vec<(usize, usize)> = (0..n).flat_map(|s| (0..n).map(move |t| (s, t))).collect();
    let m = edges.len();
    let ids = (0..m).map(|e| RelationIdentity { id: format!("SYNTHETIC-spoke-{e}"), basis: "SYNTHETIC: a spoke of a radial graph (109th)".into() }).collect();
    let fb = (0..m).map(|e| FeedbackDeclaration { edge: e, basis: "the order of reading (Origin, 109th)".into() }).collect();
    let rel = DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m]).with_identity(ids).with_feedback(fb);
    let steps: Vec<usize> = cs.windows(2).map(|w| at(w[0]) * n + at(w[1])).collect();
    // One registration per step: +1 on the spoke taken, 0 elsewhere; exact.
    let a: Vec<Vec<Vec<f64>>> = steps.iter().map(|&e| vec![(0..m).map(|f| if f == e { 1.0 } else { 0.0 }).collect()]).collect();
    let da = vec![vec![vec![0.0; m]]; steps.len()];
    let series = EdgeSeries::from_edge_values_synthetic(a.clone(), da.clone()).unwrap();
    let carrier = RelationCarrier { name: "TRIAL: the step, the movement across the spoke taken".into(), unit: "step".into(),
        a: a.into_iter().map(|t| t.into_iter().next().unwrap()).collect(), da: da.into_iter().map(|t| t.into_iter().next().unwrap()).collect(),
        range: ObservedRange::synthetic() };
    let field = RelationalField::from_relations(&rel, series, Some(&carrier)).expect("text's field forms");
    TextField { chars, rel, steps, field }
}

const TEXT: &str = " a car cras";

impl TextField {
    fn n(&self) -> usize { self.chars.len() }
    fn spoke(&self, x: char, y: char) -> usize { let at = |c: char| self.chars.iter().position(|&z| z == c).unwrap(); at(x) * self.n() + at(y) }
    fn ends(&self, e: usize) -> (usize, usize) { self.rel.edges[e] }
}

/// 0. The declaration: five elements, 25 spokes, ten steps, every variable
/// formed or named.
#[test]
fn zero_the_field_of_a_car_cras() {
    let t = text_field(TEXT);
    assert_eq!(t.chars, vec![' ', 'a', 'c', 'r', 's']);
    assert_eq!((t.rel.n_edges(), t.steps.len()), (25, 10));
    assert_eq!(t.steps[0], t.spoke(' ', 'a'));
    assert_eq!(t.steps[9], t.spoke('a', 's'));
    let g = &t.field.ground;
    assert!(g.energy.is_ok() && g.freedom.is_ok() && g.connection.is_ok(), "all three read");
}

/// 1. A at every step is the spoke taken.
#[test]
fn one_a_is_the_spoke_taken() {
    let t = text_field(TEXT);
    for (s, &e) in t.steps.iter().enumerate() {
        for f in 0..t.rel.n_edges() { assert_eq!(t.field.spatial[s].a[0][f], if f == e { 1.0 } else { 0.0 }, "step {s}"); }
    }
}

/// 2. The readback: out of the element left, into the element entered (a
/// self-spoke both leaves and enters its element).
#[test]
fn two_the_readback_is_the_movement_of_the_step() {
    let t = text_field(TEXT);
    for (s, &e) in t.steps.iter().enumerate() {
        let (from, to) = t.ends(e);
        for v in 0..t.n() {
            assert_eq!(t.field.spatial[s].leaving[0][v], if v == from { 1.0 } else { 0.0 }, "step {s}, leaving");
            assert_eq!(t.field.spatial[s].entering[0][v], if v == to { 1.0 } else { 0.0 }, "step {s}, entering");
        }
    }
}

/// 3. A_p: +1 on the next spoke, −1 on the last; the readback of the change
/// shows where the movement turned.
#[test]
fn three_the_change_is_one_spoke_to_the_next() {
    let t = text_field(TEXT);
    for s in 0..t.steps.len() - 1 {
        let (last, next) = (t.steps[s], t.steps[s + 1]);
        for f in 0..t.rel.n_edges() {
            let want = if last == next { 0.0 } else if f == next { 1.0 } else if f == last { -1.0 } else { 0.0 };
            assert_eq!(t.field.persistence[s].a[0][f], want, "step {s}");
        }
    }
}

/// 4. B: the spoke taken, and every spoke entering where it starts. Ξ: what
/// follows less what precedes, by its definition.
#[test]
fn four_b_and_xi_by_their_definitions() {
    let t = text_field(TEXT);
    let m = t.rel.n_edges();
    let succ = |e: usize| -> Vec<usize> { (0..m).filter(|&g| t.rel.edges[g].0 == t.rel.edges[e].1).collect() };
    let pred = |e: usize| -> Vec<usize> { (0..m).filter(|&g| t.rel.edges[g].1 == t.rel.edges[e].0).collect() };
    for (s, &e) in t.steps.iter().enumerate() {
        let from = t.ends(e).0;
        let a = |f: usize| if f == e { 1.0 } else { 0.0 };
        let b = |f: usize| a(f) + succ(f).iter().map(|&g| a(g)).sum::<f64>();
        for f in 0..m {
            let want_b = (f == e) as u8 as f64 + (t.rel.edges[f].1 == from) as u8 as f64;
            assert_eq!(b(f), want_b, "the definition, as stated");
            assert_eq!(t.field.spatial[s].b[0][f], want_b, "step {s}, spoke {f}: B");
            let xi: f64 = succ(f).iter().map(|&g| b(g)).sum::<f64>() - pred(f).iter().map(|&p| b(p)).sum::<f64>();
            assert_eq!(t.field.spatial[s].passage[0][f], xi, "step {s}, spoke {f}: Ξ");
        }
    }
}

/// 5. Energy: the signed exchange toward the element entered at every step;
/// accumulated, the net inflow at each element cancels except where the text
/// began (−1) and where it is now (+1). A finding, for Origin: under this
/// registration the content tells nothing of the path between.
#[test]
fn five_energy_moves_and_its_content_cancels() {
    let t = text_field(TEXT);
    let en = t.field.ground.energy.as_ref().unwrap();
    for (s, &e) in t.steps.iter().enumerate() {
        for f in 0..t.rel.n_edges() {
            let x = &en.signed[s][f];
            if f == e { assert_eq!((x.value, x.side), (1.0, Side::Entering), "step {s}: moving toward the element entered"); }
            else { assert_eq!((x.value, x.side), (0.0, Side::NotResolved)); }
        }
    }
    let mut net = vec![Expansion::new(); t.n()];
    for s in 0..t.steps.len() {
        for v in 0..t.n() {
            net[v] = net[v].plus(&Expansion::of(t.field.spatial[s].entering[0][v])).minus(&Expansion::of(t.field.spatial[s].leaving[0][v]));
        }
    }
    let (start, now) = (t.ends(t.steps[0]).0, t.ends(*t.steps.last().unwrap()).1);
    for v in 0..t.n() {
        let want = (v == now) as i32 as f64 - (v == start) as i32 as f64;
        assert_eq!(net[v].nearest(), want, "element {:?}", t.chars[v]);
    }
}
