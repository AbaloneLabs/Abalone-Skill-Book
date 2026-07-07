---
name: longitudinal_and_panel_design.md
description: Use when the agent is designing a longitudinal panel or repeated-measures study, deciding how many waves are needed, handling attrition across waves, accounting for autocorrelation, choosing between fixed and random effects for panel data, weighing a longitudinal design against cross-sectional snapshots, or deciding when measuring change is necessary versus when it is overkill.
---

# Longitudinal And Panel Design

Cross-sectional data can describe differences between groups, but they cannot separate change within persons from differences between persons, and that confusion is the root of most errors in longitudinal reasoning. A longitudinal design exists precisely to disentangle temporal dynamics from static composition, yet researchers often collect repeated measures and then analyze them as if each wave were independent, or fit a complex panel model to a question that needed only two time points. The judgment problem is to decide when longitudinal design is necessary, to design the time structure so it can answer the intended question, to manage the attrition that will erode the sample, and to choose an analytic approach whose assumptions match the structure of the data. Getting the design wrong cannot be repaired by a sophisticated model.

Use this skill when planning a longitudinal, panel, or repeated-measures study, when deciding on the number and spacing of waves, when managing attrition, or when choosing between fixed and random effects. The goal is to keep the agent from treating time as a minor feature of the design, from underestimating attrition, and from applying panel models whose assumptions the data violate. The agent has latitude in design and modeling, but the temporal structure, the change the study aims to capture, and the attrition plan must be reasoned through explicitly.

## Core Rules

### Decide Whether The Question Requires Longitudinal Data

Longitudinal designs are expensive and loss-prone, and they are not always necessary. The first decision is whether the question is about change, dynamics, or temporal ordering, all of which require repeated measures, or about group differences, which may not.

Ask whether the question requires:

- within-person change over time, which no cross-sectional design can capture;
- temporal ordering of cause and effect, which needs observations spanning the proposed sequence;
- trajectories, transitions, or lagged effects, which need multiple waves;
- separation of aging or period effects from cohort differences;
- or merely between-person differences, which a well-designed cross-sectional study may answer.

Using a longitudinal design for a question that only needs group differences wastes resources and exposes the study to attrition for no gain. Using a cross-sectional design for a question about change produces a structural answer the data cannot support.

### Design The Time Structure To Match The Change Being Studied

Time is a variable in a longitudinal design, and its structure, the number of waves, their spacing, and their span, determines what kinds of change can be detected. A design that samples time wrongly will miss the dynamics it set out to capture.

Specify deliberately:

- the number of waves, and whether two points can show change but not trajectory;
- the interval between waves, matched to the expected rate of the process, not to administrative convenience;
- the total span, long enough to capture the change of interest but short enough to limit attrition;
- whether measurement occasions are fixed or event-contingent;
- whether the design is balanced or will tolerate unequal spacing and missing waves.

A process that unfolds in weeks cannot be captured by annual waves; a process that unfolds over years cannot be captured by a two-week panel. The time structure must be derived from the phenomenon, not from the calendar.

### Plan For Attrition Before Collecting The First Wave

Attrition is the central threat to longitudinal inference, and it is rarely random. The people who drop out usually differ systematically from those who remain, and a shrinking, non-representative panel can turn a sound design into a biased one.

Plan for attrition by:

- estimating likely dropout per wave from comparable prior studies, not optimism;
- oversampling or enlarging the initial sample to preserve power after expected loss;
- designing retention efforts, incentives, and contact-tracking from the start;
- measuring variables that predict dropout, to enable later attrition analysis;
- pre-specifying how missing data will be handled, with methods that assume missing-at-random only where defensible;
- planning sensitivity analyses for missing-not-at-random scenarios.

Treating attrition as a nuisance to be cleaned up later, rather than a design feature to be managed, is how longitudinal studies lose their inferential force.

### Account For Autocorrelation And Non-Independence

Repeated observations on the same unit are not independent, and treating them as independent understates uncertainty and inflates significance. The analysis must respect the clustered, correlated structure of the data.

Address by:

- recognizing that repeated measures within a person are correlated over time;
- choosing an analytic approach that models this correlation, such as mixed models or generalized estimating equations;
- specifying the correlation structure, such as autoregressive or unstructured, based on the expected decay of dependence;
- accounting for additional clustering, such as persons within households or schools;
- reporting the assumed structure and checking its fit.

Ignoring autocorrelation does not make it go away; it makes the standard errors wrong and the p-values misleading.

