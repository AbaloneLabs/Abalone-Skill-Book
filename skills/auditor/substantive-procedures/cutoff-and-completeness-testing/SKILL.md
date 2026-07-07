---
name: cutoff_and_completeness_testing.md
description: Use when the agent is testing cutoff and completeness assertions for revenue expenses assets and liabilities, designing procedures to verify transactions are recorded in the correct accounting period, testing for unrecorded liabilities and completeness of revenue, performing roll-forward and search-for-unrecorded-items procedures, or determining whether period-end transactions are recorded in the proper period and the financial statements capture all items that should be included.
---

# Cutoff And Completeness Testing

Cutoff and completeness are assertions that determine whether the financial statements capture the right transactions in the right period. Cutoff ensures that transactions are recorded in the correct accounting period, neither pulled forward nor pushed back to manipulate results. Completeness ensures that all transactions and balances that should be recorded have been recorded, with nothing omitted. These assertions are particularly vulnerable at period-end, where the pressure to meet targets creates incentive to manipulate timing, and where the boundary between periods is most easily blurred. The recurring failure is treating cutoff as a mechanical matching of invoices to dates, without understanding the shipping and revenue recognition patterns, and treating completeness as a search for a known list of items rather than a search for what might be missing. Effective cutoff and completeness testing requires understanding the transaction cycles, identifying where unrecorded items would arise, and designing procedures that search for the unknown, not merely confirm the recorded.

Use this skill when testing cutoff and completeness for revenue, expenses, assets, and liabilities, and when designing procedures to verify that transactions are recorded in the correct period and that all items that should be recorded have been. The goal is testing that addresses the timing and completeness of recording, not merely the accuracy of recorded amounts.

## Core Rules

### Understand The Distinct Objectives Of Cutoff And Completeness

Cutoff and completeness are related but distinct assertions. Cutoff addresses whether transactions are recorded in the correct accounting period. Completeness addresses whether all transactions and balances that should be recorded have been recorded. Confusing the two leads to procedures that test one but not the other.

Understand that:

- cutoff focuses on the period in which a transaction is recorded, ensuring it matches the period in which the underlying economic event occurred;
- completeness focuses on whether items that should be recorded are recorded, ensuring nothing is omitted;
- the two assertions are tested with different procedures: cutoff by examining transactions near the period boundary, completeness by searching for unrecorded items;
- both assertions are particularly vulnerable at period-end and in high-volume, manual, or complex transaction cycles.

Understanding the distinction ensures both assertions are addressed.

### Map The Transaction Cycles And Recording Points

Effective cutoff and completeness testing requires understanding the transaction cycles and the points at which transactions are recorded. For revenue, this means understanding when goods or services are transferred and when revenue is recognized. For expenses, it means understanding when goods or services are received and when liabilities are recorded.

Map by:

- walking through the revenue, expense, and acquisition cycles to identify recording points;
- identifying the documents that trigger recording, such as shipping documents, receiving reports, and supplier invoices;
- understanding the lag, if any, between the economic event and the recording;
- identifying where manual entries or period-end accruals affect recording.

Mapping the cycles focuses testing on the points where cutoff and completeness risk is highest.

### Design Cutoff Procedures For Revenue

Revenue cutoff is a significant risk area, given the presumption that revenue recognition is susceptible to fraud. The procedures must verify that revenue is recorded in the period in which the underlying transfer of goods or services occurs.

Design by:

- selecting a sample of revenue transactions recorded in the period and tracing to shipping documents, delivery confirmation, or service performance evidence;
- selecting transactions recorded shortly after period-end and tracing back to determine whether they should have been recorded in the period;
- for services, testing the pattern of performance over time and the cutoff of revenue recognition;
- examining returns and credit memos after period-end for indications of revenue recorded prematurely;
- comparing revenue patterns near period-end to expectations and prior periods.

Revenue cutoff procedures must address both the recorded transactions and the transactions shortly after period-end.

### Design Cutoff Procedures For Expenses And Inventory

Expense and inventory cutoff ensure that expenses are recorded in the period in which the related goods or services are consumed, and that inventory and the corresponding liability are recorded in the same period.

Design by:

