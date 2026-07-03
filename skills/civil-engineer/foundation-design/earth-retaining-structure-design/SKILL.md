---
name: earth-retaining-structure-design.md
description: Use when the agent is designing retaining walls, basement walls, or bulkheads, computing earth pressure (active, at-rest, passive) and surcharge, checking sliding, overturning, and bearing stability, or specifying drainage and reinforcement for cantilever, gravity, anchored, or mechanically stabilised earth walls. Applies before fixing wall geometry and reinforcement, while verifying global and internal stability, and when reviewing surcharge, water, and seismic loading on retaining systems.
---

# Earth Retaining Structure Design

Earth retaining structure design is the engineering of a wall that holds back soil that would otherwise collapse to its natural angle of repose, and the wall must resist the lateral pressure of that soil, the surcharges behind it, the water that accumulates against it, and the seismic inertial force, all while staying within the bearing capacity of its foundation and the overall stability of the slope it creates. The visible wall is the easy part; the real engineering is the earth pressure, the drainage, and the global stability, and the most common retaining wall failures are not structural but geotechnical: the wall slides, overturns, or the whole mass slides on a weak layer below the foundation. The harm this skill prevents is a wall that collapses onto the property below, that tilts and tears the structure it supports, or that bulges and fails because the drainage was never provided and the hydrostatic pressure doubled the design load. Because retaining walls are often adjacent to property and life, the design must address the hidden failure modes, not just the visible wall.

## Core Rules

### Establish the Earth Pressure Theory and Condition for the Wall Movement

The lateral earth pressure depends on the wall movement, and the correct pressure must be selected for the actual condition. Active pressure develops when the wall moves outward enough to mobilise the soil shear strength (a rotation of roughly 0.002 radians or a translation of 0.001 times the wall height); at-rest pressure applies when the wall does not move (typical of braced walls and rigid basement walls); and passive pressure develops in front of the wall or embedded toe when the wall pushes into the soil. Confirm that the wall type and foundation allow the movement required for the active condition, because a wall designed for active pressure but restrained from movement will experience the higher at-rest pressure and may be underdesigned. For walls retaining compacted fill, use the compaction-induced pressure, which can exceed the at-rest value behind the wall.

### Compute the Earth Pressure With the Correct Soil Properties and Backslope

The earth pressure coefficient (Ka, K0, or Kp) is computed from the soil friction angle, the wall friction, the backslope angle, and the wall batter, by Coulomb or Rankine theory or by log-spiral or wedge methods where the geometry is complex. Use the soil friction angle and unit weight from the geotechnical report for the actual backfill material, not a default value, and confirm that the specified backfill is free-draining and compatible with the assumed friction angle. For a backslope above the wall, add the surcharge of the slope and the additional earth pressure of the inclined backfill. For surcharges behind the wall (traffic, buildings, construction loads), add the lateral pressure by the appropriate method (Boussinesq for point and line loads, equivalent surcharge for uniform loads), because a surcharge within a horizontal distance equal to the wall height produces a significant lateral pressure on the wall.

### Provide Drainage as a Primary Design Element

Water is the single largest cause of retaining wall distress and failure, because hydrostatic pressure behind a wall can double the lateral load and because water softens the foundation soil and reduces bearing capacity. Provide a free-draining backfill (clean sand, gravel, or crushed stone), a drainage collection system (perforated pipe at the heel, draining to daylight or a sump), and weep holes or a geocomposite drain to prevent water build-up. Design the wall for the drained condition, with the water level at the drainage outlet, and confirm that the drainage system can carry the design inflow. For walls where drainage cannot be guaranteed (basement walls, walls with landscape irrigation against them), design for the hydrostatic pressure of the full water head, because the wall will see it eventually. A wall designed for drained soil pressure without a functioning drainage system will fail at the first storm that saturates the backfill.

### Check External Stability: Sliding, Overturning, and Bearing

For a gravity or cantilever wall, check the external stability under the design earth pressure and surcharge. Sliding: confirm that the horizontal resistance (base friction plus any shear key or passive toe resistance, with the appropriate factor of safety) exceeds the horizontal driving force. Overturning: confirm that the resisting moment of the wall weight and the vertical component of the earth pressure about the toe exceeds the overturning moment of the horizontal earth pressure, with the factor of safety required by the code (commonly 1.5 for static, with seismic and temporary cases evaluated separately). Bearing: confirm that the bearing pressure under the resultant (which may be eccentric) is within the allowable, using the reduced effective area method for an eccentric resultant. A wall that is structurally adequate but fails sliding, overturning, or bearing has failed geotechnically, regardless of its concrete or steel.

### Check Global Stability of the Wall and the Slope

The wall and the soil behind and below it form a slope system, and the global stability (the stability of the whole mass against a deep-seated slip surface) must be checked by a limit-equilibrium slope-stability analysis, especially for walls on sloping ground, walls with weak foundation soil, walls with a high water table, and walls under seismic loading. Confirm that the factor of safety against global instability meets the project criteria (commonly 1.5 for static, 1.1 for seismic), and that the critical slip surface does not pass below the wall foundation. A wall that meets sliding, overturning, and bearing but sits on a slope whose global factor of safety is below 1.0 will slide as a mass, taking the wall with it, because the global stability, not the wall, governs the system.

