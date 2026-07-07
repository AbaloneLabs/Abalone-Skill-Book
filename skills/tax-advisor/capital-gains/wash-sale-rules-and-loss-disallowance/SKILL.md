---
name: wash_sale_rules_and_loss_disallowance.md
description: Use when the agent is evaluating whether a realized investment loss is disallowed under wash sale rules, determining whether replacement securities are substantially identical, computing basis adjustments for replacement shares, or reviewing loss harvesting strategies for wash sale compliance.
---

# Wash Sale Rules And Loss Disallowance

Wash sale rules exist to prevent taxpayers from claiming a tax loss while maintaining the same or a substantially identical investment position. The rule is deceptively simple in statement: if you sell a security at a loss and buy substantially identical securities within a 61-day window, the loss is disallowed. In application, the rules are full of traps involving what counts as substantially identical, how the 61-day window is measured, how the disallowed loss is recovered through basis adjustment, and the especially harsh rule that applies when replacement shares are purchased in a retirement account. A wash sale error can disallow a loss the taxpayer believed was realized, defer the tax benefit indefinitely, or in the IRA case eliminate it entirely.

Agents tend to treat wash sales as a minor footnote to loss harvesting and miss the definitional ambiguity of substantially identical, the per-share rather than per-account application, the automatic broker reporting for covered securities, and the catastrophic permanent disallowance for IRA purchases. The harm is a loss harvesting strategy that fails to produce the intended deduction, or worse, a taxpayer who repeatedly triggers wash sales through automated reinvestment without realizing the losses are being disallowed.

This skill covers the wash sale rules for U.S. federal income tax purposes. It is not tax advice; wash sale rules have interpretive nuances, and a qualified tax professional must be consulted for actual situations.

## Core Rules

### Apply The 61-Day Window Correctly

A wash sale occurs when a taxpayer sells a security at a loss and purchases substantially identical securities within the period beginning 30 days before the sale and ending 30 days after the sale. This is a 61-day window: the day of the sale itself plus 30 days on each side. The window is measured in calendar days, not business days, and includes weekends and holidays.

The most common error is assuming the window applies only to purchases after the sale. A purchase 20 days before the sale, followed by the sale at a loss, triggers a wash sale if no other exclusion applies. The 30-day lookback is just as important as the 30-day forward period. When evaluating a loss harvesting transaction, check both directions: any purchase of substantially identical securities in the 30 days before the planned sale, and any purchase in the 30 days after. This includes reinvested dividends, automated purchases, and option exercises that occur within the window.

### Determine Whether Replacement Securities Are Substantially Identical

The wash sale rule applies only to purchases of substantially identical securities, not to any security in the same industry or sector. The standard for substantially identical is stricter than many taxpayers assume. Shares of the same company's common stock are substantially identical to each other. Different classes of stock in the same company (common versus preferred, or different share classes) generally are not substantially identical unless they are economically equivalent.

For mutual funds and exchange-traded funds, the substantially identical question is more nuanced. Two different mutual funds tracking the same index may or may not be substantially identical depending on the specific facts; the IRS has not provided comprehensive guidance, and practitioners generally treat different funds from different providers tracking the same index as not substantially identical, but this is a judgment call. Bonds and options raise additional questions: a call option on a stock may be substantially identical to the stock itself if the option is deep in the money and functionally equivalent to holding the shares. Do not assume that any two investments in the same sector are substantially identical, but also do not assume that different tickers are automatically safe.

### Compute The Basis Adjustment For Replacement Shares

When a wash sale disallows a loss, the disallowed loss is not lost permanently (in most cases); it is added to the cost basis of the replacement securities. This basis adjustment preserves the economic loss but defers its recognition until the replacement shares are sold. The holding period of the replacement shares is also adjusted to include the holding period of the original shares, which can affect whether a future gain or loss is short-term or long-term.

For example, if a taxpayer sells 100 shares at a 5,000 dollar loss and purchases 100 replacement shares within the window, the 5,000 dollar loss is disallowed and added to the replacement shares' basis. If the replacement shares cost 10,000 dollars, the adjusted basis becomes 15,000 dollars. When the replacement shares are later sold, the 5,000 dollar economic loss is embedded in the higher basis and recognized at that time. Track the basis adjustment carefully, because brokers report adjusted basis for covered securities but the taxpayer is responsible for accuracy.

### Apply The Rule At The Share Level, Not The Account Level

Wash sales are evaluated at the individual share or lot level, not at the overall account level. A taxpayer who sells 200 shares at a loss and purchases 100 replacement shares within the window has a wash sale on 100 shares (the matched portion) but not on the remaining 100 shares. The unmatched 100 shares' loss is allowed.

This per-share application creates complexity when partial purchases occur. If a taxpayer sells 500 shares at a loss and buys 100 shares on day 10, another 200 shares on day 20, and another 100 shares on day 25, all within the window, the wash sale applies to 400 of the 500 sold shares, and the remaining 100 shares' loss is allowed. The basis adjustments are applied to the specific replacement lots in the order purchased. Do not treat the wash sale as an all-or-nothing account-level event.

### Recognize The IRA Wash Sale Permanent Disallowance

The most severe wash sale rule applies when replacement securities are purchased in an IRA or other retirement account. In this case, the disallowed loss is not added to the basis of the replacement shares; it is permanently disallowed with no basis adjustment and no future recovery. The loss vanishes entirely for tax purposes.

