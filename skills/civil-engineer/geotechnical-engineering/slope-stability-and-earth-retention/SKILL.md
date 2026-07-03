---
name: slope_stability_and_earth_retention.md
description: Use when the agent is analyzing slope stability, designing retaining walls or excavation support, evaluating landslide or embankment failure risk, selecting earth pressures, or specifying drainage and reinforcement for slopes and retained earth before construction.
---

# Slope Stability And Earth Retention

Slope stability and earth retention are judgments about whether a mass of soil will remain where it is or move, and the movement, when it occurs, is large, fast, and often catastrophic, burying what lies below or collapsing what is retained above. Agents who compute a factor of safety against a circular slip surface and stop miss that the real questions are about which failure mechanism governs, how water changes everything, how the construction sequence and the temporary condition often control the design, and how a retaining wall designed for the wrong earth pressure or without drainage fails by sliding, overturning, or structural distress. This skill covers the judgment exercised while analyzing slopes and designing earth-retaining structures, with the goal that the slope, the wall, and the excavation support each have a defensible margin against the failure mode that actually governs, and that water, the cause of most failures, is controlled.

## Core Rules

### Identify The Governing Failure Mechanism Before Computing A Factor

A slope or a retaining wall can fail by several mechanisms, a circular or planar slip through the soil, a wedge failure along a weak layer, a sliding or overturning of a wall, a bearing failure of the wall's base, or a global stability failure that engulfs the wall and the slope behind it, and the governing mechanism is the one with the lowest factor of safety. Identify the credible mechanisms for the geometry, the soil, and the groundwater, and analyze each, because computing a high factor against one mechanism while ignoring another that governs produces a false sense of safety. For layered soils with weak planes, check the planar and wedge failures along the weak layers; for walls, check sliding, overturning, bearing, and global stability; for excavations, check the basal heave and the bottom stability. The factor of safety is meaningful only against the mechanism that actually controls.

### Model Water Explicitly, Because Water Governs Most Failures

Water is the trigger of most slope and retaining wall failures, through increased unit weight, reduced effective stress, seepage forces, and pore pressure buildup, and an analysis that ignores water or models it incorrectly over-predicts the factor of safety. Determine the design groundwater or pore pressure condition, considering rainfall, the worst credible saturation, and the performance of any drainage system, and model it in the stability analysis, because a slope stable under dry conditions can fail catastrophically under saturation. For retaining walls, design the drainage, weep holes, geocomposite drains, or a drain layer, so that hydrostatic pressure does not build behind the wall, because a wall designed for drained earth pressure that becomes saturated by a failed drain fails under the full hydrostatic load. Water must be in the model, and its control must be in the design, because ignoring water is ignoring the dominant cause of failure.

### Use The Correct Earth Pressure For The Wall's Movement

The earth pressure on a retaining wall depends on the wall's movement: an unmoving wall sees the at-rest pressure, a wall that moves away from the soil sees the active pressure, and a wall that moves into the soil sees the passive pressure, and the values differ by tens of percent. Select the earth pressure based on the wall's expected movement, considering the foundation stiffness, the bracing, and the tolerance for movement, because a wall assumed to be active that does not move enough develops pressures closer to at-rest and can be under-designed. For braced excavations, use the apparent earth pressure distributions that account for the staged construction and the struts, because the classical active distribution does not represent the braced condition. Include the surcharge from adjacent loads, traffic, or buildings, because a surcharge behind the wall increases the pressure and is often the governing load. The earth pressure is not a fixed soil property; it is a function of the wall's behavior.

### Check The Temporary And Construction Conditions

The construction sequence often produces the critical condition, before the wall is fully braced, before the drainage is installed, before the slope is reinforced, or during an open excavation, and a design that checks only the final, completed condition misses the temporary condition that fails. Analyze each stage of the construction, the excavation lifts, the brace installations, the embankment lifts, with the soil, water, and support conditions at that stage, because the factor of safety can be lowest when the excavation is open and the support is incomplete. Specify the construction sequence on the drawings, because the temporary stability depends on the sequence, and a contractor who excavates in a different order can create an unstable condition the design did not consider. The temporary condition is part of the design, not the contractor's problem, because the design must be stable at every stage.

### Provide Drainage As A Primary Design Element

Drainage is not a courtesy; it is a primary load-reducing and stability-preserving element, and its absence or failure is a leading cause of wall and slope failure. Design the drainage system to intercept, collect, and discharge the water before it builds pressure or saturates the soil: surface drainage to divert runoff from the slope crest, subsurface drains to lower the groundwater behind walls and within slopes, and toe drains to collect the seepage. Size the drains for the credible flow, specify the filter to prevent the soil from piping into the drain, and provide cleanouts so the drain can be maintained, because a drain that clogs fails silently and the pressure builds until the wall or slope fails. The drainage design is as important as the structural design, because a well-drained wall sees active earth pressure and a saturated wall sees hydrostatic pressure, and the difference is the difference between standing and failing.

### Design Wall Components For Sliding, Overturning, And Structural Capacity

A retaining wall must resist sliding along its base, overturning about its toe, and the structural bending and shear in its stem and footing, and each is a separate check with its own factor of safety and demand. Check sliding against the base friction and the passive resistance in front of the toe, with the appropriate factor, because a wall that slides provides no retention. Check overturning against the resisting moment of the wall's weight and the soil on the heel, with the factor, because a wall that overturns collapses. Check the stem for bending and shear under the earth pressure, and the footing for bending under the soil pressure, because a wall that stands but cracks structurally is a failure. Provide the key or the pile at the base where friction alone cannot resist sliding, and design the reinforcement for the structural demands. Each component must be checked, because a wall is a system and its weakest component governs.

### Consider The Global Stability Including The Wall And The Slope

