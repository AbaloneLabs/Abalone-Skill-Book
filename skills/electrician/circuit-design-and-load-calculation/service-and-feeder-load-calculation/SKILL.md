---
name: service-and-feeder-load-calculation.md
description: Use when the agent is calculating service entrance or feeder loads, applying NEC Article 220 demand factors, sizing service conductors and main breakers, or determining the capacity required for a new or expanded electrical service.
---

# Service and Feeder Load Calculation

The service entrance is the point where electricity enters a building, and its capacity determines the total electrical load the building can support. Undersizing a service means the main breaker trips under normal usage, the voltage sags under load, and future expansion is impossible without a costly service upgrade. Oversizing wastes money on conductors and equipment that will never be fully utilized. The judgment problem is that service load calculation is governed by a detailed set of demand factors in NEC Article 220 that reduce the connected load based on the statistical probability that not all loads will operate simultaneously. An electrician who applies these factors incorrectly — either too aggressively (undersizing the service) or too conservatively (oversizing) — produces a service that either fails under load or wastes the owner's money. This skill covers how to apply the NEC demand factors, calculate general lighting loads, account for appliances and HVAC, and size the service conductors and main overcurrent protection.

## Core Rules

### Start With the General Lighting Load Based on Occupancy Type

The NEC specifies a minimum volt-ampere per square foot for general lighting and receptacles, indexed by occupancy type: 3 VA/sq ft for dwellings, 2 VA/sq ft for hospitals, 1 VA/sq ft for warehouses, and so on. This load is calculated from the floor area of the building and represents the baseline demand for lighting and general-purpose receptacles. For dwellings, the general lighting load is then subjected to a demand factor from Table 220.42: the first 3,000 VA at 100%, the next 117,000 VA at 35%, and the remainder at 25%. This reflects the reality that in a large building, not all lights and receptacles are used simultaneously. The trap is using a flat VA/sq ft without applying the demand factor, which dramatically overstates the lighting load for large buildings and leads to excessive service sizing. The defense is to apply the specific demand factor table for the occupancy type after calculating the base load.

### Apply the Demand Factor to Residential Receptacles Correctly

For dwellings, general-purpose receptacles are included in the general lighting load (the 3 VA/sq ft covers both lighting and receptacles), and small-appliance and laundry branch circuits are added at 1,500 VA each. The NEC requires a minimum of two small-appliance branch circuits for kitchen and dining receptacles and one laundry branch circuit, each calculated at 1,500 VA. These loads, combined with the general lighting load, are then subjected to the Table 220.42 demand factors. The trap is double-counting receptacles — adding individual receptacle loads on top of the per-square-foot allowance, which already accounts for general receptacles. The defense is to use the per-square-foot method for general lighting and receptacles and to add only the specific small-appliance and laundry circuits as separate line items.

### Size HVAC Loads by the Largest Motor or the Connected Load

Heating, ventilation, and air conditioning loads are a major component of service sizing, and the NEC provides specific rules. For electric heating (baseboard, furnace), the connected load at 100% is used. For air conditioning with multiple units, the largest motor is counted at 100% and additional units at 75% (NEC 220.60), reflecting that not all units run simultaneously. When both heating and cooling are present but cannot operate simultaneously (interlocked), the larger of the two loads is used, not both. The trap is adding both heating and cooling loads together when they are interlocked, which double-counts a load that can never occur, or counting all AC units at 100% when the demand factor allows 75% for additional units. The defense is to determine whether the heating and cooling systems are interlocked and to apply the specific demand rules for multiple motors.

### Apply the Neutral Sizing Rules for Feeders and Services

The neutral conductor carries the unbalanced current in a single-phase three-wire or three-phase four-wire system. For services and feeders, the neutral is sized to carry the maximum unbalanced load — the load between the neutral and any one ungrounded conductor. For dwellings, the neutral is typically the same size as the ungrounded conductors for services up to 200 amps, but for larger services and for specific load types (electric ranges, clothes dryers), the NEC permits a reduced neutral based on the calculated unbalanced load. The trap is reducing the neutral without performing the calculation, or failing to reduce it when the calculation permits, leading to either an undersized neutral (overheating) or an oversized neutral (wasted money). The defense is to calculate the unbalanced load for each service and to size the neutral per NEC 220.61, which permits reductions based on the specific load composition.

### Account for Future Expansion in Service Sizing

While the NEC demand factors produce a minimum service size, the electrician and the owner should consider future loads that are not yet connected. Electric vehicle charging, heat pump water heaters, induction cooking, and solar interconnection are increasingly common additions that can dramatically increase a service's load. A service sized to the exact current load leaves no room for these additions, and the owner will face a service upgrade (panel replacement, conductor upsizing, possibly utility coordination) when they install an EV charger or convert from gas to electric heating. The judgment is to size the service with a reasonable margin for foreseeable future loads, even though this exceeds the code minimum. The trap is sizing to the absolute minimum to reduce first cost, which is a disservice to the owner and a likely source of future call-backs.

### Coordinate the Service Conductors, Main Breaker, and Equipment Ratings

