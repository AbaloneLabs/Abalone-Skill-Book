---
name: evidence_synthesis_and_grading.md
description: Use when the agent is synthesizing a body of evidence across studies, grading the certainty of evidence, applying GRADE or evidence hierarchy frameworks, weighing conflicting findings, deciding whether evidence is sufficient to support a recommendation, or moving from a set of studies to a conclusion about what is known.
---

# Evidence Synthesis And Grading

A single study, however well conducted, is rarely enough to settle what is known. Decisions in medicine, policy, education, and social science depend on bodies of evidence, collections of studies that vary in design, population, setting, and quality. The judgment problem is how to integrate that heterogeneous, often conflicting body into a defensible statement about certainty, and how to move from that certainty to a recommendation. Researchers routinely fail here in two opposite directions. They either treat a pile of studies as an additive vote count, where the side with more studies wins, or they privilege one favored study and ignore the rest. Neither is synthesis.

Use this skill when combining evidence across studies, grading certainty, writing a recommendation, appraising a systematic review or meta-analysis, deciding whether evidence is strong enough to act on, or reconciling studies that disagree. The goal is to keep the agent from confusing the quality of an individual study with the certainty of a body of evidence, and from presenting a recommendation as if it followed automatically from the data when it actually depends on judgments about directness, consistency, and risk of bias. The agent has freedom to choose a synthesis framework, but must make the grading logic transparent and defensible.

## Core Rules

### Separate Study Quality From Certainty Of The Body

The quality of a single study is how well that study was designed and conducted. The certainty of a body of evidence is how confident one can be in a conclusion drawn from all relevant studies together. These are related but distinct, and conflating them produces errors in both directions.

A body of high-quality studies can yield low certainty if the studies disagree, measure different things, or address indirect populations. A body of weaker studies can, in some cases, yield moderate certainty if they converge consistently and bias is likely in a predictable direction. Grade the body, not just its best or worst member.

Identify for the body:

- the studies included and how they were found;
- the designs represented and their risk of bias;
- the consistency of direction and magnitude of effects;
- the directness to the question of interest;
- the precision of pooled estimates;
- the likelihood of publication bias;
- the magnitude of the effect;
- the presence of a dose-response gradient;
- the plausibility of residual confounding.

Each of these can raise or lower certainty, and the reasoning for each change should be explicit.

### Use Evidence Hierarchies As Heuristics, Not Laws

Evidence hierarchies rank designs, typically placing systematic reviews of randomized trials above single randomized trials, above observational studies, above expert opinion. They are useful starting points because some designs are, on average, less prone to certain biases. But they are heuristics, and rigid application produces false confidence.

A well-conducted observational study with a strong natural experiment can be more credible than a poorly conducted randomized trial with severe attrition. A meta-analysis of biased studies produces a precise but wrong pooled estimate. A single large, rigorous trial can outweigh several small, flawed ones. Use the hierarchy to orient, then judge each study and the body on its actual strengths and threats.

### Apply A Structured Certainty Framework Explicitly

Frameworks such as GRADE and its variants provide a transparent, reproducible way to move from a set of studies to a certainty rating. They force the researcher to name the factors that raise or lower certainty and to justify each decision.

In a typical GRADE-style process:

- start with a certainty level based on design, high for randomized trials, low for observational studies;
- consider lowering certainty for risk of bias, inconsistency, indirectness, imprecision, and publication bias;
- consider raising certainty for large effects, dose-response, and plausible residual confounding that would only underestimate the effect;
- arrive at a final rating of high, moderate, low, or very low certainty;
- state the conclusion and the certainty behind it together.

The value of the framework is the transparency of the reasoning, not the rating itself. Two reviewers applying the same framework to the same evidence can reasonably disagree, and the disagreement should be visible in the documented judgments.

### Detect And Handle Heterogeneity

Studies in a body rarely measure exactly the same thing in exactly the same way. Heterogeneity in effects can be real and informative, reflecting genuine differences across populations, interventions, or contexts, or it can be an artifact of methodological differences.

Investigate:

- whether heterogeneity is statistical, clinical, or methodological;
- whether subgroup differences are plausible and pre-specified or data-driven;
- whether a single outlier is driving apparent inconsistency;
- whether random-effects versus fixed-effect assumptions change the conclusion;
- whether the studies are similar enough that pooling is meaningful at all.

Pooling studies that are too different produces an average that applies to no one. Reporting only a pooled estimate while hiding large heterogeneity misleads readers about how generalizable the finding is.

### Weigh Conflicting Studies Deliberately

When studies disagree, the temptation is to pick a side. A better approach is to ask why they disagree and what that implies for certainty.

Consider:

- Are differences explained by population, setting, dose, duration, or implementation?
- Are differences explained by design quality, with stronger studies pointing one way?
- Are differences explained by bias, such as publication bias favoring early positive studies?
- Are differences within the range expected by chance given sample sizes?
- Is the conflict so fundamental that certainty must be lowered regardless of which side is larger?

