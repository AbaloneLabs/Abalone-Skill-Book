---
name: quality-standards-definition.md
description: Use when the agent is defining product quality standards, setting the definition of done, establishing acceptance criteria, deciding what level of polish and correctness a feature must reach, or calibrating quality expectations across teams and feature types.
---

# Quality Standards Definition

"Quality" is one of the most used and least defined words in product development. Without a shared, explicit standard, every team and every feature operates on a different implicit definition, and the result is inconsistent: some features ship polished and reliable, others ship half-finished, and no one can explain why. The product manager's job is to make quality explicit — to define what "good enough to ship" means for each feature, in each context, and to hold the line consistently. A vague commitment to "high quality" produces neither high quality nor predictable delivery.

This skill covers the judgment needed to define, communicate, and apply quality standards that are rigorous enough to protect the user and flexible enough to fit different feature types and stages.

## Core Rules

### Make quality explicit and context-dependent, not universal

The most damaging quality mistake is treating "quality" as a single, universal bar that every feature must meet. Features differ in risk, audience, reversibility, and stage, and the appropriate quality standard differs with them. A universal standard is either too low for high-risk features or too high for low-risk experiments, and it gets ignored either way.

Define quality along dimensions that can be calibrated per feature:

- **Correctness:** does it do what it claims, including edge cases and error states? The bar is highest for features touching data integrity, money, or safety.
- **Reliability:** how often does it fail, and what happens when it does? The bar is highest for features in critical workflows users depend on constantly.
- **Performance:** how fast and responsive is it under realistic conditions? The bar is highest for real-time or high-frequency interactions.
- **Usability and polish:** how smooth and clear is the experience? The bar is highest for first-run and high-visibility surfaces.
- **Completeness:** does it cover the full intended scope, or is it partial? The bar is highest for features replacing existing functionality.

State, for each feature, which dimensions matter most and what the bar is. A feature can legitimately have a high correctness bar and a lower polish bar (an internal admin tool) or the reverse (a marketing landing page). The standard is explicit, not assumed.

### Define a shared "definition of done" that is verifiable

Quality standards fail when "done" means different things to different people. The engineer considers it done when the code is written; the designer considers it done when it matches the spec; the PM considers it done when it meets the user need. Each is partially right, and the gaps between them are where quality leaks.

Establish a shared, verifiable definition of done that covers the full lifecycle:

- Functional requirements met and verified against acceptance criteria.
- Edge cases and error states handled deliberately, not left to chance.
- Accessibility, performance, and security considerations addressed to the agreed bar.
- Instrumentation and monitoring in place so problems are detectable after release.
- Documentation, support readiness, and communication completed for user-facing changes.
- Regression risk assessed and tested.

The definition of done is a contract. It must be specific enough to verify ("acceptance criteria X, Y, Z pass") rather than aspirational ("high quality"). If it cannot be checked, it is not a standard.

### Calibrate the bar to risk and reversibility

Not every feature warrants the same investment in quality, and pretending otherwise either slows everything down or results in the standard being routinely violated. Calibrate deliberately.

- **High-risk, low-reversibility features** (data migration, security changes, irreversible actions, features for regulated industries) warrant the highest bar, including extensive testing, staged rollout, and explicit rollback plans. The cost of a defect is high and lasting.
- **Medium-risk, reversible features** (most product features) warrant a solid bar: functional correctness, key edge cases, acceptable performance, basic monitoring. Defects are fixable.
- **Low-risk, highly reversible experiments** (prototypes, learning-focused tests) warrant a lower bar focused on the specific learning goal, with explicit acknowledgment of what is not production-quality. The risk is that an experiment escapes into production usage without being upgraded.

The calibration must be explicit. An experiment shipped with low quality must be labeled as such and must not be promoted to general availability without being brought to the full bar. The most common failure is an experiment that "worked" and then gets scaled without the quality investment being made.

### Distinguish must-fix from nice-to-fix before release

Within any feature, there is a set of issues that must be fixed before release and a set that can follow. Confusing them either blocks release unnecessarily or ships with serious problems. Distinguish them using clear criteria.

- **Must-fix:** issues that cause data loss, security exposure, incorrect results in common cases, failure of a core workflow, or significant accessibility violations. These block release regardless of schedule pressure.
- **Should-fix-soon:** issues that degrade the experience but do not break core value, affecting edge cases or non-critical flows. These can ship with a documented plan to address.
- **Nice-to-fix:** polish items, minor inconsistencies, non-blocking improvements. These go to the backlog.

