---
name: soil-and-rock-laboratory-testing-interpretation.md
description: Use when the agent is interpreting soil or rock laboratory test results, selecting strength, consolidation, or index parameters for design, evaluating triaxial, direct shear, oedometer, or CBR data, or resolving scatter and outliers in a geotechnical parameter set. Applies before adopting a design parameter from test data, while distinguishing drained and undrained behaviour, and when reviewing whether the laboratory programme and the reported parameters are reliable enough for the foundation or slope design.
---

# Soil and Rock Laboratory Testing Interpretation

Laboratory testing interpretation is the conversion of measured data on soil and rock samples into the engineering parameters that drive the design, and it is the step where raw numbers become the strength, the stiffness, and the compressibility that the calculations assume. The test is only as good as the sample, the sample is only as good as the recovery, and the parameter is only as good as the interpretation that selects it from the data, with its scatter, its disturbance, and its relevance to the field condition. The harm this skill prevents is a design parameter that is too high because a disturbed sample was treated as undisturbed, a drained strength used where the undrained governs, or a single high test adopted because the average looked conservative, any of which can leave a foundation, a slope, or a retaining wall underdesigned against the real soil behaviour. Because the parameters are the input to every geotechnical calculation, the interpretation must be deliberate, conservative, and honest about the uncertainty the data carry.

## Core Rules

### Evaluate Sample Quality Before Trusting the Test Result

The first question for any laboratory test on a soil sample is whether the sample represents the in-situ soil. Undisturbed samples (Shelby tubes, piston samples) can be disturbed by the sampling, the transport, and the extrusion, and the disturbance reduces the measured strength and stiffness and increases the measured compressibility. Evaluate the sample quality by the recovery, the visual disturbance, the void ratio change, and (for soft clays) the quality criteria in the governing method (such as the Lunne criteria based on the strain to 50 percent of the failure stress). Reject or downgrade samples that are disturbed, and do not use a high-quality parameter from one sample to represent a depth where only disturbed samples were recovered. A test result on a disturbed sample is not a measurement of the in-situ soil; it is a measurement of the remoulded, altered sample, and adopting it as in-situ is an error that propagates into the design.

### Select the Strength Test for the Field Drainage and Loading Condition

Soil strength depends on the drainage during loading, and the test must match the field condition. For the short-term stability of a saturated clay foundation or excavation, the undrained strength governs, and the test is the unconsolidated-undrained (UU) triaxial, the unconfined compression, or (better) the field vane or the consolidated-undrained triaxial with pore pressure measurement. For the long-term stability of a slope, a retaining wall, or a drained foundation, the drained strength (the effective stress friction angle and cohesion) governs, and the test is the consolidated-drained triaxial or the consolidated-undrained triaxial with pore pressure measurement, interpreted for effective stress. For the residual strength on a pre-existing slip surface or a fissured clay, use the ring shear or the direct shear to large displacement, because the peak strength overestimates the available strength on a surface that has already failed. Selecting the wrong test for the drainage condition produces a parameter that is correct for a different condition and wrong for the design.

### Interpret Strength Data With the Stress Path and the Failure Criterion

Triaxial and direct shear tests produce a set of strength points, and the interpretation must fit the failure criterion appropriate to the soil and the stress range. For normally consolidated clay and for sand, the strength is frictional, with zero or near-zero effective cohesion, and fitting a cohesion intercept to data that is really frictional overestimates the strength at low stress and can produce an unsafe design. For overconsolidated clay, the peak strength includes a cohesion that drops to the residual on a slip surface, and the design must distinguish the peak from the residual and use the appropriate value for the condition (first-time slip versus reactivation). For rock, interpret the uniaxial compressive strength and the Hoek-Brown or Mohr-Coulomb parameters from the triaxial data, with the joint and discontinuity strength considered separately from the intact rock. Do not average strength points blindly; interpret them against the soil model and the failure criterion that the soil obeys.

### Interpret Consolidation Data for Compressibility and Stress History

The oedometer (consolidation) test on clay produces the compression curve, from which the preconsolidation pressure, the compression index, the recompression index, and the secondary compression coefficient are derived. Interpret the preconsolidation pressure by the Casagrande or Becker method, recognising that sample disturbance smears the break and underestimates the preconsolidation, and that an underestimated preconsolidation overpredicts the settlement in the overconsolidated range. Use the compression and recompression indices appropriate to the stress range of the design load, and apply Schmertmann's correction for sample disturbance to the laboratory curve. For organic or highly compressible soil, account for the secondary (creep) settlement, which can exceed the primary settlement over the design life. A settlement prediction that uses the uncorrected laboratory curve and ignores the stress history will mispredict the settlement, often substantially, because the preconsolidation and the disturbance govern the response.

### Address Scatter, Outliers, and the Selection of the Design Parameter

Soil and rock data scatter, because the deposit is variable and the tests carry error, and the interpretation must address the scatter rather than average it away. Plot the data by depth and by location to identify trends and outliers, investigate the outliers (a sample disturbance, a different soil layer, a test error) rather than discarding them silently, and select the design parameter by a deliberate method: a conservative percentile (commonly a lower-bound or a characteristic value with the required reliability), a correlation with an in-situ test, or a back-calculation from a known failure or settlement. Do not select the highest test as the design value, and do not select the average where the consequence of the low values is high. The design parameter is a decision about the acceptable risk, and it must be made against the scatter and the consequence, not by a default statistic.

