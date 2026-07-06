---
name: login-and-account-recovery-support.md
description: Use when the agent is helping a customer who cannot sign in, needs account recovery, lost access to email or phone, is locked out, suspects compromise, faces disabled or suspended access, or needs guidance that restores access without weakening security or privacy.
---

# Login And Account Recovery Support

Account recovery is a high-trust support workflow. The customer may be locked out of something important, but the requester may also be an attacker trying to gain access. Agents often focus on resolving the visible login problem and miss identity, ownership, security, privacy, and downstream harm. This skill helps the agent restore legitimate access without bypassing controls or exposing account information.

## Core Rules

### Treat access recovery as both help and risk control

The goal is not simply to get someone back in. The goal is to restore access to the rightful user through approved verification. Check whether the case is ordinary password trouble, lost device, expired session, account suspension, disabled email, organization admin issue, suspected takeover, or service incident.

Do not jump to manual overrides before understanding which path applies.

### Verify before revealing account facts

Even confirming that an email has an account, that MFA is enabled, that a phone number is on file, or that a login occurred can be sensitive. Use approved verification flows before disclosing account state or making changes.

If verification fails, keep the response helpful but non-revealing. Explain the secure path without confirming private details.

### Use the least risky recovery path

Prefer self-service and standard recovery flows when they are available and safe. Manual recovery should require stronger evidence, approval, and documentation because it bypasses normal protections. If the customer lost email or phone access, follow policy for alternate verification rather than improvising.

Do not remove controls merely because the customer is urgent or upset.

### Watch for account takeover signals

Signals include unfamiliar login alerts, changed email or phone, disabled MFA, suspicious payment activity, password reset emails not initiated by the customer, new devices, unusual location, recovery pressure, or requests to bypass security. Route according to account takeover procedures when present.

Do not treat suspected compromise as a routine login problem.

### Distinguish user access from account ownership

In business, family, school, or shared accounts, the requester may be a user but not the owner or admin. They may be an ex-employee, contractor, former partner, reseller, parent, child, estate representative, or third-party assistant. Check role, authorization, and organization policy before changing ownership or admin access.

Helping the wrong person can create data exposure and business harm.

### Explain secure friction

Recovery steps can feel frustrating. Explain that verification protects the customer's account and data. Keep instructions clear, but do not disclose enough detail to teach attackers how to pass checks.

Avoid apologizing for security controls as if they are unnecessary obstacles.

### Preserve service continuity where safe

If access recovery may take time, consider safe interim options: status update, billing hold review, admin contact route, documentation of lockout impact, or escalation for severe business interruption. These options should not grant unauthorized access.

Support can reduce harm without bypassing recovery rules.

### Document recovery actions carefully

Record verification status, recovery path used, account changes made, signals checked, approvals, denied requests, customer-facing expectations, and any security escalation. Avoid storing secrets or sensitive evidence in broad notes.

Future reviewers need to know why access was restored, denied, or escalated.

### Consider notification and containment

Access recovery can affect other legitimate users, admins, billing owners, or security contacts. When policy requires it, notify existing verified channels, revoke suspicious sessions, preserve logs, or add monitoring after recovery. If the customer reports compromise, consider whether recovery should happen only after containment steps are planned.

Do not restore access in a way that silently hands a compromised account back to an attacker-controlled environment.

## Common Traps

- Treating lockout as a simple convenience problem rather than an access-control decision.
- Confirming account existence or security settings before verification.
- Removing MFA, changing email, or resetting credentials based only on customer pressure.
- Ignoring takeover signals because the requester sounds legitimate.
- Confusing account user, billing contact, owner, admin, and authorized representative; helping an ex-employee, family member, assistant, or reseller without authorization
- Explaining recovery checks in enough detail to help an attacker; offering business-continuity help that effectively grants access without approval
- Recording secrets, full identity documents, or security signals in ordinary notes; closing the case after recovery without checking whether compromise cleanup or notification is needed
- Restoring access without notifying existing verified channels or reviewing active sessions where policy expects it; recovering the account into an email, phone, or device that may still be attacker controlled

## Self-Check

- Was the access problem classified as routine login, lost factor, suspended access, organization issue, lost email or phone, suspected takeover, or service incident?
- Were approved verification flows completed before revealing account facts or making changes?
- Is the chosen recovery path the least risky approved path available?
- Were takeover signals checked, including unfamiliar login, changed contact details, MFA changes, payment activity, reset emails, new devices, unusual location, and bypass pressure?
- Is the requester authorized as owner, admin, user, billing contact, representative, or organization member for the requested action?
- Does the response explain secure friction without revealing attack-useful details?
- Are interim options limited to safe continuity support that does not grant unauthorized access?
- Are verification status, recovery path, changes, approvals, denials, expectations, and escalations documented?
- Are secrets, unnecessary identity data, and sensitive security signals kept out of broad notes?; if compromise is possible, is cleanup, notification, or security escalation still open after access restoration?
- Were required notifications, session revocations, log preservation, or monitoring considered?; is the recovery destination itself trustworthy enough for restored access?
