---
name: netting_and_offsets.md
description: Use when the agent is netting capital gains and losses for a corporation, applying the rule that corporate capital losses offset only capital gains with no 3000 dollar ordinary offset, computing Section 1231 netting, distinguishing ordinary loss from capital loss characterization, or carrying back and forward corporate capital losses.
---

# Netting And Offsets For Corporations

Corporate capital loss netting differs fundamentally from individual netting, and applying individual rules to a corporation produces materially wrong results. The central rule is that a C corporation's capital losses can offset only capital gains; there is no $3,000 annual deduction of capital losses against ordinary income as individuals enjoy. A corporation with a large capital loss and no capital gains gets no current deduction at all and must carry the loss back three years and then forward five years to find capital gains to absorb it. The judgment problem is that agents trained on individual rules default to the $3,000 ordinary offset and the indefinite carryforward, both of which do not exist for corporations.

Agents frequently miss that corporate capital losses have a strict 3-year carryback and 5-year carryforward (not the indefinite carryforward individuals have), that the loss characterization as capital versus ordinary depends on Section 1231 netting which can flip a net Section 1231 gain into ordinary income through recapture, and that net operating losses and capital losses are different attributes with different rules. The deeper blind spot is conflating ordinary losses (fully deductible against ordinary income) with capital losses (restricted to capital gains for corporations), which can overstate the current deduction by millions for a corporation with an asset sale loss. Correct characterization and netting determine whether a loss is usable now, in three years, or never.

This skill covers corporate capital gain and loss netting, Section 1231 netting, ordinary versus capital loss characterization, and the carryback and carryforward rules for corporate capital losses. It is not tax advice; characterization depends on asset facts and holding periods, and a qualified tax professional (CPA or tax attorney) must be consulted for actual transactions.

## Core Rules

### Apply The Corporate Capital Loss Restriction: Gains Only, No $3,000 Offset

A C corporation may deduct capital losses only to the extent of capital gains. There is no $3,000 annual deduction of net capital losses against ordinary income, which is a rule available only to individuals (and trusts and estates in some respects). If a corporation has $500,000 of capital losses and $0 of capital gains in the current year, the corporation deducts nothing currently against ordinary income; the entire $500,000 capital loss is carried to other years to offset capital gains.

This is the most consequential difference from individual treatment and the most common error. An agent who applies the individual $3,000 offset to a corporate return overstates the deduction by $3,000 in the current year and misstates the carryforward. For a corporation, the question is never "how much capital loss can we deduct this year" but rather "do we have capital gains to absorb the capital loss, and if not, what carryback or carryforward years have gains." Confirm the entity is a corporation before applying the gains-only restriction, and never import the $3,000 rule into a C corporation analysis.

### Carry Corporate Capital Losses Back 3 Years And Forward 5 Years

A corporate capital loss that exceeds current-year capital gains is carried back 3 years (to the earliest year first, then forward chronologically) and then forward 5 years, but only against capital gains in those years. This is a strict 8-year total window (3 back plus 5 forward), after which any unused corporate capital loss expires worthless. There is no indefinite carryforward for corporate capital losses, unlike the indefinite carryforward individuals have for capital losses.

The limited window creates urgency: a corporation with a large capital loss must find capital gains within 3 prior years or 5 subsequent years, or the loss is permanently lost. This can drive transaction planning, such as harvesting capital gains in carryforward years to absorb the loss before it expires. Always identify the year the capital loss arose and compute the expiration year (the loss year plus 5 forward years for the unused portion after carryback). A corporate capital loss from 2024 must be used by 2029 (after the 3-year carryback is exhausted), or it expires.

### Distinguish Ordinary Loss From Capital Loss By Asset Characterization

Whether a loss is ordinary or capital depends on the character of the asset sold and how it was held. Capital assets (investment stocks, securities, most assets held for investment or appreciation) produce capital gains and losses. Ordinary assets (inventory, accounts receivable, depreciable business property under certain rules) produce ordinary income and losses. The characterization matters enormously because ordinary losses are fully deductible against ordinary income with no netting restriction, while capital losses for corporations are restricted to capital gains.

The characterization analysis requires examining the asset and the transaction. A loss on the sale of investment stock is capital. A loss on the sale of inventory is ordinary. A loss on the sale of business equipment or real estate may be Section 1231 property, which has its own netting rules (see below) that can produce ordinary or capital treatment depending on the net result. Do not assume a business asset sale produces a capital loss; many business asset losses are ordinary or are converted to ordinary through depreciation recapture. Correct characterization is the foundation of correct netting.

### Apply Section 1231 Netting And Recapture

Section 1231 property (depreciable business real property and personal property held more than one year) has a unique netting rule. Individual Section 1231 gains and losses for the year are netted. If the result is a net Section 1231 gain, it is treated as long-term capital gain (favorable). If the result is a net Section 1231 loss, it is treated as ordinary loss (fully deductible, favorable for corporations because it avoids the capital loss restriction). This looks like a win-win, but recapture rules complicate it.

Section 1245 and Section 1250 recapture convert what might appear to be Section 1231 capital gain into ordinary income to the extent of prior depreciation. Section 1245 (personal property, equipment) recaptures all prior depreciation as ordinary income. Section 1250 (real property) generally recaptures only excess (accelerated) depreciation as ordinary income, though for corporations, Section 291 adds an additional 20% recapture of depreciation on real property. A corporation selling depreciated equipment may find that the entire gain is ordinary under Section 1245 recapture, leaving no capital gain and no benefit from Section 1231 capital treatment. Always run the recapture analysis before concluding Section 1231 gain is capital.

