---
name: library_system_administration.md
description: Use when the agent is administering, configuring, upgrading, migrating, or evaluating a library management system or integrated library system (ILS), library services platform (LSP), or supporting systems such as link resolvers, knowledge bases, ERM, interlibrary loan, course reserves, or digital asset management, including vendor management, change control, environment management, integration configuration, service continuity, and operational health monitoring.
---

# Library System Administration

Library systems, the ILS or LSP, the discovery layer, link resolver, electronic resource management, interlibrary loan, course reserves, and digital asset platforms, are the operational backbone of the library. When they are healthy, services run invisibly in the background; when they fail, circulation stops, cataloging stalls, e-resource access breaks, and patrons are turned away. Administering these systems is not a casual IT task. It involves vendor relationships, complex configuration, integrations between many products, scheduled upgrades, data migrations, and the responsibility to keep services running through changes. A mistake in system administration, a bad configuration push, a skipped test, an unvetted upgrade, can take down core services for hours or corrupt data that took years to build.

The judgment problem is that library systems are often administered reactively, by whoever has access, under time pressure, with incomplete documentation. Changes are made directly in production because there is no test environment. Upgrades are applied on vendor schedule without local validation. Integrations between the ILS, discovery, link resolver, and authentication are understood by one person who then leaves. Vendor communications about changes, deprecations, or incidents are not tracked. The result is fragile systems that fail unpredictably and are hard to recover. The agent's job is to administer library systems with change discipline, environmental separation, integration awareness, vendor management, and continuity planning, treating system administration as a professional operational practice rather than ad hoc troubleshooting.

Use this skill when administering, configuring, upgrading, migrating, integrating, or evaluating library systems. The goal is to prevent the agent from making untested production changes, skipping documentation, neglecting vendor management, breaking integrations through ignorance, or treating system administration as firefighting rather than disciplined operations.

## Core Rules

### Maintain Separate Test And Production Environments

Production is not the place to test changes. A test or staging environment is essential for safe administration.

Environment separation:

- maintain a test or sandbox instance that mirrors production configuration;
- apply all configuration changes, upgrades, and migrations to test first;
- validate behavior in test with representative data and workflows before promoting to production;
- use vendor sandboxes for integration and API testing;
- never test destructive changes, data loads, or schema migrations directly in production.

Without a test environment, every change is a gamble. The cost of a sandbox is far less than the cost of a production outage.

### Apply Rigorous Change Control

Changes to library systems must be deliberate, reviewed, and reversible. Ad hoc changes cause outages.

Change control practices:

- document every change: what, why, who, when, and how to roll back;
- schedule changes during maintenance windows with advance notice;
- review significant changes with stakeholders before applying;
- maintain a change log accessible to all administrators;
- test the rollback procedure, not just the change;
- avoid stacking multiple untested changes in one window.

Change control is not bureaucracy; it is how a team survives personnel turnover and time pressure without breaking systems.

### Understand And Document Integrations

Library systems do not operate in isolation. They integrate with discovery, authentication, link resolvers, ERMS, learning management systems, and more.

Integration management:

- map all integrations: which systems connect, via what protocols, and what data flows;
- document the owner and contact for each integrated system;
- understand the dependency chain, such as authentication feeding discovery feeding the link resolver;
- test integrations after any change to a connected system;
- monitor integration points for failures, since a silent integration failure degrades service invisibly;
- maintain credentials and configuration for each integration securely and accessibly.

An integration understood by one person is a single point of failure. Document and share integration knowledge.

### Manage Vendor Relationships Actively

Library systems are largely vendor-provided. Vendor management is a core administrative duty.

Vendor management:

- maintain current contacts and support contracts for every system;
- track product roadmaps, deprecations, and end-of-life announcements;
- understand the support process and escalation paths;
- participate in user groups to influence product direction;
- negotiate maintenance windows, service levels, and upgrade timing;
- document vendor commitments and hold vendors accountable;
- plan for migration well before a product is end-of-life.

Passive vendor management leaves the library at the mercy of vendor schedules and decisions. Active management protects service continuity.

### Plan And Validate Upgrades And Migrations

Upgrades and migrations are high-risk events. They require planning and validation.

Upgrade and migration practices:

- review release notes and assess impact before applying;
- test in a sandbox with realistic data and workflows;
- back up data and configuration before any upgrade;
- schedule upgrades in maintenance windows with rollback plans;
- validate critical workflows, circulation, cataloging, discovery, authentication after upgrade;
- communicate changes and potential disruptions to stakeholders in advance;
- avoid upgrading under deadline pressure without testing.

