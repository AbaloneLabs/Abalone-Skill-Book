---
name: mt_quality_monitoring_and_improvement.md
description: Use when the agent is monitoring machine translation quality over time, tracking MT performance across content domains and language pairs, using post-editing data and quality metrics to improve an MT engine, diagnosing quality degradation, or running a continuous MT quality improvement cycle.
---

# MT Quality Monitoring And Improvement

An MT engine is not a one-time deployment; it is a system whose quality drifts, and without monitoring, that drift is invisible until it causes harm. Engines degrade when the content they translate shifts domain, when source quality changes, when the engine's training data ages, or when upstream pipeline changes alter the input. They also improve when fed good post-editing data and tuning, but only if someone captures that data and acts on it. The failure mode of unmonitored MT is silent quality decay: the engine keeps producing fluent output, the volume keeps moving, and no one notices that accuracy has eroded until complaints surface or, worse, until a fluent error reaches users in high-stakes content. MT quality monitoring and improvement is the discipline of measuring engine performance continuously, diagnosing where and why quality changes, and feeding corrections back so the engine improves rather than decays. An MT program without this cycle is a program that optimizes once and then watches quality decline without understanding why.

Use this skill when monitoring MT quality over time, tracking performance across domains and pairs, using post-editing and quality data to improve an engine, diagnosing degradation, or running a continuous improvement cycle. The goal is to keep MT quality measurable, understood, and improving rather than drifting silently downward.

## Core Rules

### Establish A Quality Baseline Before Monitoring

You cannot detect drift without a baseline. Establish the engine's quality baseline before monitoring begins.

Measure the engine's quality on a representative benchmark at deployment, using both automated metrics, for tracking consistency, and human evaluation, for assessing actual fitness. The benchmark should represent the content domains, language pairs, and content types the engine will handle in production. Record the baseline per domain and per pair, because quality varies across them. This baseline is the reference against which all future measurements detect improvement or degradation.

Without a baseline, later measurements have nothing to compare against, and drift becomes undetectable until it is severe.

### Monitor Quality Continuously, Not Just At Launch

MT quality must be monitored continuously, because drift happens between launches. Build ongoing measurement into the pipeline.

Track quality through sampled human evaluation of production output at regular intervals, through post-editing metrics such as edit distance and post-editing effort, and through automated metrics on a stable benchmark for trend detection. Monitor per domain, per language pair, and per content type, because quality can hold in one area while degrading in another. Set thresholds that trigger investigation when quality drops, so degradation is caught early rather than after harm occurs.

Monitoring only at launch assumes quality is static, which it never is in a production MT system.

### Use Post-Editing Data As The Primary Improvement Signal

Post-editing data is the richest signal for improving an MT engine, because it shows exactly where and how the engine's output was corrected. Capture and use it.

Track which segments required heavy post-editing, which error types recurred, which terminology the engine got wrong, and which domains or pairs under-performed. Aggregate this data to identify patterns: systematic terminology errors, structural failures, recurring mistranslations. Feed corrected segments back into the engine's training or tuning data, so the engine learns from its corrections. Without capturing post-editing data, the engine cannot improve from production, and every correction is wasted as a one-off fix.

An engine that never receives its corrections never improves; it only accumulates unaddressed weaknesses.

### Diagnose The Cause Of Quality Changes

When quality changes, whether improving or degrading, diagnose the cause, because effective correction depends on understanding why. Do not treat symptoms.

Quality degradation may stem from domain shift, where production content moved into a domain the engine was not trained on. It may stem from source quality change, where the source content became rougher or more ambiguous. It may stem from pipeline changes, where preprocessing or segmentation altered the input. It may stem from engine or model drift in adaptive systems. It may stem from terminology evolution, where domain terms changed and the engine's vocabulary aged. Identify the cause before applying a fix, because tuning an engine for a symptom that has a different root cause wastes effort and may introduce new problems.

### Segment Quality Analysis By Domain And Language Pair

