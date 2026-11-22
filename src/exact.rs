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
//! **Legacy mathematics (W12 as amended, Origin 2026-11-23, 42nd).** This
//! module uses a general numerical method: Shewchuk's expansions, as in
//! Python's `math.fsum`, with fused multiply-add. It adds no form, input or
//! construct to any reading. Stated necessity: the 41st rule; without it, a
//! closure depends on its traversal order. Listed as OC-LM-1, item 1, for
//! Origin to confirm.
//!
//! **The range of exact arithmetic (the Verifier's batch-2 findings,
//! 2026-11-23, 46th).** The two-sum and the fused-multiply-add product are
//! exact only while no intermediate overflows and no product underflows.
//! *Was:* an assumption ("finite inputs whose sums and products neither
//! overflow nor underflow"), which admission did not enforce. The Verifier
//! found two consequences:
//! - a PC3 state read as resolved from values near 2⁻¹⁰⁷⁴, where a product's
//!   residual underflowed;
//! - `[MAX, MAX, −MAX]` summed to NaN while `[MAX, −MAX, MAX]` gave MAX.
//!
//! Now:
//! 1. **Admission.** Every registered value, bound, offset and gain is 0 or
//!    has magnitude in [2⁻²⁰⁰, 2²⁰⁰] (`in_exact_range`; about 10⁻⁶⁰ to
//!    10⁶⁰), checked where V10 data enters. A value outside it is refused.
//! 2. **Every operation checks itself.** A sum whose running total is not
//!    finite, or a product of two non-zero factors below 2⁻⁹⁶⁹ (where the
//!    residual may not be exact) or not finite, panics: outside the range of
//!    exact arithmetic. So a result is exact or refused, never silently
//!    wrong, whatever the inputs.
//!
//! Inside the admitted range the second check does not fire on the V10 forms:
//! every non-zero partial they form lies between about 2⁻⁹⁰⁴ and 2⁴²⁰
//! (values and bounds ≥ 2⁻²⁰⁰, their exact differences ≥ 2⁻²⁵², gains ≥
//! 2⁻²⁰⁰, at most two of each multiplied). Tested by
//! `tests/v10_exact_range.rs`.

/// An exact sum of f64 terms.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Expansion {
    /// Non-overlapping partials, increasing in magnitude. Their sum is exact.
    partials: Vec<f64>,
}

/// The admitted range of exact arithmetic: 0, or 2⁻²⁰⁰ ≤ |x| ≤ 2²⁰⁰ (46th).
pub fn in_exact_range(x: f64) -> bool {
    x == 0.0 || (x.is_finite() && x.abs() >= 2f64.powi(-200) && x.abs() <= 2f64.powi(200))
}

/// Below this, a product's fused-multiply-add residual may not be exact.
const PRODUCT_FLOOR: f64 = f64::from_bits(54u64 << 52); // 2⁻⁹⁶⁹: biased exponent −969 + 1023

impl Expansion {
    pub fn new() -> Self { Expansion { partials: Vec::new() } }

    /// Add x exactly.
    pub fn add(&mut self, x: f64) {
        assert!(x.is_finite(), "a term outside the range of exact arithmetic (OC-NUM-1): {x}");
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
        assert!(x.is_finite(), "a sum outside the range of exact arithmetic (OC-NUM-1): it overflowed");
        self.partials.truncate(i);
        self.partials.push(x);
    }

    /// Add the product a·b exactly: p = fl(a·b) and the error fma(a, b, −p).
    pub fn add_product(&mut self, a: f64, b: f64) {
        let p = a * b;
        assert!(p.is_finite(), "a product outside the range of exact arithmetic (OC-NUM-1): it overflowed");
        assert!(a == 0.0 || b == 0.0 || p.abs() >= PRODUCT_FLOOR,
            "a product outside the range of exact arithmetic (OC-NUM-1): {a:e} × {b:e} underflows");
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

/// a + b = hi + lo exactly, with hi = fl(a + b) (Knuth's two-sum).
pub fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let hi = a + b;
    let bb = hi - a;
    (hi, (a - (hi - bb)) + (b - bb))
}

impl Expansion {
    /// The exact value x.
    pub fn of(x: f64) -> Self { let mut e = Expansion::new(); e.add(x); e }
    /// a − b exactly.
    pub fn diff(a: f64, b: f64) -> Self { let mut e = Expansion::of(a); e.add(-b); e }
    /// −self.
    pub fn neg(&self) -> Self { Expansion { partials: self.partials.iter().map(|p| -p).collect() } }
    /// self + o exactly.
    pub fn plus(&self, o: &Expansion) -> Self { let mut e = self.clone(); e.add_expansion(o); e }
    /// self − o exactly.
    pub fn minus(&self, o: &Expansion) -> Self { self.plus(&o.neg()) }
    /// self · o exactly: every pair of partials through `add_product`.
    pub fn times(&self, o: &Expansion) -> Self {
        let mut e = Expansion::new();
        for &p in &self.partials { for &q in &o.partials { e.add_product(p, q); } }
        e
    }
    /// self · g exactly.
    pub fn times_f64(&self, g: f64) -> Self { let mut e = Expansion::new(); e.add_times(self, g); e }
}

/// |v| > h, decided exactly.
pub fn exceeds(v: &Expansion, h: &Expansion) -> bool { v.abs().minus(h).signum() > 0 }

/// The bound on x·y over [x ± hx] and [y ± hy], |x|·hy + |y|·hx + hx·hy,
/// exactly (the product bound of V2, PC2, PC3).
pub fn product_bound(x: &Expansion, hx: &Expansion, y: &Expansion, hy: &Expansion) -> Expansion {
    x.abs().times(hy).plus(&y.abs().times(hx)).plus(&hx.times(hy))
}

/// A gain bound γ on an entry with value v and bound h: h + γ·(|v| + h),
/// exactly (Z1a under the scaling model).
pub fn gain_widened(v: &Expansion, h: &Expansion, g: f64) -> Expansion {
    if g == 0.0 { return h.clone(); }
    h.plus(&v.abs().plus(h).times_f64(g))
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
