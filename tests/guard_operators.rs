//! The guard: no reading enters the kernel unless the operators form it or
//! Origin has admitted it (Origin, 2026-11-26: "We need to get back to our
//! unit tests that make this impossible and figure out a way to run them
//! whenever you produce code").
//!
//! What went wrong three times (73rd to 89th, 90th, 102nd): a familiar method
//! was built in place of the operators and described in V10's vocabulary. The
//! derivation register (92nd, `tests/v10_derivation.rs`) asks only that a
//! part *declare* a derivation; a part could declare itself legacy, state its
//! necessity in its own words, and pass. `Level.continuations` did exactly
//! that ("No operator forms it"). This guard closes that door:
//!
//! 1. Every legacy part is admitted by Origin, by name, with the Part 9 entry
//!    that admits it, in `tests/guard/origin_admitted.rs`, changed only on
//!    Origin's word. A legacy part not listed fails, whatever its
//!    derivation says.
//! 2. A part whose derivation says no operator forms it is never a reading:
//!    it is listed as not admitted, or admitted only as a check.
//! 3. A public structure added after the guard has every field registered
//!    in `src/derivation.rs`, so a new reading cannot enter undeclared and
//!    then escape (1) and (2) by never being reported.
//! 4. No name in the kernel's code is a familiar method (suffix, n-gram,
//!    cosine, …) unless Origin has allowed it in that file.
//!
//! Each check is a function, and each is shown here to fail on a planted
//! violation (W: a test that cannot fail tells nothing).
//!
//! **What this cannot tell (named):** whether an `Operator` derivation is
//! true, i.e. that the code computes what its form states; that needs, for
//! each reading, a test that it is formed by the operators it names, on a
//! whole field (the guard to add, handoff 2026-11-26 §4). A familiar method
//! written under a new name passes (4); it is still caught by (1) or (3) if
//! it reports a part, and not at all if it is honestly labelled an operator
//! reading that is not one.
//!
//! The hooks that run this after every edit, and the full checks before the
//! Generator stops, are in `.claude/settings.json` and `.claude/hooks/`.
//! Beyond these tests there is no prevention (Origin, 2026-11-26): Origin
//! asks whether relation is treated as primitive, followed by calculating
//! freedom, connection and energy in linear recursive algebra, and the
//! Generator answers truthfully.

#[path = "guard/origin_admitted.rs"]
mod origin;

use abr_kernel::derivation::{legacy_in_use, of_part, register, Derivation};
use origin::*;

const README: &str = include_str!("../docs/kernel/V10_README.md");

/// Every source file of the kernel, as (path, text).
fn sources() -> Vec<(String, String)> {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/src");
    let mut out: Vec<(String, String)> = std::fs::read_dir(dir).unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "rs"))
        .map(|p| (format!("src/{}", p.file_name().unwrap().to_string_lossy()), std::fs::read_to_string(&p).unwrap()))
        .collect();
    out.sort();
    out
}

/// All the text a derivation states.
fn text(d: &Derivation) -> String {
    match d {
        Derivation::Registered { what } | Derivation::Identity { what } => what.to_string(),
        Derivation::Operator { form, .. } => form.to_string(),
        Derivation::Rule { rule, .. } => rule.to_string(),
        Derivation::Legacy { method, provenance, in_relation_to_operators, necessity } =>
            format!("{method} {provenance} {in_relation_to_operators} {}", necessity.unwrap_or("")),
    }
}

// ── The checks ───────────────────────────────────────────────────────────

