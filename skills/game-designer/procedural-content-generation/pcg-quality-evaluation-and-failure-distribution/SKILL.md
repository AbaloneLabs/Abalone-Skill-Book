---
name: pcg-quality-evaluation-and-failure-distribution.md
description: Use when the agent is evaluating the quality of procedural generation output, designing metrics and curation for generated content, managing the distribution of good and bad outputs, or reviewing whether a generator's worst cases are acceptable; trigger contexts include procedural generation quality, PCG evaluation, generation metrics, content curation, failure distribution, generator worst case, rejection sampling, quality gates, generated content testing, procedural metrics, output distribution, generator curation; important risks include shipping the failure tail, metrics that miss player experience, generators whose average is fine but whose worst case is unacceptable, and curation that hides rather than fixes problems.
---

# PCG Quality Evaluation And Failure Distribution

A procedural generator is not judged by its average output; it is judged by its failure distribution, because players do not experience the average — they experience the specific worlds, levels, and items the generator happened to produce for them, and a generator whose average is acceptable but whose tail is broken will ruin the experience for the players who land in the tail. The agent is usually asked to evaluate a generator's quality or design its curation while the evaluation metrics, the rejection criteria, and the acceptable failure rate are under-specified. The judgment problem is that procedural content varies stochastically, and quality must be assessed across a distribution, not a single sample, which requires metrics, sampling, and curation that hand-authored content never needed.

The agent tends to miss this because the generator is evaluated by eyeballing a few outputs rather than sampling the distribution, and because quality metrics are borrowed from authored content without adapting to variance. The harm is generators that ship with broken tails — unsolvable levels, degenerate encounters, incoherent structures — that reach players and define their experience, while the team believes the generator is fine because its samples looked good. Use this skill to slow the evaluation down enough to expose the full distribution, then make the recommendation appropriately conservative when the failure tail, player experience, and curation are at stake.

## Core Rules

### Evaluate The Distribution, Not The Sample
A generator's quality is a property of its output distribution, and evaluating a few samples gives a false picture. The agent should sample the generator at scale (thousands or millions of outputs), measure the distribution of key properties (solvability, difficulty, length, density, feature presence), and look specifically at the tails, not just the average. A generator judged by samples ships its failures; a generator judged by its distribution ships only what the distribution actually contains.

### Define Player-Facing Quality Metrics, Not Algorithm Metrics
Quality metrics should measure what the player experiences, not what the algorithm produces. The agent should define metrics such as solvability (can the level be completed), fairness (is the encounter winnable), coherence (does the structure make sense), pacing (does difficulty escalate appropriately), and interest (does the output offer meaningful variety), rather than metrics such as density or probability-table coverage that do not map to experience. Algorithm metrics that do not connect to player experience optimize the wrong thing.

### Establish Explicit Rejection And Re-Roll Criteria
Not every generated output is acceptable, and the system must decide what to reject. The agent should define rejection criteria (unsolvable, degenerate, incoherent, off-tone), decide whether rejected outputs are re-rolled, repaired, or replaced with authored fallbacks, and measure the rejection rate to understand how often the generator fails. A generator without rejection criteria ships its full distribution, including its worst cases.

### Measure And Bound The Failure Tail
The failure tail — the worst outputs the generator can produce — defines the worst player experience, and it must be measured and bounded. The agent should identify the generator's worst-case outputs, measure how often they occur, and decide whether the rate is acceptable or whether constraints must be tightened to bound the tail. A generator whose tail is unexamined will eventually hand a player an unacceptable experience, and that experience will define their impression of the game.

### Test For Solvability And Win-Ability Automatically
For content where solvability matters (levels, puzzles, encounters), the generator should verify solvability automatically, through a solver, a reachability analysis, or a simulated playthrough, rather than relying on the generator's general correctness. The agent should ensure unsolvable outputs are rejected or repaired before they reach players. Shipping an unsolvable level because the generator "usually" produces solvable ones is a defect that punishes the player who lands on the exception.

