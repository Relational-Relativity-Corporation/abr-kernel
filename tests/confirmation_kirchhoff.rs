//! **Confirmation (legacy).** Not a basis: removing this file changes no kernel
//! statement (Origin, 95th). Every claim here is an intersection (W14, class
//! C) of the kernel's relational result with a legacy result projected
//! through a declared mapping (Origin, 2026-11-26, 101st: *"as long as we're
//! careful about legacy as projection even though it could be a valid
//! relational result as well that intersection is what we actually want"*).
//!
//! **Domain and source:** electrical networks of resistors, every resistance
//! 1 Ω, with sources of EMF in branches. Legacy results: Kirchhoff's laws
//! (1845, 1847); the balanced bridge (Christie, 1833; Wheatstone, 1843);
//! mesh currents (Maxwell, *A Treatise on Electricity and Magnetism*, 1873);
//! Tellegen's theorem (1952); the superposition theorem; the currents of a
//! unit-resistance network as the orthogonal projection of its EMFs onto its
//! cycles (Weyl, 1923). The networks are hand-derived worked examples and
//! enumerated synthetic networks; no measurement is used.
//!
//! **The declared mapping (M, a domain declaration, never the kernel's):**
//! each branch a relation, in the direction of its reference current; the
//! branch EMF (V) the relation's registered value; 1 Ω on every branch the
//! equal inner product (100th); the branch current (A) compared with the
//! **balanced part**; e − R·i, the EMF not dropped across the branch's
//! resistance, compared with the **potential part** (differences of node
//! potentials, in the kernel's sign, x[src] − x[tgt]).
//!
//! **The kernel relationship confirmed:** the 99th's two parts and the
//! 100th's orthogonal split. **What it cannot establish:** that energy,
//! connection and freedom are best read through the split in other domains;
//! in this domain the agreement is expected, Kirchhoff being where the
//! structure was found. The intersection holds or fails exactly; it is not a
//! fit.

#[path = "support/synthetic_whole_field.rs"]
mod support;

use abr_kernel::exact::Ratio;
use abr_kernel::operators::DeclaredRelations;
use abr_kernel::persistence_change::EdgeSeries;
use abr_kernel::relational_field::{InnerProduct, RelationalField};
use abr_kernel::topology::cycle_basis;
use support::*;

/// A whole field over a network: the EMFs at the first registration, and
/// twice them at the second (superposition, below). One component.
fn network(rel: &DeclaredRelations, emf: &[f64]) -> RelationalField {
    let m = rel.n_edges();
    let a = vec![vec![emf.to_vec()], vec![emf.iter().map(|x| 2.0 * x).collect()]];
    let series = EdgeSeries::from_edge_values_synthetic(a, vec![vec![vec![0.0; m]]; 2]).unwrap();
    whole_of_relations(rel, series, &relation_carrier(vec![emf.to_vec(), emf.to_vec()], vec![vec![0.0; m]; 2]))
}

fn r(x: f64) -> Ratio { Ratio::from_f64(x) }

/// 1. A single loop, worked by hand. A triangle 0 → 1 → 2 with 0 → 2; a
/// source of 6 V in branch 0 → 1; 1 Ω in each branch. Kirchhoff's voltage
/// law around the loop 0 → 1 → 2 → 0: 6 = 1·i + 1·i + 1·i, so i = 2 A around
/// the loop: +2 in 0 → 1 and 1 → 2, −2 in 0 → 2 (traversed against its
/// reference). e − R·i = (4, −2, 2), the differences of node potentials.
#[test]
fn confirmation_legacy_a_single_loop_worked_by_hand() {
    let rel = rel(3, vec![(0, 1), (1, 2), (0, 2)]);
    let f = network(&rel, &[6.0, 0.0, 0.0]);
    let s = f.two_parts(&InnerProduct::equal());
    let current = [2.0, 2.0, -2.0];
    let drop = [4.0, -2.0, 2.0];
    for e in 0..3 {
        assert_eq!(s.spatial[0].exact_balanced[0][e], r(current[e]), "branch {e}: the current is the balanced part");
        assert_eq!(s.spatial[0].exact_potential[0][e], r(drop[e]), "branch {e}: e − R·i is the potential part");
    }
}

/// 2. The balanced Wheatstone bridge (Christie, 1833; Wheatstone, 1843).
/// Loci a = 0, b = 1, c = 2, d = 3. Arms a → b, a → c, b → d, c → d; the
/// bridge b → c; the source d → a with 10 V; 1 Ω in every branch (the
/// source's own resistance included). Equal arms balance the bridge, so its
/// current is 0. Each path a → b → d and a → c → d has 2 Ω, together 1 Ω;
/// with the source's 1 Ω the loop has 2 Ω, so the source carries 5 A and
/// each arm 2.5 A.
#[test]
fn confirmation_legacy_the_balanced_wheatstone_bridge() {
    let rel = rel(4, vec![(0, 1), (0, 2), (1, 3), (2, 3), (1, 2), (3, 0)]);
    let f = network(&rel, &[0.0, 0.0, 0.0, 0.0, 0.0, 10.0]);
    let s = f.two_parts(&InnerProduct::equal());
    let current = [2.5, 2.5, 2.5, 2.5, 0.0, 5.0];
    for e in 0..6 {
        assert_eq!(s.spatial[0].exact_balanced[0][e], r(current[e]), "branch {e}");
    }
    assert!(s.spatial[0].exact_balanced[0][4].is_zero(), "the balanced bridge carries no current");
}