### Design the Wall Structurally and Detail the Reinforcement

For reinforced concrete cantilever walls, design the stem for the flexure and shear from the earth pressure (with the moment at the base and the shear at the critical section), the footing for the moment from the soil pressure (with the reinforcement in the top of the heel and the bottom of the toe), and the key or dowels for the connection. For mechanically stabilised earth (MSE) walls, design the internal stability (the reinforcement tension, pullout, and connection at each layer) and the external stability (sliding, overturning, bearing of the reinforced mass as a block). For anchored walls, design the anchor capacity (by pullout and creep), the wall facing for the span between anchors, and the global stability including the anchor zone. Confirm that the reinforcement, connections, and corrosion protection are detailed for the design life, especially for metallic reinforcement in corrosive soil.

### Address Seismic Loading on Retaining Walls

Under seismic loading, the wall and the retained soil experience horizontal inertia, and the earth pressure increases by the seismic increment (computed by the Mononobe-Okabe method for gravity walls, or by the Wood method for restrained walls), while the wall itself experiences inertia. Confirm that the wall meets the reduced factor of safety under the seismic load case (commonly 1.1 for permanent walls), that the bearing pressure under the seismic resultant is within the allowable (often increased for the seismic case), and that the global stability under the seismic condition meets the reduced criterion. A wall designed for static earth pressure alone will be underdesigned for the seismic case, and the seismic increment often governs the wall geometry in seismic regions.

## Common Traps

### The Active Pressure On A Restrained Wall

A basement or braced wall is designed for active earth pressure, on the assumption that the wall will move enough to mobilise the active condition. The trap is that the restrained wall cannot move, the soil remains at the at-rest condition, and the actual pressure is 30 to 50 percent higher than the active value the design used. The false signal is the computed active pressure; the harm is a wall that is underdesigned for the real at-rest load and that cracks, deflects, or fails as the full pressure develops.

### The Wall Without Drainage

A wall is designed for drained soil pressure, with no drainage system or with weep holes that clog, and the backfill is not free-draining. The trap is that the wall meets the drained earth pressure, while the backfill saturates at the first storm, the hydrostatic pressure adds to the earth pressure, and the wall slides, overturns, or bulges under the combined load. The false signal is the verified drained stability; the harm is a wall that fails at the first saturation event, because the drainage that the design assumed was never provided or never maintained.

### The Surcharge Behind The Wall

A wall is designed for the earth pressure of the backfill, and a traffic load, a building, or a construction stockpile is placed within a horizontal distance equal to the wall height. The trap is that the wall meets the earth pressure, while the surcharge adds a lateral pressure that the design did not include, and the wall is underdesigned for the combined load. The false signal is the adequate earth-pressure design; the harm is a wall that tilts or fails when the surcharge is applied, because the surcharge-induced pressure was not in the load case.

### The External Stability Without Global Stability

A wall meets sliding, overturning, and bearing, and the design is declared complete, without a global stability check. The trap is that the wall is externally stable, while the soil mass behind and below the wall has a deep-seated slip surface with a factor of safety below 1.0, and the whole mass slides, taking the wall with it. The false signal is the verified external stability; the harm is a wall that is stable in itself but slides as part of a larger mass failure, because the global stability, not the wall, governed the system.

### The Bearing Pressure Under The Eccentric Resultant

A wall under earth pressure has a resultant that is eccentric toward the toe, and the bearing pressure is checked at the average value. The trap is that the average is within the allowable, while the pressure at the toe is far higher (triangular distribution) and exceeds the allowable, and the toe settles or the foundation soil yields locally. The false signal is the compliant average pressure; the harm is differential settlement or bearing failure at the toe, where the real pressure is highest.

### The Seismic Increment Ignored

A wall in a seismic region is designed for static earth pressure, and the seismic increment is not added, because the wall "looked adequate." The trap is that the wall meets the static case, while the seismic earth pressure and the wall inertia under the design earthquake exceed the wall's capacity, and the wall slides, overturns, or tilts in the earthquake. The false signal is the static stability; the harm is a wall that fails under the seismic load case that was never checked, in the very event the wall was implicitly expected to survive.

## Self-Check

- Is the earth pressure theory (active, at-rest, passive) selected for the actual wall movement, with the compaction-induced pressure accounted for in compacted fill?
- Are the earth pressure coefficients and unit weights from the geotechnical report for the specified backfill, with the backslope and surcharge included?
- Is drainage provided as a primary element (free-draining backfill, collection pipe, weep holes or geocomposite), or is the wall designed for the hydrostatic head if drainage cannot be guaranteed?
- Are sliding, overturning, and bearing stability verified under the design load combinations, with the bearing pressure checked under the eccentric resultant?
- Is the global stability of the wall and slope checked by limit-equilibrium analysis, especially for sloping ground, weak foundation soil, high water table, and seismic loading?
- Is the wall structurally designed (stem, footing, connections) for the earth pressure, with MSE internal and external stability or anchor pullout and facing designed as applicable?
- Is corrosion protection detailed for metallic reinforcement in corrosive soil, suited to the design life?
- Is the seismic load case checked (Mononobe-Okabe or Wood increment, wall inertia, reduced factor of safety), and does the wall meet the seismic criteria?
