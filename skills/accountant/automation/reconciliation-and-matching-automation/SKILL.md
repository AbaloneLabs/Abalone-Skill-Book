---
name: reconciliation_and_matching_automation.md
description: Use when the agent is automating account reconciliations, building bank or credit card matching rules, configuring transaction matching between subledgers and the general ledger, or evaluating controls and exception handling for automated reconciliation.
---

# Reconciliation And Matching Automation

Reconciliation is the process of confirming that two independent records of the same activity agree. It is foundational to financial integrity, but it is also repetitive, high-volume, and tedious, which makes it a prime candidate for automation. Automated matching can clear the vast majority of routine transactions instantly, leaving only genuine exceptions for human review. Done well, it accelerates the close, improves accuracy, and frees accountants for investigation. Done poorly, it produces false matches that hide real discrepancies, clears items that should be investigated, and creates a false sense of assurance that the accounts are correct when they are not. Automation does not make a reconciliation correct; it makes a correct reconciliation faster and a flawed one faster too.

Use this skill before automating account reconciliations, building matching rules, configuring subledger-to-GL matching, or evaluating controls over automated reconciliation. The goal is to prevent the agent from building matching logic that produces false confidence, clears exceptions prematurely, or removes the independent verification that gives reconciliation its value.

## Core Rules

### Distinguish Clearing From Reconciling

Clearing a matched item and reconciling an account are not the same. Clearing is a mechanical act of marking two records as corresponding. Reconciling is the accounting act of confirming the account balance is correct and complete. Automation accelerates clearing; it does not replace reconciliation.

Ensure that:

- automated matching clears corresponding items but does not by itself certify the account;
- a human reviews the reconciliation as a whole, including uncleared items, after matching;
- the reconciliation confirms not only that records agree but that the balance is supportable;
- aging and composition of uncleared items are reviewed, not just the matched total.

A reconciliation that only runs matching and never reviews the residual is not a reconciliation; it is a clearing exercise dressed up as one.

### Define Matching Logic With Explicit Tolerances And Keys

Matching rules must be explicit about how two records correspond. Implicit or fuzzy matching without defined keys produces false matches.

For each matching scenario define:

- the matching keys, such as amount, date, reference, invoice number, or a combination;
- the tolerance for amount and date differences, and the reason each tolerance is acceptable;
- the priority order when multiple rules could match the same item;
- whether partial matches are allowed and how they are split;
- the treatment of one-to-many and many-to-many matches.

Document the logic so an auditor can understand why items cleared. A rule that matches on amount alone, with no reference or date check, will clear unrelated transactions that happen to share an amount and hide real discrepancies.

### Prevent False Matches That Hide Discrepancies

The greatest risk in automated matching is the false positive, a match that clears two unrelated items. False matches hide the very discrepancies reconciliation exists to find.

Prevent false matches by:

- requiring multiple matching keys, not amount alone, unless the population is tightly controlled;
- flagging matches that rely solely on tolerance for human review;
- avoiding overly broad fuzzy matching that clears items based on similarity rather than identity;
- reviewing the match rate and the cleared-item population for plausibility each period;
- investigating sudden changes in the auto-match percentage, which can signal a rule change or a data feed problem.

A high auto-match rate is not automatically good. It can indicate that the rules are too permissive and are clearing items that should be exceptions.

### Route Exceptions To Defined Owners With Aging Tracking

The value of automation is that it surfaces exceptions quickly. That value is lost if exceptions sit unassigned.

For exceptions ensure that:

- each exception type has a defined owner or queue;
- exceptions are aged and tracked, with escalation for old items;
- the reason an item did not auto-match is captured to guide investigation;
- write-off or clearing of an exception requires approval and documentation;
- recurring exception types are analyzed for root cause, such as a data quality problem or a missing rule.

Exceptions that are cleared without investigation, or that age indefinitely, indicate that the matching process is not actually controlling the account.

### Reconcile Between Independent Systems, Not Copies Of The Same Data

