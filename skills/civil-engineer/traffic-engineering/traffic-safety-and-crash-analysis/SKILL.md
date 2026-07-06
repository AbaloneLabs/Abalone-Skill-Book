---
name: traffic-safety-and-crash-analysis.md
description: Use when the agent is analyzing traffic safety and crash data, applying the Highway Safety Manual, computing crash modification factors and safety performance functions, conducting before-after evaluations, performing road safety audits, identifying high-crash and systemic risk locations, or evaluating sight distance and speed in relation to crash risk.
---

# Traffic Safety and Crash Analysis

Traffic safety analysis is the systematic study of where, why, and how crashes occur, and the use of that evidence to prioritize treatments and to predict the safety effect of design and operational changes. The discipline has shifted from descriptive crash counting toward the predictive, quantitative methods of the Highway Safety Manual (HSM), which uses safety performance functions (SPFs), crash modification factors (CMFs), and empirical Bayes adjustment to estimate expected crashes and the effect of countermeasures. The harm this skill prevents is a safety program driven by anecdote, by the loudest complaint, or by the naive use of raw crash counts that confound exposure with risk, leading to treatments placed where they are politically visible rather than where they prevent the most harm, and to claims of safety improvement that are artifacts of regression to the mean. The analyst's job is to distinguish true risk from random variation, to account for exposure (volume and segment length), to select countermeasures with credible evidence of effectiveness, and to evaluate treatments with methods that isolate the treatment effect from confounders and trend.

## Core Rules

### Account for Exposure and Use Predictive Methods, Not Just Raw Crash Counts

Raw crash counts confound risk with exposure: a busy intersection will have more crashes than a quiet one even if each is equally safe per vehicle, so ranking locations by crash count without normalizing for volume and length identifies busy places, not necessarily dangerous ones. Use the HSM predictive method, which applies a safety performance function calibrated to the facility type to estimate the expected crashes for the exposure, then adjusts by crash modification factors for the site's specific geometric and operational features, and combines the prediction with the observed crash history by the empirical Bayes method to produce a more accurate estimate of expected crashes than either prediction or observation alone. This expected-crash estimate, not the raw count, is the basis for identifying high-crash locations and for comparing sites, because it corrects for both exposure and regression to the mean. Calibrate the SPFs to local conditions where local calibration factors are available, because national default SPFs can mispredict a region's crash frequency substantially.

### Evaluate Countermeasures With Credible CMFs and Before-After Methods

A crash modification factor (CMF) quantifies the expected change in crashes from a treatment, and it should come from a credible source (the HSM, the CMF Clearinghouse) with a star rating that reflects the quality of the underlying study, because a CMF from a single weak before-after study is not reliable evidence. When selecting a countermeasure, confirm that the CMF applies to the facility type, crash type, and severity for which it was developed, because a CMF for reducing run-off-road crashes on rural two-lane roads does not necessarily apply to urban intersection angle crashes. For evaluating an implemented treatment, use the empirical Bayes before-after method, which compares the observed after-period crashes to the expected crashes that would have occurred without the treatment (estimated from a reference group), thereby isolating the treatment effect from volume change, trend, and regression to the mean. A naive before-after comparison that simply subtracts post-treatment crashes from pre-treatment crashes will overstate the treatment's effect, because crash counts at high-crash sites tend to regress toward the mean regardless of treatment.

### Combine Site-Specific and Systemic Approaches to Identify Risk

The traditional site-specific (hotspot) approach identifies the highest-crash individual locations and treats them, which is effective for concentrating crashes but misses the lower-density sites that collectively account for most severe crashes, especially fatal and serious-injury run-off-road and lane-departure crashes on rural roads. The systemic approach identifies the risk factors (curve radius, shoulder width, lane width, edge conditions, intersection type, speed) associated with severe crashes and treats all sites with those risk factors across the network, regardless of whether each site has a high individual crash count. Use both: site-specific analysis for urban intersections and corridors where crashes concentrate, and systemic analysis for rural and lane-departure risk where severe crashes are dispersed. A safety program that uses only the hotspot approach will under-serve the rural network where a large share of fatalities occur.

### Examine Sight Distance, Speed, and Design Consistency as Crash Contributors

Beyond statistical prediction, the analyst should examine the physical and operational conditions that produce crash risk, because the crash data shows where crashes happen but the engineering analysis shows why. Verify stopping and intersection sight distance against the AASHTO Green Book criteria for the operating speed, because inadequate sight distance is a recurring contributor to intersection and curve crashes. Examine speed, because speed governs both crash probability and crash severity, and a road where the 85th percentile speed exceeds the design speed, or where operating speeds vary sharply between successive elements, is a crash predictor. Check design consistency, because a sharp curve after a long tangent, a hidden intersection over a crest, or an unexpected lane drop produces crashes driven by driver expectation, not by any single deficient dimension. The crash analysis and the engineering condition review must inform each other.

## Common Traps

### The Hotspot List Built on Raw Crash Counts

Locations are ranked by raw crash count without normalizing for traffic volume or segment length, and the highest-count sites are treated. The false signal is the defensible-looking ranked list; the harm is that the list identifies the busiest locations, not necessarily the riskiest per vehicle, and resources are spent where exposure is high rather than where the per-vehicle crash risk and severity are greatest.

### The Naive Before-After Study That Ignores Regression to the Mean

A site is treated because it had an unusually high crash count, and after treatment the count drops, so the treatment is declared a success by simple subtraction. The false signal is the dramatic crash reduction; the harm is that crash counts at high-crash sites tend to fall back toward the mean regardless of treatment, so the naive method credits the treatment with a reduction that was partly or wholly regression to the mean, overstating effectiveness and misdirecting future investment.

### The CMF Applied Outside Its Valid Context

A CMF from the clearinghouse is applied to a facility type, crash type, or severity for which it was not developed, because it is the only available number. The false signal is a quantitative safety prediction that looks rigorous; the harm is that the CMF may not transfer, the predicted effect is unreliable, and a countermeasure may be selected that does not produce the claimed benefit in the actual context.

### The Hotspot-Only Program That Misses Dispersed Severe Crashes

The safety program treats only the highest-crash individual sites and ignores the rural and lane-departure crashes that are dispersed across the network. The false signal is the visible, concentrated treatment of problem intersections; the harm is that the program misses the systemic risk factors that collectively produce most fatal and serious-injury crashes, so severe-injury reductions fall short while the budget is consumed at a few urban sites.

## Self-Check

- Are high-crash locations identified using expected crashes from the HSM predictive method with empirical Bayes adjustment, rather than raw crash counts that confound risk with exposure?
- Are safety performance functions calibrated to local conditions where local calibration factors are available, and are exposure (volume, segment length) and severity accounted for in the comparison of sites?
- Are countermeasures selected using CMFs from credible sources (HSM, CMF Clearinghouse) with documented star ratings, and is each CMF confirmed to apply to the facility type, crash type, and severity in question?
- For evaluating implemented treatments, is the empirical Bayes before-after method used to isolate the treatment effect from volume change, trend, and regression to the mean, rather than naive before-after subtraction?
- Does the safety program combine site-specific hotspot analysis (for concentrated urban crashes) with systemic analysis of risk factors (for dispersed rural and lane-departure severe crashes)?
- Are sight distance, operating speed, and design consistency examined as engineering contributors to crash risk, connecting the crash data to the physical and operational conditions that produce it?
