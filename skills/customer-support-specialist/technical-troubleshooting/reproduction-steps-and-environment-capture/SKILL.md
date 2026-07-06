---
name: reproduction-steps-and-environment-capture.md
description: Use when the agent is capturing reproduction steps, environment details, logs, screenshots, device or browser context, version information, timing, expected versus actual behavior, affected users, or escalation evidence for a technical issue, bug report, known issue, or engineering handoff.
---

# Reproduction Steps And Environment Capture

Reproduction information is the bridge between a customer report and a fixable technical issue. Weak capture creates engineering back-and-forth; excessive capture burdens customers and can expose sensitive data. This skill helps the agent gather the smallest set of environment and reproduction details needed to verify, route, or escalate the issue.

## Core Rules

### Capture expected and actual behavior

Always separate what the customer expected from what happened. "It does not work" is not enough. Ask for the task, expected result, actual result, exact error, and whether the outcome changed over time.

This distinction prevents support from treating a product limitation, configuration issue, user education gap, and defect as the same thing.

### Reconstruct steps in order

Ask for the sequence leading to the issue: starting state, action taken, page or feature, inputs, button or command, result, and any retry. If the issue is intermittent, capture frequency and conditions. If it is not reproducible, capture the best known timeline.

Do not ask for perfect reproduction when the issue is intermittent but impact is high.

### Gather environment only as needed

Relevant environment may include app version, browser, OS, device, network, account role, permissions, region, feature flags, integration, API version, file type, data size, locale, accessibility tools, or third-party service. Ask for the details that plausibly affect the issue.

Avoid environment questionnaires that do not narrow cause.

### Capture timing and scope

Record when the issue started, last known good state, time zone, duration, affected users, affected accounts, frequency, recent changes, release timing, and whether others report the same symptom. Timing helps identify incidents, regressions, and configuration changes.

Relative timing like "yesterday" becomes ambiguous after handoff.

### Use safe logs and screenshots

Logs, HAR files, screenshots, screen recordings, and console output can contain tokens, private messages, personal data, account IDs, or payment details. Provide redaction instructions and secure upload paths where needed. Do not ask for secrets.

If full logs are needed, use the approved restricted process.

### Link reproduction to customer impact

Engineering severity depends on impact: blocked workflow, data loss, wrong results, security exposure, accessibility barrier, revenue loss, deadline, or workaround availability. Include impact with reproduction, not as a separate emotional note.

A perfectly reproduced low-impact bug and a poorly reproduced severe blocker require different handling.

### Note attempted workarounds and outcomes

Record steps already tried, such as refresh, reinstall, different browser, permission change, cache clear, alternate network, data re-entry, or feature toggle. Also record whether each step changed the symptom.

Do not make the customer repeat risky or already failed steps without reason.

### Package for the receiving owner

For engineering, include steps, environment, error, timing, scope, impact, logs, screenshots, account examples, and expected decision. For support tier escalation, include what was checked and what remains unknown. Keep sensitive data restricted.

The receiving owner should not need to reread the full thread.

### Preserve negative findings and respect production and customer data boundaries

Reproduction packets should include what did not happen or did not matter: issue does not occur in another browser, only affects one role, persists after cache clear, does not affect mobile, began before a release, or does not reproduce with a new file. Negative findings prevent repeated dead-end diagnostics.

Do not omit failed tests simply because they feel less useful than successful reproduction.

Some issues involve production data, confidential files, regulated records, private workspaces, or customer-owned systems. Do not ask customers to share live confidential data broadly or create test actions that alter production state. Use sanitized examples, test accounts, restricted upload, or internal reproduction where possible.

Escalation evidence should not create a second privacy or operational incident.

## Common Traps

- Recording only "customer says feature broken."
- Capturing actual behavior but not expected behavior.
- Asking for broad environment details that do not affect diagnosis.
- Missing timing, time zone, last known good state, and recent changes.
- Treating intermittent issues as invalid because they are hard to reproduce; requesting logs or screenshots without redaction guidance
- Ignoring customer impact when escalating a technical issue; making the customer repeat attempted workarounds
- Sending engineering a narrative with no concrete steps or examples; pasting sensitive logs into broad notes
- Omitting negative findings and causing the next owner to repeat failed checks; asking customers to expose confidential production data or alter live systems to prove the bug

## Self-Check

- Are expected behavior, actual behavior, task, exact error, and change over time captured?
- Are steps ordered from starting state through action, input, result, and retry?
- Is intermittent behavior captured by frequency, conditions, and timeline?
- Are environment details relevant: app, browser, OS, device, network, role, permissions, region, flags, integration, API, file, data size, locale, or accessibility tool?
- Are start time, last known good state, time zone, duration, affected users, scope, and recent changes recorded?
- Are logs, screenshots, recordings, and console output collected with redaction and secure upload where needed?
- Is customer impact tied to reproduction?
- Are attempted workarounds and outcomes recorded?
- Is the escalation packet usable without rereading the full thread?; are sensitive logs and account examples kept in restricted locations?
- Are negative findings and failed reproduction attempts recorded where they narrow the issue?; are production data, confidential files, regulated records, private workspaces, and live-system risks handled safely?
