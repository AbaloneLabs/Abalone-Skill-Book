---
name: geophysical-and-in-situ-site-characterization.md
description: Use when the agent is selecting or interpreting geophysical methods (seismic refraction, MASW, GPR, electrical resistivity) or in-situ tests (CPT, SPT, pressuremeter, dilatometer, vane shear) for site characterisation, profiling bedrock, mapping anomalies, or deriving engineering parameters. Applies before relying on geophysical or in-situ data for foundation or ground-improvement design, while resolving non-unique inversions, and when integrating indirect measurements with borings and laboratory data.
---

# Geophysical and In-Situ Site Characterization

Geophysical and in-situ site characterisation is the engineering of learning about the ground without (or alongside) extracting samples, by measuring a physical property at the surface or in a borehole and converting that measurement into the soil or rock profile and the engineering parameters. These methods are powerful because they cover ground between borings, they sample the soil in its in-situ state without the disturbance of sampling, and they detect features that borings can miss (a void, a soft layer, a rock interface). They are also indirect: every method measures a physical property (velocity, resistance, penetration resistance) and converts it to an engineering property by a correlation or an inversion that is non-unique, and the conversion carries uncertainty that the user must understand. The harm this skill prevents is a design based on a geophysical inversion that is mathematically plausible but geologically wrong, a CPT correlation applied outside its calibrated range, or an anomaly dismissed as noise that was actually the void or the weak layer the project needed to find. Because these methods are indirect, their results must be integrated with, not substituted for, the direct investigation.

## Core Rules

### Select the Method for the Target and the Site Conditions

Each geophysical and in-situ method has a target it detects well and conditions that defeat it. Seismic refraction and MASW profile the shear-wave velocity and the depth to rock or hard layer, and they underpin the seismic site classification and the liquefaction screening; they struggle where a low-velocity layer lies below a high-velocity layer (the refraction hidden-layer problem). Ground-penetrating radar profiles shallow layering, voids, and buried structures in non-conductive soil, and it is defeated by conductive clay or saline groundwater. Electrical resistivity maps the water table, the contamination plume, and the layer contrasts, and it requires a resistivity contrast and electrode contact. The CPT profiles the soil type, the density, and the strength continuously, and it cannot penetrate gravel, cobbles, or hard rock. The SPT gives a sample and a blow count at intervals, and it is variable in method and energy. Select the method that detects the target in the actual site condition, and do not use a method whose physical basis does not suit the target, because the method will return a result that is not the target.

### Resolve the Non-Uniqueness of Geophysical Inversions

Geophysical methods invert a measured field (travel time, resistivity, radar return) into a subsurface model, and the inversion is non-unique: multiple subsurface models can produce the same measured field, and the inversion selects one by its assumptions (smoothness, layer boundaries, starting model). Recognise that the inversion is an interpretation, not a measurement, and that the reported layer depths and velocities carry an uncertainty that depends on the data quality, the geometry, and the assumptions. Constrain the inversion with the borings (use the boring logs to fix the layer depths and the velocities at the boring locations), and confirm that the inversion is consistent with the geology and the boring data across the site. An inversion that is run without boring control and presented as the profile is a model with no check, and the design that uses it inherits the model's hidden uncertainty.

### Calibrate In-Situ Correlations for the Soil and the Region

In-situ tests (CPT, SPT, vane, pressuremeter) measure a physical response, and the conversion to an engineering parameter (friction angle, undrained strength, modulus, liquefaction resistance) is by a correlation that was developed for a specific soil and region. Confirm that the correlation is appropriate to the site soil: the CPT correlation for undrained strength depends on the plasticity and the stress level; the SPT correlation for friction angle depends on the soil type and the overburden; the liquefaction correlation depends on the fines content and the magnitude. Apply the energy and overburden corrections to the SPT before using the correlations, because the raw blow count is method- and rig-dependent. Do not apply a correlation outside its calibrated range (a sand correlation to a clay, a low-stress correlation to a high-stress depth), because the correlation will return a number that is not the parameter.

### Integrate Indirect and Direct Methods

The strength of geophysical and in-situ methods is realised when they are integrated with the direct investigation, not when they replace it. Use the geophysics to extend the boring profile across the site and to target the borings at the anomalies; use the CPT to profile continuously between the sampled borings; use the in-situ tests to provide the parameters that the laboratory cannot (the in-situ modulus, the liquefaction resistance, the undrained strength at the field stress). Confirm that the indirect and the direct data agree at the boring locations, and investigate any disagreement, because the disagreement is either a method limitation or a real feature that one method detected and the other missed. A site characterisation that uses only indirect methods has no ground truth, and one that uses only borings has no continuity between them.

### Characterise the Seismic Site Class and Liquefaction Potential

For sites in seismic regions, the geophysical and in-situ methods are central to the seismic site characterisation. The shear-wave velocity profile (by MASW, refraction, or downhole) defines the seismic site class (per the governing code, such as the NEHRP or IBC classes by Vs30), which governs the design spectrum and the seismic loads. The CPT or SPT profile, with the groundwater and the soil type, supports the liquefaction potential assessment (by the simplified methods, with the magnitude scaling and the overburden correction), which identifies the zones that will liquefy and the settlement and lateral spreading that will result. Confirm that the velocity profile and the penetration profile are consistent, and that the liquefaction-prone zones are mapped across the site, because the seismic site class and the liquefaction potential are among the highest-consequence outputs of the site characterisation.

