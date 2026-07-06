---
name: lateral-force-resisting-system-selection-and-design.md
description: Use when the agent is selecting a lateral force-resisting system (shear walls, braced frames, moment frames, or combinations), designing for seismic or wind loads, balancing stiffness against ductility, assigning seismic response coefficients, or evaluating system seismic design category limits and redundancy.
---

# Lateral Force-Resisting System Selection and Design

The selection of a lateral force-resisting system is the single most consequential decision in the structural design of a building subject to wind or seismic load, because it determines the building's stiffness, its ductility, its architectural footprint, the height it may reach, and the detailing that every connection must carry. The recurring failure is to choose the system by architectural habit or by what was used on the last project, then to size its members for strength and declare the design complete, when the real questions are about drift, ductility, redundancy, the seismic design category's limits on the system, and the load path continuity that makes the system actually function as a system. The engineer's job is to select the system against the height, occupancy, and seismic design category; to balance stiffness against ductility; to distribute lateral force by stiffness with torsion included; and to verify the complete load path from origin to foundation. This skill covers the judgment used to select and design a lateral force-resisting system, with the overriding principle that the system is governed by drift, stability, and ductility far more than by raw strength.

## Core Rules

### Select the System Against Height, Occupancy, and Seismic Design Category

Each lateral system — shear walls, braced frames (concentric or eccentric), moment frames (special, intermediate, or ordinary), buckling-restrained brace frames, or combinations — has code-permitted height limits that depend on the seismic design category and the occupancy, and selecting a system outside its permitted range is non-compliant regardless of its calculated capacity. Match the system to the height and seismic design category first, then to the architectural constraints: shear walls are stiff and efficient but demand continuous solid walls; braced frames are efficient but occupy bays; moment frames permit openness but are flexible and drift more. The system choice also sets the seismic response coefficients (R, Ω, Cd) that govern the design forces and the deflection amplification, so the choice ripples through every subsequent calculation.

### Balance Stiffness Against Ductility Deliberately

Stiffness and ductility are the two strategies for resisting lateral load, and the system selection is a choice between them. A stiff system (shear walls, braced frames) limits drift and protects non-structural components but attracts more force and demands less ductility; a ductile system (special moment frames) absorbs energy through inelastic deformation but drifts more and demands rigorous detailing. The balance depends on the building: a stiff system suits a building with drift-sensitive occupants or cladding, while a ductile system suits a building where openness is required and the drift can be tolerated. Make the tradeoff explicitly, because a system that is both very stiff and very ductile is rarely achievable, and the choice drives both the member sizes and the detailing requirements.

### Distribute Lateral Force by Computed Relative Stiffness, With Torsion

Lateral force distributes among the resisting elements in proportion to their stiffness, not equally, and an engineer who assumes equal distribution overloads the stiff elements and under-loads the flexible ones. Compute the stiffness of each wall and frame — accounting for cracking in concrete and for the contributions of each element — distribute the story shear by relative stiffness, and check each element's demand against its capacity. Where the layout is asymmetric, the stiffness centroid does not coincide with the mass centroid, torsion develops, and the elements on the stiff side are amplified while those on the flexible side are reduced. Model the torsion explicitly, including accidental torsion, and design the perimeter elements for the amplified demand. Force follows stiffness, and an equal-share assumption designs a building where the stiff elements fail first.

### Control Story Drift With Inelastic, Not Elastic, Values

Story drift — the relative horizontal displacement of one floor relative to the next — is the governing serviceability and safety criterion for lateral systems, and it must be checked under design wind and design seismic against the code limits, which are a fraction of a percent to a few percent of story height depending on the system and occupancy. The elastic drift from the analysis grossly under-predicts the real inelastic drift, because concrete cracks and steel yields under the design event; the code captures this with the deflection amplification factor (Cd) for seismic and with cracked-section properties for concrete. Compute the amplified drift and confirm every story meets the limit, because a single soft story violates the limit and creates a weakness that concentrates damage and can collapse. Drift, not strength, usually governs the lateral system.

### Provide Redundancy and Avoid Lateral Systems With Few Load Paths

A lateral system with a small number of primary elements (a few shear walls or braces) concentrates the consequences of any single element's failure, and the code penalizes such systems with a redundancy factor that amplifies the design forces. Design for redundancy where feasible — multiple lines of resistance, distributed elements — so that the loss or yielding of one element does not catastrophically redistribute to the others. Where redundancy is architecturally impossible, accept the amplified forces and the heightened detailing, and recognize that the system's reliability rests on the performance of a few critical elements. Redundancy is a seismic safety strategy, not a luxury, and a system designed without it is one element failure from distress.

### Ensure Complete Load Path Continuity From Origin to Foundation

The lateral force originates at the mass, travels through the floor diaphragm to the vertical lateral elements, travels down those elements to the foundation, and is resisted by the soil; every link must be designed, and a single gap collapses the system. Design the diaphragm for chord and collector forces, detail the collectors to develop their force into the walls, design the connections between the diaphragm and the walls (a wall not connected to the diaphragm receives no force and provides no resistance), and design the foundation for the overturning moment and the shear, with tension elements where the dead weight cannot resist uplift. The load path is a chain, and the engineer must identify and design every link, because the system is only as strong as its weakest connection.

### Apply Capacity Design So Ductile Elements Yield Before Brittle Elements Fail

