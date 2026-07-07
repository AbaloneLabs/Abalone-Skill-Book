---
name: quality_reviewer_calibration_and_consistency.md
description: Use when the agent is calibrating translation quality reviewers to produce consistent ratings, resolving disagreements between reviewers on the same content, managing inter-rater reliability, training reviewers on a shared quality model, or auditing reviewer consistency to prevent one reviewer's severity inflation or deflation from distorting quality measurement.
---

# Quality Reviewer Calibration And Consistency

Translation quality measurement is only as reliable as the reviewers who apply it. When two reviewers evaluate the same translated segment and reach different conclusions, one calling it a critical error and the other calling it acceptable, the quality score is not measuring the translation; it is measuring the reviewers' disagreement. Reviewer calibration is the discipline of aligning multiple reviewers to apply the same quality model, the same error categories, and the same severity scales consistently, so that quality scores are comparable across reviewers, projects, and time. Without calibration, quality data is noise: a vendor scored 90 by one reviewer and 75 by another on equivalent content reveals nothing about the vendor and everything about the reviewers' inconsistency.

Agents tend to miss that reviewer disagreement is the dominant source of quality measurement error, that calibration is an ongoing process not a one-time training, that severity inflation and deflation are systematic biases that distort scores, and that uncalibrated review produces quality data that cannot drive improvement because it cannot be trusted. The harm is quality decisions based on unreliable data: vendors penalized or rewarded based on reviewer variance rather than actual quality, quality trends that are invisible because the measurement is unstable, and feedback that translators dismiss because the severity ratings are inconsistent.

Use this skill when calibrating quality reviewers, resolving reviewer disagreements, managing inter-rater reliability, training reviewers on a shared quality model, or auditing reviewer consistency. The goal is to produce quality measurements that are consistent across reviewers and trustworthy as a basis for decisions.

## Core Rules

### Recognize That Reviewer Consistency Is A Prerequisite For Meaningful Quality Data

Before any quality score can inform a decision, the reviewers producing it must be consistent. A quality model, error categories, and severity scales are necessary but not sufficient; they must be applied the same way by every reviewer. Inter-rater reliability, the degree to which different reviewers agree on the same content, is the metric that determines whether quality data is meaningful.

Measure inter-rater reliability by having multiple reviewers evaluate the same sample content and comparing their ratings. Where agreement is high, the quality data is trustworthy. Where agreement is low, the data reflects reviewer variance, not translation quality. Do not report or act on quality scores from uncalibrated reviewers as if they were reliable measurements; they are opinions, not data. Establish calibration as a prerequisite before collecting quality data that will drive decisions.

### Run Calibration Sessions On Shared Samples

Calibration is achieved through structured sessions where reviewers evaluate the same content independently and then compare and discuss their ratings. These sessions surface disagreements, reveal their causes, and build shared understanding of how to apply the quality model.

Run calibration regularly: initially when onboarding new reviewers, then periodically (monthly or quarterly) to maintain alignment and address drift. Use representative samples that include the error types and severity levels reviewers are likely to encounter. After independent evaluation, compare ratings segment by segment. For each disagreement, discuss why the reviewers rated differently: did they identify different errors? Did they agree on the error but disagree on severity? Did they apply different category definitions? Resolve each disagreement by reaching a consensus rating and documenting the reasoning, which becomes calibration guidance for future cases. Calibration sessions are most effective when led by a senior reviewer or quality lead who can arbitrate and articulate the principles.

### Address Severity Inflation And Deflation As Systematic Biases

Two systematic biases distort quality scores: severity inflation (rating errors as more severe than the model intends) and severity deflation (rating errors as less severe). A reviewer who inflates severity penalizes translators and vendors unfairly; a reviewer who deflates severity lets real problems pass. Both biases make scores incomparable across reviewers.

Detect inflation and deflation by comparing each reviewer's severity distribution to the calibration consensus. A reviewer who consistently rates more errors as critical or major than the consensus is inflating; one who consistently rates fewer is deflating. Provide targeted feedback: show the reviewer their distribution alongside the consensus, discuss specific cases where their severity diverged, and clarify the severity definitions with examples. Re-calibrate after feedback to confirm the bias is corrected. Severity bias is often unconscious, so it requires data and examples to correct, not just instruction.

### Resolve Category And Boundary Disagreements

Reviewers disagree not only on severity but on which error category applies. A given issue might be classified as terminology, accuracy, or fluency depending on the reviewer's interpretation of the category boundaries. Category disagreements matter because they affect where improvement effort is directed: if terminology errors are split between terminology and accuracy categories, the terminology problem looks smaller than it is.

