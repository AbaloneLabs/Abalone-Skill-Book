---
name: pattern-analysis-and-data-mining.md
description: Use when the agent is searching for patterns across large datasets for an investigation, mining records for anomalies or trends, building a data-driven thesis, assessing whether an observed pattern is meaningful or coincidental, or using quantitative analysis to surface and substantiate investigative findings.
---

# Pattern Analysis And Data Mining

Investigations increasingly run on data: thousands of contracts, millions of transactions, years of enforcement records. Pattern analysis is the discipline of finding meaningful signal in that volume, and data mining is the set of techniques used to surface anomalies, clusters, and trends that no single record would reveal. The harm this skill prevents is twofold and opposite: seeing patterns that are not really there, and missing patterns that are. A journalist who mines data without rigor manufactures false accusations from noise; one who mines without curiosity misses systemic wrongdoing hiding in plain sight. The work demands statistical humility, domain understanding, and a relentless willingness to ask whether a pattern is real, meaningful, and causally connected to the conclusion being drawn.

## Core Rules

### Start with a clear question, not just a dataset
Data mining without a question produces noise dressed as insight. Define what you are looking for, why it matters, and what a finding would look like, before running analyses. A hypothesis focuses the search; an open-ended trawl invites false positives and apophenia, the human tendency to find meaning in randomness.

### Understand the data before analyzing it
Every dataset has a history, a structure, and limits. Learn how it was collected, what each field means, what is missing, how it has been cleaned or transformed, and what biases the collection method introduces. Analysis without this grounding produces confident conclusions about data that does not mean what you think it means.

### Clean and normalize rigorously, and document every step
Real-world data is messy: duplicates, inconsistent formats, missing values, encoding errors. Clean it systematically, and record every transformation so the analysis is reproducible. Undocumented cleaning is where accidental distortion creeps in, and where challenges to your findings become unanswerable.

### Distinguish pattern from chance
Any large dataset contains apparent patterns. The question is whether a pattern is more than you would expect from random variation. Use appropriate statistical tests, compare against a meaningful baseline, and report effect sizes and uncertainty, not just whether a result is "significant." A pattern that is statistically real but tiny in magnitude may not be newsworthy.

### Establish the right comparison group
A rate means nothing without a denominator and a comparison. "X group has a high rate of Y" requires knowing the rate in comparable groups. Choosing the wrong comparison, or omitting one, manufactures false patterns. Define the comparison group deliberately and defend the choice.

### Beware the ecological fallacy and Simpson's paradox
Patterns at the group level do not necessarily hold at the individual level, and trends can reverse when data is disaggregated. A pattern found in aggregate data may disappear or invert when broken down by subgroup. Test your findings at multiple levels of aggregation before claiming a pattern holds.

### Corroborate quantitative patterns with qualitative reporting
Data surfaces the what; reporting establishes the why and the who. A statistical pattern is a lead, not a conclusion. Follow it with documents, interviews, and on-the-ground reporting that explain the mechanism, identify the actors, and confirm the pattern reflects reality rather than a data artifact.

### Subject findings to expert review before publication
Quantitative analysis in journalism should be reviewed by an independent statistician or domain expert who can challenge the methods, the comparison groups, and the interpretation. Publish your data and methods so others can reproduce and contest your findings. Transparency is the armor of data-driven investigation.

## Common Traps

### Finding patterns in noise (apophenia)
Given enough data, the human eye finds patterns everywhere. Multiple comparisons multiply the chance of false positives. Pre-register your primary question, correct for multiple testing, and demand that a pattern clear a higher bar than "it looks striking."

### The Texas sharpshooter fallacy
Firing at a barn and drawing a target around the tightest cluster of holes produces a false bullseye. Defining the pattern after seeing the data, or choosing the time window and categories to fit the finding, manufactures significance. Define categories and windows in advance or justify them independently.

### Confusing correlation with causation
Two variables moving together does not mean one causes the other. There may be a confounding factor, reverse causation, or pure coincidence. Claim correlation when that is what you have; assert causation only with a demonstrated mechanism and supporting evidence.

### Overfitting a model to the data
A complex model can be tuned to fit the data at hand perfectly while failing completely on new data. Prefer simpler models, validate on data the model has not seen, and be skeptical of findings that depend on fragile parameter choices.

### Ignoring missing and biased data
Missing records are rarely missing at random. If enforcement data is sparse for a region, that may reflect under-policing, not an absence of the underlying behavior. Investigate the mechanism of missingness before drawing conclusions from what is present.

### Cherry-picking the time window or subgroup
Choosing the start date, end date, or demographic slice that produces the most dramatic result is a form of fabrication. Show the pattern across reasonable alternative windows and groupings; if it only appears under one convenient framing, it is not robust.

### Presenting uncertain findings with false precision
A point estimate with a wide confidence interval is not a precise finding. Reporting "47.3%" when the true value could plausibly be anywhere from 30% to 65% misleads readers. Convey uncertainty honestly; precision is not the same as accuracy.

## Self-Check

- [ ] Did I begin with a defined question and hypothesis, rather than an open-ended trawl for anything striking?
- [ ] Do I understand the dataset's origin, structure, fields, missingness, and collection biases before analyzing?
- [ ] Is every cleaning and transformation step documented so the analysis is reproducible?
- [ ] Have I tested the pattern against an appropriate comparison group and baseline, not just in isolation?
- [ ] Did I check the pattern at multiple levels of aggregation to guard against ecological fallacy and Simpson's paradox?
- [ ] Have I distinguished correlation from causation, and claimed only what the data and supporting reporting support?
- [ ] Did I corroborate the quantitative pattern with documents, interviews, and on-the-ground reporting?
- [ ] Was the analysis reviewed by an independent expert, and are the data and methods available for scrutiny?
- [ ] Did I test the finding across alternative time windows and subgroup definitions rather than the most convenient one?
- [ ] Before publication, do I convey the uncertainty in the findings honestly rather than with false precision?
