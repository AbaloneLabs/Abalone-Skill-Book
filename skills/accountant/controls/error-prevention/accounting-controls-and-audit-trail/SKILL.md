---
name: accounting_controls_and_audit_trail.md
description: Use when the agent is designing accounting controls, reviewing approval workflows, preventing bookkeeping errors or fraud, protecting financial data, maintaining audit trails, managing accounting system access, or deciding how changes to accounting records should be governed.
---

# Accounting Controls And Audit Trail

Accounting controls are practical guardrails that keep financial records complete, accurate, authorized, and explainable. They are not only for large public companies. A small team still needs controls over cash, vendor payments, payroll, refunds, journal entries, system access, source documents, and changes to closed periods. Without controls, an agent may design a process that is efficient for entry but weak against mistakes, duplicate payments, unauthorized changes, privacy leaks, or fraud.

Use this skill before changing an accounting workflow, granting access to financial systems, approving payments, creating journal-entry routines, handling sensitive accounting data, reviewing audit readiness, or deciding how accounting records should be corrected. The goal is to prevent the agent from optimizing convenience while losing accountability.

## Core Rules

### Identify The Financial Risk Before Designing The Control

Controls should respond to real risks. Do not add random approvals or checklists without understanding what could go wrong.

Identify risks such as:

- revenue omitted or recorded twice;
- expenses classified incorrectly;
- vendor paid twice;
- fictitious vendor created;
- bank account changed without authorization;
- payroll paid to the wrong person;
- refund issued improperly;
- journal entry used to hide an error;
- closed period changed without notice;
- sensitive documents exposed;
- source documents deleted;
- reconciliation skipped;
- financial report presented before review.

For each risk, define what the control prevents or detects, who performs it, what evidence it leaves, and how exceptions are resolved.

### Separate Incompatible Duties Where Practical

Segregation of duties reduces the chance that one person can create, approve, execute, and conceal a financial transaction. Small teams may not have enough staff for perfect separation, but the risk still needs visible compensating controls.

Separate or review combinations of:

- vendor setup and vendor payment approval;
- bank account changes and payment release;
- invoice entry and payment approval;
- payroll setup and payroll approval;
- refund request and refund issuance;
- bank reconciliation and cash handling;
- journal entry preparation and approval;
- system administration and transaction posting;
- financial report preparation and final approval.

When separation is impossible, add owner review, bank alerts, exception reports, dual approval for sensitive changes, independent reconciliation, or periodic outside review.

### Control Master Data Changes

Master data changes can be more dangerous than individual transactions. A fraudulent vendor bank account, incorrect customer terms, wrong payroll record, or changed product mapping can cause repeated errors.

Review changes to:

- vendor name, address, tax identifier, and bank details;
- customer billing details and credit terms;
- employee payroll details;
- chart of accounts;
- product or service mappings;
- tax codes;
- payment approval limits;
- user roles and permissions;
- recurring journal entries;
- recurring invoices and bills.

Require evidence for sensitive changes. For bank changes, use a verification process that does not rely only on the contact information included in the change request.

### Preserve Audit Trails For Entries And Changes

An audit trail should show what happened, when, who did it, why it was done, and what evidence supports it. If the system allows silent deletion or overwriting, use process rules to preserve history.

For transactions and changes, keep:

- creator;
- approver;
- timestamp;
- source document;
- business purpose;
- original value;
- changed value;
- reason for change;
- related approval;
- impact on reports;
- link to reconciliation or review.

Do not design a process where the final spreadsheet becomes the only record. Spreadsheets can be useful, but they need version control, access control, formulas protected from accidental change, and clear linkage to source data.

### Require Approval Based On Risk, Not Only Amount

Dollar thresholds are useful but incomplete. Some low-dollar transactions are sensitive, and some high-dollar transactions are routine. Approval rules should consider amount, account, vendor, payment method, data sensitivity, related party, unusual timing, and whether the transaction affects external reporting.

Require stronger review for:

- new vendors;
- vendor bank changes;
- owner or related-party transactions;
- payroll changes;
- refunds and credits;
- manual journal entries;
- entries to revenue, cash, debt, tax, equity, or suspense accounts;
- changes to closed periods;
- unusual or nonrecurring transactions;
- transactions without complete support.

Approval should be documented before payment or posting whenever possible. After-the-fact approval is a weaker detective control and should not become normal practice.

### Protect Sensitive Financial Data

Accounting records often contain information that should not be broadly exposed. The agent should not recommend sharing raw data when summaries or redacted extracts are enough.

Sensitive data may include:

- bank account numbers;
- tax identifiers;
- payroll details;
- employee addresses;
- customer personal information;
- vendor banking details;
- card data;
- loan documents;
- legal settlements;
- medical or benefits information;
- owner compensation;
- investor information;
- authentication tokens or system exports.

Use least privilege. Give each person or system only the access needed for the task. Redact, aggregate, or separate files when full detail is not required. Keep exports under the same control expectations as the system of record.

### Reconcile And Review Exception Reports

Controls should detect errors after processing, not only prevent them upfront. Reconciliations and exception reports help identify missed items, duplicate activity, stale balances, and policy breaches.

Use exception reports for:

- duplicate invoice numbers;
- duplicate payment amounts to same vendor;
- payments just under approval thresholds;
- new vendors with immediate payments;
- vendor bank changes;
- manual journal entries;
- entries posted to closed periods;
- stale unreconciled transactions;
- old open receivables or payables;
- negative inventory;
- suspense or uncategorized balances;
- user permission changes.

Exception reports must be reviewed by someone with authority to act. A report that nobody reads is not a control.

### Govern Journal Entries And Period Changes

Manual journal entries can fix problems, but they can also bypass normal transaction controls. Treat them as high-risk when they affect revenue, cash, debt, tax, equity, payroll, inventory, estimates, or closed periods.

For journal entries, require:

- clear description;
- preparer;
- reviewer;
- affected accounts;
- calculation;
- source support;
- reason for entry;
- recurring or one-time status;
- reversal date if applicable;
- approval evidence.

Changes to closed periods should require stricter review. If the change affects issued reports, filings, debt compliance, tax returns, investor communication, or compensation, escalate before posting.

### Make Control Evidence Easy To Inspect

A control that cannot be evidenced is hard to rely on. Approval messages, reconciliation files, review notes, timestamps, and exception resolution should be organized for later inspection.

Keep evidence:

- close to the transaction or period;
- named consistently;
- linked to source records;
- protected from unauthorized editing;
- retained according to policy;
- readable without relying on one person's memory;
- available to internal reviewers, external accountants, auditors, or regulators when appropriate.

Good evidence reduces rework. It also protects the organization when staff changes or questions arise months later.

## Common Traps

### Designing For Trust Instead Of Verification

Trustworthy people still make mistakes, face pressure, forget steps, and misunderstand policies. Controls should not be framed as personal suspicion. They are how the process remains reliable when humans are busy, absent, or wrong.

### Giving Admin Access For Convenience

Broad access makes work easier until something goes wrong. Admin access can change users, permissions, bank details, accounting periods, integrations, and audit settings. Grant it sparingly and review it regularly.

### Letting Email Approval Become The Whole Control

Email approvals can be useful, but they are weak if the approver cannot see support, if the thread is hard to retrieve, or if the approved transaction changes afterward. Link approval to the exact transaction and preserve the final support.

### Ignoring Spreadsheet Risk

Many accounting processes depend on spreadsheets. Hidden rows, broken formulas, overwritten cells, stale exports, and uncontrolled versions create real reporting risk. Use locked formulas, change history, source-data tabs, review checks, and clear version naming.

### Treating Automation As A Control By Itself

Automation repeats instructions quickly. If the rule is wrong, automation scales the error. Review mappings, recurring transactions, integrations, approval routing, and import rules periodically.

### Overcontrolling Low-Risk Work While Missing Sensitive Events

Too many approvals on routine transactions create fatigue. Meanwhile, a bank detail change, manual revenue entry, or payroll change may pass unnoticed. Focus effort where errors or misuse would matter.

### Deleting Evidence To Clean Up Records

Cleaning up should not mean erasing history. Void, reverse, annotate, archive, or correct in a way that preserves what happened. Deletion can break audit trails and make later explanation impossible.

## Self-Check

- Have the key accounting risks been identified before choosing controls?
- Are incompatible duties separated where practical, with compensating review where separation is impossible?
- Are vendor, customer, payroll, chart-of-account, tax-code, payment-rule, and user-permission changes controlled?
- Does each material transaction or change leave evidence of creator, approver, timestamp, reason, support, and impact?
- Are approvals based on sensitivity and risk, not only dollar amount?
- Is sensitive financial, payroll, tax, banking, customer, vendor, and owner data protected by least privilege?
- Are reconciliations and exception reports actually reviewed and resolved?
- Are manual journal entries, recurring entries, and closed-period changes supported and approved?
- Are accounting exports and spreadsheets controlled for access, versions, formulas, and source data?
- Can an independent reviewer inspect the evidence without relying on one person's memory?
