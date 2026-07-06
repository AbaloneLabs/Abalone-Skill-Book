---
name: evidence-collection-with-privacy-boundaries.md
description: Use when the agent is requesting, receiving, storing, linking, redacting, or reviewing support evidence such as screenshots, logs, IDs, invoices, order details, account data, security reports, payment information, or customer documents while protecting privacy, security, minimization, and retention boundaries.
---

# Evidence Collection With Privacy Boundaries

Support evidence helps resolve disputes, reproduce bugs, verify identity, review refunds, investigate abuse, and escalate incidents. It can also create privacy, security, compliance, and customer-trust risk when agents ask for too much, use unsafe channels, or retain sensitive material unnecessarily. This skill helps the agent collect enough evidence for the decision without turning the ticket into a data exposure.

## Core Rules

### Define the evidence purpose first

Before requesting evidence, identify the decision it must support: identity verification, refund eligibility, charge matching, bug reproduction, warranty proof, delivery dispute, abuse report, security review, accessibility barrier, or legal/compliance routing. Evidence should be tied to a decision and should not be collected "just in case."

If the purpose is unclear, clarify the decision before asking the customer for documents or logs.

### Use the least sensitive sufficient evidence

Choose evidence that proves the point with minimal exposure. A transaction ID may be safer than a full bank statement. A redacted screenshot may be enough instead of a full screen recording. A timestamp and error code may be enough instead of full logs.

When possible, ask the customer to redact unrelated names, addresses, account numbers, messages, or documents before uploading.

### Use approved channels and storage

Sensitive evidence may require secure upload, identity verification flow, restricted fields, encrypted storage, or special handling. Do not request passwords, full payment card numbers, private keys, secrets, recovery codes, full access tokens, or unnecessary government IDs through ordinary replies.

If a customer sends sensitive data unprompted, follow handling rules: do not quote it back, restrict or redact where possible, and document the action taken.

### Distinguish evidence from explanation

Customers may provide a story, screenshot, receipt, log, or document. The agent should record what the evidence shows and what remains unproven. A screenshot of an error proves the error appeared at a moment; it may not prove cause. A receipt proves purchase detail; it may not prove account ownership.

Do not overstate what evidence establishes.

### Protect third parties and bystanders

Evidence may include other customers, family members, employees, minors, patients, coworkers, addresses, messages, or payment details. Ask for redaction where possible and avoid copying bystander information into notes.

Privacy boundaries apply even when the customer willingly sends the material.

### Preserve integrity for disputes and escalations

For chargebacks, fraud, abuse, safety, warranty, legal, or incident cases, evidence integrity matters. Keep source links, timestamps, original channel, redaction status, and chain of handling where required. Do not edit evidence in a way that makes later review unreliable.

If evidence is incomplete or altered, say so rather than treating it as conclusive.

### Balance retention and deletion expectations

Some evidence must be retained for audit, legal, fraud, warranty, chargeback, or incident review. Other evidence should be deleted, redacted, or not collected once the decision is made. Follow retention policy and avoid keeping sensitive material because it might be useful someday.

If the customer requests deletion or data access, route to the proper privacy process rather than improvising.

### Explain evidence requests respectfully

Customers may feel mistrusted when asked for proof. Explain why evidence is needed and how to provide it safely. Keep the request specific and avoid accusatory language.

The goal is to support a fair decision, not to make the customer feel interrogated.

### Reuse verified internal evidence before asking again

Support systems may already contain sufficient evidence: payment records, delivery scans, account events, identity verification status, entitlement flags, device logs, prior screenshots, or previous approvals. Use reliable internal evidence before asking the customer to gather new material.

If internal evidence conflicts with customer-provided evidence, preserve both and describe the conflict rather than forcing the customer to prove the same point repeatedly.

## Common Traps

- Asking for broad documents or logs before defining the decision.
- Requesting more sensitive evidence than the case requires.
- Asking for passwords, full payment numbers, keys, tokens, recovery codes, or unnecessary IDs.
- Accepting sensitive data in unsafe channels and then quoting it into notes.
- Treating a screenshot, receipt, or log as proof of more than it actually shows; ignoring third-party or bystander data included in evidence
- Redacting or editing evidence without preserving integrity where review requires it; keeping sensitive evidence after it is no longer needed or allowed
- Handling deletion or data access requests casually instead of routing to privacy process; framing evidence requests as suspicion rather than fair verification
- Asking the customer for proof already available in reliable internal systems; discarding customer-provided evidence because it conflicts with internal records instead of documenting the conflict

## Self-Check

- Is every evidence request tied to a specific decision such as verification, refund, charge, bug, warranty, delivery, abuse, security, accessibility, or compliance?
- Is the requested evidence the least sensitive sufficient proof?
- Were redaction instructions provided for unrelated names, addresses, account numbers, messages, or documents?
- Are approved secure channels, restricted fields, or verification flows used for sensitive evidence?
- Are passwords, full payment numbers, private keys, secrets, recovery codes, full tokens, and unnecessary government IDs avoided?
- If sensitive data was sent unprompted, was it handled without quoting back and restricted or redacted where possible?
- Does the record state what the evidence shows and what remains unproven?
- Are third-party and bystander privacy risks minimized?
- Is evidence integrity preserved where disputes, fraud, abuse, safety, warranty, legal, or incidents require it?; are retention, redaction, deletion, and data-request obligations handled through the correct process?
- Was reliable internal evidence checked before asking the customer to collect more proof?; if internal and customer-provided evidence conflict, is the conflict preserved and routed rather than flattened?
