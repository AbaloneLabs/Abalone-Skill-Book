---
name: roth_conversion_analysis.md
description: Use when the agent is analyzing whether and how much to convert from a traditional IRA or 401k to a Roth account, sizing conversions to fill a tax bracket, modeling the five-year seasoning rule, IRMAA Medicare premium surcharges, ACA premium subsidy cliffs, or evaluating current versus future marginal rates for an irreversible conversion decision.
---

# Roth Conversion Analysis

A Roth conversion moves pre-tax dollars out of a traditional IRA or employer plan and into a Roth account, and the converted amount becomes taxable ordinary income in the year of conversion. The judgment problem is not whether Roth is good in the abstract but whether converting a specific dollar amount in a specific year is better than leaving those dollars deferred, and that question turns on a rate comparison that is easy to state and hard to model correctly. Because the conversion is irreversible and creates an immediate, known tax bill against an uncertain future benefit, the decision demands conservative sizing rather than aggressive optimization. An agent who converts too much pushes the taxpayer into a higher bracket, triggers stealth surcharges, or pays tax at a rate the taxpayer will never otherwise face, while an agent who converts too little forfeits a low-bracket year that may never recur.

The blind spots are numerous and expensive. Agents routinely convert to "the top of a bracket" without confirming where the bracket actually ends for the taxpayer's filing status, or they ignore that conversion income stacks on top of all other income so the marginal rate on the last converted dollar can be far higher than the headline bracket. They forget that each conversion starts its own five-year clock for penalty-free access, that IRMAA income from two years earlier drives current Medicare premiums so a large conversion creates a premium surcharge that arrives later, and that ACA premium tax credits phase out on a steep cliff where a single extra dollar of conversion income can cost thousands in lost subsidy. They also overlook the pro-rata aggregation rule, which can make an intended "tax-free" backdoor-style conversion mostly taxable, and they assume conversions can be undone when the law repealed recharacterization of conversions back in 2018. The harm is irreversible: tax paid at a rate higher than the taxpayer would ever have faced, surcharges that compound over years, and liquidity locked behind a seasoning clock.

This skill covers the sizing analysis for traditional-to-Roth conversions under U.S. federal tax law, including bracket fill-up, the five-year rule, IRMAA and ACA effects, and the pro-rata rule. It is educational guidance, not personalized tax advice; retirement tax planning is high-risk and a conversion is irreversible, so a qualified tax professional must be consulted before acting. Rules are jurisdiction-dependent and change over time, the 2017 tax law (TCJA) repealed conversion recharacterization and set bracket rates scheduled to sunset after 2025, and dollar amounts and bracket widths are indexed or legislated. Figures shown are 2024 and 2025 amounts and must be re-verified for the applicable tax year.

## Core Rules

### Frame The Conversion As An Irreversible Current Versus Future Rate Trade

A conversion is economically equivalent to paying tax now at the current marginal rate in exchange for withdrawing tax-free later, so the core comparison is the marginal rate paid on conversion against the marginal rate the converted dollars would otherwise face at withdrawal. If the current rate is lower than the realistic future withdrawal rate, converting is favorable; if higher, it is not. Because the TCJA repealed recharacterization of Roth conversions starting in 2018, a conversion cannot be undone once the tax year closes, which makes the rate analysis load-bearing rather than advisory.

Model the realistic future rate, not a guess. A retiree with a large traditional balance and eventual required minimum distributions may face a higher marginal rate in the future than today, which favors converting during low-income gap years before Social Security and RMDs begin. A high-earning worker in peak brackets converting now pays a rate they may never face again, which disfavors converting. Always state the assumed future rate explicitly and stress-test it, because overconfidence in a low future rate is the most common sizing error. When the comparison is genuinely ambiguous, conservative partial conversions that fill but do not exceed a bracket are safer than an all-or-nothing move.

### Fill To The Top Of A Bracket, Not Into The Next One

