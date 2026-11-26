//! Kernel V10.3: I-SC-D15, scale invariance stated whole (Proposed, Generator,
//! 2026-11-23; V10_README §2.6).
//!
//! A scale multiplies each (component, edge) entry of a field by a factor
//! λ ≠ 0, value and repeatability bound together (δ is declared in the
//! registered unit): A′ = λ·A, δA′ = |λ|·δA. Which readings it leaves
//! unchanged depends on the factor's pattern:
//!
//! | kind | factor            |
//! |------|-------------------|
//! | C    | per component λ_c |
//! | E    | per edge λ_e      |
//! | CE   | λ_c·λ_e           |
//! | X    | per entry λ_ce    |
//!
//! Each kind is either the same at every registration (run) or may differ
//! between registrations (reg). The statement is `invariant` below. Every
//! cell marked invariant is checked to hold on every enumerated field. Every cell
//! marked not invariant must be *exercised*: some enumerated field must change
//! its classification. So the test fails if a cell is misstated either way.
//!
//! The same mechanism also settles the Z1a gain bounds for K7 turn sums
//! (part (v)). It found that a declared per-edge or per-entry gain did not
//! reach `turn_sum` or `turn_change_sum`.
//!
//! [IMPL] SYNTHETIC FIXTURES: values and factors are chosen to exercise the
//! forms, and are dyadic so every product is exact. They establish that the
//! forms compute as stated, nothing empirical.

#[path = "support/synthetic_feedback.rs"]
mod synthetic_feedback;
#[path = "support/synthetic_identity.rs"]
mod synthetic_identity;
use abr_kernel::ground_variables::*;
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::persistence_change::*;
use abr_kernel::topology::{turn_basis, TurnBasis};
use std::sync::Mutex;

fn rel(n: usize, edges: Vec<(usize, usize)>) -> DeclaredRelations {
    let m = edges.len();
    synthetic_feedback::declare_loops(synthetic_identity::identify(
        DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m])))
}

// ── Self-contained generator (SplitMix64), as in v10_properties ────────────

const CASES: u64 = 300;

struct Seeds(u64);
impl Seeds {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn int(&mut self, lo: i64, hi: i64) -> i64 { lo + (self.next() % (hi - lo + 1) as u64) as i64 }
    fn usize(&mut self, lo: usize, hi: usize) -> usize { self.int(lo as i64, hi as i64) as usize }
    fn eighth(&mut self, lo: f64, hi: f64) -> f64 { self.int((lo * 8.0) as i64, (hi * 8.0) as i64) as f64 / 8.0 }
    fn delta(&mut self, hi: f64) -> f64 { self.int(0, (hi * 16.0) as i64) as f64 / 16.0 }
    fn selects(&mut self, p_in_8: u64) -> bool { self.next() % 8 < p_in_8 }
}

fn for_all(name: &str, prop: impl Fn(&mut Seeds)) {
    for seed in 0..CASES {
        let mut r = Seeds(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ 0xD15);
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| prop(&mut r)));
        if caught.is_err() { panic!("property {name} failed at seed {seed}"); }
    }
}

/// An enumerated field with turns: distinct directed edges, no self-relation.
fn enumerated_rel(r: &mut Seeds) -> DeclaredRelations {
    let n = r.usize(3, 5);
    let m = r.usize(n, 8.min(n * (n - 1)));
    let mut edges: Vec<(usize, usize)> = Vec::new();
    while edges.len() < m {
        let (s, t) = (r.usize(0, n - 1), r.usize(0, n - 1));
        if s != t && !edges.contains(&(s, t)) { edges.push((s, t)); }
    }
    rel(n, edges)
}

/// A spanning forest stated by the test (the kernel never chooses one):
/// each edge in declared order that joins two parts.
fn stated_basis(rel: &DeclaredRelations) -> TurnBasis {
    let mut part: Vec<usize> = (0..rel.n_nodes).collect();
    fn root(p: &mut [usize], v: usize) -> usize { let mut v = v; while p[v] != v { v = p[v]; } v }
    let mut tree = Vec::new();
    for (e, &(s, t)) in rel.edges.iter().enumerate() {
        let (a, b) = (root(&mut part, s), root(&mut part, t));
        if a != b { part[a] = b; tree.push(e); }
    }
    turn_basis(rel, &tree).expect("a spanning forest of the declared field")
}

