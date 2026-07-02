---
name: journal_entry_creation_and_approval.md
description: Use when the agent is drafting journal entries, approving or reviewing general ledger postings, setting up journal entry workflows, deciding account coding for non-routine transactions, or evaluating whether entries have adequate description, support, and segregation of duties.
---

# Journal Entry Creation And Approval

Journal entries are the connective tissue of the general ledger. A single entry can move millions between accounts, reclassify revenue, adjust a prior period, or create a balance that flows into financial statements and tax returns. Because entries look mechanically simple, agents and accountants often treat them as data entry. The real risk is not in debiting the wrong account, it is in creating entries that lack evidence, bypass approval, override automated controls, or misstate a period without anyone noticing until audit.

Use this skill before drafting non-routine journal entries, designing journal entry approval workflows, reviewing entries during the close, or assessing whether a set of entries is audit-ready. The goal is to prevent entries that balance numerically but fail on substance, documentation, authorization, or timing.

This skill covers general journal entries. Routine subledger-driven postings, reversing entries, and recurring entries have separate skills. Jurisdiction-specific approval thresholds, SOX requirements, and audit trail rules vary, confirm the governing framework before applying any rule here.

## Core Rules

### Require A Business Reason And Source Support For Every Entry

Every journal entry must answer why it exists. An entry without a clear business reason is a control weakness regardless of whether the amount is correct.

Before approving an entry, confirm it has:

- a description that explains the business event, not just the account names;
- supporting documentation such as an invoice, contract, calculation, schedule, allocation memo, or subledger report;
- a reference or link to the source so a reviewer can find it later;
- an amount that ties to the support, with any rounding difference explained.

Entries that reclassify, estimate, accrue, or adjust reserves carry higher risk because they involve judgment. These should carry a memo explaining the methodology, not just the result.

### Enforce Segregation Of Duties

The person who creates an entry should not be the only person who approves it. Segregation of duties prevents a single individual from both initiating and authorizing a change to the books.

Review the workflow for:

- who prepares the entry;
- who reviews and approves it;
- who has posting access;
- whether the same person can create, approve, and post without independent check.

In smaller organizations where full segregation is impractical, implement compensating controls such as periodic review of all entries by an owner, detailed transaction reporting, or rotation of duties. Document why segregation is limited and what compensating control covers the gap.

### Apply Materiality And Risk-Tiered Approval

Not every entry needs the same scrutiny. Design approval thresholds based on materiality and risk type.

Higher-risk entries that should require elevated approval include:

- entries to revenue or cost of goods sold accounts;
- entries affecting cash, debt, or equity;
- entries to reserve, accrual, or estimate accounts;
- entries to prior periods or closed periods;
- entries involving related parties;
- manual entries that override automated postings;
- entries above a defined monetary threshold.

Low-risk, small, routine reclassifications may use streamlined approval, but the threshold should be documented and the population should be periodically sampled.

### Control Manual And Top-Side Entries

Manual entries and top-side entries, those posted directly to the general ledger outside the normal subledger process, are the most common source of financial misstatement. They bypass the controls built into automated systems.

For manual and top-side entries, require:

- explicit justification for why the entry cannot be processed through the normal subledger;
- identification of the initiating person and the approving person;
- post-posting review by someone independent of the preparer;
- monitoring of the volume and pattern of manual entries over time.

A spike in manual entries at period end is a red flag that the normal process is breaking down or that someone is managing earnings.

### Preserve Description Quality And Searchability

Entry descriptions are audit evidence. A description that reads "per JE" or "to fix" is useless to a reviewer six months later.

Strong descriptions include:

- the nature of the transaction or adjustment;
- the period or date range affected;
- the source document or calculation reference;
- any related entry or reversal reference.

Consistent description formats make it possible to search, filter, and review entry populations during audit or investigation. Poor descriptions make it impossible to distinguish legitimate adjustments from errors or manipulation.

### Manage Period Lock And Post-Close Entries

Once a period is closed, additional entries should require elevated authorization. Post-close entries may indicate a control failure, a missed accrual, or an intentional adjustment to managed results.

Establish rules for:

- who can reopen or post to a closed period;
- what documentation is required for post-close entries;
- whether post-close entries require controller or CFO approval;
- how post-close entries are reported and accumulated for audit.

### Document The Review Trail

The approval and review process must leave evidence. An auditor should be able to see who created each entry, who approved it, when it was posted, and whether it was reviewed after the fact.

Confirm the system or process captures timestamps, user identities, approval status, and any changes. If the accounting system does not track these automatically, maintain a manual log.

## Common Traps

### Treating Balanced Entries As Correct Entries

Double-entry balance proves only that debits equal credits. It says nothing about whether the accounts are right, the period is correct, the amount is supportable, or the entry is authorized. Balance is necessary but never sufficient.

### Approving Without Examining Support

Rubber-stamp approval, where a manager clicks approve without reviewing the underlying documentation, defeats the entire control. The approver becomes a formality rather than a check. This is especially dangerous for entries to sensitive accounts.

### Using Suspense Or Clearing Accounts As Permanent Parking

Suspense and clearing accounts are meant to hold items temporarily. When entries sit in these accounts period after period, they hide unresolved problems, potential errors, or unrecorded liabilities. Review and clear suspense balances every period.

### Backdating Entries To Manage Results

Moving an entry to a prior period to smooth earnings, hit a target, or shift a loss is a serious integrity violation. Even when the intent seems benign, backdating without proper authorization and disclosure undermines the reliability of financial statements.

### Allowing The Same Person To Create And Approve

In practice, especially in small teams, one person may have access to do both. This eliminates the independent check that catches errors and deters manipulation. Even a lightweight second-person review is better than none.

### Ignoring Round-Trip And Offset Patterns

Pairs of entries that move a balance out of one account and back in, or that offset each other across periods, can mask the true activity. These patterns sometimes indicate window dressing or an attempt to hide a problem. Review offsetting entry patterns during the close.

### Assuming Automated Entries Need No Review

Entries generated by subledgers, allocations, or recurring templates can carry forward errors indefinitely. Automated does not mean correct. Sample automated entries periodically and review any that fail exception reports.

## Self-Check

- Does every non-routine entry have a business reason, supporting documentation, and a traceable source reference?
- Is there segregation between the person who prepares an entry and the person who approves it, or a documented compensating control?
- Are higher-risk entries to revenue, cash, equity, reserves, related parties, and prior periods subject to elevated approval?
- Are manual and top-side entries justified, monitored for volume, and independently reviewed after posting?
- Do entry descriptions explain the business event, period, and source rather than reading as placeholders?
- Are closed periods protected, with post-close entries requiring elevated authorization and accumulated for audit?
- Are suspense and clearing accounts reviewed and cleared each period rather than accumulating balances?
- Is the review trail captured with timestamps, user identities, and approval status that an auditor could follow?
- Have jurisdiction-specific approval, SOX, and audit trail requirements been confirmed before relying on this guidance?