Conversion income is ordinary income that stacks on top of wages, pensions, Social Security, and other income, so the marginal rate on each converted dollar depends on the taxpayer's taxable income after everything else. The standard strategy is to convert an amount that brings taxable income up to the top of a target bracket, capturing that bracket's lower rate on every converted dollar without spilling into the next bracket. For 2024 the 12 percent bracket for a married couple ends at roughly 94,000 dollars of taxable income and the 22 percent bracket ends at roughly 201,000 dollars; for 2025 these rise to roughly 96,700 and 206,700 dollars. Converting to the top of the 12 percent bracket means the last converted dollar is taxed at 12 percent, but one dollar over the boundary is taxed at 22 percent.

The error is treating the bracket boundary as a soft target. Compute taxable income including the conversion, then size the conversion so that taxable income lands at or just below the boundary, leaving a safety margin for late-arriving income, a bonus, or an unexpected gain. Remember that the standard or itemized deduction creates a 0 percent zone at the bottom, so a retiree with little other income can convert a chunk tax-free up to the deduction amount before the 10 percent bracket even begins. Always recompute with the actual deduction, filing status, and other income, and verify the bracket width for the specific tax year, because these figures change annually.

### Respect The Five-Year Rule And The Roth Ordering Rules

Roth IRAs impose two distinct five-year clocks. The first is the five-year clock for qualified earnings, which starts when the first Roth IRA is established and must be satisfied along with age 59.5 for earnings to be tax- and penalty-free. The second is a separate five-year clock on each conversion, which governs whether the conversion principal can be withdrawn penalty-free before age 59.5; each conversion starts its own clock measured from January 1 of the conversion year. A pre-59.5 withdrawal of conversion principal before its five-year clock expires can trigger the 10 percent early-distribution penalty even though the conversion itself was taxed.

The Roth IRA ordering rule means distributions are deemed to come first from contributions, then conversions (oldest first), then earnings. This protects older conversions and direct contributions, but a taxpayer who converts and then needs liquidity may find the most recent conversion locked behind its seasoning clock. For Roth 401(k) conversions, the five-year rule for the destination Roth IRA applies, and rolling a Roth 401(k) into a Roth IRA can reset or interact with clocks in ways that must be checked. Never assume a converted dollar is freely accessible; confirm the applicable clock and the taxpayer's age before recommending a conversion that the taxpayer might need to tap.

### Model The IRMAA Threshold Cliff Before Sizing

The Income-Related Monthly Adjustment Amount (IRMAA) adds surcharges to Medicare Part B and Part D premiums based on modified adjusted gross income from two years prior, and the surcharges jump at specific income thresholds rather than phasing in gradually. For 2024, the thresholds begin at 103,000 dollars of MAGI for a single filer and 206,000 dollars for a married couple, with tiers that add roughly 70 to over 400 dollars per month per person in combined Part B and D surcharges at higher tiers. A conversion that pushes MAGI just over a threshold can cost a couple thousands of dollars in premium surcharges that apply for a full year, effectively raising the marginal cost of the last converted dollars far above the tax bracket rate.

Because IRMAA is based on income from two years earlier, a conversion done in the current year affects premiums two years later, which surprises taxpayers who expect the cost immediately. When sizing a conversion, treat each IRMAA tier boundary as a cliff and prefer to land below it rather than spill a few thousand dollars over. For a married couple both on Medicare, the surcharge applies to each spouse, doubling the cost of crossing a threshold. Always model IRMAA alongside the bracket, because the effective marginal rate including the surcharge can exceed the statutory bracket rate by a wide margin.

### Model The ACA Premium Subsidy Cliff For Pre-65 Taxpayers

For taxpayers under 65 buying coverage on the Affordable Care Act marketplace, premium tax credits phase out as income rises, and the subsidy can fall on a steep cliff where a modest increase in income eliminates a large subsidy. A conversion that pushes household income just over a threshold can wipe out thousands of dollars of annual premium assistance, and the marginal cost of that last converted dollar can be enormous. In some years and subsidy structures, the effective marginal rate from losing the subsidy can exceed 30 or 40 percent on top of the tax bracket.

