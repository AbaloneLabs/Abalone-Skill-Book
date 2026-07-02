---
name: liquidation_tax.md
description: Use when the agent is analyzing the tax consequences of a C corporation liquidation, applying Section 331 shareholder gain or loss recognition on liquidating distributions, Section 336 corporate-level gain or loss as if assets were sold at fair market value, computing the double tax layer, determining shareholder basis step-up, distinguishing C corporation from S corporation liquidation through accumulated adjustments account or AAA, or handling installment obligations in a liquidating distribution.
---

# Liquidation Tax

A corporate liquidation is the termination of a corporation's existence through the distribution of all its assets to its shareholders. For a C corporation, liquidation triggers tax at two levels: the corporation recognizes gain or loss as if it had sold the assets at fair market value under Section 336, and the shareholders recognize gain or loss on the liquidating distribution under Section 331, treated as full payment in exchange for their stock. The judgment problem is that liquidation is one of the few transactions where the double tax that defines C corporation status is fully realized in a single event, and agents frequently understate the combined burden, mishandle the basis step-up, or confuse C corporation liquidation with the very different S corporation regime.

The harm is that a liquidation can produce a tax bill with no corresponding cash, because the corporate-level gain is measured on appreciated assets regardless of whether they are sold, and the shareholder-level gain is measured on the value of the distributed property. Agents often miss that the corporation is deemed to sell at fair market value even for non-cash assets distributed in kind, that the shareholder's basis in the distributed property is stepped up to fair market value (creating a future deduction that only partially offsets the current double tax), and that installment obligations and receivables have special acceleration rules. Confusing C and S corporation liquidation is a particularly costly error, because the S corporation's accumulated adjustments account (AAA) permits a single-layer distribution that the C corporation cannot achieve.

This skill covers federal US corporate liquidation analysis under Sections 331 and 336, with comparison to S corporation liquidation. It does not cover reorganizations or spinoffs, which are separate skills. All statutory references, rates, and thresholds reflect the current federal framework but must be verified against current law. This is structural transactional tax analysis, not tax advice; a qualified tax professional or CPA must be consulted for any specific determination.

## Core Rules

### Recognize The Double Tax As The Defining Feature Of C Corporation Liquidation

A C corporation liquidation triggers two separate tax events. First, under Section 336, the corporation recognizes gain or loss as if it had sold each distributed asset to the shareholder at fair market value. Second, under Section 331, the shareholder recognizes gain or loss on the receipt of the liquidating distribution, treated as full payment in exchange for the stock surrendered. The corporate-level tax is at corporate rates (21 percent on the gain), and the shareholder-level tax is at capital gains or ordinary rates depending on the stock's character and holding period.

The agent must model both layers together. A corporation with $1,000,000 of built-in gain on its assets faces roughly $210,000 of corporate-level tax (at 21 percent), reducing the net value distributed, and the shareholder then recognizes gain on the distribution measured against their stock basis. The combined effective rate can approach or exceed 40 percent of the pre-tax appreciation. Presenting only one layer understates the cost and can lead a client to liquidate without provisioning for the total liability.

### Apply Section 336 At The Corporate Level As A Deemed Sale

Section 336 treats the distribution of property in liquidation as a sale by the corporation at fair market value. The corporation recognizes gain or loss on each asset measured by the difference between fair market value and adjusted basis. This applies to cash-basis receivables, inventory, equipment, real estate, and intangibles. Character follows the asset: inventory gain is ordinary, capital assets produce capital gain, and Section 1231 property follows its own rules. Depreciation recapture under Section 1245 and Section 1250 applies as on any sale.

The agent must compute the deemed sale asset by asset, applying the correct character and recapture rules to each. A common error is treating the corporate-level gain as a single capital gain when the asset mix includes inventory, receivables, and recapture property that produce ordinary income. The corporate-level ordinary income can be substantial and is taxed at corporate rates, increasing the effective burden beyond a simple capital gains computation.

### Apply Section 331 At The Shareholder Level As Full Payment

Section 331 treats the shareholder's receipt of a liquidating distribution as full payment in exchange for the surrendered stock. The shareholder recognizes gain or loss measured by the difference between the fair market value of the property received (including cash) and the adjusted basis of the stock. The gain or loss is capital if the stock is a capital asset in the shareholder's hands, which it typically is for individual investors. The holding period of the stock determines long-term versus short-term treatment.

The agent must compute the shareholder-level gain using the fair market value of the distributed property, not the corporation's basis. A shareholder with low stock basis can recognize substantial gain even on a corporation that has paid corporate-level tax and distributed reduced net value. The two layers compound: the corporate-level tax reduces what is distributed, and the shareholder-level tax applies to the reduced value, but the combined burden is still large relative to the pre-liquidation appreciation.

### Determine The Shareholder Basis Step-Up In Distributed Property

Under Section 334(a), a shareholder receiving property in a liquidation takes a basis in the property equal to its fair market value at the time of distribution. This basis step-up aligns the shareholder's basis with the value on which the corporation was deemed to have paid tax, preventing a second corporate-level tax when the shareholder later sells the property. The step-up is a future benefit (lower future gain), not a current offset to the liquidation tax.

