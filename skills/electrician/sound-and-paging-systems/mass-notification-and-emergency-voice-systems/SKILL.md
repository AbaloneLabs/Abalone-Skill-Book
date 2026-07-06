---
name: mass-notification-and-emergency-voice-systems.md
description: Use when the agent is designing or wiring emergency voice alarm communication systems (EVACS) or mass notification systems, addressing NFPA 72 speaker coverage for intelligibility, survivability and pathway class, redundant power, or distinguishing mass notification from fire alarm voice evacuation.
---

# Mass Notification and Emergency Voice Systems

Emergency voice systems are the one class of sound system where a failure can kill people, because the message they carry is the instruction to evacuate, shelter, or take cover, and a message that is audible but not understood is the same as no message at all. The judgment problem is that these systems look like paging systems, which invites electricians to apply paging rules, when in fact they are governed by NFPA 72, they must achieve intelligibility rather than mere audibility, their wiring must survive the emergency long enough to deliver the message, and they must do so on redundant power after the normal power fails. An installer who spaces speakers for audibility, who routes wiring without survivability, or who confuses mass notification with fire alarm voice evacuation will build a system that tests loud and fails when it matters. This skill covers the decisions that determine whether an emergency voice system is intelligible, survivable, and correctly scoped.

## Core Rules

### Design for Intelligibility, Not Just Audibility

NFPA 72 emergency voice alarm communication systems (EVACS) require that voice messages be intelligible, meaning understandable, not merely audible. Audibility is a measure of sound pressure level; intelligibility is a measure of whether the words can be understood, and the two are not the same. A speaker system can produce a loud, reverberant wash of sound that registers on a meter but is unintelligible in a hard-surfaced lobby or a long corridor. Intelligibility is measured by metrics such as the Speech Transmission Index (STI) or the Common Intelligibility Scale (CIS), and design for intelligibility requires controlling reverberation, overlapping speaker coverage to avoid echoes, and using more, lower-power speakers rather than fewer, louder ones in reverberant spaces.

The trap is designing to an audibility target and declaring success. The defense is to design for measured intelligibility using more speakers at lower levels in reverberant spaces, to verify with STI or CIS measurements, and to treat audibility as necessary but not sufficient.

### Apply the Correct Survivability and Pathway Class to the Wiring

Emergency voice system wiring must survive the emergency long enough to deliver the message, and NFPA 72 defines pathway classes that specify the level of protection. Pathway class X provides performance under fault conditions including a short, class N addresses network pathways, class C provides a pathway with integral supervision but not physical protection, and class W addresses survivability of the pathway through a fire. Two-hour rated survivability is required for certain applications, achieved by listed cable in listed raceway, by routing in a two-hour rated assembly, or by listed cable with a two-hour fire-resistance rating. The pathway class must be identified during design and verified during installation, because a class C pathway routed through an unprotected space fails the survivability requirement even though it is electrically supervised.

The trap is treating any supervised pathway as adequate. The defense is to identify the required pathway class for each circuit, to achieve survivability with listed cable or rated assemblies where required, and to verify the installed pathway meets the class before acceptance.

### Provide Redundant Power Sized for Standby and Alarm Duration

Emergency voice systems must operate on loss of normal power, which requires a secondary power supply sized for both standby and alarm duration. NFPA 72 specifies the required durations, typically 24 hours of standby plus a defined alarm period for fire alarm systems, and the battery calculation must sum the standby load of all components and the alarm load of all speakers and notification appliances for their respective durations. A battery sized only for standby will exhaust before the alarm message is delivered, and a battery sized without the speaker load will collapse when the system goes into alarm. Generators may serve as the secondary supply but introduce their own start time and transfer considerations.

The trap is sizing the battery for standby only. The defense is to compute battery capacity for both standby and alarm durations, to include the full alarm load of speakers and appliances, and to replace batteries on a schedule before aging reduces them below capacity.

### Distinguish Mass Notification From Fire Alarm Voice Evacuation

