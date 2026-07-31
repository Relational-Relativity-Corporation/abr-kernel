# The Operators — Plain Language

**Metatron Dynamics, Inc.** V8. Bounded over D. No claim beyond D.

---

## The primary observable and mathematical primitive

**Change is the primary observable.**

Before any variable is declared. Before any locus is named. Before any operator acts. What M reports — at every scale, in every domain, across every declared region — is that something differs from something else. This is the irreducible observable within the bounded domain of human perceptual and cognitive information.

**Relation is the invariant structure of change.**

Change across a declared relation has a source, a target, and a direction traceable to an observable through M. Without a declared relation, change is present but unlocated. With a declared relation, change becomes the directed difference — the input to every operator in this framework.

The operators Δ and A are the same formula: x[s] − x[t]. They are the minimal mathematical expression of the primary observable. Every other operator — B, R, Σ, C — acts on what Δ and A produce. The operators do not produce change. They read it.

This claim is scoped to the bounded domain of human perceptual and cognitive information — to what instruments and cognition can declare through M into D. Bounded over D. No claim beyond D.

---

A statement is admissible when every quantity in it can be traced to an observable through a declared measurement mapping M.
When it cannot, the statement is inadmissible — regardless of whether it computes, sounds reasonable, or produces a result.

Declarability is necessary but not sufficient. A quantity that is declared but not traceable to an observable through M is not inadmissible — it is simply not about anything in D. It exists only within the formal language.

Declaration is never a substitute for traceability to an observable through M. Declaration is the act of stating traceability to an observable through M and taking responsibility for it. A relation, quantity, or structure that cannot be traced to an observable through M is fabricated within D — regardless of who declared it, regardless of whether it computes, and regardless of whether it sounds reasonable. The admissibility condition is not relaxed by the act of declaration. It is what declaration is required to satisfy.

The operators below make those constraints explicit.
They do not add structure. They require that structure already present be declared before it is used.

The reader remains Origin.
This document states constraints.
Verification remains available to you directly.

---

## Before anything is measured

A domain must be declared before any operator acts.
A measurement mapping M must be declared before any operator acts.
The operators act on M(o) only — on what M produces from the observable, and nothing else.

*Constraint: there is nothing to evaluate before something is declared.*

**For Phase 2 (persistence), M must declare a sequence, not a single observation.**

Phase 1 acts on one declared observation M(o). This is admissible.

Phase 2 acts on the directed difference between two consecutive kernel outputs — the current one and the prior one. For this to be admissible, M must declare a sequence of actual observations: {M(o₁), M(o₂), ..., M(oₙ)}, where each oₖ is a real observable at one declared relational step, and the relational step is the system's own declared process step, not an externally imposed clock increment.

A sequence of model-generated values is not a sequence of observations through M. It is a formal construction. It may be declared within M only when the model itself is declared as a transducer from prior observable inputs — not as a substitute for observation.

On the first declared step, E_prior = zero is admissible. This is the cold start: no relational history has been declared. What Phase 2 produces on a cold start is the relational structure of the current observation against no prior history. This is not relational evolution. Treating it as such is inadmissible.

After the first step, E_prior must come from the preceding actual observation in the declared sequence.

When the observable does not change between two declared steps — when M(oₖ) = M(oₖ₋₁) — A_persistence = zero. This is the correct result. It states that no relational evolution occurred across that step. It is not a failure of the operators.

*Constraint: Phase 2 requires a declared sequence of actual observations. Cold start is admissible on the first step only. Model-generated sequences are not observations through M.*

---

## Every relation has one direction

Every declared relation has exactly one admissible direction — the direction traceable to an observable through M.

The direction of a relation is not a choice. It is determined by what was observed. If you observed something moving from s to t, the direction is s → t. Declaring the reverse — t → s — requires a separate observable that supports that direction independently. If you do not have one, the reverse direction is not traceable to an observable through M. It is inadmissible.

This applies to every kind of declared relation: spatial relations between measurement loci, transitions between particle configurations, evolution from one state to the next. In every case, the direction is determined by the observable, not by the declaring party's preference.

**What this means for pairs of relations:**

If both (s,t) and (t,s) are declared, each direction must be independently traceable to an observable through M. When both directions have independent provenance, they are treated as distinct relations — not as the same relation observed from two sides. If one direction is simply the mathematical negation of the other with no independent observable supporting it, it is inadmissible.

