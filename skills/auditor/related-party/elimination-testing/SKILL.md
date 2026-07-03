---
name: elimination_testing.md
description: Use when the agent is testing consolidation eliminations, verifying intercompany transaction and balance elimination, testing unrealized intercompany profit in inventory, reconciling intercompany loans and interest, evaluating elimination completeness, or investigating unreconciled intercompany differences in group accounts.
---

# Elimination Testing

Consolidation eliminations are the mechanism by which a group presents itself as a single economic entity, and they are also where group accounts most often go wrong. The arithmetic of consolidation looks mechanical, which invites auditors to tick the elimination schedules without asking whether every intercompany transaction and balance has been captured, whether unrealized profits have been correctly removed, and whether the eliminations actually reconcile across the group. An elimination that is incomplete, or that nets against unreconciled differences, leaves revenue, cost of sales, assets, and liabilities overstated or distorted in ways that are hard to detect later. The harm is that users read the group as if it were a clean single entity when in fact the perimeter is leaking.

Use this skill when testing consolidation eliminations, intercompany transaction and balance elimination, unrealized intercompany profit, intercompany loans and interest, or reconciling intercompany differences. The goal is to confirm that the consolidated accounts genuinely present the group as one entity, with no residual intercompany items left in the reported figures.

## Core Rules

### Understand The Consolidation Structure And Perimeter First

Elimination testing is meaningless without a clear picture of which entities are consolidated, by what method, and from what date. The perimeter drives what must be eliminated.

Establish:

- the list of consolidated entities and the consolidation method for each, full, proportionate, or equity;
- acquisition and disposal dates and the related partial-period consolidation;
- entities held for sale or excluded from consolidation and why;
- the reporting currency and each entity's functional currency;
- the group's accounting policies for intercompany profit and foreign currency;
- any changes in the group structure during the period.

A perimeter that is unclear or shifting makes elimination completeness untestable. Resolve the structure before testing any elimination.

### Build The Intercompany Population From Both Ends

Intercompany transactions and balances must be identified from each entity's books, not only from a central schedule. Differences between the two sides of a transaction are the most common source of error.

Build the population by:

- extracting intercompany receivables and payables from each entity's ledger;
- matching intercompany sales and purchases by counterparty;
- identifying intercompany loans, interest, fees, and management charges;
- capturing intercompany transfers of assets, including fixed assets and inventory;
- recording intercompany dividends and capital transactions;
- reconciling each pair of counterparties to confirm both sides are recorded.

If the two sides of a relationship do not match, the difference must be explained before elimination, because eliminating unmatched figures embeds the error in the consolidated balance.

### Test The Completeness Of Eliminations

Completeness is the central assertion for eliminations. A single omitted intercompany relationship leaves consolidated revenue and payables overstated. Test completeness structurally, not by sampling a few lines.

Test completeness by:

- reconciling total consolidated revenue to the sum of entity revenues less intercompany sales;
- reconciling consolidated payables and receivables to the net of entity balances after elimination;
- confirming that every intercompany relationship identified appears in the elimination schedule;
- checking that eliminations cover sales, purchases, loans, interest, fees, and dividends;
- verifying that intra-group inventory profits are eliminated in full;
- confirming that unrealized profits on intercompany asset transfers are removed;
- testing that eliminations are made in the correct period, especially around year-end cutoff.

Any intercompany item present in the entity ledgers but absent from the eliminations is a completeness failure.

### Eliminate Unrealized Intercompany Profit In Inventory

When one group entity sells inventory to another, the group must remove the profit embedded in inventory still on hand at period end. This is a frequent source of misstatement because it requires matching intercompany sales to closing inventory.

Test by:

- obtaining the intercompany markup or margin on inventory transfers;
- identifying intercompany inventory still on hand at the acquiring entity at period end;
- computing the unrealized profit as the markup on the on-hand quantity;
- confirming the elimination removes the profit from both inventory and cost of sales;
- checking the treatment of any inventory write-down that may have absorbed the profit;
- verifying that prior-year unrealized profit eliminated is released in the current year as inventory is sold externally;
- assessing whether the markup is consistent and arm's-length, since distorted transfer prices distort the elimination.

An elimination that removes intercompany sales but not the embedded inventory profit leaves consolidated profit overstated.

### Eliminate Intercompany Loans, Interest, And Fees

Intercompany financing must be eliminated in full, including the interest and fees that flow between group entities. Partial elimination of the principal but not the income is a common error.

Verify:

- intercompany loan principals are eliminated against each other;
- accrued interest receivable and payable are eliminated;
- interest income and expense are eliminated for the period;
- management charges, service fees, and royalties between group entities are removed;
- any exchange differences on intercompany loans of a monetary nature are treated per the framework;
- intercompany guarantees and commitments are disclosed rather than eliminated from disclosure.

Confirm that financing eliminated on the balance sheet is also eliminated in the income statement and cash flow statement.

### Reconcile Unreconciled Intercompany Differences

Intercompany balances that do not match between counterparties are a diagnostic, not a nuisance. Unreconciled differences can represent timing items, errors, unrecorded transactions, or fraud.