Mass notification systems (MNS) and fire alarm voice evacuation systems are related but distinct, and confusing them leads to both scope and code errors. A fire alarm voice evacuation system delivers evacuation instructions in response to a fire, and its priority and operation are governed by the fire alarm code. A mass notification system delivers a range of emergency messages, weather, lockdown, shelter-in-place, evacuation, and may integrate with or override the fire alarm system. The integration rules are specific: an MNS may override a fire alarm evacuation signal only under defined conditions, and the priority and conflict resolution between the two systems must be designed and documented. Treating an MMS as just another paging system, or treating a fire alarm voice system as if it can carry any message, violates the code governing each.

The trap is assuming voice is voice and any message can go over any system. The defense is to distinguish the systems by their governing code and purpose, to design the integration and priority rules explicitly, and to document the conflict resolution for the authority having jurisdiction.

### Coordinate Speaker Coverage With Notification Appliance Circuits

In a building with both emergency voice speakers and fire alarm visible notification appliances, the two must be coordinated so that the coverage, the circuiting, and the activation sequence are consistent. Speakers and strobes are often mounted at the same location but on separate circuits, because voice and visible circuits have different supervision and survivability requirements, and the spacing that achieves intelligibility for voice may differ from the spacing that achieves the required candela coverage for visible appliances. The design must reconcile the two coverage plans so that a person in any location both hears the voice message and sees the visible signal, and the circuits must be wired so that a fault on one does not disable the other.

The trap is laying out speakers and strobes independently. The defense is to coordinate the two coverage plans, to circuit voice and visible separately per their requirements, and to verify that both cover every required location.

### Supervise the System End-to-End and Verify on Acceptance

Emergency voice systems are supervised, meaning that a fault on any circuit produces a trouble signal at the control panel, and the supervision must extend from the panel through the wiring to the end of line. Speaker circuits are supervised for opens and shorts, and the supervision must be verified during acceptance by introducing a fault at the end of line and confirming the panel reports it. A system that operates correctly but is not supervised can lose a circuit and not know it, delivering silence to a zone during an emergency. Acceptance testing must exercise every circuit, every message, and every fault condition, and the results must be documented for the authority having jurisdiction.

The trap is testing that the speakers work and skipping the supervision test. The defense is to introduce end-of-line faults on every circuit during acceptance, to confirm trouble signals at the panel, and to document the results.

### Use Listed Equipment and Compatible System Components

Emergency voice and mass notification components, speakers, amplifiers, control panels, and power supplies, must be listed for their purpose and compatible as a system. A non-listed amplifier or speaker on an EVACS voids the system listing and the authority having jurisdiction's acceptance. Compatibility matters because the supervision and signaling depend on the components working together as designed, and a substitute component can alter the electrical characteristics that the supervision relies on. Substituting a similar-looking but non-listed part to save cost or speed a delivery causes the entire system to fail acceptance and lose its life-safety certification.

The trap is substituting a similar-looking part. The defense is to install only listed components, to verify compatibility from the manufacturer's documents, and to keep the listing labels and documentation for inspection.

## Common Traps

### Designing to Audibility and Declaring Intelligibility

The installer spaces speakers to meet an audibility target, measures the sound pressure level, and declares the system compliant. The mechanism of the trap is that audibility and intelligibility are different, and a system that produces a loud, reverberant wash of sound can register above the audibility threshold while being completely unintelligible, especially in hard-surfaced or long, reflective spaces. The false signal is a sound level meter reading that meets the target, which proves loudness but not understandability. The harm is an emergency message that occupants hear but cannot understand, leaving them without actionable instruction at the moment it matters most. The defense is to design for measured intelligibility using STI or CIS, to use more lower-power speakers in reverberant spaces, and to verify with intelligibility measurements.

### Supervised Pathway Routed Without Survivability

