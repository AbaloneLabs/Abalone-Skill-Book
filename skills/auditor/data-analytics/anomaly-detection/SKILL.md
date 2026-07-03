---
name: anomaly-detection.md
description: Use when the agent is using anomaly detection techniques to identify unusual transactions or patterns in financial data, designing rules and statistical thresholds, evaluating Benford's Law and outlier detection, assessing the risk that anomalies indicate fraud or error, or deciding how to investigate and resolve flagged anomalies without drowning in false positives.
---

# Anomaly Detection

Anomaly detection identifies items or patterns that deviate from the expected, to surface potential errors, fraud, or unusual transactions for investigation. It is powerful because fraud and error are, by nature, unusual — they hide in the tails of distributions, in rare combinations, and in patterns that normal sampling will almost never encounter. But anomaly detection is also a generator of noise: most flagged items are benign, and an undisciplined approach either drowns the auditor in false positives or, worse, sets thresholds so high that real anomalies slip through. The discipline is to design detection methods that target the specific fraud and error schemes relevant to the population, to calibrate thresholds to separate signal from noise, and to investigate flagged anomalies to resolution.

## Core Rules

### Start from the fraud and error schemes you are trying to detect

Anomaly detection is most effective when designed to surface specific schemes. Before running any detection, enumerate the schemes relevant to the population:

- ghost vendor or ghost employee payments (duplicate or near-duplicate master data);
- manual journal-entry fraud (weekend, holiday, round-amount, top-of-period entries);
- revenue manipulation (cut-off violations, round-tripping, channel stuffing);
- duplicate payments or duplicate expense claims;
- unusual related-party transactions;
- override of controls (entries by senior staff, entries bypassing workflow).

Each scheme suggests a specific detection approach. Generic anomaly detection ("find anything unusual") produces unmanageable noise; scheme-targeted detection produces actionable exceptions.

### Choose detection methods matched to the scheme

Different schemes are surfaced by different methods:

- **Rule-based tests**: explicit rules for known patterns (duplicate invoice numbers, payments to employees, entries on holidays). High precision, limited to known patterns.
- **Statistical outlier detection**: items deviating from a distribution (Z-score, interquartile range) on amount, frequency, or timing. Catches extreme values.
- **Benford's Law analysis**: compares the distribution of leading digits to the expected Benford distribution; deviations may indicate fabricated numbers. Most useful on large, naturally occurring populations.
- **Clustering and segmentation**: groups similar items and flags those that do not fit any group, or that form small unusual clusters.
- **Link analysis**: surfaces relationships (shared addresses, bank accounts, phone numbers) between entities that should be unrelated.

Match the method to the scheme. Benford's Law will not find a duplicate payment; a duplicate test will not find a fabricated journal-entry pattern.

### Calibrate thresholds to balance signal against noise

Threshold setting is the central practical challenge. Too tight, and real anomalies are missed; too loose, and the auditor drowns in false positives that get rubber-stamped dismissed. Calibrate by:

- running the detection on the population and examining the exception volume;
- adjusting thresholds so the exception set is small enough to investigate thoroughly;
- layering filters (e.g., round-amount AND weekend AND manual entry) to sharpen precision;
- reviewing a sample of flagged items to confirm the detection is surfacing genuine candidates.

Document the threshold and the rationale. A threshold chosen without examining its effect on exception volume is a guess.

### Investigate flagged anomalies to resolution, not to dismissal

Every flagged anomaly must be investigated, and the investigation must reach a resolution:

- is the item a confirmed error, a legitimate unusual transaction, or a false positive?
- for legitimate unusual transactions, what is the business reason, corroborated by evidence?
- for confirmed errors, what is the correction and the root cause?
- for false positives, does the detection logic need refinement?

The failure mode is dismissal: flagging items and then accepting vague explanations ("it's a one-off," "the system did it") without corroboration. Require the same standard of corroboration as for any analytical difference.

### Use Benford's Law appropriately and within its limits

Benford's Law is a powerful tool for detecting potentially fabricated numbers, but it is frequently misused. Apply it only where its conditions hold:

- the population is large (hundreds or thousands of items);
- the data spans several orders of magnitude (not constrained to a narrow range);
- the numbers are naturally occurring (not assigned, like invoice numbers or account codes).

Do not apply Benford's Law to small populations, to assigned numbers, or to populations constrained by thresholds or caps. Interpret deviations as a prompt for further investigation, not as proof of fraud; many legitimate factors (mix of transaction types, business changes) cause Benford deviations.

### Combine multiple detection methods for coverage

No single method surfaces all schemes. Combine:

- rule-based tests for known patterns;
- statistical outliers for extreme values;
- Benford for fabricated-number indications;
- link analysis for relationship-based schemes;
- temporal analysis for timing schemes (cut-off, weekend entries).

A layered approach catches schemes that any single method would miss. Document which methods address which schemes so the coverage is explicit.

### Segment the population before detecting anomalies

Anomalies are relative to their peer group. A large transaction in one segment may be normal in another. Segment by:

- transaction type, account, or category;
- location, business unit, or counterparty;
- time period or user.

Then detect anomalies within each segment. Detecting anomalies on an unsegmented population produces both false positives (normal large items flagged) and false negatives (a large item hidden among larger items in another segment).

### Connect anomaly findings to the fraud risk assessment

Anomaly detection is a fraud-risk procedure. Connect its findings to the fraud risk assessment:

- do the schemes targeted reflect the assessed fraud risks?
- do confirmed anomalies indicate a fraud risk that needs additional procedures?
- does the absence of anomalies in a high-risk area genuinely lower the risk, or reflect detection limits?

Do not treat anomaly detection as a standalone technical exercise; it is part of the response to assessed fraud risk, and its results should inform and be informed by that assessment.

### Document the methods, thresholds, exceptions, and resolutions

For each anomaly detection exercise, document:

- the schemes targeted and the methods used;
- the population and any segmentation;
- the thresholds and how they were calibrated;
- the exceptions identified, their investigation, and resolution;
- any refinement of the detection logic based on false positives.

This documentation is what makes the procedure defensible and repeatable. An anomaly detection run without documented methods, thresholds, and resolution cannot be reviewed or relied upon.

## Common Traps

- **Running generic "find anything unusual" detection** without targeting specific fraud schemes, producing unmanageable noise.
- **Setting thresholds without examining exception volume**, either drowning in false positives or missing real anomalies.
- **Dismissing flagged anomalies with vague explanations** ("one-off," "system error") without corroboration.
- **Misapplying Benford's Law** to small, assigned, or range-constrained populations where its conditions do not hold.
- **Relying on a single detection method** that misses schemes other methods would catch.
- **Detecting anomalies on unsegmented populations**, producing false positives and false negatives from inappropriate peer groups.
- **Treating anomaly detection as a technical exercise** disconnected from the fraud risk assessment.
- **Generating exception lists that are never investigated to resolution**, leaving the procedure incomplete.
- **Failing to document methods, thresholds, exceptions, and resolutions**, making the procedure indefensible and irreproducible.
- **Over-interpreting a single statistical signal as proof of fraud**, when many legitimate factors cause deviations.

## Self-Check

- Did I enumerate the specific fraud and error schemes the detection is meant to surface, and choose methods matched to each?
- Are the detection methods (rule-based, statistical, Benford, clustering, link) appropriate to the schemes targeted?
- Did I calibrate thresholds by examining exception volume and reviewing a sample of flagged items, and document the rationale?
- Is every flagged anomaly investigated to a confirmed resolution — error, corroborated legitimate transaction, or refined false positive?
- Where I used Benford's Law, does the population meet its conditions (large, multi-order-of-magnitude, naturally occurring)?
- Did I combine multiple detection methods to cover schemes that any single method would miss?
- Did I segment the population so anomalies are detected against appropriate peer groups?
- Did I connect the detection design and findings to the fraud risk assessment, in both directions?
- Did I document the schemes, methods, population, segmentation, thresholds, exceptions, investigation, and resolution so the procedure is defensible and repeatable?
