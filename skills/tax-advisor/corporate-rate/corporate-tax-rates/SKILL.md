---
name: corporate_tax_rates.md
description: Use when the agent is applying the federal corporate income tax rate, modeling C corporation tax cost, comparing corporate versus individual rates, evaluating personal service corporation treatment, computing effective rate after deductions, or layering state corporate taxes on top of the federal rate.
---

# Corporate Tax Rates

The U.S. federal corporate income tax rate is not a graduated schedule. Since the Tax Cuts and Jobs Act (TCJA) of 2017, most C corporations pay a flat 21% on taxable income, with no brackets and no phaseout. This apparent simplicity is itself a trap: agents treat 21% as the whole story and forget that the effective rate is shaped by deductions, the corporate alternative minimum tax, state corporate taxes, and the second layer of tax when profits are distributed as dividends. A rate analysis that stops at "21%" understates the real cost of operating as a C corporation and produces poor entity-choice and distribution decisions.

Agents frequently miss that personal service corporations (PSCs) are locked into a flat 35% with no benefit from the lower brackets that existed before TCJA, that the 21% rate applies before the accumulated earnings tax and the CAMT, and that state corporate taxes can add 3% to 9% or more on top of the federal rate depending on apportionment. The deeper blind spot is treating the corporate rate in isolation from the double-tax structure: the corporation pays 21%, and the shareholder pays individual tax on dividends, so the combined burden on distributed earnings often exceeds the top individual rate. Quoting 21% without the second layer makes a C corporation look cheaper than it is.

This skill covers the structure and application of U.S. federal corporate income tax rates, the PSC exception, effective-rate computation, and state layering. It is not tax advice; the TCJA provisions are scheduled to sunset after 2025 pending legislative action, state rates vary, and a qualified tax professional (CPA or tax attorney) must be consulted for actual situations.

## Core Rules

### Apply The Flat 21% Federal Rate Without Brackets

The TCJA replaced the prior graduated corporate rate structure (15% to 35% with a phaseout) with a single flat 21% rate on C corporation taxable income, effective for tax years beginning after 2017. There are no income brackets, no surtax bubbles, and no personal exemption phaseout. A corporation with $50,000 of taxable income and a corporation with $50 million pay the same 21% federal rate. This flatness simplifies computation but eliminates the rate-arbitrage planning that existed under the old graduated system.

The 21% rate is the statutory rate on taxable income as defined, not the effective rate the corporation actually bears. Taxable income is reduced by deductions, credits, and NOLs before the 21% is applied. A corporation with $1,000,000 of gross income and $600,000 of deductions has $400,000 of taxable income taxed at 21%, producing $84,000 of federal tax, an effective rate of 8.4% on gross income. Always distinguish the statutory rate (21%) from the effective rate (tax divided by some income measure) when presenting analysis, and state which income base the effective rate is computed against.

### Recognize The Personal Service Corporation 35% Flat Rate

Personal service corporations (PSCs) are taxed at a flat 35% federal rate with no bracket benefit. A PSC is a corporation whose principal activity is the performance of services in a qualifying field (health, law, engineering, architecture, accounting, actuarial science, performing arts, or consulting) and substantially all of whose stock is held by employees performing the services. The purpose of the 35% flat rate was historically to prevent PSCs from using the low graduated brackets to defer tax by retaining earnings; under TCJA the flat 21% would otherwise apply, but the PSC rule keeps the 35% in place as an anti-deferral measure.

This is a significant trap for professionals who incorporate expecting the 21% rate. A law firm, medical practice, accounting firm, or consulting corporation structured as a C corporation PSC pays 35% federal, not 21%. Combined with the double tax on dividends, the effective burden on distributed PSC earnings can approach or exceed 50% at the federal level alone before state tax. Before recommending C corporation status for a service business, confirm whether it is a PSC and model the 35% rate, not the general 21%.

### Layer State Corporate Taxes On Top Of The Federal Rate

