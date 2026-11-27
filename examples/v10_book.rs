//! A whole book through the radial field (Origin uploaded *Pride and
//! Prejudice*, Project Gutenberg's edition, 2026-11-27).
//!
//! [IMPL] Actual text (111th (2)), read as given: every character a step,
//! the line endings (carriage return and line feed) and the Gutenberg header
//! and footer included, since they are the input. A capital is the capital
//! mark ⇧ and its letter (113th). The elements are declared from the text in
//! order of first appearance, each with its radial graph (109th).
//!
//! Every value is the kernel's (`radial::RadialField`). At every point:
//! whether the element is met for the first time; whether the character the
//! text took next was indicated (its spoke from the current element taken
//! before: exactly the indications' continuations, 112th) or not (new to
//! the field). Framework counts, not a rule. At the end: the indications at
//! the last point, with how far each runs alongside.
//!
//! Run: `cargo run --release --example v10_book -- <path>`

use std::time::Instant;

use abr_kernel::radial::RadialField;

const CAPITAL: char = '⇧';

fn main() {
    let path = std::env::args().nth(1).expect("the path of a text file");
    let raw = std::fs::read(&path).expect("the file reads");
    let text = String::from_utf8(raw).expect("UTF-8");
    let mut registered: Vec<char> = Vec::with_capacity(text.len());
    for c in text.chars() { if c.is_uppercase() { registered.push(CAPITAL); registered.extend(c.to_lowercase()); } else { registered.push(c); } }
    let mut chars: Vec<char> = Vec::new();
    for &c in &registered { if !chars.contains(&c) { chars.push(c); } }
    let n = chars.len();
    let index: std::collections::HashMap<char, usize> = chars.iter().enumerate().map(|(i, &c)| (c, i)).collect();

    let t0 = Instant::now();
    let mut r = RadialField::new(n);
    let (mut first, mut indicated, mut new) = (0usize, 0usize, 0usize);
    // The points at which the text has stood at each element so far.
    let mut stood = vec![0usize; n];
    let mut narrowed = vec![0usize; n + 1];
    for w in registered.windows(2) {
        let (x, y) = (index[&w[0]], index[&w[1]]);
        // Before the step: at the current element x, did any earlier point
        // stand here (an indication), and had the spoke x→y been taken?
        if stood[x] == 0 { first += 1 } else if r.a(x, y) > 0 { indicated += 1 } else { new += 1 }
        // How far the field narrows: the distinct continuations it indicates
        // at x (spokes from x taken before), out of the n elements.
        if stood[x] > 0 { let k = (0..n).filter(|&z| r.a(x, z) > 0).count(); narrowed[k] += 1; }
        stood[x] += 1;
        r.push(x, y).unwrap();
    }
    let t_read = t0.elapsed();

    // The readback in order spells the text back exactly.
    let t1 = Instant::now();
    let mut back: Vec<char> = vec![chars[r.step(0).0]];
    for p in 0..r.steps() { back.push(chars[r.step(p).1]); }
    let mut spelled = String::new();
    let mut cap = false;
    for c in back { if c == CAPITAL { cap = true; continue; } if cap { spelled.extend(c.to_uppercase()); cap = false; } else { spelled.push(c); } }
    let exact = spelled == text;
    let t_back = t1.elapsed();

    let show = |c: char| match c { ' ' => "␣".to_string(), '\n' => "⏎".to_string(), '\r' => "␍".to_string(), c => c.to_string() };
    println!("[IMPL] {path}");
    println!("{} characters as given; {} steps registered (capitals through ⇧); {n} elements; {} spokes declared, {} taken",
        text.chars().count(), r.steps(), n * n, (0..n).flat_map(|x| (0..n).map(move |y| (x, y))).filter(|&(x, y)| r.a(x, y) > 0).count());
    println!("read in {:.3} s; spelled back in {:.3} s: {}", t_read.as_secs_f64(), t_back.as_secs_f64(), if exact { "exactly the input" } else { "NOT the input" });
    let total = first + indicated + new;
    println!("\nAt every point (framework counts):");
    println!("  the element met for the first time:        {first:>8}");
    println!("  the next character was indicated:          {indicated:>8}  ({:.3}% of points)", 100.0 * indicated as f64 / total as f64);
    println!("  the next character was new to the field:   {new:>8}");
    let pts: usize = narrowed.iter().sum();
    let mean = narrowed.iter().enumerate().map(|(k, c)| k * c).sum::<usize>() as f64 / pts as f64;
    let mut acc = 0; let median = narrowed.iter().position(|&c| { acc += c; acc * 2 >= pts }).unwrap();
    println!("  continuations indicated, of {n} elements:  mean {mean:.1}, median {median}, at most {}", narrowed.iter().rposition(|&c| c > 0).unwrap());

    let t2 = Instant::now();
    let d = r.indications(r.steps());
    let t_ind = t2.elapsed();
    let here = r.step(r.steps() - 1).1;
    let mut best: Vec<_> = d.iter().map(|x| (x.alongside.1 - x.alongside.0, x.point, x.leaving % n)).collect();
    best.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    println!("\nAt the last point the text stands at {:?}: {} earlier points indicate (read in {:.3} s); the furthest alongside:", show(chars[here]), d.len(), t_ind.as_secs_f64());
    for (k, j, y) in best.iter().take(5) {
        let ctx: String = registered[j.saturating_sub(30)..*j + 1].iter().map(|&c| show(c)).collect();
        println!("  point {j:>7}: alongside {k:>5}, continues to {:?}   …{ctx}", show(chars[*y]));
    }
}
