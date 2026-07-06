---
name: harmonic-measurement-and-thd-assessment.md
description: Use when the agent is measuring power system harmonics, computing total harmonic distortion, interpreting FFT spectra, or evaluating compliance with IEEE 519 voltage and current distortion limits at the point of common coupling.
---

# Harmonic Measurement and THD Assessment

Harmonic measurement looks deceptively simple because a modern power quality analyzer produces a spectrum and a THD number with the push of a button. The judgment problem is that the number is only as valid as the measurement point, the windowing, the aggregation interval, and the reference quantity chosen. Agents frequently treat a single snapshot at a load panel as proof of IEEE 519 compliance, when the standard is evaluated at the point of common coupling (PCC) and over defined statistical intervals. The risk is that a facility is declared compliant when it is not, or that expensive mitigation is specified against a distortion source that is actually upstream of the measurement.

## Core Rules

### Measure at the Correct Point Relative to the PCC
IEEE 519 evaluates distortion at the point of common coupling, which is the boundary between the utility and the customer, not at an arbitrary load panel. If the PCC is inaccessible, measure at the service entrance or main transformer secondary and document the approximation, because impedance and distortion both change as you move toward loads. Never extrapolate a load-side reading to the PCC by simple subtraction; the system impedance at the load bus differs from the source impedance at the PCC, so distortion magnitudes do not translate linearly.

### Distinguish Voltage THD from Current THD and Use Both Limits
IEEE 519 imposes separate limits on voltage distortion (TDD of voltage, expressed as a percentage of fundamental) and current distortion (TDD, total demand distortion, expressed as a percentage of maximum demand current, not fundamental current). A common error is to report current THD as a percentage of the instantaneous fundamental and compare it to the TDD limit. Always determine the maximum 15- or 30-minute demand current for the facility and use it as the denominator for current distortion assessment, because TDD is defined against demand, not against the present load.

### Select Instruments and Transducers Matched to the Task
Use a true power quality analyzer with adequate sampling rate (at least 256 samples per cycle, preferably 1024 or more) and current clamps rated for the expected frequency range. Standard clamp meters and low-cost loggers often roll off above the 7th or 11th harmonic and will under-report high-order content. Rogowski coils are suitable for high currents and wide bandwidth, but verify their phase response, because phase error corrupts power and direction calculations even when magnitude reads correctly.

### Apply Correct Windowing and Aggregation
Steady-state harmonics should be measured with a rectangular window synchronized to the fundamental, while fluctuating loads require aggregation over time. IEEE 519 uses statistical evaluation: very short time (3 seconds) and short time (10 minutes) values, with the 95th percentile compared to limits. A single instantaneous reading is not a compliance measurement. Record long enough to capture the worst-case operating profile, including full production load and any load cycling, because a short sample can miss peak distortion.

### Identify the Dominant Harmonic Orders and Their Source
Do not stop at a THD number. Identify which individual orders dominate, because the order reveals the source: 5th and 7th are characteristic of six-pulse rectifiers and VFDs; 3rd and triplens point to single-phase nonlinear loads or unbalanced wye circuits; 2nd and even harmonics suggest half-wave rectification or transformer saturation. Use the phase angle of each harmonic to determine whether it is being injected by the load or imported from the utility, because mitigation differs entirely depending on direction of flow.

### Account for Resonance Before Interpreting Magnitudes
A capacitor bank or long cable capacitance can parallel-resonate with source inductance and amplify a harmonic that the load barely produces. If a specific order (often the 5th or 7th) is disproportionately large, check whether a power factor capacitor is near resonance at that frequency before concluding the load is the problem. Resonance amplification can turn a benign source into a damaging overvoltage, and removing or detuning the capacitor may resolve the distortion without touching the nonlinear load.

### Report Uncertainty and the Measurement Basis Explicitly
State the measurement point, the instrument and clamp model, the sampling window, the aggregation period, the demand current used for TDD, and whether the PCC was directly accessible. A THD figure without this context is uninterpretable. When results are borderline, present the 95th-percentile statistics rather than a single value, and note the load condition at the time of measurement, because compliance can flip between loaded and unloaded states.

## Common Traps

### Treating a Single Snapshot as a Compliance Verdict
The mechanism is that a handheld analyzer gives an instant number, and the false signal is that the number looks stable and definitive. The harm is that a facility is declared compliant or noncompliant based on a few seconds of data that miss the 95th-percentile behavior the standard actually evaluates, leading to either unwarranted confidence or unnecessary mitigation spending.

### Reporting Current THD Against Fundamental Instead of Demand
The mechanism is that instruments default to expressing current distortion as a percentage of the present fundamental current. The false signal is that the percentage looks comparable to the IEEE 519 TDD table. The harm is that at light load the same harmonic current reads as a huge percentage, triggering false noncompliance, while at full load the same distortion may actually exceed limits but read low because the fundamental denominator is large.

### Measuring Downstream of a Filter and Concluding the Source Is Clean
The mechanism is that an analyst measures at a bus that already has a harmonic filter or detuned capacitor installed. The false signal is a clean spectrum at that bus. The harm is concluding the load is benign when the distortion is actually being absorbed or amplified by the upstream mitigation, masking the true source and leading to incorrect sizing of additional filters.

### Ignoring the Neutral Current from Triplen Harmonics
The mechanism is that single-phase nonlinear loads inject 3rd harmonic that sums in the neutral of a wye system rather than canceling. The false signal is that phase current THD looks moderate. The harm is neutral overheating, transformer derating, and fire risk in shared neutrals, all of which are invisible if only phase-to-neutral voltage distortion is reported.

### Confusing Source-Side and Load-Side Distortion
The mechanism is that harmonics can be imported from the utility or generated by neighboring customers and appear at the service entrance. The false signal is high distortion measured at the main. The harm is that the facility installs expensive mitigation for distortion it did not create, while the actual utility-side problem persists and the investment yields no improvement.

## Self-Check

- Is the measurement point documented as the PCC, or, if not, is the approximation and its limitation stated?
- For current distortion, is the maximum demand current identified and used as the TDD denominator rather than the instantaneous fundamental?
- Were 3-second and 10-minute aggregated 95th-percentile values computed, not just a single instantaneous reading?
- Does the report identify the dominant harmonic orders and correlate them to likely sources rather than reporting only a total?
- Was the phase angle or power direction of each harmonic checked to distinguish load-injected from utility-imported distortion?
- If a capacitor bank is present, was parallel resonance at a characteristic harmonic order checked before attributing amplification to the load?
- Is the instrument sampling rate and current transducer bandwidth adequate for the highest harmonic order of interest?
- Are borderline results presented with the load condition and aggregation interval so the verdict can be independently reproduced?
