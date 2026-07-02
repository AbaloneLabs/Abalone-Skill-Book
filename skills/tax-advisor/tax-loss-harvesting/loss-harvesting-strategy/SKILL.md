---
name: loss_harvesting_strategy.md
description: Use when the agent is designing or evaluating a tax-loss harvesting strategy, deciding which losses to realize, sizing the harvest against the 3000 ordinary income offset and carryforwards, replacing sold positions without triggering wash sales, or modeling the annual tax benefit of harvesting. Covers capital gain and loss netting, the 3000 ordinary offset, carryforward mechanics, basis adjustment of replacements, and the tradeoff between tax savings and portfolio drift.
---

# Loss Harvesting Strategy

Tax-loss harvesting is the practice of intentionally realizing investment losses to offset realized capital gains and, to a limited extent, ordinary income, thereby reducing current-year tax. The mechanics are governed by the capital gain and loss netting rules of IRC Section 1222, the $3,000 annual ordinary income offset, and indefinite carryforward of unused losses. The strategy sounds simple (sell what is down, capture the loss), but the value depends on the taxpayer's gain position, their ordinary bracket, the quality and basis of the replacement security, and the long-term cost of a permanently reduced cost basis. A poorly executed harvest can save a few hundred dollars today and cost thousands in future taxes or portfolio distortion.

The judgment problem is that agents treat harvesting as a free benefit: any loss is worth realizing. In reality, harvesting exchanges a current tax deduction for a lower cost basis, which means the replacement position will generate a larger gain when eventually sold. The benefit is a timing benefit (deferring tax), not a permanent elimination, unless the losses are used against the $3,000 ordinary offset (which is a permanent benefit at the ordinary rate) or until death (when basis steps up and the deferral becomes permanent). Agents also routinely ignore wash sale rules, harvest tiny losses that are eaten by transaction costs and bid-ask spread, and replace a sold position with a security that does not preserve the portfolio's risk and factor exposure. The harm is a strategy that looks productive on paper but destroys value through basis reduction, wash sale disallowance, or portfolio drift.

This skill applies to tax-loss harvesting design, capital gain and loss netting, the $3,000 ordinary offset, carryforward planning, and replacement security selection. It is not tax advice; harvesting interacts with wash sales, the NIIT, state tax rules, and the taxpayer's overall gain position, and outcomes depend on facts that must be verified. Consult a qualified tax professional (CPA or tax attorney) before relying on any harvesting conclusion.

## Core Rules

### Quantify The Tax Benefit Against Gains And Ordinary Income

The value of a harvested loss depends on what it offsets. Losses first offset capital gains of the same type (short-term losses offset short-term gains; long-term losses offset long-term gains), then cross-offset to the other type, and finally up to $3,000 of net loss offsets ordinary income at the taxpayer's marginal ordinary rate. Any remaining loss carries forward indefinitely. The $3,000 ordinary offset is the most valuable use because ordinary rates (up to 37%) exceed long-term capital gains rates (0%, 15%, 20%).

Model the harvest against the taxpayer's actual gain position. A taxpayer with $20,000 of short-term gains harvesting $20,000 of short-term losses saves at their ordinary rate (e.g., 32% = $6,400). A taxpayer with no gains harvesting $3,000 of losses saves only at the ordinary rate on $3,000 (e.g., 32% = $960), with the rest carrying forward. Size the harvest to the gain position: harvesting beyond the gains plus $3,000 produces only a carryforward with uncertain future value. Do not harvest losses that will be consumed by transaction costs.

### Account For The Basis Reduction Cost Of Harvesting

Harvesting a loss reduces the cost basis of the replacement position by the amount of the loss realized. If you sell a position at a $10,000 loss and buy a replacement, the replacement's eventual sale will show a gain that is $10,000 larger than the original would have. The current tax saving is offset by a future tax cost, making the benefit primarily a deferral rather than a permanent saving. The exception is the $3,000 ordinary offset (permanent, because ordinary rates exceed capital gains rates) and death (basis step-up erases the deferred gain).

