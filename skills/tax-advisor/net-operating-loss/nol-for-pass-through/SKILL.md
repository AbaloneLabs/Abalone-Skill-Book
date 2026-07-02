---
name: nol_for_pass_through.md
description: Use when the agent is analyzing net operating losses for pass-through entities, applying basis limitations under Section 704(d), at-risk rules under Section 465, passive activity loss rules under Section 469, or the excess business loss limitation under Section 461(l) for owners of S corporations partnerships and LLCs.
---

# NOL For Pass-Through Entities

Pass-through entities (S corporations, partnerships, and LLCs taxed as partnerships) do not generate net operating losses at the entity level. The entity's losses flow through to the owners via Schedule K-1, and the owners apply a stack of limitations at the owner level before determining how much loss is currently deductible. The judgment problem is that agents treat the K-1 loss as immediately deductible by the owner, when in fact basis, at-risk, passive activity, and excess business loss rules can each suspend the loss at the owner level. A $100,000 K-1 loss may produce zero current deduction if the owner lacks basis, is not at risk, or the activity is passive.

Agents frequently miss the correct order of the limitations (basis first, then at-risk, then passive, then excess business loss), that each limitation produces a different kind of suspended loss with different release conditions, and that the Section 461(l) excess business loss limitation (which applies to non-corporate owners, not the entity) converts disallowed excess into an NOL carryforward at the owner level. The deeper blind spot is conflating corporate loss rules with pass-through owner rules: a C corporation's losses are limited at the entity level by a different stack, while a pass-through owner's losses are limited personally. Applying entity-level rules to an owner, or owner-level rules to an entity, produces incorrect results.

This skill covers pass-through loss flow via K-1, the basis limitation (Section 704(d)), at-risk rules (Section 465), passive activity rules (Section 469), and the excess business loss limitation (Section 461(l)). It is not tax advice; the limitation rules have detailed regulations and depend on the owner's specific facts, and a qualified tax professional (CPA or tax attorney) must be consulted for actual situations.

## Core Rules

### Understand Losses Flow Via K-1 And Are Limited At The Owner Level

A pass-through entity (S corporation or partnership) does not pay entity-level tax on its income, and its losses do not generate an entity-level NOL. Instead, the entity's income, losses, deductions, and credits are allocated to the owners and reported on Schedule K-1. The owners then report their shares on their personal returns (Form 1040 for individuals), where the loss limitations are applied. The entity-level loss is the starting point, not the deductible amount.

This means the pass-through loss analysis always has two stages: first, compute the entity-level loss and the owner's allocable share (per the operating agreement, capital accounts, and tax rules); second, apply the owner-level limitations (basis, at-risk, passive, excess business loss) to determine the currently deductible portion. An agent who stops at the K-1 loss amount overstates the deduction. Always carry the analysis through to the owner-level limitations, because that is where most pass-through losses are suspended.

### Apply The Basis Limitation First Under Section 704(d)

The basis limitation under Section 704(d) is the first gate: a partner or S corporation shareholder may deduct losses only to the extent of their adjusted basis in the partnership interest or S corporation stock. Basis is generally the owner's capital contribution plus share of income minus distributions and losses, adjusted for additional contributions, liabilities (for partners, who get basis for their share of partnership recourse and certain nonrecourse debt; S corporation shareholders get basis only for direct loans they make to the corporation), and other items. Losses in excess of basis are suspended and carried forward until basis increases (through additional capital, share of income, or debt assumptions).

The basis limitation is entity-structure-specific. Partners generally get basis for their share of partnership liabilities, which means a partnership financed with debt can give partners basis to deduct losses beyond their cash contribution. S corporation shareholders do not get basis for the corporation's debt unless the shareholder personally loans money to the corporation (a direct shareholder loan creates basis; a guarantee may not, depending on the facts). This structural difference means an S corporation shareholder with a loss and no basis may have zero current deduction, while a partner in an equivalent debt-financed partnership may deduct the loss. Confirm the entity type and compute basis under the correct rules before allowing the loss.

### Apply The At-Risk Limitation Second Under Section 465

