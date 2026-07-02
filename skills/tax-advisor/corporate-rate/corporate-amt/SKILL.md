---
name: corporate_amt.md
description: Use when the agent is evaluating the corporate alternative minimum tax, determining whether a corporation is an applicable corporation under the 1 billion dollar revenue threshold, computing adjusted financial statement income, applying the 15 percent CAMT rate, or analyzing how book income minimum tax interacts with regular corporate tax and foreign-parented groups.
---

# Corporate Alternative Minimum Tax

The corporate alternative minimum tax (CAMT), enacted as part of the Inflation Reduction Act of 2022 and effective for tax years beginning after 2022, imposes a 15% minimum tax on the adjusted financial statement income (AFSI) of very large corporations. It is not a tax on most businesses. It applies only to "applicable corporations" whose three-year-average book income exceeds $1 billion, a threshold that excludes the overwhelming majority of companies. The judgment problem is that agents either panic and apply CAMT to companies far below the threshold, or dismiss CAMT entirely without testing whether a client is part of a controlled group whose aggregated income crosses $1 billion even though no single entity does.

Agents frequently miss that CAMT is computed on book income, not taxable income, so it reaches income that regular tax does not (because of deductions, credits, and accelerated depreciation), and that it operates as a floor rather than a standalone tax: the corporation pays the greater of regular tax or CAMT, with the excess treated as a minimum tax credit carryforward usable in years when regular tax exceeds CAMT. The foreign-parented corporation rules are a particular blind spot: a U.S. subsidiary of a foreign parent can be an applicable corporation based on the foreign parent's global financial statement income, even though the U.S. entity's own books are modest. Failing to identify applicable-corporation status, or miscomputing AFSI, can produce a seven-figure underpayment.

This skill covers CAMT applicability, the AFSI base, interaction with regular corporate tax, and foreign-parented group rules. It is not tax advice; CAMT regulations are evolving, the financial-statement-income definition has detailed exceptions, and a qualified tax professional (CPA or tax attorney) must be consulted for any corporation near the threshold.

## Core Rules

### Test Applicable Corporation Status Using Three-Year Average Book Income

An applicable corporation is one whose average annual adjusted financial statement income exceeds $1 billion for the three-consecutive-tax-year period ending with the current tax year. The test looks backward: a corporation becomes an applicable corporation in the year after it crosses the three-year-average threshold, and once it is an applicable corporation it generally remains one unless a statutory exception applies. A corporation that had AFSI of $1.2 billion, $900 million, and $1.1 billion over three years has an average of approximately $1.067 billion and is an applicable corporation in the following year.

The threshold is measured on financial statement income, not tax return income, which means a corporation can be an applicable corporation even if its taxable income is far lower due to deductions. Do not apply CAMT to a corporation with $200 million of revenue simply because it is "large"; the $1 billion three-year-average book income test is specific and must be computed from audited or applicable financial statements. Confirm which financial statement qualifies (audited financial statements filed with the SEC, or other applicable financial statements under the regulations) before concluding.

### Aggregate Income For Controlled Group And Foreign-Parented Rules

The $1 billion threshold is applied at the group level for affiliated corporations under common control, not entity-by-entity. A group of corporations under common control aggregates AFSI to test the threshold. A single corporation with $400 million of AFSI that is part of a group totaling $1.2 billion is an applicable corporation even though its standalone income is below $1 billion. The aggregation rules follow a more-than-50% ownership test, and the mechanics for allocating the $1 billion threshold among group members are detailed in the regulations.

For foreign-parented multinational groups, the rules are broader and more easily missed. A foreign-parented group with global financial statement income exceeding $1 billion can make its U.S. subsidiaries applicable corporations subject to CAMT, even if each U.S. subsidiary is small. The foreign-parented corporation rules generally require the aggregate U.S. AFSI of the group to exceed a separate threshold (statutorily $100 million) for CAMT to apply to the U.S. members. A U.S. subsidiary of a large foreign parent must be tested for CAMT even if the U.S. entity's own books are modest; assuming CAMT does not apply because the U.S. entity is small is a serious error.

### Compute Adjusted Financial Statement Income (AFSI)

AFSI is the starting point for CAMT, and it is not the same as net income on the financial statements. AFSI begins with the financial statement net income (or loss) and then makes specified adjustments. Key adjustments include eliminating federal and state income tax expense reflected in the financial statements, adding back the corporate equity reduction transaction income, adjusting for certain partnership income, and making other regulatory adjustments. The purpose of the adjustments is to bring the financial statement number closer to an economic-income concept for minimum-tax purposes.

The most consequential adjustment is the elimination of income tax expense. Financial statement net income is stated after deducting income tax expense, but for CAMT the tax expense is added back because CAMT is itself a tax. A corporation with $1.5 billion of pre-tax book income and $300 million of tax expense has financial statement net income of $1.2 billion; AFSI adds back the tax expense, producing approximately $1.5 billion. Using the financial statement net income number without the add-back understates AFSI and can cause a corporation to fall below the threshold incorrectly.

### Apply The 15% Rate As A Floor Against Regular Tax

CAMT is not a tax that is paid in addition to regular corporate tax. It is a minimum: the corporation pays the greater of (a) the regular corporate tax (21% of taxable income, less credits subject to limitations), or (b) 15% of AFSI minus certain book-tax alternative amount. If regular tax exceeds the CAMT tentative minimum tax, the corporation pays regular tax and no CAMT is due. If the CAMT amount exceeds regular tax, the corporation pays the excess as CAMT liability.