Within the declared admissibility conditions of this framework, a declared structure where every edge has a matching reverse edge signals a declaration error — not a legitimate relational configuration. The operators will detect this and report it.

**Why rings are inadmissible:**

A ring declares that a node is its own relational predecessor through a chain: 0 → 1 → 2 → ... → n−1 → 0. Following this chain, the state at node 0 depends on a chain that includes node 0 as its own input. Within the declared admissibility conditions, no quantity has itself as a relational predecessor — such a quantity would need to solve an equation involving itself to have a definite value, and that solution is not an observable. The closing edge of a ring therefore cannot be traced to an observable through M. It is inadmissible. The ring fails at the closing edge, and the closing edge is the ring.

*Constraint: every declared relation must have a single admissible direction independently traceable to an observable through M. The reverse direction requires independent provenance. Without it, it is inadmissible.*

---

## Relational evolution has a direction

When a process unfolds across declared relational steps, that unfolding has a direction. The direction is a property of the process — not a property of the index used to label the steps.

Writing the observations as o₁, o₂, o₃ assigns labels. It does not declare direction. The number 2 follows 1 by arithmetic convention. Whether o₂ follows o₁ in the process requires a separate declaration: what observable property of the process establishes that oₖ₊₁ is later in the process than oₖ?

For a physical process, this is always answerable from the observable. The declared process step has an identifiable direction because the process itself moves in one direction — and that direction is traceable to what was observed, not to what the index says. An oscillating system moves from floor through ascent, dwell, expression, and recovery — one declared process step in one direction. Energy transfer moves from higher relational contrast toward lower. A conformational change proceeds from one declared state toward another that the process itself distinguishes. A decaying system moves irreversibly from one configuration toward its terminal state. A growing system moves from one scale of declared structure toward a larger one. None of these require recurrence or periodicity. What they share is that the direction is readable from the observable — not assigned by the analyst, not inherited from the index.

**Temporal ordering is not the primitive.** Time is a downstream projection — a way of labeling the steps of relational evolution after the direction has been identified from the observable. Saying "o₂ came after o₁ in time" is only admissible when what made it later is traceable to an observable property of the process, not to a clock reading. A clock reading is itself a relational observable in its own domain. It does not transfer its direction to a different process without a declared connection between the two.

This means:

A sequence of observations ordered by a clock is admissible only when the clock's relational evolution and the process's relational evolution are declared to be in the same direction by an observable connection between them. The clock does not establish the direction of the process; the process establishes its own direction, and the clock may be one way of tracking it.

A sequence ordered by index alone — o₁, o₂, o₃ because we numbered them that way — carries no declared direction of relational evolution. It is a list, not a process.

*Constraint: the direction of relational evolution must be declared by identifying the observable property of the process that establishes which observation is prior and which is current. Index order is not a direction declaration. Temporal order is a declared projection of relational evolution, not its source. A declared relational evolution direction that cannot be traced to an observable property of the process is fabricated within D.*

---

## Δ and Σ — the primary operators

At the smallest scale of declared relational structure — where there is no confirmed path interior and no accumulated relational history — two questions are irreducible:

1. Does anything distinguishable exist across the declared relations?
2. Is what exists symmetrically or asymmetrically organized in its immediate neighborhood?

**Δ — directed difference** asks the first question.

Δ takes the directed difference of the observable field across each declared relation: Δ(x)[e] = x[s] − x[t] for each declared edge e = (s, t). It produces one value per declared relation — the contrast across that relation in the direction it was declared.

*Constraint: take the directed difference and nothing else. No relation and no direction may be added that the declaration did not trace to an observable through M.*

**Σ — local antisymmetry** asks the second question.

Σ takes the Δ output and asks: at each declared relation, is the immediately adjacent contrast distributed asymmetrically around it? It adds the differences from edges that continue forward and subtracts the differences from edges that arrived from behind, scaled by local contrast. It acts on immediate neighbors only — no path accumulation, no assumed interior.

*Constraint: couple only by the asymmetry present in the immediately declared neighborhood. Do not assume adjacency that was not declared.*

**Together** they constitute the primary kernel: E_primary = Σ(Δ(x)). Two operators. One composition. No path structure assumed.

