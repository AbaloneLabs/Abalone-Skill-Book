---
name: overcurrent-protection-device-selection-and-coordination.md
description: Use when the agent is selecting circuit breaker types, choosing between thermal-magnetic and electronic trip breakers, applying GFCI and AFCI protection, designing selective coordination, relying on series ratings, or verifying interrupting capacity (AIC) against available fault current for branch and feeder protection.
---

# Overcurrent Protection Device Selection and Coordination

Overcurrent protection is the system that keeps a fault from becoming a fire, and the device selection determines whether a fault clears at the breaker nearest the problem or trips a main and drops an entire building. The judgment problem is that overcurrent devices are often chosen from a default mental catalog, standard thermal-magnetic breakers in standard ratings, without considering whether the available fault current exceeds the interrupting rating, whether the upstream and downstream devices coordinate, or whether the special-purpose devices like GFCI and AFCI are required and correctly placed. An electrician who treats breakers as commodities will install a system that nuisance-trips, fails to clear faults, or drops loads that should have stayed online. This skill covers the decisions that determine whether overcurrent protection actually protects, coordinates, and survives the faults it is asked to clear.

## Core Rules

### Match the Interrupting Capacity to the Available Fault Current at Every Device

Every overcurrent device has an interrupting rating, expressed in symmetrical amperes (AIC, in kA), which is the maximum fault current it can safely interrupt. The available fault current at the device line terminals is determined by the transformer size and impedance and the conductor impedance back to the source. A device applied above its interrupting rating may fail to clear, rupture, or sustain an arc, because the fault energy exceeds what it was tested to handle. Standard residential breakers are often 10 kA, commercial applications commonly require 22 kA, 65 kA, or higher, and industrial and service mains can require 100 kA or more.

The trap is assuming a default rating is adequate because the panel and breakers are listed together. The defense is to calculate or obtain the available fault current at each device location, especially near large transformers, and to specify devices whose AIC equals or exceeds that value, documenting the basis for the selection.

### Choose the Trip Unit Type for the Application

Thermal-magnetic breakers use a bimetallic strip for overload (long-time) protection and a magnetic armature for instantaneous (short-circuit) protection. They are simple, robust, and adequate for most general-purpose branch circuits. Electronic trip breakers use microprocessor-based trip units with adjustable long-time, short-time, and instantaneous settings, and often ground-fault protection. They are required where coordination, adjustable pickup, or high interrupting ratings are needed, typically on larger frame feeders and mains. The choice is driven by the need for adjustability and coordination, not by a preference for newer technology.

The trap is defaulting to thermal-magnetic on a feeder that needs to coordinate with downstream devices, where the fixed trip curve cannot be set to avoid a downstream fault, or specifying electronic trips on simple branch circuits where the adjustability is unused and adds cost and failure modes. The defense is to map out which levels of the system require coordination and adjustability and to reserve electronic trips for those levels.

### Apply GFCI and AFCI Where Required and at the Correct Location

GFCI (ground-fault circuit interrupter) protection detects current imbalance between hot and neutral and trips to protect people from shock, typically at 5 mA. AFCI (arc-fault circuit interrupter) protection detects arcing signatures and trips to prevent fires. Both are required in specific locations by current codes, and the required locations expand over time. GFCI is required in wet and damp areas; AFCI is required in most dwelling-unit living spaces. The device must be installed at the origin of the circuit, either as a breaker or as the first receptacle, so that it protects the entire circuit including the branch-circuit wiring.

The trap is installing a GFCI receptacle at the end of a circuit, leaving the wiring upstream unprotected, or omitting AFCI because the local jurisdiction lags the current code. The defense is to install the protective device at the circuit origin, to verify the required locations against the adopted code edition, and to use combination-type AFCI where the wiring method or shared neutrals require it.

### Design Selective Coordination Where Continuity Matters

Selective coordination means only the device closest to a fault trips, leaving upstream devices and their loads online. It is achieved by matching trip curves so that the downstream device clears before the upstream device's short-time or instantaneous element operates. True coordination is required for critical systems such as life safety, healthcare essential systems, and legally required standby, and it is recommended wherever downtime is costly. Coordination is harder than it looks because instantaneous elements on upstream devices can operate faster than downstream devices can clear, and because series ratings, which are discussed below, are incompatible with coordination by design.

The trap is assuming coordination exists because the devices are different sizes, when in fact the upstream instantaneous element trips on a downstream fault. The defense is to plot the device time-current curves together, to use electronic trips with adjustable short-time bands and set instantaneous elements to coordinate, and to accept that coordination may require sacrificing series ratings on coordinated feeders.

### Apply Series Ratings Only With Tested and Listed Pairings

A series-rated combination allows a lower-interrupting-rated downstream device to be protected by a higher-rated upstream device, but only for the exact pairings tested and listed by the manufacturer, at the specified voltage, and with any restrictions on downstream motor or generator contribution. Series ratings reduce cost but are not interchangeable across manufacturers, cannot be inferred from individual device ratings, and are generally not permitted where downstream sources add to the fault current.

The trap is claiming a series rating for an untested combination because the upstream device has a high rating. The defense is to use the manufacturer's published series-rating table for the exact catalog numbers, to verify the voltage and any downstream-contribution restrictions, and to remember that series ratings and selective coordination are mutually exclusive, so a coordinated system cannot also rely on series ratings.

### Size Overcurrent Devices to the Load and the Conductor Together

The overcurrent device protects the conductor, so the device rating, the conductor ampacity, and the load must all agree. Continuous loads must be sized at 125 percent of the load. The device rating cannot exceed the conductor ampacity unless the conductor is part of a listed assembly or the next standard size rule applies for 800 A and below. The device must also carry the non-continuous load plus 125 percent of the continuous load without exceeding its rating.

