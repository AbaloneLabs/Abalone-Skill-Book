---
name: accounting_software_selection_and_setup.md
description: Use when the agent is selecting accounting software, configuring a new accounting system, migrating data between systems, setting up a chart of accounts, mapping integrations, or deciding whether a proposed system configuration will support reliable bookkeeping, reporting, controls, and audit needs.
---

# Accounting Software Selection And Setup

Accounting software is the backbone of financial recordkeeping. The choice and configuration of that system shape every downstream activity: how transactions are recorded, how accounts reconcile, how reports are produced, how controls operate, how audits proceed, and how the business scales. A system chosen for price or familiarity alone, or configured hastily to start invoicing quickly, can create years of rework. Common consequences include unmapped accounts, broken integrations that duplicate or drop transactions, tax codes that misreport, periods that cannot be locked, audit trails that can be silently overwritten, and reporting structures that cannot answer the questions leadership asks. System setup is not an IT task. It is an accounting design decision with long-lived consequences.

Use this skill before selecting accounting software, configuring a new system, migrating from one system to another, restructuring a chart of accounts, setting up integrations, or reviewing whether an existing configuration is fit for purpose. The goal is to prevent the agent from producing a system that is easy to launch but structurally incapable of supporting reliable records, controls, and reporting.

## Core Rules

### Define Requirements From Accounting And Reporting Needs, Not From Features

Software selection often starts with feature lists and demos. It should start with the organization's accounting and reporting requirements. A system that cannot meet the core needs is wrong regardless of how polished its interface appears.

Gather requirements across:

- accounting basis, cash, accrual, or hybrid, and multi-entity needs;
- revenue recognition complexity, such as deferred revenue or percentage of completion;
- inventory method and valuation requirements;
- multi-currency, multi-tax, and multi-jurisdiction needs;
- consolidation and intercompany elimination;
- budgeting, forecasting, and variance reporting;
- fixed asset tracking and depreciation;
- payroll integration or in-system payroll;
- approval workflows and segregation of duties;
- audit trail, period lock, and change history;
- user roles and access control;
- integration with bank feeds, payment processors, CRM, inventory, and payroll;
- data extraction, API access, and reporting flexibility;
- retention, backup, and disaster recovery.

Prioritize requirements as mandatory, important, or desirable. A system that misses a mandatory requirement should be eliminated, not worked around.

### Design The Chart Of Accounts For The Business, Not For The Default Template

The default chart of accounts in most software is generic. Using it unchanged produces reports that cannot answer business questions and accounts that accumulate meaningless detail.

Design the chart to support:

- revenue by meaningful stream, product, or channel;
- expenses by controllable category and responsibility;
- balance sheet accounts that reconcile to subledgers or external statements;
- tax-specific accounts for sales tax, VAT, payroll taxes, and income taxes;
- tracking by department, project, location, class, or tag as needed;
- separation of operating from non-operating items;
- equity accounts that reflect the entity structure;
- clearing, suspense, and review accounts that are monitored, not permanent dumps.

Avoid over-fragmentation, where every vendor or product becomes an account, and under-fragmentation, where everything collapses into a few opaque accounts. The chart should be detailed enough to inform decisions and simple enough to maintain.

### Configure Tax Settings To Match Jurisdiction Obligations

Tax misconfiguration is one of the most damaging setup errors because it propagates silently across every transaction. Incorrect tax codes produce wrong filings, wrong remittances, and exposure to penalties and interest.

Configure:

- the correct tax authorities, rates, and rules for each jurisdiction;
- sales tax, VAT, GST, or equivalent output tax on sales;
- input tax credit or equivalent on purchases where applicable;
- tax-exempt, zero-rated, and reverse-charge treatments;
- tax on shipping, fees, and discounts per local rules;
- payroll tax accounts and mapping;
- tax reporting periods and remittance schedules;
- tax rounding rules per jurisdiction.

Tax rules change and vary by location, product type, customer status, and nexus. Do not assume a default tax setting is correct. Confirm the configuration against the applicable jurisdiction's requirements, and engage a tax professional for complex or multi-jurisdiction setups.

### Plan Data Migration With Reconciliation Checkpoints

Moving from one system to another is where data integrity is most often lost. Balances that do not tie, duplicate historical transactions, lost subledger detail, and broken opening balances are common migration failures.

Plan migration by:

- defining a clean cutover date;
- reconciling all accounts in the old system before migration;
- deciding what historical detail to migrate versus summarize;
- mapping old accounts to the new chart of accounts;
- migrating opening balances for all balance sheet accounts;
- verifying that total debits equal total credits after import;
- reconciling migrated subledgers to the general ledger;
- testing a sample of transactions for accuracy;
- running parallel systems for a period if risk is high;
- documenting the migration for audit support.

If opening balances do not tie to the prior closing balances, the new system is already unreliable. Resolve the difference before going live.

### Configure Integrations To Prevent Duplicate And Missing Transactions

