---
name: compliance_privacy_and_data_governance_policy.md
description: Use when the agent is drafting a privacy or data governance policy, defining data classification and handling rules, connecting data retention to privacy obligations, or reviewing how personal and sensitive data is classified, protected, retained, and governed across its lifecycle.
---

# Compliance Privacy And Data Governance Policy

Privacy and data governance are inseparable. Privacy obligations, such as those under GDPR, CCPA, HIPAA, and sectoral laws, cannot be met unless the organization knows what data it holds, how it is classified, how it is protected, who can access it, how long it is kept, and where it flows. A privacy policy that states principles without an underlying data governance framework is unenforceable, because the people who must apply it cannot find, classify, or control the data it covers. The failure mode is a privacy policy full of commitments the organization cannot keep because it has no operational handle on its data. The judgment is building data classification, tying governance to privacy obligations, connecting retention to purpose limitation, and ensuring the policy reflects what the organization can actually do with its data.

Use this skill before drafting a privacy or data governance policy, defining classification, connecting retention to privacy, or reviewing data lifecycle controls. The goal is to make the agent think about classification, lawful basis, access, retention, cross-border flow, and the operational reality of governing data.

## Core Rules

### Build Data Classification As The Foundation

Privacy and governance depend on knowing what data exists and how sensitive it is. Classification is the foundation that makes every other control possible.

Classify data by:

- personal data, including identifiers, contact, and behavioral data;
- special category or sensitive data, such as health, biometric, genetic, racial, religious, sexual, or political data;
- financial data, including payment and account data;
- credentials and authentication data;
- confidential business data, such as trade secrets and strategy;
- public or non-sensitive data.

Define handling rules for each class: who may access it, how it must be stored and transmitted, how long it is kept, and whether it may be shared or transferred. Classification must be usable, so that data owners can assign a class without expert help, and consistent, so that the same type of data is handled the same way everywhere.

### Tie Governance To Lawful Basis And Purpose

Privacy law requires that personal data be processed only with a lawful basis and for specified, legitimate purposes. Data governance must enforce these limits operationally.

Tie governance to basis and purpose by:

- recording the lawful basis for each processing activity;
- limiting use to the purposes for which the data was collected, unless a new basis exists;
- preventing repurposing of data without assessing the new basis and notice;
- mapping data flows to purposes so that data collected for one purpose is not silently used for another;
- ensuring governance controls, such as access and retention, reflect the purpose limitations.

A governance framework that does not enforce purpose limitation allows data to drift into uses the organization cannot justify, which is a core privacy violation.

### Connect Retention To Purpose And Minimization

Retention is where privacy and governance intersect most directly. Privacy law's data-minimization and purpose-limitation principles require that personal data not be kept longer than necessary for the purpose. Retention must reflect this.

Connect retention by:

- setting retention periods tied to the purpose for which data was collected;
- ensuring retention does not exceed what privacy law permits, even where other obligations allow longer;
- reconciling retention obligations, which may require keeping data, with privacy obligations, which may require deleting it, and documenting the resolution;
- providing for deletion or anonymization at end of retention;
- handling data subject deletion requests against retention and legal hold obligations.

Retention that ignores privacy creates over-retention violations; retention that ignores legal obligations creates under-retention violations. The policy must reconcile both deliberately.

### Govern Access On A Need-To-Know Basis

Privacy requires that access to personal data be limited to those who need it for the purpose. Broad access defeats privacy controls regardless of other protections.

Govern access by:

- assigning access based on role and legitimate need, not convenience or seniority;
- reviewing access periodically and at role changes;
- restricting access to special category data more tightly;
- logging access to sensitive data so misuse can be detected;
- removing access promptly at termination or role change;
- addressing privileged and administrative access, which can bypass ordinary controls.

Access governance is a primary privacy control. Uncontrolled access is treated by regulators as a failure to protect personal data, even where no breach has occurred.

### Manage Cross-Border Data Transfers

Personal data often moves across borders, and privacy law imposes specific conditions on international transfers. Governance must account for where data is stored, processed, and accessed.

Manage transfers by:

- identifying where personal data is stored, processed, and remotely accessed;
- determining the transfer mechanisms required, such as adequacy, standard contractual clauses, or binding corporate rules;
- assessing the risks of transfers to jurisdictions with different protection levels;
- documenting transfer mechanisms and keeping them current as laws change;
- addressing remote access and support, which can constitute a transfer even without data relocation.

