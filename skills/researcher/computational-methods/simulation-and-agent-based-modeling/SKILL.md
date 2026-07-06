---
name: simulation-and-agent-based-modeling.md
description: Use when the agent is building a simulation or agent-based model, designing model assumptions and rules, calibrating and validating a model against empirical data, conducting sensitivity and uncertainty analysis, or guarding against building a model whose outputs reflect unexamined assumptions rather than the phenomenon under study.
---

# Simulation And Agent-Based Modeling

Simulation and agent-based modeling (ABM) build artificial worlds to explore how macro-level patterns emerge from micro-level rules and interactions. Their great strength is the ability to experiment with mechanisms that are hard or impossible to manipulate in reality: what happens if agents decide this way, interact on this network, or adapt with this rule? Their great danger is that a simulation is only as credible as its assumptions, and those assumptions are easy to hide inside elegant code. A model can produce striking emergent patterns that look like the phenomenon but actually reflect the modeler's built-in rules rather than any real mechanism. The judgment problem is to justify each assumption against theory or evidence, to calibrate and validate the model against empirical data, and to treat simulation outputs as conditional on assumptions rather than as discoveries about the world. A simulation whose outputs are never tested against data, or whose assumptions are never examined, is a thought experiment presented as a result.

Use this skill when building or evaluating a simulation or agent-based model. The goal is to prevent the agent from building unvalidated models, from hiding assumptions in code, and from presenting conditional outputs as empirical findings. The agent has substantial freedom in model design, but every assumption, calibration, and output must be explicit, justified, and tested.

## Core Rules

### Justify Every Model Assumption Against Theory Or Evidence

A model is a collection of assumptions. Each one must be defensible, because outputs flow directly from them.

Justify by:

- the agent decision rules and where they come from, theory, evidence, or convenience;
- the interaction structure and whether it reflects real networks or spaces;
- the heterogeneity of agents and whether it captures relevant variation;
- the time dynamics and whether they match the phenomenon's timescale.

Assumptions chosen for coding convenience rather than empirical or theoretical grounding produce outputs that reflect the convenience, not the phenomenon. Each assumption needs a stated basis.

### State The Model's Purpose Precisely

A simulation can explain, predict, or explore. The purpose determines what validation and claims are appropriate.

State by:

- whether the goal is explanation of an observed pattern, prediction of future states, or theoretical exploration;
- what question the model is built to answer;
- how the purpose shapes the required validation;
- the scope of claims the purpose licenses.

An exploratory model need not predict accurately, but it must illuminate mechanisms; a predictive model must be tested against held-out data. Conflating purposes produces claims the model cannot support.

### Calibrate The Model Against Empirical Data

A model whose parameters are guessed cannot be trusted to represent the phenomenon. Calibration grounds it.

Calibrate by:

- estimating parameters from empirical data where possible;
- matching model outputs to observed stylized facts or patterns;
- using multiple empirical targets, not just one;
- reporting how calibration was done and how well the model fits.

A model calibrated to one pattern may miss others. Calibration against several empirical regularities is stronger evidence that the model captures something real.

### Validate Beyond Calibration Data

Calibration fits known data; validation tests against data the model did not see. Both are needed.

Validate by:

- testing the model against held-out or independent empirical patterns;
- checking whether the model reproduces phenomena not used in calibration;
- comparing alternative models on the same validation targets;
- acknowledging where validation succeeds and where it fails.

A model that fits its calibration data but fails on independent validation has memorized, not explained. Honest validation includes reporting failures.

### Conduct Sensitivity And Uncertainty Analysis

Model outputs depend on parameters and assumptions whose values are uncertain. Sensitivity analysis reveals how much.

Conduct by:

- varying parameters across plausible ranges and observing output changes;
- identifying which assumptions drive the key results;
- exploring structural uncertainty, not just parameter uncertainty;
- reporting where conclusions are robust and where they are fragile.

A result that holds only for one parameter value is not a finding; it is an artifact of that value. Robustness across plausible variation is essential before trusting outputs.

### Examine Emergence Honestly

ABM's appeal is emergence, macro patterns from micro rules. But emergence must be analyzed, not just admired.

Examine by:

- showing which macro patterns actually emerge from the micro rules;
- distinguishing emergent outcomes from those built in by construction;
- tracing the mechanism by which micro rules produce the macro pattern;
- testing whether removing or altering a rule removes the pattern.

A "surprising" emergent pattern may be a direct consequence of a rule the modeler inserted. Honest emergence analysis shows the causal chain from rules to outcome.

### Avoid Circular Model-Data Relationships

A model calibrated and validated on the same data, or built to reproduce a pattern by encoding it, proves little.

Avoid by:

- separating calibration and validation data;
- not encoding the target pattern directly into agent rules;
- testing the model on patterns it was not built to reproduce;
- recognizing the risk of building in the answer.

If the model reproduces a pattern because the rules were designed to, the reproduction is circular and confirms nothing. The model must earn its fit against independent evidence.

### Document And Share The Model For Reproducibility

A simulation that cannot be re-run is unverifiable. Full documentation and open code are expected.

Document by:

- the full model specification, following standards such as ODD where relevant;
- the code, ideally open-source and versioned;
- the parameter values and random seeds for reported runs;
- the data used for calibration and validation.

A model described only in prose cannot be checked or extended. Reproducibility is part of credibility, especially given the many hidden choices in simulation code.

## Common Traps

### Unjustified Assumptions

Rules chosen for convenience, not theory or evidence, produce outputs that reflect the convenience.

### Model Purpose Conflated

Treating an exploratory model as predictive, or a predictive model as explanatory, overclaims what it shows.

### No Empirical Calibration

A model with guessed parameters cannot be trusted to represent the real phenomenon.

### Validation On Calibration Data

Testing only against the data used to fit the model confirms memorization, not explanation.

### Skipped Sensitivity Analysis

Reporting outputs without testing robustness to parameter and structural uncertainty hides fragile results.

### Built-In Emergence

Presenting as emergent a pattern that is a direct consequence of inserted rules.

### Circular Reproduction

Encoding the target pattern into the model and then claiming the model reproduces it.

### Undocumented Unverifiable Model

A model without open code and specification cannot be checked, reproduced, or trusted.

## Self-Check

- Is every model assumption justified against theory or evidence, with its basis stated?
- Is the model's purpose, explanation, prediction, or exploration, stated precisely and matched to validation?
- Is the model calibrated against empirical data using multiple targets where possible?
- Is the model validated against independent or held-out data, with failures reported?
- Is sensitivity and uncertainty analysis conducted across plausible parameter and structural variation?
- Is emergence examined honestly, distinguishing emergent from built-in outcomes?
- Are circular model-data relationships avoided, with independent validation targets?
- Is the model fully documented and shared, with code, parameters, seeds, and data?
- Are the conditions under which conclusions hold, and break, made explicit?
- Does the report present outputs as conditional on assumptions rather than as bare discoveries?