/// Edge values [t][c][e] and repeatability bounds, two or three components.
fn enumerated_edge_values(r: &mut Seeds, m: usize) -> (Vec<Vec<Vec<f64>>>, Vec<Vec<Vec<f64>>>) {
    let (k, regs) = (r.usize(2, 3), r.usize(3, 4));
    let mut v: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| r.eighth(-4.0, 4.0)).collect()).collect();
    let d: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| r.delta(0.25)).collect()).collect();
    let (mut aa, mut dd) = (Vec::new(), Vec::new());
    for _ in 0..regs {
        aa.push(v.clone()); dd.push(d.clone());
        for c in v.iter_mut() { for x in c.iter_mut() { if r.selects(5) { *x += r.eighth(-2.0, 2.0); } } }
    }
    (aa, dd)
}

// ═══════════════════════════════════════════════════════════════════════════
// The statement, (i)–(iii)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind { C, E, CE, X }
#[derive(Clone, Copy, Debug, PartialEq)]
enum Over { Run, Reg }
#[derive(Clone, Copy, Debug, PartialEq)]
enum Form { Pc1, Pc2a, Pc2b, Pc3, TurnSum, TurnChangeSum }

const KINDS: [Kind; 4] = [Kind::C, Kind::E, Kind::CE, Kind::X];
const OVERS: [Over; 2] = [Over::Run, Over::Reg];
const FORMS: [Form; 6] = [Form::Pc1, Form::Pc2a, Form::Pc2b, Form::Pc3, Form::TurnSum, Form::TurnChangeSum];

/// I-SC-D15 for edge fields: true where every scale of that kind leaves the
/// form's classifications unchanged.
///
/// One rule decides each cell. Every form is a sum of terms, each a product
/// of entries, and its half-width is built from the same products. A scale
/// multiplies the form and its half-width by one factor exactly when it
/// gives every term the same factor. Otherwise some field changes.
/// - PC1 and PC2(b) have one entry, or one pair of entries, per term, at two
///   registrations. So any kind passes over the run, and none passes between
///   registrations (Δ′ = λ_{s+1}·A_{s+1} − λ_s·A_s).
/// - PC2(a) and PC3 multiply entries of two components and two edges, so
///   they need λ_ie·λ_jf = λ_je·λ_if: C, E and CE pass, X does not. PC3
///   compares the classification of two states, each read in its own
///   registration, so it also passes per registration.
/// - A turn sum sums entries of one component across edges, so it needs one
///   factor per component: C only. The state turn sum passes per
///   registration; the change turn sum does not.
fn invariant(form: Form, kind: Kind, over: Over) -> bool {
    use Form::*;
    match (form, over) {
        (Pc1 | Pc2b, Over::Run) => true,
        (Pc2a, Over::Run) => kind != Kind::X,
        (Pc3, _) => kind != Kind::X,
        (TurnSum, _) => kind == Kind::C,
        (TurnChangeSum, Over::Run) => kind == Kind::C,
        (_, Over::Reg) => false,
    }
}

const FACTORS: [f64; 6] = [0.5, 0.75, 1.0, 1.125, 1.25, 2.0];

fn draw(r: &mut Seeds, signed: bool) -> f64 {
    let v = FACTORS[r.usize(0, FACTORS.len() - 1)];
    if signed && r.selects(3) { -v } else { v }
}

/// Factors [t][c][e] of one kind. A negative factor is a declared mapping
/// (W14), drawn only over the run: its sign is the same at every registration.
fn factors(r: &mut Seeds, kind: Kind, over: Over, regs: usize, k: usize, m: usize, signed: bool) -> Vec<Vec<Vec<f64>>> {
    let one = |r: &mut Seeds| -> Vec<Vec<f64>> {
        let fc: Vec<f64> = (0..k).map(|_| draw(r, signed)).collect();
        let fe: Vec<f64> = (0..m).map(|_| draw(r, signed)).collect();
        let mut f = vec![vec![0.0; m]; k];
        for c in 0..k {
            for e in 0..m {
                f[c][e] = match kind { Kind::C => fc[c], Kind::E => fe[e], Kind::CE => fc[c] * fe[e], Kind::X => draw(r, signed) };
            }
        }
        f
    };
    match over {
        Over::Run => { let f = one(r); vec![f; regs] }
        Over::Reg => (0..regs).map(|_| one(r)).collect(),
    }
}

