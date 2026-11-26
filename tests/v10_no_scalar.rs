//! The operators never calculate a scalar (Origin, 2026-11-26, 104th).
//!
//! Origin: *"the only thing that in the operators is relational information
//! and mathematics there can be no scalars"*; *"We can always input
//! projections provided that we describe their provenance otherwise we would
//! not be able to calculate on legacy information I just don't want the
//! operators themselves to calculate a scalar"*.
//!
//! **The test.** An operator that calculates no scalar (no magnitude, norm,
//! product, ratio) is linear in the registered information:
//!   reversal  f(−x) = −f(x)        a magnitude fails: |−x| = |x|;
//!   sum       f(x + y) = f(x) + f(y)   a product fails.
//! Three whole fields are formed over the same declared relations and δ, from
//! the registrations x, −x and x + y (carrier included), each through
//! `RelationalField::new`, so every variable is evaluated together (53rd).
//! Every numeric part the field reports (read from its structure, as the
//! derivation register reads it) is checked, relation by relation, at every
//! registration and step. A part whose entries change in number between the
//! three fields (a selection by δ, 98th) is not linear either. δ itself
//! (`Registration.delta`) is the declared resolution, the same in all three:
//! framework, not checked. On this family no run-constant parts are declared
//! (Z1a), so the gain parts are zero and pass trivially; they are rules.
//!
//! **What it reports.** The parts that fail, sorted by their derivation:
//! - an *operator reading* that fails calculates a scalar inside the
//!   operators (against the 104th): listed in `OPERATOR_SCALARS`, each shown
//!   to fail here, for Origin to retire;
//! - *legacy mathematics* that fails is listed in `LEGACY_SCALARS`;
//! - *rules* (bounds, decisions at δ, shares, counts) are the framework
//!   (104th (2)) and are not required to be linear.
//! Each list must be exactly what fails: a part retired or made linear must
//! leave its list, and a new part that calculates a scalar fails this test.
//!
//! **Cannot tell (named):** linearity on this family is necessary for "no
//! scalar", not sufficient: a nonlinear form that happens to be linear on
//! every field of the family would pass. Parts that are integers (identities,
//! counts, pairs) or mix integers with values are not checked here and are
//! listed. Readings outside the field (W5's radial graphs in `coherence.rs`,
//! V3b's `connected_exchange`) are not reached.
//!
//! [IMPL] SYNTHETIC FIXTURES, enumerated from declared seeds (W17): dyadic
//! values, so every sum is exact and equality is exact.

#[path = "support/synthetic_whole_field.rs"]
mod support;

use std::cell::RefCell;
use std::collections::BTreeSet;

use abr_kernel::derivation::{of_part, Derivation};
use abr_kernel::relational_field::InnerProduct;
use support::*;

/// Operator readings that calculate a scalar, shown to fail below: for
/// Origin to retire (104th).
/// *Was:* `&["EnergyExchange.epsilon"]`, the magnitude |A(c)|, retired by
/// Origin (104th).
const OPERATOR_SCALARS: &[&str] = &[];

/// Legacy mathematics that calculates a scalar, shown to fail below.
const LEGACY_SCALARS: &[&str] = &["ChangeCoupling.minor"];

