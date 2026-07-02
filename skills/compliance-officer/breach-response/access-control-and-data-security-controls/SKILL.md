---
name: access_control_and_data_security_controls.md
description: Use when the agent is designing or reviewing access controls for personal data, applying least privilege and role-based access, selecting encryption at rest and in transit, configuring pseudonymization, logging, monitoring, or managing insider risk to personal data.
---

# Access Control And Data Security Controls

Security controls are the operational expression of the privacy promise. GDPR Article 32 and analogous provisions require measures appropriate to the risk, but "appropriate" is judged after a breach, when the gap between what was in place and what was needed becomes clear. The recurring failures are not exotic zero-days; they are over-permissive access, unencrypted data stores, missing monitoring, and insiders whose excessive access turned into harm. A privacy program that does not engage with access control and security architecture is incomplete.

Use this skill before granting access to a new system, designing controls for a new data store, reviewing access for a role change or departure, or hardening a system that processes sensitive data. The goal is to make the agent choose controls based on data sensitivity and risk, and to verify they operate, not merely exist.

## Core Rules

### Apply Least Privilege And Need-To-Know

Access to personal data should be the minimum necessary for a person or system to perform their function. Over-broad access is the single most common contributor to insider harm and breach severity.

Enforce least privilege by:

- granting access by role and by data category, not by blanket group membership;
- scoping access to the specific records and fields needed;
- removing access promptly on role change or departure;
- requiring approval for elevation and recording the basis;
- reviewing access periodically, especially for sensitive and special category data.

If everyone in engineering can read production personal data, least privilege has failed regardless of the policy text.

### Implement Role-Based Access Control With Segregation

Role-based access control (RBAC) structures access around job functions. Effective RBAC requires that roles are well-defined and that conflicting duties are segregated.

Design RBAC by:

- defining roles with the minimum data and action permissions needed;
- segregating duties so that the person who configures access is not the only person who can audit it;
- separating production from non-production data, with personal data scrubbed or pseudonymized in non-production;
- enforcing break-glass procedures for emergency access that are logged and reviewed;
- avoiding shared or generic accounts that obscure individual accountability.

### Encrypt At Rest And In Transit

Encryption is a baseline Article 32 measure and a mitigating factor in breach risk assessment. The question is not whether to encrypt but how and with what key management.

Apply encryption by:

- encrypting personal data at rest in databases, file stores, backups, and portable media;
- enforcing encryption in transit using current protocols and cipher suites;
- managing keys separately from the data they protect, with rotation and access controls;
- considering application-level or field-level encryption for the most sensitive data;
- documenting the encryption scope so that breach risk assessments can rely on it.

Encryption that exists in policy but not in a specific data store is not a control for that store.

### Use Pseudonymization To Reduce Exposure

Pseudonymization replaces direct identifiers with tokens, reducing the value of exposed data and limiting who can re-identify. It is explicitly encouraged by the GDPR.

Apply pseudonymization by:

- separating identifiers from attribute data in analytics and ML pipelines;
- restricting re-identification keys to a small, controlled set of roles;
- using pseudonymized data for development, testing, and analytics where full identifiers are not needed;
- documenting which datasets are pseudonymized and how re-identification is controlled.

Remember that pseudonymized data is still personal data. It reduces risk but does not remove the dataset from scope.

### Log, Monitor, And Alert On Access

Without logging and monitoring, misuse is invisible. The duty is not only to prevent unauthorized access but to detect and respond to it.

Implement monitoring by:

- logging access to personal data, especially bulk access, exports, and privileged actions;
- alerting on anomalous patterns such as off-hours bulk queries or access outside a user's role;
- retaining logs long enough to support investigation while respecting log data minimization;
- protecting logs from tampering and from becoming a new exposure surface;
- reviewing privileged access and break-glass events regularly.

Logs that are collected but never reviewed provide detection capability only on paper.

