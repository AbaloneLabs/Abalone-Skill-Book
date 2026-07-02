---
name: share_based_payment_accounting.md
description: Use when the agent is accounting for share-based payment including stock options, RSUs, SARs, or ESPP, measuring fair value at grant date, recognizing expense over vesting, handling service performance or market conditions, modifications, cancellations, or forfeitures.
---

# Share Based Payment Accounting

Share-based payment lets a company pay employees or suppliers with equity instruments instead of cash, but the accounting is deceptively heavy. Every grant requires a fair value measurement at grant date, an estimate of how many awards will vest, and expense recognition over the vesting period, with re-vesting, modification, cancellation, and forfeiture rules layered on top. The expense is real even though no cash leaves, and the difference between equity-settled and cash-settled awards changes whether the liability is remeasured. Agents often skip the fair value work, recognize expense only when shares vest, or forget to reverse forfeitures, producing compensation expense that disagrees with the cap table and the equity rollforward.

Use this skill before granting equity awards, measuring fair value, recognizing compensation expense, modifying or cancelling awards, or preparing share-based payment disclosures. The goal is to prevent the agent from treating equity compensation as a paperwork event and missing the measurement, recognition, and repricing mechanics.

Share-based payment accounting differs between US GAAP (ASC 718) and IFRS (IFRS 2), particularly on forfeiture handling, the definition of a modification, and the treatment of cash-settled awards and reload features. Local GAAP and tax rules may vary further. This skill gives the general framework; the applicable standard governs. Fair value measurement of complex awards and tax accounting should involve a valuation specialist and a qualified accountant. This is structural guidance, not a valuation or tax conclusion.

## Core Rules

### Classify The Award As Equity-Settled Or Cash-Settled

The classification drives the entire accounting. An equity-settled award (stock option, RSU settled in shares) is measured at fair value at grant date and not remeasured. A cash-settled award (SAR settled in cash, phantom stock) creates a liability measured at fair value at each reporting date through settlement. An award with a choice of settlement, or where the employer has the present obligation to settle in cash, follows the cash-settled model.

Identify the settlement mechanism before measuring anything. An award that looks like equity but is cash-settled creates a remeasured liability and a different expense pattern. A misclassification distorts compensation expense, equity, and liabilities from grant date onward.

### Measure Fair Value At Grant Date

For equity-settled awards, the fair value is measured at grant date and is not updated. For options, this typically requires an option pricing model (Black-Scholes, binomial lattice) that incorporates the share price, exercise price, expected term, expected volatility, risk-free rate, and expected dividends. For RSUs, fair value is generally the share price at grant date adjusted for any post-vesting restrictions.

Document the inputs and the model. Expected volatility, expected term, and forfeiture rates are estimates that must be supportable. For awards with market conditions (target share price), the fair value already reflects the condition, so no separate probability assessment is layered on top. Do not measure equity-settled awards at intrinsic value unless the exception for nonpublic entities applies.

### Recognize Expense Over The Requisite Service Period

Compensation expense is generally recognized over the period during which the employee is required to provide service to earn the award (the requisite or vesting period), typically graded or cliff vesting. For equity-settled awards, the total grant-date fair value is spread over the vesting period and not trued up for share price changes after grant.

For graded vesting, recognize expense on a straight-line basis over the total vesting period for the entire award, or accelerate recognition by tranche; confirm the applicable policy. The grant-date fair value times the number of awards expected to vest (adjusted for estimated forfeitures, depending on the framework) is the total expense to recognize.

### Handle Service, Performance, And Market Conditions Differently

A service condition requires only continued employment; expense is recognized over the service period and reversed if the employee leaves before vesting. A performance condition requires meeting a target (revenue, EBITDA); expense is recognized based on the probability of meeting the target, with a true-up when the outcome is known. A market condition (share price target) is already embedded in the grant-date fair value, so the expense is recognized regardless of whether the market condition is ultimately met, as long as service is completed.

Confusing these conditions is a frequent error. A performance condition requires probability assessment and reversal if not met; a market condition does not. Identify the condition type for each award before recognizing expense.

### Account For Modifications By Comparing Fair Values

A modification (change to terms, exercise price, vesting schedule, or settlement) is accounted for by comparing the fair value of the modified award with the fair value of the original award immediately before modification. The incremental fair value (if positive) is recognized over the remaining vesting period in addition to the original expense. If the modification reduces fair value, the original recognition continues.

