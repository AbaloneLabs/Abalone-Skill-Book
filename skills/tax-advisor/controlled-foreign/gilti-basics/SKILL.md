---
name: gilti_basics.md
description: Use when the agent is analyzing Global Intangible Low-Taxed Income from controlled foreign corporations, computing the GILTI inclusion under Section 951A, applying the 50 percent deduction and the high-tax exception, or determining the GILTI tax burden for US corporate and individual CFC shareholders.
---

# GILTI Basics

Global Intangible Low-Taxed Income (GILTI), enacted in the Tax Cuts and Jobs Act of 2017, is an anti-deferral regime that requires U.S. shareholders of a controlled foreign corporation (CFC) to include their share of the CFC's income that exceeds a 10% return on tangible business assets. GILTI targets income that is lightly taxed abroad, ensuring that foreign earnings face a minimum U.S. tax. The mechanics differ significantly for corporate shareholders (who get a 50% deduction and the foreign tax credit) and individual shareholders (who generally do not). An agent who applies GILTI without understanding the computation, the deduction, and the corporate-versus-individual distinction will misstate the tax burden.

Agents commonly miss that GILTI applies to income above a 10% return on tangible assets, that corporate shareholders get a 50% deduction (effectively a 10.5% tax rate before FTC), that individual shareholders generally do not get the deduction or the FTC, and that a high-tax exception may exclude income taxed above 18.9% abroad. The harm is a shareholder who underestimates the GILTI tax or fails to claim available benefits.

This skill covers GILTI basics under U.S. federal tax law. It is not tax advice; GILTI is complex and depends on the CFC's income and assets, and a qualified tax professional must be consulted for actual situations.

## Core Rules

### Understand The GILTI Computation

GILTI is the U.S. shareholder's share of the CFC's income that exceeds a 10% return on the CFC's qualified business asset investment (QBAI). The computation is: GILTI equals the shareholder's tested income minus the shareholder's tested loss (if any) minus the 10% QBAI return. Tested income is the CFC's gross income minus certain exclusions (Subpart F income, effectively connected income, high-tax exception income) minus deductions allocable to the income. QBAI is the CFC's tangible business assets (property, plant, equipment) used in the trade or business, measured at adjusted basis.

Compute the GILTI. Determine the CFC's tested income (gross income minus exclusions minus deductions). Determine the CFC's QBAI (tangible business assets at adjusted basis). Compute the 10% QBAI return (10% of QBAI). The GILTI is the U.S. shareholder's share of the tested income minus the 10% QBAI return (if positive). If the tested income is below the 10% QBAI return, there is no GILTI. The 10% QBAI return is a deemed return on tangible assets; income above this return is treated as intangible income (GILTI) subject to the minimum tax. Track the tested income, the QBAI, and the GILTI computation.

### Apply The 50% Deduction For Corporate Shareholders

Corporate shareholders (C corporations) get a 50% deduction (Section 250) on the GILTI inclusion, effectively reducing the tax rate. For 2018-2025, the deduction is 50% (reducing the GILTI rate to 10.5% before the FTC). For 2026 and later, the deduction is 37.5% (reducing the rate to 13.125%). The deduction is available only to corporate shareholders; individual shareholders (individuals, partnerships, S corporations) generally do not get the deduction. The deduction is a key benefit that makes GILTI manageable for corporate shareholders.

Apply the 50% deduction for corporate shareholders. A C corporation that is a U.S. shareholder of a CFC claims a 50% deduction (Section 250) on the GILTI inclusion. The deduction reduces the taxable GILTI to 50% of the inclusion, resulting in a 10.5% effective tax rate (21% corporate rate multiplied by 50%) before the FTC. For 2026 and later, the deduction is 37.5%, resulting in a 13.125% rate. The deduction is available only to C corporations. Individual shareholders, partnerships, and S corporations generally do not get the deduction, resulting in a much higher effective rate on GILTI (up to 37% for individuals, without the FTC). Confirm the shareholder's entity type and apply the deduction if eligible.

### Apply The Foreign Tax Credit For GILTI

Corporate shareholders can claim a foreign tax credit for the foreign taxes paid by the CFC on the GILTI income, but the FTC for GILTI is subject to special rules. The GILTI FTC is computed in a separate category (the GILTI category), and only 80% of the foreign taxes are creditable (a haircut). The GILTI FTC cannot be carried back or carried forward (use it or lose it in the current year). The combination of the 50% deduction and the 80% FTC makes GILTI a minimum tax of 10.5% for corporate shareholders in countries with tax rates at or above 13.125%.

Apply the GILTI FTC for corporate shareholders. The foreign taxes paid by the CFC on the GILTI income are creditable, but only 80% (a 20% haircut). The GILTI FTC is computed in a separate category (not the general or passive category). The GILTI FTC cannot be carried back or carried forward; it must be used in the current year or lost. For a corporate shareholder, the combination of the 50% deduction (10.5% U.S. rate) and the 80% FTC means that if the CFC's foreign tax rate is at least 13.125%, the GILTI FTC offsets the U.S. tax entirely (no additional U.S. tax). If the foreign rate is below 13.125%, the corporate shareholder pays a residual U.S. tax. Track the GILTI FTC computation and the 80% haircut.

### Handle The Individual Shareholder Disadvantage

Individual shareholders (individuals, partnerships, S corporations) generally do not get the 50% Section 250 deduction and do not get the GILTI FTC. This means the individual is taxed on the full GILTI inclusion at their ordinary income rate (up to 37%), without the deduction or the credit. The individual shareholder disadvantage makes GILTI much more burdensome for individuals than for corporations. An individual holding CFC shares should consider an entity-level election (a Section 962 election) to be taxed as a corporation, which may allow the 50% deduction and the FTC.