State corporate income taxes are a separate layer that can materially increase the total rate. State top corporate rates range from 0% (states with no corporate income tax, such as Nevada, Ohio's gross receipts tax instead, Texas franchise tax, South Dakota, Wyoming) to approximately 9% to 11% (states such as California, New Jersey, Minnesota, Illinois). Some states impose both a corporate income tax and a gross receipts or franchise tax. The state rate is applied to state-taxable income, which is apportioned to the state based on a formula (typically property, payroll, and sales factors, with many states now using single-sales-factor apportionment).

The federal deduction for state income taxes paid by a corporation partially offsets the state layer, but the interaction is not a simple subtraction. A corporation in a 9% state bracket does not pay 30% combined (21% + 9%); because the state tax is deductible federally, the combined effective rate is lower than the sum. For a corporation with $100,000 of pre-tax income, $9,000 of state tax reduces federal taxable income to $91,000, producing $19,110 of federal tax, for a combined $28,110 or 28.1%. Model the deductibility interaction rather than adding rates. Note that state apportionment means the effective state rate depends on where sales, property, and payroll are located, not just the headquarters state.

### Compute The Effective Rate After Deductions And Credits

The effective tax rate is the actual tax divided by an income measure, and it is usually lower than the 21% statutory rate because of deductions, credits, and loss carryforwards. A corporation claiming the research and development credit, domestic production activities, accelerated depreciation, or the dividends-received deduction may have an effective federal rate well below 21%. Conversely, a corporation denied deductions (entertainment, fines, certain lobbying) or subject to the CAMT may have an effective rate above 21% on book income.

When presenting effective-rate analysis, always state the denominator: effective rate on taxable income, on book income, on gross revenue, or on EBITDA will all differ and serve different purposes. A 21% statutory rate producing a 14% effective rate on book income because of bonus depreciation tells a different story than 21% producing 26% on book income because of CAMT. Do not let the 21% headline obscure the real burden the corporation faces.

### Model The Double Tax On Distributed Earnings

The C corporation is subject to two layers of tax: the corporate-level 21% and the shareholder-level tax on dividends. Qualified dividends are taxed at the long-term capital gains rates (0%, 15%, or 20%) plus the 3.8% net investment income tax for higher earners, bringing the shareholder layer to as much as 23.8%. The combined federal burden on a dollar of corporate earnings distributed as a qualified dividend to a top-bracket individual is 21% at the corporate level plus 23.8% on the after-corporate-tax remainder, which equals approximately 39.8% combined.

This combined rate exceeds the top individual ordinary rate of 37%, which is why C corporation status is often tax-inefficient for businesses that distribute most earnings to owners. The double tax is avoided only when earnings are retained in the corporation (which can trigger the accumulated earnings tax), or when the corporation is an S corporation or other pass-through. Entity-choice analysis must model the combined corporate-plus-shareholder burden, not the 21% alone. A business that retains earnings for reinvestment may benefit from the 21% rate, but a business that distributes profits usually bears a higher combined rate than a pass-through.

### Factor In The Accumulated Earnings And Personal Holding Company Taxes

Beyond the 21% base rate, two penalty taxes can increase the effective corporate burden. The accumulated earnings tax (Section 531) imposes a 20% penalty tax on earnings retained beyond the reasonable needs of the business, intended to prevent C corporations from deferring shareholder-level tax indefinitely. The personal holding company tax (Section 541) imposes a 20% penalty tax on undistributed personal holding company income of closely-held corporations whose income is primarily passive (dividends, interest, royalties, rents).

These penalty taxes are not automatic; they are asserted by the IRS when a corporation appears to be accumulating earnings without a business reason. However, they materially change the rate analysis for closely-held C corporations. A corporation accumulating cash to avoid the dividend tax may face 21% plus a 20% accumulated earnings tax on the excess, effectively 41% on retained earnings beyond reasonable needs. Document the business purpose for retained earnings (expansion, working capital, debt retirement, product development) to defend against the penalty tax.

### Compare Corporate Rate To Individual Rates For Entity Choice

The 21% corporate rate is lower than the top individual rate of 37%, which creates the appearance that C corporation status is rate-favorable. But the comparison is incomplete without the second layer. For a business that retains all earnings, the 21% rate is genuinely lower than individual rates, making C corporation status attractive for reinvestment-heavy businesses. For a business that distributes earnings, the combined 39.8% burden exceeds the 37% top individual rate, making pass-through status more efficient.

The entity-choice rate analysis must also account for qualified business income (QBI) deductions for pass-throughs (up to 20% of qualified business income, subject to limitations), self-employment tax savings from S corporation status, and the ability of C corporations to deduct certain fringe benefits that pass-throughs cannot. There is no universal answer; the rate comparison must be modeled on the specific facts of earnings retention, distribution, owner compensation, and fringe benefits.

## Common Traps

### Quoting 21% As The Total Tax Cost

The 21% federal rate is only the first layer. Quoting 21% without the state layer, without the CAMT for applicable corporations, and without the shareholder dividend tax understates the real cost of C corporation earnings and can lead to poor entity-choice decisions.

### Assuming The Graduated Brackets Still Exist

Before TCJA, C corporations had graduated rates from 15% to 35%. Agents trained on the old system may apply graduated brackets that no longer exist. The rate is flat 21% for all C corporation taxable income levels, with no brackets.

### Forgetting Personal Service Corporations Pay 35%

A C corporation that qualifies as a PSC pays 35%, not 21%. Professionals incorporating a service business may expect the 21% rate and face a much higher bill. Always test PSC status for health, law, accounting, engineering, architecture, actuarial, performing arts, and consulting corporations.

### Adding State Rate Directly To The Federal Rate

A 21% federal rate plus a 9% state rate is not 30% combined, because state taxes are deductible federally. The combined effective rate is lower than the sum. Model the deductibility interaction rather than simple addition.

### Ignoring The Double Tax On Dividends

C corporation earnings are taxed twice: 21% at the corporate level and up to 23.8% at the shareholder level on qualified dividends. The combined federal burden on distributed earnings approaches 40%, exceeding the top individual rate. Entity-choice analysis that ignores the second layer makes C corporations look cheaper than they are.

### Treating The Effective Rate As Equal To The Statutory Rate

The 21% statutory rate is applied to taxable income after deductions. The effective rate on book income or gross revenue can be much lower (due to deductions and credits) or higher (due to CAMT and disallowed deductions). Always distinguish which rate is being discussed.

### Overlooking The Accumulated Earnings Tax

Retaining earnings to defer the dividend tax can trigger the 20% accumulated earnings penalty tax. The rate advantage of the 21% rate on retained earnings is not free; it comes with a penalty-tax risk that must be managed with documented business purposes.

## Self-Check

- [ ] Has the flat 21% federal rate been applied to C corporation taxable income, with the pre-TCJA graduated brackets confirmed as repealed?
- [ ] Has personal service corporation status been tested for service businesses, and has the 35% PSC rate been applied where applicable rather than 21%?
- [ ] Has the state corporate income tax layer been added, with the federal deductibility interaction modeled (combined rate lower than the sum of federal and state)?
- [ ] Has the effective rate been computed against a stated income base (taxable income, book income, gross revenue) and distinguished from the 21% statutory rate?
- [ ] Has the double tax on distributed earnings been modeled, including the corporate 21% and the shareholder-level qualified dividend rate plus NIIT?
- [ ] Has the accumulated earnings tax and personal holding company tax risk been considered for closely-held C corporations retaining earnings?
- [ ] Has the corporate rate been compared to individual rates and pass-through QBI treatment for entity-choice analysis, accounting for earnings retention versus distribution?
- [ ] Has the TCJA sunset after 2025 been flagged as a risk that the 21% rate may change pending legislation, with the analysis dated to the specific tax year?
- [ ] Has the agent noted that this is general U.S. federal rate information, not tax advice, and recommended consulting a qualified tax professional (CPA or tax attorney) for the specific situation?