A pre-65 early retiree in a low-income gap year is the classic conversion candidate, but the ACA subsidy makes the math delicate. Model the conversion income against the applicable premium credit threshold and treat the cliff as a hard boundary. If a desired conversion would cross the cliff, reduce the conversion to land below it, or accept the cost knowingly. Do not recommend a conversion that silently destroys a subsidy without flagging the dollar impact, because the lost subsidy is an irreversible cost just like the tax paid.

### Apply The Pro-Rata Aggregation Rule Across All IRAs

When a taxpayer converts any portion of their traditional IRA to Roth, the taxable fraction of the conversion is determined by the ratio of after-tax basis to total balance across all traditional, SEP, and SIMPLE IRAs, not just the account being converted. A taxpayer with 90 percent pre-tax IRA assets who converts a small after-tax slice still pays tax on 90 percent of the conversion under the pro-rata rule. This is the trap that breaks the backdoor Roth strategy for anyone with existing pre-tax IRA balances, and it applies to every conversion regardless of intent.

Model the pro-rata calculation before recommending any conversion that involves after-tax basis. If the taxpayer holds significant pre-tax IRA assets, options include rolling those pre-tax assets into an employer plan that accepts them (removing them from the IRA aggregation) before converting, or accepting that the conversion will be largely taxable. Never present a conversion of non-deductible contributions as tax-free without confirming the aggregated basis ratio. Note that employer plan balances like a 401(k) are excluded from the IRA pro-rata calculation, which is why moving pre-tax IRA money into a 401(k) can clean up the backdoor path.

### Plan Multi-Year Conversions Across Low-Income Gap Years

The most valuable conversion opportunities occur in gap years when income is unusually low, such as early retirement before Social Security and RMDs begin, a sabbatical, or a business loss year. Spreading conversions across several gap years lets the taxpayer fill the lower brackets repeatedly rather than converting a large lump sum that overflows into higher brackets. A multi-year plan that converts to the top of the target bracket each year can move a substantial balance to Roth at low rates, whereas a single large conversion in one year would pay much higher marginal rates.

Build the multi-year plan around the taxpayer's expected income trajectory, the years remaining before RMDs begin at age 73 (or 75 under SECURE 2.0 for those born after 1960), and the interaction with Social Security claiming. Confirm that each year's conversion is sized to its own bracket and IRMAA boundaries, and revisit the plan annually because income and law change. The risk is over-committing to a multi-year schedule that assumes future law and income remain favorable; keep each year's conversion an independent, reversible-in-size decision even though the tax paid is irreversible.

## Common Traps

### Converting Without Modeling The IRMAA Cliff

The symptom is a retiree who converts a round number and then receives a Medicare premium surcharge notice two years later. The trap is treating IRMAA as a minor side effect when it is a cliff-based surcharge that can raise the effective marginal cost of the last converted dollars far above the bracket rate, doubled for a couple. The direction is to model every IRMAA tier boundary as a hard cliff and size the conversion to land below it.

### Ignoring The ACA Subsidy Cliff For Pre-65 Retirees

The symptom is an early retiree who converts and then sees marketplace premiums jump or subsidies vanish at reconciliation. The trap is assuming conversion income is harmless because the taxpayer is not yet on Medicare, when the ACA subsidy cliff can impose an effective marginal rate exceeding the bracket. The direction is to model the premium credit threshold and treat crossing it as a costly boundary.

### Assuming Converted Funds Are Immediately Accessible

The symptom is a taxpayer who converts and then withdraws the conversion principal within five years while under 59.5, incurring the 10 percent penalty. The trap is forgetting that each conversion has its own five-year seasoning clock for penalty-free access of the principal. The direction is to confirm the applicable clock and the taxpayer's age, and to avoid converting dollars that may be needed for liquidity before seasoning completes.

