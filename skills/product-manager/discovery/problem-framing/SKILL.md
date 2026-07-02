---
name: problem_framing.md
description: Use when the agent is defining a product problem, evaluating whether a feature idea is worth pursuing, clarifying user needs, framing discovery work, or turning vague product requests into a decision-ready problem statement.
---

# Problem Framing

Product work fails early when the team starts with a solution-shaped request and never names the underlying problem. A feature idea, stakeholder demand, competitor comparison, or customer complaint may be useful evidence, but it is not yet a product problem. The product manager must turn ambiguity into a clear problem frame before prioritizing, scoping, designing, or building.

Use this skill before answering broad questions such as "what should be considered before building this feature", "how should a product problem be defined", "how should we evaluate a feature request", or "how should discovery start". The goal is to prevent the agent from accepting the proposed solution as the requirement without checking user segment, pain, evidence, alternatives, business relevance, and success criteria.

## Core Rules

### Separate Request, Problem, And Solution

Start by separating three layers:

- Request: what someone asked for.
- Problem: the user, customer, business, or operational difficulty behind the request.
- Solution: one possible way to address the problem.

Example:

- Request: "Add export to Excel."
- Possible problem: "Finance users cannot reconcile monthly data in their existing workflow."
- Possible solution: Excel export, scheduled email, API access, dashboard improvements, better filtering, or accounting integration.

Do not let the request become the problem statement. A product manager should ask why the request exists, who has the pain, how often it occurs, how severe it is, and what users do today.

### Identify The User Segment Precisely

"Users" is rarely specific enough. Different segments can have different goals, constraints, willingness to pay, risk, and workflows.

Define:

- primary user;
- buyer or decision maker;
- administrator or operator;
- internal support team;
- power user versus novice;
- free versus paid customer;
- regulated versus non-regulated customer;
- new versus retained customer.

A problem for one segment may be irrelevant or harmful for another. If the proposed feature helps a loud minority but complicates the core workflow for most users, the framing must expose that tradeoff.

### Describe The Situation And Job

A good problem statement includes the situation in which the pain occurs. The same user may need different product behavior depending on urgency, device, workflow stage, team context, or external pressure.

Ask:

- What is the user trying to accomplish?
- When does the problem happen?
- What triggers it?
- What tool or process comes before and after?
- What constraints exist: time, data quality, permissions, budget, regulation, collaboration, device?
- What outcome would the user consider successful?

Avoid abstract problems like "users need better visibility". Clarify visibility into what, for whom, at what moment, to make which decision.

### Gather Evidence Before Committing Scope

Evidence can be qualitative, quantitative, operational, or strategic. The product manager should know what evidence exists and what is missing.

Evidence sources:

- user interviews;
- support tickets;
- sales calls;
- churn reasons;
- usage analytics;
- funnel drop-off;
- search logs;
- session recordings;
- customer advisory input;
- competitive analysis;
- internal operational cost;
- revenue impact;
- compliance or risk events.

Do not overfit to one anecdote. Also do not ignore repeated qualitative signals just because analytics are incomplete. Name the confidence level and missing evidence.

### Size The Pain And The Opportunity

A real problem still may not be worth solving now. Estimate severity, frequency, reach, willingness to pay, strategic relevance, and cost of delay.

Useful questions:

- How many users are affected?
- How often does the problem occur?
- How painful is it when it occurs?
- Does it block activation, retention, conversion, expansion, trust, or compliance?
- Is there a workaround?
- How expensive is the workaround?
- Would solving it change behavior or only satisfaction language?

Avoid treating "users asked for it" as proof of importance.

### Define Success Before Solution

A problem frame should include what would change if the problem were solved. Success criteria do not need to be perfect metrics at discovery stage, but they should be concrete enough to guide scope.

Examples:

- reduce time to complete reconciliation;
- increase invite acceptance;
- reduce support tickets about billing confusion;
- improve activation milestone completion;
- increase self-serve configuration success;
- reduce manual operations work;
- lower error rate in a critical workflow.

If success cannot be described without naming the feature, the team may still be solution-locked.

### Identify Non-Goals And Boundaries

Problem framing should also say what is not being solved. Non-goals prevent scope creep and make tradeoffs explicit.

Define:

- user segments not targeted now;
- workflows excluded;
- platforms or devices excluded;
- compliance or data boundaries;
- performance or reliability expectations;
- manual processes that remain manual;
- integrations not included.

Non-goals are not excuses. They are clarity for prioritization and delivery.

## Common Traps

### Treating Stakeholder Rank As Evidence

A senior stakeholder can identify important problems, but authority does not replace evidence. The agent should preserve the request and still ask for user impact, data, cost, and alternatives.

### Framing The Problem Too Broadly

"Improve onboarding" or "make reporting better" is too broad to guide work. Break it into user, situation, pain, and desired outcome.

### Framing The Problem Too Narrowly

"Add a blue button to export CSV" is already a solution. Narrow the user problem, not the implementation.

### Ignoring Current Workarounds

Workarounds reveal pain, willingness, and constraints. They can also show that the requested feature is only one step in a larger workflow.

### Confusing Frequency With Importance

A rare problem can be critical if it affects trust, compliance, revenue, or irreversible loss. A frequent annoyance may still be low priority if it has easy workarounds and low impact.

### Skipping Discovery Because The Solution Seems Obvious

Obvious features still need framing. The cost of a small wrong feature is not only build time; it includes maintenance, UI complexity, support burden, and future product direction.

## Self-Check

- [ ] The request, underlying problem, and possible solution are separated.
- [ ] The primary user segment, buyer, operator, or stakeholder group is specific.
- [ ] The problem statement includes situation, trigger, workflow context, and desired outcome.
- [ ] Evidence sources are named, and confidence level or missing evidence is explicit.
- [ ] Severity, frequency, reach, workaround cost, and business relevance were considered.
- [ ] Success is described as a changed user or business outcome, not merely feature delivery.
- [ ] Non-goals and boundaries are clear enough to prevent uncontrolled scope expansion.
- [ ] Alternative solutions were considered before accepting the requested implementation.
- [ ] The problem is neither too broad to act on nor so narrow that it is already a solution.
- [ ] The conclusion avoids treating a feature request as validated solely because someone asked for it.
