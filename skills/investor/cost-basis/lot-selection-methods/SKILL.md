---
name: lot_selection_methods.md
description: Use when the agent is choosing a cost basis method (FIFO, LIFO, specific identification, HIFO, average cost), selecting which tax lots to sell, optimizing after-tax return through lot selection, or evaluating the tradeoffs of different basis methods. Covers specific-ID versus default methods, matching lots to tax goals, the irreversibility of method elections, and how lot selection interacts with tax-loss harvesting and long-term holding.
---

# Lot Selection Methods

Cost basis lot selection determines which specific purchase lots are deemed sold when a security is partially sold, and this choice directly changes the gain or loss realized, its character (short-term versus long-term), and the tax owed. A portfolio with identical holdings can produce very different tax bills depending on lot selection. Specific identification (specific-ID) lets the investor choose which lots to sell to minimize current tax, harvest losses, or preserve low-basis lots; default methods (FIFO, average cost) apply a fixed rule that is often tax-suboptimal. The choice is powerful, but it must be made correctly, before settlement, and with awareness that some elections are irreversible.

Agents tend to accept the broker's default method or pick a method based on a rule of thumb without modeling the after-tax outcome. The judgment problem is choosing the lot selection method and the specific lots that best serve the investor's tax situation, understanding the tradeoffs between methods, and recognizing the constraints (irreversibility, recordkeeping, mutual fund average-cost rules) that govern the choice.

This skill applies to selecting cost basis methods, choosing lots to sell, optimizing after-tax return, and coordinating lot selection with harvesting and holding-period strategy. It is not tax or investment advice; basis rules vary by jurisdiction and change, method availability and reversibility depend on the broker and asset type, and consult a qualified tax professional.

## Core Rules

### Default To Specific Identification For Maximum Flexibility

Specific-ID allows the investor to designate the exact lots sold at the time of sale, which is the most flexible and usually the most tax-efficient method. It enables selling high-basis lots to realize losses (harvesting), selling high-basis lots to minimize gains, or preserving low-basis lots for long-term holding or step-up at death. FIFO and other default rules remove this control and often realize the oldest (often low-basis, long-term) lots first, creating larger taxable gains.

Elect specific-ID as the default method for taxable accounts where the broker supports it (most do for stocks and ETFs). The flexibility to choose lots per sale is valuable for tax management. Only use a default method if specific-ID is unavailable or the investor's situation makes a fixed rule preferable.

### Match Lot Selection To The Tax Goal Of The Sale

Different sales have different tax goals, and lot selection should serve the specific goal:

- To minimize current taxable gains, sell the highest-basis lots first (HIFO), realizing the smallest gain or largest loss.
- To harvest a tax loss, sell lots with a basis above the current price.
- To manage short-term versus long-term character, prefer selling lots held over one year to access lower long-term rates, unless a short-term loss is specifically valuable.
- To preserve low-basis lots for step-up at death or charitable giving, avoid selling them when higher-basis lots can serve the same purpose.

State the goal before selecting lots. Selling without a tax goal, or under a fixed FIFO rule, often realizes gains that specific-ID could have avoided or defers losses that could have been harvested.

### Understand FIFO, LIFO, And HIFO Tradeoffs

FIFO (first-in, first-out) sells the oldest lots first. In a rising market, these are the lowest-basis lots, producing the largest taxable gains, which is generally tax-inefficient but may be acceptable if the gains are long-term and the investor wants to reset basis or has losses to offset. LIFO (last-in, first-out) sells the newest, often highest-basis lots, minimizing current gains but tending to realize short-term character. HIFO (highest-in, first-out) sells the highest-basis lots first, minimizing current gains, similar to loss-minimizing specific-ID.

Each has a place: FIFO may suit an investor wanting long-term gains and basis reset; HIFO suits current-tax minimization; LIFO is rarely optimal but can realize short-term losses. The key is that the choice changes the tax bill and should be deliberate, not accidental via default.

### Recognize The Irreversibility And Timing Of Method Elections

For some asset types and brokers, the cost basis method election is difficult or impossible to reverse after a sale settles, and for mutual funds, once average cost is elected and a sale occurs, revoking it is restricted. The method election must be made correctly and on time. Specific-ID designations must be made at or before the time of settlement, with adequate records; designations after the fact are generally not honored.

Confirm the broker's method election mechanics and deadlines before selling. For mutual funds, understand that electing average cost can lock in that method going forward, forfeiting specific-ID flexibility. Treat the method election as a consequential, sometimes irreversible decision, not a setting to change casually.

