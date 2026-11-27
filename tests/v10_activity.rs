//! The activity at a point, and coherence (Proposed, under trial, Origin,
//! 2026-11-27, 116th: *"Potentiation could be described as the sensitivity
//! to connected neurons which are connected by virtue of stimulation in
//! other words they are more likely to fire, but of course we should avoid
//! any sort of probability and instead use coherence which is what our
//! brains use"*).
//!
//! The brain is a metaphor, labelled (108th (4)): a neuron is a character's
//! locus, a synapse a spoke, a firing a step. *Amended (Origin, 2026-11-27):*
//! *"it is synaptic activity which describes neural Nets ... It is information
//! across the synapse which drives neural firing"*. The activity at a point is
//! the synapses fired at the steps behind it, within a declared reach (a scope
//! of analysis, 85th). At each degree of freedom of the current point (108th),
//! the active synapse at distance k agrees where the relation fired k steps
//! behind that earlier point is the same relation. *Was:* the elements (the
//! neurons) compared, kept fenced as `element_activity` (test 8). The coherent degrees are those whose
//! agreement no other degree's agreement strictly contains.
//!
//! What is shown: (1) every degree is kept, none merged, each continuing as
//! registered; (2) the agreement is exactly the elements at matching points,
//! recomputed here from the text itself, never from the field's code;
//! (3) the 108th's arrival alongside is the unbroken start of the agreement;
//! (4) the coherent degrees are exactly those no other contains, recomputed
//! by comparing every pair; (5) a degree whose arrival runs alongside
//! furthest is always coherent; (6) the reach is a declared scope, and the
//! field refuses what it cannot read; (7) on actual text, coherent degrees
//! keep agreement past where their arrivals part (relation across a
//! difference).
//!
//! [IMPL] Actual text (111th): *Pride and Prejudice* (`tests/data/`), its
//! first chapter, at every 37th point (a fixed sample, nothing chosen).
//! **What these establish:** that the activity computes as stated on actual
//! text; not that coherence chooses well (measured in
//! `examples/v10_book_activity.rs`). **Not evaluated here (named):** energy
//! beyond the step (text's carrier is undeclared); the choice among coherent
//! degrees inclusion cannot order (open).

#[path = "support/text_field.rs"]
mod text;

use std::collections::{BTreeSet, HashMap};

use abr_kernel::radial::{ActivityRefusal, RadialField};
use text::project;

const BOOK: &str = include_str!("data/pride_and_prejudice.txt");

/// The first chapter, projected (113th: the capital mark), its elements
/// declared as observed, and its radial field.
fn chapter() -> (Vec<usize>, RadialField) {
    let start = BOOK.find("It is a truth universally acknowledged").unwrap();
    let end = BOOK[start..].find("CHAPTER II.").map(|e| start + e).unwrap();
    let reg = project(&BOOK[start..end]);
    let mut index: HashMap<char, usize> = HashMap::new();
    let mut r = RadialField::new(0);
    let elems: Vec<usize> = reg.iter().map(|c| *index.entry(*c).or_insert_with(|| r.declare())).collect();
    for w in elems.windows(2) { r.push(w[0], w[1]).unwrap(); }
    (elems, r)
}

fn points(r: &RadialField) -> impl Iterator<Item = usize> { (1..r.steps()).step_by(37) }

const REACHES: [usize; 3] = [4, 12, 40];

/// 1. Every degree of the current point is kept, in order of evolution, none
/// merged, each continuing by the relation registered at its point.
#[test]
fn every_degree_is_kept_and_none_merged() {
    let (_, r) = chapter();
    for p in points(&r) {
        let d = r.indications(p);
        for reach in REACHES {
            let a = r.activity(p, reach).unwrap();
            assert_eq!(a.point, p);
            assert_eq!(a.agreements.len(), d.len(), "point {p}: one agreement per degree");
            for (g, x) in a.agreements.iter().zip(&d) {
                assert_eq!((g.point, g.leaving), (x.point, x.leaving), "point {p}: the degree's point and its continuation");
            }
        }
    }
}

/// 2. The agreement is the synapses at matching points: at distance k
/// (1 ≤ k ≤ reach) the earlier point j and the current point p agree where
/// the relation fired k steps behind j (from the element at j − k to the
/// element at j − k + 1) is the relation fired k steps behind p; beyond,
/// where either reaches the record boundary; every distance in exactly one
/// of the three.
#[test]
fn the_agreement_is_read_from_the_text_at_matching_points() {
    let (elems, r) = chapter();
    for p in points(&r) {
        for reach in REACHES {
            let a = r.activity(p, reach).unwrap();
            assert_eq!(a.reach, reach);
            for g in &a.agreements {
                let j = g.point;
                assert_eq!(elems[j], elems[p], "a degree stands at the current element");
                let (mut agrees, mut differs, mut beyond) = (vec![], vec![], vec![]);
                for k in 1..=reach {
                    if k > j || k > p { beyond.push(k) } else if (elems[j - k], elems[j - k + 1]) == (elems[p - k], elems[p - k + 1]) { agrees.push(k) } else { differs.push(k) }
                }
                assert_eq!((&g.agrees, &g.differs, &g.beyond), (&agrees, &differs, &beyond), "point {p}, degree {j}, reach {reach}");
            }
        }
    }
}

