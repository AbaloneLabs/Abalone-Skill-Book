---
name: system_security_and_backups.md
description: Use when the agent is securing library systems and data, applying patches and updates, managing vulnerabilities and incidents, planning backup and disaster recovery for library platforms and digital collections, defending against ransomware and account compromise, handling breach response and notification, conducting security audits, or ensuring recoverability of library systems, catalog data, patron records, and digital repository content.
---

# System Security And Backups

Library systems hold valuable and sensitive data: patron records that reveal reading habits, financial data for fines and fees, unique digital collections, years of cataloging effort, and licensed resource configurations. Securing these systems and ensuring they can be recovered after failure is a core operational responsibility. Security and backups are two sides of the same coin: security reduces the chance of loss, and backups guarantee recovery when prevention fails. A library that secures its systems but does not test backups will still lose data to ransomware or corruption; a library that backs up religiously but leaves systems unpatched will spend its life restoring from avoidable disasters.

The judgment problem is that security and backups are invisible when they work and obvious only when they fail, so they are chronically under-resourced. Patches are deferred, backups run untested, incident plans live in a document no one has read, and the assumption is that nothing bad will happen, until it does. Library systems are also attractive targets: public-facing services, sometimes legacy software, sometimes constrained staffing, and patron data with privacy value. The agent's job is to treat security and backups as disciplined, tested, ongoing practices, not as one-time setups, and to recognize that the most important question is not whether a backup exists but whether recovery has been proven.

Use this skill when securing library systems, managing patches and vulnerabilities, planning or testing backups and disaster recovery, responding to incidents, handling breach notification, or auditing recoverability. The goal is to prevent the agent from deferring patches, leaving backups untested, neglecting incident planning, assuming vendor-hosted means vendor-responsible, or treating security as compliance theater rather than genuine protection.

## Core Rules

### Layer Defenses; Do Not Rely On A Single Control

No single security control is sufficient. Defense in depth reduces the chance that one failure leads to catastrophe.

Defense in depth layers:

- network controls such as firewalls and segmentation;
- host hardening, including patching and disabling unneeded services;
- strong authentication and least privilege, covered in the authentication skill;
- application security and current vendor versions;
- data encryption in transit and at rest;
- monitoring and alerting for suspicious activity;
- backups as the final recovery layer.

Each layer assumes others may fail. Design so that a single compromise does not cascade into total loss.

### Patch And Update Promptly And Deliberately

Unpatched software is the most common entry point for attackers. Patching must be timely but controlled.

Patching practices:

- track all systems and their current versions, including vendor-hosted services;
- monitor vendor and security advisories for relevant vulnerabilities;
- assess severity and prioritize critical patches;
- test patches in a sandbox before production where possible;
- apply security patches promptly, especially for internet-facing systems;
- document patch status and deferred patches with justification;
- plan for end-of-life software that no longer receives patches.

Deferred patches accumulate into unmanageable vulnerability debt. Establish a regular patch cadence and stick to it.

### Protect Patron And Sensitive Data Specifically

Library data includes categories with elevated risk and obligation: patron records, financial data, and unique digital collections.

Sensitive data protections:

- encrypt patron data in transit and at rest;
- restrict access to patron records on a need-to-know basis;
- minimize retention of borrowing and reading history, aligning with privacy policy;
- protect financial data with appropriate controls and audit;
- treat digital collection master files as irreplaceable and back them up rigorously;
- segregate sensitive data from general systems where feasible.

Patron reading data has both privacy and legal sensitivity; treat it as you would other regulated personal data.

### Design Backups For Recovery, Not Just For Possession

A backup that has never been restored is an assumption. Design and test for actual recovery.

Backup design:

- follow the 3-2-1 principle: three copies, on two media types, with one offsite or offline;
- back up data, configuration, and system state, not just files;
- define recovery point and recovery time objectives for each system;
- protect backups from ransomware by keeping at least one copy offline or immutable;
- automate backups and monitor for failures, since silent backup failure is common;
- document what is backed up, where, and how to restore.

The question is never "do we have backups?" but "have we proven we can recover, and how recently?"

### Test Restoration Regularly

Untested backups fail at the worst moment. Restoration testing is mandatory.

Restoration testing:

- perform test restores on a schedule, at least quarterly for critical systems;
- verify restored data is complete, uncorrupted, and usable;
- test recovery of full systems, not just individual files;
- measure actual recovery time against objectives;
- document test results and fix any failures discovered;
- include vendor-hosted systems by confirming vendor recovery commitments in writing.

A restore test that reveals a corrupt backup is a success, because it prevents a real disaster from being unrecoverable. Test before you need it.

