---
name: access-control-wiring-and-door-hardware.md
description: Use when the agent is wiring access control panels, door strikes, magnetic locks, request-to-exit devices, and readers, or deciding fail-safe versus fail-secure hardware, lock output supervision, fire alarm interface release, and NEC 725 Class 2 wiring separation for egress compliance.
---

# Access Control Wiring and Door Hardware

Access control looks like simple low-voltage wiring, but every controlled door is also a life-safety egress path, and the same hardware that keeps a door locked against intrusion can trap occupants against egress during a fire. The judgment problem is that an installer who treats a door strike or mag lock as just another relay output will produce a system that authenticates correctly but fails the egress, fire alarm release, and supervision requirements that make it legal and safe. The fail-safe versus fail-secure choice, the request-to-exit wiring, the fire alarm interface, and the Class 2 wiring separation are not afterthoughts; they are the decisions that determine whether the door releases when power fails and whether the system is code-compliant. This skill covers the wiring, hardware selection, and interface decisions that determine whether an access control installation is both functional and safe.

## Core Rules

### Choose Fail-Safe or Fail-Secure Hardware for Each Opening

The fail-safe versus fail-secure choice is the foundational decision for every controlled opening, and it must be made per door based on the use of the space, not as a building-wide default. A fail-safe device unlocks when power is removed; this includes electromagnetic locks (mag locks), which depend on continuous power to hold, and fail-safe strikes. A fail-secure device locks when power is removed; this includes most electric strikes and electromechanical locks, which require power only to unlock. Egress doors, stairwells used in evacuation, and any door on a required means of egress must release on loss of power and on fire alarm, which usually dictates fail-safe hardware or a fail-safe release path. Server rooms and secure storage may use fail-secure hardware because the contents must stay locked during a power event, but only where egress is not compromised. The choice must be documented and consistent with the authority having jurisdiction and the local building code.

The trap is defaulting every door to fail-secure for security. The defense is to evaluate each opening against its egress role, to use fail-safe hardware or a release path on means of egress, and to document the choice per door.

### Wire Lock Outputs as Supervised Circuits, Not Bare Relay Contacts

Access control lock outputs must be supervised so that a cut wire or a shorted line is reported as a trouble condition, rather than silently disabling the lock or holding it unlocked. Supervision uses an end-of-line resistor at the lock so the panel can distinguish a normal line, an open, and a short. A bare relay contact driving a strike with no supervision will fail invisibly: a cut conductor leaves the door unlocked or locked depending on the wiring, with no alarm, until someone tests it. Power supervision modules and listed power transfer devices provide the supervision where the panel cannot directly supervise the load. The supervision also extends to the power supply feeding the lock, which should report a loss of output.

The trap is treating the lock output as a simple relay. The defense is to supervise lock outputs with end-of-line devices or listed modules, and to verify that a cut or short produces a trouble signal at the panel.

### Provide a Compliant Request-to-Exit and Egress Path

Every controlled door on a means of egress must allow free egress without depending on the access control system, which is achieved with request-to-exit (REX) devices such as motion sensors, push bars, and exit switches. The egress path must not require a credential, a fob, or any action that could fail during an emergency. Mag locks in particular require a listed release mechanism and often a second independent means of egress, because a mag lock holds the door with hundreds of pounds of force and a single failed sensor can trap an occupant. The REX device must be positioned so that a person approaching the door from the inside reliably triggers it, and the wiring must be arranged so that loss of the REX does not leave the door locked. Local codes may require a manual release with signage for mag-locked doors.

The trap is relying on a single REX sensor for a mag lock. The defense is to provide a listed and code-compliant egress path, to use redundant release methods for mag locks where required, and to verify egress works with the access panel powered down.

### Interface the Fire Alarm System to Release Doors on Alarm

Controlled doors on means of egress must release on a fire alarm signal so that occupants can evacuate, and this interface is a life-safety function that must be wired and tested to the fire alarm code. The release is typically achieved through a fire alarm relay that drops power to the fail-safe locks, or through a signal to the access panel commanding all doors to unlock. The interface must be fail-safe in the sense that loss of the fire alarm interface or loss of power releases the doors. The wiring must be documented, and the release must be tested as part of fire alarm acceptance. Coordinating with the fire alarm installer is essential, because the access system and the fire system must agree on which doors release, on what signal, and under what conditions.

The trap is assuming the access panel will handle the fire release in software. The defense is to wire a hardware fire alarm release to fail-safe locks, to coordinate with the fire alarm installer, and to test the release during acceptance.

### Separate Class 2 Wiring From Power and Fire Alarm Circuits

NEC Article 725 governs Class 2 and Class 3 circuits, which include most access control wiring, and the separation rules exist to prevent power and fire alarm energy from invading the low-voltage circuits. Class 2 wiring must be separated from power and Class 1 circuits by a physical barrier or by the spacing prescribed in the code, and it must not share a raceway with fire alarm or power wiring unless the separation is maintained by listed barriers. Mixing Class 2 access wiring with line voltage in the same conduit can induce voltage on the low-voltage conductors, damage the access panel, and create a fire hazard. The power supply for the locks is often Class 2 as well, but a higher-voltage lock supply may be Class 1, and the two must not be combined without separation.

The trap is pulling access wiring in the same conduit as power to save a run. The defense is to keep Class 2 wiring in its own raceway or to maintain code-compliant separation, and to verify the class of every circuit before combining conductors.