/// 3. The 108th's arrival alongside is the unbroken start of the agreement:
/// every distance up to how far it runs alongside agrees, and where it parts
/// (within the reach, before the boundary) the next distance differs.
#[test]
fn the_arrival_alongside_is_the_unbroken_start_of_the_agreement() {
    let (_, r) = chapter();
    for p in points(&r) {
        let d = r.indications(p);
        let a = r.activity(p, 40).unwrap();
        for (g, x) in a.agreements.iter().zip(&d) {
            let run = x.alongside.1 - x.alongside.0;
            for k in 1..=run.min(40) { assert!(g.agrees.contains(&k), "point {p}, degree {}: alongside {run}, distance {k}", g.point); }
            if run < 40 && x.parts.is_some() { assert!(g.differs.contains(&(run + 1)), "point {p}, degree {}: parts at {}", g.point, run + 1); }
        }
    }
}

/// 4. The coherent degrees are exactly those whose agreement no other
/// degree's agreement strictly contains (inclusion, never a count).
#[test]
fn the_coherent_are_exactly_those_no_other_contains() {
    let (_, r) = chapter();
    for p in points(&r).step_by(5) {
        for reach in REACHES {
            let a = r.activity(p, reach).unwrap();
            let sets: Vec<BTreeSet<usize>> = a.agreements.iter().map(|g| g.agrees.iter().copied().collect()).collect();
            let expect: Vec<usize> = (0..sets.len())
                .filter(|&i| !sets.iter().any(|s| s.len() > sets[i].len() && sets[i].is_subset(s)))
                .collect();
            assert_eq!(a.coherent, expect, "point {p}, reach {reach}");
        }
    }
}

/// 5. A degree whose arrival runs alongside furthest is always coherent: the
/// activity never loses what the 108th's comparison holds.
#[test]
fn a_furthest_alongside_degree_is_always_coherent() {
    let (_, r) = chapter();
    for p in points(&r) {
        let d = r.indications(p);
        if d.is_empty() { continue; }
        for reach in REACHES {
            let a = r.activity(p, reach).unwrap();
            let run = |i: usize| (d[i].alongside.1 - d[i].alongside.0).min(reach);
            let best = (0..d.len()).map(run).max().unwrap();
            assert!(a.coherent.iter().any(|&i| run(i) == best), "point {p}, reach {reach}");
        }
    }
}

/// 6. The reach is a declared scope: at reach 0 nothing is active and every
/// degree is coherent; a point not in the field, or a reach the field cannot
/// hold, is refused, never repaired.
#[test]
fn the_reach_is_a_declared_scope() {
    let (_, r) = chapter();
    let a = r.activity(500, 0).unwrap();
    assert_eq!(a.coherent, (0..a.agreements.len()).collect::<Vec<_>>());
    assert!(a.agreements.iter().all(|g| g.agrees.is_empty() && g.differs.is_empty() && g.beyond.is_empty()));
    assert_eq!(r.activity(0, 4), Err(ActivityRefusal::Through { through: 0, steps: r.steps() }));
    assert_eq!(r.activity(r.steps() + 1, 4), Err(ActivityRefusal::Through { through: r.steps() + 1, steps: r.steps() }));
    assert_eq!(r.activity(500, 65), Err(ActivityRefusal::Reach { reach: 65, most: 64 }));
}

/// 7. On actual text, coherent degrees keep agreement past where their
/// arrivals part: an agreeing distance beyond a differing one (relation
/// across a difference, which the 108th's comparison stops before).
#[test]
fn coherence_keeps_agreement_past_where_arrivals_part() {
    let (_, r) = chapter();
    let mut found = 0;
    for p in points(&r) {
        let a = r.activity(p, 12).unwrap();
        for &i in &a.coherent {
            let g = &a.agreements[i];
            if let Some(&first) = g.differs.first() { if g.agrees.iter().any(|&k| k > first) { found += 1; } }
        }
    }
    assert!(found > 0, "no coherent degree agrees past where its arrival parts");
    eprintln!("[activity] coherent degrees agreeing past where their arrivals part: {found}");
}

/// 8. *Fenced (Origin, 2026-11-27):* the neuron form, the elements compared
/// at matching points, kept as a check of the first reading of the 116th;
/// never a reading.
#[test]
fn the_fenced_neuron_form_compares_elements() {
    let (elems, r) = chapter();
    for p in points(&r).step_by(3) {
        let a = r.element_activity(p, 12).unwrap();
        for g in &a.agreements {
            let j = g.point;
            let agrees: Vec<usize> = (1..=12).filter(|&k| k <= j && elems[j - k] == elems[p - k]).collect();
            assert_eq!(g.agrees, agrees, "point {p}, degree {j}");
        }
    }
}
