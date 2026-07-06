---
name: timezone-handoff-and-follow-the-sun-coverage.md
description: Use when the agent is designing or reviewing timezone handoffs, follow-the-sun operations, regional coverage, global queue ownership, offshore or distributed team transitions, cross-region escalation, overnight work transfer, or service continuity across locations and working hours.
---

# Timezone Handoff And Follow The Sun Coverage

Follow-the-sun coverage can extend service hours and reduce backlog, but only if ownership, context, authority, and standards transfer cleanly. Agents often assume that a later time zone can simply continue the work. In practice, regional calendars, incomplete notes, different permissions, language constraints, local holidays, unclear escalation rights, and inconsistent quality standards can turn global coverage into fragmented work. This skill helps the agent design timezone handoffs as a controlled operating model rather than a relay of unfinished tasks.

## Core Rules

### Define The Follow-The-Sun Promise

Name what the coverage model is meant to achieve: faster response, continuous processing, incident follow-up, overnight backlog reduction, customer language coverage, regional compliance handling, or extended approval availability. The promise should include covered hours, regions, queues, customer segments, service tiers, and exceptions.

Do not imply twenty-four-hour service unless every hour has real capability, authority, and escalation support. A region that can monitor a queue but cannot act on most cases is not full coverage.

### Assign Ownership At Every Moment

Each work item needs an owner at all times. Define when ownership transfers, what happens to items not accepted by the receiving region, and who owns work during overlap gaps, holidays, weekends, and system outages. "The next region will pick it up" is not an ownership model.

For high-risk cases, consider retaining a named accountable owner while transferring execution tasks. Accountability and activity can be different, but the distinction must be explicit.

### Build Real Overlap Windows

Handoffs need overlap time for clarification, prioritization, and escalation. If regions have no overlap, the written handoff must be stronger and the service promise should reflect slower clarification. A five-minute overlap at the end of a shift is rarely enough for complex queues, incidents, or customer commitments.

Use overlap to confirm priorities, blocked items, customer promises, unusual risks, staffing issues, and decisions needed. Do not spend the overlap only on general status if urgent work needs transfer.

### Standardize Work Item State

The receiving team needs to know exactly what has happened and what must happen next. Handoff records should include current status, customer or stakeholder impact, severity, next action, owner, deadline, evidence, decisions made, blocked reason, escalation history, and any promise already communicated.

Avoid vague states such as "checking," "pending," or "in progress" without next action. These labels force the receiving team to reconstruct context and can lead to duplicate work or missed deadlines.

### Respect Regional Constraints

Regions may differ in holidays, labor rules, language capability, access permissions, data residency, vendor availability, customer expectations, regulatory limits, and escalation hours. A global schedule that ignores local constraints will overstate coverage.

Maintain a regional constraint map. Include local non-working days, restricted work types, special approvals, systems unavailable outside certain regions, and vendor or customer hours. Update it before peak seasons, product launches, or organizational changes.

### Keep One Source Of Operational Truth

Follow-the-sun work fails when each region keeps a private tracker. Use one system of record for queues, incidents, customer promises, decisions, and evidence. Chat can support coordination, but it should not be the only place where state exists.

Define required fields and quality standards for notes. Inconsistent documentation standards make later regions slower and increase risk. If a region routinely sends poor handoffs, treat it as a process quality issue, not an individual annoyance.

### Align Authority And Escalation Across Regions

The receiving region must know what it can decide independently. Some decisions may be global; others may require regional leaders, compliance, finance, product, legal, or customer owners. If authority changes by region, document it clearly.

Escalation paths should cross regions cleanly. Define who to contact when the accountable owner is asleep, when a vendor is unavailable, when a customer in another region is impacted, or when a decision must be made before the original owner returns.

### Preserve Quality And Customer Consistency

Customers and internal stakeholders should not experience a different standard merely because work moved time zones. Align templates, tone, refund or remediation rules, exception criteria, investigation steps, evidence requirements, and closure definitions.

At the same time, allow local adaptation where needed for language, regulation, cultural expectations, or market practice. The agent should distinguish global standards from legitimate regional variation.

### Measure Handoff Health

Track dropped handoffs, reopened work, duplicate effort, missed deadlines after transfer, clarification loops, aged items by region, after-hours escalations, and customer complaints about inconsistent answers. Handoff quality should be visible, not inferred from whether the queue eventually clears.

Review failed handoffs with both sending and receiving regions. The cause may be weak notes, unrealistic service promises, missing access, poor overlap, unclear priority, or regional staffing gaps.

## Common Traps

- Calling the model follow-the-sun when later regions can only monitor work and cannot actually resolve it.
- Leaving ownership ambiguous during regional gaps, holidays, weekends, or items rejected by the receiving team.
- Assuming written notes can replace overlap for complex incidents, high-risk customers, or urgent exceptions.
- Using vague statuses that hide the true next action, decision needed, deadline, or blocker.
- Ignoring local holidays, data restrictions, permissions, languages, vendor hours, and labor constraints.
- Letting chat threads become the real system of record, making later review and accountability difficult.
- Applying global rules rigidly where legitimate regional legal or customer differences require adaptation; allowing each region to use different closure standards, causing reopened cases and inconsistent customer experience
- Measuring only total throughput and missing dropped handoffs, duplicate work, clarification loops, and missed transfer deadlines; treating handoff failures as personality conflicts instead of process, authority, tooling, or documentation problems

## Self-Check

- Is the follow-the-sun promise explicit about hours, regions, queues, tiers, capabilities, and exceptions?
- Does every work item have an owner before, during, and after regional transfer?
- Are overlap windows sufficient for the complexity and urgency of the work being transferred?
- Does the handoff record include current state, impact, severity, next action, owner, deadline, evidence, decisions, blockers, and promises made?
- Have regional constraints such as holidays, labor rules, data residency, language, access, vendor hours, and customer expectations been checked?
- Is there one source of operational truth rather than separate regional trackers or chat-only state?
- Are decision rights and escalation paths clear when the original owner or region is offline?
- Are global quality standards separated from legitimate regional variation?
- Are handoff health metrics tracked beyond total throughput?
- Has the plan identified whether failures are caused by process, authority, tooling, staffing, documentation, or regional constraints?
