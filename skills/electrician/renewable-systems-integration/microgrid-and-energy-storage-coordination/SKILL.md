---
name: microgrid-and-energy-storage-coordination.md
description: Use when the agent is designing microgrid architecture and islanding schemes, coordinating battery storage state-of-charge management, defining load prioritization and critical load shedding, engineering seamless grid-to-island transfer, or sequencing generator-PV-storage hybrid control hierarchies for multi-source resilient power systems.
---

# Microgrid and Energy Storage Coordination

A microgrid is not a collection of generation sources and storage wired to common loads; it is a control problem in which multiple sources — PV, batteries, generators, and the grid — must be coordinated in real time to keep the loads served, transition cleanly between grid-connected and islanded operation, and protect every source from the others' behavior. The judgment problem is that the difficulty of a microgrid lives in the control hierarchy and the transitions, not in the equipment: a system that runs fine grid-connected can fail to island, black out on transfer, or fight itself when the generator and battery both try to set the voltage. An electrician or designer who specifies sources and storage without defining the control architecture, the load priorities, the state-of-charge strategy, and the transfer logic produces a system that is unreliable precisely when resilience is needed. This skill covers the integration and control decisions that distinguish a functioning microgrid from a pile of connected equipment: microgrid architecture and islanding, storage coordination, load prioritization and shedding, seamless transfer, and multi-source control hierarchy.

## Core Rules

### Define the Microgrid Boundary and Islanding Scheme Explicitly

The microgrid boundary — which loads, sources, and switchgear are inside the island and which are outside — must be defined precisely, because it determines the interconnection switch (the point where the microgrid separates from the grid), the protection scheme in both grid-connected and islanded modes, and the load-generation balance that must hold in island. The discipline is to draw the boundary on the one-line, to identify the separation device (typically a fast static switch or a circuit breaker with sync-check), and to define what is served in island versus what is shed. A boundary drawn loosely leaves ambiguity about which loads must be supported and which sources must operate, and that ambiguity becomes a blackout during the first real island event. Define also the islanding mode: planned (operator-initiated) versus unintentional (grid loss detected), because the detection and transfer requirements differ.

### Manage Battery State of Charge Across Competing Objectives

Battery state of charge (SOC) is the finite energy reserve that makes a microgrid resilient, and it is contested by multiple objectives: the owner wants it discharged for economic arbitrage or self-consumption, the resilience requirement wants it held in reserve for outage, and the generator wants it used to absorb transients. The discipline is to define an SOC management strategy that reconciles these: a reserve floor (minimum SOC held for backup, e.g., 30%), a cycling band (the SOC range used for daily economics, e.g., 30-90%), and charge-source priorities (charge from PV first, from grid only when needed and allowed). The strategy must also handle the recovery phase after an outage, where the battery must be recharged while still serving load. An SOC strategy that optimizes only economics leaves the battery depleted when an outage hits; one that only reserves leaves economic value on the table. Define the strategy explicitly and configure the controller to it.

### Prioritize Loads and Define the Load-Shedding Sequence

Most microgrids cannot serve all loads in island with available generation and storage, so loads must be prioritized and a shedding sequence defined: critical loads (life safety, essential process) are never shed; important loads are served as capacity allows; deferrable or non-critical loads are shed first when capacity is tight. The discipline is to classify every load by priority tier, to map each tier to a controllable breaker or contactor, and to define the controller logic that sheds tiers in sequence as the generation-storage balance deteriorates, and restores them in reverse as balance recovers. The logic must be deterministic and tested, because a load-shed scheme that sheds the wrong load or fails to shed fast enough causes a frequency or voltage collapse that blacks out the whole island. Define the tiers, the shed order, the triggers (frequency, SOC, generation margin), and the restore hysteresis, and verify them in commissioning.

### Engineer the Grid-to-Island Transfer for Seamlessness or Defined Interruption

The transition between grid-connected and islanded operation is the moment most microgrid failures occur, and it must be engineered deliberately. A seamless transfer uses a fast static switch and inverter-based sources to ride through the transition with no perceptible interruption (sub-cycle), suitable for sensitive loads; a break-before-make transfer using a breaker with sync-check imposes a brief interruption (seconds to tens of seconds) while the microgrid sources stabilize in island. The discipline is to choose the transfer method based on load sensitivity, to define the detection logic for grid loss (voltage, frequency, rate-of-change), and to ensure the inverter and generator sources can form the island (set voltage and frequency) immediately after separation. A transfer that relies on the grid to "always be there" for voltage reference will collapse when the grid disappears and no source takes over. Define which source is the grid-forming source in island and verify the transfer in witness testing.

### Establish the Generator-PV-Storage Control Hierarchy

When a microgrid combines a rotating generator, PV inverters, and battery storage, the sources must have a defined control hierarchy or they will conflict: the generator and battery may both try to set voltage and frequency, the PV may over-generate and force the battery to absorb beyond its rate, and the generator may be loaded below its minimum (causing wet-stacking) or above its capability. The discipline is to define which source is grid-forming (sets voltage and frequency) in island — typically the battery inverter or the generator, not the PV — and which sources are grid-following (match the established voltage and frequency). Define the real and reactive power dispatch: how PV output is curtailed when the battery is full and load is low, how the generator is started and loaded, and how the battery absorbs or injects to smooth transients. The hierarchy must be documented as control logic and verified, because conflicts between sources manifest as instability, hunting, or collapse.

### Coordinate Protection for Both Grid-Connected and Islanded Modes

