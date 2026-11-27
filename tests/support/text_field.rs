//! Text's relational field (Origin, 109th, 111th, 113th), for the tests.
//!
//! Every character a locus with its own radial graph: every letter, the
//! space, every punctuation mark, and the capital mark ⇧ (113th (3)): a
//! capital letter is registered as ⇧ and then its letter, a declared
//! projection of the text. Each radial graph has a spoke to every element,
//! itself included, declared with the basis "the order of reading". Each
//! registration is the radial graphs as they stand (111th); the step taken
//! is the change between registrations.
//!
//! [IMPL] Text as the metaphorical basis (108th (4)); a domain declaration
//! for the tests, not kernel code.
#![allow(dead_code)]

use abr_kernel::operators::{DeclaredRelations, EdgeProvenance};
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::relational_field::RelationalField;
use abr_kernel::topology::{FeedbackDeclaration, RelationIdentity};

/// The capital mark (113th).
pub const CAPITAL: char = '⇧';

/// The text as registered: each capital as the capital mark and its letter.
pub fn project(text: &str) -> Vec<char> {
    let mut out = Vec::new();
    for c in text.chars() {
        if c.is_uppercase() { out.push(CAPITAL); out.extend(c.to_lowercase()); } else { out.push(c); }
    }
    out
}

/// The projection read back: the capital mark and its letter as the capital.
pub fn read_back(registered: &[char]) -> String {
    let mut out = String::new();
    let mut capital = false;
    for &c in registered {
        if c == CAPITAL { capital = true; continue; }
        if capital { out.extend(c.to_uppercase()); capital = false; } else { out.push(c); }
    }
    out
}

pub struct TextField {
    /// The elements, in order of first registration.
    pub chars: Vec<char>,
    /// The text as registered (through the projection).
    pub registered: Vec<char>,
    pub rel: DeclaredRelations,
    /// The spoke taken at each step: spoke (s, t) is relation s·n + t.
    pub steps: Vec<usize>,
    pub field: RelationalField,
}

impl TextField {
    pub fn at(&self, c: char) -> usize { self.chars.iter().position(|&x| x == c).unwrap() }
    pub fn spoke(&self, x: char, y: char) -> usize { self.at(x) * self.chars.len() + self.at(y) }
    pub fn entered(&self, e: usize) -> char { self.chars[self.rel.edges[e].1] }
}

pub fn text_field(text: &str) -> TextField {
    let registered = project(text);
    let mut chars: Vec<char> = Vec::new();
    for &c in &registered { if !chars.contains(&c) { chars.push(c); } }
    let n = chars.len();
    let at = |c: char| chars.iter().position(|&x| x == c).unwrap();
    let edges: Vec<(usize, usize)> = (0..n).flat_map(|s| (0..n).map(move |t| (s, t))).collect();
    let m = edges.len();
    let ids = (0..m).map(|e| RelationIdentity { id: format!("spoke-{e}"), basis: "a spoke of a radial graph (109th)".into() }).collect();
    let fb = (0..m).map(|e| FeedbackDeclaration { edge: e, basis: "the order of reading (Origin, 109th)".into() }).collect();
    let rel = DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m]).with_identity(ids).with_feedback(fb);
    let steps: Vec<usize> = registered.windows(2).map(|w| at(w[0]) * n + at(w[1])).collect();
    let mut stand = vec![0.0; m];
    let a: Vec<Vec<Vec<f64>>> = steps.iter().map(|&e| { stand[e] += 1.0; vec![stand.clone()] }).collect();
    let series = EdgeSeries::from_edge_values_synthetic(a, vec![vec![vec![0.0; m]]; steps.len()]).unwrap();
    let field = RelationalField::from_relations(&rel, series, None).expect("text's field forms");
    TextField { chars, registered, rel, steps, field }
}
