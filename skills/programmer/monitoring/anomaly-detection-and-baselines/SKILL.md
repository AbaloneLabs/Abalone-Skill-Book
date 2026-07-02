---
name: anomaly_detection_and_baselines.md
description: Use when the agent is building or tuning anomaly detection for metrics, learning baselines from historical data, handling seasonality (daily, weekly, yearly), choosing an anomaly detection algorithm, tuning thresholds to reduce false positives, or diagnosing metric drift. Also covers the failure mode of static thresholds that ignore seasonality and trend, anomaly detection that floods responders with false positives until alerts are muted, baselines trained on contaminated data, and the difficulty of distinguishing a real anomaly from expected behavior in a changing system.
---

# Anomaly Detection And Baselines

Anomaly detection is the practice of flagging metrics that deviate from expected behavior, where "expected" is defined by a baseline learned from history rather than a fixed threshold. The judgment problem is that real systems are not stationary: traffic has daily, weekly, and seasonal patterns; it grows over time; and it changes when the system changes (a deploy, a new feature, a marketing push). A static threshold that ignores these patterns either floods responders with false positives (set too tight) or misses real anomalies (set too loose), and a baseline learned naively bakes in the patterns and contaminations of its training window. The discipline is to model the baseline with the seasonality and trend the metric actually has, to learn the baseline from clean history (excluding incidents, deploys, and anomalies), to choose an algorithm matched to the metric's characteristics, to tune thresholds to the false-positive rate responders will tolerate, and to recognize that anomaly detection is probabilistic — it flags candidates for investigation, it does not diagnose causes.

Agents tend to set a static threshold and move on, or to enable anomaly detection without tuning. The harm appears as alert storms from thresholds that ignore the daily traffic cycle (every peak and trough is "anomalous"), as missed anomalies from thresholds set to tolerate noise, as baselines that learned from an incident window and now treat the incident as normal, and as responders muting the alerts because the false-positive rate is unbearable. The judgment is to decompose the metric into its components (level, trend, seasonality, noise), to learn the baseline from representative clean data, to tune the detector to the operational false-positive tolerance, and to treat the output as a signal to investigate rather than a verdict. Anomaly detection that is not tuned is noise; anomaly detection that is tuned is one of the strongest signals in monitoring.

## Core Rules

### Model The Baseline With The Metric's Seasonality And Trend

Most operational metrics are not stationary: they have daily cycles (business hours vs. night), weekly cycles (weekday vs. weekend), yearly seasonality (holidays, events), and trend (growth or decline). A baseline that does not model these components flags every predictable peak and trough as anomalous.

- **Decompose the metric into level, trend, seasonality, and noise.** Understand which components are present; a metric with a strong daily cycle needs a model that knows the cycle, not a flat threshold.
- **Choose a model that captures the relevant patterns.** Simple moving averages work for stationary metrics; seasonal models (e.g., seasonal decomposition, STL) for periodic metrics; regression with features (time of day, day of week) for metrics driven by known factors.
- **Account for growth and decline.** A baseline learned six months ago on a system that has grown 50% will flag current normal traffic as anomalous; the model must track trend or be retrained on recent history.

### Learn The Baseline From Clean, Representative History

The baseline encodes what "normal" looks like, so it must be learned from history that is actually normal — excluding incidents, deploys, outages, and anomalies. A baseline learned from contaminated data treats the contamination as normal and fails to detect its recurrence.

- **Exclude incidents, outages, and deploys from the training window.** If the model learns from a week that included a major incident, it treats the incident pattern as normal and will not flag a similar incident later.
- **Use enough history to capture the relevant cycles.** Learning a weekly cycle requires at least several weeks of data; learning yearly seasonality requires years. Too-short history underfits the patterns.
- **Retrain or update as the system evolves.** A baseline that is never updated drifts from reality as the system grows, changes, or shifts; update on a cadence or use a model that adapts online.
- **Validate the baseline against held-out normal periods.** Before trusting the detector, confirm it does not flag known-normal history as anomalous.

### Choose An Algorithm Matched To The Metric's Characteristics

Different metrics have different statistical properties (distribution, autocorrelation, burstiness), and no single algorithm suits all. Match the algorithm to the metric.

- **For roughly normal, stationary metrics**, simple statistical methods (z-score, EWMA, control charts) work and are interpretable.
- **For seasonal metrics**, use methods that model seasonality explicitly (seasonal decomposition, STL, time-series forecasting) rather than methods that assume stationarity.
- **For bursty or heavy-tailed metrics** (error counts, rare events), methods assuming normality produce false positives; use methods suited to the distribution (Poisson for counts, robust statistics for outliers).
- **Prefer interpretable methods where responders must understand the alert.** A responder who cannot understand why an alert fired cannot act on it; an interpretable method ("this value is 5x the seasonal expectation for this hour") beats a black-box score.

