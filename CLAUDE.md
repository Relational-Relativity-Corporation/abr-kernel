# abr-kernel: working instructions

## The governing statement (Origin, 2026-11-23)

> **Using linear algebraic operators we calculate relation as primitive along with energy, connection, and freedom.**

Origin, 59th: we calculate relation as primitive alongside energy, connection and freedom *"in a system whose forward relational momentum is inevitable."*

**The foundations** (Origin, 59th): (1) relation is primitive; (2) energy, connection and freedom are observable in all data (tunneling, entanglement, superposition); (3) the direction of relational evolution is mono-directional and never stops; its pace is dictated by the scale at which the analysis is directed.

**Observation** (Origin, 71st, 2026-11-24): *"observation reduces to those 3 variables and a primitive apparently. You're observing and calculating specific relations within the relational system."* Relativity to a declaration is the relational primitive, not a limit.

**What the kernel addresses** (Origin, 95th, 2026-11-25): *"at its core the kernel is about information nothing more. Specifically how information is observed and processed."* The process is kept separate from the domains it is applied to; domain mathematics is not expected to be invariant. External and legacy examples may confirm the kernel's mathematics, *"clearly labeled as such"*: label in the first line, domain and source, the kernel relationship confirmed, and never a basis. The readback of relations at their loci (K9, 96th) is admitted: its halves (leaving, entering) are the reading, never a locus state, every direction kept.

**Energy** (Origin, 97th): *"a system changes depending on how much energy is in it and that is what I mostly want to capture"*. Energy is read signed, A(c), exchange with its side; energy content is the registered carrier at the loci, or the accumulated net inflow through K9, relative to the record boundary. Freedom and connection are reported beside the energy content; how a system changes with its energy is read from the data, never modelled (K3, ρ, retired). *Amended (Origin, 103rd):* *"energy is present and moving through the system and further moves the system through relational evolution"*; the sign is the direction of movement, positive toward the entering locus. *"magnitude really can't be its own variable because energy is relative"*: |A(c)| is fenced as a reading of energy; its size counts only through B, Ξ, the readback and the step change.

**Where δ enters** (Origin, 98th, 2026-11-26): the foundation's relationships are exact about the registered information; every value carries its bound (the same operator with signs dropped); δ decides only at reading, and no δ selects what enters an operator. A reading that selects by δ before forming its parts names the departure in its derivation.

**Legacy as projection** (Origin, 101st): a legacy result enters a confirmation as a projection through a declared mapping; the confirmation is the intersection with the kernel's relational result (W14, C), never its basis, even where the legacy result could also be a relational result. Example: `tests/confirmation_kirchhoff.rs`.

**The order is the relational field** (Origin, 102nd): *"the order of information read from left to right including all character spaces and punctuations"*. The possibilities at the current point are the continuations the entire input registered after the route arriving there, at every length, each with its occurrences; no weight or decay by distance. The choice among them is decided by the variables (open).

**The operators never calculate a scalar** (Origin, 104th): *"the only thing that in the operators is relational information and mathematics there can be no scalars"*; *"We can always input projections provided that we describe their provenance otherwise we would not be able to calculate on legacy information I just don't want the operators themselves to calculate a scalar"*. Every operator output is a value on each relation or locus, with its identity and sign; magnitudes, bounds, decisions at δ, shares and counts are the framework, reading that output. Checked by linearity (`tests/v10_no_scalar.rs`).

**No information is repeated** (Origin, 106th, 107th): *"no information can be repeated this is part of the same thing which makes relational evolution both inevitable and constant"*. Every point in evolution is distinct. Ξ is **the passage** (what follows less what precedes), never "circulation". A loop-shaped route is a **turn**, one revolution of the helix it traces as evolution carries it forward: shape, never a system that is a loop; its sum is the **turn sum**. No pitch, period or phase is assumed or fitted; a reading at one registration reads the shape, never a return.