A retaining wall can be stable against sliding and overturning while the entire mass, wall plus the soil behind it, fails along a deep slip surface that passes beneath the wall, a global stability failure. Check the global stability with the wall modeled as an element, because a wall founded on a slope or near the crest of a slope can be part of a larger failure mechanism, and a wall that satisfies its local checks can be carried away by a global failure. The global check uses the same slope stability methods applied to the wall-slope system, with the groundwater and the weak layers, and it is the check that catches the deep failures the local checks miss. Global stability is the system check, and it must accompany the component checks, because a wall is only as stable as the ground it sits in.

## Common Traps

### The Single Circular Slip Surface Assumed Governing

The engineer runs a circular slip search and reports a factor of safety, treating the circular mechanism as the only credible failure. The mechanism is that slopes in layered soils, with weak planes or bedding, or in soils with structural features, fail along planar or wedge surfaces that a circular search may not find, and the lowest factor may lie on a non-circular surface. The false signal is that the search "found the critical surface." The harm is that the real failure occurs along a weak plane the analysis missed, at a lower factor than reported, and the slope fails unexpectedly. The credible mechanisms, circular, planar, wedge, and along weak layers, must all be checked, because the governing mechanism is the one with the lowest factor, and a single search may not find it.

### Water Omitted Or Modeled At The Dry Condition

The engineer analyzes the slope or the wall under dry conditions, or with the water at a low level, because that is the condition in the borings, and reports a satisfactory factor of safety. The mechanism is that water is the dominant trigger of slope and wall failure, and a slope stable when dry can fail catastrophically when saturated by rainfall or rising groundwater, because the unit weight increases, the effective stress drops, and seepage forces develop. The false signal is that the dry analysis "passes." The harm is that the first heavy rain saturates the slope, the factor drops below unity, and the slope fails, often with casualties. The design water condition must be the credible worst, full saturation or the design storm, and the drainage must be designed to control it, because a dry-condition analysis is an analysis of a condition that will not persist.

### Active Earth Pressure Used For A Wall That Cannot Move

The engineer designs a rigid retaining wall, founded on a stiff foundation or braced at the top, using the active earth pressure, which assumes the wall moves enough to mobilize the soil's full shear strength toward the active state. The mechanism is that active pressure develops only with movement, a rotation or a translation of the wall, and a wall that is restrained develops pressures closer to the at-rest value, which is significantly higher. The false signal is that the wall "is a retaining wall" and so active pressure applies. The harm is that the restrained wall develops at-rest pressures that exceed its active-pressure design, and it is under-designed, cracking or sliding under the higher real load. The earth pressure must match the wall's expected movement, with at-rest or higher pressures used for walls that cannot move, because the pressure is a function of the wall's behavior, not a fixed soil property.

### The Construction Stage Not Analyzed

The engineer checks the completed wall or slope and declares it stable, without analyzing the open excavation or the partially built embankment that exists during construction. The mechanism is that the temporary condition, with the excavation open and the supports not yet installed, or the embankment partially placed, often has a lower factor of safety than the completed condition, because the full support system is not yet active. The false signal is that the final design "is stable." The harm is that the excavation collapses during construction, or the embankment slips during placement, injuring workers and delaying the project, because the critical condition was never analyzed. Every construction stage must be analyzed with the soil, water, and support conditions at that stage, and the sequence specified, because the temporary condition is part of the design.

### Drainage Treated As An Afterthought

The engineer designs the wall structurally and adds a note for "drainage as required," without designing the drain, its flow capacity, its filter, or its discharge. The mechanism is that drainage feels like a detail and the structural design feels like the engineering, but a wall without designed drainage accumulates hydrostatic pressure behind it, and the pressure can double the design load, because water is heavy and the wall must hold it back along with the soil. The false signal is that drainage "is noted." The harm is that the drain, unspecified, is omitted or built too small, the wall saturates, and it fails by sliding or overturning under the hydrostatic load the design never considered. The drainage must be designed, sized for flow, filtered against piping, and provided with cleanouts, because a wall's drainage is a primary load-reducing element, not a courtesy.

### Global Stability Missed For A Wall On Or Near A Slope

The engineer checks the wall for sliding, overturning, and structural capacity and declares it adequate, without checking the global stability of the wall-slope system. The mechanism is that a wall founded on a slope or near a slope crest can be part of a deep failure surface that passes beneath the wall, and the wall's local checks, which assume the foundation is stable, do not catch the global mechanism. The false signal is that the wall "passes its checks." The harm is that the entire wall, with the retained soil, slides along a deep surface during a storm or an earthquake, because the global stability was the governing mechanism and it was never analyzed. The global stability must be checked with the wall modeled, because a wall is only as stable as the ground it sits in, and the local checks assume a stable foundation that the global check must verify.

## Self-Check

- [ ] Have all credible failure mechanisms, circular, planar, wedge, and along weak layers, been analyzed, with the governing one identified as the lowest factor?
- [ ] Is water modeled at the credible worst condition, full saturation or the design storm, with the drainage designed to control it?
- [ ] Does the earth pressure match the wall's expected movement, with at-rest or higher pressures used for walls that cannot move toward the active state?
- [ ] Has every construction stage been analyzed with the soil, water, and support conditions at that stage, and is the sequence specified?
- [ ] Is the drainage system designed for flow, filtered against piping, and provided with cleanouts, as a primary load-reducing element?
- [ ] Are the wall's sliding, overturning, bearing, and structural capacities each checked with the appropriate factors of safety?
- [ ] Has the global stability of the wall-slope system been checked, including the deep slip surfaces that pass beneath the wall?
- [ ] Are surcharges from adjacent loads, traffic, or buildings included in the earth pressure, where present?
