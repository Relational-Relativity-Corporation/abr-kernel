//! A complete relational field (Origin, 2026-11-27, 113th): *"a relational
//! field that is complete … to establish a connection between the observer
//! and the observed by calculating enough observable information to maintain
//! that connection"*. Capitals by the capital mark ⇧ (113th (3)).
//!
//! Every text field is held to completeness, on actual text with capitals:
//!   1. the projection: a capital is ⇧ and its letter; the letters keep one
//!      radial graph each, and ⇧ has its own;
//!   2. **backwards**: the readback of the field in order, read back through
//!      the projection, recovers the observed exactly, capitals included;
//!   3. **forwards**: the field indicates the continuation from what it
//!      holds: after a sentence ends, it indicates the capital mark.
//!
//! The texts: the worked text written as a sentence would be; the opening
//! clauses of *A Tale of Two Cities* (Dickens, 1859) and *Moby-Dick*
//! (Melville, 1851), both in the public domain, kept short so the suite stays
//! quick.
//!
//! **Not read here (named):** the choice among the indications (110th);
//! energy beyond the trial carrier.

#[path = "support/synthetic_whole_field.rs"]
mod support;
#[path = "support/text_field.rs"]
mod text;

use abr_kernel::order::indications;
use text::*;

const TEXTS: [&str; 3] = [
    "He swung the bat. He swung the ball. The cat sat. He swung the ba",
    "It was the best of times, it was the worst of times",
    "Call me Ishmael. Some years ago",
];

/// 1. A capital is the capital mark and its letter: *He* and *he* share the
/// letter's radial graph; ⇧ has its own.
#[test]
fn one_a_capital_is_the_mark_and_its_letter() {
    assert_eq!(project("He swung"), vec!['⇧', 'h', 'e', ' ', 's', 'w', 'u', 'n', 'g']);
    let t = text_field(TEXTS[0]);
    assert!(t.chars.contains(&CAPITAL) && !t.chars.iter().any(|c| c.is_uppercase()), "one element for capitals, no capital letters as elements");
    let h = t.at('h');
    assert_eq!(t.field.relations.a.last().unwrap()[0][t.spoke(CAPITAL, 'h')], 3.0, "⇧→h: He, He, He");
    assert_eq!(t.field.relations.a.last().unwrap()[0][t.spoke('t', 'h')], 4.0, "t→h: the, the, The, the: one radial graph for the letter");
    assert!(t.rel.edges.iter().filter(|e| e.0 == h).count() == t.chars.len(), "h has its radial graph, a spoke to every element");
}

/// 2. Backwards: the observed is recovered from the field exactly.
#[test]
fn two_the_observed_is_recovered_from_the_field() {
    for text in TEXTS {
        let t = text_field(text);
        let one = |v: &[f64]| -> usize { let at: Vec<usize> = (0..v.len()).filter(|&i| v[i] != 0.0).collect(); assert_eq!(at.len(), 1); at[0] };
        let s0 = &t.field.spatial[0];
        let mut read = vec![t.chars[one(&s0.leaving[0])], t.chars[one(&s0.entering[0])]];
        for p in &t.field.persistence { read.push(t.chars[one(&p.entering[0])]); }
        assert_eq!(read, t.registered, "the readback in order is the text as registered");
        assert_eq!(read_back(&read), text, "and through the projection, the text as observed, capitals included");
    }
}

/// 3. Forwards: after the second sentence ends (*…ball. *), the field
/// indicates the capital mark: the earlier point after *bat. * runs
/// alongside for *. * and offers ␣→⇧; the text then takes it.
#[test]
fn three_after_a_sentence_ends_the_field_indicates_the_capital() {
    let t = text_field(TEXTS[0]);
    // The point where the text stands at the space after "ball.", found by
    // character (⇧ is more than one byte).
    let want: Vec<char> = "ball. ".chars().collect();
    let p = (0..t.registered.len()).find(|&i| t.registered[i..].starts_with(&want)).unwrap() + want.len() - 1;
    let ind = indications(&t.field, &t.rel, p).unwrap();
    let furthest = ind.iter().max_by_key(|d| d.alongside.1 - d.alongside.0).unwrap();
    assert_eq!(t.entered(furthest.leaving), CAPITAL, "the arrival that runs alongside furthest offers the capital mark");
    assert_eq!(t.entered(t.steps[p]), CAPITAL, "and the text takes it: The cat sat");
}
