---
name: fire-alarm-and-detection-system-design.md
description: Use when the agent is designing or reviewing fire alarm and detection systems, selecting initiating devices, notification appliances and circuits, pathways, survivability, smoke and heat detection spacing, ADA strobe coverage, or off-premises monitoring under NFPA 72.
---

# Fire Alarm and Detection System Design

A fire alarm system is a life-safety device whose function is to detect, to notify, and to communicate, and each of those functions depends on design decisions that are easy to make incorrectly because the system appears to work during testing. The civil engineer designing detection and notification under NFPA 72 is responsible for a system that will detect the fire at the right stage, notify all occupants including those with sensory disabilities, and survive the fire long enough to perform. The persistent failure is treating device placement as a coverage exercise and the notification circuit as a wiring exercise, when the detection spacing is driven by airflow and ceiling geometry and the notification is driven by audibility and visibility calculations against ambient conditions. Smoke detection spacing, heat detector response, and notification appliance circuit (NAC) design each have specific performance requirements, and a system that "sounds during the test" may not be audible over a building's mechanical noise or visible around a partition. This skill forces the decisions that determine whether the system performs in a fire: correct device selection and spacing, correct notification design, pathway survivability for the hazard, and a monitoring and inspection regime that keeps the system credible.

## Core Rules

### Select and Space Initiating Devices for the Hazard and Geometry

Initiating devices—smoke detectors, heat detectors, sprinkler waterflow switches, manual pull stations—must be selected for the expected fire signature and spaced for the ceiling geometry per NFPA 72. Smoke detector spacing is based on the assumed smoke layer and the airflow, with a nominal 30-foot spacing for smooth ceilings, but the engineer must reduce spacing for beams, joists, and solid barriers that interrupt the flow of smoke to the detector. Heat detector spacing is based on the RTI (response time index) and the listed spacing, and the engineer must apply the spacing reductions for ceiling height and for the expected fire growth rate, because a heat detector at full listed spacing on a high ceiling may respond too slowly to meet the design objective. The engineer must verify that waterflow switches on sprinkler systems are connected to the alarm and that the retard is set correctly, because a waterflow alarm is the primary initiating device in a sprinklered building. Manual pull stations must be located at exits and travel distances per the code, and the engineer must not omit them on the assumption that automatic detection suffices, because the code requires manual initiation for most occupancies.

### Design Notification for Audibility and Visibility Against Ambient Conditions

Notification appliances—horns, speakers, strobes—must produce a signal that is audible and visible to all occupants, and the engineer must calculate the sound pressure level and the illumination rather than rely on device count. For audibility, NFPA 72 requires the alarm to be at least 15 dBA above the average ambient sound level (or 5 dBA above the maximum, whichever is greater) throughout the occupiable space, and the engineer must use the actual ambient levels, including mechanical equipment, not a default. For visible notification, the strobe coverage must meet the candela ratings and spacing for the room size, with wall-mounted strobes at the required heights and spacing, and the engineer must ensure that strobes are visible from any point in the space, accounting for partitions and furniture. ADA and NFPA 72 require strobes in restrooms, common areas, and accessible spaces, and the engineer must not treat strobe placement as a code minimum but as a coverage calculation, because a strobe behind a partition does not notify. The notification appliance circuit must be sized so that the voltage drop at the last device still produces the rated output, and the engineer must calculate the circuit loading rather than assume a number of devices per circuit.

### Specify Pathway Survivability for the Building Hazard

The wiring pathways connecting devices to the control panel must survive the fire for the time required to perform their function, and NFPA 72 defines pathway classes (Class A, Class B, Class X, and equivalents) and survivability levels that depend on the building construction and the system role. The engineer must specify the pathway class based on the redundancy required: a Class A pathway continues to operate with a single fault, while a Class B pathway does not, and the choice depends on the reliability objective and the building code requirement. For pathways that must survive in a fire (for example, in-building fire emergency voice/alarm communications in high-rise buildings), the engineer must specify the survivability method—two-hour fire-rated cable, a two-hour fire-rated enclosure, or an inherently survivable pathway per the building construction—and must verify that the method is listed and installed correctly. The engineer must not route pathways through unprotected combustible concealed spaces without survivability treatment, because a fire in such a space can sever the pathway and silence the notification before occupants are warned.

### Establish Monitoring, Inspection, and Testing for System Credibility

A fire alarm system that is not monitored and not inspected is a system whose reliability is unknown, and the engineer must specify off-premises monitoring (supervising station) per NFPA 72 with a listed transmission method and a defined response. The monitoring connection must be supervised so that a loss of the communication path is annunciated, and the engineer must verify the transmission technology meets the current NFPA 72 requirements, which have moved away from legacy phone lines. Inspection, testing, and maintenance must be specified per NFPA 72 Chapter 14, with the devices tested on the required schedule (semiannual for most, with sensitivity testing and cleaning on the documented schedule), and the engineer must require that the records be maintained. The engineer should treat a system without a defined inspection and testing program as incomplete, because an untested detector may be drifted out of sensitivity and a dead battery in a secondary supply will not be discovered until the power fails.

## Common Traps

### Spacing Smoke Detectors on Smooth-Ceiling Nominal Without Beam Reductions

The false signal is the nominal 30-foot spacing applied; the mechanism is that beams and joists interrupt the smoke flow, and the effective spacing must be reduced per the NFPA 72 beam rules. The harm is delayed or missed detection, because smoke never reaches a detector shielded by the ceiling geometry.

### Sizing Notification on Device Count Without dBA and Candela Calculations

The false signal is that horns and strobes are present; the mechanism is that audibility depends on ambient noise and circuit voltage drop, and visibility depends on room size and candela, none of which is captured by a device count. The harm is an alarm that is inaudible over mechanical noise or invisible around a partition, because the notification was never engineered.

### Routing Pathways Through Unprotected Combustible Concealed Spaces

The false signal is that the wiring is connected and tests good; the mechanism is that a fire in a combustible concealed space severs an unsupervised or non-survivable pathway before notification completes. The harm is a system that is silent during the fire, because survivability was not specified for the hazard.

### Relying on Legacy Phone Line Monitoring

The false signal is that the system is monitored; the mechanism is that legacy telephone lines are being phased out and do not meet current NFPA 72 transmission technology requirements, and the path is unsupervised or single-point. The harm is a monitoring failure that goes undetected until the alarm fails to transmit, because the transmission method was never updated.

## Self-Check

- Are initiating devices selected for the fire signature and spaced for the ceiling geometry, with smoke detector spacing reduced for beams and joists and heat detector spacing adjusted for ceiling height and fire growth?
- Is notification designed with audibility at least 15 dBA above average ambient (or 5 dBA above maximum) and visible strobe coverage calculated for room size, candela, and obstructions including ADA-required spaces?
- Is the notification appliance circuit voltage drop calculated so the last device receives adequate voltage, with circuit loading verified rather than assumed by device count?
- Are pathway classes (A, B, X) specified for the required redundancy, and is survivability (rated cable, rated enclosure, or inherent) specified for pathways that must operate during a fire?
- Is off-premises monitoring specified with a current NFPA 72-compliant supervised transmission method, not legacy unsupervised phone lines?
- Is an inspection, testing, and maintenance program specified per NFPA 72 Chapter 14, with device sensitivity, secondary supply, and transmission path tested on the required schedule and records maintained?
