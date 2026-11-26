//! Exact sums of registered values (Origin, 2026-11-23, 41st: "we are
//! decomposing complex systems but we're not simplifying them").
//!
//! A V10 reading decomposes a field into parts: turn sums, accumulations,
//! passages. Each is a sum of registered entries with integer
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
//! turn sum depends on its traversal order. Listed as OC-LM-1, item 1, for
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

// ═══════════════════════════════════════════════════════════════════════════
// Exact rationals (Origin, 2026-11-26, 100th: the orthogonal split)
// ═══════════════════════════════════════════════════════════════════════════
//
// The orthogonal split of a field of relations into its two parts needs a
// solve over the loci, whose results are rationals that no f64 holds exactly.
// They are computed here exactly: arbitrary-precision integers (base 2³²,
// schoolbook multiplication, binary long division) and rationals kept in
// lowest terms (Euclid's algorithm), so equal values compare equal. Each
// value is converted to f64 once, to the nearest or upward.
//
// [IMPL] Legacy arithmetic at the code layer (OC-LM-1; `derivation.rs`). No
// external dependency (Cargo.toml). Conversion to f64 does not produce
// subnormal values; a magnitude outside the normal range panics.

/// A non-negative integer of any size, base 2³², least significant first,
/// without trailing zeros (zero is empty).
#[derive(Debug, Clone, PartialEq, Eq)]
struct Nat(Vec<u32>);

impl Nat {
    fn zero() -> Self { Nat(Vec::new()) }
    fn from_u64(x: u64) -> Self { let mut n = Nat(vec![x as u32, (x >> 32) as u32]); n.trim(); n }
    fn trim(&mut self) { while self.0.last() == Some(&0) { self.0.pop(); } }
    fn is_zero(&self) -> bool { self.0.is_empty() }
    fn bits(&self) -> usize { match self.0.last() { None => 0, Some(&top) => 32 * (self.0.len() - 1) + (32 - top.leading_zeros() as usize) } }
    fn bit(&self, i: usize) -> bool { self.0.get(i / 32).is_some_and(|w| w >> (i % 32) & 1 == 1) }
    fn cmp(&self, o: &Nat) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        if self.0.len() != o.0.len() { return self.0.len().cmp(&o.0.len()); }
        for i in (0..self.0.len()).rev() { if self.0[i] != o.0[i] { return if self.0[i] < o.0[i] { Less } else { Greater }; } }
        Equal
    }
    fn add(&self, o: &Nat) -> Nat {
        let mut r = Vec::with_capacity(self.0.len().max(o.0.len()) + 1);
        let mut carry = 0u64;
        for i in 0..self.0.len().max(o.0.len()) {
            let s = *self.0.get(i).unwrap_or(&0) as u64 + *o.0.get(i).unwrap_or(&0) as u64 + carry;
            r.push(s as u32); carry = s >> 32;
        }
        if carry > 0 { r.push(carry as u32); }
        let mut n = Nat(r); n.trim(); n
    }
    /// self − o, for self ≥ o.
    fn sub(&self, o: &Nat) -> Nat {
        let mut r = Vec::with_capacity(self.0.len());
        let mut borrow = 0i64;
        for i in 0..self.0.len() {
            let mut d = self.0[i] as i64 - *o.0.get(i).unwrap_or(&0) as i64 - borrow;
            borrow = if d < 0 { d += 1 << 32; 1 } else { 0 };
            r.push(d as u32);
        }
        assert_eq!(borrow, 0, "Nat::sub below zero");
        let mut n = Nat(r); n.trim(); n
    }
    fn mul(&self, o: &Nat) -> Nat {
        if self.is_zero() || o.is_zero() { return Nat::zero(); }
        let mut r = vec![0u64; self.0.len() + o.0.len() + 1];
        for (i, &a) in self.0.iter().enumerate() {
            let mut carry = 0u64;
            for (j, &b) in o.0.iter().enumerate() {
                let t = r[i + j] + a as u64 * b as u64 + carry;
                r[i + j] = t & 0xFFFF_FFFF; carry = t >> 32;
            }
            let mut k = i + o.0.len();
            while carry > 0 { let t = r[k] + carry; r[k] = t & 0xFFFF_FFFF; carry = t >> 32; k += 1; }
        }
        let mut n = Nat(r.into_iter().map(|x| x as u32).collect()); n.trim(); n
    }
    fn shl(&self, s: usize) -> Nat {
        if self.is_zero() { return Nat::zero(); }
        let (w, b) = (s / 32, s % 32);
        let mut r = vec![0u32; w];
        let mut carry = 0u32;
        for &x in &self.0 {
            r.push(if b == 0 { x } else { (x << b) | carry });
            carry = if b == 0 { 0 } else { x >> (32 - b) };
        }
        if carry > 0 { r.push(carry); }
        let mut n = Nat(r); n.trim(); n
    }
    /// (quotient, remainder) of self / d, d ≠ 0: binary long division.
    fn divrem(&self, d: &Nat) -> (Nat, Nat) {
        assert!(!d.is_zero(), "division by zero");
        if self.cmp(d) == std::cmp::Ordering::Less { return (Nat::zero(), self.clone()); }
        let mut q = vec![0u32; self.0.len()];
        let mut r = Nat::zero();
        for i in (0..self.bits()).rev() {
            r = r.shl(1);
            if self.bit(i) { if r.0.is_empty() { r.0.push(1); } else { r.0[0] |= 1; } }
            if r.cmp(d) != std::cmp::Ordering::Less { r = r.sub(d); q[i / 32] |= 1 << (i % 32); }
        }
        let mut qn = Nat(q); qn.trim(); (qn, r)
    }
    fn gcd(&self, o: &Nat) -> Nat {
        let (mut a, mut b) = (self.clone(), o.clone());
        while !b.is_zero() { let (_, r) = a.divrem(&b); a = b; b = r; }
        a
    }
    fn low_u64(&self) -> u64 { *self.0.first().unwrap_or(&0) as u64 | (*self.0.get(1).unwrap_or(&0) as u64) << 32 }
}