The installer runs EVACS speaker circuits in ordinary conduit through unprotected spaces and relies on the electrical supervision to satisfy the code. The mechanism of the trap is that supervision detects a fault but does not protect the pathway from the fire that causes it, so a fire that burns through the conduit disables the circuit before the message is fully delivered, even though the panel dutifully reports the trouble. The false signal is that the panel supervises the circuit and reports a test fault, which proves supervision but not survivability. The harm is a message cut short by the very fire it warns about. The defense is to identify the required pathway class, to achieve two-hour survivability with listed cable or rated assemblies where required, and to route through protected shafts.

### Battery Sized for Standby Only

The installer computes the battery for the 24-hour standby load and declares the secondary power adequate. The mechanism of the trap is that the battery must also power the alarm condition, with all speakers and notification appliances active for the required alarm duration, and a battery sized only for standby will collapse when the system goes into alarm, cutting the message short. The false signal is that the battery holds the system up during a short standby test, which never exercises the alarm load. The harm is an emergency message that stops partway through because the battery exhausted on the speakers. The defense is to compute battery capacity for both standby and alarm durations, including the full alarm load.

### Confusing Mass Notification With Fire Alarm Voice Evacuation

The installer treats the voice system as a single entity and routes a lockdown message over the fire alarm evacuation system. The mechanism of the trap is that mass notification and fire alarm voice evacuation are governed by different code sections and have different priority and conflict rules, and a message appropriate to one system, such as a shelter-in-place or lockdown instruction, may be inappropriate or illegal on the other, and may also conflict with an active fire alarm evacuation signal. The false signal is that the speakers work and the message is audible, which proves the audio path but not the code compliance. The harm is conflicting or contradictory instructions during an emergency, or a code violation that fails acceptance. The defense is to distinguish the systems by purpose and governing code, to design the integration and priority rules explicitly, and to document them for the AHJ.

### Independent Speaker and Strobe Layouts With Coverage Gaps

The installer lays out the voice speakers for intelligibility and the visible strobes for candela coverage without coordinating the two, leaving locations that have one but not the other. The mechanism of the trap is that voice and visible notification serve occupants with different needs, and a location covered by voice but not by visible, or vice versa, leaves some occupants without the cue they require, particularly in high-noise or hearing-impaired contexts. The false signal is that each layout meets its own coverage target independently. The harm is occupants who miss the notification because the two systems were not reconciled. The defense is to coordinate the two coverage plans so every required location has both voice and visible coverage, while circuiting them separately per their supervision requirements.

### Non-Listed Component Substituted to Save Cost or Time

A listed amplifier is backordered, so the installer substitutes a similar-looking commercial audio amplifier to keep the job moving. The mechanism of the trap is that EVACS components must be listed for life-safety service, and a commercial audio amplifier is not listed for the supervision, survivability, and reliability requirements of the code, so the substitution voids the system listing and the AHJ acceptance. The false signal is that the amplifier powers the speakers and sounds fine, which proves function but not listing. The harm is a system that fails acceptance and loses its life-safety certification, requiring replacement and re-testing. The defense is to install only listed components, to verify compatibility, and to keep documentation for inspection.

## Self-Check

- Did I design speaker coverage for measured intelligibility (STI or CIS), using more lower-power speakers in reverberant spaces, rather than relying on an audibility target alone?
- Did I identify the required NFPA 72 pathway class for each circuit and achieve the required survivability with listed cable, rated raceway, or rated assemblies where two-hour performance is required?
- Did I compute secondary power capacity for both the standby duration and the alarm duration, including the full load of all speakers and notification appliances in alarm?
- Did I distinguish mass notification from fire alarm voice evacuation by purpose and governing code, design the integration and priority rules explicitly, and document them for the authority having jurisdiction?
- Did I coordinate voice speaker and visible strobe coverage so every required location has both, while circuiting them separately per their supervision and survivability requirements?
- Did I introduce end-of-line faults on every circuit during acceptance, confirm trouble signals at the panel, and document the supervision test results for the AHJ?
- Did I install only listed components, verify compatibility from the manufacturer's documents, and retain listing labels and documentation for inspection?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
