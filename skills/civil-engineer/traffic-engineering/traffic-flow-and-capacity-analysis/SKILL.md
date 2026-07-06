---
name: traffic-flow-and-capacity-analysis.md
description: Use when the agent is analyzing traffic flow and capacity, computing free-flow speed, density, and level of service for basic freeway and multilane segments, evaluating weaving and merge-diverge areas, sizing signalized intersection capacity and saturation flow, or applying the Highway Capacity Manual to assess performance under existing or projected demand.
---

# Traffic Flow and Capacity Analysis

Traffic flow and capacity analysis is the quantitative assessment of how a roadway, intersection, or interchange performs under a given demand, and it is the basis on which lane additions, signal timings, interchange forms, and development impacts are justified. The discipline is governed by the Highway Capacity Manual (HCM), which provides calibrated methods for basic freeway segments, weaving areas, ramps and merge-diverge junctions, multilane and two-lane highways, and signalized and unsignalized intersections. The harm this skill prevents is a capacity analysis that produces a clean level-of-service letter while misrepresenting the real operating condition, leading to overbuilt or underbuilt infrastructure, denied or approved development on faulty grounds, or signal timing that fails in the field. Capacity is not a fixed number of vehicles; it is a function of free-flow speed, geometry, heavy vehicles, driver population, grade, and, for intersections, saturation flow, phasing, and lost time. The analyst's job is to assemble the correct inputs for the correct HCM method, to recognize where the method's assumptions break down, and to avoid presenting a single deterministic result as if it were certain when traffic is stochastic and demand is forecast.

## Core Rules

### Use the Correct HCM Method for the Facility and Assemble Its Required Inputs

Each facility type has a distinct HCM methodology with its own required inputs, and applying the wrong method or omitting key inputs invalidates the result. For basic freeway segments, the method requires free-flow speed (measured or estimated from lane width, right-shoulder width, and interchange density), demand flow rate adjusted for heavy vehicles and driver population, and the resulting density in passenger cars per mile per lane, which maps to level of service A through F. For weaving segments, the method accounts for weaving versus non-weaving vehicle configuration, the number of lanes, and the weaving volume ratio, because weaving turbulence reduces capacity well below a basic segment. For merge and diverge areas, the method checks the capacity of the ramp influence area and the adjacent freeway lanes. Confirm that the analysis uses the current HCM edition's method and that every required input is supplied from field data or defensible defaults, because a missing or default input (a generic free-flow speed, an unverified heavy-vehicle percentage) can shift the level of service by a full letter.

### Base Capacity on Field-Measured Free-Flow Speed and Saturation Flow Where Possible

Free-flow speed and saturation flow rate are the two inputs most often defaulted and most often wrong. For freeways and multilane highways, free-flow speed should be measured in the field under low-volume conditions rather than estimated, because it reflects the actual geometry, driver behavior, and posted speed environment of the facility; a defaulted free-flow speed that is too high inflates the estimated capacity and masks congestion. For signalized intersections, saturation flow rate (the maximum flow per lane per hour of green) should be measured in the field at the study intersection or taken from local data, then adjusted by HCM adjustment factors for lane width, grade, parking, bus blockage, area type, and turning movements, because the base ideal saturation flow of 1900 passenger cars per hour per lane is rarely achieved in real urban conditions. Document the source of every free-flow speed and saturation flow value, because these inputs dominate the capacity result.

### Adjust for Heavy Vehicles, Grades, and Driver Population Honestly

Capacity and speed are stated in passenger-car equivalents, so heavy vehicles, steep grades, and unfamiliar driver populations must be accounted for through the HCM adjustment factors rather than ignored. Apply the passenger-car equivalent (PCE) for heavy vehicles based on the terrain type (level, rolling, or mountainous) or, for extended grades, the grade-specific method that accounts for length and percent grade, because a sustained upgrade can reduce a truck's speed and the segment's capacity dramatically. Adjust for driver population where the route serves a high proportion of recreational or unfamiliar drivers, who have lower saturation flow and different headway behavior. The trap is to apply a default heavy-vehicle percentage or to ignore a sustained grade because it is inconvenient, which overstates capacity and understates congestion; the correction is to use actual classification counts and the grade-specific method where grades are significant.

