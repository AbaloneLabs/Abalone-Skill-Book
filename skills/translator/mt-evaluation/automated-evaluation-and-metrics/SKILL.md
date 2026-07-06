---
name: automated_evaluation_and_metrics.md
description: Use when the agent is running automated MT evaluation pipelines, computing BLEU, chrF, COMET, BERTScore, TER, or YiSi, building or selecting test sets and challenge sets, setting up regression testing across retraining cycles, normalizing and tokenizing text before scoring, interpreting score deltas, or automating quality gates in a continuous MT pipeline.
---

# Automated Evaluation And Metrics

Automated evaluation is the cheap, repeatable layer of MT quality assessment that lets you score thousands of segments in seconds and track an engine across every retraining cycle. Its strength is exactly its weakness: it is automated, so it is blind to everything a metric cannot capture. A well-built automated evaluation pipeline catches regressions early, ranks candidate engines consistently, and frees human effort for the segments that need it. A poorly built one produces numbers that move for reasons unrelated to quality, hides regressions behind stable aggregates, and lulls the team into trusting a gate that passes fluent wrong text. The craft of automated evaluation is not in running a metric, which is trivial, but in controlling every condition that the metric is sensitive to: tokenization, normalization, casing, punctuation, reference choice, test set composition, and metric version. Get those wrong and the scores are not comparable across runs, and non-comparable scores are worse than no scores because they invite confident wrong decisions. Agents miss this because the tool prints a number and moves on, and the conditions that produced the number are invisible in the output.

The harm this skill prevents is regression pipelines that miss real regressions, score deltas that reflect preprocessing changes rather than engine changes, and quality gates that pass content no human would accept. The agent's freedom is to design the automated layer, but it must be treated as a triage and monitoring tool, never as a certification of accuracy.

## Core Rules

### Fix Every Condition The Metric Is Sensitive To

Automated metrics are exquisitely sensitive to preprocessing, and inconsistent preprocessing makes scores incomparable across runs. Fix, in code and in documentation, every condition that affects the score: tokenization scheme, normalization form, casing, punctuation handling, stripping of markup and placeholders, and segment ordering. Apply identical preprocessing to candidate and reference. When you change any preprocessing step, every historical score becomes incomparable and you must re-baseline. Treat the preprocessing pipeline as part of the metric, version it, and record its version with every score. A regression alarm that fires because tokenization changed is a false alarm that erodes trust in the pipeline; a regression that fails to fire because two runs used different normalization is a silent failure that ships worse output.

### Pin Metric Versions And Model Checkpoints

Metric implementations and the underlying models drift. A COMET score computed with one checkpoint is not comparable to one computed with another, even if the number looks similar. BLEU implementations differ in smoothing and tokenization across libraries. Pin the exact metric version, library version, and model checkpoint in your pipeline, and record them with every score. When you upgrade a metric, re-score a held-out historical set to measure the shift and re-baseline. Never compare a score from the old version to a score from the new version and call it a regression or an improvement; that is a comparison of metrics, not of engines.

### Build Test Sets That Represent Production Content

An automated evaluation is only as meaningful as its test set, and a test set that does not resemble production content measures the engine on the wrong distribution. Build and curate test sets that span the domains, content types, registers, and lengths the engine will face, and hold them fixed across runs so scores are comparable. Include challenge sets, targeted test suites that probe specific phenomena such as negation, long-distance agreement, numerical expressions, named entities, idioms, and morphology, because corpus-level metrics hide weaknesses that challenge sets expose. Refresh the test set periodically to reflect production drift, but keep a stable core for longitudinal comparison. Document the test set provenance, composition, and refresh policy alongside the scores.

### Use Multiple Metrics And Read Their Divergence

No single automated metric captures quality; each captures a facet. Run a panel: a character or word overlap metric such as chrF for surface match, a semantic metric such as COMET for meaning proximity, and an effort metric such as TER for post-editing cost. Read the agreement and divergence among them as signal. If overlap drops while a semantic metric holds, the engine may be paraphrasing more, which is not necessarily worse. If a semantic metric holds while a challenge set shows negation errors rising, the engine is producing fluent wrong text that the corpus metric cannot see. A single headline metric hides these patterns; a panel with interpretation reveals them.

### Set Regression Thresholds From Observed Variance, Not Gut Feel