### Manage Insider Risk Deliberately

Insiders are a leading cause of privacy incidents, through error, curiosity, malice, or departure. Insider risk requires controls distinct from external threat defense.

Manage insider risk by:

- scoping access to the minimum necessary and reviewing it on life events;
- monitoring for unauthorized queries, exports, or access to records outside a role;
- controlling the ability to export or download bulk personal data;
- ensuring departures trigger immediate access revocation and device return;
- training staff on the privacy implications of curiosity-driven access, such as viewing records of acquaintances or public figures.

Curiosity access to records of celebrities, neighbors, or ex-partners is a recurring and serious insider failure.

### Secure Non-Production And Analytical Environments

Personal data often leaks into development, testing, and analytics environments that have weaker controls than production. These environments are frequent breach sources.

Secure non-production by:

- using synthetic, masked, or pseudonymized data in development and testing;
- applying production-equivalent controls where real personal data must be used;
- restricting and logging access to analytical environments with personal data;
- preventing uncontrolled exports to notebooks, local files, or unsanctioned tools.

### Align Controls To Data Sensitivity And Risk

Controls should scale with the sensitivity and volume of data and the likelihood and impact of harm. Not all data warrants the same posture.

Calibrate controls by:

- classifying data by sensitivity (public, internal, confidential, special category);
- applying stronger controls to special category, children's, financial, and credentials data;
- considering volume and concentration, since a large centralized store is a higher-value target;
- documenting the risk basis for the control set chosen.

### Verify Controls Operate, Not Just Exist

A control that is designed but not operating is not a control. Verification is part of the obligation.

Verify by:

- testing access reviews, encryption scope, and monitoring alerts periodically;
- reviewing exception logs and break-glass usage;
- conducting independent assessments or audits for high-risk systems;
- tracking remediation of findings to closure.

## Common Traps

### Blanket Production Access For Engineers

Wide production read access "for debugging" defeats least privilege and turns every engineer into a breach risk.

### Encryption In Policy But Not In A Specific Store

Claiming encryption as a mitigation while a database or backup is unencrypted is exposed the moment a breach occurs.

### Logs Collected But Never Reviewed

Detection capability exists only if someone reviews and acts on the logs. Unreviewed logs are storage, not monitoring.

### Curiosity Access To Sensitive Records

Accessing records of acquaintances or public figures out of curiosity is a serious insider violation that monitoring should catch.

### Personal Data In Non-Production

Copying production personal data into development or analytics environments with weaker controls is a frequent breach source.

### Departures Without Immediate Revocation

Delayed access revocation after a departure creates a window for data theft or sabotage.

### Pseudonymization Treated As Anonymization

Pseudonymized data is still personal data and still needs controls; treating it as out of scope is a costly error.

### Static Access That Never Gets Reviewed

Access granted once and never reviewed accumulates entitlements that no longer match the user's role.

## Self-Check

- Is access granted on a least-privilege, need-to-know basis, scoped to specific records and fields, and removed promptly on role change or departure?
- Is role-based access control structured with segregation of duties, production separated from non-production, and break-glass logged and reviewed?
- Is personal data encrypted at rest and in transit, with key management separated from the data and rotation in place?
- Is pseudonymization used to reduce exposure in analytics, ML, and non-production, with re-identification keys tightly controlled?
- Are access, bulk queries, exports, and privileged actions logged, monitored, and alerted on, with logs protected and retained appropriately?
- Is insider risk managed through scoped access, export controls, departure revocation, and monitoring of curiosity-driven access?
- Are non-production and analytical environments secured with masked or synthetic data, or production-equivalent controls where real data is used?
- Are controls calibrated to data sensitivity and risk, with stronger controls for special category, children's, financial, and credentials data?
- Are controls verified to operate through testing, exception review, independent assessment, and remediation tracking?
- Is the control set documented with its risk basis so that breach risk assessments can rely on it?
