---
name: translation_quality_models_and_metrics.md
description: Use when the agent is defining or applying a translation quality model, using quality frameworks like DQF MQM or LISA, defining error categories and severity scales, setting quality targets and acceptance thresholds, or measuring translation quality consistently across projects vendors and content types.
---

# Translation Quality Models And Metrics

Translation quality is not a feeling. Without a defined model, every reviewer applies their own intuitive standard, and the same translation can be judged excellent by one reviewer and unacceptable by another. This subjectivity destroys the value of review: vendors cannot improve because feedback is inconsistent, quality cannot be compared across projects because the yardstick changes, and acceptance decisions become arguments rather than measurements. Quality models and metrics exist to make quality measurable, comparable, and actionable. They define what counts as an error, how severe each error is, how errors aggregate into a score, and what threshold separates acceptable from unacceptable output. Choosing and applying a model is a deliberate act: the model must fit the content type, the severity scale must reflect real risk, and the metrics must be applied consistently or they produce noise instead of signal. Agents often reach for a single number like a score and treat it as truth, but a score without a defined model behind it is just an opinion with arithmetic.

Use this skill when defining or applying a translation quality model, selecting a framework, designing error categories and severity scales, setting quality targets, or measuring quality consistently across projects and vendors. The goal is to make quality measurable, comparable, and actionable rather than subjective and inconsistent.

## Core Rules

### Choose A Model That Fits The Content And Purpose

Different quality models suit different purposes, and choosing the wrong one produces measurements that mislead. Select a model deliberately based on what you need the measurement to do.

Industry frameworks such as MQM, DQF, and LISA offer structured error typologies with categories and severity. MQM offers fine-grained categories suited to detailed analysis and vendor management. DQF integrates with productivity measurement and suits large-scale MT and post-editing programs. LISA is older and simpler but still used. Bespoke models suit organizations with specific content types and risk profiles that off-the-shelf frameworks do not capture. For quick gisting or internal content, a lighter model may suffice; for published or regulated content, a comprehensive model is required.

Match the model's granularity to the decision it must support. A model too heavy for the content wastes effort; one too light misses serious errors.

### Define Error Categories Comprehensively

A quality model's error categories determine what the review can detect. Define categories that cover the full range of translation defects.

Comprehensive categories typically include accuracy errors such as mistranslation, omission, addition, and untranslated text; fluency errors such as grammar, spelling, punctuation, and register; terminology errors such as deviation from the termbase and inconsistency; style errors against the style guide; locale convention errors such as date, number, and currency format; and technical errors such as broken placeholders, tags, formatting, and layout. Each category must be defined clearly enough that two reviewers classify the same error the same way.

Missing categories mean whole classes of error go undetected. Vague categories mean reviewers classify inconsistently, and the data becomes noise.

### Calibrate Severity To Real-World Consequence

Severity scales must reflect the real-world consequence of each error, or the score does not represent actual quality risk. Calibrate severity to impact.

A common scale uses critical, major, minor, and neutral. Critical errors cause harm, legal liability, or product failure, such as a wrong medical dose or a broken safety instruction. Major errors mislead the reader or obscure meaning, such as a mistranslated condition or a significant omission. Minor errors are noticeable but do not impede understanding, such as a style infraction or a minor fluency issue. Neutral or cosmetic errors affect presentation only. The same error type can have different severity depending on context: a terminology deviation in marketing is minor, but in a medical device label it is critical.

Severity that is not calibrated to consequence produces scores that over-weight trivial issues and under-weight dangerous ones.

### Define How Errors Aggregate Into A Score

A list of errors is not a measurement until it aggregates into a score, and the aggregation method determines what the score means. Define it explicitly.

Common approaches include penalty weighting, where each severity carries a point penalty and the score sums penalties; pass-or-fail thresholds, where exceeding a penalty limit for critical or major errors fails the batch; and score normalization per word, so quality can be compared across texts of different length. Decide whether critical errors automatically fail regardless of score, because a single critical error in safety content should fail the batch even if the rest is perfect. Decide how repeat errors are counted, because the same error repeated across segments may indicate a systematic problem worth more weight than a single instance.

An aggregation method that is undefined or inconsistent produces scores that cannot be compared or trusted.

### Set Quality Targets And Acceptance Thresholds

A quality model must connect to decisions through targets and thresholds. Define what quality level is acceptable and what triggers rejection or rework.