fn scaled(a: &[Vec<Vec<f64>>], d: &[Vec<Vec<f64>>], f: &[Vec<Vec<f64>>]) -> EdgeSeries {
    let mul = |x: &[Vec<Vec<f64>>], abs: bool| -> Vec<Vec<Vec<f64>>> {
        x.iter().zip(f).map(|(xt, ft)| xt.iter().zip(ft).map(|(xc, fc)| xc.iter().zip(fc)
            .map(|(v, l)| v * if abs { l.abs() } else { *l }).collect()).collect()).collect()
    };
    EdgeSeries::from_edge_values_synthetic(mul(a, false), mul(d, true)).unwrap()
}

/// The classifications of one form, in a fixed order. Signs are not part of
/// a classification (they reverse under a negative factor; part (iv)).
fn classifications(s: &EdgeSeries, rel: &DeclaredRelations, basis: &TurnBasis, form: Form) -> String {
    let holds = |(v, h): (f64, f64)| v.abs() <= h;
    match form {
        Form::Pc1 => format!("{:?}", relation_changes(s).iter().map(|x| (x.step, x.component, x.edge, x.reading)).collect::<Vec<_>>()),
        Form::Pc2a => format!("{:?}", coupling_across_components(s).map(|c| c.readings.iter().map(|x| (x.e, x.f, x.over, x.reading)).collect::<Vec<_>>())),
        Form::Pc2b => format!("{:?}", coupling_across_steps(s).map(|c| c.readings.iter().map(|x| (x.e, x.f, x.over, x.reading)).collect::<Vec<_>>())),
        Form::Pc3 => format!("{:?}", arrangement_steps(s).map(|xs| xs.iter().map(|x| (x.step, x.e, x.f, x.i, x.j, x.reading)).collect::<Vec<_>>())),
        Form::TurnSum => format!("{:?}", (0..s.n_registrations()).flat_map(|t| (0..s.k()).flat_map(move |c|
            basis.turns.iter().map(move |cy| holds(turn_sum(s, rel, cy, t, c).unwrap())))).collect::<Vec<_>>()),
        Form::TurnChangeSum => format!("{:?}", (0..s.n_steps()).flat_map(|st| (0..s.k()).flat_map(move |c|
            basis.turns.iter().map(move |cy| holds(turn_change_sum(s, rel, cy, st, c).unwrap())))).collect::<Vec<_>>()),
    }
}

fn cell(form: Form, kind: Kind, over: Over) -> usize {
    FORMS.iter().position(|&x| x == form).unwrap() * 8
        + KINDS.iter().position(|&x| x == kind).unwrap() * 2
        + OVERS.iter().position(|&x| x == over).unwrap()
}

/// (i)–(iii) on edge-registered fields, positive factors. Each invariant cell
/// holds on every field; each other cell changes some field.
#[test]
fn prop_isc_d15_every_cell_of_the_scale_table() {
    let changed = Mutex::new(vec![0usize; 48]);
    let tried = Mutex::new(vec![0usize; 48]);
    for_all("I-SC-D15 table", |r| {
        let rel = enumerated_rel(r);
        let basis = stated_basis(&rel);
        let (a, d) = enumerated_edge_values(r, rel.n_edges());
        let (regs, k, m) = (a.len(), a[0].len(), a[0][0].len());
        let base = EdgeSeries::from_edge_values_synthetic(a.clone(), d.clone()).unwrap();
        for kind in KINDS {
            for over in OVERS {
                let f = factors(r, kind, over, regs, k, m, false);
                let s = scaled(&a, &d, &f);
                for form in FORMS {
                    let same = classifications(&base, &rel, &basis, form) == classifications(&s, &rel, &basis, form);
                    let i = cell(form, kind, over);
                    tried.lock().unwrap()[i] += 1;
                    if invariant(form, kind, over) {
                        assert!(same, "{form:?} changed under a {kind:?} scale over the {over:?}");
                    } else if !same {
                        changed.lock().unwrap()[i] += 1;
                    }
                }
            }
        }
    });
    let changed = changed.into_inner().unwrap();
    for form in FORMS { for kind in KINDS { for over in OVERS {
        if !invariant(form, kind, over) {
            assert!(changed[cell(form, kind, over)] > 0,
                "{form:?} under a {kind:?} scale over the {over:?} is stated not invariant, but no field changed");
        }
    } } }
}