Regression alarms need thresholds, and thresholds pulled from intuition produce either constant false alarms or missed regressions. Measure the run-to-run variance of your metrics on a stable engine and baseline, then set thresholds at a level that exceeds normal noise, expressed as a percentage or absolute delta with a stated confidence. Distinguish statistically meaningful deltas from noise, and report confidence intervals where the metric supports them. A gate that alarms on every 0.2 BLEU wobble will be ignored; a gate that never alarms will ship regressions. Calibrate against observed variance and re-calibrate when the engine or test set changes.

### Stratify Reporting By Domain And Content Type

Corpus-level aggregates hide stratum-level regressions. An engine can hold its aggregate score while degrading sharply on a domain that matters, because strong performance elsewhere averages it out. Report scores by stratum: by domain, content type, length band, and challenge-set phenomenon. Make the stratum-level view the primary diagnostic, with the aggregate as a summary. This is what turns automated evaluation from a gate into an improvement tool, because it tells you where to direct training effort.

### Automate Quality Gates With Explicit Fallback To Humans

Automated metrics can gate low-stakes routing decisions, such as whether to send content to raw MT or light post-editing, but they must not be the final authority on accuracy-critical content. Design gates with explicit fallback: when a metric drops below threshold, route to human review rather than auto-rejecting or auto-approving; when content is high-stakes, bypass the automated gate entirely and require human evaluation. Document what each gate decides and what it does not decide, so no one mistakes an automated pass for a certification of accuracy. The automated layer triages; humans decide on accuracy.

### Log Everything And Make Runs Reproducible

An automated evaluation result is only defensible if it can be reproduced. Log the engine version or checkpoint, the test set version, the metric versions and model checkpoints, the preprocessing pipeline version, the random seeds, and the full score breakdown by stratum. Store the candidate outputs alongside the scores so a result can be re-checked without re-running the engine. Reproducibility is what separates an evaluation pipeline from a number generator, and it is what lets you investigate a regression months later when the conditions have changed.

## Common Traps

### Comparing Scores Across Different Preprocessing

Tokenization, normalization, or casing changes shift scores independently of quality. Fix preprocessing, version it, and re-baseline on change.

### Upgrading A Metric And Treating Old Scores As Comparable

Metric and checkpoint drift make old and new scores incomparable. Pin versions and re-score a historical set on upgrade.

### Evaluating On A Test Set Unlike Production Content

A test set that does not match the production distribution measures the wrong performance. Curate representative and challenge sets.

### Trusting A Single Headline Metric

One metric hides what it is blind to. Run a panel and interpret divergence among metrics.

### Thresholds Set By Intuition Instead Of Measured Variance

Uncalibrated thresholds produce false alarms or missed regressions. Set thresholds from observed run-to-run variance.

### Aggregate Scores Hiding Stratum Regressions

A stable corpus score can mask sharp degradation on a domain that matters. Report and diagnose by stratum.

### Letting An Automated Gate Certify Accuracy

Automated metrics pass fluent wrong text. Gates triage; humans certify accuracy, especially for high-stakes content.

### Non-Reproducible Runs

Without logged versions, seeds, and stored outputs, a score cannot be re-checked. Reproducibility is the difference between a pipeline and a number.

## Self-Check

- Is every preprocessing condition, tokenization, normalization, casing, punctuation, markup handling, fixed, versioned, and recorded with every score?
- Are metric versions, library versions, and model checkpoints pinned, with re-baselining performed on any upgrade?
- Does the test set represent production content across domains, types, registers, and lengths, with challenge sets probing specific phenomena, and is its composition documented?
- Are multiple metrics capturing different facets run as a panel, with agreement and divergence interpreted rather than a single headline number reported?
- Are regression thresholds derived from measured run-to-run variance on a stable baseline, with confidence intervals where supported?
- Are scores reported by stratum, domain, content type, length, and challenge phenomenon, so stratum-level regressions are visible and not averaged away?
- Do automated gates route to human review on threshold breach and bypass for high-stakes content, with their decision scope documented?
- Are engine version, test set version, metric and checkpoint versions, preprocessing version, seeds, and full stratum breakdown logged, with candidate outputs stored for reproducibility?
- No automated score is reported or used as proof of accuracy, and no comparison is drawn across differing conditions.
- The pipeline is reproducible end to end from its logged conditions.
