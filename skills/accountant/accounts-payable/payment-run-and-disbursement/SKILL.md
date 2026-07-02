---
name: payment_run_and_disbursement.md
description: Use when the agent is preparing a payment run, selecting invoices for payment, releasing vendor disbursements, scheduling electronic transfers or checks, managing payment approvals, stopping or voiding payments, or reviewing whether cash outflows are authorized, timed correctly, and traceable to supported liabilities.
---

# Payment Run And Disbursement

A payment run is the moment when recorded liabilities become actual cash outflows. It is also one of the highest-risk routine activities in accounting, because it converts internal records into irreversible money movement. A clean payment run depends on everything that came before it: correct vendor records, matched and approved invoices, accurate banking details, and a selection logic that pays the right amount to the right party at the right time. Weak disbursement processes look smooth because payments go out on schedule, but they fail when money is sent to the wrong account, when discounts are missed, when early or late payments damage terms, or when a fraudulent invoice slips into the run.

Use this skill before building a payment proposal, releasing a batch of payments, scheduling recurring disbursements, handling payment exceptions, voiding or stopping checks, reconciling paid items, or reviewing whether a payment process is safe enough to operate repeatedly. The goal is to prevent the agent from releasing cash that is unsupported, misdirected, duplicated, or mistimed.

Payment systems, banking rails, and regulatory requirements differ by country and currency. Where a disbursement involves cross-border transfer, sanctions screening, or unfamiliar payment instruments, confirm the requirements with a qualified professional rather than assuming domestic rules apply.

## Core Rules

### Build The Payment Proposal From Supported Liabilities

A payment run should start from invoices that are approved, matched, and due. It should not start from a vendor's request, an email, or a balance that has not been reconciled.

Before selecting invoices for payment, confirm:

- the invoice is fully approved and not on hold;
- any required three-way match is complete;
- credit memos and unapplied payments are netted;
- disputed items are excluded or explicitly resolved;
- the vendor's banking and remittance details are current and verified;
- payment terms, due dates, and early-payment discounts are reflected in the selection logic.

The proposal is a recommendation, not a command. Every item in it should be reviewable against its source before release.

### Time Payments To Terms, Not To Convenience

Paying too early wastes cash and forgoes discounts in the wrong direction; paying too late damages vendor relationships and may trigger late fees or lost early-payment discounts. The selection logic should reflect the business's payment policy.

Consider:

- net term due dates versus early-payment discount windows;
- the value of taking a discount versus holding cash;
- vendor concentration and relationship importance;
- cash availability and forecast;
- weekend and holiday cut-offs that shift effective payment dates.

Document the policy so that timing decisions are consistent rather than ad hoc. If the business deliberately pays a key vendor early or holds a payment for cash reasons, that should be a conscious decision, not an accident of the run.

### Segregate Duties Across The Disbursement Cycle

The person who can release a payment should not be the same person who set up the vendor, entered the invoice, or approved it. Segregation of duties is the primary control against both fraud and undetected error in disbursements.

At minimum, separate:

- vendor setup and banking changes;
- invoice entry and approval;
- payment proposal generation;
- payment release or signing authority;
- bank reconciliation of cleared items.

Where staffing makes full segregation impossible, implement compensating controls such as supervisory review of the payment run, dual authorization above a threshold, and periodic independent review of vendor master changes and payment exceptions.

### Verify Banking And Beneficiary Details Before Release

The single most damaging disbursement error is sending funds to the wrong account. Before releasing a batch, confirm that the beneficiary name and account match the verified vendor record.

Watch for:

- recent banking changes that have not been independently verified;
- vendors with names that differ between the invoice, the master record, and the bank;
- offshore or third-party accounts that do not match the vendor's country;
- payments to individuals when the vendor is a company, or vice versa;
- sudden requests to redirect a routine payment to a new account.

Any anomaly should stop the payment until verified. The cost of a delayed legitimate payment is far smaller than the cost of an irreversible fraudulent transfer.

### Control Payment Instruments By Risk

Different payment instruments carry different risk and different reversibility. Match the instrument to the amount and urgency.

