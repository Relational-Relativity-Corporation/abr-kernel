//! Kernel V10.3: §2.7, the declared ground (Declared, Origin, 2026-11-23,
//! 57th): the domain declaration, the declared field, the ground record and
//! the boundaries of stages (`src/domain.rs`).
//!
//! Every test forms a declared whole field (Df6, W11): the declaration maps
//! energy, freedom and connection; every variable is evaluated together; the
//! claims are read from the field's own records. Fixtures are enumerated from
//! declared seeds (W17).
//!
//! [IMPL] SYNTHETIC FIXTURES: dyadic values, so every sum and product is
//! exact. They establish that the forms compute as stated, nothing
//! empirical.

#[path = "support/synthetic_whole_field.rs"]
mod wf;
use abr_kernel::domain::*;
use abr_kernel::exact::{exceeds, Expansion};
use abr_kernel::ground_variables::{AtDelta, EnergyCarrier, ObservedRange};
use abr_kernel::persistence_change::{Reading, ReadingPair};
use std::sync::Mutex;
use wf::{declared, domain, enumerated_declared, enumerated_rel, for_all, registrations, rel, transfer, Declared};

fn state_kind(s: &State) -> usize { match s { State::Active => 0, State::NotedAsStructure => 1, State::NotEvaluated(_) => 2 } }

// ═══════════════════════════════════════════════════════════════════════════
// (2) The ground record: all three, for every relation, component and step
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn every_relation_component_and_step_carries_all_three_variables() {
    // Each state is the one its parts give: energy active where an exchange
    // or a change of exchange is resolved; freedom active where PC1, or a PC2
    // or PC3 reading involving the relation, reads freedom; connection active
    // where the imbalance changes state or a pair through the relation
    // reorganizes. Every state occurs for each variable over the family.
    let seen = Mutex::new([[0usize; 3]; 3]);
    for_all("ground_record", 0x257, |r| {
        let rel = enumerated_rel(r);
        let (k, regs) = (r.usize(2, 3), r.usize(2, 4));
        let z = r.selects(4);
        let d = enumerated_declared(r, &rel, k, regs, 0.25, z, z);
        let held = r.selects(6);
        let ts: Vec<EnergyTransfer> = if r.selects(4) {
            let e = r.usize(0, rel.n_edges() - 1);
            vec![transfer(r, TransferSite::Edge(e), regs - 1)]
        } else { vec![] };
        let df = declared(&rel, &d, domain(&d, held, ts, vec![]));
        assert_eq!(df.records.len(), rel.n_edges() * k * (regs - 1), "every (e, c, s) has a record");
        let pc = df.field.ground.freedom.as_ref().unwrap();
        let mut g = seen.lock().unwrap();
        for rec in &df.records {
            let (e, c, s) = (rec.edge, rec.component, rec.step);
            // Energy.
            let any_energy = !rec.energy.held.is_empty() || !rec.energy.transfer.is_empty();
            let active = rec.energy.held.iter().any(|h| h.1.resolved || h.2.resolved || h.3 == Reading::Freedom)
                || rec.energy.transfer.iter().any(|t| t.1.resolved || t.2.map_or(false, |x| x.resolved));
            match &rec.energy.state {
                State::NotEvaluated(why) => { assert!(!any_energy); assert!(!why.is_empty(), "the reason is named"); }
                State::Active => assert!(active),
                State::NotedAsStructure => assert!(any_energy && !active),
            }
            // Freedom, against the field's own PC readings.
            let pc1 = pc.relations.iter().find(|x| x.step == s && x.component == c && x.edge == e).unwrap().reading;
            assert_eq!(rec.freedom.pc1, pc1);
            let pc2_free = [&pc.across_components, &pc.across_steps].iter().filter_map(|f| f.as_ref().ok())
                .flat_map(|f| f.readings.iter()).filter(|x| (x.e == e || x.f == e) && x.reading == Reading::Freedom && match x.over {
                    ReadingPair::Components { step, i, j } => step == s && (i == c || j == c),
                    ReadingPair::Steps { component, step } => step == s && component == c,
                }).count();
            assert_eq!(rec.freedom.pc2.0, pc2_free);
            let active_f = pc1 == Reading::Freedom || rec.freedom.pc2.0 > 0 || rec.freedom.pc3.0 > 0;
            assert_eq!(rec.freedom.state == State::Active, active_f);
            assert_ne!(state_kind(&rec.freedom.state), 2, "freedom is always read on a field");
            // Connection.
            if let Some((a, b)) = rec.connection.imbalance {
                if a != b { assert_eq!(rec.connection.state, State::Active); }
            }
            g[0][state_kind(&rec.energy.state)] += 1;
            g[1][state_kind(&rec.freedom.state)] += 1;
            g[2][state_kind(&rec.connection.state)] += 1;
        }
    });
    let g = seen.into_inner().unwrap();
    assert!(g[0].iter().all(|&n| n > 0), "energy active / noted / not evaluated: {:?}", g[0]);
    assert!(g[1][0] > 0 && g[1][1] > 0, "freedom active / noted: {:?}", g[1]);
    assert!(g[2][0] > 0 && g[2][1] > 0, "connection active / noted: {:?}", g[2]);
}

