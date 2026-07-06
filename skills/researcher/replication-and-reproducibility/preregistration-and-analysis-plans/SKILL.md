---
name: preregistration_and_analysis_plans.md
description: Use when the agent is writing a preregistration, locking an analysis plan before data collection, deciding which analyses are confirmatory versus exploratory, managing deviations from a registered plan, or defending results against concerns about hidden multiplicity.
---

# Preregistration And Analysis Plans

Preregistration is the practice of committing to design and analysis decisions before seeing the results. It exists because researchers, acting in good faith, can otherwise shape analysis choices around the data they observe, producing results that do not hold up. A study can be honestly conducted and still mislead if the analytic flexibility was exercised after the results were known. The judgment problem is deciding what to lock down, when, at what level of detail, and how to handle the inevitable deviations honestly.

Use this skill when writing a preregistration or registered report, when separating confirmatory from exploratory analyses, when deciding whether a change counts as a deviation, and when reporting results relative to a prior plan. The goal is to keep the agent from treating preregistration as a formality, from writing plans too vague to constrain anything, and from silently abandoning the plan when results disappoint.

This is a high-stakes domain because unmanaged analytic flexibility is a root cause of false findings that waste resources and mislead fields. When the right registration platform or the binding force of a plan is unclear, the agent should consult a methodologist or the relevant registry rather than self-certifying compliance.

## Core Rules

### Preregister Before Observing The Outcome Data

The binding force of preregistration comes from timing. A plan written after the analyst has seen the results cannot constrain the very flexibility it claims to constrain.

Lock down decisions before:

- observing outcome data for the confirmatory analysis;
- running pilot analyses on the main study data;
- learning which conditions or subgroups look favorable;
- exploring which covariates change the result.

For secondary data analysis, preregister before inspecting the variables relevant to the hypothesis, even if the dataset is already public. Knowing the answer shapes every later decision, including decisions that feel unrelated.

### Specify The Confirmatory Analysis In Enough Detail To Be Binding

A vague plan constrains nothing. "We will run a regression" leaves dozens of choices open after the fact.

Specify:

- the exact hypothesis and its primary outcome;
- the primary predictor and comparison;
- the statistical model, including family and link;
- covariates and why each is included;
- how variables are constructed, transformed, or recoded;
- exclusion rules and how missing data are handled;
- the inference criterion and how multiplicity is addressed;
- the planned sample size or stopping rule;
- the direction of expected effects where relevant.

Each unspecified choice is a degree of freedom that can be exercised later to reach a desired result. The plan should leave as few as feasible.

### Separate Confirmatory And Exploratory Analyses Explicitly

Preregistration does not forbid exploration; it requires honesty about what was planned versus what was discovered in the data.

Distinguish:

- confirmatory analyses, locked in advance and reported as tests of the hypothesis;
- exploratory analyses, conducted after seeing data and reported as hypothesis-generating;
- the inference each can support.

Confirmatory results can support claims about the hypothesis. Exploratory results can suggest patterns for future study but should not be presented as confirmation. Conflating the two is the core error preregistration is meant to prevent.

### Choose The Right Platform And Level Of Commitment

Different platforms serve different purposes and carry different force.

Consider:

- a general registry such as the Open Science Framework for time-stamped plans;
- field-specific registries such as clinical trial registries or economics registries;
- registered reports, where the plan is peer-reviewed and accepted before data collection;
- institutional or funder-mandated registries;
- whether the registry is public, embargoed, or private, and when it becomes visible.

Match the platform to the field's norms and the strength of commitment needed. A private, never-visible plan provides little external accountability.

### Manage Deviations Honestly And Transparently

Real studies deviate from plans. Equipment fails, assumptions break, and unanticipated problems arise. The integrity problem is not deviation itself but hiding it.

When deviating:

- document what changed and why;
- distinguish deviations forced by circumstances from those chosen for convenience;
- report both the planned analysis and the deviated analysis when feasible;
- state whether the deviation was decided before or after seeing relevant results;
- explain the inferential consequences.

A study that reports a clean match to a plan that was actually abandoned is making a false claim about its own evidential strength.

### Predefine How Multiplicity Will Be Handled

Multiple comparisons, outcomes, subgroups, and covariate sets inflate the chance of false positives. Preregistration is the moment to decide how to address this.

Specify:

- which test is primary and which are secondary;
- whether and how corrections for multiple comparisons will be applied;
- which subgroup analyses are planned and which are exploratory;
- how optional stopping or multiple outcome measures are handled;
- the role of effect sizes and confidence intervals alongside significance tests.

Deciding the multiplicity correction after seeing which results survive is itself a form of researcher flexibility.

### Coordinate Preregistration With Ethics, Data, And Reporting

Preregistration interacts with ethics approval, data sharing, and reporting standards. Treating them as separate produces contradictions.

Align:

- the registered plan with the approved ethics protocol;
- the registered sample size with the power analysis and feasibility;
- the registered analysis with the reporting standard the journal requires;
- the registered data sharing plan with consent and privacy constraints;
- the registered timeline with recruitment and analysis reality.

A registered plan that contradicts the ethics protocol or the consent language creates problems that surface late and are hard to fix.

## Common Traps

### Preregistering After Seeing The Data

A plan written after results are known provides no constraint. It is documentation of choices already made, not commitment.

### Writing A Plan Too Vague To Bind Anything

Plans that omit model specifications, exclusion rules, or multiplicity handling leave the analyst free to choose the favorable option later while claiming preregistration.

### Silently Abandoning The Plan For A Better Result

Switching to an unregistered analysis because it reaches significance, without reporting the planned analysis, defeats the purpose and misrepresents the evidence.

### Treating Exploratory Findings As Confirmatory

Discovering a striking subgroup effect after looking at the data and presenting it as a planned test inflates the apparent strength of evidence.

### Forgetting That Secondary Data Still Needs Timing Discipline

Public datasets are not exempt. Knowing the answer before planning the analysis reintroduces all the flexibility preregistration is meant to remove.

### Ignoring Multiplicity Until Results Are In

Choosing whether to correct for multiple comparisons based on which option yields significance is a researcher degree of freedom dressed up as rigor.

### Treating Preregistration As A Box To Check

A registration filed for compliance, with no intention to follow or report it, provides accountability in appearance only and can mislead readers who trust it.

## Self-Check

- Is the analysis plan locked down before observing outcome data for the confirmatory analysis?
- Does the plan specify model, variables, exclusions, missing data handling, multiplicity, and inference criteria in enough detail to be binding?
- Are confirmatory and exploratory analyses explicitly separated, with the inferential role of each stated?
- Is the registration platform matched to the field and the level of commitment needed, with visibility appropriate to the study?
- Are deviations from the plan documented with reasons, timing relative to results, and both planned and deviated analyses reported?
- Is multiplicity handling predefined rather than chosen after seeing which results survive?
- Is the preregistration coordinated with ethics approval, consent, data sharing, and reporting standards?
- For uncertain questions about binding force, platform choice, or deviation reporting, has a methodologist or the relevant registry been consulted rather than self-certifying compliance?
