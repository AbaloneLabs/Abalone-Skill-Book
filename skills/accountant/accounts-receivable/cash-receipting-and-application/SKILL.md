---
name: cash_receipting_and-application.md
description: Use when the agent is recording customer payments, applying cash receipts to open invoices, handling unapplied or unidentified cash, managing lockbox or payment-processor deposits, reconciling customer remittances, or reviewing whether incoming cash is matched to the correct customer and invoice before the receivable balance is reduced.
---

# Cash Receipting And Application

Receiving cash is not the end of the receivables process; applying it correctly is. When a customer payment arrives, it must be recorded as cash received, identified as belonging to a specific customer, and matched to the specific invoice or invoices it settles. If any of those steps is weak, the receivables ledger drifts from reality: customers appear to owe money they have paid, credits accumulate against the wrong invoices, and collection effort is wasted chasing balances that no longer exist. Weak cash application looks efficient because the bank balance goes up, but it fails when a paid customer receives a dunning notice, when a large unapplied cash pool grows without explanation, or when a payment is applied to whatever invoice makes the account look current rather than the invoice the customer actually paid.

Use this skill before recording customer receipts, configuring lockbox or remittance processing, handling unidentified cash, applying payments in bulk, reconciling remittance advice to bank deposits, or reviewing whether the receivables ledger reflects real settlement. The goal is to prevent the agent from reducing customer balances based on cash that was misidentified, misapplied, or not yet truly received.

Payment rails, chargeback rules, and remittance conventions differ by country and channel. Where a receipt involves foreign currency, payment processors, or marketplace net deposits, confirm the gross and fee breakdown before applying.

## Core Rules

### Record Cash When It Is Truly Received

The trigger for recording a receipt is when the business obtains control of the funds, which depends on the payment instrument. An electronic transfer is generally received when it settles to the bank; a check is received when collected, not merely when mailed; a card payment is received when settled by the processor, net of fees.

Before recording, confirm:

- the funds have cleared or are irrevocably available;
- the amount, currency, and value date;
- the payer identity, which may differ from the customer name;
- the remittance detail identifying which invoices are being paid.

Recording a receipt before collection, or recording a processor's gross amount while only the net settles, creates reconciliation breaks and overstated cash. Match the recording trigger to the instrument.

### Identify The Customer And The Invoices Paid

A bank deposit without remittance detail is only half the information. The receipt must be tied to a customer and to specific open invoices before the receivable is reduced.

Build identification around:

- remittance advice from the customer (email, portal, EDI, letter);
- the reference or invoice number quoted in the payment;
- the payer name matched to the customer master;
- the amount matched to one or more open invoices, allowing for short pays and overpays;
- deductions for discounts, claims, or unauthorized short payments.

When remittance is missing, research the payment before guessing. Applying an unidentified receipt to the oldest invoice, or to the invoice that makes the account current, hides the real state and corrupts aging.

### Handle Unapplied And Unidentified Cash Actively

Not every receipt can be matched immediately. Customer prepayments, missing remittances, and ambiguous references all create unapplied cash. The danger is not the existence of unapplied cash; it is letting it age without resolution.

Manage unapplied cash by:

- recording it in a dedicated suspense or unapplied account, not against a random invoice;
- assigning each item an owner and a research deadline;
- contacting the customer or bank for remittance detail;
- clearing items as they are identified, with the application documented;
- reviewing the unapplied balance at every close for aging and growth.

A growing unapplied cash balance is a signal that the application process is falling behind or that customers are paying in ways the system cannot parse. Treat growth as a process issue, not a storage issue.

### Reconcile Remittance To The Bank Deposit

The total of customer remittances applied in a period should reconcile to the cash deposited and cleared in the bank, net of processor fees, chargebacks, and currency differences. This reconciliation catches missing receipts, duplicate applications, and receipts recorded but never deposited.

Reconcile:

- daily or per deposit batch where volume is high;
- gross remittance versus net deposit for processor and marketplace flows;
- fees, reserves, and holdbacks as separate components;
- chargebacks and reversals as reductions, not new entries;
- currency conversion at the correct rate and date.

