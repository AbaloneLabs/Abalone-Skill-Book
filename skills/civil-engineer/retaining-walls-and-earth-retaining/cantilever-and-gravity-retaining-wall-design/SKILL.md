---
name: cantilever-and-gravity-retaining-wall-design.md
description: Use when the agent is designing a cantilever or gravity retaining wall, determining earth pressure using Coulomb or Rankine theory, checking sliding, overturning, and bearing stability, detailing drainage and a keyway, or evaluating surcharge and global stability. Applies before the wall geometry and footing are fixed, while selecting earth pressure coefficients, drainage strategy, and the deep-seated and global stability checks that govern long-term performance.
---

# Cantilever and Gravity Retaining Wall Design

Cantilever and gravity retaining wall design is the engineering of rigid walls that retain soil through their own weight and the weight of the soil acting on the footing. A gravity wall relies on its mass to resist overturning and sliding, while a cantilever wall (a stem and a base footing) uses the soil over the heel to add stabilizing weight and is economical for moderate heights. The core judgment problem is that the wall is a stability-governed structure: the earth pressure, the surcharge, the water pressure, and the bearing and sliding resistances are all interdependent, and the hidden risks (water behind the wall, a weak foundation, a deep-seated failure, or global slope instability) govern far more than the stem reinforcement. The harm this skill prevents is a wall that slides or overturns, a footing that fails in bearing, a wall that tilts from a weak foundation or unbalanced water pressure, and a global slope failure that passes behind the wall and engulfs it. These are routine structures but they fail routinely when drainage, water, and global stability are ignored; agents support the licensed engineer and must defer final stability and geotechnical decisions to that team.

## Core Rules

### Determine Earth Pressure With the Correct Theory and Coefficients

Select the earth pressure theory and coefficients for the wall movement and the backfill geometry. For a wall that can rotate enough to mobilize the active wedge (typically a cantilever wall of moderate height), use active earth pressure, with Rankine coefficients for a level backfill and a smooth vertical wall, or Coulomb coefficients when wall friction, a sloped backfill, or a battered wall must be accounted for. For a wall restrained against movement (a basement wall, a braced wall, a wall rigidly connected to a structure), use at-rest earth pressure, which is significantly higher. For a wall pushed into the backfill (a wall driven by compaction or a passive resistance check at the toe), use passive pressure, but apply a reduction factor because full passive requires large movement. Using active pressure for a wall that cannot move, or ignoring wall friction and backfill slope, misstates the load; match the coefficient to the actual wall movement and geometry.

### Check Sliding, Overturning, and Bearing as Separate Stability Modes

A retaining wall must satisfy three independent external stability checks. Sliding resistance (the base friction plus any passive resistance at the toe and the keyway shear) must exceed the horizontal earth and water thrust, with the required factor of safety. Overturning resistance (the stabilizing moment from the wall and the soil over the heel about the toe) must exceed the overturning moment from the earth and water thrust. Bearing resistance (the contact pressure under the footing, accounting for the eccentricity of the resultant) must be within the allowable bearing capacity of the foundation soil, with the required factor of safety against a bearing failure. Each mode can govern depending on the geometry and the soil, so all three must be checked explicitly; a wall that is stable against overturning can still slide, and a wall that does not slide can still fail in bearing under an eccentric load.

### Provide Drainage and Design for the Water Pressure Case

Water behind a retaining wall is the single most common cause of failure, because the lateral pressure of water is roughly double the active earth pressure of soil, and a fully hydrostatic wall will almost always fail. Provide drainage: free-draining backfill (gravel or crushed stone) behind the wall, weep holes or geocomposite drains at the base, and a perforated collector pipe daylighting to a storm system, so that water does not build up. Design for a realistic water case (a water table at a defined level, or a partially hydrostatic condition) where drainage could be compromised, and check that the wall is stable under that case. A wall designed for dry backfill with no drainage provision and no water check is a latent failure, because the drainage will eventually clog or be overwhelmed and the water pressure will act.

### Include Surcharge, Compaction, and Seismic Loads Where They Apply

