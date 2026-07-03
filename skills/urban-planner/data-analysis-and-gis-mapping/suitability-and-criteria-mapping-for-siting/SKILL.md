---
name: suitability-and-criteria-mapping-for-siting.md
description: Use when the agent is building a suitability model to site a facility or direct growth, selecting and weighting criteria, combining factors into a suitability score, interpreting results, or using mapped suitability to justify a siting or land allocation decision.
---

# Suitability and Criteria Mapping for Siting

Suitability analysis — combining mapped factors into a score that identifies where a land use or facility is most appropriate — is powerful precisely because it converts complex, multi-criteria judgments into a single, legible map. That power is also its danger: a suitability map looks objective, but it encodes every choice about which factors mattered, how they were measured, and how they were weighted, and those choices can be tuned to produce almost any desired outcome. The judgment problem is not how to run the model but how to select criteria that genuinely relate to the siting goal, weight them defensibly rather than to reach a preferred answer, handle factors that conflict or correlate, and present the result as decision support rather than as a directive. Planners commonly fail by hiding subjective weighting inside an apparently objective model, by omitting factors that would change the result, or by treating the highest-scoring parcels as the answer rather than as one input. Rigorous suitability practice makes the model's assumptions visible and its limits honest.

## Core Rules

### Select Criteria That Genuinely Determine Suitability

The criteria in the model should be the factors that actually determine whether a site is suitable for the intended use — for a school, that includes population of children, land availability, accessibility, environmental constraints, and equity considerations; for a solar array, solar resource, grid proximity, land cover, and environmental sensitivity. Each criterion should have a defensible causal or logical relationship to the siting goal, not just be available data that is easy to map. Omitting a genuinely important criterion (because the data is hard to get) or including an irrelevant one (because the data is handy) distorts the results. Document the rationale for each criterion's inclusion and the consequences of any omissions, so the model's scope is transparent.

### Measure Each Criterion with Data That Represents It

For each criterion, choose an indicator that actually measures the underlying condition: accessibility measured by network distance or transit access, not straight-line distance; environmental sensitivity measured by specific resources (wetlands, habitat, steep slopes), not a generic "greenness" layer; demand measured by the relevant population, not total population. Proxy indicators that correlate loosely with the real criterion introduce error and can bias the result toward areas that score well on the proxy but poorly on the real condition. Document the indicator chosen for each criterion and its fitness, and acknowledge where a weak proxy had to be used.

### Weight Criteria Defensibly and Transparently

The weights assigned to criteria often determine the result more than the criteria themselves, and they are the most subjective and contestable part of the model. Choose weights through a defensible process — structured expert elicitation, stakeholder weighting exercises, or analytic hierarchy process — rather than analyst intuition, and document the process so weights can be scrutinized and adjusted. Sensitivity-test the weights: run the model with different weighting schemes and see whether the top-scoring sites change. If small weight changes flip the result, the model is fragile and the decision should not rest on it alone. Never tune weights to make a preferred site score highest; this is the cardinal sin of suitability modeling and is detectable when the weighting is examined.

### Handle Conflicting and Correlated Criteria Honestly

Criteria often conflict — a site close to transit (good for housing) may be on contaminated land (bad) — and the model must decide how to combine them, typically through additive scoring, multiplicative combination, or hard constraints (exclusions). Choose the combination method deliberately: hard constraints for factors that are true disqualifiers (a school cannot go on a toxic site), additive scoring for tradeable factors, and multiplicative combination where factors must all be present. Beware of correlated criteria (e.g., income and educational attainment) that effectively double-count the same underlying condition and skew the result. Examine the correlation structure and avoid redundant criteria that inflate one dimension's influence.

### Present the Model as Decision Support, Not as the Answer

A suitability map identifies where conditions are more or less favorable according to the modeled criteria, but it does not capture everything that matters: parcel availability and owner willingness, political feasibility, costs not in the model, site-specific conditions, and community priorities. Present the results as one input to a decision that also requires site visits, community input, feasibility analysis, and policy judgment. Show the top-tier sites as candidates for further evaluation, not as the designated location. Overstating what the model determines invites decisions that ignore unmodeled factors and that fail when those factors assert themselves.

### Validate the Model Against Known Cases and Ground Truth

Test the model by running it against sites whose suitability is already known — existing facilities of the same type, sites previously considered and rejected, or sites with documented constraints — and see whether the model scores them sensibly. If the model ranks a known-bad site highly or a known-good site poorly, the criteria, indicators, or weights need revision. Ground-truth a sample of high-scoring sites with site visits or imagery to confirm the model's findings. Validation is what separates a defensible suitability model from a polished but unreliable one.

## Common Traps

### Hidden Subjective Weighting Inside an Objective-Looking Model

Assigning weights by analyst intuition, then presenting the result as if the model determined the outcome, conceals the most contestable assumption behind a veneer of objectivity. The false signal is a data-driven suitability map; the harm is decisions that reflect the analyst's (or sponsor's) preferences, presented as neutral analysis. Derive weights through a documented, defensible process and subject them to sensitivity testing.

### Omitting Criteria That Would Change the Result

Leaving out a criterion because the data is hard to obtain, or because including it would disadvantage a preferred site, produces a model whose results are artifacts of its scope. The false signal is a comprehensive-looking criteria set; the harm is suitability rankings that would reverse if the omitted factor were included. Include all genuinely determining criteria, and where data is unavailable, acknowledge the gap and its likely effect.

### Double-Counting Correlated Criteria

Including multiple criteria that measure the same underlying condition (income, education, and employment, which are correlated) gives that condition excessive weight in the model. The false signal is a rich, multi-factor model; the harm is results skewed toward or against areas defined by that condition. Examine correlations and consolidate or drop redundant criteria.

### Treating the Highest-Scoring Parcel as the Designated Site

Presenting the top-ranked parcel as the answer skips the feasibility, availability, and community-input steps that determine whether a site can actually be developed. The false signal is that the model has "selected" the site; the harm is decisions that ignore parcel availability, owner willingness, political feasibility, and unmodeled site conditions. Treat the model as producing candidates for further evaluation, not a designation.

### Skipping Validation Against Known Cases

Failing to test the model against sites of known suitability allows criteria, indicator, and weighting errors to reach the decision undetected. The false signal is a complete model run; the harm is rankings that do not correspond to reality and that collapse when a high-scoring site proves unbuildable. Validate against known cases and ground-truth before relying on the results.

## Self-Check

- Does each criterion in the model have a defensible causal or logical relationship to the siting goal, and are any genuinely important criteria omitted?
- Does each criterion use an indicator that actually measures the underlying condition, rather than a loose proxy whose weakness is unacknowledged?
- Were weights derived through a documented, defensible process (expert elicitation, stakeholder weighting, AHP), and were they sensitivity-tested?
- Did you handle conflicting criteria with an appropriate combination method (constraints, additive, multiplicative) and avoid double-counting correlated factors?
- Is the model presented as decision support — identifying candidates for further evaluation — rather than as designating the answer?
- Did you validate the model against sites of known suitability and ground-truth a sample of high-scoring sites?
- Did you avoid hiding subjective weighting inside an apparently objective model?
- Did you avoid omitting criteria because the data was hard to get or because inclusion would change the result?
- Did you avoid treating the highest-scoring parcel as the designated site without feasibility, availability, and community evaluation?
- Could a reviewer examine the criteria, indicators, weights, and validation and conclude the model is defensible — or would the assumptions not withstand scrutiny?