/// (iv) A negative factor, the same at every registration (a declared
/// mapping, W14). The same cells hold: every classification is unchanged.
/// Values change sign with the factor: Δ′ = λ·Δ and δΔ′ = |λ|·δΔ exactly,
/// and a minor changes by λ_ie·λ_jf wherever the cell holds.
#[test]
fn prop_isc_d15_negative_factors_reverse_signs_only() {
    let flipped = Mutex::new(0usize);
    for_all("I-SC-D15 signed", |r| {
        let rel = enumerated_rel(r);
        let basis = stated_basis(&rel);
        let (a, d) = enumerated_edge_values(r, rel.n_edges());
        let (regs, k, m) = (a.len(), a[0].len(), a[0][0].len());
        let base = EdgeSeries::from_edge_values_synthetic(a.clone(), d.clone()).unwrap();
        for kind in KINDS {
            let f = factors(r, kind, Over::Run, regs, k, m, true);
            let s = scaled(&a, &d, &f);
            for form in FORMS {
                if invariant(form, kind, Over::Run) {
                    assert_eq!(classifications(&base, &rel, &basis, form), classifications(&s, &rel, &basis, form),
                        "{form:?} changed under a signed {kind:?} scale");
                }
            }
            for (x, y) in relation_changes(&base).iter().zip(relation_changes(&s).iter()) {
                let l = f[0][x.component][x.edge];
                assert_eq!(y.change, l * x.change);
                assert_eq!(y.half_width, l.abs() * x.half_width);
                if l < 0.0 && x.reading == Reading::Freedom { *flipped.lock().unwrap() += 1; }
            }
            if kind != Kind::X {
                if let (Ok(cb), Ok(cs)) = (coupling_across_components(&base), coupling_across_components(&s)) {
                    for (x, y) in cb.readings.iter().zip(&cs.readings) {
                        let ReadingPair::Components { i, j, .. } = x.over else { unreachable!() };
                        assert_eq!(y.minor, f[0][i][x.e] * f[0][j][x.f] * x.minor);
                    }
                }
            }
        }
    });
    assert!(flipped.into_inner().unwrap() > 0, "no resolved change was reversed: the signed generator is vacuous");
}

/// (ii) on node-derived fields. A node-derived field admits one scale on its
/// relations: a factor per component, shared by every locus (C). A factor per
/// locus is not a scale of A: λ_s·x_s − λ_t·x_t is not λ·(x_s − x_t). It is
/// Z1a's unshared gain (iii), which enters the bounds. The connection grades
/// (identity, proportional, sustained) read states registration by
/// registration, so a per-component factor leaves them unchanged over the run
/// or per registration. The coupled-change grade reads change (PC2), so it
/// holds over the run only.
#[test]
fn prop_isc_d15_node_derived_grades_under_component_scales() {
    let (coupled_moved, locus_moved) = (Mutex::new(0usize), Mutex::new(0usize));
    for_all("I-SC-D15 node grades", |r| {
        let rel = enumerated_rel(r);
        let (k, regs, n) = (r.usize(2, 3), r.usize(3, 4), rel.n_nodes);
        let mut x: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| r.eighth(-4.0, 4.0)).collect()).collect();
        let delta: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| r.delta(0.25)).collect()).collect();
        let mut xs = Vec::new();
        for _ in 0..regs {
            xs.push(x.clone());
            for c in x.iter_mut() { for v in c.iter_mut() { if r.selects(5) { *v += r.eighth(-2.0, 2.0); } } }
        }
        let build = |f: &dyn Fn(usize, usize, usize) -> f64| -> Vec<Registration> {
            xs.iter().enumerate().map(|(t, xt)| Registration::synthetic(
                NodeField::new((0..k).map(|c| (0..n).map(|i| f(t, c, i) * xt[c][i]).collect()).collect()),
                (0..k).map(|c| (0..n).map(|i| f(t, c, i).abs() * delta[c][i]).collect()).collect())).collect()
        };
        let grades = |seq: &[Registration]| -> (String, String) {
            let c = connection(seq, &rel).unwrap();
            let held = c.sustained.as_ref().map(|s| (s.held_edges.clone(), s.reorganizing_edges.clone(), s.not_formable.clone()));
            (format!("{:?}{:?}{:?}", c.identity, c.proportional, held), format!("{:?}", c.coupled_change))
        };
        let base = grades(&build(&|_, _, _| 1.0));
        // Over the run, signs allowed.
        let fc: Vec<f64> = (0..k).map(|_| draw(r, true)).collect();
        assert_eq!(grades(&build(&|_, c, _| fc[c])), base, "a per-component scale over the run changed a grade");
        // Per registration, positive: the state grades hold; coupled change may not.
        let ft: Vec<Vec<f64>> = (0..regs).map(|_| (0..k).map(|_| draw(r, false)).collect()).collect();
        let g = grades(&build(&|t, c, _| ft[t][c]));
        assert_eq!(g.0, base.0, "a per-component scale per registration changed a state grade");
        if g.1 != base.1 { *coupled_moved.lock().unwrap() += 1; }
        // A factor per locus is not a scale on the relations.
        let fl: Vec<Vec<f64>> = (0..k).map(|_| (0..n).map(|_| draw(r, false)).collect()).collect();
        let pc1 = |seq: &[Registration]| format!("{:?}", relation_changes(&EdgeSeries::from_registrations(seq, &rel).unwrap())
            .iter().map(|x| x.reading).collect::<Vec<_>>());
        if pc1(&build(&|_, c, i| fl[c][i])) != pc1(&build(&|_, _, _| 1.0)) { *locus_moved.lock().unwrap() += 1; }
    });
    assert!(coupled_moved.into_inner().unwrap() > 0, "coupled change never moved under per-registration factors");
    assert!(locus_moved.into_inner().unwrap() > 0, "per-locus factors never moved PC1");
}

