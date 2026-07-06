---
name: power-factor-correction-capacitor-banks.md
description: Use when the agent is designing power factor correction with capacitor banks, sizing reactive steps, selecting fixed versus automatic banks, specifying detuning reactors, and evaluating harmonic resonance and switching transient risks.
---

# Power Factor Correction Capacitor Banks

Power factor correction seems like a straightforward kvar calculation, but capacitor banks interact dynamically with system impedance and with the harmonics produced by nonlinear loads. The judgment problem is that an incorrectly sized or un-detuned bank can resonate with the source inductance, amplify harmonic voltages to damaging levels, draw excessive capacitor current, trip breakers, and destroy the capacitors themselves. Agents tend to focus on the target power factor number and ignore the resonance condition, the switching transient, and the difference between real power factor (including harmonics) and displacement power factor. The skill exists to force the agent to treat the bank as a system component, not a bolt-on accessory.

## Core Rules

### Base kvar on Measured Load and Real Power Factor, Not Nameplate
Compute required correction from recorded kilowatt demand and the measured power factor at the main, not from motor nameplate values, because actual loading rarely matches nameplate and power factor varies with load. Use the standard relation Q = P(tan(theta_1) - tan(theta_2)) where the angles come from the present and target power factors. Over-correction is as harmful as under-correction because it drives leading power factor, raises voltage, and can cause utility penalties and capacitor switching difficulties; size for the 95th-percentile demand, not the peak instantaneous.

### Decide Fixed Versus Automatic Based on Load Variability
Use a fixed bank only when the reactive load is constant and large, such as a continuously running motor. Use an automatic (stepped) bank with a controller whenever load varies, because applying fixed capacitance to a lightly loaded system produces leading power factor and overvoltage. Step sizing matters: steps should be small enough to track the load without overshooting and large enough to avoid excessive switching operations. A common ratio is that the smallest step is no more than half the largest, and total steps should resolve the load to within the target band.

### Always Check Harmonic Resonance Before Finalizing the Rating
The single most important calculation is the parallel resonance frequency between the bank and the source impedance: f_r = f_1 * sqrt(S_sc / Q_c), where S_sc is the short-circuit capacity at the connection point and Q_c is the capacitor rating. If f_r lands near a characteristic harmonic of the load (typically the 5th, 7th, 11th, or 13th for six-pulse drives), the bank will amplify that harmonic catastrophically. If resonance is a risk, specify a detuning reactor that shifts the bank's resonance below the lowest significant harmonic, usually to the 3rd or 4th (around 189 Hz or 252 Hz on a 60 Hz system).

### Specify Capacitor Voltage and Current Ratings for the Harmonic Environment
Standard capacitors are rated for sinusoidal service and a nominal voltage. In a harmonic-rich system the capacitor sees the sum of fundamental and harmonic voltages, and its current is proportional to frequency, so high-order harmonics disproportionately heat the dielectric. Specify capacitors rated at least 110 percent and often 135 percent or more of nominal voltage, and verify the rms current including harmonics against the capacitor's rated current. Underrated capacitors fail by internal heating, bulging, and rupture, which is a fire and arc-flash hazard.

### Evaluate Switching Transients and Inrush
Energizing a capacitor bank produces an inrush current limited only by system inductance and any series reactor, and the transient can be many times rated current at kilohertz frequency. This transient stresses the capacitor switching device, can nuisance-trip upstream breakers, and can couple into nearby loads as a voltage notch. For automatic banks, use contactors with damping resistors or early-make late-break contacts to limit inrush on back-to-back step switching. For large fixed banks, consider a pre-insertion resistor or synchronized closing breaker to limit the transient.

### Coordinate the Capacitor Protection With the System
Capacitor branch protection must protect against capacitor failure without tripping on normal harmonic current. Use fuses or breakers sized to the capacitor's rated current times a factor (typically 1.25 to 1.35) that accounts for capacitance tolerance, harmonics, and overvoltage, and ensure the protective device can interrupt the available fault current. Harmonic filters add a series reactor that changes fault current and protection coordination, so re-check the time-current curve after adding detuning. Unbalanced protection is required on larger banks to detect failed capacitor elements before the bank ruptures.

### Place the Bank to Minimize Losses and Avoid Leading Operation
Locate correction as close to the load as practical to reduce line losses and free transformer capacity, but balance this against maintainability and the risk of leading power factor when that load is off. When correcting individual motors, ensure the capacitor does not cause self-excitation when the motor disconnects, which can produce damaging overvoltage; the rule of thumb is to limit motor-connected correction to roughly 90 percent of the magnetizing current. For system-level correction, place the bank on the bus that sees the bulk of the reactive load.

## Common Traps

### Applying a Fixed Bank to a Variable Load
The mechanism is that a fixed bank sized for full load remains connected when load drops. The false signal is that the power factor controller or meter shows improvement at full load. The harm is leading power factor and overvoltage at light load, which can damage equipment, trip drives on overvoltage, and trigger utility leading-power-factor penalties that exceed the original lagging penalty.

### Ignoring Resonance Because the Load Looks Linear
The mechanism is that a designer sizes the bank based on a load survey that predates the addition of VFDs or electronic power supplies. The false signal is a clean-looking power factor problem with no obvious harmonic source. The harm is that when the nonlinear loads run, the bank resonates with the source at a characteristic harmonic, multiplying voltage distortion, overheating the capacitors, and tripping the bank offline repeatedly.

### Using Standard Capacitors in a Harmonic Environment
The mechanism is that procurement selects the lowest-cost capacitor with a standard voltage and current rating. The false signal is that the rating exceeds the nominal system voltage, so it appears adequate. The harm is that harmonic current, which scales with frequency, overheats the dielectric well before the voltage limit is reached, causing premature failure, case rupture, and potential fire within months of installation.

### Switching Steps Without Inrush Limiting
The mechanism is that an automatic bank energizes a step while other steps are already energized, producing back-to-back inrush limited only by the bus inductance. The false signal is that the controller logic appears correct and the bank tracks load. The harm is welded contactors, breaker nuisance trips, and voltage transients that reset sensitive loads, turning the correction system into a source of power quality problems.

### Confusing Displacement and True Power Factor
The mechanism is that the target power factor is specified without distinguishing displacement from true power factor, and harmonics depress true power factor even when displacement is corrected. The false signal is that the capacitor brings displacement power factor to unity. The harm is that the utility, billing on true power factor, continues to apply penalties, and the added capacitance may worsen harmonic resonance, making the true power factor problem worse rather than better.

## Self-Check

- Is the required kvar based on measured kW demand and present power factor, with a stated target and avoidance of over-correction?
- Has the parallel resonance frequency f_r been calculated and checked against the characteristic harmonics of the load?
- If harmonics are present, is a detuning reactor specified, and is its tuning frequency documented (commonly near the 4th or 3rd)?
- Are the capacitor voltage and current ratings specified to handle harmonic content, typically 110 percent or higher voltage and rms current margin?
- For automatic banks, are step sizes chosen to track load without overshoot, and is the smallest step no more than half the largest?
- Is inrush limiting specified for step switching (damping contactors, pre-insertion resistors, or synchronized closing)?
- Is capacitor branch protection sized with the harmonic and tolerance factor and coordinated with upstream devices?
- Is the bank location chosen to minimize losses without creating leading power factor or motor self-excitation risk?
