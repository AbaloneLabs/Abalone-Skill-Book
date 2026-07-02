---
name: procure_to_pay_and_order_to_cash.md
description: Use when the agent is designing or improving the procure-to-pay or order-to-cash process, mapping the flow from purchasing through invoice payment or from sales order through cash collection, or building controls and efficiency into P2P and O2C processes that feed the general ledger.
---

# Procure To Pay And Order To Cash

Procure-to-pay (P2P) and order-to-cash (O2C) are the two largest transactional flows that feed the general ledger. P2P runs from requisition through purchase order, receipt, invoice, and payment. O2C runs from sales order through fulfillment, billing, collection, and cash receipt. Together they generate most of the accounts payable and accounts receivable activity, and most of the cash movement, in the business. When these processes are well designed, payables and receivables are accurate, cash flow is predictable, and the ledger reflects reality. When they are poorly designed, the business overpays, collects late, carries duplicate or unauthorized transactions, and spends the close reconciling chaos instead of confirming balances.

Use this skill before designing or reengineering P2P or O2C processes, mapping the transaction flow, or building controls and efficiency into purchasing, payables, billing, or collections. The goal is to prevent the agent from designing processes that move transactions quickly but lack the controls and reconciliations that make the resulting ledger reliable.

## Core Rules

### Map Each Process End To End With Control Points

P2P and O2C must be mapped from origin to final recording, with control points identified at each risk.

For P2P map:

- requisition and budget check;
- purchase order approval and vendor authorization;
- goods or services receipt and inspection;
- invoice receipt and three-way match to PO and receipt;
- approval for payment and segregation from PO creation;
- payment execution and recording;
- reconciliation of AP subledger to the GL.

For O2C map:

- sales order and customer credit check;
- fulfillment or service delivery and evidence of performance;
- billing and revenue recognition at the correct point;
- receivable recording and aging;
- collection activity and dispute resolution;
- cash receipt and application;
- reconciliation of AR subledger to the GL.

A map without control points at each risk produces a process that moves paper but does not protect the business.

### Enforce The Three-Way Match In P2P

The three-way match, purchase order, receipt, and invoice, is the primary control against overpayment, duplicate payment, and unauthorized purchasing. It should be enforced, not optional.

Enforce the match by:

- requiring a valid PO before commitment;
- confirming receipt of goods or services before invoice approval;
- matching invoice price, quantity, and terms to the PO and receipt;
- routing exceptions, mismatches, and over-limit invoices for approval;
- blocking payment where the match fails until resolved.

A P2P process that pays invoices without a match, or that allows overrides without review, is a primary source of loss and fraud.

### Control Vendor Master Data And Onboarding

Vendor master data is a major fraud and error vector. Fake vendors, duplicate vendors, and unauthorized bank detail changes are classic schemes.

Control vendor data by:

- verifying vendor identity and legitimacy at onboarding;
- segregating vendor creation from invoice entry and payment;
- requiring approval for new vendors and changes;
- independently verifying bank detail changes through a known channel;
- periodically reviewing the vendor master for duplicates and inactives.

A process that lets anyone create a vendor, or that accepts bank detail changes by email without verification, is vulnerable to payment fraud.

### Manage Customer Credit And Billing Accuracy In O2C

O2C begins before the sale. Credit decisions and billing accuracy determine whether revenue becomes cash.

Manage:

- customer credit approval before order acceptance;
- accurate billing based on contract terms and delivered quantities;
- revenue recognition at the correct point under the applicable framework;
- timely invoicing to start the receivable clock;
- dispute and short-pay resolution to protect collectability.

Billing errors and delayed invoicing extend days sales outstanding and increase bad debt risk. Credit decisions made after the sale are too late.

### Drive Collections And Cash Application

Revenue that is not collected is not value. O2C must include active collection and accurate cash application.

Drive collections by:

- aging receivables and prioritizing follow-up;
- defining collection actions by aging bucket;
- resolving disputes and deductions promptly;
- escalating severely past-due accounts;
- assessing credit loss and maintaining an adequate allowance.

Apply cash accurately by:

- matching receipts to open invoices;
- handling unapplied cash and on-account items promptly;
- investigating short payments and deductions;
- reconciling the bank deposit to the AR posting.

Unapplied cash and unresolved deductions distort the receivable balance and obscure collection performance.

### Reconcile Subledgers To The GL Every Period

P2P and O2C feed the AP and AR subledgers, which must reconcile to the GL every period. This is the proof that the processes produced reliable ledger impact.

Reconcile:

- AP subledger to the AP control account;
- AR subledger to the AR control account;
- payment and receipt activity to bank postings;
- accruals for received-not-invoiced goods and services;
- the allowance for credit losses to the aging.

Breaks indicate process or configuration errors. Investigate and resolve, rather than adjusting to force a tie.

### Design For Efficiency Through Automation And Root Cause

P2P and O2C are high-volume processes where efficiency matters. But efficiency must not come at the cost of control.

Improve efficiency by:

- automating PO-to-invoice matching where possible;
- using electronic invoicing and payment where supported;
- automating dunning and collection workflows;
- resolving recurring exceptions at their root cause;
- standardizing approval workflows and limits.

Automation that removes the match or the approval gate is not efficiency; it is risk. Automate the mechanics, keep the controls.

### Acknowledge Framework And Professional Limits

P2P and O2C processes implement accounting policy, but revenue recognition, credit loss allowance, accrual, and tax determination must comply with the applicable reporting framework, tax, and regulatory requirements. These areas often involve framework-specific rules, such as the revenue recognition model or the expected credit loss model. Confirm significant process and policy decisions with qualified accounting and tax professionals, and validate that the processes produce framework-compliant results. Do not treat P2P and O2C design as purely operational; they are accounting integrity processes.

## Common Traps

### Process Maps Without Control Points

A flow that moves transactions without identifying controls at each risk does not protect the business.

### Weak Or Overridden Three-Way Match

Paying without matching, or allowing routine overrides, causes overpayment, duplicate payment, and fraud.

### Uncontrolled Vendor Master

Letting anyone create vendors or change bank details without verification invites payment fraud.

### Billing Errors And Late Invoicing

Inaccurate or delayed billing extends collections and increases bad debt.

### Passive Collections

Without active aging management and follow-up, receivables deteriorate and credit losses rise.

### Unapplied Cash And Unresolved Deductions

Items left on account distort the receivable and obscure collection performance.

### Unreconciled Subledgers

AP and AR subledgers that do not tie to the GL indicate process or configuration errors.

### Efficiency Without Control

Automating away the match or approval removes the controls that make the ledger reliable.

### Process Design Without Framework Review

Revenue, credit loss, accrual, and tax treatments must be confirmed against the applicable framework by qualified professionals.

## Self-Check

- Are P2P and O2C mapped end to end with control points at requisition, PO, receipt, match, payment, credit, billing, collection, and application?
- Is the three-way match enforced in P2P, with exceptions routed and payment blocked on mismatch?
- Is vendor master data controlled through verified onboarding, segregation, approval, and independent bank detail verification?
- Are customer credit approval, billing accuracy, revenue recognition, and timely invoicing managed in O2C?
- Are collections actively driven by aging, and is cash accurately applied with unapplied items resolved?
- Do AP and AR subledgers reconcile to the GL every period, with breaks investigated?
- Is efficiency improved through automation and root-cause resolution without removing match, approval, or reconciliation controls?
- Do the processes reflect framework-compliant revenue, credit loss, accrual, and tax policy confirmed with qualified professionals?