This governs everything we do. Check every change against it: relation is calculated first, as the primitive; energy, connection and freedom are calculated along with it; the means is linear algebraic operators. Its text and the check of each V10 form against it are at the head of `docs/kernel/V10_README.md`.

## Rules kept with it

- **Decompose, do not simplify** (§1.5, 41st). Every part is reported with its identity, value and bound. Nothing registered is dropped by arithmetic, by a summary standing in for its parts, or by a choice of basis or tree. Sums are exact (`src/exact.rs`); bounds are never rounded down; a part not read is *not evaluated*, never filled.
- **Legacy mathematics only where identified as necessary for analysis**, usually domain-specific (W12 as amended, 42nd). Name the method and state its necessity (Part 7, OC-LM-1).
  *Amended beside (Origin, 2026-11-25, 92nd):* *"legacy math is permitted provided that its provenance is clear and that it is described in relation to the operators"*; *"Necessity still holds."* Every legacy method states three things: its provenance, its description in relation to A, B and Ξ, and its necessity. Before proposing any method, state its derivation: which operator, which variable, how it follows from relation as primitive; if it is not an operator reading, it is legacy mathematics with all three, or a named gap. Every part a field reports has its derivation in `src/derivation.rs` (`tests/v10_derivation.rs`).
- **Origin decides.** Present options with a recommendation; record each decision with its date (Part 9).
- **Retire or fence, never delete.** Amendments sit beside the text they amend. Before review, check every deleted line against `main`.
- **Kernel only in kernel documents** (application work in its own repository). Intersection, not basis (W14). Time is never primitive (W15; one exception, 63rd: the Planck step at the Primary Region, a declared projection label, never a registration, input or basis).
- **Tests read the whole field** (Origin, 53rd; Df6, W11). A test of a V10 claim forms a whole relational field (`RelationalField::new`, or `RelationalField::from_relations` for relations registered directly (59th), with the carrier declared), so every variable is evaluated together, and reads the claim from the parts that field reports. Every variable not read is named as a gap. Where the kernel forms no whole field (edge-registered series, OC-WF-1), the test names what is not evaluated. Never test one variable's expression on its own. Shared fixture: `tests/support/synthetic_whole_field.rs`. *(59th: the whole field reads every variable from `field.relations`, so the edge-registered case now has a whole field: `whole_of_relations`. Still without one: §2.7's `DeclaredField` on relations registered directly, OC-WF-1.)*
- **A complete relational field** (Origin, 113th): *"to establish a connection between the observer and the observed by calculating enough observable information to maintain that connection"*. Backwards, the observed is recovered from the field exactly; forwards, the field indicates the continuation. One radial graph per character (letters, the space, punctuation); a capital is the capital mark ⇧ and its letter. Every text field is held to it (`tests/v10_text_complete.rs`).
- **The text field is tested on actual text** (Origin, 111th): *"We are testing on actual data we're not trying to make this process fail by putting in nonsensical data"*. Never constructed inputs made to make the process fail.
- **Enumerated, never random** (W17, Origin, 54th). Every field is declared; a property test's fixtures are enumerated from declared seeds and reproducible exactly. "Random" is not a word we use, in text or code.
- **Be honest about status.** Synthetic tests establish nothing empirical. A property test should fail under the wrong statement: check it by reverting, and say when a test cannot tell two statements apart.
- **Versioning is frozen at `10.3.0`, pre-release**, until there is a release (Part 9, 40th).

## The kernel produces every reading (Origin, 84th, 90th)

