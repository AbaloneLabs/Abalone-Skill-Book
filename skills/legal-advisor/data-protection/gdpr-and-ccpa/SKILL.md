---
name: gdpr_and_ccpa.md
description: Use when the agent is assessing lawful basis for processing personal data, managing consent and data-subject rights, appointing a data protection officer, conducting a data protection impact assessment, handling controller-processor relationships, or evaluating GDPR, CCPA, or equivalent privacy-law exposure before escalation to privacy counsel.
---

# GDPR And CCPA

Privacy law has fragmented into overlapping regimes with different triggers, rights, and enforcement styles. The GDPR, the CCPA and its successor, and a growing list of state and national laws each apply based on who is processed, where, and for what purpose, and they interact in ways that can layer obligations rather than substitute for each other. Agents often pick one regime, apply its vocabulary, and miss that the same processing can trigger multiple laws with conflicting requirements on consent, retention, transfers, and breach notice.

Use this skill before launching a product or feature that processes personal data, drafting privacy notices and consent flows, appointing data protection roles, conducting impact assessments, structuring controller-processor relationships, or assessing privacy-law exposure. The goal is to force the agent to identify applicable regimes, establish lawful basis, honor data-subject rights, manage transfers and breach duties, and escalate to privacy counsel for high-stakes matters. This skill is decision support, not legal advice.

## Core Rules

### Identify Every Applicable Regime, Not Just One

Privacy obligations depend on who is processed, where the controller is, and where data flows.

Map:

- GDPR applicability (offering goods or services to, or monitoring, people in the EEA/UK, regardless of controller location);
- UK GDPR and post-Brexit divergence;
- CCPA/CPRA and the growing list of U.S. state privacy laws, each with different triggers and rights;
- sector-specific laws (health, financial, children's, telecommunications, employment);
- national laws outside the EU and U.S. (Brazil, Canada, China, India, and others);
- the strictest applicable rule where regimes overlap.

Do not assume U.S.-only operations avoid GDPR, or that compliance with one state law satisfies all others. Identify every triggered regime and design to the strictest.

### Establish A Valid Lawful Basis For Each Processing Purpose

Processing personal data requires a legal basis, and the basis differs by regime and purpose.

For GDPR, identify:

- consent (freely given, specific, informed, unambiguous, withdrawable);
- contract necessity;
- legal obligation;
- vital interests;
- public task;
- legitimate interests (with balancing assessment).

For CCPA and state laws, identify:

- the right to opt out of sale or sharing;
- the right to limit sensitive-data processing;
- exemptions for employment, B2B, and health data where applicable;
- consent requirements for minors.

Sensitivity changes the analysis. Special-category, children's, biometric, health, financial, and precise-location data often require explicit consent or heightened basis.

### Honor Data-Subject Rights With Process And Timelines

Rights are operational obligations, not policy statements.

Build processes for:

- access and copies of personal data;
- rectification of inaccurate data;
- erasure (the right to be forgotten, with limits);
- restriction of processing;
- data portability;
- objection to processing;
- opt-out of sale, sharing, profiling, and targeted advertising;
- withdrawal of consent;
- rights related to automated decision-making.

Define intake channels, identity verification, response timelines (often short), exemption handling, and logging. Failing to respond within statutory deadlines is itself a violation.

### Provide Transparent Notices That Match Actual Practice

Notices must accurately describe what the organization does.

Ensure notices cover:

- identity and contact of the controller and DPO or representative;
- purposes and legal basis for each processing activity;
- categories of personal data and recipients;
- international transfers and safeguards;
- retention periods;
- data-subject rights and how to exercise them;
- automated decision-making and profiling;
- right to complain to a supervisory authority;
- updates and effective dates.

A notice that promises less processing than actually occurs, or that buries material terms, is non-compliant. Align the notice with practice and keep it current.

### Manage Controller, Processor, And Joint-Controller Relationships

Roles determine responsibilities, and contracts are required.

Address:

- controller versus processor determination for each activity;
- data-processing agreements with mandatory clauses;
- sub-processor approval and flow-down;
- joint-controller arrangements and allocation of duties;
- audit and assistance obligations;
- breach notification duties on processors;
- deletion or return of data at termination;
- international-transfer mechanisms in the contract.

Using a vendor to process personal data without a compliant processing agreement leaves the controller exposed and is itself a violation.

### Conduct Data Protection Impact Assessments Where Required

High-risk processing requires formal assessment.

Trigger DPIAs for:

- large-scale processing of sensitive data;
- systematic monitoring of public areas;
- profiling with significant effects;
- large-scale systematic monitoring;
- innovative technology with high risk;
- processing affecting vulnerable individuals.

A DPIA must assess necessity, proportionality, risks, and mitigations, and consult the supervisory authority where high residual risk remains. Skipping a required DPIA is an independent violation.

### Appoint DPOs And Representatives Where Required

Governance roles may be mandatory.

Address:

- DPO requirement for public authorities, large-scale systematic monitoring, or large-scale special-category processing;
- DPO independence, reporting line, and contact publication;
- EU or UK representative for non-EU/UK controllers subject to those laws;
- contact points for data subjects and authorities.

Failing to appoint a required DPO or representative, or appointing one without independence, is a violation.

### Handle Retention, Minimization, And Security As Core Duties

Privacy by design requires limiting what is kept and protecting it.

Address:

- data minimization (collect only what is necessary);
- purpose limitation (do not repurpose without basis);
- storage limitation (define and enforce retention);
- accuracy (keep data correct and current);
- integrity and confidentiality (security appropriate to risk);
- accountability (document compliance).

Hoarding data "in case it is useful" violates minimization and storage limitation. Define retention and enforce deletion.

### Prepare For Enforcement, Fines, And Complaints

Enforcement is active and penalties can be severe.

Address:

- cooperation with supervisory authorities;
- handling of complaints and data-subject requests;
- records of processing activities;
- breach-notification readiness;
- mitigation and remediation strategies;
- the role of effective compliance in reducing penalties.

Max fines under GDPR reach a percentage of global turnover. Treat privacy as enterprise risk, not a compliance footnote.

## Common Traps

### Applying One Regime And Ignoring Others

The same processing can trigger multiple laws. Identify all and design to the strictest.

### Consent Used As A Default Basis

Consent must be freely given and withdrawable and is often the wrong basis. Choose the correct lawful basis per purpose.

### Notices That Do Not Match Practice

Over-promising or burying terms is non-compliant. Align notice with actual processing.

### Vendors Without Compliant Processing Agreements

Using processors without required contract terms is itself a violation.

### Skipping Required DPIAs

High-risk processing without assessment is an independent violation.

### Data Hoarding

Retaining data without purpose violates minimization and storage limitation. Define and enforce retention.

### Missing Data-Subject Response Deadlines

Late or absent responses are violations. Build operational processes with timelines.

## Self-Check

- Are all applicable regimes (GDPR, UK GDPR, CCPA/CPRA, state laws, sector-specific, non-EU/US national laws) identified with the strictest rule applied where they overlap?
- Is a valid lawful basis established for each processing purpose, with heightened basis for sensitive, children's, biometric, health, financial, and location data?
- Are data-subject rights (access, rectification, erasure, restriction, portability, objection, opt-out, withdrawal, automated-decision rights) supported by intake, identity verification, timelines, exemptions, and logging?
- Do privacy notices accurately and currently describe identity, purposes, legal basis, categories, recipients, transfers, retention, rights, automated decisions, complaint rights, and updates?
- Are controller, processor, and joint-controller roles determined with compliant processing agreements, sub-processor flow-down, audit and assistance, breach duties, deletion, and transfer mechanisms?
- Are DPIAs conducted for high-risk processing with necessity, proportionality, risk, mitigation, and authority consultation?
- Are DPOs and representatives appointed where required with independence, reporting line, and published contact?
- Are minimization, purpose limitation, retention, accuracy, security, and accountability implemented and documented?
- Is enforcement readiness (cooperation, complaints, records of processing, breach notification, mitigation, compliance-for-penalty-reduction) maintained?
- Does the output escalate multi-regime analysis, DPIAs, high-risk processing, and enforcement exposure to qualified privacy counsel?