/// Every `Struct.field` in a structure's `Debug` form, with the text of its
/// value, in order of appearance (nested fields included).
fn leaves(debug: &str) -> Vec<(String, String)> {
    let b = debug.as_bytes();
    let mut out = Vec::new();
    let mut holders: Vec<String> = Vec::new();
    let mut last = String::new();
    let (mut i, mut in_str) = (0usize, false);
    while i < b.len() {
        let c = b[i];
        if in_str {
            if c == b'\\' { i += 2; continue; }
            if c == b'"' { in_str = false; }
            i += 1;
            continue;
        }
        match c {
            b'"' => { in_str = true; i += 1; }
            b'{' => { holders.push(std::mem::take(&mut last)); i += 1; }
            b'}' => { holders.pop(); i += 1; }
            _ if c.is_ascii_alphabetic() || c == b'_' => {
                let start = i;
                while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') { i += 1; }
                let ident = &debug[start..i];
                let before = debug[..start].trim_end();
                if debug[i..].starts_with(": ") && (before.ends_with('{') || before.ends_with(',')) {
                    let v0 = i + 2;
                    let (mut j, mut depth, mut s) = (v0, 0i32, false);
                    while j < b.len() {
                        let d = b[j];
                        if s { if d == b'\\' { j += 1; } else if d == b'"' { s = false; } }
                        else {
                            match d {
                                b'"' => s = true,
                                b'[' | b'(' | b'{' => depth += 1,
                                b']' | b')' | b'}' => { if depth == 0 { break; } depth -= 1; }
                                b',' if depth == 0 => break,
                                _ => {}
                            }
                        }
                        j += 1;
                    }
                    let holder = holders.last().cloned().unwrap_or_default();
                    out.push((format!("{holder}.{ident}"), debug[v0..j].to_string()));
                } else {
                    last = ident.to_string();
                }
            }
            _ => { i += 1; }
        }
    }
    out
}

/// The values of a part, if it is numeric: `Some(values)` when every token
/// is a number and at least one is a float; `None` for integers, mixed or
/// structured values.
fn floats(text: &str) -> Option<Vec<f64>> {
    let toks: Vec<&str> = text.split(|c: char| "[](), \n".contains(c)).filter(|t| !t.is_empty()).collect();
    if toks.is_empty() { return None; }
    let is_float = |t: &str| t.contains('.') || t.contains('e') || t == "inf" || t == "-inf" || t == "NaN";
    if !toks.iter().all(|t| t.parse::<f64>().is_ok() && is_float(t)) { return None; }
    Some(toks.iter().map(|t| t.parse().unwrap()).collect())
}

/// Every numeric part of a field, concatenated in order, by name.
fn numeric_parts(debug: &str) -> (Vec<(String, Vec<f64>)>, BTreeSet<String>) {
    let mut parts: Vec<(String, Vec<f64>)> = Vec::new();
    let mut other = BTreeSet::new();
    for (name, text) in leaves(debug) {
        match floats(&text) {
            Some(v) => match parts.iter_mut().find(|p| p.0 == name) { Some(p) => p.1.extend(v), None => parts.push((name, v)) },
            None => { other.insert(name); }
        }
    }
    other.retain(|n| !parts.iter().any(|p| &p.0 == n));
    (parts, other)
}

fn scaled(d: &Declared, k: f64) -> Declared {
    Declared { values: d.values.iter().map(|t| t.iter().map(|c| c.iter().map(|v| k * v).collect()).collect()).collect(), ..d.clone() }
}

fn summed(d: &Declared, e: &Declared) -> Declared {
    Declared {
        values: d.values.iter().zip(&e.values).map(|(t, u)| t.iter().zip(u).map(|(c, w)| c.iter().zip(w).map(|(a, b)| a + b).collect()).collect()).collect(),
        ..d.clone()
    }
}