This rule catches taxpayers who sell at a loss in a taxable brokerage account and, within the window, have an automated purchase of the same security in their IRA. The IRA purchase may be an automated dividend reinvestment or a scheduled contribution purchase that the taxpayer does not actively manage. The result is a permanent loss of the tax deduction. When advising on loss harvesting, explicitly check whether the taxpayer has any IRA or retirement account that might purchase substantially identical securities within the window, including automated transactions.

### Account For Automatic Broker Reporting And Taxpayer Responsibility

For covered securities, brokers are required to detect and report wash sales that occur within the same account. The adjusted basis and disallowed loss appear on Form 1099-B. However, brokers are not required to detect wash sales across accounts at different brokers, and they do not detect wash sales involving IRA purchases. The taxpayer bears full responsibility for identifying and reporting these cross-account and cross-institution wash sales.

This means a taxpayer who sells at a loss at Broker A and buys replacement shares at Broker B within the window has a wash sale that neither broker reports. The taxpayer must adjust the basis manually on the tax return. Relying solely on the 1099-B without checking for cross-account activity understates wash sales and overstates deductible losses. Always ask whether the taxpayer has accounts at multiple institutions and whether any purchased substantially identical securities within the window.

### Evaluate Loss Harvesting Strategies For Wash Sale Compliance

Tax-loss harvesting is a common year-end strategy, but it only produces a deductible loss if it avoids wash sale disallowance. A compliant harvest requires either waiting more than 30 days before repurchasing substantially identical securities or purchasing a different (not substantially identical) security as a placeholder to maintain market exposure during the waiting period.

When recommending a replacement security, evaluate whether it is genuinely not substantially identical while providing similar market exposure. Replacing one S&P 500 index fund with a total market fund, or replacing a large-cap growth fund with a large-cap value fund, may provide diversification while avoiding the wash sale. However, the replacement must be chosen on its merits, not solely to game the wash sale rule, because the replacement may perform differently. Document the rationale for the replacement and confirm the 30-day window is respected on both sides of the sale.

## Common Traps

### Forgetting The 30-Day Lookback Before The Sale

The wash sale window includes 30 days before the sale. A reinvested dividend or automated purchase in the weeks before a planned loss harvest can trigger a wash sale. Always check the 30 days preceding the sale, not just the 30 days after.

### Treating Any Same-Sector Investment As Substantially Identical

Substantially identical is a strict standard. Two different technology stocks are not substantially identical. Two different ETFs tracking different indices are generally not substantially identical even if they overlap in holdings. Over-applying the standard causes taxpayers to avoid safe replacement investments during loss harvesting.

### Assuming The Same Ticker Is Always Safe After 31 Days

The 31-day rule is generally correct, but taxpayers sometimes count the days incorrectly, or a reinvested dividend on day 29 restarts the analysis. Count calendar days carefully and suspend automated reinvestment during the harvest window to avoid accidental purchases.

### Missing The IRA Permanent Disallowance

A wash sale involving an IRA purchase permanently disallows the loss with no basis adjustment. This is the harshest wash sale outcome and is frequently missed because the IRA purchase may be automated and the taxpayer does not connect the two accounts. Always check for IRA and retirement account activity during the window.

### Relying Solely On Broker 1099-B Reporting

Brokers report only same-account wash sales for covered securities. Cross-account, cross-broker, and IRA wash sales are not reported by brokers and must be identified by the taxpayer. Relying on the 1099-B without independent verification overstates deductible losses.

### Treating Wash Sales As Account-Level All-Or-Nothing

Wash sales apply at the share level. A partial replacement purchase creates a partial wash sale, with the unmatched portion of the loss still allowed. Applying the rule to the entire sale overstates the disallowance.

### Forgetting The Holding Period Tack Onto Replacement Shares

The replacement shares' holding period includes the original shares' holding period. This can extend a position into long-term status or complicate the holding period analysis for future sales. Track the adjusted holding period, not just the adjusted basis.

## Self-Check

- [ ] Has the full 61-day window (30 days before through 30 days after the sale) been checked for purchases of substantially identical securities, including reinvested dividends and automated purchases?
- [ ] Has the substantially identical standard been applied correctly, distinguishing same-company stock (substantially identical) from same-sector or similar-but-different investments (not substantially identical)?
- [ ] Has the disallowed loss been added to the basis of the specific replacement shares, with the holding period of the original shares tacked onto the replacement shares?
- [ ] Has the wash sale been applied at the share or lot level, with partial replacement purchases creating partial wash sales rather than account-level disallowance?
- [ ] Has the taxpayer's IRA and other retirement accounts been checked for purchases of substantially identical securities within the window, recognizing that IRA wash sales result in permanent disallowance?
- [ ] Has the analysis gone beyond the broker's 1099-B to check for cross-account and cross-institution wash sales that the broker does not report?
- [ ] For loss harvesting strategies, has a compliant replacement security been selected that is not substantially identical, or has a full 30-day waiting period been observed before repurchasing?
- [ ] Has automated reinvestment been suspended during the harvest window to prevent accidental replacement purchases?
- [ ] Has the wash sale analysis been documented with sale dates, purchase dates, substantially identical determination, and basis adjustments so it can be verified?
- [ ] Has the agent noted that this is general U.S. federal wash sale information, not tax advice, and recommended consulting a qualified tax professional for the specific situation?
