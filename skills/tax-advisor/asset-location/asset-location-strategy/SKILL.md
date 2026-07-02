---
name: asset_location_strategy.md
description: Use when the agent is designing an asset location strategy across taxable, tax-deferred, and Roth accounts, deciding which holdings go in which account bucket, sequencing fills across account types, evaluating location tradeoffs, or reviewing a portfolio's tax efficiency by account wrapper.
---

# Asset Location Strategy

Asset location is the practice of deciding not just what to own, but where to own it. A household with multiple account types can hold the same overall asset allocation but distribute the holdings differently across wrappers to minimize the lifetime tax drag. The core idea is to place assets that generate heavy, high-rate taxable income into tax-advantaged accounts, and to place assets that are naturally tax-efficient into taxable accounts where they benefit from deferral and preferential long-term capital gains rates. Done well, asset location can add a meaningful fraction of after-tax return over decades with no change in risk.

The judgment problem is that agents often treat asset location as a one-line rule ("bonds in tax-deferred, stocks in Roth") without quantifying the actual tax drag of each holding, without respecting the constraints of each account, and without sequencing the placement when account space is limited. They may overfill a tax-deferred account with bonds and leave no room for other tax-inefficient assets, ignore that international equities generate a foreign tax credit only usable in a taxable account, or fail to account for the fact that the highest-expected-return assets benefit most from the strongest shield (Roth). The harm is silent: a suboptimal location costs a small percentage every year that compounds into a large gap over a working lifetime.

This skill applies to building or reviewing an asset location plan across account buckets, sequencing fills, evaluating tradeoffs between competing tax-inefficient assets, and assessing a portfolio's wrapper-level tax efficiency. It uses US federal treatment as a baseline framework. Account rules, rates, and contribution limits change over time and vary by jurisdiction. This is not tax advice; consult a qualified tax professional or financial advisor before implementing a location strategy.

## Core Rules

### Prioritize By Tax Drag, Not By Asset Label

The foundation of asset location is measuring the annual tax drag each holding generates if placed in a taxable account. Tax drag is the tax cost expressed as a percentage of the holding's value, paid every year out of returns. A taxable bond fund yielding 4.5% throws off ordinary interest taxed at the marginal rate; at a 32% bracket that is roughly 1.4% annual drag. A REIT fund distributing non-qualified dividends at ordinary rates can drag similarly. A broad equity index fund with a 1.5% qualified dividend yield taxed at 15% costs only about 0.2% annually. The asset with the highest drag gets the most valuable tax-advantaged space.

Rank holdings by their estimated taxable-account drag before placing them. Do not rely on category names alone; two bond funds can have very different yields, and an active equity fund with 80% turnover can generate far more realized gain than a passive index fund. Estimate the drag from yield, turnover, and distribution type, then allocate limited tax-advantaged room to the highest-drag assets first.

### Place Highest-Expected-Return Assets In The Strongest Shield

Not all tax-advantaged accounts shield equally. Roth accounts (tax-exempt) shield growth entirely; tax-deferred accounts shield growth but tax withdrawals as ordinary income. Because the Roth shield is absolute, the assets expected to grow the most benefit most from sitting in Roth, since every dollar of untaxed growth is a dollar saved at the full marginal rate. Conversely, the tax-deferred account's shield is partial, so it is well-suited to tax-inefficient assets whose annual drag would otherwise be taxed at ordinary rates.

Apply this as a sequencing principle. After capturing any employer match, a common framework is to fill tax-advantaged space with the most tax-inefficient assets (bonds, REITs, high-turnover funds) first, and to bias the Roth bucket toward the highest-expected-return, most tax-efficient assets (equities) when possible, because their growth is permanently shielded. This is a general framework, not a rigid law; the exact split depends on the household's account sizes, allocation, and withdrawal expectations.

### Respect The Foreign Tax Credit For International Equities

International equity funds pay foreign taxes on their holdings, and US taxpayers can claim a foreign tax credit for those taxes, but only in a taxable account. If international equities are placed entirely in a tax-advantaged account, the foreign tax credit is permanently lost, which can be worth roughly 0.1-0.3% annually depending on the fund and the foreign markets involved. This creates a specific exception to the general "equities in Roth" preference.

Keep at least some international equity exposure in the taxable account to capture the foreign tax credit. The tradeoff is that international equities are slightly less tax-efficient than domestic index funds due to higher dividends, but the credit often offsets more than the added drag. Identify which holdings generate foreign taxes and ensure they are not entirely trapped in tax-advantaged wrappers.

### Size Each Account Bucket Against The Target Allocation

Asset location only works if there is enough room in each wrapper to absorb the assets assigned to it. A household that is 80% equities and 20% bonds but has 90% of its assets in tax-advantaged accounts has abundant room; the same household with only 30% in tax-advantaged accounts cannot shelter all its bonds and must decide which tax-inefficient assets get priority. The location plan must be feasible given the actual account sizes.

Build the plan from the actual account balances. Calculate how much tax-advantaged space exists, how much of the allocation is tax-inefficient, and whether the inefficient assets fit. When they do not fit, prioritize the highest-drag assets for the limited shelter and accept that some inefficient assets will sit in taxable. A location plan that assumes more shelter than exists is not executable.

