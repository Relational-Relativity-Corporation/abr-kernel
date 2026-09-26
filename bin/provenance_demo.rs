// provenance_demo.rs — Metatron Dynamics, Inc. Kernel V8.
//
// Minimal public demonstration of Observable Provenance and Reverse
// Traceability (see observable_provenance_and_reverse_traceability.md).
//
// This is not a new invariant and not new physics. It takes exactly
// one already-declared, already-documented case from this kernel —
// the H-alpha (Balmer-alpha) photon edge, sourced from NIST ASD v5.12,
// documented in the comments above epsilon_photon_edge() and
// n_relational_cycles() in derived_invariants.rs — and demonstrates
// the one thing that was not previously executable: given only the
// record, reconstruct the result from its recorded input and identified
// calculation.
//
// Before this file: the H-alpha case's input and source existed as a
// code comment a human could read. A bare f64 returned from
// epsilon_photon_edge() carried none of that with it.
// After this file: given a ProvenanceRecord, the numerical result can be
// reconstructed from its recorded input and identified calculation
// without external session context. The record also carries the declared
// source attribution required to trace that input back to its observable
// record. Verification of the source attribution itself requires
// comparison with that source.
//
// Kernel V8 (2026-09-26): built against metatron_kernel_v8. Wording
// synchronized with the Kernel V8 O → M → D declaration and vocabulary, and
// narrowed per Verifier disposition to what the code establishes:
// reconstruction of a calculation from recorded provenance, distinct from
// independent verification of the observable source. No computation,
// input value, or test expectation changed.
//
// Bounded over D. No claim beyond D.

use metatron_kernel_v8::derived_invariants::{epsilon_photon_edge, H_PLANCK, C_DECLARED};

/// A provenance-carrying wrapper around a single ε[e] computation.
///
/// This is the minimal structural change the kernel needs, applied to
/// exactly one function, as a worked example rather than a claim that
/// the whole kernel has been converted. See
/// observable_provenance_and_reverse_traceability.md, "Minimum
/// requirement for a reverse-traceable invariant."
#[derive(Debug, Clone)]
pub struct ProvenanceRecord {
    /// The computed value. Never handed out without the fields below.
    pub value_joules: f64,

    /// The numerical input declared through M and used in this
    /// calculation — the actual input value, not a description.
    pub declared_wavelength_m: f64,

    /// The declared source attribution for the input: the named record
    /// the input is stated to come from. The record carries this
    /// attribution; it does not verify it. Verifying the attribution
    /// requires comparison with the named source.
    pub observable_source: &'static str,

    /// Which operator/function computed this value from the
    /// declared input. Named explicitly so the reverse check
    /// knows what to re-run.
    pub operator: &'static str,
}

impl ProvenanceRecord {
    /// Forward direction: declared input -> operator -> value, with the
    /// recorded input, source attribution, and calculation attached at
    /// construction, not after.
    pub fn compute_epsilon_photon_edge(
        lambda_m: f64,
        observable_source: &'static str,
    ) -> Self {
        let value_joules = epsilon_photon_edge(lambda_m);
        ProvenanceRecord {
            value_joules,
            declared_wavelength_m: lambda_m,
            observable_source,
            operator: "epsilon_photon_edge (hc/lambda, derived_invariants.rs)",
        }
    }

    /// Reverse direction: given only this record, recompute the value
    /// from its own recorded input and check that the stored result
    /// reconstructs. Does not verify the source attribution. This is reconstruction from recorded
    /// provenance, not mathematical inversion of the operator — see
    /// observable_provenance_and_reverse_traceability.md's explicit
    /// distinction between the two.
    pub fn verify_reverse_traceable(&self) -> Result<(), String> {
        let recomputed = epsilon_photon_edge(self.declared_wavelength_m);
        let diff = (recomputed - self.value_joules).abs();
        // IMPLEMENTATION-ONLY NUMERICAL TOLERANCE (J): guards finite-precision
        // arithmetic. Not a measurement-provenance threshold.
        if diff < 1e-30 {
            Ok(())
        } else {
            Err(format!(
                "REVERSE CHECK: stored result does not reconstruct from \
                 recorded input — stored value {:.6e} J \
                 (wavelength {:.6e} m, source: {}); recomputed {:.6e} J, \
                 diff {:.6e}",
                self.value_joules, self.declared_wavelength_m,
                self.observable_source, recomputed, diff
            ))
        }
    }
}