// ── Worked counterexamples, one per kind of failure ────────────────────────

/// Triangle 0 → 1 → 2 with 0 → 2; the turn e0, e1, then e2 against its
/// direction: turn sum A[e0] + A[e1] − A[e2].
fn triangle() -> (DeclaredRelations, Vec<(usize, i8)>) {
    (rel(3, vec![(0, 1), (1, 2), (0, 2)]), vec![(0, 1), (1, 1), (2, -1)])
}

#[test]
fn isc_d15_worked_counterexamples() {
    let h = 0.0625;
    // PC1, a factor that differs between registrations: A = 1 at both, read
    // as structure; ×1 then ×1.25 gives Δ′ = 0.25 > δΔ′ = 0.140625.
    let s = EdgeSeries::from_edge_values_synthetic(vec![vec![vec![1.0]], vec![vec![1.0]]], vec![vec![vec![h]]; 2]).unwrap();
    assert_eq!(relation_changes(&s)[0].reading, Reading::Structure);
    let s = EdgeSeries::from_edge_values_synthetic(vec![vec![vec![1.0]], vec![vec![1.25]]], vec![vec![vec![h]], vec![vec![1.25 * h]]]).unwrap();
    assert_eq!(relation_changes(&s)[0].reading, Reading::Freedom);
    // A turn sum, a factor per edge: (1, 1, 2) sums to 0; ×1.25 on e2 gives
    // −0.5 against a half-width of 0.1875 + 0.0625·0.25 = 0.203125.
    let (r, cy) = triangle();
    let d = |f2: f64| vec![vec![vec![h, h, f2 * h]]; 2];
    let s = EdgeSeries::from_edge_values_synthetic(vec![vec![vec![1.0, 1.0, 2.0]]; 2], d(1.0)).unwrap();
    assert_eq!(turn_sum(&s, &r, &cy, 0, 0).unwrap(), (0.0, 0.1875));
    let s = EdgeSeries::from_edge_values_synthetic(vec![vec![vec![1.0, 1.0, 2.5]]; 2], d(1.25)).unwrap();
    assert_eq!(turn_sum(&s, &r, &cy, 0, 0).unwrap(), (-0.5, 0.203125));
    // The same turn sum under one factor for the component: still holds.
    let s = EdgeSeries::from_edge_values_synthetic(vec![vec![vec![1.25, 1.25, 2.5]]; 2], d(1.25).iter()
        .map(|t| t.iter().map(|c| c.iter().map(|_| 1.25 * h).collect()).collect()).collect()).unwrap();
    let (v, w) = turn_sum(&s, &r, &cy, 0, 0).unwrap();
    assert!(v.abs() <= w);
}