- Electronic transfers (ACH, wire, SEPA, real-time rails) are fast and often irreversible; they require the strongest pre-release verification.
- Checks are slower and can be stopped, but they carry forgery, alteration, and mail-interception risk.
- Corporate cards are convenient for small purchases but bypass the invoice-matching control if not reconciled.
- Cash and manual disbursements should be avoided or tightly controlled.

For wires and same-day transfers above a threshold, require dual authorization. For checks, use positive pay and secure stock. For cards, reconcile transactions to receipts and statements monthly.

### Handle Exceptions, Holds, And Stops Deliberately

A payment run is never perfectly clean. Invoices on hold, partial payments, disputed amounts, and banking errors all create exceptions that must be resolved, not ignored.

For each exception, capture:

- the reason for the hold or partial payment;
- the person who authorized the exception;
- the expected resolution and follow-up date;
- the impact on vendor terms and discounts.

For stopped or voided payments, document the action, ensure the underlying liability is reinstated if still owed, and confirm the stop or void is reflected in both the ledger and the bank reconciliation. A voided check that is not recorded leaves a dangling liability and a reconciliation break.

### Reconcile Released Payments To The Bank

The payment run is not complete when the batch is released; it is complete when the funds clear the bank and reconcile to the ledger. Reconcile cleared payments promptly to detect errors, fraud, and timing differences.

Investigate:

- payments that have not cleared within the expected window;
- cleared amounts that differ from the released amount;
- duplicate clearings of the same invoice;
- payments to vendors not in the master file.

Prompt reconciliation is also the control that catches a diverted payment before the trail goes cold.

## Common Traps

### Releasing The Run Without Reviewing The Proposal

When a payment proposal is large or routine, it is tempting to release it in bulk. But bulk release bypasses the review that catches duplicate invoices, wrong vendors, and banking errors. Review the proposal at a level of detail appropriate to the risk.

### Assuming Recurring Payments Are Still Correct

Standing instructions and recurring transfers keep paying until someone stops them. Without periodic review, the business pays for cancelled services, expired leases, and former vendors. Review recurring payments on a schedule.

### Letting Urgency Override Verification

A vendor demanding immediate payment, a late fee threat, or an executive request can pressure staff to skip verification. Urgency is itself a fraud signal. A legitimate urgent payment can still wait long enough to confirm the banking details.

### Paying Disputed Invoices To Keep The Vendor Happy

Paying a disputed invoice to preserve a relationship, then seeking a credit later, forfeits leverage and often results in the dispute being forgotten. Resolve disputes before payment, or pay the undisputed portion and document the hold.

### Forgetting To Reinstate Liabilities After Voids And Stops

When a check is voided or a payment is stopped, the underlying obligation usually still exists. If the liability is not reinstated, the vendor appears paid when it is not, and the reconciliation breaks. Always confirm the accounting effect of a void or stop.

### Ignoring Sanctions And Beneficiary Screening For Cross-Border Payments

Cross-border disbursements may require sanctions, anti-money-laundering, or beneficiary screening depending on jurisdiction and counterparties. Assuming domestic controls are sufficient can create legal exposure. Flag unfamiliar foreign beneficiaries for review.

## Self-Check

- Does the payment proposal start only from approved, matched, and due invoices, with credits and holds reflected?
- Are payment timings driven by documented terms and discount analysis rather than convenience?
- Are vendor setup, invoice approval, payment release, and bank reconciliation segregated or covered by compensating controls?
- Are beneficiary names and banking details verified against the master record before each release, with recent changes independently confirmed?
- Are high-value and irreversible instruments subject to dual authorization and strong pre-release checks?
- Are payment exceptions, holds, partial payments, and stops documented with reasons and resolution dates?
- Are voided and stopped payments reinstated as liabilities where the obligation still exists?
- Are cleared payments reconciled to the bank and ledger promptly, with uncleared items investigated?
- Are recurring and standing payments reviewed periodically to catch obsolete disbursements?
- Are cross-border payments screened for sanctions and beneficiary requirements appropriate to the jurisdiction?
