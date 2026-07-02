---
name: data_localization_and_residency_requirements.md
description: Use when the agent is designing data storage architecture to meet localization rules in Russia, China,India, the EU, or sector-specific regimes, assessing government access risk, or determining where personal data must physically reside and be processed.
---

# Data Localization And Residency Requirements

Data localization is the obligation to store or process certain personal data within a specific country, and it is spreading. Russia, China, India, the EU, and many sector regimes impose residency duties that override commercial cloud preferences. The operational challenge is that localization is often buried in broad laws, applies to specific data categories, and interacts awkwardly with cross-border transfer mechanisms. A global architecture that ignores localization can be lawful in one country and unlawful in another for the same dataset.

Use this skill before designing storage architecture for a new market, configuring cloud regions, or assessing whether a regulator or contract requires local residency. The goal is to make the agent identify localization duties by jurisdiction and data type, design architecture that satisfies them, and distinguish genuine localization from transfer obligations that merely need a mechanism.

## Core Rules

### Distinguish Localization From Transfer Mechanisms

Localization is a hard residency duty: the data must be stored or processed in a specific country. Transfer obligations, by contrast, allow cross-border movement if a valid mechanism exists. Confusing the two leads to either over-engineering or non-compliance.

Distinguish by:

- localization requires in-country storage or processing, often of primary or specific data categories;
- transfer obligations permit movement subject to mechanisms such as SCCs or adequacy;
- some regimes layer both: residency for certain data plus transfer rules for the rest.

Identify which duty applies before designing architecture. Applying a transfer mechanism to data that must be localized does not cure the breach.

### Identify Localization Regimes By Jurisdiction

Localization duties vary widely in scope, trigger, and enforcement. Map each market where the organization operates.

Survey common regimes:

- Russia: primary storage of certain personal data of Russian citizens on databases located in Russia;
- China: the Personal Information Protection Law and Cybersecurity Law, with critical information infrastructure operators and large-volume processors subject to security assessment before cross-border transfer, plus sector-specific data residency;
- India: the Digital Personal Data Protection Act and RBI and sector rules requiring local storage of certain payments and financial data;
- EU: no general localization, but sector rules and adequacy dynamics can push toward EU or EEA residency;
- other markets: many countries impose sector-specific localization in health, finance, telecoms, and government data.

Treat published regulator guidance as the operative source, because localization rules change and draft rules sometimes become enforceable before finalization.

### Scope Localization To The Required Data Categories

Localization rarely applies to all personal data. It usually targets specific categories such as primary records, financial data, health data, or data of local residents.

Scope by:

- identifying which data categories trigger the duty (for example, primary customer records versus derived analytics);
- determining whether the duty applies to data of local residents only or to all data processed in-country;
- separating data that must be localized from data that may be transferred under a mechanism;
- designing data segregation so localized data does not mix freely with transferable data.

Architecting one global dataset that includes localized data defeats the segregation the law expects.

### Design Architecture For Genuine Residency

Residency must be real, not nominal. Storing data in a local region while replicating it abroad, or routing it through foreign support, can defeat the residency claim.

Design residency by:

- selecting cloud regions or data centers physically in the required country;
- configuring primary storage and processing in-country;
- controlling replication, backups, and disaster recovery so they do not silently move data abroad;
- restricting remote access by foreign personnel that would constitute a transfer undermining residency;
- using local support models or tightly controlled access where foreign access is unavoidable.

A local database with nightly backups to a foreign region is not genuine residency for the backed-up data.

### Manage Sector-Specific Localization

Many localization duties are sector-specific rather than general. Financial, health, telecom, and government data frequently carry their own residency rules.

Address sector rules by:

- mapping sector regulators and their data residency requirements;
- applying stricter residency where sector rules exceed general privacy law;
- coordinating with sector compliance teams (banking, health, telecom) to avoid gaps;
- documenting the sector basis for each residency decision.

### Assess Government Access Risk In Residency Choices

Choosing where to localize is also a choice about whose government can access the data. Residency in one country can create surveillance exposure that triggers transfer concerns elsewhere.

Assess government access by:

- considering the surveillance and government access laws of the residency country;
- evaluating whether residency increases exposure for data subjects from other jurisdictions;
- balancing localization duties against transfer impact assessment concerns for data of foreign nationals;
- documenting the trade-off and any mitigations.

Localizing to comply with one country while exposing the data to that country's surveillance can create a new problem for data subjects elsewhere.

### Handle Conflicts Between Localization And Transfer Law

A dataset may be subject to localization in one country and transfer restrictions from another. Conflicts require explicit resolution.

Resolve conflicts by:

- segregating data by jurisdiction so conflicting duties attach to different datasets;
- minimizing the data subject to conflicting duties;
- seeking regulator guidance or approvals where conflict is unavoidable;
- documenting the conflict and the resolution chosen.

### Document And Monitor Residency Compliance

Residency compliance must be demonstrable. Regulators and customers may ask where data is stored and processed.

Document by:

- recording the residency basis, location, and data categories for each in-country store;
- maintaining architecture diagrams that show where data resides and replicates;
- monitoring for configuration drift that moves data out of region;
- reviewing residency when cloud topology, vendors, or support models change.

## Common Traps

### Confusing Transfer Mechanisms With Localization

Applying SCCs to data that must be localized does not satisfy a residency duty, and over-localizing data that only needs a transfer mechanism wastes resources.

### Nominal Residency Undermined By Replication

Storing primary data in-country while replicating backups or analytics abroad defeats the residency claim.

### Ignoring Sector-Specific Rules

General privacy law may permit transfer while sector rules (banking, health, telecom) require residency. Missing the sector rule leaves a gap.

### Remote Access That Defeats Residency

Foreign support teams accessing localized data can constitute a transfer that undermines the residency arrangement.

### Treating Draft Rules As Non-Binding

Some markets enforce draft or interim localization rules. Waiting for finalization can result in exposure.

### Mixing Localized And Transferable Data

A single global dataset that includes localized data prevents the segregation the law expects.

### Ignoring Government Access Trade-Offs

Localizing to satisfy one country without assessing that country's surveillance exposure can harm data subjects from other jurisdictions.

### Configuration Drift After Initial Design

An architecture that is compliant at launch can drift as teams add regions, replicas, or integrations, silently moving data abroad.

## Self-Check

- Has each applicable localization duty been distinguished from transfer obligations and identified by jurisdiction?
- Are the data categories that trigger localization scoped precisely, separating localized data from transferable data?
- Is residency genuine, with primary storage, processing, backups, and disaster recovery configured in-country?
- Are sector-specific localization rules (finance, health, telecom, government) mapped and applied where they exceed general privacy law?
- Has government access risk in the residency country been assessed for its effect on data subjects from other jurisdictions?
- Are conflicts between localization and transfer law resolved through segregation, minimization, or regulator guidance?
- Is remote access by foreign personnel controlled so it does not defeat residency?
- Are residency basis, location, and data categories documented with current architecture diagrams?
- Is configuration drift monitored to ensure data remains in the required regions over time?
- Is residency compliance reviewed when cloud topology, vendors, support models, or laws change?
