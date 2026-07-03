---
name: service-organization-reports.md
description: Use when the agent is using a service organisation's SOC 1 or equivalent report, evaluating the scope and period covered, assessing complementary user entity controls, determining whether the report supports reliance on the service organisation's controls, designing user entity procedures where the report is insufficient, or applying the service auditor's results to the user auditor's risk assessment.
---

# Service Organization Reports

Many entities use service organisations to process transactions, hold data, or perform functions material to their financial statements — payroll providers, custodians, fund administrators, cloud platforms, payment processors. The controls at those service organisations are part of the user entity's control environment, but the user auditor cannot test them directly. Service organisation reports (SOC 1 / ISAE 3402 / equivalent) provide a way to gain assurance about those controls, assessed by a service auditor. The discipline is to evaluate whether the report actually covers the controls, the period, and the risks relevant to the user entity, to identify and test the complementary user entity controls the report assumes, and to design additional procedures where the report leaves gaps.

## Core Rules

### Confirm the report covers the services and processes material to the user entity

A service organisation may offer many services, but only some are material to the user entity's financial statements. Before relying on a report, confirm:

- the services described include those the user entity actually uses (not just services the organisation offers);
- the in-scope services cover the processes and data that affect the user entity's significant accounts;
- the systems and locations in scope are those that actually process the user entity's transactions.

A report that covers the service organisation's general operations but not the specific service the user entity uses provides no relevant assurance. Read the system description carefully to confirm scope alignment.

### Confirm the report covers the relevant period

The report's period must cover the user entity's audit period, or at least the periods material to the user entity's assertions:

- a type II report covers design and operating effectiveness over a stated period; confirm that period overlaps the user entity's period of reliance;
- gaps between the report period and the user entity's period (e.g., the report ended three months before year-end) require additional procedures for the gap;
- a type I report covers design only at a point in time and does not support operating effectiveness across a period.

Match the report type and period to the assertion and the period of reliance; a mismatched period is a coverage gap.

### Identify and test the complementary user entity controls

Service organisation reports almost always assume that the user entity implements certain controls — complementary user entity controls — without which the service organisation's controls cannot achieve the control objectives. Examples:

- the user entity authorises users and access to the service;
- the user entity reviews exception reports provided by the service organisation;
- the user entity reconciles the service organisation's output to its own records;
- the user entity controls the data it sends to the service organisation.

Identify these controls from the report, confirm the user entity has implemented them, and test their operation. A service organisation's effective controls provide no assurance if the complementary user entity controls on which they depend are absent or not operating.

### Evaluate the service auditor's opinion and the control objectives

Read the report's opinion and the control objectives critically:

- did the service auditor express an unqualified opinion, or were there qualifications or exceptions?
- do the control objectives cover the risks relevant to the user entity, or are there gaps?
- for any exceptions noted, what was the effect, and does it affect the user entity's reliance?

A report with exceptions is not automatically unusable, but the exceptions must be evaluated for their effect on the specific controls and risks relevant to the user entity. A control objective that the user entity needs but that is not covered by the report is a gap requiring user entity procedures.

### Assess the suitability of the criteria and the service auditor's work

The report's value depends on the criteria being suitable and the service auditor's work being adequate. Consider:

- are the criteria (the framework the service auditor used) appropriate for the user entity's purposes?
- is the service auditor independent and competent?
- does the report describe the tests performed and the results, allowing the user auditor to evaluate their relevance and sufficiency?

A report that states conclusions without describing the tests performed is harder to evaluate; the user auditor needs enough detail to judge whether the service auditor's procedures support the conclusions relevant to the user entity.

### Design user entity procedures where the report is insufficient

Where the report does not cover a relevant service, period, control objective, or where exceptions undermine reliance, design procedures at the user entity to obtain the needed assurance:

- test the user entity's reconciliation of the service organisation's output to its own records;
- test transactions processed by the service organisation through user-entity-held source documents;
- obtain a bridging report or supplementary information for period gaps;
- in some cases, request that the service organisation obtain an extended or additional report.

