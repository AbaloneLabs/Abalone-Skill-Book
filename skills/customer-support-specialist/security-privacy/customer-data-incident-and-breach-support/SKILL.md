---
name: customer-data-incident-and-breach-support.md
description: Use when the agent is supporting a suspected or confirmed customer data incident, data exposure, breach concern, unauthorized data access, misdirected disclosure, privacy incident, security incident customer inquiry, affected-customer communication, or post-incident follow-up where risks include premature confirmation, legal notification mistakes, over-disclosure, under-escalation, inconsistent customer messaging, weak evidence handling, or support promises that conflict with incident response ownership.
---

# Customer Data Incident And Breach Support

Customer data incidents are not ordinary support cases. A customer may report seeing another customer's data, receiving the wrong file, discovering unauthorized access, or asking whether they are affected by a public incident. Frontline agents can cause serious harm by confirming scope too early, speculating, copying sensitive evidence, promising notification timelines, or treating the case as routine troubleshooting. This skill helps the agent support customers while preserving incident response, legal notification, privacy, and trust boundaries.

Use this skill when a ticket involves suspected data exposure, breach concern, unauthorized access, misdirected communication, privacy incident, affected-customer question, incident follow-up, or customer request for breach documentation. The agent should route and communicate safely, not independently investigate or make legal determinations.

## Core Rules

### Treat suspected exposure as high sensitivity

If a customer says they saw data they should not see, received someone else's information, lost access control, or believes their data was accessed, treat it as a potential data incident. The agent does not need to prove the incident before escalation.

Do not downplay the issue because the evidence is incomplete, the customer is upset, or the exposure seems small. A single misdirected email, screenshot, export, support attachment, or tenant boundary failure can have legal and trust implications.

### Separate customer care from incident determination

Support can acknowledge receipt, collect safe facts, preserve the customer's immediate needs, and route to incident owners. Security, privacy, legal, or incident response owners determine scope, reportability, notification obligations, root cause, and official language.

Avoid statements such as "this is a breach," "you were not affected," "no one else saw it," or "we will notify regulators" unless approved. Use careful language: "We are escalating this for review" or "The appropriate team is investigating."

### Gather minimum safe facts

Collect what helps triage: customer account, what data was seen or missing, where it appeared, time observed, affected feature, screenshot with redaction if safe, recipient or sender if relevant, whether data included personal, financial, health, credential, or regulated information, and whether the issue is ongoing.

Do not ask the customer to access unauthorized data again, forward exposed data broadly, download more evidence, or share secrets. If they already possess another person's data, instruct them through approved language and route immediately.

### Preserve evidence in restricted handling paths

Incident evidence may include personal data, credentials, screenshots of other tenants, message headers, logs, exports, or attachments. Keep it in restricted fields or approved incident systems. Do not paste sensitive content into broad ticket notes, public replies, Slack channels, or unapproved documents.

Record source, timestamp, channel, original wording, attachments, redaction state, and handling path. Evidence integrity matters for investigation and potential notification.

### Use approved customer messaging only

Data incident communication must be consistent. Use approved templates or incident owner guidance for acknowledgment, status updates, affected-customer notices, post-incident summaries, and documentation requests. If no approved language exists, escalate rather than improvising.

Do not speculate about cause, attacker, employee mistake, vendor fault, number of affected users, data categories, containment, remediation, or legal obligations. Inconsistent answers can damage trust and create legal exposure.

### Manage affected and unaffected customer questions carefully

Customers may ask, "Was I affected?", "What data was exposed?", "Do I need to notify my users?", "Can you provide a report?", or "What controls failed?" These questions often require privacy, legal, security, or account management coordination.

If the customer is confirmed affected, follow the official response path. If status is unknown, say the review is ongoing and provide only approved next steps. If the customer is not affected, communicate that only if authorized and based on incident owner confirmation.

### Protect parallel support needs

The customer may still need account security help, password resets, access revocation, data correction, service restoration, or billing support. Separate immediate support actions from the incident investigation so neither is lost.

For example, if unauthorized access is suspected, route account security steps through the proper workflow while the incident team reviews data exposure. Do not make the customer wait for full incident closure to receive safe protective guidance.

### Track commitments and handoffs tightly

Incident-related promises must have owners and approved language. If support promises a callback, update, report, or escalation, record who owns it and when it will happen. Do not promise investigation timelines that incident owners have not approved.

Handoffs should include severity signal, customer impact, evidence location, immediate support actions taken, customer-facing language used, and open commitments.

## Common Traps

- Treating a reported data exposure as an ordinary bug or complaint.
- Saying "breach" or "not a breach" before legal or incident owners decide.
- Asking the customer to reproduce unauthorized data access or gather more exposed data.
- Copying screenshots or personal data into broad ticket notes or chat.
- Speculating about scope, cause, containment, or affected users.
- Giving different customers different unofficial explanations.
- Ignoring immediate account-protection steps because the formal investigation is ongoing; promising post-incident reports, regulatory notices, or timelines without approval
- Losing evidence source, timestamp, redaction state, or customer-facing language history; closing the support ticket while incident commitments remain unowned

## Self-Check

- Is the issue treated as a potential data incident even if evidence is incomplete?
- Has the agent avoided legal or incident determinations such as breach status or notification obligation?
- Were only minimum safe facts requested?
- Did the agent avoid asking the customer to access, download, or forward unauthorized data?
- Is evidence stored through restricted, approved paths with source and timestamp preserved?
- Is customer-facing language approved or explicitly routed for approval?
- Are affected-status, scope, cause, data categories, containment, and remediation statements avoided unless authorized?
- Are immediate customer protection needs handled separately from the incident investigation?
- Are handoffs clear about evidence location, severity signal, customer impact, and commitments?
- Would the response remain defensible if reviewed by security, privacy, legal, and the affected customer?
