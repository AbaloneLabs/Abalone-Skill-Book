---
name: expense_occurrence_testing.md
description: Use when the agent is testing whether recorded expenses actually occurred, vouching expenses to source documents, matching invoices purchase orders and receiving documents to recorded transactions, or detecting fictitious, duplicate, or personal expenses booked to the company.
---

# Expense Occurrence Testing

Occurrence testing asks whether recorded expenses represent real transactions that actually happened, and it is the primary defense against fictitious expenses, ghost vendors, duplicate payments, and personal costs disguised as business expense. Agents often vouch a recorded expense to an invoice, see that an invoice exists, and conclude occurrence is proven, when the real risks lie in whether the goods or services were received, whether the vendor is genuine, whether the same cost was paid twice, and whether the expense has a legitimate business purpose. A fabricated invoice is trivially easy to create, so the invoice alone is weak evidence; occurrence is proven by the independent corroboration of the full transaction chain.

Use this skill when vouching expenses to source documents, testing the occurrence assertion, or designing procedures to detect fictitious, duplicate, or improper expenses. The goal is to corroborate each recorded expense with independent evidence across the transaction chain, not merely to confirm that a document exists.

## Core Rules

### Vouch The Full Transaction Chain, Not Just The Invoice

An invoice proves that a document was created; it does not prove that goods or services were received or that the obligation is genuine. Occurrence is supported when independent documents corroborate the same transaction: a purchase order initiating it, a receiving record confirming delivery, an approval authorizing it, and a payment settling it. Gaps or inconsistencies in the chain are occurrence exceptions.

For each vouchered expense, obtain and tie together:

- the purchase order or contract initiating the commitment;
- the vendor invoice with matching details;
- the receiving report, service confirmation, or delivery proof;
- the approval and authorization evidence;
- the payment record or check;
- the matching of amounts, quantities, and dates across documents;
- evidence the vendor is genuine and active;
- evidence the goods or services match the business need;
- the general ledger coding and period;
- any exception or variance documentation.

A clean invoice with no receiving or approval evidence is an occurrence failure, not a pass.

### Confirm The Vendor Is Genuine

Fictitious expenses usually flow through fictitious or shell vendors. The vendor master is therefore central to occurrence testing, yet agents rarely test it. A vendor that shares an employee's address, was created shortly before payment, or has no other activity is a strong occurrence red flag.

Test vendor genuineness by:

- reconciling the vendor master to active, legitimate vendors;
- identifying vendors with no activity outside the tested payments;
- matching vendor addresses, banks, and contacts to employee records;
- identifying vendors created shortly before their first invoice;
- testing new-vendor setup approvals and documentation;
- identifying vendors with sequential or duplicate invoice numbers;
- checking for vendors with only one employee able to approve;
- reviewing vendors with post-office-box or virtual-office addresses;
- confirming vendor existence through independent sources;
- identifying dormant vendors reactivated before payment.

A genuine expense requires a genuine vendor; test both.

### Detect Duplicate Payments And Invoices

Duplicate payment is a common occurrence issue because the same legitimate obligation can be paid twice through different invoices, systems, or periods. Duplicates are easy to miss because each individual payment looks valid. The detection requires analytic comparison across the population, not sample selection.

Run duplicate detection for:

- same vendor, invoice number, amount, and date;
- same amount and date across different vendors;
- same invoice with slight variations in number or amount;
- manual payments matching system-generated payments;
- payments across multiple systems or entities;
- invoices split just below approval thresholds;
- credit notes that do not offset the original duplicate;
- payments to the same bank account from different vendors;
- reissued invoices after the original was paid.

Review hits against the payment history and recover confirmed duplicates.

### Rule Out Personal Expenses Booked To The Company

Personal expenses charged to the company are an occurrence failure because the transaction, while real, is not a business expense. These are hardest to detect because they often have valid-looking receipts and approvals. Business purpose is the key test, supported by policy and corroboration.

Assess business purpose for:

- travel and entertainment with vague or no business rationale;
- retail, subscription, or personal-service charges;
- expenses that benefit an individual rather than the entity;
- charges near an approver's home or personal locations;
- recurring charges to personal-style vendors;
- expenses just below reimbursement or approval thresholds;
- charges lacking attendee or purpose documentation;
- personal device, app, or membership charges;
- charges by executives with self-approval authority;
- patterns inconsistent with the employee's role.

A receipt is not proof of business purpose; require the supporting rationale.

### Test For Fictitious Expenses And Ghost Transactions

Fictitious expenses have no underlying economic substance: the vendor may be shell, the service may never have been rendered, or the documentation may be fabricated. These require testing beyond the invoice because the invoice is the fabrication. Look for patterns and corroboration gaps.

Look for:

- round-dollar invoice amounts disproportionate to the population;
- invoices with generic descriptions and no detail;
- services with no measurable deliverable or confirmation;
- vendors invoicing on a predictable schedule with no variation;
- payments approved by the same person who set up the vendor;
- invoices dated weekends, holidays, or after hours;
- supporting documents that appear scanned, copied, or templated;
- gaps in receiving or service-confirmation evidence;
- payments to vendors with no independent online presence;
- expenses recorded through manual journal entries bypassing AP.

