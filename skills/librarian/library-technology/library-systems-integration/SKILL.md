---
name: library_systems_integration.md
description: Use when the agent is integrating library systems, planning data flows between an ILS, discovery layer, link resolver, institutional repository, authentication, and eresources, designing or evaluating APIs and middleware for library systems, managing system migration and data conversion, or troubleshooting interoperability between library technology platforms.
---

# Library Systems Integration

Library systems integration is the work of making distinct platforms, the integrated library system, discovery layer, link resolver, institutional repository, eresource management, authentication, and identity systems, function together as a coherent service. The danger is that integration looks like a technical task of connecting systems and is in fact a judgment-intensive discipline of managing data flows, dependencies, standards, and failure modes across systems owned by different vendors and governed by different logics. Agents tend to treat integration as plumbing, focusing on making the connection work in the happy path, while under-attending to data quality, standards compliance, failure handling, and the long-term maintainability of what is built. A brittle integration that works in the demo but breaks when a vendor changes an API, or that silently drops or corrupts data, is worse than no integration because it provides false assurance. Library systems integration also carries real risk to services: a failed migration or broken data flow can take discovery, circulation, or access offline, and the integration design must account for that risk.

Use this skill when integrating library systems, designing data flows and APIs, planning migrations, or troubleshooting interoperability. The goal is to prevent the agent from building brittle point-to-point integrations, ignoring data quality and standards, under-planning failure handling, or treating integration as connection rather than ongoing maintenance.

## Core Rules

### Map The Full Data Flow Before Building Any Connection

Integration begins with understanding the complete data flow: what data originates where, how it moves, what transformations occur, what systems consume it, and what happens when it changes. Building a connection without this map produces integrations that work for one path while breaking others or creating data inconsistencies. The data flow map is the specification against which integration is designed.

Map the data flow:

- identify each system and the data it holds as system of record for specific elements;
- trace how each data element moves between systems, including the direction and frequency of sync;
- identify transformations, where data is converted between formats, schemas, or vocabularies;
- note the dependencies, where one system's function depends on another's data;
- identify the failure points, where a broken connection or bad data disrupts services.

The map reveals where standards, middleware, and failure handling are needed, and it exposes hidden dependencies that point-to-point connections miss.

### Prefer Standards Over Custom Point-To-Point Connections

Integrations built on library standards, such as SIP2 for circulation, NCIP for circulation and patron APIs, OAI-PMH for harvesting, Z39.50 and SRU for search, OpenURL and the link resolver protocol for resolution, SAML or OIDC for authentication, and IIIF for images, are more maintainable than custom point-to-point connections. Standards-based integrations survive vendor changes, staff turnover, and system replacement better than bespoke code, because the interface contract is documented and shared.

Prefer standards by:

- using established library standards for each integration type rather than custom APIs where a standard exists;
- recognizing where standards are insufficient and custom integration is justified, and documenting that justification;
- evaluating vendor support for standards, because a vendor's partial or non-compliant implementation undermines the benefit;
- avoiding proprietary or undocumented APIs that create lock-in and maintenance risk;
- treating middleware and integration platforms as the place where standards meet, reducing point-to-point sprawl.

Standards-based integration is an investment in maintainability; custom connections are technical debt from day one.

### Treat Data Quality As An Integration Concern

Integration does not fix data quality; it exposes and amplifies it. When systems sync, inconsistent, duplicate, or malformed data propagates across the environment, and the integration is often blamed for problems that originate in the source data. Data quality must be addressed at the source and monitored across the integration, not assumed. This is especially true for bibliographic and authority data, where legacy shortcuts surface during migration or sync.

Address data quality in integration:

- assess source data quality before integration, identifying duplicates, inconsistencies, and encoding issues;
- define data validation at integration points, catching malformed data before it propagates;
- establish data ownership, clarifying which system is authoritative for each data element to prevent conflicting updates;
- plan for deduplication and reconciliation during migrations, which is where legacy problems concentrate;
- monitor data quality after integration, because drift occurs over time.

An integration that moves bad data faster is not an improvement; data quality is a precondition for valuable integration.

### Design For Failure, Because Integrations Fail

Every integration will fail at some point: a vendor API changes, a network connection drops, a data feed sends malformed records, or a system is down for maintenance. Integrations designed only for the happy path fail badly, propagating errors, corrupting data, or taking services offline. Failure handling must be designed into the integration from the start.

Design for failure:

- define what happens when a connection fails: retry, queue, alert, or degrade gracefully;
- prevent partial updates from corrupting data, using transactions or idempotent operations;
- monitor integrations so failures are detected promptly, not discovered when a user complains;
- establish error logging that supports diagnosis, not just detection;
- plan for vendor API changes, which happen, by isolating vendor-specific code and tracking deprecation notices;
- maintain rollback or recovery procedures for migrations and bulk operations.

