---
name: diagnostic-log-and-file-handling.md
description: Use when the agent is requesting, receiving, reviewing, redacting, attaching, sharing, or escalating diagnostic logs, crash reports, screenshots, HAR files, exports, configuration files, sample data, recordings, or customer-provided files where risks include sensitive data exposure, malware or unsafe file handling, excessive collection, weak evidence quality, retention errors, or sending private artifacts to the wrong team or vendor.
---

# Diagnostic Log And File Handling

Diagnostic files can turn vague symptoms into actionable evidence, but they often contain secrets, personal data, account identifiers, third-party customer data, tokens, URLs, device details, payment fragments, internal configuration, or regulated information. Agents often ask for "logs" without defining which file, why it is needed, how to sanitize it, where to upload it, and who may view it. This skill helps the agent collect useful diagnostic artifacts while controlling privacy, security, and evidence quality.

## Core Rules

### Request the minimum artifact that can answer the question

Before asking for logs or files, identify what the artifact should prove: error code, timestamp, failed request, browser behavior, integration payload, crash sequence, permission state, configuration mismatch, or data import problem. Ask for the narrowest artifact and time window that can answer the question.

Do not request full exports, entire databases, broad log bundles, or production data when a screenshot, single error trace, sanitized sample, or timestamped request ID is enough. Overcollection increases risk and slows escalation.

### Give precise collection instructions

Tell the customer exactly which file, screen, export option, date range, browser console output, HAR capture, crash report, or diagnostic package is needed. Include safe stopping points and what not to include. If the customer is nontechnical, break the request into clear steps.

Vague requests produce poor evidence and repeated contacts. A good request reduces back-and-forth while avoiding unnecessary exposure.

### Warn about sensitive contents before upload

Many diagnostic artifacts contain session cookies, authorization headers, API keys, tokens, email addresses, user IDs, payment references, IP addresses, names, addresses, messages, or business data. Tell customers to redact or avoid including sensitive data when possible. For artifacts that cannot be safely redacted by the customer, use approved secure upload and access control.

Never ask the customer to paste secrets into chat. If a token or password is exposed, route according to security policy and advise rotation where appropriate.

### Use approved transfer and storage paths

Files should be uploaded through approved support portals, secure attachment tools, encrypted channels, or designated evidence systems. Avoid personal email, public links, unapproved file shares, broad internal chat, or local downloads unless policy explicitly allows them.

Access should be limited to people who need the artifact for support, engineering, security, legal, or vendor escalation. Do not forward files casually because "engineering might need it."

### Inspect files safely

Customer-provided files may be malicious, corrupted, too large, or unsafe to open directly. Follow company guidance for scanning, sandboxing, previewing, and handling executable or macro-enabled files. Be cautious with archives and files that require installing customer-specific tooling.

If the support role is not permitted to open a file type, document receipt and route it through the correct analysis path.

### Preserve evidence integrity

Record the artifact type, collection time, affected user or system, relevant timestamps, reproduction context, product version, environment, and customer-described behavior. If redaction was performed, note that it may remove some diagnostic detail.

Do not edit a customer's diagnostic file and then present it as the original. If a sanitized copy is created, keep clear labeling so reviewers understand what changed.

### Share only the relevant excerpt when escalating

Escalations often need the error line, request ID, stack trace, screenshot crop, sanitized sample, or timestamp, not the full customer file. Extract and summarize the relevant evidence when policy permits. If the full file is necessary, state why and restrict access.

The agent should make it easy for the next team to diagnose without exposing more customer data than needed.

### Close the loop on retention and deletion

Know whether attachments are retained in tickets, copied to engineering tools, or shared with vendors. If policy requires deletion after analysis or customer request handling, follow it. Tell the customer only what the organization can honor.

Artifacts can outlive the case and become a future privacy problem if not controlled.

## Common Traps

- Asking for "all logs" without defining purpose, time window, or sensitivity.
- Having customers paste API keys, cookies, tokens, passwords, or personal data into chat.
- Accepting a screenshot or HAR file without warning about sensitive headers or account data.
- Downloading customer files locally or sharing them in broad channels.
- Opening unknown executable, macro, archive, or script files without safe-handling process.
- Escalating full artifacts when a redacted excerpt would be enough.
- Losing diagnostic value by failing to capture timestamps, environment, version, or reproduction context; modifying a file without labeling the copy as sanitized or transformed
- Forgetting that ticket attachments may be retained and searchable; sending customer artifacts to a vendor without authorization, data-processing approval, or customer permission when required

## Self-Check

- Is the requested artifact tied to a specific diagnostic question?
- Is the requested scope limited by file type, date range, user, environment, or event?
- Are collection instructions precise enough to reduce repeated requests?
- Has the customer been warned not to include secrets, credentials, unnecessary personal data, or unrelated records?
- Is the upload or transfer path approved and access-controlled?
- Are unsafe file types handled according to security policy?
- Are timestamps, versions, environment details, and reproduction context captured with the artifact?
- Are original and sanitized artifacts clearly distinguished?
- Is the escalation limited to relevant excerpts unless the full file is necessary?
- Are retention, deletion, vendor-sharing, and internal access rules followed?
