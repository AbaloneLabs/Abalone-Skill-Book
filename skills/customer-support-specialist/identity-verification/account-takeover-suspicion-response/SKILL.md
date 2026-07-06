---
name: account-takeover-suspicion-response.md
description: Use when the agent is responding to signs of account takeover, unauthorized login, changed credentials, suspicious MFA reset, unknown charges, altered email or phone, session hijack, unauthorized admin action, social engineering, or customer reports that someone else may control the account.
---

# Account Takeover Suspicion Response

Account takeover suspicion changes the support problem from access inconvenience to security containment. The person contacting support may be the rightful customer, the attacker, or both at different moments if channels are compromised. Agents often rush to restore access without preserving evidence or stopping ongoing harm. This skill helps the agent identify takeover signals, route securely, and communicate without revealing defensive details.

## Core Rules

### Treat suspicion as provisional but serious

Account takeover may be reported directly or implied through changed email, unknown MFA, unfamiliar login, password reset emails, missing funds, unauthorized orders, admin changes, API key use, unexpected sessions, or customer statements that "someone else is in my account." Do not require perfect proof before using a safer path.

At the same time, do not label the account compromised as fact until evidence supports it.

### Preserve containment before convenience

The first priority is preventing further harm. Follow approved procedures for locking risky actions, revoking sessions, disabling suspicious tokens, freezing payments or orders, preserving logs, or escalating to security. The exact actions depend on policy and tooling.

Do not restore access or change recovery details in a way that leaves an attacker active.

### Verify through a trustworthy channel

If email, phone, device, or session may be compromised, ordinary verification may no longer be trustworthy. Use the designated secure recovery or identity review path. Be careful when sending codes or links to a channel the attacker may control.

Do not assume the channel used yesterday is safe today.

### Avoid revealing security signals

Do not tell an unverified requester which IPs, devices, risk flags, detection rules, MFA methods, or internal security actions are visible. Provide safe customer-facing information: the account requires security review, verification is needed, or suspicious activity is being investigated.

Attackers can use detailed security feedback to adapt.

### Collect customer impact and evidence safely

Ask for necessary, safe evidence: approximate time, unexpected email, unauthorized charge reference, order number, changed setting, device type, or screenshot with sensitive data redacted. Avoid asking for passwords, full card numbers, tokens, private keys, or secret recovery codes.

Record impact such as financial loss, business interruption, data exposure concern, safety issue, or lost admin control.

### Coordinate related workflows

Takeover may touch billing, refunds, chargebacks, fulfillment, data privacy, admin ownership, legal, trust and safety, and customer success. Route each affected issue to the right owner while keeping security containment central.

Do not resolve a refund or shipment issue while ignoring that the account may still be compromised.

### Communicate uncertainty and next steps

Tell the customer what can be safely said: the case is being reviewed through security procedures, what they should do next, what information is needed, what not to share, and how updates will arrive. Avoid promising account restoration, refund, or timeline before review.

Security review can be stressful; clarity matters, but over-disclosure is risky.

### Document security handling carefully

Record suspicion basis, customer claims, system facts, evidence links, containment actions, verification path, escalations, customer instructions, and unresolved risks in the correct restricted locations. Keep sensitive security details out of broad notes.

The record must support security review without spreading attacker-useful information.

## Common Traps

- Treating takeover signs as ordinary login trouble.
- Demanding conclusive proof before using a safer escalation path.
- Restoring access through a channel that may be compromised.
- Revealing IPs, device details, risk flags, MFA state, or internal detection logic to an unverified requester.
- Resetting credentials without revoking sessions, tokens, or suspicious devices where policy requires it.
- Asking the customer to send passwords, full card numbers, recovery codes, tokens, or private keys.
- Handling refunds or orders while ignoring ongoing account control risk; promising restoration, compensation, or timing before security review
- Storing security signals in broadly visible ticket notes; closing the case when login works but account integrity, charges, data exposure, or admin control remain unresolved

## Self-Check

- Were takeover signals checked, including changed contact details, unfamiliar login, unknown MFA, reset emails, charges, orders, sessions, admin changes, API activity, and customer reports?
- Is the case treated as serious and provisional rather than ignored or over-confirmed?
- Were containment actions considered before convenience, according to policy and tooling?
- Is verification routed through a trustworthy channel when usual email, phone, device, or session may be compromised?
- Does customer-facing language avoid revealing security signals, internal detection, devices, IPs, MFA state, or risk flags?
- Is requested evidence necessary, safe, redacted where appropriate, and free of passwords, full card data, keys, tokens, and recovery codes?
- Are related billing, fulfillment, privacy, admin, legal, trust and safety, and customer success workflows coordinated without losing security ownership?
- Does the customer know the safe next step, what not to share, and how updates will happen?
- Are suspicion basis, facts, claims, evidence, containment, verification, escalation, instructions, and unresolved risks documented in appropriate restricted locations?
- Before closure, are account integrity, access, charges, data exposure, admin control, and residual security risks addressed or routed?