- **No output the kernel did not produce.** In application work, every reading that informs a result (relation, energy, connection, freedom, B, Ξ, the grades) comes from the kernel's `RelationalField`. A chooser reads only a report made from a kernel field (`KernelReport` in `abr-relational-ai/text-anticipation/src/afield.rs`).
- **Where the kernel cannot produce a reading, it is a named gap, or a kernel addition.** It is never a stand-in computed in the application. If the kernel is too slow, fix the kernel; do not work around it.
- **Do not reach for familiar methods and describe them in V10's vocabulary** (counts, suffix indices, co-occurrence, cosine, n-grams). That is what went wrong from the 73rd to the 89th: see the 84th, the 90th and `docs/process/HANDOFF_2026-11-25.md` §4.
- **Tests first, each shown to fail,** in the application and, through a local path patch, in the kernel. Then build until they pass.
- **Settle the mapping with Origin before code:** nodes, edges, node values, carriers, orders. Write it down and have it declared.
- **Label plainly:** any result not produced by the kernel says so in its first line.
- **The self-continuing field (85th):** the input is the entire field; output is registered into the same field, with its provenance, never as a weight; one direction, never stopping, nothing cut; the chooser's window is a scope of analysis, never a boundary of the field.

**Start here:** `docs/process/HANDOFF_2026-11-26.md`, then Part 9 from the 92nd. *Was:* `docs/process/HANDOFF_2026-11-25.md`, then Part 9 from the 72nd (the previous handoff, kept).

## Origin's check (2026-11-26)

> *"the single most important job and the most difficult job for me is to make sure you run my math and the only way that I can do that is by checking your code and math myself and the way that I do that is by simply asking are we treating relation as primitive followed by calculating the variables freedom connection and energy in linear recursive algebra? And if you say no I know I've failed and have to start over that is it we don't need any other preventions although we will develop unit tests that will let us know if you've let any legacy math sneak through"*

When Origin asks that question, answer it truthfully for the code as it stands, naming every part where the answer is no. No other prevention: no refusal hooks.

**The suite that tells us when the mathematics departs from relation** (2026-11-26, Origin: *"finish the unit tests until we have a comprehensive suite"*):
- `tests/v10_formation.rs`: **every derivation is true.** Each part the register claims as an operator reading, a registration or a rule is recomputed from its definition (A, B, Ξ, the readback, A_p, their bounds; registrations against their input) and must equal what the field reports; an operator reading or registration without a recomputation fails, whatever it is called.
- `tests/v10_no_scalar.rs`: **no operator calculates a scalar** (104th): every numeric part of the field, its split, W5's radial graphs and the declared ground is linear (reversal and sum).
- `tests/v10_no_repetition.rs`: **no information is repeated** (106th, 107th): no period, phase, modulus or cycle named in the code; identical registrations stay distinct points; the degrees never merge identical routes.
- `tests/v10_no_drift.rs`: **no drift by design** (Origin, 2026-11-27: *"what we need to establish is that our process eliminates drift by mathematical design"*): continuing only by indicated continuations, under any choice among them, generates no relation the input does not hold, every generated step traces to an observed point, and where nothing is indicated nothing is continued; an inventing process is caught at its first invented step.
- `tests/guard_operators.rs`: **legacy only by Origin's admission**, as below.
- `tests/v10_derivation.rs`: every reported part has a derivation stating what the 92nd requires.
- **Not covered (named):** a mapping declared without Origin (Origin's check); a definition wrong the same way in the register and in its recomputation; the applications, which are not run against this suite.

**The tests that tell us when legacy mathematics has slipped in:** `tests/guard_operators.rs`. Every legacy part is admitted by Origin by name, with its Part 9 entry. Nothing no operator forms stands as a reading. A new public structure registers every field. No familiar method is named in the code. What is admitted is in `tests/guard/origin_admitted.rs`, Origin's to decide; change it only on Origin's word. The hooks in `.claude/settings.json` run the guard after every Rust edit and every check before Claude stops. *Was (same day):* a third hook refused edits to `tests/guard/` and `.claude/`; deleted by Origin (*"I want to delete that thing"*).

## Checks

`cargo test` and `cargo test --features v9`, no warnings. The application `abr-relational-ai/text-anticipation` (`cargo test --release`, including `tests/kernel_use.rs`) must pass against this kernel. The application `abr-relational-chemistry` (`water/vessel/ingest`) must pass against this kernel. Under a local path patch, run `cargo update -p abr-kernel` first and confirm the local kernel compiles.