// ═══════════════════════════════════════════════════════════════════════════
// (v) What follows for declared gains (Z1a): turn sums
// ═══════════════════════════════════════════════════════════════════════════
//
// A gain is a positive factor 1 + γ′, |γ′| ≤ γ, the same over the run. By the
// table, a gain of a kind that a form's cell passes changes none of that
// form's classifications. A gain of any other kind must enter that form's
// bound. In 10.3.0 the edge-registered gains met this for PC1, PC2 and PC3,
// but not for K7 turn sums. A gain per edge (`EdgeGains::edge`) or per entry
// (`EdgeGains::independent`) differs between the edges of a turn sum, so it
// must enter the turn sum bound. A gain per component does not.

fn gains(component: Vec<f64>, edge: Vec<f64>, independent: Vec<Vec<f64>>) -> EdgeGains {
    EdgeGains { component, edge, independent }
}

#[test]
fn z1a_a_gain_per_edge_enters_the_closure_bound() {
    let (r, cy) = triangle();
    let zero = || vec![vec![vec![0.0; 3]]; 2];
    let no_offset = vec![vec![0.0; 3]];
    // Registered (1, 1, 2), then (2, 1, 3): turn sums 0 and 0, change turn sum 0.
    let a = |f2: f64| vec![vec![vec![1.0, 1.0, 2.0 * f2]], vec![vec![2.0, 1.0, 3.0 * f2]]];
    // The model permits a factor 1.125 on e2: turn sums −0.25 and −0.375, change −0.125.
    let truth = EdgeSeries::from_edge_values_synthetic(a(1.125), zero()).unwrap();
    assert_eq!(turn_sum(&truth, &r, &cy, 0, 0).unwrap(), (-0.25, 0.0));
    assert_eq!(turn_change_sum(&truth, &r, &cy, 0, 0).unwrap(), (-0.125, 0.0));
    // Declared as a gain of 1/8 on e2, both the registered values and the
    // values the model permits hold turn sum.
    let g = || gains(vec![0.0], vec![0.0, 0.0, 0.125], vec![vec![0.0; 3]]);
    for f2 in [1.0, 1.125] {
        let s = EdgeSeries::from_edge_values_synthetic(a(f2), zero()).unwrap().with_run_constant(no_offset.clone(), g());
        for t in 0..2 {
            let (v, h) = turn_sum(&s, &r, &cy, t, 0).unwrap();
            assert!(v.abs() <= h, "turn sum at {t} under factor {f2}: {v} against {h}");
        }
        let (v, h) = turn_change_sum(&s, &r, &cy, 0, 0).unwrap();
        assert!(v.abs() <= h, "change turn sum under factor {f2}: {v} against {h}");
    }
    // Registered values: h = 0.125·(|2| + 0) = 0.25, and 0.125·|1| on the change.
    let s = EdgeSeries::from_edge_values_synthetic(a(1.0), zero()).unwrap().with_run_constant(no_offset.clone(), g());
    assert_eq!(turn_sum(&s, &r, &cy, 0, 0).unwrap(), (0.0, 0.25));
    assert_eq!(turn_change_sum(&s, &r, &cy, 0, 0).unwrap(), (0.0, 0.125));
    // The same gain declared independent (on the entry) widens the same way.
    let gi = gains(vec![0.0], vec![0.0; 3], vec![vec![0.0, 0.0, 0.125]]);
    let s = EdgeSeries::from_edge_values_synthetic(a(1.0), zero()).unwrap().with_run_constant(no_offset.clone(), gi);
    assert_eq!(turn_sum(&s, &r, &cy, 0, 0).unwrap(), (0.0, 0.25));
    // A gain per component scales the whole turn sum: it does not enter.
    let gc = gains(vec![0.125], vec![0.0; 3], vec![vec![0.0; 3]]);
    let s = EdgeSeries::from_edge_values_synthetic(a(1.0), zero()).unwrap().with_run_constant(no_offset, gc);
    assert_eq!(turn_sum(&s, &r, &cy, 0, 0).unwrap(), (0.0, 0.0));
    assert_eq!(turn_change_sum(&s, &r, &cy, 0, 0).unwrap(), (0.0, 0.0));
}

