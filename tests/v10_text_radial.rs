//! The radial graphs as the field evolves (Origin, 2026-11-27, 111th): *"I
//! see the radial graphs changing continuously as the relational field
//! evolves"*. Tested on actual text (111th (2)).
//!
//! The mapping (109th): every element of the alphabet a locus; each locus's
//! radial graph a spoke to every element, itself included, declared with the
//! basis "the order of reading". **Each registration is the radial graphs as
//! they stand** at that point of evolution: the movement every spoke has
//! carried so far. **The step the text takes is the change** between one
//! registration and the next (A_p). The trial carrier (the Generator's, for
//! Origin): the step itself, the movement across the spoke taken.
//!
//! Checked against the definitions, at every point of evolution:
//!   A         the radial graphs as they stand (each spoke's movement so far);
//!   A_p       the step taken next, and nothing else;
//!   readback  of the step: out of the element left, into the element
//!             entered (energy moving); of the field as it stands: the
//!             movement out of and into each element so far;
//!   B, Ξ      of the field as it stands, by their definitions;
//!   energy    the signed exchange toward the element entered;
//!   evolution every registration kept, one per step.
//!
//! **Stated, not hidden:** the value on each spoke is the number of times it
//! has been taken (the table an n-gram model counts); nothing here reads it
//! as a frequency: the operators read relation across it and its change.
//! **Not read here (named):** the subject (110th) and the choice of the next
//! step; connection's grades and freedom's pair levels are formed, not read.
//!
//! [IMPL] Text as the metaphorical basis (108th (4)); actual text, exact.

#[path = "support/synthetic_whole_field.rs"]
mod support;

use abr_kernel::exact::Expansion;
use abr_kernel::ground_variables::{ObservedRange, RelationCarrier, Side};
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance};
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::relational_field::RelationalField;
use abr_kernel::topology::{FeedbackDeclaration, RelationIdentity};

struct TextField { chars: Vec<char>, rel: DeclaredRelations, steps: Vec<usize>, field: RelationalField }

/// Text's field: the complete radial graphs (spoke (s, t) is relation
/// s·n + t), registration t the graphs as they stand after step t.
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
    // The radial graphs as they stand: the movement each spoke has carried.
    let mut stand = vec![0.0; m];
    let a: Vec<Vec<Vec<f64>>> = steps.iter().map(|&e| { stand[e] += 1.0; vec![stand.clone()] }).collect();
    let da = vec![vec![vec![0.0; m]]; steps.len()];
    let series = EdgeSeries::from_edge_values_synthetic(a, da.clone()).unwrap();
    // The trial carrier: the step itself.
    let step_of = |e: usize| (0..m).map(|f| if f == e { 1.0 } else { 0.0 }).collect::<Vec<f64>>();
    let carrier = RelationCarrier { name: "TRIAL: the step, the movement across the spoke taken".into(), unit: "step".into(),
        a: steps.iter().map(|&e| step_of(e)).collect(), da: da.iter().map(|t| t[0].clone()).collect(), range: ObservedRange::synthetic() };
    let field = RelationalField::from_relations(&rel, series, Some(&carrier)).expect("text's field forms");
    TextField { chars, rel, steps, field }
}

const TEXTS: [&str; 2] = [" a car cras", "he swung the bat. he swung the ball. the cat sat. he swung the ba"];

/// 1. A at every point is the radial graphs as they stand: each spoke's
/// movement through that step; the evolution keeps every registration.
#[test]
fn one_the_radial_graphs_as_they_stand() {
    for text in TEXTS {
        let t = text_field(text);
        assert_eq!(t.field.spatial.len(), t.steps.len(), "one registration per point of evolution, every one kept");
        for p in 0..t.steps.len() {
            for e in 0..t.rel.n_edges() {
                let moved = t.steps[..=p].iter().filter(|&&s| s == e).count() as f64;
                assert_eq!(t.field.spatial[p].a[0][e], moved, "{text:?}, point {p}, spoke {e}");
            }
        }
    }
}

