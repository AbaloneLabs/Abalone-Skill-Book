---
name: ap_ledger_reconciliation.md
description: Use when the agent is reconciling the accounts payable subledger to the general ledger, investigating aged payables, confirming vendor balances, reconciling to AP aging reports, clearing unmatched or suspended invoices, or reviewing whether the payables balance is complete, accurate, and supported at period end.
---

# AP Ledger Reconciliation

The accounts payable balance on the general ledger is a summary. Behind it sits a subledger of individual vendor invoices, credits, payments, and adjustments. When the two disagree, or when the detail contains stale, duplicated, or unsupported items, the summarized liability is not trustworthy. AP reconciliation is the process of proving that the total owed to vendors is real, complete, properly aged, and supported by evidence. Weak reconciliation looks thorough because it produces a reconciling schedule, but it fails when old invoices are parked indefinitely, when credits are applied to the wrong vendors, when payments clear but are never matched, or when the subledger and general ledger drift apart without anyone noticing.

Use this skill before closing the AP ledger at period end, investigating an aging exception, responding to a vendor balance dispute, preparing payables for audit, cleaning up suspended or unmatched invoices, or reviewing whether the AP balance can be relied upon for reporting. The goal is to prevent the agent from certifying a payables balance that hides errors, duplicates, fraud, or unrecorded obligations.

Aging thresholds, statement cadence, and write-off authority differ by entity policy and jurisdiction. Where an item requires write-off, derecognition, or disclosure, confirm the treatment with a qualified accounting professional.

## Core Rules

### Prove The Subledger Ties To The General Ledger

The first test is arithmetic. The sum of open vendor balances in the accounts payable subledger must equal the accounts payable control account in the general ledger, for each entity and each currency.

When they do not tie:

- identify whether the difference is a single entry or an accumulation;
- check for manual journal entries posted directly to the control account instead of through the subledger;
- check for timing differences between subledger posting and general ledger posting;
- check for postings to the wrong entity or company code;
- check for cutoff errors where a payment or invoice posted in one system but not the other.

A control account that receives manual entries is a red flag. The control account should move only through the subledger; any direct adjustment bypasses the vendor detail and breaks the reconciliation.

### Investigate Every Reconciling Item With A Plan

Reconciling items are not normal; they are exceptions that must be explained and resolved. Each item should have a documented cause and an expected clearing action.

Common reconciling items include:

- payments recorded in the ledger but not yet cleared or matched in the subledger;
- invoices entered in the subledger but not yet posted to the ledger;
- manual checks or electronic payments not reflected in one system;
- foreign-currency revaluation differences;
- credit memos entered in one system but not the other;
- entries posted to the wrong period.

Assign each item an owner and a target resolution date. Items that linger across multiple closes indicate a process failure, not a data quirk.

### Manage The Aging, Not Just The Total

A payables balance can tie perfectly to the ledger and still be unhealthy. Aging reveals whether the business is paying on time, whether invoices are stuck in approval, and whether old balances are being ignored.

Review the aging by bucket:

- current: invoices not yet due;
- 0 to 30, 31 to 60, 61 to 90, and over 90 days past due.

For each bucket, ask:

- Are past-due items delayed by a legitimate dispute, missing approval, or cash constraint?
- Are very old items still genuine liabilities, or are they duplicates, errors, or items that should be derecognized?
- Are credits sitting unused against old invoices, inflating the apparent balance?
- Are invoices on hold or in suspense correctly flagged?

Very old payables that no vendor is pursuing may indicate a duplicate, an error, a bartered-away balance, or a liability that should be evaluated for derecognition under the applicable framework. Do not silently leave them; investigate and document.

### Reconcile To Vendor Statements Independently

Internal records are not the only source of truth. Vendor statements provide an independent view of what each supplier believes is owed. Reconciling to statements catches invoices never recorded, payments the vendor has not applied, and credits the vendor has not reflected.

For material vendors, reconcile the internal open balance to the vendor statement and investigate:

- items on the vendor statement but not in the ledger (possible unrecorded liability);
- items in the ledger but not on the vendor statement (possible payment not applied by vendor, or duplicate);
- credits the vendor has not yet issued;
- disputed balances with no documented resolution.

