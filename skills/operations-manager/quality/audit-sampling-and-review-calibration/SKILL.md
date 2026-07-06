---
name: audit-sampling-and-review-calibration.md
description: Use when the agent is designing audit samples, quality review samples, reviewer calibration, QA rubrics, operational audit coverage, risk-based review, reviewer consistency checks, or deciding whether quality evidence is representative enough to support conclusions.
---

# Audit Sampling And Review Calibration

Audit sampling and review calibration decide whether quality evidence can be trusted. A low defect rate means little if the sample avoided hard cases, reviewers applied standards inconsistently, or the rubric rewarded easy compliance over real customer impact. Agents often propose a sample size or review checklist without thinking about selection bias, risk coverage, reviewer drift, and what decision the audit is meant to support.

Use this skill before designing a QA audit, selecting review samples, interpreting review results, calibrating reviewers, or reporting quality performance from sampled work. The goal is to make review evidence representative, fair, and useful for decisions.

## Core Rules

### Start with the decision the review must support

Define why the audit exists. It may support compliance assurance, coaching, process improvement, vendor management, fraud detection, customer-impact review, control testing, new-hire ramp, incident follow-up, or leadership reporting. The purpose determines the sample and rubric.

Do not design one generic sample for every decision. A coaching sample may target recent work by an agent. A compliance sample may need risk-based coverage. A process-improvement sample may focus on defects, reopens, complaints, or edge cases.

### Choose sampling logic deliberately

Sampling methods include random sampling, stratified sampling, risk-based sampling, targeted review, full review of high-risk cases, post-incident review, complaint-triggered review, new-hire review, and exception sampling. Each has bias.

Random samples can miss rare high-harm cases. Targeted samples can overstate defect rate. Convenience samples often hide problems. The review should state what the sample can and cannot prove.

### Cover the work mix that matters

A representative review should consider work type, channel, region, customer segment, product, shift, seniority, vendor, language, complexity, risk level, age, and outcome. If only easy or recently closed work is reviewed, the audit may look healthy while high-risk areas fail.

Include old, escalated, reopened, blocked, manually overridden, and customer-complaint-driven cases when they matter to the decision. These are often where the operation's real weaknesses appear.

### Define rubrics with observable criteria

A rubric should tell reviewers what to look for and how to score it. Criteria should distinguish critical failure, major defect, minor defect, coaching note, and non-issue. Include examples of pass and fail for ambiguous cases.

Avoid rubrics that reward cosmetic compliance while missing harm. A response can use the right template and still be wrong, unsafe, unfair, or unhelpful.

### Calibrate reviewers before relying on results

Reviewers drift. Calibrate with shared cases, independent scoring, discussion of differences, updated examples, and documented standard changes. Measure inter-reviewer agreement where practical.

Calibration is especially important when review results affect performance ratings, vendor penalties, compliance reporting, or customer remediation. Inconsistent reviewers create unfairness and unreliable data.

### Separate audit findings from coaching tone

The same review may support both quality measurement and coaching, but the outputs differ. Measurement needs consistent scoring and evidence. Coaching needs actionable examples, context, and skill development.

Do not let coaching kindness weaken evidence, and do not let audit language become punitive when the finding points to process design, workload, or tool failure.

### Interpret rates with denominator and severity

A defect rate is meaningless without denominator, sample design, confidence, severity, and trend. Ten minor defects in a broad sample are different from one privacy breach in a high-risk sample. Report severity and customer impact alongside counts.

Be careful when comparing teams or vendors. Differences may reflect sample mix, work complexity, region, customer segment, reviewer strictness, or tool support rather than true performance.

### Use findings to improve the system

Audit results should lead to action: process changes, training, tool fixes, policy clarification, control redesign, staffing adjustments, vendor review, or customer remediation. Assign owners and follow-up checks.

If every audit cycle reports the same findings without change, the review program is documenting defects rather than controlling quality.

### Protect privacy, fairness, and trust

Audit samples may contain sensitive customer, employee, financial, health, security, or legal information. Limit access, redact where appropriate, and use findings fairly. Reviewers should not expose private details in coaching notes or broad reports.

When audits affect individuals, ensure the sample, rubric, and review process are fair enough to support that use. Do not use a thin or biased sample for high-stakes personnel decisions.

## Common Traps

- Choosing sample size before defining the decision the audit must support.
- Reviewing only easy, recent, visible, or convenient cases.
- Treating random sampling as sufficient for rare but high-harm defects.
- Using a rubric that checks format while missing correctness, safety, fairness, or customer impact.
- Letting reviewers score differently without calibration.
- Comparing teams or vendors without adjusting for work mix and sample design.
- Reporting defect rate without severity, denominator, confidence, trend, or customer impact; using audit results only for individual coaching when findings show process or tool failure
- Exposing sensitive data in review notes, examples, or reports; treating repeated audit findings as normal rather than forcing system improvement

## Self-Check

- Is the audit purpose clear: compliance, coaching, process improvement, vendor review, control testing, incident follow-up, or reporting?
- Is the sampling method chosen deliberately and are its limits stated?
- Does the sample cover relevant work type, channel, region, customer segment, product, shift, seniority, vendor, language, complexity, and risk?
- Are high-risk, reopened, escalated, aged, complaint-driven, or manually overridden cases included where relevant?
- Does the rubric define observable criteria, severity levels, and examples of pass and fail?
- Have reviewers been calibrated with shared cases and documented standards?
- Are audit results separated from coaching feedback where the uses differ?
- Are defect rates interpreted with denominator, severity, confidence, trend, and customer impact?
- Are findings tied to owners, corrective actions, and follow-up checks?
- Are privacy, fairness, and sample limitations controlled before using results for reporting or personnel decisions?
