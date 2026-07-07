---
name: quality_risk_and_review_prioritization.md
description: Use when the agent is deciding where to focus translation review effort, prioritizing high-risk segments over low-risk ones, applying risk-based review sampling, allocating review resources by content criticality, or determining how much review each part of a translation needs based on stakes machine translation use and audience impact.
---

# Quality Risk And Review Prioritization

Review resources are always finite, and the worst mistake in translation quality is to spend them uniformly. When every segment receives the same review depth, two failures follow: high-risk segments that carry legal, safety, financial, or reputational consequence receive too little attention, and low-risk segments that carry little consequence absorb effort that should have gone elsewhere. A marketing tagline and a medical dosage instruction do not warrant the same scrutiny, and treating them as if they do is how dangerous errors slip through while reviewers polish trivia. Risk-based review prioritization is the discipline of matching review depth to consequence: identifying which segments, content types, and domains carry the highest stakes, and concentrating review effort there. This is not cutting corners. It is applying the same total effort more intelligently, so that the segments where errors cause harm receive the scrutiny they require. Agents who review linearly from start to finish, giving equal attention to each sentence, systematically under-review what matters most.

Use this skill when deciding where to focus review effort, prioritizing high-risk segments, applying risk-based sampling, or allocating review resources by content criticality. The goal is to ensure that the segments where errors cause the most harm receive the most scrutiny, and that review effort is proportional to consequence.

## Core Rules

### Assess Risk Per Segment And Content Type

Review prioritization begins with risk assessment. Not all content carries the same consequence, and review depth should reflect that.

Classify content by the consequence of error. High-risk content includes legal obligations, medical instructions and dosages, safety warnings, financial figures and disclosures, regulatory compliance text, security and access controls, and any text where an error causes harm, liability, or product failure. Medium-risk content includes procedural instructions, technical specifications, and user-facing guidance where errors cause confusion or rework but not harm. Low-risk content includes marketing prose, internal communications, and gisting content where errors are noticed but cause little consequence.

Map risk across the document or project, so review effort can be allocated by consequence rather than uniformly.

### Concentrate Full Bilingual Review On High-Risk Segments

High-risk segments require the most rigorous review: full source-target alignment by a qualified reviewer. Concentrate that effort there.

For high-risk segments, perform complete meaning alignment, checking every negation, condition, obligation, number, unit, deadline, and term against the source. Verify terminology against the termbase. Check that warnings, dosages, legal obligations, and financial figures are exactly correct. For these segments, fluency is not enough; the reviewer must confirm the target carries identical meaning and consequence to the source. A smooth but shifted high-risk segment is the most dangerous defect, because it reads well while being wrong.

Do not let high-risk segments receive the same light pass as low-risk prose. The stakes demand depth.

### Apply Lighter Review To Low-Risk Content

Low-risk content does not warrant the same depth. Apply lighter review so effort is available for what matters.

For low-risk content, a target-language read for fluency, basic accuracy, and consistency may suffice, without full source-target alignment of every segment. Sampling, where a portion of segments is reviewed in depth and the rest checked lightly, can be appropriate for large volumes of low-risk content. The goal is to catch systematic problems, such as terminology drift or style inconsistency, without expending full bilingual review on every sentence.

Releasing review capacity from low-risk content is what makes thorough review of high-risk content possible.

### Identify And Prioritize High-Risk Error Types

Within review, certain error types carry disproportionate risk. Prioritize detecting them.

High-risk error types include omitted or added negation, which reverses meaning; wrong numbers, dates, dosages, quantities, and units, which cause operational or safety failures; mistranslated modals such as must, may, shall, which change obligations and permissions; wrong actor or pronoun reference, which shifts responsibility; broken placeholders, tags, and variables, which break the product; and terminology deviations in regulated or specialized domains. These errors cause harm even when the surrounding text is fluent.

Build focused passes into review that hunt specifically for these high-risk error types, because general rereading often misses them.

### Factor Machine Translation Use Into Review Depth

When machine translation is used, review depth should reflect both content risk and MT reliability for that content and language pair.

