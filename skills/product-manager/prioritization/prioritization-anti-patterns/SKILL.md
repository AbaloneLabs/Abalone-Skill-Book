---
name: prioritization_anti_patterns.md
description: Use when the agent is reviewing a prioritization process for failure modes, diagnosing why a backlog is unhealthy, recognizing common prioritization mistakes such as loudest-voice or recency bias, auditing a scoring framework for manipulation, or rebuilding a prioritization system that has stopped producing good decisions.
---

# Prioritization Anti-Patterns

Prioritization frameworks are easy to learn and hard to use honestly. A team can adopt RICE, WSJF, ICE, or weighted scoring, run it faithfully, and still produce a roadmap that serves the loudest stakeholder, chases the most recent request, and starves the work that actually matters. The frameworks themselves are rarely the problem. The problem is the human and organizational pressures that bend prioritization toward predictable failure modes, and the fact that these failures are usually invisible to the people inside them, because every individual decision still looks reasonable.

The harm this skill prevents is the slow corruption of a prioritization system until it produces decisions no better than gut feel, while retaining the appearance of rigor. A scoring model that has been gamed, a backlog that sorts by who shouted loudest, a roadmap that reflects the last meeting rather than the strategy, or a planning process that rewards whoever brings the most inflated estimates, all look like functioning prioritization from the inside. The skill is recognizing the anti-patterns early, before they become embedded as "how we do things," and rebuilding the process before the output loses all signal.

Use this skill when reviewing an existing prioritization process, when a backlog or roadmap feels incoherent despite a formal framework, when stakeholders distrust the prioritization outcome, or when diagnosing why good decisions are not emerging from a process that should produce them. The goal is to prevent the agent from mistaking a corrupted process for a working one, and from prescribing a new framework when the real fix is removing the anti-pattern that bent the old one.

## Core Rules

### Recognize That Frameworks Do Not Prevent Anti-Patterns

A prioritization framework is a structure for making tradeoffs visible. It does not prevent the pressures that corrupt tradeoffs. A team can run RICE faithfully and still produce a roadmap dominated by the loudest voice, because the estimates that feed RICE were shaped by that voice. The framework makes the corruption legible only if someone reads the inputs critically.

Treat the framework as a lens, not a shield. The presence of a scoring model is not evidence that prioritization is healthy. Audit the inputs, the process, and the outcomes, not only the formula. The most dangerous anti-pattern is believing that because a framework was used, the decision must be sound.

### Identify The Loudest-Voice And HiPPO Anti-Patterns

The most common corruption is that priority follows power or volume rather than value. The highest-paid person's opinion, the stakeholder who escalates, the sales rep with the biggest account, or the executive who emails the CEO, all exert pressure that bends prioritization regardless of the formal framework.

Signs of this anti-pattern:

- the same names or accounts repeatedly drive priority changes;
- items rise or fall based on who attended the last planning meeting;
- the framework's scores are adjusted after the fact to match a predetermined outcome;
- quiet but high-value work is consistently deferred because no one champions it;
- prioritization decisions reverse when a senior leader questions them.

The fix is not to remove stakeholder input but to separate evidence from influence. Require that priority changes cite evidence, not authority, and that the displaced work is named. Make the influence visible so it can be weighed rather than hidden.

### Detect Recency Bias And Last-Meeting Prioritization

Backlogs and roadmaps drift toward whatever was discussed most recently, because recent requests feel urgent and older requests fade. A team that reprioritizes based on the last standup, the last customer call, or the last executive review will produce a roadmap that reflects the sequence of conversations rather than the underlying value.

Signs of recency bias:

- the top of the backlog tracks the last few meetings rather than the strategy;
- older high-value items sink without being consciously deprioritized;
- priority churns frequently with no change in underlying evidence;
- the roadmap reverses direction based on the latest data point;
- items that were critical last month are forgotten this month with no decision recorded.

Counter recency bias with stable review cadences that re-examine the full backlog, not only the recent additions. Force items to be consciously deprioritized and recorded, rather than allowed to sink silently. Time-box reactive reprioritization so the roadmap is not rewritten after every conversation.

### Audit Scoring Frameworks For Gaming And Inflation

Any scoring framework that drives decisions will be gamed, intentionally or not, because the people who want an item prioritized will learn what scores well and produce estimates that score well. Over time, the scores drift upward, the relative ranking loses meaning, and the framework becomes a ritual that justifies predetermined choices.

Audit for gaming by examining:

- whether estimates cluster conveniently just above the threshold for inclusion;
- whether the same items score high cycle after cycle without evidence of their value;
- whether time-criticality or impact scores are inflated for favored items;
- whether size estimates are systematically lower for items someone wants to expedite;
- whether the scoring produces a ranking that matches a pre-existing gut order, suggesting the scores were reverse-engineered.

Gaming is not always deliberate. Well-intentioned people produce optimistic estimates for work they believe in. The defense is calibration: compare past estimates to actual outcomes, and adjust the estimation process when the gap is systematic. A framework whose estimates are never checked against reality will drift toward whatever is convenient.

### Watch For The Everything-Is-High-Priority Failure

