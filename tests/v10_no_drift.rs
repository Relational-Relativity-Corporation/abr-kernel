//! The process eliminates drift by its design (Origin, 2026-11-27: *"what
//! we need to establish is that our process eliminates drift by mathematical
//! design"*).
//!
//! The property: the field indicates only continuations that were observed
//! (each indication is an earlier point of the order, its continuation the
//! relation registered there). A process that continues only by an indicated
//! continuation therefore generates no relation the input does not hold;
//! by induction this holds when a step is indicated from an earlier generated
//! point, since that point was itself indicated from an observed one. Where
//! nothing is indicated, nothing is continued: the process stops (and asks,
//! 113th). No probability is computed at any step (104th; `v10_no_scalar`).
//! The property holds whatever the choice among the indications: tested here
//! under three different choices.
//!
//! The indications are read through `Order::degrees`, equal exactly to
//! `order::indications` on the field (`v10_indications`, 1), so the field is
//! not re-formed at every generated step.
//!
//! **What it does not claim (named):** new strings can be formed from
//! observed relations (the 106th requires that evolution never repeats);
//! novelty composed of observed relations is relational evolution, not
//! drift. Sense (bat, not ball) is not tested here.

#[path = "support/synthetic_whole_field.rs"]
mod support;
#[path = "support/text_field.rs"]
mod text;

use std::collections::BTreeSet;

use abr_kernel::order::{Degree, Order};
use text::*;

const TEXTS: [&str; 3] = [
    "He swung the bat. He swung the ball. The cat sat. He swung the ba",
    "It was the best of times, it was the worst of times, it was the age of wisdom, it was the age of foolishness",
    "Call me Ishmael. Some years ago, never mind how long precisely, having little or no money in my purse",
];

/// Three choices among the indications: the first, the last, and the one
/// whose arrival runs alongside furthest.
fn choose(rule: usize, d: &[Degree]) -> &Degree {
    match rule {
        0 => &d[0],
        1 => d.last().unwrap(),
        _ => d.iter().max_by_key(|x| x.alongside.1 - x.alongside.0).unwrap(),
    }
}

/// The drift in a continued order: every generated step whose relation was
/// never observed, or whose trace through the points that indicated it does
/// not end at an observed point registering the same relation.
fn drift(steps: &[usize], n0: usize, observed: &BTreeSet<usize>, from: &[Option<usize>]) -> Vec<String> {
    let mut why = Vec::new();
    for (q, &e) in steps.iter().enumerate().skip(n0) {
        if !observed.contains(&e) { why.push(format!("step {q}: relation {e} was never observed")); continue; }
        let mut p = q;
        let mut hops = 0;
        while p >= n0 {
            match from[p] { Some(j) => p = j, None => { why.push(format!("step {q}: indicated from nowhere")); break; } }
            hops += 1;
            if hops > steps.len() { why.push(format!("step {q}: the trace does not end")); break; }
        }
        if p < n0 && steps[p] != e { why.push(format!("step {q}: traced to point {p}, which registered another relation")); }
    }
    why
}

/// Every generated relation is observed; every generated step traces back,
/// through the points that indicated it, to an observed point; where nothing
/// is indicated, the process stops.
#[test]
fn the_process_generates_only_what_was_observed() {
    for text in TEXTS {
        let t = text_order(text);
        let observed: BTreeSet<usize> = t.steps.iter().copied().collect();
        let n0 = t.steps.len();
        for rule in 0..3 {
            let mut steps = t.steps.clone();
            // The point each generated step was indicated from.
            let mut from: Vec<Option<usize>> = vec![None; n0];
            for _ in 0..60 {
                let d = Order::new(&t.rel, steps.clone()).unwrap().degrees();
                if d.is_empty() { break; } // nothing indicated: stop, never invent
                let c = choose(rule, &d);
                assert_eq!(c.leaving, steps[c.point], "an indication's continuation is the relation registered at its point");
                steps.push(c.leaving);
                from.push(Some(c.point));
            }
            let why = drift(&steps, n0, &observed, &from);
            assert!(why.is_empty(), "{text:?}, choice {rule}: {why:?}");
        }
    }
}

/// Where the current element has never been met before, nothing is
/// indicated: the process has nothing to continue with.
#[test]
fn nothing_observed_nothing_indicated() {
    let t = text_order("Call me Ishmael");
    // At the start of the order, the first element has no earlier point.
    let d = Order::new(&t.rel, t.steps[..1].to_vec()).unwrap().degrees();
    assert!(d.iter().all(|x| x.point < 1));
    let first_l = t.registered.iter().position(|&c| c == 'l').unwrap();
    let d = Order::new(&t.rel, t.steps[..first_l].to_vec()).unwrap().degrees();
    assert!(d.is_empty(), "the first time the text stands at l, nothing is indicated");
}

/// The check can fail: a process allowed to invent (here: from the current
/// element, a spoke to an element it was never observed to enter from it)
/// is caught at its first invented step.
#[test]
fn an_inventing_process_is_caught() {
    let t = text_order(TEXTS[1]);
    let observed: BTreeSet<usize> = t.steps.iter().copied().collect();
    let n = t.chars.len();
    let here = t.rel.edges[*t.steps.last().unwrap()].1;
    let invented = (0..n).map(|y| here * n + y).find(|e| !observed.contains(e)).expect("some spoke was never taken");
    let d = Order::new(&t.rel, t.steps.clone()).unwrap().degrees();
    assert!(d.iter().all(|x| x.leaving != invented), "no indication offers it");
    let mut steps = t.steps.clone();
    steps.push(invented);
    let mut from = vec![None; t.steps.len()];
    from.push(None);
    let why = drift(&steps, t.steps.len(), &observed, &from);
    assert_eq!(why.len(), 1, "the drift is caught at the invented step: {why:?}");
}
