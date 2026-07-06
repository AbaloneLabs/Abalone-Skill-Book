---
name: live-reproduction-session-control.md
description: Use when the agent is running a live troubleshooting or reproduction session with a customer, including trying steps together, gathering evidence in real time, testing account behavior, reproducing intermittent bugs, coordinating with engineering, or deciding whether to stop live debugging where risks include wasting customer time, changing variables uncontrollably, missing evidence, exposing private data, overpromising a fix, or failing to convert the session into a useful escalation.
---

# Live Reproduction Session Control

Live reproduction sessions are high-signal but easy to mismanage. The customer is present, time is expensive, the environment may be real, and every step can change the evidence. Agents often keep trying guesses because the customer is available, then leave without a clean reproduction, useful artifacts, or a clear escalation. This skill helps the agent run live troubleshooting as a controlled diagnostic session.

## Core Rules

### Start with a testable objective

Define what the session is trying to prove: reproduce the error, identify the failing step, compare expected and actual behavior, capture timestamps, confirm account scope, test a workaround, or gather evidence for engineering. State the objective to the customer.

Do not begin a live session as open-ended exploration. If the goal is unclear, first gather the symptom, impact, environment, recent changes, and urgency.

### Freeze the baseline before changing variables

Capture the current state: user, role, account, device, browser or app version, network, feature flags if visible, integration state, timestamp, error text, workflow step, data sample, and what changed recently. Then change one variable at a time.

If multiple settings, devices, browsers, permissions, and data inputs change at once, the session may appear productive while destroying diagnostic clarity.

### Use safe reproduction data

Avoid reproducing with private customer records, payment data, regulated data, production destructive actions, or messages to real users unless necessary and approved. Prefer test records, duplicate cases, sandbox accounts, read-only views, or non-destructive paths.

If real data is required to reproduce, explain why, minimize exposure, and capture only the evidence needed.

### Timebox guessing and decide when to escalate

Set a reasonable live troubleshooting window. If the session does not produce evidence within that window, pause and switch to evidence collection, escalation, or follow-up. Long live sessions can exhaust the customer and increase random changes.

Escalate when the issue is reproducible but unexplained, has high customer impact, requires backend logs, affects multiple customers, involves data integrity, or needs engineering analysis.

### Capture evidence while the behavior is visible

When the issue appears, record exact time, timezone, request ID, error message, affected object, user action, account, screenshot or screen recording if policy permits, browser console or network detail if needed, and customer impact. Do not rely on memory after the session.

Evidence should be attached or summarized in the ticket in a way that the next team can act without replaying the whole conversation.

### Avoid overpromising during live diagnosis

Customers may ask, "Can you fix it now?" Be clear about what is known. A reproduced issue is not the same as root cause or fix commitment. Use careful language: "We reproduced the failure and captured the evidence needed for escalation" rather than "engineering will fix it today."

If a workaround is tested, explain limitations and risks.

### Keep the customer oriented

Narrate why each step matters, ask before changing settings, and summarize findings at checkpoints. If a step is only diagnostic and not expected to fix the issue, say so. This prevents confusion and reduces the feeling that the agent is guessing blindly.

Customer trust depends on visible diagnostic discipline, not only speed.

### End with a clean diagnostic package

Close the session with status: reproduced or not reproduced, exact evidence captured, actions attempted, changes made, workaround status, escalation owner, expected next update, and what the customer should avoid changing before follow-up if evidence depends on the current state.

The session should produce either a resolution or a better case file. If it produces neither, the process failed.

## Common Traps

- Starting a live session without a clear reproduction objective.
- Changing browser, permissions, data, settings, and network path all at once.
- Reproducing with sensitive production data when a safer test object would work.
- Continuing live guessing long after evidence collection would be more useful.
- Seeing the bug but failing to capture timestamp, error, request ID, screenshot, or affected object.
- Promising a fix because the issue was reproduced.
- Letting the customer make risky changes without explanation or authorization; ending with "we will look into it" and no diagnostic package
- Forgetting to document changes made during the session; asking engineering for help without a concise reproduction path and evidence summary

## Self-Check

- Is the live session objective testable and stated to the customer?
- Has the baseline environment, account, role, version, workflow, timestamp, and recent change context been captured?
- Are variables changed one at a time where practical?
- Is reproduction performed with safe, minimal, non-destructive data when possible?
- Is there a timebox or decision point for stopping live guessing?
- Are key evidence items captured while the behavior is visible?
- Has the agent avoided promising root cause, fix timing, or engineering outcome prematurely?
- Does the customer understand which steps were diagnostic, corrective, or temporary?
- Are actions attempted, changes made, results, and next steps documented?
- Does the final case package allow another team to continue without re-running the entire session?