Under the directional admissibility condition, every admissible declared structure is asymmetric. This means Σ will always detect non-zero local antisymmetry wherever adjacency is declared and the observable is not uniform. Symmetry — where every declared relation has a matching reverse — is a signal that the declaration contains inadmissible structure, not a legitimate state the operators can act on.

---

## The three Primary Region quantities

The primary kernel produces a field. Three quantities describe the shape of
that field, and together they are how the framework decides whether the
primary kernel is still the right kernel.

**How many independent directions the Δ output spans.** The directed
differences across all declared edges may point in many different directions,
or they may all lie along one. If they span more than one direction, the
declared structure is carrying contrast on several relations at once,
unresolved.

**How many independent directions the Σ output spans.** The same question
asked after the coupling step.

**How much of the declared structure has somewhere to continue to.** Not every
declared edge has a successor. Some are terminal. This quantity counts the
edges that do have somewhere to continue — the part of the structure through
which accumulation could travel if accumulation were active.

The second quantity divided by the third is the ratio the framework watches.
When it is far below one, the declared structure is at the Primary Region:
contrast exists across relations, but not enough of it is spanning the parts of
the structure that could carry accumulation. The primary kernel is the correct
kernel and B is not invoked. As the ratio approaches one, the structure has
begun to carry enough distinguishable relational contrast that accumulating
along declared continuation would add something. That is the condition named
elsewhere in these documents as persistence being confirmed, and it is what
brings B into use.

**Two things about this ratio are open, and both matter for how it is read.**

The threshold itself has not been derived. There is no computed value at which
the framework states that the transition has occurred; the ratio's behaviour is
declared, its cut point is not.

And the ratio has a ceiling that comes from the declaration rather than from
the system. The number of independent directions the Σ output can span is
limited by how many components the declared variable set carries. If a
declaration has few components and many edges with successors, the ratio cannot
approach one no matter what the observable does — the limit is a property of
what was declared, not a finding about what was observed. A ratio far below one
on a single-component declaration says nothing about the system. It says the
declaration cannot express the quantity.

The formal statement of all three quantities, the ratio, and both open
conditions is in `operators_notation_and_constraint.md`.

---

## A — directed difference (ABR kernel)

A measures directed difference across declared relations.

In the spatial domain, a relation is a directed edge (s, t) whose existence is traceable to an observable through M. A takes the difference x[s] − x[t] and nothing else.

In the relational-evolutionary domain (V6), a relation connects two complete kernel output states across one declared relational step. A takes the difference E_current[e] − E_prior[e] — the same directed-difference formula, applied to edge-valued loci rather than node-valued loci. The direction is fixed: E_current − E_prior. E_prior is the state before the declared relational step; E_current is the state after it. The reverse direction is not traceable to an observable through M.

For A_persistence to carry admissible content beyond the first declared step, E_prior must be the kernel output produced from the preceding actual observation in the declared sequence {M(o₁), M(o₂), ...}. E_prior cannot be a zero field (except on the first step), a model-generated value, or a repeated snapshot of a static configuration and still produce admissible relational evolution output. On a cold start, A_persistence = E_current — the entire current kernel state is declared new. This is admissible as a declared first step. It does not constitute evidence of relational evolution.

*Constraint: no relation and no direction may be added that the declaration did not trace to an observable through M. This holds for both spatial and persistence loci. After the first declared step, E_prior must come from a prior actual observation in the declared sequence.*

---

## B — accumulation along declared continuation

B accumulates along declared continuation and nowhere else.

At each edge, B adds the values of edges that continue forward from it.
It adds the immediate continuations only — the edges that directly follow.
It does not follow those edges onward and add what follows them.
B is one step of accumulation, not a sum along the whole downstream chain.
A terminal edge accumulates nothing.
No boundary is closed to supply continuation that was not declared.

B is absent from the primary kernel — it is not an identity operator that happens to do nothing. It is simply not invoked. B activates when persistence is confirmed: when enough consistently non-zero relational contrast has been established across declared steps that accumulation along paths adds distinguishable structure.

*Constraint: accumulation follows declared structure. It does not supply structure.*

---

## R — coupling through observed asymmetry (ABR kernel)

R couples declared relations through observed asymmetry.

At each edge, R adds the difference between what continues forward and what arrives from behind, scaled by local contrast. Where declared relation families couple, the asymmetry of each contributes to the other according to the declared coupling.

