---
name: tax_loss_harvesting.md
description: Use when the agent is harvesting tax losses, selling losers to offset gains, managing wash sale rules, replacing sold positions without drifting from target allocation, or evaluating whether tax-loss harvesting adds value after costs and wash sale risk. Covers wash sale rule compliance, replacement security selection, maintaining market exposure, and the hidden cost of harvesting (forgiven losses and future higher basis).
---

# Tax-Loss Harvesting

Tax-loss harvesting is the practice of realizing investment losses to offset realized capital gains and, to a limited extent, ordinary income. Done well, it defers tax and can add modest after-tax return. Done carelessly, it triggers wash sale disallowances, drifts the portfolio away from its target allocation, incurs transaction costs that exceed the tax benefit, or merely converts a useful long-term loss into a deferred tax liability with a higher future basis. The benefit is real but smaller and more conditional than the popular framing suggests.

Agents tend to overstate the value of harvesting by counting the immediate tax deduction as pure gain, while ignoring three things: the wash sale rule that can disallow the loss entirely, the basis reset that reduces future losses (or increases future gains), and the tracking complexity across accounts. The judgment problem is deciding when a harvest genuinely improves after-tax outcome, how to comply with wash sale rules, and how to replace the exposure without changing the portfolio's risk.

This skill applies to harvesting losses, wash sale compliance, replacement security selection, and after-tax evaluation of harvesting. It is not tax or investment advice; rules vary by jurisdiction and change over time, consult a qualified tax professional, and harvesting decisions depend on the investor's specific tax situation.

## Core Rules

### Confirm The Loss Is Real And Worth Harvesting

A harvest only helps if there is an actual unrealized loss and a tax liability to offset. Harvesting a loss to offset zero gains wastes the loss (the limited ordinary-income offset may be the only benefit) and incurs costs. Confirm the investor has realized gains, expected near-term gains, or can use the annual ordinary-income offset, and that the loss is large enough to matter after transaction costs.

Also confirm the loss is not a temporary dip the investor expects to reverse; harvesting locks in the tax benefit but requires selling and repurchasing, with the basis consequences below. Small losses in tax-advantaged accounts provide no benefit at all, since those accounts have no taxable gains.

### Respect The Wash Sale Rule Across All Accounts

In US tax law, a wash sale occurs when a substantially identical security is bought within 30 days before or after the sale at a loss; the loss is then disallowed and added to the basis of the replacement. The 30-day window applies both before and after the sale, and critically, the IRS applies it across all the taxpayer's accounts, including IRAs, and increasingly brokers aggregate across spouses in practice. A wash sale in an IRA permanently disallows the loss with no basis benefit.

Before harvesting, ensure no substantially identical security will be purchased in any account within the 61-day window. This includes automatic dividend reinvestment, systematic purchases, and rebalancing. Pause reinvestment on the position being harvested. The cross-account reach of the rule is the most common way harvests are accidentally invalidated.

### Choose A Replacement That Preserves Exposure Without Being Substantially Identical

To maintain market exposure during the wash sale window, replace the sold security with one that is correlated but not substantially identical. For an S&P 500 fund, a total market fund or a different large-cap fund is often acceptable; for a sector fund, a related but distinct sector or broad market fund. The replacement must differ enough to avoid the wash sale rule while tracking closely enough to preserve the intended exposure.

Document why the replacement is not substantially identical (different index, different provider, different structure). Avoid replacements that are economically identical but nominally different if the IRS could view them as substantially identical (e.g., different share classes of the same fund, or an ETF and a mutual fund tracking the same index with the same holdings). When in doubt, choose a more clearly distinct replacement and accept minor tracking difference.

### Account For The Basis Reset And Its Future Cost

Harvesting replaces a low-basis position with a higher-basis one. This is the core tradeoff: you get a tax deduction now but give up future losses (or create future gains) on the higher-basis replacement. If the security later declines further, the higher basis means smaller future harvestable losses; if it rises, the higher basis means smaller future gains when eventually sold. The harvest defers tax rather than eliminating it.

Model the net benefit as the time value of the deferral minus the expected future cost of the basis reset. In low-return or declining scenarios, the basis reset can erode much of the benefit. Harvesting is most valuable when the investor is in a high tax bracket now and expects to be in a lower bracket, or to hold until stepped-up basis at death, in retirement, or otherwise.

