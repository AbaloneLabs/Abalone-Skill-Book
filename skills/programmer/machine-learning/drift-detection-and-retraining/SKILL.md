---
name: drift_detection_and_retraining.md
description: Use when the agent is monitoring a deployed machine-learning model for data drift or concept drift, deciding when to retrain, designing a retraining trigger and pipeline, diagnosing model performance degradation over time, distinguishing drift from incident, or planning the operational lifecycle of a model after deployment. Also covers the failure mode of a model that silently decays as the world changes, retraining that fixes symptoms without addressing the drift cause, retraining cadence set by calendar rather than by measured drift, and the gap between deploying a model and operating it across its lifetime.
---

# Drift Detection And Retraining

A deployed model begins to decay the moment it meets the world, because the world changes. Data drift is when the input distribution shifts (the features the model sees look different from training); concept drift is when the relationship between inputs and the target shifts (the same inputs now correspond to different outcomes). Both degrade a model silently — there is no exception thrown, no health check that fails; the model keeps producing predictions, they just get worse, and the degradation is visible only if you measure it. The judgment problem is that drift is inevitable and ongoing, so operating a model is a continuous activity, not a one-time deployment. The discipline is to monitor for drift (input distribution, prediction distribution, and where labels are available, performance), to distinguish drift from a temporary incident, to trigger retraining on measured drift rather than on a calendar, and to recognize that retraining addresses the symptom — the right response depends on what kind of drift is occurring and whether the model's assumptions still hold.

Agents tend to treat deployment as the finish line and drift as a future problem. The harm appears as models that decay for months before anyone notices (because no one monitors prediction quality in production), as retraining that is triggered too late (after harm) or too often (wasting resources and churning the model), and as retraining cycles that "fix" degradation without diagnosing whether the drift is benign, a temporary incident, or a fundamental change that requires redesign. The judgment is to monitor drift continuously with the right signals for the use case, to ground retraining in measured drift and its business impact, to verify that a retrained model actually improves production before deploying it, and to recognize when drift signals a need for redesign rather than retraining. A model that is deployed and unmonitored is decaying; a model that is monitored and maintained is operating.

## Core Rules

### Monitor For Drift Continuously, With The Right Signals

