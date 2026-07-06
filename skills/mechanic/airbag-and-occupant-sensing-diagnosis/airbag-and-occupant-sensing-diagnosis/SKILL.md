---
name: airbag-and-occupant-sensing-diagnosis.md
description: Use when the agent is diagnosing airbag warning lamp, SRS module or crash sensor faults, occupant classification or weight sensor issues, seat belt pretensioner or buckle switch faults, clock spring or spiral cable faults, or evaluating SRS deployment loops and side or curtain airbag circuits on supplemental restraint systems.
---

# Airbag and Occupant Sensing Diagnosis

The supplemental restraint system (SRS) is the system where a wrong diagnosis or a wrong probe is most dangerous, because the airbags deploy with explosive force and an accidental deployment injures the technician and destroys the vehicle, and because the system is designed to fail-safe (deploy or set a lamp) at any sign of a fault. The judgment problem is that SRS faults (a warning lamp, a code for a sensor, a circuit resistance out of range) overlap across clock spring, crash sensor, occupant sensor, seat belt pretensioner, and wiring causes, and because the diagnostic must not trigger a deployment. A technician who probes an airbag circuit with a test light (deploying the bag), or who replaces the SRS module for a broken clock spring, hands back a vehicle with the same lamp and a serious safety risk. This skill covers the disciplined diagnosis of SRS and occupant sensing faults: safe procedures, code interpretation, and circuit isolation.

## Core Rules

### Disable and Verify the SRS Is Safe Before Any Work

The first action in any SRS work is to disable the system and verify it is safe, because an accidental deployment during diagnosis or service injures and destroys. The disciplined disable: turn the ignition off, disconnect the negative battery cable, wait the specified time (often 10 minutes, sometimes longer, for the backup capacitors in the SRS module to discharge — the module has internal capacitors that can deploy the bags even after the battery is disconnected), and verify the system is de-energized before touching any airbag connector or circuit. The tradeoff is that the wait is mandatory and not optional, and a technician who skips it risks a deployment from the backup power. The disciplined technician follows the OEM disable procedure and the wait time on every SRS job.

### Never Probe Airbag Circuits With a Test Light or Low-Impedance Meter

An airbag deploys when its squib (the explosive initiator) receives sufficient current, and a test light or a low-impedance meter can deliver that current and deploy the bag. The disciplined rule: never probe an airbag circuit with a test light, a grounded probe, or any meter that is not specified as high-impedance and airbag-safe; use only the OEM-specified test procedures, which often use a specified load resistor or a dedicated SRS test harness, and measure at the module connector (with the module disconnected and the bag isolated) rather than at the bag. The tradeoff is that this restricts the testing methods, but a deployment from a wrong probe is catastrophic. The disciplined technician uses the OEM procedure and the safe tools exclusively.

### Interpret SRS Codes as Circuit and Resistance Faults, Not Component Verdicts

SRS codes describe circuit faults (resistance too high, resistance too low, voltage out of range, no communication) and not component verdicts, and the disciplined interpretation reads the circuit fault and isolates the cause. A "resistance too high" code for a driver's airbag points to an open in the circuit — the clock spring (the most common cause, a spiral cable that breaks with steering wheel rotation), the wiring, the connector, or the bag itself; a "resistance too low" code points to a short across the circuit; a "voltage out of range" code points to a short to power or ground. The disciplined diagnosis isolates the circuit by measuring the resistance at the module connector (with the module disconnected and the bag safely isolated), by wiggling the wiring and the clock spring to reproduce an intermittent, and by substituting a known-good clock spring or load resistor to confirm the circuit. The tradeoff is that the code is the starting point, not the verdict, and the component is confirmed by circuit measurement.

### Diagnose the Clock Spring as the Most Common Rotating-Circuit Fault

