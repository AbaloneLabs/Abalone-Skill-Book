---
name: vfd-cable-selection-and-motor-termination.md
description: Use when the agent is selecting VFD output cable, terminating motor leads, mitigating reflected wave and bearing current issues, specifying shielded VFD cable with symmetrical grounds, or choosing dV/dt filters, sine wave filters, and shaft grounding rings for variable-frequency drive motor installations.
---

# VFD Cable Selection and Motor Termination

The cable between a variable-frequency drive and its motor is not an ordinary power circuit; it is a transmission line carrying high-frequency pulse-width modulation pulses, and the way that cable is selected, shielded, grounded, and terminated determines whether the motor survives. The judgment problem is that the VFD's fast-switching IGBTs generate voltage pulses with extremely rapid rise times, and those pulses reflect off the motor's impedance, amplify at the motor terminals, capacitively couple to the frame as common-mode voltage, and drive destructive current through the bearings. An electrician who pulls standard THHN in conduit, lands the leads, and walks away will destroy motors through winding insulation failure and bearing fluting, and will blame the motor manufacturer for a failure the cable and termination caused. This skill covers the cable selection, the symmetrical grounding, the lead-length limits and their mitigation, and the bearing current defenses that together make a VFD-driven motor reliable.

## Core Rules

### Use a Listed VFD Cable With Symmetrical Conductors and Continuous Shield

A proper VFD cable has three phase conductors arranged symmetrically with three symmetric ground conductors (or a continuous ground layer) and a continuous foil-and-braid shield, all designed to balance the phase inductance and capacitance and to contain the high-frequency electromagnetic energy the drive produces. The symmetry minimizes common-mode voltage, the shield contains radiated EMI that would otherwise couple into nearby signal and communication cables, and the continuous ground provides a low-impedance return path for high-frequency common-mode current back to the drive. The trap is using three single THHN conductors and a green ground in conduit, which has random spacing, unbalanced impedance, no shielding, and a high-impedance ground path that maximizes common-mode voltage and bearing current. The defense is to specify a listed VFD cable (such as TC-ER VFD or MC VFD cable) with symmetric grounds and a continuous shield for every drive-to-motor run, regardless of length.

### Bond the Shield and Ground at Both Ends With Low-Impedance High-Frequency Paths

The VFD cable's shield and ground conductors must be bonded to the drive enclosure and the motor frame with low-impedance, high-frequency connections, which means using proper EMC cable glands or 360-degree bonding rather than pigtail connections. A pigtail ground wire has significant high-frequency impedance and defeats the shield's function of returning common-mode current to the drive. The trap is landing the shield on a terminal block with a pigtail or wrapping it back on the cable jacket, which provides a DC bond but a poor high-frequency bond, leaving common-mode current to find other paths (through the motor bearings, through the building steel, through signal cables). The defense is to use 360-degree EMC bonding at both ends, bond the motor frame to the drive enclosure through the cable's continuous ground, and verify the bonding with a high-frequency measurement where critical.

### Respect the Motor Lead Length Limit and Mitigate Reflected Waves

The fast-rising PWM pulses travel down the motor leads and reflect off the motor's impedance, and on long leads the reflected wave adds to the incident pulse to produce peak voltages at the motor terminals of up to twice the DC bus voltage, repeated thousands of times per second. This reflected wave punctures the motor's winding insulation. The critical lead length depends on the drive's voltage and rise time: a 480V drive with fast IGBTs may see damaging reflections at 50 to 100 feet, a 575V drive is worse, and 460V inverter-duty motors are typically rated to withstand about 1600V peak. The trap is running long leads from a central motor control center to a distant rooftop motor with no mitigation, then replacing the motor repeatedly. The defense is to calculate the critical lead length for the drive and voltage, keep leads shorter where possible, and install a dV/dt filter, a load reactor, or a sine-wave filter at the drive output where the lead length exceeds the limit.

### Choose Between dV/dt Filters, Load Reactors, and Sine-Wave Filters by Application