A repricing (changing the exercise price of options) is a modification that typically creates incremental value. A change in vesting conditions, a change from equity-settled to cash-settled, or adding a reload feature each require the modification analysis. Do not restart the clock from zero; layer the incremental value onto the existing recognition.

### Account For Cancellations And Forfeitures

When an award is cancelled or forfeited, any unrecognized expense is reversed. For a cancellation that is replaced by a new award with substantially the same terms, the cancellation and new grant may be treated as a modification rather than a fresh grant. A cashout of an award for its intrinsic value is a settlement.

Forfeiture estimates affect equity-settled awards under frameworks that require estimation; if an entity elects to account for forfeitures as they occur, no estimate is needed. Whichever policy is chosen, apply it consistently and reconcile to actual forfeitures. Cash-settled awards are trued up to actual at each date.

### Recognize The Tax Effects And Deferred Tax

Share-based payment often creates a book-tax difference. For equity-settled awards, the deferred tax asset is measured based on the intrinsic value at the reporting date (the spread between market price and exercise price), not the grant-date fair value. This can create a deferred tax asset that grows as the share price rises, or a windfall or shortfall recognized in equity when the award vests.

Track the tax effect separately from the compensation expense. A shortfall (tax deduction less than book expense) reduces additional paid-in capital; a windfall (tax deduction more than book expense) increases additional paid-in capital, subject to the applicable rules. Missing the tax accounting understates deferred taxes and misstates equity.

### Disclose The Expense, Inputs, And Outstanding Awards

Disclose the total compensation expense recognized, the fair value method and inputs (volatility, expected term, risk-free rate, dividends), the number and weighted-average grant-date fair value of awards granted, vested, forfeited, and outstanding, the weighted-average remaining life, and the aggregate intrinsic value. Users rely on this to assess dilution and compensation cost.

## Common Traps

### Recognizing Expense Only At Vesting

The expense belongs to the service period, not the vesting date. Booking the full expense when shares vest understates earlier-period compensation cost and overstates later-period profit.

### Remeasuring Equity-Settled Awards At Fair Value Each Period

Equity-settled awards are measured once at grant date. Updating the expense for share price changes treats them like cash-settled awards and misstates both expense and equity.

### Confusing Performance And Market Conditions

A performance condition requires probability assessment and reversal if not met; a market condition is embedded in grant-date fair value and is not separately trued up. Applying the wrong model over- or under-states expense.

### Skipping The Modification Incremental Value Analysis

A repricing or term change creates incremental fair value that must be layered onto existing recognition. Treating the modification as a fresh grant or ignoring it leaves expense understated.

### Using Intrinsic Value For Options Without Justification

Options must be measured with an option pricing model unless a specific nonpublic-entity exception applies. Using intrinsic value (zero for at-the-money options) understates the grant and the expense.

### Forgetting Forfeiture Reversals Or True-Ups

If forfeitures were estimated and the employee leaves, reverse the unrecognized expense. If forfeitures are accounted for as they occur, ensure the policy is consistent. Stale forfeiture estimates leave expense overstated.

### Missing The Deferred Tax Accounting

The deferred tax asset is based on intrinsic value at the reporting date, and windfalls or shortfalls hit equity. Ignoring this understates deferred taxes and misstates the equity rollforward.

## Self-Check

- Has each award been classified as equity-settled, cash-settled, or with a choice, with the matching measurement and remeasurement model applied?
- Is the grant-date fair value measured using an appropriate option pricing model or share price, with documented inputs (volatility, expected term, risk-free rate, dividends)?
- Is compensation expense recognized over the requisite service period on the correct vesting basis, not only at vesting?
- Are service, performance, and market conditions distinguished, with probability assessment and true-ups applied only where required?
- Are modifications analyzed by comparing modified to original fair value, with incremental value recognized over the remaining vesting period?
- Are cancellations, replacements, and forfeitures accounted for, with unrecognized expense reversed and replacement awards treated as modifications where appropriate?
- Are cash-settled awards remeasured at fair value at each reporting date through settlement?
- Is the deferred tax asset measured on intrinsic value, with windfalls and shortfalls recorded in equity?
- Does the equity rollforward reconcile shares and additional paid-in capital to the cap table and grant records?
- Are expense, valuation inputs, and outstanding award activity disclosed, and has a valuation specialist or qualified accountant reviewed fair value and tax positions?