### Detect and Characterise Anomalies and Hazards

Geophysical and in-situ methods are the primary tools for detecting the anomalies and hazards that borings alone are unlikely to find: karst voids and sinkholes (by GPR, resistivity, or microgravity), buried utilities and structures (by GPR or electromagnetic induction), abandoned mines and cavities (by microgravity or resistivity), and lateral or vertical contrasts that indicate faulting or buried channels. Confirm that the anomaly is investigated by a targeted boring or test pit before it is incorporated into the design, because the geophysical anomaly is an indirect signature that requires ground truth. Do not dismiss an anomaly as noise without a reason, because the dismissed noise may be the void or the weak layer that the project most needs to find.

### Document the Methods, the Limitations, and the Integration

The site characterisation report must document the methods used, their physical basis and limitations, the data and the inversions or correlations, the integration with the borings and the laboratory data, and the resulting profile and parameters, with the uncertainty carried through. State where the indirect data extend or contradict the direct data, and how the contradiction was resolved. A report that presents the geophysical profile or the CPT correlations without the method, the inversion, the correlation, and the integration conceals the uncertainty that the design must manage, and it presents an interpretation as a measurement.

## Common Traps

### The Inversion Without Boring Control

A seismic refraction or resistivity inversion is run across the site and the resulting profile is used for the foundation design, without boring control at the inversion locations. The trap is that the inversion is mathematically plausible and visually smooth, while it is non-unique and the selected model is one of several that fit the data, and the real interface may be deeper, shallower, or differently shaped than the inversion shows. The false signal is the smooth, defensible-looking profile; the harm is a design based on a model that the borings would have corrected, because the inversion was treated as a measurement rather than an interpretation.

### The Correlation Outside Its Calibrated Range

A CPT correlation for undrained strength, developed for low-plasticity clay at low stress, is applied to a high-plasticity clay at depth, and the resulting strength is adopted for the design. The trap is that the correlation returns a number, while the soil and the stress are outside the calibration, and the number does not represent the parameter. The false signal is the computed, parameterised output; the harm is a design parameter that the correlation cannot support, because the method was applied outside its validated range.

### The SPT Without Energy Correction

The raw SPT blow counts are used in the friction angle and liquefaction correlations, without the energy correction for the specific hammer and the overburden correction. The trap is that the blow counts are real measurements, while they are rig- and method-dependent, and the uncorrected values can be 30 to 50 percent off the energy-corrected value, and the derived parameters are correspondingly wrong. The false signal is the reported blow count; the harm is a friction angle or a liquefaction resistance that is based on an uncorrected measurement, with the error propagating into the design.

### The Hidden Layer In Refraction

A seismic refraction survey is run on a site with a low-velocity layer below a high-velocity layer, and the inversion does not detect the low-velocity layer, because refraction requires velocity to increase with depth. The trap is that the refraction profile shows a competent layer at depth, while a softer, lower-velocity layer lies below it undetected, and the foundation that bears on the apparent competent layer actually sits above a hidden weak layer. The false signal is the verified rock or hard layer; the harm is a foundation on a layer that the method could not see, because the physics of refraction defeated the survey.

### The Anomaly Dismissed As Noise

A geophysical survey identifies an anomaly (a resistivity low, a GPR reflection, a microgravity residual), and the interpreter dismisses it as noise or as a cultural interference, without investigation. The trap is that the dismissed anomaly may be a void, a soft pocket, a buried structure, or a contamination plume, and the design proceeds as if the ground were uniform. The false signal is the clean, anomaly-free profile; the harm is a hazard that the survey detected and the interpretation discarded, surfacing during construction or in service with consequences the survey was meant to prevent.

### The Indirect Method Substituted For The Direct

A site is characterised by CPT and geophysics alone, with no sampled borings and no laboratory testing, on the reasoning that the indirect methods are faster and cheaper. The trap is that the indirect methods provide the profile and the correlated parameters, while they provide no sample for visual classification, no undisturbed sample for consolidation, and no check on the correlations, and the design carries the full uncertainty of the correlations with no ground truth. The false signal is the complete-looking indirect profile; the harm is a design with no direct verification of the ground it depends on, because the indirect methods were substituted for, rather than integrated with, the direct investigation.

## Self-Check

- Is the geophysical or in-situ method selected for the target and the actual site condition, with its physical basis suited to detecting the target?
- Are geophysical inversions constrained by boring control, and is the resulting profile consistent with the geology and the boring data across the site?
- Are in-situ correlations appropriate to the site soil and stress range, with the SPT energy and overburden corrections applied before the correlations are used?
- Are the indirect methods integrated with the direct investigation, with agreement confirmed at the boring locations and disagreements investigated?
- For seismic regions, are the shear-wave velocity profile (site class) and the liquefaction potential (by CPT or SPT) characterised and mapped across the site?
- Are anomalies investigated by targeted borings or test pits before incorporation into the design, and are they not dismissed as noise without reason?
- Does the report document the methods, the inversions and correlations, the integration, the resulting profile and parameters, and the carried uncertainty?
- Is the indirect data presented as interpretation (with its limitations and uncertainty) rather than as measurement, and is the design carried out against that uncertainty?
