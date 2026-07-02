---
name: corporate_loss_limits.md
description: Use when the agent is applying corporate loss limitations, computing net operating loss usage under the 80 percent taxable income limit, evaluating Section 163(j) business interest expense 30 percent limitation, testing at-risk and passive activity rules for closely-held C corporations, or distinguishing which loss limits apply to C corps versus pass-throughs.
---

# Corporate Loss Limits

Corporate loss limitations are a stack of overlapping rules that determine how much of a corporation's economic loss is actually deductible in the current year. A C corporation can have a large book loss but a much smaller deductible loss for tax purposes because of net operating loss (NOL) usage limits, business interest expense caps, at-risk limitations, passive activity rules, and capital loss restrictions. The judgment problem is that agents often apply only one limit (usually the NOL rule) and miss that multiple limits stack, each independently reducing the currently deductible amount. A corporation that clears the NOL limit can still be capped by the Section 163(j) interest limitation or the passive activity rules.

Agents frequently miss that the Section 461(l) excess business loss limitation does not apply to C corporations (it applies to non-corporate taxpayers), that the at-risk and passive activity rules do apply to closely-held C corporations in specific circumstances, and that the Section 163(j) 30% business interest limitation applies to most businesses regardless of entity type. The deeper blind spot is treating loss limits as a single computation when they are a sequence: each limit is applied in order, and the result of one feeds the next. Failing to apply the full stack overstates the current deduction and understates the tax or the carryforward.

This skill covers the NOL 80% limitation, the inapplicability of Section 461(l) to C corporations, at-risk and passive rules for closely-held C corps, and the Section 163(j) interest limitation. It is not tax advice; the loss limitation rules have detailed regulations and exceptions, and a qualified tax professional (CPA or tax attorney) must be consulted for actual situations.

## Core Rules

### Apply The NOL 80% Of Taxable Income Limitation For Post-2017 NOLs

For net operating losses arising in tax years beginning after 2017 (post-TCJA), the NOL deduction in any carryforward year is limited to 80% of taxable income (computed before the NOL deduction itself). A corporation with $1,000,000 of taxable income (before NOL) and a $1,500,000 NOL carryforward can use only $800,000 of the NOL (80% of $1,000,000), leaving $700,000 to carry forward indefinitely. The 80% limit creates a permanent partial disallowance in the sense that the corporation can never fully offset taxable income with an NOL; at least 20% of taxable income remains subject to tax each year until the NOL is exhausted.

The 80% limit is computed against taxable income before the NOL deduction and before the qualified business income deduction, but after most other deductions and the DRD. The base for the 80% computation must be stated clearly, because using taxable income after the NOL would be circular. Pre-2018 NOLs (those arising in tax years beginning before 2018) are not subject to the 80% limit and can offset 100% of taxable income; they are used first (before post-2017 NOLs) under the ordering rules. Always identify the NOL vintage to apply the correct limitation percentage.

### Recognize Section 461(l) Excess Business Loss Does Not Apply To C Corporations

The Section 461(l) excess business loss limitation applies to non-corporate taxpayers (individuals, trusts, estates, and pass-through owners), not to C corporations. For non-corporate taxpayers, aggregate business deductions exceeding business income plus a threshold amount (approximately $305,000 single, $610,000 married filing jointly for 2024, indexed annually) generate an excess business loss that is disallowed currently and carried forward as an NOL. A C corporation is exempt from this rule entirely.

This is a critical distinction that agents often get wrong by importing the non-corporate rule. A C corporation with $5,000,000 of business losses and $1,000,000 of business income does not face the Section 461(l) limitation; the $4,000,000 net business loss flows into the NOL computation without the excess loss cap. The C corporation's loss is limited only by the NOL rules (80% usage), the interest limitation, and the at-risk and passive rules where applicable. Do not apply Section 461(l) to a C corporation; confirm the entity type before applying the excess business loss rule.

### Apply The Section 163(j) 30% Business Interest Expense Limitation

