---
name: control-system-commissioning-and-troubleshooting.md
description: Use when the agent is commissioning a DDC or BAS control system, performing point-to-point I/O checkout, verifying the sequence of operations against the written sequence, setting up trend logging and analytics, running functional performance tests, or diagnosing common control faults such as failed sensors, hunting and short cycling, stuck actuators, or communication loss.
---

# Control System Commissioning and Troubleshooting

A control system is not commissioned when the controller powers up and the graphics display; it is commissioned when every point is verified end to end, when every sequence of operations is tested against the written sequence under realistic conditions, and when the system is proven to perform as designed across its operating range. The judgment problem is that commissioning is slow and detailed, so it is often reduced to a power-on check and a demonstration, and the building is accepted with latent defects that surface only over the first season of operation. A sensor wired backward, a sequence that was never tested in the heating mode, an actuator that strokes the wrong way, or a trend log that was never enabled are all defects that a proper commissioning would have caught. This skill covers the point-to-point checkout, the sequence verification, the trend logging, the functional performance testing, and the common faults that together define whether a control system is truly ready for handover.

## Core Rules

### Perform Point-to-Point Checkout on Every I/O Before Testing Sequences

Point-to-point checkout verifies that every field wire lands on the correct controller point and maps to the correct software object, and it is the foundation on which all higher-level testing rests. For each input, the field device is exercised, a sensor is heated, a contact is closed, a pressure is applied, and the controller is confirmed to see the correct response on the correct point. For each output, the controller commands the point and the field device is confirmed to respond, in the correct direction and over the correct range. Checkout is done point by point against the I/O point list, with discrepancies recorded and resolved before any sequence testing begins, because a sequence test on a mis-wired point produces a false result that masks the wiring fault.

The trap is spot-checking a few points and moving to sequence testing. The defense is to check every point against the point list, to confirm direction and range on every output, and to resolve all discrepancies before testing any sequence.

### Verify Each Sequence of Operations Against the Written Sequence

The sequence of operations is the written description of what the system is supposed to do, and commissioning verifies that the controller actually does each of those things, under the conditions that trigger them. Each sequence is tested by driving the inputs to the condition that triggers it, using simulated or actual values, and confirming the outputs respond as written, in the correct order, with the correct timing and interlocks. A sequence that resets supply air temperature based on outdoor air is tested across the outdoor air range; a sequence that stages boilers based on load is tested across the load range; a failure sequence is tested by forcing the fault. Sequences that are only tested at one operating point pass at that point and fail everywhere else.

The trap is demonstrating the system at one condition and calling the sequence verified. The defense is to test each sequence across its full trigger range, including failure and fallback modes, and to document the conditions and results.

### Enable and Use Trend Logging for Diagnosis and Acceptance

Trend logging records point values over time, and it is the primary tool for diagnosing control problems and for demonstrating performance over a season. A trend log captures the hunting that a spot check misses, the slow drift that a single reading hides, and the interaction between points that only appears over time. During commissioning, trends are enabled on the key points of each sequence, the supply and return temperatures, the actuator positions, the sensor values, and the controller outputs, and the trends are reviewed to confirm that the sequence operates as intended over hours and days, not just at the moment of test. A system handed over without trends enabled has no way to prove it performs, and no baseline for future diagnosis.

The trap is leaving trends disabled or set too coarsely to be useful. The defense is to enable trends on the key points of each sequence at an interval that captures the dynamics, to review the trends during commissioning, and to hand over with trends running.

### Run Functional Performance Testing Under Realistic Load

Functional performance testing confirms that the system meets its design performance under realistic conditions, not just that it operates. This means testing the cooling sequence on a cooling day, or simulating one, and confirming that the space reaches setpoint within tolerance and that the equipment stages as designed; testing the heating sequence similarly; testing the failure and fallback modes; and measuring the actual performance against the design intent. A system that operates but cannot hold setpoint under load has passed an operational test and failed a performance test, and the difference only appears when the load is real. Performance testing often spans seasons, with deferred testing for sequences that cannot be exercised at handover.