Aggregate quality metrics hide the variation that matters. Segment analysis by domain and language pair to find where the engine succeeds and fails.

An engine may perform well in technical documentation but poorly in marketing, or well in one language pair but poorly in another. Segmented analysis reveals these patterns, allowing targeted improvement where it is needed and protecting strong areas from unnecessary changes. It also informs content routing, because an engine strong in a domain may safely handle more content there while a weak domain warrants heavier review or human translation.

Aggregate-only metrics produce averages that conceal the domains and pairs where the engine is actually failing.

### Maintain The Termbase And Feed It To The Engine

Terminology is where MT most consistently fails, and the termbase is the primary correction. Maintain it and integrate it with the engine.

Keep the termbase current as domain terminology evolves, adding new terms and updating deprecated ones. Configure the MT pipeline to apply termbase terms, so the engine uses approved terminology rather than its own variants. When post-editing reveals recurring terminology errors, add the corrections to the termbase and ensure the engine applies them. Terminology consistency is often the single largest quality gain available, because MT's terminology drift is its most visible and most damaging failure pattern.

An engine without termbase integration produces terminology chaos that no amount of post-editing can keep up with.

### Tune And Retrain Deliberately

Engine tuning and retraining are powerful but risky interventions, and they must be done deliberately with measurement before and after.

Use the diagnosed causes and post-editing data to target tuning or retraining at the domains, pairs, or error types that need it. Measure quality before and after the intervention on the benchmark, to confirm the change improved quality and did not regress it elsewhere. Avoid undisciplined retraining that changes the engine's behavior broadly without measurement, because it can fix one problem while introducing others that go undetected without before-and-after comparison.

### Close The Loop Between Monitoring And Action

Monitoring produces value only if it drives action. Close the loop between measurement and improvement.

Route quality findings to the people who can act: terminology issues to the termbase owner, domain-shift problems to routing or training, pipeline issues to engineering, and engine weaknesses to the MT team. Track whether actions taken produced the intended quality improvement. A monitoring program that produces reports but drives no action is measurement overhead without benefit, and quality continues to drift despite the data being visible.

## Common Traps

### Monitoring Only At Launch

Quality is not static; without continuous monitoring, drift goes undetected until it causes harm.

### No Quality Baseline

Without a baseline reference, later measurements cannot detect whether quality improved or degraded.

### Wasting Post-Editing Data

Corrections not captured and fed back to the engine are lost, and the engine never improves from production.

### Treating Symptoms Instead Of Causes

Tuning for a symptom with a different root cause wastes effort and can introduce new problems.

### Aggregate-Only Quality Metrics

Averages conceal the domains and language pairs where the engine is actually failing.

### Letting The Termbase Age

Terminology evolves; an aging termbase leaves the engine using outdated terms and producing drift.

### Retraining Without Before-And-After Measurement

Undisciplined tuning can fix one problem while introducing others that go undetected.

### Monitoring Without Driving Action

Reports that produce no action are overhead; quality drifts despite the data being visible.

## Self-Check

Before approving an MT quality monitoring and improvement setup, verify:

- A quality baseline was established per domain and language pair before monitoring began.
- Quality is monitored continuously through sampled human evaluation, post-editing metrics, and automated benchmark trends.
- Post-editing data is captured and used as the primary signal for identifying and correcting engine weaknesses.
- Causes of quality changes are diagnosed before fixes are applied, not treated symptomatically.
- Quality analysis is segmented by domain and language pair, not aggregated into concealing averages.
- The termbase is maintained current and integrated with the engine to enforce approved terminology.
- Tuning and retraining are deliberate, with before-and-after measurement confirming improvement without regression.
- The loop between monitoring and action is closed, with findings routed to owners and tracked for effect.
- No engine is deployed without ongoing monitoring, and no monitoring produces reports without driving improvement.
- MT quality is measurable, understood, and improving over time rather than drifting silently downward.
