//! Kernel V10.3: I-SC-D16, turn sum over a turn basis (Proposed, Generator,
//! 2026-11-23; V10_README §2.6). I-SC-D10 with K7.
//!
//! (i)   Exactly: every turn's turn sum, in state and on change, is the
//!       integer combination Σ n_i·(basis turn sum i), with n_i the walk's net
//!       count of chord i. It holds for any edge field, so for Δ as for A.
//! (ii)  At δ the basis does not settle every turn: all basis turn sums can
//!       hold while another turn is resolved. Which basis turn sums are
//!       resolved depends on the stated tree.
//! (iii) Every simple turn holds at δ exactly when some node field x puts
//!       every relation within its bound of x[s] − x[t]. So a resolved turn sum
//!       is a reading no node-derived field could give.
//! (iv)  A walk through a resolved basis turn sum's chord can still hold.
//!
//! [IMPL] SYNTHETIC FIXTURES: dyadic values, so every sum is exact. They
//! establish that the forms compute as stated, nothing empirical. The node
//! field in (iii) exists only inside the test, to check the equivalence; no
//! kernel reading constructs one.

#[path = "support/synthetic_feedback.rs"]
mod synthetic_feedback;
#[path = "support/synthetic_identity.rs"]
mod synthetic_identity;
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance};
use abr_kernel::persistence_change::*;
use abr_kernel::topology::{basis_turn_sums, turn_basis, express_in_basis, TurnBasis};
use std::collections::VecDeque;
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
        let mut r = Seeds(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ 0xD16);
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| prop(&mut r)));
        if caught.is_err() { panic!("property {name} failed at seed {seed}"); }
    }
}

fn enumerated_rel(r: &mut Seeds) -> DeclaredRelations {
    let n = r.usize(3, 5);
    let m = r.usize(n, 7.min(n * (n - 1)));
    let mut edges: Vec<(usize, usize)> = Vec::new();
    while edges.len() < m {
        let (s, t) = (r.usize(0, n - 1), r.usize(0, n - 1));
        if s != t && !edges.contains(&(s, t)) { edges.push((s, t)); }
    }
    rel(n, edges)
}

/// A spanning forest stated by the test, taking edges in the given order.
fn stated_basis(rel: &DeclaredRelations, order: &[usize]) -> TurnBasis {
    let mut part: Vec<usize> = (0..rel.n_nodes).collect();
    fn root(p: &[usize], v: usize) -> usize { let mut v = v; while p[v] != v { v = p[v]; } v }
    let mut tree = Vec::new();
    for &e in order {
        let (s, t) = rel.edges[e];
        let (a, b) = (root(&part, s), root(&part, t));
        if a != b { part[a] = b; tree.push(e); }
    }
    turn_basis(rel, &tree).expect("a spanning forest of the declared field")
}

fn enumerated_series(r: &mut Seeds, k: usize, m: usize) -> EdgeSeries {
    let regs = r.usize(2, 4);
    let mut v: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| r.eighth(-3.0, 3.0)).collect()).collect();
    let d: Vec<Vec<f64>> = (0..k).map(|_| (0..m).map(|_| r.delta(0.25)).collect()).collect();
    let (mut aa, mut dd) = (Vec::new(), Vec::new());
    for _ in 0..regs {
        aa.push(v.clone()); dd.push(d.clone());
        for c in v.iter_mut() { for x in c.iter_mut() { if r.selects(4) { *x += r.eighth(-1.0, 1.0); } } }
    }
    EdgeSeries::from_edge_values_synthetic(aa, dd).unwrap()
}

/// Undirected incidence: (neighbour, edge, sign of travelling it).
fn incidence(rel: &DeclaredRelations) -> Vec<Vec<(usize, usize, i8)>> {
    let mut adj = vec![Vec::new(); rel.n_nodes];
    for (e, &(s, t)) in rel.edges.iter().enumerate() {
        adj[s].push((t, e, 1));
        adj[t].push((s, e, -1));
    }
    adj
}

