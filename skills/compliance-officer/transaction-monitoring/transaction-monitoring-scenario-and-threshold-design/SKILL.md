---
name: transaction_monitoring_scenario_and_threshold_design.md
description: Use when the agent is designing transaction monitoring scenarios, calibrating detection thresholds, mapping typologies to detection rules, segmenting monitoring by customer and product risk, or building risk-based coverage for money laundering, terrorist financing, sanctions evasion, and fraud typologies.
---

# Transaction Monitoring Scenario And Threshold Design

Transaction monitoring is only as effective as the scenarios it runs and the thresholds that determine what becomes an alert. Under FATF Recommendation 20, the BSA, the EU AMLD, FinCEN expectations, and equivalent national regimes, obliged entities must maintain risk-based monitoring that detects activity consistent with known laundering, terrorist financing, sanctions evasion, and fraud typologies. The central judgment problem is that a monitoring system is not a set of generic anomaly detectors. It is a typology-mapped detection architecture where each scenario targets a specific method of illicit finance, each threshold is calibrated against real data, and gaps in coverage are themselves a compliance deficiency. A system that produces many alerts but cannot explain which typology each scenario targets, or why the threshold is set where it is, is not defensible.

Use this skill before designing monitoring scenarios, calibrating thresholds, mapping typologies to rules, or assessing coverage gaps. The goal is to make the agent think about typology relevance, threshold calibration, segmentation, and the decisions that are easy to make too casually. A monitoring system tuned only to reduce workload, or built from generic templates without typology mapping, will miss the very activity it exists to detect.

This skill addresses jurisdiction-specific obligations. Monitoring requirements, typology expectations, and validation standards differ across FATF member states and national regimes. Always confirm the applicable national law and regulator guidance, and consult qualified AML or model risk counsel for specific scenario and threshold design decisions.

## Core Rules

### Map Every Scenario To A Specific Typology

Each monitoring scenario must target a specific money laundering, terrorist financing, sanctions evasion, or fraud typology. Generic anomaly detection without a typology basis generates noise without insight and cannot be defended as risk-based monitoring.

Typology families that scenarios should cover:

- structuring and smurfing below currency or wire transfer reporting thresholds;
- rapid pass-through of funds with no apparent economic purpose;
- activity inconsistent with the customer's expected profile or stated business;
- trade-based money laundering indicators, including over- and under-invoicing and phantom shipments;
- virtual asset conversion and crypto-to-fiat layering;
- funnel account and third-party aggregation patterns;
- sudden dormancy break or in-and-out activity;
- large cash deposits inconsistent with occupation or business type;
- internal fraud, embezzlement, and unauthorized transaction indicators;
- sanctions and proliferation financing indicators, including counterparty and geography exposure;
- terrorist financing indicators, including small-value collection and routing to high-risk geographies;
- human trafficking and modern slavery indicators in payment patterns.

Each scenario should have a documented typology rationale, a defined data feed, detection logic, and an expected outcome. A scenario without a typology basis should be removed or re-designed.

### Calibrate Thresholds Against Real Data

Thresholds determine what becomes an alert. Thresholds set too low flood analysts with false positives; set too high, they miss genuine risk. Calibration must be data-driven, not arbitrary.

Calibration inputs:

- historical transaction distribution by customer segment, product, and channel;
- expected activity defined in CDD;
- population coverage and analyst alert capacity;
- back-testing against known suspicious cases and confirmed SARs;
- segmentation so that thresholds reflect the behavior of comparable customer groups;
- seasonality and business cycle adjustments;
- counterparty and geography overlays for high-risk exposure.

Document the methodology, the data sources, the segmentation logic, and the resulting thresholds. Thresholds that were never calibrated, or that were set by vendor default and never reviewed, are a common examination finding.

### Segment Monitoring By Customer And Product Risk

A single threshold applied to all customers treats a retail checking account the same as a private banking relationship. Monitoring must be segmented so that thresholds and scenarios reflect the risk and behavior of comparable groups.

Segmentation dimensions:

- customer type, such as retail, small business, corporate, private banking, correspondent banking;
- product type, such as checking, wire, trade finance, virtual asset, prepaid card;
- delivery channel, such as branch, digital, agent, correspondent;
- geography of customer and counterparty;
- risk rating from CDD.

Segmentation allows thresholds to be tighter for high-risk segments and looser for low-risk segments, improving detection while managing false positives. Without segmentation, the system either over-alerts on low-risk volume or under-detects on high-risk activity.