#[test]
fn with_no_energy_declared_every_energy_entry_names_the_gap() {
    let mut r = wf::Seeds(0x258);
    let rel = enumerated_rel(&mut r);
    let d = enumerated_declared(&mut r, &rel, 2, 3, 0.25, false, false);
    let df = declared(&rel, &d, domain(&d, false, vec![], vec![]));
    assert!(df.energy_not_declared());
    assert!(df.records.iter().all(|x| x.energy.state == State::NotEvaluated("no energy expression declared (R9a, W11)".into())));
    // Freedom and connection are still read on every record.
    assert!(df.records.iter().all(|x| state_kind(&x.freedom.state) < 2));
}

// ═══════════════════════════════════════════════════════════════════════════
// (1) Energy declared once: held, with every part of δ
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn held_energy_is_declared_once_with_every_part_of_delta() {
    // The held exchange is the carrier component's state reading, offsets and
    // gains included, on every field (I-SC-D17 (i), now without its scope
    // limit). The 10.3.0 path (an `EnergyCarrier` on repeatability alone)
    // differs on some fields with run-constant parts: the finding of the 53rd,
    // answered. Held energy at loci is reported with its state bound.
    let differs = Mutex::new(0usize);
    for_all("held_once", 0x259, |r| {
        let rel = enumerated_rel(r);
        let (k, regs) = (r.usize(2, 3), r.usize(2, 4));
        let d = enumerated_declared(r, &rel, k, regs, 0.5, true, true);
        let df = declared(&rel, &d, domain(&d, true, vec![], vec![]));
        let c = k - 1;
        let h = &df.energy.held[0];
        let regs_ = registrations(&d);
        for t in 0..regs {
            for v in 0..rel.n_nodes {
                assert_eq!(h.at_loci[t][v].value, d.values[t][c][v]);
                assert_eq!(h.at_loci[t][v].bound, regs_[t].state_bound(c, v).up());
            }
            let old = abr_kernel::ground_variables::signed_exchanges(&EnergyCarrier { range: ObservedRange::synthetic(),
                name: "the 10.3.0 path".into(), unit: "synthetic".into(), values: d.values[t][c].clone(), delta: d.delta[c].clone() }, &rel);
            for e in 0..rel.n_edges() {
                let a = df.series.entry(t, c, e);
                assert_eq!(h.exchange[t][e].resolved, exceeds(&a, &Expansion::of(df.series.da[t][c][e])), "the component's state reading");
                // The signed exchange (104th). *Was:* `a.abs().nearest()`.
                assert_eq!(h.exchange[t][e].value, a.nearest());
                assert_eq!(old[e].value, h.exchange[t][e].value, "the same movement on both paths");
                let old_floor = old[e].side != abr_kernel::ground_variables::Side::NotResolved;
                if old_floor != h.exchange[t][e].resolved { *differs.lock().unwrap() += 1; }
            }
        }
    });
    assert!(differs.into_inner().unwrap() > 0, "the earlier path, without run-constant parts, differs on some fields");
}