After basis, the at-risk rules under Section 465 limit losses to the amount the owner has at risk in the activity. The at-risk amount generally includes cash and property contributed, borrowed amounts for which the owner is personally liable (recourse debt), and certain qualified nonrecourse financing (for real estate). Losses in excess of the at-risk amount are suspended and carried forward until the at-risk amount increases. The at-risk amount is often similar to basis but can differ, particularly for nonrecourse debt that gives basis but is not at risk.

The at-risk rules are especially relevant for activities financed with nonrecourse debt where the owner has no personal liability. A partner may have basis for their share of nonrecourse partnership debt (under Section 752), but if that debt is not qualified nonrecourse financing and the owner is not personally liable, the at-risk rules suspend the loss to the extent it exceeds the cash and recourse contributions. The at-risk limitation is applied after basis, so the deductible loss is the lesser of the basis-allowed loss and the at-risk amount. Document the nature of the debt (recourse, nonrecourse, qualified nonrecourse financing) to determine the at-risk amount correctly.

### Apply The Passive Activity Loss Rules Third Under Section 469

After basis and at-risk, the passive activity loss rules under Section 469 suspend losses from passive activities to the extent they exceed passive income. A passive activity is a trade or business in which the owner does not materially participate, or a rental activity (generally per se passive, with exceptions for real estate professionals). Passive losses can offset only passive income; they cannot offset active business income, wages, or investment income. Suspended passive losses carry forward until the owner has passive income or disposes of their entire interest in the passive activity in a fully taxable transaction.

Material participation is the key determination, and it is measured by seven tests (based on hours of participation, the nature of the work, and historical involvement). For most owners of pass-through businesses who actively work in the business, material participation makes the activity non-passive, and the losses are not suspended under Section 469. But for owners who are passive investors, or for rental activities, the passive rules suspend the loss even if the owner has basis and is at risk. Confirm the owner's material participation status (documenting hours if needed) and whether the activity is rental (per se passive unless an exception applies) before concluding the loss is deductible.

### Apply The Excess Business Loss Limitation Fourth Under Section 461(l)

After basis, at-risk, and passive, the excess business loss limitation under Section 461(l) applies to non-corporate taxpayers (individuals, trusts, estates). It limits the aggregate net business loss (from all business activities, after the prior limitations) to a threshold amount per year. For 2024, the threshold is approximately $305,000 for single filers and $610,000 for married filing jointly (indexed annually for inflation). Aggregate business deductions exceeding business income plus the threshold generate an excess business loss that is disallowed currently and carried forward as part of the owner's net operating loss carryforward.

The Section 461(l) limitation is applied at the owner level across all of the owner's business activities combined, not activity-by-activity. An owner with losses from multiple pass-through entities aggregates them and tests against the threshold. The disallowed excess becomes an NOL carryforward at the owner level (subject to the 80% usage limit when later used). This means a large pass-through loss that clears basis, at-risk, and passive can still be capped by Section 461(l) and converted to a future-year NOL. Always test the aggregate business loss against the threshold after applying the prior limitations, and recognize that the disallowed amount changes character to an NOL carryforward.

### Sequence The Limitations In The Correct Order

The four owner-level limitations are applied in a specific sequence: first basis (Section 704(d)), then at-risk (Section 465), then passive (Section 469), then excess business loss (Section 461(l)). Each limitation operates on the result of the prior one. A loss that clears basis but fails at-risk is suspended under at-risk; a loss that clears basis and at-risk but is passive is suspended under passive; a loss that clears all three but exceeds the aggregate threshold is converted to an NOL carryforward under Section 461(l).

The sequencing matters because each limitation produces a different suspended-loss treatment with different release conditions. Basis-suspended losses are released when basis increases (contributions, income, debt). At-risk-suspended losses are released when the at-risk amount increases. Passive-suspended losses are released by passive income or disposition. Section 461(l) excess is released as an NOL carryforward in future years subject to the 80% limit. Applying the limitations out of order or conflating the suspended amounts misstates when and how the loss becomes deductible. Document each limitation applied, the input, and the resulting suspended or deductible amount.

### Distinguish Noncorporate Owner Limits From Corporate Entity Limits

