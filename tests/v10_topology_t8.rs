//! Kernel V10.3 — T8: self-relations refused; a directed cycle admitted only
//! as a declared feedback loop with a stated basis; opposite directions need
//! their own basis.
//!
//! [IMPL] SYNTHETIC FIXTURES: declarations are chosen to exercise each rule.
//! They establish that the check computes as stated, nothing empirical.

#[path = "support/synthetic_identity.rs"]
mod synthetic_identity;
use abr_kernel::operators::{DeclaredRelations, EdgeProvenance};
use abr_kernel::topology::*;

fn rel(n: usize, edges: Vec<(usize, usize)>) -> DeclaredRelations {
    let m = edges.len();
    synthetic_identity::identify(DeclaredRelations::from_edges_with_provenance(n, edges, vec![EdgeProvenance::Continuation; m]))
}

fn fb(edge: usize) -> FeedbackDeclaration {
    FeedbackDeclaration { edge, basis: "synthetic: registered return relation".into() }
}

/// A directed walk: consecutive edges meet head to tail and it closes.
fn is_directed_cycle(r: &DeclaredRelations, c: &[usize]) -> bool {
    !c.is_empty() && (0..c.len()).all(|i| r.edges[c[i]].1 == r.edges[c[(i + 1) % c.len()]].0)
}

#[test]
fn t8_an_acyclic_field_is_admitted_with_no_feedback() {
    let r = rel(4, vec![(0, 1), (1, 2), (0, 2), (2, 3)]);
    assert_eq!(check_t8(&r, &[]), Ok(T8Admission { feedback: vec![], closes: vec![] }));
}

#[test]
fn t8_a_self_relation_is_refused_even_if_declared_feedback() {
    let r = rel(3, vec![(0, 1), (1, 1), (1, 2)]);
    assert_eq!(check_t8(&r, &[]), Err(vec![T8Refusal::SelfRelation { edge: 1 }]));
    let e = check_t8(&r, &[fb(1)]).unwrap_err();
    assert!(e.contains(&T8Refusal::SelfRelation { edge: 1 }));
}