### Cross-Check Laboratory Parameters Against In-Situ Tests and Correlations

Laboratory parameters carry the uncertainty of sampling and the small scale of the test, and they should be cross-checked against in-situ tests and empirical correlations. Cross-check the undrained strength of clay against the field vane, the CPT, or the SPT, corrected for the plasticity and the stress level. Cross-check the friction angle of sand against the CPT or SPT correlations. Cross-check the modulus against the pressuremeter or the dilatometer. Where the laboratory and the in-situ values disagree, investigate the cause (sample disturbance, scale effect, different stress condition) and select the parameter that best represents the field behaviour. A design parameter based on laboratory data alone, without an in-situ cross-check, carries the full risk that the sample does not represent the field.

### Carry the Uncertainty Into the Design

The interpreted parameter is not a single true value; it is an estimate with a range, and the design must carry that range. Use conservative parameters where the consequence is high, perform sensitivity analysis on the parameters that carry the widest uncertainty, and require field verification (load testing, instrumentation, observation) where the parameter is critical and the data are sparse. The geotechnical report should state the parameter, its source, its uncertainty, and the recommended design value, so that the design and the construction can manage the residual risk. A report that gives a single parameter without its uncertainty has concealed the range that the design must address.

## Common Traps

### The Disturbed Sample Treated As Undisturbed

A Shelby tube sample of soft clay is recovered with disturbance, and the laboratory triaxial test on it gives a strength that is lower than the in-situ strength, but the disturbance is not recognised and the low value is adopted as the design parameter. The trap is that the test result is a real measurement of the disturbed sample, while it underestimates the in-situ strength, and the design is unnecessarily conservative, or, conversely, a disturbed sample gives a falsely high strength in a different test and the design is unsafe. The false signal is the reported laboratory value; the harm is a design parameter that does not represent the field soil, because the sample quality was not evaluated.

### The Undrained Strength Used For A Drained Condition

A short-term undrained strength is used for a long-term slope or retaining wall design, because the UU test was run and the value was available. The trap is that the undrained strength is correct for the short-term loading, while the long-term condition is governed by the drained effective stress strength, which can be lower (especially for overconsolidated clay on a slip surface), and the slope or wall fails over time as the drained condition develops. The false signal is the available, tested strength; the harm is a long-term failure driven by a strength that the test condition did not represent.

### The Cohesion Intercept Fit To Frictional Data

A linear Mohr-Coulomb fit to triaxial data on sand or normally consolidated clay produces a cohesion intercept, and the design uses the cohesion at low normal stress. The trap is that the fit is mathematically correct, while the soil has no true cohesion, and the apparent cohesion overestimates the strength at low stress, producing an unsafe design for shallow slip surfaces or low-load conditions. The false signal is the fitted strength envelope; the harm is a design that relies on a cohesion that the soil does not actually provide.

### The Preconsolidation Underestimated By Disturbance

An oedometer test on a disturbed sample of overconsolidated clay shows a smeared break, and the interpreted preconsolidation pressure is below the true value, and the settlement is predicted using the lower preconsolidation. The trap is that the test gives a real curve, while the disturbance has underestimated the preconsolidation, and the predicted settlement in the overconsolidated range is far higher than the field will experience, leading to an overdesigned, expensive foundation. The false signal is the laboratory curve; the harm is a settlement prediction that misrepresents the field response, because the disturbance was not corrected.

### The Highest Test Adopted As The Design Value

A set of strength tests scatters, and the designer adopts the highest value, on the reasoning that the lower tests are "conservative outliers" or disturbed samples. The trap is that the high test is a real measurement of a stronger zone, while the deposit also contains the weaker zones that the lower tests represent, and the design must account for the weaker zones, not the strongest. The false signal is the high, defensible-looking value; the harm is a design based on the strongest sample, with no allowance for the weaker zones that the deposit also contains and that the foundation or slope may encounter.

### The Laboratory Parameter Without The In-Situ Cross-Check

A design parameter is adopted from laboratory data alone, without cross-check against the CPT, the vane, or the SPT, because the laboratory programme was run and the values were available. The trap is that the laboratory value carries the full uncertainty of the sampling and the small test scale, while the in-situ test, which represents the field at scale, may give a different value, and the disagreement is never discovered. The false signal is the completed laboratory programme; the harm is a design parameter that the field data would have corrected, used because the cross-check was never performed.

## Self-Check

- Has the sample quality been evaluated (recovery, disturbance, void ratio change, quality criteria) before the test results are adopted as in-situ parameters?
- Is the strength test selected for the field drainage and loading condition (undrained for short-term saturated clay, drained for long-term, residual for existing slip surfaces)?
- Are the strength data interpreted against the appropriate failure criterion, with cohesion intercepts treated cautiously on frictional soils and peak versus residual distinguished on overconsolidated clay?
- Are the consolidation parameters interpreted with the preconsolidation, the compression indices, the disturbance correction, and the secondary compression appropriate to the design stress range?
- Has the scatter and the outliers been plotted, investigated, and the design parameter selected by a deliberate method (characteristic value, correlation, back-calculation) against the consequence?
- Are the laboratory parameters cross-checked against in-situ tests (CPT, vane, SPT, pressuremeter) and correlations, with disagreements investigated?
- Does the geotechnical report state each parameter, its source, its uncertainty, and the recommended design value, so the design and construction can manage the residual risk?
- Are conservative parameters, sensitivity analysis, or field verification specified where the parameter is critical and the data are sparse?
