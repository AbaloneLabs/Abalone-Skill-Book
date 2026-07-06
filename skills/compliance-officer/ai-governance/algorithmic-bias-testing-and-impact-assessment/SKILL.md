---
name: algorithmic-bias-testing-and-impact-assessment.md
description: Use when the agent is evaluating an AI or automated decision system for bias, designing a fairness or disparate-impact assessment, selecting protected attributes and metrics, or deciding what testing and mitigation is required before deploying a system that affects employment, credit, housing, or public access.
---

# Algorithmic Bias Testing and Impact Assessment

Automated systems can scale discrimination faster and more invisibly than any human decision-maker. The judgment problem in bias testing is not whether to test but how to design an assessment that actually detects disparate impact across the relevant protected groups, how to choose metrics that match the legal and ethical context, and how to decide what level of disparity requires mitigation versus acceptance. Agents often default to aggregate accuracy metrics that hide subgroup disparities, test only the attributes they can legally collect, or treat a single fairness metric as sufficient when fairness definitions conflict and cannot all be satisfied simultaneously.

This skill applies to AI and automated systems used in employment, credit, housing, education, healthcare, insurance, and public services, as well as to vendors supplying such systems. Anti-discrimination law, data-collection rules for protected attributes, and mandatory assessment requirements differ by jurisdiction. The legality of collecting protected-class data for testing itself varies. Verify the applicable legal framework and consult counsel before designing testing that involves protected attributes.

## Core Rules

### Frame the Assessment Around the Decision and the Affected Population

Bias testing must begin from a clear statement of what decision the system informs, who is affected, and what the stakes are. A hiring-ranking tool, a credit-eligibility model, a healthcare-triage system, and a content-moderation classifier have different affected populations, different harms, and different legal frameworks. The assessment design, including which groups to test, which metrics to use, and what disparity thresholds trigger mitigation, must be calibrated to the decision context. A generic fairness audit that is not anchored to the specific decision and its consequences produces numbers without actionable judgment.

### Identify the Protected and Vulnerable Groups Relevant to the Context

The groups to test are determined by the applicable anti-discrimination framework and by the vulnerability profile of the affected population. Protected attributes typically include race, sex, age, disability, religion, and national origin, and some jurisdictions add additional categories. Beyond legally protected groups, consider vulnerable subpopulations such as non-native language speakers, low-income applicants, rural residents, or people with atypical documentation, whose needs a generic model may systematically fail. The set of groups tested should be justified against the context, not defaulted to whatever data happens to be available.

### Resolve the Data-Collection Paradox Deliberately

Bias testing across protected groups requires knowing the protected attribute of each subject, but collecting protected-attribute data is itself restricted in many jurisdictions. This creates a paradox: the data needed to detect discrimination may be the data the law restricts collecting. Resolve this deliberately and lawfully. Options include collecting the data under a specific legal basis for the purpose of bias testing, using privacy-protective techniques such as aggregation or secure enclaves, using proxy analysis where direct collection is prohibited, or engaging an independent auditor who can collect and analyze the data under appropriate controls. Do not solve the paradox by simply omitting the attribute and declaring the system tested.

### Choose Metrics That Match the Legal Theory of Harm

Fairness is not a single metric. Common definitions include demographic parity, equal opportunity, equalized odds, calibration, and predictive parity, and these definitions can be mutually incompatible, meaning no single model can satisfy all of them. The choice of metric should match the legal theory of harm that is most relevant to the decision context. In employment selection, the relevant concern is often whether qualified applicants from protected groups are selected at rates comparable to others, which maps to disparate-impact ratio analysis. In credit, the concern may be whether error rates or approval rates differ across groups. Selecting a metric because it is easy to compute, rather than because it reflects the relevant harm, produces a clean report that misses the discrimination.

### Set Disparity Thresholds and Mitigation Triggers Before Testing

Before running the assessment, define what level of disparity will trigger mitigation and what the mitigation options are. Relying on ad-hoc judgment after seeing the results invites motivated reasoning, where a disparity is explained away because fixing it is inconvenient. Thresholds should be informed by the applicable legal standards, such as the four-fifths rule in some employment contexts, and by the severity of the harm. Define the mitigation ladder: bias-aware retraining, feature review and removal of problematic proxies, threshold adjustment across groups, human-in-the-loop review for affected decisions, or non-deployment if the disparity cannot be adequately mitigated.

### Test Across the Lifecycle, Not Just at Validation

A system that is fair at validation can drift into disparity as the input data distribution changes, as the user population shifts, or as the model is retrained. Build ongoing monitoring that re-tests subgroup performance on a defined cadence and on defined triggers such as a model update, a population shift, or an incident report. A one-time fairness audit filed before deployment and never updated is a common failure mode, because the fairness finding becomes stale as soon as conditions change.

### Document the Assessment for Defensibility

The assessment should be documented as a defensible record: the decision context, the groups tested, the data basis and its legal justification, the metrics chosen and why, the results, the thresholds and mitigation decisions, and the ongoing monitoring plan. This documentation supports internal governance, regulator inquiries, and, if relevant, a defense against a discrimination claim. Documenting only positive findings, or failing to record the metrics considered and rejected, undermines defensibility.

## Common Traps

### Aggregate Accuracy Hiding Subgroup Disparities

A model with high overall accuracy can perform dramatically worse for a protected subgroup. Reporting only aggregate metrics is the most common way bias testing fails to detect discrimination. Always decompose performance by group.

### Treating Fairness as a Single Solvable Metric

Fairness definitions conflict and cannot all be optimized simultaneously. Presenting a single metric as the definition of fairness, without acknowledging the tradeoffs, produces a misleadingly clean result. State which definition is used and why it matches the harm.

### Omitting Protected Attributes Because They Are Hard to Collect

Declaring a system bias-free because protected attributes were not collected is not a valid assessment; it is an absence of evidence. Resolve the data-collection paradox lawfully rather than using its difficulty as a reason to skip the analysis.

### Testing Only the Model, Ignoring the Deployment Context

Bias can be introduced or amplified by how the system is used: the threshold the deployer sets, the population to which it is applied, the human review process around it, and the feedback loops it creates. Test the system as deployed, not only the model as validated.

### Setting No Thresholds and Deciding After Seeing Results

Without pre-defined disparity thresholds, the temptation to rationalize an acceptable disparity is strong. Define thresholds and mitigation triggers before testing to preserve the integrity of the assessment.

### Overlooking the edge case or exception

The typical or textbook scenario is analyzed thoroughly, but the unusual case is skipped. The trap is that the standard path is well-handled while the exception silently produces the wrong outcome, because the agent stopped at the common case and never tested the boundary.

## Self-Check

- Did I anchor the assessment to the specific decision, affected population, and stakes, rather than running a generic fairness audit?
- Did I identify the protected and vulnerable groups relevant to the context, justified against the applicable anti-discrimination framework?
- Did I resolve the data-collection paradox lawfully, rather than omitting protected attributes and declaring the system tested?
- Did I choose fairness metrics that match the legal theory of harm in the decision context, and acknowledge where the definitions conflict?
- Did I define disparity thresholds and a mitigation ladder before running the assessment, rather than deciding ad hoc after seeing results?
- Did I build ongoing subgroup monitoring that re-tests on a cadence and on defined triggers, not only a one-time pre-deployment audit?
- Did I test the system as deployed, including thresholds, population, and human-review context, not only the model as validated?
- Did I document the full assessment, including metrics considered and rejected, for defensibility, and confirm the applicable legal framework with counsel?
