---
name: period_cutoff_and_transaction_dating.md
description: Use when the agent is determining the correct period for a transaction, managing month-end or year-end cutoff, reviewing transaction dating near period boundaries, handling transactions that span periods, or testing whether revenue and expenses are recorded in the correct period.
---

# Period Cutoff And Transaction Dating

Period cutoff is the discipline of making sure every transaction lands in the period it economically belongs to. A sale shipped on the last day of the month belongs in that month. A bill received after month end for services consumed in the month belongs in the month. The transaction date in the system should reflect the economic event, not the date someone happened to enter it. Cutoff errors are insidious because they rarely break a reconciliation or a total. They simply shift revenue or expense between adjacent periods, distorting both, and they accumulate into trend lines that mislead management and misstate reported results. Agents often date transactions based on when they were entered or when the paperwork arrived, rather than when the underlying event occurred, and they treat cutoff as a year-end concern when it matters every period.

Use this skill before dating a transaction near a period boundary, building cutoff procedures, or testing whether transactions fall in the correct period. The goal is to ensure the transaction date reflects the economic event and that each period contains its own activity.

This skill covers cutoff and dating principles. Specific revenue cutoff rules under the applicable revenue standard, and industry-specific cutoff rules, depend on the framework and business model. Confirm the governing requirements before applying these procedures.

## Core Rules

### Date Transactions By Economic Event, Not By Entry Time

The date on a transaction should reflect when the economic event occurred, not when it was keyed into the system. An invoice entered on the fifth of the month for a shipment made on the last day of the prior month belongs in the prior month.

Confirm the correct date by asking:

- when did the underlying event happen, the shipment, the receipt, the service performance, the consumption;
- when did control or risk transfer;
- does the system default date reflect the event or just the entry time;
- is the dating consistent with the entity's cutoff policy.

System default dates are a frequent source of cutoff error because they stamp the entry date rather than the event date.

### Manage The Cutoff Window Around Period End

The days immediately before and after period end are the highest-risk window for cutoff errors. Transactions in this window must be dated and reviewed with extra care.

Build cutoff procedures that:

- identify all transactions in the last few days and first few days of adjacent periods;
- confirm each is dated based on the economic event;
- check that shipments, receipts, and services are on the correct side of the boundary;
- verify that no transactions were held back or pulled forward to manage results.

A defined cutoff procedure, executed every period, prevents the slow drift of transactions into the wrong period.

### Separate Sales Cutoff From Purchase Cutoff

Sales cutoff and purchase cutoff run in opposite directions and must be tested separately.

For sales cutoff:

- goods shipped before period end are revenue in that period;
- goods shipped after period end are revenue in the next period;
- the shipping document date, not the invoice date, is usually the trigger.

For purchase cutoff:

- goods received before period end are inventory or expense in that period, with a corresponding payable;
- goods received after period end belong in the next period;
- the receiving document date, modified by shipping terms, is the trigger.

Testing only one direction leaves half the boundary unguarded.

### Apply Shipping Terms To Goods In Transit

Goods in transit at period end belong to whichever party has the risks and rewards of ownership under the shipping terms. This determines whether they are inventory or cost of sales, and whether a sale or payable is recognized.

Under FOB shipping point, the buyer takes ownership at shipment, so in-transit goods belong to the buyer. Under FOB destination, the seller retains ownership until delivery, so in-transit goods belong to the seller. Applying the wrong terms puts the goods and the related sale or purchase in the wrong period and on the wrong entity's books.

### Handle Transactions That Span Periods

Some transactions naturally span a period boundary. A service performed across the last week of one month and the first week of the next must be allocated to both periods. A subscription billed annually must be recognized over the periods it covers.

For spanning transactions:

- recognize revenue or expense over the period of performance or benefit;
- defer or accrue the portion that belongs to a future or prior period;
- apply the allocation consistently each period;
- document the method so it is applied the same way every time.

Dropping a spanning transaction entirely into one period distorts both periods.

### Verify Cutoff With Source Documents

Cutoff assertions must be supported by source documents. The shipping log, the receiving log, the service completion record, and the bank statement are the evidence that a transaction occurred when claimed.

For cutoff testing, examine:

- shipping documents for the last and first days of the period;
- receiving documents for the same window;
- bank statements for deposits and disbursements near the boundary;
- service or milestone completion records.

A transaction dated near the boundary without supporting source evidence is a cutoff risk.

### Prevent Intentional Cutoff Manipulation

Because cutoff shifts results between periods, it can be used to manage earnings. Holding shipments to push revenue into the next period, or pulling them forward to boost the current period, is manipulation even when the amounts are real.

Controls that deter this include:

- reviewing the pattern of period-end shipping and billing volumes;
- investigating unusual spikes or dips at the boundary;
- requiring approval for period-end dating overrides;
- comparing cutoff results across periods for consistency.

### Reconcile Subledger Cutoff To General Ledger Cutoff

The subledger and the general ledger must reflect the same cutoff. If the receivable subledger records a sale in one period but the GL posts it in another, the two will not tie. Confirm that subledger posting and GL posting use the same event date and period.

## Common Traps

### Dating By Entry Time Or Paperwork Arrival

The system default date, the invoice arrival date, or the entry date often differs from the economic event date. Dating by convenience rather than event shifts transactions between periods.

### Treating Cutoff As A Year-End-Only Task

Cutoff matters every period because every period's results feed into trends, forecasts, and management decisions. Skipping monthly cutoff lets errors accumulate into a larger year-end problem.

### Testing Only Sales Or Only Purchases

Sales and purchase cutoff move in opposite directions. Testing one and not the other leaves half the boundary unverified.

### Ignoring Shipping Terms For In-Transit Goods

Assuming all in-transit goods belong to one party ignores the actual FOB terms. The result is misstated inventory, cost of sales, payables, and revenue.

### Dropping Spanning Transactions Into One Period

A service or subscription that crosses a boundary must be allocated. Putting it all in one period distorts both.

### Holding Or Pulling Transactions To Manage Results

Intentionally shifting transactions across the boundary to smooth or boost results is earnings management. The pattern is detectable and is a serious integrity issue.

### Relying On Dates Without Source Documents

A date asserted without supporting shipping, receiving, or bank evidence is unverifiable. Cutoff must be tied to source documents.

### Subledger And GL Cutoff Mismatch

If the subledger and GL post the same transaction to different periods, they will not reconcile, and the balance sheet will be wrong.

## Self-Check

- Is each transaction dated by the economic event date rather than the entry, invoice arrival, or system default date?
- Is a defined cutoff procedure executed every period for transactions in the boundary window?
- Are sales cutoff and purchase cutoff tested separately, in their opposite directions?
- Are shipping terms, FOB shipping point or destination, applied correctly to goods in transit at period end?
- Are transactions spanning a period boundary allocated across the periods of performance or benefit?
- Is each cutoff assertion supported by source documents such as shipping logs, receiving logs, and bank statements?
- Are period-end shipping and billing patterns reviewed for signs of intentional cutoff manipulation?
- Do the subledger and general ledger reflect the same cutoff, so they reconcile?
- Have the applicable revenue standard and industry-specific cutoff rules been confirmed before relying on these procedures?
