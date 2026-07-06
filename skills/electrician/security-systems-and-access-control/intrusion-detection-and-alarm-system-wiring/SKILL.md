---
name: intrusion-detection-and-alarm-system-wiring.md
description: Use when the agent is wiring burglar alarm panels, door contacts, motion detectors, glass break sensors, and communication paths, configuring zone wiring with end-of-line resistors, supervised circuits, sensor placement, cellular or IP communication, and partition arming.
---

# Intrusion Detection and Alarm System Wiring

A burglar alarm is only as reliable as its zone wiring and its communication path, and both are easy to get superficially right while being fundamentally wrong. The judgment problem is that an installer who wires a door contact as a simple open-or-closed loop, who places a motion detector for convenience, and who trusts a single phone line for central station reporting will build a system that arms and disarms cleanly but that a knowledgeable intruder can defeat with a jumper, that false-alarms on every heating vent, and that fails to report when the phone line is cut. The end-of-line resistor supervision, the sensor placement, the dual-technology selection, and the redundant communication path are the decisions that determine whether the system actually detects intrusion and reports it. This skill covers the zone wiring, supervision, sensor, and communication decisions that determine whether an intrusion system is a real deterrent or theater.

## Core Rules

### Wire Zones as Supervised Circuits With End-of-Line Resistors

Every intrusion zone must be wired as a supervised circuit with an end-of-line (EOL) resistor installed at the last device, not at the panel, so that the panel can distinguish a normal closed loop, a cut wire (open), and a shorted wire. The EOL resistor value is set by the panel manufacturer and must be installed at the farthest device on the loop, because installing it at the panel defeats the supervision: an intruder can short the loop at the panel and the panel sees the resistor. Double end-of-line wiring adds a second resistor to report both the normal state and the sensor state separately, increasing the information the panel has. A zone wired without an EOL, or with the EOL at the panel, is unsupervised and can be defeated by a jumper across the conductors anywhere along the run.

The trap is installing the EOL resistor at the panel for convenience. The defense is to install the EOL resistor at the last device on every loop, to use the manufacturer's value, and to verify that a cut and a short each produce the correct panel response.

### Place Sensors Based on Detection Geometry, Not Mounting Convenience

Sensor placement determines what the system detects, and each sensor type has a detection geometry that must be matched to the target and the environment. Door and window contacts detect opening and must be placed at the opening edge where the magnet and switch align. Passive infrared (PIR) motion detectors sense moving heat and must be aimed across the likely path of travel, not toward it, because a PIR detects lateral motion across its fingers better than motion directly toward it. Glass break detectors listen for the specific frequency of breaking glass and must be within line of sight and range of the protected glass, with the blinds and drapes considered. Placement must also avoid sources of false alarms: PIRs aimed at heating vents, sunlight, or animals, and glass breaks near sources of sharp noise. Dual-technology sensors combining PIR and microwave reduce false alarms by requiring both technologies to trigger.

The trap is mounting sensors where the cable is easy to run. The defense is to place each sensor per its detection geometry, to aim motion detectors across the path of travel, and to avoid heat, sunlight, and noise sources that cause false alarms.

### Select the Right Sensor Technology for the Threat and Environment

Sensor technology must match the threat and the environment, because the wrong sensor either misses real intrusion or false-alarms constantly. Contacts are reliable for doors and windows that open but do not detect glass breakage, so a window with a contact can be defeated by breaking the glass, which is why glass break or shock sensors supplement contacts on vulnerable glazing. Motion detectors cover interior spaces but require the space to be unoccupied when armed, which makes them unsuitable for 24-hour occupancy without partitioning. Vibration and shock sensors detect forced entry through walls and safes but must be tuned to ignore normal building vibration. Environmental conditions matter: dusty or insect-prone areas cause PIR false alarms unless the sensor is sealed, and temperature extremes affect sensor range. The selection should consider the value of the asset, the occupancy pattern, and the environment.

The trap is using one sensor type everywhere. The defense is to select sensor technology per opening and space, to combine contacts with glass break on vulnerable glazing, and to use dual-technology sensors where false alarms are a risk.

### Provide a Supervised and Redundant Communication Path

The communication path is how the alarm reaches the central station, and a single unmonitored path is a single point of failure that an intruder can cut before entering. Traditional phone lines can be cut at the demarc, defeating the report, so modern systems use supervised communication paths that report the loss of the path as a trouble signal. Cellular communicators, IP communicators, and long-range radio provide paths independent of the building's physical phone line, and the highest reliability comes from two diverse paths, such as cellular plus IP, so that cutting one does not disable reporting. The communication path must be supervised so that the panel reports a lost path within the required time, and the path must have its own backup power at the communicator. The choice of path affects the monitoring contract and the code compliance for the application.

The trap is trusting a single phone line. The defense is to use a supervised communication path, to provide a second diverse path for high-risk applications, and to verify that a lost path produces a trouble signal.

### Configure Partitions and Arming Modes for the Occupancy Pattern

Partitions divide a system into independently armed areas, which is essential for buildings with mixed occupancy or 24-hour operation, such as a retail store with a stockroom or an office with a server room. Without partitions, arming the whole building forces everyone out, or leaving one area disarmed leaves everything disarmed. Partitioning allows the office to be armed while the server room stays armed separately, or the showroom to be disarmed during business hours while the stockroom stays armed. Arming modes, stay and away, control whether interior motion detectors are active: stay mode arms perimeter sensors but bypass interior motions so occupants can move inside, while away mode arms everything. The partition and arming configuration must match how the building is actually used, or the system will be bypassed or left disarmed by frustrated users.

