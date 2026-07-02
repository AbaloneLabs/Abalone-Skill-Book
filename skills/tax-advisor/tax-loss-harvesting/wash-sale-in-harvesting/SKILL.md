---
name: wash_sale_in_harvesting.md
description: Use when the agent is checking whether a tax-loss harvest triggers the wash sale rule, determining if two securities are substantially identical, sizing the 61-day window, or identifying wash sales caused by dividend reinvestment, spouse accounts, or IRA purchases. Covers IRC Section 1091, the 61-day window, substantially identical standard for stocks bonds and funds, related-party and account wash sales, and the permanent loss from IRA wash sales.
---

# Wash Sale In Harvesting

The wash sale rule under IRC Section 1091 disallows a loss when a taxpayer sells a security at a loss and buys a substantially identical security within 30 days before or 30 days after the sale, a 61-day window centered on the sale. The disallowed loss is not gone; it is added to the basis of the replacement security and recovered when that replacement is sold. But the deferral can be indefinite if the replacement is held long-term, and in certain cases (purchases within an IRA) the loss is permanently lost with no basis recovery. The rule is the single most common way a tax-loss harvest fails, and it applies far more broadly than most agents assume.

The judgment problem is that agents reduce wash sales to a simple check: did the taxpayer buy the same stock back within 30 days? In practice, the rule reaches substantially identical securities (not just the same ticker), covers purchases in spouse accounts and IRAs, is triggered by automatic dividend reinvestments, and can be triggered across multiple accounts at different brokers with no consolidated reporting. The harm is a harvest that appears successful but has its loss disallowed on examination, requiring an amended return and producing a larger gain later when the basis adjustment is finally realized. The most damaging variant is the IRA wash sale, where the disallowed loss vanishes entirely because the IRA has no basis to adjust, converting a deferral into a permanent destruction of the tax benefit.

This skill applies to wash sale detection, the substantially identical analysis, the 61-day window calculation, related-account wash sales, and the permanent-loss risk from IRA purchases. It is not tax advice; the substantially identical standard is fact-specific, the IRS has issued limited guidance on ETFs and mutual funds, and broker reporting on Form 1099-B is often incomplete or incorrect. Consult a qualified tax professional (CPA or tax attorney) before relying on any wash sale conclusion.

## Core Rules

### Apply The 61-Day Window Precisely

The wash sale window is 61 days: the day of the sale plus 30 days before and 30 days after. A sale on November 15 creates a window from October 16 to December 15. Any purchase of a substantially identical security on any day in that window, including the sale date itself, triggers the wash sale rule. The window is calendar days, including weekends and holidays, with no exceptions for market closures.

Calculate the window from the trade date, not the settlement date. As of May 2024, settlement is T+1, but the tax-relevant date is the trade date for both the sale and the replacement purchase. When planning a harvest, identify the exact 61-day window and check every account for purchases within it. A common error is checking only the 30 days after the sale and missing a purchase made in the 30 days before, which equally triggers the rule.

### Apply The Substantially Identical Standard By Security Type

The substantially identical standard varies by security type. For stocks and bonds, it is strict: stock of the same corporation is substantially identical, including across share classes (e.g., Berkshire A and B are the same company). Different companies in the same industry (e.g., two bank stocks) are not substantially identical. For mutual funds and ETFs, the standard is more flexible: two funds tracking the same index are generally not considered substantially identical, but the IRS has issued little definitive guidance, so the safe harbor is to use funds tracking different indexes or with different managers.

Classify the replacement against the original carefully. Replacing an S&P 500 ETF with another S&P 500 ETF is risky (same index, potentially substantially identical); replacing it with a total US market ETF is safer (different, though overlapping, index). Replacing a corporate bond with a bond of the same issuer and similar maturity is substantially identical; replacing with a different issuer's bond of similar maturity is not. When the standard is ambiguous, choose a replacement that is clearly different (different index, different issuer, different structure) to avoid the risk of disallowance on examination.

### Check All Related Accounts For Purchases

The wash sale rule applies to purchases by the taxpayer, but it also reaches purchases by related parties in certain circumstances. The clearest cases are purchases in the taxpayer's own IRA (traditional or Roth), purchases in a spouse's account when filing jointly (the IRS takes the position that spouse purchases trigger wash sales), and purchases across multiple brokerage accounts owned by the taxpayer. Brokers report wash sales only within their own platform, so cross-broker wash sales do not appear on Form 1099-B and must be self-identified.

Audit every account the taxpayer (and spouse, if filing jointly) controls for purchases of substantially identical securities within the 61-day window. This includes taxable brokerage accounts at multiple brokers, IRAs, 401(k)s (though 401(k) wash sales are rare because the taxpayer rarely controls individual stock purchases), and spouse accounts. The cross-broker and cross-account wash sale is the most commonly missed because no single broker sees the full picture. Maintain a consolidated view of all accounts during the harvest window.

### Identify The IRA Wash Sale As A Permanent Loss

When a wash sale is triggered by a purchase inside an IRA (traditional or Roth), the disallowed loss is permanently lost. Normally, a wash sale disallowed loss is added to the basis of the replacement security and recovered when sold. But an IRA has no basis in individual positions (contributions and earnings are tracked at the account level, not the security level), so the disallowed loss cannot be added to any basis and simply disappears. This converts the wash sale from a deferral into a permanent destruction of the tax benefit.

Treat any IRA purchase of a substantially identical security within the 61-day window as a catastrophic outcome to be avoided absolutely. The most common trigger is automatic dividend reinvestment in the IRA buying more of a security the taxpayer just sold at a loss in their taxable account. Before any harvest, check whether the security pays dividends and whether dividend reinvestment is active in any IRA; if so, turn off reinvestment before the harvest and for the full 61-day window. This single check prevents the most damaging wash sale outcome.

