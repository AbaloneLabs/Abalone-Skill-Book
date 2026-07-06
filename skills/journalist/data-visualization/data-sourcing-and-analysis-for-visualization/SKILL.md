---
name: data-sourcing-and-analysis-for-visualization.md
description: Use when the agent is acquiring, cleaning, analyzing, and preparing datasets for journalistic visualization, evaluating data quality and provenance, handling missing or biased data, or deciding what a dataset can and cannot legitimately support before visualizing it.
---

# Data Sourcing and Analysis for Visualization

A visualization is only as honest as the data and analysis beneath it, and the data beneath most journalistic visualizations is messy, incomplete, biased, and collected by someone with an interest in how it is used. Crime statistics underreport; survey samples skew; administrative datasets reflect the priorities and prejudices of the agencies that compiled them; "raw data" is a myth. The journalist's job is not to pour whatever data is at hand into a chart, but to understand where the data came from, what it actually measures, what it leaves out, and what conclusions it can and cannot support. A beautiful visualization built on misunderstood or misanalyzed data is misinformation with a professional gloss. This skill covers the judgment involved in sourcing, evaluating, cleaning, and analyzing data for journalistic visualization, so that the graphic represents reality rather than distorting it.

## Core Rules

### Understand Provenance Before You Visualize

Before visualizing any dataset, understand its provenance in detail. Who collected it, why, and how? Is it a complete census, a sample, an administrative record, a model output, or a self-reported survey? What definitions and categories does it use, and have those changed over time? A dataset's origin shapes what it can tell you: police-recorded crime data reflects reporting and enforcement patterns, not just underlying crime; economic indicators use specific definitions that may not match common understanding. Document the provenance and surface it for the reader, because the data's limitations are part of the story the visualization tells.

### Evaluate Data Quality and Known Biases

Every dataset has biases and gaps, and the journalist must identify them before visualizing. Is the data complete, or are there systematic missing values (e.g., agencies that did not report, demographics that are undercounted)? Does the collection method systematically exclude or underrepresent certain groups? Are there known measurement problems or definitional shifts that affect comparability over time? Evaluate these honestly and decide whether the data can support the claim you want to visualize. If the bias is severe, either do not visualize, visualize with prominent caveats, or make the bias itself the story. Never visualize data as if it were a transparent window onto reality.

### Define What the Data Can and Cannot Support

Be precise about what a given dataset can legitimately tell you, and do not visualize claims the data cannot support. A correlation is not causation; a trend in one variable does not explain another; a snapshot does not show change. Before designing the visualization, write down the specific claim the data supports and the claim you want to make, and check that they match. If the data supports "X is associated with Y" but not "X causes Y," the visualization must reflect the weaker claim. Overstating what data shows is one of the most common and damaging errors in data journalism.

### Clean and Transform Data Transparently

Real data requires cleaning: handling missing values, standardizing formats, resolving inconsistencies, and often aggregating or transforming. Document every cleaning and transformation step, because each is an analytical choice that affects the result. Decide deliberately how to handle missing data (exclude, impute, or show as missing) and disclose the approach. Do not silently drop inconvenient data points or apply transformations that change the story without noting them. The cleaning is part of the analysis, and like all analysis, it should be reproducible and defensible. Where possible, publish the cleaned data and method so others can verify.

### Avoid Common Analytical Errors

Several analytical errors routinely distort data visualizations. Ecological fallacy: drawing conclusions about individuals from aggregate data. Simpson's paradox: a trend that appears in aggregate but reverses within subgroups. Base rate neglect: comparing raw counts without accounting for population size. Survivorship bias: analyzing only what survived a filter and ignoring what was filtered out. Confounding: attributing an effect to the wrong cause because a third variable was not considered. Learn to recognize these errors and check your analysis against them. When a finding seems striking, ask which error could be producing it.

### Normalize and Compare Appropriately

Many misleading visualizations result from comparing things that should not be directly compared, or failing to normalize. Compare rates (per capita, per unit) rather than raw counts when population differs. Adjust for inflation when comparing dollar values over time. Use consistent definitions and geographies when comparing places. Be careful with percentage changes from small bases, which can produce dramatic-looking but meaningless swings. Appropriate normalization and comparison is not optional polish; it is the difference between a chart that informs and one that misleads.

### Account for Uncertainty in the Analysis

Data analysis produces estimates, not truths, and the visualization should reflect the uncertainty in the underlying estimates. Survey data has sampling error; model projections have ranges; administrative data has measurement error. Carry uncertainty through the analysis and into the visualization, showing confidence intervals or noting margins where they affect the conclusion. A finding that is within the margin of error of "no difference" should not be visualized as a meaningful difference. Honesty about uncertainty protects the reader and the publication.

### Reproduce and Verify the Analysis

Before publishing a data visualization, reproduce the analysis independently, ideally by a second person, to confirm the numbers are correct and the method is sound. Check that the chart matches the underlying data, that aggregations are correct, and that no computational error has shaped the result. Data journalism errors are often computational, not conceptual, and they are caught by verification, not by intuition. Treat the analysis as reporting that must be checked like any other sourcing.

## Common Traps

### Visualizing Data Without Understanding Provenance

Pouring a dataset into a chart without understanding who collected it and how. This is a trap because the data's biases and definitions shape what it can tell you. Understand provenance first.

### Treating Correlation as Causation

Visualizing an association as if it proves one thing causes another. This is a trap because correlation can reflect confounding or coincidence. Visualize only the association the data supports.

### Ignoring Known Biases and Gaps

Visualizing incomplete or biased data as if it were a transparent record. This is a trap because the bias becomes invisible in a polished graphic. Identify and disclose biases.

### Comparing Incomparable Figures

Comparing raw counts across different populations, or unadjusted dollars across time. This is a trap because the comparison is meaningless or misleading. Normalize and compare appropriately.

### Silently Dropping or Transforming Data

Excluding inconvenient data points or applying transformations that change the story without disclosure. This is a trap because the cleaning becomes a hidden analytical choice. Document and disclose all steps.

### Overstating Precision

Presenting estimates as exact truths without uncertainty. This is a trap because it overstates the reliability of the finding. Show uncertainty where it matters.

### Skipping Independent Verification of the Analysis

Publishing computational results without a second check. This is a trap because data errors are often computational and invisible to intuition. Reproduce and verify before publishing.

## Self-Check

- Do you understand the dataset's provenance, including who collected it, how, and with what definitions, and is that disclosed to the reader?
- Have you identified and disclosed known biases, gaps, and measurement limitations in the data, rather than visualizing it as a transparent record?
- Does the visualization claim only what the data can support, with correlation not presented as causation and snapshots not presented as trends?
- Are all cleaning and transformation steps documented, defensible, and disclosed, with no silent exclusion of inconvenient data?
- Have you checked the analysis against common errors (ecological fallacy, Simpson's paradox, base rate neglect, confounding)?
- Are comparisons appropriately normalized (per capita, inflation-adjusted, consistent definitions) so that the comparison is meaningful?
- Is uncertainty in the estimates carried through to the visualization, with findings within the margin of error not presented as meaningful differences?
- Has the analysis been independently reproduced and verified, and for data involving legally sensitive, health, or financially consequential claims, has an editor and, where appropriate, a statistician or domain expert reviewed it, with counsel consulted where misrepresentation could cause harm?