/// An enumerated turn: a seed-selected walk of up to `len` steps from a locus with
/// a relation, then the shortest way back. Edges may repeat.
fn enumerated_walk(r: &mut Seeds, rel: &DeclaredRelations, len: usize) -> Vec<(usize, i8)> {
    let adj = incidence(rel);
    let starts: Vec<usize> = (0..rel.n_nodes).filter(|&v| !adj[v].is_empty()).collect();
    let v0 = starts[r.usize(0, starts.len() - 1)];
    let (mut at, mut walk) = (v0, Vec::new());
    for _ in 0..r.usize(1, len) {
        let (w, e, sg) = adj[at][r.usize(0, adj[at].len() - 1)];
        walk.push((e, sg));
        at = w;
    }
    // Back to v0 by breadth-first search.
    let mut via: Vec<Option<(usize, usize, i8)>> = vec![None; rel.n_nodes];
    let mut seen = vec![false; rel.n_nodes];
    seen[at] = true;
    let mut q = VecDeque::from([at]);
    while let Some(v) = q.pop_front() {
        for &(w, e, sg) in &adj[v] { if !seen[w] { seen[w] = true; via[w] = Some((v, e, sg)); q.push_back(w); } }
    }
    let mut back = Vec::new();
    let mut u = v0;
    while u != at { let (v, e, sg) = via[u].unwrap(); back.push((e, sg)); u = v; }
    back.reverse();
    walk.extend(back);
    walk
}

/// Every simple turn (distinct loci, distinct edges), each in both directions.
fn simple_turns(rel: &DeclaredRelations) -> Vec<Vec<(usize, i8)>> {
    let adj = incidence(rel);
    let mut out = Vec::new();
    fn go(adj: &[Vec<(usize, usize, i8)>], u0: usize, at: usize, on: &mut Vec<usize>, path: &mut Vec<(usize, i8)>, out: &mut Vec<Vec<(usize, i8)>>) {
        for &(w, e, sg) in &adj[at] {
            if path.iter().any(|p| p.0 == e) { continue; }
            if w == u0 { let mut c = path.clone(); c.push((e, sg)); out.push(c); continue; }
            if w < u0 || on.contains(&w) { continue; }
            on.push(w); path.push((e, sg));
            go(adj, u0, w, on, path, out);
            on.pop(); path.pop();
        }
    }
    for u0 in 0..rel.n_nodes { go(&adj, u0, u0, &mut vec![u0], &mut Vec::new(), &mut out); }
    out
}

/// *Legacy mathematics (W12 as amended, 42nd; OC-LM-1, items 2 and 3):*
/// difference constraints solved by Bellman–Ford. Used here only to check
/// the cited intersection in I-SC-D16 (iii), "⇐"; it is no kernel reading.
///
/// Is there a node field x with |g[e] − (x[s] − x[t])| ≤ b[e] for every
/// relation? Difference constraints solved by Bellman–Ford from a virtual
/// source. Returns x when there is one, and checks it directly.
fn node_field_within(rel: &DeclaredRelations, g: &[f64], b: &[f64]) -> Option<Vec<f64>> {
    // x_s − x_t ≤ g + b  : arc t → s, weight g + b
    // x_t − x_s ≤ b − g  : arc s → t, weight b − g
    let arcs: Vec<(usize, usize, f64)> = rel.edges.iter().enumerate()
        .flat_map(|(e, &(s, t))| [(t, s, g[e] + b[e]), (s, t, b[e] - g[e])]).collect();
    let mut x = vec![0.0; rel.n_nodes];
    for _ in 0..=rel.n_nodes {
        let mut moved = false;
        for &(v, u, w) in &arcs { if x[v] + w < x[u] { x[u] = x[v] + w; moved = true; } }
        if !moved {
            for (e, &(s, t)) in rel.edges.iter().enumerate() { assert!((g[e] - (x[s] - x[t])).abs() <= b[e]); }
            return Some(x);
        }
    }
    None
}

fn holds((v, h): (f64, f64)) -> bool { v.abs() <= h }

