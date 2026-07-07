---
name: pcg-determinism-seeds-and-reproducibility.md
description: Use when the agent is designing the determinism, seeding, and reproducibility of a procedural generation system, deciding how seeds are created and shared, managing the relationship between randomness and repeatability, or reviewing whether generated content can be reproduced, debugged, and shared; trigger contexts include procedural seed, seed design, deterministic generation, reproducibility, shared seed, daily seed, seed exchange, debug replay, deterministic RNG, generation determinism, procedural reproducibility, world seed; important risks include non-deterministic bugs, unreproducible failures, seeds that change meaning across versions, and shared content that cannot be reproduced.
---

# PCG Determinism Seeds And Reproducibility

A procedural system's value depends in part on whether its output can be reproduced, and that reproducibility is not automatic — it must be designed into the system from the start, because determinism is fragile and breaks silently under threading, floating-point variation, library updates, and unordered iteration. The agent is usually asked to design or review a generator's seeding and determinism while the system's randomness sources, its version stability, and its debugging needs are under-specified. The judgment problem is that a generator that cannot reproduce its output cannot be debugged, cannot share content, cannot support community seed culture, and cannot guarantee that two players who "have the same seed" actually have the same experience.

The agent tends to miss this because determinism is treated as a default property of code rather than a property that must be actively maintained, and because the failure modes (a bug that only appears on one seed, a shared seed that produces different worlds after a patch) only surface late. The harm is generators whose bugs cannot be reproduced or fixed, shared seeds that lie, daily challenges that are not actually shared, and a community whose seed culture is undermined by instability. Use this skill to slow the decision down enough to expose what must be reproducible and why, then make the recommendation appropriately conservative when determinism, version stability, and debuggability are at stake.

## Core Rules

### Make Determinism A Deliberate Design Property
Treat determinism as a designed property of the generator, not a default. The agent should specify that the generator produces identical output for an identical seed under identical conditions, use a deterministic random number generator (not system entropy) for all generation decisions, and ensure that every source of variation in generation is captured by the seed. A generator whose determinism is assumed rather than enforced will produce unreproducible output and unfixable bugs.

### Design The Seed As A Meaningful Identifier
The seed is the player-facing handle for a generated world, and its design matters. The agent should decide the seed's format (a number, a string, a code), its length and entropy, whether it is human-memorable or shareable, and whether it encodes metadata (version, mode, options). A well-designed seed enables sharing, daily challenges, and community culture; a poorly designed one is a number no one can remember or use.

### Isolate And Control All Randomness Sources
Every source of randomness in generation must be routed through the seeded RNG, with no leakage from system time, thread scheduling, hash-table iteration order, or unseeded libraries. The agent should audit the generation pipeline for unseeded randomness, ensure ordered iteration over collections, and guard against threading that introduces non-deterministic ordering. Randomness leakage is the most common cause of silent determinism breakage.

### Plan For Version Stability And Seed Migration
Seeds must produce stable meaning across versions, or version changes must be explicit and migrated. The agent should version the generator (include a version in the seed or in the generation parameters), decide whether old seeds remain valid after generation changes, and provide migration or explicit invalidation when the algorithm changes. A seed that silently produces a different world after a patch breaks sharing and trust.

### Support Debug Replay And Failure Reproduction
The generator must support reproducing any generated output for debugging. The agent should ensure every generated world is reproducible from its seed and version, that crash reports and bug tickets capture the seed, and that developers can regenerate any reported world locally. A bug that cannot be reproduced from its seed is a bug that likely cannot be fixed.

### Decide What Is Seeded Versus What Is Runtime
Not everything in a generated world must be seeded at generation time; some state is runtime (physics outcomes, AI behavior, player modifications). The agent should distinguish what the seed determines (the generated structure) from what is determined at runtime (the live simulation), and ensure the boundary is clear so that "same seed" means "same starting world," not "same entire playthrough." Conflating seeded and runtime state produces false expectations of reproducibility.

