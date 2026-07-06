---
name: support-engineering-collaboration.md
description: Use when the agent is escalating issues from support to engineering, preparing technical evidence, discussing customer-impacting bugs, managing known issues, clarifying severity, coordinating fixes, or translating between customer impact and engineering investigation.
---

# Support Engineering Collaboration

Support and engineering collaboration fails when support sends vague urgency and engineering sends technical questions that do not connect to customer impact. Customers then receive slow updates, repeated evidence requests, or speculative answers. Good collaboration turns customer pain into reproducible evidence, clear severity, and safe communication. This skill helps the agent work with engineering without losing the customer context or making unsupported technical promises.

## Core Rules

### Translate customer impact into technical investigation

Engineering needs more than "customer is blocked." Include the customer workflow, expected behavior, actual behavior, affected objects, environment, time window, frequency, workaround, business impact, and whether the issue is reproducible. Tie emotional urgency to operational impact.

Customer impact should not be buried below screenshots. A severe bug is often severe because of money, access, data, safety, SLA, or launch risk, not because the stack trace looks interesting.

### Provide evidence before escalation

Collect logs, request IDs, timestamps, account IDs, browser or device details, app version, region, integration, plan, permissions, screenshots, video, error messages, reproduction steps, and recent changes where appropriate. Redact sensitive data and follow access controls.

Do not force engineers to ask for basic facts that support could collect safely. But do not ask customers for risky or unnecessary data just to make a ticket look complete.

### Separate suspected cause from confirmed cause

Support may have a hypothesis based on patterns, but engineering owns technical confirmation. Label guesses clearly. Avoid telling customers that the cause is deployment, database, vendor, cache, or regression until confirmed.

Speculation creates customer distrust if the final diagnosis changes.

### Align on severity and priority

Severity should consider customer impact, scope, workaround quality, affected segment, revenue, safety, data integrity, accessibility, security, contractual SLA, public visibility, and launch timing. Priority also depends on engineering capacity and product tradeoffs.

If support and engineering disagree, escalate the severity criteria rather than arguing from volume alone.

### Preserve customer commitments

Engineering investigation does not pause customer obligations. Track promised updates, escalation owners, ETA language, workaround guidance, and whether the customer expects a fix, explanation, or remediation.

Do not let internal silence become customer silence. If there is no new technical finding, update the customer with what is still being checked and the next checkpoint.

### Create known-issue discipline

When multiple customers are affected, use known-issue records, tags, watchlists, macros, and escalation criteria. Link duplicates instead of creating separate unconnected engineering tickets.

A known issue should state symptoms, affected scope, evidence needed, workaround, customer language, owner, status, and when to re-escalate.

### Close the loop after resolution

After engineering ships a fix or explains expected behavior, support must update customers, verify customer recovery, retire temporary macros, update help content, and capture lessons for triage. Do not close engineering work as customer resolution automatically.

If the fix only prevents future occurrences, identify customers who still need remediation.

### Respect engineering and customer boundaries

Do not demand direct engineer contact with customers unless the process supports it. Do not expose internal comments, blame, raw logs, or unapproved technical detail. Translate between teams with enough fidelity for action and enough restraint for customer trust.

### Decide incident versus backlog routing and keep customer updates synchronized with engineering state

Not every engineering escalation belongs in the same path. A live customer-impacting outage, data integrity issue, security concern, launch regression, or high-severity enterprise blocker may need incident or hotfix routing. A low-frequency usability defect may belong in backlog triage with a known workaround.

Support should state why the selected route fits the customer impact. If the case starts as a backlog item but impact grows, re-evaluate rather than letting the original routing hide urgency.

Define what engineering status means for customer communication: investigating, needs more evidence, reproduced, cannot reproduce, expected behavior, fix planned, fix shipped, monitoring, or no fix planned. Each state needs different wording.

Avoid telling customers "engineering is working on it" when the issue is waiting for evidence, untriaged, or accepted as a backlog item. Accurate state prevents false expectations.

## Common Traps

- Escalating with "urgent" but no workflow, scope, evidence, or impact.
- Sending raw customer frustration without technical facts.
- Asking customers for excessive logs, secrets, private data, or risky reproduction.
- Presenting support hypotheses as confirmed technical cause.
- Equating ticket volume with severity while ignoring data, safety, security, SLA, or accessibility risk; letting engineering investigation stall customer updates
- Creating many duplicate engineering tickets instead of one known-issue record; closing customer cases when a code fix merges but customer recovery is unverified
- Exposing internal engineering notes or blame to customers; failing to update macros, docs, or triage rules after resolution
- Routing all engineering asks the same way regardless of incident, hotfix, known issue, or backlog status; saying engineering is working on an issue when it is actually waiting, untriaged, or accepted as backlog

## Self-Check

- Does the escalation include customer workflow, expected behavior, actual behavior, affected objects, time window, frequency, workaround, and business impact?
- Are technical artifacts collected safely: logs, request IDs, timestamps, environment, app version, region, integration, plan, permissions, screenshots, errors, and reproduction steps?
- Are sensitive data and access controls respected?
- Are suspected causes labeled as hypotheses until engineering confirms?
- Is severity based on scope, workaround, segment, revenue, safety, data, accessibility, security, SLA, public visibility, and launch timing?
- Are customer commitments, update cadence, owner, and ETA language tracked?
- Are duplicates linked through a known-issue record with symptoms, scope, workaround, owner, and re-escalation criteria?
- Is customer recovery verified after engineering resolution?
- Are temporary macros and help content updated or retired?; is internal technical detail translated appropriately for customers?
- Is the routing path appropriate for incident, hotfix, known issue, backlog, or expected-behavior handling?; does customer messaging match the actual engineering state?