### Maintain Target Allocation And Risk Exposure

Selling a loser to harvest and then sitting in cash, or buying a poorly correlated replacement, drifts the portfolio. The replacement should preserve the asset class, factor, geography, and duration exposure of the original so the portfolio's risk profile is unchanged through the wash sale window. Drift during the window can be small but compounds if harvesting is frequent.

Define the replacement to match the original's role: same equity region and style, same fixed-income duration and credit, same factor tilt. Re-evaluate at the end of the 30-day window whether to switch back to the original (another transaction with its own cost) or hold the replacement permanently. Frequent switching back and forth incurs costs and complexity.

### Weigh Transaction Costs Against The Tax Benefit

Every harvest involves a sale and a purchase, each with bid-ask spread, commission, and potential market impact. For small losses or illiquid securities, the transaction cost can exceed the tax benefit. Compute the net benefit after costs before executing.

Also consider the operational cost: tracking harvests, replacement positions, wash sale windows, and basis adjustments across accounts and years is non-trivial and error-prone. For modest losses, the complexity may not be worth it. Prioritize harvesting the largest losses in the most liquid positions first.

### Consider The Investor's Full Tax Picture And Time Horizon

Harvesting value depends on the investor's current and future tax rates, the type of gains available to offset (short-term vs long-term matching matters), whether losses can be carried forward, and the holding horizon. An investor who will sell soon at long-term rates and step up basis benefits more than one in a low bracket with no gains to offset. An investor near the top of a bracket may find harvesting pushes other income into a lower bracket.

Integrate harvesting with the overall tax plan, not as an isolated tactic. Harvesting that generates a benefit now but creates a larger future liability or complicates estate planning may be net negative. Always evaluate the after-tax, full-horizon outcome.

## Common Traps

### Triggering A Wash Sale Via Reinvestment Or Cross-Account Purchases

Dividend reinvestment or systematic buys in any account within 30 days disallow the loss. The trap is harvesting without pausing reinvestment or checking all accounts. Audit all purchase activity in the window.

### Using A Substantially Identical Replacement

Different share classes or funds with identical holdings are substantially identical. The trap is thinking a nominally different ticker avoids the rule. Choose clearly distinct replacements and document why.

### Counting The Tax Saving As Pure Gain

The harvest defers tax, it does not eliminate it, because the basis resets higher. The trap is reporting the deduction as free money. Model the basis reset cost.

### Harvesting In Tax-Advantaged Accounts

Losses in IRAs or 401(k)s provide no tax benefit. The trap is harvesting there as if it helped. Harvesting only matters in taxable accounts.

### Drifting From Target Allocation

Selling to cash or buying an uncorrelated replacement changes risk. The trap is ignoring allocation during the window. Match the original's role.

### Harvesting Tiny Losses Where Costs Exceed Benefit

Small losses in costly-to-trade names can be net negative after spreads. The trap is harvesting everything by default. Prioritize large, liquid losses.

### Ignoring Short-Term Versus Long-Term Character Matching

Short-term losses offset short-term gains first; long-term offset long-term. The trap is assuming any loss offsets any gain optimally. Consider character when sequencing harvests.

## Self-Check

- [ ] The loss is real, in a taxable account, and large enough to exceed transaction costs and offset an actual or expected tax liability.
- [ ] No substantially identical security will be purchased in any account (including IRAs and a spouse's accounts) within the 30-day window; reinvestment is paused.
- [ ] The replacement preserves the original's asset class, factor, duration, and geography exposure while being clearly not substantially identical.
- [ ] The basis reset and its future cost (reduced future losses or increased future gains) are modeled; the benefit is treated as tax deferral, not elimination.
- [ ] The target allocation and risk profile are maintained through the wash sale window.
- [ ] Transaction costs (spreads, commissions, impact) are weighed against the tax benefit; small or illiquid losses are deprioritized.
- [ ] The harvest is integrated with the investor's full tax picture: current and future brackets, gain character matching, carryforward rules, and holding horizon.
- [ ] Harvesting is not recommended in tax-advantaged accounts where it provides no benefit.
- [ ] The short-term versus long-term character of losses and gains is considered in sequencing.
- [ ] The conclusion notes rules vary by jurisdiction and change over time, recommends consulting a qualified tax professional, and is not personalized tax or investment advice.
