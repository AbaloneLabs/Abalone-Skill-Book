---
name: ticket_triage_and_prioritization.md
description: Use when the agent is triaging customer support tickets, prioritizing incoming issues, deciding severity, routing cases, checking entitlement, identifying risk, or determining what information is needed before responding or escalating.
---

# Ticket Triage And Prioritization

Support triage is the point where scattered customer language becomes actionable work. A customer may describe frustration, confusion, urgency, a bug, a billing concern, a security worry, or a misunderstanding in the same message. The support specialist must identify the real issue, protect the customer, set the right priority, route the case correctly, and avoid exposing private information or making unsupported promises.

Use this skill before answering broad questions such as "how should support tickets be triaged", "what should be checked before responding to a customer issue", "how should priority be assigned", or "when should a case be escalated". The goal is to prevent the agent from treating all tickets as ordinary messages.

## Core Rules

### Identify The Customer, Product, And Entitlement

Before diagnosing, confirm the support context. The same issue can require different handling depending on customer type, account status, product tier, contract, region, channel, and permission.

Check:

- customer identity and account;
- whether the requester is authorized for the account;
- product, plan, feature, or service involved;
- subscription, warranty, service level, or support entitlement;
- region, language, and regulatory context where relevant;
- whether the issue concerns another user, tenant, organization, or child account.

Do not disclose account details merely because someone asks. When identity or authorization is unclear, use the approved verification path.

### Classify The Issue Type

Classifying the ticket helps route and prioritize it. A single message may contain several issue types.

Common types:

- how-to or usage question;
- account access issue;
- bug or defect;
- outage or performance degradation;
- billing, refund, invoice, or payment issue;
- data loss or data correction;
- security, privacy, or abuse concern;
- complaint or escalation;
- feature request;
- cancellation or retention;
- legal, compliance, or policy-sensitive request.

Do not force every ticket into "question" or "bug". Sensitive categories need special handling.

### Determine Severity By Impact And Urgency

Priority should reflect customer impact, time sensitivity, business risk, safety, privacy, and contractual obligations. It should not be based only on message tone.

Consider:

- number of affected users;
- whether work is blocked;
- revenue, safety, privacy, or compliance impact;
- whether data is lost, exposed, or corrupted;
- whether a workaround exists;
- deadline or time sensitivity;
- customer tier or service commitment;
- repeat contacts or escalation history;
- public visibility or reputational risk.

An angry low-impact message may need empathy but not top severity. A calm report of data exposure may be critical.

### Capture Reproduction And Evidence

For technical or product issues, triage should gather enough evidence for resolution or escalation.

Collect where appropriate:

- exact steps taken;
- expected result;
- actual result;
- timestamp and timezone;
- account, workspace, order, device, browser, app version, or environment;
- error messages and screenshots;
- logs or request ids if available;
- whether issue repeats;
- affected users;
- workaround tried.

Do not ask for sensitive data unless necessary. Avoid requesting passwords, full payment details, secrets, government ids, or private documents over unsafe channels.

### Route To The Right Owner

Routing is part of triage. A ticket should move to the team that can act, with enough context to prevent rework.

Possible routes:

- frontline support;
- billing or finance;
- technical support;
- product or engineering;
- trust and safety;
- security or privacy;
- legal or compliance;
- account management;
- customer success;
- operations or fulfillment.

When routing, include summary, customer impact, evidence, urgency, prior actions, and requested decision. Do not forward raw frustration without framing the issue.

### Detect Patterns

Triage should not only solve one ticket. Repeated tickets may indicate a product defect, unclear documentation, broken onboarding, billing confusion, abuse pattern, or outage.

Watch for:

- multiple customers reporting the same symptom;
- spike in a category;
- repeated confusion around the same feature;
- many contacts after a release;
- recurring billing disputes;
- similar security or fraud reports;
- unresolved tickets reopening.

Pattern detection turns support from reactive answering into product and operations intelligence.

### Set Initial Expectations Carefully

The first response should often acknowledge, classify, and set expectations. Do not promise a fix date, refund, exception, or engineering outcome before the owner confirms.

Set:

- what support understands so far;
- what information is needed;
- whether the issue is being investigated;
- expected next update time where possible;
- safe workaround if known;
- escalation path if severe.

## Common Traps

### Prioritizing By Emotion Alone

Customer emotion matters for communication tone, but priority should be tied to impact and risk.

### Missing Account Authorization

Support can accidentally expose sensitive information when responding to someone who is not authorized for the account.

### Asking For Unsafe Evidence

Screenshots and logs can contain personal data, tokens, payment details, or confidential business information. Ask narrowly and provide safe redaction guidance.

### Treating Bugs As One-Off Questions

If several customers report the same confusion or failure, it may need product, documentation, or incident handling.

### Routing Without Context

Escalated tickets without summary, impact, evidence, and request waste specialist time and delay the customer.

### Ignoring Service Commitments

Some customers have service levels, contracts, warranties, or regulatory timelines. Triage must surface them.

## Self-Check

- [ ] Customer identity, account, authorization, product, plan, and entitlement were checked where relevant.
- [ ] The issue type is classified, including sensitive categories such as security, privacy, billing, legal, abuse, or data loss.
- [ ] Severity reflects impact, urgency, affected users, data risk, workaround, service commitments, and customer consequence.
- [ ] Technical issues include steps, expected result, actual result, timestamps, environment, errors, affected users, and workaround attempts where appropriate.
- [ ] Sensitive information is not requested or exposed unnecessarily.
- [ ] Routing includes summary, evidence, impact, urgency, prior actions, and the specific decision or help needed.
- [ ] Repeated patterns, spikes, reopenings, and post-release clusters were considered.
- [ ] Initial expectations avoid unsupported promises about fixes, refunds, exceptions, or timelines.
- [ ] The triage outcome is useful to the next owner, not just the current responder.
- [ ] The customer receives an acknowledgment or next-step expectation appropriate to severity.
