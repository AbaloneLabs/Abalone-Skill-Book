---
name: pv-system-grounding-and-arc-fault-protection.md
description: Use when the agent is grounding PV arrays, bonding module frames, and configuring arc-fault and ground-fault protection for solar systems, covering functional grounding, GEC sizing, array bonding, DC arc-fault detection per NEC 690.11, and ground-fault detection and isolation.
---

# PV System Grounding and Arc-Fault Protection

A PV array is a large, energized, weather-exposed DC power plant on a roof or in a field, and its grounding and fault protection are what keep it from becoming a fire, shock, or equipment-damage hazard over its decades of service. The judgment problem is that grounding and bonding look like routine connections of metal to wire, and fault protection looks like a matter of fitting a device, which ignores the distinction between equipment grounding and functional grounding, the corrosion and thermal cycling that attack outdoor bonds, and the physics of series arcs that ordinary overcurrent devices cannot detect. When these are missed, frames become energized and undetected, bonds fail silently, and arcs smolder until they ignite. This skill covers the grounding architecture, the bonding practices, the arc-fault detection requirement, and the ground-fault detection and isolation that together protect a PV system.

## Core Rules

### Distinguish Equipment Grounding, Functional Grounding, and the GEC

PV grounding has three distinct functions that must not be conflated. Equipment grounding bonds all exposed non-current-carrying metal, such as module frames and racking, to the grounding system so that a fault to frame is cleared and the metal never rises to a hazardous voltage. Functional grounding (a system reference, found in grounded or functionally grounded arrays) ties one current-carrying conductor, often the negative, to ground through the inverter or a reference device, which sets the system's voltage reference and enables ground-fault detection. The grounding electrode conductor (GEC) ties the system grounding point to the building's grounding electrode system. Each has its own sizing rules, connection points, and code requirements, and confusing them produces a system that is neither safe nor correctly referenced.

### Size the GEC and Equipment Grounding Conductors to Code

The GEC and the equipment grounding conductors (EGC) on the DC side are sized per NEC 690 and Article 250 rules, which are stricter for PV than for conventional circuits because of the sustained current capability of the array. The GEC is sized to the array's short-circuit current and the electrode type, and the DC EGC is sized to the overcurrent protection and the corrected Isc, often larger than an installer expects. Undersizing the EGC means it cannot carry a fault long enough to clear it, leaving frames energized. Sizing is a calculation against the corrected and derated current, documented in the design, and verified at inspection.

### Bond Module Frames and Racking With Listed, Corrosion-Resistant Devices

Module frames and racking sections must be bonded together and to the EGC so that the entire array is at one potential and any fault is cleared. The bonding devices must be listed for the purpose, compatible with the anodized or coated aluminum they contact, and resistant to the corrosion and thermal cycling of the outdoor environment. Piercing anodized coatings without a listed device, or relying on a friction or sliding contact, produces a bond that reads zero ohms at installation and opens after years of corrosion and movement. Bonding is done with listed lay-in lugs, grounding clips, or washer-type devices, torqued to specification, and the continuity of the bonded array is verified across the installation.

### Provide DC Arc-Fault Detection Per NEC 690.11

Series arcs in PV wiring, caused by loose connectors, broken conductors, or damaged modules, are not detected by ordinary overcurrent devices because the arc current is within the normal operating range, yet the arc carries enough energy to ignite a fire. NEC 690.11 requires arc-fault circuit interruption (AFCI) on PV systems above a threshold, which detects the high-frequency signature of a series arc and shuts the inverter down or isolates the fault. The detection device must be specified, enabled, and commissioned, and its trip must be verified. Treating arc-fault detection as optional or leaving it disabled defeats the single protection against the most common PV fire ignition mode.

### Configure Ground-Fault Detection and Isolation Correctly

Ground faults, where a current-carrying conductor contacts a grounded frame or racking, must be detected and isolated so that the fault does not persist and energize metal or start a fire. Ground-fault detection in a grounded or functionally grounded system uses a current sensor on the bond, and on ungrounded systems uses residual current monitoring. The detection must be set to the right sensitivity, the isolation must de-energize the appropriate circuits, and the alarm must be visible so the fault is found and repaired rather than ignored. A ground-fault indicator that trips and is repeatedly reset without repair is a sign of a defect being masked, not managed.

### Coordinate Fault Protection Across the DC and AC Sides

