---
name: tribal-knowledge-capture-and-transfer.md
description: Use when the agent is capturing, transferring, or reducing dependence on tribal knowledge, single points of expertise, experienced operators, informal workarounds, undocumented exceptions, tacit judgment, departure risk, cross-training, or operational knowledge held in private memory.
---

# Tribal Knowledge Capture And Transfer

Tribal knowledge is operational knowledge that works only because specific people remember it. It may include judgment, warning signs, relationships, exceptions, shortcuts, timing, supplier behavior, customer preferences, or historical reasons a process exists. Agents often convert tribal knowledge into shallow notes, but useful transfer requires capturing the decision logic and validating that others can apply it. This skill helps the agent reduce single-person dependency without stripping away the context that makes expert judgment useful.

## Core Rules

### Identify operational dependency before interviewing

Start by naming what depends on the knowledge: customer delivery, incident response, scheduling, vendor coordination, system workaround, quality inspection, compliance evidence, facility readiness, inventory continuity, or internal service handling. The goal is not to document everything a senior person knows. The goal is to reduce specific operational risk.

Prioritize knowledge that is high impact, time-sensitive, rarely used, hard to rediscover, held by one person, or used during exceptions. Routine steps that are already documented need less attention than the judgment people use when the routine fails.

### Capture decision logic, not only steps

Ask what signals matter, what tradeoffs are being weighed, what data is trusted, what exceptions are allowed, what past failures shaped the rule, and what would make the expert stop or escalate. A step list without decision logic leaves novices unable to handle real variation.

Capture examples of normal cases, edge cases, false alarms, and cases where the expert changed their mind. These examples teach pattern recognition better than abstract advice.

### Surface hidden relationships and timing

Operational knowledge often lives in relationships: which vendor contact actually resolves urgent issues, which finance owner approves a rare exception, which site lead must be warned before a change, or which customer requires special notice. It also lives in timing: cutoff times, seasonal peaks, shift routines, inspection windows, system batch jobs, and informal deadlines.

Document relationship-based knowledge carefully. Replace personal dependency with role, channel, escalation rule, and evidence requirement where possible. Do not create a document that says "ask Maria" unless Maria's role and backup path are defined.

### Distinguish legitimate workaround from risky shortcut

Experienced operators often know workarounds that keep service moving. Some are legitimate temporary paths; others bypass controls, create privacy risk, hide defects, or depend on personal authority. Capture the workaround, why it exists, when it is allowed, who approves it, what risk it carries, and what permanent fix would remove the need.

Do not publish risky shortcuts as standard practice without control-owner review. Undocumented knowledge can be dangerous, but badly documented shortcuts can scale the danger.

### Convert knowledge into usable artifacts

Choose the artifact that fits the use: runbook, checklist, decision tree, escalation map, glossary, annotated example, training scenario, FAQ, form guidance, dashboard note, vendor contact card, or incident appendix. Not every transfer belongs in a long article.

Place knowledge where it will be used. A critical exception path buried in an onboarding document will not help during an incident.

### Validate transfer by performance

Knowledge is not transferred until someone else can use it correctly. Test with a real or realistic case. Watch whether the learner can find the artifact, interpret the signal, choose the next action, explain the risk, and know when to escalate.

If the learner succeeds only because the expert is nearby, more context or a different artifact is needed. If the learner follows the artifact but gets a poor result, the captured knowledge is incomplete or the original process is fragile.

### Respect people and incentives

Experts may resist transfer because knowledge gives status, they are overloaded, they fear blame for undocumented workarounds, or they have never been given time to teach. Treat capture as risk reduction and recognition, not extraction.

Give experts time, credit, and a clear review role. Avoid implying that the goal is to make them replaceable tomorrow; the practical goal is resilience and shared quality.

### Keep transferred knowledge alive

After transfer, assign ownership, review triggers, and feedback paths. Newly documented knowledge decays when systems change, vendors rotate, customers change requirements, or the original expert leaves. Tie the artifact to incidents, training, audits, process changes, and operational reviews.

If the knowledge reveals a brittle process, create an improvement action. Documentation can reduce dependency, but it should not be used to preserve unnecessary fragility forever.

## Common Traps

- Asking experts to "document what you know" without naming the operational dependency.
- Capturing steps while missing judgment, signals, tradeoffs, and stop conditions.
- Writing down names of helpful people instead of roles, channels, backups, and authority.
- Publishing personal workarounds without checking privacy, compliance, safety, financial, or customer risk.
- Treating a long interview transcript as transferred knowledge.
- Storing critical exception guidance somewhere users will not search during the real event.
- Assuming transfer succeeded because an article exists; ignoring the expert's incentives, workload, fear of blame, or need for recognition
- Failing to update the knowledge after the original expert leaves; using documentation to preserve a fragile process that should be redesigned

## Self-Check

- Is the specific operational dependency and risk of tribal knowledge clear?
- Has priority been given to high-impact, time-sensitive, rare, hard-to-rediscover, or single-person knowledge?
- Does the captured content include signals, tradeoffs, trusted data, exceptions, stop conditions, and historical reasons?
- Are normal cases, edge cases, false alarms, and changed-mind examples included where useful?
- Are relationship and timing dependencies converted into roles, channels, backups, and escalation rules?
- Are workarounds classified by legitimacy, approval, risk, allowed conditions, and permanent-fix need?
- Is the chosen artifact appropriate for how and when the knowledge will be used?
- Has a non-expert validated the artifact on a realistic case without relying on the expert's live coaching?
- Are expert incentives, time, credit, and review role handled respectfully?
- Does the transferred knowledge have owner, review trigger, feedback path, and improvement follow-up for brittle processes?
