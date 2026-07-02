---
name: intercompany_transactions.md
description: Use when the agent is analyzing transactions between members of a consolidated group, applying the Treas Reg 1.1502-13 intercompany transaction matching and deferral rules, determining when gain or loss on intercompany sales of inventory or depreciable property is deferred, handling intercompany dividends and items that eliminate on consolidation, tracking deferred items through subsequent depreciation or sale to an outsider, or assessing the compliance complexity of intercompany matching.
---

# Intercompany Transactions

When members of a consolidated group transact with each other, the consolidated return regulations under Treas Reg 1.1502-13 defer or eliminate the tax consequences of those transactions to prevent the group from recognizing income, gain, loss, or deduction that has no economic reality outside the group. The judgment problem is that a sale between two group members is not a real recognition event for consolidated purposes; the gain or loss is matched and deferred until the item leaves the group or is consumed through use, depreciation, or amortization. Agents tend to treat intercompany sales like third-party sales and record gain or loss immediately, which both distorts consolidated taxable income and violates the matching principle that the regulations enforce.

The harm is twofold. First, premature recognition can understate or overstate consolidated tax, producing errors that compound through deferred-item tracking and that surface painfully on examination. Second, the matching rules create a multi-year tracking obligation that, if mishandled, leads to double counting or permanent loss of the correct tax result. An intercompany sale of depreciable property, for example, triggers gain deferral that is then recovered over the buyer's depreciation of the asset, and if the buyer later sells the asset to an outsider, the deferred gain is triggered. Agents frequently miss the downstream matching, fail to maintain the intercompany item schedules, or assume that intercompany dividends and other eliminated items need no tracking.

This skill covers federal US consolidated return intercompany transaction rules under Treas Reg 1.1502-13. It presumes familiarity with consolidated group eligibility and complements the consolidated loss limitation analysis. It does not cover the eligibility election or the SRLY loss rules in detail, which are separate skills. All regulatory references reflect the current federal framework but must be verified against current law. This is structural tax analysis, not tax advice; a qualified tax professional must be consulted for any specific determination.

## Core Rules

### Understand The Purpose Of Intercompany Deferral

The intercompany transaction rules exist because, for consolidated purposes, a sale between two group members is economically a transfer within a single entity. If Member A sells inventory to Member B at a $100,000 gain, and Member B has not yet sold the inventory to an outsider, the group has not realized any external gain. Allowing the group to recognize the $100,000 gain would tax an internal transfer. The regulations therefore defer the gain until the inventory leaves the group through a sale to an outsider, or until the asset is consumed, depreciated, or amortized by the buying member.

The agent must internalize this purpose. Every intercompany transaction should be screened for whether it produces gain, loss, income, or deduction that is internal to the group and therefore subject to deferral or elimination. The default analytical question is: has this item left the group, or has it been consumed within the group? If neither, the item is likely deferred.

### Apply The Matching Principle For Intercompany Sales Of Property

Under Treas Reg 1.1502-13, an intercompany transaction is divided into the selling member's intercompany item (the gain or loss the seller recognizes) and the buying member's corresponding item (the basis, cost, or expense the buyer takes). The matching rule accelerates or defers the seller's item to align with when the buyer's corresponding item is taken into account. For inventory, the seller's gain is deferred until the buyer sells the inventory outside the group. For depreciable property, the seller's gain is deferred and then recovered through the buyer's excess depreciation over the deferred basis.

