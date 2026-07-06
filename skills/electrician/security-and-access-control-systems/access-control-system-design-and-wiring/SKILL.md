---
name: access-control-system-design-and-wiring.md
description: Use when the agent is laying out card reader doors, selecting credential readers and lock hardware, wiring request-to-exit and door position switches, sizing access power supplies and batteries, or interfacing access control with fire alarm and egress code requirements.
---

# Access Control System Design and Wiring

Access control sits at the intersection of life safety, physical security, and low-voltage electronics, and that intersection is where most of the harm happens. A door that locks people in during a fire, a maglock that releases on every power blip, a reader cable run alongside line voltage that picks up noise and false-reads credentials, or a battery that is too small to keep a door secured through an outage are all consequences of decisions made casually during layout and wiring. The judgment problem is that access control components are individually simple, which invites electricians to treat the system as a collection of doorbells and solenoids rather than as a supervised, code-constrained life-safety-adjacent system. An electrician who wires a fail-secure strike on an egress path, or who forgets the fire alarm release on a maglock, can trap occupants in a building during an emergency. This skill covers the decisions that determine whether an access system is secure, code-compliant, and survivable.

## Core Rules

### Select Fail-Safe Versus Fail-Secure Locking Based on Egress Path and Code

Locking hardware fails in a defined state when power is removed, and that state must match the door's life-safety role. Fail-safe hardware (electromagnetic locks, most electric strikes on egress doors) unlocks on power loss, so that occupants can always exit during a fire or power failure. Fail-secure hardware (most electric strikes on exterior or storage doors) locks on power loss, preserving security when power is lost. The decision is not a preference; it is dictated by the door's position on the egress path and by the authority having jurisdiction. Any door in a required means of egress must fail to a state that permits free egress, which almost always means fail-safe, and electromagnetic locks on egress paths carry additional requirements for redundant release methods.

The trap is selecting hardware by habit or by what is in the truck. The defense is to classify each door as egress-path, exterior, or secured-storage, to select fail-safe hardware on egress paths, and to document the fail state of every door so the fire marshal can verify it during acceptance.

### Provide Redundant Request-to-Exit and Egress That Meets Code

A request-to-exit (REX) device signals the controller that an occupant is leaving, which both shunts the door-forced alarm and, on maglock installations, releases the lock. REX devices are typically active infrared motion sensors above the door, supplemented by a manual push button. On egress doors with electromagnetic locks, model codes typically require two independent release methods, commonly the built-in REX motion sensor plus a clearly labeled manual release button mounted at the door, in addition to any fire alarm interface release. A single REX motion sensor as the sole release is rarely acceptable on a maglock-secured egress door.

The trap is relying on one motion sensor and treating the manual button as optional. The defense is to provide redundant egress on every maglock door, to mount the manual release at the required height and with clear signage, and to confirm the combination of release methods with the local code official before rough-in.

### Wire Credential Readers According to the Protocol and Distance

Credential readers communicate with controllers using either the legacy Wiegand protocol or the modern OSDP (Open Supervised Device Protocol). Wiegand is a one-way, unsupervised signal over multiple conductors, vulnerable to interception and limited in distance, and it cannot report a tamper or offline reader back to the controller. OSDP is a two-way, encrypted, supervised serial protocol that supports longer runs, tamper reporting, and secure channel communication, and it is increasingly required for higher-security applications. The wiring differs: Wiegand uses separate data-zero, data-one, and LED control conductors, while OSDP uses a two-wire RS-485 bus with polarity that must be observed.

The trap is pulling Wiegand cabling for every reader because it is familiar, then being unable to upgrade to OSDP without re-pulling. The defense is to pull cabling that supports OSDP (typically shielded twisted pair plus spare conductors) even when the initial reader is Wiegand, to observe polarity on OSDP runs, and to respect the distance limits of the chosen protocol.

### Treat the Door Position Switch as a Supervised Input, Not an Afterthought

The door position switch (DPS), usually a magnetic contact or balanced switch, tells the controller whether the door is open or closed, and it is the basis for door-forced-open and door-held-open alarms. A DPS wired as a simple dry contact into an unsupervised input can be defeated by a cut wire that the controller reads as a stable closed door, masking a forced entry. For supervised integrity, the DPS should land on a supervised input with an end-of-line resistor, so that a cut or short produces a distinct trouble state. The placement also matters: a surface contact on the door frame is easily defeated, while a concealed balanced switch is far harder to bypass.

