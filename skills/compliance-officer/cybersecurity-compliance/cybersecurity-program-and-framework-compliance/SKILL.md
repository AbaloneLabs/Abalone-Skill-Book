---
name: cybersecurity_program_and_framework_compliance.md
description: Use when the agent is establishing cybersecurity programs, implementing NIST CSF or ISO 27001 frameworks, evaluating security control requirements, conducting cybersecurity risk assessments, or ensuring compliance with sector-specific cybersecurity regulations including SEC, NYDFS, HIPAA Security Rule, and critical infrastructure requirements.
---

# Cybersecurity Program And Framework Compliance

Cybersecurity program and framework compliance governs how organizations build, operate, and demonstrate the adequacy of their security controls. The defining feature is that cybersecurity is now subject to multiple overlapping regulatory regimes—SEC public company disclosure, NYDFS for financial services, HIPAA Security Rule for healthcare, TSA for critical infrastructure, and executive orders for federal contractors—each with different but converging expectations. The central difficulty is that "reasonable security" is a context-dependent standard, that frameworks provide structure but not safe harbors, and that regulators increasingly demand evidence of governance, not just technical controls.

Use this skill before advising on cybersecurity program design, framework selection, control implementation, or regulatory compliance. The goal is to make the agent identify the applicable regulatory regimes, the framework fit, the governance structure, and the evidence requirements before concluding that a cybersecurity program is compliant.

## Core Rules

### Identify All Applicable Cybersecurity Regulatory Regimes

Cybersecurity obligations come from multiple sources.

Map:

- sector-specific cybersecurity rules (NYDFS Part 500, HIPAA Security Rule, GLBA Safeguards Rule);
- public company disclosure obligations (SEC cybersecurity disclosure rules);
- critical infrastructure requirements (TSA pipeline, rail, aviation directives; CIRCIA reporting);
- federal contractor requirements (CUI, FISMA, FedRAMP);
- state data breach notification laws (security obligation components);
- international regimes (EU NIS2 Directive, DORA for financial entities);
- privacy law security requirements (GDPR Article 32, CCPA reasonable security);
- industry-specific frameworks (PCI DSS for payment cards).

Multiple regimes may apply simultaneously. Each has different scope, requirements, and enforcement mechanisms. The program must satisfy the most stringent applicable requirement. Regulators increasingly coordinate and share information, so gaps in one regime may surface in another.

### Select And Implement A Recognized Framework

A recognized framework provides structure and demonstrates reasonableness.

Choose among:

- NIST Cybersecurity Framework (CSF) 2.0 (Govern, Identify, Protect, Detect, Respond, Recover);
- ISO/IEC 27001 and 27002 (information security management system);
- NIST SP 800-53 (federal and high-security environments);
- CIS Critical Security Controls;
- COBIT for governance;
- sector-specific frameworks (PCI DSS, HITRUST);
- the framework's fit to the organization's size, sector, and risk profile.

Framework selection should match the organization's needs and regulatory environment. NIST CSF is widely recognized and flexible. ISO 27001 provides certifiable structure. The framework should be implemented substantively, not as a paper exercise. Certification (where available) provides external validation but is not a safe harbor.

### Establish Governance And Board Oversight

Cybersecurity governance requires clear accountability.

Implement:

- board and senior management oversight of cybersecurity risk;
- a designated CISO or equivalent with appropriate authority and resources;
- defined roles and responsibilities across the organization;
- risk appetite statements for cybersecurity risk;
- integration with enterprise risk management;
- regular reporting to the board on cyber risk posture;
- cybersecurity as a board-level agenda item;
- accountability for control ownership and remediation;
- third-party and supply chain risk governance.

Governance is a core expectation under SEC rules, NYDFS, NIS2, and other regimes. The board must oversee cyber risk, not merely receive reports. The CISO needs authority and resources. Risk appetite should be defined and communicated. Third-party risk governance is increasingly scrutinized.

### Conduct Risk Assessments To Drive Control Selection

Controls should be risk-based, not checklist-driven.

Perform:

- enterprise cybersecurity risk assessments identifying threats, vulnerabilities, and assets;
- the likelihood and impact of identified risks;
- the mapping of risks to controls;
- the identification of control gaps;
- the prioritization of remediation;
- periodic reassessment (at least annually and after major changes);
- the documentation of risk acceptance decisions;
- the integration of risk assessment with control implementation.

Risk assessment drives control selection. A risk that is not identified cannot be controlled. Risk acceptance decisions must be documented and owned. Reassessment must occur after major changes (new systems, acquisitions, threat landscape shifts). The assessment should cover both internal and external threats.

### Implement Controls Across All Framework Functions

Controls must span the full lifecycle.

Cover:

- identify (asset management, risk assessment, supply chain risk);
- protect (access control, data security, protective technology, training);
- detect (anomaly and event detection, continuous monitoring);
- respond (response planning, communications, analysis, mitigation);
- recover (recovery planning, improvements, communications);
- govern (organizational context, risk management strategy, roles, policy, oversight).

Controls must be comprehensive across all functions. Common gaps include: unmanaged assets (shadow IT), inadequate access controls (over-privilege), insufficient detection (no continuous monitoring), untested response plans, and no recovery procedures. The Govern function (added in NIST CSF 2.0) elevates governance to a core function.