Document the reconciliation and retain it. A vendor statement reconciliation done only verbally leaves no evidence for audit or dispute resolution.

### Clear Unmatched And Suspended Items Actively

Many AP systems accumulate suspended, on-hold, or unmatched invoices that never reach the ledger. These represent either genuine obligations that are stuck, or errors that should be removed. Either way, leaving them unmanaged hides risk.

Establish a routine to review:

- invoices waiting for a purchase order or receipt match;
- invoices waiting for approval past a threshold number of days;
- credits awaiting application;
- payments in transit or unmatched to invoices;
- duplicate detections awaiting disposition.

Each aging item in suspense should have a reason and an owner. Suspense is a holding area, not a permanent home.

### Handle Foreign Currency And Multi-Entity Payables Carefully

When vendors are paid in a foreign currency or when payables span multiple entities, reconciliation becomes more complex. Exchange-rate movements create revaluation gains and losses, and intercompany payable balances must eliminate on consolidation.

Check:

- whether the invoice, the ledger, and the bank use consistent exchange rates and dates;
- whether revaluation entries at period end are reversed or carried correctly;
- whether intercompany payables and receivables between related entities match and eliminate;
- whether gains and losses on settlement are recorded to the correct account.

Currency and intercompany mismatches are a frequent source of persistent, hard-to-explain reconciling items.

### Retain Evidence Of The Reconciliation

A reconciliation that is not documented did not happen, from an audit and control perspective. Retain the reconciliation schedule, the supporting vendor statements, the explanation of each reconciling item, and evidence of review.

The retained package should let a reviewer reconstruct, months later, why the balance was certified as correct.

## Common Traps

### Accepting A Tying Balance Without Reviewing The Detail

When the subledger ties to the ledger, it is tempting to sign off. But a tying total can hide offsetting errors: a duplicate invoice offset by an unapplied credit, or an old balance that should have been written off. Review the composition, not just the total.

### Parking Old Invoices Indefinitely

Invoices that sit in the aging for many months often represent errors, duplicates, or items the vendor has effectively forgiven. Leaving them inflates the liability and distorts ratios. Investigate and resolve or derecognize with documentation.

### Applying Credits Across Vendors To Force A Tie

When a reconciliation does not balance, it is tempting to apply a credit to whichever vendor makes the numbers work. Credits belong to specific vendors and specific invoices. Cross-application hides the real problem and corrupts the detail.

### Letting Manual Journal Entries Hit The Control Account

Direct journal entries to the AP control account bypass the subledger and guarantee a reconciliation break. The control account should move only through validated subledger transactions. Investigate any manual entry immediately.

### Ignoring Credits And Unapplied Payments

Unapplied payments and open credits are easy to overlook because they reduce what is owed. But they represent cash already out or obligations already reduced, and if not tracked they lead to duplicate payments or overstated balances. Reconcile them as actively as invoices.

### Treating Vendor Non-Response As Agreement

If a vendor does not respond to a balance confirmation, that is not the same as agreement. A missing response may mean the vendor's records are incomplete, the contact is wrong, or the vendor is disputing silently. Follow up rather than assuming silence confirms the balance.

## Self-Check

- Does the AP subledger tie to the general ledger control account for every entity and currency, with no manual entries bypassing the subledger?
- Does every reconciling item have a documented cause, an owner, and a target resolution date?
- Is the aging reviewed by bucket, with past-due, very old, and on-hold items investigated rather than carried forward silently?
- Are material vendor balances reconciled to independent vendor statements, with differences documented and investigated?
- Are suspended, unmatched, and on-hold invoices reviewed actively rather than accumulating indefinitely?
- Are foreign-currency revaluations and intercompany payables handled consistently and eliminated correctly?
- Are open credits and unapplied payments tracked and applied to the correct vendors and invoices?
- Is the reconciliation package retained with enough detail to reconstruct the certification months later?
- Are items requiring derecognition or write-off routed to a qualified professional with documented authority?
- Are persistent reconciling items across multiple closes treated as process failures requiring root-cause fixes?
