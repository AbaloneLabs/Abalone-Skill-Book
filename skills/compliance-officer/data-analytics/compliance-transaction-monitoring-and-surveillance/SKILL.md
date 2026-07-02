---
name: compliance_transaction_monitoring_and_surveillance.md
description: Use when the agent is designing or reviewing transaction monitoring or trade surveillance systems, configuring scenarios and parameters, managing alert investigation workflows, or evaluating whether surveillance effectively detects market abuse, financial crime, and prohibited transactions.
---

# Compliance Transaction Monitoring And Surveillance

Transaction monitoring and trade surveillance are the automated and analyst-driven processes that scan transactions, orders, and trades for potential financial crime, market abuse, sanctions evasion, fraud, and other prohibited activity. These systems are often the primary regulatory expectation in financial services and other high-flow environments. They fail in characteristic ways: scenarios are tuned to reduce noise rather than to catch risk, alert backlogs grow until they are unmanageable, investigations are disposed superficially to close volume, and the systems are never re-validated after business or regulatory change. Surveillance that looks operational on paper can be effectively blind in practice.

Use this skill before designing a monitoring scenario, tuning parameters, building an alert workflow, evaluating surveillance effectiveness, or responding to a regulator's question about why a known pattern was missed. The goal is to make the agent treat surveillance as a risk-detection control with scenario coverage, calibration, workflow quality, and outcome measurement, not as a system that runs and therefore works.

## Core Rules

### Map Scenario Coverage To The Full Typology Of Risk To Be Detected

Surveillance effectiveness begins with coverage. A system with strong scenarios for one risk and none for another leaves a blind spot that regulators will probe.

Map coverage against:

- the regulatory obligations in scope, such as anti-money laundering, market abuse, insider dealing, front-running, spoofing, sanctions, fraud, bribery, and tax evasion;
- the product and transaction types in the business;
- the channels through which transactions flow;
- the customer, counterparty, and employee populations;
- the specific typologies for each risk, including how each manifests in the data.

For each obligation and typology, identify which scenario is intended to detect it. Gaps where no scenario exists are the highest priority. A scenario that covers a typology no longer relevant is wasted effort; a missing scenario for an active typology is an exposure.

### Calibrate Scenarios And Parameters Against Risk And Capacity

Scenario parameters determine what is flagged. Miscalibration produces either floods of false positives that overwhelm analysts or thresholds so high that real cases pass unflagged.

Calibrate by:

- defining the typology each scenario targets and the behavior it should catch;
- back-testing against historical known cases and confirmed suspicious activity;
- analyzing the population distribution to set thresholds that isolate the risk-relevant tail without drowning in noise;
- estimating alert volume and comparing to analyst capacity and service levels;
- documenting the basis for each parameter, including data, benchmark, and judgment;
- reviewing calibration regularly and after changes in business mix, volume, or regulation.

Tuning that reduces volume without measuring the effect on detection is dangerous. Before suppressing a scenario or raising a threshold, confirm through analysis that the alerts being removed are false positives and that true cases would still be caught.

### Govern The Alert Lifecycle From Generation To Disposition

Alerts must be worked, not just generated. An unworked alert is worse than no alert, because it implies coverage that does not exist.

Define the lifecycle:

- alert generation, deduplication, and routing;
- triage prioritization by risk, with service levels for each tier;
- investigation steps, required evidence, and documentation standards;
- decision criteria for escalation, closure, or filing (such as a suspicious activity report);
- four-eyes or independent review for closures of higher-risk alerts;
- retention of alert records and dispositions;
- feedback of outcomes into scenario tuning.

Track aging of alerts and investigations. A growing backlog is a leading indicator that capacity, calibration, or workflow is broken. Report backlog and aging to leadership as a risk, not only as an operational metric.

### Ensure Investigation Quality, Not Just Volume

The temptation to close alerts quickly under volume pressure produces superficial dispositions that do not withstand regulatory review. Quality matters as much as throughput.

Investigation standards should require:

- review of the transaction or order pattern that triggered the alert;
- consideration of related transactions, accounts, and parties, not only the flagged item;
- review of customer or counterparty context, including risk rating and history;
- documented rationale for the disposition, referencing the evidence reviewed;
- escalation when the pattern matches a typology even if no single item is conclusive;
- independent review or challenge for closures above defined risk thresholds.

Disposition codes such as "no further action" without rationale are a red flag. Audit samples of closed alerts to confirm that dispositions are evidence-based and that genuine cases were not closed superficially.

### Validate Surveillance After Any Material Change

Surveillance systems decay silently when the business or systems change. A scenario that worked last year may be blind after a product launch, system migration, or data feed change.