### Tune Thresholds To The False-Positive Rate Responders Will Tolerate

Anomaly detection is probabilistic, and the threshold trades sensitivity against false positives. The right threshold is the one that produces a false-positive rate responders will not mute — because an alert that is muted provides no signal at all.

- **Set the threshold based on the operational false-positive tolerance.** A noisier channel (a dashboard annotation) tolerates more sensitivity; a paging channel requires few false positives or responders stop trusting it.
- **Tune on historical data with labeled incidents.** Run the detector over history where you know which periods were anomalous and tune the threshold to catch those without flooding on normal periods.
- **Use multi-window or confirmation logic to suppress transient anomalies.** A single anomalous point may be noise; requiring persistence (anomalous over a window) reduces false positives.
- **Re-tune as the system and metric evolve.** Thresholds that worked at launch drift; review false-positive and missed-anomaly rates periodically.

### Treat Anomaly Detection As Detection, Not Diagnosis

Anomaly detection flags that something deviates from expected; it does not explain why. A spike in error rate is detected; the cause (a deploy, a dependency, a bad data push) is diagnosed by the responder using logs, traces, and context. Conflating detection with diagnosis produces alerts that fire without actionable context.

- **Provide context with the alert.** Which metric, what was expected, what was observed, over what window, and how anomalous — so the responder can judge whether and how to act.
- **Correlate with changes.** An anomaly that coincides with a deploy, a config change, or a dependency event is far more actionable than one in a vacuum; surface recent changes alongside the anomaly.
- **Accept that some anomalies are benign.** A traffic spike from a marketing push is anomalous but not a problem; the detector flags it, the responder contextualizes it.

## Common Traps

### Static Thresholds Ignoring Seasonality And Trend

A flat threshold on a metric with strong daily or weekly cycles, flagging every predictable peak and trough as anomalous, or set so loose it misses real anomalies. Model the baseline with the metric's seasonality and trend.

### Baseline Trained On Contaminated Data

A baseline learned from a window that included an incident or outage, treating the contamination as normal and failing to flag its recurrence. Learn from clean, representative history excluding incidents and deploys.

### False-Positive Floods That Get Muted

Thresholds set too tight or without confirmation logic, flooding responders with false positives until the alerts are muted, after which the detector provides no signal. Tune to the operational false-positive tolerance and use confirmation logic.

### Wrong Algorithm For The Metric's Distribution

A normality-assuming method (z-score) on a bursty or heavy-tailed metric (error counts), producing false positives on natural bursts. Match the algorithm to the metric's statistical properties.

### Untuned "Turned On And Forgotten" Detection

Anomaly detection enabled with defaults and never tuned or retrained, drifting from reality as the system grows and changes. Tune on labeled history, retrain as the system evolves, and review false-positive and missed-anomaly rates.

### Detection Confused With Diagnosis

Alerts that fire ("error rate anomalous") without context or correlation, so responders know something is wrong but not what to do. Provide context (expected vs. observed, window, magnitude) and correlate with recent changes.

### Treating Every Anomaly As A Problem

Flagging benign anomalies (a marketing-driven traffic spike, a planned load test) as incidents, wasting responder attention. Contextualize anomalies; some are expected deviations, not problems.

## Self-Check

- [ ] The baseline models the metric's seasonality (daily, weekly, yearly) and trend (growth/decline), decomposing the metric into level, trend, seasonality, and noise, and using a model that captures the relevant patterns rather than a flat threshold.
- [ ] The baseline is learned from clean, representative history excluding incidents, outages, and deploys, with enough history to capture the relevant cycles, retrained or updated as the system evolves, and validated against held-out normal periods.
- [ ] The algorithm is matched to the metric's statistical properties (stationary metrics use simple statistical methods; seasonal metrics use seasonal models; bursty/heavy-tailed metrics use distribution-appropriate methods), with interpretable methods preferred where responders must understand the alert.
- [ ] Thresholds are tuned to the operational false-positive tolerance (sensitive for dashboards, strict for paging), tuned on historical data with labeled incidents, using multi-window/confirmation logic to suppress transient anomalies, and re-tuned as the system evolves.
- [ ] Anomaly detection is treated as detection, not diagnosis: alerts carry context (metric, expected vs. observed, window, magnitude), correlate with recent changes (deploys, configs, dependency events), and accept that some anomalies are benign.
- [ ] False-positive and missed-anomaly rates are reviewed periodically, and the detector is retrained/re-tuned rather than enabled once and forgotten.
- [ ] The highest-risk cases were verified — a seasonal baseline that did not flag predictable peaks, a contaminated training window excluded, a threshold tuned to a tolerable false-positive rate, and an anomaly correlated with a deploy to become actionable — not only the clean stationary-metric path.