The three output mitigation devices address different problems at different costs. A load reactor (typically 3 to 5 percent impedance) limits the rate of current change and slightly reduces peak voltage, helping with lead length and protecting the drive, but it does not fully eliminate reflected-wave peaks. A dV/dt filter reduces the voltage rise time and peak more effectively, extending the allowable lead length significantly. A sine-wave filter converts the PWM output to a near-sinusoidal waveform, eliminating reflected-wave and bearing current problems almost entirely and allowing very long leads, but it is the most expensive and largest. The trap is applying a cheap load reactor where a dV/dt or sine-wave filter is required, or overspending on a sine-wave filter where a dV/dt filter would suffice. The defense is to match the device to the lead length and the motor's insulation rating per the manufacturer's curves, and to prefer a sine-wave filter only where leads are very long or the motor is critical.

### Mitigate Bearing Currents With Shaft Grounding and Insulated Bearings

Common-mode voltage from the PWM drive capacitively couples to the rotor and drives current through the bearings, causing electrical discharge machining (EDM) that flutes the bearing races and destroys the bearing in months. The mitigation is to give the common-mode current a lower-impedance path around the bearing than through it. A shaft grounding ring conducts current from the shaft to the frame through a brush or conductive ring, bypassing the bearing. An insulated bearing (ceramic-coated or ceramic balls) blocks the current through the bearing on one end, forcing it to find another path. For larger motors and higher drives, both may be needed. The trap is installing a VFD on a motor with standard bearings and no shaft grounding, then rebuilding the motor with the same bearings when it fails. The defense is to install a shaft grounding ring on motors above roughly 100 hp (or per the manufacturer's guidance), to use insulated bearings on the non-drive end for larger motors, and to recognize fluting as an electrical failure signature.

### Terminate the Motor Leads to Preserve the Insulation and the Shield

At the motor terminal box, the VFD cable's shield must be bonded to the motor frame with a 360-degree connection, the phase conductors must be terminated without sharp bends or nicks that concentrate the high-voltage stress, and the conduit or armor must be continuous to the box. A poor termination that leaves the shield unbonded or the conductors stressed creates a localized weak point where insulation fails. The trap is treating the motor termination like any power termination, with a pigtail ground and conductors jammed into the box, creating a high-frequency weak point. The defense is to use an EMC cable gland at the motor box, bond the shield 360 degrees to the frame, terminate the conductors with the proper lugs and without stress, and use a terminal box large enough for the VFD cable.

### Verify the Motor Is Inverter-Duty Rated for the Application

The motor must be rated for VFD service, with inverter-duty insulation (typically rated for 1600V peak or higher on a 460V system) and the thermal capability for the speed range and cooling method. A standard-duty motor on a VFD will fail early from insulation breakdown and overheating at low speed. The trap is applying a VFD to an existing standard motor to add speed control, without checking the insulation class or the cooling. The defense is to verify the motor's nameplate indicates inverter-duty insulation, to confirm the cooling method (TEFC motors lose cooling at low speed and may need a constant-speed cooling fan), and to replace or upgrade motors that are not inverter-rated for the application.

## Common Traps

### Using Random-Lay THHN Instead of Symmetric Shielded VFD Cable

The electrician runs three single THHN conductors and a green ground in conduit between the drive and the motor, the same as for any motor. The mechanism of the failure is that random-lay conductors have unequal spacing and therefore unequal inductance and capacitance per phase, which increases the common-mode voltage, worsens bearing currents, and radiates EMI that couples into nearby signal cables. The false signal is that the motor runs, which proves conductivity but ignores the high-frequency behavior that destroys bearings and corrupts signals. The harm is chronic bearing failure, signal interference, and motors that fail far short of their rated life. The defense is to use a listed VFD cable with symmetric grounds and a continuous shield for every drive-to-motor run.

### Pigtailing the Shield Instead of 360-Degree Bonding

The VFD cable's shield is landed on a terminal block with a short pigtail wire, or wrapped back on the jacket, instead of bonded with a 360-degree EMC gland. The mechanism of the failure is that a pigtail has high impedance at the PWM frequencies, so the shield cannot return common-mode current to the drive, and the current finds paths through the motor bearings, the building steel, and signal cables. The false signal is that the shield is connected and shows continuity, which proves a DC bond but not a high-frequency bond. The harm is bearing current damage and EMI that the shield was supposed to prevent. The defense is to use 360-degree EMC bonding at both ends.

### Running Long Leads With No Reflected-Wave Mitigation

The drive is in a central motor control center and 200 feet of cable runs to a rooftop fan motor with no output filter. The mechanism of the failure is that the fast PWM pulses reflect off the motor impedance and constructively add to produce peak voltages of 1500 to 2000 volts at the motor terminals, repeated thousands of times per second, which breaks down even inverter-duty insulation. The false signal is that the motor runs fine initially, which proves the system works until the insulation fails in weeks to months. The harm is repeated motor replacement with the root cause unaddressed. The defense is to calculate the critical lead length and install a dV/dt filter, reactor, or sine-wave filter where the lead exceeds it.

### Using a Load Reactor Where a dV/dt or Sine-Wave Filter Is Required

A 3 percent load reactor is installed on a long-lead application where the reflected-wave peaks still exceed the motor's insulation rating. The mechanism of the failure is that a load reactor limits current rise but does not adequately clip the voltage peaks, so the insulation still sees damaging voltage and fails. The false signal is that a reactor is installed, which proves mitigation effort but not adequate mitigation for the lead length. The harm is a motor failure despite the apparent protection. The defense is to match the mitigation device to the lead length and insulation rating per the manufacturer's curves.

### Installing a VFD on Standard Bearings With No Shaft Grounding

A VFD is applied to a 150 hp motor with standard bearings and no shaft grounding ring or insulated bearing. The mechanism of the failure is that common-mode voltage capacitively couples to the rotor, drives current through the bearings, and electrical discharge machining flutes the races in months. The false signal is that the motor runs fine for the first few months, which proves the system works until the bearing fails. The harm is a bearing failure misattributed to the bearing manufacturer, with the motor rebuilt using the same bearings and failing again. The defense is to install shaft grounding rings and insulated bearings per the motor size and drive voltage.

### Applying a VFD to a Non-Inverter-Duty Motor

A VFD is added to an existing standard motor for speed control, without verifying the insulation rating or the cooling capability. The mechanism of the failure is that the standard motor's insulation cannot withstand the PWM voltage peaks, and the TEFC cooling fan loses effectiveness at low speed, so the motor fails from insulation breakdown or overheating. The false signal is that the motor turns and produces torque, which proves operation but not durability under PWM. The harm is premature motor failure. The defense is to verify inverter-duty insulation and adequate cooling across the speed range, and to upgrade or replace motors that are not inverter-rated.

## Self-Check

- Did I specify a listed VFD cable (TC-ER VFD or MC VFD) with three symmetric phase conductors, symmetric ground conductors, and a continuous foil-and-braid shield for every drive-to-motor run?
- Did I bond the shield and ground at both ends with 360-degree EMC connections (proper EMC cable glands), and avoid pigtails that have high impedance at PWM frequencies?
- Did I calculate the motor lead length against the drive's critical length for the applied voltage, and install a dV/dt filter, load reactor, or sine-wave filter where the lead length exceeds the limit?
- Did I choose the mitigation device (reactor, dV/dt filter, or sine-wave filter) based on the lead length and the motor's insulation rating per the manufacturer's curves, rather than defaulting to the cheapest option?
- Did I install shaft grounding rings on motors above the manufacturer's guidance (typically around 100 hp), and use insulated bearings on the non-drive end for larger motors?
- Did I terminate the motor leads with an EMC gland at the motor box, bond the shield 360 degrees to the frame, and avoid conductor stress and sharp bends?
- Did I verify the motor is inverter-duty rated for the insulation class and the cooling method across the speed range, and replace or upgrade motors that are not inverter-rated?
- Is the cable selection, lead-length calculation, and mitigation documented so another practitioner can confirm the basis and reproduce the result?