Handle the individual shareholder disadvantage. An individual shareholder of a CFC is taxed on the full GILTI inclusion at their ordinary income rate (up to 37%), without the 50% deduction and generally without the GILTI FTC. This can result in a GILTI tax rate of up to 37% for individuals, compared to 10.5% for corporations. An individual shareholder may make a Section 962 election to be taxed as a corporation on the CFC income, which may allow the 50% deduction and the GILTI FTC, reducing the effective rate. The Section 962 election has its own complexities (potential double taxation on distribution) and should be modeled carefully. Advise individual shareholders to consult a tax professional about the Section 962 election.

### Apply The High-Tax Exception

The high-tax exception (HTE) allows a U.S. shareholder to exclude from GILTI (and Subpart F) the CFC's income that is subject to a high foreign tax rate. Under the current HTE (effective 2020), the CFC can elect to exclude income that is subject to an effective foreign tax rate of 18.9% or higher (the 90% of 21% corporate rate). The election is made annually and applies to the CFC (not the shareholder). The HTE simplifies compliance for CFCs in high-tax jurisdictions, as the excluded income is not subject to GILTI or Subpart F.

Apply the high-tax exception. If the CFC's income is subject to an effective foreign tax rate of 18.9% or higher, the CFC can elect the HTE to exclude that income from GILTI (and Subpart F). The election is made annually on Form 8992 (for the shareholder) or Form 8993 (for the CFC). The HTE is beneficial for CFCs in high-tax jurisdictions (such as European countries with rates above 18.9%), as the excluded income is not subject to the U.S. minimum tax. The election is binding for the year and cannot be changed without IRS consent. Determine whether the CFC's income qualifies for the HTE and model the benefit.

### Report GILTI On Form 8992 And Form 5471

GILTI is reported on Form 8992 (U.S. Shareholder Calculation of Global Intangible Low-Taxed Income (GILTI)) for the U.S. shareholder, and Form 5471 (Information Return of U.S. Persons With Respect to Certain Foreign Corporations) for the CFC. Form 8992 computes the shareholder's GILTI inclusion, the 50% deduction (for corporations), and the GILTI FTC. Form 5471 reports the CFC's financial information, including the tested income and the QBAI. The forms are required for each CFC and each U.S. shareholder.

Ensure Form 8992 and Form 5471 are filed. Form 8992 computes the U.S. shareholder's GILTI inclusion, the 50% deduction (for corporate shareholders), and the GILTI FTC (with the 80% haircut). Form 5471 reports the CFC's financial information, including the tested income, the QBAI, and the GILTI computation. Failure to file can trigger significant penalties and extend the statute of limitations for the entire return. Track the filing requirement and ensure the forms are complete and timely. The GILTI computation is detailed and requires the CFC's income, deductions, QBAI, and foreign taxes.

## Common Traps

### Treating CFC Income As Deferred Until Distributed

GILTI requires current inclusion of the U.S. shareholder's share of the CFC's income above the 10% QBAI return, regardless of distributions. Treating the income as deferred understates the U.S. tax.

### Forgetting The 50% Deduction Is Only For Corporations

The 50% Section 250 deduction is available only to C corporations. Individual shareholders, partnerships, and S corporations generally do not get the deduction, resulting in a much higher GILTI rate.

### Overlooking The 80% GILTI FTC Haircut

The GILTI FTC allows only 80% of the foreign taxes (a 20% haircut). Applying the full foreign tax overstates the credit and understates the U.S. tax.

### Ignoring The Individual Shareholder Disadvantage

Individual shareholders face GILTI rates up to 37% without the deduction or the FTC. A Section 962 election may help, but it has complexities. Advise individuals accordingly.

### Overlooking The High-Tax Exception

The HTE allows exclusion of income taxed at 18.9% or higher abroad. CFCs in high-tax jurisdictions may benefit. Model the HTE before defaulting to the GILTI inclusion.

### Not Filing Form 8992 And Form 5471

GILTI is reported on Form 8992 and Form 5471. Failure to file triggers significant penalties and extends the statute of limitations for the entire return.

### Forgetting The GILTI FTC Cannot Be Carried Over

The GILTI FTC cannot be carried back or carried forward. It must be used in the current year or lost. Track the GILTI FTC carefully.

## Self-Check

- [ ] Has the GILTI been computed as the U.S. shareholder's share of the CFC's tested income minus the 10% QBAI return, with the tested income and QBAI determined correctly?
- [ ] Has the 50% Section 250 deduction been applied for corporate shareholders, reducing the effective GILTI rate to 10.5% (or 13.125% for 2026 and later)?
- [ ] Has the GILTI FTC been computed with the 80% haircut, in the separate GILTI category, with no carryback or carryforward?
- [ ] Has the individual shareholder disadvantage been addressed, with the Section 962 election modeled if beneficial?
- [ ] Has the high-tax exception been considered for CFC income taxed at 18.9% or higher abroad, with the election made if beneficial?
- [ ] Is the GILTI reported on Form 8992 (shareholder) and Form 5471 (CFC), with the tested income, QBAI, deduction, and FTC computed?
- [ ] Has the interaction with Subpart F been coordinated, ensuring income is either Subpart F or GILTI, not both?
- [ ] Is the GILTI analysis documented with the tested income, QBAI, inclusion, deduction, FTC, HTE, and reporting forms?
- [ ] Has the agent noted that this is general U.S. federal GILTI information, not tax advice, and recommended consulting a qualified tax professional for the specific situation?