### Plan And Rehearse Incident Response

Security incidents will happen. A rehearsed plan determines whether they are contained or catastrophic.

Incident response planning:

- maintain a written incident response plan with roles and contacts;
- include procedures for ransomware, account compromise, and data breach;
- define escalation paths to institutional IT, security, legal, and leadership;
- preserve evidence and logs for investigation;
- have communication templates for stakeholders and patrons;
- conduct tabletop exercises to rehearse the plan;
- review and update the plan after every incident or exercise.

A plan that has never been exercised will fail under pressure. Rehearse before the real incident.

### Prepare For Breach Notification Obligations

If patron or personal data is breached, the library may have legal notification obligations. Prepare in advance.

Breach preparation:

- know which data triggers notification under applicable law, such as state breach laws or GDPR;
- understand notification timelines and recipients;
- coordinate with institutional legal counsel and privacy officers;
- prepare patron notification templates in advance;
- document the incident, scope, and response for accountability;
- review vendor contracts for breach responsibilities and notification clauses.

Scrambling to understand obligations during a breach wastes critical time. Know the requirements beforehand.

### Audit Security And Access Periodically

Security drifts. Periodic audits catch excessive access, unpatched systems, and policy gaps.

Audit practices:

- review user accounts and permissions for orphans and excess;
- verify patch status across all systems;
- audit access logs for anomalies;
- review vendor security practices and contractual protections;
- assess configuration against security baselines;
- document findings and remediation.

Treat audits as improvement opportunities, not compliance theater. Act on findings.

### Clarify Vendor Security And Recovery Responsibilities

Many library systems are vendor-hosted. Vendor responsibilities must be explicit, not assumed.

Vendor clarity:

- review vendor security practices, certifications, and incident history;
- confirm vendor backup and recovery commitments in contracts;
- understand vendor breach notification obligations and timelines;
- clarify data ownership, portability, and deletion on contract end;
- verify vendor applies patches and maintains supported versions;
- do not assume vendor-hosted means vendor-responsible for your recovery.

Vendor-hosted systems still require the library to understand and verify security and recovery. Get commitments in writing.

### Align Backups And Security With Privacy And Retention

Security and backup practices must align with the library's privacy and retention policies.

Alignment:

- do not back up data longer than retention policy permits, or backups become a privacy liability;
- ensure deleted data is also removed from backups within a defined period where feasible;
- secure backups with the same access controls as production data;
- document how retention applies across primary and backup copies;
- coordinate with privacy policy so backups do not undermine minimization.

Backups that retain sensitive data indefinitely contradict privacy commitments. Align retention across all copies.

## Common Traps

### Relying On A Single Security Control

One layer is never enough. Use defense in depth so a single failure does not cascade.

### Deferred Patches

Postponed patches accumulate into exploitable vulnerability. Establish and follow a patch cadence.

### Untested Backups

Backups never restored are assumptions. Test restoration regularly and measure recovery time.

### No Offline Or Immutable Backup Copy

Online-only backups are vulnerable to ransomware. Keep at least one copy offline or immutable.

### Unrehearsed Incident Response

A plan never exercised fails under pressure. Run tabletop exercises and update the plan.

### Assuming Vendor-Hosted Means Vendor-Responsible

Vendor-hosted systems still require the library to verify security and recovery. Get commitments in writing.

### Backups That Contradict Retention Policy

Retaining deleted data in backups indefinitely undermines privacy. Align retention across primary and backup copies.

### Security As Compliance Theater

Checklist-only security misses real risks. Treat audits as improvement, not box-checking.

## Self-Check

- Are defenses layered across network, host, authentication, application, data, monitoring, and backups, so no single failure cascades?
- Is there a tracked, timely patch process covering all systems, including vendor-hosted services, with critical patches prioritized?
- Is patron and sensitive data encrypted, access-restricted, retention-minimized, and treated with the care owed to regulated personal data?
- Do backups follow the 3-2-1 principle, include data and configuration, and keep at least one offline or immutable copy protected from ransomware?
- Has restoration been tested recently, including full system recovery, with measured recovery time against objectives?
- Is there a written, rehearsed incident response plan covering ransomware, account compromise, and breach, with defined escalation paths?
- Are breach notification obligations understood in advance, with templates and coordination with legal counsel ready?
- Are security and access audited periodically, with findings acted upon rather than filed away?
- Are vendor security, backup, recovery, and breach responsibilities explicit in contracts and verified, not assumed?
- Do backup and security practices align with privacy and retention policies, including removal of deleted data from backups within a defined period?
