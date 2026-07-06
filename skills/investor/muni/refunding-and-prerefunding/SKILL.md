---
name: refunding_and_prerefunding.md
description: Use when the agent is analyzing refunded or prerefunded municipal bonds, escrow structuring, refinancing economics, yield differences between prerefunded and new-money bonds, interest-cost savings, remaining maturity, and the residual risks after defeasance.
---

# Refunding And Prerefunding

Refunding is the replacement of an outstanding bond with a new bond, usually to lower interest cost. Prerefunding is a specific, powerful variant: the outstanding bond is "defeased" — secured by an escrow of government securities whose cash flows exactly cover the remaining payments — so that the original bond becomes effectively backed by US Treasuries rather than the issuer. This transforms the bond's credit profile, but it also changes its yield, its call risk, and what the investor actually owns. Misunderstanding defeasance leads to buying a "safe" prerefunded bond at a yield that no longer compensates for what was given up, or to misreading the call structure.

Use this skill before answering questions such as "what is a prerefunded muni", "why does this bond yield less now", "is a prerefunded bond safe", or "what happens when a bond is refunded". The goal is to prevent the agent from treating a prerefunded bond as identical to the original, and from missing that the investor now holds an escrow-backed security with different credit, yield, and call characteristics.

## Core Rules

### Understand The Mechanics Of Defeasance

In a prerefund, the issuer deposits cash or proceeds from a new bond into an escrow. The escrow holds US government securities (or similarly safe instruments) structured so that their principal and interest exactly match the original bond's remaining debt service. Once the escrow is fully funded, the original bond is legally "defeased" — removed from the issuer's debt — and its payment depends entirely on the escrow, not on the issuer's credit.

The practical consequence: a prerefunded muni is, in substance, a US Treasury obligation for the remaining life of the escrow. Its credit quality jumps to that of the escrow assets, typically AAA. This is why prerefunded bonds trade at lower yields than the same issuer's live bonds.

### Distinguish Current Refunding From Advance (Pre)Refunding

The two are different transactions with different economics:

- Current refunding: the new bond is issued within a short window (typically 90 days) of redeeming the old bond. Simple, immediate interest-cost savings.
- Advance refunding / prerefunding: the escrow is set up and the old bond is defeased before the first call date or redemption date. The old bond remains outstanding but is escrow-backed until its call or maturity.

Historically, the federal tax exemption for interest on advance-refunding bonds was eliminated for new issues (post-2017 tax reform in the US), sharply reducing advance refunding volume. Current refunding remains common. Confirm which type and era a bond relates to, because the economics and tax treatment differ.

### Recalibrate Credit Quality After Defeasance

Once defeased, the bond's credit is the escrow's credit, not the issuer's. This has several implications:

- The bond's effective rating becomes that of the escrow assets (often AAA / US-government-backed).
- The investor no longer bears the issuer's fiscal or revenue risk for the defeased bond.
- A deteriorating issuer's prerefunded bonds remain safe even as the issuer's live bonds weaken.

This is a genuine safety improvement, but it is paid for through lower yield. The investor is trading issuer credit risk for Treasury-like credit risk and accepting a lower return. Judge whether the yield give-up is worth the safety gain.

### Re-Examine Yield And Pricing After Prerefunding

Prerefunded bonds trade on their escrow-backed, government-like credit quality, so they yield less than the issuer's live bonds. Key pricing points:

- Yield compresses toward the equivalent-maturity Treasury yield plus a small spread, not the issuer's spread.
- Because the escrow cash flows are fixed, the bond's cash flows are fixed and known.
- Yield-to-call and yield-to-maturity must be recalculated against the escrow-supported redemption schedule.

An investor who bought a bond for its issuer-specific yield may find the prerefund turns it into a low-yielding government proxy. This is not a loss of safety, but it is a change in the investment's character that may no longer fit the original objective.

### Model The Call Structure Carefully

