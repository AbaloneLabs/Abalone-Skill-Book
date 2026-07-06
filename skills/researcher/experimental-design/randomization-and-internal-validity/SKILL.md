---
name: randomization_and_internal_validity.md
description: Use when the agent is designing a randomized experiment, choosing a randomization scheme, protecting internal validity, planning blinding and allocation concealment, or diagnosing threats such as selection, history, maturation, and attrition that could undermine causal inference.
---

# Randomization And Internal Validity

An experiment earns its causal claim through design, not through statistics. Randomization is what, on average, balances known and unknown confounders across conditions, and the rest of internal validity is the work of ensuring that nothing other than the intended manipulation produces the observed differences. Agents often treat randomization as a magic word that confers causality, then ignore the threats (differential attrition, failed blinding, experimenter effects, contamination) that can corrupt even a randomized design. A well-randomized study with broken blinding or selective attrition can produce a biased causal estimate as readily as an observational study.

Use this skill when designing a randomized experiment, choosing a randomization scheme, or auditing a design for internal-validity threats. The goal is to prevent the agent from assuming randomization alone guarantees a valid causal inference.

## Core Rules

### Match The Unit Of Randomization To The Unit Of Analysis

Randomization must occur at the unit that is assigned to conditions, and analysis must respect that unit. Mismatched units produce false precision.

Align by:

- randomizing individuals when individuals are assigned;
- randomizing clusters (classes, clinics, teams) when groups are assigned, and analyzing at the cluster level or with multilevel models;
- reflecting the design's intraclass correlation in power and analysis;
- avoiding treating clustered units as independent.

Individual-level analysis of a cluster-randomized design inflates false positives.

### Choose A Randomization Scheme Suited To The Context

Simple randomization can produce imbalanced groups in small studies. Restricted schemes improve balance and credibility.

Consider:

- simple randomization for large samples;
- blocked or stratified randomization to balance prognostic factors;
- matched or paired designs for small samples;
- minimization for small trials with several prognostic factors.

Document the scheme and its implementation so the process is auditable.

### Conceal Allocation Until Assignment

If those recruiting or enrolling participants can foresee the next assignment, selection bias can enter even a randomized study.

Ensure allocation concealment by:

- using sequentially numbered, opaque, sealed envelopes or a central system;
- keeping the randomization sequence away from those enrolling;
- preventing foreknowledge of upcoming assignments;
- documenting that concealment held until assignment.

Randomization without concealment is compromised randomization.

### Blind Where Outcomes Are Subjective Or Measurable By Staff

Blinding reduces performance and detection bias, especially when outcomes involve judgment. Where blinding is infeasible, its absence must be addressed.

Blind:

- participants, where feasible and ethical;
- outcome assessors, especially for subjective measures;
- analysts, where possible, through masked analysis;
- document who was blinded and who could not be.

Unblinded subjective outcomes invite expectancy effects; if unblinded, acknowledge and mitigate.

### Anticipate And Address Attrition

Differential attrition between conditions destroys the balance randomization created and biases the estimate.

Plan for attrition by:

- estimating and powering for expected dropout;
- tracking and reporting attrition by condition;
- using intent-to-treat analysis to preserve randomization;
- conducting sensitivity analysis for missing data assumptions.

Per-protocol analysis on a subset with differential attrition reintroduces selection bias.

### Diagnose The Classic Internal-Validity Threats

Even randomized designs face threats beyond confounding. Each must be considered and, where possible, designed out.

Check for:

- history: external events coinciding with the intervention;
- maturation: natural change over time, especially in within-subject designs;
- testing and instrumentation: changes due to repeated measurement or altered instruments;
- regression to the mean: selection on extreme scores;
- contamination: exposure of control to the intervention;
- resentful demoralization or compensatory rivalry in controls.

Add control groups, placebo conditions, or design features to isolate the manipulation.

### Verify The Manipulation Worked

A null result is ambiguous if the intervention was not delivered or not perceived as intended. A manipulation check confirms the independent variable operated.

Verify by:

- measuring whether the manipulation produced the intended psychological or physical state;
- checking implementation fidelity and dose;
- confirming participants noticed and understood the condition;
- distinguishing a true null from a failed manipulation.

### Plan The Analysis To Respect The Design

The analysis must match the design that generated the data. Post-hoc analytic flexibility erodes the causal claim.

Pre-specify:

- the primary contrast and outcome;
- covariates chosen a priori, not for significance;
- the handling of clustering, repeated measures, and missing data;
- intent-to-treat as the primary analysis for trials.

### Use Intent-To-Treat As The Primary Causal Estimate and distinguish Efficacy From Effectiveness

Once randomized, participants belong to their assigned group for analysis, regardless of adherence. This preserves the randomization comparison.

Apply ITT by:

- analyzing participants in their assigned condition;
- not excluding non-adherers, which reintroduces selection bias;
- using per-protocol or as-treated only as secondary, with caveats;
- recognizing ITT estimates the effect of assignment, which may differ from the effect of adherence.

A tightly controlled efficacy experiment answers whether the intervention can work under ideal conditions; effectiveness asks whether it works in practice. Conflating them overclaims.

Clarify:

- efficacy: ideal conditions, selected participants, high control;
- effectiveness: real-world conditions, broader populations, practical implementation;
- the design's setting and sample determine which question is answered.

## Common Traps

### Randomization Treated As Causality Guarantee

Randomization balances confounders on average, but broken blinding, attrition, and contamination can still bias the estimate.

### Unit Of Randomization And Analysis Mismatched

Individual analysis of cluster-randomized data inflates precision and false positives.

### Visible Or Predictable Allocation

Foreseeable assignment allows selection bias to enter despite randomization.

### Differential Attrition Ignored

Dropout that differs by condition destroys the balance randomization created.

### Failed Manipulation Read As Null Effect

A null with no manipulation check is ambiguous between no effect and no delivery.

### Per-Protocol Treated As Primary

Excluding non-adherers reintroduces selection bias and compromises the causal estimate.

### Efficacy Overclaimed As Effectiveness

Ideal-condition results do not automatically transfer to real-world practice.

## Self-Check

- Does the unit of randomization match the unit of analysis, with clustering reflected?
- Is the randomization scheme appropriate to the sample size and context, and documented?
- Is allocation concealed from those enrolling until assignment occurs?
- Are participants, assessors, and analysts blinded where outcomes are subjective, with gaps acknowledged?
- Is attrition anticipated, tracked by condition, and addressed with intent-to-treat and sensitivity analysis?
- Are the classic internal-validity threats (history, maturation, testing, regression, contamination) diagnosed and designed against?
- Is there a manipulation or fidelity check confirming the intervention was delivered as intended?
- Does the pre-specified analysis respect the design, with covariates chosen a priori?
- Is intent-to-treat the primary causal estimate, with per-protocol secondary and caveated?
- Is the claim calibrated as efficacy or effectiveness to match the design's conditions?
