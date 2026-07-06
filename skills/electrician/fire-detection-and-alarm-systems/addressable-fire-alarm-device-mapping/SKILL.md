---
name: addressable-fire-alarm-device-mapping.md
description: Use when the agent is mapping addressable fire alarm devices, programming signaling line circuits, assigning device addresses, or configuring cause-and-effect matrix logic, with attention to SLC loop loading, device addressing, NFPA 72 acceptance testing, and zone-to-output correlation.
---

# Addressable Fire Alarm Device Mapping

An addressable fire alarm system can pinpoint the exact device in alarm, but that precision is worthless, and dangerous, if the device-to-address mapping, the loop loading, and the cause-and-effect logic are wrong. The judgment problem is that an installer who auto-enrolls devices, accepts the default labels, and programs the cause-effect matrix by guesswork will produce a system that reports an address but not a location, that overloads a loop and loses devices in alarm, and that activates the wrong notification zones during a fire. The device mapping, the loop loading margin, the address assignment, and the cause-effect matrix are the decisions that determine whether the system tells the truth during a fire. This skill covers the addressing, loop loading, mapping, and logic decisions that determine whether an addressable fire alarm is accurate and survivable.

## Core Rules

### Assign and Document Unique Addresses With Meaningful Labels

Every addressable device must have a unique address on its signaling line circuit (SLC) and a label that identifies its physical location in terms a responder can use, not just an address number. The label should describe the device type and location, such as smoke detector third floor east corridor, because the responding firefighters and the facility staff act on the label, not the raw address. Auto-enrollment that assigns addresses without labeling leaves a panel full of address numbers that mean nothing to the user. The address assignment must follow the manufacturer's method, whether dip switches, rotary switches, or soft addressing, and the documented device list must be kept current as devices are added or moved. A mismatch between the documented address and the installed device defeats the purpose of addressability.

The trap is auto-enrolling and accepting default labels. The defense is to assign unique addresses per the manufacturer method, to label every device with its type and location, and to keep the device list current and accurate.

### Size SLC Loop Loading With Margin for Devices and Current

Each SLC has a maximum device count and a maximum current limit, and both must be respected with margin, because a loop loaded to its absolute maximum works on the bench but fails under alarm conditions when many devices draw current simultaneously. The device count limit is set by the panel's addressing capacity, and the current limit is set by the loop power supply. In alarm, detectors and modules draw more current than in standby, and a loop with no margin can drop devices off the loop exactly when they are in alarm, producing an ambiguous or wrong report. The loop should be loaded below the maximum count and current, with a margin recommended by the manufacturer, and the standby and alarm current calculations must be documented. Devices should be distributed across multiple loops so that a single loop fault does not disable the whole system.

The trap is loading a loop to its listed maximum. The defense is to size loop loading with count and current margin, to document the standby and alarm calculations, and to distribute devices across multiple loops.

### Map Devices to Physical Locations and Verification Zones

Addressability is only useful if the panel's reported location matches the physical device, and this requires that each device be mapped to its actual location and verified during installation. A device that reports third floor corridor but is physically on the second floor sends responders to the wrong place, which is worse than no information. The mapping is verified by activating each device in place during commissioning, confirming that the panel reports the correct location, and correcting any discrepancy before acceptance. Devices should be grouped into verification zones that match the building's zones, such as floors or wings, so that the panel's zone reports are meaningful for evacuation and response. The mapping documentation should include a device address list cross-referenced to the physical location and the drawing.

The trap is trusting the enrollment list without field verification. The defense is to activate each device in place during commissioning, confirm the reported location matches the physical device, and correct any mismatch before acceptance.

### Program Cause-and-Effect Logic to Match the Building Response Plan

The cause-and-effect matrix defines what outputs activate for which inputs, and it must match the building's fire response and evacuation plan, not a generic default. A typical cause-effect for a smoke detector in a zone is to activate the notification appliances for that zone and adjacent zones, to release hold-open doors, to recall elevators, and to shut down certain HVAC, but the exact mapping depends on the building's phased evacuation plan and the authority having jurisdiction. The matrix must be designed to avoid both over-activation, which evacuates the whole building for a single detector and breeds complacency, and under-activation, which fails to notify occupants in the affected zone. The logic must be documented and tested input by input during acceptance, because a single wrong correlation can send the notification to the wrong zone during a fire.

The trap is accepting the default cause-effect matrix. The defense is to design the matrix to match the building's response plan, to document each correlation, and to test every input-to-output path during acceptance.

### Supervise the SLC and Verify Fault Reporting

The SLC must be supervised so that a fault on the loop, an open or a short, produces a trouble signal at the panel and the system continues to operate with the remaining capacity. Addressable loops are typically wired as a T-tap or Class A (style 6/7) or Class B configuration, and the class determines whether the loop survives a single fault. Class A wiring returns to the panel so that a single open does not disable devices beyond the break, while Class B does not, so the class must be chosen based on the survivability required for the application. The supervision must be verified during acceptance by introducing an open and a short at the end of line and confirming the panel reports the correct trouble. A loop that works but is not supervised can lose devices and not know it.

