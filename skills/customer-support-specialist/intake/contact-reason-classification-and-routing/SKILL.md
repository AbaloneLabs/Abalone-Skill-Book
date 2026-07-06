---
name: contact-reason-classification-and-routing.md
description: Use when the agent is classifying customer contact reasons, selecting ticket categories, routing cases to the right queue or owner, correcting misrouted tickets, designing intake taxonomy, or deciding whether a support conversation belongs to billing, technical, account, policy, incident, trust, sales, customer success, or product feedback workflows.
---

# Contact Reason Classification And Routing

Contact reason classification determines who sees the case, which macro or policy is used, what metrics are recorded, and whether the customer reaches the right resolution path. It is easy to treat classification as administrative tagging, but wrong classification creates rework, reporting distortion, policy mistakes, and customer frustration. This skill helps the agent classify and route by the customer's actual need and risk.

## Core Rules

### Classify by customer intent and support action

A strong contact reason captures both what the customer is trying to achieve and what support must do. "Billing" may be too broad; "refund request," "unexpected charge," "invoice copy," and "payment failed" have different evidence, authority, and risk.

Avoid classifying only by the product surface mentioned. The customer may mention one feature while asking for account recovery, compensation, or policy exception.

### Distinguish symptom from root category

At intake, the visible symptom may not be the true route. "I cannot log in" may be password reset, account suspension, identity verification, outage, device issue, fraud, or user education. Start with the best provisional route, but keep the case easy to reclassify.

Do not force root cause certainty before routing if the case needs an owner now.

### Route sensitive categories conservatively

Billing disputes, account access, security, privacy, trust and safety, legal requests, safety-critical reports, abuse, harassment, regulated data, and fraud should go to trained queues or escalation paths. These cases may require special verification, restricted notes, approved language, and audit records.

If the case has mixed signals, route to the safer owner or ask a lead before taking risky action.

### Keep routing rules explainable

Agents should know why a case goes to a queue: ownership, authority, skills, system access, language, account tier, region, risk, or service level. Routing rules that are opaque become folklore and create inconsistent behavior.

Document common examples and boundary cases.

### Avoid category inflation

Too many categories make agents choose inconsistently; too few categories hide important work. Create or use categories that support action, reporting, staffing, quality review, and product feedback. If a tag does not change decisions or insight, it may not need to exist.

Where fine detail is useful, consider secondary tags rather than many primary queues.

### Correct misroutes visibly

When correcting a misroute, update category, queue, priority, notes, and customer expectation. Include what evidence caused the reroute and what the receiving team should do. If the customer has already waited, consider whether priority or apology is needed.

Do not simply bounce tickets between queues with no ownership.

### Watch classification effects on metrics

Contact reason data feeds staffing, self-service investment, product prioritization, defect trends, and policy review. Wrong tags can make a real problem disappear or make a team look worse than it is.

If many agents choose "other," "general," or "unknown," the taxonomy or training is failing.

### Respect customer language while using internal taxonomy

Customers do not speak in internal categories. Translate their words into the internal taxonomy without losing nuance. Capture customer phrasing when it reveals confusion, harm, or product feedback.

Do not let internal labels flatten the customer's real experience.

### Preserve cross-functional signal when routing away

Routing a case to another queue does not mean the original signal disappears. A cancellation caused by confusing onboarding may belong to retention, but it may also contain product education feedback. A refund request caused by a defect may belong to billing, but it should still preserve defect evidence. A safety complaint may require trust and safety ownership while also informing policy or product teams.

When a case contains more than one useful signal, choose one accountable primary route and add secondary tags or notes for the remaining signals. This prevents ownership confusion while keeping reporting useful. Do not split a customer conversation into multiple disconnected tickets unless the workflow requires it and the customer will not be forced to repeat context.

## Common Traps

- Treating category selection as clerical work rather than routing and risk control.
- Classifying by product area when the actual need is billing, access, policy, or safety.
- Waiting for perfect root cause before routing to an owner.
- Routing sensitive cases through normal queues because the first message seems routine.
- Creating so many categories that agents choose randomly; using broad categories that hide staffing, defect, or policy problems
- Correcting a misroute without updating notes, priority, and customer expectation; allowing queue-to-queue bouncing with no accountable owner
- Ignoring high "other" or "unknown" usage; losing the customer's language and experience in internal tags
- Dropping product, policy, or education signal because the ticket was routed to a transactional queue; creating multiple parallel cases without a clear owner and customer-visible continuity

## Self-Check

- Does the classification capture customer intent and support action?
- Is the visible symptom distinguished from possible underlying route?
- Are mixed or uncertain cases routed provisionally without pretending root cause is known?
- Are sensitive categories such as billing, access, security, privacy, legal, safety, abuse, and fraud routed conservatively?
- Are routing rules based on ownership, authority, skill, access, language, tier, region, risk, and service level?
- Are boundary examples documented enough for consistent routing?
- Are categories useful for action, reporting, staffing, quality, self-service, and product feedback?
- If a case was misrouted, are category, queue, priority, notes, and customer expectation corrected?
- Are "other," "general," and "unknown" usage monitored as taxonomy or training signals?; does the record preserve customer wording where it reveals confusion, harm, or feedback?
- If the case includes multiple signals, is there one primary accountable route plus secondary tags or notes?; would the receiving team understand which part of the customer's need they own and which signals should be preserved for other teams?
