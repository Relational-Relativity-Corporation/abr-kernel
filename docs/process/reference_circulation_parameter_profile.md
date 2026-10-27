# Reference circulation parameter profile — Origin proposal

**Status:** proposed reference profile for Origin/Verifier review; does not alter the frozen Kernel V9 operators, existing admission requirements, or kernel defaults.

## Historical basis and intended use

The V5 reference implementation used the saturation form `ρ = rho_base × m/(1+m)`, and its tests repeatedly exercised `rho_base = 0.3`. The denominator's fixed `1` was subsequently replaced by explicit `χ₀` to avoid an undeclared unit choice (V7.1). The historical numerical use of 0.3 is documented; its adequacy across applications is not established by those tests alone.

The **candidate reference profile** is:

| Parameter | Reference value | Condition |
| --- | --- | --- |
| `rho_base` | `0.3` | Dimensionless; historical test value, candidate general starting point |
| `χ₀` | `1.0` **in the declared unit of χ** | M must explicitly supply the χ unit and reference scale; not a unitless kernel constant |
| `cc[p]` | **No reference default** | Required only when a component pair is declared; its coupling must be specified by M |

These are **candidate application-level reference values**, not universally validated constants. An application may explicitly select this profile through its declared M, with the actual dimensional χ₀ stated. The kernel must not silently insert these values when M has not supplied them.

## Mathematical function

`χ[i] = max |A(x)[e]|` over incident declared edges (or Δ in the Primary Region), and

`ρ[i] = rho_base × χ[i]/(χ₀ + χ[i])`.

For positive χ₀ and rho_base, `0 ≤ ρ < rho_base`, and `ρ = rho_base/2` at `χ = χ₀`. This limits the **coupling coefficient**, not the unweighted circulation imbalance or the full R/Σ output. Its effect on numerical scale is directly calculable even where the original rationale was not narrated in historical prose.

## Domain and scale variability

- The operator forms remain invariant. The measured scale, units, topology, component coupling, and numerical adequacy can vary across domains. Every parameter may need reconsideration in a particular application.
- A reference profile is a reproducible starting point, **not** a demand to recalibrate all parameters in every new domain. Compare computed coupling and circulation with the declared domain; override when those calculations or observations demonstrate the need.
- The Origin expects the historical configuration to be useful for many prior application domains. This is a testable expectation, not yet a cross-domain validation result.
- **Primary Region:** Δ → Σ omits B, so inspect numerical behavior separately; do not assume this reference profile transfers unchanged.

## Verification and adoption

1. Reconstruct historical parameter values from prior repositories and tests.
2. For each declared domain, report χ range, χ₀, rho_base, computed ρ range, unweighted antisymmetric imbalance, and weighted contribution; assess scale sensitivity and comparison with observable data.
3. Record any overrides and their observed/calculated rationale in the application's M and calculation record.
4. Obtain Origin disposition and Verifier review before promoting this **candidate** into an adopted application reference profile. Kernel V9 still has **no implicit numerical defaults**.

Related discussion: [issue #1](https://github.com/Relational-Relativity-Corporation/abr-kernel/issues/1). Canonical operator notation: [operators_notation_and_constraint.md](kernel/operators_notation_and_constraint.md).
