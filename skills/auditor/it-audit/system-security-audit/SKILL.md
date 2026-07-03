---
name: system-security-audit.md
description: Use when the agent is auditing information system security, assessing access controls and authentication mechanisms, evaluating security architecture and configuration, testing vulnerability management, reviewing identity and access management, auditing cryptographic controls, or determining whether systems and data are protected against unauthorized access, modification, disclosure, and disruption.
---

# System Security Audit

A system security audit determines whether the controls protecting an information system and its data are designed appropriately and operating effectively to manage confidentiality, integrity, and availability risks. The central judgment problem is that security is a defense-in-depth discipline where no single control is sufficient and the attacker only needs one gap; agents frequently test controls in isolation, accept configuration screenshots as evidence of operation, or assess policy and design while skipping whether controls actually block real attack paths. Security auditing that does not think like an adversary produces false assurance.

## Core Rules

### Map the system boundary and data flows before testing controls

Define what is in scope: the applications, servers, networks, endpoints, cloud services, databases, and third-party integrations that constitute the system. Map how data enters, moves through, is stored in, and exits the system. Controls tested outside their actual context produce meaningless conclusions. A control that protects the perimeter is irrelevant if the real exposure is a direct cloud API; a control on the application is moot if the database is directly accessible. Boundary and data-flow mapping directs testing to where risk actually lives.

### Assess security against a recognized framework, adapted to the entity

Use an authoritative framework (NIST, ISO 27001/2, CIS Controls, COBIT, or sector-specific standards) as the criteria baseline. Adapt it to the entity's risk profile, regulatory obligations, and technology stack rather than applying it generically. Document which framework and which control objectives are in scope. Framework-free security auditing produces inconsistent, indefensible conclusions; rote framework application without adaptation misses what matters for this specific entity.

### Test the full access control lifecycle, not just existence

Access control is the most common security failure area. Evaluate the complete lifecycle:

- **identification and authentication** — are identities unique, is authentication strong (MFA where appropriate), are credentials protected?;
- **authorization** — is access granted on least-privilege and need-to-know, are privileged accounts restricted and monitored?;
- **provisioning** — is access granted through a controlled process tied to role and approval?;
- **modification** — are access changes processed promptly when roles change?;
- **deprovisioning** — is access removed promptly on transfer or termination?;
- **periodic review** — is access recertified regularly to remove creep and orphan accounts?.

A strong provisioning process with weak deprovisioning or no periodic review leaves dormant excessive access that is a primary breach vector.

### Verify control operation through testing, not documentation

Security policies, standards, and configuration baselines describe intent; they do not prove operation. Obtain evidence that controls actually function:

- reconcile access lists to system-generated reports, not to a maintained spreadsheet;
- test authentication by attempting access with test accounts where feasible;
- review actual configuration settings in the live system, not the documented standard;
- examine logs to confirm monitoring and alerting actually fire on test events;
- sample terminated users to confirm access was truly removed.

The gap between documented and actual control operation is where most security audit value is found.

### Evaluate vulnerability and patch management as a living process

Static security posture decays rapidly. Assess whether the entity maintains current security through:

- vulnerability scanning and penetration testing on a defined cadence;
- timely remediation within risk-based service levels;
- patch management for operating systems, applications, and firmware;
- tracking of exceptions and accepted risks with documented owner and expiry.

A point-in-time vulnerability list is less informative than whether the remediation process keeps up with emerging threats. Report on process health, not just current open findings.

### Assess cryptographic controls for appropriateness and key management

Cryptography underpins confidentiality and integrity. Evaluate:

- whether encryption is applied to data in transit, at rest, and where appropriate in use;
- whether algorithms and key lengths are current and not deprecated;
- whether key management (generation, storage, rotation, distribution, destruction) is secure and documented;
- whether certificate management prevents expiry and weak-certificate exposure.

Strong encryption with weak key management provides little protection. Deprecated algorithms (legacy TLS, weak hashes) are common silent failures.

### Consider the human and process dimensions of security

Technical controls fail without supporting human and process controls. Assess:

- security awareness training and its effectiveness, not just completion;
- incident detection and response capability, including roles, runbooks, and testing;
- segregation of duties between development, operations, and security;
- change management that prevents unreviewed security-impacting changes;
- third-party and supply-chain security, including vendor access and software supply chain.

A technically hardened system operated by untrained staff with untested incident response is not secure in practice.

### Evaluate logging, monitoring, and detection capability

Preventive controls always have residual failure; detective controls determine whether breaches are noticed. Assess whether:

- security-relevant events are logged with sufficient detail and protected from tampering;
- logs are centralized, retained per requirement, and monitored;
- alerts are tuned to surface real threats without overwhelming analysts;
- detection covers privileged activity, authentication anomalies, data exfiltration, and configuration changes;
- the entity can demonstrate detection of a simulated or actual incident.

Without detection, a successful breach may persist undetected for months; detection capability is a core security control, not an optional extra.

### Report security findings by risk and exploitability, not just count

A small number of critical, easily exploitable vulnerabilities is more dangerous than many low-severity issues. Rank findings by combined risk (impact and likelihood and exploitability) and illustrate realistic attack paths that chain multiple weaknesses. A findings list sorted by count or alphabetical category obscures the real risk picture.

## Common Traps

- **Documentation-as-operation.** Accepting policies, standards, or configuration baselines as evidence that controls operate, without testing live systems.
- **Control-in-isolation testing.** Evaluating each control separately without assessing whether they collectively block realistic attack paths.
- **Perimeter myopia.** Focusing on network and perimeter controls while ignoring application, cloud, API, and insider threats.
- **Lifecycle gaps.** Testing provisioning but not deprovisioning or periodic access review, leaving dormant excessive access unexamined.
- **Point-in-time snapshot.** Reporting current vulnerabilities without assessing whether the remediation process keeps pace with emerging threats.
- **Deprecated-crypto blindness.** Accepting "encrypted" as sufficient without checking algorithm currency and key management strength.
- **Detection omission.** Evaluating only preventive controls and ignoring whether breaches would actually be detected.
- **Human-factor neglect.** Assessing technical controls while skipping awareness, incident response, and segregation of duties.
- **Spreadsheet-as-truth.** Reconciling access to a manually maintained list rather than system-generated evidence.
- **Count-based ranking.** Prioritizing findings by number rather than combined risk and exploitability, misrepresenting the real threat picture.

## Self-Check

- Have I mapped the system boundary and data flows before testing, so controls are evaluated in their actual risk context?
- Am I assessing against a recognized, adapted security framework with documented control objectives, rather than ad hoc criteria?
- Did I test the full access control lifecycle (authentication, authorization, provisioning, modification, deprovisioning, periodic review), not just provisioning?
- Did I verify control operation through live testing and system-generated evidence, rather than relying on documentation and configuration baselines?
- Have I assessed vulnerability and patch management as an ongoing process with risk-based remediation service levels, not just a current snapshot?
- Did I evaluate cryptographic controls for algorithm currency and key management quality, not just presence of encryption?
- Have I assessed human and process controls (awareness, incident response, segregation of duties, change management, third-party security)?
- Did I evaluate logging, monitoring, and detection capability, including whether alerts fire and whether incidents would be detected?
- Are findings ranked by combined risk and exploitability with realistic attack-path illustrations, not just by count or category?
