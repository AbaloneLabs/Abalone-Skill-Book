---
name: customer-impact-summary-for-defects.md
description: Use when the agent is summarizing customer impact for bugs, regressions, incidents, defects, known issues, product escalations, engineering tickets, launch issues, or prioritization reviews where technical teams need to understand real customer harm and urgency.
---

# Customer Impact Summary For Defects

Defect prioritization often fails when support describes symptoms but not customer consequences. Engineering may see a small UI bug; support may see blocked invoices, missed deadlines, lost trust, or accessibility exclusion. A strong impact summary translates customer harm into prioritization evidence without exaggeration. This skill helps the agent summarize defect impact in a way product teams can trust and act on.

## Core Rules

### Name the affected customer job

State what customers are trying to do: pay, export, invite users, fulfill orders, complete setup, send messages, access data, run reports, verify identity, file a claim, or meet a deadline. The job makes the defect meaningful.

Avoid summaries that only name the broken component.

### Describe severity in customer terms

Severity should include whether the defect blocks the task, slows it, creates incorrect output, causes rework, exposes data, affects money, undermines trust, or creates safety, legal, accessibility, or compliance risk.

Include whether the workaround is safe, complete, expensive, manual, or unavailable.

### Quantify without false precision

Use known counts: tickets, customers, accounts, users, revenue, regions, plans, repeat contacts, complaints, refunds, cancellations, or affected objects. If numbers are incomplete, state what is known and what is unknown.

Do not inflate impact by guessing.

### Include segment and timing

Impact changes by segment: enterprise, small business, consumer, admin, end user, marketplace seller, region, language, device, plan, accessibility need, or regulated customer. Timing also matters around launches, renewals, holidays, billing cycles, or customer deadlines.

Context helps teams prioritize the right cases.

### Preserve representative examples

Include concise examples or paraphrases showing the real consequence. Redact private data. Good examples explain why the defect matters more than a metric alone.

Do not overload the ticket with long transcripts.

### Distinguish current harm from potential risk and identify customer commitments

Current harm is already happening. Potential risk may happen if the defect spreads, is left unfixed, affects a larger segment, or interacts with a launch or policy. Label the difference.

This keeps urgency credible.

Record update promises, escalations, workaround guidance, refund review, SLA review, incident follow-up, or success involvement. Product teams need to know whether delay creates customer-facing commitment risk.

### Explain confidence and evidence gaps and keep the summary decision-ready

State whether impact is confirmed, likely, emerging, or anecdotal. Name missing evidence: telemetry, logs, affected cohort, reproduction, or customer confirmation.

End with what decision or action is needed: investigate, reproduce, prioritize, pause rollout, update docs, create workaround, declare incident, or accept known limitation.

### Update as evidence changes and include the cost of delay

Impact summaries should evolve when new customers report, workaround improves, severity drops, or issue is fixed. Stale impact can mislead prioritization.

Prioritization depends on what happens if the defect is not addressed soon. Describe whether delay will increase manual work, missed deadlines, refunds, churn, bad data, customer workarounds, support backlog, public complaints, or contractual exposure.

If delay has low impact because a safe workaround exists, say that too. Balanced summaries build trust.

### Avoid converting every defect into escalation theater and include support load separately from customer harm

Some defects are real but low priority compared with safety, data, security, billing, or broad availability issues. A good impact summary should be honest about limits and not inflate urgency to get attention.

If the requested action is backlog tracking rather than urgent fix, state that clearly.

Support volume and agent effort matter, but they are not the same as customer harm. A defect may create many simple contacts with low customer impact, or few contacts with severe harm. Report support load as its own signal: contacts, handle time, manual work, escalation load, and knowledge burden.

Separating the two helps product decide whether the fix is about customer risk, operational cost, or both.

## Common Traps

- Summarizing the broken feature but not the customer's blocked job.
- Calling every defect "critical" without evidence.
- Counting tickets but ignoring severity, segment, and workaround quality.
- Guessing affected population without labeling uncertainty; omitting accessibility, legal, safety, data, or compliance impact because volume is low
- Using raw customer quotes with private data; mixing confirmed impact with hypothetical risk
- Leaving customer commitments out of the product escalation; sending a long narrative with no requested decision
- Failing to update impact after workaround, fix, or new evidence; omitting what will happen if the defect is delayed
- Inflating impact for a real but lower-priority defect until product teams distrust support summaries; mixing support workload and customer harm into one vague severity claim

## Self-Check

- Is the affected customer job named?
- Is severity described through blockage, delay, incorrect output, rework, money, data, trust, safety, legal, accessibility, compliance, or customer-facing impact?
- Is workaround quality assessed?
- Are counts and scope stated with uncertainty where incomplete?
- Are segment and timing factors included?
- Are representative examples concise and redacted?
- Are current harm and potential risk separated?
- Are customer commitments and escalation risks captured?; is confidence labeled and evidence gaps named?
- Is the requested product or engineering decision clear?; does the summary explain the cost of delay or why delay is tolerable?
- Is the requested priority honest relative to safety, data, security, billing, availability, and workaround quality?; is support load separated from customer harm and described with contact, handle time, manual work, escalation, or knowledge burden?