/// (1) The legacy parts not admitted by Origin, and admissions that cite no
/// Part 9 entry of Origin's or name no part in the code.
fn unadmitted(
    legacy_parts: &[&str], legacy_methods: &[&str],
    admitted: &[(&str, &str, Use)], not_admitted: &[(&str, &str)], readme: &str,
) -> Vec<String> {
    let mut why = Vec::new();
    let held = |k: &str| admitted.iter().any(|a| a.0 == k) || not_admitted.iter().any(|a| a.0 == k);
    for p in legacy_parts {
        if !held(p) { why.push(format!("{p}: legacy, not admitted by Origin")); }
    }
    for m in legacy_methods {
        if !admitted.iter().any(|a| m.starts_with(a.0)) { why.push(format!("{m}: legacy method, not admitted by Origin")); }
    }
    for (k, entry, _) in admitted {
        if !legacy_parts.contains(k) && !legacy_methods.iter().any(|m| m.starts_with(k)) {
            why.push(format!("{k}: admitted, but no legacy part or method of that name is in the code"));
        }
        if !readme.contains(&format!("({entry}) |")) { why.push(format!("{k}: cites the {entry}, which Part 9 does not hold")); }
    }
    for (k, _) in not_admitted {
        if !legacy_parts.contains(k) { why.push(format!("{k}: listed as not admitted, but not in the code")); }
    }
    why
}

/// (2) The parts whose derivation says no operator forms them, standing as a
/// reading.
fn unformed_readings(parts: &[(&str, String)], admitted: &[(&str, &str, Use)], not_admitted: &[(&str, &str)]) -> Vec<String> {
    parts.iter()
        .filter(|(_, t)| t.to_lowercase().contains("no operator forms"))
        .filter(|(n, _)| !not_admitted.iter().any(|a| a.0 == *n) && !admitted.iter().any(|a| a.0 == *n && a.2 == Use::Check))
        .map(|(n, _)| format!("{n}: no operator forms it, and it stands as a reading"))
        .collect()
}