### Enable Sharing, Daily Seeds, And Community Features Where Appropriate
Where the game supports it, determinism enables sharing (players exchange seeds), daily challenges (all players get the same seed on the same day), and community culture (speedruns on a fixed seed, seed-of-the-week). The agent should design the seed system to support these features if they serve the game, with stable meaning, shareable formats, and cross-player consistency. These features are only possible on a deterministic foundation.

## Common Traps

### Assumed Determinism That Breaks Silently
The team assumes the generator is deterministic because it uses an RNG, but unseeded randomness, threading, or unordered iteration leaks in, and output varies run to run. The trap is that determinism seems to come for free. The false signal is that sample runs matched. The harm is unreproducible bugs, shared seeds that lie, and a generator whose output no one can reliably reproduce or debug.

### Randomness Leakage From System Sources
System time, thread scheduling, hash-table iteration order, or unseeded libraries introduce variation the seed does not capture. The trap is that these sources feel negligible. The false signal is that the RNG is seeded. The harm is determinism breaks in ways that are hard to trace, because the leakage is invisible in the code path.

### Seeds That Change Meaning Across Versions
A patch changes the generation algorithm, and seeds that produced one world now produce another, silently. The trap is that the algorithm improved. The false signal is that the seed format is unchanged. The harm is shared seeds become inaccurate, daily challenges are not consistent, and community trust in the seed system erodes.

### Unreproducible Bugs
A player reports a bug on a specific seed, but the developers cannot reproduce it because the seed, version, or runtime state was not captured. The trap is that the bug seems environment-specific. The false signal is that the generator works locally. The harm is the bug cannot be fixed, because the conditions that produced it cannot be reconstructed.

### Conflating Seeded And Runtime State
The team promises that "the same seed gives the same game," but runtime simulation (physics, AI, player choices) diverges, and players expect full reproducibility. The trap is that the seed feels comprehensive. The false signal is that the starting world matches. The harm is false expectations, perceived bugs when playthroughs diverge, and a mismatch between what the seed guarantees and what players assume.

### Shareable Features Built On Non-Determinism
Daily challenges, shared seeds, or speedrun modes are built on a generator whose determinism was not enforced, and the features are inconsistent across players. The trap is that the features shipped. The false signal is that the seed is shared. The harm is daily challenges that differ, shared seeds that mislead, and community features that undermine trust.

### Presenting RNG Choice As Neutral Engineering
Seed and determinism decisions are often judgment calls, but the agent should not present RNG selection as if it were neutral. State what is known (the reproducibility requirements, the sharing features, the debug needs), what is inferred (the stability cost), and what is a design choice, so the team decides with the tradeoffs visible.

## Self-Check

- [ ] Is determinism treated as a deliberately designed property, with a seeded deterministic RNG used for all generation decisions?
- [ ] Is the seed designed as a meaningful, shareable identifier with appropriate format, length, entropy, and metadata?
- [ ] Are all randomness sources audited and routed through the seeded RNG, with no leakage from system time, threading, or unordered iteration?
- [ ] Is the generator versioned, with explicit decisions about seed validity and migration when the algorithm changes?
- [ ] Can any generated world be reproduced from its seed and version for debugging, with seeds captured in crash reports and bug tickets?
- [ ] Is the boundary between seeded generation state and runtime simulation state clear, so "same seed" means "same starting world"?
- [ ] Does the seed system support sharing, daily challenges, and community features where they serve the game, with stable cross-player meaning?
- [ ] Does the output distinguish determinism decisions that serve debuggability and trust from those that serve convenience?
- [ ] Are the seed format, version policy, and reproducibility guarantees specific enough for engineering to implement without re-deciding?
- [ ] Is uncertainty about long-term version stability and sharing consistency named, with the tradeoffs that would change the recommendation made explicit?
