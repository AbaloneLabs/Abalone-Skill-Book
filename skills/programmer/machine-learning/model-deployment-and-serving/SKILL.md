---
name: model_deployment_and_serving.md
description: Use when the agent is deploying a machine-learning model to production, choosing a serving strategy (batch vs real-time vs streaming), designing a rollout (canary, shadow, A/B), versioning models and ensuring rollback, managing serving latency and throughput, handling model warm-up and cold starts, or integrating a model behind an API with the surrounding application. Also covers the failure mode of a model that scores well offline but fails under production latency, traffic, or distribution conditions, the difficulty of rolling back a model that has already shifted user behavior, and the operational gaps between a notebook prototype and a served system.
---

# Model Deployment And Serving

Deploying a model is the work of turning an offline artifact that scored well in a notebook into a served system that produces predictions under production conditions — at the required latency, throughput, and reliability — without harming users when it is wrong. The judgment problem is that offline evaluation and production serving measure different things. Offline evaluation asks "does this model score well on a held-out set"; production serving asks "does this model produce correct, fast, reliable predictions on real traffic, at scale, while the world drifts, and can we tell when it is wrong and recover." A model that aces offline evaluation can fail in production for reasons the evaluation never tested: it is too slow for the latency budget, it falls over under real throughput, it behaves differently on the production distribution than on the curated test set, or its predictions drive user behavior in ways that make rollback non-trivial. The discipline is to treat deployment as a staged rollout under real conditions (shadow, canary, A/B), to measure production behavior (latency, throughput, prediction distribution, downstream metric impact) rather than trusting offline metrics, to version models with clean rollback, and to recognize that a model is not deployed when it serves traffic; it is deployed when its production behavior is understood and reversible.

Agents tend to under-invest in deployment discipline because the offline metric is the visible measure of success and the serving plumbing is unglamorous. The harm appears as models that pass offline and degrade the downstream metric they were meant to improve, as latency regressions that surface only under load, and as rollouts that cannot be cleanly reversed because the model has already changed the data it predicts on. The judgment is to shadow before serving, to canary before full rollout, to measure the downstream business metric (not only model accuracy), to keep a fast rollback path, and to instrument the model in production so that drift, latency, and errors are visible. A model in production is an operating system, not a finished artifact.

## Core Rules

### Choose A Serving Strategy Matched To The Latency And Freshness Need

Models can be served in real-time (prediction on request), batch (predictions pre-computed and stored), or streaming (predictions as part of a pipeline). The choice should follow the latency and freshness requirements of the use case, not the convenience of the team.

- **Real-time serving** for low-latency, per-request predictions (ranking, fraud detection at request time) where the prediction must reflect the current context.
- **Batch serving** for predictions consumed later, where latency is not critical and throughput matters (daily recommendations, periodic scoring); batch is simpler, cheaper, and easier to operate.
- **Streaming serving** for predictions that are part of a data pipeline (anomaly scoring on an event stream) where the prediction must be computed as data flows.
- **Match the strategy to the need, not to familiarity.** Forcing a real-time service when batch would do adds operational cost; forcing batch when real-time is needed sacrifices freshness.

### Stage The Rollout Under Real Conditions

A model that scored well offline must be validated under production conditions before it serves all traffic. Stage the rollout so that production behavior is observed at low risk before full exposure.

- **Shadow before serving.** Run the new model alongside the incumbent, scoring real production traffic without serving the predictions, and compare outputs; divergence reveals where the new model differs before users are affected.
- **Canary before full rollout.** Serve the new model to a small slice of traffic, watch latency, errors, and the downstream metric, and expand only if stable; never cut over all traffic at once.
- **A/B test when the downstream metric matters.** When the goal is a business outcome (clicks, revenue, retention), compare the new model against the incumbent on that outcome, not only on model accuracy — a more accurate model can move the wrong metric.

### Measure The Downstream Metric, Not Only Model Accuracy

Model accuracy is a proxy for the thing you actually want (a business or user outcome). A model that improves accuracy but degrades the downstream metric is a failure, and you will only detect this if you measure the downstream metric during rollout.

- **Define the business metric the model is meant to move**, and instrument it so that the model's effect is observable during rollout, not inferred from accuracy after the fact.
- **Watch for metric movement in the wrong direction.** A more accurate model can degrade the business metric if it optimizes the wrong target, or if its predictions change user behavior in unexpected ways.
- **Measure long enough to see the effect.** Short observation windows miss delayed effects (retention, lifetime impact); design the A/B to capture the relevant timescale.

### Version Models And Keep A Fast, Clean Rollback

Models in production must be versioned and reversible. When a deployed model misbehaves, the ability to revert to the previous version in minutes is what contains the incident; a model that cannot be rolled back is a model that can hold the business hostage.