/// 2. The change is the step taken next, and nothing else; its readback is
/// the movement out of one element and into the next.
#[test]
fn two_the_change_is_the_step() {
    for text in TEXTS {
        let t = text_field(text);
        for p in 0..t.steps.len() - 1 {
            let next = t.steps[p + 1];
            let (from, to) = t.rel.edges[next];
            for e in 0..t.rel.n_edges() { assert_eq!(t.field.persistence[p].a[0][e], if e == next { 1.0 } else { 0.0 }, "{text:?}, point {p}"); }
            for v in 0..t.chars.len() {
                assert_eq!(t.field.persistence[p].leaving[0][v], if v == from { 1.0 } else { 0.0 });
                assert_eq!(t.field.persistence[p].entering[0][v], if v == to { 1.0 } else { 0.0 });
            }
        }
    }
}

/// 3. The readback of the field as it stands: the movement out of and into
/// each element so far.
#[test]
fn three_the_readback_of_the_field_as_it_stands() {
    for text in TEXTS {
        let t = text_field(text);
        for p in 0..t.steps.len() {
            for v in 0..t.chars.len() {
                let out = t.steps[..=p].iter().filter(|&&e| t.rel.edges[e].0 == v).count() as f64;
                let into = t.steps[..=p].iter().filter(|&&e| t.rel.edges[e].1 == v).count() as f64;
                assert_eq!((t.field.spatial[p].leaving[0][v], t.field.spatial[p].entering[0][v]), (out, into), "{text:?}, point {p}, {:?}", t.chars[v]);
            }
        }
    }
}

/// 4. B and Ξ of the field as it stands, by their definitions.
#[test]
fn four_b_and_xi_of_the_field_as_it_stands() {
    for text in TEXTS {
        let t = text_field(text);
        let m = t.rel.n_edges();
        let edges = &t.rel.edges;
        let succ = |e: usize| -> Vec<usize> { (0..m).filter(|&g| edges[g].0 == edges[e].1).collect() };
        let pred = |e: usize| -> Vec<usize> { (0..m).filter(|&g| edges[g].1 == edges[e].0).collect() };
        for p in 0..t.steps.len() {
            let a = &t.field.spatial[p].a[0];
            let b = |e: usize| -> Expansion { let mut v = Expansion::of(a[e]); for g in succ(e) { v = v.plus(&Expansion::of(a[g])); } v };
            for e in 0..m {
                assert_eq!(t.field.spatial[p].b[0][e], b(e).nearest(), "{text:?}, point {p}, spoke {e}: B");
                let mut xi = Expansion::new();
                for g in succ(e) { xi = xi.plus(&b(g)); }
                for q in pred(e) { xi = xi.minus(&b(q)); }
                assert_eq!(t.field.spatial[p].passage[0][e], xi.nearest(), "{text:?}, point {p}, spoke {e}: Ξ");
            }
        }
    }
}

/// 5. Energy moves at every step toward the element entered.
#[test]
fn five_energy_moves_toward_the_element_entered() {
    for text in TEXTS {
        let t = text_field(text);
        let en = t.field.ground.energy.as_ref().expect("energy is read");
        for (p, &e) in t.steps.iter().enumerate() {
            assert_eq!((en.signed[p][e].value, en.signed[p][e].side), (1.0, Side::Entering), "{text:?}, point {p}");
            assert!(en.signed[p].iter().enumerate().all(|(f, x)| f == e || x.side == Side::NotResolved));
        }
        assert!(t.field.ground.freedom.is_ok() && t.field.ground.connection.is_ok(), "freedom and connection read");
    }
}

/// 6. The readback of the input in order (Origin, 2026-11-27: *"Are we
/// missing a read back of the inputted text in order?"*): read registration
/// by registration, the readback's halves of each step (out of one element,
/// into the next) spell the input back exactly.
#[test]
fn six_the_readback_in_order_spells_the_input() {
    for text in TEXTS {
        let t = text_field(text);
        let one = |v: &[f64]| -> usize { let at: Vec<usize> = (0..v.len()).filter(|&i| v[i] != 0.0).collect(); assert_eq!(at.len(), 1, "one element each way"); at[0] };
        // The first step is the first registration itself; every later one
        // is the change between registrations.
        let s0 = &t.field.spatial[0];
        let mut read: String = [t.chars[one(&s0.leaving[0])], t.chars[one(&s0.entering[0])]].iter().collect();
        for p in &t.field.persistence {
            assert_eq!(t.chars[one(&p.leaving[0])], read.chars().last().unwrap(), "each step leaves where the last entered");
            read.push(t.chars[one(&p.entering[0])]);
        }
        assert_eq!(read, text, "the readback in order is the input");
    }
}