Prerefunded bonds often have a call feature that becomes exercisable on the original first call date. The escrow is structured to support redemption at that call. This means:

- The bond is likely to be called on its first call date if rates have fallen (which is why it was prerefunded).
- Yield-to-call, not yield-to-maturity, is often the relevant measure.
- After the call, the investor receives principal and must reinvest at then-current (likely lower) rates — reinvestment risk.

Do not assume a prerefunded bond will run to maturity. Model the call scenario and the reinvestment consequence.

### Assess The Issuer's Refunding Economics And Incentives

Refunding is driven by interest-cost savings. The issuer refinances when:

- market rates have fallen enough that new-bond interest cost plus transaction and escrow costs is below the old bond's remaining cost;
- the savings clear internal or regulatory thresholds (often a present-value saving requirement);
- call provisions permit redemption.

When an issuer is actively refunding, its outstanding callable bonds trading above par are call candidates. Investors holding those bonds face call and reinvestment risk. Conversely, an issuer unable to access markets (credit stress) cannot refund, leaving older high-coupon bonds outstanding — beneficial for holders but a sign of issuer distress.

### Identify Residual Risks After Defeasance

Even after defeasance, residual risks remain:

- Reinvestment risk: called bonds return cash to be reinvested at lower rates.
- Tax-law risk: changes to the tax treatment of refunding bonds affect future issuer behavior and market structure.
- Liquidity: prerefunded bonds may trade differently (often more like governments) but seasoned issues can still be illiquid.
- Escrow reinvestment risk: in structures where the escrow holds non-Treasury securities, the escrow's own credit matters; verify the escrow assets are truly government-backed.

Defeasance removes issuer credit risk but does not remove rate, reinvestment, liquidity, or structural risk.

## Common Traps

### Treating A Prerefunded Bond As The Same Investment As Before

Defeasance changes the credit, yield, and call profile. The investor now holds an escrow-backed security, not the issuer's live obligation. Comparing pre- and post-prerefund yields as if nothing changed misrepresents the investment.

### Quoting Yield-To-Maturity For A Bond Likely To Be Called

Prerefunded bonds are frequently called at the first call date. Yield-to-maturity is then fictional; yield-to-call is the relevant measure, and reinvestment risk follows.

### Assuming Defeasance Removes All Risk

Defeasance removes issuer credit risk. Rate risk, reinvestment risk, call risk, liquidity risk, and (in some structures) escrow-asset risk remain.

### Ignoring The Yield Give-Up

The safety improvement of prerefunding is paid for through lower yield. An investor seeking income may find the prerefund no longer meets the objective, even though it is "safer."

### Conflating Current And Advance Refunding

Current and advance refunding have different mechanics, tax treatments, and historical availability (advance-refunding tax exemption was eliminated for new issues after 2017). Treat them separately.

### Overlooking Reinvestment Risk At Call

When a high-coupon prerefunded bond is called, the investor receives cash to reinvest at lower prevailing rates. The "safe" bond can still deliver a disappointing total return over the investor's horizon.

## Self-Check

- [ ] The mechanics of defeasance are explained, and the bond is recognized as escrow-backed rather than issuer-backed after prerefunding.
- [ ] Current refunding and advance (pre)refunding are distinguished, including the post-2017 elimination of advance-refunding tax exemption where relevant.
- [ ] Credit quality is recalibrated to the escrow assets (typically government-backed), and the yield give-up is judged as the cost of the safety gain.
- [ ] Yield-to-call is used when call is likely, and reinvestment risk at call is modeled.
- [ ] The issuer's refunding economics and incentives are assessed, and callable bonds trading above par are flagged as call candidates.
- [ ] Residual risks (rate, reinvestment, liquidity, escrow-asset, tax-law) are identified after defeasance.
- [ ] The conclusion avoids presenting a prerefunded bond as identical to the original and references the investor's income objective and reinvestment horizon.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