#[test]
fn the_operators_never_calculate_a_scalar() {
    let fails: RefCell<BTreeSet<String>> = RefCell::new(BTreeSet::new());
    let checked: RefCell<BTreeSet<String>> = RefCell::new(BTreeSet::new());
    let not_checked: RefCell<BTreeSet<String>> = RefCell::new(BTreeSet::new());
    let moved = RefCell::new(0usize);
    for_all("no scalar: linearity of every reported part", 0x104_01, |r| {
        let rel = enumerated_rel(r);
        let k = r.usize(2, 3);
        let regs = r.usize(2, 4);
        let x = enumerated_declared(r, &rel, k, regs, 0.5, false, false);
        let y = Declared { values: enumerated_declared(r, &rel, k, regs, 0.5, false, false).values, ..x.clone() };
        let fx = whole(&rel, x.clone());
        let fneg = whole(&rel, scaled(&x, -1.0));
        let fy = whole(&rel, y.clone());
        let fsum = whole(&rel, summed(&x, &y));
        // The field, and the orthogonal split it reports (100th).
        let of = |w: &Whole| format!("{:?} {:?}", w.field, w.field.two_parts(&InnerProduct::equal()));
        let (px, other) = numeric_parts(&of(&fx));
        let (pneg, _) = numeric_parts(&of(&fneg));
        let (py, _) = numeric_parts(&of(&fy));
        let (psum, _) = numeric_parts(&of(&fsum));
        not_checked.borrow_mut().extend(other);
        let get = |p: &[(String, Vec<f64>)], n: &str| p.iter().find(|q| q.0 == n).map(|q| q.1.clone());
        for (name, vx) in &px {
            // δ is the declared resolution of the observation, the same in
            // all three fields: framework (104th (2)), not registered data.
            if name == "Registration.delta" { continue; }
            checked.borrow_mut().insert(name.clone());
            if vx.iter().any(|v| *v != 0.0) { *moved.borrow_mut() += 1; }
            let linear = match (get(&pneg, name), get(&py, name), get(&psum, name)) {
                (Some(vn), Some(vy), Some(vs)) =>
                    vn.len() == vx.len() && vy.len() == vx.len() && vs.len() == vx.len()
                    && vx.iter().zip(&vn).all(|(a, b)| *b == -a)
                    && vx.iter().zip(&vy).zip(&vs).all(|((a, b), s)| *s == a + b),
                _ => false,
            };
            if !linear { fails.borrow_mut().insert(name.clone()); }
        }
    });
    assert!(*moved.borrow() > 0, "the family reaches nonzero values");

    let kind = |n: &str| match of_part(n) {
        Some(Derivation::Operator { .. }) => "operator",
        Some(Derivation::Legacy { .. }) => "legacy",
        Some(Derivation::Rule { .. }) => "rule",
        Some(Derivation::Registered { .. }) => "registered",
        Some(Derivation::Identity { .. }) => "identity",
        None => "underived",
    };
    let fails = fails.into_inner();
    let by = |k: &str| fails.iter().filter(|n| kind(n) == k).cloned().collect::<Vec<_>>();
    for k in ["operator", "legacy", "rule", "registered", "identity", "underived"] {
        eprintln!("[no scalar] not linear, {k}: {:?}", by(k));
    }
    let passes: Vec<_> = checked.borrow().iter().filter(|n| !fails.contains(*n)).cloned().collect();
    eprintln!("[no scalar] linear: {passes:?}");
    eprintln!("[no scalar] not checked (integers, mixed or structured): {:?}", not_checked.borrow().iter().filter(|n| kind(n) == "operator" || kind(n) == "legacy").collect::<Vec<_>>());

    assert_eq!(by("operator"), OPERATOR_SCALARS.to_vec(), "operator readings that calculate a scalar (104th)");
    assert_eq!(by("legacy"), LEGACY_SCALARS.to_vec(), "legacy mathematics that calculates a scalar");
    assert!(by("registered").is_empty(), "a registered part is the input itself, linear by construction: {:?}", by("registered"));
}

/// The test can fail: a magnitude and a product, planted, fail it; A's own
/// difference passes.
#[test]
fn a_planted_magnitude_and_product_fail_the_test() {
    let linear = |f: &dyn Fn(f64, f64) -> f64| {
        let xs = [(1.5, -0.25), (-2.0, 0.75), (0.5, 0.5)];
        let ys = [(0.25, 1.0), (1.0, -1.5), (-0.75, 0.125)];
        xs.iter().zip(&ys).all(|(&(a, b), &(c, d))| f(-a, -b) == -f(a, b) && f(a + c, b + d) == f(a, b) + f(c, d))
    };
    assert!(linear(&|s, t| s - t), "A: x[s] − x[t]");
    assert!(!linear(&|s, t| (s - t).abs()), "|A| is a magnitude");
    assert!(!linear(&|s, t| s * t), "a product");
    // And the Debug reading finds a part inside nested structures.
    let found = leaves("Outer { a: [1.0, -2.5], inner: Inner { b: [[0.5]], id: 3 }, name: \"x, y\" }");
    assert_eq!(found.iter().map(|x| x.0.as_str()).collect::<Vec<_>>(), vec!["Outer.a", "Outer.inner", "Inner.b", "Inner.id", "Outer.name"]);
    assert_eq!(floats("[[0.5, -1.0]]"), Some(vec![0.5, -1.0]));
    assert_eq!(floats("3"), None);
}

