---
name: dividends_received_deduction.md
description: Use when the agent is computing the dividends-received deduction, determining the 70 80 or 100 percent ownership tier, measuring ownership by voting power or profit interests, verifying the 45-day holding period, applying debt-financed portfolio stock reduction, or analyzing how the DRD prevents triple taxation of corporate dividends.
---

# Dividends Received Deduction

The dividends-received deduction (DRD) under IRC Section 243 is a corporate-only provision that prevents the same dollar of corporate profit from being taxed multiple times as it moves through chains of corporate ownership. When Corporation A receives a dividend from Corporation B, that dividend represents profit already taxed at B's corporate level; without relief, taxing it again at A's level (and again when A distributes to its shareholders) would produce triple taxation. The DRD allows Corporation A to deduct a percentage of the dividend received based on its ownership stake in B. The judgment problem is that the percentage is not automatic; it depends on precisely measured ownership, a holding period, and adjustments for debt-financed stock.

Agents frequently miss that the DRD is available only to C corporations (not individuals, partnerships, or S corporations at the entity level), that the ownership percentage that determines the tier is measured by both voting power and value, not just one, and that the 45-day holding period requirement can disqualify an otherwise qualifying dividend. The deeper blind spot is the debt-financed portfolio stock reduction: when stock is purchased with borrowed funds, the DRD is reduced proportionally, which can eliminate most of the benefit for leveraged corporate investment portfolios. Applying the wrong ownership tier, missing the holding period, or ignoring the debt-financed reduction understates tax and produces an incorrect return.

This skill covers the DRD percentage tiers, ownership measurement, the holding period requirement, debt-financed reduction, and the anti-triple-taxation rationale. It is not tax advice; ownership attribution, affiliated group rules, and the debt-financed computation have detailed regulations, and a qualified tax professional (CPA or tax attorney) must be consulted for actual situations.

## Core Rules

### Apply The Correct DRD Percentage By Ownership Tier

