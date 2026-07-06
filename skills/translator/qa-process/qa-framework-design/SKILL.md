---
name: qa_framework_design.md
description: Use when the agent is designing or selecting an overall quality assurance framework for a translation program, defining the quality model, choosing between TQM, ISO 17100, ASTM, or MQM-based approaches, setting quality tiers, defining metrics and sampling strategy, or building the organizational system that governs how translation quality is planned, measured, and improved.
---

# QA Framework Design

A translation quality assurance framework is the architecture that decides how quality is defined, produced, measured, and improved across a program. It is not a checklist and not a single review pass. It is the set of interlocking choices: which quality dimensions matter for this content, which process stages produce and verify them, which metrics signal health, which sampling depth fits the risk, and which roles own each decision. Agents frequently reduce "QA" to "we will review the translation," which collapses an entire system into one activity and leaves the program without a defensible, repeatable, comparable notion of quality. The harm is structural: every project is assessed differently, quality scores cannot be compared, reviewers disagree because they were never given shared criteria, and improvement efforts scatter because no one owns the quality model. Designing the framework is upstream of every project; getting it wrong guarantees that downstream review, however diligent, is measuring the wrong thing in the wrong way.

Use this skill when standing up a QA program, choosing a quality model, defining quality dimensions and tiers, setting sampling and metrics, or restructuring how an organization governs translation quality. The goal is a coherent, documented framework that makes quality definable, measurable, and improvable across projects.

## Core Rules

### Start From Purpose And Risk, Not From Tools

A framework exists to serve a purpose: producing translations that are fit for their intended use at acceptable cost. Before choosing any model, metric, or tool, define what quality means for this program. Identify the content types the program handles, the audiences and locales, the regulatory or safety exposure, the brand and voice requirements, and the cost and turnaround constraints. A life-sciences program where a mistranslated dose causes harm has a fundamentally different quality problem than a high-volume e-commerce catalog where speed and consistency dominate. The framework must encode these priorities. Designing from tools first, for example "we will use this CAT tool's QA checker," produces a framework that measures what the tool measures rather than what the program needs. Purpose and risk come first; tools serve them.

### Choose A Recognized Quality Model As The Spine

Build the framework on a recognized quality model so that dimensions, terminology, and metrics are shared and defensible. Relevant models include the ISO 17100 process standard for translation services, the ASTM F2575 guide, the Multidimensional Quality Metrics (MQM) framework, the DQF/Taus quality model, and the older LISA quality model. Each defines quality dimensions such as accuracy, fluency, terminology, style, locale convention, and design, and each provides a vocabulary for severity. Adopting a recognized model means the framework inherits years of community calibration, reviewers can be trained against a shared reference, and quality data is comparable to industry benchmarks. A bespoke model invented from scratch is rarely wrong in intent but is usually under-specified, inconsistently applied, and impossible to benchmark. If a custom model is unavoidable, derive it from a recognized one and document every deviation.

### Define Quality Dimensions Explicitly And Completely

A framework must state which quality dimensions are in scope and how each is defined. At minimum, address accuracy against source, fluency and readability in the target, terminology conformance, style and register, locale conventions such as dates, numbers, and units, design and formatting, and compliance with domain-specific rules such as legal or medical phrasing. For each dimension, define what counts as conforming and what counts as a defect, with examples. Vague dimensions produce vague review: two reviewers reading "accuracy" differently will classify the same segment differently and the resulting scores will be noise. Complete definition also means deciding what is out of scope, such as source-text defects the translator cannot fix, so reviewers do not waste effort or penalize translators unfairly.

### Tier Content And Match Process To Tier

Not all content deserves the same process or the same quality bar. Define content tiers and assign each a production and verification process, a sampling depth, and an acceptance threshold. A typical tiering separates life-critical or legally binding content, which earns full bilingual and monolingual revision by qualified reviewers with one hundred percent coverage; business-critical content such as marketing or documentation, which earns sampled revision against defined thresholds; and high-volume low-risk content such as support articles or product descriptions, which may rely on post-edited machine translation with automated checks and targeted sampling. The framework must publish the tier definitions, the process for each tier, and the rule for assigning content to a tier. Without tiering, a program either over-spends on low-risk content or under-protects high-risk content, and quality becomes a single blunt setting that fits nothing well.

### Define Metrics And Make Them Comparable