The discipline is resisting schedule pressure that tries to move must-fix items into should-fix. A must-fix issue shipped because of a deadline becomes a much more expensive problem after release, in support cost, trust damage, and rework.

### Make quality measurable, not just aspirational

A quality standard that cannot be measured cannot be enforced or improved. Define observable indicators for each quality dimension, even imperfect ones.

- Correctness: defect rates found in testing, defect rates reported post-release, test coverage of critical paths.
- Reliability: error rates, uptime, mean time to recovery.
- Performance: latency percentiles under realistic load, frame rates, time to interactive.
- Usability: task success rates in testing, support contact rates for the feature, qualitative feedback.

The measures do not need to be perfect; they need to be consistent enough to detect regressions and to make tradeoffs visible. A team that cannot see its quality cannot manage it.

### Involve the whole team in owning quality

Quality is not the QA team's job or the PM's job; it is the responsibility of everyone who contributes to the feature. When quality is outsourced to a downstream function, it is detected late, fixed expensively, and resented. Build quality ownership into the whole process.

- Engineers own correctness and reliability through design, automated testing, and code review.
- Designers own usability and polish through specs, review, and usability validation.
- PMs own completeness and the alignment of the feature to the user need and standard.
- QA owns systematic verification and the discovery of issues the builders missed, not the entirety of quality.

A culture where "quality is everyone's job" requires the standards to be shared and explicit, which is why the definition work matters.

### Revisit standards as the product and context evolve

Quality standards set at one stage of a product's life become inappropriate at another. A startup shipping fast to find product-market fit operates on different standards than a mature product serving enterprise customers with uptime expectations. Revisit the standards when the context changes: a new customer segment with higher expectations, a move into a regulated domain, growth that makes reliability more critical, or a strategic shift toward polish.

Failing to evolve standards leaves a team either over-investing in quality for a stage that no longer needs it or under-investing as expectations have risen without the standards catching up.

## Common Traps

### "High quality" as an uncalibrated universal bar

A single, vague commitment to quality that is never defined produces inconsistent results and gets violated under pressure. Define the dimensions and the per-feature calibration explicitly.

### The definition of done that cannot be verified

A definition of done full of aspirations ("excellent user experience," "robust performance") that cannot be checked is not a standard. Make it specific and verifiable, or it provides no protection.

### Experiments escaping into production without quality investment

A prototype or test ships at low quality, "works," and gets scaled to all users without the quality bar being applied. The result is a fragile, unpolished feature in production that erodes trust. Label experiments explicitly and require a quality upgrade before general availability.

### Schedule pressure moving must-fix to should-fix

Under deadline pressure, blocking issues get relabeled as acceptable, and the feature ships with known serious problems. The short-term relief is paid for in support cost, trust damage, and rework that exceeds the time "saved." Hold the must-fix line.

### Quality outsourced to QA

When quality is "QA's job," builders do less to ensure it, issues are found late, and QA becomes a bottleneck and a scapegoat. Distribute quality ownership across the whole team through shared, explicit standards.

### Standards frozen while the context evolves

Standards set for an early-stage product persist into a mature stage where they are inadequate, or vice versa. Revisit standards when customer expectations, regulatory context, or strategic priorities change.

### Measuring activity instead of quality

Tracking hours spent testing or number of test cases written measures activity, not quality. Measure outcomes: defect rates, reliability, performance, user-reported issues. Activity metrics create the illusion of quality management without the substance.

### Inconsistent standards across teams or features

Different teams apply different implicit standards, so the product feels inconsistent: some areas polished, others rough. Shared, explicit standards across teams produce a coherent product experience.

## Self-Check

- Have I defined quality along explicit dimensions (correctness, reliability, performance, usability, completeness) rather than as a single universal bar?
- Is there a shared, verifiable definition of done that covers the full lifecycle, including edge cases, monitoring, and support readiness?
- Have I calibrated the quality bar to the risk and reversibility of this specific feature, and made the calibration explicit?
- Did I clearly distinguish must-fix from should-fix and nice-to-fix, and am I holding the must-fix line under schedule pressure?
- Are quality dimensions measurable with observable indicators, so regressions and tradeoffs are visible?
- Is quality ownership distributed across the team, or is it implicitly outsourced to QA?
- Have I revisited the standards recently given changes in customer expectations, scale, or strategic context?
- Am I measuring quality outcomes (defects, reliability, user issues) rather than just testing activity?
- Are standards consistent across teams and features, producing a coherent product experience?
- If a serious defect ships, would the standards and process have been expected to catch it, or was the gap foreseeable?