Resolve category disagreements by clarifying category definitions with decision rules and examples. For each category, document what it includes, what it excludes, and how to handle borderline cases. Provide decision trees: "If the error involves a wrong term, classify as terminology; if it involves a wrong meaning that is not term-related, classify as accuracy; if it involves grammar or style that does not affect meaning, classify as fluency." Use calibration sessions to test the decision rules on borderline cases and refine them. Update the quality model documentation with clarified definitions and examples as new boundary cases are resolved.

### Document Calibration Decisions As Precedent

Each calibration session produces consensus decisions on specific cases that serve as precedent for future reviews. These decisions are valuable because they encode the shared understanding that the quality model's abstract definitions cannot capture alone. Document them in a calibration knowledge base that reviewers can reference.

For each precedent case, record: the source segment, the target segment, the error identified, the category and severity assigned by each reviewer, the consensus rating, and the reasoning. Organize the knowledge base by error type and severity so reviewers can find relevant precedents when evaluating similar cases. A calibration knowledge base turns one-time discussions into reusable guidance, accelerating the calibration of new reviewers and reducing recurring disagreements.

### Audit Reviewer Consistency Over Time

Calibration is not permanent; reviewers drift over time, especially as they encounter new content types or work independently for long periods. Regular consistency audits detect drift and trigger re-calibration before it corrupts quality data.

Audit consistency by periodically including calibration samples in regular review work (blind double-review of a sample of segments) and measuring agreement. Track each reviewer's agreement rate over time. When a reviewer's agreement drops below a threshold, investigate the cause (new content type, ambiguous model guidance, personal bias) and re-calibrate. Also audit for systematic differences between reviewers that persist despite calibration, which may indicate that the quality model itself needs refinement. Consistency auditing ensures that calibration is maintained, not just established once.

### Separate Reviewer Quality From Translation Quality In Analysis

When analyzing quality data, separate the variance attributable to reviewers from the variance attributable to translations. If a vendor's quality score drops, determine whether the drop reflects worse translation or a different reviewer with different severity calibration. Without this separation, quality trends are confounded by reviewer effects.

Control for reviewer effects by ensuring the same reviewers evaluate the same vendors over time, or by statistically adjusting for known reviewer biases. When reviewer assignments change, note the change in quality reports so trends are interpreted in context. For high-stakes quality decisions (vendor selection, contract renewal), use calibrated review panels rather than single reviewers to reduce the influence of individual bias.

## Common Traps

### Acting On Quality Data From Uncalibrated Reviewers

Quality scores from uncalibrated reviewers reflect reviewer variance, not translation quality. Do not use them for vendor evaluation, feedback, or improvement decisions without first establishing calibration.

### Treating Calibration As One-Time Training

Reviewers drift over time. Calibration must be ongoing, with regular sessions and consistency audits, not a single onboarding event.

### Ignoring Severity Inflation And Deflation

Systematic severity bias makes scores incomparable across reviewers. Detect it through distribution comparison and correct it with targeted feedback and re-calibration.

### Leaving Category Boundaries Ambiguous

Without decision rules and examples for borderline cases, reviewers classify the same error differently, distorting category-level analysis. Clarify boundaries and document precedents.

### Not Documenting Calibration Precedents

Calibration discussions that are not documented are lost. Each session's consensus decisions should be recorded as reusable precedent in a knowledge base.

### Assuming Agreement Will Emerge Naturally

Reviewer agreement does not emerge without structured calibration. Without sessions, comparison, and discussion, reviewers apply the model according to their own interpretation.

### Confounding Reviewer Variance With Translation Quality In Trend Analysis

Quality trends that coincide with reviewer changes may reflect reviewer effects, not translation changes. Control for reviewer assignments when interpreting trends.

## Self-Check

- [ ] Has inter-rater reliability been measured by having multiple reviewers evaluate the same sample content, with agreement rates reported before quality data is used for decisions?
- [ ] Are calibration sessions run regularly (onboarding plus periodic) on representative samples, with independent evaluation followed by comparison, discussion, and consensus?
- [ ] Have severity inflation and deflation been detected through distribution comparison and corrected with targeted feedback and re-calibration?
- [ ] Have category boundary disagreements been resolved with decision rules, examples, and updated quality model documentation?
- [ ] Is a calibration knowledge base maintained with precedent cases (segment, error, ratings, consensus, reasoning) organized for reviewer reference?
- [ ] Are consistency audits conducted periodically to detect reviewer drift, with re-calibration triggered when agreement drops below threshold?
- [ ] In quality trend analysis, has reviewer variance been separated from translation quality, with reviewer assignments controlled or noted?
- [ ] For high-stakes decisions, are calibrated review panels used rather than single reviewers to reduce individual bias?
- [ ] Is the calibration process documented with reliability measurements, session records, precedent knowledge base, and audit results so reviewer consistency can be verified?
