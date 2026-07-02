---
name: multi_year_projection_and_bunching.md
description: Use when the agent is projecting tax over multiple years to optimize lifetime tax rather than a single year, sizing Roth conversions to fill brackets, bunching itemized deductions into alternating years, timing capital gains across years, or smoothing self-employment and variable income. Covers multi-year marginal-rate modeling, the standard-deduction bunching mechanic, conversion sizing against bracket ceilings, and the assumption-risk that makes multi-year optimization fragile.
---

# Multi-Year Projection And Bunching

Single-year tax optimization is a trap. A move that minimizes this year's tax — deferring a gain, skipping a Roth conversion, spreading deductions evenly — can raise total tax across a lifetime, because the real objective is the sum of tax paid over all years, not the lowest number on one return. Multi-year projection is the discipline of modeling several years together so that income and deductions are placed where they are cheapest, not merely deferred. The judgment problem is that the cheapest placement depends on the marginal rate in each future year, and those rates are projections, not facts — they depend on future earnings, future law, life events, and mortality, all of which are uncertain.

The blind spots are about false precision and the failure to see across year boundaries. Agents optimize one year in isolation, accelerating or deferring without checking where the moved dollars land later. They bunch deductions without confirming the off-year actually falls under the standard deduction, or they bunch so aggressively that they cross a charitable percentage limit. They size Roth conversions to the top of a bracket without modeling the next five years, over-converting into a year that turns out to be a low-income year, or under-converting and leaving cheap bracket room permanently unused. They treat a multi-year projection as deterministic, plugging in a single income path and presenting the output as a plan, when the projection is only as good as its assumptions and a single bad assumption (a retirement that happens earlier, a law that sunsets, a market crash) invalidates it. The harm is a plan that is locally optimal and globally worse, or a plan presented with false confidence that collapses when the future differs.

This skill covers multi-year tax projection, deduction bunching, Roth conversion sizing, gain timing, and income smoothing across a multi-year horizon. It is educational guidance, not personalized tax advice; projections depend on assumptions that must be stated and stress-tested, and a qualified tax professional (CPA or tax attorney) must be consulted for any actual situation. All rules are U.S. federal, jurisdiction-dependent, and subject to change — TCJA provisions sunset after 2025, bracket thresholds and the standard deduction are indexed annually, and conversion and contribution rules evolve with legislation.

## Core Rules

### Optimize Lifetime Tax, Not Single-Year Tax

The governing objective is the total tax paid across the planning horizon, discounted for time value, not the tax in any one year. A move that raises this year's tax but lowers future tax by more is worth making. A Roth conversion increases current-year tax (the converted amount is ordinary income) but removes future tax on growth and qualified distributions; if the conversion is taxed at a lower rate than the eventual withdrawal would have been, it is a lifetime win even though it raises this year's bill. Conversely, deferring a gain to "save tax this year" that lands in a higher bracket next year is a lifetime loss.

Reframe every recommendation against the multi-year total. Before recommending deferral, ask where the deferred dollar lands and at what rate. Before recommending a conversion, model the current cost against the avoided future tax. The single-year view is the most common source of bad multi-year advice, because it rewards pushing cost into the future without measuring the future cost. Always state the planning horizon (typically 5-10 years for conversion and gain planning, longer for estate-linked decisions) and sum the tax across it.

### Model The Marginal Rate In Each Year Of The Horizon

Multi-year optimization is marginal-rate arbitrage across years. For each year in the horizon, estimate taxable income, then read off the marginal bracket (10%, 12%, 22%, 24%, 32%, 35%, 37% under TCJA, reverting after 2025). Income should be placed in the cheapest years; deductions should be placed in the most expensive (itemizing) years. A taxpayer whose income drops sharply in a sabbatical, early-retirement, or pre-Social-Security year has a temporarily cheap year that is ideal for Roth conversions, gain realization, and Roth conversions of pre-tax money.

Build a year-by-year marginal-rate table. For each year, list expected wages, business income, investment income, required distributions (once RMDs begin), and Social Security, then compute taxable income and the marginal bracket. The cheap years (low bracket, often before RMDs and Social Security start) are where income should be pulled forward via conversion or gain realization. The expensive years (high bracket, often after RMDs begin and push income up) are where income should have been pulled out earlier. The table is the backbone of the plan, and every bunching or conversion decision reads off it.

### Bunch Deductions By Concentrating Itemizable Items Into Alternating Years