The DC arc-fault and ground-fault protection, the overcurrent devices, and the inverter's internal protection must be coordinated so that a fault is cleared by the device closest to it, without nuisance tripping of upstream devices. The AC side interconnection must also coordinate with the utility's protection requirements. Coordination is a design exercise that maps each fault to the device that should clear it, confirms the ratings and settings, and documents the scheme. Uncoordinated protection either fails to clear faults or trips unnecessarily, eroding confidence in the system.

### Maintain Bonding and Fault Protection Over the System Life

Grounding and fault protection are not install-and-forget. Bonds corrode, connectors loosen, and detection devices can be disabled by software updates or nuisance trips. The system should include a maintenance plan that periodically verifies frame-to-frame and frame-to-EGC continuity, tests the arc-fault and ground-fault detection, and inspects connectors and bonds for corrosion. A PV system whose protection silently degrades carries the same fire and shock risk as one that was never protected, and the degradation is invisible without testing.

## Common Traps

### EGC Undersized to the Corrected Array Current

The installer sizes the DC equipment grounding conductor to a rule of thumb rather than to the corrected Isc and overcurrent rating. The mechanism is that a fault current flows through the EGC, and an undersized conductor cannot carry it long enough to clear the fault, so frames stay energized or the conductor burns. The false signal is that the conductor reads continuous at installation. The harm is a grounding system that cannot perform its protective function under a real fault.

### Relying on Sliding Contact for Bonding

The installer assumes that module-to-rail and rail-to-rail contact through anodized surfaces provides bonding without listed devices. The mechanism is that anodized aluminum is an insulator until pierced, and sliding or friction contacts corrode and open over years of thermal cycling and wind vibration. The false signal is that continuity reads low at commissioning. The harm is a bonded array that silently unbonds, leaving sections of racking and frames ungrounded and able to energize on a fault.

### Arc-Fault Detection Specified but Left Disabled

The installer installs an inverter with arc-fault capability but leaves the function disabled, or disables it after nuisance trips. The mechanism is that a series arc then has no detection, so the arc persists and ignites nearby material, because overcurrent devices cannot see it. The false signal is that the system runs without nuisance trips. The harm is the loss of the primary protection against the most common PV fire ignition mode, with the risk hidden until a fire starts.

### Ground-Fault Trip Repeatedly Reset Without Repair

The operator resets a ground-fault alarm repeatedly without locating and repairing the fault. The mechanism is that the fault persists, intermittently energizing frames or leaking current, while the reset masks the symptom. The false signal is that the system returns to production after each reset. The harm is an unlocated ground fault that can start a fire or energize metal, with the recurring alarm normalized instead of investigated.

### Connector or Bond Failure Creates a Series Arc

A field-installed or mismatched connector, or a corroded bond, develops high resistance and becomes a series arc source. The mechanism is that the poor connection arcs under current, generating heat that melts the connector or ignites material, undetected by overcurrent devices. The false signal is that the circuit produces power normally until the arc begins. The harm is a fire originating at a hidden connection under the array, exactly where arc-fault detection must catch it.

### GEC Tied to an Inadequate Electrode

The installer connects the GEC to a water pipe or rod that does not meet the electrode requirements, or fails to bond to the building's grounding electrode system. The mechanism is that the reference is poor, so fault currents do not clear effectively and ground-fault detection behaves erratically. The false signal is that the system references ground and the inverter starts. The harm is unreliable fault clearing and detection, and potential differences between the PV and building grounding systems during faults.

## Self-Check

- Did I distinguish equipment grounding, functional grounding, and the GEC, applying each to its correct purpose, connection point, and code requirement without conflating them?
- Did I size the GEC and the DC equipment grounding conductors per NEC 690 and Article 250 against the corrected and derated array current, and document the calculations?
- Did I bond module frames and racking with listed, corrosion-resistant devices compatible with anodized aluminum, torqued to specification, and verify continuity across the array?
- Did I specify, enable, and commission DC arc-fault detection per NEC 690.11, and verify its trip rather than leaving it disabled or suppressed after nuisance trips?
- Did I configure ground-fault detection and isolation to the correct sensitivity and alarm, with a process to investigate and repair faults rather than repeatedly reset them?
- Did I coordinate fault protection across the DC and AC sides so each fault is cleared by the closest device without nuisance tripping, and document the coordination scheme?
- Did I establish a maintenance plan that periodically verifies bonding continuity, tests arc-fault and ground-fault detection, and inspects connectors and bonds for corrosion?
- Does the output stay within the agent's scope, deferring licensed engineering, stamped design, AHJ approval, and specialist sign-off to the qualified person where the question exceeds the agent's competence?