### Size Lock Power Supplies for Inrush and Simultaneous Operation

Electric strikes, mag locks, and electrified panic hardware draw inrush current at the moment of actuation, and a power supply sized only for the holding current will sag or fail when multiple locks actuate at once. The supply must be sized for the sum of the holding currents plus the inrush margin, and for the simultaneous operation that occurs during a fire alarm release when many doors unlock together. Battery backup must be considered where the system must remain operational during a power loss, but note that fail-safe locks release on power loss by design, so the battery supports the readers and the panel, not necessarily the locks. Voltage drop on the lock wiring must be checked, because a strike that receives too little voltage chatters or fails to unlock.

The trap is sizing the supply for holding current only. The defense is to size for inrush and simultaneous operation, to check voltage drop on long lock runs, and to provide battery backup for the panel and readers where required.

## Common Traps

### Defaulting Every Door to Fail-Secure and Trapping Egress

The installer configures all strikes as fail-secure for uniform security, including a door on a required means of egress. The mechanism of the trap is that a fail-secure door stays locked when power is removed, so during a fire or power loss the door remains locked and occupants cannot exit, which directly violates the egress code. The false signal is that the door authenticates and locks correctly during normal testing, which proves access control function but not egress compliance. The harm is a life-safety violation that can trap occupants during an emergency and that fails inspection. The defense is to evaluate each door against its egress role and to use fail-safe hardware or a release path on means of egress.

### Mag Lock With a Single REX Sensor and No Backup Release

The installer wires a mag lock with one infrared request-to-exit sensor and no independent manual release, trusting the sensor to release the door. The mechanism of the trap is that a mag lock holds with high force and a single sensor can fail, be blocked, or be defeated, leaving the door held shut with no alternative release, which the code typically forbids without a second means. The false signal is that the sensor releases the door reliably during walk-up testing, which proves the sensor works but not that the egress path is redundant. The harm is an occupant trapped at a held door when the sensor fails. The defense is to provide a listed second release, such as a push bar or manual release with signage, and to verify egress with the sensor disabled.

### Fire Alarm Release Implemented in Access Panel Software Only

The installer programs the access panel to unlock doors on a fire alarm input but does not wire a hardware release, relying on the panel software and its power. The mechanism of the trap is that if the access panel crashes, loses power, or hangs, the software release never executes and the doors stay locked during the fire, defeating the life-safety function. The false signal is that the doors unlock during a test of the software command, which proves the logic but not a fail-safe hardware path. The harm is doors that fail to release during a real fire. The defense is to wire a hardware fire alarm release to fail-safe locks and to test it with the access panel powered down.

### Lock Output Wired as a Bare Relay With No Supervision

The installer runs a relay contact directly to the strike with no end-of-line supervision, treating the lock like any switched load. The mechanism of the trap is that an unsupervised lock line fails silently, so a cut wire can leave the door unlocked for days with no trouble signal, or a short can hold it locked, and the panel reports nothing until someone physically tests the door. The false signal is that the door locks and unlocks on command, which proves the relay works but not that the line is supervised. The harm is a security breach or an egress failure that goes undetected. The defense is to supervise lock outputs with end-of-line devices or listed modules and to verify fault reporting.

### Class 2 Access Wiring Pulled in the Same Conduit as Power

The installer pulls the reader and lock wiring in the same conduit as line voltage to save conduit runs. The mechanism of the trap is that NEC 725 prohibits mixing Class 2 with power and Class 1 circuits without separation, and the induced voltage and shared raceway can damage the access panel, corrupt reader data, and create a fire hazard. The false signal is that the system works initially, which proves the wiring conducts but not that it is code-compliant. The harm is equipment damage, unreliable operation, and a code violation that fails inspection. The defense is to keep Class 2 wiring in its own raceway or to maintain listed separation and to verify the circuit class of every conductor.

### Power Supply Sized for Holding Current and Sagging on Release

The installer sizes the lock power supply for the holding current of each lock and ignores inrush and simultaneous release. The mechanism of the trap is that strikes and mag locks draw a surge at actuation, and during a fire alarm release many locks actuate at once, so an undersized supply sags, chatters the strikes, or drops out, leaving some doors partially locked. The false signal is that a single door releases cleanly when tested alone, which proves the supply handles one lock but not the simultaneous load. The harm is doors that fail to release fully during an emergency. The defense is to size for inrush and simultaneous operation and to test a full building release.

## Self-Check

- Did I choose fail-safe or fail-secure hardware for each door based on its egress role, and did I use fail-safe hardware or a release path on every means of egress?
- Did I supervise lock outputs with end-of-line devices or listed modules, and did I verify that a cut or short produces a trouble signal at the panel?
- Did I provide a listed, code-compliant request-to-exit and egress path that works without the access system, including a second independent release for mag locks where required?
- Did I wire a hardware fire alarm release to fail-safe locks, coordinate with the fire alarm installer, and test the release with the access panel powered down?
- Did I keep Class 2 wiring separated from power and fire alarm circuits per NEC 725, and did I verify the class of every circuit before combining conductors?
- Did I size the lock power supply for inrush and simultaneous operation, check voltage drop on long runs, and provide battery backup for the panel and readers where required?
- Did I document the fail-safe versus fail-secure choice, the egress path, and the fire alarm interface for each door for the authority having jurisdiction?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
