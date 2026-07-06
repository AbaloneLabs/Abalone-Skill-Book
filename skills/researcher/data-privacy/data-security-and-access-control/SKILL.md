---
name: data_security_and_access_control.md
description: Use when the agent is securing research data, designing storage and access controls, managing encryption and authentication, planning for data breaches, or deciding who within a team can access identifiable or sensitive data.
---

# Data Security And Access Control

Research data, especially identifiable or sensitive data, must be protected against unauthorized access, loss, and breach. Security is not a single tool but a layered system of storage, access control, encryption, and breach response. Researchers often default to convenient storage that is insecure, or grant broad access within a team without considering who truly needs identifiable data. The judgment problem is designing security proportional to the data's sensitivity and the harm that breach would cause, and enforcing least-privilege access honestly.

Use this skill when securing research data, designing access controls, managing encryption and authentication, and planning breach response. The goal is to keep the agent from treating security as an afterthought and to make the protections match the sensitivity and consequences of the data.

This is a high-stakes domain because breaches expose participants to identity theft, discrimination, and harm, and can violate legal obligations. When data are highly sensitive or security requirements are uncertain, the agent should consult the institutional data security office and the IRB rather than improvising protections.

## Core Rules

### Match Security To Sensitivity And Potential Harm

Security effort should scale with the data's sensitivity and the harm a breach would cause. Uniform low effort under-protects high-risk data; uniform high effort wastes resources on low-risk data.

Calibrate by:

- the identifiability of the data and whether direct or quasi-identifiers are present;
- the sensitivity of the content, such as health, genetic, legal, or sexual information;
- the population and the harm they would suffer from disclosure;
- legal and contractual obligations, such as data protection law or data use agreements;
- the number of people and systems that will touch the data.

High-sensitivity identifiable data demand strong encryption, strict access, and often secure enclaves. Low-sensitivity public data need far less. Match the protection to the risk.

### Enforce Least-Privilege Access

Not everyone on a project needs access to identifiable data. Broad internal access increases breach risk and often exceeds what the consent and approval authorized.

Apply least privilege by:

- separating identifiers from research data so analysts work on de-identified or coded data;
- restricting access to identifiable or linked data to those with a specific need;
- using role-based access so team members see only what their role requires;
- removing access promptly when members leave or change roles;
- documenting who has access to what, and why.

Access decisions should be justified by need, not convenience. A team member who does not need identifiers should not have them.

### Secure Data In Storage, In Transit, And At End Of Life

Data must be protected across their entire lifecycle, not only while actively used. Each stage has distinct risks.

Protect:

- data at rest through encryption of storage, devices, and databases;
- data in transit through encrypted channels and secure transfer, never unencrypted email or unsecured links;
- backups through encryption and access control, since backups are a common breach vector;
- data at end of life through secure deletion or destruction, since discarded devices retain recoverable data;
- physical media through secure handling and disposal.

A strong main store with unencrypted backups or un-wiped old devices is not secure. The protection must cover every copy.

### Use Strong Authentication And Access Logging

Access controls are only as strong as the authentication behind them and the logging that detects misuse.

Implement:

- strong, unique authentication, ideally multi-factor, for access to sensitive data;
- no shared accounts, so that actions are attributable to individuals;
- audit logging of who accessed what data and when;
- regular review of logs for anomalous access;
- secure management of credentials and keys.

Shared accounts and weak passwords defeat accountability. If a breach occurs, logs are what allow detection and response.

### Avoid Insecure Tools And Practices Of Convenience

Common convenience tools are frequent breach vectors. Their ease does not justify their risk for sensitive data.

Avoid:

- personal email or consumer cloud storage for identifiable data;
- spreadsheets emailed between collaborators;
- unencrypted laptops or USB drives carrying sensitive data;
- sharing login credentials or API keys;
- storing data on personal devices without encryption and approval.

The convenience cost of secure alternatives is small compared to the harm of a breach through a convenience tool.

### Plan For Breach Detection And Response

Breaches happen despite precautions. A response plan reduces harm and satisfies legal obligations.

Plan for:

- how a breach would be detected, given logging and monitoring;
- the internal chain of notification and decision-making;
- the external notifications required by law, regulation, or data use agreements, and their timelines;
- communication with affected participants, honestly and promptly;
- containment, such as revoking access and isolating affected systems;
- post-incident review to prevent recurrence.

Discovering a breach without a plan leads to delay, legal violation, and loss of participant trust. The plan should exist before it is needed.

### Coordinate Security With Consent, Approval, And Agreements

Security obligations are often specified in consent forms, ethics approvals, and data use agreements. The security plan must honor those terms.

Align:

- the consent language about data protection with what is actually implemented;
- the data protection plan submitted to the IRB with operational reality;
- data use agreement terms on storage location, access, and sharing;
- cross-border transfer restrictions on where data may be stored or processed;
- funder and institutional security requirements.

A security plan that contradicts the consent or the approval is a compliance failure, even if the protections are strong in absolute terms.

## Common Traps

### Defaulting To Convenient Insecure Storage

Personal cloud accounts, email, and unencrypted devices are easy and dangerous for sensitive data. Convenience does not justify the risk.

### Granting Broad Internal Access

Team members who do not need identifiable data should not have it. Broad access exceeds consent and increases breach risk.

### Protecting The Main Store But Not Backups Or Old Devices

Backups and decommissioned hardware are common breach vectors. Protection must cover every copy.

### Using Shared Accounts

Shared logins defeat accountability and make breach investigation impossible. Access must be individual.

### Skipping Audit Logging

Without logs, misuse cannot be detected or investigated. Logging is essential for sensitive data.

### Having No Breach Response Plan

Discovering a breach without a plan causes delay, legal violation, and eroded trust. The plan must precede the incident.

### Contradicting Consent And Approval Terms

Security that does not match what participants were told or what the IRB approved is a compliance failure regardless of absolute strength.

## Self-Check

- Is security calibrated to the data's sensitivity, identifiability, and the harm a breach would cause?
- Is least-privilege access enforced, with identifiers separated and access restricted to those with a specific need?
- Are data protected at rest, in transit, in backups, and at end of life, including secure deletion?
- Is strong, individual authentication with multi-factor and audit logging in place for sensitive data?
- Are insecure convenience tools such as personal email and consumer cloud avoided for identifiable data?
- Is a breach detection and response plan in place, covering internal and external notification timelines?
- Does the security plan align with consent language, IRB approval, data use agreements, and cross-border restrictions?
- Where data are highly sensitive or security requirements are uncertain, has the institutional data security office and the IRB been consulted rather than improvising protections?