### Forgetting The Pro-Rata Rule In A Backdoor Or Partial Conversion

The symptom is an unexpectedly large tax bill on a conversion intended to be tax-free. The trap is converting a small after-tax slice while ignoring that the taxable fraction is determined by the ratio of basis to total IRA balances across all traditional, SEP, and SIMPLE IRAs. The direction is to model the aggregated pro-rata calculation and consider moving pre-tax IRA assets to an employer plan first.

### Converting During A High-Income Year

The symptom is a peak-earning worker who converts to "get it over with" and pays a rate higher than any future withdrawal rate. The trap is converting based on intent rather than the current-versus-future rate comparison, paying tax at a marginal rate the taxpayer may never face again. The direction is to defer conversions to low-income gap years and convert only when the current rate is below the realistic future rate.

### Spilling Over A Bracket Boundary

The symptom is a conversion sized to "the top of the bracket" that actually lands a few thousand dollars into the next bracket, taxing the overflow at a much higher rate. The trap is treating the bracket boundary as approximate rather than exact, or ignoring other income that stacks with the conversion. The direction is to recompute taxable income including the conversion and leave a safety margin below the boundary.

### Assuming A Conversion Can Be Recharacterized Or Undone

The symptom is a taxpayer who converts, sees the market fall, and expects to reverse the conversion and redo it. The trap is relying on recharacterization, which the 2017 tax law repealed for conversions starting in 2018, so a conversion is locked in once the year closes. The direction is to treat every conversion as final, size conservatively, and consider splitting into smaller conversions rather than one large irreversible move.

### Overlooking State Income Tax On The Conversion

The symptom is a taxpayer who models only federal brackets and is surprised by a state tax bill, or who converts in a high-tax state before relocating. The trap is ignoring state-level ordinary income tax, which adds to the conversion cost and can change the optimal timing, especially across a relocation. The direction is to include state tax in the rate comparison and consider timing conversions relative to a move to a no-tax state.

## Self-Check

- [ ] Has the conversion been framed as an irreversible current-versus-future marginal rate comparison, with the assumed future rate stated explicitly and stress-tested rather than guessed?
- [ ] Has the conversion been sized to fill to the top of a target bracket (for example 2024 married 12 percent bracket ending near 94,000 dollars of taxable income, 22 percent near 201,000 dollars; 2025 near 96,700 and 206,700 dollars) with a safety margin and recomputation of taxable income including all other income?
- [ ] Have both Roth five-year clocks been addressed, the lifetime qualified-earnings clock and the per-conversion seasoning clock, and has the Roth IRA ordering rule been confirmed for any potential liquidity need?
- [ ] Has the IRMAA cliff been modeled (2024 thresholds starting at 103,000 dollars single and 206,000 dollars married MAGI, with per-person surcharges that double for a couple), sizing the conversion to land below a tier boundary?
- [ ] Has the ACA premium subsidy cliff been modeled for any pre-65 taxpayer, treating the loss of subsidy as a high effective marginal cost on the conversion?
- [ ] Has the pro-rata aggregation rule been computed across all traditional, SEP, and SIMPLE IRA balances before treating any conversion of after-tax basis as tax-free?
- [ ] Has a multi-year conversion plan been considered for low-income gap years, with each year sized to its own bracket and IRMAA boundaries and revisited annually?
- [ ] Has the irreversibility of conversions (recharacterization repealed for tax years after 2017) been confirmed, and has the conversion been split into smaller tranches rather than one large irreversible move?
- [ ] Has state income tax been included in the rate comparison, and has conversion timing been considered relative to any planned relocation?
- [ ] Has the agent noted that this is general educational guidance, not personalized tax advice, and recommended consulting a qualified tax professional before executing any conversion, given that conversions are irreversible, brackets and IRMAA thresholds change annually, and SECURE 2.0 altered RMD ages and related mechanics?
