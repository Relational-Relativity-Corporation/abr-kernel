//! The field read over all the input in order indicates the next character
//! (Origin, 2026-11-27, 112th): *"The only way that this model works at all
//! as if the relational field determined by the 26 radial graphs read over
//! all of the input in order is used to indicate the next character"*.
//!
//! `order::indications` reads the relational field itself: the readback
//! (K9) of every step, out of one element and into the next, kept in order
//! and never summed. At the current point, every earlier point where the
//! text stood at the current element indicates its continuation; how far its
//! arrival runs alongside the current one, and where they part, are read by
//! comparing the two readbacks in order at matching points.
//!
//! 1. On actual text, at every point of evolution, the indications equal
//!    the degrees (108th) exactly: the same reading, now formed from the
//!    field's readback.
//! 2. A field whose change is not one step of reading is refused, named.
//! 3. What the indications show on the worked text, at every point: where
//!    the text's actual next character was indicated (a framework count,
//!    not a rule).
//!
//! **Not read here (named):** the choice among the indications (110th,
//! open); energy beyond the trial carrier. Actual text (111th (2)).

#[path = "support/synthetic_whole_field.rs"]
mod support;

use abr_kernel::operators::{DeclaredRelations, EdgeProvenance};
use abr_kernel::order::{indications, IndicationGap, Order};
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::relational_field::RelationalField;
use abr_kernel::topology::{FeedbackDeclaration, RelationIdentity};

struct TextField { rel: DeclaredRelations, steps: Vec<usize>, field: RelationalField }

fn text_field(text: &str) -> TextField {
    let cs: Vec<char> = text.chars().collect();
    let mut chars: Vec<char> = Vec::new();
    for &c in &cs { if !chars.contains(&c) { chars.push(c); } }
    let n = chars.len();
    let at = |c: char| chars.iter().position(|&x| x == c).unwrap();
    let edges: Vec<(usize, usize)> = (0..n).flat_map(|s| (0..n).map(move |t| (s, t))).collect();
    let m = edges.len();
    let ids = (0..m).map(|e| RelationIdentity { id: format!("spoke-{e}"), basis: "a spoke of a radial graph (109th)".into() }).collect();
    let fb = (0..m).map(|e| FeedbackDeclaration { edge: e, basis: "the order of reading (Origin, 109th)".into() }).collect();
    let rel = DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m]).with_identity(ids).with_feedback(fb);
    let steps: Vec<usize> = cs.windows(2).map(|w| at(w[0]) * n + at(w[1])).collect();
    let mut stand = vec![0.0; m];
    let a: Vec<Vec<Vec<f64>>> = steps.iter().map(|&e| { stand[e] += 1.0; vec![stand.clone()] }).collect();
    let series = EdgeSeries::from_edge_values_synthetic(a, vec![vec![vec![0.0; m]]; steps.len()]).unwrap();
    let field = RelationalField::from_relations(&rel, series, None).expect("text's field forms");
    TextField { rel, steps, field }
}

const TEXTS: [&str; 2] = [" a car cras", "he swung the bat. he swung the ball. the cat sat. he swung the ba"];

/// 1. The indications, read from the field's readback in order, equal the
/// degrees at every point of evolution.
#[test]
fn one_the_indications_are_the_degrees_read_from_the_field() {
    for text in TEXTS {
        let t = text_field(text);
        for p in 1..=t.steps.len() {
            let from_field = indications(&t.field, &t.rel, p).expect("a field read in order");
            let degrees = Order::new(&t.rel, t.steps[..p].to_vec()).unwrap().degrees();
            assert_eq!(from_field, degrees, "{text:?}, through step {p}");
        }
    }
}

/// 2. A field whose change is not one step of reading (here: two spokes
/// moving at once) is refused, and the point named.
#[test]
fn two_a_field_not_read_in_order_is_refused() {
    let t = text_field(" a car cras");
    let m = t.rel.n_edges();
    let mut a: Vec<Vec<Vec<f64>>> = (0..t.field.relations.a.len()).map(|p| t.field.relations.a[p].clone()).collect();
    a[3][0][0] += 1.0;
    let series = EdgeSeries::from_edge_values_synthetic(a, vec![vec![vec![0.0; m]]; t.steps.len()]).unwrap();
    let f = RelationalField::from_relations(&t.rel, series, None).unwrap();
    assert_eq!(indications(&f, &t.rel, t.steps.len()), Err(IndicationGap::NotOneStep { point: 3 }));
}

/// 3. On the worked text, at every point: how often the character the text
/// actually took next was among those indicated. Framework counts, not a
/// rule; they equal the reading across the input (108th example).
#[test]
fn three_what_the_indications_show_on_the_worked_text() {
    let t = text_field(TEXTS[1]);
    let (mut first, mut beyond, mut indicated) = (0, 0, 0);
    for p in 1..t.steps.len() {
        let ind = indications(&t.field, &t.rel, p).unwrap();
        let took = t.steps[p];
        if ind.is_empty() { first += 1 } else if ind.iter().any(|d| d.leaving == took) { indicated += 1 } else { beyond += 1 }
    }
    assert_eq!((first, beyond, indicated), (13, 9, 41), "the element met for the first time; the text took what nothing indicated; the text took what was indicated");
}
