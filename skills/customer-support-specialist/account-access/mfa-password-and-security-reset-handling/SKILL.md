---
name: mfa-password-and-security-reset-handling.md
description: Use when the agent is handling password reset, MFA reset, authenticator loss, backup code issues, security questions, email or phone change, session reset, device revocation, or requests to disable security controls while preserving account protection and customer access.
---

# MFA Password And Security Reset Handling

Security resets are among the most dangerous support actions because they change the controls that protect an account. A legitimate customer may be locked out, but an attacker may ask for the same reset. Agents often treat MFA and password issues as routine inconvenience. This skill helps the agent handle resets with enough friction, evidence, and documentation to protect rightful access.

## Core Rules

### Treat reset type as a security decision

Password reset, MFA reset, email change, phone change, backup code replacement, security question reset, trusted-device removal, session revocation, and account unlock carry different risk. Identify which control is being changed and what access it would grant.

Do not use one generic reset workflow for all security controls.

### Require stronger proof for stronger control changes

Disabling MFA, changing recovery email, replacing phone number, removing admin lockout, or bypassing device trust is higher risk than sending a normal password reset to a verified channel. Use the policy-required evidence and approval for the action requested.

The more a reset weakens future security, the stronger the verification should be.

### Prefer verified channels and self-service

When safe, direct the customer to password reset or MFA recovery through existing verified email, phone, device, admin, or recovery-code flows. Manual support intervention should be reserved for approved exceptions or blocked standard flows.

Do not bypass a verified self-service path because a manual response feels faster.

### Check for compromise before resetting

Before changing controls, look for suspicious recent activity: new devices, location changes, contact change, failed login spikes, password reset requests, payment changes, admin changes, disabled notifications, or customer reports of unauthorized access. If present, route to account takeover or security procedures.

Resetting access without containment may give the attacker another chance.

### Avoid revealing security configuration

Do not disclose whether a specific MFA method, backup code, phone number, device, or security flag is present until verification allows it. Use careful wording that explains the recovery path without confirming exploitable details.

Security transparency has limits in unauthenticated conversations.

### Protect linked accounts and organizations

Changing security on one user may affect team workspaces, billing, integrations, shared data, admin controls, or downstream systems. For organizations, check admin authority and whether another verified admin can restore access.

Do not reset a user into control of an organization they no longer represent.

### Communicate friction clearly

Customers locked out of accounts may be stressed. Explain that verification protects their account, data, billing, and organization. State the safe next step, expected review path, and what cannot be done through the current channel.

Do not apologize for refusing unsafe bypasses as if the control is unreasonable.

### Record resets and post-reset expectations

Document verification, evidence, reset type, approvals, security signals, customer notification, sessions revoked, devices removed, recovery methods updated, and any recommended next steps. After a reset, tell the customer to review account activity, update recovery methods, use strong credentials, and report unfamiliar activity where appropriate.

The reset is not complete if the account remains vulnerable.

### Use cooldowns and staged access where policy supports them

Some systems require waiting periods, limited access, admin confirmation, or delayed sensitive changes after high-risk resets. These controls reduce harm if recovery was requested by an attacker. Explain the customer-visible effect without disclosing bypass criteria.

Do not remove cooldowns or staged restrictions unless an approved emergency path exists and the risk has been reviewed.

## Common Traps

- Treating password, MFA, phone, email, and admin recovery as equivalent risk.
- Disabling MFA or changing recovery channels based only on urgency.
- Skipping self-service verified flows because manual handling is faster.
- Missing takeover signals before granting a reset.
- Confirming which MFA method or phone number is on the account before verification; resetting a former employee or non-admin into organizational control
- Explaining security friction in a way that reveals bypass criteria; failing to revoke sessions, review devices, or advise activity review after compromise signals
- Recording backup codes, secrets, or full identity documents in notes; leaving no audit trail for a high-risk security change
- Removing cooldowns, sensitive-action delays, or staged access because the customer wants immediate full control; forgetting that a reset can protect login while leaving billing, admin, API, or integration risk open

## Self-Check

- Is the exact reset type identified: password, MFA, email, phone, backup code, security question, device trust, session, or account unlock?
- Is verification strength appropriate to the control being changed and the access it grants?
- Were verified self-service flows used where safe before manual intervention?
- Were compromise signals checked, including new devices, locations, contact changes, failed logins, reset attempts, payment changes, admin changes, disabled notices, and unauthorized access reports?
- Does the response avoid revealing specific security configuration before verification?
- For organization accounts, were admin authority, team impact, integrations, billing, and former-employee risk checked?
- Is the customer given a clear secure path without unsafe bypass details?
- Are verification, evidence, reset type, approval, signals, notifications, session or device actions, and recovery-method updates documented?
- Are secrets, backup codes, full identity documents, and sensitive security data kept out of ordinary notes?; after the reset, are activity review, recovery updates, credential hygiene, and suspicious activity reporting addressed where relevant?
- Are cooldowns, staged access, sensitive-action delays, admin confirmation, or emergency bypass rules applied according to policy?; were downstream risks such as billing, admin rights, API keys, integrations, and active sessions considered after the reset?
