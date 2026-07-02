---
name: payment_processing_and_merchant_accounts.md
description: Use when the agent is accounting for payment processors and merchant accounts, reconciling gross to net settlement, recording merchant fees and chargebacks, handling reserves and holdbacks, matching ACH, card, PayPal, or Stripe settlements, or investigating unremitted funds and timing differences.
---

# Payment Processing And Merchant Accounts

Payment processors sit between the entity and its bank, and that middle position is where a large share of reconciliation errors hide. Processors settle net of fees, retain reserves, reverse chargebacks, net out refunds, and operate on their own settlement timing. When the accounting simply records the net deposit that hits the bank, the entity loses visibility into gross sales, cannot confirm that every transaction was captured, cannot verify that fees and chargebacks are legitimate, and cannot detect missing or duplicated settlements. Over time, small unexplained differences accumulate into material misstatements that are painful to unwind.

Use this skill before recording processor settlements, reconciling a merchant account, investigating a chargeback or reserve, confirming that all settlement batches landed, or closing a period that includes card or digital wallet activity. The goal is to prevent the agent from treating the processor's net deposit as the complete record when the gross activity and every deduction must be independently verified.

Processor rules, chargeback timeframes, reserve arrangements, and settlement conventions vary by processor, payment rail, jurisdiction, and currency. Do not state a single universal rule. Complex disputes, reserve negotiations, or suspected processor errors should be routed to a qualified accountant or the processor directly. This skill provides operational reconciliation and recording judgment, not a final resolution of any specific processor dispute.

## Core Rules

### Reconcile From Gross Activity, Not From The Net Deposit

The net deposit is the end of the story, not the whole story. A trustworthy reconciliation starts from gross sales or transaction batches and walks down to the net settlement, confirming every component.

Build the walk:

- gross sales or authorized batches from the order or point-of-sale system;
- refunds and voided transactions;
- chargebacks and retrievals;
- processor and interchange fees;
- reserves or holdbacks retained;
- adjustments, corrections, and recovered items;
- the net settlement that reaches the bank.

If the reconciliation only matches net to net, none of the deductions are verified and missing sales or unauthorized deductions pass undetected.

### Confirm Settlement Timing And Cut-off

Processors settle on their own schedule, often daily or rolling, and the settlement date rarely aligns perfectly with the entity's period-end. Unremitted batches, in-transit settlements, and timing differences must be identified and recorded in the correct period.

Confirm:

- which batches have settled into the bank and which are still in transit;
- whether period-end sales are recorded in the correct period even if settled after period-end;
- whether a settlement spanning two periods is split correctly;
- whether held or delayed settlements are tracked and followed up.

A batch that sold before period-end but settled after must be recorded in the earlier period as a receivable or deposit in transit, not pushed into the next period.

### Record Fees, Chargebacks, And Refunds In The Right Accounts

Each deduction has a different accounting character and should land in its own account, not be buried in a single "merchant fees" line.

Separate:

- processing and interchange fees as an operating expense;
- refunds as a reduction of revenue or a contra-revenue account, depending on policy;
- chargebacks as a reduction of revenue or a separate loss account;
- reserves and holdbacks as a receivable or restricted asset, not an expense;
- recovered chargebacks or released reserves as a reversal of the original entry.

Mixing these into one account destroys the ability to analyze fee trends, chargeback rates, and refund behavior, and it makes audit support nearly impossible.

### Track Reserves And Holdbacks Until Released

Many processors retain a rolling reserve or holdback to cover future chargebacks. This is the entity's money, temporarily withheld, and it must be tracked as a receivable or restricted asset until released.

Track:

- the reserve percentage and the retention period;
- the cumulative amount held;
- the scheduled release dates;
- releases back to the bank or offset against chargebacks;
- any reserve that is forfeited or never released.

A reserve that is simply netted out and forgotten is either overstating expenses or understating assets, depending on how it was originally recorded.

