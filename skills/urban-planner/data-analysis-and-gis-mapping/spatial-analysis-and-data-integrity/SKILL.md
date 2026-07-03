---
name: spatial-analysis-and-data-integrity.md
description: Use when the agent is conducting GIS analysis for a plan, choosing data sources, projecting and transforming data, calculating densities and distances, validating results, or building the spatial evidence base for planning decisions.
---

# Spatial Analysis and Data Integrity

GIS analysis has become the evidentiary backbone of planning — the maps and numbers that justify a rezoning, a facility siting, an equity finding, or a capital investment — and because the outputs look authoritative, the integrity of the underlying analysis is rarely questioned until it is wrong. The judgment problem is not how to run the software but how to choose data that actually represents what it claims to, to apply spatial operations correctly for the question being asked, to recognize the uncertainty embedded in every layer, and to communicate results honestly rather than with false precision. Planners commonly fail by treating all datasets as equivalent, by running analyses whose assumptions they cannot articulate, by ignoring scale and projection errors that distort results, or by presenting model outputs as fact. Rigorous spatial analysis is what makes a plan's evidence trustworthy; sloppy analysis produces confident-looking maps that are quietly wrong.

## Core Rules

### Know Your Data Sources, Their Vintage, and Their Limits

Every dataset has a source, a collection method, a vintage, a resolution, and known limitations — and these determine what questions it can credibly answer. Census ACS data is a survey with margins of error that can be large at small geographies; assessor data may lag transactions and misclassify uses; parcel boundaries may not match legal descriptions; land cover data generalizes reality at a pixel size. Before using any dataset, document its provenance, its date, its resolution, and its known issues, and assess whether it is fit for the intended purpose. A demographic analysis at the block-group level using ACS data with 40% margins of error is not a precise finding; it is a rough indicator, and the plan should say so. Mismatched data to question is the most common analytical failure.

### Match the Spatial Unit to the Question Being Asked

The choice of analysis unit — parcel, block, block group, tract, district, buffer — determines what the analysis can reveal and what it hides. A school-capacity analysis by attendance area answers a different question than one by census tract; a "within half a mile" buffer analysis answers a different question than a network-distance analysis. The modifiable areal unit problem (MAUP) means that different aggregations of the same underlying data can produce different, even opposite, results, so the unit choice is an analytical decision that must be justified, not defaulted. State why the chosen unit is appropriate to the question, and test whether results are robust to alternative aggregations before presenting them as findings.

### Handle Projection, Topology, and Scale Correctly

Spatial data must be in a projection appropriate to the analysis, or distances, areas, and shapes will be distorted in ways that bias results. Use an equal-area projection for area calculations (acreage, density) and an equidistant or local planar projection for distance and buffer analysis; never calculate areas or distances in a geographic (lat/long) coordinate system. Verify topology — that parcels do not overlap or have gaps, that lines connect where they should — because topological errors propagate through overlay and network analysis. Be conscious of scale: a dataset compiled at 1:100,000 should not be used to make parcel-level findings. These technical decisions are not cosmetic; they determine whether the numbers are right.

### Validate Analysis Results Against Ground Truth

Before presenting GIS findings, validate them against independent reality: do field checks confirm the land use the map shows, do the parcel counts match the assessor's totals, do the demographic totals reconcile with published counts, do the buffer results make sense on a satellite image? Discrepancies reveal data errors, projection problems, or analytical mistakes that would otherwise propagate into the plan. A culture of validation — checking outputs against known references before publishing — is what separates reliable analysis from confident error. Document the validation steps so reviewers can confirm the analysis was checked.

### Quantify and Communicate Uncertainty

Every analytical output carries uncertainty: sampling error in survey data, classification error in land use, model error in suitability or forecast models, and propagation of these errors through multi-step analyses. Where possible, quantify the uncertainty (confidence intervals, sensitivity ranges, scenario bounds) and present findings as ranges or probabilities rather than point estimates. At minimum, describe the uncertainty qualitatively so decision-makers understand that a "23% increase" may be a 15-30% range, not a precise figure. False precision — presenting model outputs as exact — misleads decision-makers and undermines credibility when reality diverges from the prediction.

### Make Analysis Reproducible and Documented

An analysis that cannot be reproduced cannot be verified, defended, or updated. Document the data sources (with versions and dates), the processing steps, the tools and parameters used, and the assumptions made, so that another analyst could reproduce the results from the documentation. Use scripted or model-based workflows where possible, rather than manual click sequences that are not captured. Reproducibility protects the analysis against challenge, allows it to be refreshed as data updates, and builds institutional knowledge rather than depending on one analyst's memory.

## Common Traps

### Treating All Data as Equally Reliable

Assuming that a downloaded dataset is accurate because it came from an official source, without examining its vintage, resolution, and known errors, leads to analyses built on unreliable foundations. The false signal is a polished map from authoritative-looking data; the harm is findings that do not hold up under scrutiny. Document each dataset's provenance and limitations, and downgrade confidence where data quality is weak.

### Ignoring the Modifiable Areal Unit Problem

Aggregating data into arbitrary units (census tracts, districts) and treating the resulting patterns as meaningful, without recognizing that different aggregations could show different patterns, produces findings that may be artifacts of the unit choice. The false signal is a clear spatial pattern; the harm is conclusions that reverse under a different aggregation. Test robustness to alternative units and justify the chosen unit.

### Calculating in the Wrong Projection

Computing areas, densities, or distances in a geographic coordinate system or an inappropriate projection produces numbers that are quietly wrong, often by significant margins. The false signal is precise-looking outputs; the harm is plan findings (densities, service areas, acreages) that are technically incorrect. Use projections appropriate to the analysis and verify the results.

### Presenting Model Outputs as Fact

Suitability models, forecasts, and suitability scores are estimates laden with assumptions, but they are often presented as definitive findings, lending false authority to contested conclusions. The false signal is a confident, data-rich map; the harm is decision-makers treating estimates as truth and being surprised when reality differs. Quantify or at least describe uncertainty, and present model outputs as scenarios or indicators, not facts.

### Skipping Validation

Publishing analysis without checking it against ground truth, independent totals, or visual inspection allows data and processing errors to reach the plan undetected. The false signal is a complete-looking analysis; the harm is avoidable errors that undermine the plan's credibility when caught later. Build validation into the workflow before findings are presented.

## Self-Check

- Have you documented each dataset's source, vintage, resolution, and known limitations, and assessed whether it is fit for the question being asked?
- Did you choose the analysis unit (parcel, block, tract, buffer) deliberately to match the question, and test whether findings are robust to alternative aggregations?
- Did you use projections appropriate to the analysis (equal-area for areas, equidistant for distances) and verify topology before running operations?
- Did you validate results against ground truth, independent totals, or visual inspection before publishing findings?
- Have you quantified or described the uncertainty in the analysis, rather than presenting estimates as precise facts?
- Is the analysis reproducible from documentation — data sources, processing steps, tools, parameters, and assumptions all recorded?
- Did you avoid treating all downloaded data as equally reliable without examining its provenance and quality?
- Did you avoid the modifiable areal unit problem by justifying your unit choice and testing robustness?
- Did you avoid presenting model outputs, forecasts, or suitability scores as definitive rather than as estimates with uncertainty?
- Could another analyst reproduce your results, and could a reviewer trace each finding to its data and methods with confidence?