// ═══════════════════════════════════════════════════════════════════════════
// (1) Energy transferred across a relation
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn a_transfer_is_read_per_step_with_its_change_and_its_gain_in_magnitudes_only() {
    let seen = Mutex::new([0usize; 4]);
    for_all("transfer", 0x25A, |r| {
        let rel = enumerated_rel(r);
        let regs = r.usize(3, 5);
        let d = enumerated_declared(r, &rel, 2, regs, 0.25, false, false);
        let e = r.usize(0, rel.n_edges() - 1);
        let t = transfer(r, TransferSite::Edge(e), regs - 1);
        let df = declared(&rel, &d, domain(&d, false, vec![t.clone()], vec![]));
        let x = &df.energy.transfers[0];
        for s in 0..regs - 1 {
            assert_eq!(x.per_step[s].resolved, t.values[s].abs() > t.delta[s]);
            assert!(x.magnitude_bound[s] >= t.delta[s] + t.gain * (t.values[s].abs() + t.delta[s]));
            if s + 1 < regs - 1 {
                assert_eq!(x.change[s].resolved, (t.values[s + 1] - t.values[s]).abs() > t.delta[s] + t.delta[s + 1]);
            }
            let rec = df.record(e, 0, s);
            assert_eq!(rec.energy.transfer.len(), 1);
            assert_eq!(rec.energy.transfer[0].1, x.per_step[s]);
            let mut g = seen.lock().unwrap();
            g[x.per_step[s].resolved as usize] += 1;
            if let Some(ch) = rec.energy.transfer[0].2 { g[2 + ch.resolved as usize] += 1; }
            // Every other relation: energy not registered there, and named.
            for f in (0..rel.n_edges()).filter(|&f| f != e) {
                assert!(matches!(&df.record(f, 0, s).energy.state, State::NotEvaluated(w) if w.contains("not registered at this relation")));
            }
        }
        // A shared gain scales value and bound together: no classification
        // moves (I-SC-D15); scale by 2 with δ scaled.
        let scaled = EnergyTransfer { values: t.values.iter().map(|v| 2.0 * v).collect(), delta: t.delta.iter().map(|v| 2.0 * v).collect(), ..t };
        let ds = declared(&rel, &d, domain(&d, false, vec![scaled], vec![]));
        let cls = |f: &DeclaredField| f.energy.transfers[0].per_step.iter().chain(&f.energy.transfers[0].change).map(|x| x.resolved).collect::<Vec<_>>();
        assert_eq!(cls(&df), cls(&ds));
    });
    let g = seen.into_inner().unwrap();
    assert!(g.iter().all(|&n| n > 0), "transfer held / resolved, change held / resolved: {g:?}");
}

