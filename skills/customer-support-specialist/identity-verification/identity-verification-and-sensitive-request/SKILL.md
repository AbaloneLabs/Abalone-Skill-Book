---
name: identity-verification-and-sensitive-request.md
description: Use when the agent is verifying a customer's identity before handling sensitive support requests, account details, billing changes, access changes, data disclosure, deletion, security review, refunds, ownership transfer, or any action where the requester must prove they are allowed to receive information or cause account changes.
---

# Identity Verification And Sensitive Request

Identity verification is not a formality before "real" support begins. It determines whether support may disclose information, change account state, issue remedies, or continue a sensitive workflow. Agents often verify too little when the customer is urgent, or ask for too much sensitive data when a lighter check would do. This skill helps the agent match verification strength to risk while minimizing data exposure.

## Core Rules

### Classify the sensitivity of the request

First identify what the requester wants: general help, account status, billing detail, refund, password or MFA change, data export, deletion, ownership transfer, admin change, security investigation, legal request, or sensitive support history. The higher the impact of the action, the stronger the verification and authority check should be.

Do not use the same verification threshold for a how-to question and a security reset.

### Verify identity separately from authorization

Identity proves who the requester is. Authorization proves whether that person can take the requested action. A verified user may not be the account owner, admin, billing contact, legal representative, or data subject for the requested workflow.

Always connect the verification result to the action being requested.

### Use approved verification evidence

Follow the product's approved methods: signed-in session, secure link, one-time code to verified channel, admin confirmation, account portal, billing portal, domain ownership, government ID flow, notarized or legal document path, or other defined process. Use the least invasive method that satisfies the risk.

Do not invent ad hoc verification questions from private account data if policy does not allow them.

### Minimize sensitive data collection

Ask only for evidence needed for the decision. Avoid collecting full payment numbers, passwords, private keys, recovery codes, full identity documents, health data, or unnecessary personal information through ordinary support channels. If a regulated or high-risk workflow requires documents, use the approved secure path.

Verification should not create a larger privacy problem than the original request.

### Avoid account enumeration and private-detail leaks

When the requester is not verified, avoid confirming whether an account exists, what email is on file, whether MFA is enabled, what plan applies, whether a charge occurred, or what data is stored. Give the safe path without revealing private facts.

Small confirmations can help attackers build a profile.

### Treat failed or inconsistent verification as signal

Failed verification may be simple confusion, old contact information, shared account history, or disability/access barrier. It may also be social engineering, account takeover, fraud, or unauthorized proxy access. Ask only allowed follow-up questions and route suspicious patterns appropriately.

Do not shame the customer for failed verification, but do not proceed as if it passed.

### Preserve accessibility and legitimate hardship paths

Some legitimate customers cannot complete standard verification because of lost phone, changed email, disability, death, travel, device loss, domestic safety risk, incarceration, or organization admin turnover. Use approved alternate verification and escalation paths where available.

Do not improvise a shortcut; also do not abandon legitimate users when a defined hardship path exists.

### Document verification outcome and limits

Record the method used, status, action allowed or denied, authority limitation, information disclosed, escalation, and next path. Avoid recording verification secrets or unnecessary documents in normal notes.

The record should prevent later agents from accidentally treating a partial verification as full approval.

### Treat verification as time-bound and context-bound

A verification result may be valid only for the current session, channel, action, or risk level. A customer verified for a billing question may still need stronger review for an email change, data deletion, or MFA reset. If the conversation moves to another channel or risk signals appear, re-verify as policy requires.

Do not let an old or low-risk verification unlock a later high-risk request.

## Common Traps

- Treating verification as a checkbox unrelated to the requested action.
- Confusing identity with authorization.
- Using the same verification level for low-risk and high-risk requests.
- Asking for excessive personal documents when a secure session or verified channel would suffice.
- Requesting passwords, full payment numbers, recovery codes, keys, or unnecessary IDs; confirming account existence or private details before verification
- Treating failed verification as either proof of fraud or reason to bypass controls; ignoring hardship or accessibility routes for legitimate customers
- Storing verification secrets or full documents in broad notes; documenting "verified" without saying what method was used and what action it permits
- Reusing a past verification for a different channel, action, or higher-risk request; treating partial verification as permission for every sensitive workflow

## Self-Check

- Is the request sensitivity classified by the action and potential harm?
- Is the verification level appropriate for account detail, billing, refund, access change, data disclosure, deletion, transfer, admin change, security, or legal workflow?
- Are identity and authorization evaluated separately?
- Are approved verification methods used instead of ad hoc questions?
- Is the least invasive sufficient evidence requested?
- Are passwords, full payment numbers, private keys, recovery codes, full documents, health data, and unnecessary personal information avoided?
- Does the response avoid account enumeration and private-detail leaks before verification?
- Are failed or inconsistent verification attempts handled as possible confusion, hardship, or risk signal without unsafe continuation?
- Are accessibility, hardship, lost-device, death, travel, safety, incarceration, and admin-turnover paths considered where relevant?; are method, status, allowed action, limits, disclosure, escalation, and next path documented without storing secrets?
- Is the verification still valid for this channel, session, action, and risk level?; if the request changed from low-risk to high-risk, was verification strengthened rather than reused casually?