### Account For Mutual Fund Average Cost Specifics

Mutual funds (in the US) uniquely allow an average-cost method, which averages the basis across all shares of the fund. Average cost simplifies recordkeeping but loses lot-level granularity: you cannot selectively harvest specific lots or preserve low-basis shares. Once average cost is elected and a sale occurs, switching away is restricted, and the averaging can produce suboptimal outcomes versus specific-ID.

For mutual funds in taxable accounts, weigh the simplicity of average cost against the tax efficiency of specific-ID. For most active tax management, specific-ID is preferable. Reserve average cost for investors who will not manage lots and for whom simplicity outweighs the tax cost.

### Coordinate Lot Selection With Wash Sale And Harvesting Rules

Lot selection interacts with the wash sale rule: selling a lot at a loss and repurchasing a substantially identical security within 30 days disallows the loss. When harvesting losses via specific-ID, ensure the replacement does not trigger a wash sale across accounts. Conversely, lot selection can manage wash sale exposure by choosing which lots to sell.

Also coordinate with holding-period management: prefer selling lots held over one year for long-term rates, unless a short-term loss is more valuable (short-term losses offset short-term gains first, which are taxed higher). Lot selection, harvesting, and character management are a single integrated tax-trading process, not separate decisions.

### Preserve Low-Basis Lots For Estate And Charitable Planning

Low-basis lots held until death receive a stepped-up basis (under current law), eliminating the unrealized gain for heirs. Low-basis lots gifted to charity can be deducted at fair market value while never realizing the gain. These benefits make low-basis lots especially valuable to preserve. Specific-ID lets the investor sell higher-basis lots for liquidity while preserving the low-basis lots for these estate and charitable advantages.

When the investor intends to hold for legacy or charitable giving, use specific-ID to sell lots that do not carry these advantages, preserving the low-basis lots. This integrates lot selection with the broader estate and giving plan, not just current-year tax.

## Common Traps

### Accepting The Broker Default Without Review

FIFO or average-cost defaults often realize low-basis or short-term lots, raising tax. The trap is not electing specific-ID. Choose the method deliberately.

### Selling Low-Basis Lots When Higher-Basis Lots Would Do

Realizing large gains unnecessarily. The trap is FIFO in a rising market. Use HIFO or specific-ID to minimize gains when liquidity is the goal.

### Electing Average Cost For Mutual Funds Without Considering Lock-In

Average cost simplifies but forfeits lot-level control and is hard to reverse. The trap is electing it casually. Weigh simplicity versus tax efficiency.

### Missing The Settlement Deadline For Specific-ID Designation

Designations after settlement are not honored. The trap is intending specific-ID but failing to designate on time. Confirm mechanics before selling.

### Ignoring Short-Term Versus Long-Term Character

Selling a lot held 11 months realizes a short-term gain at ordinary rates. The trap is not checking holding periods. Prefer long-term lots unless a short-term loss is valuable.

### Harvesting A Loss And Triggering A Wash Sale Via Another Lot Or Account

Repurchasing the same security in another account within 30 days disallows the loss. The trap is harvesting without checking all accounts and reinvestment. Coordinate across accounts.

### Selling Low-Basis Lots That Could Have Been Preserved For Step-Up Or Charity

Low-basis lots have special estate and charitable value. The trap is selling them for liquidity when higher-basis lots would do. Preserve them via specific-ID.

## Self-Check

- [ ] Specific-ID is elected as the default where available, rather than accepting FIFO or average cost without review.
- [ ] Lots are selected to serve a stated tax goal (minimize gains, harvest losses, manage character, preserve low-basis lots), not sold under a fixed rule.
- [ ] The tradeoffs of FIFO, LIFO, HIFO, and average cost are understood and the method matches the investor's situation.
- [ ] Method election mechanics, deadlines, and irreversibility (especially for mutual fund average cost) are confirmed before selling.
- [ ] For mutual funds, average cost is chosen only where simplicity outweighs the tax cost of losing lot-level control.
- [ ] Lot selection is coordinated with wash sale rules across all accounts and with holding-period character management.
- [ ] Low-basis lots are preserved for step-up at death and charitable giving where those plans exist, via specific-ID selection of higher-basis lots for liquidity.
- [ ] Short-term versus long-term character is checked before each sale.
- [ ] The chosen lots are confirmed designated at or before settlement with adequate records.
- [ ] The conclusion notes basis rules vary by jurisdiction and change, method availability and reversibility depend on broker and asset type, recommends consulting a qualified tax professional, and is not personalized tax or investment advice.
