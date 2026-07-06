---
name: automatic-transfer-switch-wiring-and-sequencing.md
description: Use when the agent is wiring automatic transfer switches, programming transfer timing and time delays, sequencing loads on emergency and legally required standby systems, configuring load shed contacts, or applying NFPA 110 and NEC Article 700 requirements for emergency system transfer performance.
---

# Automatic Transfer Switch Wiring and Sequencing

An automatic transfer switch (ATS) is the device that decides when a building runs on utility power and when it runs on generator power, and that decision must be correct, fast, and sequenced so the generator accepts the load without stalling and the critical systems restore in the right order. The judgment problem is that an ATS is not just a big selector switch; it is a controller with timers, sensors, and logic that must be wired and programmed to match the generator's capability, the load's tolerance, and the code classification of the system. An electrician who wires the ATS by the prints and leaves the timers at factory defaults will deliver a system that either transfers too fast (before the generator is ready, stalling it or dropping the load) or too slow (failing the code-required time for life safety), that sheds the wrong loads, or that retransfers in a way that shocks rotating equipment. NFPA 110 and NEC Article 700 impose specific performance requirements that the wiring and programming must satisfy. This skill covers the control wiring, the time delay settings, the transition types, the load shed and sequencing logic, and the code framework that governs emergency system transfer.

## Core Rules

### Wire the ATS Control Circuits With the Logic the System Requires

An ATS has control inputs and outputs that include the utility and generator voltage sensing, the engine start contact (typically a dry contact that closes to signal the generator to start), the load shed outputs, the exerciser, and the communication to the generator controller. The wiring must match the logic: the engine start contact must be wired to the generator's remote start input, the voltage sensors must see the actual source voltages, and the load shed contacts must be wired to the loads they are meant to drop. The trap is wiring the control circuits by terminal number without understanding the logic, so a miswire causes the generator to start when it should not, or fails to start when it should. The defense is to study the ATS and generator control diagrams together, confirm the engine start signal is compatible (two-wire start versus three-wire start), and verify each control function by simulating a utility failure during commissioning.

### Program the Time Delays to Match Generator Capability and Code Requirements

An ATS has several programmable time delays, and each protects a different part of the system. The start time delay (typically 0 to 3 seconds) prevents nuisance starts on momentary utility dips. The engine warmup delay lets the generator reach rated voltage and frequency before accepting load. The transfer delay controls when the switch actually moves. The retransfer delay holds the load on generator until utility is stable. The engine cooldown delay lets the generator run unloaded after retransfer to dissipate heat. The trap is leaving these at factory defaults that do not match the generator's warmup needs or the code's transfer time requirement. The defense is to set the start delay to ride through momentary dips, the warmup delay to the generator manufacturer's recommendation, the transfer to occur within the NFPA 110 Level 1 time (typically 10 seconds for life safety), and the cooldown to the manufacturer's recommended unloaded run time.

### Select Open Transition Versus In-Phase (Closed) Transition for the Load

Open-transition ATS switches break before make, creating a brief power interruption that most loads tolerate but that drops motors, computers, and process equipment. In-phase (delayed) transition waits until the generator and utility voltages are in synchronism before transferring, minimizing the transient but requiring the generator to be synchronized. Closed-transition switches overlap the sources momentarily (make before break), requiring utility permission for the brief paralleling. The choice depends on the load: life safety loads often tolerate open transition, while data centers and processes with rotating equipment need in-phase or closed transition to avoid destructive transients. The trap is applying an open-transition switch to a load with large motors or sensitive electronics, so the retransfer transient trips breakers or damages equipment. The defense is to analyze the load's tolerance for interruption and transient, select the transition type accordingly, and coordinate closed transition with the utility.

### Sequence the Loads So the Generator Accepts Them Without Stalling

On systems with multiple loads or multiple ATS units, the loads must be sequenced so the generator accepts them in steps rather than all at once, because the largest step load determines the generator size and a single massive step can stall the engine or droop the voltage beyond tolerance. Sequencing is achieved by staggered ATS transfer, by load shed contacts that drop non-essential loads during emergency operation and restore them in order, or by a programmable logic controller that commands steps. The trap is wiring all loads to transfer simultaneously, so the generator sees the full connected load as one step and stalls or droops. The defense is to identify the largest acceptable step from the generator sizing, sequence the loads via staggered timers or load shed contacts, and verify during commissioning that each step is within the generator's recovery capability.

### Configure Load Shed Contacts to Protect the Generator From Overload

Load shed contacts on an ATS or its controller drop selected non-critical loads when the generator is at risk of overload, typically signaled by generator voltage droop, frequency droop, or a current sensor. The shed contacts must be wired to the right loads (not the life safety loads the system exists to protect) and the shed logic must be tested. The trap is wiring the shed contacts to loads that are not actually droppable, or not testing the shed function, so the first real overload stalls the generator instead of shedding load. The defense is to identify which loads are droppable under emergency conditions, wire the shed contacts to those loads, set the shed threshold to the generator's capability, and test the shed by overloading the generator during commissioning.

### Apply NFPA 110 Level and Class Requirements to Transfer Performance

NFPA 110 classifies emergency power systems by Type (the duration of the interruption the system can tolerate), Class (the duration the system can operate), and Level (the criticality, with Level 1 being life safety and Level 2 being legally required standby). A Level 1, Type 10 system must restore power within 10 seconds, which constrains the start delay, the warmup, and the transfer time. The trap is designing an ATS installation without knowing the system's NFPA 110 classification, so the timers do not meet the required restoration time and the system fails its acceptance test. The defense is to determine the NFPA 110 Type, Class, and Level from the project requirements and authority having jurisdiction, set the timers to achieve the required restoration time, and document the measured restoration time during commissioning.

