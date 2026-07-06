---
name: webhook-and-event-delivery-support.md
description: Use when the agent is troubleshooting webhooks, event delivery, callback URLs, signing secrets, retries, duplicate events, missing events, delayed events, endpoint failures, event schemas, subscription configuration, or customer webhook consumers where risks include exposing signing secrets, confusing delivery with processing, ignoring idempotency, escalating without event IDs, or giving advice that causes data duplication or missed business events.
---

# Webhook And Event Delivery Support

Webhook support is easy to misdiagnose because the product, network, and customer's endpoint all participate. A missing event may be a subscription configuration issue, delivery failure, customer endpoint error, retry delay, filter condition, schema mismatch, or customer-side processing bug. Agents often treat webhook issues like ordinary API errors and miss event IDs, retry state, signing secrets, and idempotency. This skill helps the agent triage webhook cases with the evidence and caution they require.

Use this skill when customers report missing, delayed, duplicate, or failed webhooks; signature verification errors; event schema questions; callback URL problems; or event subscription configuration issues. The agent should distinguish delivery from customer processing and protect secrets.

## Core Rules

### Define the expected event

Ask what event the customer expected, what action should have triggered it, when it occurred, which account or environment was involved, and which endpoint or subscription should receive it. Many webhook tickets begin with "webhooks are broken" but involve one event type or filter.

Confirm whether the event is supported for the product, plan, API version, environment, and configuration. Do not assume every product action emits a webhook.

### Separate generation, delivery, and processing

Webhook flow has stages: event generated, event queued, delivery attempted, endpoint responded, retry scheduled, customer processed event. A product may deliver successfully while the customer's application fails later. A customer's endpoint may return 200 while silently dropping the event.

Triage by stage. Ask for event ID, timestamp, delivery attempt, response code, endpoint URL domain, subscription ID, and redacted logs. Avoid treating customer processing failure as product delivery failure without evidence.

### Protect signing secrets and payload data

Do not ask customers to share webhook signing secrets, raw secrets, or full sensitive payloads. If signature verification fails, use redacted headers, timestamp, algorithm, documentation, and safe sample payloads where available.

Webhook payloads may include personal, billing, order, or account data. Keep screenshots and logs limited to what is needed.

### Handle duplicate and out-of-order events carefully

Webhook systems often retry, deliver duplicates, or deliver events out of order. Support should explain approved idempotency and ordering guidance from documentation. Do not promise exactly-once delivery unless the product officially guarantees it.

If a customer's business process cannot tolerate duplicates, point them to supported design guidance or route to developer support. A support reply should not invent delivery guarantees.

### Check endpoint health and customer-side changes

Customer endpoint failures can come from DNS changes, TLS certificate expiration, firewall rules, redirects, authentication changes, deploys, rate limits, slow response times, or maintenance. Ask whether the customer changed infrastructure around the failure window.

Do not blame the customer's endpoint prematurely, but do request evidence that distinguishes product delivery from endpoint behavior.

### Use retry and replay features safely

If the product supports retry, replay, or manual resend, check scope and consequences before advising it. Replaying events can duplicate downstream actions such as invoices, emails, orders, or provisioning unless the customer consumer is idempotent.

Before replay, confirm event range, event type, endpoint, customer's ability to handle duplicates, and whether approval is required.

### Encourage customer-side monitoring

Webhook reliability depends on the customer's consumer noticing failures. Where appropriate, point customers to documented delivery logs, alerting, dead-letter handling, or failure dashboards. Support should not design their monitoring architecture, but it should flag when the customer has no way to detect missed processing.
### Escalate with event-level evidence

Engineering escalation should include event type, event ID, subscription ID, endpoint, environment, trigger action, timestamp, delivery status, response code, retry state, API version, customer impact, and whether multiple customers are affected.

Avoid escalating only with customer logs that show "missing webhook." The internal system needs event identifiers.

### Communicate delivery guarantees accurately

Webhook products differ in retry window, ordering, timeout, signature, payload versioning, filtering, and event availability. Use documented guarantees. If the documentation does not promise ordering, latency, or exactly-once delivery, do not imply it.

Set customer expectations about design patterns they must own, such as idempotent processing and monitoring.

## Common Traps

- Treating webhook failure as a generic API issue without event IDs or delivery attempts.
- Confusing product delivery success with customer processing success.
- Asking for signing secrets or unredacted payloads.
- Promising event ordering or exactly-once delivery without documented support.
- Advising replay without warning about duplicate downstream actions.
- Ignoring whether the customer can monitor delivery failures after the current case.
- Ignoring customer endpoint infrastructure changes.
- Escalating without subscription ID, endpoint, event type, timestamp, and response status; failing to check whether the expected event is actually supported or configured

## Self-Check

- Is the expected event type, trigger action, environment, and endpoint identified?
- Has the flow been separated into generation, queueing, delivery, retry, and customer processing?
- Are event ID, subscription ID, timestamp, delivery status, response code, and retry state captured where available?
- Are signing secrets and sensitive payloads protected?
- Is duplicate, retry, and ordering guidance limited to documented guarantees?
- Has customer endpoint health and recent infrastructure change been considered?
- Is replay or resend advice checked for duplicate business impact?
- Has customer-side monitoring or delivery-log visibility been considered?
- Does escalation contain event-level evidence engineering can trace?
- Are unsupported event expectations corrected clearly?; would the customer understand what part of the webhook flow is confirmed and what remains uncertain?
