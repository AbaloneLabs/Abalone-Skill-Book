---
name: aml_inherent_risk_assessment.md
description: Use when the agent is assessing inherent AML risk across customers, products, geographies, and channels, calibrating a risk scoring methodology, defining risk appetite, or aligning risk ratings with controls and monitoring coverage.
---

# AML Inherent Risk Assessment

An AML inherent risk assessment identifies the money laundering and terrorist financing exposure an institution faces before considering controls. FATF Recommendation 1, the BSA, the US AML Act, FinCEN expectations, and EU AMLD all require a risk-based approach grounded in a documented, periodically refreshed risk assessment. The assessment drives customer risk ratings, product approvals, monitoring coverage, resourcing, and risk appetite. A weak or generic assessment produces a program that looks compliant but is blind to its own highest exposures.

Use this skill before building or refreshing an AML risk assessment, designing the risk scoring methodology, calibrating customer risk ratings, or aligning the assessment with controls. The goal is to make the agent think about the inherent risk factors, the scoring logic, the risk appetite, and the calibration that makes ratings meaningful. A risk assessment that rates everything medium is not risk-based.

## Core Rules

### Assess Risk Across The Four Inherent Dimensions

Inherent AML risk is driven by who the institution serves, what it offers, where it operates, and how it delivers. Assess each dimension explicitly.

- customer risk: types of customers (individuals, private companies, public companies, trusts, NGOs, financial institutions, PEPs, high-risk businesses), ownership complexity, occupation, and behavior;
- product and service risk: cash-intensive products, private banking, correspondent banking, trade finance, virtual assets, prepaid cards, anonymous instruments, and high-value services;
- geographic risk: customer location, counterparty location, activity geography, and exposure to high-risk or sanctioned jurisdictions;
- delivery channel risk: face-to-face, non-face-to-face, digital onboarding, agents, intermediaries, and third-party distribution.

Each dimension should be scored, weighted, and combined into an overall inherent risk rating. Document the rationale for each factor and weight.

### Use A Defensible Scoring Methodology

Scoring must be transparent, reproducible, and explainable. Examiners will ask how a rating was derived.

Methodology elements:

- defined risk factors per dimension with clear definitions;
- scoring scale (e.g., low, medium, high, or numeric) with criteria for each band;
- weighting that reflects the institution's actual exposure;
- combination logic (additive, multiplicative, or matrix);
- treatment of overrides with documented rationale;
- data sources and refresh frequency;
- validation and back-testing against SARs, alerts, and losses.

Avoid opaque models. The methodology should allow any reviewer to reproduce the rating from the inputs.

### Calibrate Ratings So They Discriminate

A common failure is ratings collapse, where most customers or products land in a single band. If 90 percent of customers are medium risk, the rating adds no value.

Calibration practices:

- set thresholds that produce a meaningful distribution across bands;
- use data on actual activity to validate band boundaries;
- ensure high-risk triggers (PEP, high-risk jurisdiction, complex ownership) reliably elevate the rating;
- avoid defaulting unknown or incomplete data to low risk;
- review the distribution periodically and adjust.

The rating model should be able to distinguish a low-risk salaried retail customer from a high-risk cash-intensive business with offshore ties.

### Define And Document Risk Appetite

Risk appetite states how much inherent risk the institution is willing to accept, and in what segments. It drives product approvals, customer acceptance, and de-risking decisions.

Risk appetite should address:

- which customer types, products, geographies, and channels are within appetite;
- which are restricted, require senior approval, or are prohibited;
- concentration limits for high-risk segments;
- thresholds that trigger escalation or exit;
- the link between appetite and the business strategy.

Risk appetite should be approved at board or senior management level, documented, and communicated. It should not be an aspirational statement disconnected from decisions.

### Align The Assessment With Controls And Monitoring

Inherent risk is only half the picture. Residual risk is inherent risk after controls. The assessment should map inherent risk to the controls expected to mitigate it, and to the monitoring coverage that should detect residual exposure.

Alignment checks:

- do high-risk customer segments receive EDD and enhanced monitoring?
- do high-risk products have appropriate scenarios and approval gates?
- do high-risk geographies trigger screening and enhanced review?
- are control owners, evidence, and testing defined for each high-risk area?
- are gaps between inherent risk and control coverage identified and remediated?

A risk assessment that does not connect to controls is academic. It must drive action.

### Refresh On A Cycle And On Triggers

Risk assessments must be periodically refreshed and updated on material change.

Refresh triggers:

- new product, service, channel, or geography;
- significant customer mix change;
- regulatory or typology developments;
- enforcement actions or examination findings;
- internal incidents, SAR spikes, or control failures;
- mergers, acquisitions, or divestitures.

Document the refresh cycle, the triggers, and the resulting changes. An assessment that has not been updated in years is stale and indefensible.

### Use The Assessment To Prioritize Resources

The assessment should drive where compliance, monitoring, and investigative resources are concentrated. High inherent risk areas deserve more attention, more scenarios, more frequent review, and stronger controls. Resource allocation that ignores the assessment undermines the risk-based premise.

## Common Traps

### Generic Factor Lists Without Weighting

Listing risk factors without weights or combination logic produces ratings that cannot be explained or reproduced.

### Ratings Collapse Into One Band

If most customers or products are medium risk, the model is not discriminating and resources cannot be targeted.

### Defaulting Unknown Data To Low Risk

Missing or incomplete data should not produce a low rating. It should be treated as elevated risk or an information gap.

### Disconnecting Appetite From Decisions

A risk appetite statement that does not influence product, customer, or channel decisions is decorative.

### Assessing Inherent Risk Without Mapping Controls

Inherent risk alone does not tell the institution whether it is protected. The residual view requires control mapping.

### Stale Assessments

A risk assessment that is never refreshed misses new products, geographies, and typologies and loses credibility.

### Ignoring Concentration

Even individually moderate risks can become material through concentration in one customer type, geography, or product.

## Self-Check

- Are customer, product, geographic, and delivery channel risk each assessed explicitly with documented factors?
- Is the scoring methodology transparent, reproducible, and explainable, with defined factors, weights, and combination logic?
- Do ratings discriminate across bands rather than collapsing into a single medium band?
- Is risk appetite documented, board-approved, and connected to product, customer, and channel decisions?
- Is inherent risk mapped to controls, monitoring coverage, and residual risk?
- Are refresh cycles and trigger events defined and documented?
- Does the assessment drive resource allocation toward high-risk areas?
- Are overrides documented with rationale and approver?
- Are high-risk triggers such as PEP status, high-risk jurisdiction, and complex ownership reliably elevating ratings?
- Is the methodology validated against SARs, alerts, and losses?