// ═══════════════════════════════════════════════════════════════════════════
// The worked case, shaped like the vessel record (application example)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn worked_energy_entering_a_locus_from_a_supply_outside_the_field() {
    // *Example (application), synthetic values:* loci hs (heater surface),
    // bc, bw, mc, mw; relations hs → bc, hs → bw, bc → mc, bw → mw, bc → bw.
    // Components T and G, structural. Energy enters hs from a supply outside
    // the field (a pulse meter) and orders the sequence. No carrier is held at
    // the sites. Each record of a relation leaving hs carries the supply as
    // context at its source; energy at every relation is not evaluated, named.
    let rel = rel(5, vec![(0, 1), (0, 2), (1, 3), (2, 4), (1, 2)]);
    let values = vec![
        vec![vec![1.0, 0.5, 0.5, 0.25, 0.25], vec![2.0, 2.0, 2.0, 2.0, 2.0]],
        vec![vec![2.0, 1.0, 0.75, 0.5, 0.25], vec![2.0, 2.0, 2.0, 2.0, 2.0]],
        vec![vec![3.0, 2.0, 1.25, 1.0, 0.5], vec![2.5, 2.25, 2.0, 2.0, 2.0]],
    ];
    let d = Declared::repeatability(values, vec![vec![1.0 / 32.0; 5]; 2]);
    let supply = EnergyTransfer { name: "SYNTHETIC heater supply".into(), unit: "pulses".into(),
        site: TransferSite::Entering { locus: 0, source: "SYNTHETIC supply meter".into() },
        values: vec![4.0, 4.0], delta: vec![0.5, 0.5], gain: 1.0 / 64.0,
        mapping: Some("SYNTHETIC: J per pulse, declared".into()), participation: EnergyParticipation::Supplied,
        range: ObservedRange::synthetic() };
    let constraints = vec![Constraint::Relations { name: "SYNTHETIC wall".into(), edges: vec![4] }];
    let decl = domain(&d, false, vec![supply], constraints);
    let df = declared(&rel, &d, decl);
    assert_eq!(df.energy.ordering.len(), 1);
    for s in 0..2 {
        for e in 0..5 {
            let rec = df.record(e, 0, s);
            assert!(matches!(&rec.energy.state, State::NotEvaluated(w) if w.contains("present by ground (G1)")));
            assert_eq!(rec.energy.entering_at_source.len(), (rel.edges[e].0 == 0) as usize);
        }
        assert!(df.record(0, 0, s).energy.entering_at_source[0].1.resolved, "the supply is resolved at hs");
    }
    assert_eq!(df.record(4, 0, 0).connection.constraints, vec!["SYNTHETIC wall".to_string()]);
    // Freedom: T changes at the heater relation at both steps; G holds at step 0.
    assert_eq!(df.record(0, 0, 0).freedom.pc1, Reading::Freedom);
    assert_eq!(df.record(0, 1, 0).freedom.state, State::NotedAsStructure);
}

// ═══════════════════════════════════════════════════════════════════════════
// Refusal, not repair
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn a_declaration_that_does_not_match_its_registrations_is_refused() {
    let mut r = wf::Seeds(0x25B);
    let rel = enumerated_rel(&mut r);
    let d = enumerated_declared(&mut r, &rel, 2, 3, 0.25, false, false);
    let regs = registrations(&d);
    let ok = domain(&d, true, vec![], vec![]);
    let refuse = |decl: DomainDeclaration| DeclaredField::new(&rel, regs.clone(), decl).unwrap_err();
    let mut x = ok.clone(); x.freedom.components.pop();
    assert_eq!(refuse(x), DomainRefusal::Components { declared: 1, registered: 2 });
    let mut x = ok.clone(); x.freedom.components[1].1 = ComponentRole::Structural;
    assert_eq!(refuse(x), DomainRefusal::HeldCarrier(1));
    let mut x = ok.clone(); x.freedom.components[0].1 = ComponentRole::EnergyHeld;
    assert_eq!(refuse(x), DomainRefusal::HeldCarrier(0));
    let short = EnergyTransfer { values: vec![1.0], delta: vec![0.1], ..transfer(&mut r, TransferSite::Edge(0), 2) };
    assert!(matches!(refuse(domain(&d, false, vec![short], vec![])), DomainRefusal::Transfer(_)));
    let off = transfer(&mut r, TransferSite::Edge(rel.n_edges()), 2);
    assert!(matches!(refuse(domain(&d, false, vec![off], vec![])), DomainRefusal::Transfer(_)));
    let wall = Constraint::Relations { name: "SYNTHETIC".into(), edges: vec![rel.n_edges()] };
    assert!(matches!(refuse(domain(&d, false, vec![], vec![wall])), DomainRefusal::Constraint(_)));
}