/// An exact rational, in lowest terms, the denominator positive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ratio { negative: bool, num: Nat, den: Nat }

impl Ratio {
    fn new(negative: bool, num: Nat, den: Nat) -> Self {
        if num.is_zero() { return Ratio::zero(); }
        let g = num.gcd(&den);
        let (num, den) = if g == Nat::from_u64(1) { (num, den) } else { (num.divrem(&g).0, den.divrem(&g).0) };
        Ratio { negative, num, den }
    }
    pub fn zero() -> Self { Ratio { negative: false, num: Nat::zero(), den: Nat::from_u64(1) } }
    pub fn from_i64(x: i64) -> Self { Ratio::new(x < 0, Nat::from_u64(x.unsigned_abs()), Nat::from_u64(1)) }
    /// The exact value of a finite f64.
    pub fn from_f64(x: f64) -> Self {
        assert!(x.is_finite(), "a finite value");
        if x == 0.0 { return Ratio::zero(); }
        let bits = x.to_bits();
        let exp = ((bits >> 52) & 0x7FF) as i64;
        let frac = bits & ((1u64 << 52) - 1);
        let (mant, e) = if exp == 0 { (frac, -1074) } else { (frac | (1u64 << 52), exp - 1075) };
        let m = Nat::from_u64(mant);
        if e >= 0 { Ratio::new(x < 0.0, m.shl(e as usize), Nat::from_u64(1)) } else { Ratio::new(x < 0.0, m, Nat::from_u64(1).shl((-e) as usize)) }
    }
    pub fn is_zero(&self) -> bool { self.num.is_zero() }
    pub fn neg(&self) -> Self { if self.is_zero() { self.clone() } else { Ratio { negative: !self.negative, ..self.clone() } } }
    pub fn abs(&self) -> Self { Ratio { negative: false, ..self.clone() } }
    pub fn add(&self, o: &Ratio) -> Ratio {
        let (a, b) = (self.num.mul(&o.den), o.num.mul(&self.den));
        let den = self.den.mul(&o.den);
        if self.negative == o.negative { return Ratio::new(self.negative, a.add(&b), den); }
        match a.cmp(&b) {
            std::cmp::Ordering::Equal => Ratio::zero(),
            std::cmp::Ordering::Greater => Ratio::new(self.negative, a.sub(&b), den),
            std::cmp::Ordering::Less => Ratio::new(o.negative, b.sub(&a), den),
        }
    }
    pub fn sub(&self, o: &Ratio) -> Ratio { self.add(&o.neg()) }
    pub fn mul(&self, o: &Ratio) -> Ratio { Ratio::new(self.negative != o.negative, self.num.mul(&o.num), self.den.mul(&o.den)) }
    pub fn div(&self, o: &Ratio) -> Ratio {
        assert!(!o.is_zero(), "division by zero");
        Ratio::new(self.negative != o.negative, self.num.mul(&o.den), self.den.mul(&o.num))
    }
    /// self ≤ o.
    pub fn le(&self, o: &Ratio) -> bool { let d = o.sub(self); d.is_zero() || !d.negative }
    /// The nearest f64 (ties to even).
    pub fn to_f64_nearest(&self) -> f64 { self.to_f64(false) }
    /// The least f64 not below the value.
    pub fn to_f64_up(&self) -> f64 { self.to_f64(true) }
    fn to_f64(&self, up: bool) -> f64 {
        if self.is_zero() { return 0.0; }
        let (n, d) = (&self.num, &self.den);
        let s = 55 + d.bits() as i64 - n.bits() as i64;
        let (q, r) = if s >= 0 { n.shl(s as usize).divrem(d) } else { n.divrem(&d.shl((-s) as usize)) };
        let sticky = !r.is_zero();
        let shift = q.bits() - 53;
        let mut mant = q.divrem(&Nat::from_u64(1).shl(shift)).0.low_u64();
        let low = q.divrem(&Nat::from_u64(1).shl(shift)).1;
        let half = Nat::from_u64(1).shl(shift - 1);
        // Magnitude rounding: toward +∞ means away from zero for a positive
        // value and toward zero for a negative one.
        let away = if up {
            !self.negative && (!low.is_zero() || sticky)
        } else {
            match low.cmp(&half) {
                std::cmp::Ordering::Greater => true,
                std::cmp::Ordering::Less => false,
                std::cmp::Ordering::Equal => sticky || mant & 1 == 1,
            }
        };
        if away { mant += 1; }
        let e2 = shift as i64 - s;
        assert!((-1000..1000).contains(&(e2 + 53)), "outside the normal range of f64");
        let v = mant as f64 * 2f64.powi(e2 as i32);
        if self.negative { -v } else { v }
    }
}
