// work.rs — Metatron Dynamics, Inc. Kernel V10.
// The work of forming the operator readings (Origin, 2026-11-25, 92nd).
// Bounded over D. No claim beyond D.
//
// A, B and Ξ are local: each relation reads itself and its declared
// successors and predecessors. The work of forming them must therefore grow
// with the relations and their neighbours, never with their square. Comparing
// every relation with every other is the shape of attention, the sign that a
// reading is not using the primitive (92nd).
//
// This module counts, exactly, the terms visited while forming the operator
// readings of a relational field: A, B and Ξ at every registration and over
// every step, and the sustained grade's signs of Ξ(B(A)). A term is one
// coefficient or entry visited, whether or not it is zero, so the count is
// the work done, not the work needed.
//
// [IMPL] The count is instrumentation, not a reading: it changes no value.
// It covers only the loops that call `terms`; work in any other loop is not
// counted, and a test of it cannot see that work (named in the test).
// Readings declared as comparisons of every pair (the identity and
// proportional grades, the V2 held arrangement) are not operator readings
// and are not counted here; their scope is stated in `derivation.rs`.

use std::cell::Cell;

thread_local! {
    static TERMS: Cell<u64> = const { Cell::new(0) };
}

/// Count `n` terms visited in forming an operator reading.
#[inline]
pub(crate) fn terms(n: usize) {
    TERMS.with(|c| c.set(c.get() + n as u64));
}

/// Run `f` and return its result with the operator terms it visited on this
/// thread.
pub fn operator_terms<T>(f: impl FnOnce() -> T) -> (T, u64) {
    let before = TERMS.with(|c| c.get());
    let r = f();
    (r, TERMS.with(|c| c.get()) - before)
}
