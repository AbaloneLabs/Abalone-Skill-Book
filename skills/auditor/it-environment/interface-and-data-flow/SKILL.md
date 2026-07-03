---
name: interface_and_data_flow.md
description: Use when the agent is evaluating interfaces and data flows between systems, assessing completeness and accuracy of data transfers, testing interface controls and reconciliations, identifying data transformation and mapping risk, or determining whether data moving between systems could be lost duplicated or altered.
---

# Interface And Data Flow

Interfaces are the connective tissue of a modern financial reporting environment, and they are where data integrity is most fragile. Every time data moves between systems, it can be lost, duplicated, delayed, transformed incorrectly, or mapped to the wrong account. Agents often test the systems on either side of an interface but never test the interface itself, assuming that if both systems look right, the transfer must be right. That assumption fails silently: a revenue system and a general ledger can both appear correct while the interface between them drops a class of transactions or maps them to the wrong place. Interface and data flow risk is a distinct category that must be assessed and tested on its own.

Use this skill when evaluating interfaces between financial systems, when assessing the completeness and accuracy of data transfers, and when designing procedures over data that moves between systems. The goal is to confirm that data arriving in one system completely and accurately reflects what left another, and that interface failures are detected and corrected.

## Core Rules

### Identify Every Interface That Affects Material Balances

Begin by enumerating the interfaces that move data affecting material balances. The list should follow from the system landscape map.

Identify interfaces that:

- transfer transactions from a sub-ledger to the general ledger;
- move data between operational and financial systems;
- consolidate data from multiple locations or entities;
- import data from third parties, such as banks or pricing services;
- export data to reporting or disclosure tools;
- feed valuation, allocation, or calculation engines.

For each material interface, document the source and target systems, the data transferred, the frequency, and the mechanism. An interface that affects a material balance but is not identified is an unassessed risk.

### Assess The Mechanism And Its Failure Modes

Different interface mechanisms carry different failure modes. Understand the mechanism to understand what could go wrong.

Assess:

- whether the transfer is real-time, scheduled batch, event-driven, or manual;
- whether data is pushed, pulled, or re-keyed;
- whether the interface uses direct integration, file transfer, or an intermediate tool;
- whether the transfer is one-to-one, one-to-many, or many-to-one;
- what happens when the source or target is unavailable;
- whether failed transfers are retried, held, or silently dropped.

A batch interface that fails silently and is never retried can drop an entire period's transactions. A manual re-key interface can introduce transcription errors. Map the mechanism to its failure modes.

### Evaluate Data Transformation And Mapping Risk

Interfaces rarely move data unchanged. Data is transformed, mapped, converted, or enriched, and each transformation is a point where errors and manipulation enter.

Evaluate:

- field mappings between source and target, and whether they are one-to-one or many-to-one;
- currency or unit conversions performed at the interface;
- default values applied when source data is missing;
- lookups or enrichments that depend on master data;
- aggregations or summarizations that lose detail;
- rules that classify or code transactions during transfer.

A mapping error can route transactions to the wrong account for months before it is noticed. A transformation that depends on master data inherits the risk that the master data is wrong.

### Test Completeness And Accuracy Of The Transfer

The core interface assertion is that what arrived completely and accurately reflects what left. This must be tested, not assumed.

Test completeness by:

- reconciling control totals, record counts, and monetary totals between source and target;
- identifying items in the source that did not appear in the target;
- testing over a period that captures batch boundaries and failures.

Test accuracy by:

- selecting items that transferred and confirming the amount, coding, and timing match between source and target;
- testing transformations, conversions, and mappings on selected items;
- confirming that defaults, lookups, and enrichments were applied correctly.

Document the reconciliation and the items tested. An interface tested only by inquiry or by confirming both systems have records has not been tested for the transfer itself.

### Assess Whether Interface Failures Are Detected And Corrected

Even well-designed interfaces fail. The question is whether failures are detected, escalated, and corrected before they affect the financial statements.

Assess:

- whether the interface produces error or exception reports;
- whether someone reviews those reports and resolves exceptions;
- whether failed transfers are held and retried, or silently lost;
- whether reconciliations exist between source and target and are performed timely;
- whether unresolved exceptions are escalated;
- whether there is a history of undetected interface failures.

An interface with no exception reporting and no reconciliation is an interface whose failures are invisible. Treat the absence of failure detection as a control deficiency.

### Evaluate Reconciliation Controls As Compensating Controls

Reconciliations between systems are often the primary compensating control for interface risk. Assess whether they are designed and operating effectively.

For each reconciliation, assess:

- what is reconciled, such as totals, counts, or individual items;
- the frequency and whether it is timely enough to catch errors before reporting;
- who performs it and whether they are independent;
- whether differences are investigated and resolved, not just noted;
- whether the reconciliation covers the full population or a subset;
- whether the reconciliation itself relies on complete and accurate data.

A reconciliation that is performed late, by an interested party, or that leaves differences unresolved does not compensate for interface risk.

### Consider The Effect Of Interface Changes During The Period

Interfaces change, and changes introduce risk. A mapping updated mid-period, a new interface go-live, or a decommissioning can create errors that affect the period under audit.

Identify:

- interface changes during the period, including mapping, frequency, or mechanism changes;
- new interfaces that went live, and whether parallel runs or validations were performed;
- interfaces that were decommissioned and whether their data flow was fully migrated;
- changes made to fix prior errors, and whether the fix introduced new ones.

For material interface changes, test the transition point, including the first periods under the new configuration. Errors at change points are common and often material.

## Common Traps

### Testing The Systems But Not The Interface

Confirming both systems have records does not test the transfer. The interface itself, including completeness and accuracy of the move, must be tested.

### Assuming Real-Time Means Reliable

Real-time interfaces can still drop, duplicate, or mis-map transactions. The mechanism's reliability depends on its controls, not its speed.

### Overlooking Data Transformation And Mapping

Data rarely moves unchanged. Mappings, conversions, defaults, and enrichments are where errors and manipulation enter, and they must be tested.

### Ignoring Silent Failures

An interface with no exception reporting and no reconciliation has invisible failures. The absence of failure detection is itself a control deficiency.

### Accepting Reconciliations Without Assessing Them

A reconciliation performed late, by an interested party, or with unresolved differences does not compensate for interface risk. Assess the reconciliation as a control.

### Missing Interface Changes During The Period

Changes at interfaces, including new go-lives and decommissioning, create errors at the transition point. Test the first periods under any new configuration.

### Reconciling Totals But Not Individual Items

A total reconciliation can hide individual errors that net to zero, such as duplicated and dropped transactions offsetting. Test individual items as well as totals.

## Self-Check

- Is every interface that affects material balances identified, with source, target, data, frequency, and mechanism documented?
- Is the interface mechanism assessed for its failure modes, including silent drops, retries, and manual re-keying?
- Is data transformation and mapping risk evaluated, including field mappings, conversions, defaults, lookups, and aggregations?
- Is completeness and accuracy of the transfer tested through control totals, record counts, and selected item matching, not just inquiry?
- Are interface failure detection and correction assessed, including exception reports, reconciliations, and escalation of unresolved items?
- Are reconciliations between systems assessed as compensating controls, including frequency, independence, and resolution of differences?
- Are interface changes during the period identified and tested at the transition point, including new go-lives and decommissioning?
- Could the auditor demonstrate, from the documentation, that data arriving in the target system completely and accurately reflects what left the source?