/// The code of a Rust source with comments and quoted text blanked, so only
/// names remain to be read.
fn code_only(src: &str) -> String {
    let b = src.as_bytes();
    let mut out = String::with_capacity(src.len());
    let mut i = 0;
    while i < b.len() {
        if b[i..].starts_with(b"//") {
            while i < b.len() && b[i] != b'\n' { i += 1; }
        } else if b[i..].starts_with(b"/*") {
            let mut depth = 0;
            while i < b.len() {
                if b[i..].starts_with(b"/*") { depth += 1; i += 2; }
                else if b[i..].starts_with(b"*/") { depth -= 1; i += 2; if depth == 0 { break; } }
                else { if b[i] == b'\n' { out.push('\n'); } i += 1; }
            }
        } else if b[i] == b'r' && (b.get(i + 1) == Some(&b'"') || b.get(i + 1) == Some(&b'#'))
            && (i == 0 || !(b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_')) {
            let mut j = i + 1;
            let mut hashes = 0;
            while b.get(j) == Some(&b'#') { hashes += 1; j += 1; }
            if b.get(j) != Some(&b'"') { out.push('r'); i += 1; continue; }
            let close: Vec<u8> = std::iter::once(b'"').chain(std::iter::repeat(b'#').take(hashes)).collect();
            j += 1;
            while j < b.len() && !b[j..].starts_with(&close) { j += 1; }
            out.push_str("\"\"");
            i = j + close.len();
        } else if b[i] == b'"' {
            i += 1;
            while i < b.len() && b[i] != b'"' { if b[i] == b'\\' { i += 1; } i += 1; }
            out.push_str("\"\"");
            i += 1;
        } else if b[i] == b'\'' && (b.get(i + 2) == Some(&b'\'') || b.get(i + 1) == Some(&b'\\')) {
            // A character literal ('x' or '\x'); a lifetime is left as it is.
            let mut j = i + 1;
            if b[j] == b'\\' { j += 2; while j < b.len() && b[j] != b'\'' { j += 1; } } else { j += 1; }
            out.push_str("' '");
            i = j + 1;
        } else {
            let ch = src[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    out
}

/// (4) The familiar methods named in the code of each file.
fn familiar_in(files: &[(String, String)], methods: &[&str], allowed: &[(&str, &str)]) -> Vec<String> {
    let mut why = Vec::new();
    for (path, src) in files {
        for (line, code) in code_only(src).lines().enumerate() {
            let lower = code.to_lowercase();
            for m in methods {
                if lower.contains(m) && !allowed.iter().any(|a| a.0 == path && a.1 == *m) {
                    why.push(format!("{path}:{}: names a familiar method ({m}): {}", line + 1, code.trim()));
                }
            }
        }
    }
    why
}

/// The public structures with named public fields, as (structure, fields).
fn public_structs(src: &str) -> Vec<(String, Vec<String>)> {
    let code = code_only(src);
    let mut out = Vec::new();
    let mut rest = code.as_str();
    while let Some(at) = rest.find("pub struct ") {
        rest = &rest[at + "pub struct ".len()..];
        let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
        let head_end = rest.find(|c| c == '{' || c == ';' || c == '(').unwrap_or(rest.len());
        if rest.as_bytes().get(head_end) != Some(&b'{') { continue; }
        let body = &rest[head_end + 1..];
        let mut depth = 1;
        let mut end = 0;
        for (k, c) in body.char_indices() {
            match c { '{' => depth += 1, '}' => { depth -= 1; if depth == 0 { end = k; break; } } _ => {} }
        }
        let fields = body[..end].lines()
            .filter_map(|l| l.trim().strip_prefix("pub "))
            .filter_map(|l| l.split_once(':').map(|(f, _)| f.trim().to_string()))
            .filter(|f| f.chars().all(|c| c.is_alphanumeric() || c == '_'))
            .collect::<Vec<_>>();
        if !fields.is_empty() { out.push((name, fields)); }
    }
    out
}

/// (3) The fields of public structures added after the guard that have no
/// derivation.
fn undeclared_fields(files: &[(String, String)], before: &[&str], derived: &dyn Fn(&str) -> bool) -> Vec<String> {
    let mut why = Vec::new();
    for (path, src) in files {
        for (s, fields) in public_structs(src) {
            if before.contains(&s.as_str()) { continue; }
            for f in fields {
                let n = format!("{s}.{f}");
                if !derived(&n) { why.push(format!("{path}: {n} has no derivation in src/derivation.rs")); }
            }
        }
    }
    why
}

// ── The checks on the kernel ─────────────────────────────────────────────

fn legacy_parts() -> Vec<&'static str> {
    register().iter().filter(|(_, d)| matches!(d, Derivation::Legacy { .. })).map(|(n, _)| *n).collect()
}

fn legacy_methods() -> Vec<&'static str> {
    legacy_in_use().iter().filter_map(|d| match d { Derivation::Legacy { method, .. } => Some(*method), _ => None }).collect()
}

/// 1. Every legacy part and method in the kernel is admitted by Origin, by
/// name, citing a Part 9 entry; nothing admitted is missing from the code.
#[test]
fn every_legacy_part_is_admitted_by_origin() {
    let why = unadmitted(&legacy_parts(), &legacy_methods(), ADMITTED_LEGACY, NOT_ADMITTED, README);
    assert!(why.is_empty(), "\n{}\n", why.join("\n"));
    for (n, waits) in NOT_ADMITTED {
        eprintln!("[guard] NOT ADMITTED, a named gap: {n} (waits on {waits})");
    }
}

/// 2. Nothing no operator forms stands as a reading.
#[test]
fn nothing_no_operator_forms_stands_as_a_reading() {
    // Readings only: a registration or an identity is an input, never formed
    // by an operator, and says so truthfully (2026-11-26: `Degree.leaving`).
    // *Was:* every register entry.
    let parts: Vec<(&str, String)> = register().iter()
        .filter(|(_, d)| !matches!(d, Derivation::Registered { .. } | Derivation::Identity { .. }))
        .map(|(n, d)| (*n, text(d))).collect();
    let why = unformed_readings(&parts, ADMITTED_LEGACY, NOT_ADMITTED);
    assert!(why.is_empty(), "\n{}\n", why.join("\n"));
    // And no operator reading or rule says it is not one.
    for (n, d) in register() {
        if matches!(d, Derivation::Operator { .. } | Derivation::Rule { .. }) {
            assert!(!text(d).to_lowercase().contains("no operator forms"), "{n} is declared an operator reading or rule, and says no operator forms it");
        }
    }
}

/// 3. A public structure added after the guard registers every field.
#[test]
fn a_new_structure_declares_every_field() {
    let why = undeclared_fields(&sources(), STRUCTS_BEFORE_THE_GUARD, &|n| of_part(n).is_some());
    assert!(why.is_empty(), "\n{}\n", why.join("\n"));
    // The baseline names structures that exist (a renamed one would escape).
    let all: Vec<String> = sources().iter().flat_map(|(_, s)| public_structs(s).into_iter().map(|x| x.0)).collect();
    for s in STRUCTS_BEFORE_THE_GUARD {
        assert!(all.iter().any(|a| a == s), "{s} is in the baseline but no longer in the code: Origin strikes it");
    }
}

/// 4. No name in the kernel's code is a familiar method.
#[test]
fn no_familiar_method_is_named_in_the_code() {
    let why = familiar_in(&sources(), FAMILIAR_METHODS, FAMILIAR_ALLOWED);
    assert!(why.is_empty(), "\n{}\n", why.join("\n"));
}

// ── Each check fails on a planted violation ──────────────────────────────

#[test]
fn check_1_fails_on_an_unadmitted_legacy_part() {
    let readme = "| 2026-11-26 (100th) |";
    assert!(unadmitted(&["Split.potential"], &[], &[("Split.potential", "100th", Use::Reading)], &[], readme).is_empty());
    let planted = unadmitted(&["Split.potential", "Order.next"], &[], &[("Split.potential", "100th", Use::Reading)], &[], readme);
    assert_eq!(planted.len(), 1, "{planted:?}");
    assert!(planted[0].starts_with("Order.next"));
    // An admission citing an entry Part 9 does not hold fails.
    assert_eq!(unadmitted(&["Split.potential"], &[], &[("Split.potential", "999th", Use::Reading)], &[], readme).len(), 1);
    // A legacy method in use, not admitted, fails.
    assert_eq!(unadmitted(&[], &["suffix arrays"], &[], &[], readme).len(), 1);
}

#[test]
fn check_2_fails_on_a_reading_no_operator_forms() {
    let parts = [("Level.next", "a suffix match. No operator forms it".to_string())];
    assert_eq!(unformed_readings(&parts, &[], &[]).len(), 1);
    assert_eq!(unformed_readings(&parts, &[("Level.next", "103rd", Use::Reading)], &[]).len(), 1, "admitted as a reading still fails");
    assert!(unformed_readings(&parts, &[("Level.next", "103rd", Use::Check)], &[]).is_empty(), "a check passes");
    assert!(unformed_readings(&parts, &[], &[("Level.next", "103rd")]).is_empty(), "a named gap passes");
}

#[test]
fn check_3_fails_on_a_new_structure_with_an_undeclared_field() {
    let src = "/// doc\npub struct Next {\n    /// a field\n    pub counts: Vec<(usize, usize)>,\n    pub route: Vec<usize>,\n}\npub struct Old { pub a: u8 }\n";
    let files = vec![("src/x.rs".to_string(), src.to_string())];
    let why = undeclared_fields(&files, &["Old"], &|n| n == "Next.route");
    assert_eq!(why, vec!["src/x.rs: Next.counts has no derivation in src/derivation.rs".to_string()]);
}

#[test]
fn check_4_fails_on_a_familiar_method_and_not_on_comments_or_text() {
    let src = "// a suffix index, in a comment\nlet s = \"cosine, in text\";\nlet r = r#\"markov\"#;\nfn suffix_match() {}\nlet c = '\"'; let ngram_counts = 1;\n";
    let files = vec![("src/x.rs".to_string(), src.to_string())];
    let why = familiar_in(&files, FAMILIAR_METHODS, &[]);
    assert_eq!(why.len(), 2, "{why:?}");
    assert!(why[0].starts_with("src/x.rs:4") && why[1].starts_with("src/x.rs:5"), "{why:?}");
    assert!(familiar_in(&files, FAMILIAR_METHODS, &[("src/x.rs", "suffix"), ("src/x.rs", "ngram")]).is_empty());
}
