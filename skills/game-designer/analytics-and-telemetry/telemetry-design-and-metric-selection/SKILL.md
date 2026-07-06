---
name: telemetry-design-and-metric-selection.md
description: Use when the agent is designing telemetry and instrumentation, selecting metrics to track, planning event logging and data collection, or evaluating whether the telemetry captures meaningful player behavior or produces noise, privacy risk, and data that cannot answer the design questions that motivated collecting it.
---

# Telemetry Design and Metric Selection

Telemetry — the instrumentation that captures player behavior — is the foundation of data-driven game design, and it is also where data collection most commonly fails to serve design, through metrics that capture noise rather than signal, events that cannot answer the questions that motivated them, and privacy-invasive collection that erodes player trust. The judgment problem is that telemetry must capture meaningful behavior (so the data answers design questions), must be scoped to what is needed (so it does not drown in noise or violate privacy), and must be designed before launch (so the questions that arise post-launch can be answered), and agents tend to miss this because collecting everything feels safe (more data is better) while producing noise and privacy risk, and because the metrics that are easy to collect (sessions, clicks) are often not the metrics that answer design questions. The harm is data that cannot inform decisions, privacy violations that erode trust, and post-launch questions that the telemetry cannot answer because it was not designed to. This skill covers how to design telemetry that captures meaningful behavior, select metrics that answer design questions, and avoid the noise and privacy traps. The agent has latitude in the instrumentation, but the obligation to make telemetry serve design is not optional.

## Core Rules

### Design Telemetry Around Specific Design Questions

Telemetry must be designed around specific design questions — what do we need to know, what decisions will the data inform — rather than collecting data speculatively, because data collected without a question produces noise that cannot inform decisions. The decision rule: for each metric, define the design question it answers and the decision it informs, and cut metrics that do not serve a question. Speculative collection produces noise, because the data was not tied to a question.

### Capture Behavioral Funnel and Churn Points

The most important telemetry captures the behavioral funnel (where players progress, where they drop off) and the churn points (where players leave), because these reveal the engagement and retention dynamics that determine the game's health, and the instrumentation must capture these with granularity. The decision rule: instrument the funnel and churn points with sufficient granularity to identify where players drop off and why, and ensure these metrics are captured from launch. Uninstrumented funnels leave churn unexplained, because the drop-off points were not captured.

### Scope Collection to What Is Needed, Avoiding Noise and Privacy Risk

Telemetry scope must be calibrated to what is needed — capturing the metrics that answer design questions, not collecting everything possible — because over-collection produces noise (drowning the signal) and privacy risk (collecting data that is not needed). The decision rule: scope collection to the metrics that serve design questions, avoid collecting data that is not needed, and minimize privacy exposure. Over-collection produces noise and privacy risk, because the scope exceeded the need.

### Instrument From Launch, Not Retroactively

Telemetry must be instrumented from launch, because the questions that arise post-launch (why are players churning, where is the bottleneck) can only be answered with data captured from the start, and retroactive instrumentation cannot answer questions about the launch period. The decision rule: instrument all critical telemetry before launch, and avoid assuming it can be added later. Retroactive instrumentation cannot answer launch questions, because the data was not captured.

### Ensure Data Quality Through Validation and Cleaning

Telemetry data must be validated and cleaned — checking for logging errors, missing events, corrupt records — because unvalidated data produces misleading analyses (the noise is mistaken for signal), and data quality processes must be part of the telemetry pipeline. The decision rule: implement data validation and cleaning in the pipeline, check for logging errors and missing events, and avoid analyzing unvalidated data. Unvalidated data misleads, because the errors were mistaken for signal.

### Respect Player Privacy and Comply With Regulations

Telemetry that captures player data must respect privacy — collecting only what is needed, anonymizing where possible, disclosing collection — and comply with privacy regulations (GDPR, CCPA), because privacy-invasive collection erodes trust and regulatory non-compliance incurs legal risk. The decision rule: minimize personal data collection, anonymize where possible, disclose telemetry to players, and ensure regulatory compliance. Privacy-invasive telemetry erodes trust, because the collection exceeded what was needed or disclosed.

## Common Traps

### Speculative Collection Producing Noise

The team collects data speculatively — logging everything, just in case — and the volume produces noise that drowns the signal and cannot inform decisions. The trap is that more data feels safe. The false signal is that the telemetry is comprehensive. The harm is that the noise obscures the signal, the analysts cannot find meaningful patterns in the volume, the data cannot answer design questions because it was not designed to, and the telemetry investment is wasted on noise, because the collection was not tied to questions.

### Uninstrumented Funnels Leaving Churn Unexplained

The team does not instrument the behavioral funnel and churn points with granularity, and when players churn post-launch, the team cannot identify where or why. The trap is that high-level metrics (daily active users) are tracked. The false signal is that the telemetry shows player counts. The harm is that the high-level metrics do not reveal where players drop off or why, the churn is unexplained, the team cannot diagnose or fix the retention problem, and the questions that determine the game's health cannot be answered, because the funnel was not instrumented.

### Over-Collection Creating Privacy Risk

The team collects more data than needed — including personal or sensitive data — and the over-collection creates privacy risk and regulatory exposure. The trap is that comprehensive collection feels thorough. The false signal is that the telemetry is rich. The harm is that the unnecessary data exposes players to privacy risk, the collection may violate regulations (GDPR, CCPA), the trust erosion and legal risk attach to the game, and the data that was not needed incurs harm, because the scope exceeded the need.

### Retroactive Instrumentation Missing Launch Data

The team assumes telemetry can be added post-launch, and when questions arise about the launch period, the data was not captured. The trap is that instrumentation can be iterated. The false signal is that the game is live. The harm is that the launch-period questions (why did initial churn occur, where was the launch bottleneck) cannot be answered, the data for that period was never captured, the team cannot learn from the launch, and the critical window's data is lost, because the instrumentation was not in place from launch.

### Unvalidated Data Misleading Analysis

The team analyzes telemetry without validating or cleaning it, and logging errors and missing events produce misleading patterns. The trap is that the data is available. The false signal is that the analysis shows patterns. The harm is that the errors are mistaken for signal, the analysis reaches wrong conclusions, the decisions based on the misleading data are wrong, and the team acts on noise as if it were insight, because the data was not validated.

### Privacy-Invasive Collection Eroding Trust

The team collects personal or sensitive data without minimizing, anonymizing, or disclosing, and the privacy-invasive collection erodes player trust and risks regulatory action. The trap is that the data could be useful. The false signal is that the telemetry is detailed. The harm is that players who discover the invasive collection lose trust, the regulatory non-compliance incurs legal risk, the reputational damage attaches to the game, and the collection that exceeded privacy norms harms the player relationship, because privacy was not respected.

## Self-Check

- Is each metric tied to a specific design question and decision, with speculative collection cut?
- Are the behavioral funnel and churn points instrumented with granularity to identify where and why players drop off?
- Is collection scoped to what is needed, avoiding noise and minimizing privacy exposure?
- Is critical telemetry instrumented from launch, so post-launch questions about the launch period can be answered?
- Is the telemetry data validated and cleaned to prevent logging errors from misleading analysis?
- Does telemetry respect player privacy (minimization, anonymization, disclosure) and comply with regulations?
- Did I confirm the telemetry captures meaningful behavior that can inform design decisions, not just noise?
