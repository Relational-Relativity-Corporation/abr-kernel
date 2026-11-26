//! Kernel V10.3 — K7: route enumeration, the two-route turn sum, and a turn
//! basis from a stated spanning tree.
//!
//! [IMPL] SYNTHETIC FIXTURES: declarations and values are chosen to exercise
//! each statement. They establish that the forms compute as stated, nothing
//! empirical. Values are dyadic, so the linear identities are exact.

#[path = "support/synthetic_feedback.rs"]
mod synthetic_feedback;

#[path = "support/synthetic_identity.rs"]
mod synthetic_identity;
use abr_kernel::ground_variables::Registration;
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance, NodeField};
use abr_kernel::persistence_change::{turn_sum, is_turn, EdgeSeries};
use abr_kernel::topology::*;

fn rel(n: usize, edges: Vec<(usize, usize)>) -> DeclaredRelations {
    let m = edges.len();
    synthetic_identity::identify(DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m]))
}

/// 0 → 1 → 3, 0 → 2 → 3, 0 → 3: three routes from 0 to 3.
fn diamond() -> DeclaredRelations { rel(4, vec![(0, 1), (1, 3), (0, 2), (2, 3), (0, 3)]) }

fn edge_series(a: Vec<Vec<Vec<f64>>>) -> EdgeSeries {
    let da = a.iter().map(|t| t.iter().map(|c| vec![0.0625; c.len()]).collect()).collect();
    EdgeSeries::from_edge_values_synthetic(a, da).unwrap()
}

// ── Routes ─────────────────────────────────────────────────────────────────

#[test]
fn k7_routes_are_listed_shortest_first_and_complete() {
    let r = diamond();
    let e = routes(&r, 0, 3, 100);
    assert!(e.complete);
    assert_eq!(e.routes, vec![vec![4], vec![0, 1], vec![2, 3]]);
    assert!(e.routes.iter().all(|p| is_route(&r, p)));
    assert_eq!(routes(&r, 3, 0, 100), RouteEnumeration { routes: vec![], complete: true });
    assert_eq!(routes(&r, 1, 1, 100), RouteEnumeration { routes: vec![], complete: true });
}

#[test]
fn k7_parallel_edges_are_routes_of_length_one() {
    let r = rel(2, vec![(0, 1), (0, 1)]);
    assert_eq!(routes(&r, 0, 1, 10).routes, vec![vec![0], vec![1]]);
}

#[test]
fn k7_a_stated_limit_reports_an_incomplete_list() {
    let e = routes(&diamond(), 0, 3, 2);
    assert!(!e.complete);
    assert_eq!(e.routes.len(), 2);
}

#[test]
fn k7_two_route_closure_is_the_difference_of_route_sums() {
    let r = diamond();
    // Edge-registered values with a route disagreement of 1.
    let s = edge_series(vec![vec![vec![1.0, 2.0, 0.5, 2.5, 4.0]], vec![vec![1.0, 2.0, 0.5, 2.5, 3.0]]]);
    let cy = two_route_turn(&r, &[0, 1], &[4]).unwrap();
    assert!(is_turn(&r, &cy));
    assert_eq!(cy, vec![(0, 1), (1, 1), (4, -1)]);
    // B_π₁ − B_π₂ = (1 + 2) − 4 = −1 at registration 0; 0 at registration 1.
    assert_eq!(turn_sum(&s, &r, &cy, 0, 0), Some((-1.0, 3.0 * 0.0625)));
    assert_eq!(turn_sum(&s, &r, &cy, 1, 0).unwrap().0, 0.0);
    assert!(two_route_turn(&r, &[0], &[4]).is_none(), "different targets");
    assert!(two_route_turn(&r, &[0, 3], &[4]).is_none(), "not a route");
}

// ── Turn basis ────────────────────────────────────────────────────────────

#[test]
fn k7_basis_from_a_stated_tree_has_m_minus_n_plus_c_cycles() {
    let r = diamond();
    let b = turn_basis(&r, &[0, 1, 2]).unwrap();
    assert_eq!(b.tree, vec![0, 1, 2]);
    assert_eq!(b.chords, vec![3, 4]);
    assert_eq!(b.turns.len(), 5 - 4 + 1);
    assert_eq!(b.turns[0], vec![(3, 1), (1, -1), (0, -1), (2, 1)]);
    assert_eq!(b.turns[1], vec![(4, 1), (1, -1), (0, -1)]);
    assert!(b.turns.iter().all(|c| is_turn(&r, c)));
}

