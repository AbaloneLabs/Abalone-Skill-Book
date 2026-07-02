---
name: technical_debt_and_architecture_decisions.md
description: Use when the agent is taking a shortcut under deadline pressure, deciding whether to defer a fix, tracking or prioritizing technical debt, recording a significant architecture or technology decision, deciding whether to rewrite or patch, evaluating the boy scout rule in a messy area, planning a debt paydown sprint, or reviewing whether an intentional compromise is still acceptable or has become a liability. Also covers intentional versus accidental debt, debt quadrants, architecture decision records (ADR), decision logs, the cost of undocumented decisions, re-architecting triggers, and the difference between debt that enables delivery and debt that silently compounds.
---

# Technical Debt And Architecture Decisions

Technical debt is a metaphor, and like any metaphor it is precise about some things and misleading about others. The precise part: a shortcut taken now trades future speed for present speed, and the principal plus interest will come due. The misleading part: unlike financial debt, technical debt is rarely tracked, the interest rate is unknown, and the "lender" is the future team that inherits the code — often a different team that never agreed to the loan. The judgment problem is not "avoid all debt" — a team that never takes a shortcut ships nothing — but "take debt deliberately, record it, and pay it down before the interest exceeds the value of what the shortcut bought."

Agents tend to err in two directions. Some treat every imperfection as debt that must be paid immediately, turning every task into a refactor and never delivering. Many more take shortcuts silently, with no record, no plan, and no trigger for repayment, so the debt compounds invisibly until a feature that should take a day takes a month because it must first navigate a tangle no one documented. This skill exists to make debt a deliberate, recorded decision rather than an accidental accumulation, and to make architecture decisions durable artifacts rather than vanishing conversations.

## Core Rules

### Distinguish Intentional Debt From Accidental Debt

Not all debt is the same, and the response differs by kind. The useful classification is the debt quadrant: deliberate versus inadvertent, and prudent versus reckless.

- **Deliberate and prudent.** You knowingly take a shortcut to hit a deadline or validate a hypothesis, you understand the cost, and you record it. This is debt as a tool. Example: shipping a single-region deployment to launch, knowing multi-region failover is deferred.
- **Deliberate and reckless.** You take a shortcut knowing it is bad and with no plan or record. "We'll fix it later" with no later defined. This is debt as negligence.
- **Inadvertent and prudent.** You did not know better at the time, but in hindsight the design was limited. This is the debt of learning; it appears as the system matures and you understand it better.
- **Inadvertent and reckless.** You did not know better, and the shortcut was also unstructured — copy-pasted code, no tests, magic strings. This is the most expensive kind, because there was never a moment it was a good idea.

The response differs. Deliberate-prudent debt needs a record and a repayment trigger. Inadvertent debt needs recognition and a decision about whether to pay it down or accept it. Deliberate-reckless and inadvertent-reckless debt need to stop being created, which means changing how shortcuts are taken, not just paying down the existing pile.

### Record The Decision, Not Just The Outcome

An architecture decision that lives only in a chat thread or a developer's memory is lost the moment that context is gone. The team will revisit the same question, re-derive the same answer or a worse one, and never know a decision was already made. An architecture decision record (ADR) is a short, immutable document that captures the context, the decision, the alternatives considered, and the consequences.

A useful ADR is brief and structured:

- **Context.** What force or problem forced the decision? What constraints (scale, team, deadline, compliance) were in play?
- **Decision.** What did we choose, stated in one paragraph?
- **Alternatives considered.** What else was on the table, and why was it rejected? This is the most valuable section, because it prevents re-litigating.
- **Consequences.** What do we accept by choosing this? What becomes easier, what becomes harder, and what new risk did we introduce?

Write the ADR when the decision is made, not retroactively. Date it, and when a decision is superseded, mark it superseded and write a new one rather than editing the old — the history of decisions is itself valuable. A repository with thirty ADRs answers "why is it like this?" in seconds; one with none re-asks the question forever.

### Make Debt Visible And Trackable