/// Maxwell's mesh currents, computed independently of the kernel's split:
/// over the declared cycles C (K7), solve (CᵀC) z = Cᵀe for the mesh
/// currents z, exactly, and form the branch currents i = C z. The kernel's
/// split works over the loci; this works over the cycles.
fn mesh_currents(rel: &DeclaredRelations, emf: &[Ratio]) -> Vec<Ratio> {
    let m = rel.n_edges();
    let mut p: Vec<usize> = (0..rel.n_nodes).collect();
    fn root(p: &mut [usize], v: usize) -> usize { let mut v = v; while p[v] != v { v = p[v]; } v }
    let mut tree = Vec::new();
    for (e, &(s, t)) in rel.edges.iter().enumerate() { let (a, b) = (root(&mut p, s), root(&mut p, t)); if a != b { p[a] = b; tree.push(e); } }
    let b = cycle_basis(rel, &tree).unwrap();
    let k = b.cycles.len();
    let col = |j: usize| -> Vec<Ratio> {
        let mut c = vec![Ratio::zero(); m];
        for &(e, sg) in &b.cycles[j] { c[e] = if sg > 0 { c[e].add(&Ratio::from_i64(1)) } else { c[e].sub(&Ratio::from_i64(1)) }; }
        c
    };
    let cols: Vec<Vec<Ratio>> = (0..k).map(col).collect();
    let dot = |x: &[Ratio], y: &[Ratio]| x.iter().zip(y).fold(Ratio::zero(), |acc, (a, b)| acc.add(&a.mul(b)));
    // [CᵀC | Cᵀe], Gauss–Jordan.
    let mut a: Vec<Vec<Ratio>> = (0..k).map(|i| { let mut row: Vec<Ratio> = (0..k).map(|j| dot(&cols[i], &cols[j])).collect(); row.push(dot(&cols[i], emf)); row }).collect();
    for c in 0..k {
        let piv = (c..k).find(|&i| !a[i][c].is_zero()).unwrap();
        a.swap(c, piv);
        let pv = a[c][c].clone();
        for j in 0..=k { a[c][j] = a[c][j].div(&pv); }
        for i in 0..k { if i != c && !a[i][c].is_zero() { let f = a[i][c].clone(); for j in 0..=k { let t = a[c][j].mul(&f); a[i][j] = a[i][j].sub(&t); } } }
    }
    (0..m).map(|e| (0..k).fold(Ratio::zero(), |acc, j| acc.add(&cols[j][e].mul(&a[j][k])))).collect()
}

/// 3. Maxwell's mesh currents agree with the kernel's balanced part exactly,
/// on every enumerated network with an enumerated EMF in each branch.
#[test]
fn confirmation_legacy_maxwell_mesh_currents() {
    for_all("confirmation: Maxwell mesh currents", 0x101_03, |s| {
        let rel = enumerated_rel(s);
        let emf: Vec<f64> = (0..rel.n_edges()).map(|_| s.int(-6, 6) as f64).collect();
        let f = network(&rel, &emf);
        let parts = f.two_parts(&InnerProduct::equal());
        let i = mesh_currents(&rel, &emf.iter().map(|&x| r(x)).collect::<Vec<_>>());
        for e in 0..rel.n_edges() {
            assert_eq!(parts.spatial[0].exact_balanced[0][e], i[e], "branch {e}: mesh current and balanced part");
        }
    });
}

/// 4. Tellegen's theorem (1952) and the power balance: power supplied by the
/// sources, Σ e·i, equals power dissipated, Σ R·i² = Σ i² with 1 Ω. In the
/// kernel this is orthogonality: Σ e·q = Σ (p + q)·q = Σ q², since Σ p·q = 0.
#[test]
fn confirmation_legacy_tellegen_power_balance() {
    for_all("confirmation: Tellegen", 0x101_04, |s| {
        let rel = enumerated_rel(s);
        let emf: Vec<f64> = (0..rel.n_edges()).map(|_| s.int(-6, 6) as f64).collect();
        let f = network(&rel, &emf);
        let q = &f.two_parts(&InnerProduct::equal()).spatial[0].exact_balanced[0];
        let supplied = emf.iter().zip(q).fold(Ratio::zero(), |acc, (&e, i)| acc.add(&r(e).mul(i)));
        let dissipated = q.iter().fold(Ratio::zero(), |acc, i| acc.add(&i.mul(i)));
        assert_eq!(supplied, dissipated, "power supplied equals power dissipated");
    });
}

/// 5. Superposition: doubling every EMF doubles every current, and the
/// change of the EMFs over the step drives the change of the currents
/// (the split of a change is the change of the split, 100th).
#[test]
fn confirmation_legacy_superposition() {
    for_all("confirmation: superposition", 0x101_05, |s| {
        let rel = enumerated_rel(s);
        let emf: Vec<f64> = (0..rel.n_edges()).map(|_| s.int(-6, 6) as f64).collect();
        let f = network(&rel, &emf);
        let parts = f.two_parts(&InnerProduct::equal());
        for e in 0..rel.n_edges() {
            let (i0, i1) = (&parts.spatial[0].exact_balanced[0][e], &parts.spatial[1].exact_balanced[0][e]);
            assert_eq!(i1.clone(), i0.add(i0), "doubled EMF, doubled current");
            assert_eq!(parts.persistence[0].exact_balanced[0][e], i0.clone(), "the change of current is the current of the change");
        }
    });
}