Reconciliation has value only when the two records are independently generated. Matching a subledger to a GL that is fed by that same subledger is not an independent reconciliation; it is a self-match.

Ensure that:

- bank reconciliations match the internal cash record to the bank statement, an independent external record;
- subledger-to-GL reconciliations confirm the posting mechanism, but the more valuable reconciliation is often subledger to an external or operational source;
- intercompany reconciliations match records held by two different entities, not two views of one entity's books;
- inventory reconciliations match the perpetual record to a physical count or an independent system.

A reconciliation between two records that derive from the same source confirms only that the feed worked, not that the underlying activity is correct.

### Preserve The Audit Trail Of Every Match And Clearance

Every cleared item must be traceable to the rule that cleared it and the records that were matched. This is essential for audit.

Preserve:

- the matching rule and version that cleared each item;
- the matched pair of records, including amounts, dates, and references;
- the timestamp and user or system that performed the match;
- any manual override or forced clearance, with the reason and approver;
- the reconciliation certification, including the reviewer and date.

An auditor must be able to take any cleared item and reconstruct why it cleared. Matching with no preserved trail cannot satisfy audit evidence requirements.

### Monitor Match Rates, Exception Volumes, And Rule Effectiveness

Automation performance must be monitored, not assumed. Drift in match rates signals problems.

Monitor:

- the auto-match percentage by account and rule;
- the exception volume and aging trend;
- the false-match rate discovered during review;
- the manual override frequency, which can indicate rules are too strict or too loose;
- the time from period end to reconciliation completion.

A sudden drop in match rate may indicate a data feed change, a new transaction type, or a rule that no longer fits the business. Investigate changes rather than accepting them.

### Acknowledge Framework And Professional Limits

Reconciliation and matching automation supports the integrity of balances reported under the applicable framework, but it does not by itself establish correctness. Reconciliations involving revenue, cash, intercompany, inventory, and estimates often involve framework-specific recognition, cutoff, and valuation questions that matching alone cannot resolve. Confirm significant reconciliation conclusions with qualified accounting professionals, and validate that automated matching supports, rather than substitutes for, framework-compliant balance substantiation. Do not treat a high match rate as proof that a balance is correct.

## Common Traps

### Confusing Clearing With Reconciling

Clearing matched items without reviewing the residual balance is not a reconciliation.

### Matching On Amount Alone

Amount-only matching clears unrelated transactions that share an amount and hides real discrepancies.

### Overly Permissive Fuzzy Matching

Broad fuzzy matching inflates the match rate by clearing items based on similarity rather than identity.

### High Match Rate Treated As Success

A very high auto-match rate can indicate rules are too permissive, not that the account is correct.

### Exceptions Cleared Without Investigation

Clearing exceptions to reduce the queue hides the discrepancies the reconciliation exists to find.

### Self-Matching Presented As Reconciliation

Matching two records derived from the same source confirms the feed, not the correctness of the activity.

### No Preserved Audit Trail

Matching with no record of which rule cleared which item cannot satisfy audit evidence requirements.

### No Monitoring Of Match Rate Drift and matching Treated As Balance Substantiation

A changing match rate can signal a data feed problem or a rule that no longer fits the business.

A high match rate does not establish framework-compliant correctness of a balance; confirm with qualified professionals.

## Self-Check

- Does the process distinguish clearing matched items from certifying the account balance as correct and complete?
- Is matching logic explicit about keys, tolerances, priority, partial matches, and many-to-many cases?
- Are false matches prevented through multi-key matching, tolerance review, and plausibility checks on the match rate?
- Are exceptions routed to defined owners, aged, escalated, investigated, and analyzed for root cause?
- Do reconciliations compare independent records rather than two views of the same source data?
- Is the audit trail preserved for every match, including rule version, matched records, overrides, and certification?
- Are match rates, exception volumes, false-match rates, and override frequency monitored for drift?
- Does automated matching support, rather than substitute for, framework-compliant balance substantiation confirmed with qualified professionals?
