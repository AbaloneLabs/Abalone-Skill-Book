---
name: support_escalation_and_incidents.md
description: Use when the agent is deciding whether to escalate a support case, communicating during an incident, handling outage reports, coordinating with engineering or operations, updating customers, or managing high-risk unresolved customer issues.
---

# Support Escalation And Incidents

Escalation is not forwarding a difficult message. It is the act of bringing the right evidence, urgency, and decision need to the team that can act. Incident support is not only technical coordination; it is customer communication under uncertainty. The support specialist must avoid panic, avoid false certainty, protect customers, and keep internal teams focused on useful facts.

Use this skill before escalating a case, summarizing an urgent customer issue, communicating during an outage, writing incident updates, or deciding whether a support pattern requires incident handling. The goal is to prevent vague escalation and unsafe customer promises.

## Core Rules

### Escalate For Impact, Risk, Or Missing Authority

Not every hard ticket needs escalation, and not every escalation is technical. Escalate when the case exceeds frontline authority, requires specialist diagnosis, affects many customers, involves data/security/privacy risk, threatens contractual commitments, or creates significant customer harm.

Escalation triggers may include:

- service outage or degradation;
- suspected data loss or exposure;
- payment or billing failure affecting many customers;
- security or abuse concern;
- VIP or contractual service commitment;
- repeated unresolved contact;
- customer unable to perform critical work;
- legal, privacy, or compliance-sensitive request;
- public or reputational risk.

Do not escalate only because the customer is angry. Do escalate when anger is attached to real impact.

### Provide An Escalation Packet

The receiving team should not need to reconstruct the issue from a long ticket thread.

Include:

- concise summary;
- customer and account context;
- impact and severity;
- timeline of events;
- exact symptoms;
- reproduction steps;
- environment;
- screenshots, logs, request ids, or examples where safe;
- workaround tried;
- customer expectation;
- decision or help needed;
- next update deadline.

Good escalation reduces time to resolution and prevents customers from repeating themselves.

### Communicate Uncertainty Honestly

During incidents or unresolved escalations, support often lacks the full cause. Customers still need useful updates.

Separate:

- what is confirmed;
- what is being investigated;
- what is not yet known;
- what customers can do now;
- when the next update will arrive.

Avoid speculation, blame, internal debate, or overly technical details before confirmation. Do not say "resolved" until the fix is verified from the customer-impact perspective.

### Keep Updates Timely Even Without Resolution

Silence increases anxiety. If the issue is still under investigation, say that and provide the next update time. A useful update does not require a complete fix.

Updates should include:

- current status;
- customer impact;
- workaround if available;
- progress since last update;
- next update time;
- what support needs from the customer, if anything.

Do not keep customers waiting because the internal team has no final answer.

### Coordinate Internal And External Messages

Support, engineering, operations, customer success, account management, status page, and leadership should not send conflicting messages. Align on severity, customer impact, known facts, workaround, and public wording.

When messages differ by audience, keep the facts consistent. Enterprise customers may receive account-specific guidance, but the underlying incident state should not contradict the public status.

### Track Customer Commitments

Escalations create commitments: update times, callbacks, refunds to review, logs to send, fixes to verify, or follow-up after resolution. Track them explicitly.

Do not rely on memory during high-pressure incidents. Missed follow-up can turn a solved technical issue into a trust problem.

### Close With Resolution And Learning

After resolution, customers need to know what changed and whether they need to act. Internally, support should capture lessons.

Close with:

- issue resolved or mitigated;
- customer action needed;
- affected period;
- workaround removed or still needed;
- follow-up owner;
- any compensation or review path if applicable;
- documentation or product feedback needed.

For recurring issues, create or update known issue notes, macros, knowledge base articles, or product feedback.

## Common Traps

### Escalating Without A Clear Ask

"Please look into this" is weak. State what decision, diagnosis, action, or approval is needed.

### Promising Engineering Outcomes

Support should not promise fixes, root causes, or timelines before the responsible team confirms.

### Using Internal Jargon With Customers

Incident communication should be understandable. Customers care about impact, workaround, and next update.

### Declaring Resolution Too Early

A technical fix is not enough if customers still experience the issue or need recovery steps.

### Forgetting Affected Customers After Public Resolution

Some customers may need data repair, retries, refunds, or reassurance even after the incident is marked resolved.

### Not Capturing The Pattern

If several escalations share a cause, support should turn that into known issue tracking, documentation, or product feedback.

## Self-Check

- [ ] Escalation is justified by impact, risk, authority, severity, customer harm, or sensitive category.
- [ ] The escalation packet includes summary, account context, impact, timeline, symptoms, evidence, workaround, expectation, and specific ask.
- [ ] Customer communication separates confirmed facts, investigation, unknowns, workaround, and next update time.
- [ ] Updates continue on a reliable cadence even before resolution.
- [ ] Internal and external messages use consistent facts and severity.
- [ ] Support does not promise fixes, causes, refunds, or timelines without the right owner confirming.
- [ ] Customer commitments and follow-up deadlines are tracked.
- [ ] Resolution is verified from the customer-impact perspective.
- [ ] Post-resolution communication explains what changed and whether customer action is needed.
- [ ] Lessons are converted into known issue notes, macros, documentation, product feedback, or prevention work.