The trap is landing the DPS on whatever input is free without supervision. The defense is to use supervised inputs with correctly placed end-of-line resistors for door position, to conceal switches where security level warrants, and to verify that a cut wire produces a trouble, not a silent normal.

### Size the Power Supply and Battery for Lock Inrush, Hold, and Standby

Access control power supplies must support the controller, the readers, the locking hardware, and the REX devices simultaneously, and locking hardware has a significant inrush current that can collapse an undersized supply. Battery standby capacity must keep the system operational for the code-required duration, typically a minimum of four hours for fire alarm systems and often specified by the owner for access systems. The calculation sums the steady-state current of all devices, adds the inrush of simultaneously actuated locks, and then sizes the battery to carry the standby load for the required hours plus a margin. A supply sized only for average current will drop out when several locks fire at once.

The trap is adding up nameplate hold currents and ignoring inrush and battery duty. The defense is to calculate worst-case simultaneous inrush, to size the supply with headroom, and to compute battery amp-hours for the required standby time, replacing the battery on the maintenance schedule before it ages below capacity.

### Locate Controllers for Security, homeruns, and Serviceability

Access controllers are the brains of the system and the point where all door wiring converges, so their placement affects both security and signal integrity. Controllers should be located in secured, environmentally controlled spaces, not in the plenum above the door they control, and each door's wiring should homerun to the controller rather than daisy-chain through adjacent doors. Homeruns isolate faults, simplify troubleshooting, and prevent a single cut cable from taking down multiple doors. Locating a controller above the door it secures also creates a security weakness, because an attacker who defeats that door has access to the controller for every door on that panel.

The trap is daisy-chaining door wiring to save cable or mounting the controller in the nearest ceiling. The defense is to homerun each door to a secured controller location, to keep controllers in locked enclosures in conditioned spaces, and to avoid placing a controller within the area of the door it controls.

### Interface With Fire Alarm for Coordinated Maglock Release

Any access-controlled door on a means of egress must release during a fire alarm so occupants can exit, and this release is achieved by a dedicated interface between the fire alarm system and the access control power. For electromagnetic locks, the fire alarm control unit typically drops power to the lock through a listed interface, independently of what the access controller does, so that a fire alarm signal overrides the access decision. The interface must be listed for the purpose, the wiring must be supervised where required, and the release must be tested during fire alarm acceptance. Relying on the access controller to interpret a fire alarm input and release the lock adds a software failure mode to a life-safety function.

The trap is wiring the fire alarm signal into a programmable input and trusting the logic. The defense is to use a listed hardware interface that drops lock power on alarm, to supervise the interface wiring, and to test the release under fire alarm acceptance with the access controller in every state.

## Common Traps

### Wiring a Fail-Secure Strike on an Egress Path

An electrician installs a fail-secure electric strike on an interior office door that is part of the means of egress, on the assumption that the door is normally unlocked during business hours. The mechanism of the trap is that fail-secure hardware locks on power loss, so a fire combined with a power failure leaves the door latched against occupants trying to exit, even if the lever hardware is free. The false signal is that the door operates normally under power and the mechanical exit device feels free, which proves daytime operation but not emergency egress. The harm is occupants trapped behind a door that failed secure during the precise emergency it was supposed to permit escape from, a direct life-safety violation. The defense is to classify each door against the egress path and to use fail-safe hardware wherever free egress must be maintained on power loss.

### Single REX Motion Sensor as the Only Maglock Release

An installer mounts one active-infrared REX motion sensor above a maglock-secured egress door and treats it as sufficient release, omitting the manual push button. The mechanism of the trap is that a single sensor is a single point of failure, and motion sensors can be blinded, misaligned, or fail to detect a slow-moving or non-moving occupant, leaving the maglock energized against someone trying to leave. The false signal is that the door releases when the installer waves a hand under the sensor, which proves the sensor works but not that it is redundant or code-compliant. The harm is a door that can trap an occupant when the one release method fails, plus a code violation that surfaces at final inspection. The defense is to provide redundant release, including a labeled manual button, and to verify the combination against the local egress code.

### Pulling Wiegand-Only Cabling That Cannot Support OSDP