Re-validate after:

- new products, markets, channels, or transaction types;
- system migrations, data feed changes, or reference-data changes;
- regulatory changes affecting typologies or thresholds;
- mergers, acquisitions, or divestitures;
- significant changes in customer or counterparty mix;
- changes to entity or party reference data that affect aggregation.

Validation confirms scenarios still fire on the intended patterns, parameters remain appropriate, and data feeds remain complete. Document validation evidence and treat unvalidated post-change surveillance as a known gap.

### Reconcile Data Feeds And Confirm Completeness

Surveillance is only as good as the data it receives. Missing feeds, delayed feeds, or incomplete fields create blind spots that may never surface until a case is missed.

Reconcile:

- that all in-scope systems and transaction types feed the surveillance platform;
- that feed volumes reconcile to source systems;
- that fields required for scenarios are populated and valid;
- that reference data such as customer risk ratings and party relationships is current;
- that there is no manual or off-system channel bypassing surveillance.

Where a channel cannot be surveilled automatically, define a compensating manual control and document the residual risk. A business that routes a transaction type outside the surveilled system has created a blind spot by design.

### Measure Effectiveness End To End

Surveillance effectiveness is not measured by alerts generated but by whether risk is detected and acted upon. Build end-to-end metrics.

Measure:

- scenario coverage against the typology map and identified gaps;
- alert-to-investigation and investigation-to-action conversion rates;
- true-positive and false-positive rates by scenario;
- time from transaction to alert and from alert to disposition;
- backlog and aging of alerts and investigations;
- outcomes such as reports filed, accounts closed, or controls tightened;
- feedback loops from confirmed cases back into scenario tuning.

A scenario that generates many alerts but no confirmed cases may be poorly calibrated or may reflect a real but uninvestigated risk. Interpret metrics together, not in isolation.

### Coordinate With Related Functions

Surveillance does not operate alone. It interacts with sanctions screening, KYC, investigations, legal, and the business.

Coordinate:

- handoffs between surveillance alerts and investigation teams;
- alignment of customer risk ratings used across functions;
- escalation paths for alerts that indicate market abuse requiring legal or regulatory action;
- feedback to KYC when surveillance reveals inaccurate customer information;
- consistency of typologies and risk definitions across functions.

Inconsistent risk definitions across functions create gaps where a transaction is not flagged because the customer is mis-rated or the typology is defined differently.

## Common Traps

### Tuning To Reduce Noise Without Measuring Detection Effect

Suppressing scenarios or raising thresholds to cut volume can hide real cases. Require evidence-based tuning.

### Backlogs Treated As Operational Rather Than Risk

A growing alert backlog means coverage is not real. Report it as risk and address capacity or calibration.

### Superficial Dispositions Under Volume Pressure

Closing alerts as "no further action" without rationale fails regulatory review. Require documented, evidence-based disposition.

### No Re-Validation After Business Or System Change

A working scenario can go blind after a migration or product launch. Re-validate after change.

### Missing Data Feeds Creating Silent Blind Spots

Unsurveilled channels or incomplete fields create gaps that surface only when a case is missed. Reconcile feeds.

### Coverage Focused On One Risk While Others Are Uncovered

Strong AML scenarios with no market-abuse coverage leave a regulator-visible gap. Map coverage to the full typology.

### Treating Alert Volume As The Effectiveness Measure

Alerts generated is not risk detected. Measure end-to-end outcomes.

## Self-Check

- Is scenario coverage mapped against the full set of regulatory obligations, products, channels, populations, and typologies, with gaps identified?
- Are scenario parameters calibrated through back-testing, population analysis, capacity comparison, and documented rationale, with detection effect measured before tuning?
- Does the alert lifecycle define generation, deduplication, triage, investigation standards, decision criteria, independent review, retention, and feedback to tuning?
- Are investigation standards requiring related-transaction review, customer context, documented rationale, escalation, and independent review for higher-risk closures?
- Is surveillance re-validated after new products, migrations, data feed changes, regulatory change, M&A, and reference-data changes, with evidence retained?
- Are data feeds reconciled to source systems for completeness, volume, field validity, reference-data currency, and off-system channel bypass?
- Does effectiveness measurement cover coverage gaps, conversion rates, true- and false-positive rates, timeliness, backlog aging, outcomes, and feedback loops?
- Are backlog and aging reported to leadership as risk, with capacity and calibration addressed rather than normalized?
- Are dispositions audited through sampling to confirm evidence-based closure and that genuine cases were not superficially closed?
- Is surveillance coordinated with sanctions screening, KYC, investigations, legal, and the business for consistent risk definitions and handoffs?
