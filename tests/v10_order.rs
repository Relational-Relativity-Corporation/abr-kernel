//! The order is the relational field (Origin, 2026-11-26, 102nd): *"the order
//! of information read from left to right including all character spaces and
//! punctuations"*. The possibilities at the current point are the relations
//! the entire input registered after the route arriving there, at every
//! length of route the input carries, each with its occurrences. No weight
//! or decay by distance (74th).
//!
//! The fixture is a synthetic declared sequence written as text for
//! legibility: each distinct character a locus, each distinct ordered pair a
//! relation, each step of the text one registered relation. Mapping text to
//! relations is an application's declaration; here it only makes the hand
//! derivation readable.
//!
//! **Not read by these tests (named):** the choice among the possibilities
//! (open, 102nd (3)); energy, connection and freedom at the current point;
//! the whole relational field formed from the order (registrations at every
//! step). The order's reading is checked against a derivation by hand.
//!
//! [IMPL] SYNTHETIC FIXTURES: they establish that the reading computes as
//! stated, nothing empirical.

#[path = "support/synthetic_whole_field.rs"]
mod support;

use abr_kernel::operators::DeclaredRelations;
use abr_kernel::order::Order;

/// The text as a declared sequence: loci (characters) and relations
/// (ordered pairs) in order of first occurrence, and the order of steps.
struct Text { chars: Vec<char>, pairs: Vec<(char, char)>, rel: DeclaredRelations, steps: Vec<usize> }

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
    let rel = support::rel(chars.len(), edges);
    Text { chars, pairs, rel, steps }
}

impl Text {
    /// The character a relation enters.
    fn entered(&self, e: usize) -> char { self.pairs[e].1 }
}

const TEXT: &str = "he swung the bat. he swung the ball. the cat sat. he swung the ba";

/// 1. The possibilities at the current point, at every length of route,
/// worked by hand (102nd): after *a* alone, l once and t three times (bat,
/// cat, sat; ball); after every route from *ba* to *he swung the ba*, l once
/// and t once (ball, bat); after * he swung the ba* and longer, to *at. he
/// swung the ba*, l once (only *…bat. he swung the ball* has occurred); no
/// longer route occurs.
#[test]
fn the_possibilities_narrow_with_the_route_as_worked_by_hand() {
    let t = declare(TEXT);
    let order = Order::new(&t.rel, t.steps.clone()).expect("the text is a declared order");
    let levels = order.possibilities();
    assert_eq!(levels.len(), 19, "routes of length 0 to 18 occur earlier");
    let conts = |l: usize| -> Vec<(char, usize)> { levels[l].continuations.iter().map(|&(e, n)| (t.entered(e), n)).collect() };
    let sorted = |mut v: Vec<(char, usize)>| { v.sort(); v };
    assert_eq!(sorted(conts(0)), vec![('l', 1), ('t', 3)], "after the bare letter a");
    for l in 1..=14 { assert_eq!(sorted(conts(l)), vec![('l', 1), ('t', 1)], "route length {l}"); }
    for l in 15..=18 { assert_eq!(conts(l), vec![('l', 1)], "route length {l}"); }
    // Each level reports its route and every occurrence.
    for (l, lv) in levels.iter().enumerate() {
        assert_eq!(lv.route.len(), l);
        assert_eq!(lv.occurrences.len(), lv.continuations.iter().map(|x| x.1).sum::<usize>(), "every occurrence has one continuation");
    }
    let route_text: String = levels[5].route.iter().map(|&e| t.entered(e)).collect();
    assert_eq!(route_text, "he ba", "the route of length 5 enters h, e, space, b, a: the last five steps");
}

/// 2. Order matters: two texts with the same pair counts in a different
/// order give different possibilities, though their summed relations (each
/// relation's count) are equal. The summed field cannot tell them apart; the
/// order can.
#[test]
fn the_order_tells_apart_what_the_sum_cannot() {
    // Both register a → a four times, a → b once, b → a once, and end at a.
    // After the bare letter a, both offer a four times and b once; the route
    // a → a, arriving at the end, occurred earlier only in the second.
    // (Found by enumeration and checked independently; an earlier pair,
    // abcacba and acbabca, could not tell them apart and was replaced.)
    let x = declare("aaaaaba");
    let y = declare("aaaabaa");
    let count = |t: &Text, p: (char, char)| t.steps.iter().filter(|&&e| t.pairs[e] == p).count();
    for p in [('a', 'a'), ('a', 'b'), ('b', 'a')] {
        assert_eq!(count(&x, p), count(&y, p), "the same count of {p:?}");
    }
    let px = Order::new(&x.rel, x.steps.clone()).unwrap().possibilities();
    let py = Order::new(&y.rel, y.steps.clone()).unwrap().possibilities();
    let at = |t: &Text, lv: &abr_kernel::order::Level| -> Vec<(char, usize)> {
        let mut v: Vec<(char, usize)> = lv.continuations.iter().map(|&(e, n)| (t.entered(e), n)).collect(); v.sort(); v
    };
    assert_eq!(at(&x, &px[0]), at(&y, &py[0]), "after the bare letter, the same: the summed field");
    let longer_differ = (1..px.len().min(py.len())).any(|l| at(&x, &px[l]) != at(&y, &py[l])) || px.len() != py.len();
    assert!(longer_differ, "along the route arriving, the order tells them apart");
}

/// 3. One direction, nothing cut: an order whose step does not continue
/// from the locus the previous step entered is refused, never repaired.
#[test]
fn an_order_that_breaks_is_refused() {
    let t = declare(TEXT);
    let mut broken = t.steps.clone();
    let e = broken[3];
    broken[3] = (0..t.pairs.len()).find(|&f| t.pairs[f].0 != t.pairs[e].0).unwrap();
    assert!(Order::new(&t.rel, broken).is_err(), "a step that does not continue the order is refused");
    assert!(Order::new(&t.rel, Vec::new()).is_err(), "an empty order has no current point");
    assert!(t.chars.len() > 1);
}
