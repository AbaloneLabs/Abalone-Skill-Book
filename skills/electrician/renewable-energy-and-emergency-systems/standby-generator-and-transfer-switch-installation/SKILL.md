---
name: standby-generator-and-transfer-switch-installation.md
description: Use when the agent is sizing standby or backup generators, selecting automatic or manual transfer switches, installing and grounding separately derived systems, or bonding neutral and grounding for generator-backed emergency, legally required, or optional standby power systems.
---

# Standby Generator and Transfer Switch Installation

A standby generator with a transfer switch is a second power source that must connect to a building without backfeeding the utility, without creating a neutral-to-ground fault, and without dropping the load it is meant to protect. The judgment problem is that a generator is a separately derived system under specific conditions, and the grounding, bonding, and transfer switch configuration determine whether it is treated as one. Get the transfer switch type or the neutral bonding wrong, and the generator either backfeeds the grid (killing lineworkers), leaves equipment ungrounded during generator operation, or creates multiple neutral-to-ground bonds that put current on grounding conductors. An electrician who treats a generator install as "just another feeder" will create one of these three hazards. This skill covers the decisions that determine whether a standby system actually protects the load, protects the utility, and protects the people who depend on it.

## Core Rules

### Size the Generator to the Real Starting and Running Load, Not the Connected Nameplate

Generator sizing must account for motor starting inrush, the largest step load, and the continuous running load, not just the sum of connected nameplate values. Motors, especially air conditioning compressors and pumps, draw five to seven times running current at start, and a generator that carries the running load may stall or droop severely when the largest motor starts. The sizing sequence matters: the generator must recover from each step load within the voltage and frequency tolerance the loads can accept. Generator set sizing also depends on altitude and temperature derating, which reduce available output.

The trap is summing the connected load and buying the next size up without checking motor starting. The defense is to perform a step-load sizing that includes the largest motor's starting kVA, to apply altitude and temperature derating, and to verify the generator recovers within voltage and frequency tolerance for each step.

### Select the Transfer Switch Type for the System's Required Performance

Transfer switches are automatic (ATS) or manual, and open-transition (break-before-make) or closed-transition (make-before-break). The type is chosen by the system's required performance: life-safety and legally required standby systems generally require automatic transfer within a defined time; optional standby systems may use manual transfer; closed-transition switches are used where the load cannot tolerate even a momentary interruption and the utility permits momentary paralleling. The switch must also be rated for the available fault current at the point of connection and for the load type, including inductive and non-linear loads.

The trap is using a manual transfer switch on a system that requires automatic life-safety transfer, or an open-transition switch on a load that cannot tolerate the interruption. The defense is to determine the required transfer performance from the system classification, to select the switch type accordingly, and to verify the fault current and load ratings.

### Decide Neutral Switching Based on Whether the Generator Is a Separately Derived System

Whether the generator is a separately derived system determines whether the transfer switch must switch the neutral. If the generator has a direct electrical connection, solidly bonded neutral, to the utility-supplied system neutral, it is not separately derived, the neutral is not switched, and there is no neutral-to-ground bond at the generator. If the generator has no direct connection to the utility neutral, it is separately derived, the neutral must be switched in the transfer switch, and a neutral-to-ground bond is established at the generator or its disconnect. Getting this wrong creates either an ungrounded system during generator operation or multiple neutral-to-ground bonds that put load current on grounding conductors.

The trap is assuming all generators are separately derived and bonding the neutral at the generator when a 4-pole transfer switch is not used, creating a parallel neutral path. The defense is to determine separately-derived status from whether the neutral is switched, to install a neutral-to-ground bond only when the generator is separately derived, and to match the transfer switch pole count to the bonding decision.

### Ground and Bond Separately Derived Systems at Their Source

A separately derived generator system requires its own grounding electrode connection at the generator or its first disconnect, bonded to the building grounding electrode system, and a neutral-to-ground bond at the source. The equipment grounding conductors of the loads connect to this bonded neutral at the generator, exactly as they connect to the neutral at the utility service. This bond creates the fault-clearing path for the generator system. Without it, a ground fault during generator operation has no reliable return path and will not clear.

The trap is installing a separately derived generator without the grounding electrode connection or the neutral-to-ground bond, leaving the system ungrounded on generator power. The defense is to install a grounding electrode at the generator bonded to the building electrode system, to establish the neutral-to-ground bond at the generator or its disconnect, and to verify the equipment grounding conductors connect to the bonded neutral.

### Prevent Backfeed to the Utility Through Approved Transfer Means

The transfer switch's sole safety function is to prevent the generator from energizing the utility lines. This is achieved by a listed transfer switch that mechanically or electrically guarantees the generator and utility sources cannot be connected simultaneously, except for permitted momentary closed-transition paralleling. Portable generator installations using a manual transfer switch or interlock kit must use listed equipment that prevents simultaneous connection; backfeeding through a receptacle or a homemade interlock is lethal to lineworkers and is a code violation.

The trap is using a suicide cord or a double-male cord to backfeed a receptacle, or relying on an unlisted interlock. The defense is to use only listed transfer equipment that prevents simultaneous sources, to install it per its listing, and to never backfeed through a receptacle.

### Coordinate the Generator With the Load's Voltage, Frequency, and Phase

Generators must match the load's voltage, frequency, and phase configuration. A single-phase generator on a three-phase load, a 60 Hz generator on 50 Hz equipment, or a 120/240 V generator on a 120/208 V load will not work and can damage equipment. Reconnection generators can be reconfigured for different voltages, but the configuration must match the building system before connection. Frequency stability and waveform quality also matter for sensitive electronic loads.