Drift is invisible without monitoring, and the right signals depend on what you can observe. Input drift (feature distribution shifts) is always observable. Prediction drift (output distribution shifts) is always observable. Performance drift (the model's error rate rises) is observable only where ground truth eventually arrives. Monitor all that apply.

- **Monitor input feature distributions for drift** (population stability index, KL divergence, statistical tests, or simple summary statistics over time). A shift in inputs is the earliest signal that the world has changed.
- **Monitor prediction distribution for drift.** If the model's outputs shift markedly from baseline, either inputs shifted or the input-output relationship shifted; either way it warrants investigation.
- **Monitor performance where ground truth is available.** When labels arrive (eventually, for many use cases), track the model's live performance; this is the most direct signal of degradation and the one most tied to business impact.
- **Alert on drift, but with context.** A drift alert without context is noise; pair the alert with the magnitude, the affected features, and the downstream metric impact so the responder can judge whether it matters.

### Distinguish Drift From A Temporary Incident

Not every distribution shift is drift. A spike can be a temporary incident (a one-time event, a data pipeline bug, a seasonal peak) rather than a lasting change in the world. Retraining on a temporary incident bakes the anomaly into the model; the right response is to investigate the cause first.

- **Investigate before retraining.** When drift is detected, determine whether it reflects a real, lasting change in the world or a temporary anomaly (a bug, a one-time event, a seasonal pattern).
- **Do not retrain on contaminated data.** If the shift is from a pipeline bug or an anomalous event, retraining incorporates the anomaly; fix the cause and exclude the contaminated period.
- **Distinguish seasonal from secular change.** A recurring seasonal pattern is handled by features or by scheduled retraining around the season; a secular (lasting) change warrants retraining or redesign.

### Trigger Retraining On Measured Drift And Business Impact, Not A Calendar

A common failure is retraining on a fixed schedule (weekly, monthly) regardless of whether the model has drifted. Calendar-based retraining wastes resources when there is no drift, churns the model unnecessarily, and can still miss fast drift between cycles. Trigger retraining on measured drift tied to business impact.

- **Trigger on drift signals tied to performance or business impact.** Retrain when monitoring shows drift that is actually degrading the metric that matters, not merely when a distribution shifted.
- **Let drift severity set the urgency.** Minor drift may warrant queued retraining; severe drift with business impact warrants immediate response.
- **Calendar-based retraining is acceptable only when drift is predictable and regular** (e.g., a known seasonal pattern); otherwise, trigger on measurement.

### Verify A Retrained Model Actually Improves Production

Retraining produces a candidate model, not a guaranteed improvement. A retrained model can be worse than the incumbent (if the new data is noisy, if the drift was misdiagnosed, or if the retraining introduced a regression). Verify the candidate against the incumbent before deploying.

- **Evaluate the retrained model against the incumbent on recent, production-representative data**, not only on a stale test set. The question is whether it does better on the world as it is now.
- **Shadow or canary the retrained model before full rollout**, exactly as for a new deployment; retraining does not bypass the rollout discipline.
- **Roll back if the retrained model underperforms.** A retrained model that does not improve production is not an improvement; keep the incumbent and investigate.

### Recognize When Drift Signals Redesign, Not Retraining

Retraining addresses drift within the current model's assumptions. Some drift means the assumptions no longer hold: a key feature has become unavailable, the target definition has changed, a new confounder has appeared, or the relationship has shifted so fundamentally that the model's functional form no longer fits. In those cases, retraining on the same architecture produces a marginally better version of a now-wrong model. Recognize when drift signals the need for redesign (new features, new architecture, new target definition) rather than retraining.

- **Ask whether the model's assumptions still hold.** If a feature disappeared, the target changed meaning, or a new confounder emerged, retraining will not fix the underlying mismatch.
- **Escalate fundamental drift to a redesign rather than endlessly retraining a wrong model.** A model retrained every month that still decays is a signal that the design is wrong, not that retraining should be more frequent.

### Treat The Model Lifecycle As Continuous Operation

A model in production is an operating system with a lifecycle: it drifts, it is monitored, it is retrained or redesigned, and eventually it is retired. Plan for this lifecycle at deployment time, not when degradation is discovered.

- **Define the monitoring, retraining trigger, and ownership at deployment.** A model deployed without a monitoring and maintenance plan is a model that will decay unnoticed.
- **Define end-of-life criteria.** A model that can no longer be maintained (drifts faster than it can be retrained, depends on deprecated data) should be retired and replaced, not kept on life support.

## Common Traps

### Deployed And Unmonitored, Decay Discovered Late

A model deployed without drift monitoring, decaying for months until a downstream metric moves or a user complains. Monitor input, prediction, and (where available) performance drift continuously with contextual alerts.

### Retraining On A Calendar Regardless Of Drift

Retraining weekly or monthly whether or not the model has drifted, wasting resources when stable and missing fast drift between cycles. Trigger retraining on measured drift tied to business impact.

### Retraining On A Temporary Incident

Retraining on data contaminated by a pipeline bug, a one-time event, or an anomaly, baking the anomaly into the model. Investigate the cause before retraining; exclude contaminated data.

### Assuming Retraining Always Improves Things

Deploying a retrained model without verifying it beats the incumbent, shipping a regression. Evaluate the candidate on recent production-representative data and shadow/canary before full rollout; roll back if it underperforms.

### Treating All Drift As Retraining-Worthy

Retraining on any distribution shift regardless of whether it degrades the metric that matters, churning the model and wasting effort. Trigger on drift tied to performance or business impact, not on any statistical shift.

### Missing Fundamental Drift That Needs Redesign

Endlessly retraining a model whose assumptions no longer hold (a feature disappeared, the target changed, a new confounder emerged), producing a marginally better version of a wrong model. Recognize when drift signals redesign rather than retraining.

### No Monitoring Plan At Deployment

Deploying a model without defining who monitors it, what signals are watched, and what triggers retraining, so decay is discovered late by someone unrelated to the deployment. Define the lifecycle plan at deployment time.

## Self-Check

- [ ] Drift is monitored continuously with the right signals for the use case: input feature distributions, prediction distribution, and (where ground truth arrives) live performance, with alerts that carry magnitude, affected features, and downstream impact rather than bare notifications.
- [ ] Drift is distinguished from temporary incidents before retraining: the cause is investigated, contaminated data (pipeline bugs, one-time events) is excluded, and seasonal patterns are handled by features or scheduled retraining rather than baked into the model.
- [ ] Retraining is triggered on measured drift tied to performance or business impact (not on a calendar), with drift severity setting urgency, except where drift is predictably regular and calendar-based retraining is justified.
- [ ] A retrained model is verified against the incumbent on recent production-representative data (not a stale test set), shadowed or canaried before full rollout, and rolled back if it underperforms — retraining does not bypass rollout discipline.
- [ ] Fundamental drift (a feature disappeared, the target changed meaning, a new confounder emerged, the functional form no longer fits) is recognized as a signal for redesign rather than endless retraining of a wrong model.
- [ ] The model lifecycle is planned at deployment: monitoring, retraining triggers, ownership, and end-of-life criteria are defined then, not discovered when degradation appears.
- [ ] The highest-risk cases were verified — drift detected by monitoring before a business metric moved, a temporary incident excluded from retraining data, a retrained model verified to beat the incumbent before rollout, and fundamental drift escalated to redesign — not only the clean stable-world path.
- Is the reasoning documented clearly enough that another practitioner could review the basis and reproduce the conclusion?
