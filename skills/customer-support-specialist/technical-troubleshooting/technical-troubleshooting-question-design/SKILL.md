---
name: technical-troubleshooting-question-design.md
description: Use when the agent is designing troubleshooting questions, deciding what technical information to request, reducing customer effort, diagnosing software or hardware issues, avoiding irrelevant checklists, or reviewing whether questions will actually narrow cause, severity, routing, or workaround.
---

# Technical Troubleshooting Question Design

Technical troubleshooting questions shape both customer effort and diagnostic quality. A long generic checklist can frustrate the customer and still fail to narrow the issue. Too few questions can lead to bad routing or premature fixes. This skill helps the agent ask questions that are safe, necessary, sequenced, and tied to a decision.

## Core Rules

### Start with the decision the question supports

Before asking, identify what the answer will decide: reproduce locally, rule out configuration, route to engineering, identify account issue, detect incident, choose workaround, verify known issue, or confirm customer impact. If the answer will not change the next action, do not ask it now.

Questions are not evidence collection for its own sake.

### Use existing context first

Check ticket history, account metadata, product version, logs available to support, known issues, feature flags, prior attachments, error codes, order of events, and recent changes before asking the customer. If a detail is visible but uncertain, ask for confirmation rather than full repetition.

Customer effort should be spent only where support cannot safely observe the answer.

### Sequence from broad routing to narrow diagnosis

Start with high-value discriminators: affected feature, error message, timing, frequency, platform, account scope, affected users, recent change, and business impact. Then ask narrower questions only if needed. Do not ask for environment dumps before confirming the case is technical and reproducible.

Good sequencing prevents support from overwhelming the customer.

### Make questions answerable

Use plain language and define technical terms. Ask one or two grouped questions at a time when the customer is stressed. For each question, say what format is useful: exact error text, screenshot with redaction, timestamp, device model, browser, app version, steps, expected result, actual result, or affected account.

Avoid compound questions that hide several asks in one sentence.

### Avoid unsafe or destructive steps

Do not ask customers to delete data, reset devices, disable security controls, share passwords, expose tokens, run risky commands, change permissions, or test unsafe hardware unless approved and necessary. Put warnings before risky steps and provide backup or rollback guidance where policy supports it.

Troubleshooting must not create a larger incident.

### Distinguish diagnosis from workaround

Some questions support root cause analysis; others support immediate workaround. If the customer is blocked, first ask what determines a safe workaround. Deep diagnostics can follow after the customer has a path forward.

Do not make the customer complete engineering-grade investigation before giving a known mitigation.

### Capture impact and urgency

Technical severity depends on who is affected, whether there is data loss, financial impact, accessibility barrier, security concern, deadline, or no workaround. Include impact questions early enough to route correctly.

Do not assume a single-user bug is low priority if it blocks critical work.

### Stop when enough is known

Once the answers identify a route, known issue, workaround, escalation, or non-technical cause, stop asking diagnostic questions and move the case. Continuing to ask can look like deflection.

Document what was asked, what was answered, and what remains unknown.

### Adapt questions to customer capability and channel

Live chat, email, phone, community, and enterprise tickets support different question depth. A mobile customer in the field may not be able to capture logs; an admin in an enterprise portal may be able to provide configuration and affected-user scope. Match the ask to channel, device, role, accessibility needs, and time pressure.

If the customer cannot perform a step, ask for an alternative signal instead of treating the case as blocked.

## Common Traps

- Sending a generic troubleshooting checklist unrelated to the next decision.
- Asking for information support already has in logs or account metadata.
- Starting with technical environment questions before understanding impact.
- Asking many compound questions that the customer cannot answer cleanly.
- Using jargon without explaining what to provide; asking for passwords, tokens, private keys, full logs with secrets, or unsafe system changes
- Delaying workaround while pursuing perfect diagnosis; missing data loss, security, accessibility, or business impact
- Continuing diagnosis after the case should be routed; failing to document which questions were answered and what remains unknown
- Asking for logs, admin checks, or screenshots that the customer cannot reasonably capture in their current channel; treating inability to answer a technical question as lack of cooperation rather than a support constraint

## Self-Check

- Does every question support a concrete decision about reproduction, configuration, routing, incident, workaround, known issue, or impact?
- Was existing ticket, account, log, version, known issue, feature flag, attachment, error, and recent-change context checked first?
- Are questions sequenced from high-value routing to narrower diagnosis?
- Are questions clear, answerable, and formatted with expected response type?
- Are customer effort and emotional state considered?
- Are unsafe actions and sensitive data requests avoided or guarded by approved warnings?
- Is workaround need separated from root-cause diagnosis?
- Are impact, affected users, data loss, money, accessibility, security, deadline, and workaround availability captured?
- Did questioning stop once routing, workaround, known issue, or cause was clear enough?; are answers, unknowns, and next decision documented?
- Are questions adapted to the customer's channel, device, role, accessibility needs, and time pressure?; if the customer cannot answer, is there an alternative signal or escalation path?