Bunching exploits the standard deduction. In a year the taxpayer itemizes, each deductible dollar saves at the marginal rate; in a year the taxpayer claims the standard deduction, itemizable deductions save nothing. With the TCJA's higher standard deduction (roughly $29,200 MFJ, $14,600 single for 2024), many taxpayers are just above or below the itemizing threshold, making bunching powerful. The mechanic is to concentrate two years of charitable gifts, and where possible state tax prepayments, into one itemizing year, then claim the standard deduction in the off-year.

Confirm the bunching math before executing. If total itemizable deductions in a normal year are $18,000 (below the $29,200 MFJ standard), the taxpayer gets the standard deduction either way and bunching does nothing. But if prepaying next year's charitable gift pushes the itemizing year to $36,000, that year itemizes ($36,000 instead of $29,200 standard, saving tax on $6,800) while the off-year still claims the $29,200 standard — capturing both the standard deduction and the excess itemized amount. A donor-advised fund is the common vehicle: contribute two years of giving in the itemizing year, grant it out over two years. Watch the charitable percentage limits (60% of AGI for cash to public charities) and the SALT cap ($10,000), which limits how much state tax can be bunched.

### Size Roth Conversions To Fill, Not Overflow, The Bracket Ceiling

Roth conversion sizing is the canonical multi-year decision. The goal is to convert pre-tax money up to the top of a target bracket (often the 24% bracket, or the top of the 0% capital-gains bracket) in each cheap year, maximizing the amount moved at the low rate without spilling into the next bracket. Spilling one dollar into the 32% bracket taxes that dollar at 32% instead of 24%, an 8-point jump that can erase the benefit of the conversion below it. The conversion must be sized to the bracket ceiling, accounting for all other income in the year.

Model the conversion against the full-year income, not in isolation. A taxpayer with $150,000 of other taxable income in 2024 (MFJ) has the 24% bracket ceiling at roughly $383,900, so up to about $233,900 can be converted at 24% or below before hitting 32%. But each conversion also affects NIIT (3.8% above $250,000 MAGI), IRMAA tiers (two-year lookback), and the taxation of Social Security. Size conservatively below the ceiling to leave room for late-year income surprises, and re-check the actual bracket before executing in December, because a bonus or unexpected gain can consume the planned conversion room. Never convert a fixed round number; convert the number the model produces.

### Time Capital Gains Into Cheap Years And Against Bracket Space

Long-term capital gains have their own bracket structure (0%, 15%, 20%) that is independent of but aligned with the ordinary brackets. A taxpayer in the 0% capital-gains bracket (roughly taxable income up to $94,050 MFJ for 2024) pays no federal tax on long-term gains, which makes realizing gains in a cheap year extremely valuable. Conversely, realizing gains in a year the taxpayer is in the 20% bracket plus 3.8% NIIT is expensive. Gain timing is the placement of realizations into the years with the lowest capital-gains rate.

Coordinate gain realization with the ordinary-income model. In a low-income year, a taxpayer may have room in the 0% capital-gains bracket to realize gains tax-free, and may even pair gain realization with a Roth conversion (ordinary income) up to the point where both fill the cheap brackets without spilling. In a high-income year, defer gains if possible or harvest losses to offset them. Watch the interaction: ordinary income pushes up the taxable-income line that determines the capital-gains bracket, so a large conversion can push gains from the 0% to the 15% bracket. Model the two together.

### Smooth Variable And Self-Employment Income Across Years

Self-employed and variable-income taxpayers face feast-or-famine taxation: a high-income year lands in a high bracket while a low-income year leaves bracket space unused. Multi-year smoothing places income — through timing of invoicing, retirement-plan contributions, and deferral mechanisms — so that the average rate across years is lower than the peak-year rate. A consultant earning $300,000 one year and $50,000 the next pays more total tax than one earning $175,000 each year, because of the progressive bracket structure.

Use legitimate timing levers to smooth. Cash-method taxpayers can defer invoicing from a strong December into January to shift income to a weaker next year, or accelerate billing into a strong year only if next year is stronger. Retirement-plan contributions (Solo 401(k), SEP-IRA, defined-benefit plans for high earners) shift income into tax-deferred accounts. However, smoothing is bounded by constructive receipt (income already earned cannot be deferred) and by the risk that the "weak" next year turns out strong. State the smoothing levers as conditional on the income forecast, and do not over-rely on a single forecast.

### Stress-Test Every Projection Against Assumption Risk

A multi-year projection is only as reliable as its assumptions, and the key assumptions — future income, future tax law, future health and longevity, market returns — are all uncertain. A conversion plan built on the assumption that the taxpayer retires at 65 and takes RMDs at 75 collapses if the taxpayer works to 72 (raising conversion-year brackets) or dies early (eliminating the future tax the conversion was meant to avoid). The TCJA sunset assumption (rates rising after 2025) may or may not occur. Presenting a single deterministic projection as "the plan" is false precision.