The excess of CAMT over regular tax is not lost permanently; it becomes a minimum tax credit (MTC) carryforward that can be used to offset regular tax in future years when regular tax exceeds CAMT, but generally only to the extent of that excess (the credit cannot reduce tax below the CAMT floor in future years). This means CAMT is, in effect, a prepayment of tax for profitable corporations whose regular tax will eventually exceed the minimum, but it can be a real cost increase for corporations whose book income consistently exceeds taxable income due to deductions and credits.

### Identify The Interaction With Tax Credits And Accelerated Depreciation

CAMT often bites precisely because of the deductions and credits that reduce regular tax below the book-income floor. A corporation claiming large research and development credits, accelerated or bonus depreciation, or the clean energy credits enacted under the Inflation Reduction Act may have regular tax near zero while AFSI remains high, triggering CAMT. The general business credit limitation against regular tax does not directly limit credits against CAMT in the same way, but the CAMT computation has its own interaction rules for credits.

The practical consequence is that a corporation planning heavy reliance on tax credits must model whether CAMT limits the benefit. A corporation expecting to use $500 million of clean energy credits to eliminate regular tax may still owe 15% of AFSI under CAMT, partially eroding the credit benefit. Credit-driven tax planning for large corporations that ignores CAMT can overstate the credit benefit by a wide margin. Always test whether the corporation is an applicable corporation before assuming credits flow fully to the bottom line.

### Distinguish CAMT From The Repealed Pre-2018 Corporate AMT

The CAMT enacted in 2022 is a different tax from the old corporate alternative minimum tax that was repealed by the TCJA effective for tax years beginning after 2017. The old corporate AMT was based on a modified taxable income computation with preference items (accelerated depreciation, intangible drilling costs, tax-exempt interest on private activity bonds) and a 20% rate, and it affected a much broader range of corporations. The new CAMT is based on book income (AFSI) and applies only to applicable corporations above the $1 billion threshold.

Agents familiar with the old AMT may incorrectly apply its concepts (preference items, the AMT credit, the exemption amount) to the new CAMT, which has no exemption amount and no preference-item add-back structure. The new CAMT has no exemption phaseout and no preference-item list; it is a straightforward 15% of AFSI floor. Do not import the old AMT mechanics into CAMT analysis.

## Common Traps

### Applying CAMT To Corporations Below The $1 Billion Threshold

CAMT applies only to applicable corporations with three-year-average AFSI exceeding $1 billion. Applying a 15% book-income minimum tax to a $300 million corporation is a serious overstatement. Always test the three-year average threshold before concluding CAMT applies.

### Missing Controlled Group Aggregation

The $1 billion threshold is tested at the group level for commonly controlled corporations. A corporation with standalone AFSI of $400 million that is part of a $1.5 billion group is an applicable corporation. Testing each entity separately misses the aggregation rule.

### Forgetting Foreign-Parented U.S. Subsidiaries

A U.S. subsidiary of a foreign parent with global book income above $1 billion can be an applicable corporation subject to CAMT, even if the U.S. entity is small. Assuming CAMT does not apply because the U.S. entity is modest ignores the foreign-parented group rules.

### Using Financial Statement Net Income Without The Tax Add-Back

AFSI adds back income tax expense to financial statement net income. Using the net income figure directly understates AFSI and can drop a corporation below the threshold incorrectly. Always make the tax-expense add-back.

### Treating CAMT As A Tax On Top Of Regular Tax

CAMT is a floor, not an additional tax. The corporation pays the greater of regular tax or the CAMT amount, not both. Treating CAMT as additive double-counts the liability.

### Importing The Old Pre-2018 AMT Mechanics

The repealed corporate AMT used preference items, an exemption amount, and a 20% rate on modified taxable income. The new CAMT uses AFSI, no exemption, and a 15% floor. Do not apply old AMT concepts to CAMT.

### Ignoring CAMT When Modeling Credit Benefits

Large corporations relying on R&D, depreciation, or clean energy credits may find that CAMT limits the credit benefit because the book-income floor still applies. Credit planning that ignores CAMT overstates the tax savings.

## Self-Check

- [ ] Has applicable corporation status been tested using the three-year-average AFSI exceeding $1 billion threshold, computed from the appropriate financial statements?
- [ ] Has controlled group aggregation been applied so that commonly controlled corporations are tested at the group level, not entity-by-entity?
- [ ] Has foreign-parented group status been checked, so that a U.S. subsidiary of a large foreign parent is tested for CAMT even if the U.S. entity's own books are modest?
- [ ] Has AFSI been computed with the income tax expense add-back and other required adjustments, rather than using financial statement net income directly?
- [ ] Has CAMT been applied as a floor (the greater of regular tax or 15% of AFSI), not as an additional tax on top of regular tax?
- [ ] Has the minimum tax credit carryforward been recognized for CAMT paid in excess of regular tax, with its future usability noted as limited to years when regular tax exceeds CAMT?
- [ ] Has the interaction with tax credits (R&D, depreciation, clean energy credits) been modeled so that CAMT's potential erosion of credit benefits is reflected?
- [ ] Has the CAMT been distinguished from the repealed pre-2018 corporate AMT, with old preference-item and exemption mechanics confirmed as not applicable?
- [ ] Has the agent noted that CAMT regulations are evolving, this is not tax advice, and recommended consulting a qualified tax professional (CPA or tax attorney) for any corporation near the threshold?
