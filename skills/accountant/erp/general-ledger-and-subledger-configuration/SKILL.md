---
name: general_ledger_and_subledger_configuration.md
description: Use when the agent is configuring a general ledger or subledger in an ERP, setting up posting rules and account determinations, designing the relationship between subledgers and the GL, or reviewing whether ERP configuration produces correct, auditable, and reconcilable financial records.
---

# General Ledger And Subledger Configuration

The general ledger and its subledgers are the backbone of the financial system. Their configuration determines how every transaction is classified, posted, summarized, and reported. Configuration decisions that look technical, such as which account a particular posting hits, how subledgers reconcile to the GL, or how posting rules handle exceptions, have direct accounting consequences. When configuration is wrong, the error is systematic: every transaction of a given type is misposted, subledgers do not tie to the ledger, and the problem persists silently until someone tries to reconcile or report.

Use this skill before configuring a general ledger or subledger, setting posting rules or account determinations, designing subledger-to-GL integration, or reviewing whether existing configuration produces correct and reconcilable records. The goal is to prevent the agent from building configuration that posts beautifully but misclassifies transactions, breaks reconciliation, or creates an audit trail that cannot be followed.

## Core Rules

### Design The GL Structure Around Reporting And Control

The general ledger structure, chart of accounts, segments, and hierarchies, should serve reporting, control, and reconciliation, not just transaction entry.

Design the structure to support:

- financial statement presentation under the applicable framework;
- management reporting by entity, segment, location, or product;
- reconciliation points between subledgers and the GL;
- segregation of duties and approval authority;
- tax and regulatory reporting requirements;
- consolidation and elimination if multiple entities are involved.

A GL structure built only for data entry speed produces reporting that requires extensive manual rework and reconciliation that cannot be automated.

### Define Clear Subledger-To-GL Relationships

Subledgers, such as accounts payable, accounts receivable, fixed assets, inventory, and payroll, must reconcile cleanly to their GL control accounts. This relationship must be designed, not assumed.

For each subledger define:

- the GL control account or accounts it posts to;
- the posting rules that move subledger transactions to the GL;
- the reconciliation point, usually at period end, where subledger totals tie to the GL;
- the handling of intercompany, tax, and discount postings;
- the treatment of adjustments, reversals, and reclassifications.

A subledger that posts to the GL without a clean reconciliation point produces control accounts that drift and eventually require large adjustments.

### Make Account Determination Explicit And Documented

Account determination, the rules that decide which GL account a transaction hits, is where configuration most often goes wrong. Rules that are implicit or undocumented cannot be reviewed or audited.

For each transaction type document:

- the fields and conditions that drive account determination;
- the resulting GL account for each condition;
- the handling of exceptions and fallback accounts;
- the tax and intercompany implications of each posting;
- the person or role responsible for maintaining the rules.

When a transaction posts to a surprise account, the cause is almost always an undocumented or misunderstood determination rule. Make the rules explicit so they can be reviewed and challenged.

### Preserve A Complete And Followable Audit Trail

ERP configuration should produce an audit trail that lets a reviewer trace any GL entry back to its source and follow any source document forward to the GL.

Ensure the system captures:

- the original transaction and its source, subledger, manual entry, or interface;
- the posting rule or account determination applied;
- the user or process that created and posted the entry;
- timestamps for creation, posting, and any modification;
- the link between the GL entry and the supporting subledger detail;
- change logs for configuration and master data.

A GL entry that cannot be traced to its source, or a source that cannot be followed to the GL, breaks the audit trail and weakens control.

### Control Configuration Changes Rigorously

Configuration changes affect every subsequent transaction. Uncontrolled changes produce silent shifts in classification and reporting.

Control changes by:

- requiring a documented business reason for each change;
- testing changes in a non-production environment before promotion;
- obtaining approval from finance and, where relevant, IT and audit;
- recording the change, the requester, the approver, and the effective date;
- assessing the effect on open periods, comparatives, and reconciliations.

Ad hoc configuration changes, made without testing or approval, are a common source of unexplained reporting shifts and audit findings.

### Reconcile Subledgers To The GL Every Period

Configuration is proven by reconciliation. A well-configured system produces subledger-to-GL ties every period; a poorly configured one produces breaks.

Reconcile each period:

- AP and AR subledgers to their control accounts;
- fixed asset subledger to the asset GL accounts;
- inventory subledger to inventory GL accounts;
- payroll subledger to payroll liability and expense accounts;
- bank subledger or cash module to the bank GL accounts.

Investigate breaks promptly. A persistent break indicates a configuration or process error that must be fixed, not absorbed.

### Handle Period Close And Cutoff In Configuration

Configuration should support clean period close and cutoff, not work against it. Posting rules, period controls, and cutoff logic must reflect the accounting policy.

Configure for:

- period locking once close is complete, with controlled reopening only by approval;
- cutoff rules for revenue, expenses, and inventory at period end;
- accrual and reversal posting rules that match policy;
- the handling of transactions dated near the period boundary;
- consolidation and elimination timing for multi-entity closes.

Configuration that allows backdated postings into closed periods, or that mishandles cutoff, undermines the integrity of every close.

### Acknowledge Framework And Professional Limits

ERP configuration implements accounting policy, but the policy itself must comply with the applicable reporting framework, GAAP, IFRS, tax, or regulatory rules. Configuration decisions about revenue recognition, lease accounting, hedging, intercompany, and tax determination often involve framework-specific requirements that require professional accounting judgment. Confirm significant configuration with qualified accounting professionals, and validate that the configured behavior produces framework-compliant results. Do not treat system configuration as a substitute for accounting policy review.

## Common Traps

### GL Structure Built Only For Data Entry

A chart of accounts optimized for entry speed produces reporting and reconciliation that require extensive manual rework.

### Subledgers Without Clean Reconciliation Points

Subledgers that post without a designed tie-out to control accounts drift and require large adjustments.

### Implicit Account Determination

Posting rules that are undocumented or misunderstood produce surprise accounts and unexplained classification shifts.

### Broken Or Incomplete Audit Trail

GL entries that cannot be traced to source, or sources that cannot be followed to the GL, weaken control and auditability.

### Uncontrolled Configuration Changes

Ad hoc changes without testing or approval produce silent shifts in classification and reporting.

### Absorbed Reconciliation Breaks

Persistent subledger-to-GL breaks that are written off rather than investigated indicate configuration or process errors.

### Poor Cutoff And Period Control

Configuration that allows backdating or mishandles cutoff undermines every close.

### Configuration Without Framework Review

System settings that implement accounting policy must be confirmed against the applicable framework by qualified professionals.

## Self-Check

- Does the GL structure support financial statement presentation, management reporting, reconciliation, segregation, tax, and consolidation?
- Is each subledger designed with a clear control account, posting rules, and a period-end reconciliation point to the GL?
- Are account determination rules explicit, documented, and owned, with exceptions and fallbacks defined?
- Does the system preserve a complete audit trail from source to GL and back, with change logs for configuration and master data?
- Are configuration changes controlled with documented reason, testing, approval, and effective dating?
- Do subledgers reconcile to the GL every period, with breaks investigated rather than absorbed?
- Does configuration support clean period close and cutoff, with controlled period locking and accrual reversal?
- Does the configuration reflect framework-compliant accounting policy confirmed with qualified professionals, rather than substituting system settings for policy review?