The agent must record the fair market value basis for each distributed asset and communicate that the step-up reduces future gain, not the current tax. A frequent error is presenting the basis step-up as if it eliminates the double tax; it does not. It merely ensures the shareholder is not taxed again on the same appreciation when the property is later sold. The holding period for the stepped-up basis begins at the distribution date.

### Distinguish C Corporation From S Corporation Liquidation

S corporation liquidation differs fundamentally because the S corporation's income has already been taxed to the shareholders at the shareholder level through pass-through. The accumulated adjustments account (AAA) tracks the previously taxed income, and distributions from AAA are generally tax-free to the extent of the shareholder's stock basis. This means an S corporation liquidation can often be accomplished with a single tax layer at the shareholder level, in contrast to the C corporation's double layer.

The agent must confirm the corporation's classification before analyzing liquidation. Treating an S corporation liquidation under the C corporation Section 336 deemed-sale rules overstates the tax, while treating a C corporation liquidation under S corporation single-layer rules understates it. The AAA, previously taxed income, and the order of distributions from different accounts (AAA, accumulated earnings and profits for former C corporations, and other accounts) determine the S corporation result and must be traced carefully.

### Handle Installment Obligations And Receivables

Installment obligations and accounts receivable distributed in liquidation can trigger acceleration of income. Under Section 336 and the related installment rules, the distribution of an installment obligation in a C corporation liquidation can cause the corporation to recognize the remaining deferred gain as if the obligation were sold. Similarly, cash-basis receivables distributed are treated as realized at fair market value, producing ordinary income at the corporate level.

The agent must identify all installment obligations, accrued but unpaid receivables, and deferred income items before computing the liquidation tax. These items can generate ordinary corporate-level income that does not correspond to cash, creating a liquidity problem. The timing of receivable collection and the characterization of the income should be analyzed, because accelerating deferred income into the liquidation year can push the corporation into higher effective rates and leave it without cash to pay the tax.

### Coordinate Timing, Filing, And Distribution Mechanics

A liquidation must follow statutory and regulatory procedures, including the adoption of a plan of liquidation, the filing of a final corporate return reporting the Section 336 deemed sale, the filing of shareholder returns reporting the Section 331 exchange, and the formal dissolution under state law. The timing of distributions, the valuation of distributed property, and the documentation of fair market value (often requiring appraisals for non-cash assets) are critical. A plan of liquidation can affect the timing of certain items under specific provisions.

The agent should map the procedural sequence and ensure that valuations, returns, and state dissolution are coordinated. The final corporate return must report the deemed sale and any resulting tax, and the shareholders must report the exchange in the correct year. Errors in timing or valuation can produce mismatches between corporate and shareholder reporting that surface on examination.

## Common Traps

### Presenting Only One Tax Layer

C corporation liquidation triggers both Section 336 corporate-level tax and Section 331 shareholder-level tax. Modeling only one understates the combined burden, which can approach or exceed 40 percent of appreciation.

### Treating Corporate-Level Gain As A Single Capital Gain

The deemed sale applies asset by asset, with ordinary character for inventory and receivables and recapture for depreciation. The ordinary component increases the effective corporate-level tax.

### Assuming The Basis Step-Up Eliminates The Double Tax

The Section 334(a) step-up reduces future gain on the distributed property; it does not offset the current liquidation tax. It prevents a third layer, not the second.

### Confusing C Corporation And S Corporation Liquidation

S corporation liquidation can achieve a single tax layer through AAA and previously taxed income. Applying C corporation rules to an S corporation overstates tax, and vice versa.

### Overlooking Installment Obligation And Receivable Acceleration

Distributing installment obligations or cash-basis receivables can accelerate deferred income into the liquidation year, generating tax without corresponding cash.

### Using Book Value Instead Of Fair Market Value

Both the Section 336 deemed sale and the Section 331 exchange are measured at fair market value. Book or adjusted basis produces incorrect gain computations and requires appraisals for non-cash assets.

### Failing To Coordinate Corporate And Shareholder Return Timing

The final corporate return and the shareholder exchange must be reported in the correct years and with consistent valuations. Mismatches create examination exposure.

## Self-Check

- Has the liquidation been modeled with both the Section 336 corporate-level deemed sale tax and the Section 331 shareholder-level exchange tax, showing the combined effective burden?
- Is the corporate-level gain computed asset by asset at fair market value, with correct character for inventory, receivables, capital assets, and Section 1245 and 1250 recapture?
- Is the shareholder-level gain computed using the fair market value of distributed property against the adjusted basis of the stock, with capital character and holding period applied?
- Has the Section 334(a) basis step-up to fair market value been recorded for each distributed asset, and communicated as a future benefit rather than a current offset?
- Has the corporation's classification been confirmed, distinguishing C corporation double-layer liquidation from S corporation single-layer treatment through AAA and previously taxed income?
- Have installment obligations, cash-basis receivables, and deferred income items been identified and their acceleration into the liquidation year analyzed for tax without cash?
- Are fair market value valuations supported by appraisals or defensible methodology for all non-cash distributed assets, avoiding reliance on book value?
- Has the procedural sequence been mapped, including plan of liquidation adoption, final corporate return, shareholder reporting, and state law dissolution?
- Are corporate and shareholder returns coordinated in timing and valuation to avoid reporting mismatches?
- Has the agent escalated to a qualified tax professional or CPA for any specific liquidation determination, recognizing this analysis is not tax advice?