### Catch Automatic Dividend Reinvestment As A Hidden Trigger

Automatic dividend reinvestment (DRIP) is a frequent, hidden wash sale trigger. If a taxpayer sells a security at a loss and the next scheduled dividend reinvestment (within the 61-day window) buys additional shares of the same or a substantially identical security, the reinvestment is a purchase that triggers the wash sale rule on a portion of the loss equal to the reinvested shares. This applies in the taxable account itself and, more dangerously, in an IRA.

Before harvesting, identify all securities with active DRIP and the dates of upcoming reinvestments. Turn off DRIP for the harvested security across all accounts for the full 61-day window, or ensure the reinvestment date falls outside the window. Even small reinvestments (a $50 dividend) can disallow a proportional part of a large loss, and the administrative cost of untangling partial wash sales often exceeds the benefit. The cleanest practice is to disable reinvestment for any harvested position and reinvest manually after the window closes.

### Understand The Basis Adjustment And Recovery Mechanics

When a wash sale disallows a loss, the disallowed amount is added to the cost basis of the replacement security. If the taxpayer sold 100 shares at a $5,000 loss and the replacement cost $10,000, the replacement's basis becomes $15,000. When the replacement is eventually sold, the larger basis reduces the gain (or increases the loss) by $5,000, recovering the deferred benefit. The holding period of the replacement also includes the holding period of the sold shares, which can affect short-term vs long-term classification.

Track the basis adjustment on the replacement so the eventual recovery is captured. Brokers adjust basis on Form 1099-B for within-account wash sales, but not for cross-account or IRA wash sales. For cross-account wash sales, the taxpayer must adjust the basis manually on their records and tax return. Document the disallowed amount, the replacement security, and the adjusted basis so that the recovery is not lost when the replacement is sold years later.

### Verify Broker Reporting And Do Not Rely On It Alone

Brokers report wash sales on Form 1099-B, but only for transactions within their own platform. Cross-broker wash sales (sale at Broker A, purchase at Broker B) are not reported by either broker and must be self-identified and adjusted on the tax return. Broker reporting is also frequently corrected via amended 1099-Bs well into tax season, so a 1099-B received in January may differ from the final version in March.

Do not rely solely on the 1099-B wash sale figures. Reconcile the broker-reported wash sales against the taxpayer's own account-by-account audit of the 61-day windows. If the taxpayer's audit finds wash sales the broker missed (cross-account, spouse, IRA), adjust the return accordingly. If the broker reports a wash sale the taxpayer believes is incorrect, investigate before accepting it, as broker adjustments are sometimes erroneous, particularly around corporate actions and mergers.

## Common Traps

### Checking Only The 30 Days After The Sale

The trap is checking only post-sale purchases and missing a purchase made in the 30 days before the sale, which equally triggers the wash sale rule. Use the full 61-day window.

### Treating Same-Index ETFs As Always Safe

Two ETFs tracking the same index may be substantially identical; IRS guidance is limited. The trap is assuming same-index is always safe. Use different indexes or managers to be safe.

### Missing Cross-Broker And Spouse Account Wash Sales

Brokers report only within-platform wash sales. The trap is relying on 1099-B and missing a purchase at another broker or in a spouse's account. Audit all accounts.

### Triggering A Permanent Loss Through An IRA Purchase

An IRA purchase of a substantially identical security within the window permanently destroys the loss. The trap is not checking IRA accounts before harvesting. Check IRAs absolutely.

### Forgetting Dividend Reinvestment Creates Purchases

DRIP buys shares within the window, triggering a partial wash sale. The trap is leaving reinvestment on during the harvest. Turn off DRIP for the full 61-day window.

### Ignoring The Basis Adjustment On The Replacement

The disallowed loss is added to the replacement's basis and recovered later. The trap is not tracking the adjusted basis, losing the recovery when the replacement is sold. Document the adjustment.

### Relying On Broker 1099-B For Cross-Account Wash Sales

Brokers do not report cross-account wash sales. The trap is trusting the 1099-B without an independent audit. Reconcile broker reporting against a full account audit.

### Assuming Different Share Classes Or Bond Issues Are Not Identical

Different share classes of the same fund, or bonds of the same issuer with similar terms, can be substantially identical. The trap is treating them as different. Apply the strict standard to same-issuer instruments.

## Self-Check

- [ ] The full 61-day window (30 days before, sale date, 30 days after) is calculated from the trade date and checked for all purchases.
- [ ] The substantially identical standard is applied by security type (strict for stocks and bonds, flexible but uncertain for funds), with replacements chosen to be clearly different.
- [ ] All related accounts (taxable at multiple brokers, IRAs, spouse accounts) are audited for purchases of substantially identical securities within the window.
- [ ] IRA purchases within the window are identified and treated as a permanent loss of the disallowed amount, to be avoided absolutely.
- [ ] Automatic dividend reinvestment is checked for all harvested securities across all accounts and disabled for the full 61-day window.
- [ ] The basis adjustment on the replacement is documented (disallowed amount added to replacement basis) so the eventual recovery is captured when the replacement is sold.
- [ ] Broker-reported wash sales on Form 1099-B are reconciled against an independent account-by-account audit, recognizing that cross-account wash sales are not broker-reported.
- [ ] The holding period of the replacement includes the sold shares' holding period, which is factored into short-term vs long-term classification.
- [ ] Same-issuer instruments (share classes, bonds) are evaluated under the strict substantially identical standard, not assumed to be different.
- [ ] The conclusion notes the substantially identical standard is fact-specific with limited IRS guidance on funds, broker reporting is often incomplete, this is not tax advice, and recommends consulting a qualified tax professional (CPA or tax attorney) before relying on any wash sale conclusion.
