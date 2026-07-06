---
name: publication_bias_and_meta-analysis-reporting.md
description: Use when the agent is assessing publication or small-study bias in a meta-analysis, interpreting funnel plot asymmetry, running trim-and-fill or Egger tests, or writing up a meta-analysis following PRISMA and reporting standards so the synthesis is reproducible and honest.
---

# Publication Bias And Meta-Analysis Reporting

A meta-analysis synthesizes the studies that exist, but the studies that exist are a biased sample of the studies that were conducted. Positive, significant, large studies are more likely to be published; null, small, or unfavorable studies are more likely to languish in file drawers. If this selection bias is ignored, a meta-analysis gives a precise, confident estimate of a biased literature. Equally, even an unbiased pool is useless if the report hides the methods and decisions that produced it. Reporting is not cosmetic; it is what makes the synthesis auditable.

Use this skill when assessing publication or small-study bias, interpreting asymmetry diagnostics, or writing up a meta-analysis so that readers can judge and reproduce it. The goal is to prevent the agent from presenting a polished pooled estimate that conceals selection bias or undocumented methods.

## Core Rules

### Anticipate Publication Bias As A Structural Problem

Assume, rather than dismiss, that the available literature is selected. The question is how much and in which direction, not whether it exists.

Anticipate by:

- searching grey literature, registries, and conference abstracts;
- noting the proportion of published versus unpublished or registered-but-unpublished studies;
- considering which results are structurally harder to publish in the field;
- checking for studies registered but not published, especially with reported outcomes.

A body of evidence drawn only from published, peer-reviewed, English-language journals is presumptively subject to selection.

### Use Multiple Diagnostics For Small-Study Effects

No single test is definitive, especially with few studies. Use complementary approaches and interpret them together.

Common diagnostics:

- funnel plot visual inspection for asymmetry;
- Egger's or similar regression test;
- rank-based or correlation tests;
- comparison of pooled estimates with and without small studies.

Each has low power with few studies and assumptions about the underlying mechanism. A symmetric funnel does not prove absence of bias, and asymmetry can have causes other than publication bias.

### Distinguish Publication Bias From Other Causes Of Asymmetry

Funnel plot asymmetry is a signal, not a diagnosis. Several phenomena can produce it, and the explanation changes the remedy.

Consider:

- true heterogeneity where small studies study higher-risk populations;
- methodological differences where smaller studies are lower quality;
- chance, especially with few studies;
- selective outcome or analysis reporting within studies;
- genuine publication selection.

Investigate the cause before attributing asymmetry to publication bias and adjusting for it.

### Apply Adjustment Methods With Disclosed Assumptions

Methods like trim-and-fill or selection models estimate what the effect might be under assumptions about suppressed studies. They are sensitivity analyses, not corrections to truth.

When applying:

- state the method and its assumptions explicitly;
- present the adjusted estimate as exploratory, not definitive;
- show the original and adjusted estimates together;
- note that adjustment methods themselves rest on untestable assumptions.

A trim-and-filled estimate is a model-based speculation about missing studies, not a recovered truth.

### Report The Direction And Plausibility Of Bias

Bias matters most when it plausibly inflates the conclusion. Interpret diagnostics in light of the direction and size of effects.

Ask:

- does the bias plausibly push toward a larger or more significant effect?
- would correcting for it change the conclusion materially?
- is the field one where null results are routinely suppressed?
- are the largest, most rigorous studies closer to or further from the null than the pool?

If small or low-quality studies drive the effect while large rigorous studies sit near the null, the conclusion is on shaky ground regardless of the pooled p-value.

### Follow A Recognized Reporting Standard

A meta-analysis should be reported so a reader can judge and reproduce it. Ad hoc reporting hides decisions.

Follow a standard such as PRISMA, including:

- the registration or protocol reference;
- the full search strategy;
- the PRISMA flow of records;
- eligibility criteria and changes;
- risk-of-bias and certainty assessments;
- the synthesis methods, model, and heterogeneity handling;
- funding and conflicts of interest.

Reporting standards are checklists of disclosure, not bureaucracy.

### Report Methods In Enough Detail To Reproduce

A reader with the included studies should be able to recompute the pooled estimate. Vague methods prevent this.

Disclose:

- the effect metric and any conversions with formulas;
- the pooling model and estimator (e.g., which variance estimator);
- how multi-arm and zero-event studies were handled;
- software, version, and packages or functions used;
- all pre-specified and post-hoc analyses, clearly labeled.

Reproducibility is the test of whether the methods were real.

### Separate The Pooled Estimate From The Certainty

The pooled effect and the certainty of the evidence are independent judgments. Report both and keep them distinct in the conclusion.

Report:

- the pooled effect with confidence interval;
- the certainty rating and the reasons for any downgrading;
- the implication of any gap between a notable effect and low certainty.

A conclusion that reports only the effect size, or only the certainty, omits half of what the reader needs.

### Disclose Limitations At The Synthesis Level and resist Overstating The Synthesis

A meta-analysis has limitations beyond those of its studies. Naming them is more credible than implying the pooling removed uncertainty.

Disclose:

- the likely impact of publication bias;
- unmeasured heterogeneity and its sources;
- the limits of the included evidence's directness;
- any deviation from protocol and its impact;
- the scope of the conclusion (which populations, settings, outcomes).

A meta-analysis is not the final word; it is a structured summary of available, selected evidence. Calibrate the language to that status.

Calibrate by:

- avoiding "proves" or "definitively shows";
- noting what the synthesis can and cannot establish;
- distinguishing a confirmed effect from one needing replication;
- stating where evidence is insufficient to conclude.

## Common Traps

### Assuming A Symmetric Funnel Means No Bias

Funnel diagnostics have low power; symmetry does not rule out selection, especially with few studies.

### Treating Adjustment As Correction

Trim-and-fill and selection models are assumption-laden sensitivity analyses, not recovery of missing truth.

### Ignoring The Direction Of Bias

Bias that would shrink the conclusion matters more than bias that would inflate it only trivially.

### Vague Methods That Prevent Reproduction

Reporting a pooled estimate without the metric, model, estimator, and handling of edge cases blocks verification.

### Conflating Effect With Certainty

A large pooled effect from low-certainty evidence is a hypothesis, not a conclusion.

### Overstating The Synthesis As Definitive

A meta-analysis of selected studies is still a summary of selected studies, not proof.

### Hiding Protocol Deviations

Changes made after seeing data, if unreported, convert a systematic synthesis into a post-hoc narrative.

## Self-Check

- Has publication and small-study bias been anticipated and investigated with multiple diagnostics?
- Is funnel plot asymmetry investigated for causes beyond publication bias?
- Are adjustment methods applied with assumptions disclosed and results shown alongside the original?
- Is the likely direction and materiality of bias assessed against the conclusion?
- Does the report follow a recognized standard such as PRISMA, including registration and flow?
- Are methods detailed enough (metric, model, estimator, edge-case handling, software) to reproduce the pool?
- Are the pooled effect and the certainty of evidence reported and kept distinct?
- Are synthesis-level limitations, including selection and heterogeneity, explicitly disclosed?
- Are pre-specified and post-hoc analyses clearly labeled?
- Is the conclusion calibrated to the status of the evidence rather than overstated as definitive?