Corroborate with independent evidence; never accept the invoice alone.

### Validate Manual And Journal-Entry Expenses

Expenses recorded through manual journal entries bypass the purchase-to-pay controls that normally support occurrence. They are a favored channel for fictitious or improper entries because they require only general ledger access. Manual entries near period end, round-dollar amounts, and unusual accounts warrant focused testing.

For manual expense entries, test:

- the business purpose and supporting documentation;
- the preparer and approver and their segregation;
- the source documents underlying the entry;
- whether the entry bypasses the normal AP process;
- the timing, especially near period end;
- recurring manual entries to the same account;
- entries with vague or generic narratives;
- entries posted by users with override authority;
- the reconciliation of manual entries to supporting detail.

### Match Quantities, Prices, And Terms Across Documents

Occurrence is undermined when documents exist but do not agree. A purchase order for one quantity, a receiving report for another, and an invoice for a third indicate either error or manipulation. Three-way matching is an occurrence control, and exceptions must be investigated.

Reconcile across documents:

- quantities ordered, received, and invoiced;
- unit prices and extended amounts;
- payment terms and due dates;
- freight, tax, and other charges;
- vendor and remit-to details;
- dates of order, receipt, invoice, and payment;
- account coding and cost center;
- currency and exchange rates;
- discounts and early-payment terms;
- variances and their approval.

Unresolved variances are occurrence exceptions, not clerical noise.

### Assess Occurrence In The Correct Period

Occurrence and cutoff overlap: an expense can be real but recorded in the wrong period, which affects both assertions. When testing occurrence, note period errors because they often accompany occurrence manipulation. Period-end accruals and estimates are particularly prone to occurrence issues.

Consider period for:

- accrued expenses with no subsequent settlement;
- estimates and reserves not supported by underlying activity;
- prepaid amortization with no original cost support;
- expenses reversed and re-accrued to shift periods;
- invoices dated in one period and recorded in another;
- period-end manual entries with no follow-up support;
- recurring accruals never trued to actual;
- expenses recorded before goods or services were received;
- the reversal of prior-period accruals in the current period.

### Evaluate Exceptions For Root Cause And Pattern

A single occurrence exception may be isolated or systemic, and the response depends on the distinction. Agents often clear exceptions as one-off errors without assessing whether they reveal a control failure or a scheme. Each exception should be evaluated for cause, pattern, and implication.

For each exception, determine:

- whether documentation is missing or the transaction failed;
- whether the exception is isolated or part of a pattern;
- the monetary impact and the accounts affected;
- the control that should have prevented it;
- whether the same vendor, approver, or preparer recurs;
- whether management's explanation is supported;
- whether additional procedures are warranted;
- whether the exception indicates fraud risk;
- the root cause and recommended remediation.

## Common Traps

### Vouching Only To The Invoice

An invoice is the easiest document to fabricate. Corroborate occurrence with receiving, approval, and payment evidence across the chain.

### Treating A Valid-Looking Receipt As Business Purpose

A receipt proves a charge was made, not that it was a business expense. Require the business rationale, attendees, and policy compliance.

### Skipping The Vendor Master

Fictitious expenses flow through fictitious vendors. Test vendor genuineness, not just the transaction documents.

### Missing Duplicate Payments Across Systems

Duplicates often appear across entities, systems, or manual and automated channels. Run population-wide analytics, not just sample review.

### Accepting Manual Entries Without Support

Manual journal entries bypass purchase-to-pay controls. Test their business purpose and supporting detail with heightened skepticism.

### Clearing Variances As Clerical

Quantity and price variances across documents can indicate manipulation. Investigate variances rather than explaining them away.

### Ignoring Period-End Accruals

Accruals with no subsequent settlement may be fictitious. Confirm accruals reverse or settle to real activity.

### Sampling Away From High-Risk Items

Random samples miss the few fictitious transactions. Target round-dollar, manual, new-vendor, and self-approved items explicitly.

## Self-Check

- Does each vouchered expense tie together the purchase order, invoice, receiving record, approval, and payment?
- Is the vendor confirmed genuine through master-file testing and independent corroboration?
- Have duplicate-payment analytics been run across vendors, systems, entities, and manual channels?
- Are personal expenses tested for business purpose beyond the existence of a receipt?
- Are fictitious-expense indicators such as round-dollar amounts, generic descriptions, and missing receiving evidence investigated?
- Are manual and journal-entry expenses tested for business purpose, preparer/approver segregation, and supporting detail?
- Do quantities, prices, terms, and dates reconcile across the purchase order, receiving report, and invoice?
- Are period-end accruals and estimates confirmed to settle or reverse to real activity?
- Is each occurrence exception evaluated for root cause, pattern, monetary impact, and fraud implication?
- Does the conclusion distinguish transactions that occurred from those merely documented, and does it address targeted high-risk items?
