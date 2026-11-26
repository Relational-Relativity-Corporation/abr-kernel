//! The degrees of freedom of the current point (Origin, 2026-11-26, 108th).
//!
//! Origin: *"It sounds as though what you are describing is degrees of
//! freedom within the system?"*; *"freedom, energy, and connection combined
//! together to create forwards relational evolution in the text field in a
//! coherent manner"*. Under the 106th (no information can be repeated) and
//! the 107th (a turn, never a cycle), every earlier point where the order
//! stood at the current locus is a distinct revolution of the helix, never
//! another occurrence of the same route. Each is a degree of freedom, kept
//! distinct, nothing merged:
//!   point      the earlier point in evolution;
//!   leaving    the relation that left it there (the readback's leaving
//!              half at the current locus, kept decomposed by point, K9);
//!   turn       its turn to now: the span of the order between it and the
//!              current point;
//!   alongside  the span its arrival runs alongside the current arrival,
//!              relation by relation back from each point;
//!   parts      where the two arrivals part (the relation there, the
//!              relation now), or the record boundary.
//! No count: the 102nd's counts per continuation are a framework reading
//! at most (104th); `Order::possibilities` is kept as a fenced check.
//!
//! The fixture is text, the metaphorical basis for calculation (108th (4)):
//! each distinct character a locus, each distinct ordered pair a relation,
//! each step one registered relation. [IMPL] SYNTHETIC, a domain example;
//! it establishes that the reading is formed as stated, nothing empirical.
//!
//! **Not read here (named gaps):** connection and freedom are read on each
//! degree only as the order gives them (alongside, parts); energy on each
//! degree is a gap until text's carrier is declared (108th (2)); the whole
//! relational field formed from the order (registrations at every step) is
//! not formed; the combination of the three (108th (3)) is open.

#[path = "support/synthetic_whole_field.rs"]
mod support;

use std::cell::Cell;

use abr_kernel::derivation::{of_part, reported_names};
use abr_kernel::operators::DeclaredRelations;
use abr_kernel::order::{Degree, Order};
use support::*;

/// The text as a declared sequence: loci (characters) and relations
/// (ordered pairs) in order of first occurrence, and the order of steps.
struct Text { pairs: Vec<(char, char)>, rel: DeclaredRelations, steps: Vec<usize> }

fn declare(text: &str) -> Text {
    let cs: Vec<char> = text.chars().collect();
    let mut chars: Vec<char> = Vec::new();
    for &c in &cs { if !chars.contains(&c) { chars.push(c); } }
    let at = |c: char| chars.iter().position(|&x| x == c).unwrap();
    let mut pairs: Vec<(char, char)> = Vec::new();
    let mut steps = Vec::new();
    for w in cs.windows(2) {
        let p = (w[0], w[1]);
        if !pairs.contains(&p) { pairs.push(p); }
        steps.push(pairs.iter().position(|&x| x == p).unwrap());
    }
    let edges = pairs.iter().map(|&(x, y)| (at(x), at(y))).collect();
    Text { rel: rel(chars.len(), edges), pairs, steps }
}

impl Text {
    fn order(&self) -> Order { Order::new(&self.rel, self.steps.clone()).unwrap() }
    fn pair(&self, e: usize) -> String { format!("{}→{}", self.pairs[e].0, self.pairs[e].1) }
    /// The text a span of the order runs through, from its first locus.
    fn text_of(&self, span: (usize, usize)) -> String {
        let mut s: String = if span.0 < span.1 { self.pairs[self.steps[span.0]].0.to_string() } else { String::new() };
        for &e in &self.steps[span.0..span.1] { s.push(self.pairs[e].1); }
        s
    }
}

const TEXT: &str = "he swung the bat. he swung the ball. the cat sat. he swung the ba";

/// 1. The worked example, by hand (108th): four earlier points stood at the
/// current locus *a*, each a distinct degree.
#[test]
fn one_the_worked_degrees_of_freedom() {
    let t = declare(TEXT);
    let d = t.order().degrees();
    let n = t.steps.len();
    let row = |x: &Degree| (x.point, t.pair(x.leaving), x.turn, t.text_of(x.alongside),
        x.parts.map(|(here, now)| (t.pair(here), t.pair(now))));
    let got: Vec<_> = d.iter().map(row).collect();
    let want = vec![
        (14, "a→t".to_string(), (14, n), "he swung the ba".to_string(), None),
        (32, "a→l".to_string(), (32, n), "at. he swung the ba".to_string(), Some(("b→a".to_string(), "s→a".to_string()))),
        (42, "a→t".to_string(), (42, n), String::new(), Some(("c→a".to_string(), "b→a".to_string()))),
        (46, "a→t".to_string(), (46, n), String::new(), Some(("s→a".to_string(), "b→a".to_string()))),
    ];
    assert_eq!(got, want);
    assert_eq!(n, 64, "the current point");
}

/// An enumerated walk over enumerated declared relations: from the first
/// relation, each step a seed-selected relation leaving where the last
/// entered, up to `len` steps.
fn enumerated_walk(r: &mut Seeds, rel: &DeclaredRelations, len: usize) -> Vec<usize> {
    let mut walk = vec![r.usize(0, rel.n_edges() - 1)];
    while walk.len() < len {
        let at = rel.edges[*walk.last().unwrap()].1;
        let out: Vec<usize> = (0..rel.n_edges()).filter(|&e| rel.edges[e].0 == at).collect();
        if out.is_empty() { break; }
        walk.push(out[r.usize(0, out.len() - 1)]);
    }
    walk
}

