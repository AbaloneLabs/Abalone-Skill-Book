---
name: human_evaluation_method.md
description: Use when the agent is designing or running human evaluation of machine translation, using direct assessment, adequacy and fluency scales, ranking or pairwise comparison, MQM error annotation, post-editing-based evaluation, or crowd-sourced rating, recruiting and qualifying raters, writing rating guidelines, controlling rater bias, or reporting human evaluation results with reliability.
---

# Human Evaluation Method

Human evaluation is the only method that can certify the facets of MT quality that matter most, accuracy, meaning preservation, register, and effect on the reader, and it is also the method most prone to producing noisy, biased, or meaningless numbers. The reason is that a human rating is the output of a process, and the process is fragile. Raters differ in expertise, native language, dialect, and familiarity with the domain. Guidelines differ in what they ask raters to judge and how they define the scale. Anchoring shifts scores depending on what a rater saw first. Order effects and fatigue degrade later ratings. Aggregation hides disagreement behind a mean. A poorly designed human evaluation produces a number with the authority of human judgment and the reliability of a coin flip, and because it carries the word "human," it is trusted more than it deserves. Agents miss the fragility because the rating sheet looks rigorous, and they report means without ever checking whether the raters agreed.

The harm this skill prevents is decisions built on unreliable human ratings: declaring an engine better because raters anchored on a weak baseline, accepting content because an adequacy mean crossed a threshold that no rater applied consistently, or comparing systems using a method whose inter-rater agreement was never measured. The agent's freedom is to design the evaluation, but the design must address rater qualification, task definition, bias control, and reliability measurement before any score is reported.

## Core Rules

### Choose The Evaluation Method For The Question Being Asked

Different human evaluation methods answer different questions, and choosing the wrong method wastes effort. Direct assessment, rating quality or adequacy on a continuous scale, suits estimating overall perceived quality and produces scores that correlate with automated metrics, but it is noisy and benefits from multiple raters per segment. Adequacy and fluency scales separate meaning preservation from surface readability and are useful when you need to know whether the problem is accuracy or style. Pairwise comparison or ranking is more reliable than absolute scoring because humans compare better than they calibrate, and it is the right choice for deciding which of several systems is better. MQM error annotation, classifying errors by type and severity, is the most informative method for diagnosing weaknesses and is the standard for high-stakes evaluation, but it requires trained evaluators and is expensive. Post-editing-based evaluation, measuring edit distance to a corrected reference, proxies effort rather than quality. Match the method to the question: ranking to choose among systems, MQM to diagnose, adequacy and fluency to separate meaning from style, direct assessment for broad coverage.

### Define The Rating Task With Unambiguous Guidelines

Raters cannot apply a standard they do not share. Write guidelines that define exactly what each scale point means, what counts as each error type and severity, and what the rater should ignore. Provide examples anchored to each scale point or severity level, including borderline cases, because examples communicate the standard far better than prose. Specify whether the rater sees the source, the reference, both, or neither, because each configuration measures something different: source comparison enables adequacy judgment, reference comparison enables overlap-style judgment, target-only enables fluency judgment. Pilot the guidelines on a small set, collect rater questions, and revise before scaling. Vague guidelines produce ratings that mean different things to different raters, and the mean of those ratings is meaningless.

### Qualify Raters For The Language, Domain, And Task

Rater qualification determines whether the ratings measure quality or measure rater confusion. Require native or near-native competence in the target language, and match the rater's dialect and register to the audience the content serves, because a rater from one region may flag as errors what is correct in another. For specialized domains such as medical, legal, or technical content, require domain familiarity, because a non-expert cannot judge whether terminology is correct. For MQM, require trained evaluators who can apply the error taxonomy consistently. For crowd-sourced rating, screen raters with qualification tests and exclude those who fail attention checks or who rate too fast to be reading. Unqualified raters produce confident numbers that are wrong in ways only an expert would catch.

### Control Bias: Anchoring, Order, And Scale Compression

Human ratings are systematically biased, and the design must counteract the biases rather than ignore them. Anchoring shifts all ratings depending on the first segments a rater sees; randomize segment order per rater and, where possible, interleave systems so a rater does not see all of one system first. Order and fatigue effects degrade later ratings; limit session length and rotate which content appears late. Scale compression sees raters cluster on the middle or top of a scale; counteract with clear anchors, with pairwise comparison as an alternative, and with analysis that flags raters who use only two or three scale points. Reference bias occurs when raters who see a reference drift toward it; decide deliberately whether to show references and report the choice. Name the biases you controlled and the ones you could not.

