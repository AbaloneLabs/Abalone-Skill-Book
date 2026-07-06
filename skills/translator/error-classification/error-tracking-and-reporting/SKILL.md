---
name: error_tracking_and_reporting.md
description: Use when the agent is recording translation errors in a tracking system, building QA reports and dashboards, aggregating error data by type severity translator or content type, computing quality metrics and scores, presenting quality findings to clients or stakeholders, or managing the data and reporting layer that turns classified errors into comparable actionable quality information.
---

# Error Tracking And Reporting

Once errors are classified, they must be captured, stored, aggregated, and reported so that the data drives decisions. This is the tracking and reporting layer: the system of records, the metrics computed from them, and the reports that present quality to translators, reviewers, project managers, and clients. Agents often treat this as a mechanical afterthought, assuming that classified errors will somehow become useful information on their own. They will not. Errors recorded inconsistently, in incompatible formats, without the metadata needed to aggregate them, produce a data set that cannot answer even basic questions: which translator struggles with terminology, which content type concentrates accuracy errors, whether quality is improving over time. The harm is that the effort spent classifying errors is wasted because the data cannot be used, acceptance decisions rest on impressions rather than evidence, and improvement efforts are directed by anecdote rather than by pattern. Good tracking and reporting turn classified errors into the most valuable quality asset a program has; poor tracking turns them into an unreadable log.

Use this skill when designing or operating the error tracking system, computing and presenting quality metrics, building QA reports or dashboards, or reporting quality findings to stakeholders. The goal is error data that is captured consistently, aggregated meaningfully, and reported so it drives correction, acceptance, and improvement.

## Core Rules

### Capture Complete And Structured Records

Every classified error should be recorded as a structured record with the fields needed for aggregation and analysis. At minimum, capture a stable identifier, the project and batch, the source and target segment reference, the error type and subtype from the typology, the severity, a description of the defect, the reviewer who classified it, the translator or post-editor responsible, the content type and tier, the locale pair, and the date. Without these fields, the data cannot be sliced by the dimensions that matter. A free-text comment log is not a tracking system; it is a notebook that no analysis can read. Define the record schema once, enforce it across reviewers, and store records in a system that supports query and aggregation rather than in scattered documents.

### Record The Metadata That Enables Aggregation

The value of tracking is aggregation, and aggregation depends on metadata. Decide in advance which dimensions the program needs to analyze: by translator, to target support; by reviewer, to detect calibration drift; by content type and tier, to find weak domains; by error type and severity, to find systemic patterns; by locale pair, to find language-specific issues; and over time, to detect trends. Capture the metadata for each dimension on every record. Metadata added later, by memory or guesswork, is unreliable and often missing. The schema should make the necessary fields required, not optional, so that a record cannot be saved incomplete.

### Define Metrics By Formula And Keep Them Stable

Metrics must be defined by formula and applied identically across projects and time. Define, for each metric, the numerator, the denominator, the severity weighting, and the population over which it is computed. Common metrics include error density as weighted errors per thousand words, the MQM score, the pass rate against an acceptance threshold, the percentage of segments reviewed, and the critical-defect count. State explicitly whether density is computed over all words or reviewed words, whether preference is included or excluded, and how critical defects that block acceptance are treated in the score. Publish the definitions and freeze them; a metric whose definition drifts produces a time series that looks continuous but is not, hiding real trends behind definitional changes. When a definition must change, version it and restate historical figures on the new basis where possible.

### Reconcile Metrics With The Sampling Basis

A metric is only as valid as the sampling behind it. If quality is scored on a sample, the score is an estimate of the whole, and the report must state the sampling basis and the confidence it supports, not present the estimate as a census. A report that states "the batch scored 98" without noting that only a five percent sample was reviewed overstates certainty and misleads the reader. Reconcile every reported metric with what was actually reviewed, and where sampling was used, present the result as an estimate with its basis. This is especially important for client-facing reports, where an unsupported impression of certainty can create liability if a defect later emerges in unreviewed content.

### Report To Each Audience At The Right Level