/// Over enumerated fields: the values the model permits are
/// (1 + γ′_c)(1 + γ′_e)(1 + γ′_ce)·(v + ε), each γ′ within its declared bound
/// and fixed over the run, and |ε| within the repeatability bound at each
/// registration. Every turn sum the declared series reads as resolved has the
/// same sign on those values, in state and in change. Without the declaration
/// some resolved turn sums do not; this fails under the 10.3.0 turn sum bound.
#[test]
fn prop_z1a_declared_gains_never_reverse_a_resolved_closure() {
    let (resolved, would_flip) = (Mutex::new(0usize), Mutex::new(0usize));
    for_all("Z1a turn sums", |r| {
        let rel = enumerated_rel(r);
        let basis = stated_basis(&rel);
        let (a, d) = enumerated_edge_values(r, rel.n_edges());
        let (regs, k, m) = (a.len(), a[0].len(), a[0][0].len());
        let pick = |r: &mut Seeds| [0.0, 0.125, 0.25][r.usize(0, 2)];
        let gc: Vec<f64> = (0..k).map(|_| pick(r)).collect();
        let ge: Vec<f64> = (0..m).map(|_| pick(r)).collect();
        let gi: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| pick(r)).collect()).collect();
        // γ′ a multiple of γ/4 in [−γ, γ]; ε a multiple of h/4 in [−h, h].
        let prime = |r: &mut Seeds, g: f64| 1.0 + g * (r.usize(0, 8) as f64 - 4.0) / 4.0;
        let fc: Vec<f64> = gc.iter().map(|&g| prime(r, g)).collect();
        let fe: Vec<f64> = ge.iter().map(|&g| prime(r, g)).collect();
        let fi: Vec<Vec<f64>> = gi.iter().map(|gr| gr.iter().map(|&g| prime(r, g)).collect()).collect();
        let truth: Vec<Vec<Vec<f64>>> = (0..regs).map(|t| (0..k).map(|c| (0..m).map(|e| {
            let eps = d[t][c][e] * (r.usize(0, 8) as f64 - 4.0) / 4.0;
            fc[c] * fe[e] * fi[c][e] * (a[t][c][e] + eps)
        }).collect()).collect()).collect();
        let truth = EdgeSeries::from_edge_values_synthetic(truth, vec![vec![vec![0.0; m]; k]; regs]).unwrap();
        let declared = EdgeSeries::from_edge_values_synthetic(a.clone(), d.clone()).unwrap()
            .with_run_constant(vec![vec![0.0; m]; k], gains(gc.clone(), ge.clone(), gi.clone()));
        let undeclared = EdgeSeries::from_edge_values_synthetic(a.clone(), d.clone()).unwrap();
        let same_sign = |x: f64, y: f64| x * y > 0.0;
        for cy in &basis.turns {
            for c in 0..k {
                let check = |read: &dyn Fn(&EdgeSeries) -> (f64, f64)| {
                    let (tv, _) = read(&truth);
                    let (v, h) = read(&declared);
                    if v.abs() > h {
                        *resolved.lock().unwrap() += 1;
                        assert!(same_sign(v, tv), "a resolved turn sum {v} ± {h} reads {tv} on permitted values");
                    }
                    let (u, hu) = read(&undeclared);
                    if u.abs() > hu && !same_sign(u, tv) { *would_flip.lock().unwrap() += 1; }
                };
                for t in 0..regs { check(&|s| turn_sum(s, &rel, cy, t, c).unwrap()); }
                for st in 0..regs - 1 { check(&|s| turn_change_sum(s, &rel, cy, st, c).unwrap()); }
            }
        }
    });
    assert!(resolved.into_inner().unwrap() > 0, "no turn sum was resolved under the declared gains");
    assert!(would_flip.into_inner().unwrap() > 0, "undeclared gains never reversed a turn sum: the property does not test the widening");
}

// ═══════════════════════════════════════════════════════════════════════════
// I-SC-D10 qualified: an offset enters the state turn sums, not the change
// ═══════════════════════════════════════════════════════════════════════════