### Separate Net Operating Losses From Capital Losses

Net operating losses (NOLs) and capital losses are different attributes with different rules, and they must not be conflated. An NOL is the excess of allowable deductions over gross income, computed under the NOL rules; it is an ordinary loss attribute that offsets ordinary income subject to the post-TCJA 80% limitation and indefinite carryforward. A capital loss is a loss from the sale of capital assets, restricted for corporations to offsetting only capital gains, with a 3-year carryback and 5-year carryforward.

The distinction matters because an NOL is far more valuable to a corporation than a capital loss: an NOL can offset up to 80% of taxable income (including ordinary income), while a capital loss can offset only capital gains. A corporation with a large ordinary loss (say, from operations) generates an NOL that is broadly usable; a corporation with a large capital loss (from selling investment stock at a loss) is restricted to capital gains. Always identify whether the loss is ordinary (NOL-eligible) or capital (restricted), because the usability and carryforward treatment differ entirely.

### Net Gains And Losses Within Character Classes Before Cross-Netting

The netting process requires grouping gains and losses by character before combining. Capital gains and losses are netted among themselves (short-term with short-term, long-term with long-term, then the two combined). Section 1231 gains and losses are netted separately first (with recapture applied), and the net Section 1231 result is then characterized as capital or ordinary. Ordinary gains and losses are netted within the ordinary category. Only after each class is netted internally are the results combined for the overall tax computation.

The sequence matters because the character of a net Section 1231 gain (capital) or loss (ordinary) is determined by the netting, not by individual transactions. A corporation with a $100,000 Section 1231 gain on one asset and a $120,000 Section 1231 loss on another has a net Section 1231 loss of $20,000, treated as ordinary (fully deductible), not as a capital loss. Running the Section 1231 netting first, then characterizing the net result, is essential to avoid misclassifying the loss as a restricted capital loss.

### Document The Character, Netting, And Carryforward Of Each Loss

Because corporate capital losses are restricted and have a limited carry window, the analysis must document the character of each loss (capital versus ordinary versus Section 1231), the netting within each class, the resulting carryback or carryforward amount, and the expiration year for any capital loss carryforward. A corporate capital loss carryforward that is not tracked to its expiration year may be lost. The NOL carryforward must be tracked separately, with its vintage year and the applicable limitation (pre-2018 NOLs have no 80% limit; post-2017 NOLs do).

Retain the asset records, sale documents, depreciation schedules (for recapture), and the netting worksheets. The capital loss carryforward and NOL carryforward should be reconciled annually so that the available amounts and expiration years are current. A corporation that loses track of its capital loss carryforward expiration may forgo a valuable deduction.

## Common Traps

### Applying The Individual $3,000 Capital Loss Offset To A Corporation

Corporations cannot deduct capital losses against ordinary income at all; there is no $3,000 offset. Applying the individual rule overstates the current deduction and misstates the carryforward.

### Giving Corporate Capital Losses An Indefinite Carryforward

Corporate capital losses carry back 3 years and forward 5 years only, then expire. There is no indefinite carryforward as individuals have. A corporate capital loss must be used within the 8-year window or it is lost.

### Conflating Ordinary Loss With Capital Loss

Ordinary losses are fully deductible against ordinary income; capital losses for corporations are restricted to capital gains. Mischaracterizing an ordinary loss as capital (or vice versa) overstates or understates the usable deduction dramatically.

### Skipping Section 1231 Netting And Recapture

Section 1231 gains and losses must be netted together, and recapture under Sections 1245, 1250, and 291 can convert capital gain to ordinary income. Skipping the netting or recapture mischaracterizes the result.

### Treating An NOL As A Capital Loss (Or Vice Versa)

NOLs and capital losses are different attributes. An NOL offsets up to 80% of taxable income (including ordinary); a capital loss offsets only capital gains. Conflating them overstates the usability of a capital loss.

### Assuming A Business Asset Sale Produces A Capital Loss

Many business asset sales produce ordinary income (through recapture) or ordinary loss, not capital treatment. Depreciated equipment gains are often fully ordinary under Section 1245. Characterize each asset before netting.

## Self-Check

- [ ] Has it been confirmed that the entity is a C corporation, so that capital losses are restricted to offsetting only capital gains with no $3,000 ordinary deduction?
- [ ] Has the corporate capital loss been carried back 3 years and forward 5 years (not indefinitely), with the expiration year identified for any unused carryforward?
- [ ] Has each loss been characterized as capital, ordinary, or Section 1231 based on the asset and transaction, rather than assumed?
- [ ] Has Section 1231 netting been applied (gains and losses netted together), with the net result characterized as capital gain (if net gain) or ordinary loss (if net loss)?
- [ ] Has depreciation recapture under Sections 1245, 1250, and 291 (corporate real property) been computed, converting recaptured gain to ordinary income before any capital treatment?
- [ ] Have NOLs and capital losses been tracked as separate attributes, with NOLs subject to the 80% limitation and capital losses restricted to capital gains?
- [ ] Has netting been performed within each character class (short-term capital, long-term capital, Section 1231, ordinary) before cross-netting for the overall computation?
- [ ] Are capital loss carryforwards and NOL carryforwards documented with vintage years, applicable limitations, and expiration years for capital losses?
- [ ] Has the agent noted that asset characterization and recapture depend on detailed facts, this is not tax advice, and recommended consulting a qualified tax professional (CPA or tax attorney) for the specific transactions?
