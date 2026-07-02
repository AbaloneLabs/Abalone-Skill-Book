---
name: algorithmic_bias_testing_and_fairness_audits.md
description: Use when the agent is testing AI systems for bias, conducting algorithmic fairness audits, evaluating disparate impact across protected groups, selecting fairness metrics, or ensuring compliance with anti-discrimination laws and AI fairness requirements in automated decision systems, credit scoring, hiring, and insurance.
---

# Algorithmic Bias Testing And Fairness Audits

Algorithmic bias testing and fairness audits governs how organizations detect, measure, and mitigate unfair discrimination in automated decision systems. The defining feature is that bias can enter at every stage—data collection, labeling, model training, deployment, and feedback loops—and that fairness is not a single mathematical property but a set of competing definitions that cannot all be satisfied simultaneously. The central difficulty is that a system can appear accurate overall while systematically failing for subgroups, that fairness metrics conflict with each other, and that the legal standards for algorithmic discrimination are evolving across anti-discrimination, credit, employment, and insurance law.

Use this skill before advising on bias testing methodology, fairness metric selection, audit scope, or disparate impact compliance. The goal is to make the agent identify the protected groups at risk, the appropriate fairness definitions, the testing methodology, and the mitigation strategy before concluding that an algorithmic system is fair.

## Core Rules

### Identify Protected Groups And Decision Context

Bias testing begins with understanding who is affected and how.

Map:

- the protected characteristics relevant to the jurisdiction (race, sex, age, disability, religion, national origin, and others);
- proxy variables that correlate with protected characteristics (zip code, name, education);
- the decision context (credit, employment, housing, insurance, criminal justice, healthcare);
- the outcome variable and how it is defined;
- the stakes of the decision (high-stakes vs. low-stakes);
- the historical context of discrimination in the domain;
- the affected population and subgroups within it.

Protected characteristics vary by jurisdiction but commonly include race, sex, age, disability, religion, and national origin. Proxy variables can recreate protected characteristics even when they are excluded. The decision context determines the applicable anti-discrimination law and the stakes. High-stakes decisions (credit denial, hiring, criminal sentencing) require more rigorous testing. Historical discrimination in the domain means the training data may encode past bias.

### Select Appropriate Fairness Definitions And Metrics

Fairness is not unitary; definitions conflict.

Understand:

- demographic parity (equal selection rates across groups);
- equal opportunity (equal true positive rates);
- equalized odds (equal true and false positive rates);
- predictive parity (equal positive predictive value);
- calibration (equal calibration across groups);
- individual fairness (similar individuals treated similarly);
- the impossibility theorems (cannot satisfy multiple definitions simultaneously except in edge cases);
- which definition aligns with the legal and ethical requirements.

Different fairness definitions are appropriate for different contexts. Demographic parity may be required where historical representation is the goal. Equal opportunity focuses on not missing qualified candidates from disadvantaged groups. The impossibility results (Kleinberg, Chouldechova) show that equalized odds, predictive parity, and calibration cannot all hold simultaneously when base rates differ across groups. The choice of metric must be justified by the legal and ethical context, not selected for convenience.

### Design Rigorous Testing Methodology

Testing must be methodologically sound.

Implement:

- a held-out test set that is representative of the affected population;
- sufficient sample sizes for each subgroup to detect differences;
- intersectional analysis (not just single-axis: race and sex, disability and age);
- the distinction between training, validation, and testing data;
- pre-deployment testing and post-deployment monitoring;
- statistical significance testing to distinguish real disparities from noise;
- confidence intervals around fairness metrics;
- the handling of small subgroup samples;
- documentation of methodology, data, and results.

Testing methodology determines whether bias is detected. Small subgroup samples can hide or exaggerate disparities. Intersectional analysis is critical: a system may appear fair by race and by sex separately while disadvantaging women of color. Statistical significance distinguishes real disparities from sampling noise. Methodology and results must be documented for regulatory and legal defense.

### Test For Both Direct And Indirect Discrimination

Bias operates through multiple pathways.

Test:

- direct discrimination (protected characteristics used as inputs);
- indirect discrimination (facially neutral variables that proxy for protected characteristics);
- disparate impact (facially neutral practices with disproportionate effect);
- disparate treatment (different treatment based on protected status);
- the effect of feature selection on protected groups;
- the effect of model architecture on subgroup performance;
- feedback loops that amplify bias over time.

Direct discrimination is often illegal per se. Indirect discrimination through proxies is harder to detect but equally harmful. Disparate impact—the theory that facially neutral practices with disproportionate effects on protected groups may be unlawful—is the primary legal framework for algorithmic discrimination in the US (under Title VII, ECOA, Fair Housing Act). Feature selection, model architecture, and feedback loops can all introduce or amplify bias.

### Conduct Intersectional Analysis

Single-axis analysis misses compound discrimination.

Analyze:

- outcomes at the intersection of multiple protected characteristics;
- race and sex, disability and age, national origin and religion;
- whether subgroups that appear protected on single-axis analysis are disadvantaged at intersections;
- the sample size challenges of intersectional analysis;
- the need for targeted data collection where intersectional samples are small;
- the ethical and legal significance of intersectional harm.

