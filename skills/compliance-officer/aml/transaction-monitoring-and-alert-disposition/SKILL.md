---
name: transaction_monitoring_and_alert_disposition.md
description: Use when the agent is designing transaction monitoring scenarios, tuning detection thresholds, investigating and dispositioning alerts, reducing false positives, or validating monitoring models for AML and financial crime detection.
---

# Transaction Monitoring And Alert Disposition

Transaction monitoring is the detective engine of an AML program. It is how an institution identifies activity that deviates from expected behavior or matches known money laundering, terrorist financing, sanctions evasion, or fraud typologies. FATF Recommendation 20, the BSA, the US AML Act, FinCEN expectations, and EU AMLD all require risk-based monitoring supported by effective investigation of alerts. Monitoring fails when scenarios are mis-calibrated, alerts are closed without genuine investigation, or the system drowns analysts in noise such that real risk is buried.

Use this skill before designing monitoring scenarios, tuning thresholds, defining alert disposition standards, reducing false positives, or validating a monitoring model. The goal is to make the agent think about scenario coverage, threshold calibration, investigation quality, disposition evidence, and model validation. A monitoring system that produces many alerts but few defensible conclusions is not effective.

## Core Rules

### Design Scenarios Around Typologies And Risk

Monitoring scenarios should be built to detect specific typologies and risk, not generic anomalies. Start from the institution's risk assessment and the laundering methods most relevant to its products, customers, and geographies.

Scenario families include:

- structuring and smurfing below reporting thresholds;
- rapid movement of funds with no economic purpose (pass-through);
- activity inconsistent with customer profile or expected activity;
- high-risk geography exposure;
- trade-based money laundering indicators;
- virtual asset and crypto conversion patterns;
- funnel account and third-party aggregation;
- sudden dormancy break or in-and-out activity;
- large cash deposits inconsistent with occupation;
- internal fraud and embezzlement indicators;
- sanctions and proliferation financing indicators.

Each scenario should have a documented typology rationale, a defined data feed, a detection logic, and an expected outcome. Scenarios without a clear typology basis generate noise without insight.

### Calibrate Thresholds Deliberately

Thresholds determine what becomes an alert. Thresholds set too low flood analysts with false positives; set too high, they miss genuine risk. Calibration should be data-driven and periodically re-tuned.

Calibration inputs:

- historical transaction distribution by customer segment;
- expected activity from CDD;
- population coverage and alert volume capacity;
- back-testing against known suspicious cases;
- segmentation by customer type, product, and channel;
- seasonality and business cycles.

Document the methodology, the data, and the resulting thresholds. Avoid tuning solely to reduce workload, which is a common examination finding.

### Investigate Alerts With Genuine Analysis

An alert is not resolved by a glance. Disposition requires analysis of the underlying activity, the customer context, and whether the behavior is explainable.

Investigation should examine:

- the transactions that triggered the alert, in full;
- the customer's expected activity from CDD;
- historical behavior and prior alerts;
- related accounts, counterparties, and beneficial owners;
- external data such as adverse media, sanctions, and PEP status;
- supporting documentation such as invoices or contracts where relevant;
- whether the activity is consistent with the customer's stated business.

The analyst should reach a reasoned conclusion, not merely confirm that the data matches the rule.

### Apply Consistent Disposition Standards

Dispositions must be consistent and defensible. Common outcomes include true positive, false positive, and escalated for SAR consideration.

For each disposition:

- true positive: activity is suspicious or unusual and warrants further action or SAR filing;
- false positive: activity is fully explained by expected behavior and documented customer context;
- escalated: activity is ambiguous and requires deeper review or EDD.

Each closed alert should record the rationale, the evidence reviewed, the analyst, the reviewer, and the date. A disposition of "no action" with no rationale is not a disposition.

### Manage False Positives Without Losing Signal

High false positive rates erode analyst attention and hide real cases. Reducing false positives is legitimate, but it must not come at the cost of suppressing genuine alerts.

Reduction techniques:

- refine thresholds using segment-level data;
- incorporate expected activity and customer context into the rule;
- use suppression logic for known recurring patterns;
- add filters for documented benign explanations;
- improve data quality and counterparty enrichment;
- apply feedback loops from confirmed true positives.

Never reduce alerts by silently widening thresholds to avoid workload. Every tuning change should be documented, tested, and approved.

### Validate The Monitoring Model

Model validation is expected by regulators and is essential for confidence in the system. Validation should be independent and periodic.

Validation covers:

- conceptual soundness of scenarios and typology coverage;
- data completeness and quality;
- threshold calibration methodology;
- population coverage and segmentation;
- alert-to-investigation-to-SAR conversion;
- back-testing against known typologies and historical cases;
- tuning governance and change control;
- performance metrics and drift over time.

Document validation findings, remediation, and re-validation. A monitoring system that has never been independently validated is a vulnerability.

### Close The Loop With Feedback

Monitoring improves when dispositions feed back into tuning. Track alert outcomes, SAR filings, and quality assurance findings, and use them to refine scenarios and thresholds. A monitoring program that never learns from its own results will drift.

## Common Traps

### Scenarios Without A Typology Basis

Rules that detect generic anomalies without a laundering rationale produce noise. Tie every scenario to a specific typology.

### Tuning Only To Reduce Volume

Cutting thresholds to lower alert volume reduces workload but also reduces detection. Documented, tested tuning is defensible; blanket suppression is not.

### Dispositions Without Rationale

Closing an alert as a false positive with no documented reasoning is indistinguishable from not investigating. Examiners and auditors will challenge empty closures.

### Ignoring The Customer Context

An alert that looks unusual in isolation may be entirely consistent with a documented customer profile. Conversely, normal-looking activity may be suspicious for a high-risk customer. Always read the alert against CDD.

### Stale Thresholds And Scenarios

Customer behavior, products, and typologies change. Thresholds and scenarios that are never re-tuned become blind spots.

### Data Quality Gaps

Missing counterparty data, unpopulated fields, or inconsistent coding can cause both false positives and missed alerts. Treat data quality as a monitoring control.

### No Independent Validation

A monitoring system validated only by its operators lacks credibility. Independent validation is expected and surfaces blind spots.

## Self-Check

- Are scenarios designed around specific typologies from the institution's risk assessment, with documented rationale?
- Are thresholds calibrated with data, segmentation, and back-testing, and is the methodology documented?
- Are alerts investigated with genuine analysis of transactions, customer context, and related parties?
- Are dispositions consistent, with true positive, false positive, and escalation outcomes documented with rationale and evidence?
- Are false positive reductions achieved through documented tuning rather than blanket suppression?
- Has the monitoring model been independently validated, covering conceptual soundness, data, thresholds, and performance?
- Are feedback loops in place so dispositions, SARs, and QA findings inform future tuning?
- Is the alert-to-SAR conversion rate monitored for reasonableness?
- Are data quality issues tracked and remediated as monitoring controls?
- Are tuning changes governed through change control with testing and approval?