/// The locus at point j of a walk (point 0 is where step 0 leaves).
fn locus(rel: &DeclaredRelations, steps: &[usize], j: usize) -> usize {
    if j == 0 { rel.edges[steps[0]].0 } else { rel.edges[steps[j - 1]].1 }
}

/// 2. Every earlier point at the current locus is a degree, exactly once,
/// in order of evolution; nothing is merged, so two points that offer the
/// same relation stay two degrees.
#[test]
fn two_every_earlier_point_is_its_own_degree() {
    let merged = Cell::new(0usize);
    for_all("degrees: one per earlier point", 0x108_01, |r| {
        let rel = enumerated_rel(r);
        let len = r.usize(4, 40);
        let steps = enumerated_walk(r, &rel, len);
        let n = steps.len();
        let d = Order::new(&rel, steps.clone()).unwrap().degrees();
        let here = locus(&rel, &steps, n);
        let want: Vec<usize> = (0..n).filter(|&j| locus(&rel, &steps, j) == here).collect();
        assert_eq!(d.iter().map(|x| x.point).collect::<Vec<_>>(), want, "one degree per earlier point, in order");
        for x in &d {
            assert_eq!(x.leaving, steps[x.point], "what left the point there");
            assert_eq!(x.turn, (x.point, n), "its turn to now");
        }
        let mut leaving: Vec<usize> = d.iter().map(|x| x.leaving).collect();
        leaving.sort_unstable();
        if leaving.windows(2).any(|w| w[0] == w[1]) { merged.set(merged.get() + 1); }
    });
    assert!(merged.get() > 0, "the family reaches points offering the same relation, kept apart");
}

/// 3. Alongside is exactly the shared arrival: the span behind the earlier
/// point equals, relation by relation, the span behind the current point,
/// and where they part the two relations differ (or the earlier arrival
/// reaches the record boundary). **Cannot tell (named):** it holds for a
/// reading that merges points too; tests 1, 2 and 4 catch that.
#[test]
fn three_alongside_is_the_shared_arrival_and_parts_where_it_differs() {
    for_all("degrees: alongside and parts", 0x108_02, |r| {
        let rel = enumerated_rel(r);
        let len = r.usize(4, 40);
        let steps = enumerated_walk(r, &rel, len);
        let n = steps.len();
        for x in Order::new(&rel, steps.clone()).unwrap().degrees() {
            let (a, b) = x.alongside;
            assert_eq!(b, x.point, "the span ends at the earlier point");
            let k = b - a;
            assert_eq!(&steps[a..b], &steps[n - k..n], "the arrivals run alongside");
            match x.parts {
                None => assert_eq!(a, 0, "the earlier arrival reaches the record boundary"),
                Some((here, now)) => {
                    assert!(a > 0);
                    assert_eq!((here, now), (steps[a - 1], steps[n - k - 1]));
                    assert_ne!(here, now, "they part where the relations differ");
                }
            }
        }
    });
}

/// 4. The 102nd's possibilities are contained, as a fenced check: at every
/// route length L the possibilities report, the degrees whose arrivals run
/// alongside for at least L relations are exactly its occurrences, and
/// their leaving relations are its continuations.
#[test]
fn four_the_possibilities_are_a_reading_of_the_degrees() {
    let check = |rel: &DeclaredRelations, steps: Vec<usize>| {
        let o = Order::new(rel, steps).unwrap();
        let d = o.degrees();
        for level in o.possibilities() {
            let l = level.route.len();
            let at: Vec<&Degree> = d.iter().filter(|x| x.alongside.1 - x.alongside.0 >= l).collect();
            assert_eq!(at.iter().map(|x| x.point).collect::<Vec<_>>(), level.occurrences, "route length {l}");
            let mut counts: Vec<(usize, usize)> = Vec::new();
            for x in &at {
                match counts.iter_mut().find(|c| c.0 == x.leaving) { Some(c) => c.1 += 1, None => counts.push((x.leaving, 1)) }
            }
            counts.sort_unstable();
            assert_eq!(counts, level.continuations, "route length {l}");
        }
    };
    let t = declare(TEXT);
    check(&t.rel, t.steps.clone());
    for_all("degrees: the possibilities contained", 0x108_03, |r| {
        let rel = enumerated_rel(r);
        let len = r.usize(4, 40);
        let steps = enumerated_walk(r, &rel, len);
        check(&rel, steps);
    });
}

/// 5. The order, not the sum (102nd): *aaaaaba* and *aaaabaa* have the same
/// relations the same number of times, and different degrees. **Cannot
/// tell (named):** a merging reading tells them apart too.
#[test]
fn five_the_degrees_tell_apart_what_a_sum_cannot() {
    let (x, y) = (declare("aaaaaba"), declare("aaaabaa"));
    let (dx, dy) = (x.order().degrees(), y.order().degrees());
    let summary = |t: &Text, d: &[Degree]| d.iter().map(|g| (g.point, t.pair(g.leaving), t.text_of(g.alongside))).collect::<Vec<_>>();
    assert_ne!(summary(&x, &dx), summary(&y, &dy));
}

/// 6. Every part of a degree is registered with its derivation, and none is
/// a count (104th). Structural: it holds for a merging reading too.
#[test]
fn six_every_part_of_a_degree_is_declared_and_none_is_a_count() {
    let t = declare(TEXT);
    let names = reported_names(&format!("{:?}", t.order().degrees()));
    for part in ["Degree.point", "Degree.leaving", "Degree.turn", "Degree.alongside", "Degree.parts"] {
        assert!(names.iter().any(|n| n == part), "{part} is reported: {names:?}");
        assert!(of_part(part).is_some(), "{part} has a derivation");
    }
    assert!(!names.iter().any(|n| n.contains("count") || n.contains("continuations")), "no count among the degrees: {names:?}");
}