#[test]
fn k7_a_tree_per_connected_part() {
    // Two parts: 0 → 1 → 2 with 0 → 2, and 3 → 4 twice.
    let r = rel(5, vec![(0, 1), (1, 2), (0, 2), (3, 4), (3, 4)]);
    let b = turn_basis(&r, &[0, 1, 3]).unwrap();
    assert_eq!(b.turns.len(), 5 - 5 + 2);
    assert_eq!(b.turns[1], vec![(4, 1), (3, -1)]);
}

#[test]
fn k7_a_stated_tree_that_is_not_a_spanning_tree_is_refused() {
    let r = diamond();
    assert_eq!(turn_basis(&r, &[0, 1, 4]).unwrap_err(), vec![
        TreeRefusal::FormsTurn { edge: 4 },
        TreeRefusal::DoesNotSpan { loci: (0, 2) },
        TreeRefusal::DoesNotSpan { loci: (2, 3) },
    ]);
    assert!(turn_basis(&r, &[0, 0, 2, 3]).unwrap_err().contains(&TreeRefusal::DuplicateEdge { edge: 0 }));
    assert!(turn_basis(&r, &[0, 2, 9]).unwrap_err().contains(&TreeRefusal::EdgeOutOfRange { edge: 9 }));
}

#[test]
fn k7_closure_is_zero_by_identity_on_node_values_and_a_test_on_edge_values() {
    let r = diamond();
    let b = turn_basis(&r, &[0, 1, 2]).unwrap();
    let seq: Vec<Registration> = (0..2).map(|t| Registration::synthetic(
        NodeField::new(vec![vec![4.0 + t as f64, 3.0, 1.5, 0.0]]), vec![vec![0.0625; 4]])).collect();
    let n = EdgeSeries::from_registrations(&seq, &r).unwrap();
    assert!(basis_turn_sums(&n, &r, &b, 0, 0).iter().all(|(v, _)| *v == 0.0));
    let s = edge_series(vec![vec![vec![1.0, 2.0, 0.5, 2.5, 4.0]], vec![vec![1.0, 2.0, 0.5, 2.5, 3.0]]]);
    let c0 = basis_turn_sums(&s, &r, &b, 0, 0);
    assert_eq!(c0[1], (1.0, 3.0 * 0.0625)); // 4 − (1 + 2): resolved, turn sum fails at δ
    assert!(c0[1].0.abs() > c0[1].1);
    assert_eq!(basis_turn_sums(&s, &r, &b, 1, 0)[1].0, 0.0);
}

#[test]
fn k7_reversing_a_cycle_negates_its_closure() {
    let r = diamond();
    let s = edge_series(vec![vec![vec![1.0, 2.0, 0.5, 2.5, 4.0]], vec![vec![1.0; 5]]]);
    let cy = two_route_turn(&r, &[0, 1], &[4]).unwrap();
    let rev: Vec<(usize, i8)> = cy.iter().rev().map(|&(e, sg)| (e, -sg)).collect();
    let (a, h) = turn_sum(&s, &r, &cy, 0, 0).unwrap();
    assert_eq!(turn_sum(&s, &r, &rev, 0, 0).unwrap(), (-a, h));
}

#[test]
fn k7_a_closed_walk_is_the_chord_combination_of_the_basis() {
    let r = diamond();
    let b = turn_basis(&r, &[0, 1, 2]).unwrap();
    // The two-route walk 0→2→3 against 0→1→3 uses chord 3 once, chord 4 not.
    let cy = two_route_turn(&r, &[2, 3], &[0, 1]).unwrap();
    assert_eq!(express_in_basis(&r, &b, &cy), Some(vec![1, 0]));
    assert_eq!(express_in_basis(&r, &b, &[(0, 1)]), None);
}

// ── Properties ─────────────────────────────────────────────────────────────

struct Seeds(u64);
impl Seeds {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn usize(&mut self, lo: usize, hi: usize) -> usize { lo + (self.next() % (hi - lo + 1) as u64) as usize }
    fn eighth(&mut self) -> f64 { (self.next() % 65) as f64 / 8.0 - 4.0 }
}

fn enumerated_rel(g: &mut Seeds, n: (usize, usize), m: (usize, usize)) -> DeclaredRelations {
    let n = g.usize(n.0, n.1);
    let m = g.usize(m.0, m.1);
    let edges: Vec<(usize, usize)> = (0..m).map(|_| loop {
        let (s, t) = (g.usize(0, n - 1), g.usize(0, n - 1));
        if s != t { break (s, t); }
    }).collect();
    synthetic_feedback::declare_loops(rel(n, edges))
}