A body with genuine, unexplained conflict should be graded lower in certainty than a body that converges, even if the converging body is smaller. Consistency is a property of the body, not a vote count.

### Assess The Risk Of Publication And Reporting Bias

The published body of evidence is not a random sample of the studies conducted. Studies with positive, novel, or statistically significant results are more likely to be published, cited, and included in syntheses. This distorts the apparent body of evidence.

Examine:

- whether small-study effects are visible, such as asymmetry in a funnel plot;
- whether the number of published studies is implausibly favorable given typical study power;
- whether grey literature and unpublished studies were sought;
- whether the field has a history of selective reporting;
- whether trial registries show unreported outcomes or studies.

When publication bias is likely, lower certainty even if the published studies agree. A consistent body built only from published positive studies may reflect the file drawer, not nature.

### Connect Certainty To Recommendations, But Do Not Collapse Them

A recommendation depends on more than certainty of evidence. It also depends on the balance of benefits and harms, the values and preferences of those affected, the resources required, equity, acceptability, and feasibility. High certainty that an effect exists does not automatically mean the intervention should be recommended, and low certainty does not automatically mean it should not.

Separate the judgments:

- What is the certainty that the effect exists and what is its size?
- How do benefits compare to harms and burdens?
- What do the affected people value, and does the evidence reflect their priorities?
- What are the resource and feasibility constraints?
- What are the equity implications?

A strong recommendation can be made on low-certainty evidence when benefits clearly outweigh harms and values strongly favor action. A weak or conditional recommendation can be appropriate even with high-certainty evidence when tradeoffs are close. Make the basis of the recommendation explicit rather than implying it follows mechanically from the evidence.

### Make The Synthesis Reproducible

A synthesis or grading judgment should be checkable by another reviewer. Document the search, inclusion decisions, risk-of-bias assessments, the reasoning behind each certainty adjustment, and any quantitative pooling methods. Two reviewers working independently and then reconciling differences strengthens credibility. A synthesis whose conclusions cannot be traced through its documented methods is an opinion, not evidence integration.

## Common Traps

### Voting By Study Count

Counting studies for and against a conclusion ignores sample size, quality, and bias. Ten small flawed studies do not outweigh one large rigorous one, and a majority of published studies can reflect publication bias rather than truth.

### Treating A Precise Pooled Estimate As High Certainty

A meta-analysis can produce a narrow confidence interval around a biased estimate. Precision is not accuracy. If the included studies share a common bias, the pooled result is precisely wrong.

### Hiding Heterogeneity Behind A Single Number

Reporting a summary effect while ignoring large inconsistency gives a false impression of a generalizable finding. The average may apply to none of the subgroups it spans.

### Privileging The Study That Agrees With The Researcher

Selecting the most favorable study as the basis for certainty, or discarding inconvenient studies on thin pretexts, is advocacy dressed as synthesis. Disagreement should be investigated, not suppressed.

### Equating Randomized Design With Low Risk Of Bias

Randomization reduces confounding at assignment, but poor allocation concealment, blinding failures, attrition, and selective reporting can reintroduce severe bias. Risk of bias must be assessed on conduct, not on design label alone.

### Letting Certainty Silently Determine The Recommendation

Collapsing the certainty rating into the recommendation hides the values, resource, and feasibility judgments that actually drive it. A recommendation should state the certainty of evidence and then the additional considerations separately.

### Ignoring Indirectness

Studies in different populations, using different interventions or outcomes, or measured over different timeframes provide indirect evidence. Indirectness lowers certainty even when the studies are individually strong and mutually consistent.

### Overstating Certainty In A Rapid Or Narrative Synthesis

Narrative and rapid syntheses are legitimate but carry more judgment and less transparency than full systematic reviews. Their conclusions should be hedged accordingly, not stated with the confidence of a full graded review.

## Self-Check

- Is the certainty of the body of evidence distinguished from the quality of individual studies, and is each graded appropriately?
- Is an evidence hierarchy used as a heuristic rather than a rigid rule, with actual study strengths and threats weighed?
- Is a structured certainty framework applied explicitly, with each factor that raises or lowers certainty documented and justified?
- Is heterogeneity investigated and reported, rather than hidden behind a single pooled estimate?
- Are conflicting studies examined for explanation rather than resolved by vote count, and is certainty lowered for unexplained inconsistency?
- Is the risk of publication and reporting bias assessed, and is certainty adjusted when bias is likely?
- Is the recommendation separated from the certainty rating, with benefits, harms, values, resources, equity, and feasibility made explicit?
- Is indirectness of population, intervention, comparator, or outcome identified and reflected in the certainty rating?
- Are the search, inclusion, risk-of-bias, and grading methods documented well enough for another reviewer to reproduce or challenge the conclusion?
- Does the stated certainty match the language used in the conclusion, avoiding overconfident claims from low-certainty bodies and underconfident claims from high-certainty bodies?