The measure of integration robustness is not whether it works when everything is fine but how it behaves when something breaks.

### Plan Migrations As Integration Projects, Not Data Moves

System migrations, replacing an ILS, moving to a new discovery layer, or switching repository platforms, are integration projects of the highest risk, because they involve transforming and moving large volumes of data under time pressure, with service continuity at stake. Treating migration as a data move underestimates the transformation, reconciliation, testing, and rollback complexity. Migrations must be planned as integration projects with dedicated analysis, testing, and contingency.

Plan migrations as integration:

- analyze the source and target schemas and the transformation required, which is where most migration effort lives;
- plan data cleanup before migration, because migrating dirty data multiplies the problems;
- design a testing strategy that verifies data integrity and service function after migration, not just that data arrived;
- plan a rollback path, because some migrations must be reversed and the ability to do so safely is essential;
- schedule migrations to minimize service disruption, with clear communication to stakeholders;
- allocate realistic time, because migrations consistently take longer than the data volume suggests.

A migration without analysis, testing, and rollback planning is a gamble with core services.

### Manage Vendor APIs And Dependencies Actively

Modern library systems depend heavily on vendor APIs and cloud services, and these dependencies carry risks that must be managed actively. Vendors deprecate APIs, change authentication, rate-limit, or alter behavior, and unmanaged dependencies break without warning. The institution must track its dependencies, monitor for changes, and maintain the capacity to adapt.

Manage vendor dependencies:

- inventory the APIs and services the institution depends on, with versions and authentication methods;
- track vendor deprecation and change notices, and plan adaptations before deadlines;
- isolate vendor-specific code so changes have limited blast radius;
- maintain documentation of each dependency sufficient for a successor to manage it;
- evaluate vendor API quality and stability when selecting systems, not only features;
- negotiate API access and uptime terms in contracts where possible.

Dependencies that are not tracked and managed break at the worst time; active dependency management is ongoing integration work.

### Treat Integration As Ongoing Maintenance, Not A Project

Integration is not a project that completes; it is an ongoing operational responsibility. Systems update, vendors change APIs, data evolves, and new systems are added, and the integration environment must be maintained continuously. Treating integration as a one-time build leaves the institution with decaying connections that fail unpredictably. Integration must be staffed and sustained.

Sustain integration:

- assign ownership for each integration, with someone responsible for monitoring and maintenance;
- schedule regular review of integrations, APIs, and data flows;
- budget for integration maintenance as operational, not project, expense;
- document integrations thoroughly so they survive staff turnover;
- build monitoring and alerting into the routine, not as an afterthought.

## Common Traps

### Point-To-Point Connection Sprawl

Building custom connections between every pair of systems creates unmaintainable spaghetti. This is a trap because each connection is independent debt that compounds.

### Ignoring Data Quality Until Migration

Deferring data cleanup until a migration surfaces all legacy problems under time pressure. This is a trap because the cleanup is harder and riskier when rushed.

### Happy-Path-Only Integration

Designing integrations that work when everything is fine but fail badly under errors. This is a trap because failures propagate and corrupt data.

### Assuming Standards Compliance

Trusting that vendors implement standards fully, when partial compliance undermines interoperability. This is a trap because the assumption breaks under real use.

### Migration Without Testing Or Rollback

Migrating data without verifying integrity or planning reversal gambles with services. This is a trap because a failed migration without rollback is a crisis.

### Untracked Vendor Dependencies

Relying on vendor APIs without inventorying or monitoring them leads to surprise breakages. This is a trap because the failure is unexpected and the fix is urgent.

### Integration As Completed Project

Treating integration as done after launch leaves decaying connections. This is a trap because the decay is invisible until a failure.

### Proprietary Lock-In Without Exit Path

Building on proprietary APIs or formats without an exit strategy traps the institution. This is a trap because switching costs become prohibitive and the vendor gains leverage.

## Self-Check

- Did you map the full data flow, including origins, transformations, dependencies, and failure points, before building connections?
- Are integrations built on established library standards (SIP2, NCIP, OAI-PMH, Z39.50, OpenURL, SAML, IIIF) rather than custom point-to-point connections where standards exist?
- Is data quality assessed at the source, validated at integration points, and monitored after integration?
- Does the integration design handle failure explicitly, with retry, queueing, alerting, error logging, and graceful degradation?
- Are migrations planned as integration projects with schema analysis, data cleanup, testing, rollback, and realistic timelines?
- Are vendor APIs and dependencies inventoried, monitored for changes, isolated, and documented?
- Is integration treated as ongoing operational maintenance with assigned ownership, scheduled review, and operational budget?
- Are proprietary dependencies evaluated for exit paths and lock-in risk during system selection?
- Is integration documentation sufficient to survive staff turnover?