Section 163(j) limits the deduction of business interest expense to the sum of business interest income plus 30% of adjusted taxable income (ATI), with any disallowed interest carried forward. This limitation applies to most businesses regardless of entity type, including C corporations, and it is one of the most commonly overlooked corporate loss limits. ATI is generally taxable income before business interest expense, business interest income, depreciation, amortization, and depletion (with the addback of depreciation, amortization, and depletion scheduled to phase out, though extended by legislation).

A corporation with $2,000,000 of adjusted taxable income and $800,000 of business interest expense can deduct only $600,000 of the interest (30% of $2,000,000), with $200,000 disallowed and carried forward. The disallowed interest is not lost permanently; it carries forward indefinitely and becomes deductible in future years when the limitation allows (subject to the 80% NOL interaction in certain cases). Small businesses with average annual gross receipts under approximately $30 million (the Section 163(j) small-business exemption threshold, indexed and aggregated for related parties) are exempt from the limitation. Always test whether the corporation meets the gross-receipts exemption before applying Section 163(j), and compute ATI carefully because the depreciation addback treatment has changed.

### Test At-Risk Rules For Closely-Held C Corporations

The at-risk rules under Section 465 limit loss deductions to the amount the taxpayer has at risk in the activity. For C corporations, the at-risk rules generally do not apply except for closely-held C corporations engaged in certain activities (historically film, leasing, and some other activities, with the scope narrowed over time). A closely-held C corporation (one where more than 50% of the value of its stock is owned, directly or indirectly, by or for not more than 5 individuals) may be subject to the at-risk limitation for specified activities.

The at-risk amount generally includes cash and property contributed to the activity, borrowed amounts for which the taxpayer is personally liable (recourse debt), and certain qualified nonrecourse financing. Losses in excess of the at-risk amount are suspended and carried forward until the at-risk amount increases. For most operating C corporations, the at-risk rules are not a binding limit because the corporation's capital and recourse debt provide sufficient at-risk basis. But for closely-held C corporations in activities financed with nonrecourse debt or with thin capitalization, the at-risk rules can suspend losses. Confirm whether the corporation is closely-held and whether the activity is one to which the at-risk rules apply.

### Apply Passive Activity Rules To Closely-Held C Corporations And Personal Service Corporations

The passive activity loss rules under Section 469 limit the deduction of passive activity losses against active (non-passive) income. For C corporations, the passive activity rules apply to personal service corporations (PSCs) and to closely-held C corporations. A closely-held C corporation's passive losses can offset only passive income, not active business income; a PSC's passive losses are similarly restricted. For most other C corporations, the passive activity rules do not apply directly at the entity level (they apply to the owners of pass-throughs).

A closely-held C corporation with a profitable operating business and a loss-generating passive rental activity cannot deduct the passive rental loss against the operating income; the passive loss is suspended and carried forward until the corporation has passive income or disposes of the passive activity. The passive activity determination depends on material participation, which for a corporation is generally measured by the participation of its employees (subject to specific hour thresholds and the grouping rules). Confirm whether the corporation is a PSC or closely-held C corporation, and whether the loss-generating activity is passive, before assuming the loss is currently deductible.

### Sequence The Loss Limits In The Correct Order

The loss limitation rules are applied in a specific sequence, and the result of each limit feeds the next. Generally, the order is: first, characterize the loss (capital losses are restricted to capital gains separately); second, apply the passive activity rules (Section 469) to suspend passive losses; third, apply the at-risk rules (Section 465) to suspend losses beyond the at-risk amount; fourth, apply the business interest limitation (Section 163(j)); fifth, compute the NOL and apply the 80% usage limitation. Each limit operates on the result of the prior limits.

The sequencing matters because applying limits out of order can overstate or understate the deductible amount and the carryforward. For example, the Section 163(j) interest limitation reduces taxable income before the NOL computation, which affects both the NOL generated and the NOL usage in carryforward years. Document each limit applied, the input to that limit, and the output (current deduction versus suspended or carried-forward amount). A loss analysis that applies only one limit and ignores the stack is incomplete and likely wrong.

