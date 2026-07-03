---
name: it_environment_and_systems_understanding.md
description: Use when the agent is understanding an entity's IT environment for an audit, mapping financial systems and interfaces, assessing the use of service organizations and cloud platforms, evaluating the reliability of system-generated reports, or scoping IT general controls and automated controls before relying on system data.
---

# IT Environment And Systems Understanding

Modern entities run on systems. Financial figures, controls, reconciliations, and reports are produced by applications, interfaces, spreadsheets, and cloud services. If the auditor does not understand the IT environment, the auditor cannot judge whether the numbers and the controls built on them are reliable. A report that looks complete can be missing rows because of a broken interface. An automated control that looks effective can be silently bypassed by an administrator. A spreadsheet that bridges two systems can contain errors that propagate into the general ledger. Agents often treat IT understanding as a side note, gather a list of application names, and then rely on system output without ever testing whether the output is complete and accurate.

Use this skill before relying on system-generated reports, before relying on automated or IT-dependent controls, before assessing IT general controls, and whenever the audit depends on data extracted from the entity's systems. The goal is to determine where the financial information comes from, how it is processed and protected, and what could cause it to be wrong.

## Core Rules

### Map The Financial Systems Landscape

Begin by identifying every system that materially affects the financial statements, not only the core general ledger.

Map:

- the general ledger and consolidation system;
- subledgers for revenue, payables, payroll, inventory, and fixed assets;
- billing, order management, and point-of-sale systems;
- procurement and expense systems;
- treasury and banking platforms;
- budgeting, forecasting, and reporting tools;
- spreadsheets and end-user computing tools used in the close;
- consolidation and statutory reporting systems;
- data warehouses and lakes feeding reporting;
- interfaces and integrations between these systems.

For each system, record its name, vendor or in-house status, hosting model, the financial processes it supports, and its materiality to the statements. A system that feeds a material balance or disclosure is in scope for understanding and, where relied upon, for testing.

### Understand Hosting, Deployment, And Ownership

Where and how a system runs affects the controls available and the evidence the auditor can obtain.

Determine for each system:

- on-premise, private cloud, public cloud, or hybrid;
- vendor-managed software as a service or self-managed;
- which entity or third party is responsible for operations and support;
- data center locations and any cross-border data movement;
- multi-tenant versus dedicated environment;
- version, patch level, and upgrade history;
- end-of-life or unsupported software in use;
- disaster recovery and backup arrangements.

Unsupported or end-of-life systems raise security and integrity risk. Cloud and vendor-managed systems shift some controls to the service provider, which may require service organization reports.

### Identify And Assess Service Organizations

Many entities depend on external service providers for core processing. The auditor must understand this dependency and obtain evidence about the provider's controls where the entity's controls are not sufficient on their own.

For each service organization:

- identify the services provided and the processes affected;
- determine whether the service is material to the financial statements;
- obtain and review the service organization's control report, such as a type 1 or type 2 report;
- assess whether the report covers the relevant period, scope, and trust services or control criteria;
- identify complementary user entity controls the entity must perform;
- evaluate the effect of any qualified or modified opinion in the report;
- consider the need for a service auditor's or user auditor procedures when no report exists or scope is limited.

Do not assume a service organization is reliable because it is well known. Obtain and read the report, and confirm the entity is actually performing the complementary controls the report assumes.

### Evaluate The Reliability Of System-Generated Reports

Controls and substantive procedures routinely depend on reports pulled from systems. If the report is wrong, everything built on it is wrong.

For each report relied upon:

- understand the source data and tables behind it;
- review the report logic, parameters, and filters;
- identify whether it is standard, customized, or ad hoc;
- determine who can modify the report definition;
- test completeness by reconciling the report total to the system or ledger;
- test accuracy by re-performing selected calculations or tracing items to source records;
- consider whether the report captures all relevant transactions for the period;
- assess whether manual adjustments are applied after extraction.

A report that cannot be reconciled to an independent total should not be relied upon without further testing. Document the completeness and accuracy procedures performed.

### Understand Automated And IT-Dependent Controls

Many key controls are automated or depend on IT. Understanding them requires knowing how they are configured and what could make them fail.

For each automated or IT-dependent control:

- identify the risk or assertion it addresses;
- understand the configuration, rule, or algorithm;
- determine who can change the configuration and how changes are controlled;
- assess whether the control runs without human intervention or requires a person to act on its output;
- identify the evidence it produces, such as logs, exception reports, or block messages;
- consider what happens when the control fails, such as whether it fails open or closed;
- evaluate dependency on accurate master data, such as approval limits or vendor records.

An automated control is only as reliable as the configuration and the access controls that protect it. If anyone can silently change the rule, the control can be overridden without a trace.

### Scope The Relevant IT General Controls

IT general controls underpin every automated control and system-generated report. Weak general controls undermine reliance on application controls.

Assess general controls across:

- logical access, including user provisioning, privileged access, authentication, and periodic access reviews;
- change management, including how application and configuration changes are requested, tested, approved, and migrated;
- computer operations, including job scheduling, backup, recovery, and incident handling;
- program development and acquisition, including testing and authorization of new systems;
- interface controls, including reconciliation and error handling between systems;
- data management, including completeness and accuracy of data extracts and conversions.

Focus on the general controls that support the specific systems and reports being relied upon. A blanket statement that general controls are adequate is not support without evidence tied to the in-scope systems.

### Assess End-User Computing And Spreadsheets

Spreadsheets and end-user tools frequently perform material calculations in the close, in estimates, and in consolidations. They are often poorly controlled.

For material end-user tools:

- identify the tool, its owner, and the calculation it performs;
- understand the inputs, formulas, and logic;
- assess the risk of error in logic, links, or hardcoded values;
- evaluate version control and change history;
- test input completeness and accuracy;
- test the calculation by re-performance or independent recalculation;
- consider access controls and whether the file is protected.

A complex, unprotected spreadsheet that drives a material estimate is a significant risk that requires specific testing, not a footnote.

### Consider Cybersecurity, Availability, And Data Integrity

The integrity of financial data depends on the security and availability of the systems that hold it. A breach, ransomware event, or data corruption incident can affect completeness, accuracy, and existence.

Understand:

- significant security incidents during the period;
- availability events, outages, or data losses;
- data integrity issues, corruption, or conversion errors;
- privileged or shared accounts with broad access;
- segregation between development, test, and production environments;
- logging and monitoring of sensitive transactions and access;
- backup and recovery testing results.

Where incidents occurred, assess their impact on the financial statements and on the reliability of controls and reports, and consider whether additional procedures or disclosure evaluation is needed.

### Link IT Understanding To Audit Risk And Procedures

The IT understanding is not a standalone artifact. It must inform risk assessment and the design of procedures.

Use the understanding to:

- decide which reports need completeness and accuracy testing;
- decide which automated controls can be relied upon and must be tested;
- decide whether to rely on service organization reports and complementary controls;
- identify where general control weaknesses prevent reliance on application controls;
- identify where spreadsheets require targeted testing;
- adjust the nature, timing, and extent of substantive procedures based on IT risk;
- flag areas where data integrity concerns require expanded testing.

Document the link explicitly so that IT risk flows into the audit response rather than sitting in a separate file.

## Common Traps

### Listing Applications Without Understanding Materiality

A system inventory is not an understanding. The auditor must know which systems materially affect the statements and focus effort there.

### Relying On Reports Without Completeness And Accuracy Testing

Pulling a report and using it for sampling or reconciliation, without testing that the report is complete and accurate, builds the audit on unverified data.

### Assuming Cloud Or Vendor Systems Are Inherently Reliable

Well-known platforms still have configuration errors, scope gaps, and failed complementary controls. Obtain and read the service organization report and confirm user entity controls operate.

### Ignoring Spreadsheets Because They Are Not Core Systems

Material estimates, consolidations, and allocations often live in spreadsheets. Treating them as out of scope leaves a major error source untested.

### Treating Automated Controls As Self-Evidently Effective

An automated control is only effective if the configuration is correct, protected from unauthorized change, and supported by adequate general controls. Test the configuration and the access around it.

### Overlooking Privileged Access And Override

Administrators and shared accounts can bypass application controls silently. If the understanding does not address privileged access, override risk is unaddressed.

### Forgetting Interfaces And Data Conversions

Errors often enter at interfaces and during data migrations or conversions. Unreconciled interfaces are a prime source of undetected completeness and accuracy errors.

### Disconnecting IT Understanding From Substantive Procedures

A thorough IT memo that never changes the nature or extent of testing is wasted. IT risk must flow into the audit response.

## Self-Check

- Is every system that materially affects the financial statements identified, with hosting, ownership, and materiality recorded?
- Are service organization dependencies identified, with control reports obtained, reviewed for period and scope, and complementary user entity controls confirmed as operating?
- For each report relied upon, have completeness and accuracy procedures been performed and documented, including reconciliation to an independent total?
- Are automated and IT-dependent controls understood as to configuration, change control, evidence, failure behavior, and master-data dependency?
- Are relevant IT general controls, including access, change management, operations, interfaces, and data management, scoped to the in-scope systems and assessed with evidence?
- Are material spreadsheets and end-user computing tools identified and tested for logic, inputs, version control, and access?
- Have cybersecurity incidents, availability events, and data integrity issues been considered for their impact on statements, controls, and reports?
- Does the IT understanding explicitly link to risk assessment and to the nature, timing, and extent of planned procedures?
- Is the understanding refreshed for system changes, migrations, conversions, and new service providers during the period?
- Is there no reliance on system output, automated controls, or service organizations that has not been supported by tested evidence?