The DRD percentage depends on the recipient corporation's ownership of the payer corporation's stock, measured by voting power and value. There are three tiers. A corporation owning less than 20% of the payer receives a 70% DRD (deducting 70% of the dividend, taxing 30%). A corporation owning 20% or more but less than 80% receives an 80% DRD (deducting 80%, taxing 20%). A corporation that is a member of the same affiliated group (generally owning 80% or more of the payer's voting power and value, with the affiliation relationship) receives a 100% DRD, eliminating tax entirely on the intercompany dividend.

The critical first step is determining the ownership percentage, because a small change crosses a tier boundary and changes the deduction by 10 or 30 percentage points. A corporation owning 19.9% is in the 70% tier; one owning 20.0% is in the 80% tier. Ownership must be measured precisely, and attribution rules (constructive ownership through related parties) can push a corporation over a threshold. Do not estimate ownership; compute it from the capitalization table, including all classes of stock with voting or value rights.

### Measure Ownership By Voting Power And Value, Not Just One Dimension

Ownership for DRD tier purposes is measured by the percentage of the payer corporation's total combined voting power of all classes of voting stock AND the percentage of the total value of all classes of stock (other than nonvoting preferred that is limited and preferred as to dividends). Both tests must be satisfied for a given tier. A corporation that holds 25% of the voting power but only 15% of the value (or vice versa) does not cleanly fit a tier based on one measure; the regulations require analyzing both.

The practical consequence is that corporations with complex capital structures (multiple classes of stock, preferred shares, convertible instruments) cannot determine the DRD tier from a single ownership number. A corporation holding common stock representing 30% of votes but with preferred holdings that dilute value to 18% may not qualify for the 80% tier. Confirm the ownership percentage under both the voting-power and value tests, applying attribution rules, before selecting the DRD tier. Where the two measures diverge significantly, the analysis requires careful application of the Section 243(c) definitions and may require professional review.

### Satisfy The 45-Day Holding Period Requirement

To claim the DRD, the recipient corporation must have held the stock on which the dividend is paid for more than 45 days during the 91-day period beginning 45 days before the ex-dividend date and ending 45 days after the ex-dividend date. This is the holding period requirement under Section 246(c), and it prevents corporations from buying stock just before a dividend, collecting the DRD-eligible dividend, and selling immediately after. The 45 days must be within the 91-day window; holding the stock for 45 days outside the window does not qualify.

The holding period is reduced by days for which the taxpayer's risk of loss is substantially diminished through hedging, options, or similar positions. A corporation that holds the stock for 50 days but has an offsetting short sale or put option that eliminates risk of loss for 20 of those days has an effective holding period of 30 days, which fails the 45-day test. Verify not only the raw holding period but also whether any hedging or risk-reduction positions reduce the qualified days. A dividend received on stock held less than 45 qualified days is fully taxable with no DRD.

### Apply The Debt-Financed Portfolio Stock Reduction

When a corporation purchases stock with borrowed funds (debt-financed portfolio stock), the DRD is reduced under Section 246A. The reduction is proportional to the indebtedness attributable to the stock. If a corporation borrows $60 of the $100 cost of portfolio stock (less than 20% owned), the indebtedness percentage is 60%, and the 70% DRD is reduced by 60% of itself: the allowed DRD becomes 70% minus (60% times 70%) = 70% minus 42% = 28%. The dividend is taxed at 72% instead of 30%.

This reduction can nearly eliminate the DRD for heavily leveraged corporate investment portfolios. A corporation that borrows to acquire a portfolio of dividend-paying stock may find that the debt-financed reduction brings the effective DRD close to zero, making the dividends nearly fully taxable. The computation requires identifying which indebtedness is attributable to the portfolio stock (directly traceable debt and, where tracing is not possible, an average indebtedness allocation). Always test whether the stock is debt-financed before claiming the full DRD, particularly for corporate investment portfolios funded with borrowing.

### Recognize The DRD Is Corporate-Only And Prevents Triple Taxation

The DRD is available only to C corporations. Individuals, partnerships, S corporations, and trusts do not claim the DRD at the recipient level (an S corporation that receives a dividend passes the dividend through to shareholders, who may not claim the DRD). The policy rationale is that corporate profits are taxed at the entity level; when distributed as dividends to another corporation, taxing them again would compound the burden, and when that second corporation distributes to its shareholders, the same dollar would be taxed a third time. The DRD removes the second layer to prevent triple taxation.

Understanding the rationale helps apply the rule correctly. The 100% DRD for affiliated group members reflects that the parent and subsidiary are effectively a single economic unit for tax purposes. The 80% tier reflects significant ownership influence. The 70% tier reflects portfolio investment where the corporations are separate economic actors but the dividend still represents already-taxed profit. When analyzing whether a dividend qualifies, ask whether the payment represents already-taxed corporate profit flowing between corporate entities; if so, the DRD framework applies, subject to the ownership, holding period, and debt-financed rules.

### Verify Affiliated Group Status For The 100% Deduction

The 100% DRD requires that the recipient and payer be members of the same affiliated group, which generally means the parent owns at least 80% of the subsidiary's voting power and 80% of the total value, and the affiliation tests of Section 1504 are met. The 100% DRD eliminates tax entirely on intercompany dividends within the group. However, affiliated group status is a technical determination that involves not just the 80% ownership but also the exclusion of certain corporations (foreign corporations, certain insurance companies, etc.) and the consistency of the tax year.

A common error is assuming that 80% ownership automatically creates an affiliated group for DRD purposes. The ownership must be of voting power and value, and the group must satisfy the definitional requirements of Section 1504. Where the ownership is close to 80% or the capital structure is complex, confirm affiliated group status before claiming the 100% DRD. An incorrect 100% DRD claim that is actually an 80% or 70% tier dividend understates tax by 20 or 30 percentage points of the dividend amount.

### Document Ownership, Holding Period, And Debt-Financing Facts

Because the DRD depends on ownership percentage, holding period, and debt-financing, the analysis must document each element. Record the ownership percentage under both voting-power and value tests (with attribution applied), the number of qualified holding days within the 91-day window (with any hedging reductions), and the indebtedness attributable to the stock (if any). A DRD claim that does not document these facts is not defensible on examination.

The DRD is reported on Form 1120 (and related corporate returns), and the deduction ties to the dividend income reported. Retain the brokerage statements or payer records showing the dividend, the trade confirmations showing acquisition and holding dates, the capitalization table supporting ownership percentage, and any debt-tracing analysis supporting the debt-financed reduction. The computation should be reproducible so that a reviewer or examiner can verify each tier determination.

## Common Traps

### Applying The Wrong Ownership Tier

The 70%, 80%, and 100% tiers turn on precise ownership percentages (under 20%, 20% to 80%, 80% plus affiliated group). Estimating ownership or ignoring attribution rules can place the dividend in the wrong tier, understating or overstating the deduction by 10 to 30 percentage points.

### Measuring Ownership By Only Voting Power Or Only Value

Ownership for DRD tier purposes requires both voting power and value tests. A corporation with 30% of votes but 15% of value does not cleanly qualify for the 80% tier. Measure both dimensions, especially with complex capital structures.

### Missing The 45-Day Holding Period

The DRD requires holding the stock more than 45 days within the 91-day window around the ex-dividend date. Buying just before a dividend and selling just after fails the test, and the full dividend is taxable with no DRD.

### Ignoring Hedging Reductions To The Holding Period

Days for which risk of loss is substantially reduced through options, short sales, or hedges do not count toward the 45-day holding period. A 50-day holding period with 20 hedged days yields only 30 qualified days, failing the test.

### Forgetting The Debt-Financed Portfolio Stock Reduction

Stock purchased with borrowed funds has its DRD reduced proportionally to the indebtedness. Heavily leveraged corporate portfolios may see the DRD nearly eliminated. Always test for debt-financing before claiming the full DRD.

### Assuming 80% Ownership Automatically Creates An Affiliated Group

The 100% DRD requires affiliated group status under Section 1504, not just 80% ownership. The group must meet definitional requirements including exclusions and tax-year consistency. Confirm affiliated group status before claiming 100%.

### Applying The DRD To Non-Corporate Recipients

The DRD is available only to C corporations. Individuals, partnerships, and S corporations do not claim the DRD. An S corporation receiving a dividend passes it through; the shareholders cannot apply the DRD.

## Self-Check

- [ ] Has the correct DRD tier (70%, 80%, or 100%) been determined based on the recipient corporation's ownership of the payer, with the ownership percentage computed precisely?
- [ ] Has ownership been measured under both the voting power and value tests, with attribution rules applied, rather than relying on a single ownership number?
- [ ] Has the 45-day holding period within the 91-day window been verified, with any days of substantially diminished risk of loss (hedging, options, short sales) excluded from the qualified days?
- [ ] Has the debt-financed portfolio stock reduction under Section 246A been tested, with the DRD reduced proportionally for any indebtedness attributable to the stock?
- [ ] Has affiliated group status under Section 1504 been confirmed before claiming the 100% DRD, rather than assuming 80% ownership is sufficient?
- [ ] Has it been confirmed that the recipient is a C corporation, since the DRD is not available to individuals, partnerships, or S corporations at the recipient level?
- [ ] Has the anti-triple-taxation rationale been applied to confirm the dividend represents already-taxed corporate profit flowing between corporate entities?
- [ ] Are the ownership percentage, holding period computation, debt-financing analysis, and tier determination documented with supporting records (capitalization table, trade confirmations, debt tracing)?
- [ ] Has the agent noted that attribution and affiliated group rules have detailed regulations, this is not tax advice, and recommended consulting a qualified tax professional (CPA or tax attorney) for the specific ownership and holding facts?