// ═══════════════════════════════════════════════════════════════════════════
// (i) Exact decomposition, in state and on change
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn prop_isc_d16_every_walk_is_the_chord_combination_in_state_and_change() {
    let walks = Mutex::new(0usize);
    for_all("I-SC-D16 (i)", |r| {
        let rel = enumerated_rel(r);
        let order: Vec<usize> = (0..rel.n_edges()).collect();
        let basis = stated_basis(&rel, &order);
        let k = r.usize(1, 2);
        let s = enumerated_series(r, k, rel.n_edges());
        for _ in 0..4 {
            let w = enumerated_walk(r, &rel, 8);
            let n = express_in_basis(&rel, &basis, &w).expect("a turn");
            for c in 0..k {
                for t in 0..s.n_registrations() {
                    let (vw, _) = turn_sum(&s, &rel, &w, t, c).unwrap();
                    let vb = basis_turn_sums(&s, &rel, &basis, t, c);
                    assert_eq!(vw, n.iter().zip(&vb).map(|(ni, (v, _))| *ni as f64 * v).sum::<f64>());
                    // |turn sum| ≤ Σ |n_i|·|basis turn sum i|.
                    assert!(vw.abs() <= n.iter().zip(&vb).map(|(ni, (v, _))| ni.abs() as f64 * v.abs()).sum::<f64>());
                }
                for st in 0..s.n_steps() {
                    let (dw, _) = turn_change_sum(&s, &rel, &w, st, c).unwrap();
                    let db: f64 = n.iter().zip(&basis.turns)
                        .map(|(ni, cy)| *ni as f64 * turn_change_sum(&s, &rel, cy, st, c).unwrap().0).sum();
                    assert_eq!(dw, db);
                    // I-SC-D10: the change turn sum is the difference of the state turn sums.
                    let (a0, _) = turn_sum(&s, &rel, &w, st, c).unwrap();
                    let (a1, _) = turn_sum(&s, &rel, &w, st + 1, c).unwrap();
                    assert_eq!(dw, a1 - a0);
                }
            }
            *walks.lock().unwrap() += 1;
        }
    });
    assert!(walks.into_inner().unwrap() > 0);
}

// ═══════════════════════════════════════════════════════════════════════════
// (ii)–(iv): the diamond, worked
// ═══════════════════════════════════════════════════════════════════════════

/// Diamond 0 → 1 → 2, 0 → 2, 0 → 3 → 2; every bound 1/8, one registration
/// repeated. Values: e0 = 3/8, e4 = −3/8, the rest 0.
fn diamond() -> (DeclaredRelations, EdgeSeries) {
    let r = rel(4, vec![(0, 1), (1, 2), (0, 2), (0, 3), (3, 2)]);
    let a = vec![vec![0.375, 0.0, 0.0, 0.0, -0.375]];
    let s = EdgeSeries::from_edge_values_synthetic(vec![a.clone(), a], vec![vec![vec![0.125; 5]]; 2]).unwrap();
    (r, s)
}

#[test]
fn isc_d16_the_basis_does_not_settle_every_cycle_and_depends_on_the_tree() {
    let (r, s) = diamond();
    // Tree A = {e2, e0, e3}: chords e1 and e4. Both basis turn sums hold, at their bound.
    let ta = stated_basis(&r, &[2, 0, 3, 1, 4]);
    assert_eq!(ta.chords, vec![1, 4]);
    assert_eq!(basis_turn_sums(&s, &r, &ta, 0, 0), vec![(0.375, 0.375), (-0.375, 0.375)]);
    // The outer turn 0 → 1 → 2 ← 3 ← 0 is resolved: 6/8 against its own 4/8.
    let outer = vec![(0, 1), (1, 1), (4, -1), (3, -1)];
    assert_eq!(turn_sum(&s, &r, &outer, 0, 0).unwrap(), (0.75, 0.5));
    assert_eq!(express_in_basis(&r, &ta, &outer).unwrap(), vec![1, -1]);
    // Tree B = {e0, e1, e3}: chords e2 and e4. Now one basis turn sum is resolved.
    let tb = stated_basis(&r, &[0, 1, 3, 2, 4]);
    assert_eq!(tb.chords, vec![2, 4]);
    let cb = basis_turn_sums(&s, &r, &tb, 0, 0);
    assert_eq!(cb, vec![(-0.375, 0.375), (-0.75, 0.5)]);
    assert_eq!(cb.iter().filter(|x| !holds(**x)).count(), 1);
    // (iii): a resolved turn, so no node field fits within the bounds.
    assert!(node_field_within(&r, &s.a[0][0], &s.da[0][0]).is_none());
    // (iv): the triangle 0 → 3 → 2 → 0 passes through chord e4, whose basis
    // turn sum is resolved, and still holds: −3/8 against 3/8.
    let tri = vec![(3, 1), (4, 1), (2, -1)];
    assert_eq!(express_in_basis(&r, &tb, &tri).unwrap(), vec![-1, 1]);
    assert_eq!(turn_sum(&s, &r, &tri, 0, 0).unwrap(), (-0.375, 0.375));
}

/// Fields near a node difference, where turn sums sit near their bounds:
/// g[e] = x[s] − x[t] + a deviation up to 3/8, every bound 1/8.
fn near_node_series(r: &mut Seeds, rel: &DeclaredRelations) -> EdgeSeries {
    let regs = r.usize(2, 3);
    let a: Vec<Vec<Vec<f64>>> = (0..regs).map(|_| {
        let x: Vec<f64> = (0..rel.n_nodes).map(|_| r.eighth(-2.0, 2.0)).collect();
        vec![rel.edges.iter().map(|&(s, t)| x[s] - x[t] + r.eighth(-0.375, 0.375)).collect()]
    }).collect();
    EdgeSeries::from_edge_values_synthetic(a, vec![vec![vec![0.125; rel.n_edges()]]; regs]).unwrap()
}