A botched upgrade can corrupt data or break services for days. Treat every upgrade as a project, not a routine click.

### Monitor System Health Proactively

Waiting for users to report problems is reactive. Proactive monitoring catches issues early.

Monitoring practices:

- monitor uptime and response time for critical services;
- set up alerts for failures in integrations, authentication, and batch jobs;
- monitor disk space, database health, and error logs;
- track scheduled jobs such as overnight indexing and notices;
- review vendor status pages and incident reports;
- maintain a known issue list and communicate workarounds.

Proactive monitoring turns outages into minor incidents instead of major service failures.

### Secure Administrative Access

Administrative access to library systems is high-value. It must be protected.

Access security:

- use strong, unique credentials and a password manager or vault;
- require multi-factor authentication for administrative access;
- restrict administrative accounts to those who need them, on a least-privilege basis;
- audit administrative access and changes periodically;
- remove access promptly when staff change roles or leave;
- segregate duties so no single person holds all critical access.

Compromised administrative access can expose patron data and disable services. Protect it accordingly.

### Maintain Data Backups And Recovery

Library system data, bibliographic records, patron accounts, circulation history, and configuration, represents years of work and must be recoverable.

Backup and recovery:

- back up databases and configuration on a regular schedule;
- store backups geographically separated from production;
- test restoration periodically, since an untested backup is an assumption;
- document recovery procedures for each system;
- understand vendor backup responsibilities and the limits of vendor recovery;
- retain backups according to a defined retention policy aligned with privacy requirements.

Backups that are never tested or stored only on-site provide false assurance. Verify recoverability.

### Document Systems And Procedures

Knowledge that lives in one administrator's head is a risk. Documentation is operational insurance.

Documentation to maintain:

- system architecture and integration maps;
- administrative procedures for common and critical tasks;
- vendor contacts, contracts, and support processes;
- change logs and incident postmortems;
- recovery and rollback procedures;
- onboarding materials for new administrators.

Documentation enables continuity through staff turnover and reduces reliance on any single person.

### Plan For Service Continuity And Disaster Recovery

Library systems will fail at some point. Plan for it.

Continuity planning:

- identify critical services and acceptable downtime for each;
- define recovery time and recovery point objectives;
- maintain redundant components where feasible;
- test disaster recovery procedures periodically;
- coordinate with institutional IT and vendors on continuity;
- communicate with stakeholders during incidents.

A continuity plan that exists only on paper is untested. Practice recovery before you need it for real.

## Common Traps

### Making Changes Directly In Production

Without a test environment, every change risks an outage. Always test in a sandbox first.

### Skipping Change Control

Undocumented, unreviewed changes cause outages and are hard to roll back. Document and review every change.

### Single-Person Integration Knowledge

Integrations understood by one person fail when that person leaves. Document and share integration knowledge.

### Passive Vendor Management

Ignoring vendor roadmaps and deprecations leads to forced migrations and broken services. Manage vendors actively.

### Untested Upgrades

Applying upgrades without sandbox testing risks data corruption and outages. Treat upgrades as projects with validation.

### Reactive Monitoring

Waiting for user reports means outages last longer. Monitor proactively with alerts.

### Untested Backups

Backups that are never restored are assumptions. Test recovery periodically.

### Undocumented Systems

Knowledge in one person's head is a continuity risk. Document architecture, procedures, and vendor relationships.

## Self-Check

- Is there a test or sandbox environment where changes, upgrades, and migrations are validated before production?
- Is every change documented with what, why, who, when, and a rollback plan, and reviewed for significant changes?
- Are all system integrations mapped, documented, owned, and tested after connected changes?
- Are vendor relationships actively managed, with current contacts, tracked roadmaps, and migration planning for end-of-life products?
- Are upgrades and migrations planned, tested in a sandbox, backed up, and validated against critical workflows before completion?
- Is system health monitored proactively with alerts for failures, integrations, and batch jobs?
- Is administrative access secured with strong credentials, multi-factor authentication, least privilege, and prompt removal on role change?
- Are data backups regular, geographically separated, tested for restoration, and retained per policy?
- Are systems, procedures, vendor relationships, and recovery steps documented and accessible to the team?
- Is there a tested service continuity and disaster recovery plan with defined objectives for critical services?