In seismic design, the intended ductile mechanism (the fuse — plastic hinges in moment beams, yielding in brace buckling, shear wall yielding) must form and absorb energy before any brittle element (a connection, a column, a foundation) reaches its capacity. Capacity design protects the brittle elements by designing them for the maximum force the ductile fuse can deliver, which is its over-strength capacity, not the code design force. Identify the intended fuse, design it to yield in a ductile mode, and design the elements it feeds (columns, connections, collectors, foundations) for the fuse's over-strength capacity. A seismic system without capacity design relies on the code design force being correct, which it is not, and the brittle elements fail first.

## Common Traps

### System Selected Outside Its Seismic Design Category Height Limit

The engineer selects a system that is not permitted for the building's seismic design category and height, often an ordinary moment frame where a special system is required, and the design is non-compliant regardless of its calculated capacity. The mechanism is that the height and category limits are tabular and easy to overlook, and the system "works" by strength. The false signal is a system that "has capacity." The harm is a non-compliant design that fails plan review or, worse, is built and performs poorly in the design event, because the system's detailing and ductility do not match the demand. The system must be permitted for the seismic design category and height before any sizing.

### Equal Distribution of Shear Among Lateral Elements

The engineer divides the story shear equally among the walls and frames regardless of stiffness, and the stiff elements receive more force than designed while the flexible elements sit idle. The mechanism is that force follows stiffness, and a stiff shear wall attracts several times the shear of a flexible moment frame in the same line. The false signal is that the shear "is shared." The harm is that the under-designed stiff element fails first, because it received more force than its capacity. Shear must be distributed by computed relative stiffness, with torsion included.

### Drift Checked With Elastic, Uncracked Stiffness

The engineer checks story drift using the elastic analysis output, which uses gross or nominal stiffness, and the reported drift is below the limit, so the system is declared acceptable. The mechanism is that concrete cracks and steel yields under the design event, and the real drift is several times the elastic value, captured by Cd for seismic and cracked-section properties for concrete. The false signal is that the drift check "passes." The harm is that the built building drifts far more than predicted, damaging non-structural components, exceeding seismic separations, and in the extreme becoming unstable under P-delta. Drift must be checked with the amplified inelastic value.

### The Soft Story Created by Architectural Openness

The architect demands an open ground floor, the lateral elements are removed or reduced at that level, and the resulting story is far more flexible than those above — a soft story. The mechanism is that drift concentrates in the flexible story, because the stiff stories above force the displacement into the weak link, and the soft story's drift and P-delta demand exceed its capacity. The false signal is that the building "has lateral elements" on most floors. The harm is that the soft story collapses in an earthquake, as has happened repeatedly in real events. Soft stories must be avoided or, where unavoidable, designed with the special detailing and amplified forces the code requires.

### The Incomplete Load Path at the Collector or Connection

The engineer designs the shear wall and the diaphragm but not the collector that drags the diaphragm shear into the wall, or the connection between them, and the force has no path from the floor to the wall. The mechanism is that the load path is a chain, and the collector and connection are links easy to overlook because they are details rather than members. The false signal is that the wall "is designed." The harm is that under lateral load the diaphragm shears but the force does not reach the wall, the connection fails, and the diaphragm and non-structural components are damaged while the wall sits idle. Every link of the load path must be designed and detailed.

### Capacity Design Omitted, Brittle Elements Designed for Code Force

The engineer designs the connections, columns, and foundations for the code seismic design force rather than for the over-strength capacity of the intended ductile fuse, and in the design event the fuse yields and delivers more force than the brittle elements can carry. The mechanism is that the code force is reduced by the R factor to account for ductility, but the actual force the fuse delivers when it yields is its over-strength capacity, which is higher. The false signal is that the elements "meet code." The harm is that the brittle elements — connections, columns — fail before the ductile fuse can absorb energy, and the system fails in a non-ductile mode. Capacity design must protect the brittle elements for the fuse's over-strength.

### Redundancy Omitted, System Reliant on Few Elements

The lateral system relies on a small number of primary elements with no redundancy, and the failure or yielding of one redistributes catastrophically to the others. The mechanism is that redundancy is architecturally inconvenient and the redundancy factor is treated as a penalty to be minimized rather than as a safety signal. The false signal is an efficient, minimal system. The harm is that the system's reliability rests on a few elements, and any compromise of one — a construction defect, an unforeseen load — cascades. Redundancy must be designed where feasible, and where it cannot, the amplified forces and detailing must be accepted.

## Self-Check

- Is the selected lateral system permitted for the building's height, seismic design category, and occupancy, with the correct R, Ω, and Cd coefficients applied?
- Is the stiffness-versus-ductility tradeoff made explicitly, with the system matched to the building's drift tolerance and architectural needs?
- Is the story shear distributed by computed relative stiffness, with torsion from plan asymmetry and accidental torsion explicitly modeled?
- Is story drift checked under design wind and seismic against code limits, using cracked concrete stiffness and the seismic deflection amplification factor?
- Are soft or weak stories avoided, or where unavoidable, designed with the special detailing and amplified forces the code requires?
- Is the complete load path designed and detailed — diaphragm chords and collectors, diaphragm-to-wall connections, walls, and foundations — with no gap?
- Is capacity design applied so the intended ductile fuse yields before any brittle element (connection, column, foundation) reaches capacity, using the fuse's over-strength?
- Is redundancy provided where feasible, and where it cannot be, are the redundancy factor and heightened detailing applied?