Choose metrics that are defined the same way across projects and reviewers so data aggregates meaningfully. Common metrics include error count per thousand words, the MQM score derived from weighted error severities, the pass or fail rate against an acceptance threshold, the percentage of segments reviewed, and turnaround and revision effort. Define the formula for each metric, the weighting of severities, and the population over which it is computed. A metric that one project computes over all words and another computes over reviewed words only is not comparable. Publish the metric definitions and enforce them. Metrics that drift in definition over time destroy the ability to see trends, which is the whole point of measuring.

### Decide Sampling Strategy Deliberately

Full review of every word is rarely affordable and sometimes unnecessary. Decide sampling deliberately and document it. Options include one hundred percent review for high-risk content, statistically valid sampling for medium-risk content where the sample size and selection method support a defensible inference about the whole, and risk-targeted sampling that concentrates review on high-stakes segments such as warnings, legal clauses, or UI strings. The framework must state, for each tier, whether review is full or sampled, the sample size and selection rule, and what the sample result implies about acceptance of the whole. Convenience sampling, where the reviewer simply reads what is easy to reach, produces a false sense of coverage and lets systemic defects pass.

### Assign Ownership And Governance

A framework needs owners. Define who owns the quality model, who maintains the error typology and severity definitions, who calibrates reviewers, who sets and revises thresholds, and who reviews the metrics and drives improvement. Without ownership, the framework decays: dimensions get reinterpreted, metrics drift, thresholds erode under deadline pressure, and no one notices until quality complaints arrive. Governance also means versioning: when the framework changes, record the change, the rationale, and the effective date, so historical data can be interpreted in context. A framework that changes silently cannot support trend analysis.

### Make The Framework Documented And Trainable

The framework must exist as a document that translators, reviewers, project managers, and clients can read. It should cover the quality model and dimensions, the content tiers and their processes, the metrics and thresholds, the sampling rules, the roles and ownership, and the escalation path for disagreements. It should be paired with training so that people apply it consistently. An unwritten framework that lives only in a lead reviewer's head is not a framework; it is a dependency on one person, and it disappears when they do.

## Common Traps

### Collapsing The Framework Into One Review Activity

Treating QA as "we review the translation" hides every architectural decision and leaves quality undefined and unmeasured.

### Designing From Tools Instead Of Purpose

Choosing a CAT tool's built-in checker as the framework measures what the tool can detect, not what the program needs, and silently omits dimensions the tool cannot check.

### Inventing A Bespoke Model Without Reference

A from-scratch quality model is usually under-specified, inconsistently applied, and impossible to benchmark against industry data.

### Vague Dimensions Producing Noisy Review

If "accuracy" or "style" is not defined with examples, reviewers classify differently and scores become noise that cannot drive improvement.

### Single Quality Setting For All Content

Applying full revision everywhere wastes budget on low-risk content; applying light review to high-risk content invites harm. Tiering is not optional.

### Metrics That Drift In Definition

Computing the "same" metric differently across projects or over time destroys comparability and hides trends, defeating the purpose of measurement.

### Convenience Sampling Disguised As Coverage

Reviewing whatever is easy to reach gives a false sense of thoroughness and lets systemic defects in unreviewed segments pass undetected.

### No Governance Or Versioning

Without an owner and a change record, the framework erodes silently under deadline pressure and historical data becomes uninterpretable.

## Self-Check

Before approving a QA framework design, verify:

- The framework starts from the program's purpose, audiences, risk exposure, and constraints, and tools are chosen to serve those, not the reverse.
- A recognized quality model such as ISO 17100, ASTM F2575, MQM, or DQF is adopted as the spine, with any custom deviations documented and justified.
- Quality dimensions, including accuracy, fluency, terminology, style, locale, design, and domain compliance, are explicitly defined with conforming and defect examples, and out-of-scope items are stated.
- Content tiers are defined, each with a production and verification process, a sampling depth, and an acceptance threshold, plus a rule for assigning content to a tier.
- Metrics are defined by formula, severity weighting, and computation population, and are applied consistently so data is comparable across projects and over time.
- Sampling strategy is deliberate and documented per tier, with sample size, selection method, and the inference the sample supports about the whole.
- Ownership and governance are assigned for the model, typology, calibration, thresholds, and improvement, with versioning of changes.
- The framework is written down and paired with training so translators, reviewers, and project managers apply it consistently.
- No single quality setting is applied to all content, and no metric is allowed to drift in definition across projects.
- The framework makes quality definable, measurable, and improvable rather than collapsing into one review activity.
