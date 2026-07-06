---
name: survey_sampling_and_nonresponse.md
description: Use when the agent is designing a survey sampling plan, choosing a sampling frame, computing response rates, diagnosing nonresponse bias, applying weights, or deciding whether survey findings can be generalized given coverage and nonresponse.
---

# Survey Sampling And Nonresponse

A survey's value depends on whether its respondents represent the target population, and representation is decided by sampling and response, not by sample size alone. A survey of ten thousand volunteers can be more biased than a probability sample of one thousand. Agents often focus on the number of responses and ignore the two processes that determine bias: who had a chance to be selected (coverage), and who among the selected actually responded (nonresponse). When either process is non-random with respect to the survey variables, the estimates are biased, and weights can reduce but not eliminate the problem. The discipline is to design sampling for representation, to maximize response, and to diagnose and disclose nonresponse honestly.

Use this skill when planning survey sampling, computing response rates, diagnosing nonresponse, or applying weights. The goal is to prevent the agent from treating a large convenience sample as representative and from hiding nonresponse bias behind a big N.

## Core Rules

### Define The Target Population And Build A Sound Sampling Frame

Generalization requires a clear target population and a frame that covers it. Gaps between frame and population produce coverage bias.

Define and build by:

- specifying the target population precisely;
- obtaining or constructing a frame that covers the population;
- identifying and quantifying frame undercoverage;
- noting groups excluded by the frame (e.g., no internet, no phone, institutionalized).

Estimates from a frame that misses part of the population generalize only to the frame, not the population.

### Use Probability Sampling Where Representation Matters

Probability sampling, where each unit has a known nonzero chance of selection, is the foundation for statistical generalization. Non-probability samples require strong assumptions.

Use probability sampling by:

- simple, stratified, cluster, or multistage designs as appropriate;
- documenting selection probabilities for weighting;
- stratifying to improve precision on key subgroups;
- accounting for clustering in design and analysis.

If non-probability sampling is used, its limitations and the assumptions required for inference must be explicit.

### Compute And Report Response Rates Rigorously

The response rate is a primary indicator of potential nonresponse bias. It must be computed by a recognized standard, not invented.

Report by:

- a standard definition (e.g., AAPOR) with the disposition of every case;
- response rates overall and by key subgroup;
- cooperation and refusal rates where relevant;
- the gap between contacted, eligible, and responding samples.

A vague "we got 400 responses" without a denominator and eligibility dispositions reveals nothing about bias.

### Diagnose Nonresponse Bias, Not Just Rate

A low response rate is a warning, not a verdict; bias depends on whether nonrespondents differ from respondents on the survey variables. A high rate with differential nonresponse can still be biased.

Diagnose by:

- comparing respondents to known population benchmarks;
- analyzing early versus late respondents as a proxy;
- following up with nonrespondents on key variables;
- examining nonresponse patterns across subgroups.

Nonresponse bias cannot be assumed away by a high rate or by weighting alone.

### Apply Weights With Justification And Limits

Weights adjust for unequal selection probabilities and known differences between sample and population. They reduce but do not eliminate bias from unmeasured differences.

Apply by:

- design weights for unequal selection probabilities;
- post-stratification or calibration to population totals;
- documenting the weighting variables and their source;
- acknowledging that weights correct only for variables they include.

Weighting to demographics does not correct for attitudinal or behavioral differences that drive nonresponse.

### Maximize Response Through Proven Methods

Response rates are influenced by design choices made before fielding. Proven methods reduce nonresponse at the source.

Use:

- advance contact and notifications;
- incentives calibrated to burden and population;
- multiple contact attempts and modes;
- a respondent-friendly questionnaire;
- tailored protocols for hard-to-reach groups.

Throwing a survey online and hoping for responses is a design for bias.

### Account For Coverage Error In Online And Access Panels

Online and panel surveys often miss populations without internet access or panel membership. Coverage error can exceed nonresponse error.

Account by:

- assessing the gap between the panel and the target population;
- using probability-based online panels where possible;
- weighting to adjust for known coverage gaps;
- acknowledging residual coverage bias.

A large online opt-in sample covers the online, panel-willing population, not necessarily the target population.

### Handle Mode And Mixed-Mode Effects

Different modes recruit and elicit different respondents and answers. Mixed-mode designs balance coverage against mode effects.

Handle by:

- choosing modes to maximize coverage of the target population;
- measuring or assuming mode effects on responses;
- designing questions to be mode-equivalent where possible;
- reporting the mode composition of the sample.

Mixing modes without considering their differential effects complicates interpretation.

### Report Sampling Limitations Transparently and distinguish Descriptive From Analytic Inference

Every survey has limitations. Naming them lets the reader calibrate the conclusions.

Report:

- the sampling design and frame coverage;
- response rates and nonresponse analysis;
- weighting methods and their limits;
- coverage gaps and mode effects;
- the scope of generalization the survey supports.

Descriptive population estimates require probability sampling and good response; analytic inference about relationships may be more robust to some nonresponse but is not immune.

Clarify:

- descriptive claims (prevalence, means) are highly sensitive to representation;
- relational claims (associations, differences) may be less sensitive but assume nonresponse is ignorable for the relationship;
- both require honest treatment of sampling and nonresponse.

## Common Traps

### Big N Mistaken For Representativeness

A large convenience sample is still biased; size does not fix coverage or nonresponse error.

### Unreported Or Inflated Response Rates

Vague or non-standard response reporting hides the real risk of bias.

### Nonresponse Bias Assumed Away By Weighting

Weights correct only for measured variables; unmeasured differences remain.

### Ignored Coverage Error

Online or panel samples miss populations, and coverage bias can dominate.

### No Nonresponse Diagnosis

A rate without analysis of who responded versus who did not reveals nothing about bias.

### Mode Effects Unaddressed

Mixed modes without considering differential recruitment and response complicate comparison.

### Descriptive Claims From Non-Probability Samples

Population estimates from opt-in samples require strong, often untestable assumptions.

## Self-Check

- Is the target population defined and the sampling frame assessed for coverage gaps?
- Is probability sampling used where representation matters, with non-probability limitations explicit?
- Is the response rate computed by a recognized standard and reported by subgroup?
- Is nonresponse bias diagnosed via benchmarks, early-late comparison, or follow-up, not assumed away?
- Are weights justified, documented, and acknowledged to correct only for included variables?
- Are proven response-maximization methods used rather than passive fielding?
- Is coverage error in online or panel samples assessed and addressed?
- Are mode and mixed-mode effects on recruitment and responses considered?
- Are sampling limitations, response rates, and weighting reported transparently?
- Are descriptive and analytic inference distinguished in their sensitivity to representation?
