//! Exact sums of registered values (Origin, 2026-11-23, 41st: "we are
//! decomposing complex systems but we're not simplifying them").
//!
//! A V10 reading decomposes a field into parts: closures, accumulations,
//! circulations. Each is a sum of registered entries with integer
//! coefficients. Adding them one at a time in floating point can lose a small
//! entry beside a large one, and the result then depends on the order of
//! addition. Example: 2⁵⁴ + (−0.5) − 2⁵⁴ gives 0, while the exact sum is
//! −0.5. Traversal is an order of analysis (Df2a), so its order must not
//! change a reading.
//!
//! Here every sum is held exactly, as a list of non-overlapping partials
//! (Shewchuk's expansions, the method behind Python's `math.fsum`). Products
//! of two values are held exactly too, through a fused multiply-add. A sum
//! then leaves this module in one of two ways:
//!
//! - `nearest`: rounded once, to the nearest f64 (ties to even), for a value.
//! - `up`: the smallest f64 at or above the exact sum, for a bound.
//!
//! **What this guarantees for a reading at δ.** Let a value v and a bound h
//! be exact. The kernel reads fl(v) = `nearest` and H = `up`, with H ≥ h.
//! Rounding is monotone, so if |v| ≤ h ≤ H then |fl(v)| ≤ H. Hence a
//! resolution (|fl(v)| > H) is read only where |v| > h exactly. The one
//! remaining effect of rounding is at the boundary: a value within one
//! rounding unit above its bound can read as not resolved. Never the other
//! way, so no distinction is claimed that the registrations do not make.
//!
//! Assumes finite inputs whose sums and products neither overflow nor
//! underflow f64 (every V10 entry is checked finite at admission).

/// An exact sum of f64 terms.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Expansion {
    /// Non-overlapping partials, increasing in magnitude. Their sum is exact.
    partials: Vec<f64>,
}

impl Expansion {
    pub fn new() -> Self { Expansion { partials: Vec::new() } }

    /// Add x exactly.
    pub fn add(&mut self, x: f64) {
        let mut x = x;
        let mut i = 0;
        for j in 0..self.partials.len() {
            let mut y = self.partials[j];
            if x.abs() < y.abs() { std::mem::swap(&mut x, &mut y); }
            let hi = x + y;
            let lo = y - (hi - x);
            if lo != 0.0 { self.partials[i] = lo; i += 1; }
            x = hi;
        }
        self.partials.truncate(i);
        self.partials.push(x);
    }

    /// Add the product a·b exactly: p = fl(a·b) and the error fma(a, b, −p).
    pub fn add_product(&mut self, a: f64, b: f64) {
        let p = a * b;
        let e = a.mul_add(b, -p);
        self.add(p);
        if e != 0.0 { self.add(e); }
    }

    /// Add every partial of another expansion, times an integer coefficient
    /// (exact: an integer of magnitude below 2⁵³ is an exact f64).
    pub fn add_scaled(&mut self, other: &Expansion, k: i64) {
        for &p in &other.partials { self.add_product(k as f64, p); }
    }

    /// Add another expansion times f64 factor g, exactly.
    pub fn add_times(&mut self, other: &Expansion, g: f64) {
        for &p in &other.partials { self.add_product(p, g); }
    }

    /// Add another expansion, exactly.
    pub fn add_expansion(&mut self, other: &Expansion) {
        for &p in &other.partials { self.add(p); }
    }

    /// The sign of the exact sum: −1, 0 or +1.
    pub fn signum(&self) -> i8 {
        match self.partials.iter().rev().find(|p| **p != 0.0) {
            None => 0,
            Some(p) if *p > 0.0 => 1,
            Some(_) => -1,
        }
    }

    /// The exact sum's magnitude, as an expansion.
    pub fn abs(&self) -> Expansion {
        if self.signum() < 0 { Expansion { partials: self.partials.iter().map(|p| -p).collect() } } else { self.clone() }
    }

    /// The exact sum rounded once to the nearest f64, ties to even.
    pub fn nearest(&self) -> f64 {
        let p = &self.partials;
        let n = p.len();
        if n == 0 { return 0.0; }
        let mut i = n - 1;
        let mut hi = p[i];
        let mut lo = 0.0;
        while i > 0 {
            let x = hi;
            i -= 1;
            let y = p[i];
            hi = x + y;
            let yr = hi - x;
            lo = y - yr;
            if lo != 0.0 { break; }
        }
        // A half-way case: the rest of the partials decides the direction.
        if i > 0 && ((lo < 0.0 && p[i - 1] < 0.0) || (lo > 0.0 && p[i - 1] > 0.0)) {
            let y = lo * 2.0;
            let x = hi + y;
            if y == x - hi { hi = x; }
        }
        hi
    }

    /// The smallest f64 at or above the exact sum.
    pub fn up(&self) -> f64 {
        let r = self.nearest();
        let mut residual = self.clone();
        residual.add(-r);
        if residual.signum() > 0 { r.next_up() } else { r }
    }
}

/// The exact sum of `xs`, rounded once to the nearest f64.
pub fn sum_nearest(xs: impl IntoIterator<Item = f64>) -> f64 {
    let mut s = Expansion::new();
    for x in xs { s.add(x); }
    s.nearest()
}

/// The smallest f64 at or above the exact sum of `xs`.
pub fn sum_up(xs: impl IntoIterator<Item = f64>) -> f64 {
    let mut s = Expansion::new();
    for x in xs { s.add(x); }
    s.up()
}

/// The exact Σ k·x over (k, x), rounded once to the nearest f64.
pub fn combination_nearest(terms: impl IntoIterator<Item = (i64, f64)>) -> f64 {
    let mut s = Expansion::new();
    for (k, x) in terms { if k != 0 { s.add_product(k as f64, x); } }
    s.nearest()
}

/// The smallest f64 at or above the exact Σ k·x over (k, x).
pub fn combination_up(terms: impl IntoIterator<Item = (i64, f64)>) -> f64 {
    let mut s = Expansion::new();
    for (k, x) in terms { if k != 0 { s.add_product(k as f64, x); } }
    s.up()
}