Under the directional admissibility condition, every admissible declared structure is asymmetric — each declared relation has a single admissible direction, and the reverse requires independent provenance. Symmetry is therefore not a legitimate declared state within the admissibility conditions of this framework. Do not assume the two directions are equal: without independent provenance for each direction, only one direction is admissible.

*Constraint: couple by the asymmetry present. Every declared relation has one admissible direction. The reverse requires independent provenance.*

---

## ρ — local contrast

ρ scales coupling according to local contrast.

At each node, ρ is derived from the largest gradient present at that node in the operator's output.
ρ does not aggregate beyond the node.

*Constraint: coupling strength is derived locally. It is not assigned globally.*

---

## C — declared projection

C reports a declared projection and states what was discarded.

Any reduction of the field — to one value per node, to a bound, to a variance — is a projection.
A projection is admissible when what it preserves and what it discards are stated.

*Constraint: no silent reduction.*

---

## Before Δ / Before A

Differences may not be altered before measurement unless the alteration is declared within M.

Admissible before Δ or A: uniform shift, declared unit scale.
Everything else requires declaration with preserved and discarded invariants stated.

*Constraint: do not change what you are about to measure without saying so.*

---

## What this produces

**At the Primary Region** — where persistence is not yet confirmed and path accumulation has not been established — the primary kernel applies:

Δ, then Σ: the operators ask whether anything distinguishable exists, and whether what exists is asymmetrically organized in its immediate neighborhood. The result reflects the declared relational structure of the observable at its most minimal. No interior is assumed. No history is carried.

**When persistence is confirmed** — the ABR kernel applies in two phases:

Phase 1 (spatial): A → B → R over declared spatial relations produces the spatial relational field. One declared observation M(o) is sufficient.

Phase 2 (persistence): A → B → R over declared persistence relations — each connecting the prior complete kernel output to the current one across one relational step — produces the relational-evolution field over one declared relational step. This phase requires a declared sequence of actual observations. The relational step is the system's own declared process step, not an externally imposed increment. What the operators detect is how the relational field changes from one actual observation to the next — not the structure of a single frozen configuration, and not the output of a model used as a substitute for observation.

Both phases use the same operator formulas. What changes between phases is what the operators act over, not how they act. The direction of every relation — spatial or persistence — is determined by the observable and fixed by the declaration.

A Phase 1 analysis on a single snapshot is admissible and informative. It reveals the relational structure present in that observation. It does not reveal how that structure evolves. Phase 2 requires the process, not the photograph.

The result is not an interpretation.
It is a function of the declaration and the observable.
Change the declaration and the result changes.
The same declaration on the same observable produces the same result.

---

## What the Primary Region produces — and why it matters at every scale

At the Primary Region, three conditions hold simultaneously that are not confined to this region — they hold at every declared scale and domain. Their consequences are observable everywhere. The phenomena themselves are not directly observable — only their consequences are. This is not a limitation of measurement. It is a structural feature of the framework: the conditions that make observation possible are prior to observation and cannot themselves be directly observed.

**Tunneling** is the condition that no energy transition edge has an infinite barrier. The quantum boundary component of the directed difference Δ is nonzero at every finite barrier edge. What this means in plain language: energy is never completely blocked. There is always a nonzero component of the directed difference at any declared transition edge. What we observe as energy transfer at any scale is the consequence of this — the classical component dominates at larger scales, but the tunneling component is structurally present. A system with no tunneling has no energy transfer. This is a framework-level observation, not yet formally derived from the kernel.

**Superposition** is the condition that rank(Im Δ) > 1 — multiple edges carry nonzero directed contrast simultaneously before B activates. In plain language: before any system resolves into a single outcome, it is in a state of genuine multiple directed contrasts. B activation is what measurement is. What we observe as an outcome is always the post-resolution consequence of B having acted on a prior superposition state. Every self-organizing, evolving system was in a superposition state before it evolved. This is a framework-level observation, not yet formally derived from the kernel.

**Entanglement** is the condition that a declared edge with 𝟙[e]=1 between two loci establishes a relational constraint that the operators preserve regardless of spatial separation. In plain language: two loci that share an active declared edge are not independent. Spatial separation is a projection — the relational edge is prior to it. What we observe as correlation, structural coherence, and collective behavior are consequences of persisting active edges. Decoherence — the loss of correlation — is the admissible statement of what happens when the edge is removed: 𝟙[e]=0 and ε[e]=0. This is a framework-level observation, not yet formally derived from the kernel.

