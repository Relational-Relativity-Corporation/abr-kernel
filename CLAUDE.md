# abr-kernel: working instructions

## The governing statement (Origin, 2026-11-23)

> **Using linear algebraic operators we calculate relation as primitive along with energy, connection, and freedom.**

Origin, 59th: we calculate relation as primitive alongside energy, connection and freedom *"in a system whose forward relational momentum is inevitable."*

**The foundations** (Origin, 59th): (1) relation is primitive; (2) energy, connection and freedom are observable in all data (tunneling, entanglement, superposition); (3) the direction of relational evolution is mono-directional and never stops; its pace is dictated by the scale at which the analysis is directed.

**Observation** (Origin, 71st, 2026-11-24): *"observation reduces to those 3 variables and a primitive apparently. You're observing and calculating specific relations within the relational system."* Relativity to a declaration is the relational primitive, not a limit.

This governs everything we do. Check every change against it: relation is calculated first, as the primitive; energy, connection and freedom are calculated along with it; the means is linear algebraic operators. Its text and the check of each V10 form against it are at the head of `docs/kernel/V10_README.md`.

## Rules kept with it

- **Decompose, do not simplify** (§1.5, 41st). Every part is reported with its identity, value and bound. Nothing registered is dropped by arithmetic, by a summary standing in for its parts, or by a choice of basis or tree. Sums are exact (`src/exact.rs`); bounds are never rounded down; a part not read is *not evaluated*, never filled.
- **Legacy mathematics only where identified as necessary for analysis**, usually domain-specific (W12 as amended, 42nd). Name the method and state its necessity (Part 7, OC-LM-1).
- **Origin decides.** Present options with a recommendation; record each decision with its date (Part 9).
- **Retire or fence, never delete.** Amendments sit beside the text they amend. Before review, check every deleted line against `main`.
- **Kernel only in kernel documents** (application work in its own repository). Intersection, not basis (W14). Time is never primitive (W15; one exception, 63rd: the Planck step at the Primary Region, a declared projection label, never a registration, input or basis).
- **Tests read the whole field** (Origin, 53rd; Df6, W11). A test of a V10 claim forms a whole relational field (`RelationalField::new`, or `RelationalField::from_relations` for relations registered directly (59th), with the carrier declared), so every variable is evaluated together, and reads the claim from the parts that field reports. Every variable not read is named as a gap. Where the kernel forms no whole field (edge-registered series, OC-WF-1), the test names what is not evaluated. Never test one variable's expression on its own. Shared fixture: `tests/support/synthetic_whole_field.rs`. *(59th: the whole field reads every variable from `field.relations`, so the edge-registered case now has a whole field: `whole_of_relations`. Still without one: §2.7's `DeclaredField` on relations registered directly, OC-WF-1.)*
- **Enumerated, never random** (W17, Origin, 54th). Every field is declared; a property test's fixtures are enumerated from declared seeds and reproducible exactly. "Random" is not a word we use, in text or code.
- **Be honest about status.** Synthetic tests establish nothing empirical. A property test should fail under the wrong statement: check it by reverting, and say when a test cannot tell two statements apart.
- **Versioning is frozen at `10.3.0`, pre-release**, until there is a release (Part 9, 40th).

## Checks

`cargo test` and `cargo test --features v9`, no warnings. The application `abr-relational-chemistry` (`water/vessel/ingest`) must pass against this kernel. Under a local path patch, run `cargo update -p abr-kernel` first and confirm the local kernel compiles.