The trap is wiring the loop and skipping the fault test. The defense is to choose the loop class for the required survivability, to introduce open and short faults during acceptance, and to confirm trouble reporting.

### Test Per NFPA 72 Acceptance Requirements and Document Results

NFPA 72 defines the acceptance testing requirements for fire alarm systems, and addressable device mapping is tested by activating each device and confirming the correct address, location, and output response. The acceptance test exercises every device, every cause-effect correlation, and every fault condition, and the results must be documented in a record that the authority having jurisdiction accepts. Field devices must be tested in their installed positions, with smoke or heat applied to detectors at their rated sensitivity, and with notification outputs verified for the correct zone. The documentation must include the device list, the cause-effect matrix, the battery calculations, and the test results. A system that is not tested to NFPA 72 cannot be accepted, and a system whose documentation is incomplete cannot be maintained.

The trap is partial testing and thin documentation. The defense is to test every device and correlation per NFPA 72, to document the results completely, and to deliver the record to the AHJ.

## Common Traps

### Auto-Enrolled Devices With Default or Generic Labels

The installer auto-enrolls the devices and leaves the labels as the default address strings, trusting that the address is enough. The mechanism of the trap is that responders act on the label, and a panel full of address numbers with no location sends firefighters to search the building instead of going straight to the device, wasting the minutes that addressability was meant to save. The false signal is that the panel reports the correct address when a device is activated, which proves the mapping but not the usability. The harm is delayed response and confusion during a fire. The defense is to label every device with its type and location and to keep the device list current.

### SLC Loaded to Maximum Count and Current

The installer loads the SLC to its listed maximum device count and current, assuming the listing guarantees operation. The mechanism of the trap is that the listing is a ceiling, not a design target, and a loop at maximum has no margin for the alarm current surge when many devices activate at once, so the loop voltage sags and devices drop off in alarm, producing ambiguous or missing reports exactly when accuracy matters. The false signal is that the loop passes a bench test with all devices in standby, which never exercises the alarm load. The harm is lost devices during a real fire. The defense is to load the loop below maximum with documented margin and to verify alarm current.

### Device Reports the Wrong Location After Enrollment

The installer enrolls the devices and accepts the panel's location list without activating each device in place to verify the mapping. The mechanism of the trap is that enrollment order and physical order rarely match, so a device that reports third floor east may be physically on the second floor, and without field verification the panel sends responders to the wrong location during a fire. The false signal is that the panel shows a clean device list with no troubles, which proves enrollment but not mapping accuracy. The harm is responders sent to the wrong place. The defense is to activate each device in place during commissioning and to correct any mismatch.

### Default Cause-Effect Matrix Activates the Wrong Zones

The installer accepts the panel's default cause-effect matrix without aligning it to the building's evacuation plan. The mechanism of the trap is that the default matrix activates outputs based on generic assumptions that may not match the building, so a detector in one zone might trigger notification in the wrong zone, evacuate the whole building for a single device, or fail to notify the affected zone, and any of these errors can be lethal during a fire. The false signal is that activating a device produces some notification during testing, which proves the logic runs but not that it is correct. The harm is wrong or excessive evacuation. The defense is to design the matrix to the response plan and to test every correlation.

### Loop Wired Without Supervision Verification

The installer wires the SLC and confirms devices communicate but never introduces an open or short to verify fault reporting. The mechanism of the trap is that a loop that communicates is not necessarily supervised, and without a fault test the installer cannot confirm that a cut or short will be reported, so a damaged loop may lose devices silently until a fire reveals the gap. The false signal is that all devices report normally during commissioning, which proves communication but not supervision. The harm is undetected loop damage that disables devices. The defense is to introduce open and short faults during acceptance and to confirm trouble reporting.

### Partial Acceptance Testing With Incomplete Documentation

The installer tests a sample of devices and fills in a generic test record to save time. The mechanism of the trap is that NFPA 72 requires every device and correlation to be tested, and untested devices and logic paths may be wrong, so the system passes a partial test but fails during a real event, and the incomplete record cannot support maintenance or AHJ acceptance. The false signal is that the tested devices pass, which proves those devices but not the system. The harm is an unverified system that fails when needed. The defense is to test every device and correlation per NFPA 72 and to document the results completely.

## Self-Check

- Did I assign a unique address to every device per the manufacturer method, label each with its type and location, and keep the device list current and accurate?
- Did I size each SLC loop below the maximum device count and current with documented margin, calculate standby and alarm current, and distribute devices across multiple loops?
- Did I activate each device in place during commissioning, confirm the reported location matches the physical device, and correct any mismatch before acceptance?
- Did I design the cause-and-effect matrix to match the building's fire response and evacuation plan, document each correlation, and test every input-to-output path during acceptance?
- Did I choose the SLC loop class for the required survivability, introduce open and short faults during acceptance, and confirm trouble reporting at the panel?
- Did I test every device and correlation per NFPA 72 acceptance requirements, document the results completely, and deliver the record to the authority having jurisdiction?
- Did I include the device list, cause-effect matrix, battery calculations, and test results in the documentation for ongoing maintenance?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