Cross-border transfer failures are a frequent enforcement focus. Governance must know where data goes and ensure each transfer has a valid mechanism.

### Address The Full Data Lifecycle

Privacy and governance apply across the data lifecycle, from collection through use, sharing, retention, and disposal. A policy that controls collection but ignores disposal leaves data exposed at the end.

Address the lifecycle:

- collection, with minimization, notice, and lawful basis;
- storage, with classification-appropriate protection;
- use, with purpose limitation and access control;
- sharing, with contracts, transfer mechanisms, and need-to-know;
- retention, with purpose-tied periods and reconciliation with legal obligations;
- disposal, with secure deletion or anonymization and evidence of disposition.

Each stage has distinct controls, and a gap at any stage creates privacy exposure. The policy must cover the lifecycle end to end.

### Integrate With Security, Vendor, And Incident Frameworks

Data governance does not operate alone. It must integrate with security controls, third-party management, and breach response, which together determine whether privacy commitments are met.

Integrate by:

- aligning governance classification with security protection levels;
- ensuring vendors and processors are bound by contract to equivalent governance and privacy obligations;
- coordinating with breach detection and notification so governance data, such as what was held and who was affected, is available in an incident;
- aligning with data subject request handling so governance supports timely response;
- coordinating with records management and legal hold.

A privacy policy that is not integrated with security, vendors, and incident response makes promises the organization cannot keep under pressure.

### Keep The Policy Aligned To Law And Refreshed

Privacy law evolves rapidly, with new jurisdictions, rights, transfer rules, and enforcement priorities. A privacy and governance policy that is not refreshed becomes wrong and creates exposure.

Keep aligned by:

- assigning ownership, usually a privacy officer or data protection officer;
- tracking legal developments in each applicable jurisdiction;
- reviewing the policy on a cycle and when laws, products, or data uses change;
- updating classification, retention, and transfer rules as law evolves;
- retraining affected roles after material changes.

A static privacy policy in a dynamic legal environment is a liability. The program must visibly track and adapt to change.

## Common Traps

### Privacy Policy Without Underlying Data Governance

A policy that states privacy principles without operational classification, access, and retention controls cannot be applied and makes commitments the organization cannot keep.

### No Or Inconsistent Data Classification

Without classification, handling rules cannot be applied, and sensitive data is treated the same as non-sensitive, creating exposure.

### Retention Ignoring Purpose Limitation

Retention that keeps personal data longer than the purpose requires violates minimization, even where other obligations might allow longer retention.

### Access Based On Convenience Or Seniority

Broad access defeats privacy controls and is treated by regulators as a failure to protect personal data.

### Unmanaged Cross-Border Transfers

Transfers without valid mechanisms, including remote access, are a frequent enforcement focus and a common source of violations.

### Controlling Collection But Not Disposal

A policy that governs collection but not secure disposal leaves data exposed at the end of the lifecycle.

### No Integration With Security, Vendors, And Incident Response

A privacy policy disconnected from security, vendor management, and breach response cannot meet its commitments under pressure.

### Static Policy In A Dynamic Legal Environment

A privacy policy that is not refreshed against evolving law becomes wrong and creates exposure rather than protection.

## Self-Check

- Is data classification established as the foundation, with usable, consistent classes and handling rules for access, storage, transmission, retention, and sharing?
- Is governance tied to lawful basis and purpose, preventing repurposing of data without a new basis and notice?
- Is retention connected to purpose and minimization, reconciled with legal obligations, with deletion or anonymization at end of retention?
- Is access governed on a need-to-know basis, reviewed periodically, logged for sensitive data, and removed promptly at role change?
- Are cross-border transfers identified, with valid mechanisms, risk assessment, and documentation kept current as laws change?
- Does the policy cover the full data lifecycle from collection to disposal, with controls at each stage?
- Is the policy integrated with security protection levels, vendor and processor contracts, breach response, and data subject request handling?
- Is ownership assigned and the policy reviewed on a cycle and when laws, products, or data uses change?
- Are affected roles retrained after material policy changes?
- Does the policy reflect what the organization can operationally do with its data, rather than commitments it cannot keep?