/// An enumerated spanning forest: edges in a seed-declared order, kept when they join parts.
fn enumerated_tree(g: &mut Seeds, r: &DeclaredRelations) -> (Vec<usize>, usize) {
    let mut order: Vec<usize> = (0..r.n_edges()).collect();
    for i in (1..order.len()).rev() { let j = g.usize(0, i); order.swap(i, j); }
    let mut p: Vec<usize> = (0..r.n_nodes).collect();
    fn f(p: &mut [usize], v: usize) -> usize { if p[v] == v { v } else { let x = f(p, p[v]); p[v] = x; x } }
    let mut tree = Vec::new();
    for e in order {
        let (a, b) = (f(&mut p, r.edges[e].0), f(&mut p, r.edges[e].1));
        if a != b { p[a] = b; tree.push(e); }
    }
    let parts = (0..r.n_nodes).filter(|&v| f(&mut p, v) == v).count();
    (tree, parts)
}

#[test]
fn prop_k7_routes_match_brute_force() {
    let mut checked = 0;
    for seed in 0..1500u64 {
        let mut g = Seeds(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ 0x4B4B);
        let r = enumerated_rel(&mut g, (2, 4), (1, 8));
        let (from, to) = (g.usize(0, r.n_nodes - 1), g.usize(0, r.n_nodes - 1));
        let e = routes(&r, from, to, 10_000);
        assert!(e.complete);
        // Brute force: every edge sequence of length ≤ 3 (n ≤ 4, so every
        // simple route has length ≤ 3) that is a route from → to visiting
        // no locus twice.
        let m = r.n_edges();
        let mut expect: Vec<Vec<usize>> = Vec::new();
        let mut seqs: Vec<Vec<usize>> = (0..m).map(|e| vec![e]).collect();
        for _ in 0..3 {
            let mut next = Vec::new();
            for p in &seqs {
                let simple = {
                    let mut loci = vec![r.edges[p[0]].0];
                    loci.extend(p.iter().map(|&e| r.edges[e].1));
                    let mut l = loci.clone(); l.sort(); l.dedup();
                    l.len() == loci.len()
                };
                if is_route(&r, p) && simple && r.edges[p[0]].0 == from && r.edges[*p.last().unwrap()].1 == to && from != to {
                    expect.push(p.clone());
                }
                if p.len() < 3 { for e in 0..m { let mut q = p.clone(); q.push(e); next.push(q); } }
            }
            seqs = next;
        }
        expect.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
        assert_eq!(e.routes, expect, "seed {seed}");
        if !expect.is_empty() { checked += 1; }
    }
    assert!(checked > 300, "vacuous: {checked}");
}

#[test]
fn prop_k7_basis_size_and_exact_chord_combination() {
    let (mut walks, mut cross) = (0, 0);
    for seed in 0..1500u64 {
        let mut g = Seeds(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ 0x6D6D);
        let r = enumerated_rel(&mut g, (2, 6), (1, 9));
        let (tree, parts) = enumerated_tree(&mut g, &r);
        let b = turn_basis(&r, &tree).unwrap();
        assert_eq!(b.turns.len(), r.n_edges() + parts - r.n_nodes, "seed {seed}");
        assert!(b.turns.iter().zip(&b.chords).all(|(c, &ch)| is_turn(&r, c) && c[0] == (ch, 1)));
        let a: Vec<Vec<Vec<f64>>> = (0..2).map(|_| vec![(0..r.n_edges()).map(|_| g.eighth()).collect()]).collect();
        let s = edge_series(a);
        let base = basis_turn_sums(&s, &r, &b, 0, 0);
        let check = |w: &[(usize, i8)]| {
            let coef = express_in_basis(&r, &b, w).unwrap();
            let combined: f64 = coef.iter().zip(&base).map(|(k, (v, _))| *k as f64 * v).sum();
            assert_eq!(turn_sum(&s, &r, w, 0, 0).unwrap().0, combined, "seed {seed}");
        };
        // Two-route walks between a seed-selected pair of loci.
        let (from, to) = (g.usize(0, r.n_nodes - 1), g.usize(0, r.n_nodes - 1));
        let rs = routes(&r, from, to, 50).routes;
        for i in 0..rs.len() { for j in 0..rs.len() {
            check(&two_route_turn(&r, &rs[i], &rs[j]).unwrap()); walks += 1;
        } }
        // Another stated tree's fundamental turns, in this basis.
        let (tree2, _) = enumerated_tree(&mut g, &r);
        for c in turn_basis(&r, &tree2).unwrap().turns { check(&c); cross += 1; }
    }
    assert!(walks > 300 && cross > 300, "vacuous: {walks} walks, {cross} cross-basis turns");
}
