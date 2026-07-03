---
name: system_landscape_mapping.md
description: Use when the agent is mapping the entity's system landscape, identifying financial reporting systems and their roles, understanding which systems affect material balances and disclosures, assessing complexity and reliance on third-party or cloud platforms, or determining which IT systems warrant deeper evaluation in the audit.
---

# System Landscape Mapping

The financial statements are produced by systems, and the auditor cannot assess risk or design procedures without knowing which systems affect material balances. System landscape mapping is the act of identifying the systems that matter for financial reporting, understanding what each does, and determining where the IT risk actually lives. Agents often accept a high-level list of applications from IT management without probing which systems feed the ledger, which are third-party or cloud-hosted, or which carry complex configurations that affect recognition and measurement. The result is an audit that treats "the system" as a black box and misses the specific systems where errors, manipulation, or IT general control failures would create material misstatement.

Use this skill during planning and entity understanding, when identifying the systems that affect financial reporting, and when deciding which systems warrant deeper IT evaluation. The goal is a complete, prioritized map of the financial reporting system landscape that drives IT risk assessment and procedure design.

## Core Rules

### Identify Every System That Affects Material Balances And Disclosures

Begin by enumerating the systems that initiate, record, process, or report transactions affecting material balances. The map must be complete, because an omitted system is an unassessed risk.

Identify systems that:

- initiate transactions, such as order entry, procurement, or payroll;
- process and post transactions, such as sub-ledgers and the general ledger;
- perform calculations or allocations, such as revenue, depreciation, or inventory costing;
- hold master data that drives processing, such as customer, vendor, or pricing data;
- produce reports used by controls or by management;
- feed disclosures, such as segment, related-party, or fair-value reporting tools;
- support estimates, such as valuation models or actuarial systems.

Reconcile the list to the financial statements: for each material account and disclosure, identify which system produces the number. An account with no identified source system has a gap in the map.

### Determine The Role And Materiality Of Each System

Not every system matters equally. Prioritize by determining each system's role in financial reporting and the materiality of what it affects.

Classify each system by:

- whether it is financial or operational only;
- the materiality of the balances and transactions it affects;
- whether it feeds the general ledger directly or through another system;
- whether it performs recognition, measurement, or classification decisions;
- whether it holds master data that materially affects processing;
- whether it produces reports relied on by controls or management.

Focus deeper evaluation on systems that affect material balances, perform significant processing, or are relied on by controls. A system that only supports operational tracking with no ledger impact is lower priority.

### Capture The Hosting And Architecture Model

How a system is hosted and architected changes the IT risk profile and the controls available. Capture the model for each material system.

Capture:

- whether the system is on-premise, cloud-hosted, or software-as-a-service;
- whether it is a packaged application, customized, or bespoke;
- whether it is a single instance or distributed across locations;
- the database and infrastructure it relies on;
- whether the entity or a third party operates and controls the environment;
- whether the system is shared with or operated by a service organization.

Cloud and service-organization models shift control responsibility to the provider and introduce the need for service organization reports. On-premise bespoke systems introduce development and change-control risk.

### Identify Third-Party And Service Organization Reliance

Many entities rely on third parties to process, host, or support financial systems. This reliance is itself a risk that must be identified and assessed.

Identify:

- service organizations that process transactions or host systems affecting material balances;
- third parties that provide data feeds, valuations, or calculations;
- business process outsourcers handling payroll, billing, or reconciliation;
- banks or custodians holding or reporting assets;
- cloud and infrastructure providers hosting financial applications.

For each material service organization, determine whether a service organization report is available and how its controls will be assessed. Reliance on a service organization without assessing its controls is an unassessed risk.

### Assess Complexity And Configuration Risk

Some systems are simple and stable; others carry complex configurations that affect recognition and measurement. Complexity concentrates IT risk.

Assess complexity by considering:

- complex pricing, revenue recognition, or allocation rules that are configured;
- estimates or calculations embedded in the system, such as depreciation or inventory costing;
- interfaces and data transformations between systems;
- customizations or bolt-ons to packaged applications;
- frequent configuration changes during the period;
- reliance on spreadsheets or end-user computing tools for material calculations.

Complex configurations warrant evaluation of how the rules work, how they are changed, and how errors would be detected. A complex system treated as a simple one hides configuration-driven misstatement.

### Map Interfaces And Data Flows Between Systems

The boundaries between systems are where data is transferred, transformed, and sometimes lost or altered. Interface risk is a distinct category of IT risk.

Map interfaces by identifying:

- which systems exchange data and in which direction;
- whether transfers are real-time, batch, or manual;
- whether data is transformed, mapped, or re-keyed at the interface;
- whether interface failures or exceptions are detected and resolved;
- whether reconciliations exist between systems for transferred data;
- whether manual adjustments are posted to correct interface issues.

An interface that drops, duplicates, or mis-maps transactions can create material completeness or accuracy errors that no single system's controls would catch.

### Document The Landscape As The Basis For IT Risk Assessment

The system map is the foundation for IT risk assessment and for deciding where to focus IT general controls and application controls testing. It must be documented clearly.

Document:

- the list of financial reporting systems and their roles;
- the materiality of what each system affects;
- the hosting and architecture model;
- third-party and service organization reliance;
- complexity and configuration risk;
- interfaces and data flows;
- the systems selected for deeper evaluation and why.

A system landscape that exists only in IT's slides cannot support audit decisions. The documented map must allow the auditor to explain which systems affect which balances and where the IT risk concentrates.

## Common Traps

### Accepting A High-Level Application List Without Probing

A list of applications from IT, without reconciling to material balances, omits systems that affect the financial statements and includes systems that do not matter.

### Treating The System As A Black Box

Failing to understand what each system does, how it is configured, and where it carries complexity leaves configuration-driven misstatement risk unassessed.

### Overlooking Third-Party And Service Organization Reliance

Systems hosted or operated by third parties shift control responsibility. Reliance without assessing the provider's controls is an unassessed risk.

### Ignoring Interface And Data Flow Risk

The boundaries between systems are where data is lost, duplicated, or altered. A map that does not capture interfaces misses a distinct category of IT risk.

### Failing To Prioritize By Materiality

Treating all systems uniformly wastes effort on immaterial systems and under-covers the systems that affect material balances. Prioritize by role and materiality.

### Missing Spreadsheets And End-User Computing Tools

Spreadsheets that perform material calculations are part of the system landscape. Omitting them hides a category of manual intervention and configuration risk.

### Leaving The Map Undocumented Or Undated

A system landscape that is not documented and dated cannot support IT risk assessment and becomes stale as systems change.

## Self-Check

- Is every system that affects material balances and disclosures identified, with the list reconciled to the financial statements?
- Is each system classified by role and materiality, with deeper evaluation focused on systems affecting material balances and significant processing?
- Is the hosting and architecture model captured, including on-premise, cloud, and service-organization arrangements?
- Is third-party and service organization reliance identified, with a plan for assessing provider controls through reports or other procedures?
- Is complexity and configuration risk assessed, including pricing rules, embedded calculations, customizations, and frequent changes?
- Are interfaces and data flows mapped, with interface failure, transformation, and reconciliation risk considered?
- Are spreadsheets and end-user computing tools that perform material calculations included in the landscape?
- Is the documented, dated map sufficient to explain which systems affect which balances and where IT risk concentrates, as the basis for IT risk assessment?