### Design For Coverage And Identify Gaps

Monitoring coverage is not only about the scenarios that exist. It is also about the typologies that are not covered. The institution should periodically assess coverage against its risk assessment and identify gaps.

Coverage assessment:

- map each typology in the risk assessment to the scenario or scenarios that detect it;
- identify typologies with no corresponding scenario;
- assess whether data feeds are complete enough to support the intended detection logic;
- review whether new products, channels, or customer types have monitoring coverage;
- confirm that cross-product and cross-account activity is visible to monitoring, not siloed.

A coverage gap is a compliance deficiency. If the risk assessment identifies trade-based money laundering as a key risk but no scenario detects trade-based indicators, the monitoring program is incomplete regardless of how many other alerts it produces.

### Govern Threshold And Scenario Changes

Changes to scenarios and thresholds must be governed, tested, and documented. Ungoverned changes, particularly silent threshold widening to reduce alert volume, are a serious deficiency.

Change governance:

- document the business case for each change;
- test the change against historical data before deployment;
- obtain approval from the appropriate authority, such as the BSA officer or model risk committee;
- implement in change-controlled environments with version tracking;
- monitor post-change alert volume, conversion, and detection metrics;
- retain records of the change rationale, testing, and approval.

Tuning to reduce false positives is legitimate when documented and tested. Tuning to reduce workload by suppressing alerts is not.

### Validate The Monitoring Model Independently

Model validation is expected by regulators and is essential for confidence in the monitoring system. Validation should be independent of the team that operates the system.

Validation scope:

- conceptual soundness of scenarios and typology coverage;
- data completeness, quality, and lineage;
- threshold calibration methodology and documentation;
- segmentation logic and population coverage;
- alert-to-investigation-to-SAR conversion analysis;
- back-testing against known typologies and historical cases;
- tuning governance and change control;
- performance drift over time.

Document validation findings, remediation actions, and re-validation. A monitoring system that has never been independently validated is a vulnerability that examiners will identify.

### Close The Feedback Loop

Monitoring improves when dispositions feed back into tuning. The institution should track alert outcomes, SAR filings, and quality assurance findings, and use them to refine scenarios and thresholds.

Feedback mechanisms:

- track true positive, false positive, and escalation rates by scenario;
- analyze why false positives occurred and whether threshold or logic refinement can reduce them;
- incorporate confirmed SAR cases into back-testing;
- use QA findings on investigation quality to improve alert context;
- periodically review whether scenarios still match current typologies.

A monitoring program that never learns from its own results will drift and degrade over time.

## Common Traps

### Scenarios Without A Typology Basis

Rules that detect generic anomalies without a laundering rationale produce noise. Tie every scenario to a specific typology.

### Vendor Default Thresholds Never Calibrated

Thresholds shipped with a monitoring system reflect the vendor's assumptions, not the institution's customers. They must be calibrated to local data.

### No Segmentation Across Customer Types

A single threshold for all customers treats low and high risk identically. Segment so thresholds reflect comparable behavior.

### Coverage Gaps Against The Risk Assessment

Scenarios that do not map to the institution's key typologies leave blind spots. Coverage must be assessed against the risk assessment.

### Silent Threshold Widening To Reduce Volume

Widening thresholds to cut alert volume without documentation or testing suppresses detection. Every tuning change must be governed.

### No Independent Validation

A system validated only by its operators lacks credibility. Independent validation surfaces blind spots that operators miss.

### No Feedback Loop

A program that never learns from dispositions and SARs will drift. Feedback must inform future tuning.

## Self-Check

- Is every monitoring scenario mapped to a specific money laundering, terrorist financing, sanctions, or fraud typology with documented rationale?
- Are thresholds calibrated against real data with documented methodology, segmentation, and back-testing?
- Is monitoring segmented by customer type, product, channel, geography, and risk rating so thresholds reflect comparable behavior?
- Has coverage been assessed against the risk assessment to identify typologies with no corresponding scenario?
- Are data feeds complete enough to support the intended detection logic, including cross-product and cross-account visibility?
- Are threshold and scenario changes governed through documentation, testing, approval, and post-change monitoring?
- Has the monitoring model been independently validated, covering conceptual soundness, data, thresholds, segmentation, and performance?
- Are feedback loops in place so dispositions, SARs, and QA findings inform future tuning?
- Is the alert-to-investigation-to-SAR conversion rate monitored for reasonableness across scenarios?
- Is the scenario and threshold design confirmed against the applicable national law and regulator guidance rather than a generic standard?