### Separate Average Quality From Worst-Case Quality
A generator can have good average quality and unacceptable worst-case quality, and both must be assessed. The agent should report both the average (what most players experience) and the worst case (what the unlucky player experiences), and should treat a high-variance generator with a bad tail as a problem even if its average is fine. Reporting only the average hides the tail that will reach real players.

### Iterate The Generator Against Player Data
Generator quality must be iterated against what players actually experience, not just against internal metrics. The agent should instrument generated content (track completion rates, abandonment, complaints per seed), correlate player behavior with generator properties, and tune the generator based on where players struggle or churn. A generator tuned only against internal metrics drifts from the player experience it was meant to serve.

## Common Traps

### Sample-Based Evaluation Hiding The Distribution
The team evaluates the generator by eyeballing a handful of outputs, concludes it is good, and ships, never measuring the distribution. The trap is that the samples looked fine. The false signal is that the generator "works." The harm is the failure tail reaches players, and the team is surprised by complaints about outputs they never sampled.

### Algorithm Metrics Mistaken For Player Experience
Quality is measured in algorithm terms (density, coverage, probability) that do not map to what the player experiences, and the generator is optimized against the wrong target. The trap is that the metrics are quantifiable. The false signal is that quality is being measured. The harm is the generator produces outputs that score well on metrics but play poorly, because the metrics never connected to experience.

### No Rejection Criteria, So The Full Distribution Ships
The generator has no rejection or re-roll criteria, and every output it produces reaches players, including the broken ones. The trap is that the generator is correct on average. The false signal is that most outputs are fine. The harm is the worst outputs define some players' entire experience, and the generator's average quality cannot protect the player who lands in the tail.

### The Unexamined Failure Tail
The generator's worst-case outputs are never measured, and the team does not know how bad the tail is or how often it occurs. The trap is that the average is acceptable. The false signal is that samples look good. The harm is a player eventually receives an unacceptable output, and the team has no framework to understand or fix it, because the tail was never characterized.

### Unsolvable Content Shipping Without Verification
Levels or puzzles ship without automated solvability checks, relying on the generator's general correctness, and unsolvable outputs reach players. The trap is that the generator usually produces solvable content. The false signal is that most levels are completable. The harm is the player who lands on an unsolvable output is stuck, blames themselves or the game, and churns.

### Curation Hiding Rather Than Fixing Generator Flaws
The team adds curation to reject bad outputs but never fixes the generator flaws that produce them, and the rejection rate climbs or the generator's weaknesses remain. The trap is that curation catches the failures. The false signal is that shipped content is clean. The harm is the generator never improves, curation becomes a permanent crutch, and the underlying distribution remains poor.

### Presenting Metric Choice As Neutral Measurement
Quality metric decisions are often judgment calls, but the agent should not present a metric set as if it were objective. State what is known (the distribution, the solvability rate, the tail), what is inferred (player experience), and what is a design judgment about acceptable failure rates, so the team decides with the tradeoffs visible.

## Self-Check

- [ ] Is the generator evaluated at scale across its full output distribution, with specific attention to the tails, not just samples?
- [ ] Are quality metrics defined in player-facing terms (solvability, fairness, coherence, pacing, interest) rather than algorithm terms?
- [ ] Are explicit rejection and re-roll criteria defined, with the rejection rate measured to understand generator failure frequency?
- [ ] Is the failure tail measured and bounded, with constraints tightened where the worst case is unacceptable?
- [ ] Is solvability or win-ability verified automatically for content where it matters, with unsolvable outputs rejected or repaired?
- [ ] Are both average quality and worst-case quality reported, with high-variance generators treated as problems even when the average is fine?
- [ ] Is the generator iterated against player data (completion, abandonment, complaints per seed), not just internal metrics?
- [ ] Does the output distinguish evaluation that protects player experience from evaluation that optimizes algorithm metrics?
- [ ] Are the metrics, rejection criteria, and acceptable failure rates specific enough for engineering and QA to implement without re-deciding?
- [ ] Is uncertainty about the failure distribution and player experience named, with the tradeoffs that would change the recommendation made explicit?
