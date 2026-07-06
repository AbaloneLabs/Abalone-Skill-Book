---
name: design_fit_and_claim_matching.md
description: Use when the agent is checking whether a chosen study design can actually answer the research question, matching design to the intended claim type, assessing whether the design supports descriptive causal or exploratory conclusions, defining primary outcomes and estimands, or auditing a protocol for inferential gaps before data collection.
---

# Design Fit And Claim Matching

A study can be executed flawlessly and still answer the wrong question. Design fit is the discipline of verifying that the structure of the study, its sampling, its comparison conditions, its outcomes, and its analysis actually support the specific claim the researcher intends to make. Agents often treat a design as adequate because it is familiar or because it produced a significant result, when in fact it cannot bear the weight of the intended inference. A randomized trial of the wrong population, for the wrong duration, with a surrogate outcome, may be rigorous yet unable to support the clinical or policy claim attached to it.

The harm this skill prevents is the silent gap between method and claim, where a study is reported as answering a question it was never built to answer. This produces misleading conclusions, misdirected policy, wasted resources, and in clinical or safety contexts, harm to people who act on overstated results. The agent has freedom in how to construct a study, but must verify the chain from question to estimand to design to claim, and must refuse to let a strong result paper over a structural mismatch.

## Core Rules

### Start From The Research Question And Work Downward

Design fit is judged relative to a precise question, not relative to available data or a favored method. Before evaluating any design, state the question in concrete terms, including the population, the exposure or intervention, the comparator, the outcome, and the time frame. Only then ask whether the design can answer that specific question.

Articulate:

- the population to which the claim is meant to apply;
- the intervention or exposure and its meaningful contrast;
- the comparator, whether no intervention, standard care, or an alternative;
- the outcome that matters for the claim, not merely the outcome that is easy to measure;
- the timeframe over which the effect must hold.

A design that drifts from any of these has a fit problem, even if it is otherwise well executed.

### Classify The Claim Type And Match The Design To It

Different claim types require different inferential scaffolding. Mismatching claim to design is the core failure this skill targets.

Match claim to design:

- descriptive claims require representative sampling and valid measurement of the target population;
- associational claims require adequate measurement of both variables and sufficient spread;
- causal claims require either randomization or a defensible strategy to address confounding and temporality;
- mechanistic claims require evidence of the pathway, not just an input-output difference;
- predictive claims require validation on held-out or external data, not in-sample fit.

The strongest word in the intended conclusion dictates the minimum design requirement.

### Define The Estimand Before Choosing The Analysis

The estimand is the precise quantity the study targets, such as an average treatment effect, a treatment effect on the treated, or a risk difference at a specific time. Many fit failures arise because the analysis estimates a different quantity than the claim asserts. Define the estimand first, then ensure the design and analysis can estimate it, handling intercurrent events such as noncompliance, dropout, or competing risks.

Specify:

- the target estimand in plain and statistical terms;
- how intercurrent events are handled, via treatment-policy, while-on-treatment, composite, or principal-stratum strategies;
- whether the estimand is causal or associational;
- the population to which the estimand generalizes.

An analysis that estimates a convenient parameter cannot support a claim about a different parameter.

### Predefine The Primary Outcome And Resist Outcome Drift

The primary outcome is the one the study is powered to detect and the one the main claim rests on. Adding outcomes, switching outcomes, or recategorizing outcomes after seeing data inflates false positives and undermines the claim. Fit includes whether the primary outcome was chosen and locked before analysis.

Ensure:

- the primary outcome is clinically or substantively meaningful, not merely statistically convenient;
- the primary outcome is prespecified and registered;
- secondary and exploratory outcomes are clearly labeled as such;
- multiplicity from many outcomes is addressed in the analysis plan;
- surrogate outcomes are justified by validated links to outcomes that matter.

A significant result on a non-primary, post-hoc outcome cannot carry a confirmatory claim.

### Check That The Population And Setting Match The Claim's Scope

A design may answer the question for the sample studied but not for the population named in the claim. Generalization is a fit question, not an afterthought.

Verify:

- whether the inclusion and exclusion criteria define the population the claim targets;
- whether the setting resembles the contexts where the claim will be applied;
- whether the sample is representative or a convenience subset;
- whether subgroup differences threaten the aggregate claim;
- whether transportability assumptions are stated and defensible.