MT tends to produce fluent errors, terminology drift, hallucinations, and silent normalization of source ambiguity. For high-risk content produced by MT, apply full post-editing review regardless of MT quality, because MT's fluent surface hides errors. For medium-risk MT content, apply targeted review of error-prone areas. For low-risk MT content, lighter review or sampling may suffice. Factor in the MT engine's known reliability for the domain and language pair: an engine strong in a domain may warrant less scrutiny than one known to struggle there.

MT use does not reduce review obligation for high-risk content; it often increases it, because MT errors are harder to detect.

### Use Sampling Strategically For Large Volumes

For large volumes where full review is infeasible, use risk-based sampling, but apply it where sampling is safe and never where it is not.

Sampling suits large bodies of low-to-medium-risk content where the goal is to detect systematic problems rather than catch every error. Define the sample size to give meaningful confidence, target error-prone segments, and extrapolate findings to assess overall quality. Sampling must not be applied to high-risk segments, because a sampled high-risk segment that happens to contain an unreviewed critical error causes harm. Where sampling reveals quality problems above threshold, escalate to fuller review of the affected content.

Sampling is a tool for efficient allocation, not a license to skip review of consequential content.

### Re-Prioritize When Review Reveals Problems

Risk-based prioritization is not static. When review reveals more problems than expected in an area, re-prioritize.

If a content type, locale, vendor, or domain shows error rates above threshold, shift review effort toward it, because elevated problems signal elevated risk. If high-risk segments are clean, capacity can move to medium-risk areas. Treat the initial risk assessment as a hypothesis to be updated by evidence, not a fixed plan.

Rigid prioritization that ignores emerging evidence either over-reviews clean areas or under-reviews deteriorating ones.

### Document The Risk Assessment And Review Plan

The risk assessment and review plan should be explicit, so review decisions are defensible and repeatable.

Record the risk classification of content, the review depth assigned to each, the sampling approach where used, and the rationale. This protects the reviewer and the program: if a defect later surfaces in low-risk content that received lighter review, the decision is documented and justifiable. If auditors or stakeholders question why some content received less review, the risk-based rationale is available.

Undocumented prioritization looks like negligence after the fact; documented prioritization looks like judgment.

## Common Traps

### Reviewing Uniformly Across All Content

Equal review depth under-reviews high-risk segments and wastes effort on low-risk ones. Match depth to consequence.

### Letting Fluent High-Risk Segments Pass Lightly

Smooth prose hides shifted meaning in high-risk content. These segments demand full source-target alignment.

### Spending Full Effort On Low-Risk Content

Over-reviewing trivia consumes capacity needed for consequential segments.

### Missing High-Risk Error Types In General Rereading

Negation, numbers, modals, and placeholders require focused passes; general reading misses them.

### Reducing Review Because MT Was Used

MT use does not reduce the obligation for high-risk content review; it often increases it.

### Sampling High-Risk Segments

Sampling is for low-to-medium-risk volume, never for segments where an unreviewed error causes harm.

### Rigid Prioritization That Ignores Evidence

Failing to re-prioritize when problems emerge leaves deteriorating areas under-reviewed.

### Undocumented Risk-Based Decisions

Without documentation, lighter review of some content looks like negligence if a defect surfaces.

## Self-Check

Before approving a risk-based review plan or completing review, verify:

- Content was classified by consequence of error, with high-risk segments, legal, medical, safety, financial, and regulatory, identified.
- High-risk segments received full bilingual review with complete meaning alignment, not a light fluency pass.
- Low-risk content received appropriately lighter review or sampling, freeing capacity for consequential segments.
- Focused passes targeted high-risk error types: negation, numbers, units, modals, actor reference, placeholders, and regulated terminology.
- Machine translation use was factored into review depth, with high-risk MT content receiving full review regardless of fluency.
- Sampling, where used, was applied only to low-to-medium-risk volume, never to high-risk segments, with escalation above threshold.
- Review priorities were re-assessed based on emerging evidence, with effort shifted toward areas showing elevated problems.
- The risk assessment and review plan are documented, making review decisions defensible and repeatable.
- No high-risk segment was reviewed less than its consequence demands, and no review capacity was spent where consequence is low.
- The review effort is proportional to risk across the entire content.
