//! The word scale (Origin, 2026-11-27, 114th): words and separators as
//! elements, the capital mark ⇧ its own element; the same text read with a
//! larger step, by the same operators (*"make sure you stick to the operator
//! relational math"*).
//!
//! 1. The elements of a sentence, by hand.
//! 2. Completeness (113th): the whole of *Pride and Prejudice* (Project
//!    Gutenberg's edition, `tests/data/`), read at word scale by the radial
//!    field, reads back exactly.
//! 3. The operators at word scale are the definitions: on a short actual
//!    text, the radial field's A, halves, B and Ξ equal `RelationalField`'s
//!    on the complete radial graphs over its words, at every point.
//! 4. No drift at word scale: continuing only by indicated continuations
//!    generates no word step the input does not hold.
//! 5. The two scales related by the readback (97th): a word step is a route
//!    at character scale; the sum of its character steps' changes has, in
//!    the readback's balance, only its ends (out of where the route began,
//!    into where it ended): the word's inside cancels.
//!
//! Actual text (111th (2)).

#[path = "support/synthetic_whole_field.rs"]
mod support;
#[path = "support/text_field.rs"]
mod text;

use std::collections::{BTreeSet, HashMap};

use abr_kernel::operators::{DeclaredRelations, EdgeProvenance};
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::radial::RadialField;
use abr_kernel::relational_field::RelationalField;
use abr_kernel::topology::{FeedbackDeclaration, RelationIdentity};
use text::*;

const BOOK: &str = include_str!("data/pride_and_prejudice.txt");

/// The word field read by the radial field: elements declared as observed.
fn word_field(tokens: &[String]) -> (RadialField, Vec<String>) {
    let mut r = RadialField::new(0);
    let mut elems: Vec<String> = Vec::new();
    let mut index: HashMap<String, usize> = HashMap::new();
    let mut id = |t: &String, r: &mut RadialField, elems: &mut Vec<String>| -> usize {
        if let Some(&i) = index.get(t) { return i; }
        let i = r.declare();
        index.insert(t.clone(), i);
        elems.push(t.clone());
        i
    };
    let mut ids = Vec::with_capacity(tokens.len());
    for t in tokens { ids.push(id(t, &mut r, &mut elems)); }
    for w in ids.windows(2) { r.push(w[0], w[1]).unwrap(); }
    (r, elems)
}

/// 1. The elements of a sentence of the book, by hand.
#[test]
fn one_the_elements_of_a_sentence() {
    let w = words("Mr. Bennet’s, my dear.\r\n\r\n“Is");
    assert_eq!(w, vec!["⇧", "mr", ". ", "⇧", "bennet’s", ", ", "my", " ", "dear", ".\r\n\r\n“", "⇧", "is"]);
    assert_eq!(words_back(&w), "Mr. Bennet’s, my dear.\r\n\r\n“Is");
}

/// 2. The whole book at word scale reads back exactly.
#[test]
fn two_the_book_at_word_scale_reads_back_exactly() {
    let tokens = words(BOOK);
    let (r, elems) = word_field(&tokens);
    assert_eq!(r.steps(), tokens.len() - 1);
    let mut back = vec![elems[r.step(0).0].clone()];
    for p in 0..r.steps() { back.push(elems[r.step(p).1].clone()); }
    assert_eq!(words_back(&back), BOOK, "the word field recovers the book exactly");
}

