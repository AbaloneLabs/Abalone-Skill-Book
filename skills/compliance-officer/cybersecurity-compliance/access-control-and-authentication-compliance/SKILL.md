---
name: access_control_and_authentication_compliance.md
description: Use when the agent is designing access control systems, implementing multi-factor authentication, managing privileged access, evaluating identity and access management compliance, or ensuring compliance with access control requirements under HIPAA Security Rule, PCI DSS, NYDFS, SOX IT general controls, and cybersecurity frameworks.
---

# Access Control And Authentication Compliance

Access control and authentication compliance governs how organizations verify the identity of users and systems and limit their access to only what they need. The defining feature is that access control sits at the intersection of security, privacy, and operational efficiency, that over-privilege and stale access are among the most common audit findings, and that regulatory expectations for authentication have risen sharply with MFA mandates across financial services, healthcare, and critical infrastructure. The central difficulty is that access models must balance the principle of least privilege against operational friction, that privileged accounts are both the most dangerous and the hardest to constrain, and that joiner-mover-leaver processes span multiple systems and teams.

Use this skill before advising on access control design, MFA implementation, privileged access management, or access review programs. The goal is to make the agent identify the applicable regulatory requirements, the access model, the authentication standards, and the review cadence before concluding that access control is compliant.

## Core Rules

### Identify Applicable Access Control Regulatory Requirements

Access control requirements come from multiple sources.

Map:

- HIPAA Security Rule access control and authentication standards (164.312(a), (d));
- PCI DSS Requirement 7 (restrict access to cardholder data) and Requirement 8 (identify users and authenticate);
- NYDFS Part 500 multi-factor authentication requirements;
- SOX IT general controls for financial reporting systems;
- GLBA Safeguards Rule access control requirements;
- FedRAMP and FISMA access control (NIST SP 800-53 AC family);
- GDPR Article 32 security requirements including access control;
- sector-specific requirements (defense CMMC, energy NERC CIP).

Multiple regimes impose access control requirements. HIPAA requires unique user identification, emergency access, automatic logoff, and encryption/decryption. PCI DSS requires role-based access, MFA for all access to cardholder data, and unique IDs. NYDFS mandates MFA for remote access and privileged accounts. SOX requires controls over financial system access. The program must satisfy the most stringent applicable requirement.

### Implement Least Privilege And Need-To-Know

Access must be minimized to what is necessary.

Implement:

- role-based access control (RBAC) mapped to job functions;
- the principle of least privilege (minimum access for the role);
- need-to-know restrictions for sensitive data and systems;
- the separation of duties for sensitive functions (no single person can complete a high-risk transaction);
- default-deny posture (access denied unless explicitly granted);
- periodic access reviews and certification;
- the removal of access no longer needed;
- the documentation of access decisions and approvals.

Least privilege limits damage from compromised accounts and insider threats. RBAC standardizes access by role. Need-to-know adds a contextual layer for sensitive data. Separation of duties prevents fraud and errors. Default-deny ensures access is intentional. Periodic reviews catch over-privilege and stale access. Access decisions must be documented for audit.

### Enforce Multi-Factor Authentication Where Required

MFA is increasingly mandated.

Implement:

- MFA for all remote access (NYDFS, PCI DSS, HIPAA guidance);
- MFA for all privileged accounts;
- MFA for access to sensitive data (cardholder data, PHI, financial records);
- MFA for administrative access to cloud and infrastructure;
- the selection of MFA factors (something you know, have, are);
- the strength of factors (phishing-resistant authentication where feasible);
- the handling of MFA for service accounts and APIs;
- the fallback and recovery processes that do not undermine MFA.

MFA is mandated by NYDFS for remote access and privileged accounts, by PCI DSS for all cardholder data access, and recommended by HIPAA guidance. The strength of factors matters: SMS-based MFA is vulnerable to SIM swapping; phishing-resistant authentication (FIDO2, WebAuthn) provides stronger protection. Service accounts and APIs need MFA alternatives (token rotation, mutual TLS). Recovery processes must not create MFA bypass vulnerabilities.

### Manage Privileged Access With Heightened Controls

Privileged accounts require the strongest controls.

Implement:

- the inventory of all privileged accounts (administrator, root, service, cloud);
- a privileged access management (PAM) solution for credential vaulting and session recording;
- just-in-time access (privilege granted only when needed, for a limited time);
- the elimination of standing privilege where possible;
- dual control for the most sensitive privileged actions;
- session monitoring and recording;
- the rotation of privileged credentials;
- the separation of administrative duties.

Privileged accounts are the highest-value targets. PAM solutions vault credentials, provide just-in-time access, and record sessions. Standing privilege (permanent admin access) should be minimized; just-in-time access grants privilege only when needed. Dual control requires two people for the most sensitive actions. Session recording deters misuse and supports investigation. Credential rotation limits the value of stolen credentials.

### Implement Joiner-Mover-Leaver (JML) Processes

Access must be provisioned, modified, and deprovisioned promptly.

Implement:

- automated provisioning from HR systems for new hires;
- role-based default access for the position;
- timely modification when employees change roles (mover);
- timely deprovisioning when employees leave (leaver);
- the coordination across multiple systems and teams;
- the handling of contractor and temporary worker access;
- the handling of access for acquired company employees;
- the tracking of JML timeliness metrics.

JML failures are a leading cause of stale and excessive access. New hires should receive appropriate access on day one. Role changes should trigger access modification. Departures should trigger immediate deprovisioning. The process must span all systems. Contractors and temporary workers need time-limited access. Acquisitions require integration of access management.

### Conduct Periodic Access Reviews And Certification

Access must be reviewed and certified periodically.

Implement:

- periodic access reviews (quarterly for high-risk, at least annually for all);
- manager certification of direct reports' access;
- application owner certification of user access;
- the review of privileged access with heightened scrutiny;
- the documentation of review results;
- the remediation of identified excessive or inappropriate access;
- the tracking of remediation to completion;
- the integration of review data with access governance tools.

Access reviews verify that access remains appropriate. Managers certify their direct reports' access. Application owners certify user access to their systems. Privileged access requires heightened scrutiny. Review results must be documented. Identified issues must be remediated and tracked. Access governance tools automate and support the review process.

### Manage Service Accounts And Non-Human Identities

Service accounts present unique access control challenges.

Manage:

- the inventory of all service accounts, API keys, and machine identities;
- the assignment of service account ownership;
- the principle of least privilege for service accounts;
- the rotation of service account credentials;
- the monitoring of service account usage;
- the elimination of shared service accounts where possible;
- the use of managed identities or workload identity where available;
- the handling of service accounts in cloud environments.

Service accounts and machine identities are often overlooked. They accumulate privilege, are rarely rotated, and lack individual accountability. Each service account needs an owner. Privilege must be minimized. Credentials must be rotated. Usage must be monitored for anomalies. Cloud managed identities reduce the credential management burden.

### Ensure Authentication And Identity Assurance Levels

Authentication strength must match the risk.

Match:

- identity assurance levels (IAL) to the verification required;
- authenticator assurance levels (AAL) to the authentication strength required;
- federation assurance levels (FAL) for federated identity;
- NIST SP 800-63 digital identity guidelines;
- the risk of the transaction to the required assurance level;
- the use of step-up authentication for higher-risk actions;
- the continuous evaluation of authentication risk.

NIST SP 800-63 defines assurance levels for identity proofing (IAL), authentication (AAL), and federation (FAL). Higher-risk transactions require higher assurance levels. Step-up authentication adds stronger authentication for sensitive actions. Continuous risk evaluation adjusts authentication requirements based on context (location, device, behavior). The assurance level must match the risk of the transaction.

## Common Traps

### Over-Privilege Through Role Creep

Access that accumulates over time as roles change without removal.

### MFA Implemented For Users But Not Service Accounts

Securing human accounts while leaving service accounts with static credentials.

### JML Deprovisioning Delays

Delayed deprovisioning when employees leave creates a window of unauthorized access.

### Access Reviews That Are Rubber-Stamp

Access reviews where managers approve without examining are not meaningful.

### Standing Privilege For All Administrators

Permanent admin access for all administrators increases the attack surface.

### Shared Service Accounts With No Individual Accountability

Shared accounts prevent attribution of actions to individuals.

### SMS-Based MFA Treated As Sufficient For High-Risk

SMS-based MFA is vulnerable to SIM swapping and should not be used for high-risk access.

## Self-Check

- Are applicable access control regulatory requirements identified including HIPAA, PCI DSS, NYDFS, SOX ITGCs, GLBA, FedRAMP/FISMA, GDPR, and sector-specific (CMMC, NERC CIP)?
- Is least privilege and need-to-know implemented with RBAC, minimum access, need-to-know restrictions, separation of duties, default-deny, periodic reviews, access removal, and documented decisions?
- Is MFA enforced where required for remote access, privileged accounts, sensitive data, and administrative access, with appropriate factor selection, phishing-resistant authentication where feasible, service account/API handling, and secure recovery?
- Is privileged access managed with account inventory, PAM solution, just-in-time access, standing privilege elimination, dual control, session monitoring/recording, credential rotation, and duty separation?
- Are JML processes implemented with automated provisioning, role-based defaults, timely mover modification, timely leaver deprovisioning, multi-system coordination, contractor handling, acquisition integration, and timeliness tracking?
- Are periodic access reviews and certification conducted quarterly for high-risk and at least annually for all, with manager certification, application owner certification, privileged access scrutiny, documentation, remediation, tracking, and governance tool integration?
- Are service accounts and non-human identities managed with inventory, ownership, least privilege, credential rotation, usage monitoring, shared account elimination, managed identities, and cloud handling?
- Are authentication and identity assurance levels matched to risk with IAL, AAL, FAL per NIST SP 800-63, transaction risk matching, step-up authentication, and continuous risk evaluation?
- Is access control evidence maintained for audit including access policies, review records, MFA configuration, PAM logs, and JML timeliness metrics?
- Are access control controls tested through independent audit or penetration testing?