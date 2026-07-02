---
name: kyc_customer_risk_rating_and_ongoing_due_diligence.md
description: Use when the agent is calibrating customer risk-rating models, deciding simplified versus enhanced due diligence triggers, defining periodic review cycles, or setting trigger-based review events for ongoing KYC and CDD refresh across the customer lifecycle.
---

# KYC Customer Risk Rating And Ongoing Due Diligence

A risk-based KYC program is only as effective as its risk-rating model and its ongoing due diligence cycle. Under FATF Recommendation 1, the BSA, the EU AMLD, and equivalent regimes, obliged entities must apply a risk-based approach that segments customers by risk, calibrates the depth of due diligence accordingly, and refreshes customer information on a risk-sensitive basis. The central judgment problem is that a risk rating is not a label. It is a control that determines how much diligence is applied, how often the relationship is reviewed, and what monitoring intensity is appropriate. When the model is miscalibrated, the institution either overworks low-risk relationships or, more dangerously, under-protects high-risk ones. And when ongoing CDD is treated as a calendar exercise, material changes that happen mid-cycle are missed.

Use this skill before designing or recalibrating a customer risk-rating model, defining simplified versus enhanced due diligence triggers, setting periodic review cadence, or building trigger-based review processes. The goal is to make the agent think about risk-factor selection, model discrimination, EDD triggers, and the difference between calendar-driven and event-driven review. A flat or uncalibrated risk model defeats the purpose of a risk-based program.

This skill addresses risk-based obligations that are jurisdiction-specific. Risk factors, thresholds, and EDD requirements differ across FATF member states and national regimes. Always confirm the applicable national law and regulator guidance, and consult qualified AML or legal counsel for specific risk-model design decisions.

## Core Rules

### Build The Risk Model Around Meaningful Risk Factors

A risk rating should reflect the actual money laundering, terrorist financing, and sanctions risk presented by the customer. The model must use risk factors that are relevant to the institution's products, customers, and geographies.

Core risk-factor categories include:

- customer type, such as individual, private company, public listed company, trust, nonprofit, financial institution, or money service business;
- product or service used, including cash-intensive products, private banking, correspondent banking, trade finance, or virtual assets;
- geography of residence, incorporation, and activity, including high-risk jurisdictions and sanctions exposure;
- delivery channel, such as face-to-face, non-face-to-face, agent, or digital;
- expected transaction volume and value;
- occupation or business sector, particularly cash-intensive or high-risk sectors;
- exposure to cash, virtual assets, or high-risk products;
- PEP status;
- adverse media or sanctions history;
- complexity of ownership or control structure.

Each factor should have a documented rationale tied to a laundering or financing typology. Factors that do not discriminate risk add noise without insight.

### Calibrate The Model So Risk Levels Discriminate

The most common model failure is that nearly every customer lands in the same risk band, usually medium. When the model cannot distinguish high from low risk, the institution cannot focus resources where they matter.

Calibration checks include:

- distribution analysis showing the percentage of customers in each risk tier;
- back-testing the rating against known suspicious cases and SAR filings;
- sensitivity testing of individual risk factors to ensure each moves the rating meaningfully;
- periodic re-weighting as typologies and customer behavior evolve;
- override tracking to detect systematic rating suppression by business units.

If the model assigns 80 percent of customers to medium risk, it is not discriminating. The institution should re-examine its factor weights and thresholds until the distribution reflects genuine risk variation.

### Apply Simplified Due Diligence Only To Demonstrable Low Risk

Simplified due diligence (SDD) reduces the depth of CDD and is appropriate only where the institution can demonstrate low inherent risk. FATF Recommendation 10 and national regimes permit SDD for relationships such as listed companies subject to disclosure obligations, financial institutions subject to AML obligations, and low-value products with limited laundering potential.

SDD conditions that may be appropriate:

- customer is a listed company on a recognized exchange with disclosure obligations;
- customer is a regulated financial institution in a FATF-compliant jurisdiction;
- product is a low-value account with capped transaction limits;
- customer and activity are domestic and low-risk with stable profile.

SDD must never be used simply to reduce workload. Each SDD decision should be documented with the low-risk justification, and the institution should periodically re-confirm that SDD customers remain low risk.

### Trigger Enhanced Due Diligence For Elevated Risk

Enhanced due diligence (EDD) increases the depth and seniority of CDD for higher-risk relationships. FATF, the BSA, and the EU AMLD require EDD for defined high-risk situations.

