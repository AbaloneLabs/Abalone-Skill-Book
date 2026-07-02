---
name: intercompany_profit_elimination.md
description: Use when the agent is eliminating unrealized intercompany profit or loss on inventory, fixed assets, or other transfers between consolidated entities, distinguishing downstream from upstream transfers, accounting for noncontrolling interest impact, or applying inventory and asset profit elimination mechanics.
---

# Intercompany Profit Elimination

When one entity in a group sells goods or assets to another, the selling entity records a profit, but to the consolidated group that profit is not real until the asset is sold to a party outside the group. Until then, the profit is unrealized and must be eliminated against the carrying amount of the asset still held within the group. The mechanics differ depending on whether the sale was downstream (parent to subsidiary) or upstream (subsidiary to parent), and the noncontrolling interest must be given its share of any eliminated upstream profit. Agents frequently eliminate the profit against consolidated retained earnings only, forgetting the noncontrolling interest, or apply the same treatment to downstream and upstream, or fail to reverse the elimination when the asset is finally sold externally. The result is consolidated profit and asset balances that are wrong in ways that compound over multiple periods.

Use this skill before eliminating intercompany profit on inventory or fixed assets, calculating the noncontrolling interest share, reversing prior-period eliminations, or handling intercompany losses. The goal is to prevent the agent from treating intercompany profit elimination as a flat debit to retained earnings and missing the downstream-versus-upstream distinction, the noncontrolling interest, and the reversal mechanics.

Intercompany profit elimination follows the consolidation guidance of the applicable framework (US GAAP, IFRS, local GAAP), and the noncontrolling interest mechanics differ in detail. This skill gives the general framework; the applicable standard governs. For groups with complex structures, partial ownership, or multi-step intercompany chains, involve a qualified accountant. This is structural guidance, not a determination for a specific group.

## Core Rules

### Identify Unrealized Intercompany Profit Each Period

Unrealized intercompany profit is the profit recorded by a group entity on a sale to another group entity, to the extent the asset is still held within the group at the reporting date. It arises on inventory, fixed assets, intangibles, and any asset transferred between group entities. Identify it each period by comparing intercompany sales against the inventory or assets still on hand.

Calculate the profit to eliminate as: intercompany markup percentage multiplied by the intercompany cost of the assets still held; or the difference between the transfer price and the original cost to the group, for the units still held. Apply the elimination to the carrying amount of the asset (inventory, fixed asset) and to consolidated profit. Do not eliminate profit on assets already sold to external parties; that profit is realized.

### Distinguish Downstream From Upstream Transfers

A downstream transfer is from the parent to a subsidiary. An upstream transfer is from a subsidiary to the parent, or between two subsidiaries. The distinction matters because of the noncontrolling interest: for a downstream transfer, the entire unrealized profit is eliminated against the parent's interest; for an upstream transfer, the unrealized profit is allocated between the controlling interest (the parent) and the noncontrolling interest based on the selling subsidiary's ownership.

Apply the rule: downstream eliminations hit only the parent's retained earnings; upstream eliminations are shared between the parent and the noncontrolling interest in proportion to the selling entity's ownership. Getting this wrong misstates both consolidated retained earnings and noncontrolling interest.

### Apply The Noncontrolling Interest Share Correctly

For an upstream transfer from a subsidiary that is not wholly owned, the noncontrolling interest bears its percentage share of the unrealized profit eliminated. The elimination reduces consolidated profit, with the reduction split between the parent's retained earnings and the noncontrolling interest.

For example, if a subsidiary 80 percent owned by the parent sells inventory upstream with 100 of unrealized profit still held in the group, the elimination reduces the parent's retained earnings by 80 and the noncontrolling interest by 20. For a downstream transfer, the full 100 reduces the parent's retained earnings and the noncontrolling interest is unaffected. Document the ownership percentages and the direction of the transfer for each elimination.

### Eliminate Against The Correct Asset

The unrealized profit is embedded in the carrying amount of the asset held within the group. For inventory, eliminate against consolidated inventory. For fixed assets and intangibles, eliminate against the carrying amount of the fixed asset or intangible, and the elimination amortizes over the remaining useful life of the asset as it is depreciated or amortized.

For inventory, the elimination reverses in the next period when the inventory is sold externally (or persists if the inventory is still on hand). For fixed assets, the elimination creates a temporary difference that unwinds through depreciation over the asset's life. Match the elimination and its reversal to the asset type.

### Reverse Prior-Period Eliminations As Assets Are Sold

Unrealized profit eliminated in one period becomes realized when the asset is sold to an external party. Reverse the prior-period elimination at that point: for inventory, the reversal occurs when the inventory is sold externally in the following period; for fixed assets, the reversal occurs gradually through reduced depreciation, or fully on disposal of the asset.