/// I-SC-D10's implication (turn sum at δ at both registrations ⇒ turn sum on
/// the change) was derived before Z1a, with δ as repeatability alone. With an
/// offset declared, the state bound carries the offset and the change bound
/// does not, so the implication can fail, correctly. Turn sums +0.125 and
/// −0.125 each fit an offset bound of 3 × 1/16, but no one offset fits both,
/// and the change turn sum −0.25 is resolved at zero repeatability.
#[test]
fn isc_d10_with_an_offset_the_change_can_resolve_while_both_states_hold() {
    let (r, cy) = triangle();
    let s = EdgeSeries::from_edge_values_synthetic(vec![vec![vec![1.0, 1.0, 1.875]], vec![vec![1.0, 1.0, 2.125]]], vec![vec![vec![0.0; 3]]; 2])
        .unwrap().with_run_constant(vec![vec![0.0625; 3]], gains(vec![0.0], vec![0.0; 3], vec![vec![0.0; 3]]));
    assert_eq!(turn_sum(&s, &r, &cy, 0, 0).unwrap(), (0.125, 0.1875));
    assert_eq!(turn_sum(&s, &r, &cy, 1, 0).unwrap(), (-0.125, 0.1875));
    assert_eq!(turn_change_sum(&s, &r, &cy, 0, 0).unwrap(), (-0.25, 0.0));
}

// ═══════════════════════════════════════════════════════════════════════════
// The Verifier's probes on 28f39d8 / a0bf7ec (2026-11-23, 39th)
// ═══════════════════════════════════════════════════════════════════════════

/// A small declared gain is not lost. With a gain of 2⁻⁵⁴ on an edge of
/// magnitude 2⁵⁴ the bound must grow by 1. The product form
/// (1 + γ_e)(1 + γ_ce) − 1 rounded 1 + 2⁻⁵⁴ to 1 and returned −0.5 ± 0.
#[test]
fn z1a_a_small_gain_is_not_rounded_away() {
    let tiny = 2f64.powi(-54);
    let big = 2f64.powi(54);
    assert_eq!(1.0 + tiny, 1.0, "the probe depends on 1 + 2⁻⁵⁴ rounding to 1");
    let (r, _) = triangle();
    // Walk from locus 2: e2 against, e0, e1: −A[e2] + A[e0] + A[e1].
    let walk = vec![(2, -1), (0, 1), (1, 1)];
    let a = vec![vec![vec![big, -0.5, big]]; 2];
    let zero = vec![vec![vec![0.0; 3]]; 2];
    // Declared on the edge, and declared on the entry: both give −0.5 ± 1.
    for (edge, independent) in [(vec![0.0, 0.0, tiny], vec![vec![0.0; 3]]), (vec![0.0; 3], vec![vec![0.0, 0.0, tiny]])] {
        let s = EdgeSeries::from_edge_values_synthetic(a.clone(), zero.clone()).unwrap()
            .with_run_constant(vec![vec![0.0; 3]], gains(vec![0.0], edge, independent));
        assert_eq!(turn_sum(&s, &r, &walk, 0, 0).unwrap(), (-0.5, 1.0));
    }
    // The same in a reported change magnitude, through the shared gain.
    let s = EdgeSeries::from_edge_values_synthetic(vec![vec![vec![0.0]], vec![vec![big]]], vec![vec![vec![0.0]]; 2]).unwrap()
        .with_run_constant(vec![vec![0.0]], gains(vec![tiny], vec![0.0], vec![vec![0.0]]));
    assert_eq!(s.shared_gain[0][0], tiny);
    assert_eq!(s.change_magnitude(0).1[0][0], 1.0);
    // The exact 2⁻⁵³ + 2⁻¹⁰⁸ is not an f64: a bound is rounded up to the next
    // f64 above 2⁻⁵³ (OC-NUM-1, 45th). *Was:* `2.0 * tiny + tiny * tiny`,
    // the nearest f64, 2⁻⁵³, which lies below the exact bound.
    assert_eq!(combined_gain(tiny, tiny), (2.0 * tiny).next_up());
}

/// An open walk is refused before any registration or step is read.
#[test]
fn k7_an_open_walk_is_none_whatever_the_registration() {
    let (r, _) = triangle();
    let s = EdgeSeries::from_edge_values_synthetic(vec![vec![vec![1.0, 1.0, 2.0]]; 2], vec![vec![vec![0.0; 3]]; 2]).unwrap();
    let open = vec![(0, 1), (1, 1)];
    assert_eq!(turn_sum(&s, &r, &open, 99, 0), None);
    assert_eq!(turn_change_sum(&s, &r, &open, 99, 0), None);
}