To save a few dollars per run, the installer pulls four-conductor unshielded cable adequate only for legacy Wiegand readers. The mechanism of the trap is that OSDP requires a shielded twisted pair for reliable RS-485 communication, and an unshielded four-conductor run will not support a future OSDP upgrade without a re-pull, which costs far more than the initial savings. The false signal is that the Wiegand reader benches out fine on the installed cable, which proves today's install but strangles tomorrow's security upgrade. The harm is a system locked into an unsupervised, interceptable protocol because the cabling decision was made once and cheaply. The defense is to pull shielded twisted pair with spare conductors on every reader run regardless of the initial protocol.

### Unsolicited Door Position Input That Masks a Cut Wire

The DPS lands on an unsupervised dry-contact input with no end-of-line resistor. The mechanism of the trap is that an unsupervised input cannot distinguish a closed door from a cut wire, because both present the same open-or-closed state to the controller, so an attacker who cuts the DPS lead can force the door and the controller will read a stable closed door with no alarm. The false signal is that opening the door produces a clean state change at the controller, which proves the switch works but not that the wiring is supervised against defeat. The harm is a door-forced alarm that never fires because the supervision was never there, defeating the security purpose of the DPS. The defense is to land door position on supervised inputs with correctly placed end-of-line resistors and to verify that a cut produces a trouble.

### Power Supply Sized to Hold Current With No Inrush or Battery Margin

The installer sums the nameplate hold currents of the locks and readers, picks a supply whose rating matches that sum, and installs a battery sized only to buffer momentary outages. The mechanism of the trap is that electric locks draw several times their hold current for the first tens of milliseconds of actuation, and when several locks fire at once, the supply collapses and the controller resets, dropping all doors to their fail state. The battery, sized without a real standby calculation, also falls short of the code-required four hours. The false signal is that the system works when one door is tested at a time, which never exercises the simultaneous inrush or the standby duration. The harm is a system that resets during busy periods and fails to meet standby requirements, both a nuisance and a code issue. The defense is to calculate worst-case simultaneous inrush, to size the supply with headroom, and to compute battery amp-hours for the required standby.

### Controller Mounted in the Ceiling Above Its Own Door

To minimize wire runs, the installer mounts the access controller in a backbox in the ceiling directly above the door it controls. The mechanism of the trap is that a controller located within the secured area of its own door is reachable by anyone who defeats that door, giving them command of every other door on the same panel, and a ceiling location also exposes the electronics to heat and to unauthorized access by anyone with a ladder. The false signal is that the wiring is short and tidy, which looks efficient but concentrates risk. The harm is a single door defeat that cascades to the whole system, plus environmental failures from the unconditioned location. The defense is to homerun each door to a controller in a locked, conditioned, secured space outside the area of the doors it controls.

### Trusting Software Logic for Fire Alarm Maglock Release

The fire alarm dry contact lands on a programmable input of the access controller, and the installer relies on the controller's programming to drop the maglock on alarm. The mechanism of the trap is that this adds a software dependency to a life-safety release, so a firmware bug, a logic error, or a controller hang can leave the maglock energized during a fire. The false signal is that the lock releases when the input is tested, which proves the logic works today but not that it is failsafe. The harm is an egress door that stays locked during a fire because the software path failed. The defense is to use a listed hardware interface that physically interrupts lock power on alarm, supervised and independent of the controller's logic.

## Self-Check

- Did I classify each door as egress-path, exterior, or secured-storage, and select fail-safe hardware on every door in a required means of egress so it unlocks on power loss?
- On every maglock-secured egress door, did I provide at least two independent release methods, including a labeled manual push button, and verify the combination against the local code?
- Did I pull reader cabling that supports OSDP (shielded twisted pair with spare conductors) even where the initial reader is Wiegand, and did I observe polarity on OSDP runs?
- Did I land door position switches on supervised inputs with correctly placed end-of-line resistors, and did I verify that a cut wire produces a trouble rather than a silent normal?
- Did I calculate worst-case simultaneous lock inrush, size the power supply with headroom, and compute battery amp-hours for the code-required standby duration (typically four hours minimum)?
- Did I homerun each door to a controller located in a locked, conditioned, secured space outside the area of the doors it controls, rather than daisy-chaining or ceiling-mounting?
- Did I interface fire alarm release through a listed hardware interface that physically interrupts maglock power, supervised and independent of controller software logic, and test it during acceptance?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
