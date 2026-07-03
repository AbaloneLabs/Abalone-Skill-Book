---
name: backlog_triage_and_debt_management.md
description: Use when the agent is triaging incoming product requests, bugs, and ideas, balancing new feature work against tech debt and maintenance, running recurring backlog refinement, or deciding how to allocate effort between fixing and building.
---

# Backlog Triage And Debt Management

A backlog is not a wishlist; it is a working system that must be fed, sorted, pruned, and balanced continuously. The recurring job is to decide what enters, what stays, what gets done now, what is deferred on purpose, and what must be killed so it stops consuming attention. Product managers fail at triage by letting the backlog grow without bound, by treating every bug as equally urgent, by starving maintenance in favor of visible features, or by paying down debt indiscriminately without asking which debt actually matters.

Use this skill before answering broad questions such as "how should we handle incoming bugs and requests", "how much effort should go to tech debt versus features", "how do we keep the backlog healthy", or "when should we fix versus build". The goal is to prevent the agent from producing a one-time ranking instead of a repeatable process, or from letting the new-value-versus-debt balance drift until reliability or velocity collapses.

## Core Rules

### Run A Repeatable Triage Process, Not One-Time Ranking

Triage is a cadence, not an event. Inbound bugs, requests, and ideas arrive continuously, and without a repeatable process the backlog becomes a graveyard where items wait forever. Define how each inbound item is captured, assessed, and routed.

A healthy triage loop includes:

- a single intake path so nothing is lost across support, sales, feedback, and internal channels;
- a minimum information bar before an item is even accepted: repro steps, affected segment, expected versus actual behavior;
- a fixed cadence for review so items do not age silently;
- a small set of outcomes: do now, schedule, defer intentionally, investigate, or kill;
- an owner who can move an item out of the inbox.

The point is not to process everything perfectly. It is to ensure every item gets a deliberate decision rather than passive accumulation. A backlog where most items have no decision is a backlog that has lost control.

### Score Bugs On Severity, Frequency, And Affected Segment

Not all bugs are equal. A crash affecting a paid segment in a core workflow is not the same as a cosmetic glitch seen by free users once a month. Triage must separate urgency from noise using more than a single label.

Assess each bug across:

- severity: data loss, security, broken core flow, degraded experience, cosmetic;
- frequency: how often it occurs for affected users;
- affected segment: who hits it, and is that segment strategic or vulnerable;
- business impact: revenue, retention, trust, compliance, or contractual exposure;
- workaround availability and cost;
- blast radius: isolated case or systemic risk.

Severity alone over-prioritizes dramatic-but-rare bugs. Frequency alone over-prioritizes annoying-but-trivial ones. Segment context prevents the trap of a high-severity bug that almost no real user encounters. Combine the three to set response order.

### Set SLA And Response-Time Expectations Explicitly

Without explicit expectations, every bug feels urgent to someone. SLAs and response-time targets turn triage from reactive firefighting into a managed system, and they make tradeoffs visible when capacity is tight.

Define:

- response time targets by severity: how quickly the item is acknowledged and assessed;
- resolution targets or expectations: when a fix is likely, stated as a range;
- what "response" means versus "fix": triaged is not solved;
- escalation paths for contractual, security, or compliance-critical bugs;
- communication expectations to affected customers or internal teams.

SLAs should be realistic given capacity, not aspirational. A target the team cannot meet becomes a source of broken trust. When capacity cannot meet the target, the right move is to renegotiate the target or reduce incoming volume, not to silently miss it.

### Budget Maintenance And New Value Separately

The core ongoing tension is how much capacity goes to fixing, stabilizing, and maintaining versus building new value. This balance should be an explicit budget, not an emergent accident of whichever work feels loudest.

Decide and revisit:

- what fraction of capacity is reserved for maintenance, bugs, and reliability;
- what fraction goes to new value and growth;
- what fraction supports platform and tech debt paydown;
- how the split changes with product stage and current reliability pressure.

Separating the budgets prevents two failure modes: features starving maintenance until reliability breaks, and maintenance consuming everything until growth stalls. When a quarter's maintenance spend spikes, name it and decide whether it is a one-off or a signal to rebalance.

### Decide Deliberately When To Pay Down Debt

Not all debt should be paid down, and paying down all debt indiscriminately wastes capacity that could create value. Debt is acceptable when the cost of carrying it is lower than the cost of fixing it. The product manager must judge which debt actually constrains the future.

For each debt candidate, ask:

- Does it slow down current delivery or only hypothetical future delivery?
- Does it create reliability, security, or compliance risk?
- Is it in code that is actively changing or code that is stable and rarely touched?
- Will the planned next features hit this debt, making paydown a prerequisite?
- Is the debt reversible or compounding?

