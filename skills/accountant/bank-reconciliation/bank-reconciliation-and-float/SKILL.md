---
name: bank_reconciliation_and_float.md
description: Use when the agent is performing or reviewing a bank reconciliation, clearing deposits in transit and outstanding checks, investigating NSF checks and bank errors, reconciling merchant processor settlements gross to net, chasing stale reconciling items, or assessing whether a reconciliation is genuinely complete beyond a zero difference.
---

# Bank Reconciliation And Float

A bank reconciliation is the single most important cash control most entities perform, and it is also the one most often reduced to a mechanical exercise. When the goal becomes "make the difference zero," reconciliations start hiding problems instead of revealing them. Old outstanding checks, duplicate imports, uncleared deposits, merchant settlements that never tie, and stale suspense items all sit comfortably inside a reconciliation that balances on paper. The harm is delayed but serious: misstated cash, undetected duplicate payments, lost receipts, fraud that goes unnoticed for months, and audit findings that surface only at year-end when they are expensive to fix.

Use this skill before signing off a bank reconciliation, investigating a reconciling item, reconciling a merchant processor, clearing old outstanding checks, or deciding whether a cash account is truly reconciled. The goal is to prevent the agent from treating a zero difference as proof of correctness when it may simply be proof that two wrong numbers were made to match.

Banking rules, check clearing conventions, NSF handling, and merchant settlement practices vary by jurisdiction, currency, and banking system. Do not state a single universal rule. Complex fraud investigation or disputed bank-error resolution should be routed to a qualified accountant or the entity's bank. This skill provides operational reconciliation judgment, not a final determination in any specific dispute.

## Core Rules

### Reconcile To External Evidence, Not To The Ledger

A real reconciliation compares the accounting record to an independent external source: the bank statement, a processor settlement report, or a card network file. Reconciling one internal report to another internal report is not a reconciliation.

Use as the external source:

- the official bank statement for the period;
- the processor settlement report for merchant activity;
- the card network or payment rail confirmation where relevant;
- a loan or facility statement for interest-bearing accounts.

If the only evidence is internal, the reconciliation is not detecting errors or misuse; it is only confirming internal consistency.

### Start From The Prior Reconciled Balance

Each reconciliation should begin from the previously reconciled ending balance and roll forward. If the beginning balance does not agree, nothing downstream can be trusted.

Confirm:

- the opening balance equals the prior period's reconciled closing balance;
- any prior-period adjustment is explained and approved;
- no opening reconciling items were silently cleared;
- the bank statement opening balance matches the prior statement closing.

A reconciliation that starts from a fresh number each month cannot demonstrate continuity and hides prior errors.

### Validate Every Reconciling Item, Not Just The Total

Deposits in transit and outstanding checks are legitimate reconciling items only when they are real, recent, and expected. Old items are a warning sign, not a normal part of the reconciliation.

For each item ask:

- deposits in transit: is the deposit real, dated before period-end, and expected to clear shortly;
- outstanding checks: is the check genuine, issued in the correct period, and still expected to be presented;
- bank errors: is the error documented, reported to the bank, and tracked until corrected;
- NSF or returned items: has the item been reclassified to a receivable or written off, not left sitting in cash;
- bank fees and interest: have they been recorded in the correct period and account.

A reconciling item that has sat for months is no longer a timing difference. It is an unresolved problem.

### Reconcile Merchant Processors Gross To Net

Payment processors settle net of fees, reserves, chargebacks, and refunds. Reconciling only the net deposit to the bank hides the gross activity and makes it impossible to confirm that all sales were captured and all deductions were legitimate.

Reconcile:

- gross sales or batches from the source system;
- refunds and chargebacks processed;
- merchant fees and interchange deductions;
- reserves or holdbacks retained by the processor;
- the net settlement that hits the bank.

A net-to-net reconciliation cannot detect missing sales, duplicate refunds, or unauthorized fee deductions. Gross-to-net reconciliation is what makes processor activity trustworthy.

### Investigate Stale Items Immediately

