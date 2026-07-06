---
name: pedagogy_and_intervention_evaluation.md
description: Use when the agent is evaluating a teaching method, curriculum, or educational intervention, designing a study of instructional effectiveness, measuring learning gains, or deciding whether a pedagogical change actually caused observed outcomes.
---

# Pedagogy And Intervention Evaluation

Evaluating an educational intervention is harder than it looks. Learners are nested in classes, classes in teachers, teachers in schools, and all of these move and change over time. A teaching method that seems to work often reflects the enthusiasm of a new approach, the skill of a particular teacher, or simply the passage of time, rather than the method itself. When intervention evaluation is handled casually, three harms follow. Schools adopt methods that do not actually work, wasting money and students' time. Ineffective or harmful practices persist because weak studies could not distinguish effect from noise. And the most vulnerable students bear the cost of interventions tried on them without rigorous evidence. Pedagogy evaluation is a causal inference problem, and it deserves causal inference rigor.

The agent should use this skill when designing a study of a teaching method, curriculum, or educational technology, measuring learning outcomes, interpreting claims about what works, or advising on whether to adopt a practice. The goal is to keep the agent from confusing enthusiasm and anecdote with evidence, when the question of whether an intervention works demands a design that can separate cause from coincidence.

## Core Rules

### Define The Intervention And Its Active Ingredients Precisely

An intervention is not just a name. Two implementations of project-based learning can be entirely different practices. Define what the intervention actually is.

- The specific activities, materials, and procedures.
- The dose, how often, how long, how intense.
- The intended mechanism, why it should affect learning.
- What the control or comparison condition receives.
- The boundary, what is and is not part of the intervention.

Without a precise definition, the study measures an undefined treatment, and the result cannot be replicated or applied. A finding about an unspecified intervention tells no one what to do.

### Choose A Design That Supports Causal Inference

The question of whether an intervention works is causal. Match the design to the strength of claim desired.

- Randomized controlled trials, when feasible, assign students, classes, or schools to conditions.
- Cluster randomization, when individuals cannot be randomized independently, randomizing classes or schools.
- Quasi-experimental designs, such as regression discontinuity or matched comparison, when randomization is not possible.
- Single-subject or within-participant designs for individual-focused questions.
- Pre-post designs only as weak preliminary evidence, because they cannot rule out maturation or history effects.

A pre-post comparison of one class before and after a new method cannot establish that the method caused any change. State honestly what the design can and cannot infer.

### Account For Clustering And Nesting

Educational data are nested. Students within a class share a teacher and environment; classes within a school share a context. Ignoring clustering produces false precision.

- Identify the unit of treatment, whether the intervention is delivered to students, classes, or schools.
- Match the unit of analysis to the unit of treatment where possible.
- Use multilevel models or cluster-robust inference to account for nesting.
- Report intraclass correlation to show how much variance is at each level.

Treating clustered data as independent observations inflates significance and produces conclusions that do not replicate. Clustering is not a statistical nuisance; it is the structure of the setting.

### Measure Learning With Valid And Sensitive Instruments

An intervention can only be shown to affect learning if learning is measured well.

- Use assessments with evidence of validity for the intended construct and population.
- Ensure the assessment is sensitive to the kind of learning the intervention targets.
- Avoid instruments so easy or hard that ceiling or floor effects hide change.
- Consider multiple measures, including transfer and retention, not only immediate performance.
- Beware of tests aligned to the intervention in ways that favor it unfairly.

A study using a researcher-made test tightly matched to the intervention's content will show gains that disappear on independent measures. Measurement choice can manufacture or mask effects.

### Control For Maturation, History, And Practice Effects

Learners change over time regardless of intervention. They mature, practice, and encounter outside experiences. Distinguish intervention effects from these.

- Use a comparison group experiencing the same time passage.
- Consider practice effects from repeated testing.
- Account for historical events that coincide with the intervention period.
- Beware of novelty and Hawthorne effects, where any change produces temporary improvement.

