---
name: deep-foundation-and-pile-design.md
description: Use when the agent is designing driven piles, drilled shafts, or auger-cast piles, calculating axial bearing capacity (side friction and end bearing) and lateral capacity, estimating pile settlement, specifying pile load testing (static, dynamic, statnamic), designing pile caps and pile-to-cap connections, evaluating pile driving feasibility and drivability, or selecting pile types for given soil and loading conditions.
---

# Deep Foundation and Pile Design

Deep foundation design is a domain where the capacity is not a single number but a function of the installation method, the soil disturbance it causes, the group interaction, and the time-dependent setup; where the calculation methods carry wide uncertainty that only a load test can resolve; and where the failure mode that governs is often not the axial capacity the engineer calculated but the lateral capacity, the settlement, the structural capacity of the pile itself, or the buckling in soft soils. The recurring failure is to report a single axial capacity from a formula without a load test, to ignore the difference between the capacity of an isolated pile and the efficiency of a pile group, and to treat the pile as a structural element only in compression while ignoring lateral, buckling, and tensile demands. The engineer's job is to select the pile type against the soil and the structure, to calculate capacity with methods appropriate to the pile type and the soil, to specify a load test that confirms the capacity, and to design the pile and the cap as a structural system carrying all the demands. This skill covers the judgment used to design deep foundations, with the overriding principle that the calculated capacity is a prediction that must be verified by testing, and that the pile is a structural element carrying more than axial compression.

## Core Rules

### Select the Pile Type Against the Soil, the Structure, and the Construction Constraints

Pile types — driven displacement piles (steel pipe, concrete, prestressed), driven non-displacement piles (H-piles), drilled shafts (bored and cast in place), auger-cast-in-place (ACIP) piles, and specialty types — suit different soil profiles, loading conditions, groundwater conditions, and construction constraints. Select the type against the conditions: driven displacement piles for densification of loose granular soils and for capacity in competent bearing layers; H-piles for penetration to rock and for high point capacity; drilled shafts for large axial and lateral capacity and for conditions where vibration and noise preclude driving; ACIP piles for low-vibration, moderate-capacity applications. The type selection also sets the construction sequence, the noise and vibration, the right-of-way, and the spoils, so it must be made against the site, not against familiarity. A pile type that fits the soil but not the construction constraints is a failed selection.

### Calculate Capacity With Methods Appropriate to the Pile Type and the Soil

Axial capacity is the sum of side friction and end bearing, and the calculation methods differ by pile type and soil. For driven piles in granular soils, use the Nordlund or effective stress methods that account for displacement and soil properties; for drilled shafts, use the FHWA methods that account for the construction disturbance and the lower end bearing in drilled conditions; for clay, use the alpha or beta methods for side friction and the bearing capacity factor for end bearing. Lateral capacity is analyzed with a p-y analysis that models the soil's nonlinear resistance and the pile's structural stiffness. Use the method appropriate to the conditions, and report the capacity as a prediction with the method's inherent uncertainty, not as a precise value. The capacity of a pile is established by a load test, not by the formula alone.

### Specify a Load Test to Confirm the Capacity

Pile capacity calculations carry wide uncertainty — a factor of two or more is common — because the soil properties, the installation effects, and the method assumptions introduce error that accumulates. Specify a load test (static load test for the most reliable capacity, dynamic load testing with PDA for driven pile verification, statnamic for lateral or rapid axial, O-cell for high-capacity drilled shafts) to confirm the capacity, and use the test result to refine the production pile design or the pile driving criteria. The load test is how the calculated capacity becomes a verified capacity, and a deep foundation design without a load test relies entirely on the calculation, which may be wrong. The type and number of tests should match the consequence of a capacity shortfall and the variability of the site.

### Account for Group Effects and Efficiency

Piles in a group do not each carry the capacity of an isolated pile, because the stress bulbs overlap and the group fails as a block at a capacity that may be less than the sum of the individual capacities, and because the group settlement is greater than the single-pile settlement. Apply the appropriate group efficiency factors for axial capacity (especially in friction piles in clay, where the group block capacity can govern), check the group block failure mode, and calculate the group settlement using the equivalent raft or equivalent footing method. A foundation designed as N times the single-pile capacity, without group effects, can be over-designed or under-designed depending on the soil and spacing, and the group settlement can exceed the tolerable limit even when the capacity is adequate.

### Design the Pile as a Structural Element for All Demands