Reconciling items age. A deposit in transit that has not cleared in a week is normal; one that has not cleared in three months is a red flag. Stale items often indicate duplicates, voided transactions that were never removed, fraud, or lost receipts.

Set aging expectations:

- deposits in transit should clear within a few days;
- outstanding checks become stale after a defined period, often one to three months;
- bank errors should have a resolution timeline with the bank;
- unexplained differences should be escalated, not carried.

Establish a policy to reclassify, write off, or escalate items that exceed the aging threshold. Carrying the same reconciling items month after month is how errors become permanent.

### Record Adjustments With Evidence And Approval

When the reconciliation reveals that the books are wrong, the correction is a journal entry that needs support and approval, not a silent edit. Bank errors are not corrected in the books; they are pursued with the bank.

For each adjustment:

- document the reason and the supporting evidence;
- identify the preparer and an independent reviewer;
- record the entry in the correct account and period;
- reverse or reclassify voided, duplicate, or returned items properly;
- obtain approval for any write-off or reclassification above policy thresholds.

Adjustments to cash are high-risk because cash is the most liquid and most targeted asset. Treat every cash adjustment as needing more scrutiny than a routine expense entry.

### Review Reconciliation Quality, Not Just The Outcome

A reconciliation that balances can still be wrong. Two offsetting errors, a duplicated deposit, a transposed amount, or a fabricated reconciling item can all produce a zero difference. The review should assess the quality of the reconciling items, the aging, the support, and whether the reconciliation actually explains the cash movement.

Review for:

- legitimate, well-supported reconciling items;
- no long-outstanding or unexplained items;
- correct period cutoff for deposits and checks;
- gross-to-net reconciliation for processors;
- independent preparation and review;
- evidence retained for audit.

A reconciliation is complete when the preparer and reviewer understand and can explain every reconciling item, not merely when the difference is zero.

## Common Traps

### Treating Zero Difference As Success

The most dangerous reconciliation is one that always balances on the first try. A genuine reconciliation usually surfaces a few items to investigate. Effortless zeros can indicate that the reconciliation is being forced or that items are being netted to hide differences.

### Reconciling Internal Report To Internal Report

Comparing the general ledger to a subledger or a spreadsheet is not a bank reconciliation. Without an external source, errors and misuse go undetected.

### Net-To-Net Processor Reconciliation

Matching the processor's net deposit to the bank deposit confirms only that the money arrived. It cannot detect missing sales, duplicate refunds, or unauthorized fees deducted by the processor.

### Carrying Stale Items Indefinitely

Old outstanding checks and ancient deposits in transit are often duplicates, voids, or lost receipts that were never cleaned up. Carrying them makes the reconciliation look clean while hiding real problems.

### Leaving NSF And Returned Items In Cash

A check that bounced is no longer cash. It must be reclassified to a receivable, pursued, or written off. Leaving it in the bank reconciliation overstates cash.

### Correcting Bank Errors In The Books

When the bank is wrong, the books should not be adjusted to match. The error must be reported to the bank and tracked until corrected, with the reconciling item retained as evidence.

### One Person Preparing And Approving

If the same person reconciles, approves adjustments, and has payment access, the reconciliation provides no independent assurance. Segregation, or compensating review where impossible, is what makes the control meaningful.

## Self-Check

- Is the reconciliation performed against an external source such as the bank statement or processor settlement report?
- Does the opening balance agree to the prior period's reconciled closing balance?
- Is every reconciling item real, recent, supported, and expected to clear within a reasonable aging window?
- Is merchant processor activity reconciled gross to net, including sales, refunds, chargebacks, fees, and reserves?
- Have stale items been investigated, reclassified, written off, or escalated rather than carried forward?
- Are NSF, returned, and voided items removed from cash and recorded in the correct account?
- Are book adjustments supported, approved, recorded in the correct period, and limited to genuine book errors?
- Are bank errors pursued with the bank and retained as reconciling items rather than corrected in the books?
- Is the reconciliation independently prepared and reviewed, with evidence retained for audit?
- Does the reviewer assess the quality and aging of reconciling items, not merely confirm a zero difference?
