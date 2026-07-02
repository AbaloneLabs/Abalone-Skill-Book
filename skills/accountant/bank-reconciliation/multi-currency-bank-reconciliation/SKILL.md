---
name: multi_currency_bank_reconciliation.md
description: Use when the agent is reconciling bank accounts held in foreign currencies, handling exchange rate gains and losses, choosing between transaction-date and period-end rates, reconciling revaluation balances, matching settlements across currencies, or assessing whether multi-currency differences are real losses or rate and timing artifacts.
---

# Multi-Currency Bank Reconciliation

Reconciling a bank account in a foreign currency adds a second dimension of error on top of the usual matching problem. In a single-currency reconciliation, a difference means an amount is missing, duplicated, or wrong. In a multi-currency reconciliation, a difference can also mean the wrong exchange rate was applied, the rate date does not match the transaction date, a revaluation was missed or doubled, or a settlement cleared in a different currency than expected. These rate and timing artifacts produce differences that look like errors but are not, and they also mask real errors that look like rate artifacts but are. The agent's central challenge is to separate genuine cash movement from the noise created by currency conversion, and to do so without losing track of the realized and unrealized gains and losses that the conversion itself generates. A multi-currency reconciliation that forces the difference to zero by plugging an exchange adjustment is not reconciled. It is silenced.

Use this skill before reconciling a foreign-currency bank account, recording exchange gains and losses, choosing translation rates, reconciling revaluation, or matching cross-currency settlements. The goal is to prevent the agent from treating exchange differences as balancing plugs and from missing the real transaction errors that currency noise can hide.

Foreign currency accounting standards, revaluation requirements, exchange rate sources, tax treatment of foreign exchange gains and losses, and functional-currency rules vary by jurisdiction, reporting framework, and entity structure. Do not state a single universal rule. Functional currency determination, hedge accounting, and tax treatment of exchange differences should be confirmed with a qualified accountant. This skill provides operational multi-currency reconciliation judgment, not a final determination in any specific case.

## Core Rules

### Reconcile In The Foreign Currency First, Then Translate

The foundation of a clean multi-currency reconciliation is to match the bank statement to the books in the foreign currency itself, where the amounts are directly comparable, before introducing any translation. Translating first and reconciling second mixes transaction errors with rate errors and makes both harder to find.

Follow the sequence:

- reconcile the foreign-currency bank statement to the foreign-currency book balance first;
- resolve all transaction-level differences, missing items, duplicates, and timing items in the foreign currency;
- confirm the foreign-currency reconciliation is clean before translating;
- then translate the foreign-currency balance to the functional currency using the correct rate;
- investigate any remaining functional-currency difference as a rate or revaluation matter.

If the foreign-currency reconciliation does not balance, the functional-currency reconciliation cannot be trusted. Never use translation to paper over an unresolved foreign-currency difference.

### Apply The Correct Rate To The Correct Event

Exchange rate selection is where most multi-currency errors originate. The rate must match the event it translates, and using the wrong date's rate creates a difference that has nothing to do with the transaction itself.

Match rates to events:

- transactions are generally recorded at the rate on the transaction or settlement date, depending on the framework;
- period-end balances are translated at the closing rate at the balance sheet date;
- subsequent settlement of an earlier transaction can create a realized gain or loss;
- revaluation uses the period-end rate for open balances, not the original transaction rate.

Confirm which rate convention the applicable accounting framework requires before applying rates. Mixing transaction-date and period-end rates inconsistently produces differences that look like errors but are rate mismatches.

### Separate Realized From Unrealized Gains And Losses

Currency conversion generates two kinds of results. A realized gain or loss occurs when a foreign-currency transaction is settled, converted into the functional currency or another currency. An unrealized gain or loss arises from revaluing an open foreign-currency balance at the period-end rate. Confusing the two distorts income and misstates the nature of the result.

Distinguish:

- realized gains and losses from actual settlement or conversion events;
- unrealized gains and losses from period-end revaluation of open balances;
- the reversal of prior unrealized amounts when a balance settles in a later period;
- the cumulative running effect on each foreign-currency account.

Recording every exchange difference as a single plug hides whether the entity actually lost money on a conversion or merely revalued an open balance. The distinction matters for reporting, tax, and understanding real performance.

### Reconcile The Revaluation, Not Just The Transactions

Period-end revaluation adjusts foreign-currency balances to the closing rate and creates an unrealized gain or loss. That revaluation must itself be reconciled, because errors in the rate, the balance revalued, or the reversal of prior revaluation all produce lingering differences.

Verify the revaluation by:

- confirming the period-end rate used is the correct closing rate from a defensible source;
- ensuring every open foreign-currency balance was included in the revaluation;
- checking that prior-period unrealized amounts were reversed correctly before the new revaluation;
- reconciling the revaluation gain or loss to the exchange account in the general ledger;
- confirming no balance was revalued twice or missed entirely.