The agent must track both halves of each matched pair. The seller's deferred gain and the buyer's inflated basis (or the buyer's deducted cost) are linked, and the timing of recognition follows the buyer's use or disposition of the asset. A failure to maintain the paired items leads to either permanent deferral (gain never recognized) or premature recognition (gain taken before the item leaves the group).

### Track Deferred Items Through Depreciation And Amortization

When the buyer of intercompany property depreciates or amortizes the asset, the deferred gain is recovered over the asset's recovery period. The buyer's basis for consolidated purposes is adjusted to reflect the deferred gain, and the excess depreciation attributable to the inflated basis effectively brings the deferred gain into income over time. This is the mechanism by which the seller's gain is matched to the buyer's use of the asset within the group.

The agent should maintain a schedule for each intercompany asset showing the seller's deferred gain, the buyer's tax basis versus consolidated basis, the depreciation method and period, and the annual recovery of the deferred item. When the buyer sells the asset to an outsider, any remaining deferred gain is triggered in the year of sale. Without this schedule, the agent cannot determine the correct consolidated taxable income in any given year.

### Trigger Recognition On Sale To An Outsider Or Group Exit

The deferral ends when the asset or item leaves the group. This occurs when the buying member sells the asset to a non-member, when the asset is consumed in a transaction with an outsider, or when the buying member itself leaves the consolidated group. At that point, any remaining deferred gain or loss is taken into account by the selling member (or its successor in the attribute). The timing and amount of the trigger depend on the exit transaction and the remaining deferred balance.

The agent must monitor group exits and third-party dispositions of intercompany assets. A member leaving the group with intercompany items still deferred can trigger complex consequences, including the acceleration of deferred gain and attribute adjustments under the consolidated regulations. These exit events are high-risk because they concentrate years of deferred items into a single recognition event.

### Handle Intercompany Dividends And Eliminated Items Correctly

Dividends paid by one group member to another are generally eliminated on consolidation, because the group cannot pay a dividend to itself. The dividends received deduction and the intercompany dividend rules work together to prevent the group from recognizing income on an internal dividend. Similarly, intercompany receivables and payables, intercompany interest, and other internal items are eliminated in computing consolidated taxable income.

The trap is assuming that eliminated items need no documentation. While they may not affect current consolidated taxable income, eliminated items can matter for member-level attribute tracking, for separate return limitation year computations, and for members that leave the group. The agent should record eliminated items even though they do not appear in the consolidated return, because they may become relevant in later years or on deconsolidation.

### Distinguish Intercompany From Third-Party Transactions Precisely

The deferral rules apply only to transactions between group members. A sale by a member to a non-member is a third-party transaction recognized in full. The classification depends on the status of both parties at the time of the transaction. A transaction that begins as intercompany can become a third-party transaction if the buying member leaves the group before the item is consumed, triggering the exit rules.

The agent must confirm the group membership status of each party at the transaction date and at each subsequent relevant date. A corporation that joins or leaves the group mid-year changes the character of transactions around the boundary. Misclassifying an intercompany transaction as third-party, or vice versa, produces incorrect recognition timing.

### Maintain The Compliance Infrastructure For Multi-Year Tracking

Intercompany transaction compliance is among the most record-intensive areas of consolidated filing. The group must maintain schedules for each intercompany transaction, the matched seller and buyer items, the deferral and recovery status, the asset basis adjustments, and the recognition triggers. These schedules persist for the life of the deferred items, which can span many years for depreciable property.

The agent should treat intercompany tracking as a standing compliance function, not a year-end reconciliation. Entries should be made when transactions occur, reviewed for matching, and updated for depreciation, amortization, consumption, and exit events. The cost of disciplined tracking is far lower than the cost of reconstructing deferred items under examination, where missing records can lead to unfavorable presumptions.

## Common Traps

### Treating An Intercompany Sale Like A Third-Party Sale

Gain or loss on a sale between group members is deferred under the matching rules, not recognized immediately. Immediate recognition distorts consolidated taxable income.

### Failing To Track The Buyer's Corresponding Item

The seller's deferred gain is matched to the buyer's basis, cost, or expense. Tracking only the seller's side loses the linkage and the recovery timing.

### Ignoring Deferred Gain Recovery Through Depreciation

For intercompany depreciable property, deferred gain is recovered through the buyer's excess depreciation over the deferred basis. Without a schedule, the recovery is missed or misstated.

### Assuming Eliminated Dividends And Items Need No Record

Eliminated items may not affect current consolidated income but matter for member attribute tracking and deconsolidation. They should be documented.

### Missing The Recognition Trigger On Sale To An Outsider Or Group Exit

Remaining deferred gain is triggered when the asset leaves the group or the buying member departs. Exit events concentrate years of deferred items and are high-risk.

### Misclassifying Transactions Around Group Entry Or Exit Boundaries

A member joining or leaving mid-year changes whether a transaction is intercompany or third-party. Misclassification produces incorrect recognition timing.

### Letting Deferred-Item Schedules Lapse Across Years

Intercompany items can persist for the depreciable life of an asset. Lapsed schedules make reconstruction under examination difficult and can lead to unfavorable treatment.

## Self-Check

- Has each intercompany transaction been screened for deferral or elimination under Treas Reg 1.1502-13, rather than treated as a third-party sale?
- Are the selling member's intercompany item and the buying member's corresponding item tracked as a matched pair, with recognition timing aligned to the buyer's use or disposition?
- For intercompany depreciable or amortizable property, is a schedule maintained showing deferred gain, tax versus consolidated basis, depreciation period, and annual recovery?
- Is the recognition trigger monitored for third-party sales of intercompany assets and for group exit events that accelerate remaining deferred items?
- Are intercompany dividends, interest, receivables, and payables eliminated on consolidation while still documented for member attribute and deconsolidation purposes?
- Has the group membership status of each party been confirmed at the transaction date and at subsequent relevant dates, especially around entry and exit boundaries?
- Are deferred-item schedules maintained as a standing compliance function across the full life of the items, not reconstructed only at year-end or under examination?
- Has the agent confirmed that consolidated taxable income reflects matched and deferred treatment rather than immediate recognition of internal gains and losses?
- Are exit-event consequences, including acceleration of deferred gain and attribute adjustments, analyzed before a member departs the group?
- Has the agent escalated to a qualified tax professional or CPA for any specific intercompany transaction determination, recognizing this analysis is not tax advice?