A claim about broad practice cannot rest on a narrow, unrepresentative sample without explicit justification.

### Verify The Design Can Detect The Effect That Matters

A design that is underpowered cannot distinguish a real null from a missed effect, and a design that is overpowered may flag trivial differences as significant. Fit includes whether the study can detect an effect of meaningful magnitude.

Confirm:

- the minimally important effect size is defined and justified, not borrowed from convention;
- the sample size targets that effect with adequate power;
- the outcome measure has the sensitivity to detect that effect;
- the duration and follow-up are long enough for the effect to emerge;
- the analysis plan preserves, rather than dissipates, power.

A non-significant result in an underpowered study is not evidence of no effect.

### Map Each Threat To Validity To A Design Feature

For every plausible rival explanation, identify the specific design or analysis feature that rules it out. If a threat has no corresponding countermeasure, the design cannot rule it out, and the claim must be narrowed.

Map threats such as:

- selection, addressed by randomization or matching;
- maturation and history, addressed by a control group;
- regression to the mean, addressed by a comparison group and baseline measurement;
- attrition, addressed by retention strategies and appropriate missing-data methods;
- measurement reactivity, addressed by blinding and objective measures;
- confounding, addressed by design or adjustment with stated limits.

A threat with no countermeasure is a limit on the claim, not a detail to omit.

### Separate Confirmatory From Exploratory Inference

A design built for hypothesis generation cannot confirm a hypothesis, and treating exploratory findings as confirmatory inflates error and overstates evidence. Fit requires honesty about which mode the design is operating in.

Distinguish:

- confirmatory analyses tied to prespecified hypotheses and primary outcomes;
- exploratory analyses meant to generate future hypotheses;
- the inferential language appropriate to each;
- the need for replication before an exploratory finding is treated as established.

## Common Traps

### Treating A Significant Result As Proof Of Fit

A low p-value shows an association was detected, not that the design answers the intended question. Significance cannot repair a structural mismatch between design and claim.

### Letting Available Data Dictate The Question

Starting from a dataset and reverse-engineering a question produces a claim the data can answer, which is rarely the claim that matters. Fit is judged question-first.

### Mismatching Estimand And Claim

Estimating a treatment effect in compliers while claiming an effect for everyone, or estimating an associational parameter while claiming causation, is a fit failure invisible in the results table.

### Switching The Primary Outcome Post-Hoc

Changing the primary outcome after peeking at data turns a confirmatory study into an exploratory one while keeping confirmatory language. This manufactures false confidence.

### Generalizing Beyond The Sample Without Justification

A claim about a broad population resting on a narrow sample overstates scope. Transportability must be argued, not assumed.

### Underpowering And Calling It A Null Result

An underpowered study that finds no significant difference has not shown there is no effect. The design could not detect the effect that matters.

### Ignoring Surrogate Outcome Validity

Using a biomarker or proxy as the primary outcome without a validated link to the outcome that matters risks proving the wrong thing. A significant change in a surrogate is not a significant benefit.

### Omitting The Threat With No Countermeasure

When a rival explanation has no design feature against it, quietly moving on leaves the claim exposed. Each unaddressed threat must narrow the stated conclusion.

## Self-Check

- [ ] Is the research question stated concretely with population, intervention, comparator, outcome, and timeframe before the design is evaluated?
- [ ] Is the claim type (descriptive, associational, causal, mechanistic, predictive) explicitly matched to a design that can support it?
- [ ] Is the target estimand defined before the analysis, with intercurrent events handled by a stated strategy?
- [ ] Is the primary outcome prespecified, meaningful, and distinguished from secondary and exploratory outcomes?
- [ ] Do the sample, setting, and inclusion criteria match the scope of the population the claim is about?
- [ ] Is the study powered to detect a minimally important effect, and is a non-significant result interpreted against that power?
- [ ] Is each major threat to validity mapped to a specific design or analysis countermeasure, with unaddressed threats reflected in a narrowed claim?
- [ ] Are confirmatory and exploratory analyses clearly separated, with exploratory findings not reported as confirmed?
- [ ] Does the stated conclusion use only the inferential language the design and estimand can support, with no upgrade from association to causation or from sample to broad population?
- [ ] For high-stakes, contested, or uncertain cases, is expert methodological or biostatistical consultation sought to confirm design fit before the claim is finalized?
