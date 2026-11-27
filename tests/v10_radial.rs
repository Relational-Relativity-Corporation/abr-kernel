//! The radial field: the compute rebuild (Origin, 2026-11-27: *"I don't
//! think it should take as much as we're doing"*; *"go ahead with the
//! compute rebuild"*).
//!
//! On the complete radial graphs (109th), every element a spoke to every
//! element, the definitions of B and Ξ reduce exactly to A and the readback:
//!   B[x→y] = A[x→y] + out(y)
//!   Ξ[x→y] = out(y) + T − in(x) − n·out(x)
//! (out and in the readback's halves at an element, T the steps so far, n
//! the elements declared). `RadialField` keeps the order of steps, the
//! movement each spoke has carried and each element's halves, updated by
//! one spoke and two elements per step, never a table per registration.
//!
//! Each test checks the radial field against the full `RelationalField` on
//! actual text, at every point of evolution, so the identities are tested
//! against the definitions as the kernel forms them:
//!   1. A, the readback's halves, B and Ξ of the field as it stands;
//!   2. the step at every point (the change between registrations);
//!   3. the indications (112th), equal to `order::indications`;
//!   4. a longer actual text read in full (the scale the rebuild is for).
//!
//! **Not covered by the radial field (named):** the ground readings
//! (energy beyond the step, freedom's pair levels, connection's grades),
//! which text does not read; they stay in `RelationalField`.

#[path = "support/synthetic_whole_field.rs"]
mod support;
#[path = "support/text_field.rs"]
mod text;

use abr_kernel::order::indications;
use abr_kernel::radial::RadialField;
use text::*;

const TEXTS: [&str; 3] = [
    "He swung the bat. He swung the ball. The cat sat. He swung the ba",
    "It was the best of times, it was the worst of times",
    "Call me Ishmael. Some years ago",
];

/// The radial field read from the same text, its elements declared in the
/// same order as the full field's.
fn radial_of(t: &TextField) -> RadialField {
    let n = t.chars.len();
    let mut r = RadialField::new(n);
    for &e in &t.steps { r.push(e / n, e % n).expect("a step leaves where the last entered"); }
    r
}

/// 1. At every point: A, the halves, B and Ξ equal the full field's.
#[test]
fn one_the_readings_equal_the_full_field() {
    for text in TEXTS {
        let t = text_field(text);
        let n = t.chars.len();
        let mut r = RadialField::new(n);
        for (p, &e) in t.steps.iter().enumerate() {
            r.push(e / n, e % n).unwrap();
            let s = &t.field.spatial[p];
            for x in 0..n {
                assert_eq!((r.leaving(x) as f64, r.entering(x) as f64), (s.leaving[0][x], s.entering[0][x]), "{text:?}, point {p}: the halves");
                for y in 0..n {
                    let f = x * n + y;
                    assert_eq!(r.a(x, y) as f64, s.a[0][f], "{text:?}, point {p}: A");
                    assert_eq!(r.b(x, y) as f64, s.b[0][f], "{text:?}, point {p}: B = A + out(y)");
                    assert_eq!(r.xi(x, y) as f64, s.passage[0][f], "{text:?}, point {p}: Ξ = out(y) + T − in(x) − n·out(x)");
                }
            }
        }
    }
}

/// 2. The step at every point is the full field's change between
/// registrations.
#[test]
fn two_the_step_is_the_change() {
    for text in TEXTS {
        let t = text_field(text);
        let r = radial_of(&t);
        let n = t.chars.len();
        for p in 0..t.steps.len() - 1 {
            let (x, y) = r.step(p + 1);
            let a = &t.field.persistence[p].a[0];
            assert!((0..n * n).all(|f| a[f] == if f == x * n + y { 1.0 } else { 0.0 }), "{text:?}, point {p}");
        }
    }
}

/// 3. The indications at every point equal those read from the full field.
#[test]
fn three_the_indications_equal_the_full_field() {
    for text in TEXTS {
        let t = text_field(text);
        let r = radial_of(&t);
        for p in 1..=t.steps.len() {
            assert_eq!(r.indications(p), indications(&t.field, &t.rel, p).unwrap(), "{text:?}, through step {p}");
        }
    }
}

/// 4. A longer actual text read in full: the opening paragraph of *A Tale
/// of Two Cities* (Dickens, 1859, public domain), every character a step.
#[test]
fn four_a_paragraph_read_in_full() {
    let text = "It was the best of times, it was the worst of times, it was the age of wisdom, it was the age of foolishness, it was the epoch of belief, it was the epoch of incredulity, it was the season of Light, it was the season of Darkness, it was the spring of hope, it was the winter of despair, we had everything before us, we had nothing before us, we were all going direct to Heaven, we were all going direct the other way";
    let t = text_order(text);
    let n = t.chars.len();
    let mut r = RadialField::new(n);
    for &e in &t.steps { r.push(e / n, e % n).unwrap(); }
    assert_eq!(r.steps(), t.steps.len());
    // The readback in order spells the text back exactly (113th).
    let mut read = vec![t.chars[r.step(0).0]];
    for p in 0..r.steps() { read.push(t.chars[r.step(p).1]); }
    assert_eq!(read_back(&read), text);
    // The total movement out of the elements is the steps taken.
    assert_eq!((0..n).map(|x| r.leaving(x)).sum::<i64>(), r.steps() as i64);
    let d = r.indications(r.steps());
    assert!(!d.is_empty(), "at the end, the field indicates");
}
