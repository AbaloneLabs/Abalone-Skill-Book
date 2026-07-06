---
name: workaround-and-known-issue-guidance.md
description: Use when the agent is advising on a workaround, known issue, bug, service degradation, temporary mitigation, feature limitation, incident-related symptom, compatibility issue, or customer communication that must avoid overpromising fixes while helping the customer continue safely.
---

# Workaround And Known Issue Guidance

Workarounds can reduce customer harm, but they can also create data loss, security risk, hidden cost, or false confidence. Known issue messaging can help customers avoid repeated troubleshooting, but premature confirmation can mislead or expose internal status. This skill helps the agent give mitigation guidance that is useful, safe, and honest about uncertainty.

## Core Rules

### Confirm whether it is truly a known issue

Check approved known-issue sources, incident channels, release notes, support alerts, engineering updates, product status pages, and internal macros. Do not label an issue "known" because it sounds similar to past cases.

If similarity is uncertain, say the team is investigating related reports rather than confirming a known bug.

### Match workaround to customer context

A workaround must fit the customer's platform, permissions, data sensitivity, accessibility needs, technical skill, business workflow, and deadline. A workaround that requires admin access, data export, command-line use, manual re-entry, or third-party tools may be inappropriate for some customers.

Do not give a workaround simply because it worked for another case.

### Explain limits and side effects

State what the workaround does, what it does not fix, what risk it carries, and whether it is temporary. Mention possible data loss, duplicate work, billing effect, security tradeoff, accessibility issue, feature loss, performance impact, or need to revert later.

Hidden side effects create second-order support problems.

### Avoid unsafe mitigations

Do not recommend disabling MFA, lowering security settings, sharing credentials, bypassing permissions, deleting data, using unofficial tools, installing unknown software, changing production configurations, or ignoring safety warnings unless an approved procedure explicitly allows it.

The workaround should not be more dangerous than the issue.

### Preserve fix expectations

Be clear about whether there is a confirmed fix timeline, active investigation, no ETA, planned release, customer action, or follow-up mechanism. Do not promise engineering fixes, release dates, or root cause unless approved.

Customers can handle uncertainty better than broken promises.

### Stop redundant troubleshooting

If the symptom matches a confirmed known issue, avoid making the customer repeat irrelevant diagnostics. Capture only what is needed to confirm scope, apply workaround, track affected customer, or escalate severity.

Known-issue handling should reduce effort.

### Keep affected-customer tracking

Record customer impact, workaround offered, whether it worked, affected account or environment, desired follow-up, and any severity signal. This helps product, engineering, and support measure impact and prioritize.

Do not let each known-issue ticket disappear as an isolated solved conversation.

### Close only when the customer's path is clear

A workaround may not resolve the underlying issue. Before closure, confirm whether the customer needs follow-up, bug tracking, refund review, accessibility accommodation, incident update, or escalation. If the workaround fails, state the next evidence or route.

Do not close as solved merely because a workaround was sent.

### Make rollback and supportability explicit and watch for workaround inequity

Some workarounds require configuration changes, switching browsers, disabling integrations, using older versions, exporting data, changing workflow, or using a manual process. State whether the workaround is supported, how to revert it, and when the customer should stop using it.

Do not leave customers on a fragile workaround that becomes invisible to future support.

A workaround may be practical for technical admins but not for low-vision users, mobile-only customers, frontline workers, regulated environments, or customers without admin rights. If the workaround excludes the customer, treat that as unresolved impact and route accordingly.

Do not mark a case solved with a workaround the customer cannot realistically use.

## Common Traps

- Calling something a known issue without checking approved sources.
- Giving a workaround that assumes admin access, technical skill, or low data risk.
- Hiding side effects such as data loss, duplicate work, security reduction, or billing impact.
- Recommending unsafe security or production changes.
- Promising a fix date because engineering is aware; continuing irrelevant troubleshooting after matching a confirmed known issue
- Failing to track affected customers and workaround success; treating workaround as permanent resolution
- Not explaining what to do if the workaround fails; sharing internal bug details or root-cause speculation beyond approved language
- Giving a workaround without rollback, supportability, or end condition; treating an inaccessible or admin-only workaround as adequate for all customers

## Self-Check

- Was the known-issue status checked against approved sources?
- If unconfirmed, does the response avoid calling it a known bug or outage?
- Is the workaround suited to platform, permissions, data sensitivity, accessibility, technical skill, workflow, and deadline?
- Are limits and side effects explained, including data, billing, security, accessibility, feature, performance, and reversion concerns?
- Are unsafe mitigations avoided unless approved?
- Are fix timeline, investigation status, release expectation, customer action, and follow-up stated accurately?
- Is redundant troubleshooting avoided for confirmed known issues?
- Are impact, environment, workaround offered, success, affected account, follow-up need, and severity signal tracked?
- Does closure distinguish workaround from underlying resolution?; if the workaround fails, is the next evidence or escalation route clear?
- Does the workaround include supportability, rollback, end condition, or reversion guidance where relevant?; was the workaround checked for accessibility, mobile-only use, regulated environments, admin rights, and realistic customer capability?