Different audiences need different reports. Translators need segment-level feedback they can act on and learn from: the specific defect, its type and severity, and the correction. Reviewers and quality leads need aggregated views: error density by type and translator, calibration indicators, and trends. Project managers need pass or fail status, rework status, and schedule impact. Clients need a concise quality summary tied to the acceptance criteria, the score, the sampling basis, and any resolved blockers, without exposing internal translator-level data unless agreed. Designing one report for everyone produces a document that is too detailed for the client and too aggregated for the translator. Match granularity and content to the audience, and be explicit about what each report is and is not.

### Distinguish Findings From Resolved Defects

A tracking system must distinguish errors found from errors resolved, or it cannot tell whether quality improved. Capture the lifecycle of each record: found, accepted or disputed, corrected, verified. A report that counts only findings overstates the defect level of content that was subsequently fixed; a report that counts only post-correction state hides how many defects review caught. Both views are legitimate for different purposes: findings density measures review effectiveness and translator performance, residual defect density measures delivered quality. Define which view each metric uses and do not mix them silently.

### Make Reports Actionable, Not Just Descriptive

A report that lists numbers without implying action is a decoration. Every report should make the implication explicit: which translator needs terminology support, which content type needs a better termbase, which severity class is trending up and why, which acceptance decisions are at risk. Pair the data with recommended actions and owners, and track whether the actions were taken. A tracking system that generates reports no one acts on is a cost without a benefit; the point of the data is to change behavior, and the report should close the loop to that change.

### Protect Confidentiality And Fairness In Translator-Level Data

Translator-level error data is sensitive. It can be demoralizing if presented punitively, and it can be unfair if it reflects reviewer calibration differences rather than real performance. When reporting translator-level data, contextualize it against the content they received, the reviewers who assessed them, and the calibration baseline. Avoid public ranking that pits translators against each other, and use the data to target support and improvement, not punishment. Fairness in reporting preserves the trust that makes translators engage with feedback rather than defend against it.

## Common Traps

### Free-Text Logs Instead Of Structured Records

A comment log cannot be aggregated and turns classified errors into an unreadable notebook, wasting the classification effort.

### Missing Metadata Preventing Aggregation

Without translator, content type, tier, locale, and date on every record, the data cannot be sliced by the dimensions that reveal patterns.

### Metric Definitions That Drift

A metric computed differently across projects or over time destroys comparability and hides real trends behind definitional shifts.

### Presenting Sample Estimates As Census

Reporting a sample-based score without the sampling basis overstates certainty and can mislead clients and create liability.

### One Report For All Audiences

A single report is too detailed for clients and too aggregated for translators, serving no audience well.

### Mixing Findings And Resolved Defects

Counting only findings overstates defect levels; counting only post-correction state hides review effectiveness. Define which view each metric uses.

### Descriptive Reports With No Action

Numbers without recommended actions and owners generate reports no one acts on, making the tracking system a cost without benefit.

### Punitive Translator-Level Ranking

Public or punitive use of translator data destroys trust and makes translators defend against feedback rather than learn from it.

## Self-Check

Before approving an error tracking setup or a quality report, verify:

- Each error is captured as a structured record with identifier, project, segment reference, type, subtype, severity, description, reviewer, translator, content type, tier, locale pair, and date.
- Metadata required for aggregation, by translator, reviewer, content type, tier, locale, and time, is captured on every record as required fields, not optional.
- Metrics are defined by formula, numerator, denominator, weighting, and population, and are applied identically across projects with versioned changes.
- Every reported metric is reconciled with its sampling basis, with sample-based results presented as estimates rather than census.
- Reports are tailored by audience: segment-level for translators, aggregated for leads, status for project managers, and concise acceptance-tied summaries for clients.
- The system distinguishes findings from resolved defects, and each metric states whether it measures review effectiveness or delivered quality.
- Reports include recommended actions and owners, and the system tracks whether actions were taken, closing the loop to behavior change.
- Translator-level data is contextualized against content, reviewer, and calibration, and used for support rather than punitive ranking.
- No metric definition has drifted silently, and no sample estimate is presented as a full census.
- The tracking and reporting layer turns classified errors into comparable, actionable quality information that drives correction, acceptance, and improvement.
