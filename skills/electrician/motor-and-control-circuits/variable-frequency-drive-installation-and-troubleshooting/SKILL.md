---
name: variable-frequency-drive-installation-and-troubleshooting.md
description: Use when the agent is installing, commissioning, parameterizing, or troubleshooting a variable-frequency drive, selecting carrier frequency, managing motor lead length, addressing common-mode voltage and bearing current issues, or specifying input/output reactors and filters.
---

# Variable-Frequency Drive Installation and Troubleshooting

A variable-frequency drive (VFD) is the most capable and the most demanding motor controller in common use. It delivers precise speed control, energy savings on variable-torque loads, and soft starting with full torque — but it does so by synthesizing the motor voltage with high-frequency pulse-width modulation, and that synthesis creates a family of problems that do not exist with any other starting method. The judgment problem is that a VFD is not a drop-in replacement for a starter; it changes the electrical environment of the motor, the cable, the bearings, and the surrounding equipment, and an installation that ignores those changes will destroy motors through bearing failure, corrupt signals through electromagnetic interference, and trip on nuisance faults that resist diagnosis. This skill covers the decisions that determine whether a VFD installation runs reliably or becomes a chronic source of failure.

## Core Rules

### Match the VFD to the Motor's Constant-Torque or Variable-Torque Load

VFDs are rated for either variable-torque duty (centrifugal pumps and fans, where torque rises with the square of speed) or constant-torque duty (conveyors, positive-displacement pumps, hoists, where torque is roughly constant across the speed range). Constant-torque loads demand more current at low speed and require a drive sized for continuous heavy-duty operation, typically with a 150% overload capacity for one minute. The trap is selecting a drive rated only for variable-torque and applying it to a conveyor or hoist, where the drive runs at its current limit continuously, overheats, and trips on thermal protection. The defense is to identify the load type from the speed-torque characteristic, select a drive with the appropriate duty rating, and upsize if the load runs continuously at low speed where motor cooling is reduced.

### Manage Motor Lead Length to Prevent dv/dt and Reflected-Wave Damage

The PWM output of a VFD consists of fast-rising voltage pulses (high dv/dt) that travel down the motor leads and reflect off the motor impedance. On long leads, the reflected wave adds to the incident pulse and can produce peak voltages at the motor terminals of twice the DC bus voltage, with rapid repetition, which punctures motor winding insulation. The critical lead length depends on the voltage and the drive's rise time: 480V drives with fast IGBT outputs may see damaging reflections at 50 to 100 feet, while 575V systems are worse. The trap is installing the drive in a central motor control center and running long leads to a distant motor without any mitigation, then replacing the motor repeatedly for winding failures. The defense is to keep leads short where possible, and where long leads are unavoidable, install a load reactor or a dv/dt filter at the drive output, or a sine-wave filter or terminating network at the motor end.

### Address Common-Mode Voltage and Bearing Currents Proactively

The PWM output is not perfectly balanced between the three phases, and the imbalance produces a common-mode voltage that capacitively couples from the windings to the motor frame, driving current through the bearings. This bearing current causes electrical discharge machining (EDM) of the bearing races — fluting and frosting that destroy the bearing in months rather than years. The mechanism is well understood but invisible: the motor runs fine until the bearing fails, and the failure is attributed to the bearing manufacturer rather than the drive. The trap is installing a VFD with no mitigation and assuming the motor's standard bearings will survive. The defense is a combination of measures: use a shielded VFD cable with three symmetrical conductors plus a continuous ground (not random-lay cable), bond the motor frame to the drive with a low-impedance high-frequency path, install a shaft grounding ring on larger motors, and consider insulated bearings or an output common-mode filter for critical installations.

### Set the Carrier Frequency Deliberately, Not at the Default

The carrier frequency is the rate at which the VFD synthesizes the output waveform; typical ranges are 2 to 16 kHz. A higher carrier frequency produces a smoother output, quieter motor operation, and lower motor heating from harmonics — but it increases drive heating (switching losses), increases the drive's conducted and radiated EMI, and worsens common-mode voltage and bearing current problems. The trap is leaving the carrier at a high default or raising it to quiet a noisy motor without considering the trade-offs, then experiencing drive over-temperature trips or EMI problems in nearby instrumentation. The defense is to use the lowest carrier frequency the application tolerates, raise it only when motor noise or torque ripple is a documented problem, and derate the drive output if the carrier is set above the manufacturer's standard value.

### Install Input Power Conditioning Appropriate to the Supply and the Drive

VFDs draw pulsed current from the line, producing harmonic distortion that can trip the drive on undervoltage, overheat upstream transformers, and interfere with other loads. On a stiff supply this is tolerable; on a weak supply or a generator it causes problems. Conversely, line voltage transients (capacitor switching, lightning) can damage the drive's input rectifier. The trap is assuming the drive's built-in DC link reactor handles everything, then experiencing nuisance trips or rectifier failures on a real installation. The defense is to assess the source impedance and the harmonic environment, and to install an input line reactor (typically 3% or 5% impedance) or a passive harmonic filter where the supply is stiff and distortion is a concern, and surge protection where transients are expected.

### Program the Drive Parameters to the Actual Motor and Load, Never Leave Defaults

A VFD must be told the motor's nameplate data (voltage, current, frequency, speed, power), and the motor must be auto-tuned (rotating or static) so the drive's model of the motor matches reality. The trap is powering up a new drive, entering the motor data approximately, and running without an auto-tune, so the drive's current regulation and torque production are based on a generic motor model. The result is poor low-speed torque, nuisance overcurrent trips, or inefficient operation. The defense is to enter the exact nameplate data, perform the auto-tune procedure with the motor uncoupled (rotating) or coupled (static) per the manufacturer, and verify the tune by checking stable current and torque at several speeds.