The service conductors, the main overcurrent protection, and the service equipment (panel, meter base) must all be coordinated in rating. The main breaker protects the service conductors, so the breaker rating cannot exceed the conductor ampacity. The equipment (panel bus, meter base) must be rated for at least the main breaker rating. And the conductor ampacity, after any derating for temperature and conduit fill, must meet or exceed the main breaker rating. The trap is selecting components independently — a 200-amp breaker with conductors that derate to 175 amps, or a panel rated for 225 amps with a 200-amp breaker and 4/0 conductors that are adequate for 200 but not 225. The defense is to verify the coordination of all three ratings after derating, and to document the basis for each selection.

## Common Traps

### Using Connected Load Instead of Demand Load for Services

An electrician sums all the connected loads in a building — every light, receptacle, appliance, and motor — and sizes the service at 100% of the total. For a large building, this produces a service size far larger than necessary, because the NEC demand factors exist precisely to account for the fact that not all loads operate simultaneously. The trap is that the connected load is easy to calculate (just add everything up) while the demand load requires applying multiple tables and factors. The result is a service that is safe but excessively expensive, with oversized conductors, a larger-than-necessary transformer, and higher equipment costs. The defense is to apply the NEC Article 220 demand factors systematically, which produces a service sized to the realistic maximum demand, not the theoretical connected load.

### Forgetting the First 10,000 VA at 100% for Appliance Loads

For dwelling units, NEC 220.53 allows the appliance load (dishwasher, disposal, range, dryer, water heater) to be taken at 75% of the connected load if there are four or more appliances. But this demand factor does not apply to the first 10,000 VA, which is taken at 100%. An electrician applies the 75% factor to the entire appliance load, understating the demand and undersizing the service. The trap is that the rule has a threshold that is easy to overlook, and the error produces a service that is slightly small — enough to cause nuisance tripping under peak load but not obviously wrong. The defense is to read the demand factor rules carefully and to apply thresholds and exceptions explicitly in the calculation worksheet.

### Double-Counting Heating and Cooling

A building has electric baseboard heating and central air conditioning, controlled by a single thermostat that cannot energize both simultaneously. The electrician adds both loads to the service calculation, producing a total that exceeds what the building can actually demand. The service is oversized, and the owner pays for capacity they can never use. The trap is that both loads are real and connected, but they are interlocked — the heating and cooling cannot operate at the same time. NEC 220.60 explicitly allows the larger of two noncoincident loads to be used. The defense is to determine whether loads are noncoincident (interlocked, mutually exclusive) and to use only the larger one in the calculation.

### Sizing the Neutral the Same as the Ungrounded Conductors Without Justification

For a dwelling service, the electrician installs a neutral conductor the same size as the two ungrounded conductors (full-size neutral). This is common practice for small services and is not wrong, but for larger services and specific load compositions, the NEC permits a reduced neutral, and failing to reduce it wastes money. The trap is that the full-size neutral is "safe" (it cannot be undersized), so there is no penalty except cost, and the electrician defaults to it to avoid the calculation. For commercial services with significant harmonic loads (computers, VFDs, LED drivers), however, the neutral may need to be larger than the ungrounded conductors (200% neutral), and the default of matching the ungrounded size is then inadequate. The defense is to calculate the neutral load for the specific installation and to recognize when harmonic loads require an oversized neutral.

### Ignoring Solar and EV Future Loads

A service is sized to the exact current demand, with no margin. Two years later, the owner installs an electric vehicle charger (adding 40 to 80 amps) and a solar interconnection (which may require backfeed capacity). The service is now overloaded, and the owner faces a service upgrade. The trap is that the NEC calculation produces a minimum based on current loads, and the electrician does not consider the foreseeable future. The defense is to discuss future loads with the owner during the design phase and to size the service with a margin that accommodates likely additions, even though this exceeds the code minimum. The cost of upsizing the service during initial construction is a fraction of the cost of upgrading it later.

### Overlooking the edge case or exception

The typical or textbook scenario is analyzed thoroughly, but the unusual case is skipped. The trap is that the standard path is well-handled while the exception silently produces the wrong outcome, because the agent stopped at the common case and never tested the boundary.

## Self-Check

- Did I calculate the general lighting load using the correct VA/sq ft for the occupancy type and apply the Table 220.42 demand factors to the result?
- Did I add small-appliance and laundry branch circuits at 1,500 VA each for dwellings, without double-counting general receptacles that are included in the per-square-foot allowance?
- For HVAC, did I determine whether heating and cooling are interlocked (use the larger) and apply the 75% demand factor to additional motors?
- Did I apply the appliance demand factor (75% for four or more appliances) only to the load above the first 10,000 VA threshold?
- Did I calculate the neutral load per NEC 220.61 and size the neutral conductor accordingly, recognizing when harmonic loads require an oversized neutral?
- Did I coordinate the service conductor ampacity (after derating), the main breaker rating, and the equipment rating so that all three are compatible?
- Did I discuss foreseeable future loads (EV charging, solar, electric heating conversion) with the owner and size the service with a margin, rather than to the absolute code minimum?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