Pay down debt that is on the path of upcoming work, that creates real risk, or that measurably slows the team. Leave debt that is dormant, low-risk, and off the critical path. The goal is not a clean codebase; it is a codebase that does not block the work that matters.

### Prevent Backlog Bloat And Run Deliberate Cleanup

A backlog that only grows is a backlog that loses signal. Old items accumulate, duplicate each other, reference features that no longer exist, and train the team to ignore the list. Periodic pruning is part of the job.

Run cleanup with discipline:

- schedule recurring grooming at a fixed cadence, not only when it becomes unbearable;
- close items that are stale, duplicated, or no longer relevant, and say why;
- merge related requests to see true demand;
- demote items that have waited a long time without rising in importance;
- keep the actively-considered backlog small enough that every item is genuinely reviewable.

A large backlog is not evidence of thoroughness; it is evidence of avoidance. Closing an item is a decision, and a healthy backlog has a high close rate for items that will never be done.

### Weigh Support Burden And Reliability Against Features

New features are not free even after they ship. Each one adds support load, documentation, edge cases, migration risk, and reliability surface. Triage must account for the ongoing cost of the portfolio, not only the cost to build.

Consider:

- how much capacity is already absorbed supporting existing features;
- whether a new feature increases support volume, on-call load, or operational complexity;
- whether reliability work is falling behind because features keep shipping;
- whether reliability improvements would reduce support burden and free capacity.

Sometimes the highest-leverage triage decision is not to build the next feature but to stabilize what exists, because reliability gains compound into freed capacity for future features.

### Make The Fix-Versus-Build Tradeoff Visible

The fix-versus-build decision is the heart of ongoing triage, and it should be made openly rather than by default. Defaulting to features erodes the product over time; defaulting to fixes stalls growth.

Make the tradeoff explicit each cycle:

- what maintenance and debt work is being deferred, and what is the accumulating risk;
- what new value is being prioritized, and what is the expected outcome;
- what would need to change to shift the balance: a reliability incident, a growth target, a strategic shift;
- whether the current balance reflects strategy or merely inertia.

Visibility lets stakeholders challenge the balance intentionally instead of discovering, after an incident, that maintenance was quietly starved.

## Common Traps

### Treating Every Bug As Urgent

When severity is the only signal, dramatic bugs dominate and the steady stream of moderate issues that hurt more users goes unaddressed. Urgency without frequency and segment context misallocates fixing effort.

### Letting The Backlog Grow Without Pruning

A backlog that never shrinks loses meaning. Teams stop trusting it, duplicates multiply, and genuinely important items drown in noise. The trap is treating closure as loss rather than as a deliberate decision.

### Defaulting To Features And Starving Maintenance

New features are visible and rewarded; maintenance is invisible until it breaks. The result is accumulating debt and reliability decay that eventually forces a costly reallocation under crisis conditions.

### Paying Down All Debt Indiscriminately

Refactoring everything that "feels old" spends capacity on debt that carries no real cost while ignoring debt that actually blocks upcoming work. Indiscriminate paydown feels productive but is often low-leverage.

### Setting SLAs The Team Cannot Meet

Aspirational response targets create broken promises. When the team routinely misses the SLA, stakeholders lose trust in all commitments, and the SLA stops functioning as a prioritization signal.

### Confusing Loudness With Frequency

A bug reported loudly by a few vocal users can look urgent while a quiet bug affecting many users goes unseen. Triage must look at actual frequency and affected segment, not at ticket volume driven by a few reporters.

### Keeping Items Alive Out Of Optimism

Holding an item in the backlog for years because it might someday matter is not patience; it is avoidance. The cost is cognitive load and lost signal. Most old items will never be done and should be closed.

### Triaging Reactively Instead Of On Cadence

Handling bugs only when someone escalates means triage is driven by pressure, not by impact. Without a fixed cadence, the backlog drifts and decisions lag behind reality.

## Self-Check

- [ ] A repeatable triage process exists with a single intake path, an information bar, a cadence, and defined outcomes.
- [ ] Bugs are scored on severity, frequency, and affected segment, not severity alone.
- [ ] Response-time and resolution expectations are explicit, realistic, and communicated.
- [ ] Maintenance and new-value capacity are budgeted separately and revisited.
- [ ] Debt paydown decisions consider whether the debt is on the path of upcoming work and whether it carries real risk.
- [ ] The backlog is pruned on a recurring cadence with a high close rate for stale items.
- [ ] Support burden, on-call load, and reliability surface of existing features are weighed against new features.
- [ ] The fix-versus-build tradeoff is made visible each cycle, including what is being deferred and why.
- [ ] No bug is treated as urgent without frequency and segment context.
- [ ] SLAs and targets reflect real capacity rather than aspiration.