### Account For Different Withdrawal Tax Rates Across Buckets

The after-tax value of a holding depends on the wrapper's withdrawal treatment. Assets in a tax-deferred account will be taxed as ordinary income on withdrawal, regardless of whether the underlying growth was capital gain or dividend. Assets in a Roth account are withdrawn tax-free. Assets in a taxable account are taxed at long-term capital gains rates on sale (assuming the holding period is met) and only on the gain, not the principal. This means the same pre-tax return produces different after-tax outcomes by location.

Model location decisions on an after-tax basis. A holding expected to be sold for a long-term gain in a taxable account may actually be taxed more favorably than the same holding in a tax-deferred account that converts the gain into ordinary income on withdrawal. This is why asset location is not simply "shelter everything possible"; for tax-efficient equities held long-term, the taxable account's preferential rate can rival or beat the tax-deferred account's ordinary-rate withdrawal, especially when the deferral benefit of not realizing gains is included.

### Sequence The Fill Order When Space Is Limited

When tax-advantaged room is scarce relative to tax-inefficient assets, the order of placement matters. A practical fill sequence is: first capture any employer match (it is an immediate return), then prioritize the highest-drag assets into the available tax-advantaged space, then place remaining tax-efficient assets in taxable. Within tax-advantaged space, bias the Roth bucket toward the highest-expected-return assets and the tax-deferred bucket toward the tax-inefficient income generators.

Document the sequence so the plan is repeatable and reviewable. The sequence should reflect the household's actual marginal rate, expected retirement rate, account sizes, and allocation. Avoid a generic template that ignores these inputs; a sequence correct for a high-bracket household with large taxable assets may be wrong for a moderate-bracket household whose assets are mostly in a 401(k).

### Revisit Location When Allocation Or Account Mix Changes

Asset location is not a set-and-forget decision. As the household's allocation shifts (for example, becoming more bond-heavy near retirement), as new accounts are opened, as balances grow at different rates, or as tax law changes, the optimal placement shifts. A location plan built at age 35 with 90% equities may be badly mismatched at age 60 with a 50/50 allocation and a much larger taxable account.

Schedule periodic review of the location plan, at least when the target allocation changes materially or when a major account event occurs (rollover, inheritance, new workplace plan). The cost of a stale location plan is a persistent, silent drag that the household never sees directly but that erodes after-tax wealth over time.

## Common Traps

### Applying A One-Line Rule Without Quantifying Tax Drag

"Bonds in tax-deferred, stocks in Roth" is a rough heuristic, not an analysis. The trap is skipping the drag calculation. Estimate each holding's annual taxable drag before placing it.

### Trapping International Equities Entirely In Tax-Advantaged Accounts

The foreign tax credit is usable only in a taxable account. The trap is sheltering all international exposure and permanently losing the credit. Keep some international in taxable.

### Assuming More Shelter Is Always Better

For tax-efficient equities held long-term, the taxable account's preferential long-term capital gains rate and deferral can rival a tax-deferred account's ordinary-rate withdrawal. The trap is treating all shelter as pure benefit.

### Ignoring Account Size Constraints

A location plan that assigns more assets to tax-advantaged space than exists cannot be executed. The trap is designing in the abstract. Size the plan to actual balances.

### Forgetting That Tax-Deferred Withdrawals Are Ordinary Income

Growth inside a tax-deferred account does not keep its character as capital gain; it is taxed as ordinary income on withdrawal. The trap is assuming preferential rates apply. Model withdrawals at ordinary rates.

### Never Reviewing The Location Plan

Allocation and account mix drift over decades. The trap is treating location as a one-time setup. Revisit when allocation or accounts change materially.

### Overlooking Cash Drag In Tax-Advantaged Accounts

Holding idle cash in a tax-advantaged account wastes the shelter on a zero-return asset. The trap is leaving settlement cash uninvested. Deploy tax-inefficient assets into sheltered space.

## Self-Check

- [ ] Each holding's estimated annual taxable-account tax drag is quantified (from yield, turnover, distribution type) before placement, not assumed from a category label.
- [ ] The highest-drag assets (bonds, REITs, high-turnover funds) are prioritized for limited tax-advantaged space.
- [ ] Highest-expected-return, tax-efficient assets are biased toward the Roth bucket to maximize the absolute shield on growth.
- [ ] International equity exposure is retained in the taxable account to preserve the foreign tax credit rather than trapping it entirely in tax-advantaged wrappers.
- [ ] The plan is sized to actual account balances and is executable, not designed against more shelter than exists.
- [ ] Location tradeoffs are modeled on an after-tax basis, recognizing that tax-deferred withdrawals are taxed as ordinary income.
- [ ] A documented fill sequence reflects the household's actual marginal rate, expected retirement rate, allocation, and account sizes.
- [ ] The location plan is flagged for periodic review when the target allocation or account mix changes materially.
- [ ] The conclusion notes this is not tax advice, account rules and rates change, and a qualified tax professional or financial advisor must be consulted before implementing.
