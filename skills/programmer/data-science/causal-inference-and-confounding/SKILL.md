---
name: causal_inference_and_confounding.md
description: Use when the agent is inferring causation from observational data (rather than a randomized experiment), deciding whether a relationship is causal or merely correlated, identifying and adjusting for confounders, choosing between methods like stratification, regression adjustment, propensity scores, difference-in-differences, instrumental variables, or regression discontinuity, or interpreting a causal claim's validity and threats. Also covers the failure modes of confounding (a third variable driving both cause and effect), selection bias, reverse causation, omitted variable bias, over-trusting adjusted models that still miss unmeasured confounders, and the recurring mistake of reading an observational correlation as a causal effect without examining what could explain it away.
---

# Causal Inference And Confounding

"Correlation is not causation" is recited so often it has become a throwaway line, and yet the most expensive analytical mistakes come from violating it. A marketing team reads "users who got the email converted more" as "the email caused the conversions" and doubles spend — but the email was sent to the most engaged users, who would have converted anyway. A policy reads "regions with more police have more crime" as "police cause crime." A product reads "users who use feature X retain better" as "feature X drives retention" and forces the feature on everyone — but engaged users self-select into the feature. The judgment problem is that observational data (data not produced by a randomized experiment) is riddled with confounding: variables that influence both the supposed cause and the supposed effect, creating a correlation that looks causal but is not. Inferring causation from such data requires explicitly identifying the confounders, choosing a method that can adjust for them, and being honest about the confounders that remain unmeasured and the residual doubt they create.

Agents tend to under-invest here because the tools produce a clean number — a regression coefficient, a lift — that looks like a causal effect, and because running a randomized experiment (the gold standard, covered in the ab-test skill) is often impractical, slow, or impossible (you cannot randomize smoking, or a past policy). The harm is confident causal claims that drive decisions and are wrong. A confounder that was not adjusted for inflates or reverses the estimated effect. A selection effect (the users who adopted the feature are systematically different) is mistaken for a treatment effect. Reverse causation (does stress cause insomnia, or insomnia cause stress?) is assumed in one direction. The judgment problem is to recognize when a question is causal, to refuse to answer it with a naive correlation, to identify and adjust for confounders with an appropriate method, and to communicate the residual uncertainty honestly rather than presenting an adjusted estimate as proven fact.

This skill covers recognizing causal questions, identifying confounders, choosing adjustment methods, and the specific designs (difference-in-differences, instrumental variables, regression discontinuity) that recover causal effects from observational data. It complements the ab-test skill (randomized experiments, the cleanest causal evidence) and the statistical-pitfalls skill (general interpretation errors). Here the focus is causation from non-experimental data and the confounding that makes it hard.

## Core Rules

### Recognize When The Question Is Causal, And Refuse To Answer It With A Correlation

The first step is recognizing that a question is causal, because the analysis differs entirely. "Do users who use feature X retain better?" is a descriptive (associational) question answerable by a correlation. "Does using feature X cause better retention?" is a causal question that the correlation does not answer. Conflating them is the root error:

- **A causal question asks what would happen if you intervened.** "If we gave the treatment to a different set of units, what would their outcome be?" This counterfactual — the outcome under a different treatment — is what causation means, and it cannot be read directly from observational data because you only observe each unit under one treatment.
- **The observed difference mixes the treatment effect with selection.** The users who adopted feature X differ from those who did not (they are more engaged, more technical, etc.). The observed retention gap is the treatment effect plus the pre-existing difference; you cannot separate them by looking at the gap alone.
- **State the causal question explicitly and the assumptions it requires.** "We want the causal effect of the email on conversion, assuming we can adjust for all confounders (engagement, recency, segment)." Naming the assumption surfaces its fragility.

### Identify Confounders And Distinguish Them From Mediators And Colliders

Adjusting for the wrong variables is as harmful as not adjusting at all. The variables around a cause and effect fall into roles, and only confounders should be adjusted for:

- **A confounder causes both the treatment and the outcome.** Engagement causes both whether a user adopts a feature (engaged users adopt more) and retention (engaged users retain better). It must be adjusted for, or the feature's effect is confounded. Confounders are on the "back-door path" between cause and effect.
- **A mediator is on the causal path — do not adjust for it.** If the feature causes retention *through* increased session frequency, session frequency is a mediator, not a confounder. Adjusting for a mediator blocks part of the real effect and underestimates it. The distinction (confounder vs mediator) depends on causal direction, which requires domain knowledge, not just the data.
- **A collider is caused by both treatment and outcome — adjusting for it introduces bias.** Conditioning on a variable affected by both the cause and the outcome opens a spurious path and can create or reverse an association. Adjusting for everything "just in case" is wrong; some variables are colliders whose adjustment biases the estimate.
- **Draw the causal structure (a DAG) before adjusting.** A directed acyclic graph of the assumed causal relationships makes confounders, mediators, and colliders explicit and determines what to adjust for. Adjusting without a causal model is guesswork.

### Choose An Adjustment Method That Fits The Data And The Confounders

Once confounders are identified, choose a method to adjust for them. The methods have different assumptions, data requirements, and robustness:

- **Stratification / regression adjustment.** Adjust for confounders by stratifying on them or including them in a regression. Works when confounders are observed, few, and the relationship is correctly modeled. Assumes no unmeasured confounders (strong) and correct functional form.
- **Propensity score methods.** Estimate the probability of receiving treatment given confounders (the propensity score), then match, weight, or stratify units on it to create comparable groups. Useful with many confounders, but still assumes no unmeasured confounders and requires good overlap (both treated and control units across the propensity range).
- **Check overlap (common support).** Adjustment only works where treated and control units are comparable on confounders. If all treated units are high-engagement and all controls are low-engagement, there is no comparable control group and no method can recover the effect honestly — the estimate is an extrapolation.
- **Be honest about unmeasured confounding.** All adjustment methods above assume all confounders are measured ("no unmeasured confounding" / ignorability). This is rarely fully true. A confounder you did not measure (user motivation, underlying health, competitor actions) remains in the estimate. State this assumption and its risk.

### Use Quasi-Experimental Designs When Unmeasured Confounding Is Likely

When unmeasured confounding is a serious threat and a randomized experiment is impossible, quasi-experimental designs can recover causal effects under different, often more credible assumptions:

- **Difference-in-differences (DiD).** Compare the change in outcome before and after a treatment for a treated group versus a control group, relying on the "parallel trends" assumption (absent treatment, both groups' outcomes would have trended in parallel). Useful for policy or rollout events; invalid if trends were already diverging.
- **Instrumental variables (IV).** Use a variable that affects treatment but not the outcome except through treatment (an instrument), to isolate the variation in treatment that is free of confounding. Requires a valid instrument (affecting outcome only through treatment, no confounding itself), which is hard to find and justify; a weak or invalid instrument produces misleading estimates.
- **Regression discontinuity (RDD).** When treatment is assigned by a threshold (a score above X gets the program), compare units just above and just below the threshold, who are nearly identical except for treatment. Clean local causal evidence at the threshold, but only estimates the effect at the cutoff, not globally.
- **Match the design to the source of variation.** Each design exploits a specific source of plausibly exogenous variation (a before/after event, an instrument, a threshold). Choose the design whose assumptions are credible for your setting, and verify those assumptions (e.g., parallel trends for DiD) rather than assuming them.

### Beware Selection Bias, Reverse Causation, And Survivors

Beyond measured confounders, three structural threats routinely produce spurious causal conclusions:

- **Selection bias.** The sample itself is conditioned on an outcome-adjacent variable. Studying "what predicts success among users who reached step 5" conditions on reaching step 5; the predictors look different than in the full population. Survivorship bias (analyzing only the units that survived) is a form of this.
- **Reverse causation.** The outcome may cause the treatment, not vice versa. "Customers who use support chat churn more" — do they churn because of chat, or do they use chat because they are already unhappy and about to churn? Cross-sectional data cannot establish direction; temporal ordering or an instrument can.
- **Conditioning on a post-treatment variable.** Adjusting for or filtering on a variable that is itself an effect of the treatment introduces bias (it is a mediator or collider). Define the analysis before looking at post-treatment variables.

### Communicate Causal Claims With Their Assumptions And Residual Doubt

A causal estimate from observational data is never as certain as a well-run experiment. The communication must reflect that:

- **State the identifying assumptions.** "This estimate assumes no unmeasured confounders and parallel trends." The reader must know what the estimate rests on to judge it.
- **Present sensitivity analysis.** How strong would an unmeasured confounder have to be to explain away the result? If a modest confounder could nullify the effect, the claim is fragile; if it would take an implausibly strong confounder, the claim is more robust.
- **Do not present an observational estimate as proven.** Distinguish "associated with" from "causes." Overstating causal certainty from observational data is a common and damaging analytical failure.
- **Recommend an experiment where feasible.** For decisions that matter and are testable, the observational analysis can identify a candidate effect and an experiment can confirm it. Do not let the observational estimate substitute for an experiment when one is possible.

## Common Traps

### Reading An Observational Correlation As Causation

Seeing that treated and untreated groups differ and concluding the treatment caused the difference, when a confounder (engagement, health, selection) explains it. Recognize causal questions and refuse to answer them with a correlation alone.

### Adjusting For Everything "Just In Case"

Including every available variable in a regression, which adjusts for confounders but also for mediators (blocking real effects) and colliders (introducing bias). Draw the causal structure and adjust only for confounders.

### Assuming No Unmeasured Confounders Without Justification

Using regression or propensity adjustment and assuming all confounders are measured, when an unmeasured variable (motivation, underlying condition, competitor actions) likely remains. State the assumption and run sensitivity analysis.

### Treating A Mediator As A Confounder

Adjusting for a variable on the causal path (e.g., session frequency through which a feature affects retention), blocking part of the real effect and underestimating it. The confounder-vs-mediator distinction requires causal direction, from domain knowledge.

### No Overlap Between Treated And Control

Running propensity or regression adjustment when treated and control units do not overlap on confounders (all treated are high-engagement, all controls low), so the estimate is an extrapolation with no real comparison group.

### Reverse Causation Assumed In One Direction

Assuming the cause precedes the effect when the data cannot establish ordering, so an unhappy customer's support usage is read as chat causing churn. Use temporal ordering or an instrument to establish direction.

### Overstating Causal Certainty From Observational Data

Presenting an adjusted observational estimate as a proven causal effect, without stating assumptions or residual doubt. Communicate the identifying assumptions and run sensitivity analysis; recommend an experiment where feasible.

## Self-Check

- [ ] The causal question (the counterfactual: what would happen under intervention) was distinguished from the associational question, and the observed difference was recognized as mixing treatment effect with selection, not read as causation directly.
- [ ] Confounders were identified via a causal model (DAG), and distinguished from mediators (on the causal path, not adjusted) and colliders (conditioning on which introduces bias); adjustment targets only confounders.
- [ ] The adjustment method (stratification, regression, propensity) fits the data and confounders, overlap/common support was checked (treated and control comparable on confounders), and the no-unmeasured-confounding assumption was stated with its risk.
- [ ] Where unmeasured confounding is a serious threat, a quasi-experimental design (difference-in-differences, instrumental variables, regression discontinuity) was used, with its specific assumptions (parallel trends, valid instrument, threshold continuity) verified rather than assumed.
- [ ] Selection bias, reverse causation, and post-treatment conditioning were considered: the sample is not conditioned on an outcome-adjacent variable, causal direction was established (not assumed), and no post-treatment variable was adjusted for.
- [ ] The causal claim is communicated with its identifying assumptions, a sensitivity analysis (how strong an unmeasured confounder would need to be to nullify the result), and honest residual doubt — not presented as proven fact.
- [ ] Where the decision matters and is testable, an experiment is recommended to confirm the observational finding, rather than the observational estimate substituting for one.
- [ ] The highest-risk cases were verified — an unmeasured confounder, a mediator mistaken for a confounder, no overlap, reverse causation, and a fragile result overturned by a modest confounder — not only the clean adjusted estimate.