## Common Traps

### Ignoring Reflected-Wave Voltage on Long Motor Leads

The electrician installs the VFD in the main electrical room and runs 200 feet of THHN in conduit to a rooftop fan motor. The mechanism of the failure is that the fast-rising PWM pulses travel at roughly half the speed of light down the leads, reflect off the high impedance of the motor winding, and constructively add to produce peak voltages at the motor terminals of 1500 to 2000 volts on a 480V system, repeated thousands of times per second. The motor's insulation, rated for 1000V peak on a standard inverter-duty basis, breaks down, and the motor fails in weeks to months. The false signal is that the motor "is a lemon" or "wasn't inverter-duty rated," when in fact even an inverter-duty motor needs lead-length mitigation at 200 feet. The harm is repeated motor replacement and downtime, with the root cause never addressed. The defense is to calculate the critical lead length for the drive and voltage, and to install a reactor or filter whenever the lead length approaches the limit.

### Attributing Bearing Failure to the Motor Instead of the Drive

A VFD-driven motor runs for eight months and the bearing fails with visible fluting (washboard pattern) on the races. The mechanism is electrical discharge machining: common-mode voltage from the PWM drive capacitively couples to the rotor, drives current through the bearings, and the tiny arcs erode the race material each time the ball passes. The false signal is that the bearing failed mechanically, suggesting lubrication or load problems, and the motor is rebuilt with the same type of bearing. The harm is that the rebuilt motor fails again in the same timeframe, because the root cause — bearing current from the drive — was never addressed. The defense is to recognize fluting as an electrical failure signature, install shaft grounding or insulated bearings, and use a proper VFD cable with a continuous symmetric ground.

### Raising the Carrier Frequency to Quiet the Motor and Tripping the Drive

The motor on a HVAC fan produces an audible whine that the building owner complains about, so the electrician raises the VFD carrier frequency from 4 kHz to 12 kHz. The mechanism of the failure is that switching losses in the drive's IGBTs rise roughly with carrier frequency, and at 12 kHz the drive's heat sinks cannot dissipate the loss, the internal temperature rises, and the drive trips on over-temperature — particularly when the ambient is high or the enclosure is poorly ventilated. The false signal is that the drive "can't handle the load," when in fact the load is unchanged and the trip is a consequence of the carrier change. The harm is downtime and a misdirected diagnosis that may lead to replacing the drive with a larger unit rather than reverting the carrier. The defense is to understand the carrier-heat trade-off, derate the drive if a high carrier is required, and improve enclosure cooling rather than chasing the noise alone.

### Using Random-Lay Power Cable Instead of Symmetric VFD Cable

The electrician runs three single THHN conductors and a green ground in conduit between the drive and the motor, the same as for any motor. The mechanism of the failure is that random-lay conductors have unequal spacing and therefore unequal inductance and capacitance per phase, which increases the common-mode voltage, worsens bearing currents, and radiates EMI that couples into nearby signal cables. The false signal is intermittent instrumentation errors or communication failures attributed to the sensors or the PLC, while the motor cable is the actual source. The harm is chronic, hard-to-diagnose signal problems and accelerated bearing failure. The defense is to use a listed VFD cable with three symmetric phase conductors and a continuous foil-and-braid shield with three symmetric ground conductors, bonded at both ends, for every drive-to-motor run.

### Skipping the Auto-Tune and Running on Default Motor Parameters

The drive is installed, the motor nameplate data is entered approximately, and the system is started without an auto-tune. The mechanism of the failure is that the drive's internal motor model — which it uses to regulate current and estimate torque — is based on generic parameters rather than the actual motor's inductance and resistance. At low speed the drive cannot produce rated torque, the motor stalls or trips on overcurrent, and on dynamic loads the drive may hunt or oscillate. The false signal is that the drive "can't control this motor" or the load "is too hard," when the real problem is a mismatched model. The harm is chronic nuisance trips and poor performance that may lead to replacing the drive or the motor unnecessarily. The defense is to perform the auto-tune per the manufacturer's procedure with the motor connected and, for a rotating tune, uncoupled, and to re-tune if the motor is replaced.

### Overlooking the edge case or exception

The typical or textbook scenario is analyzed thoroughly, but the unusual case is skipped. The trap is that the standard path is well-handled while the exception silently produces the wrong outcome, because the agent stopped at the common case and never tested the boundary.

## Self-Check

- Did I select a drive rated for the load type (constant-torque versus variable-torque) with adequate overload capacity, and did I upsize for continuous low-speed operation where cooling is reduced?
- Did I calculate the motor lead length against the drive's critical length for the applied voltage, and install an output reactor, dv/dt filter, or sine filter where the lead length exceeds the limit?
- Did I address common-mode voltage and bearing currents with a symmetric shielded VFD cable, proper motor-to-drive bonding, and shaft grounding or insulated bearings on motors at risk?
- Did I set the carrier frequency to the lowest value the application tolerates, and did I derate the drive or improve enclosure cooling if I raised the carrier above the manufacturer's standard?
- Did I assess the supply impedance and harmonic environment and install an input line reactor or harmonic filter where the supply is stiff or distortion is a concern, and surge protection where transients are expected?
- Did I enter the exact motor nameplate data and perform the auto-tune (rotating, with the motor uncoupled, where possible), and re-tune after any motor replacement?
- For a nuisance-tripping or EMI-plagued installation, did I verify the cable type, shielding, bonding, and grounding rather than assuming the drive or the sensors are defective?
- Is the reasoning documented clearly enough that another practitioner could review the basis and reproduce the conclusion?
