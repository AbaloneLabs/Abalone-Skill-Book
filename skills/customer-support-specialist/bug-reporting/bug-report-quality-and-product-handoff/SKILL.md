---
name: bug-report-quality-and-product-handoff.md
description: Use when the agent is turning customer-reported defects into bug reports, preparing product or engineering handoffs, collecting reproduction evidence, tagging known issues, clarifying expected versus actual behavior, or deciding whether a support issue is a bug, configuration problem, documentation gap, or product limitation.
---

# Bug Report Quality And Product Handoff

A bug report is only useful if another team can understand, reproduce, prioritize, and communicate about it. Customer support often sees the symptom first, but weak reports omit environment, impact, expected behavior, scope, or evidence. That causes repeated customer questions and slow fixes. This skill helps the agent create bug reports that preserve customer reality while giving product and engineering actionable detail.

## Core Rules

### Define expected and actual behavior

State what the customer reasonably expected to happen and what actually happened. Expected behavior may come from documentation, prior product behavior, contract, UI copy, support guidance, or common workflow. If expected behavior is uncertain, label it.

Do not report "feature broken" without defining the mismatch.

### Capture reproduction evidence

Include steps, account state, product area, device, browser, OS, app version, plan, role, permissions, region, integration, timestamps, error messages, request IDs, screenshots, video, logs, and recent changes where relevant. Redact sensitive data.

Evidence should be enough for a teammate to attempt reproduction without re-interviewing the customer.

### Separate bug from surrounding issues

A case may include user misunderstanding, documentation gap, unsupported workflow, configuration error, permission issue, outage, data problem, or real defect. Identify which parts are confirmed and which are uncertain.

This prevents product teams from chasing the wrong problem.

### Describe scope and frequency

State whether the issue affects one customer, one account, one role, one region, one integration, one browser, all users, or only a specific data state. Include how often it occurs and whether support has seen similar cases.

Scope determines priority and routing.

### Summarize customer impact

Explain what the bug blocks: money movement, account access, order fulfillment, reporting, compliance, safety, accessibility, data integrity, onboarding, renewal, or routine convenience. Include workaround quality and customer deadline.

Impact should be written in customer terms, not only technical symptoms.

### Avoid overclaiming root cause

Support may suspect regression, release, cache, vendor, permission, or data issue. Label hypotheses as hypotheses until product or engineering confirms. Do not tell the customer the cause before confirmation.

### Choose the right handoff owner and preserve customer communication needs

Some issues belong to product, engineering, design, documentation, localization, accessibility, billing, trust and safety, data, or operations. Route by problem domain, not by habit.

Record what the customer has been told, promised update times, workaround guidance, and what answer is needed from the product owner. Engineering closure does not automatically answer the customer.

### Link duplicates and known issues and update after product response

Search for existing bugs or known issues before creating a new one. Link customer cases and add new evidence rather than fragmenting the signal.

When the product owner responds, update the customer, ticket tags, known issue, macro, help center, or escalation rule as needed. A handoff that does not come back to support is incomplete.

### Protect customer permission and data and state reproducibility and test status

Some reproduction requires access to customer accounts, files, logs, payment records, messages, or personal data. Use approved internal access processes and do not ask engineering to inspect private data casually. If customer permission is required, get it through the right channel.

Bug evidence should be useful without spreading sensitive data into broad product channels. Redact screenshots, minimize fields, and use secure links.

Indicate whether support reproduced the issue, the customer reproduced it, it is intermittent, or it is not yet reproducible. Include what was tried and what result occurred. If support could not test because of permissions or environment, say so.

This helps engineering avoid repeating failed checks and prevents support from overstating confidence.

## Common Traps

- Reporting a bug without expected versus actual behavior.
- Sending screenshots without steps, environment, or account state.
- Treating customer confusion, unsupported workflow, or documentation gap as confirmed defect.
- Omitting scope, frequency, workaround, and business impact.
- Overstating root cause or regression status before confirmation; routing every issue to engineering even when design, docs, localization, billing, or operations owns it
- Creating duplicate bug records instead of linking cases; leaving customer promises out of the bug ticket
- Asking the customer for excessive logs or private data; not updating support guidance after product responds
- Sharing customer data too broadly while trying to make the bug easier to investigate; omitting whether the issue was reproduced, intermittent, untested, or blocked by access

## Self-Check

- Are expected and actual behavior stated clearly?
- Is the source of expected behavior identified or uncertainty labeled?
- Are reproduction steps, environment, account state, plan, role, region, integration, timestamps, errors, request IDs, screenshots, video, logs, and recent changes captured where relevant?
- Is sensitive data redacted?
- Are bug, configuration, permission, documentation, unsupported workflow, outage, and data issues separated?
- Are scope and frequency described?
- Is customer impact tied to money, access, fulfillment, reporting, compliance, safety, accessibility, data, onboarding, renewal, or deadlines?
- Are hypotheses labeled rather than presented as confirmed cause?
- Is the correct owner chosen and duplicates linked?; are customer communication needs and support follow-up included?
- Are customer data, permission, internal access, and redaction requirements handled safely?; is reproducibility status and what has already been tested clearly stated?
