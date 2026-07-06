---
name: security-report-intake-and-triage.md
description: Use when the agent is receiving or triaging a security report, vulnerability claim, suspected data exposure, account compromise report, abuse of security controls, phishing report, suspicious email, unauthorized access report, or customer concern that may require security, privacy, trust and safety, legal, or incident response ownership.
---

# Security Report Intake And Triage

Security reports often reach frontline support before the security team sees them. A customer may describe a real vulnerability, a phishing email, account compromise, data exposure, or suspicious behavior in ordinary support language. Agents can cause harm by dismissing the report, asking for unsafe proof, promising outcomes, or disclosing internal security details. This skill helps the agent preserve signal and route it safely.

## Core Rules

### Treat security claims as signals, not annoyances

Do not dismiss a report because it is vague, emotionally written, technically imprecise, or sent through the wrong channel. Identify what is being claimed: vulnerability, unauthorized access, phishing, malware, exposed data, account takeover, payment fraud, suspicious email, abuse, or unsafe configuration.

The agent does not need to prove the report true before routing it through the right path.

### Gather the minimum safe triage facts

Collect information that helps route the report: affected product, account or organization where allowed, time observed, error or message, URL, email headers if safe, screenshot with redaction, reproduction outline, suspected impact, whether data or money is involved, and whether the issue is ongoing.

Do not ask the customer to exploit a vulnerability further, access someone else's data, share secrets, or prove harm by causing more harm.

### Route to the correct restricted owner

Security reports may belong to security response, privacy, trust and safety, incident command, fraud, legal, engineering, abuse operations, or customer support escalation. Use the approved path and restricted fields. If the boundary is unclear, escalate to a lead or security intake rather than leaving it in normal support.

Ordinary ticket queues are rarely appropriate for sensitive security evidence.

### Avoid confirming internal security posture

Until authorized, do not confirm whether a vulnerability exists, whether other customers are affected, whether monitoring detected it, what controls are in place, what logs show, or what remediation is underway. Use careful language: the report has been received, it is being routed, or the appropriate team will review.

Over-disclosure can increase attack risk and legal exposure.

### Preserve evidence integrity

Record the original report, source channel, timestamps, attachments, redaction state, and handling path. Avoid editing evidence in ways that obscure what was received. Use restricted storage for sensitive data, credentials, tokens, private keys, personal data, or screenshots showing other users.

If the customer already sent secrets, follow secret-handling and rotation guidance rather than copying them into notes.

### Respect vulnerability disclosure and researcher context

Some reporters are security researchers, bug bounty participants, customers, employees, or third parties. They may need a defined intake path, acknowledgement, safe harbor language, or bounty-program rules. Do not promise bounty eligibility, legal protection, public credit, or remediation timeline unless authorized.

Do not accuse a good-faith reporter of malicious behavior without evidence.

### Recognize incident thresholds

If the report suggests active exploitation, data exposure, many affected users, payment compromise, safety risk, account takeover pattern, or public disclosure, escalate as a potential incident according to severity criteria. Early routing matters more than perfect classification.

Do not let a severe signal wait for normal business hours if policy requires urgent escalation.

### Communicate next steps safely

Thank the reporter, explain the safe intake path, state what information is useful if needed, warn against sharing secrets or accessing unauthorized data, and set only approved expectations. If the customer is personally affected, separate their support needs from the security investigation.

The response should preserve trust without promising findings or outcomes.

## Common Traps

- Treating a security report as a routine complaint or technical bug.
- Requiring the customer to prove a vulnerability by exploiting it further.
- Asking for passwords, tokens, private keys, full headers with secrets, or unredacted personal data through normal support.
- Routing sensitive security reports to ordinary queues or broad notes.
- Confirming whether a vulnerability, exposure, or monitoring signal exists before authorization.
- Losing original timestamps, source channel, attachments, or evidence state.
- Promising bounty eligibility, public credit, legal protection, or remediation timelines; accusing a reporter of wrongdoing because the report is technically uncomfortable
- Missing incident thresholds such as active exploitation, data exposure, payment compromise, or many affected users; mixing the customer's personal support request with the security investigation so neither gets owned

## Self-Check

- Is the report classified by claimed security signal: vulnerability, unauthorized access, phishing, malware, data exposure, account takeover, payment fraud, suspicious email, abuse, or unsafe configuration?
- Were only minimum safe triage facts requested?
- Did the response avoid asking the customer to exploit further, access unauthorized data, or share secrets?
- Was the report routed to security, privacy, trust and safety, incident, fraud, legal, engineering, or abuse owner as appropriate?
- Are sensitive details kept in restricted fields or storage?
- Does customer-facing language avoid confirming internal security posture, detection, scope, or remediation?
- Are original report, source, timestamps, attachments, redaction state, and handling path preserved?
- Are researcher, bug bounty, customer, employee, and third-party contexts handled through approved language and program rules?
- Were urgent incident thresholds checked for active exploitation, data exposure, payment compromise, safety, account takeover pattern, public disclosure, or many affected users?
- Are the reporter's personal support needs and the security investigation separately owned where needed?