A revaluation that is not reconciled turns the exchange account into an unexplained dumping ground, which is exactly where multi-currency errors hide.

### Match Cross-Currency Settlements Carefully

When a payment is made in one currency from an account denominated in another, or when a settlement converts through an intermediate currency, the matching becomes more complex. The amount that clears the bank is not the same as the amount recorded on the invoice, and the difference is a realized exchange result that must be captured.

For cross-currency settlements:

- identify the invoice currency, the payment currency, and the account currency;
- record the settlement at the rate that actually applied to the conversion;
- capture the realized gain or loss as the difference between the recorded and settled amounts;
- reconcile the bank credit in the account currency to the converted amount;
- do not net the exchange difference into the original payable or receivable.

Cross-currency settlements are where realized exchange gains and losses are born. If they are not isolated, they either vanish or inflate the original transaction amount incorrectly.

### Use A Consistent And Defensible Rate Source

Exchange rates vary by source, and using different sources for recording, settlement, and revaluation creates artificial differences. The rate source should be consistent and defensible.

Establish:

- a primary rate source for transaction recording, such as a central bank or a stated commercial source;
- the closing-rate source for period-end revaluation;
- the treatment of weekends, holidays, and non-trading days when selecting a rate;
- documentation of the rate applied to each material transaction.

Inconsistent rate sourcing manufactures differences that have no economic cause. Consistency makes the reconciliation explainable.

### Investigate Differences As Rate, Timing, Or Transaction Errors

A multi-currency difference can have three distinct causes, and the investigation must consider all three rather than assuming the most convenient one. Jumping to an exchange plug without classifying the difference embeds real errors.

Classify each difference as:

- a rate difference, caused by using the wrong rate or rate date;
- a timing difference, caused by a transaction clearing in a different period;
- a transaction error, a genuine missing, duplicate, or misstated item.

Only after ruling out rate and timing causes should a difference be treated as a transaction error or a legitimate exchange result. Treating every difference as an exchange adjustment is how real errors disappear into the exchange account.

## Common Traps

### Plugging The Difference As An Exchange Adjustment

The most common multi-currency failure is forcing the reconciliation to balance by posting the residual difference to the exchange gain or loss account. This silences the reconciliation, hides real transaction errors, and turns the exchange account into an unexplained sink.

### Reconciling In The Functional Currency Before The Foreign Currency

Translating first and matching second mixes rate errors with transaction errors. The foreign-currency reconciliation must be clean before translation introduces its own differences.

### Mixing Transaction-Date And Period-End Rates

Applying the period-end rate to transactions, or the transaction rate to the period-end balance, creates differences that have no economic basis. Rates must match the event they translate.

### Failing To Reverse Prior-Period Revaluation

If the prior period's unrealized revaluation is not reversed before the new revaluation, the exchange account accumulates duplicate or stale amounts that never reconcile. Reversal is a required step, not optional.

### Confusing Realized And Unrealized Results

Recording all exchange differences as one amount hides whether the entity actually converted and lost money or merely revalued an open balance. The distinction affects income, tax, and performance interpretation.

### Using Inconsistent Rate Sources

Different rates from different sources for recording, settlement, and revaluation create artificial differences. Without a consistent, documented rate source, the reconciliation cannot be explained.

### Letting The Exchange Account Become An Unexplained Sink

When every unexplained difference is posted to exchange gains or losses, the account grows into an unauditable balance. The exchange account must be reconciled and explained just like any other.

### Overlooking Cross-Currency Settlement Gains And Losses

When a settlement converts through an intermediate currency, the realized exchange result must be captured separately. Netting it into the original invoice amount misstates the transaction and hides the currency result.

## Self-Check

- Is the bank account reconciled in the foreign currency first, with all transaction differences resolved, before any translation to the functional currency?
- Is each transaction translated at the rate matching its event, with period-end balances at the closing rate, consistent with the applicable accounting framework?
- Are realized gains and losses from settlement separated from unrealized gains and losses from period-end revaluation?
- Is the period-end revaluation reconciled, including correct closing rate, inclusion of all open balances, and proper reversal of prior-period unrealized amounts?
- Are cross-currency settlements reconciled with the realized exchange result captured separately rather than netted into the original payable or receivable?
- Is a consistent, documented rate source used for recording, settlement, and revaluation, with treatment of non-trading days defined?
- Is each difference classified as rate, timing, or transaction error before any exchange adjustment is posted?
- Is the exchange gain and loss account itself reconciled and explained rather than used as a plug for unexplained residuals?
- Has the functional currency determination, revaluation policy, and tax treatment of exchange results been confirmed with a qualified accountant under the applicable framework?