**Relational evolution rate** decreases monotonically from the Primary Region outward. The Primary Region is the fastest-evolving declared region in D. τ[v] is smallest at the Primary Region — relational progression intervals are shortest. At larger scales, τ[v] grows, and what was active relational evolution at the Primary Region appears as stable accumulated structure. The 78-decade span of Φ[v] confirmed across declared regions is the observable expression of this invariant. In plain language: what we observe as stable matter at larger scales is Primary Region relational evolution that has accumulated into confirmed persistence through B activation. The stability is downstream of the speed.

These four conditions are jointly necessary for any self-organizing, evolving, observable system to exist within D. That is a framework-level observation stated here in plain language. It is not a mathematical result already established by the kernel. The formal derivations are open conditions. See `derived_invariants.rs V4` for the precise statements.

## What remains with the reader

The operators do not interpret findings.
The operators do not assign meaning to departures.
The operators do not determine what constitutes a significant result.

Those determinations remain with whoever declared the domain.

*Constraint is not authority.*
*The reader remains Origin.*

---

*Bounded over D. No claim beyond D.*
*Metatron Dynamics, Inc. V8.*

---

**V5 → V6 changes:** New section: "Every relation has one direction" — states the directional admissibility condition and distinctness axiom in plain language; derives ring inadmissibility from first principles. New section: "Δ and Σ — the primary operators" — plain language description of the primary kernel, including the consequence of Position B for Σ. A section: persistence direction stated as fixed (E_current − E_prior); reverse direction named as inadmissible. B section: absence from primary kernel distinguished from identity operator. R section: symmetry recharacterized — within the admissibility conditions, every admissible declared structure is asymmetric; symmetry signals inadmissible structure. Before Δ / Before A: heading updated to cover both operators. What this produces: primary kernel output added alongside ABR kernel output, with condition for each. No universal claims about observables — all claims scoped to "within the declared admissibility conditions of this framework."

**V6 → V7 changes:** "Before anything is measured" section: sequential observation requirement added as a named constraint — Phase 2 requires a declared sequence of actual observations; cold start admissible on first step only; model-generated sequences named as inadmissible substitutes for observation; zero A_persistence on a static observable named as the correct result. New section: "Relational evolution has a direction" — relational evolution direction declared as a property of the process, not of the index; temporal ordering named as a downstream projection of relational evolution, not its source; index order alone named as inadmissible direction declaration; clock readings named as relational observables in their own domain that do not transfer direction without a declared connection. A section: sequential constraint on persistence form added — E_prior must come from a prior actual observation after the first step; cold start as declared first step distinguished from relational evolution. "What this produces" section: Phase 2 requirement for a process (not a snapshot) stated explicitly; single-snapshot Phase 1 analysis named as admissible and its limitation stated.

**V7 → V7 (this session):** Primary observable and mathematical primitive section added — change is the primary observable; relation is its invariant structure; both invariant across all declared scales and domains within the bounded domain of human perceptual and cognitive information; Δ and A named as the minimal mathematical expression of the primary observable; the operators do not produce change, they read it. Version header corrected to V7.

**V7 → V8 changes:** New section added: "What the Primary Region produces — and why it matters at every scale" — plain language statements of I-T (Tunneling), I-S (Superposition), I-E (Entanglement), and I-RE (Relational Evolution); joint necessity observation stated; epistemic status of each phenomenon stated explicitly (not directly observable; consequences observable through M; framework-level necessity statements distinguished from derived results). Version header updated to V8.

**V7 → V8 changes:** Version header corrected — the closing attribution already read V8 while the header read V7. B section: accumulation depth stated in plain language — immediate continuations only, one step, not a sum along the whole downstream chain. Added after a downstream artifact described B as full-chain accumulation.

**V8 → V8.1 changes:** New section — the three Primary Region quantities, stated in plain language without notation: the independent directions spanned by the Δ output, the independent directions spanned by the Σ output, and the count of declared edges with somewhere to continue to. The ratio governing the primary-to-ABR kernel transition described conceptually, connecting to the existing statement that B activates when persistence is confirmed. Both open conditions stated in plain terms — the threshold is not derived, and the ratio carries a ceiling imposed by the component count of the declared variable set, so a low-component declaration cannot reach activation regardless of the observable. Added after verification found the document explained every operator but never introduced the quantities used to decide which kernel applies.