Debt that is not visible cannot be prioritized. A shortcut taken in code with no record is invisible until it bites. Make debt visible where the team works:

- A `TODO(debt)` or `TODO(shortcut)` comment with a brief note on the cost and the trigger for repayment, at the site of the shortcut. This is the lightest tracking and the minimum bar.
- A tracked issue for any shortcut with a real repayment plan, tagged as debt, with the context and the cost noted.
- For systemic debt (a fragile module, a missing abstraction, a technology that must be replaced), a short note in the architecture documentation or an ADR describing the debt and the intended direction.

The goal is not to enumerate every imperfection — that produces a backlog no one works — but to ensure that debt with a real cost is findable when the time comes to prioritize. A debt that no one can find will be rediscovered under deadline pressure, when there is no time to address it properly.

### Define A Repayment Trigger, Not Just A Repayment Intention

"We'll fix it later" is not a plan; later never comes on its own. A deliberate shortcut needs a trigger: a specific condition that says "now is when this debt must be paid." Good triggers are concrete:

- **Event-driven.** "When we add the second payment provider, refactor the payment module to the port pattern." The debt is paid when the force that makes it expensive arrives.
- **Metric-driven.** "When deploy time exceeds 30 minutes, replace the build script." The debt is paid when its cost crosses a measured threshold.
- **Scheduled.** "In the next quarter's hardening sprint, replace the in-memory session store." The debt is paid at a reserved time.
- **Feature-coupled.** "When we touch this module for the next feature, pay down the debt in the area we are changing." This is the boy scout rule in its deliberate form.

A shortcut without a trigger is debt with no repayment plan, which is reckless debt regardless of the original intent. Pair every deliberate shortcut with the condition under which it will be addressed.

### Apply The Boy Scout Rule Deliberately, Not Compulsively

The boy scout rule — leave the code a little better than you found it — is sound, but it is often misapplied. The deliberate version is: when you are already in an area for a feature or fix, pay down the debt that is in your path and that makes your change harder. The compulsive version is: every time you touch a file, refactor whatever bothers you, expanding the scope of every task and delaying every delivery.

The distinction is scope discipline:

- **In path and blocking.** Clean it up; it is part of the cost of the change you are making.
- **In path but not blocking.** Note it as debt and leave it; do not expand the task.
- **Out of path.** Do not touch it; a drive-by refactor in unrelated code adds review burden and risk without unblocking the work.

The boy scout rule is a habit of incremental improvement, not a license to refactor the codebase one task at a time. Bound it to the change at hand, and route larger cleanup through dedicated debt work.

### Recognize When Debt Has Become A Liability

Some debt is tolerable for years; some crosses a threshold where it must be paid down or the system becomes unworkable. The signals that debt has become a liability rather than a manageable cost:

- **Change amplification.** A simple feature requires touching many unrelated modules, because the debt has spread coupling across the system.
- **Fear of change.** Developers avoid touching a particular area because it is fragile, untested, or misunderstood — the debt has made the code risky to modify.
- **Velocity decline.** The team's throughput drops over time even though the team is the same size, because each change must navigate accumulated debt.
- **Repeated incidents.** The same class of bug recurs in the same area, because the debt prevents a structural fix and only symptom patches are possible.

When these signals appear, incremental boy-scout cleanup is no longer sufficient; the debt needs a dedicated paydown effort, possibly a partial rewrite or re-architecture. Ignoring the signals leads to a system no one can safely change.

### Decide Rewrite Versus Refactor Based On The Debt's Nature

When debt is severe, the question becomes whether to refactor incrementally or rewrite. This is a high-stakes decision, and the right answer depends on the debt's nature:

- **Refactor incrementally** when the existing design is basically right but degraded, the code is testable or can be made testable, and the system's behavior is understood. Incremental refactor preserves behavior at every step and is reversible.
- **Rewrite** when the existing design is fundamentally wrong, the code is untestable and incomprehensible, and the cost of understanding and refactoring exceeds the cost of starting over. Rewrite is high-risk (you must reproduce all the behavior the old system accumulated, including the parts no one remembers) and should be a last resort, approached with the strangler-fig pattern rather than a big-bang replacement.

