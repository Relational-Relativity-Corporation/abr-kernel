//! No information is repeated (Origin, 2026-11-26, 106th, 107th): *"no
//! information can be repeated this is part of the same thing which makes
//! relational evolution both inevitable and constant"*. A loop-shaped route
//! is a turn, shape, never a system that is a loop; no pitch, period or
//! phase is assumed or fitted (107th (a)).
//!
//! 1. **No periodicity in the kernel's code**: no name in the code (outside
//!    comments and quoted text) is a period, phase, modulus, Fourier term,
//!    wraparound, or a cycle, except the deprecated aliases kept for the
//!    applications and the retired V9 names, listed.
//! 2. **Two registrations with identical values are two points of
//!    evolution**: the field reports each, at its own index, and the step
//!    between them; nothing is merged or skipped.
//! 3. **The degrees keep identical routes at distinct points apart** (108th):
//!    a text that repeats one word exactly gives one degree per earlier
//!    point, never one per route.
//!
//! **Cannot tell (named):** a period written under another name, or an
//! identification of two points made by arithmetic rather than by name, is
//! not seen by (1); (2) and (3) are exercised on the field and the order.
//!
//! [IMPL] SYNTHETIC FIXTURES, enumerated from declared seeds (W17).

#[path = "support/synthetic_whole_field.rs"]
mod support;

use abr_kernel::operators::DeclaredRelations;
use abr_kernel::order::Order;
use support::*;

/// Names that identify points of evolution or assume a period.
const PERIODIC: &[&str] = &["period", "phase", "modulo", "modulus", "fourier", "wraparound", "cyclic", "cycle"];

/// Names allowed, as (file, name): the deprecated aliases (107th), the
/// retired V9 names, and one finding for Origin.
const ALLOWED: &[(&str, &str)] = &[
    // FOR ORIGIN (found 2026-11-26): G3, the relative phase of two relations
    // (§2.9, 69th, Proposed). A phase is an angle, and an angle wraps (2π is
    // 0): two distinct values read as one. Listed, not changed, until Origin
    // decides whether it is renamed, restated or retired (106th).
    ("src/geometry.rs", "RelativePhase"), ("src/geometry.rs", "relative_phases"),
    ("src/topology.rs", "CycleBasis"), ("src/topology.rs", "cycle_basis"), ("src/topology.rs", "two_route_cycle"),
    ("src/persistence_change.rs", "cycle_closure"), ("src/persistence_change.rs", "cycle_closure_resolved"),
    ("src/persistence_change.rs", "cycle_change_closure"), ("src/persistence_change.rs", "cycle_change_closure_resolved"),
    ("src/derived_invariants.rs", "n_relational_cycles"), ("src/operators.rs", "abr_two_cycle_evolution"),
];

/// The code of a line with its comment and quoted text removed.
fn code(line: &str) -> String {
    let mut out = String::new();
    let (mut in_str, mut chars) = (false, line.chars().peekable());
    while let Some(c) = chars.next() {
        if in_str { if c == '\\' { chars.next(); } else if c == '"' { in_str = false; } continue; }
        if c == '"' { in_str = true; continue; }
        if c == '/' && chars.peek() == Some(&'/') { break; }
        out.push(c);
    }
    out
}

fn periodic_names(files: &[(String, String)]) -> Vec<String> {
    let mut why = Vec::new();
    for (path, src) in files {
        let mut in_block = false;
        for (i, line) in src.lines().enumerate() {
            if in_block { if line.contains("*/") { in_block = false; } continue; }
            if line.trim_start().starts_with("/*") { in_block = !line.contains("*/"); continue; }
            for word in code(line).split(|c: char| !(c.is_alphanumeric() || c == '_')).filter(|w| !w.is_empty()) {
                let lower = word.to_lowercase();
                if PERIODIC.iter().any(|p| lower.contains(p)) && !ALLOWED.iter().any(|a| a.0 == path && a.1 == word) {
                    why.push(format!("{path}:{}: {word}", i + 1));
                }
            }
        }
    }
    why
}

fn sources() -> Vec<(String, String)> {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/src");
    let mut out: Vec<(String, String)> = std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "rs"))
        .map(|p| (format!("src/{}", p.file_name().unwrap().to_string_lossy()), std::fs::read_to_string(&p).unwrap())).collect();
    out.sort();
    out
}

/// 1. No name in the kernel's code assumes a period or identifies points.
#[test]
fn one_no_periodicity_is_named_in_the_code() {
    let why = periodic_names(&sources());
    assert!(why.is_empty(), "\n{}\n", why.join("\n"));
}

#[test]
fn one_the_check_can_fail() {
    let src = "fn f() { let period = 3; }\n// a period in a comment\nlet s = \"phase in text\";\nlet x = x % modulus;\n".to_string();
    let why = periodic_names(&[("src/x.rs".into(), src)]);
    assert_eq!(why.len(), 2, "{why:?}");
}

/// 2. Identical registrations are distinct points: every registration is
/// reported at its own index, the step between identical ones is read (its
/// change zero, structure), and nothing is merged.
#[test]
fn two_identical_registrations_are_two_points_of_evolution() {
    for_all("no repetition: identical registrations", 0x106_01, |r| {
        let rel = enumerated_rel(r);
        let k = r.usize(2, 3);
        let mut d = enumerated_declared(r, &rel, k, 2, 0.5, false, false);
        let first = d.values[0].clone();
        d.values.push(first.clone());
        d.values.insert(1, first);
        let w = whole(&rel, d);
        assert_eq!(w.field.registrations.len(), 4, "every registration kept");
        assert_eq!((w.field.spatial.len(), w.field.persistence.len()), (4, 3), "every point and every step");
        assert_eq!(w.field.spatial[0], w.field.spatial[1], "the same values read the same shape");
        let pc = w.freedom();
        assert!(pc.relations.iter().filter(|x| x.step == 0).all(|x| x.change == 0.0), "the step between them is read, its change zero");
        assert_eq!(pc.relations.iter().filter(|x| x.step == 0).count(), k * rel.n_edges(), "the step is not skipped");
    });
}

/// 3. A text that repeats a word exactly: one degree per earlier point.
#[test]
fn three_an_exact_repetition_of_text_is_distinct_degrees() {
    let text = "abc abc abc abc ab";
    let cs: Vec<char> = text.chars().collect();
    let mut chars: Vec<char> = Vec::new();
    for &c in &cs { if !chars.contains(&c) { chars.push(c); } }
    let at = |c: char| chars.iter().position(|&x| x == c).unwrap();
    let mut pairs: Vec<(char, char)> = Vec::new();
    let mut steps = Vec::new();
    for w in cs.windows(2) {
        if !pairs.contains(&(w[0], w[1])) { pairs.push((w[0], w[1])); }
        steps.push(pairs.iter().position(|&x| x == (w[0], w[1])).unwrap());
    }
    let rel: DeclaredRelations = rel(chars.len(), pairs.iter().map(|&(x, y)| (at(x), at(y))).collect());
    let d = Order::new(&rel, steps).unwrap().degrees();
    // The current locus is b; it stood there after each earlier "ab".
    assert_eq!(d.iter().map(|x| x.point).collect::<Vec<_>>(), vec![1, 5, 9, 13], "four revolutions, four degrees");
    assert!(d.iter().all(|x| pairs[x.leaving] == ('b', 'c')), "each offers b→c, and each stays its own degree");
    let spans: Vec<usize> = d.iter().map(|x| x.alongside.1 - x.alongside.0).collect();
    assert_eq!(spans, vec![1, 5, 9, 13], "each runs alongside to the record boundary, by its own length");
}