State the basis reduction explicitly in the analysis. A $10,000 long-term loss harvested to offset a $10,000 long-term gain saves 15% now ($1,500) but creates a future gain that costs 15% later ($1,500), netting near zero except for the time value of money. The strategy is most valuable when the loss offsets short-term gains or ordinary income (higher rate) or when the taxpayer expects to hold until death (basis step-up). Do not present harvesting as a pure gain; present it as a deferral with conditional permanence.

### Preserve Portfolio Exposure With A Suitable Replacement

To keep the portfolio's risk and factor exposure, the sold position must be replaced with a security that is not substantially identical (to avoid wash sales) but tracks the same market segment. Replacing an S&P 500 fund with a total market fund, or a large-cap growth fund with a different large-cap growth fund, preserves exposure while resetting the basis. The replacement must be different enough to avoid wash sale treatment but correlated enough to maintain the investment thesis.

Evaluate the replacement on tracking error, expense ratio, and factor exposure relative to the original. A poor replacement (e.g., swapping a broad index for a narrow sector ETF) introduces portfolio drift that can cost more than the tax saved. For broad market exposure, consider replacing with a fund covering a similar but not identical index (e.g., S&P 500 for total US market, or a large-cap fund for another large-cap fund). Document the replacement rationale and confirm it is not substantially identical under the wash sale rules.

### Apply The Wash Sale Rule To Every Replacement

A wash sale occurs if the taxpayer buys a substantially identical security within 30 days before or 30 days after the sale (a 61-day window). The loss is disallowed and added to the basis of the replacement. Substantially identical is strict for stocks and bonds (same company's stock) but more flexible for mutual funds and ETFs, where different funds tracking the same index are generally not substantially identical.

Check the 61-day window for every replacement, including automatic dividend reinvestments in the sold security, which are common wash sale triggers. Also check spouse accounts and IRA accounts: purchases by a spouse, or by the taxpayer's IRA, of a substantially identical security within the window create a wash sale. The IRA wash sale is especially damaging because the disallowed loss is permanently lost (it cannot be added to the IRA's basis). Verify no reinvestments, no spouse purchases, and no IRA purchases within the window.

### Time The Harvest Within The Tax Year And Settlement

Losses must be realized (the trade settled) within the tax year to count for that year. The last day to trade for a 2024 tax loss is typically the last business day before year-end, accounting for settlement (T+1 settlement as of May 2024 means a trade on December 31 settles the next business day, but the trade date controls for tax purposes). Confirm the trade date is in the tax year and that the 61-day wash sale window closes within a manageable timeframe.

Plan harvesting throughout the year, not just in December. Market declines in any month create harvesting opportunities, and spreading trades reduces concentration risk and wash sale exposure. Year-end harvesting is common but creates crowding (many investors sell the same losers) and time pressure that leads to errors. A year-round harvesting discipline captures more losses at better execution prices.

### Model The Interaction With NIIT And State Taxes

Harvested losses reduce capital gains, which can reduce net investment income subject to the 3.8% NIIT. A loss that offsets a gain reduces both the capital gains tax and the NIIT on that gain, effectively saving up to 23.8% (20% + 3.8%) for high earners. State taxes vary: some states do not conform to the federal $3,000 ordinary offset or treat capital gains differently, so the state benefit may differ from the federal.

Include the NIIT layer in the benefit calculation for taxpayers over the MAGI threshold. A $10,000 loss offsetting a long-term gain saves 15% federal plus 3.8% NIIT plus state tax, which can exceed 20% total. For taxpayers below the NIIT threshold, the benefit is the capital gains rate alone. Check state conformity: California, for example, follows the $3,000 ordinary offset, but other states may not. The total benefit is federal capital gains rate plus NIIT plus state, net of the basis reduction cost.

### Reassess Carryforwards And The Long-Term Strategy