- selecting a sample of expense and inventory transactions recorded in the period and tracing to receiving reports or service evidence;
- selecting receiving reports and supplier invoices shortly before and after period-end and tracing to the recording;
- testing for unrecorded liabilities by examining cash disbursements after period-end for items relating to the period;
- examining accruals and estimates for proper period cutoff;
- ensuring inventory counts and adjustments are recorded in the correct period.

Expense and inventory cutoff procedures must trace in both directions, from records to documents and from documents to records.

### Design Completeness Procedures By Searching For Unrecorded Items

Completeness testing cannot rely on the recorded population, because the missing items are, by definition, not in the records. The auditor must design procedures that search for items that should have been recorded, using sources independent of the entity's records.

Search by:

- for liabilities, examining cash disbursements after period-end and tracing to the period, identifying items not recorded;
- for revenue, comparing shipping or delivery records to recorded revenue, identifying shipments not invoiced;
- for expenses, reviewing subsequent payments, vendor statements, and unmatched receiving reports;
- using analytical procedures to identify unexpected patterns that may indicate omitted items;
- inquiring of management and personnel about known or suspected unrecorded items.

Searching for unrecorded items requires independent sources and analytical procedures, not merely confirming the recorded population.

### Use Roll-Forward And Reconciliation Procedures

Roll-forward and reconciliation procedures support both cutoff and completeness by tying balances across periods and reconciling subsidiary records to the general ledger.

Use by:

- rolling forward beginning balances, additions, deductions, and ending balances for asset, liability, and equity accounts;
- reconciling subsidiary ledgers, such as accounts receivable and payable, to the general ledger;
- investigating reconciling items, particularly old or unusual items, for proper recording;
- using the roll-forward to identify gaps or inconsistencies that may indicate cutoff or completeness issues.

Roll-forward and reconciliation provide a structural check on cutoff and completeness across periods.

### Address Period-End And Post-Closing Risk

Period-end and post-closing are the highest-risk periods for cutoff manipulation. Entries made after the nominal close, particularly by senior personnel, may be used to adjust results. The auditor must give specific attention to these entries.

Address by:

- examining entries made in the final days of the period and in the post-closing window;
- identifying entries made by senior personnel or outside normal workflows;
- testing the support and business rationale for period-end and post-closing entries;
- comparing the pattern of period-end entries to prior periods and expectations.

Period-end and post-closing entries are where cutoff manipulation most commonly occurs.

## Common Traps

### Confusing Cutoff With Completeness

The two assertions are distinct and require different procedures. Confusing them leads to testing one but not the other.

### Treating Cutoff As Mechanical Date Matching

Cutoff requires understanding the economic event and the recording pattern, not merely matching invoice dates.

### Relying On The Recorded Population For Completeness

Completeness testing must search for unrecorded items using independent sources. Relying on recorded items tests nothing about completeness.

### Tracing In Only One Direction

Cutoff and completeness require tracing in both directions, from records to documents and from documents to records.

### Ignoring Period-End And Post-Closing Entries

Period-end and post-closing entries are the highest-risk for manipulation. Ignoring them leaves cutoff risk unaddressed.

### Missing Unrecorded Liabilities

Unrecorded liabilities are a common completeness failure. Failing to search subsequent disbursements and unmatched documents leaves them undetected.

### Overlooking Returns And Credit Memos

Returns and credit memos after period-end may indicate prematurely recorded revenue. Overlooking them leaves revenue cutoff risk unaddressed.

## Self-Check

- Are cutoff and completeness understood as distinct assertions, each requiring different procedures?
- Have the transaction cycles and recording points been mapped, with testing focused on high-risk points?
- For revenue cutoff, are procedures tracing recorded transactions to shipping or performance evidence, and post-period-end transactions back to the period?
- For expense and inventory cutoff, are procedures tracing in both directions, from records to documents and documents to records?
- For completeness, are procedures searching for unrecorded items using independent sources, including subsequent disbursements, shipping records, and analytical procedures?
- Are roll-forward and reconciliation procedures used to tie balances across periods and identify gaps?
- Are period-end and post-closing entries, particularly those by senior personnel, examined for support and business rationale?
- Could an independent reviewer confirm that cutoff and completeness are tested with appropriate procedures in both directions?
- Is the testing free of treating cutoff as mechanical date matching or relying on the recorded population for completeness?
- Are returns, credit memos, and unmatched documents examined for indications of cutoff or completeness failure?