- **Version models and record which version served which traffic when.** Without this, you cannot diagnose which model produced a given prediction or roll back cleanly.
- **Keep the previous version warm and instantly switchable.** Rollback should be a traffic switch, not a redeploy; a cold rollback that takes as long as a deploy is not a fast rollback.
- **Treat the model, its feature pipeline, and its config as a versioned bundle.** A model is served by a pipeline (feature extraction, preprocessing, inference); rolling back the model without its matching pipeline produces garbage.

### Manage Latency, Throughput, And Resource Cost Explicitly

A served model has a latency budget, a throughput target, and a resource cost. These are operational constraints that must be measured and managed, not discovered under load.

- **Measure tail latency, not only the mean.** p95 and p99 latency, not the mean, determine whether the model meets the user-facing budget; a model with a good mean and a bad tail fails under real traffic.
- **Size for peak throughput with headroom.** Production traffic spikes (events, retries, growth); a model sized for average load will fall over at peak.
- **Manage the cost of serving.** Model serving can be expensive (GPUs, large memory footprint); track cost per prediction and optimize (quantization, distillation, batching) before cost forces a redesign.
- **Handle cold starts and warm-up.** A model that is slow on the first request (lazy loading, JIT compilation) causes latency spikes on scale-out and after deploys; warm it before it serves traffic.

### Instrument The Model In Production

A deployed model must be observable: its latency, its error rate, its prediction distribution, and its drift from training. Without instrumentation, drift and degradation are invisible until a downstream metric moves.

- **Log predictions and inputs (sampled) for audit and debugging.** When something goes wrong, you need to know what the model was asked and what it answered.
- **Monitor prediction distribution for drift.** If the distribution of predictions shifts from baseline, the input distribution may have shifted (concept drift); alert on this.
- **Track latency, error rate, and resource use as first-class metrics**, alongside the downstream business metric, so operational health is visible.

## Common Traps

### Trusting Offline Metrics Under Production Conditions

A model that scored well on a held-out set but fails on production latency, throughput, or distribution. Stage the rollout under real conditions (shadow, canary, A/B) and measure production behavior rather than trusting offline metrics.

### Serving All Traffic At Once

Cutting over all traffic to a new model without a canary, so a problem affects every user immediately. Canary first, expand only if stable, and keep rollback to a traffic switch.

### Improving Accuracy, Degrading The Business Metric

A more accurate model that moves the wrong downstream metric because it optimizes the wrong target or shifts user behavior unexpectedly. Measure the business metric during rollout, not only accuracy.

### No Fast Rollback

A model that cannot be reverted in minutes, so a misbehaving model holds the business hostage until a fix ships. Version models, keep the previous version warm, and roll back by traffic switch; treat model, feature pipeline, and config as a versioned bundle.

### Mean Latency Hiding A Bad Tail

Reporting mean latency while p95/p99 violate the user-facing budget, so the model "meets" the budget on average and fails under real traffic. Measure and manage tail latency.

### Sizing For Average Load

Sizing the serving capacity for average traffic, so the model falls over at peak (events, retries, growth). Size for peak throughput with headroom.

### Cold-Start Latency Spikes

A model that is slow on the first request after deploy or scale-out, causing latency spikes exactly when reliability matters. Warm the model before it serves traffic.

### Drift Invisible Until A Metric Moves

No instrumentation on prediction distribution or drift, so degradation is detected only when a downstream business metric has already moved. Instrument the model: log sampled predictions, monitor distribution drift, track operational metrics.

## Self-Check

- [ ] The serving strategy (real-time, batch, streaming) is matched to the latency and freshness needs of the use case, not to the team's familiarity, with batch preferred where latency is not critical for its simplicity and cost.
- [ ] The rollout is staged under real conditions: shadow scoring compares the new model to the incumbent on production traffic before serving, canary serves a small slice before full rollout, and A/B testing measures the downstream business metric when one is at stake.
- [ ] The downstream business metric (not only model accuracy) is defined, instrumented, and observed during rollout over a timescale long enough to capture delayed effects, and metric movement in the wrong direction is treated as a failure.
- [ ] Models are versioned with a record of which version served which traffic when, the previous version is kept warm for instant traffic-switch rollback, and the model, feature pipeline, and config are treated as a versioned bundle so rollback produces consistent behavior.
- [ ] Latency is managed at the tail (p95/p99, not the mean), throughput is sized for peak with headroom, serving cost per prediction is tracked and optimized, and cold starts are handled by warming the model before it serves traffic.
- [ ] The model is instrumented in production: sampled predictions and inputs are logged for audit/debugging, prediction distribution is monitored for drift with alerting, and latency/error/resource metrics are first-class alongside the business metric.
- [ ] The highest-risk cases were verified — a model that passed offline but failed production latency or distribution, a canary that caught a regression before full rollout, a rollback that was a traffic switch rather than a redeploy, and drift that was detected by monitoring before a business metric moved — not only the clean offline-evaluation path.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