The clock spring (the spiral cable in the steering column that maintains electrical connection to the driver's airbag, the horn, and the steering wheel controls while the wheel rotates) is the most common SRS fault, because the cable flexes with every turn and fatigues and breaks. The disciplined diagnosis: a driver's airbag resistance-too-high code, often intermittent and sometimes accompanied by a horn or steering wheel control fault (sharing the clock spring), is confirmed by measuring the clock spring continuity while rotating the steering wheel (a broken clock spring opens at certain angles) and by substituting a known-good clock spring. The tradeoff is that the clock spring is cheap relative to the SRS module and the bag, but it is skipped in favor of the module. The disciplined technician tests the clock spring before the module or the bag.

### Evaluate Occupant Classification and Weight Sensor Faults

Modern passenger vehicles use an occupant classification system (OCS) — a weight or pressure sensor in the passenger seat — to decide whether to arm and deploy the passenger airbag based on the occupant's weight (a child or a light occupant may disable or reduce the bag, an empty seat disables the passenger bag). The OCS faults (a code, a "airbag off" lamp that is wrong, a passenger bag that does not arm with an adult) are diagnosed through the OCS sensor and its calibration. The disciplined diagnosis: read the OCS weight data on the scan tool (does it read plausibly for the occupant), check the sensor and its wiring under the seat (a common fault is wiring damaged by objects under the seat or by seat movement), and perform the OCS zero or calibration procedure (many systems require a zero-reset with the seat empty and level after replacing the sensor or the seat). The tradeoff is that the OCS is calibration-sensitive, and a sensor replaced without the zero procedure reads wrong.

### Diagnose Seat Belt Pretensioner and Buckle Switch Faults

The seat belt pretensioner (a small explosive charge that tightens the belt in a crash) and the buckle switch (which tells the SRS whether the belt is buckled) are part of the SRS loop, and their faults set codes and lamps. The disciplined diagnosis: a pretensioner code is a circuit fault (resistance too high or too low), diagnosed like an airbag circuit (measure at the module connector, check the wiring under the seat); a buckle switch fault is diagnosed by the switch state on the scan tool (does it toggle correctly when the belt is buckled and unbuckled) and the switch continuity. The tradeoff is that the pretensioner and the buckle are part of the seat belt system and are replaced as part of a seat belt or seat service, and their faults are often wiring under the seat damaged by seat movement.

### Clear Codes and Verify the Lamp and the Readiness After the Repair

After an SRS repair, the codes are cleared and the system is verified: the lamp should come on with the ignition (the bulb check) and go out after the startup self-test (confirming no active faults), and a readiness check (often a drive cycle or a scan-tool readiness test) confirms the system is armed and ready. The tradeoff is that some SRS codes require a specific sequence to clear (a repair confirmation, a number of drive cycles), and the disciplined technician verifies the lamp behavior and the readiness before returning the vehicle. A lamp that stays on is an unresolved fault; a lamp that is off but the system is not ready is a latent fault.

## Common Traps

### Probing an Airbag Circuit With a Test Light and Deploying the Bag — The technician probes an airbag connector with a test light to "check for power," and the test light's current deploys the airbag. The trap mechanism is that the airbag squib deploys on a small current, and a test light delivers that current; the deployment is instantaneous and injures the technician and destroys the vehicle. The false signal is the desire to "test" the circuit; the harm is a deployment injury and a destroyed vehicle. The disciplined technician never probes an airbag circuit with a test light and uses only the OEM-specified safe procedures.

### Replacing the SRS Module for a Clock Spring Fault — The driver's airbag code sets, the technician condemns the SRS module, and the fault persists because the cause was a broken clock spring. The trap mechanism is that the clock spring is the most common rotating-circuit fault and produces a code that looks like a module or a bag fault, and the module is the expensive, easy target. The false signal is the code pointing at the airbag circuit; the harm is an unnecessary SRS module (which requires programming). The disciplined technician tests the clock spring continuity with rotation before the module.

### Diagnosing an Occupant Sensor Fault Without the Calibration — The OCS is replaced, the technician does not perform the zero or calibration procedure, and the passenger airbag arms or disarms incorrectly because the sensor reads wrong. The trap mechanism is that the OCS is calibration-sensitive, and a replaced sensor without the zero reads offset and misclassifies the occupant. The false signal is the sensor "being installed"; the harm is an airbag that arms or disarms wrong, a safety risk. The disciplined technician performs the OCS zero or calibration after the replacement.

### Skipping the Battery Disconnect Wait and Working on a Live SRS — The technician disconnects the battery and immediately works on the airbag connector, and the backup capacitor deploys the bag. The trap mechanism is that the SRS module has internal capacitors that retain deployment energy for minutes after the battery is disconnected, and working before the wait risks a deployment. The false signal is the battery "being disconnected"; the harm is a deployment injury. The disciplined technician waits the specified time for the capacitors to discharge.

### Clearing the Code Without Verifying the Readiness — The SRS code is cleared, the lamp goes out, the technician returns the vehicle, and the lamp returns because the fault was not resolved or the system is not ready. The trap mechanism is that clearing the code does not resolve the fault, and the lamp returns on the next self-test if the fault persists. The false signal is the lamp being off after the clear; the harm is a vehicle returned with an unresolved safety fault. The disciplined technician verifies the readiness and the lamp behavior after the repair.

## Self-Check

- Did I disable the SRS (ignition off, negative battery disconnected, specified wait for capacitor discharge) and verify it is safe before any work?
- Did I use only OEM-specified, airbag-safe test procedures and tools, and never probe an airbag circuit with a test light or a low-impedance meter?
- Did I interpret the SRS code as a circuit fault (resistance, voltage) and isolate the cause rather than treat it as a component verdict?
- For a driver's airbag circuit fault, did I test the clock spring continuity with steering wheel rotation before the module or the bag?
- For an occupant classification fault, did I read the OCS weight data, check the under-seat wiring, and perform the zero or calibration procedure after any sensor or seat replacement?
- For a seat belt pretensioner or buckle switch fault, did I measure the circuit at the module connector and check the under-seat wiring?
- Did I clear the codes and verify the lamp behavior (bulb check on, off after self-test) and the system readiness after the repair?
- Did I confirm no active SRS codes and the system is armed and ready before returning the vehicle?
- Did I document the SRS disable and the safety wait, and follow the OEM procedure for any deployed component replacement?