### Interpret Level of Service With Its Density Basis and Its Limitations

Level of service (LOS) is a letter grade derived from a quantitative measure, and for freeways that measure is density, while for intersections it is control delay per vehicle. Understand what the letter represents and what it hides: LOS E is at capacity but still stable flow, while LOS F is breakdown or forced flow, and a facility can move from LOS E to F with a small demand increase because breakdown is nonlinear. Do not treat a marginal LOS D as equivalent to a comfortable LOS D, and do not average LOS across a corridor to hide a failing segment. Recognize that the HCM planning-level methods and default-based analyses are approximate, that demand forecasts carry uncertainty, and that a capacity analysis should present a range or at least acknowledge the sensitivity of the result to the key inputs rather than asserting a single deterministic level of service as if it were certain.

## Common Traps

### The Defaulted Free-Flow Speed That Inflates Capacity

The freeway analysis uses a default free-flow speed based on the design speed or posted speed rather than a field measurement, producing a higher assumed speed and capacity. The false signal is a clean capacity calculation showing acceptable LOS; the harm is that the real free-flow speed, reduced by tight geometry, frequent interchanges, or driver behavior, is lower, so the actual capacity is lower and the facility is closer to breakdown than the analysis shows.

### The Saturation Flow Taken at the Ideal Base Value

A signalized intersection analysis uses the ideal saturation flow of 1900 passenger cars per hour per lane without field measurement or adjustment for parking, bus blockage, grade, or turning conflicts. The false signal is a capacity number that suggests the intersection handles the demand; the harm is that real saturation flow in urban conditions is often far lower, the intersection is over capacity, and the signal timing designed on the inflated number fails to clear queues in the field.

### The Sustained Grade Treated as Level Terrain

A freeway segment with a sustained upgrade is analyzed as level terrain with a default heavy-vehicle factor, because the grade is short or seems minor. The false signal is an acceptable LOS; the harm is that trucks slow on the grade, the passenger-car equivalent of the truck volume rises sharply, capacity drops, and the segment breaks down at a lower demand than the level-terrain analysis predicted, producing recurring congestion the analysis missed.

### The Single Deterministic LOS Presented as Certain

The analysis reports a single LOS based on a single demand forecast, with no sensitivity testing or acknowledgment of forecast uncertainty. The false signal is the precise, defensible letter grade; the harm is that the decision to add a lane, approve development, or deny a project rests on a number that depends on uncertain future demand and defaulted inputs, and a small change in either shifts the conclusion, which the single-point presentation conceals.

## Self-Check

- Is the analysis using the correct current HCM method for the facility type (basic segment, weaving, merge-diverge, signalized intersection), with all required inputs supplied from field data or documented defaults?
- Are free-flow speed (freeways) and saturation flow rate (intersections) based on field measurement or local data and properly adjusted for lane width, grade, parking, bus blockage, and turning movements, rather than defaulted to ideal values?
- Are heavy-vehicle PCEs applied using the correct terrain or grade-specific method, and is the driver population adjusted where the route serves recreational or unfamiliar drivers?
- For weaving and merge-diverge areas, is the weaving volume ratio, ramp influence area, and lane configuration accounted for, recognizing that these reduce capacity below a basic segment?
- Is the level of service interpreted with its underlying measure (density for freeways, control delay for intersections), with LOS F breakdown distinguished from LOS E at-capacity flow?
- Is the result presented with sensitivity to key inputs and demand-forecast uncertainty, rather than as a single deterministic letter grade that conceals the analysis's real margin?