### Investigate Unremitted Funds And Unexplained Differences

When gross activity does not walk cleanly to the net settlement, the difference is a signal, not a rounding issue. Unremitted funds, failed settlements, processor errors, and duplicated batches all show up as unexplained gaps.

Investigate:

- settlements that were authorized but never reached the bank;
- duplicate settlement files or double-imported batches;
- processor adjustments with no supporting detail;
- chargebacks or fees that exceed expected patterns;
- currency conversion differences on multi-currency settlements.

Every unexplained difference should have an owner, a target resolution date, and a documented outcome. Differences that are quietly written off hide both errors and misuse.

### Reconcile Each Processor And Payment Rail Separately

Card, ACH, digital wallet, and bank-transfer processors behave differently. Netting them together into one reconciliation obscures the unique timing, fee structure, and risk of each rail.

Reconcile separately:

- card networks and acquirers;
- ACH or direct debit processors;
- digital wallets and third-party processors;
- buy-now-pay-later or installment providers;
- marketplace or platform settlements with netting.

Each rail has its own settlement file, fee structure, chargeback behavior, and reserve terms. Separate reconciliation is what makes each one auditable.

### Retain Settlement Files As Audit Evidence

The processor's settlement report is the primary evidence that gross activity was captured and deductions were legitimate. It must be retained and tied to the accounting entries.

Retain:

- the settlement report for each period;
- the fee breakdown and reserve movement;
- chargeback notifications and resolution evidence;
- refund and void records;
- the bank deposit confirmation linking settlement to cash.

Without these files, a later question about a fee, chargeback, or reserve cannot be answered, and the reconciliation becomes unverifiable.

## Common Traps

### Recording Only The Net Deposit

The most common and damaging error. Net-to-net recording hides gross sales, makes fees and chargebacks unverifiable, and prevents detection of missing or duplicated settlements.

### Burying All Deductions In One Fee Account

When refunds, chargebacks, reserves, and processing fees all land in one account, management cannot analyze trends and auditors cannot verify components. Each deduction type needs its own home.

### Forgetting Held Reserves Are Still An Asset

A rolling reserve is the entity's money withheld by the processor. Recording it as an expense understates assets and overstates costs. It must be tracked until released.

### Ignoring Settlement Timing At Period-End

Sales that occurred before period-end but settled after are often pushed into the next period, understating prior-period revenue and overstating the receivable. Cut-off must be applied to settlements, not just to invoices.

### Treating Unexplained Differences As Rounding

Processors rarely round in ways that produce persistent gaps. An unexplained difference is usually a missing batch, a duplicate, a processor error, or an unauthorized deduction, and it must be investigated.

### Reconciling All Processors Together

Combining card, ACH, and wallet activity into one reconciliation hides the distinct behavior of each rail and makes it impossible to isolate where a difference originates.

### Losing The Settlement File

If the processor's settlement report is not retained, no later question about fees, chargebacks, or reserves can be answered. The reconciliation becomes unsupported assertion.

## Self-Check

- Is each processor reconciled from gross sales down to net settlement, with every component verified?
- Are refunds, chargebacks, processing fees, and reserves recorded in separate accounts rather than one combined line?
- Are reserves and holdbacks tracked as a receivable or restricted asset until released, not expensed when withheld?
- Has settlement timing been reconciled to period-end so that pre-period-end sales are recorded in the correct period?
- Are unremitted funds, failed settlements, and unexplained differences investigated and resolved rather than written off?
- Is each payment rail, such as card, ACH, and digital wallet, reconciled separately?
- Are settlement reports, fee breakdowns, chargeback notices, and refund records retained and tied to the entries?
- Have duplicate batches and double-imported settlements been identified and removed?
- Have processor-specific reserve terms, chargeback windows, and settlement schedules been confirmed for each account?
- Would an independent reviewer be able to walk from gross activity to the bank deposit using retained settlement files?