The available fault current in a microgrid is very different in grid-connected mode (high, dominated by the utility and any rotating generation) versus islanded mode (low, dominated by the inverter current limit, which may be 1.0 to 1.5 times rated current for a fraction of a second). Protection schemes and device settings that work in grid-connected mode may not clear faults in islanded mode because the fault current is too low to operate the overcurrent devices. The discipline is to analyze fault current in both modes, to specify protection that works in both (which may require different settings, arc-flash relays, or inverter-based fault detection), and to recognize that islanded protection often relies on the inverter's own fault response rather than traditional overcurrent relays. A protection scheme designed only for grid-connected mode leaves the island unprotected, and a fault in island may not clear, damaging equipment and ending the outage-resilience the microgrid was built for.

## Common Traps

### Drawing Sources and Loads on a One-Line and Calling It a Microgrid

A one-line shows PV, battery, generator, and loads connected to a common bus, and the design is described as a microgrid, but the control architecture, islanding logic, and source hierarchy are undefined. The trap mechanism is that the electrical connectivity reads as a complete microgrid, while the control — which is what makes it actually function — is invisible on the one-line and is left to be "sorted out at commissioning." The harm is a system that cannot island, fights itself, or blacks out on transfer, discovered only when resilience is needed. The defense is to define the control architecture, islanding scheme, and source hierarchy as design deliverables, not commissioning tasks, and to verify them in integrated testing.

### Optimizing Battery SOC for Economics and Depleting the Backup Reserve

The battery is cycled daily for time-of-use arbitrage down to a low SOC to maximize economic return, with no reserve floor, so when an outage occurs the battery has little energy to support the island. The trap mechanism is that the economic objective is measurable and incentivized daily, while the resilience objective is tested rarely, so the daily cycle erodes the reserve without consequence until the outage. The harm is a microgrid that fails its primary resilience purpose. The defense is to set and enforce a reserve floor that the economic cycling cannot breach, and to define the recovery logic that restores the reserve after use.

### Defining Load Shedding in Narrative but Not in Controllable Logic

The design narrative says "non-critical loads are shed first," but no load is mapped to a controllable device, and no trigger thresholds or restore logic are defined. The trap mechanism is that the narrative reads as a load-shed strategy, while the absence of executable logic means that when capacity tightens, nothing sheds automatically and the island collapses. The harm is a blackout that the load-shed scheme was supposed to prevent. The defense is to map every priority tier to a specific controllable breaker or contactor, to define the trigger and restore logic deterministically, and to test the sequence in commissioning.

### Assuming the Grid-to-Island Transfer Is Automatic and Seamless

The design assumes that when the grid fails, the microgrid "automatically islands" with no interruption, but no grid-forming source is designated, no transfer device is specified, and the detection logic is undefined. The trap mechanism is that automatic seamless transfer is a capability that sounds inherent to "microgrid," so it is assumed rather than engineered, while it actually requires a designated grid-forming source, a fast separation device, and tuned detection. The harm is a transfer that either does not happen (the microgrid collapses with the grid) or happens with a long interruption that drops sensitive loads. The defense is to choose the transfer method deliberately, designate the grid-forming source, define the detection logic, and witness-test the transfer.

### Letting the Generator and Battery Fight for Voltage/Frequency Control

In island, both the generator and the battery inverter attempt to control voltage and frequency, and they hunt against each other, oscillate, or one trips on protection. The trap mechanism is that each source is individually capable of grid-forming, so each is enabled to do so, without recognizing that only one source can set the reference and the others must follow. The harm is instability, nuisance trips, and in severe cases collapse of the island. The defense is to designate a single grid-forming source in island (typically the battery for fast response, with the generator for sustained energy) and to configure the others as grid-following, and to verify stable operation across load steps.

### Designing Protection Only for Grid-Connected Fault Current

The protection study sizes overcurrent devices and settings for the high grid-connected fault current, and does not analyze the low inverter-dominated fault current in islanded mode. The trap mechanism is that grid-connected fault current is the large, familiar number, so it dominates the study, while the islanded case — where fault current may be too low to operate any overcurrent device — is overlooked. The harm is that a fault in island does not clear, equipment is damaged, and the outage the microgrid was meant to ride through instead destroys part of the system. The defense is to analyze fault current in both modes and to specify protection (which may include inverter-based fault detection, differential schemes, or arc-flash relays) that functions in the low-current islanded case.

## Self-Check

- Did I define the microgrid boundary on the one-line, identify the separation device, and specify which loads and sources operate in island versus grid-connected mode?
- Did I define an SOC management strategy with a reserve floor, a cycling band, charge-source priorities, and post-outage recovery logic that reconciles economics and resilience?
- Did I classify every load by priority tier, map each tier to a controllable device, and define deterministic shed/restore triggers and hysteresis?
- Did I choose the grid-to-island transfer method (seamless or defined interruption) based on load sensitivity, designate the grid-forming source, and define the grid-loss detection logic?
- Did I establish a single grid-forming source in island and configure all other sources (PV, generator) as grid-following, with defined real/reactive power dispatch and curtailment logic?
- Did I analyze available fault current in both grid-connected and islanded modes and specify protection that clears faults in the low-current islanded case?
- Did I plan integrated commissioning and witness testing of islanding, transfer, load shedding, and source hierarchy before declaring the microgrid operational?
- Does the output stay within the agent's scope, deferring stamped microgrid design, protection studies, and controller programming to the licensed electrical engineer and microgrid specialist where the question exceeds the agent's competence?
