---
name: ticket-note-quality-and-audit-trail.md
description: Use when the agent is writing internal support notes, preserving an audit trail, recording decisions, documenting policy checks, summarizing actions taken, preventing repeated work, or reviewing whether a ticket record is accurate, useful, privacy-safe, and defensible.
---

# Ticket Note Quality And Audit Trail

Ticket notes are not private scratch space. They are the operational memory of the case and may later be used by another agent, quality reviewer, escalation owner, dispute team, legal reviewer, or incident responder. Agents often write notes that are too vague, too emotional, too incomplete, or too revealing. This skill helps the agent document support work in a way that is useful, factual, and safe.

## Core Rules

### Write for the next qualified reader

Assume the next reader needs to understand the case quickly without rereading every message. Include the customer issue, relevant context, actions taken, evidence checked, decision made, pending work, owner, and customer expectation. Do not write only for yourself.

The note should make the next correct action easier, not merely prove that something happened.

### Separate facts from interpretation

Distinguish system facts, customer statements, agent observations, policy conclusions, and assumptions. For example, "customer reports duplicate charge" is different from "duplicate charge confirmed." "Suspected account takeover" is different from "account takeover confirmed."

This separation protects against wrong decisions, unfair customer treatment, and misleading escalations.

### Record decision basis

When support denies a refund, approves credit, changes priority, routes to security, refuses an exception, restores access, escalates to engineering, or closes a case, note why. Include policy, evidence, authority, approval, and any uncertainty.

Future agents should not have to guess whether a decision was arbitrary, provisional, or final.

### Preserve customer-facing commitments

Record promised updates, callbacks, compensation review, escalation, replacement timing, manual actions, customer actions, and any limits communicated. If the message explicitly avoided a guarantee, record that too.

Commitments are operational obligations. Missing them damages trust more than many original issues.

### Keep notes professional and objective

Do not use insults, sarcasm, speculation about motives, diagnoses, or emotionally loaded labels. If customer behavior matters, describe observable behavior and policy relevance: repeated messages, abusive language, threat, refusal to verify, or sensitive data sent.

Assume internal notes could be audited, exported, summarized, or read by a stakeholder outside the immediate team.

### Minimize sensitive data

Avoid copying passwords, full payment data, private keys, secrets, full identity documents, health details, unnecessary personal information, internal fraud signals, or security methods into notes. Use approved references, redaction, restricted fields, or secure attachments where required.

Do not quote sensitive customer-provided data back into ordinary notes just because it appears in the thread.

### Document handoffs and ownership changes

When a case moves between teams or agents, record the reason, receiving owner, requested action, evidence included, customer expectation, and follow-up checkpoint. If a handoff is rejected or rerouted, note why.

Handoffs without audit trail create loops and repeated customer effort.

### Update stale notes when facts change

If a note says the customer is not eligible, the issue is isolated, identity is unverified, or a refund is pending, later evidence may change that status. Add a clear update rather than leaving contradictory notes scattered in the record.

Do not silently rely on old notes that now mislead the case.

### Control audience and visibility

Different notes may be visible to different audiences: frontline support, restricted security teams, billing reviewers, customer success, legal, vendors, or sometimes the customer through data export. Put information in the field or system designed for that visibility. Do not place sensitive investigation detail in a broadly visible note because it is convenient.

If the audience is unclear, write the note as if it may be reviewed outside the immediate team and move restricted details to the approved secure location.

## Common Traps

- Writing notes that only say "handled," "updated customer," or "escalated" with no substance.
- Turning customer claims into confirmed facts.
- Omitting the policy, evidence, approval, or authority behind a decision.
- Forgetting to record customer-facing promises and update commitments.
- Using disrespectful internal labels for difficult customers; copying sensitive data into notes when a restricted reference would be safer
- Recording internal fraud or security details too broadly; handoff notes that do not say what the receiving owner is supposed to do
- Leaving stale or contradictory notes uncorrected; treating notes as lower quality because the customer cannot see them
- Putting restricted security, fraud, legal, or employee information into a broadly visible note field; assuming internal notes will never be exported, audited, subpoenaed, or summarized

## Self-Check

- Could the next qualified reader understand the issue, context, actions, evidence, decision, pending work, owner, and customer expectation quickly?
- Are system facts, customer statements, agent observations, policy conclusions, and assumptions separated?
- Are decisions documented with policy basis, evidence, authority, approval, and uncertainty where relevant?
- Are promised updates, callbacks, compensation reviews, escalations, replacements, customer actions, and non-guarantees recorded?
- Is language professional, objective, and limited to observable behavior when behavior matters?
- Are passwords, full payment data, keys, secrets, unnecessary identity documents, health data, private information, fraud signals, and security methods kept out of ordinary notes?
- Are restricted fields, redaction, secure attachments, or references used where sensitive evidence is required?
- Do handoff notes identify reason, receiving owner, requested action, evidence, expectation, and follow-up checkpoint?
- Are stale notes corrected or superseded when facts change?; is each note placed in a field with the right audience and visibility for its content?
- Would restricted information be protected if the ticket were audited, exported, or reviewed by another team?; would the note be defensible in quality review, escalation review, dispute handling, or legal/compliance review?