The trap is sizing the breaker to the load and leaving the conductor undersized, or upsizing the conductor for voltage drop and forgetting to verify the device still protects it. The defense is to check the trio together: the device rating must protect the conductor, carry the load, and respect the 125 percent continuous-load rule, and any voltage-drop upsizing must keep the device within the conductor's ampacity.

## Common Traps

### Applying a Breaker Above Its Interrupting Rating

An electrician installs a standard 10 kA residential breaker in a panel fed from a large, close transformer where the available fault current is 18 kA. The mechanism of the trap is that the interrupting rating is the fault current the breaker was tested to clear, and exceeding it means the breaker is asked to extinguish an arc it was never validated against. The false signal is that the breaker fits the panel and carries the load current, which addresses overload but not short-circuit duty. The harm is that during a bolted fault the breaker can fail to clear, rupture its case, sustain an arc, and start a fire, because the magnetic forces and arc energy exceed its tested limit. The defense is to calculate available fault current at every device and to specify the AIC to exceed it.

### Assuming Coordination Exists Because Devices Are Different Sizes

A 100 A feeder breaker feeds a 30 A branch breaker, and the electrician assumes the 30 A will trip first on a downstream fault because it is smaller. The mechanism of the trap is that instantaneous elements on both breakers can operate in the same few cycles, and without curve matching the upstream instantaneous can beat the downstream device to clear, dropping all loads on the feeder. The false signal is the size difference, which suggests selectivity but does not guarantee it because trip time depends on the curve shape, not the ampere rating. The harm is that a single branch fault blackouts an entire panel or building, exactly what coordination is meant to prevent. The defense is to overlay the time-current curves and to use adjustable short-time bands and instantaneous overrides to force selectivity.

### Installing GFCI or AFCI at the Wrong Point in the Circuit

An electrician installs a GFCI receptacle as the last device on a bathroom circuit, leaving all the wiring upstream of it unprotected, or installs an AFCI breaker but the circuit shares a neutral with another circuit, causing the AFCI to nuisance-trip. The mechanism of the trap is that these devices must be at the origin of the protected wiring, and shared-neutral (multi-wire) circuits and certain wiring methods defeat or confuse the sensing. The false signal is that a GFCI or AFCI device is present, which looks compliant but protects only part of the circuit or trips constantly. The harm is either unprotected wiring that can shock or burn, or a circuit so prone to nuisance tripping that the occupant defeats it. The defense is to install the device at the circuit origin, to handle shared neutrals with two-pole AFCI or separate neutrals, and to test the device after installation.

### Claiming a Series Rating That Was Never Tested

To avoid buying high-AIC breakers throughout a panel, the electrician assumes the 65 kA main protects all the 10 kA branch breakers to 65 kA. The mechanism of the trap is that series ratings are laboratory-verified pairings of exact devices, and an untested combination has no listed rating regardless of the individual numbers. The false signal is the high upstream rating, which feels protective but does not transfer without the tested let-through behavior. The harm is branch breakers that fail during a fault they were assumed to be protected against, with the panel subjected to energy beyond its rating. The defense is to use the manufacturer's series chart for the exact catalog numbers and to refuse unlisted combinations.

### Forgetting the 125 Percent Continuous-Load Rule

An electrician sizes a 16 A continuous lighting load on a 20 A breaker with 12 AWG conductor, which fits the 80 percent rule exactly, then adds a non-continuous load to the same circuit, pushing the total over the 16 A continuous allowance. The mechanism of the trap is that continuous loads, those lasting three hours or more, must be limited to 80 percent of the device rating, and adding other loads to a circuit already at that limit overloads it. The false signal is that the breaker rating exceeds the connected load, which is true for instantaneous load but violates the continuous-load derating. The harm is nuisance tripping, conductor heating, and accelerated insulation aging. The defense is to sum continuous loads at 125 percent plus non-continuous loads and to verify the total is within the device and conductor rating.

### Using Thermal-Magnetic Where Coordination Is Required

On a healthcare essential system feeder, the electrician installs fixed thermal-magnetic breakers because they are cheaper, then discovers that any downstream fault trips the feeder main and drops the entire essential system. The mechanism of the trap is that fixed instantaneous elements cannot be set to coordinate, and the code requires selective coordination for these systems. The false signal is that the breakers are rated for the load and the fault current, which is necessary but ignores the coordination requirement. The harm is loss of continuity to life-safety loads during a single fault, the exact failure coordination prevents. The defense is to use electronic trip breakers with adjustable short-time bands and to verify coordination with overlaid curves where it is required.

## Self-Check

- Did I calculate or obtain the available fault current at each overcurrent device, and does every device's interrupting rating (AIC) equal or exceed that value with documented basis?
- Did I choose thermal-magnetic versus electronic trip units based on whether coordination and adjustability are needed, rather than defaulting to one type for all circuits?
- Did I install GFCI and AFCI protection at the circuit origin, verify the required locations against the adopted code edition, and handle shared neutrals with two-pole devices or separate neutrals?
- Where continuity matters, did I overlay the time-current curves to confirm selective coordination, and did I use adjustable short-time bands and instantaneous overrides rather than assuming size difference guarantees selectivity?
- If I am relying on a series rating, is the exact upstream-downstream pairing listed in the manufacturer's table at the installed voltage, and am I within any restrictions on downstream motor or generator contribution?
- Did I size each overcurrent device to carry the non-continuous load plus 125 percent of the continuous load, protect the conductor ampacity, and verify that any voltage-drop conductor upsizing keeps the device within the conductor rating?
- Did I confirm that series ratings and selective coordination are not both claimed on the same coordinated feeder, since they are mutually exclusive by design?
- Are assumptions, uncertainties, and confidence levels stated explicitly rather than buried in a confident-sounding conclusion?