The trap is configuring one big area with no partitions. The defense is to define partitions that match the occupancy pattern, to set stay and away modes correctly, and to verify that each partition arms and reports independently.

### Provide Backup Power and Test the Full Reporting Chain

An intrusion panel must operate on loss of normal power, which requires a backup battery sized for the standby period required by the application. The battery must power the panel, the keypads, the sensors, and the communicator for the required duration, and a battery sized only for the panel will collapse when the sirens and communicator activate. The full reporting chain, from sensor trip to central station receipt, must be tested end to end during commissioning, because a system that arms and sounds the local siren may still fail to reach the central station if the communicator is misconfigured or the path is down. Regular test signals verify that the path remains live, and many jurisdictions and insurance policies require periodic test reporting.

The trap is testing the siren and skipping the central station test. The defense is to size the battery for the full standby and alarm load, to test the complete reporting chain to the central station, and to schedule periodic test signals.

## Common Traps

### End-of-Line Resistor Installed at the Panel

The installer mounts the EOL resistor on the panel terminals for speed, leaving the loop unsupervised against a short. The mechanism of the trap is that supervision depends on the resistor being at the end of the loop, and a panel-mounted resistor means an intruder can short the loop anywhere and the panel still sees the resistor, so the zone never reports the short as an alarm. The false signal is that the zone arms and disarms normally and trips when a contact opens, which proves the loop works but not that it is supervised. The harm is a system defeatable by a jumper. The defense is to install the EOL resistor at the last device and to test both cut and short responses.

### Motion Detector Aimed Along the Path of Travel

The installer aims a PIR motion detector straight down a corridor toward the likely intruder path. The mechanism of the trap is that a PIR detects motion across its detection fingers far better than motion directly toward or away from it, so a detector aimed along the path sees a reduced signal and may miss a slow approach, while also being more prone to false alarms from heat sources in the field. The false signal is that walking down the corridor triggers it during testing, which proves detection at a brisk walk but not at a slow creep. The harm is missed intrusion and false alarms. The defense is to aim motion detectors across the path of travel and to verify detection at slow speeds.

### Glass Window With a Contact but No Glass Break Sensor

The installer puts a contact on a window and considers it protected, ignoring that the glass can be broken to gain entry without opening the sash. The mechanism of the trap is that a contact detects only opening, so an intruder who breaks the glass and reaches through to unlock the window, or who simply climbs through the broken pane, never opens the contact and never triggers the alarm. The false signal is that opening the window trips the alarm during testing, which proves the contact works but not that the glass is protected. The harm is an undetected entry through broken glass. The defense is to supplement contacts on vulnerable glazing with glass break or shock sensors.

### Single Phone Line as the Only Communication Path

The installer connects the panel to the building's phone line and considers reporting complete. The mechanism of the trap is that a phone line is physically accessible at the demarc and can be cut before entry, and an unsupervised line cut produces no trouble signal, so the panel cannot report the intrusion and the central station never knows. The false signal is that a test signal reaches the central station during commissioning, which proves the path works but not that it is supervised or redundant. The harm is a silent system that never reports a real intrusion. The defense is to use a supervised cellular or IP path and to add a second diverse path for high-risk sites.

### Battery Sized for the Panel Only

The installer sizes the backup battery for the panel and keypads and ignores the siren and communicator load. The mechanism of the trap is that the battery must support the full alarm load including the siren and the repeated communicator transmissions, and an undersized battery holds the panel up in standby but collapses when the alarm activates, killing the siren and the report partway through. The false signal is that the panel runs on battery during a short standby test, which never exercises the alarm load. The harm is an alarm that dies before it reports. The defense is to size the battery for standby plus full alarm load and to test under alarm conditions.

### One Big Area With No Partitions Forces Bypassing

The installer programs the whole building as a single area with no partitions. The mechanism of the trap is that a single-area system cannot be selectively armed, so users in a 24-hour space must either force everyone out to arm it or leave it disarmed, and frustrated users begin bypassing zones or leaving the system off, defeating the protection. The false signal is that the system arms and disarms cleanly during commissioning, which proves function but not usability. The harm is a system that is routinely disarmed or bypassed in real use. The defense is to define partitions matching the occupancy and to set stay and away modes correctly.

## Self-Check

- Did I install the end-of-line resistor at the last device on every zone, use the manufacturer's value, and verify that both a cut and a short produce the correct panel response?
- Did I place each sensor per its detection geometry, aim motion detectors across the path of travel, and avoid heat, sunlight, and noise sources that cause false alarms?
- Did I select sensor technology per opening and space, combine contacts with glass break on vulnerable glazing, and use dual-technology sensors where false alarms are a risk?
- Did I use a supervised communication path, provide a second diverse path for high-risk applications, and verify that a lost path produces a trouble signal?
- Did I define partitions that match the occupancy pattern, set stay and away arming modes correctly, and verify that each partition arms and reports independently?
- Did I size the backup battery for the full standby and alarm load, test the complete reporting chain from sensor trip to central station receipt, and schedule periodic test signals?
- Did I document the zone list, sensor types, communication paths, and partition configuration for the user and the monitoring provider?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