A pile is a structural column embedded in the ground, and it must be designed for axial compression, axial tension, lateral shear and moment, and buckling in soft or liquefiable soils, not for axial compression alone. Check the structural capacity of the pile section under the combined demands, using the effective length for buckling (which depends on the soil's lateral support, and which becomes large in soft or liquefiable soils where the support is lost), and confirm the reinforcement (for drilled shafts and ACIP piles) and the longitudinal capacity (for driven steel and prestressed piles). A pile designed for axial capacity alone can fail structurally in lateral, tension, or buckling under seismic, wind, or uplift conditions.

### Design the Pile Cap and the Pile-to-Cap Connection

The pile cap distributes the column load to the piles and ties the piles together, and the pile-to-cap connection transfers the axial, lateral, and moment demands. Design the cap for the pile reactions (one-way and two-way shear, flexure) and for the load path from the column to the piles, and detail the pile-to-cap connection for the demands (embedment of the pile into the cap, reinforcement development, moment transfer for fixed-head piles). The connection is especially critical for seismic and lateral loading, where a pinned connection that should be fixed, or a fixed connection that lacks the reinforcement, changes the lateral response and can fail the pile or the cap. The cap and the connection are part of the deep foundation system.

### Evaluate Drivability and Construction Feasibility for Driven Piles

For driven piles, the capacity is established by the driving — the blow count, the driving stress, and the hammer energy — and the pile must be drivable to the required capacity without being damaged by excessive driving stress or without refusing at too shallow a depth. Perform a drivability analysis (wave equation analysis) to select the hammer, estimate the blow count and the driving stresses, and confirm the pile can be driven to the design depth without damage. Evaluate the risk of obstructions, the practical refusal, and the setup (the capacity gain over time in cohesive soils) in the pile driving criteria. A pile that cannot be driven to the design capacity, or that is damaged in driving, is a construction failure that delays the project and may compromise the foundation.

## Common Traps

### Single Axial Capacity Reported Without a Load Test

The engineer reports a single axial capacity from a formula or software, without a load test, and uses it as the design capacity. The mechanism is that the formula produces a number and the number feels like the capacity. The false signal is a reported capacity. The harm is that the actual capacity, which depends on the installation effects and the soil variability, can be substantially lower (or higher) than the calculated value, and the foundation is over- or under-designed without anyone knowing. A load test must confirm the capacity.

### Group Effects and Group Settlement Ignored

The foundation capacity is taken as N times the single-pile capacity, without group efficiency or block failure checks, and the group settlement is taken as the single-pile settlement. The mechanism is that the single pile is the unit of design and the group is just the count. The false signal is a total capacity. The harm is that the group block capacity is lower than the sum of the individual capacities (in friction piles in clay), or the group settlement is greater than the single-pile settlement and exceeds the tolerable limit, and the foundation fails in capacity or serviceability despite each pile "having capacity." Group effects and settlement must be checked.

### Pile Designed for Axial Compression Only

The pile section is designed for axial compression only, without checking lateral, tension, or buckling under seismic, wind, or uplift. The mechanism is that the pile is a vertical element carrying vertical load. The false signal is a pile that "has axial capacity." The harm is that under lateral or uplift loading, the pile fails structurally in flexure, tension, or buckling in soft or liquefiable soil, none of which the axial design addressed. The pile must be designed as a structural element for all demands.

### Lateral Capacity and p-y Analysis Omitted

The lateral capacity of the pile is not analyzed with a p-y method, and the pile is assumed to provide lateral resistance without checking the deflection and the structural demand. The mechanism is that lateral capacity feels secondary to axial. The false signal is a pile with "some lateral capacity." The harm is that under lateral load, the pile deflects beyond the tolerable limit or the structural moment exceeds the pile's capacity, and the foundation fails in lateral serviceability or strength. The lateral capacity must be analyzed with a p-y method.

### Drivability Not Analyzed for Driven Piles

The pile is specified to a design depth and capacity without a drivability analysis, and the contractor drives the pile with whatever hammer is available. The mechanism is that the design specifies the capacity and the driving is the contractor's means and methods. The false signal is a design depth and capacity. The harm is that the pile cannot be driven to the design depth (obstructions, refusal) or is damaged by excessive driving stress, and the foundation is short of capacity or structurally compromised. A drivability analysis must confirm the pile is drivable.

### Pile-to-Cap Connection Not Detailed for the Demands

The pile-to-cap connection is shown as a standard detail without checking the moment transfer, the embedment, or the reinforcement development for the actual demands. The mechanism is that the connection feels like a standard detail. The false signal is a connection "on the drawing." The harm is that under seismic or lateral loading, the connection that should be fixed rotates, or the connection that should transfer moment fails in bond or shear, and the lateral response of the foundation differs from the design assumption. The connection must be detailed for the demands.

### Setup or Relaxation Not Considered in Driving Criteria

The pile driving criteria (the blow count for acceptance) are set without accounting for setup (capacity gain over time in cohesive soils) or relaxation (capacity loss in some dense sands and shales). The mechanism is that the driving resistance at the end of driving feels like the capacity. The false signal is a blow count. The harm is that the pile is accepted or rejected on the end-of-driving blow count, which may under- or over-estimate the true capacity after setup or relaxation, and the foundation has the wrong capacity. The driving criteria must account for setup or relaxation, confirmed by restrike or load testing.

## Self-Check

- Is the pile type selected against the soil profile, the loading, the groundwater, the vibration and noise constraints, and the construction sequence, not against familiarity?
- Is the axial capacity calculated with methods appropriate to the pile type and soil, and is a load test (static, dynamic, statnamic, O-cell) specified to confirm it?
- Are the group efficiency, the group block failure mode, and the group settlement checked, with the foundation designed for the group response rather than N times the single-pile capacity?
- Is the pile designed as a structural element for axial compression, tension, lateral shear and moment, and buckling in soft or liquefiable soils?
- Is the lateral capacity analyzed with a p-y method, with the deflection and structural moment checked against the tolerable limits?
- For driven piles, is a drivability (wave equation) analysis performed to confirm the pile can be driven to the design capacity without damage, and are obstructions and refusal considered?
- Is the pile cap designed for the pile reactions and load path, and is the pile-to-cap connection detailed for the axial, lateral, and moment demands, especially for seismic and fixed-head conditions?
