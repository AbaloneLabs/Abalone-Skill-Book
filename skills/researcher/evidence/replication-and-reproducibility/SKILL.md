---
name: replication_and_reproducibility.md
description: Use when the agent is judging whether a finding can be reproduced, planning a replication, distinguishing direct from conceptual replication, preregistering, sharing data and code, interpreting replication failures, or assessing reproducibility risk in a published or proposed study.
---

# Replication And Reproducibility

A single study is a hypothesis about the world. Reproduction and replication are how a field decides whether to believe it. Researchers often treat their own results as established findings, omit the details that would let others re-run the analysis, or dismiss replication failures as bad luck. The result is a literature cluttered with effects that cannot be reproduced, methods that cannot be inspected, and conclusions that outrun the evidence.

Use this skill when designing a study to be reproducible, when planning or interpreting a replication, when sharing data and code, and when judging whether a published result can be trusted. The goal is to keep the agent from overclaiming from one study, from publishing analyses that cannot be re-run, and from rationalizing replication failures instead of learning from them. The agent has latitude in methods, but must build the study so that others can check the work.

## Core Rules

### Distinguish Reproducibility, Replicability, And Generalizability

These terms are often blurred, and the blurring produces confusion about what was actually tested.

Define the claim:

- computational reproducibility means re-running the same data and code yields the same results;
- replicability means collecting new data with the same design yields compatible results;
- direct replication repeats the original procedures as closely as possible;
- conceptual replication tests the same hypothesis with different procedures;
- generalizability asks whether the finding holds across populations, settings, or times.

A study can be computationally reproducible without being replicable, and replicable without generalizing. State which claim is being made.

### Design For Computational Reproducibility From The Start

Reproducibility is not a final-stage deliverable. It is built into how data, code, and analysis are managed throughout the project.

Establish:

- version control for code and analysis scripts;
- a deterministic analysis environment, with software versions and dependencies recorded;
- raw data preserved separately from cleaned data;
- a documented pipeline from raw data to figures and tables;
- random seeds for any stochastic step;
- clear file naming and folder structure;
- a README or equivalent explaining how to re-run the analysis;
- persistent identifiers for data and code where possible.

If a colleague cannot reproduce the numbers from the project files, neither can the field.

### Preregister To Separate Planned From Exploratory Work

Replication and reproduction are easier to interpret when the original analysis plan is on record. Preregistration clarifies what was planned and what emerged from exploration.

Preregister:

- hypotheses and primary outcomes;
- sample size rationale;
- exclusion rules and handling of missing data;
- analysis model and covariates;
- stopping rules and interim analyses;
- which analyses are exploratory;
- the decision rule for the primary test.

A registered report or time-stamped plan protects both original authors and replicators from post hoc reframing.

### Plan Replications To Match The Question

Replications fail for many reasons, and the design of the replication must match what is being tested.

Decide:

- whether the goal is direct or conceptual replication;
- how faithfully to reproduce original procedures, materials, and context;
- whether to use original sample size or power for the original effect estimate;
- how to handle materials that are unavailable or degraded;
- whether to consult original authors to confirm fidelity;
- how to define a successful replication, in advance;
- whether to pre-register the replication;
- how to handle deviations from the original design.

A replication powered for an inflated original effect will likely fail even if the effect is real but smaller. Plan power carefully.

### Define Replication Success Before Running It

What counts as a successful replication is contested, and deciding after seeing the result invites bias.

Predefine criteria such as:

- sign and significance of the effect;
- effect size within a predicted range or confidence interval;
- meta-analytic combination with the original study;
- equivalence or non-inferiority margins;
- prediction of the original effect from a model;
- whether subjective judgments of "same procedure" are required.

No single criterion is universally right, but the criterion must be chosen in advance and reported honestly.

### Make Data, Code, And Materials Available

Replication and reproduction require access to the building blocks. Withholding them makes verification impossible.

Share:

- raw data where ethically and legally possible;
- cleaned analysis datasets;
- analysis code and scripts;
- software environment specifications;
- stimuli, instruments, and protocols;
- transformation and coding rules;
- consent and data use terms that permit sharing;
- persistent links and identifiers, not personal web pages.

Where data cannot be shared, explain why and provide a controlled-access path. Silence is not an acceptable substitute.

### Interpret Replication Failures Without Rationalization

A replication failure is information, not an insult. The temptation to dismiss it as incompetence or context-dependence is strong and often unjustified.

Consider explanations:

- the original result was a false positive;
- the original effect is real but smaller than reported;
- the original effect is real but context-dependent;
- the replication deviated from the original in important ways;
- the replication was underpowered;
- hidden moderators differed between studies;
- analytical flexibility inflated the original.

Weigh these explanations with evidence, not loyalty. A field that explains away every failure learns nothing from replication.

### Address The Replication Ecosystem, Not Just Single Studies

Replication is a collective enterprise. Individual practices either help or harm the ecosystem.

Contribute by:

- publishing replications regardless of outcome;
- avoiding punitive treatment of replication authors;
- citing replications alongside originals;
- updating meta-analyses as new evidence arrives;
- rewarding open practices in hiring and promotion;
- disclosing flexibility and limitations in original studies;
- supporting registered reports and pre-registration;
- not penalizing honest null results.

A finding is not established by one study, and not refuted by one failure. Treat evidence as cumulative.

## Common Traps

### Treating A Single Significant Result As Established

One study, especially one with flexibility, can produce a false positive. Replication is what converts a finding into knowledge.

### Publishing Analyses That Cannot Be Re-Run

If the path from raw data to results is undocumented, the analysis is not reproducible, regardless of how clean the final paper looks.

### Dismissing Replication Failures As Context Effects

Claiming the effect "only works here" after a failure, without specifying the moderator in advance, is unfalsifiable defense of a cherished finding.

### Powering Replications For Inflated Original Effects

Original effects are often exaggerated by flexibility and publication bias. A replication powered for that inflated estimate will miss a real but smaller effect.

### Withholding Data And Code For Convenience

Claims of confidentiality, proprietary interest, or "data available on request" often mask a refusal to share. Genuine barriers have genuine solutions.

### Deciding Replication Success After Seeing Results

Choosing the criterion that flatters a preferred conclusion turns replication into rhetoric.

### Overreacting To A Single Failure

One failed replication does not erase an entire literature, just as one successful original does not establish it. Weigh the cumulative evidence.

## Self-Check

- [ ] Is the claim clearly framed as computational reproducibility, direct replication, conceptual replication, or generalizability?
- [ ] Is the analysis built for computational reproducibility, with version control, documented pipeline, random seeds, and environment capture?
- [ ] Are hypotheses, outcomes, exclusions, analyses, and decision rules preregistered, with planned and exploratory work separated?
- [ ] Is the replication design matched to its question, with fidelity to original procedures and appropriate power?
- [ ] Is the criterion for replication success defined in advance and reported honestly?
- [ ] Are data, code, materials, and software environment shared with persistent identifiers where possible?
- [ ] Are replication failures interpreted through evidence-based explanations rather than rationalization?
- [ ] Are original limitations, flexibility, and risk of bias disclosed rather than hidden?
- [ ] Does the work contribute to the cumulative replication ecosystem, including null and replication results?
- [ ] Are conclusions calibrated to the strength of reproducible and replicable evidence, not a single study?