/// Readings beyond the relational field that calculate a scalar, shown to
/// fail below: for Origin. *Empty: none found on this family.*
const BEYOND_SCALARS: &[&str] = &[];

/// The same test on the readings outside the relational field (2026-11-26,
/// the comprehensive suite): W5's radial graphs (`coherence.rs`) and the
/// declared ground (§2.7, `domain.rs`). Every numeric part they report is
/// checked for reversal and sum, as above; their bounds, shares and counts
/// (named here by the words bound, share, degree or steps, or with no
/// derivation of their own) are the framework and listed apart.
#[test]
fn the_readings_beyond_the_field_calculate_no_scalar() {
    let fails: RefCell<BTreeSet<String>> = RefCell::new(BTreeSet::new());
    let checked: RefCell<BTreeSet<String>> = RefCell::new(BTreeSet::new());
    for_all("no scalar: beyond the field", 0x104_02, |r| {
        let rel = enumerated_rel(r);
        let k = r.usize(2, 3);
        let regs = r.usize(2, 4);
        let x = enumerated_declared(r, &rel, k, regs, 0.5, false, false);
        let y = Declared { values: enumerated_declared(r, &rel, k, regs, 0.5, false, false).values, ..x.clone() };
        let of = |d: &Declared| {
            let w = whole(&rel, d.clone());
            let graphs = abr_kernel::coherence::radial_graphs(&w.field, &rel, regs - 1).ok();
            let df = declared(&rel, d, domain(d, true, vec![], vec![]));
            format!("{graphs:?} {:?}", df.energy)
        };
        let (px, _) = numeric_parts(&of(&x));
        let (pneg, _) = numeric_parts(&of(&scaled(&x, -1.0)));
        let (py, _) = numeric_parts(&of(&y));
        let (psum, _) = numeric_parts(&of(&summed(&x, &y)));
        let get = |p: &[(String, Vec<f64>)], n: &str| p.iter().find(|q| q.0 == n).map(|q| q.1.clone());
        for (name, vx) in &px {
            checked.borrow_mut().insert(name.clone());
            let linear = match (get(&pneg, name), get(&py, name), get(&psum, name)) {
                (Some(vn), Some(vy), Some(vs)) => vn.len() == vx.len() && vy.len() == vx.len() && vs.len() == vx.len()
                    && vx.iter().zip(&vn).all(|(a, b)| *b == -a) && vx.iter().zip(&vy).zip(&vs).all(|((a, b), s)| *s == a + b),
                _ => false,
            };
            if !linear { fails.borrow_mut().insert(name.clone()); }
        }
    });
    let framework = |n: &str| { let f = n.split('.').nth(1).unwrap_or(""); f.contains("bound") || f.contains("share") || f.contains("degree") || f.contains("steps") };
    let fails = fails.into_inner();
    let scalars: Vec<String> = fails.iter().filter(|n| !framework(n)).cloned().collect();
    eprintln!("[no scalar, beyond the field] checked: {:?}", checked.borrow());
    eprintln!("[no scalar, beyond the field] not linear, framework: {:?}", fails.iter().filter(|n| framework(n)).collect::<Vec<_>>());
    eprintln!("[no scalar, beyond the field] not linear, readings: {scalars:?}");
    assert!(!checked.borrow().is_empty(), "the readings beyond the field are reached");
    assert_eq!(scalars, BEYOND_SCALARS.to_vec(), "readings beyond the field that calculate a scalar (104th)");
}