### Manage Access Control And Identity Security

Access control is a foundational security domain.

Implement:

- least privilege access (users have only the access they need);
- multi-factor authentication for remote and privileged access;
- periodic access reviews and certification;
- privileged access management for administrator accounts;
- joiner-mover-leaver processes for timely access changes;
- separation of duties for sensitive functions;
- network segmentation to limit lateral movement;
- identity governance and administration.

Access control failures are a leading cause of breaches. Over-privilege, stale access, and weak authentication are common gaps. MFA is increasingly mandated by regulation and insurance. Privileged accounts require special controls. Access reviews must be periodic and documented. Joiner-mover-leaver processes prevent stale access.

### Ensure Incident Detection, Response, And Recovery

Detection, response, and recovery capabilities must be operational.

Implement:

- continuous monitoring and threat detection;
- security incident response plans with defined roles and procedures;
- tabletop exercises and plan testing;
- incident classification and severity assessment;
- forensic capabilities and evidence preservation;
- business continuity and disaster recovery plans;
- backup and restoration testing;
- post-incident review and lessons learned;
- communication plans for internal and external stakeholders.

Plans that are not tested are assumptions. Tabletop exercises reveal gaps. Incident classification drives response urgency. Forensic evidence preservation supports legal and regulatory needs. Backups must be tested for restorability. Post-incident review drives improvement.

### Manage Third-Party And Supply Chain Cybersecurity Risk

Supply chain risk is a major attack vector.

Control:

- vendor cybersecurity due diligence before onboarding;
- contractual security requirements and rights to audit;
- continuous monitoring of vendor security posture;
- vendor incident notification requirements;
- the security of the vendor's own vendors (nth-party risk);
- software supply chain security (SBOMs, code signing, secure development);
- concentration risk and critical vendor dependency;
- the security of cloud and SaaS providers.

Supply chain attacks (SolarWinds, MOVEit) demonstrate the risk. Vendor due diligence must be risk-based. Contracts must include security terms and audit rights. Continuous monitoring catches changes in vendor posture. Software supply chain security (SBOMs, secure development practices) is increasingly expected.

### Maintain Evidence And Documentation For Regulatory Examination

Regulators require evidence of compliance.

Maintain:

- the cybersecurity program documentation and policies;
- risk assessment documentation and results;
- control implementation evidence (configurations, logs, test results);
- board and senior management reporting records;
- incident response documentation and post-incident reports;
- training records and completion rates;
- vendor due diligence and monitoring records;
- audit and assessment results;
- remediation tracking and status.

Regulators ask for evidence, not assertions. Documentation must demonstrate that controls are designed and operating effectively. Board reporting records show oversight. Incident documentation supports regulatory notifications. Remediation tracking shows continuous improvement. Evidence should be organized for efficient regulatory examination.

## Common Traps

### Framework Implemented On Paper But Not In Practice

A framework that is documented but not operationally implemented provides no protection.

### No Board-Level Oversight Or CISO Authority

Governance that exists in name only does not meet regulatory expectations.

### Controls Selected By Checklist Without Risk Assessment

Checklist controls without risk assessment may miss the organization's actual risks.

### Incident Response Plan Never Tested

Untested plans fail when needed and do not satisfy regulatory expectations.

### Third-Party Risk Treated As A Checkbox

Vendor security that is assessed once at onboarding and never monitored misses ongoing risk.

### No Evidence Of Control Operation

Controls that cannot be evidenced are presumed not to exist by regulators.

### Multiple Frameworks Implemented In Silos

Redundant, inconsistent frameworks create gaps and waste resources.

## Self-Check

- Are all applicable cybersecurity regulatory regimes identified including NYDFS, HIPAA Security Rule, GLBA Safeguards, SEC disclosure, TSA/CIRCIA critical infrastructure, federal contractor, state breach laws, NIS2, DORA, GDPR security, and PCI DSS?
- Is a recognized framework selected and implemented (NIST CSF 2.0, ISO 27001, NIST 800-53, CIS Controls, COBIT, sector-specific) appropriate to size, sector, and risk profile?
- Is governance established with board oversight, CISO authority, defined roles, risk appetite, ERM integration, regular reporting, board agenda inclusion, control ownership, and third-party governance?
- Are risk assessments conducted to drive control selection covering threats, vulnerabilities, assets, likelihood/impact, risk-to-control mapping, gap identification, remediation prioritization, periodic reassessment, risk acceptance documentation, and internal/external threats?
- Are controls implemented across all functions (identify, protect, detect, respond, recover, govern) without common gaps?
- Is access control and identity security implemented with least privilege, MFA, access reviews, privileged access management, joiner-mover-leaver, separation of duties, segmentation, and identity governance?
- Are incident detection, response, and recovery operational with continuous monitoring, response plans, tabletop exercises, classification, forensics, BCP/DR, backup testing, post-incident review, and communications?
- Is third-party and supply chain cybersecurity risk managed with due diligence, contractual terms, continuous monitoring, incident notification, nth-party risk, software supply chain (SBOMs), concentration risk, and cloud/SaaS security?
- Is evidence and documentation maintained for regulatory examination including program docs, risk assessments, control evidence, board reporting, incident docs, training records, vendor records, audit results, and remediation tracking?
- Is the cybersecurity program reviewed and updated at least annually and after major incidents or changes?