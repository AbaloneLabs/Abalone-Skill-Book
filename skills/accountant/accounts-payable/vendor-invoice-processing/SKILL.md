---
name: vendor_invoice_processing.md
description: Use when the agent is processing vendor invoices, setting up supplier records, entering bills into accounts payable, matching purchase orders to receipts and invoices, approving supplier payments, or reviewing whether payables data is accurate and properly supported before posting to the ledger.
---

# Vendor Invoice Processing

Accounts payable is not just data entry. Each vendor invoice is a claim against the business that, once posted, creates a liability, may trigger a tax obligation, and becomes part of the evidence trail behind expense recognition, cost accumulation, and cash disbursements. Weak invoice processing looks efficient in the moment because invoices get entered quickly, but it fails when someone asks whether the business actually received the goods, whether the price was authorized, whether the vendor was legitimate, whether sales tax or VAT was handled correctly, or whether the same invoice was paid twice.

Use this skill before entering vendor invoices, configuring approval workflows, performing three-way matching, onboarding new suppliers, handling credit memos, preparing payment runs, or reviewing whether an accounts payable dataset is reliable enough to feed the general ledger and the close. The goal is to prevent the agent from posting liabilities that are unsupported, unauthorized, duplicated, or misstated.

This guidance is jurisdiction-neutral. Sales tax, VAT, GST, withholding tax, and retention rules differ by country, state, and entity type. Where a specific treatment is uncertain, flag it for a qualified tax or accounting professional rather than guessing.

## Core Rules

### Establish Vendor Identity Before The First Invoice

A supplier record is the anchor for every subsequent invoice, payment, and tax form. Creating a vendor casually invites fraud, duplicate records, and misdirected payments.

Before entering the first invoice, capture and verify:

- legal entity name and any doing-business-as name;
- remittance address, distinct from a service address if different;
- tax identification number or VAT/GST registration number;
- payment instructions and banking details, verified independently of the vendor's own email;
- W-9, W-8, or equivalent tax status documentation where required;
- contact person for billing disputes;
- payment terms and any negotiated discounts;
- whether the vendor is related to the business or to an owner.

Banking details deserve special caution. Changes to remittance instructions are a common fraud vector. Verify any change through a known contact or a second channel before acting on it, and never accept a banking change that arrives only through email.

### Apply Three-Way Matching For Goods And Services

An invoice is a request for payment, not proof that the obligation exists. For purchases that flow through a purchasing system, confirm that the invoice agrees with what was ordered and what was received.

A three-way match compares:

- purchase order: authorized price, quantity, terms;
- receiving record or service confirmation: goods or services actually delivered;
- vendor invoice: billed amount, quantities, terms.

Discrepancies should be resolved before payment, not after. Common mismatches include quantity differences, unit price changes, unauthorized freight or handling charges, billed-but-not-received items, and invoicing for a period that has not yet occurred. For low-value or recurring services, a documented policy may permit two-way matching or no-match processing, but the policy should state the threshold and the review expectation explicitly.

### Control The Approval Path

No invoice should be paid solely because it arrived. Approval confirms that the charge is legitimate, within budget, correctly coded, and supported by evidence appropriate to its size and risk.

Design approval so that:

- the person who approves is not the same person who can enter or pay the invoice;
- approval thresholds escalate for larger amounts;
- approvals are captured in writing or in a system audit trail;
- recurring subscriptions and auto-renewals still receive periodic review;
- owner or executive expenses are approved by someone other than the person who incurred them.

Approval is not a rubber stamp. If an approver cannot see the supporting evidence, the approval is not meaningful.

### Post To The Correct Period And Account

The timing and classification of an invoice affect expense recognition, accruals, cost of goods sold, and tax reporting. An invoice entered in the wrong period distorts margins and trends.

Check:

- invoice date versus goods or services received date versus accounting period;
- whether the cost is an expense, a prepaid asset, inventory, a fixed asset, or a loan;
- whether the cost relates to a specific project, department, or customer that requires allocation;
- whether sales tax, VAT, or GST is recoverable or is part of cost;
- whether withholding tax applies to the payment.

If an invoice arrives after period end but relates to a closed period, route it for review rather than silently booking it to the current period.

### Detect And Prevent Duplicate Invoices

Duplicate payment is one of the most common and most preventable losses in accounts payable. The same invoice can arrive by mail, by email, through a portal, and as a statement copy, and each copy can look like a new bill.

Build duplicate detection around:

- vendor number plus invoice number plus invoice date;
- similar amounts and dates across different invoice numbers;
- re-invoicing after a dispute or credit memo;
- statement entries that duplicate already-paid invoices;
- invoices re-submitted after a payment delay.

When a vendor disputes nonpayment, confirm whether the original invoice was received and paid before issuing a second payment. Never assume a vendor's aging report is correct without checking your own records.

### Handle Credit Memos And Adjustments Deliberately

Credit memos reduce what is owed, but they are easy to lose, easy to apply to the wrong invoice, and easy to forget at period end. Track open credits explicitly and reconcile them to vendor statements.

For each credit, capture the reason, the vendor, the original invoice if applicable, the amount including and excluding tax, and the approval. Apply credits before generating payment so that the payment reflects the net obligation. Do not net a credit against an unrelated invoice without documenting the linkage.

### Reconcile To Vendor Statements

Vendor statements are an independent check on internal payables records. Reconciling them catches missing invoices, unrecorded credits, payments the vendor has not yet reflected, and disputes over balances.

Reconcile at least periodically for material vendors. Investigate items that appear on the vendor statement but not in the ledger, and items that appear in the ledger but not on the statement. Aging differences caused by timing are normal; unexplained differences are not.

## Common Traps

### Paying From The Invoice Without Matching

Paying an invoice because it looks correct is not the same as verifying it. Without matching and approval, the business pays for goods never received, services never performed, unauthorized price increases, and fraudulent invoices.

### Trusting Vendor Aging Reports During Disputes

When a vendor claims a invoice is unpaid, the vendor's aging report reflects the vendor's view, which may be wrong, stale, or manipulated. Always check internal records and proof of payment before acting.

### Losing Track Of Recurring And Auto-Charge Vendors

Subscriptions, software, and auto-renewing services keep charging whether or not anyone reviews them. Without periodic review, the business pays for unused services, former employees' accounts, and duplicate tools.

### Ignoring Tax Handling On Invoices

Sales tax, VAT, GST, use tax, and withholding tax each have different rules. Recording the invoice total without separating recoverable tax from expense can understate or overstate both cost and tax liability. If the vendor's tax registration is missing or invalid, flag it.

### Booking Everything To The Current Period

Invoices that arrive after period close are often booked to whenever they are entered, distorting the period that should bear the cost. Build a cutoff routine so that late invoices are accrued or booked to the correct period.

### Assuming A Vendor's Banking Change Is Genuine

Fraudsters impersonate vendors and request changes to remittance details. A change made without independent verification can redirect a large payment to a criminal account. Always confirm banking changes through a known contact or channel.

## Self-Check

- Is each vendor's identity, tax status, and payment information verified and documented before the first payment?
- Do material invoices for goods and services pass a documented three-way or two-way match before payment?
- Is approval segregated from invoice entry and payment, with thresholds and an audit trail?
- Are invoices posted to the correct period, account, project, and tax category rather than defaulted to the entry date?
- Are duplicate invoices detected through vendor, invoice number, date, and amount comparison?
- Are credit memos tracked, approved, and applied before payment runs?
- Are vendor statements reconciled periodically for material suppliers?
- Are banking-detail changes verified independently before any payment is released?
- Are tax treatments on invoices separated and reviewed rather than absorbed into a single total?
- Are late-arriving invoices routed for period review instead of silently booked to the current period?