A gain observed only in the intervention group, with no comparison, is as likely to be maturation or novelty as the method. Design to rule out these alternatives.

### Track Fidelity Of Implementation

An intervention only works if it is actually delivered as intended. Measure fidelity.

- Observe or record whether the intervention was implemented as designed.
- Track dose actually received, not just planned.
- Document adaptations teachers make and why.
- Relate fidelity to outcomes, because low fidelity can mask a real effect or mimic one.

A study reporting no effect may simply have failed to implement the intervention. A study reporting an effect may have implemented something quite different from what was described. Fidelity data are essential to interpret either.

### Consider Differential Effects Across Learners

An intervention rarely affects all learners equally. Look for differential effects.

- Pre-specified subgroup analyses by prior achievement, background, or needs.
- Whether the intervention helps those who need it most or mainly the already-advantaged.
- Whether effects differ by teacher, class, or context.
- Whether harms appear for any subgroup.

Average effects can hide that an intervention helps some and harms others. Equity demands attention to who benefits and who does not.

### Assess Practical Significance And Sustainability

A statistically significant effect may be too small to matter, and a effect in a controlled study may not survive real-world conditions.

- Report effect sizes in meaningful units, such as months of learning or proportion of students reaching a standard.
- Judge whether the effect justifies the cost and effort.
- Test sustainability, whether effects persist after the intervention ends.
- Consider scalability, whether the method works with ordinary teachers and resources.

A method that works only with exceptional teachers or extra resources is not a general solution. Practical significance and sustainability determine whether a finding should change practice.

### Avoid Bias From Researcher And Teacher Investment

Researchers and teachers who designed or chose an intervention want it to work. This investment biases measurement and interpretation.

- Use blinded outcome assessment where possible.
- Use independent raters for qualitative judgments.
- Pre-register outcomes and analysis plans.
- Report all measured outcomes, not only favorable ones.
- Disclose the researcher's stake in the intervention.

Investment bias is not dishonesty; it is a systematic tendency that rigorous design must counter. A study run entirely by the intervention's advocates carries extra burden to demonstrate objectivity.

## Common Traps

### Undefined Interventions

A study of an unspecified method tells no one what to do. Define the active ingredients.

### Pre-Post Designs Presented As Causal Evidence

Without a comparison group, gains may reflect maturation or novelty. Use a design that supports causal inference.

### Ignoring Clustering

Treating nested data as independent inflates significance. Account for the structure.

### Tests Aligned To Favor The Intervention

Researcher-made tests matched to the intervention manufacture effects. Use valid, independent measures.

### Confusing Novelty With Effectiveness

Any change can produce temporary improvement. Rule out novelty and Hawthorne effects.

### Ignoring Implementation Fidelity

No effect may mean no implementation. Measure and report fidelity.

### Reporting Only Average Effects

Averages can hide harms to subgroups. Examine differential effects.

### Overclaiming From Controlled Conditions

A controlled-study effect may not survive real classrooms. Assess practical significance and sustainability.

## Self-Check

- Is the intervention defined precisely, including activities, dose, mechanism, comparison condition, and boundaries?
- Does the design, randomized, quasi-experimental, or other, actually support the causal claim being made?
- Is clustering and nesting accounted for in analysis, with the unit of analysis matched to the unit of treatment?
- Are learning outcomes measured with valid, sensitive, independent instruments?
- Are maturation, history, practice, and novelty effects controlled through design?
- Is implementation fidelity measured and related to outcomes?
- Are differential effects across learners, especially vulnerable groups, examined?
- Are effect sizes, practical significance, sustainability, and scalability assessed?
- Is researcher and teacher investment bias countered through blinding, pre-registration, and full outcome reporting?
- For interventions intended for wide adoption or affecting vulnerable learners, has an experienced education researcher or methodologist reviewed the evaluation design before implementation?
