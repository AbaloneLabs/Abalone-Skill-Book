---
name: master_data_and_chart_of_accounts.md
description: Use when the agent is designing or restructuring a chart of accounts, setting up master data for customers vendors and items, governing master data changes, or reviewing whether ERP master data supports correct posting, reporting, and reconciliation.
---

# Master Data And Chart Of Accounts

Master data is the static information that drives transaction processing: the chart of accounts, customer and vendor records, item and product masters, cost centers, projects, and payment terms. It looks administrative, but it governs how every transaction posts, reports, and reconciles. A wrong account number in a customer master sends revenue to the wrong place for every invoice. A wrong payment term creates cash flow distortion across a whole vendor set. A bloated or inconsistent chart of accounts makes reporting a reconstruction project every period. Master data is not a back-office concern; it is a primary control over the integrity of the financial statements.

Use this skill before designing or restructuring a chart of accounts, setting up or cleansing master data, establishing master data governance, or reviewing whether master data supports correct posting and reporting. The goal is to prevent the agent from building master data that posts cleanly but misclassifies systematically, or a chart of accounts that captures detail without supporting reporting.

## Core Rules

### Design The Chart Of Accounts Around Reporting And Analysis

The chart of accounts is the skeleton of the financial statements. It should be designed to support reporting, analysis, and control, not merely to capture every possible transaction.

Design the COA to support:

- financial statement line items under the applicable framework;
- management reporting by segment, geography, product, or function;
- tax and regulatory reporting without extensive manual reclassification;
- reconciliation points for subledgers and control accounts;
- consolidation and elimination across entities;
- a level of detail that supports decisions without overwhelming entry.

Avoid creating a new account for every unusual transaction. A COA that grows without discipline becomes unusable. Design for the reporting you need, and handle exceptions through analysis, not account proliferation.

### Use Segments And Dimensions Deliberately

Modern ERPs use segments and dimensions, such as cost center, project, location, or product line, to add reporting depth without exploding the account list. Use them deliberately.

Decide:

- which reporting dimensions are needed, entity, department, location, project, product;
- whether a dimension is mandatory or optional for each account type;
- how dimensions interact with consolidation and elimination;
- how dimensions support tax and segment reporting;
- the validation rules that prevent invalid combinations.

Dimensions that are optional or unvalidated produce inconsistent reporting. Dimensions that are too numerous burden every entry. Choose the minimum set that supports real decisions.

### Ensure Master Data Drives Correct Posting

Master records, customers, vendors, items, and projects, carry default accounts, terms, and rules that drive posting. These defaults must be correct, because they apply to every transaction.

For each master type verify:

- customer and vendor records carry the correct revenue, receivable, payable, and tax accounts;
- item masters carry the correct inventory, cost of goods sold, and revenue accounts;
- payment terms and tax codes are correct and current;
- intercompany and related party flags are set where applicable;
- bank and remittance details are controlled and verified.

A wrong default in a master record produces systematic misposting that is invisible until reconciliation or audit. Master data accuracy is a primary control.

### Govern Master Data Changes

Master data changes affect every subsequent transaction. Uncontrolled changes produce silent shifts in classification, reporting, and cash flow.

Govern changes by:

- requiring a documented business reason for each change;
- routing changes through a defined approver, often outside the data entry role;
- testing the effect of changes on posting and reporting before promotion;
- recording the change, requester, approver, and effective date;
- reviewing master data changes periodically as a control.

A new GL account added without review, or a vendor bank detail changed without verification, are common sources of error and fraud. Control master data the way you control journal entries.

### Control Sensitive Master Data Fields

Some master data fields carry elevated risk because they control money or access. These require stronger controls.

Strengthen control over:

- vendor and employee bank account and remittance details;
- tax codes and rates on customer, vendor, and item masters;
- credit limits and payment terms;
- user access and approval limits;
- intercompany and related party indicators.

Bank detail changes are a classic fraud vector. Tax code errors create systematic misstatement. Require independent verification and approval for changes to these fields.

### Maintain Data Quality And Cleanse Periodically

Master data degrades. Duplicates accumulate, obsolete records linger, and defaults drift. Periodic cleansing is necessary.

Cleanse periodically by:

- identifying and merging duplicate customers, vendors, and items;
- inactivating obsolete records;
- reviewing accounts with no activity for potential consolidation;
- validating that defaults and terms remain correct;
- reconciling master-driven postings to expectations.

A master database that has never been cleansed produces reporting noise and reconciliation difficulty that grow over time.

### Document The Master Data Model

Master data structure should be documented so that finance, IT, and auditors understand how posting and reporting are driven.

Document:

- the chart of accounts structure and the purpose of each segment;
- the master record types and the key fields that drive posting;
- the validation rules and required combinations;
- the governance process for changes;
- the mapping from master data to financial statement line items.

An undocumented master data model is understood only by the people who built it, and it becomes unintelligible when they leave.

### Acknowledge Framework And Professional Limits

Master data and chart of accounts design implement accounting policy, but the design must comply with the applicable reporting framework and with tax and regulatory requirements. Segment reporting, tax determination, intercompany, and revenue account structures often involve framework-specific requirements. Confirm significant design decisions with qualified accounting and tax professionals, and validate that master-driven posting produces framework-compliant results. Do not treat master data configuration as a substitute for accounting and tax policy review.

## Common Traps

### Account Proliferation

Creating a new account for every unusual transaction produces an unusable COA and reporting that requires constant consolidation.

### Optional Or Unvalidated Dimensions

Dimensions that are not mandatory or validated produce inconsistent and unreliable reporting.

### Wrong Defaults In Master Records

A wrong account, term, or tax code in a master record misposts every related transaction systematically.

### Uncontrolled Master Data Changes

Changes without approval or testing produce silent shifts in classification, reporting, and cash flow.

### Weak Control Over Sensitive Fields

Bank detail, tax code, and credit limit changes without verification are primary fraud and error vectors.

### No Periodic Cleansing

Master data that is never cleansed accumulates duplicates and drift, degrading reporting and reconciliation.

### Undocumented Model

A master data model understood only by its builders becomes unintelligible when they leave.

### Configuration Without Framework Review

Master data design must be confirmed against the applicable framework and tax rules by qualified professionals.

## Self-Check

- Does the chart of accounts support financial statement presentation, management reporting, tax, reconciliation, and consolidation without excessive proliferation?
- Are segments and dimensions chosen deliberately, mandatory where needed, and validated to prevent invalid combinations?
- Do customer, vendor, item, and project masters carry correct default accounts, terms, and tax codes that drive correct posting?
- Are master data changes governed with documented reason, approval, testing, and recording?
- Are sensitive fields, bank details, tax codes, credit limits, and access, controlled with independent verification?
- Is master data cleansed periodically to remove duplicates, inactivate obsolete records, and validate defaults?
- Is the master data model documented, including COA structure, posting drivers, validation rules, governance, and statement mapping?
- Does the design reflect framework-compliant accounting and tax policy confirmed with qualified professionals, rather than substituting configuration for policy review?