The trap is connecting a generator whose voltage or phase does not match the building system, or assuming a reconnection generator is set correctly from the factory. The defense is to verify the generator's voltage, frequency, and phase match the load, to confirm the reconnection settings if applicable, and to check waveform quality for sensitive loads.

## Common Traps

### Sizing the Generator to Running Load Without Motor Starting

A generator is sized to the running load of a small commercial building, but when the air conditioner compressor starts, the generator voltage droops and the compressor contactor drops out, cycling the compressor and stalling the generator. The mechanism of the trap is that motor starting inrush is many times running current, and a generator adequate for running load cannot supply the momentary starting kVA. The false signal is that the running load fits the generator rating, which is true for steady state but not for the step load. The harm is failure to start critical motors, contactor chatter, and potential generator stall. The defense is to perform step-load sizing including the largest motor's starting kVA and to verify recovery within tolerance.

### Bonding the Neutral at a Non-Separately-Derived Generator

A generator is installed with a 3-pole transfer switch (neutral not switched), so the generator neutral is solidly connected to the utility neutral, and the electrician also bonds the neutral to ground at the generator, creating a second neutral-to-ground bond. The mechanism of the trap is that with the neutral continuous between systems, a second bond creates a parallel path for neutral current through the equipment grounding conductor and any bonded metal piping. The false signal is that the generator "needs to be grounded," which conflates equipment grounding with neutral bonding. The harm is neutral current on grounding conductors, elevated touch voltage, and interference with fault-clearing. The defense is to bond the neutral at the generator only when the generator is separately derived (neutral switched), and to keep the neutral isolated from ground otherwise.

### Leaving a Separately Derived Generator Ungrounded

A separately derived generator with a 4-pole transfer switch is installed without a grounding electrode connection or a neutral-to-ground bond at the generator, so when the system transfers to generator power the system reference to earth is lost. The mechanism of the trap is that a separately derived system must establish its own system ground at its source, and omitting it leaves the system floating with no defined reference and no reliable fault-clearing path. The false signal is that the generator frame is grounded, which is equipment grounding but not system grounding. The harm is a system with no fault-clearing path during generator operation, so a ground fault does not trip the breaker and the frame can stay energized. The defense is to install the grounding electrode and neutral-to-ground bond at the separately derived source.

### Using an Unlisted Interlock or Suicide Cord to Backfeed

To avoid buying a transfer switch, the owner uses a double-male cord to backfeed a receptacle from a portable generator, or an unlisted breaker interlock is fabricated. The mechanism of the trap is that simultaneous connection of generator and utility backfeeds the utility transformer, energizing the service drop at lethal voltage for lineworkers who expect it de-energized. The false signal is that the main breaker is off so the circuits are isolated, which the operator believes but which fails if the main is accidentally closed or the interlock is defeated. The harm is electrocution of utility workers and destruction of the generator. The defense is to use only listed transfer equipment that mechanically prevents simultaneous sources.

### Selecting an Open-Transition Switch for a Load That Cannot Tolerate Interruption

A data center loads an optional standby generator with an open-transition transfer switch, and the momentary interruption during transfer drops every server even though the generator starts and runs fine. The mechanism of the trap is that open-transition switches break before make, creating a brief power gap that IT and process loads cannot tolerate, and the generator selection did not account for transfer type. The false signal is that the generator carries the running load, which is true after transfer but irrelevant to the gap during transfer. The harm is load drop precisely during the transfer the system was installed to protect. The defense is to select closed-transition transfer or a UPS bridging for loads that cannot tolerate the interruption, and to confirm the utility permits momentary paralleling for closed transition.

### Mismatching Generator Voltage or Phase to the Building System

A 120/240 V single-phase generator is connected to a 120/208 V three-phase building, or a reconnection generator is left in the factory voltage setting rather than the building's voltage. The mechanism of the trap is that voltage, frequency, and phase must match exactly, and a mismatch either prevents operation or applies the wrong voltage to the load. The false signal is that the generator produces power and the plug fits, which proves generation but not compatibility. The harm is undervoltage or phase imbalance that damages motors and electronics, or a system that simply cannot carry the load. The defense is to verify voltage, frequency, and phase match before connection and to set reconnection generators to the building configuration.

## Self-Check

- Did I size the generator using step-load analysis that includes the largest motor's starting kVA, apply altitude and temperature derating, and verify recovery within voltage and frequency tolerance for each step?
- Did I select the transfer switch type (automatic/manual, open/closed transition) based on the system classification and the load's tolerance for interruption, and verify its fault current and load ratings?
- Did I determine whether the generator is a separately derived system from whether the neutral is switched, install a neutral-to-ground bond only when separately derived, and match the transfer switch pole count to that decision?
- For a separately derived generator, did I install a grounding electrode at the generator bonded to the building electrode system and establish the neutral-to-ground bond at the source?
- Did I use only listed transfer equipment that prevents simultaneous connection of generator and utility, and avoid any backfeed through receptacles or unlisted interlocks?
- Did I verify the generator's voltage, frequency, and phase match the building system, confirm reconnection settings if applicable, and check waveform quality for sensitive loads?
- Did I coordinate with the utility for any closed-transition or paralleling operation, and confirm the installation meets the utility's requirements for standby interconnection?
- Is the reasoning documented clearly enough that another practitioner could review the basis and reproduce the conclusion?