Track eliminations by asset and by period so that reversals are complete and accurate. A common error is to eliminate profit each period without reversing the prior period's elimination, which double-counts and over-reduces consolidated assets and profit over time.

### Handle Intercompany Losses Symmetrically But Carefully

Intercompany losses on transfers between group entities are also unrealized while the asset is held within the group and are eliminated in the same manner as profits, with the same downstream-versus-upstream and noncontrolling interest mechanics. However, assess whether the loss indicates an impairment of the asset; if the asset's value has declined, recognize the impairment rather than (or in addition to) eliminating the intercompany loss.

Do not recognize an intercompany loss that is purely a transfer pricing artifact while the asset is still held and recoverable. Eliminate it; recognize impairment separately if the carrying amount exceeds recoverable amount.

### Coordinate With Impairment And Fair Value Adjustments

When an asset acquired from a group entity is subsequently impaired, the original intercompany profit must be considered in determining the recoverable amount. The group should not recognize a loss that simply reverses previously eliminated intercompany profit, nor should it carry an asset above the cost to the group. Coordinate the impairment test with the intercompany profit elimination so the asset is carried at no more than its cost to the group, less accumulated depreciation and impairment.

Similarly, fair value adjustments on a business combination that acquired an entity holding intercompany assets must be considered alongside the profit elimination. The two adjustments interact and must not double-count.

### Document The Elimination Schedule

Maintain a schedule of intercompany profit eliminations showing: the transferring and receiving entities; the direction (downstream or upstream); the ownership of the selling entity; the asset type; the unrealized profit amount; the split between parent and noncontrolling interest; the asset account adjusted; and the reversal in subsequent periods. This schedule is the basis for the consolidation entries and the audit trail.

A well-maintained schedule prevents the compounding errors that arise when eliminations are made ad hoc each period without tracking reversals.

## Common Traps

### Eliminating All Profit Against Parent Retained Earnings

For upstream transfers from partially owned subsidiaries, the noncontrolling interest must bear its share of the eliminated profit. Charging the full elimination to the parent misstates both retained earnings and noncontrolling interest.

### Treating Downstream And Upstream Identically

Downstream eliminations affect only the parent; upstream eliminations are shared with the noncontrolling interest. Applying the same treatment to both misallocates the elimination.

### Forgetting To Reverse Prior-Period Eliminations

When inventory or an asset is sold externally, the prior-period elimination must be reversed. Failing to reverse compounds the elimination over multiple periods and progressively understates consolidated assets and profit.

### Eliminating Profit On Assets Already Sold Externally

Profit on intercompany transfers where the asset has since been sold to an external party is realized and should not be eliminated. Eliminating realized profit understates consolidated revenue and profit.

### Amortizing Inventory Eliminations Like Fixed Assets

Inventory eliminations reverse when the inventory is sold externally, typically the next period. Fixed asset eliminations unwind through depreciation over the useful life. Applying the wrong reversal pattern misstates both the asset and the expense.

### Ignoring Impairment Signals In Intercompany Losses

An intercompany loss may indicate real impairment. Eliminating the loss without testing for impairment can leave the asset overstated, while recognizing a transfer-pricing loss as real overstates the group loss.

### Losing Track Across Multi-Step Intercompany Chains

When assets move through several group entities before reaching the holder, the cumulative intercompany profit must be eliminated. Tracking only the last leg understates the elimination.

## Self-Check

- Is unrealized intercompany profit on inventory, fixed assets, and other assets still held within the group identified and calculated each period?
- Are downstream transfers (parent to subsidiary) eliminated against the parent's retained earnings only?
- Are upstream transfers (subsidiary to parent or between subsidiaries) eliminated with the noncontrolling interest bearing its ownership share?
- Is the elimination recorded against the correct asset account, with inventory eliminations reversing on external sale and fixed asset eliminations unwinding through depreciation?
- Are prior-period eliminations reversed when the asset is sold to an external party or disposed of?
- Are intercompany losses eliminated symmetrically, with impairment tested separately where the loss signals a real decline in value?
- Is the elimination coordinated with impairment tests and business-combination fair value adjustments to avoid double-counting or carrying assets above cost to the group?
- Does an elimination schedule track each transfer by direction, ownership, asset type, profit amount, parent versus noncontrolling split, and reversal?
- Are multi-step intercompany chains tracked so the cumulative profit to the group is eliminated, not only the last leg?
- Has a qualified accountant reviewed eliminations involving partial ownership, multi-step chains, or significant fixed-asset transfers?