### Choose Between Fixed And Random Effects Based On The Inferential Goal

Fixed-effects and random-effects panel models answer different questions and rest on different assumptions. The choice is not a matter of software default but of what the study wants to estimate and what it can assume.

Distinguish:

- fixed-effects models control for all time-invariant confounders, measured and unmeasured, by using only within-unit variation; they cannot estimate effects of time-invariant variables;
- random-effects models use both within- and between-unit variation and can estimate time-invariant effects, but they assume the unit effects are uncorrelated with the predictors, an assumption often violated;
- the Hausman test and substantive reasoning should guide the choice, not convenience;
- between-effects and hybrid models are available when the research question concerns stable differences or decomposes within and between effects.

A random-effects model applied where unit effects are correlated with the exposure produces biased estimates; a fixed-effects model applied where time-invariant effects are of interest cannot answer the question. The inferential goal must drive the choice.

### Separate Within-Person Change From Between-Person Differences

A core strength of longitudinal data is the ability to separate change within individuals from differences between them. Conflating the two is the most common analytic error and can reverse the direction of an effect.

Ensure the analysis:

- distinguishes within-person effects, how a unit changing over time relates to outcomes, from between-person effects, how units differing from each other relate to outcomes;
- avoids the ecological fallacy of inferring within-person change from between-person differences;
- considers centering time-varying predictors by their unit mean to separate the two components;
- reports which effect is being estimated, since within and between effects can have opposite signs.

A relationship that holds between persons need not hold within persons over time, and a design that cannot separate them will blur, or invert, the true dynamic.

### Recognize When Longitudinal Design Is Overkill

Not every question needs repeated measures, and a longitudinal design imposed on a static question adds cost, complexity, and attrition risk without benefit. The agent should resist the assumption that more waves always mean more rigor.

Longitudinal design is overkill when:

- the question is about stable between-group differences, not change;
- the phenomenon is effectively time-invariant over the study span;
- the added waves cannot answer a distinct sub-question;
- the attrition and cost would outweigh the temporal information gained.

Match the design to the question. A well-chosen cross-sectional study is stronger than a longitudinal study whose waves add noise and loss without adding inferential leverage.

## Common Traps

### Using Cross-Sectional Data To Infer Change

Differences between age groups or time points are not the same as within-person change, and conflating them produces the classic age-period-cohort confusion.

### Designing Waves For Convenience Rather Than The Phenomenon

Annual waves chosen because they fit a calendar will miss fast dynamics and waste effort on slow ones. The interval must match the rate of change.

### Underestimating Attrition

Optimistic dropout estimates leave a final wave too small and too selected to support the intended inference, and the loss is often differential.

### Treating Repeated Measures As Independent

Ignoring autocorrelation understates uncertainty and manufactures significance; the analysis must model the correlated structure.

### Defaulting To Random Effects Without Checking The Assumption

Random-effects models assume unit effects are uncorrelated with predictors, an assumption frequently violated in observational panels; applying them by default invites bias.

### Conflating Within-Person And Between-Person Effects

A between-person association can have the opposite sign from a within-person change, and a model that blends them reports a meaningless average.

### Adding Waves To Look Rigorous

More waves add cost, complexity, and attrition; they help only when each wave answers a distinct temporal question.

### Ignoring Aging, Period, And Cohort Confounding

In age-based panels, aging, period, and cohort effects are confounded, and a design that does not address this cannot attribute change cleanly to any one of them.

## Self-Check

- Has it been established that the question requires longitudinal data, rather than cross-sectional group differences?
- Is the time structure, number of waves, interval, and span, designed to match the rate and span of the change being studied?
- Is attrition estimated realistically from comparable studies, with oversampling, retention, and missing-data plans specified in advance?
- Does the analysis model the autocorrelation and clustering inherent in repeated measures, with a justified correlation structure?
- Is the choice between fixed and random effects driven by the inferential goal and the plausibility of the assumptions, not by software default?
- Are within-person change and between-person differences separated, with the reported effect clearly identified?
- Has the confounding of aging, period, and cohort effects been considered where age-based panels are used?
- Is it clear that each wave earns its cost by answering a distinct temporal question, rather than being added for the appearance of rigor?
- Are time-varying predictors centered or decomposed so within and between effects are not blurred?
- Are the limitations of the longitudinal design, especially attrition and assumed missingness mechanisms, carried honestly into the conclusion?
