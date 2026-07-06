---
name: research_result_reporting.md
description: Use when the agent is reporting research results, drafting a results section, summarizing findings for different audiences, translating statistical output into claims, or deciding how strongly to state a conclusion.
---

# Research Result Reporting

Reporting results is where evidence becomes a claim. The same data can be reported honestly or misleadingly, and the difference is rarely a single lie. It is the accumulation of small choices, how uncertainty is shown, which comparisons are emphasized, whether negative findings are buried, whether effect sizes are reported or only p-values, and whether the language matches the strength of the design. When results are reported carelessly, three harms follow. Readers and decision-makers act on conclusions stronger than the evidence supports. The literature fills with overstated findings that later fail to replicate. And the researchers themselves lose the ability to see where their own confidence is unjustified.

The agent should use this skill when drafting a results or findings section, writing an abstract, preparing a report for a funder or client, summarizing a study for a non-specialist audience, or deciding how to phrase a conclusion. The goal is to keep the agent from converting ambiguous evidence into confident claims and from hiding the uncertainty that readers need in order to act wisely.

## Core Rules

### Report The Question And Design Before The Numbers

A result is only meaningful relative to the question it answers and the design that produced it. Before presenting numbers, restate the research question, the primary outcome, the comparison, and the design type. A reader who does not know whether a study was experimental or observational cannot judge whether a reported association is causal.

State whether the analysis was planned before or after seeing the data. A result from a predefined analysis carries different weight than one found by searching the data. Label exploratory analyses as exploratory.

### Report Effect Sizes And Confidence Intervals, Not Just Significance

A p-value tells whether a result is surprising under a null hypothesis. It does not tell how large an effect is, how precise the estimate is, or whether the effect matters in practice. Report effect sizes with their uncertainty, such as confidence intervals, credible intervals, or prediction intervals, for every primary result.

For each primary finding include the following.

- The estimate of the effect or relationship.
- A measure of uncertainty around it.
- The direction of the effect.
- The units in which it is expressed.
- The practical or clinical importance, where relevant.

Avoid reporting only whether a result was statistically significant. A tiny effect can be statistically significant in a large sample, and a large effect can be non-significant in a small one. Significance is not importance.

### Distinguish Statistical Significance From Practical Importance

Statistical significance answers whether an effect is detectable beyond chance given the sample. Practical importance answers whether the effect is large enough to matter for decisions, policy, treatment, or theory. These are different questions.

A study can find a statistically significant difference that is too small to care about. A study can fail to reach significance yet show an effect large enough to matter but underpowered to confirm. Report both the detectability and the magnitude, and discuss what threshold would count as important in the context of the field.

### Report Negative And Null Findings Fully

The temptation to emphasize positive results and minimize null ones produces publication bias and distorts the evidence base. Report planned analyses that found no effect with the same care as positive findings. State the effect size and confidence interval, not merely that the result was non-significant, because non-significance is not evidence of no effect.

If a study was underpowered to detect the expected effect, say so. A non-significant result from an underpowered study is inconclusive, not negative. Conflating the two misleads readers about whether an intervention works.

### Match Language To The Strength Of Evidence

The words used to describe a result imply a level of confidence. Use language calibrated to the design and the evidence.

- Experimental designs with randomization can support causal language.
- Observational designs with strong assumptions can support cautious causal language with stated assumptions.
- Cross-sectional or correlational designs support associational language only.
- Exploratory findings support hypothesis-generating language.
- Single small studies support tentative language.

Phrases such as proves, demonstrates, confirms, or establishes imply strong evidence and should be reserved for designs that warrant them. Phrases such as suggests, is associated with, or is consistent with are safer for weaker designs. Overclaiming in language is a form of misreporting even when the numbers are correct.

### Disclose Limitations Honestly And Specifically

Every study has limitations. Reporting them honestly helps readers calibrate trust and helps the field avoid overreliance on any single study. Limitations are not a confession of failure; they are part of accurate reporting.

Cover the following limitation categories.

- Sampling and generalizability, who was included and who was not.
- Measurement, whether the measures captured the intended constructs.
- Design, what threats to validity remain.
- Confounding, what alternative explanations survive.
- Missing data and attrition.
- Statistical power and multiple testing.
- Researcher positionality and conflicts for qualitative work.

Vague statements that the study has limitations are not useful. Name the specific limitation and its likely direction of effect on the conclusion.

### Show The Data, Not Just Summaries

Where possible, show distributions, not just means. Show individual data points alongside summary statistics. A mean and a confidence interval can hide bimodality, outliers, or subgroups that behave differently. For qualitative work, provide enough excerpt or evidence to let readers judge the interpretation, not just the researcher's conclusion.

For visual reporting, choose chart types that show the data honestly. Avoid truncated axes that exaggerate differences, avoid dual axes that imply false relationships, and avoid chart types that obscure variability.

### Separate Results From Interpretation

In a formal report, the results section describes what was found. The discussion interprets what it means. Mixing the two lets interpretation leak into the factual record. Keep the description of findings distinct from the claims about their meaning, so that readers can follow the evidence chain and disagree with the interpretation if they choose.

### Report Reproducibility Information

For the result to be checkable, report the tools and conditions that produced it. Include software names and versions, analysis code references, data availability, random seeds where relevant, and the analysis plan or preregistration if one exists. A result that cannot be traced to its method cannot be trusted or built upon.

## Common Traps

### Equating Non-Significance With No Effect

A non-significant p-value does not prove the absence of an effect. It often means the study could not detect it. Reporting no effect when the study was underpowered is a common and serious error.

### P-Hacking Through Selective Reporting

Running many tests and reporting only the significant ones inflates false positives. Report all planned analyses and label post hoc ones. Multiplicity correction is not optional when many tests were run.

### Hiding Uncertainty Behind Clean Numbers

Reporting a single estimate without a confidence interval or standard error hides the imprecision that readers need. Always pair estimates with uncertainty.

### Spinning Weak Results As Strong

Using words like robust or compelling for a marginal finding, or describing an underpowered non-significant result as trending toward significance, misleads readers. Calibrate language to evidence.

### Burrying Negative Findings In Supplementary Material

Moving unwanted null results to supplements while foregrounding positive ones distorts the message. Planned primary outcomes belong in the main text regardless of direction.

### Overgeneralizing From A Convenience Sample

Reporting conclusions as if they apply broadly, when the sample was drawn from a narrow or self-selected group, overstates external validity. State the population to which results can reasonably generalize.

### Ignoring Multiple Outcomes And Subgroup Analyses

When many outcomes or subgroups were examined, the chance of a false positive rises. Disclose all tests and apply correction. Selective subgroup reporting is a classic form of p-hacking.

### Confusing Clinical And Statistical Significance In Reporting

A statistically significant result with no practical importance, reported as a breakthrough, wastes attention and can drive bad decisions. Always connect statistics to practical meaning.

## Self-Check

- Are the research question, design, and planned versus exploratory analyses stated before the numbers?
- Are effect sizes and confidence intervals reported for primary results, not just p-values?
- Is statistical significance distinguished from practical importance in the discussion?
- Are null and negative findings reported with the same completeness as positive ones?
- Does the language match the strength of the design, avoiding causal claims from non-causal designs?
- Are limitations named specifically with their likely directional effect on conclusions?
- Are data shown in a way that reveals distributions and variability, not just summary points?
- Are results separated from interpretation so the evidence chain is clear?
- Is reproducibility information, including software, code, data availability, and preregistration, reported?
- For high-stakes findings that may influence policy, treatment, or public behavior, has a qualified methodologist or domain expert reviewed the reporting before release?