#[test]
fn t8_an_undeclared_cycle_is_refused_and_named() {
    // 0 → 1 → 2 → 3 → 1: the loop 1 → 2 → 3 → 1.
    let r = rel(4, vec![(0, 1), (1, 2), (2, 3), (3, 1)]);
    let e = check_t8(&r, &[]).unwrap_err();
    assert_eq!(e.len(), 1);
    match &e[0] {
        T8Refusal::UndeclaredCycle { cycle } => {
            assert!(is_directed_cycle(&r, cycle));
            let mut c = cycle.clone(); c.sort();
            assert_eq!(c, vec![1, 2, 3]);
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn t8_a_declared_feedback_loop_is_admitted_with_the_cycle_it_closes() {
    let r = rel(4, vec![(0, 1), (1, 2), (2, 3), (3, 1)]);
    let a = check_t8(&r, &[fb(3)]).unwrap();
    assert_eq!(a.feedback, vec![3]);
    assert_eq!(a.closes, vec![vec![3, 1, 2]]);
    assert!(is_directed_cycle(&r, &a.closes[0]));
    // Any edge of the loop may carry the declaration.
    assert!(check_t8(&r, &[fb(1)]).is_ok());
    // The edge into the loop is on no cycle.
    assert_eq!(check_t8(&r, &[fb(0)]).unwrap_err(),
        vec![T8Refusal::FeedbackNotOnCycle { edge: 0 },
             T8Refusal::UndeclaredCycle { cycle: check_cycle(&r) }]);
}

fn check_cycle(r: &DeclaredRelations) -> Vec<usize> {
    match &check_t8(r, &[]).unwrap_err()[0] { T8Refusal::UndeclaredCycle { cycle } => cycle.clone(), _ => unreachable!() }
}

#[test]
fn t8_opposite_directions_need_their_own_basis() {
    let r = rel(2, vec![(0, 1), (1, 0)]);
    assert!(matches!(check_t8(&r, &[]).unwrap_err()[0], T8Refusal::UndeclaredCycle { .. }));
    assert_eq!(check_t8(&r, &[fb(1)]).unwrap().closes, vec![vec![1, 0]]);
}

#[test]
fn t8_two_loops_sharing_a_locus_need_two_declarations() {
    // 0 ⇄ 1 and 1 ⇄ 2 through edges 0..3; one declaration leaves a cycle.
    let r = rel(3, vec![(0, 1), (1, 0), (1, 2), (2, 1)]);
    let e = check_t8(&r, &[fb(1)]).unwrap_err();
    assert_eq!(e.len(), 1);
    assert!(matches!(e[0], T8Refusal::UndeclaredCycle { .. }));
    let a = check_t8(&r, &[fb(3), fb(1)]).unwrap();
    assert_eq!(a.feedback, vec![1, 3]);
    assert!(a.closes.iter().all(|c| is_directed_cycle(&r, c)));
}

#[test]
fn t8_declaration_faults_are_refused() {
    let r = rel(2, vec![(0, 1), (1, 0)]);
    let blank = FeedbackDeclaration { edge: 1, basis: "  ".into() };
    assert!(check_t8(&r, &[blank]).unwrap_err().contains(&T8Refusal::FeedbackWithoutBasis { edge: 1 }));
    assert!(check_t8(&r, &[fb(1), fb(1)]).unwrap_err().contains(&T8Refusal::DuplicateFeedback { edge: 1 }));
    assert!(check_t8(&r, &[fb(1), fb(7)]).unwrap_err().contains(&T8Refusal::FeedbackEdgeOutOfRange { edge: 7 }));
}

#[test]
fn t8_parallel_edges_are_not_a_cycle() {
    let r = rel(2, vec![(0, 1), (0, 1)]);
    assert!(check_t8(&r, &[]).is_ok());
}

#[test]
fn t8_declare_checked_constructs_and_checks_together() {
    let p = vec![EdgeProvenance::Continuation, EdgeProvenance::Coupling];
    assert!(declare_checked(2, vec![(0, 1), (1, 0)], p.clone(), &[]).is_err());
    let (r, a) = declare_checked(2, vec![(0, 1), (1, 0)], p, &[fb(1)]).unwrap();
    assert_eq!((r.n_edges(), a.feedback), (2, vec![1]));
}

// ── Property: admitted exactly when the field less its feedback is acyclic ──

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
}

/// Kahn's algorithm: an independent oracle for acyclicity over kept edges.
fn acyclic(r: &DeclaredRelations, keep: &dyn Fn(usize) -> bool) -> bool {
    let mut indeg = vec![0usize; r.n_nodes];
    for e in 0..r.n_edges() { if keep(e) { indeg[r.edges[e].1] += 1; } }
    let mut q: Vec<usize> = (0..r.n_nodes).filter(|&v| indeg[v] == 0).collect();
    let mut seen = 0;
    while let Some(v) = q.pop() {
        seen += 1;
        for &e in &r.out[v] { if keep(e) { let w = r.edges[e].1; indeg[w] -= 1; if indeg[w] == 0 { q.push(w); } } }
    }
    seen == r.n_nodes
}

#[test]
fn prop_t8_admits_exactly_when_the_residual_is_acyclic() {
    let (mut admitted, mut refused) = (0, 0);
    for seed in 0..3000u64 {
        let mut g = Seeds(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ 0x7E7E);
        let n = g.usize(2, 6);
        let m = g.usize(1, 9);
        let edges: Vec<(usize, usize)> = (0..m).map(|_| loop {
            let (s, t) = (g.usize(0, n - 1), g.usize(0, n - 1));
            if s != t { break (s, t); }
        }).collect();
        let r = rel(n, edges);
        let mut feedback: Vec<usize> = (0..m).filter(|_| g.next() % 3 == 0).collect();
        // Keep only feedback edges that lie on a cycle, so the oracle is plain.
        feedback.retain(|&f| { let (s, t) = r.edges[f]; reaches(&r, t, s) });
        let decl: Vec<FeedbackDeclaration> = feedback.iter().map(|&e| fb(e)).collect();
        let expect = acyclic(&r, &|e| !feedback.contains(&e));
        match check_t8(&r, &decl) {
            Ok(a) => {
                assert!(expect, "seed {seed}: admitted a residual cycle");
                assert_eq!(a.closes.len(), a.feedback.len());
                for (f, c) in a.feedback.iter().zip(&a.closes) {
                    assert_eq!(c[0], *f);
                    assert!(is_directed_cycle(&r, c), "seed {seed}");
                }
                admitted += 1;
            }
            Err(e) => {
                assert!(!expect, "seed {seed}: refused an acyclic residual: {e:?}");
                for x in &e {
                    match x {
                        T8Refusal::UndeclaredCycle { cycle } => {
                            assert!(is_directed_cycle(&r, cycle), "seed {seed}");
                            assert!(cycle.iter().all(|e| !feedback.contains(e)), "seed {seed}");
                        }
                        other => panic!("seed {seed}: unexpected {other:?}"),
                    }
                }
                refused += 1;
            }
        }
    }
    assert!(admitted > 100 && refused > 100, "vacuous: {admitted} admitted, {refused} refused");
}

/// Directed reachability over every edge (depth-first): f = (s, t) lies on
/// a directed cycle exactly when t reaches s.
fn reaches(r: &DeclaredRelations, from: usize, to: usize) -> bool {
    let (mut seen, mut stack) = (vec![false; r.n_nodes], vec![from]);
    while let Some(v) = stack.pop() {
        if v == to { return true; }
        if seen[v] { continue; }
        seen[v] = true;
        for &e in &r.out[v] { stack.push(r.edges[e].1); }
    }
    false
}

// ── Enforcement at the reading entry points (Origin, 2026-11-22, 28th) ─────

mod entry_points {
    use super::*;
    use abr_kernel::ground_variables::{ground_reading, minor_histories, DeclarationGap, EnergyCarrier, Registration};
    use abr_kernel::operators::NodeField;
    use abr_kernel::persistence_change::EdgeSeries;
    use abr_kernel::relational_field::RelationalField;

    fn seq() -> Vec<Registration> {
        (0..3).map(|t| {
            let t = t as f64;
            Registration::synthetic(NodeField::new(vec![vec![4.0 + t, 3.0, 1.0 - t, 2.0], vec![1.0, 2.0 * t, 0.5, 3.0]]),
                vec![vec![0.0625; 4]; 2])
        }).collect()
    }

    /// 0 → 1 → 2 → 3 → 1: one loop.
    fn looped() -> DeclaredRelations { rel(4, vec![(0, 1), (1, 2), (2, 3), (3, 1)]) }

    fn is_t8(g: &DeclarationGap) -> bool { matches!(g, DeclarationGap::T8(_)) }

    #[test]
    fn t8_every_reading_entry_point_refuses_an_undeclared_loop() {
        let (r, s) = (looped(), seq());
        assert!(is_t8(&RelationalField::new(&r, s.clone(), None).unwrap_err()));
        assert!(is_t8(&EdgeSeries::from_registrations(&s, &r).unwrap_err()));
        assert!(is_t8(&minor_histories(&s, &r).unwrap_err()));
        let g = ground_reading(&s, &r, None);
        assert!(is_t8(g.freedom.as_ref().unwrap_err()));
        assert!(is_t8(g.connection.as_ref().unwrap_err()));
        assert!(is_t8(g.v2_held_arrangement.as_ref().unwrap_err()));
        let c: Vec<EnergyCarrier> = (0..3).map(|_| EnergyCarrier { range: abr_kernel::ground_variables::ObservedRange::synthetic(),
            name: "SYNTHETIC".into(), unit: "u".into(), values: vec![1.0, 2.0, 3.0, 4.0], delta: vec![0.0625; 4],
        }).collect();
        assert!(is_t8(ground_reading(&s, &r, Some(&c)).energy.as_ref().unwrap_err()));
        // The refusal names the loop and says what to return to.
        match EdgeSeries::from_registrations(&s, &r).unwrap_err() {
            DeclarationGap::T8(v) => assert!(matches!(&v[0], T8Refusal::UndeclaredCycle { cycle } if cycle.len() == 3)),
            _ => unreachable!(),
        }
        assert!(DeclarationGap::T8(vec![]).remedy().contains("feedback"));
    }

    #[test]
    fn t8_a_declared_loop_is_read() {
        let r = looped().with_feedback(vec![fb(3)]);
        let s = seq();
        assert!(RelationalField::new(&r, s.clone(), None).is_ok());
        assert!(EdgeSeries::from_registrations(&s, &r).is_ok());
        assert!(ground_reading(&s, &r, None).connection.is_ok());
    }

    #[test]
    fn t8_an_edge_valued_series_declares_no_topology_and_is_not_checked() {
        let a = vec![vec![vec![1.0, 2.0]], vec![vec![2.0, 2.0]]];
        assert!(EdgeSeries::from_edge_values_synthetic(a, vec![vec![vec![0.0625; 2]]; 2]).is_ok());
    }

    #[test]
    fn t8_declare_checked_carries_its_feedback_onto_the_field() {
        let (r, _) = declare_checked(4, vec![(0, 1), (1, 2), (2, 3), (3, 1)], vec![EdgeProvenance::Continuation; 4], &[fb(3)]).unwrap();
        assert_eq!(r.feedback, vec![fb(3)]);
        // Identity is declared separately (RPPI-V10 G-1); without it the field is refused.
        assert!(matches!(EdgeSeries::from_registrations(&seq(), &r).unwrap_err(), DeclarationGap::RelationIdentity(_)));
        let r = crate::synthetic_identity::identify(r);
        assert!(EdgeSeries::from_registrations(&seq(), &r).is_ok());
    }
}