EDD triggers include:

- politically exposed persons, domestic and foreign;
- customers or beneficial owners in high-risk jurisdictions identified by FATF or national authorities;
- complex or opaque ownership structures;
- correspondent banking relationships, particularly cross-border and nested;
- high-value or anonymous products such as private banking;
- customers with adverse media or prior sanctions exposure;
- cash-intensive businesses or trade-based money laundering exposure;
- non-resident customers from high-risk geographies.

EDD measures should include senior management approval, source of funds and source of wealth establishment, enhanced ongoing monitoring, additional identification corroboration, and more frequent periodic review. Documenting that EDD was considered but not applied is as important as documenting its application.

### Set Risk-Based Periodic Review Cycles

Ongoing CDD requires periodic review whose frequency matches risk. A flat review cycle applied to all customers wastes effort on low risk and under-reviews high risk.

Typical risk-based cadence:

- high-risk customers: annual review or more frequent;
- medium-risk customers: every two to three years;
- low-risk customers: every three to five years or trigger-based.

Reviews should re-confirm identity, beneficial ownership, expected activity, occupation, and risk rating. The review is not a checkbox refresh. It is a re-assessment of whether the original risk rating still holds and whether the relationship has changed in ways that require EDD or exit.

### Build Trigger-Based Review For Material Changes

Calendar-driven reviews alone miss material changes that happen mid-cycle. The institution must have trigger events that force an immediate CDD refresh regardless of the next scheduled review.

Trigger events include:

- material change in customer profile, ownership, or expected activity;
- new beneficial owner added to a legal entity customer;
- activity spike or deviation from expected behavior flagged by monitoring;
- new adverse media, sanctions hit, or PEP status change;
- change in jurisdiction of residence or incorporation;
- law enforcement inquiry or subpoena related to the customer;
- doubt over the reliability of previously obtained identification data.

Trigger-based review closes the gap between periodic cycles. Without it, a customer can change fundamentally between scheduled reviews without any compliance response.

### Document The Rating And Review Decision

Each risk-rating assignment and each periodic or trigger-based review should be documented with the factors considered, the rating assigned, the diligence level applied, and the approver. Overrides, downgrades, and SDD decisions require documented rationale. An examiner must be able to reconstruct why a customer was rated as they were and what diligence was applied.

## Common Traps

### Defaulting Everyone To Medium Risk

When most customers land in one band, the model cannot focus resources. Calibrate so that genuine high-risk relationships are elevated and genuine low-risk relationships are simplified.

### SDD Used To Reduce Workload

SDD without a documented low-risk justification is an examination finding. Simplified measures must be earned by demonstrable low inherent risk.

### EDD Skipped For Convenience

EDD is slower and harder, creating pressure to downgrade. Downgrading a high-risk relationship to standard CDD without rationale is a serious deficiency.

### Calendar-Only Review Missing Mid-Cycle Changes

A customer who adds a new beneficial owner or spikes in activity between scheduled reviews goes unaddressed. Trigger events must force a refresh.

### Stale Expected Activity Never Recalibrated

Expected activity set at onboarding becomes outdated. If monitoring is tuned to stale expectations, genuine risk is masked.

### Overrides Not Tracked

When business units systematically override risk ratings downward, the model is defeated. Override rates and patterns must be monitored.

### Review As Checkbox Refresh

A periodic review that re-confirms the same data without re-assessing risk is not meaningful CDD. The review must test whether the rating still holds.

## Self-Check

- Does the risk-rating model use meaningful risk factors tied to laundering and financing typologies relevant to the institution?
- Is the model calibrated so that risk levels discriminate, rather than defaulting most customers to one band?
- Is simplified due diligence applied only to demonstrably low-risk customers with documented justification?
- Are EDD triggers defined for PEPs, high-risk jurisdictions, complex ownership, correspondent banking, and other elevated-risk situations?
- Are periodic review cycles risk-based, with high-risk customers reviewed at least annually?
- Are trigger-based reviews in place for material changes such as ownership change, activity deviation, sanctions hits, or adverse media?
- Are risk-rating overrides tracked and monitored for systematic suppression?
- Is each rating and review documented with factors, rating, diligence level, rationale, and approver?
- Is expected activity recalibrated during review rather than carried forward unchanged?
- Is the risk-model design confirmed against the applicable national law and regulator guidance rather than a generic standard?