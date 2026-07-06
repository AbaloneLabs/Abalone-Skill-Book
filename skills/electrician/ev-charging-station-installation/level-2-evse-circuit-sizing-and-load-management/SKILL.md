---
name: level-2-evse-circuit-sizing-and-load-management.md
description: Use when the agent is sizing branch circuits and feeders for Level 2 EVSE chargers, planning multi-stall parking installations, applying NEC Article 625 continuous load factors and demand factors, or configuring dynamic load management and peak shaving across a bank of charging stations.
---

# Level 2 EVSE Circuit Sizing and Load Management

A Level 2 electric vehicle supply equipment (EVSE) installation looks deceptively simple: a charger on a wall, a car plugged in, power delivered. The judgment problem is that EV charging is a continuous load that runs for hours at the circuit's full rating, often simultaneously across many stalls, and the National Electrical Code treats it with rules that differ from ordinary receptacle or lighting loads. An electrician who sizes an EVSE circuit like a normal appliance circuit will undersize the conductor, the breaker, and the panel feeder, and the installation will trip on thermal overload the first time a fleet of vehicles charges together. Multi-stall installations compound the problem: each charger is continuous, the chargers are statistically correlated (everyone arrives home or to work at the same time), and the service entrance may not have the spare capacity a naive sum assumes. This skill covers the NEC Article 625 rules, the continuous and demand factor math, and the load management strategies that let a building add charging without a full service upgrade.

## Core Rules

### Treat Every EVSE Branch Circuit as a Continuous Load at 125 Percent

NEC Article 625 classifies EVSE as a continuous load, which means the branch circuit overcurrent device and the conductors must be sized at 125 percent of the charger's maximum rated current. A 40-amp Level 2 charger therefore requires a 50-amp breaker and conductors rated for 50 amps, not 40. The trap is reading the charger's nameplate, picking a breaker of the same rating, and calling it done, because the breaker will carry 40 amps for six hours and eventually trip on thermal accumulation. The defense is to take the charger's rated current, multiply by 1.25, and select the next standard breaker size and conductor ampacity, while also applying the 310.14 temperature correction and ampacity adjustment for the number of current-carrying conductors in the raceway. For a 48-amp charger the math is 48 times 1.25 equals 60 amps, which is the practical ceiling of a 60-amp circuit and leaves no room for sloppy conduit fill or elevated ambient temperature.

### Apply the 625.40 Feeder and Service Demand Factor for Multiple Chargers

When multiple EVSE units connect to a single feeder or service, NEC 625.40 permits a demand factor that reduces the calculated load, recognizing that not every charger runs at full output simultaneously. The rule allows the load to be calculated at the nameplate rating for the largest charger plus 75 percent of the remaining chargers in some interpretations, or the total load may be managed by a listed energy management system that limits the aggregate demand. The trap is either ignoring the demand factor entirely and overbuilding the service at great cost, or applying an optimistic diversity assumption that does not hold when a workplace fleet all plugs in at 9 a.m. The defense is to read the specific 625.42 and 625.40 text in the adopted code edition, document the demand factor applied, and confirm the local authority having jurisdiction accepts the calculation. Where the load is managed dynamically, the energy management system must be listed and its limiting function must be reliable, because the feeder is sized on the assumption that the cap holds.

### Size the Panel and Service Feeder for the Managed, Not the Nameplate, Load

The feeder supplying a panel of EVSE chargers must be sized to the managed load when a listed load management system is installed, or to the full continuous-load sum when it is not. The decision point is whether the building owner commits to load management; if they do, the feeder and service upgrade cost can drop dramatically, but the load management system becomes a single point of failure whose reliability must be engineered. The trap is sizing the feeder to the managed load, then having the load management controller fail in a way that lets all chargers run flat out, overloading the feeder. The defense is to specify a listed load management system with a fail-safe that caps total current at the hardware level, to document the managed load on the panel schedule, and to verify the cap cannot be bypassed by a configuration change.

### Use Dynamic Load Management to Avoid a Costly Service Upgrade

Dynamic load management, also called load balancing or power sharing, divides a fixed pool of service capacity among connected chargers in real time. As one vehicle's battery fills and its charge rate drops, the freed capacity is redistributed to other vehicles. This lets six 40-amp chargers share a 100-amp feeder without tripping, because the controller never lets the aggregate exceed the limit. The trap is assuming any networked charger does this automatically; many networked chargers only report data and do not control power. The defense is to confirm the chargers and the network controller support open or proprietary load management, to wire the controller so its command path is reliable, and to verify the fallback behavior when the controller or its network connection is lost, because a controller that fails open (full power to all) defeats the feeder sizing.

### Coordinate with Building Load for Peak Shaving and Demand Response

In commercial and multifamily installations, EV charging load is often layered onto an existing building with a demand-metered service, and uncontrolled charging can push the building into a higher demand tier that dominates the electric bill for the entire month. Load management integrated with the building energy management system can defer charging to off-peak hours, cap the EV contribution during the building's peak demand window, or use on-site battery storage to shave the peak. The trap is installing chargers on a demand-metered service with no coordination, so a single evening charging surge sets a new demand ratchet that costs the owner for twelve months. The defense is to analyze the building's load profile and rate tariff, integrate the EVSE controller with the building management system or a demand response program, and set charging schedules and caps that respect the demand window.

### Verify GFCI and Equipment Rating for the EVSE Environment

