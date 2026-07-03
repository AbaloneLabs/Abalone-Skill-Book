---
name: decision_analysis_and_option_evaluation.md
description: Use when the agent is analyzing a project decision, evaluating options against weighted criteria, structuring a tradeoff analysis, building a decision matrix, comparing build-versus-buy or vendor options, or preparing the analytical case before a commitment is made.
---

# Decision Analysis and Option Evaluation

Most project decisions are made by intuition and defended by rhetoric. A senior person forms a preference early, the team rationalizes it, and the alternatives that were never seriously considered disappear from memory. The damage shows up later as rework, as reopened decisions, and as commitments that cannot be justified when challenged. Good decision analysis is not about producing a number that proves a choice; it is about forcing the real tradeoffs into the open, making assumptions explicit, and ensuring the decision can survive scrutiny after the facts change.

The judgment problem is not "how do I fill in a scoring sheet." It is how to frame the decision correctly, generate genuinely distinct options, choose criteria that reflect what actually matters, and avoid the false precision that turns a judgment call into a math exercise. Agents tend to over-engineer simple decisions, under-analyze irreversible ones, and confuse a scoring matrix with a substitute for thinking.

## Core Rules

### Frame the Decision as a Question With a Triggering Context

Before evaluating options, write the decision as a specific question with a forcing context: what must be decided, by when, and what happens if it is deferred. "Choose a testing tool" is too loose; "Select the integration testing framework by end of Q3 so the platform team can standardize, given that deferral blocks the migration" is a framed decision. The framing determines which options and criteria are relevant, so get it explicit before option generation begins.

### Generate Genuinely Distinct Options, Not Variations of One

A decision among three flavors of the same approach is not a real choice. Force at least one qualitatively different option: a do-nothing or status-quo baseline, a build option versus a buy option, a phased option versus a big-bang option. Include the "do nothing" and "defer" options explicitly so the cost of inaction is visible. If every option converges on the same answer, the analysis is likely confirming a decision already made informally.

### Derive Criteria From the Decision Frame and Stakeholder Interests

Criteria must reflect what the decision is actually for. Pull them from the project objectives, the constraints (cost, schedule, regulatory), and the stakeholder interests that will judge success. Separate must-have constraints from nice-to-have preferences. Weight criteria only when the tradeoffs genuinely differ in importance, and set weights before scoring to avoid reverse-engineering weights to favor a preferred option.

### Separate Facts, Estimates, and Assumptions Explicitly

Every score in an evaluation rests on something: a verified fact, an estimate, or an assumption. Mark each. A criterion scored on an untested assumption carries far less certainty than one scored on measured data, and the decision should reflect that. Where assumptions dominate, the right next step may be to run an experiment or spike to convert assumptions into evidence, not to finalize the decision.

### Use Scoring to Compare, Not to Conclude

A weighted decision matrix is a comparison aid, not an oracle. Use it to reveal where options are close, where one option dominates, and where the decision is genuinely sensitive to a single criterion. When the top two options score within a few points, the matrix is telling you the decision is close and the tiebreaker should be a judgment about risk, reversibility, or strategic fit, not a decimal point.

### Evaluate Reversibility and the Cost of Being Wrong

Not all decisions deserve the same depth of analysis. A reversible, low-cost decision should be made quickly and revisited; an irreversible or high-cost commitment deserves disproportionate analysis. Explicitly classify each decision by reversibility and blast radius, and scale the rigor accordingly. Spending two weeks analyzing a reversible choice is as much a failure as spending two hours on an irreversible one.

### Surface the Dominant Risk and the Disqualifying Condition

For each option, name the single risk most likely to make it fail, and any condition that should disqualify it outright regardless of score. A vendor that cannot meet a regulatory deadline is disqualified even if it scores highest on cost. Making these explicit prevents a high-scoring option from being selected when it carries a fatal flaw the matrix smoothed over.

### Recommend, Then Pressure-Test the Recommendation

Analysis should end with a clear recommendation and the conditions under which it should change. State the recommendation, the key assumptions it depends on, and what evidence would cause you to reverse it. Then run a quick pre-mortem: assume the chosen option failed in six months and ask why. The pre-mortem often surfaces a risk the forward-looking analysis missed.

## Common Traps

### Reverse-Engineered Criteria to Justify a Preferred Option

The team already knows the answer, so criteria and weights are quietly shaped to make the preferred option win. This trap is seductive because it produces a matrix that looks rigorous while encoding the bias. The defense is to lock criteria and weights before scoring and to have someone not invested in the outcome challenge the frame.

### False Precision in Scoring

Assigning scores to seven decimal places on criteria that are really guesses gives an illusion of rigor that does not exist. The trap is that the math feels objective, so dissent gets suppressed. Scores on qualitative criteria are rough judgments; treat close scores as ties and decide on judgment, not arithmetic.

### The Do-Nothing Option Treated as Zero

Omitting or under-weighting the status-quo option hides the real cost of inaction, which is rarely zero. Continuing the current approach has ongoing cost, risk, and opportunity cost. The trap makes every change look justified by comparison. Always score "do nothing" honestly, including its deterioration over time.

### Comparing Options Against Each Other Instead of Against Criteria

Options get scored relative to each other rather than against an absolute standard, which produces scores that drift depending on which options are in the set. Add a strong option and the others look worse; remove it and they look better. Score each option against the criterion definition independently.

### Analysis Paralysis on Reversible Decisions

Spending weeks evaluating a decision that can be changed in a day wastes the very resource the analysis was meant to protect. The trap is treating all decisions as equally weighty. Match analysis depth to reversibility and blast radius, and default to deciding fast and learning on reversible choices.

### Single-Criterion Dominance Disguised as Multi-Criteria Analysis

A matrix lists six criteria, but one of them (usually cost or a sponsor's preference) is treated as decisive and the rest as decoration. The trap is that the matrix provides cover for a single-factor decision. If one criterion is truly decisive, say so explicitly and drop the pretense of tradeoff analysis.

### Ignoring Option Interactions and Dependencies

Options are scored as if independent, but in reality choosing one forecloses or enables others. Selecting a cloud provider locks in a set of downstream decisions. The trap is evaluating each decision in isolation and accumulating commitments that conflict. Map how this decision constrains future ones.

### Confusing Consensus With Correctness

A decision everyone agrees on is not necessarily the right one; it may simply be the one that created the least conflict. The trap is treating smooth agreement as evidence of quality. Actively seek the dissenting view and the option no one advocated, because those often hold the real risk.

## Self-Check

- [ ] Is the decision written as a specific question with a triggering context, deadline, and cost of deferral?
- [ ] Are there genuinely distinct options, including an explicit do-nothing or defer option?
- [ ] Were criteria and weights defined before any scoring began, and do they derive from the decision frame and stakeholder interests?
- [ ] Is each score labeled as fact, estimate, or assumption, with assumptions flagged for validation?
- [ ] Has the decision been classified by reversibility and blast radius, with analysis depth scaled accordingly?
- [ ] Does each option have a named dominant risk and any disqualifying condition stated explicitly?
- [ ] If the top two options scored closely, was the tiebreaker a judgment about risk and strategic fit rather than arithmetic?
- [ ] Does the analysis end with a recommendation plus the conditions under which it should be reversed?
- [ ] Has a pre-mortem been run to surface risks the forward analysis missed?
- [ ] Has the dissenting or unadvocated option been genuinely considered rather than dismissed?
