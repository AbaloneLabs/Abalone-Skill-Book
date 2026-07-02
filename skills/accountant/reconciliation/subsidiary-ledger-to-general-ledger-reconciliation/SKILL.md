---
name: subsidiary_ledger_to_general_ledger_reconciliation.md
description: Use when the agent is reconciling accounts receivable or accounts payable subledgers to the general ledger, investigating subledger-to-GL differences, handling posting gaps, reconciling fixed asset or inventory subledgers, or reviewing subledger control accounts.
---

# Subsidiary Ledger To General Ledger Reconciliation

A subsidiary ledger holds the detail behind a general ledger control account. The accounts receivable subledger shows every customer and invoice, while the GL control account shows one total. The accounts payable subledger shows every vendor and bill, while the GL shows one total. The reconciliation confirms that the detail sums to the control. When they do not tie, the GL balance that flows into financial statements is wrong, and the customer or vendor detail used for collections and payments is unreliable. The trap is that subledger-to-GL breaks are common, often small, and easy to paper over with a plug entry. Agents frequently reconcile by forcing the two numbers together without finding the underlying posting gap, leaving a hidden error that recurs every period.

Use this skill before reconciling a subledger to its control account, investigating a break, designing reconciliation procedures, or reviewing the quality of subledger reconciliations. The goal is to ensure the detail genuinely ties to the control, with every difference explained and corrected at its source.

This skill covers subledger-to-GL reconciliation principles. Specific subledger systems, posting rules, and control account setups vary by platform and entity structure. Confirm the system configuration before relying on these procedures.

## Core Rules

### Reconcile Every Control Account To Its Subledger Each Period

Every general ledger account that has an underlying subledger must be reconciled each period. The common control accounts are accounts receivable, accounts payable, and often fixed assets, inventory, and prepaid expenses.

Confirm that:

- the subledger total equals the GL control account balance;
- the reconciliation is performed after all period postings are complete;
- the reconciliation is reviewed by a second person;
- breaks are investigated and resolved before the close is finalized.

An unreconciled control account means the balance sheet total is unverified.

### Identify The Source Of Every Break

When the subledger and GL do not tie, the difference must be traced to a specific cause. Do not plug the difference.

Common causes of breaks include:

- a subledger transaction that did not post to the GL;
- a GL journal entry posted directly to the control account, bypassing the subledger;
- a posting to the wrong period in either system;
- a timing difference between subledger close and GL close;
- a duplicate or omitted transaction;
- a manual override or top-side entry.

Trace the difference to the specific transaction or posting that caused it, then correct it at the source.

### Prevent Direct Posting To Control Accounts

A leading cause of subledger-to-GL breaks is direct journal entry posting to a control account. Control accounts should be fed only by their subledger. When someone posts a manual entry directly to the accounts receivable or accounts payable control account, the subledger no longer sums to the GL.

Enforce that:

- the control accounts are restricted from direct manual posting where the system allows;
- any necessary direct entry is documented, approved, and mirrored in the subledger;
- exceptions are reviewed each period.

If direct posting cannot be prevented technically, compensate with strict review of any entry touching a control account.

### Reconcile In The Same Units And Same Period

A break can arise simply from comparing the subledger and GL at different points in time or in different units.

Confirm:

- both balances are as of the same period-end date;
- both are in the same currency, with no unrecorded revaluation difference;
- both reflect all postings through the same cutoff;
- intercompany or multi-entity subledgers are reconciled in the correct entity.

A timing or currency mismatch creates an artificial break that masks the real position.

### Handle Timing Differences Between Subledger And GL Closes

Subledgers and the GL sometimes close on slightly different schedules. A transaction posted to the subledger after the GL close, or vice versa, creates a temporary break.

Manage timing by:

- defining a single cutoff for both systems;
- documenting any in-transit items between subledger and GL;
- ensuring in-transit items clear in the next period;
- not letting in-transit items become permanent reconciling differences.

A recurring in-transit balance suggests the close sequence needs fixing.

### Reconcile Detail Beyond The Total When Needed

For some control accounts, tying the total is not enough. Fixed asset subledgers should reconcile by asset class. Inventory subledgers should reconcile by location or item category. AR and AP should reconcile by aging bucket if the aging is used for reporting or reserves.

Deeper reconciliation catches errors that net to zero at the total level but distort the detail, such as a customer balance in the wrong aging bucket.

### Document And Age Reconciling Items

Every reconciling item should be documented with its cause, its expected resolution, and its age. Items that persist across periods are a warning sign.

Track:

- when the item arose;
- why it has not cleared;
- what action is being taken;
- whether it indicates a process failure that needs fixing.

Reconciling items that carry forward indefinitely often hide errors, unreversed entries, or unrecorded transactions.

### Review And Approve Each Reconciliation

Like the bank rec, a subledger reconciliation should be reviewed and approved by a second person. The review confirms that breaks were genuinely investigated, not plugged, and that reconciling items are valid and aging appropriately.

## Common Traps

### Plugging The Difference Instead Of Finding It

When the subledger and GL differ, the easy path is a journal entry to force the tie. This hides the real posting error, which then recurs. Always find the source.

### Direct Posting To Control Accounts

Manual entries to AR or AP control accounts bypass the subledger and guarantee a future break. Restrict direct posting and review any exceptions.

### Comparing At Different Dates Or In Different Currencies

A break caused by a timing or unit mismatch is not a real difference, but it masks the real one. Reconcile at the same date and in the same currency.

### Accepting A Tied Total With Distorted Detail

The subledger can sum to the GL while individual customer, vendor, or asset balances are wrong. Errors that net to zero at the total still corrupt the detail used for collections, payments, and reporting.

### Letting In-Transit Items Become Permanent

A timing difference between subledger and GL close should clear next period. If the same in-transit balance appears every period, the close sequence is broken.

### Carrying Reconciling Items Forward Without Aging

Reconciling items that sit on the rec month after month without investigation hide errors. Age and document every item, and resolve old items.

### Reconciling Only AR And AP

Fixed assets, inventory, prepaids, and other subledger-backed accounts also need reconciliation. Skipping them leaves those balance sheet totals unverified.

## Self-Check

- Does every control account with an underlying subledger, including AR, AP, fixed assets, inventory, and prepaids, reconcile to its subledger each period?
- Is every break traced to a specific posting or transaction and corrected at the source rather than plugged?
- Are control accounts restricted from direct manual posting, with any exceptions documented and mirrored in the subledger?
- Are the subledger and GL compared at the same period-end date, in the same currency, and with the same cutoff?
- Are timing differences between subledger and GL close documented as in-transit and cleared the next period?
- Is detail reconciled beyond the total where aging, class, or location matters for reporting?
- Is every reconciling item documented with cause, expected resolution, and age, with old items resolved?
- Is each reconciliation reviewed and approved by a second person independent of the preparer?
- Has the system-specific posting and control account configuration been confirmed before relying on these procedures?