Run scenarios. Model a base case, an optimistic case (lower future income, lower future rates), and a pessimistic case (higher future income, higher future rates, earlier death). Prefer moves that are robust across scenarios — for example, moderate annual conversions that work whether rates rise or stay — over moves that depend on one outcome. For irreversible moves (a large one-time conversion), be more conservative because there is no do-over. Always label projections as conditional and avoid presenting point estimates of savings without the range.

## Common Traps

### Optimizing One Year And Ignoring Where The Dollars Land Later

The symptom is recommending deferral or a deduction move that minimizes this year's tax. The trap is that the deferred income or skipped conversion lands in a more expensive future year, raising lifetime tax. Always project the moved dollars across the horizon.

### Bunching Without Confirming The Off-Year Clears The Standard Deduction

The symptom is prepaying charitable gifts to itemize one year. The trap is that if the off-year's itemizable deductions still exceed the standard deduction, bunching captured no extra benefit, or if the itemizing year's deductions are too low, bunching wasted the off-year's standard deduction. Verify the math both years.

### Over-Converting Past The Bracket Ceiling

The symptom is converting a round number that spills into the next bracket. The trap is that the overflow dollars are taxed at 32% instead of 24% (or trigger NIIT and IRMAA tiers), erasing the benefit. Size to the ceiling with a margin for late income surprises.

### Treating A Deterministic Projection As A Fact

The symptom is presenting a single income path and a single tax-savings number as the plan. The trap is false precision — one changed assumption (retirement age, law, longevity, market) invalidates the output. Run scenarios and present a range, not a point estimate.

### Ignoring The Capital-Gains Bracket Interaction With Ordinary Income

The symptom is realizing gains assuming the 0% bracket applies. The trap is that a Roth conversion or other ordinary income pushes taxable income up and shifts gains from 0% to 15%. Model ordinary income and capital gains together.

### Forgetting IRMAA And NIIT In Conversion And Gain Sizing

The symptom is sizing a conversion to the ordinary bracket ceiling only. The trap is that IRMAA tiers (two-year lookback on Medicare premiums) and NIIT (3.8% above the MAGI threshold) add effective marginal cost that the bracket alone misses. Include threshold taxes in the marginal-rate model.

### Bunching State Tax Beyond The SALT Cap

The symptom is prepaying multiple years of state and local tax to bunch. The trap is the $10,000 SALT cap, which limits the deductible state-tax portion regardless of how much is prepaid, so state-tax bunching is largely ineffective while charitable bunching remains useful. Do not assume state tax can be bunched.

### Under-Using A Genuinely Cheap Year Out Of Caution

The symptom is leaving bracket space empty in a low-income year because "we can always convert later." The trap is that cheap years are finite and often temporary (pre-RMD, pre-Social-Security), and unused cheap bracket space is permanently lost. Convert modestly into cheap years rather than zero.

## Self-Check

- [ ] Has the recommendation been evaluated against lifetime (multi-year) tax, not just the current year, with the deferred or moved dollars traced to their landing year?
- [ ] Has a year-by-year marginal-rate table been built across the planning horizon, identifying the cheap years for income and the expensive years for deductions?
- [ ] Has deduction bunching been confirmed to clear the standard deduction in the itemizing year and to benefit from the standard deduction in the off-year, within charitable percentage limits?
- [ ] Has each Roth conversion been sized to fill a target bracket ceiling without overflowing, with a margin for late-year income surprises?
- [ ] Have NIIT (3.8%) and IRMAA tiers been included in the marginal-rate model for conversion and gain sizing, not just the ordinary bracket?
- [ ] Has capital-gains timing been modeled jointly with ordinary income, so that conversions do not inadvertently push gains from 0% to 15%?
- [ ] Has variable or self-employment income smoothing used only legitimate timing levers, with constructive receipt respected and the income forecast treated as uncertain?
- [ ] Has the projection been stress-tested across base, optimistic, and pessimistic scenarios, with irreversible moves sized more conservatively?
- [ ] Has the TCJA sunset (after 2025) and annually indexed amounts been flagged as assumptions that can shift the plan, rather than constants?
- [ ] Has the agent noted that this is general educational guidance, not personalized tax advice, that projections depend on assumptions and rules that are jurisdiction-dependent and change over time, and recommended consulting a qualified tax professional for the specific situation?