A model may show no bias by race and no bias by sex but significant bias against women of a particular race. Intersectional analysis is essential for detecting compound discrimination. Sample sizes at intersections are often small, requiring careful statistical methods or targeted data collection. Intersectional harm has both ethical and legal significance, as discrimination law increasingly recognizes compound claims.

### Implement Bias Mitigation Strategies

Mitigation occurs at three stages.

Apply:

- pre-processing (transforming training data to reduce bias: reweighing, resampling, disparate impact remover);
- in-processing (modifying the learning algorithm: adversarial debiasing, fairness constraints, regularization);
- post-processing (adjusting predictions: threshold optimization, calibration, rejection option classification);
- the tradeoffs between accuracy and fairness;
- the risk of reverse-engineering mitigation to undermine it;
- documentation of the mitigation approach and its effects;
- the need to re-test after mitigation.

Bias mitigation has three stages, each with different techniques. Pre-processing addresses data bias. In-processing constrains the model. Post-processing adjusts outputs. Each approach has tradeoffs with accuracy and may introduce new biases. Mitigation must be documented and re-tested. Over-aggressive mitigation can reduce model utility or create new fairness violations.

### Establish Ongoing Monitoring And Auditing

Bias is not static; models drift and populations change.

Implement:

- post-deployment monitoring of subgroup performance;
- drift detection for input distributions and outcome distributions;
- periodic re-auditing on updated data;
- the effect of model retraining on fairness;
- feedback loop monitoring (does the model's output affect future training data?);
- incident response for fairness violations;
- external audit or third-party validation for high-stakes systems;
- public transparency where appropriate.

Models drift as populations and behaviors change. Retraining can introduce new bias. Feedback loops—where model predictions influence outcomes that become future training data—can amplify bias. High-stakes systems should undergo external audit. Transparency about fairness testing methodology and results builds trust and supports legal defense.

### Address Legal And Regulatory Frameworks

Algorithmic discrimination intersects multiple legal regimes.

Comply with:

- employment discrimination law (Title VII, ADEA, ADA, state laws) for hiring and employment decisions;
- credit discrimination law (ECOA, Regulation B, Fair Housing Act) for credit and lending;
- insurance discrimination law (state insurance anti-discrimination laws, unfair claims practices);
- the EU AI Act high-risk requirements for employment, credit, and insurance AI;
- state and local AI anti-discrimination laws (New York City Local Law 144 for automated employment decision tools);
- sector-specific guidance (EEOC guidance on algorithmic hiring, CFPB on adverse credit models);
- the evolving standards for AI discrimination enforcement.

Multiple legal regimes apply depending on the decision context. Employment decisions trigger Title VII and EEOC scrutiny. Credit decisions trigger ECOA and CFPB enforcement. The EU AI Act imposes specific requirements on high-risk employment, credit, and insurance AI. New York City Local Law 144 requires bias audits of automated employment decision tools. Enforcement is increasing across all domains.

## Common Traps

### Overall Accuracy Masks Subgroup Disparities

A model with 90% overall accuracy may have 70% accuracy for a protected subgroup.

### Single Fairness Metric Selected For Convenience

Choosing the metric that shows the model is fair, rather than the metric that fits the context.

### No Intersectional Analysis

Single-axis analysis misses compound discrimination against intersectional subgroups.

### Testing Only Before Deployment

Post-deployment drift and feedback loops can introduce bias that pre-deployment testing missed.

### Proxy Variables Not Identified

Excluding protected characteristics while retaining proxies (zip code, name) does not prevent bias.

### Mitigation Not Re-Tested

Applying mitigation without re-testing can introduce new fairness violations.

### Sample Size Too Small For Meaningful Subgroup Analysis

Small subgroup samples produce unreliable fairness metrics and can hide or exaggerate disparities.

## Self-Check

- Are protected groups and decision context identified with relevant characteristics, proxy variables, decision context, outcome definition, stakes, historical discrimination, and affected population?
- Are appropriate fairness definitions and metrics selected from demographic parity, equal opportunity, equalized odds, predictive parity, calibration, and individual fairness, with justification for the choice and awareness of impossibility theorems?
- Is the testing methodology rigorous with representative held-out test sets, sufficient subgroup sample sizes, intersectional analysis, train/validation/test separation, pre- and post-deployment testing, statistical significance, confidence intervals, small sample handling, and documented methodology?
- Is testing conducted for both direct and indirect discrimination including disparate impact, disparate treatment, feature selection effects, architecture effects, and feedback loops?
- Is intersectional analysis conducted at the intersection of multiple protected characteristics with sample size management and compound discrimination detection?
- Are bias mitigation strategies implemented at pre-processing, in-processing, and post-processing stages with accuracy-fairness tradeoffs, reverse-engineering risk, documentation, and re-testing?
- Is ongoing monitoring and auditing established for subgroup performance, drift detection, periodic re-auditing, retraining effects, feedback loops, incident response, external audit, and transparency?
- Are legal and regulatory frameworks addressed including employment (Title VII, EEOC), credit (ECOA, CFPB), insurance, EU AI Act, state/local AI laws (NYC Local Law 144), and sector-specific guidance?
- Are fairness audit results documented and available for regulatory inspection?
- Is the choice of fairness metric justified by the legal and ethical context, not by which metric shows the best result?