The loss limitation landscape for pass-through owners differs fundamentally from C corporations. Pass-through owners face basis, at-risk, passive, and Section 461(l) at the personal level. C corporations face the NOL 80% usage limit, Section 163(j) interest limitation, and (for closely-held or PSC) passive rules at the entity level, but not basis, at-risk, or Section 461(l). S corporations and partnerships themselves do not face entity-level loss limits (other than the entity-level Section 163(j) interest limitation, which applies before the loss flows to owners).

This distinction is essential for entity-choice and loss-analysis work. An agent analyzing a loss for an S corporation owner must apply the owner-level stack, not the C corporation stack. Conversely, a C corporation's losses are not subject to owner-level basis or at-risk (the corporation is a separate taxpayer). Confirm whether the loss is being analyzed at the entity level (C corporation) or the owner level (pass-through owner) before applying the limitation stack. The wrong stack produces dramatically different deductible amounts.

## Common Traps

### Treating The K-1 Loss As Immediately Deductible

The K-1 loss is the starting point, not the deductible amount. Basis, at-risk, passive, and Section 461(l) limitations can each suspend the loss at the owner level. Stopping at the K-1 overstates the deduction.

### Applying The Limitations Out Of Order

The limitations sequence (basis, at-risk, passive, excess business loss) matters because each operates on the prior result and produces different suspended-loss treatment. Applying them out of order misstates the deductible and suspended amounts.

### Conflating Basis And At-Risk

Basis and at-risk are similar but differ for nonrecourse debt. A partner may have basis for nonrecourse debt but not be at risk. Compute both separately.

### Forgetting S Corporation Shareholders Do Not Get Basis For Corporate Debt

S corporation shareholders get basis only for direct shareholder loans, not for the corporation's third-party debt. Partners get basis for their share of partnership liabilities. The entity type changes the basis computation.

### Missing The Passive Activity Rules For Passive Investors And Rentals

Passive activities (rentals, businesses where the owner does not materially participate) suspend losses even with basis and at risk. Confirm material participation and rental status before allowing the loss.

### Overlooking Section 461(l) Excess Business Loss At The Owner Level

The excess business loss limitation applies to non-corporate owners across all business activities combined, converting excess to an NOL carryforward. Missing it overstates the current deduction and misstates the carryforward.

### Applying C Corporation Loss Limits To A Pass-Through Owner

C corporations face the NOL 80% limit, Section 163(j), and entity-level passive rules. Pass-through owners face basis, at-risk, passive, and Section 461(l) at the personal level. Applying the wrong stack is a category error.

## Self-Check

- [ ] Has the pass-through loss been traced from the entity-level K-1 allocation through to the owner-level limitations, rather than treating the K-1 loss as immediately deductible?
- [ ] Has the basis limitation under Section 704(d) been applied first, with basis computed under the correct rules for the entity type (partnership liabilities for partners, direct shareholder loans for S corp shareholders)?
- [ ] Has the at-risk limitation under Section 465 been applied second, with the at-risk amount computed separately from basis (distinguishing recourse, nonrecourse, and qualified nonrecourse financing)?
- [ ] Has the passive activity limitation under Section 469 been applied third, with the owner's material participation status documented and rental activities identified as per se passive unless an exception applies?
- [ ] Has the excess business loss limitation under Section 461(l) been applied fourth at the owner level, with the aggregate business loss tested against the annual threshold (approximately $305,000 single, $610,000 MFJ for 2024)?
- [ ] Has the disallowed excess business loss been recognized as converting to an NOL carryforward at the owner level, subject to the 80% usage limit when later used?
- [ ] Have the limitations been sequenced correctly (basis, at-risk, passive, excess business loss), with each operating on the prior result and the suspended amounts tracked by their release conditions?
- [ ] Have the noncorporate owner-level limits been distinguished from C corporation entity-level limits, confirming the correct stack is applied for the entity type?
- [ ] Has the agent noted that pass-through loss rules depend on detailed owner facts and have complex regulations, this is not tax advice, and recommended consulting a qualified tax professional (CPA or tax attorney) for the specific owner and entity?