Set targets per content type, because different content warrants different quality bars. Published marketing content may require near-zero major errors. Internal gisting content may tolerate more. Safety, legal, and medical content may require zero critical errors and near-zero majors. Define acceptance thresholds that map to decisions: below threshold triggers rework or rejection, above threshold triggers acceptance. Communicate thresholds to vendors and translators upfront, so they know the target they are measured against.

Targets that are undefined or uniform across content types either over-spend on low-risk content or under-protect high-risk content.

### Apply The Model Consistently Across Reviewers

A quality model produces useful data only if applied consistently. Inconsistent application destroys comparability and makes the data meaningless.

Train all reviewers on the model's categories, severity, and aggregation. Provide examples of each error type and severity to anchor judgment. Run calibration exercises where multiple reviewers score the same text and reconcile differences, because reviewers naturally drift toward their own interpretations. Supply reviewers with the termbase, style guide, and brief, because an error cannot be judged without the reference it is measured against. Monitor inter-reviewer consistency over time and re-calibrate when it drifts.

A model applied inconsistently is worse than no model, because it produces false confidence in numbers that do not measure the same thing.

### Distinguish Quality Measurement From Quality Improvement

Measuring quality is not the same as improving it. A score reports the current state; improvement requires acting on the causes.

Use quality data to identify patterns: which error types recur, which vendors or locales underperform, which content types are hardest. Feed these patterns back to translators, vendors, and the termbase or style guide. Track whether quality improves over time in response to feedback. A measurement program that produces scores but drives no action is overhead without benefit.

The value of a quality model is not the score but the improvement it enables.

### Recognize The Limits Of Automated Metrics

Automated metrics such as BLEU, TER, and similar scores measure surface similarity to a reference, not actual quality. Use them within their limits.

Automated metrics are useful for MT development, where they track whether an engine improves against a reference set, and for large-scale consistency monitoring. They are not useful for judging whether a translation is correct, accurate, or fit for purpose, because a translation can score well against a reference and still be wrong, or score poorly and still be acceptable. They cannot detect critical errors such as wrong numbers, missing negation, or broken safety instructions. Use automated metrics as a supplement to human review, never as a replacement for it in content where errors cause harm.

Treating an automated score as a quality verdict in high-stakes content is a dangerous mistake.

## Common Traps

### Measuring Quality Without A Defined Model

Without a model, every reviewer applies a personal standard, and quality data is inconsistent and uncomparable.

### Choosing A Model That Does Not Fit The Content

A model too heavy wastes effort; one too light misses serious errors. Match granularity to the decision.

### Vague Or Missing Error Categories

Undefined categories let reviewers classify inconsistently, turning quality data into noise.

### Severity Detached From Consequence

Severity that does not reflect real-world impact over-weights trivial issues and under-weights dangerous ones.

### Aggregating Errors Without A Defined Method

Scores from undefined aggregation cannot be compared, trusted, or tied to decisions.

### Uniform Targets Across Content Types

Applying one quality bar to all content over-spends on low-risk material and under-protects high-risk material.

### Inconsistent Application Across Reviewers

A model applied differently by each reviewer produces false confidence in numbers that do not measure the same thing.

### Confusing Measurement With Improvement

Scores that drive no action are overhead. The value is the improvement the model enables.

### Trusting Automated Metrics As Quality Verdicts

Surface-similarity scores cannot detect critical errors and must not replace human review in high-stakes content.

## Self-Check

Before approving a translation quality model or metric for use, verify:

- A model was chosen that fits the content type and the decision the measurement must support.
- Error categories are comprehensive and defined clearly enough for consistent classification.
- Severity is calibrated to real-world consequence, with critical errors reflecting actual harm or liability risk.
- The aggregation method is defined, including how severity weights, thresholds, critical-error auto-fail, and repeats are handled.
- Quality targets and acceptance thresholds are set per content type and communicated to vendors and translators upfront.
- Reviewers are trained and calibrated on the model, with inter-reviewer consistency monitored over time.
- Quality data is used to drive improvement, with patterns fed back to translators, vendors, and assets.
- Automated metrics, if used, are treated as supplements to human review, not as quality verdicts in high-stakes content.
- The model is applied consistently across projects, vendors, and locales so data is comparable.
- No score is treated as truth without a defined model, categories, severity, and aggregation behind it.