// ═══════════════════════════════════════════════════════════════════════════
// (3) Boundaries of stages, through the kernel's own list (I-SC-D18)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn every_reversal_boundary_has_freedom_among_its_relations() {
    // `persistence_boundaries` on declared whole fields: a Reversed boundary
    // always carries freedom among its relations (D18 (i)); Began and Ended
    // boundaries occur without it (D18 (ii)). Each kind occurs, for each
    // persistence.
    let seen = Mutex::new([[0usize; 4]; 2]);
    for_all("boundaries", 0x25C, |r| {
        let rel = enumerated_rel(r);
        let (k, regs) = (r.usize(2, 3), r.usize(2, 4));
        let z = r.selects(4);
        let d = enumerated_declared(r, &rel, k, regs, 0.25, z, z);
        let df = declared(&rel, &d, domain(&d, true, vec![], vec![]));
        let mut g = seen.lock().unwrap();
        for b in persistence_boundaries(&df) {
            let p = matches!(b.persistence, Persistence::Arrangement { .. }) as usize;
            match b.kind {
                BoundaryKind::Reversed => { assert!(b.freedom_among_its_relations); g[p][0] += 1; }
                BoundaryKind::Began if !b.freedom_among_its_relations => g[p][1] += 1,
                BoundaryKind::Ended if !b.freedom_among_its_relations => g[p][2] += 1,
                _ => g[p][3] += 1,
            }
            assert_ne!(b.before, b.after);
            assert!(!(b.before == AtDelta::NotResolved && b.after == AtDelta::NotResolved));
        }
    });
    let g = seen.into_inner().unwrap();
    assert!(g.iter().all(|row| row[0] > 0 && row[1] > 0 && row[2] > 0),
        "imbalance, arrangement: reversed / began held / ended held / with freedom: {g:?}");
}

#[test]
fn worked_energy_leaving_a_locus_into_a_sink_outside_the_field() {
    // Energy removed (58th): a transfer out of locus 2 into a named sink.
    // Each record of a relation ending at locus 2 carries it as context at
    // its target; energy at every relation is not registered there, named.
    let rel = rel(3, vec![(0, 1), (1, 2), (0, 2)]);
    let values = vec![
        vec![vec![2.0, 1.0, 0.5], vec![1.0, 1.0, 1.0]],
        vec![vec![1.5, 0.75, 0.0], vec![1.0, 1.0, 1.0]],
        vec![vec![1.0, 0.5, -0.5], vec![1.0, 1.0, 1.0]],
    ];
    let d = Declared::repeatability(values, vec![vec![1.0 / 32.0; 3]; 2]);
    let sink = EnergyTransfer { name: "SYNTHETIC energy removed".into(), unit: "pulses".into(),
        site: TransferSite::Leaving { locus: 2, sink: "SYNTHETIC metered sink".into() },
        values: vec![3.0, 3.0], delta: vec![0.5, 0.5], gain: 0.0, mapping: None,
        participation: EnergyParticipation::Removed, range: ObservedRange::synthetic() };
    let df = declared(&rel, &d, domain(&d, false, vec![sink], vec![]));
    for s in 0..2 {
        for e in 0..3 {
            let rec = df.record(e, 0, s);
            assert_eq!(rec.energy.leaving_at_target.len(), (rel.edges[e].1 == 2) as usize);
            assert!(rec.energy.entering_at_source.is_empty());
            assert!(matches!(&rec.energy.state, State::NotEvaluated(w) if w.contains("present by ground (G1)")));
        }
    }
    assert_eq!(df.energy.transfers[0].participation, EnergyParticipation::Removed);
    let off = EnergyTransfer { site: TransferSite::Leaving { locus: 3, sink: "SYNTHETIC".into() }, ..df.declaration.energy.iter()
        .find_map(|x| if let EnergyExpression::Transferred(t) = x { Some(t.clone()) } else { None }).unwrap() };
    assert!(matches!(DeclaredField::new(&rel, registrations(&d), domain(&d, false, vec![off], vec![])).unwrap_err(), DomainRefusal::Transfer(_)));
}