When everything is high priority, nothing is. This anti-pattern emerges when a team cannot or will not make tradeoffs, so it labels most of the backlog as urgent and hopes capacity will sort it out. The result is that the prioritization system produces no signal, because the distinction between high and low has been erased.

Causes include:

- fear of saying no to powerful stakeholders;
- conflating "important" with "urgent" so that all important work becomes top priority;
- absence of a capacity constraint in the prioritization process, so nothing forces a real choice;
- treating the roadmap as a wishlist rather than a commitment.

The fix is to enforce a forced ranking or a fixed capacity bucket, so that adding an item requires removing another. Without a binding constraint, prioritization degrades into labeling, and labeling does not produce decisions.

### Diagnose The Backlog-As-Graveyard Anti-Pattern

A backlog that grows faster than it is pruned becomes a graveyard: full of items that will never be done, but never removed, because removing them feels like saying no. The graveyard destroys signal, because genuine priorities are buried among hundreds of items no one intends to build.

Signs of a graveyard backlog:

- the backlog has hundreds or thousands of items, most untouched for months;
- no one can explain why most items are there;
- adding items is easy but removing them requires a conversation no one wants to have;
- the team works from a small informal list while the formal backlog is ignored;
- stakeholders have stopped reading the backlog because it contains too much noise.

The fix is regular, ruthless pruning with a clear rule: if an item has not been prioritized within a defined window and no one will defend it, archive it. A pruned backlog is more useful than a comprehensive one, because signal requires the absence of noise.

### Recognize Effort-Driven Prioritization

A subtle anti-pattern is prioritizing by what is easy to build rather than what is valuable. Small items rise to the top because they can be finished quickly, while large strategic items sink because they are intimidating. The team stays busy and ships frequently, but the work does not accumulate into meaningful progress.

This often hides inside frameworks that divide by size, like WSJF, when size estimates are treated as the dominant factor without checking whether the small items are also high-value. The result is a roadmap optimized for throughput of completed tickets rather than for outcomes.

Counter effort-driven prioritization by periodically reviewing the portfolio of completed work and asking whether it added up to strategic movement. If the team shipped many small items but the product did not advance, the prioritization is optimizing the wrong thing.

### Check For The Framework-As-Decision-Maker Illusion

The final anti-pattern is treating the framework's output as the decision, rather than as an input to a decision. A team that says "the RICE score decided" has abdicated the judgment that prioritization requires. Scores are estimates full of uncertainty, and they cannot see strategic coherence, sequencing, or context that the numbers do not encode.

The framework informs; humans decide. When the framework's output conflicts with strategic judgment, investigate the conflict rather than overruling silently or obeying blindly. A prioritization process that never overrides the score has either a perfect framework or a team that has stopped thinking, and the latter is far more common.

## Common Traps

### Believing A Framework Prevents Corruption

Adopting RICE or WSJF and assuming the scores are honest, without auditing the inputs or comparing estimates to outcomes, lets gaming persist under cover of rigor.

### Letting Power And Volume Set Priority

Allowing the loudest voice, the biggest account, or the highest title to bend priority, then adjusting scores to match, produces a roadmap that reflects influence rather than value.

### Reprioritizing After Every Conversation

Rewriting the roadmap based on the last meeting, call, or email produces churn that reflects the sequence of conversations rather than the underlying strategy.

### Everything Is High Priority

Refusing to make tradeoffs by labeling most work urgent erases the signal that prioritization is supposed to produce. Without a binding constraint, no real choice is made.

### The Backlog Graveyard

Never pruning the backlog fills it with items no one intends to build, burying genuine priorities and causing stakeholders to stop trusting it.

### Optimizing For Completed Tickets

Prioritizing small items because they ship fast, without checking whether the completed work adds up to strategic movement, keeps the team busy without advancing the product.

### Letting The Score Decide

Treating the framework's output as the decision rather than as an input abdicates the judgment that prioritization requires and hides strategic incoherence behind arithmetic.

### Never Comparing Estimates To Outcomes

A scoring framework whose estimates are never checked against actual results will drift toward whatever is convenient, because no one notices the systematic bias.

## Self-Check

- [ ] The prioritization process was audited for anti-patterns, not assumed healthy because a framework is in use.
- [ ] Priority changes are driven by evidence rather than by the loudest voice, biggest account, or highest title.
- [ ] The backlog and roadmap are reviewed on a stable cadence for the full set, not rewritten after every recent conversation.
- [ ] Scoring estimates are compared to actual outcomes periodically to detect gaming and inflation.
- [ ] A binding capacity constraint or forced ranking prevents the everything-is-high-priority failure.
- [ ] The backlog is pruned regularly, with stale items archived rather than allowed to accumulate as noise.
- [ ] Completed work is reviewed for strategic accumulation, not only for throughput of finished tickets.
- [ ] The framework's output is treated as an input to a human decision, not as the decision itself.
- [ ] When scores conflict with strategic judgment, the conflict is investigated rather than silently overruled or blindly obeyed.
- [ ] The inputs to the framework, not only the formula, are examined for the pressures that corrupt them.