The trap is declaring success when the equipment runs. The defense is to test under realistic or simulated load, to measure performance against design intent, and to defer and document testing for sequences that cannot be exercised at handover.

### Diagnose Hunting and Short Cycling as Control Problems, Not Equipment Problems

Hunting, oscillation around setpoint, and short cycling, equipment starting and stopping rapidly, are usually control tuning or sensor problems, not equipment failures, and misdiagnosing them leads to replacing perfectly good equipment. Hunting often indicates a poorly tuned loop, an oversized valve or damper, or a sensor in a noisy location; short cycling often indicates a narrow control band, a tight deadband, or a sensor with too little thermal mass. The diagnosis uses trend logs to see the oscillation and its triggers, and the cure is usually a tuning adjustment, a sensor relocation, or a control parameter change, not a hardware swap. Replacing the equipment without fixing the control leaves the problem in place.

The trap is replacing the actuator or the equipment when the control is at fault. The defense is to use trend logs to diagnose hunting and cycling, to check tuning, deadbands, and sensor location first, and to treat equipment replacement as a last resort.

### Diagnose Stuck Actuators and Failed Sensors Through the Trends and the Field

Stuck actuators and failed sensors are common control faults, and they are diagnosed by combining the trend data with a field check. A stuck actuator shows in the trend as the controller commanding a changing output while the measured variable stops responding, and a field check confirms that the actuator does not move when commanded. A failed sensor shows as a reading that flatlines, pegs at a limit, or drifts outside the plausible range, and a calibration check against a reference confirms the failure. The diagnosis distinguishes a sensor failure from an actuator failure from a control failure, because the cure is different for each, and a wrong diagnosis wastes time and parts.

The trap is assuming the controller is bad when a sensor or actuator has failed. The defense is to use trends to localize the fault, to field-check the suspect actuator or sensor, and to calibrate or stroke-test before replacing the controller.

### Treat Communication Loss as a Topology and Noise Problem First

When devices drop off a communication bus, the instinct is often to blame the device, but communication loss is usually a topology, termination, or noise problem, and replacing the device does not fix it. A device that drops intermittently often sits at the end of an unterminated bus, on a star tap, or near a noise source such as a VFD, and the cure is to fix the bus topology, add termination, or separate the bus from the noise. The diagnosis uses the bus monitoring tools to see the error pattern, and the physical inspection to check topology and termination. A device that never communicates from the start is more likely a wiring or addressing fault, while one that drops under load is more likely a noise or topology fault.

The trap is replacing the device that dropped off the bus. The defense is to check bus topology, termination, bias, and separation from noise sources first, to use bus monitoring to characterize the errors, and to treat the device as suspect only after the bus is sound.

## Common Traps

### Spot-Checking Points and Skipping Full Point-to-Point Checkout

The commissioning agent checks a handful of points, sees they work, and moves to sequence testing to save time. The mechanism of the trap is that a sequence test on a mis-wired point can produce a plausible-looking result that masks the wiring fault, because the controller reads something and commands something, and without point-to-point verification the fault is hidden inside the sequence result. The false signal is that the demonstrated sequence works, which proves the logic but not the underlying wiring. The harm is a system accepted with hidden point mismatches that surface as control problems over the first season, traced only with difficulty. The defense is to check every point against the point list before any sequence testing.

### Verifying a Sequence at One Operating Point

The agent tests the supply air reset sequence at one outdoor air temperature, confirms the supply air setpoint responds, and declares the sequence verified. The mechanism of the trap is that a sequence that works at one trigger point can fail at others, because the reset schedule, the interlocks, and the failure modes all depend on the operating condition, and a single-point test never exercises the range. The false signal is that the sequence responded correctly at the tested point, which proves that point but not the range. The harm is a sequence that fails at the conditions that occur in the real season, discovered only when the building mis-performs. The defense is to test each sequence across its full trigger range, including failure and fallback modes.

### Trends Disabled or Too Coarse to Diagnose