The report is one input; where it leaves gaps, the user auditor's own procedures must fill them. Do not conclude based on a report that does not cover the relevant risk.

### Apply the report's results to the risk assessment and testing strategy

Use the report's conclusions to calibrate the user audit:

- where the report supports effective controls at the service organisation (and complementary user entity controls are tested), the user auditor may reduce substantive testing of the affected processes;
- where the report identifies exceptions or gaps, raise the assessed risk and expand substantive testing or user entity procedures accordingly;
- where no report is available for a material service organisation, treat the service organisation's controls as unknown and design procedures accordingly, potentially including requesting a report or accessing the service organisation.

The report is an input to the risk assessment, not a substitute for it; its conclusions must be translated into the user audit's scope and approach.

### Consider the subservice organisations

Service organisations often use subservice organisations (a payroll provider using a data centre, a fund administrator using a custodian). Confirm whether the report covers the subservice organisations or uses the carve-out (or inclusive) method:

- under the carve-out method, the subservice organisation's controls are not covered; the user entity must consider whether those controls are relevant and whether a report or other procedures are needed for them;
- under the inclusive method, the subservice organisation's controls are included in the report's scope.

A carve-out for a material subservice organisation is a gap; follow the chain of service organisations to confirm coverage of each material link.

### Document the evaluation and the effect on the audit

For each service organisation report used, document:

- the services used by the user entity and their materiality;
- the report's scope, period, type, opinion, and control objectives, and their alignment with the user entity's needs;
- the complementary user entity controls identified and the testing of their operation;
- the exceptions or gaps and their evaluation;
- the additional user entity procedures performed where the report was insufficient;
- the effect on the risk assessment and the testing strategy.

This documentation is what makes the reliance defensible. A conclusion that relies on a report without documenting the scope, period, complementary controls, and gaps cannot be reviewed or relied upon.

## Common Traps

- **Assuming the report covers the services the user entity uses** without reading the system description to confirm scope alignment.
- **Using a report whose period does not cover the user entity's period of reliance**, leaving a coverage gap.
- **Overlooking complementary user entity controls** that the report assumes, and that must be tested at the user entity for the service organisation's controls to be effective.
- **Treating a report with exceptions as fully usable** without evaluating the exceptions' effect on the specific controls and risks relevant to the user entity.
- **Concluding reliance on a report that does not cover a control objective the user entity needs**, leaving a risk unaddressed.
- **Failing to design user entity procedures where the report is insufficient**, leaving the assurance gap unfilled.
- **Ignoring subservice organisations** under the carve-out method, missing a material link in the service chain.
- **Using a type I report (design only) to support operating effectiveness across a period**, which it cannot do.
- **Treating the report as a substitute for the risk assessment**, rather than as an input that must be translated into scope and approach.
- **Failing to document the scope, period, complementary controls, exceptions, gaps, and additional procedures**, leaving the reliance indefensible.

## Self-Check

- Does the report's system description cover the specific services, systems, and locations the user entity uses, not just the services the organisation offers?
- Does the report's period overlap the user entity's period of reliance, and have I addressed any gap with additional procedures?
- Did I identify the complementary user entity controls the report assumes, and test their operation at the user entity?
- Did I read the opinion and exceptions critically, evaluating their effect on the controls and risks relevant to the user entity?
- Do the report's control objectives cover the risks the user entity needs addressed, or are there gaps requiring user entity procedures?
- Where the report is insufficient, did I design user entity procedures (reconciliations, transaction testing, bridging reports) to fill the gap?
- Did I apply the report's conclusions to calibrate the risk assessment and testing strategy — reducing testing where controls are effective, expanding where they are not?
- Did I consider subservice organisations under the carve-out method and obtain coverage or procedures for material links?
- Did I use a type II report (design and operating effectiveness over a period) where I need operating effectiveness, not a type I?
- Is the scope, period, complementary controls, exceptions, gaps, additional procedures, and effect on the audit documented so the reliance is defensible?