### Coordinate the Neutral Switching With the System Grounding Scheme

Whether the ATS switches the neutral (3-pole versus 4-pole) is determined by whether the generator is a separately derived system, which in turn determines where the neutral-to-ground bond is established. A 4-pole ATS with a switched neutral makes the generator separately derived and requires a neutral-to-ground bond at the generator; a 3-pole ATS with a solid neutral does not, and bonding at the generator would create a parallel neutral path. The trap is mismatching the pole count and the bonding, creating either an ungrounded system on generator power or neutral current on grounding conductors. The defense is to decide separately-derived status from the grounding design, match the ATS pole count to that decision, and bond the neutral at the generator only when separately derived.

## Common Traps

### Leaving the Start Time Delay at Zero and Nuisance-Starting the Generator

The start time delay is left at zero seconds, so every momentary utility sag from a distant fault or a large motor start triggers a generator start. The mechanism of the failure is that utility sags are common and brief, and a zero delay starts the generator on every one, accumulating engine hours, wearing the starter and batteries, and producing nuisance run cycles. The false signal is that the ATS "responds quickly," which is true but counterproductive because most sags recover in under a second. The harm is accelerated generator wear, fuel consumption, and nuisance alarms. The defense is to set the start delay to ride through typical sags (commonly 1 to 3 seconds) while still meeting the NFPA 110 restoration time.

### Transferring Before the Generator Is Ready and Stalling the Engine

The transfer delay is set too short, so the ATS transfers the load before the generator has reached rated voltage and frequency. The mechanism of the failure is that the generator accepts full load while still at low voltage and frequency, the engine cannot sustain the torque, and it stalls or droops severely, dropping the load the system was meant to protect. The false signal is that the generator "started," which proves the engine is running but not that it is ready to accept load. The harm is a failed transfer, a stalled generator, and a delay in restoring critical power. The defense is to set the warmup delay to the manufacturer's recommendation and to confirm the ATS waits for rated voltage and frequency before transferring.

### Sequencing All Loads Simultaneously and Overwhelming the Generator

Multiple ATS units or multiple loads are wired to transfer at the same time, so the generator sees the entire connected load as one step. The mechanism of the failure is that the generator was sized for sequenced loading, and the simultaneous step exceeds its recovery capability, causing voltage and frequency droop that drops contactors and stalls motors. The false signal is that the generator carries the running load, which is true in steady state but not during the simultaneous step. The harm is a failed restoration, contactor chatter, and potential generator stall. The defense is to sequence the loads via staggered timers and to verify each step is within the generator's capability.

### Wiring Load Shed Contacts to Critical Loads

The load shed contacts are wired to loads that include life safety or critical equipment, so when the generator approaches overload the shed drops the very loads the system exists to protect. The mechanism of the failure is that the shed logic was not coordinated with the load criticality, and the shed threshold triggers on the wrong loads. The false signal is that the shed function is wired and operational, which proves the contacts work but not that they shed the right loads. The harm is loss of critical power during an overload that should have shed non-critical load instead. The defense is to identify droppable loads, wire the shed contacts to those, and exclude life safety loads from the shed scheme.

### Failing to Meet the NFPA 110 Type 10 Restoration Time

The ATS timers are set without regard to the NFPA 110 Type 10 requirement to restore life safety power within 10 seconds, and the cumulative start, warmup, and transfer delays exceed 10 seconds. The mechanism of the failure is that the restoration time is the sum of the delays plus the engine crank time, and uncoordinated timers push it past the code limit. The false signal is that the system transfers automatically, which proves function but not compliance with the time requirement. The harm is a system that fails its acceptance test and a code violation. The defense is to budget the 10 seconds across crank, start, and transfer, set the timers accordingly, and measure the restoration time during commissioning.

### Mismatching Neutral Switching and Bonding

A 4-pole ATS is installed but the generator neutral is not bonded to ground at the generator, or a 3-pole ATS is installed and the neutral is bonded at the generator, creating a mismatch. The mechanism of the failure is that the separately-derived status and the bonding do not agree, so the system is either ungrounded on generator power or has neutral current on grounding conductors. The false signal is that the ATS transfers power, which proves the switch works but not that the grounding is correct. The harm is an ungrounded system with no fault-clearing path or elevated touch voltage from neutral current on ground. The defense is to decide separately-derived status, match the pole count, and bond accordingly.

## Self-Check

- Did I study the ATS and generator control diagrams together, confirm the engine start signal compatibility (two-wire versus three-wire start), and verify each control function by simulating a utility failure?
- Did I set the start, warmup, transfer, retransfer, and cooldown time delays to match the generator capability and the NFPA 110 restoration time, rather than leaving factory defaults?
- Did I select open-transition, in-phase, or closed-transition based on the load's tolerance for interruption and transient, and did I coordinate closed transition with the utility?
- Did I sequence the loads via staggered timers or load shed contacts so the largest step is within the generator's recovery capability, and did I verify each step during commissioning?
- Did I wire the load shed contacts to droppable non-critical loads, set the shed threshold to the generator's capability, and test the shed by simulating an overload?
- Did I determine the NFPA 110 Type, Class, and Level, set the timers to achieve the required restoration time, and measure the actual restoration time during commissioning?
- Did I decide separately-derived status from the grounding design, match the ATS pole count (3-pole versus 4-pole) to that decision, and bond the neutral at the generator only when separately derived?
- Is the ATS programming, sequencing logic, and commissioning data documented so another practitioner can review and maintain the system?
