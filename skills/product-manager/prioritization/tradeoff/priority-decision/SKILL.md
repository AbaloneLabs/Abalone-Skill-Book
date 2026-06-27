---
name: priority_decision.md
description: Use when the agent is deciding product priorities, comparing roadmap options, evaluating feature tradeoffs, sequencing work, balancing customer requests, or explaining why one product initiative should happen before another.
---

# Priority Decision

Prioritization is not ranking ideas by enthusiasm. It is choosing which problems deserve limited team capacity now, which risks are acceptable, and which opportunities must wait. A product manager must compare impact, confidence, effort, timing, strategy, risk, dependencies, and opportunity cost without hiding behind a formula.

Use this skill before answering broad questions such as "what should we prioritize", "how should roadmap tradeoffs be made", "which feature should come first", or "how should customer requests be evaluated". The goal is to prevent the agent from producing a ranked list that ignores evidence quality, strategic fit, sequencing, maintenance cost, or stakeholder tradeoffs.

## Core Rules

### Prioritize Problems Before Solutions

Compare the importance of problems before comparing feature ideas. Several solutions may address the same problem, and one solution may address multiple problems poorly.

For each candidate, identify:

- user or customer problem;
- target segment;
- business outcome;
- evidence strength;
- urgency;
- available alternatives;
- cost of doing nothing;
- risk of solving it badly.

If the problem is unclear, the priority score is mostly decoration.

### Make Capacity And Opportunity Cost Explicit

Every prioritized item displaces something else. Roadmaps fail when they list what to do without naming what will not happen.

Ask:

- Which team capacity is required?
- What other work is delayed?
- Does this consume design, engineering, data, legal, sales, support, or operations time?
- Does it add future maintenance?
- Does it create new support burden or documentation work?
- Does it block or enable other work?

Opportunity cost should be discussed in plain language, not hidden inside a spreadsheet.

### Evaluate Impact With Multiple Lenses

Impact can mean revenue, retention, activation, engagement, conversion, trust, compliance, cost reduction, quality, strategic positioning, or learning. The product manager should choose the relevant lens instead of using one generic meaning.

Possible impact lenses:

- user pain relief;
- number of affected users;
- depth of pain;
- willingness to pay;
- revenue expansion;
- churn reduction;
- risk reduction;
- operational efficiency;
- market differentiation;
- strategic platform value;
- learning value.

Avoid comparing unlike goals as if they were one metric unless leadership has explicitly chosen that metric.

### Weight Confidence Separately From Impact

A large imagined impact with weak evidence is not the same as a large proven impact. Separate expected impact from confidence.

Confidence can come from:

- repeated customer interviews;
- behavioral data;
- support volume;
- sales evidence;
- retention analysis;
- experiments;
- competitive pressure;
- regulatory deadline;
- internal operational data.

Name uncertainty. Sometimes the right priority is a discovery or experiment task that reduces uncertainty before full build.

### Include Timing And Sequencing

Some work matters because of timing: contractual commitments, market windows, regulatory deadlines, seasonal demand, dependencies, migration needs, or coordination with another launch. Other work can wait without much cost.

Sequencing questions:

- Does this unblock future work?
- Does another initiative need this foundation?
- Is there a deadline?
- Would delay materially reduce value?
- Can a smaller slice produce learning sooner?
- Can discovery happen in parallel with unrelated delivery? 

Do not prioritize only by static value. Time can change value.

### Account For Risk And Reversibility

Some decisions are easy to reverse; others create long-term commitments. A low-effort feature can still be risky if it changes user expectations, pricing, permissions, data model, brand promise, or support obligations.

Review:

- security and privacy risk;
- compliance risk;
- operational risk;
- technical debt;
- user confusion;
- brand risk;
- migration risk;
- maintenance cost;
- reversibility.

High-risk work may still be high priority, but it needs stronger evidence, staged rollout, or risk mitigation.

### Communicate The Decision, Not Just The Score

Frameworks like RICE, ICE, MoSCoW, Kano, or cost-of-delay can help structure thinking. They should not replace judgment. The product manager must explain why the decision makes sense.

A useful priority explanation includes:

- what is being prioritized;
- why now;
- expected outcome;
- evidence level;
- what is deferred;
- main risks;
- next review point.

This creates alignment and makes future changes easier to explain when facts change.

## Common Traps

### Ranking By Loudest Customer

Large or loud customers matter, but their requests must be compared against target segment, revenue, strategy, support cost, and impact on other users.

### Treating Effort Estimates As Exact

Early effort estimates are uncertain. Use ranges and identify unknowns. If effort uncertainty drives the decision, prioritize discovery or technical investigation.

### Overvaluing New Features

Maintenance, reliability, performance, usability, onboarding, support tooling, and documentation can have higher impact than visible new features. Roadmaps biased toward novelty accumulate product debt.

### Hiding Strategic Bets In Fake Certainty

Some priorities are strategic bets with weak evidence but high potential. Say that directly. Do not present speculative work as proven impact.

### Ignoring Dependency Cost

A feature may look small but require platform changes, data migration, policy review, or go-to-market coordination. Count the real system cost.

### Changing Priority Without Updating The Narrative

Priorities can change, but the reason should be communicated. Otherwise teams interpret changes as randomness.

## Self-Check

- [ ] The priority comparison starts from problems and outcomes, not only requested solutions.
- [ ] Target segment, business objective, urgency, evidence, and cost of doing nothing are explicit.
- [ ] Opportunity cost and deferred work are named.
- [ ] Impact is evaluated using the relevant lens: revenue, retention, activation, trust, compliance, efficiency, strategy, or learning.
- [ ] Confidence is separated from imagined impact, and uncertainty is visible.
- [ ] Timing, deadlines, dependencies, sequencing, and unblock value were considered.
- [ ] Risk, reversibility, maintenance, support burden, privacy, security, and compliance were reviewed.
- [ ] Effort is treated as a range when uncertainty is high.
- [ ] The decision explanation includes why now, what is deferred, main risks, and next review point.
- [ ] A prioritization framework did not replace explicit product judgment.