/// 3. On a short actual text, the operators at word scale are the
/// definitions, as `RelationalField` forms them, at every point.
#[test]
fn three_the_operators_at_word_scale_are_the_definitions() {
    let text = "It is a truth universally acknowledged, that a single man in possession of a good fortune, must be in want of a wife.";
    let tokens = words(text);
    let (r, elems) = word_field(&tokens);
    let n = elems.len();
    let edges: Vec<(usize, usize)> = (0..n).flat_map(|s| (0..n).map(move |t| (s, t))).collect();
    let m = edges.len();
    let ids = (0..m).map(|e| RelationIdentity { id: format!("word-spoke-{e}"), basis: "a spoke of a word's radial graph (114th)".into() }).collect();
    let fb = (0..m).map(|e| FeedbackDeclaration { edge: e, basis: "the order of reading (Origin, 109th)".into() }).collect();
    let rel = DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m]).with_identity(ids).with_feedback(fb);
    let mut stand = vec![0.0; m];
    let a: Vec<Vec<Vec<f64>>> = (0..r.steps()).map(|p| { let (x, y) = r.step(p); stand[x * n + y] += 1.0; vec![stand.clone()] }).collect();
    let series = EdgeSeries::from_edge_values_synthetic(a, vec![vec![vec![0.0; m]]; r.steps()]).unwrap();
    let f = RelationalField::from_relations(&rel, series, None).unwrap();
    let mut q = RadialField::new(n);
    for p in 0..r.steps() {
        let (x, y) = r.step(p);
        q.push(x, y).unwrap();
        let s = &f.spatial[p];
        for u in 0..n {
            assert_eq!((q.leaving(u) as f64, q.entering(u) as f64), (s.leaving[0][u], s.entering[0][u]), "point {p}: the halves");
            for v in 0..n {
                assert_eq!(q.a(u, v) as f64, s.a[0][u * n + v], "point {p}: A");
                assert_eq!(q.b(u, v) as f64, s.b[0][u * n + v], "point {p}: B");
                assert_eq!(q.xi(u, v) as f64, s.passage[0][u * n + v], "point {p}: Ξ");
            }
        }
    }
}

/// 4. No drift at word scale: continuing the first chapter's word field
/// only by indicated continuations (read by the radial field, equal to
/// `order::indications`, `v10_radial`) generates only observed word steps.
#[test]
fn four_no_drift_at_word_scale() {
    let start = BOOK.find("It is a truth universally acknowledged").unwrap();
    let end = BOOK[start..].find("CHAPTER II.").map(|e| start + e).unwrap();
    let tokens = words(&BOOK[start..end]);
    let (mut r, elems) = word_field(&tokens);
    let n = elems.len();
    let observed: BTreeSet<(usize, usize)> = (0..r.steps()).map(|p| r.step(p)).collect();
    for _ in 0..40 {
        let d = r.indications(r.steps());
        if d.is_empty() { break; }
        let c = d.iter().max_by_key(|x| x.alongside.1 - x.alongside.0).unwrap();
        let (x, y) = (c.leaving / n, c.leaving % n);
        assert!(observed.contains(&(x, y)), "an observed word step");
        r.push(x, y).unwrap();
    }
}

/// 5. A word step is a route at character scale: the readback's balance of
/// its character steps has only the route's ends; its halves keep the inside.
#[test]
fn five_the_scales_related_by_the_readback() {
    let text = "It is a truth universally acknowledged";
    let t = text_order(text);
    let n = t.chars.len();
    let mut r = RadialField::new(n);
    for &e in &t.steps { r.push(e / n, e % n).unwrap(); }
    // The word "truth" at character scale: from the space before it to the
    // space after it.
    // Found by character (⇧ is more than one byte).
    let want: Vec<char> = " truth ".chars().collect();
    let at = (0..t.registered.len()).find(|&i| t.registered[i..].starts_with(&want)).unwrap();
    let (from, to) = (at, at + " truth".chars().count());
    let mut balance = vec![0i64; n];
    let mut halves = vec![0i64; n];
    for p in from..to {
        let (x, y) = r.step(p);
        balance[x] += 1; balance[y] -= 1;
        halves[x] += 1;
    }
    let (start, end) = (t.at(' '), t.at(' '));
    for v in 0..n {
        let want = if v == start && v == end { 0 } else if v == start { 1 } else if v == end { -1 } else { 0 };
        assert_eq!(balance[v], want, "only the route's ends remain in the balance");
    }
    for c in ['t', 'r', 'u', 'h'] { assert!(halves[t.at(c)] > 0, "the halves keep the word's inside: {c}"); }
}