### Distinguish Loss Limits That Apply To C Corps From Those That Apply To Pass-Throughs

The loss limitation landscape differs between C corporations and pass-through entities. C corporations are subject to: the NOL 80% usage limit (post-2017 NOLs), the Section 163(j) interest limitation (unless small-business exempt), the at-risk rules (closely-held, specified activities), and the passive activity rules (PSCs and closely-held C corps). C corporations are NOT subject to: the Section 461(l) excess business loss limitation (non-corporate only), the owner-level basis limitations (Section 704(d) for partnerships, Section 1366 for S corps), or the separate owner-level at-risk and passive rules that apply to pass-through owners.

This distinction is essential for entity-choice and loss-analysis work. A pass-through owner faces basis, at-risk, passive, and Section 461(l) limits at the owner level, while a C corporation faces a different (and often less restrictive) set of limits at the entity level. Do not import pass-through loss limits into a C corporation analysis, and do not assume C corporation loss limits apply to pass-through owners. Confirm the entity type and apply the corresponding loss limit stack.

## Common Traps

### Applying Section 461(l) Excess Business Loss To A C Corporation

The excess business loss limitation applies only to non-corporate taxpayers. Applying it to a C corporation is a category error that understates the deductible loss.

### Missing The Section 163(j) Interest Limitation

The 30% business interest limitation applies to most C corporations (unless small-business exempt). Overlooking it overstates the current deduction by the amount of disallowed interest.

### Forgetting The NOL 80% Usage Limit

Post-2017 NOLs can offset only 80% of taxable income, not 100%. Applying a pre-2018 100% usage rule to a post-2017 NOL overstates the deduction and understates the carryforward.

### Ignoring Passive Activity Rules For Closely-Held C Corps And PSCs

Passive activity losses of closely-held C corporations and PSCs cannot offset active income. Assuming all corporate losses are currently deductible ignores the passive restriction.

### Applying Only One Loss Limit And Missing The Stack

Loss limits stack and sequence. Applying only the NOL rule while ignoring Section 163(j), at-risk, or passive rules overstates the deductible amount.

### Importing Pass-Through Loss Limits Into A C Corporation Analysis

Basis limitations (Section 704(d)), owner-level at-risk, and Section 461(l) apply to pass-through owners, not C corporations. Confirm entity type before applying the loss limit stack.

### Misstating The Adjusted Taxable Income Base For Section 163(j)

ATI for the interest limitation includes or excludes depreciation, amortization, and the NOL deduction depending on the year and legislation. Using the wrong ATI base misstates the 30% cap.

## Self-Check

- [ ] Has the post-2017 NOL 80% of taxable income limitation been applied (with the base computed before the NOL deduction and QBI), and have pre-2018 NOLs been separated and used first at 100%?
- [ ] Has it been confirmed that Section 461(l) excess business loss limitation does NOT apply to the C corporation, and that no excess loss cap has been incorrectly imposed?
- [ ] Has the Section 163(j) 30% business interest limitation been tested, with adjusted taxable income computed correctly (including the correct depreciation addback treatment for the year)?
- [ ] Has the small-business gross receipts exemption (approximately $30 million average annual, indexed, aggregated for related parties) been checked before applying Section 163(j)?
- [ ] Have the at-risk rules (Section 465) been tested if the corporation is closely-held and engaged in specified activities financed with nonrecourse or thin-capitalized debt?
- [ ] Have the passive activity rules (Section 469) been applied if the corporation is a PSC or closely-held C corporation with passive activities generating losses?
- [ ] Has the loss limit stack been sequenced correctly (characterization, passive, at-risk, interest, NOL), with each limit's input and output documented?
- [ ] Have the C corporation loss limits been distinguished from pass-through owner-level limits (basis, at-risk, passive, Section 461(l)), confirming the correct stack for the entity type?
- [ ] Has the agent noted that loss limitation rules have detailed regulations and exceptions, this is not tax advice, and recommended consulting a qualified tax professional (CPA or tax attorney) for the specific corporation?