Capital loss carryforwards persist indefinitely but are consumed by future gains. A large carryforward reduces the incentive to harvest in future years (because gains are already offset) but also provides a buffer that allows aggressive gain realization (e.g., Roth conversions funded by offsetting gains, or rebalancing without tax cost). Track the carryforward annually and factor it into the year's harvesting decision.

Do not harvest losses that merely add to an already large carryforward with no realistic prospect of use. A carryforward that will never be consumed (because the taxpayer has few future gains and dies with a step-up) has near-zero value. Conversely, a carryforward that enables future tax-efficient rebalancing or gain realization has real option value. The harvesting decision should consider the likely future use of the loss, not just the current-year deduction.

## Common Traps

### Harvesting Losses With No Offsettable Gains Or Ordinary Income

The trap is harvesting beyond the $3,000 ordinary offset when there are no gains, creating a carryforward with uncertain value. Size the harvest to the gain position plus $3,000 unless there is a clear future use.

### Ignoring The Basis Reduction That Offsets The Current Benefit

The trap is presenting the current tax saving without acknowledging the future larger gain. The benefit is a deferral except for the $3,000 ordinary offset and death step-up. State the basis reduction.

### Choosing A Replacement That Drifts The Portfolio

The trap is replacing a sold position with a security that does not preserve exposure, introducing tracking error or factor drift that costs more than the tax saved. Match the replacement's risk and factor profile.

### Triggering A Wash Sale Through Reinvestment Or Spouse/IRA Accounts

The trap is an automatic dividend reinvestment or a spouse/IRA purchase within the 61-day window that disallows the loss. IRA wash sales permanently lose the loss. Check all related accounts.

### Harvesting Tiny Losses Eaten By Costs

The trap is realizing a loss so small that transaction costs, bid-ask spread, and the replacement's expense ratio exceed the tax benefit. Set a minimum loss threshold before harvesting.

### Forgetting That Long-Term Losses Offset Long-Term Gains First

Long-term losses first offset long-term gains (taxed at 15-20%), not short-term gains (taxed at ordinary rates). The trap is assuming losses offset the highest-rate gains first. Follow the netting order in Section 1222.

### Overlooking The NIIT Layer In The Benefit Calculation

Losses offsetting gains also reduce NIIT for high earners. The trap is calculating the benefit at the capital gains rate alone, understating the saving by 3.8%. Add the NIIT layer when applicable.

### Treating The Carryforward As Always Valuable

A carryforward has value only if it will be consumed. The trap is accumulating large carryforwards with no realistic future use. Reassess the carryforward's likely consumption each year.

## Self-Check

- [ ] The harvest is sized against the taxpayer's actual gain position plus the $3,000 ordinary income offset, not harvested blindly.
- [ ] The basis reduction of the replacement is acknowledged, and the benefit is framed as a deferral (permanent only via the $3,000 ordinary offset or death step-up).
- [ ] The replacement security preserves the portfolio's risk and factor exposure and is documented as not substantially identical.
- [ ] The 61-day wash sale window is checked for the replacement, automatic reinvestments, spouse accounts, and IRA accounts (the last being a permanent loss).
- [ ] The trade is timed to settle within the tax year, with the trade date confirmed and year-round harvesting preferred over rushed December trades.
- [ ] The NIIT layer (up to 3.8%) is included in the benefit calculation for taxpayers over the MAGI threshold, and state tax conformity is checked.
- [ ] The capital gain and loss netting order (short-term vs long-term, same-type first) is followed per Section 1222.
- [ ] Existing carryforwards are tracked and factored into the decision, and harvesting beyond realistic future use is avoided.
- [ ] The minimum loss threshold accounts for transaction costs, bid-ask spread, and replacement expense ratio so the net benefit is positive.
- [ ] The conclusion notes harvesting interacts with wash sales, NIIT, state tax, and the overall gain position, this is not tax advice, and recommends consulting a qualified tax professional (CPA or tax attorney) before relying on any strategy.