The default should be refactor. Rewrites fail when they underestimate how much behavior the old system encodes. Reserve rewrite for cases where the old system's design cannot be salvaged and where the team can sustain a parallel-run migration.

## Common Traps

### Taking Shortcuts With No Record

A shortcut taken to hit a deadline, with no TODO, no issue, and no note. The debt exists but is invisible, so it is never prioritized and is rediscovered under the next deadline, when it is again too late to fix. The minimum bar for any deliberate shortcut is a record at the site and a tracked note.

### "We'll Fix It Later" With No Later

A repayment intention without a trigger. Later never arrives on its own; the debt compounds until it becomes a liability. Pair every shortcut with the specific condition under which it will be addressed.

### Treating Every Imperfection As Debt

Cataloging every style nitpick and minor duplication as debt, producing a backlog so large it is never worked and that obscures the few items with real cost. Reserve debt tracking for shortcuts with a real future cost; let trivial imperfections be handled by normal review.

### Drive-By Refactors Under The Boy Scout Banner

Refactoring unrelated code every time a file is touched, expanding every task's scope and delaying delivery. The boy scout rule applies to debt in the path of the current change, not to whatever bothers the developer across the codebase.

### Undocumented Decisions Re-Litigated Forever

A significant architecture decision made in a meeting or chat, with no record. Six months later a new team member proposes the rejected alternative, because no one knows it was already considered and turned down. Record decisions as ADRs at the time they are made.

### Editing Old ADRs Instead Of Superseding

When a decision changes, editing the original ADR to reflect the new decision. The history of why the old decision was made is lost, and the team cannot tell what was considered when. Mark old ADRs superseded and write new ones; the sequence is the artifact.

### Big-Bang Rewrite To Escape Debt

Deciding the debt is so severe that the only option is to throw the system away and start over, underestimating how much behavior the old system encodes. Big-bang rewrites reproduce a fraction of the old behavior and rediscover the rest in production. Prefer strangler-fig incremental replacement; reserve full rewrite for cases where the design is irrecoverable.

### Ignoring The Signals That Debt Has Become A Liability

Continuing to patch around accumulating debt while change amplification, fear of change, velocity decline, and repeated incidents worsen. By the time the team accepts that a structural response is needed, the debt has often grown beyond what incremental cleanup can address. Act on the signals early, when the cost of paydown is still manageable.

## Self-Check

- [ ] Each shortcut is classified as deliberate-prudent, deliberate-reckless, inadvertent-prudent, or inadvertent-reckless, and reckless debt is being stopped at the source rather than only paid down after the fact.
- [ ] Deliberate shortcuts are recorded at the site (TODO with context) and tracked as issues with the cost noted, so debt with real cost is findable when prioritization happens.
- [ ] Every deliberate shortcut has a repayment trigger (event, metric, schedule, or feature-coupled), not merely an intention to fix it later.
- [ ] Significant architecture and technology decisions are recorded as ADRs at the time they are made, with context, decision, alternatives considered, and consequences; superseded decisions are marked, not edited.
- [ ] The boy scout rule is applied within the scope of the current change (debt in path), not as a license for drive-by refactors that expand every task.
- [ ] The signals of debt-as-liability (change amplification, fear of change, velocity decline, repeated incidents) are monitored, and a structural paydown is triggered before the debt exceeds what incremental cleanup can address.
- [ ] The refactor-versus-rewrite decision is based on the debt's nature (design salvageable and testable versus fundamentally wrong and incomprehensible), with rewrite reserved as a last resort approached via strangler-fig, not big-bang.
- [ ] Debt tracking distinguishes real-cost shortcuts from trivial imperfections, so the backlog reflects items worth working rather than every style nitpick.
- [ ] No significant decision exists only in chat or memory; the team can answer "why is it like this?" from recorded artifacts in seconds.
- [ ] The team periodically reviews accumulated debt and retires items whose cost never materialized, so the debt backlog does not grow without bound.
