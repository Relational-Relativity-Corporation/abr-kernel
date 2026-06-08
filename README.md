# ABR Invariant Relational Kernel — V4

**Metatron Dynamics, Inc.**
relationalrelativity.dev | arXiv:2601.22389

---

This repository contains the V4 ABR kernel: formal specification,
role separation protocol, plain language constraints, and
reference implementation.

The kernel states operator constraints and admissibility conditions.
Interpretation, application, and verification remain outside the kernel.

---

## Contents

**`operators_notation_and_constraint.md`**
Formal operator definitions and constraints. The notation says what
the operators are. The constraints say what may not be added.

**`role_separation_and_operator_application.md`**
Role separation protocol and application workflow. Declares the
minimum role structure required for the kernel to function correctly.

**`abr_operators_plain.md`**
Plain language statement of each operator constraint. No mathematics
required. The reader remains Origin.

**`operators.rs`**
V4 reference implementation in Rust. Declared relations only.
No ring, no torus, no undeclared topology.

---

## Scope

All definitions bounded over D := { x ∈ ℝⁿ | n < ∞, |x[i]| < ∞ }.
No claim beyond D.

---

*Metatron Dynamics, Inc. — Delaware C-Corp #10551645*