NEC 625 requires personnel protection for EVSE, and most listed Level 2 chargers include ground-fault detection built into the unit that is more sensitive and faster than a standard GFCI breaker. The branch circuit breaker does not need to be GFCI when the EVSE provides the protection, but the installer must confirm the listed charger's protection is active and not defeated. The trap is adding a GFCI breaker in addition to the charger's internal protection, creating a double-ground-fault scheme that interacts and nuisance-trips, or disabling the charger's protection and relying on a breaker that does not meet the EVSE-specific trip threshold. The defense is to read the charger's listing and instructions, install the protection it specifies, and avoid stacking protections that conflict.

### Document the Load Calculation for the Inspector and the Next Electrician

Every EVSE installation should leave behind a documented load calculation showing the charger ratings, the continuous-load factor applied, the demand factor or load management cap, and the resulting feeder and service impact. The trap is installing chargers informally with no paper trail, so the next electrician adding a stall cannot tell whether the panel has capacity, and the inspector cannot verify the continuous-load math. The defense is to prepare a panel schedule and load worksheet for every EVSE job, retain the load management system configuration record, and label the panel to indicate which circuits are managed and what the aggregate cap is.

## Common Traps

### Sizing the Branch Circuit to the Nameplate Instead of 125 Percent

The electrician reads a 40-amp charger nameplate and installs a 40-amp breaker with 8 AWG copper, treating it like any appliance circuit. The mechanism of the failure is that the charger runs at 40 amps continuously for hours, the breaker's thermal element accumulates heat, and the breaker trips partway through a charge session, resetting the vehicle's charge timer and leaving the owner with a depleted battery in the morning. The false signal is that the circuit "works" during a short test charge, which proves only that the instantaneous load is within rating, not that the continuous load is sustainable. The harm is chronic nuisance tripping, customer complaints, and a circuit that is technically a code violation because continuous loads require the 125 percent factor.

### Ignoring the Feeder Demand Factor and Oversizing the Service

A workplace installs twenty 40-amp chargers and the electrician sums them naively to 800 amps of continuous load, triggering a service upgrade that costs tens of thousands of dollars. The mechanism of the failure is that the continuous-load sum assumes perfect simultaneity that does not occur in practice, and NEC 625.40 explicitly permits a demand factor or a managed load calculation that would have sized the feeder to a fraction of that figure. The false signal is that the sum is "safe" and conservative, which is true for capacity but financially ruinous and unnecessary under the code. The harm is a massively overbuilt service that the owner pays for in transformer and conductor cost, when a listed load management system would have fit the existing service.

### Assuming a Networked Charger Automatically Balances Load

The owner buys networked chargers expecting them to share capacity, and the electrician sizes the feeder to a managed load without verifying the balancing function. The mechanism of the failure is that many networked chargers only provide monitoring and billing and do not command power reduction, so all chargers run at full output simultaneously and the undersized feeder overheats or the main breaker trips. The false signal is that the chargers are "smart" and connected to a network, which proves communication capability but not load control. The harm is an overloaded feeder and a fire or tripping risk that the load calculation assumed away.

### Stacking GFCI Breaker and Charger Internal Protection

The electrician installs a GFCI breaker feeding a listed charger that already has ground-fault protection, and the two protections interact. The mechanism of the failure is that the charger's internal detection and the breaker's detection both monitor the same residual current with slightly different thresholds and timing, so a normal capacitive charging handshake or a minor leakage event trips one device unpredictably. The false signal is that more protection is safer, when in fact the two schemes are designed to work alone and conflict when combined. The harm is nuisance tripping that interrupts charging and erodes user confidence in the system.

### Failing to Account for Conduit Fill and Temperature Derating

Six EVSE circuits are run in a single conduit, each sized individually at 125 percent, but the electrician does not apply the 310.14 adjustment factor for more than three current-carrying conductors. The mechanism of the failure is that the bundled conductors cannot dissipate heat, the effective ampacity drops to 80 percent or less, and a circuit that was correctly sized in isolation is now undersized in the bundle. The false signal is that each conductor is individually rated for the load, which is true in free air but not in a crowded raceway. The harm is conductor overheating, insulation degradation, and eventual failure or tripping under sustained charging.

### Load Management Controller Fails Open at Full Power

A load management system caps six chargers to a 100-amp feeder, but the controller fails in a way that releases all chargers to full output. The mechanism of the failure is that the feeder was sized on the assumption the cap holds, so when the cap is lost the feeder carries 240 amps on a 100-amp conductor and the main breaker trips or the conductor is damaged before protection operates. The false signal is that the controller is installed and configured, which proves presence but not fail-safe behavior. The harm is a feeder overload and fire risk during the exact failure mode the load management was meant to prevent.

## Self-Check

- Did I size every EVSE branch circuit breaker and conductor at 125 percent of the charger's rated current, and did I apply the 310.14 temperature and conduit fill adjustments to the conductor ampacity?
- For a multi-charger installation, did I apply the NEC 625.40 feeder and service demand factor or a listed energy management system cap, and did I document the specific code text and the authority having jurisdiction acceptance?
- Did I confirm whether the installation uses load management, and if so, is the controller listed, is its command path reliable, and does it fail safe (capping, not releasing, on failure)?
- Did I size the feeder to the managed load only when a reliable, listed load management system is installed, and to the full continuous-load sum otherwise?
- Did I analyze the building's demand tariff and load profile, and did I integrate EVSE scheduling and caps to avoid setting a new demand ratchet?
- Did I read the charger's listing to determine its ground-fault protection, and did I avoid stacking a GFCI breaker on a charger that already provides the protection?
- Did I prepare and retain a documented load calculation, panel schedule, and load management configuration record, and did I label the managed circuits and aggregate cap?
- Is the reasoning documented clearly enough that another practitioner could review the basis and reproduce the conclusion?