#[test]
fn prop_isc_d16_basis_holding_does_not_imply_every_cycle_holds() {
    let (gap, tree_dep) = (Mutex::new(0usize), Mutex::new(0usize));
    for_all("I-SC-D16 (ii)", |r| {
        let rel = enumerated_rel(r);
        let s = near_node_series(r, &rel);
        let order: Vec<usize> = (0..rel.n_edges()).collect();
        let rev: Vec<usize> = order.iter().rev().copied().collect();
        let (ba, bb) = (stated_basis(&rel, &order), stated_basis(&rel, &rev));
        let turns = simple_turns(&rel);
        for t in 0..s.n_registrations() {
            let resolved = |b: &TurnBasis| basis_turn_sums(&s, &rel, b, t, 0).into_iter().filter(|x| !holds(*x)).count();
            let (na, nb) = (resolved(&ba), resolved(&bb));
            if na != nb { *tree_dep.lock().unwrap() += 1; }
            let some_turn_resolved = turns.iter().any(|cy| !holds(turn_sum(&s, &rel, cy, t, 0).unwrap()));
            // A resolved basis turn sum is a resolved turn.
            if na > 0 || nb > 0 { assert!(some_turn_resolved); }
            if na == 0 && some_turn_resolved { *gap.lock().unwrap() += 1; }
        }
    });
    assert!(gap.into_inner().unwrap() > 0, "every turn held whenever the basis did: (ii) not exercised");
    assert!(tree_dep.into_inner().unwrap() > 0, "the count of resolved basis turn sums never depended on the tree");
}

// ═══════════════════════════════════════════════════════════════════════════
// (iii) Every turn holds at δ ⇔ the field is within δ of a node difference
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn prop_isc_d16_every_cycle_holds_exactly_when_a_node_field_fits() {
    let (fits, fails) = (Mutex::new(0usize), Mutex::new(0usize));
    for_all("I-SC-D16 (iii)", |r| {
        let rel = enumerated_rel(r);
        let m = rel.n_edges();
        let mut s = if r.selects(4) { near_node_series(r, &rel) } else { enumerated_series(r, 1, m) };
        // Some fields with declared gains, so the widened turn sum bound is the one tested.
        if r.selects(3) {
            let pick = |r: &mut Seeds| [0.0, 0.125, 0.25][r.usize(0, 2)];
            let g = EdgeGains { component: vec![pick(r)], edge: (0..m).map(|_| pick(r)).collect(), independent: vec![(0..m).map(|_| pick(r)).collect()] };
            s = s.with_run_constant(vec![vec![0.0; m]], g);
        }
        let turns = simple_turns(&rel);
        for t in 0..s.n_registrations() {
            let all_hold = turns.iter().all(|cy| holds(turn_sum(&s, &rel, cy, t, 0).unwrap()));
            // The per-edge bound a turn sum uses: each term's widened bound.
            let b: Vec<f64> = (0..m).map(|e| turn_sum(&s, &rel, &[(e, 1), (e, -1)], t, 0).unwrap().1 / 2.0).collect();
            let x = node_field_within(&rel, &s.a[t][0], &b);
            assert_eq!(all_hold, x.is_some(), "every simple turn holds = {all_hold}, but a node field fits = {}", x.is_some());
            if turns.is_empty() { continue; }
            if all_hold { *fits.lock().unwrap() += 1; } else { *fails.lock().unwrap() += 1; }
        }
        // The same on the change: every change turn sum holds ⇔ Δ is within δΔ
        // of a node change.
        for st in 0..s.n_steps() {
            let all_hold = turns.iter().all(|cy| holds(turn_change_sum(&s, &rel, cy, st, 0).unwrap()));
            let b: Vec<f64> = (0..m).map(|e| turn_change_sum(&s, &rel, &[(e, 1), (e, -1)], st, 0).unwrap().1 / 2.0).collect();
            let (d, _) = s.change(st);
            assert_eq!(all_hold, node_field_within(&rel, &d[0], &b).is_some(), "on the change at step {st}");
        }
    });
    let (f, x) = (fits.into_inner().unwrap(), fails.into_inner().unwrap());
    assert!(f > 0 && x > 0, "(iii) exercised on one side only: {f} fit, {x} fail");
}
