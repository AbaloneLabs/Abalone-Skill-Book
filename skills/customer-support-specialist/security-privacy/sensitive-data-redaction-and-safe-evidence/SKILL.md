---
name: sensitive-data-redaction-and-safe-evidence.md
description: Use when the agent is handling customer-provided sensitive data, screenshots, logs, documents, payment details, credentials, personal information, support attachments, internal evidence, or redaction before sharing, escalating, storing, quoting, or using evidence in a support workflow.
---

# Sensitive Data Redaction And Safe Evidence

Support evidence often contains more data than the case needs. Screenshots reveal addresses, tokens, account IDs, other users, private messages, health or financial details, and internal tools. Agents may copy, quote, forward, or store this material casually because it helps solve the ticket. This skill helps the agent use evidence while reducing privacy and security exposure.

## Core Rules

### Identify sensitive data before moving it

Before quoting, attaching, forwarding, or escalating evidence, scan for passwords, tokens, private keys, recovery codes, full card numbers, bank details, government IDs, health data, precise location, minors' data, private messages, third-party names, employee details, internal notes, and security signals.

Do not assume a screenshot is safe because the customer sent it voluntarily.

### Redact to the decision need

Redaction should preserve the information needed for the decision while removing unrelated sensitive data. Keep last four digits if needed instead of full numbers, relevant timestamp instead of full statement, specific error instead of full screen, or account ID in restricted field instead of broad note.

Over-redaction can make evidence unusable; under-redaction spreads unnecessary risk.

### Use the right channel for the data

Some evidence belongs in restricted attachments, secure upload, encrypted storage, privacy tools, security intake, legal system, or a protected field. Do not paste sensitive content into public comments, broad internal notes, chat channels, or vendor messages.

If the required secure channel is unavailable, escalate rather than improvising a risky workaround.

### Do not quote secrets back

If a customer sends a password, token, recovery code, private key, or full payment number, do not repeat it in the response or notes. Follow policy for redaction, deletion, restriction, rotation guidance, or security escalation.

Quoting secrets multiplies the exposure and may make logs harder to clean.

### Preserve evidence integrity when needed

For disputes, fraud, abuse, security, safety, warranty, chargeback, legal, and incident review, the original evidence may need to be preserved with chain of handling. Store originals only in approved restricted locations and use redacted copies for broader discussion.

Do not edit the only copy if later review depends on authenticity.

### Protect third parties and internal data

Evidence may include other customers, coworkers, family members, children, employees, vendor contacts, or internal tool output. Redact irrelevant bystander information and avoid forwarding internal data to customers or vendors unless approved.

The customer's consent to share a screenshot does not grant permission to expose everyone visible in it.

### Give customers safe submission instructions

When asking for evidence, tell customers what to include, what to omit, what to redact, and which secure channel to use. Provide examples of safe redaction where helpful: hide full card numbers, addresses, unrelated messages, tokens, and personal identifiers.

Good instructions prevent accidental exposure before support receives the evidence.

### Document handling decisions

Record that sensitive data was received, how it was restricted or redacted, what evidence was used, where the safe copy is stored, and what remains unavailable due to privacy boundaries. Avoid recording the sensitive value itself.

Future agents need enough context without reopening the exposure.

### Reassess sharing at each handoff

Evidence that was appropriate for frontline support may not be appropriate for a vendor, community moderator, engineering channel, sales owner, or public status update. Before each handoff, recheck audience, purpose, minimum necessary detail, and whether a redacted summary can replace the raw material.

Do not assume that because evidence is inside the company it is safe for every internal audience.

## Common Traps

- Copying sensitive screenshot content into ordinary ticket notes.
- Treating customer-sent secrets as safe to repeat because the customer provided them.
- Redacting so much that the receiving team cannot make the decision.
- Redacting too little before sharing with broad teams or vendors.
- Using public comments, chat, or email threads for restricted evidence; editing the only evidence copy when authenticity matters
- Forwarding internal logs or security signals to a customer to prove support investigated; ignoring bystander data in screenshots or documents
- Asking for evidence without redaction or secure-upload instructions; documenting the sensitive value instead of documenting how it was handled
- Forwarding raw evidence to vendors or broad internal channels when a redacted summary would answer the question; assuming internal sharing has no privacy boundary once the customer provided the evidence

## Self-Check

- Was evidence scanned for passwords, tokens, keys, recovery codes, full card numbers, bank details, IDs, health data, location, minors, private messages, third parties, employee details, internal notes, and security signals?
- Is redaction matched to the decision need rather than arbitrary hiding or over-sharing?
- Is sensitive evidence placed in the correct restricted, secure, encrypted, legal, privacy, or security channel?
- Are secrets and full sensitive values avoided in replies and broad notes?
- If secrets were sent, were redaction, deletion, restriction, rotation guidance, or security escalation handled according to policy?
- Is original evidence preserved where fraud, abuse, safety, warranty, chargeback, legal, security, or incident review requires integrity?
- Are redacted copies used for broader discussion where originals must remain restricted?
- Are third-party, bystander, employee, vendor, and internal tool details protected?
- Were customers instructed how to submit evidence safely before sending it?; does the record explain handling decisions without copying the sensitive data itself?
- Was each handoff checked for audience, purpose, minimum necessary detail, and whether raw evidence is truly needed?; are vendors, broad internal groups, community moderators, sales, and public communications given only approved and minimized evidence?
