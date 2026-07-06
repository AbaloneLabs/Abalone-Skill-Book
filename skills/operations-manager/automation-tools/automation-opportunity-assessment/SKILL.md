---
name: automation-opportunity-assessment.md
description: Use when the agent is evaluating whether an operational process, queue, review, handoff, decision, notification, reconciliation, or control should be automated, partially automated, simplified first, or kept human-led.
---

# Automation Opportunity Assessment

Automation can remove delay, variation, and manual effort, but it can also scale bad rules, hide exceptions, and make recovery harder. Agents often treat automation as an obvious improvement when the real question is whether the work is stable, observable, rule-bound, and safe enough to automate. This skill helps the agent assess automation opportunities before recommending tools, bots, scripts, AI workflows, routing rules, or system integrations.

## Core Rules

### Start with the operating problem, not the automation idea

Define the problem the organization is trying to solve: high volume, long cycle time, repeated rework, inconsistent routing, missed notifications, manual reconciliation, poor data capture, slow approvals, staff fatigue, or control gaps. Do not begin with "we should automate this" until the current process and pain are understood.

Ask whether the problem is caused by manual execution or by unclear policy, poor data, bad incentives, weak staffing, unnecessary approvals, confusing forms, system limitations, or unstable upstream inputs. Automating a broken process often makes the broken behavior faster and less visible.

### Assess repeatability and rule clarity

Good automation candidates have repeatable inputs, clear rules, known exceptions, reliable data, observable outcomes, and a meaningful volume or risk reduction. The agent should test whether the same case would receive the same decision from trained humans and whether the rule can be described without relying on tacit judgment.

If work requires empathy, negotiation, ambiguous policy interpretation, safety judgment, high-stakes discretion, or context not captured in systems, consider decision support rather than full automation. Human-led work can still benefit from templates, prompts, prefilled data, routing suggestions, or quality checks.

### Segment routine work from exceptions

Few processes are entirely automatable. Separate the routine flow from exceptions, sensitive cases, high-value cases, regulated cases, edge cases, and uncertain cases. Automation can handle the routine portion while routing exceptions to humans with enough context to act.

Define the automation boundary explicitly. State what the automation is allowed to do, what it must not do, what confidence or rule threshold is required, and when it should stop and ask for human review. A vague boundary creates silent overreach.

### Validate data availability and quality

Automation relies on data fields, timestamps, statuses, documents, user actions, system events, and integrations. Check whether required data exists, is current, is consistently populated, and means what the rule assumes. A field that looks structured may be optional, stale, manually interpreted, or used differently by teams.

Assess data risks: missing values, duplicate records, inconsistent categories, delayed sync, free-text ambiguity, privacy restrictions, vendor data gaps, and old records created under different rules. If data quality is weak, the first opportunity may be data cleanup or form redesign rather than automation.

### Compare benefit with operational cost

Estimate benefit in terms of cycle time, error reduction, staff capacity, service reliability, quality, customer experience, compliance evidence, and scalability. Then estimate cost: design effort, maintenance, monitoring, exception handling, support, training, tool licensing, vendor dependency, security review, and recovery complexity.

Avoid counting all manual time as savings. Some time moves to exception review, quality sampling, bot monitoring, failed-run repair, data cleanup, or user support. A strong assessment names the work that disappears and the work that changes shape.

### Consider control, fairness, and trust

Automation can create inconsistent or unfair outcomes if rules reflect biased historical practice, hidden priority, incomplete data, or unapproved exceptions. Check whether the automation affects customer eligibility, pricing, access, escalation, employee scheduling, performance measurement, compliance decisions, or sensitive personal data.

For higher-risk automation, require stronger review: legal, compliance, security, privacy, finance, HR, quality, or customer experience. The agent should be conservative when automation could deny service, create financial impact, expose private information, or make a decision people cannot contest.

### Define success and a learning path

Before recommending automation, define what success means and how it will be measured: throughput, accuracy, exception rate, cycle time, backlog age, rework, user adoption, customer complaints, control evidence, and support tickets. Include thresholds for pausing or redesigning.

When evidence is uncertain, recommend a small test, manual simulation, shadow mode, or decision-support pilot. The point is to learn whether the automation boundary and data assumptions hold before scaling.

## Common Traps

- Automating because the task is annoying. Annoyance does not prove the work is stable, valuable, or safe to automate.
- Ignoring upstream fixes. Simplifying a form, clarifying policy, or removing an unnecessary approval may outperform automation.
- Treating exceptions as rare without evidence. If exceptions are common, automation may create more handoffs and rework.
- Assuming available data is reliable. Inconsistent categories and stale statuses can make automation confidently wrong.
- Counting gross time savings. Monitoring, repair, exception review, and support work may consume much of the apparent benefit.
- Automating judgment that should remain contestable. High-impact decisions need human review, appeal, or strong governance.
- Skipping success criteria. Without a measurable learning plan, automation becomes hard to challenge after launch.

## Self-Check

- Is the underlying operating problem clear, with evidence that manual execution is a meaningful cause?
- Has simplification, policy clarification, staffing, form redesign, or upstream data improvement been considered before automation?
- Are repeatable rules, inputs, outcomes, and exceptions understood well enough to define an automation boundary?
- Are routine cases separated from exceptions, sensitive cases, high-value cases, and human-judgment cases?
- Is required data available, current, consistently used, and permitted for the automation purpose?
- Are benefits compared with maintenance, monitoring, exception handling, support, licensing, and recovery costs?
- Have fairness, privacy, compliance, customer impact, and contestability risks been considered?
- Does the recommendation include success metrics, pause criteria, and a test or learning path where uncertainty remains?