Bank feeds, payment processors, payroll, inventory, and CRM integrations automate data entry, but they also automate errors at scale. A feed that imports twice, a mapping that posts to the wrong account, or a sync that drops transactions can corrupt the books faster than manual entry.

For each integration:

- identify the source of truth for each data field;
- define how duplicates will be prevented;
- map each imported field to the correct account and tracking dimension;
- handle gross-to-net breakdowns for processor deposits;
- configure clearing accounts for items awaiting classification;
- reconcile integration totals to source system reports regularly;
- monitor for sync failures and unresolved items;
- document the mapping and review it when business activity changes.

Automation is reliable only when the mapping is correct and the output is reconciled. Treat a new integration as a control that must be tested, not a convenience that can be trusted.

### Enforce Period Lock, Approval Workflows, And Access Control From The Start

Controls configured after go-live are often too late. The period in which the system launches without a period lock is the period in which backdated changes can occur unnoticed.

Configure before or at launch:

- period lock or close that restricts posting to closed periods;
- approval workflows for bills, payments, journal entries, and refunds;
- user roles with least privilege, separating entry, approval, and administration;
- bank detail change controls and dual approval where feasible;
- audit trail that records who changed what and when;
- disabled or restricted deletion of posted transactions;
- access reviews on a regular schedule.

A system without these controls may function for data entry but cannot reliably support audit, governance, or fraud prevention.

### Preserve Data Integrity, Backup, And Retention

Accounting data must survive system failures, staff turnover, vendor changes, and audit inquiries years later. A cloud system is not automatically backed up in a way that meets accounting retention needs.

Address:

- backup frequency, location, and restore testing;
- data export in a usable format independent of the vendor;
- retention period meeting the applicable jurisdiction's requirements;
- access to historical periods if the subscription lapses;
- vendor data portability and exit terms;
- encryption and security standards appropriate to the data sensitivity;
- business continuity if the vendor experiences an outage.

Do not assume the vendor retains data indefinitely or in a usable form. Confirm retention and export capabilities before relying on them.

### Document The Configuration For Continuity And Audit

A configuration that exists only in one person's understanding is fragile. When that person leaves, the system becomes opaque.

Document:

- the chart of accounts and its design rationale;
- account mappings for integrations and imports;
- recurring transaction templates and their logic;
- approval workflow rules and thresholds;
- user roles and access assignments;
- tax code mappings and jurisdiction basis;
- reconciliation procedures for each account type;
- period close checklist and locking policy;
- known limitations, workarounds, and manual processes.

Documentation should be sufficient for a new accountant or an auditor to understand how the system produces its numbers.

## Common Traps

### Choosing On Price Or Familiarity Alone

A cheap or familiar system that cannot handle multi-entity, multi-currency, inventory, or complex revenue will force expensive workarounds or a second migration. Evaluate fit against requirements first.

### Using The Default Chart Of Accounts Unchanged

Generic accounts produce generic reports. The chart must be tailored to the business's revenue streams, cost structure, and reporting needs.

### Trusting Tax Defaults

Default tax settings rarely match a specific jurisdiction's rules. Incorrect tax configuration creates filing errors and exposure. Confirm with the applicable requirements and a tax professional.

### Migrating Without Reconciliation

Opening balances that do not tie to prior closing balances start the new system with hidden errors. Reconcile before, during, and after migration.

### Going Live Without Controls

Launching without period lock, approval workflows, and access controls creates a window where unauthorized or backdated changes can occur. Configure controls at or before launch.

### Assuming Integrations Are Self-Maintaining

Mappings drift, business activity changes, and sync failures occur. Reconcile integration outputs to source systems regularly rather than trusting automation blindly.

### No Export Or Exit Plan

If the vendor raises prices, changes terms, or experiences failure, the organization must be able to extract its data. Confirm export formats and retention before dependence deepens.

### Undocumented Configuration

When the person who set up the system leaves, configuration knowledge is lost. Document mappings, workflows, roles, and procedures so the system remains understandable.

## Self-Check

- Are requirements defined from accounting, reporting, control, and audit needs before evaluating software features?
- Is the chart of accounts designed for the business's revenue streams, cost structure, tax obligations, and reporting dimensions rather than copied from a template?
- Are tax settings configured to match the applicable jurisdiction's rules, with a tax professional engaged for complex or multi-jurisdiction setups?
- Is data migration planned with reconciliation checkpoints, opening balance verification, and subledger-to-general-ledger tie-out?
- Are integrations configured with duplicate prevention, correct field mapping, clearing accounts, and regular reconciliation to source systems?
- Are period lock, approval workflows, least-privilege access, audit trail, and bank change controls configured at or before launch?
- Are backup, export, retention, security, and vendor exit terms confirmed to meet the organization's continuity and jurisdiction requirements?
- Is the configuration documented sufficiently for a new accountant or auditor to understand how the system produces its numbers?
- Does the setup acknowledge jurisdiction-specific tax, retention, and reporting obligations and recommend qualified professional review where the configuration is complex or material?
