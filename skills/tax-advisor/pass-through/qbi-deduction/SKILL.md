---
name: qbi_deduction.md
description: Use when the agent is computing the Section 199A 20 percent qualified business income deduction, classifying a business as SSTB versus non-SSTB, testing taxable income against the QBI thresholds, applying the W-2 wage and UBIA property limits, evaluating aggregation of multiple trades or businesses, or handling REIT and PTP income in the deduction.
---

# QBI Deduction Section 199A

The Section 199A qualified business income (QBI) deduction is a 20% deduction on qualified business income from pass-through entities, but it is governed by income thresholds, business-type exclusions, and a two-part limitation (W-2 wages and unadjusted basis of property) that switch on only above certain taxable-income levels. The judgment problem is that the deduction looks simple at the bottom of the income scale and becomes a multi-step computation above the thresholds, and the business-type classification (SSTB versus non-SSTB) can eliminate the deduction entirely regardless of income.

Agents frequently apply the flat 20% to QBI without testing whether the taxable-income threshold triggers the wage/property limits, or they classify a business as non-SSTB based on the owner's self-description rather than the principal activity. The harm is an overstated deduction that collapses on audit, or conversely an understated deduction because aggregation was not elected when it would have helped. A second silent failure is ignoring that the deduction sunsets after 2025 — planning built around QBI may not survive, and multi-year projections must reflect the scheduled expiration.

This skill covers the Section 199A QBI deduction computation, SSTB classification, threshold and wage/UBIA limitations, aggregation, and REIT/PTP income. It is not tax advice; QBI rules contain detailed definitions of qualified business income, SSTB fields, and anti-abuse rules, and outcomes depend on facts that must be verified. Consult a qualified tax professional (CPA or tax attorney) before relying on any conclusion. Thresholds cited are 2024 figures and are indexed annually.

## Core Rules

### Test Taxable Income Against The Threshold Before Applying Limits

The W-2 wage and UBIA property limits apply only when taxable income exceeds a threshold that is indexed annually. For 2024, the threshold is approximately $191,950 for single and head of household filers and $383,900 for married filing jointly (these figures are inflation-adjusted; verify the exact year). Below the threshold, the deduction is simply 20% of QBI with no wage or property limit. Above the threshold, the deduction is limited to the lesser of 20% of QBI or the greater of 50% of W-2 wages or 25% of W-2 wages plus 2.5% of the unadjusted basis (UBIA) of qualified property.