Investigate differences by:

- identifying the cause of each material unmatched balance;
- determining whether the difference is timing, such as goods in transit or charges not yet booked;
- checking for one-sided bookings where one entity recorded a transaction the other did not;
- testing for unrecorded intercompany profit or loss;
- assessing whether differences are concentrated with a particular counterparty or period;
- evaluating whether persistent differences indicate weak intercompany controls or intentional manipulation.

Material unreconciled differences should not be eliminated by force; they should be corrected or, if unresolved, treated as a potential misstatement.

### Confirm Eliminations Are Symmetric And Consistent

A valid elimination removes the same amount from both sides of the consolidated accounts. Asymmetric eliminations distort net assets and equity.

Check that:

- the debit and credit of each elimination are equal and correctly classified;
- eliminations of revenue are matched by eliminations of cost of sales or payables;
- eliminations of receivables are matched by eliminations of payables;
- eliminations of investment in subsidiaries are matched by eliminations of subsidiary equity;
- non-controlling interest is adjusted correctly for any unrealized profit attributable to it;
- the sum of all eliminations nets to zero across the consolidation worksheet.

A worksheet that does not balance signals an error in classification, completeness, or the non-controlling interest calculation.

### Reconcile Eliminations To The Cash Flow Statement And Notes

Eliminations affect not only the primary statements but also the cash flow statement and the notes. Inconsistency between the elimination worksheet and these disclosures indicates incomplete consolidation.

Reconcile:

- intercompany cash flows eliminated from the consolidated cash flow statement;
- intercompany balances disclosed, if any, against the elimination residual;
- segment reporting to ensure intercompany revenue is not presented as external;
- commitments and contingencies notes for guarantees between group entities;
- related-party disclosures for any intercompany items that remain after elimination.

The group should present only third-party cash flows, revenue, and balances in its external view, with any residual intercompany exposure disclosed.

### Evaluate The Effect Of Partial-Year And Step Acquisitions

Changes in the group structure during the period complicate eliminations because the consolidation perimeter shifts. Eliminations must reflect the period each entity was actually in the group.

Address:

- partial-year consolidation of entities acquired or disposed of mid-period;
- elimination of intercompany transactions only for the period both entities were consolidated;
- step acquisitions where control is obtained in stages, with re-measurement effects;
- loss of control disposals where the remaining interest is reclassified;
- common-control combinations and their specific elimination rules;
- the impact of these changes on comparatives.

Document the perimeter for each period and confirm the eliminations respect the dates of acquisition and disposal.

## Common Traps

### Ticking The Elimination Schedule Without Testing Completeness

A schedule that looks complete may omit entire intercompany relationships. Completeness must be tested from the entity ledgers, not assumed from the schedule.

### Eliminating Matched Balances But Ignoring Unmatched Ones

Netting or forcing unmatched intercompany balances embeds the difference in the consolidated accounts. Unreconciled differences must be investigated and corrected.

### Forgetting Unrealized Inventory Profit

Removing intercompany sales but leaving the profit in closing inventory overstates consolidated profit. The markup on on-hand intercompany inventory must be eliminated.

### Eliminating Principal But Not Interest Or Fees

Intercompany loans are eliminated, but the associated interest, accruals, and management charges remain. Eliminations must cover the full economic flow.

### Asymmetric Eliminations

The debit and credit of an elimination must match. Asymmetry distorts net assets, equity, and the non-controlling interest.

### Misclassifying External Vs Intercompany In Segments

Intercompany revenue presented as external revenue in segment reporting inflates reported scale. Segment disclosures must be reconciled to the elimination of intercompany sales.

### Ignoring Partial-Year And Step Acquisition Effects

Eliminations that span periods when an entity was not part of the group misstate the consolidated figures. The perimeter dates must drive the elimination periods.

### Treating Eliminations As A Mechanical Exercise

Eliminations reflect the substance of intercompany dealings, including transfer pricing and profit recognition. Treating them as arithmetic misses the economic distortions they are meant to remove.

## Self-Check

- Is the consolidation perimeter, methods, and acquisition and disposal dates clearly established before elimination testing?
- Has the intercompany population been built from both counterparties' ledgers and reconciled, with differences explained?
- Has elimination completeness been tested by reconciling consolidated totals to the sum of entity figures less intercompany items?
- Is unrealized intercompany profit in inventory computed from the markup and on-hand quantity, and eliminated from both inventory and profit?
- Are intercompany loans, interest, fees, and accruals eliminated in full across the balance sheet, income statement, and cash flow?
- Are unreconciled intercompany differences investigated and corrected rather than forced or netted?
- Are eliminations symmetric, with equal debits and credits and correct non-controlling interest adjustments?
- Do eliminations reconcile to the cash flow statement, segment reporting, and commitments and contingencies notes?
- Are partial-year, step acquisition, and common-control effects reflected in the elimination periods?
- Could another auditor reconstruct the consolidation from the worksheet and confirm no residual intercompany items remain in the external view?
