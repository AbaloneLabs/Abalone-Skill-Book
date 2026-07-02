---
name: inter_account_and_clearing_account_reconciliation.md
description: Use when the agent is reconciling clearing accounts, suspense accounts, or intercompany accounts, investigating residual balances in transit accounts, cleaning up zero-balance or clearing accounts, or reviewing transit and suspense accounts for control quality.
---

# Inter-Account And Clearing Account Reconciliation

Clearing accounts, suspense accounts, and intercompany accounts are the transit lounges of the general ledger. They hold transactions temporarily while they wait to be matched, allocated, or settled to their final destination. A payroll clearing account holds net pay between the payroll posting and the bank funding. A suspense account holds an unidentified receipt until it is researched. An intercompany account holds balances between entities until they settle. The design intent is that these accounts return to zero, or to a known in-transit balance, every period. The reality is that they accumulate. Transactions enter and never leave, residuals build, and the account that was meant to be temporary becomes a permanent hiding place for errors, unreconciled items, and sometimes manipulation. Agents often treat these accounts as self-clearing and skip the reconciliation, when they are exactly the accounts that most need it.

Use this skill before reconciling a clearing, suspense, or intercompany account, cleaning up residual transit balances, or reviewing whether transit accounts are under control. The goal is to ensure every transit account clears to its expected balance each period and that residuals are investigated and resolved.

This skill covers transit account reconciliation. Specific clearing account designs, intercompany settlement methods, and zero-balance account configurations vary by system and entity structure. Confirm the setup before relying on these procedures.

## Core Rules

### Define The Expected End State For Each Transit Account

Every clearing, suspense, or intercompany account should have a defined expected balance after the close. Most should be zero. Some, like an intercompany account with normal in-transit timing, may have a known expected balance.

For each transit account, document:

- what the account is for;
- what transactions flow through it;
- what the expected balance is after each close, usually zero;
- how long items should remain before clearing;
- who owns the reconciliation and clearance.

Without a defined end state, a residual balance looks normal when it is actually a problem.

### Reconcile Every Transit Account Each Period

Transit accounts need reconciliation every period, not just the control accounts. Because they are designed to clear, any balance that remains is by definition an item that did not complete its journey.

The reconciliation should:

- list every item remaining in the account;
- explain why each item has not cleared;
- confirm each item is expected to clear next period;
- age the items and escalate old ones.

A transit account that is not reconciled will accumulate indefinitely, and the accumulation hides errors, missing settlements, and sometimes fraud.

### Clear Items Promptly To Their Final Destination

The purpose of a transit account is temporary holding. Items should be moved to their final account, whether an expense, an asset, a liability, or a settlement, as soon as the matching or allocation is complete.

Common clearance patterns:

- payroll clearing funds when the bank payments post;
- suspense receipts when the customer or invoice is identified;
- intercompany charges when the reciprocal entry is matched and settled;
- merchant clearing when the net deposit is reconciled to gross sales and fees.

Delaying clearance leaves balances in limbo and distorts the accounts they should have reached.

### Investigate Every Residual Balance

A residual balance in a transit account after the close is a signal. It means something did not complete. Each residual must be investigated, not carried forward as a permanent reconciling item.

Ask of each residual:

- what transaction is this, and when did it enter the account;
- why has it not cleared;
- is there a missing matching entry, a failed settlement, or an unidentified item;
- is the same residual appearing period after period.

Residuals that repeat are a process failure. The matching, settlement, or identification step is breaking down and must be fixed.

### Match Intercompany Accounts In Both Directions

Intercompany accounts must reconcile between the two entities that share them. Entity A's receivable from Entity B must equal Entity B's payable to Entity A. A difference means one entity recorded a charge that the other did not, or they recorded it in different periods or amounts.

Intercompany reconciliation requires:

- matching each charge to its reciprocal entry;
- investigating unmatched or differing entries;
- settling or eliminating the balances at consolidation;
- confirming the net intercompany position is agreed by both entities.

Unreconciled intercompany accounts corrupt consolidated financial statements and can hide profit shifting or transfer pricing issues.

### Prevent Transit Accounts From Becoming Permanent Parking

Transit accounts are meant to be temporary. When balances sit in them period after period, they become a place to park unresolved items, which hides problems and can mask manipulation.

Control this by:

- aging every item in every transit account;
- escalating items beyond a defined age;
- writing off or resolving genuinely unrecoverable items through proper approval;
- reviewing whether an account that never clears should be restructured.

### Reconcile Zero-Balance Accounts To Their Funding Source

Zero-balance accounts, often used for payroll or disbursements, are funded from a master account and should end each day at zero. The reconciliation confirms that the funding entries match the disbursements and that no balance remains.

A zero-balance account that does not return to zero has a posting or funding error that must be found the same day, not deferred.

### Review And Approve Transit Account Reconciliations

Transit account reconciliations should be reviewed and approved like any other. Because these accounts are high-risk for accumulation and hiding items, the review should specifically confirm that residuals are aged, investigated, and being cleared.

## Common Traps

### Treating Transit Accounts As Self-Clearing

Because clearing and suspense accounts are designed to return to zero, there is an assumption they will. They do not, unless someone reconciles and clears them. Skipping the reconciliation lets residuals build.

### Carrying Residuals Forward Indefinitely

A residual that appears every period without investigation is hiding a failed match, a missing settlement, or an unidentified item. Age and escalate every residual.

### Using Suspense As A Dumping Ground

When a transaction cannot be classified, posting it to suspense and moving on feels efficient. Over time, suspense becomes a graveyard of unresolved items. Every suspense item needs research and clearance.

### Failing To Match Intercompany In Both Directions

If only one entity reconciles its intercompany account, the difference hides on the other entity's books. Both sides must agree, and differences must be investigated.

### Letting Zero-Balance Accounts Keep A Balance

A zero-balance account that ends the day with a balance has a funding or posting error. Deferring the investigation lets the error compound and can mask a cash problem.

### Clearing By Plug Rather Than By Matching

Forcing a transit account to zero with a plug entry hides the unmatched item rather than resolving it. The plug creates a clean balance sheet and a hidden error.

### Ignoring Old Items Because They Are Small

Small residuals are easy to ignore, but a population of small old items can be material in aggregate and can indicate a systematic process failure. Review all items, not just the large ones.

## Self-Check

- Does every clearing, suspense, and intercompany account have a defined expected end-state balance, usually zero, after each close?
- Is every transit account reconciled each period, with each remaining item listed, explained, expected to clear, and aged?
- Are items cleared promptly to their final destination once matching or allocation is complete?
- Is every residual balance investigated for its cause, with repeating residuals escalated as a process failure?
- Do intercompany accounts reconcile in both directions, with unmatched or differing entries investigated and balances settled or eliminated at consolidation?
- Are transit accounts prevented from becoming permanent parking through aging, escalation, and proper write-off of unrecoverable items?
- Do zero-balance accounts reconcile to their funding source daily, with any non-zero balance investigated the same day?
- Is each transit account reconciliation reviewed and approved, with specific attention to aging residuals?
- Have system-specific clearing, settlement, and zero-balance configurations been confirmed before relying on these procedures?