fn main() {
    println!("Observable Provenance and Reverse Traceability — minimal demonstration\n");
    println!("Constants used (declared in derived_invariants.rs):");
    println!("  H_PLANCK   = {:.9e} J*s", H_PLANCK);
    println!("  C_DECLARED = {:.9e} m/s\n", C_DECLARED);

    // The H-alpha (Balmer-alpha) case, exactly as already documented
    // in derived_invariants.rs above n_relational_cycles():
    //   lambda = 656.279 nm, NIST ASD v5.12
    //   epsilon[H-alpha] = hc / 656.279 nm = 3.027e-19 J (documented)
    let h_alpha_wavelength_m = 656.279e-9;
    let record = ProvenanceRecord::compute_epsilon_photon_edge(
        h_alpha_wavelength_m,
        "NIST ASD v5.12, H n=3->2 transition (Balmer-alpha / H-alpha)",
    );

    println!("Forward computation:");
    println!("  Declared input through M: lambda = {:.3e} m", record.declared_wavelength_m);
    println!("  Source: {}", record.observable_source);
    println!("  Operator: {}", record.operator);
    println!("  Result: epsilon[e] = {:.6e} J\n", record.value_joules);

    println!("Documented reference value (derived_invariants.rs comment): 3.027e-19 J");
    let doc_diff = (record.value_joules - 3.027e-19).abs();
    println!("  Computed vs. documented diff: {:.3e} J\n", doc_diff);

    println!("Reverse check — given ONLY the ProvenanceRecord, with no\n\
              other context, reconstruct the value from its recorded input:");
    match record.verify_reverse_traceable() {
        Ok(()) => println!("  REVERSE-TRACEABLE: value reconstructs exactly from its own \
                             recorded input and operator."),
        Err(e) => println!("  {}", e),
    }

    println!("\nWhat this demonstrates:");
    println!("  Before: epsilon_photon_edge() returns a bare f64. Its provenance\n\
              \x20 (lambda = 656.279 nm, NIST ASD v5.12) existed only as a code\n\
              \x20 comment — readable by a person, invisible to a program.");
    println!("  After: given only a ProvenanceRecord, with no external session\n\
              \x20 context, the result reconstructs from its recorded input and\n\
              \x20 identified calculation. The record also carries the declared\n\
              \x20 source attribution for that input. Verifying the attribution\n\
              \x20 itself requires comparison with the named source.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h_alpha_matches_documented_value() {
        let record = ProvenanceRecord::compute_epsilon_photon_edge(
            656.279e-9,
            "NIST ASD v5.12, H n=3->2 transition (Balmer-alpha / H-alpha)",
        );
        // Documented in derived_invariants.rs: epsilon[H-alpha] = 3.027e-19 J
        let diff = (record.value_joules - 3.027e-19).abs();
        assert!(diff < 1e-21, "computed value should match documented reference");
    }

    #[test]
    fn reverse_check_passes_for_reconstructible_record() {
        let record = ProvenanceRecord::compute_epsilon_photon_edge(
            656.279e-9,
            "NIST ASD v5.12, H n=3->2 transition (Balmer-alpha / H-alpha)",
        );
        assert!(record.verify_reverse_traceable().is_ok());
    }

    #[test]
    fn reverse_check_errs_when_value_does_not_reconstruct() {
        // Altered record: the value does not match what its own recorded
        // wavelength produces. This is the case the reverse check exists
        // to report — a value that does not reconstruct from its recorded
        // provenance.
        let mut record = ProvenanceRecord::compute_epsilon_photon_edge(
            656.279e-9,
            "NIST ASD v5.12, H n=3->2 transition (Balmer-alpha / H-alpha)",
        );
        record.value_joules = 9.999e-19; // deliberately wrong
        assert!(record.verify_reverse_traceable().is_err());
    }
}