### Measure Inter-Rater Agreement Before Trusting Means

A mean rating is only as trustworthy as the agreement behind it. Before reporting any human evaluation result, compute inter-rater agreement using a method appropriate to the data: correlation or Krippendorff's alpha for continuous direct assessment, agreement or Fleiss' kappa for categorical judgments, agreement on pairwise comparisons. Low agreement means the raters were measuring different things, and the mean is not a measurement of quality but of noise. Report agreement alongside every mean, and if agreement is low, either improve guidelines and re-run, increase the number of raters per segment, or switch to a more reliable method such as pairwise comparison. Suppressing agreement because it is low is a form of fraud on the reader of the report.

### Use Multiple Raters Per Segment And Aggregate Deliberately

A single rater per segment is a single noisy observation. Use multiple raters per segment, especially for direct assessment, and aggregate by mean or median after screening for outliers and unreliable raters. Decide in advance how to handle disagreement: whether to use the mean, to require a third rater as tiebreaker, or to flag the segment as genuinely contested. Report the number of raters per segment and the aggregation rule, because these determine what the score means. Aggregation that hides genuine disagreement presents a false precision.

### Sample To Represent The Content The Engine Will Face

Human evaluation is expensive, so sampling matters. Stratify the sample across the domains, content types, lengths, and difficulty levels the engine will encounter in production, because aggregate ratings hide stratum-level failure. Oversample high-risk content such as numbers, negation, and entities, where errors concentrate. Report scores by stratum and not only as a corpus mean, so that a strong aggregate does not mask a catastrophic stratum. Document the sampling frame so the result is interpretable and reproducible.

### Separate Fluency, Adequacy, And Error Severity In Reporting

Collapsing everything into one quality score loses the information human evaluation exists to provide. Report fluency and adequacy separately when using those scales, so a reader knows whether the problem is readability or accuracy. Report MQM errors by type and severity, not only a weighted total, because the distribution of error types is what drives improvement. Connect the findings to action: which error types to address in training, which domains need human translation, which systems to deploy. Human evaluation that produces a single number and no actionable breakdown has wasted its greatest advantage over automated metrics.

## Common Traps

### Reporting Means Without Inter-Rater Agreement

A mean over low agreement is noise dressed as a measurement. Report agreement, and act on it when it is low.

### Using The Wrong Method For The Question

Ranking to diagnose, MQM to choose among systems, or direct assessment where pairwise is more reliable, wastes effort and answers the wrong question. Match method to question.

### Vague Guidelines That Raters Interpret Differently

Without anchored examples and explicit definitions, raters apply different standards and the mean is meaningless. Pilot and revise guidelines.

### Unqualified Or Unscreened Crowd Raters

Non-expert or inattentive raters produce confident wrong judgments, especially on domain terminology. Qualify, screen, and attention-check.

### Ignoring Anchoring And Order Bias

Unrandomized order and system-blocked presentation shift scores systematically. Randomize and interleave, and report what you controlled.

### Single Rater Per Segment Treated As Ground Truth

One rater is one noisy observation. Use multiple raters and aggregate with a stated rule.

### Unrepresentative Sampling Hidden By A Corpus Mean

A strong aggregate can mask catastrophic stratum-level performance. Stratify and report by stratum.

### Collapsing To A Single Score With No Breakdown

A single number discards the error-type and severity information that makes human evaluation worth its cost. Report by type and severity and connect to action.

## Self-Check

- Was the evaluation method, direct assessment, adequacy and fluency, pairwise or ranking, MQM, or post-editing based, chosen to answer the specific question being asked?
- Do the rating guidelines define each scale point and error type with anchored examples, including borderline cases, and were they piloted and revised?
- Are raters qualified for the target language, dialect, register, and domain, with screening and attention checks for crowd-sourced rating?
- Are anchoring, order, fatigue, and reference biases controlled through randomization, interleaving, session limits, and deliberate reference visibility, with controls reported?
- Is inter-rater agreement computed with an appropriate method and reported alongside every mean, with low agreement triggering re-design rather than suppression?
- Are multiple raters used per segment, with a stated aggregation rule and handling for disagreement?
- Is the sample stratified across domains, content types, lengths, and difficulty, with high-risk content oversampled and scores reported by stratum?
- Are fluency and adequacy reported separately, and are MQM errors reported by type and severity rather than collapsed to a single number?
- Do the findings connect to action: training targets, routing decisions, deployment choices?
- No human evaluation result is reported as authoritative without agreement, sampling frame, rater qualification, and bias controls documented.
