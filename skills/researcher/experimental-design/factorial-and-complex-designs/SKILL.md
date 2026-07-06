---
name: factorial_and_complex_designs.md
description: Use when the agent is designing a factorial or fractional factorial experiment, planning interactions between factors, deciding on between versus within factors, interpreting interaction effects, or building complex designs that test multiple independent variables and their combinations efficiently.
---

# Factorial And Complex Designs

Factorial designs let a single study answer several questions at once: does each factor matter, and do they interact? This efficiency is also their danger. Agents often add factors without planning the comparisons that matter, misinterpret main effects in the presence of interactions, or run complex designs without enough power to detect the interactions that justify the complexity. A factorial design is not several experiments glued together; it is one experiment whose logic depends on how factors combine, and whose interpretation depends on whether interactions are present.

Use this skill when designing factorial or fractional factorial experiments, planning interactions, or interpreting complex designs. The goal is to prevent the agent from building designs whose structure outstrips their power and whose results are misread.

## Core Rules

### Justify Each Factor And Its Levels Theoretically

Every factor adds complexity and demands power. Factors should be included because they address a question, not because data are available.

Justify by:

- the theoretical or practical question each factor addresses;
- the rationale for the chosen levels;
- whether the factor is expected to interact with others;
- the cost of each added factor in power and sample size.

A factor without a question is design clutter that dilutes power.

### Decide Whether Interactions Are Of Interest A Priori

The purpose of a factorial design is often to detect interactions. Whether interactions are confirmatory or exploratory changes the power and the interpretation.

Decide:

- which interactions are hypothesized, with predicted directions;
- whether interactions are the point or a nuisance to check;
- the power required to detect the smallest meaningful interaction;
- whether main effects are interpretable given possible interactions.

Underpowering the interaction while powering main effects defeats the purpose of a factorial test of moderation.

### Power For The Smallest Effect Of Interest, Usually The Interaction

Interactions are typically smaller and harder to detect than main effects. A study powered for main effects is often underpowered for interactions.

Power by:

- sizing for the smallest interaction of interest, not the largest main effect;
- accounting for the design's structure (between, within, mixed);
- expecting that interaction effects are often half the size of main effects;
- planning the analysis to respect the design.

A non-significant interaction in an underpowered design is not evidence of no interaction.

### Interpret Main Effects Carefully In The Presence Of Interactions

When an interaction is present, main effects are averaged across levels of the other factor and may be misleading. The interaction, not the main effects, tells the story.

Interpret by:

- examining simple effects at each level of the moderating factor;
- not reporting main effects as the headline when an interaction exists;
- noting that a non-significant main effect can coexist with strong simple effects;
- plotting the interaction to communicate the pattern.

Reporting only main effects when an interaction is detected misrepresents the result.

### Choose Between, Within, And Mixed Factor Structures Deliberately

Whether a factor is between or within subjects changes power, feasibility, and vulnerability to carryover.

Choose by:

- within factors for efficiency when carryover is minimal;
- between factors when the manipulation is contaminating or irreversible;
- mixed designs when some factors suit within and others between;
- counterbalancing and assessing order effects for within factors.

A within factor with unaddressed carryover produces artifacts indistinguishable from effects.

### Use Fractional Factorial Designs When Full Factorials Are Infeasible

When many factors make a full design impractical, fractional designs can estimate main effects and selected interactions efficiently, but with aliasing tradeoffs.

Use fractional designs by:

- accepting that some effects are aliased with others;
- choosing the fraction to keep effects of interest separable;
- assuming higher-order interactions are negligible, and stating it;
- recognizing the design answers fewer questions than a full factorial.

Fractional designs are efficient but rest on assumptions about which interactions can be ignored.

### Plan The Analysis To Match The Design Structure

The analysis must reflect the factorial structure, including all factors and their interactions, and the correct error terms for within and mixed designs.

Plan:

- the full model including all designed factors;
- the correct error terms, especially for within and mixed designs;
- simple-effects analyses for significant interactions;
- correction for multiple comparisons where many simple effects are tested.

Analyzing a factorial design as a series of separate t-tests discards the design's logic and inflates error.

### Distinguish Confirmatory From Exploratory Interactions

With many factors, the number of possible interactions explodes. Testing all of them and reporting significant ones is multiplicity.

Distinguish:

- confirmatory interactions pre-specified with hypotheses;
- exploratory interactions generated by examining the data;
- the correction or labeling needed for exploratory tests;
- the lower credibility of exploratory interaction findings.

A significant interaction found among many tested is a hypothesis, not a finding.

### Consider Random Versus Fixed Factors And Generalizability

Whether a factor is fixed (specific levels of interest) or random (a sample of levels) changes what generalizes.

Clarify:

- fixed factors generalize to the levels studied;
- random factors support generalization across the population of levels;
- the choice affects the error structure and the inference;
- nested or crossed structures must be reflected in the model.

Treating a random factor as fixed limits the generalization the study can claim.

## Common Traps

### Adding Factors Without Questions

Each factor without a rationale dilutes power and complicates interpretation.

### Powering Main Effects, Not Interactions

Underpowered interactions produce non-significance misread as no interaction.

### Main Effects Reported When Interaction Exists

Averaging across levels of a moderator hides the actual pattern of effects.

### Within Factors With Unaddressed Carryover

Order and practice effects masquerade as factor effects.

### Fractional Designs Ignoring Aliasing

Aliased effects presented as independent estimates mislead about what was tested.

### Exploratory Interactions As Confirmatory

Significant interactions among many tested are hypotheses, not findings.

### Wrong Error Terms In Mixed Designs

Incorrect error structure produces invalid tests, especially for within and random factors.

## Self-Check

- Is each factor and its levels justified by a theoretical or practical question?
- Are the interactions of interest specified a priori, with predicted directions?
- Is the study powered for the smallest effect of interest, typically the interaction?
- Are main effects interpreted cautiously when an interaction is present, with simple effects examined?
- Are between, within, and mixed structures chosen deliberately, with carryover addressed?
- If fractional, are aliasing assumptions stated and effects of interest kept separable?
- Does the planned analysis match the factorial structure with correct error terms?
- Are confirmatory and exploratory interactions distinguished, with multiplicity addressed?
- Are random and fixed factors distinguished, with generalization calibrated accordingly?
- Would the design's logic be clear to a reader, including which comparisons answer which questions?