The threshold is based on taxable income before the QBI deduction, not on QBI itself, so a taxpayer with modest business income but large other income (capital gains, spouse's salary) can still be pushed above the threshold and into the limitation regime. Always determine taxable income first, then determine whether the limitation applies. For a taxpayer straddling the threshold, the limitation phases in over a range (the threshold amount plus $50,000 single / $100,000 MFJ), requiring a blended computation rather than an all-or-nothing application.

### Classify SSTB Versus Non-SSTB Using The Principal Activity

A specified service trade or business (SSTB) is excluded from QBI entirely once taxable income exceeds the top of the phase-in range. SSTBs are businesses in the fields of health, law, accounting, actuarial science, performing arts, consulting, athletics, financial services, brokerage services, investing and investment management, trading, dealing in securities, and any business whose principal asset is the reputation or skill of one or more employees. Architecture and engineering are explicitly excluded from the SSTB list and qualify as non-SSTB. The classification turns on the principal activity, not a secondary or incidental one.

The trap is over-broad or under-broad classification. A consulting firm is SSTB, but a business that sells products alongside consulting may or may not be SSTB depending on whether the principal asset is the reputation/skill or the product. The "reputation or skill" catch-all was narrowed by regulation to mean the business relies on its reputation for a skill, not mere brand reputation or marketing goodwill. An agent should not classify based on the owner's label; examine what the business actually does and whether it fits a listed field or the catch-all. Misclassification in either direction produces a wrong deduction.

### Compute QBI As Ordinary Qualified Income, Not Total Revenue

QBI is the net amount of qualified items of income, gain, deduction, and loss from a qualified trade or business, effectively the ordinary business income that flows to the owner. It excludes capital gains and losses, qualified dividend income, interest income not allocable to the trade or business, wage income, and income from outside the United States. It is reduced by related deductions, and it does not include the Section 199A deduction itself. For an S corporation or partnership, QBI is the ordinary income on the K-1 before separately stated items; for a sole proprietor, it is Schedule C net profit.

QBI is computed at the entity or activity level and then allocated to the owner, but certain items are determined at the owner level (e.g., the net capital gain used in the overall 20% cap is the owner's total net capital gain). Negative QBI from one business reduces the combined QBI from all businesses of the owner, and a negative overall QBI carries forward to reduce the next year's deduction. An agent should compute QBI per business, combine them, and apply the overall limits — do not apply 20% to each business in isolation if the owner has multiple activities.

### Apply The W-2 Wage And UBIA Property Limitation Correctly

When taxable income exceeds the threshold, the deductible QBI component is limited to the greater of 50% of the owner's share of W-2 wages or 25% of W-2 wages plus 2.5% of the owner's share of UBIA of qualified property. W-2 wages are the total wages paid by the business subject to wage withholding, unreported tips, and reported tips; they do not include distributions or guaranteed payments. UBIA of qualified property is the unadjusted basis (immediately after acquisition) of tangible, depreciable property held by the business at year-end, used in the production of QBI, with a depreciable period that has not ended.

The wage/property limit is a common point of error because businesses with few employees and little property (e.g., a solo consultant) can have a very low limit, sharply reducing the deduction above the threshold. Conversely, capital-intensive businesses (manufacturing, equipment rental) often clear the limit easily. An agent should compute both alternatives (50% wages vs. 25% wages + 2.5% UBIA) and take the greater, and should confirm that the property counted is qualified property (depreciable, held at year-end, within the depreciable period) rather than land or fully depreciated assets outside the period.

### Evaluate Aggregation Across Multiple Trades Or Businesses

A taxpayer with more than one qualified trade or business may elect to aggregate them for QBI purposes, combining their QBI, W-2 wages, and UBIA property. Aggregation can increase the deduction when one business has high income but low wages and another has high wages — combining them may clear the wage/property limit. The requirements are strict: the businesses must have the same majority owner, be of the same type, and meet coordination and overlap tests (common location, shared employees, shared accounting, or actual interdependence).

The aggregation election is made annually and is not automatic; once made for a year it cannot be revoked for that year. An agent should test whether aggregation helps by computing the deduction with and without it, because aggregation is not always beneficial (combining can sometimes reduce the deduction if one business has negative QBI). Document the facts supporting each aggregation requirement, because the IRS can challenge aggregation that lacks the required commonality. Do not assume aggregation is allowed just because the same person owns the businesses.

### Include REIT Dividends And PTP Income With Separate Rules

Qualified REIT dividends and income from publicly traded partnerships (PTPs) qualify for the 20% QBI deduction but are not subject to the W-2 wage or UBIA property limitations, and are not subject to the SSTB exclusion (a REIT or PTP cannot be an SSTB). These amounts are combined separately and the deduction is 20% of the combined REIT/PTP income, then added to the qualified business income component. This means a high-income taxpayer whose own business is an SSTB (and thus fully excluded) can still get a deduction on REIT and PTP income.

The trap is missing this separate bucket or applying the wage/property limit to it. REIT dividends must be qualified (Section 858) dividends, not ordinary REIT distributions that are a return of capital, and PTP income must be from a PTP that is not an SSTB at the entity level. An agent should segregate REIT/PTP income from trade or business QBI, apply 20% to each bucket under its own rules, and combine the results subject to the overall taxable-income cap.

### Apply The Overall Taxable Income Cap

The total QBI deduction (business income component plus REIT/PTP component) cannot exceed 20% of the excess of taxable income over net capital gain. Net capital gain is the sum of net long-term capital gain and qualified dividends. This cap means a taxpayer with large capital gains and modest business income can have the deduction limited by the cap even if the wage/property limit is not binding. Compute the cap explicitly: 20% of (taxable income minus net capital gain), and take the lesser of the computed deduction or the cap.

The cap operates as an outer boundary that interacts with all the other limits. An agent who computes the per-business deduction and the REIT/PTP deduction but forgets the overall cap will overstate the deduction for taxpayers with significant investment income. Always finish the computation by testing the combined deduction against 20% of (taxable income less net capital gain).

## Common Traps

### Applying A Flat 20% Without Testing The Threshold

Below the taxable-income threshold the deduction is 20% of QBI, but above it the wage/property limit and SSTB exclusion engage. The trap is computing 20% mechanically without checking whether taxable income exceeds the threshold, overclaiming for high earners.

### Misclassifying A Business As Non-SSTB

A business whose principal asset is the reputation or skill of its employees, or that operates in a listed field, is an SSTB and is excluded above the phase-in range. The trap is classifying based on the owner's label or a secondary activity, either overstating the deduction (calling an SSTB non-SSTB) or understating it.

### Forgetting The Deduction Sunsets After 2025

Section 199A is scheduled to expire for tax years beginning after December 31, 2025. The trap is building multi-year planning around QBI without reflecting the sunset, or failing to flag that the deduction may not exist in future years.

### Ignoring The Overall Taxable Income Cap

The deduction cannot exceed 20% of taxable income minus net capital gain. The trap is computing the per-business and REIT/PTP deductions but not testing the combined result against the cap, overstating the deduction for taxpayers with large investment income.

### Overlooking Aggregation When It Would Help

A taxpayer with multiple businesses can aggregate them to combine wages and property, potentially increasing the deduction. The trap is computing each business in isolation and missing a beneficial aggregation, or aggregating without meeting the strict commonality requirements.

### Applying The Wage/Property Limit To REIT And PTP Income

REIT dividends and PTP income qualify for the 20% deduction without the wage/property limit and without the SSTB exclusion. The trap is either excluding them or applying limits that do not apply to that bucket.

### Using The Wrong Base For QBI

QBI is ordinary qualified business income, not total revenue and not including capital gains, dividends, or investment interest. The trap is using gross receipts or a net figure that includes non-qualifying items, distorting the 20% base.

## Self-Check

- [ ] Taxable income (before the QBI deduction) has been tested against the indexed threshold (2024 approx. $191,950 single / $383,900 MFJ) to determine whether the wage/property limit and SSTB rules apply, with phase-in blending handled if income is within the range.
- [ ] The business has been classified as SSTB or non-SSTB based on its principal activity (listed fields or reputation/skill catch-all), with architecture and engineering correctly treated as non-SSTB.
- [ ] QBI has been computed as ordinary qualified business income (excluding capital gains, dividends, investment interest, and non-business income), combined across all of the owner's businesses.
- [ ] The W-2 wage/UBIA property limitation has been computed as the greater of 50% of W-2 wages or 25% of wages plus 2.5% of UBIA of qualified property, when taxable income exceeds the threshold.
- [ ] Aggregation across multiple trades or businesses has been evaluated (computed with and without) and, if elected, the commonality requirements (same majority owner, same type, coordination/overlap) are documented.
- [ ] REIT dividends and PTP income have been segregated into a separate bucket, with 20% applied without the wage/property limit or SSTB exclusion, and combined with the business income component.
- [ ] The overall cap of 20% of (taxable income minus net capital gain) has been applied as the outer limit on the combined deduction.
- [ ] The scheduled sunset after 2025 has been noted in any multi-year planning, so projections do not assume the deduction continues indefinitely.
- [ ] Negative QBI from one business has been netted against positive QBI from others, and any negative overall QBI has been carried forward to reduce the next year's deduction.
- [ ] The conclusion notes QBI rules have detailed definitions and anti-abuse provisions, this is not tax advice, and recommends consulting a qualified tax professional (CPA or tax attorney) before relying on any computation.