When the applied total does not reconcile to the bank, the difference is a real error somewhere in the chain, not a rounding tolerance to be written off.

### Apply Customer Deductions Deliberately

Customers often pay less than the invoiced amount, taking discounts, claiming shortages, deducting for returns, or withholding for disputes. Each deduction is a decision point: it may be valid, it may be a negotiation, or it may be unauthorized.

For each deduction, capture:

- the invoice and the deducted amount;
- the stated reason (discount, return, claim, pricing, damage);
- whether the deduction is authorized under terms;
- the disposition: agreed, disputed, or to be recovered;
- the accounting effect, which may split the receipt between cash, a discount, a write-off, or a claim receivable.

Silently accepting every short pay trains customers to deduct freely. Silently rejecting every short pay damages relationships. Process deductions deliberately with evidence and follow-up.

### Reconcile Processor And Marketplace Deposits Gross

Payment processors, card networks, and marketplaces often deposit a net amount after fees, reserves, refunds, and chargebacks. Recording only the net deposit understates both revenue and fee expense, and it makes reconciliation to gross sales impossible.

For each processor deposit, break out:

- gross sales or settled transactions;
- processor fees and interchange;
- reserves or rolling holds;
- refunds and chargebacks;
- currency conversion;
- net cash deposited.

Match the gross components to the originating sales and the fee components to the correct expense accounts. Net-only recording is a common cause of understated revenue and distorted margin.

### Preserve The Audit Trail Of Application

Every application of cash to an invoice should be traceable: which receipt, which customer, which invoice, when, and by whom. This trail supports disputes, audits, and customer service inquiries.

Retain:

- the bank record or processor settlement;
- the remittance advice;
- the application transaction with timestamp and user;
- any manual override of an automatic match, with reason.

When a customer later disputes a balance, the application trail is the evidence that the payment was received and correctly applied.

## Common Traps

### Applying Unidentified Cash To The Oldest Invoice

When remittance is missing, applying the cash to the oldest open invoice makes the account look current but misstates which invoices are settled. The real paid invoice stays open, inviting duplicate collection and dispute. Research before applying.

### Recording Net Processor Deposits As Revenue

Recording only the net deposit from a card processor or marketplace hides the gross sales and the fees. This understates revenue and expense, distorts ratios, and breaks reconciliation to sales reports. Always break out gross and fees.

### Letting Unapplied Cash Age Silently

Unapplied cash that sits for weeks or months often represents customer prepayments that should be recognized as deferred revenue, or payments that belong to a different customer. Aging unapplied cash inflates cash and misstates receivables. Clear it actively.

### Treating Every Short Pay As A Discount

When a customer pays less, the difference is not automatically a valid discount. It may be an unauthorized deduction, a claim, or a dispute. Categorize each deduction and follow up rather than writing it off by default.

### Recording A Check When Mailed Rather Than Collected

A customer check in the mail is not yet collected cash. Recording it as received before it clears can overstate cash and understate the receivable, and a returned check creates a reversal that must be handled cleanly. Record on collection.

### Assuming The Payer Name Equals The Customer

Payments often arrive from factoring companies, parent entities, payment platforms, or individuals paying on behalf of a business customer. Matching by payer name alone can apply cash to the wrong customer. Use remittance detail and references.

## Self-Check

- Is each receipt recorded only when the business has control of irrevocable funds, with the trigger matched to the payment instrument?
- Is every receipt identified to a customer and matched to specific invoices using remittance detail rather than assumption?
- Is unapplied and unidentified cash held in a dedicated account, owned, researched, and cleared rather than aged silently?
- Does the total of applied remittances reconcile to bank deposits, with processor fees, reserves, and chargebacks broken out separately?
- Are customer deductions categorized, documented, and followed up rather than accepted or rejected by default?
- Are processor and marketplace deposits recorded gross with fees and adjustments as separate components?
- Is the application of cash to invoices traceable to a receipt, a remittance, a timestamp, and a user?
- Are foreign-currency receipts converted at the correct rate and date, with gains and losses recorded?
- Are returned, reversed, or charged-back payments handled cleanly with the receivable reinstated where appropriate?
- Is a growing unapplied cash balance treated as a process failure requiring root-cause resolution?
