# The Operators — Notation and Constraint

**Metatron Dynamics, Inc.** Reference data. Bounded over D. No claim beyond D.

**Domain.** D := { x ∈ ℝⁿ | n < ∞, |x[i]| < ∞ }. M : O → D, declared by Origin before anything is taken. The kernel acts on M(o) only.

**Relations.** A declared relation is a directed edge e = (s, t) — observed through M, or declared by Origin. *Constraint:* every relation has one of these two sources. A relation with neither is fabricated, even when it computes.

**Composition.** E(x, ρ) = R(B(A(x)), ρ(A(x))). The order A → B → R is fixed: each operator's input is its predecessor's output. C is a declared projection, not a kernel operator.

**A — relational gradient.** A(x)[e] = x[s] − x[t] over each declared edge e = (s, t); component edges x[a][i] − x[b][i] over declared pairs. *Constraint:* take the directed difference and nothing else — add no relation and no direction the declaration did not contain.

**B — local accumulation.** B(g)[e] = g[e] + Σ_{f ∈ succ(e)} g[f], same direction only. *Constraint:* accumulate only along declared continuation. A terminal edge accumulates nothing; no boundary is closed to supply one.

**R — antisymmetric circulation.** R(g)[e] = g[e] + ρ[e]·( Σ_{succ(e)} g − Σ_{pred(e)} g ); where spatial and component relations cross, the asymmetry of each enters the other. *Constraint:* couple by the asymmetry present. Do not assume the two directions equal — symmetry is a relation, and holds only if it was declared.

**ρ — per-node strength.** ρ[i] = ρ_base · m[i] / (1 + m[i]), with m[i] the largest gradient at i. *Constraint:* derived per node from A(x); no aggregation beyond the node.

**C — declared projection.** Any reduction of the edge field — to one value per node, to a bound, to σ². *Constraint:* state what it preserves and what it discards. No silent reduction.

**Before A.** *Constraint:* no transformation that alters pairwise differences precedes A unless declared within M. Admissible: uniform shift, declared unit scale.

*The notation says what the operators are; the constraints say what may not be added. Bounded over D. No claim beyond D.*