Account for surcharge behind the wall: traffic, stockpiled materials, buildings, or construction loads, which add lateral pressure through an equivalent surcharge or a Boussinesq distribution. Account for compaction pressures during construction, because heavy compaction equipment near the wall induces pressures higher than the active case and can damage the stem. For walls in seismic regions, apply a seismic increment using a Mononobe-Okabe pseudo-static analysis or a displacement-based approach, and check the wall under the seismic load combination. A wall designed for the soil alone, ignoring the surcharge it actually supports or the seismic demand, is under-designed for the real loading.

### Check Global and Deep-Seated Stability, Not Just External Stability

External stability (sliding, overturning, bearing) checks the wall as a block, but a weak foundation soil or a slope can fail on a deep-seated surface that passes beneath or behind the wall, and a global slope failure can carry the wall with it. Run a global stability analysis (circular and non-circular slip surfaces) through the wall and the surrounding slope, under static and seismic loading, and confirm the factor of safety meets the project criteria. Where the foundation soil is weak (soft clay, loose sand, organic soil), deep-seated stability often governs and may require ground improvement, a deeper footing, or a different wall type. A wall that passes external stability but sits on a weak foundation can rotate and fail globally, so global stability is a required check, not an optional refinement.

### Detail the Keyway, Reinforcement, and Construction Joints

Where sliding resistance is insufficient, provide a keyway (a shear key cast below the footing) to mobilize additional passive resistance, and design the keyway for the shear and the passive pressure. Detail the stem reinforcement for the flexure and shear at the base of the stem, with the development length and the dowels into the footing, and detail the footing reinforcement for the bending under the soil and wall loads. Provide construction and contraction joints at appropriate spacing to control cracking, and detail the drainage features (weep holes, collector pipe, filter fabric) so they function for the life of the wall. A wall with adequate calculated stability but poor drainage detailing or inadequate joint and reinforcement detailing will crack, leak, and deteriorate.

## Common Traps

### Active Pressure Used for a Restrained Wall

Active earth pressure is used for a wall that cannot rotate (a basement or braced wall), so the design load is far below the actual at-rest pressure. The false signal is a passing stability check; the harm is a wall that cracks or fails under the higher real pressure. Use at-rest pressure for restrained walls.

### No Drainage and No Water Case

The wall is designed for dry backfill with no drainage and no check of a water condition, so when drainage clogs or a storm saturates the backfill, the water pressure doubles the thrust and the wall fails. The false signal is a stable dry-wall calculation; the harm is a sudden failure in service. Provide drainage and check a realistic water case.

### External Stability Checked, Global Stability Ignored

Sliding, overturning, and bearing pass, but the wall sits on a weak foundation or a slope, and a deep-seated failure carries the wall away. The false signal is a wall that passes all three external checks; the harm is a global failure. Always run a global stability analysis.

### Surcharge or Seismic Load Omitted

The wall is designed for soil alone, but it supports traffic or a building, or it is in a seismic zone, so the real load exceeds the design. The false signal is an economical wall; the harm is under-capacity and distress or failure. Include surcharge, compaction, and seismic where they apply.

## Self-Check

- Are earth pressure coefficients (Rankine or Coulomb, active or at-rest) matched to the wall movement, wall friction, and backfill slope?
- Are sliding, overturning, and bearing each checked explicitly with the required factors of safety, including the effect of resultant eccentricity on bearing?
- Is drainage provided (free-draining backfill, weep holes or geocomposite, collector pipe) and is a realistic water case checked?
- Are surcharge (traffic, buildings, stockpiles), compaction pressures, and seismic increment (Mononobe-Okabe or displacement-based) included where they apply?
- Is a global and deep-seated stability analysis run through the wall and surrounding slope under static and seismic loading, with the factor of safety verified?
- Where sliding governs, is a keyway designed for the shear and passive resistance, and is the passive at the toe appropriately reduced?
- Are the stem and footing reinforcement, development lengths, construction joints, and drainage details specified so the wall performs for its service life?