The trends are left disabled, or set to an interval so coarse that the dynamics are invisible, and the system is handed over. The mechanism of the trap is that most control problems, hunting, drift, interaction, only appear over time, and without trends at a useful interval there is no way to see them, so problems that could be diagnosed from the trend are instead diagnosed by guesswork or missed entirely. The false signal is that the system operates at handover, which proves function but not performance over time. The harm is a system that cannot be proven or diagnosed, with problems that persist undetected. The defense is to enable trends on the key points of each sequence at an interval that captures the dynamics and to review them during commissioning.

### Calling Equipment Operation a Performance Pass

The agent confirms that the chiller, the boiler, and the air handler all run, and declares the system commissioned. The mechanism of the trap is that operation is not performance; a system can run and still fail to hold setpoint under load, stage inefficiently, or fall back to default behavior that wastes energy, and none of that appears in an operational check. The false signal is that the equipment starts and runs, which proves operation but not performance. The harm is a building that operates but never meets its comfort or efficiency intent, with the gap hidden because the system reports that it is running. The defense is to test under realistic or simulated load, measure performance against design intent, and defer testing for sequences that cannot be exercised at handover.

### Replacing Equipment for a Control Tuning Problem

A space hunts around setpoint, and the maintenance tech replaces the valve actuator, then the coil, then the controller, without improvement. The mechanism of the trap is that hunting is usually a control tuning or sensor problem, and replacing hardware does not change the tuning or the sensor location, so the problem persists through every swap, wasting parts and time. The false signal is that each new part works mechanically, which proves the hardware but not the control. The harm is repeated, ineffective repairs and a problem that was always a tuning or sensor issue. The defense is to use trend logs to diagnose hunting, to check tuning, deadbands, and sensor location first, and to treat hardware replacement as a last resort.

### Blaming the Controller for a Failed Sensor or Stuck Actuator

A loop misbehaves, and the tech assumes the controller is bad and replaces it, with no change. The mechanism of the trap is that the controller is acting on bad data from a failed sensor or commanding an output to a stuck actuator, and replacing the controller changes neither the input nor the output, so the behavior is unchanged. The false signal is that the controller is the active component, which makes it the intuitive suspect. The harm is a replaced controller that does not fix the problem and a sensor or actuator fault that goes unaddressed. The defense is to use trends to localize the fault, to field-check and calibrate the sensor, to stroke-test the actuator, and to treat the controller as suspect only after the field devices are proven.

### Replacing the Device That Dropped Off the Bus

A controller drops off the BACnet bus intermittently, and the tech replaces it, but the new controller drops off too. The mechanism of the trap is that intermittent bus drops are usually topology, termination, or noise problems, and the device is the victim, not the cause, so replacing it moves the symptom to the new device without fixing the bus. The false signal is that the device is the one reporting the error, which makes it the suspect. The harm is repeated device replacements that never fix the problem and a bus fault that persists. The defense is to check bus topology, termination, bias, and separation from noise first, to use bus monitoring to characterize the errors, and to suspect the device only after the bus is sound.

## Self-Check

- Did I perform point-to-point checkout on every input and output against the I/O point list, confirming direction and range on every output, before testing any sequence of operations?
- Did I verify each sequence of operations against the written sequence across its full trigger range, including failure and fallback modes, rather than at a single operating point?
- Did I enable trend logging on the key points of each sequence at an interval that captures the dynamics, review the trends during commissioning, and hand over with trends running?
- Did I run functional performance testing under realistic or simulated load, measure performance against design intent, and document deferred testing for sequences that cannot be exercised at handover?
- For hunting or short cycling, did I use trend logs to diagnose the cause and check tuning, deadbands, and sensor location before considering equipment replacement?
- For a misbehaving loop, did I use trends to localize the fault, field-check and calibrate the sensor, and stroke-test the actuator before suspecting the controller?
- For a device that dropped off the bus, did I check topology, termination, bias, and separation from noise sources, and use bus monitoring to characterize errors, before replacing the device?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
