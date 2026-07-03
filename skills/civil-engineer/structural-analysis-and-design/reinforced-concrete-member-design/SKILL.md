---
name: reinforced_concrete_member_design.md
description: Use when the agent is sizing or detailing reinforced concrete beams, columns, walls, or slabs for flexure, shear, axial load, or torsion, selecting reinforcement ratios, or checking strength and serviceability against ACI or equivalent concrete code provisions.
---

# Reinforced Concrete Member Design

Reinforced concrete design is the translation of internal forces from an analysis model into steel area, bar size, spacing, and member dimensions that satisfy strength, serviceability, and ductility under every governing load combination, and the work is unforgiving because concrete is brittle in shear and in compression and because the consequences of an undersized column or an under-reinforced beam are collapse under overload. Agents who treat the work as looking up a formula and solving for area miss that the real decisions are about ductility, load path completeness, constructability, and the interaction between flexural steel, shear steel, confinement, and detailing at the joints. A beam that satisfies the flexural formula can still fail in shear, a column that satisfies axial capacity can still fail in bond or confinement, and a wall that satisfies strength can still fail because its boundary element was not detailed. This skill covers the judgment exercised while designing concrete members, with the goal that each member is detailed, not merely sized, and that the design is defensible under review and under load.

## Core Rules

### Design For Strength With The Correct Load Combinations And Phi Factors

Begin every member design from the governing code's strength load combinations, applying the strength reduction factors appropriate to the failure mode, because phi is lower for brittle modes like shear and compression-controlled flexure than for ductile tension-controlled flexure, and using the wrong phi overstates capacity. Determine whether each section is tension-controlled, compression-controlled, or in the transition zone, because the phi factor and the ductility assumption depend on the net tensile strain, and a section designed as ductile that is actually compression-controlled will not yield before crushing. Check every member against the envelope of all strength combinations, including the seismic overstrength combinations where applicable, because a member sized for one combination may be governed by another. Strength is the floor of the design, not its completion.

### Provide Ductile Flexural Failure Through Tension Steel Limits

Ensure that flexural members fail in a ductile, steel-yielding mode rather than a brittle concrete-crushing mode, by limiting the tension reinforcement ratio to below the balanced and code-maximum ratios and by ensuring the net tensile strain at nominal strength exceeds the tension-controlled limit. The mechanism of ductility is that steel yields and stretches, redistributing load and signaling distress, before the concrete crushes; an over-reinforced beam crushes without warning. Where compression steel is needed to raise moment capacity or ductility, provide it with ties that prevent buckling, because untied compression steel contributes nothing once it buckles. Ductility is not optional in seismic design and is good practice everywhere, because a structure that yields before it collapses gives occupants time to evacuate.

### Design Shear With Explicit Consideration Of Brittle Failure

Shear failure in concrete is sudden and without warning, so shear design demands more conservatism and more attention than flexure. Compute the factored shear at the face of the support and at each critical section, account for the beneficial effect of axial load on shear capacity where present, and provide transverse reinforcement wherever the concrete alone cannot carry the demand or where the code requires it regardless of demand, such as in seismic frames. In seismic design, design shear for the capacity of the member, not for the analysis demand, so that the beam or column can yield in flexure without shearing, because a shear failure pre-empts the ductile flexural behavior the seismic system relies on. Detail stirrups with the required hooks, spacing, and extent, because a stirrup that is not anchored does not work.

### Detail Joints, Anchorage, And Splices For Force Transfer

The capacity of a reinforced concrete member is only as good as the force transfer at its ends, and the joint where beams frame into columns is the most highly stressed and most commonly under-detailed region. Provide development length for every bar, with the required hooks or mechanical anchorage where straight development is impossible; lap-splice bars only where permitted and with the required lap length, staggered so that no section has all bars spliced; and design the joint for the forces from the framing members, providing confinement steel so the joint concrete does not degrade under reversed cyclic load. A beam whose bars are not developed into the column has no fixed-end moment regardless of what the analysis assumed. Detailing is where most real failures originate, and where most design effort should go.

### Design Columns For Biaxial Bending And Confinement

Columns carry combined axial load and biaxial bending, and a design that checks each axis independently underestimates demand on corner columns and irregularly loaded columns. Use an interaction diagram or a biaxial method that captures the combined effect, and confirm the design point lies inside the capacity surface with adequate margin. Provide transverse reinforcement, ties or spirals, for shear, for compression-bar buckling restraint, and for confinement of the core, because a column whose shell spalls in a fire or an earthquake must still carry load through its confined core. In seismic design, provide the special confinement in the plastic hinge regions, because the column's ability to survive reversed cyclic bending depends entirely on core confinement. A column is the member whose failure is least forgiving, because its failure can trigger progressive collapse.

### Design Walls For Combined Axial, Flexural, And Shear Demand

Structural walls carry the lateral system and must be designed as combined axial-flexural-shear elements, with boundary elements where the compression demand requires confinement. Check the wall as a column-like section for axial and flexure, using the interaction diagram, and check the shear separately with the web steel. Determine whether boundary elements are required by the code's compression-strain or stress-based triggers, and where required, detail them with the same confinement as a column, because the wall's compressive toe is where crushing and failure initiate in an earthquake. Provide special inspection of the boundary element reinforcement, because its confinement steel is dense and difficult to place and is the wall's last defense against collapse. A wall designed without boundary elements where they are required is a wall that will crush at its toe before it yields.

### Check Serviceability: Deflection, Crack Width, And Vibration

Strength is necessary but not sufficient; a member that is strong enough but deflects excessively or cracks widely is a serviceability failure that damages finishes, alarms occupants, and invites litigation. Check deflection against code and project limits, using the effective moment of inertia that accounts for cracking, because gross-section deflection under-predicts the real deflection of a cracked member. Check crack width where exposure or appearance demands it, providing the distribution steel that keeps cracks small. Check floor vibration where the framing is long and slender or supports sensitive occupancy, because a floor that meets strength and deflection limits can still be perceptibly bouncy and unacceptable. Serviceability is part of the design, not a courtesy check at the end.

### Coordinate Detailing With Constructability And Inspection

A design that cannot be built or cannot be inspected is a design that will be built wrong. Space bars so that aggregate can pass between them, so that the vibrator can reach the bottom of the form, and so that field placement tolerances do not produce congestion; provide access for inspection of the dense regions like joints and boundary elements; and detail the reinforcement so that the sequence of placement is clear. Where congestion is unavoidable, consider alternative details, mechanical splices, or member size increases, because a congested joint built wrong is worse than a slightly larger joint built right. The drawing is not the design; the design is what gets built, and constructability is what determines that.

## Common Traps

### Sizing For Flexure And Forgetting Shear

The engineer solves for flexural steel area, provides it, and moves on, treating shear as a secondary check that is "usually fine." The mechanism is that flexure dominates the engineer's mental model of a beam and shear feels like a formality, but shear failure is sudden and without the warning that flexural yielding provides. The false signal is that the beam has "enough steel" for the moment. The harm is that a beam with adequate flexural capacity but inadequate shear capacity fails in shear under a load below its flexural capacity, with no warning, because the failure mode pre-empts the ductile mode the engineer assumed. Shear must be designed explicitly, with capacity-based design in seismic regions, because the failure mode that occurs first is the one that governs, and shear occurs first when it is under-designed.

### The Under-Detailed Joint That Becomes A Pin

The engineer designs the beam and the column separately and assumes the connection between them transmits the fixed-end moment the analysis used, without designing or detailing the joint. The mechanism is that the joint is the region of force transfer and is where beam bars anchor into column bars, and without confinement and development the joint degrades into something closer to a pin than a fixed connection. The false signal is that the analysis assumed fixity and so the joint "is" fixed. The harm is that under reversed cyclic load, an unconfined joint loses capacity, the fixed-end moment the beam was designed for cannot develop, and the frame's lateral system performs far below its intended capacity. The joint must be designed and detailed for the forces from the framing members, with confinement steel, because the connection is the structure.

### Compression-Controlled Section Designed As Ductile

The engineer designs a column or a heavily reinforced beam assuming tension-controlled, ductile behavior and uses the corresponding phi factor, when the section is actually compression-controlled because the steel ratio is high or the axial load is large. The mechanism is that ductility and phi depend on the net tensile strain, and a section with high steel ratio or high compression crushes before the steel yields. The false signal is that the section "has steel" and therefore yields. The harm is that the member is assigned a higher capacity and a ductility it does not have, and in a seismic event it crushes rather than yields, failing in the brittle mode the design assumed away. The strain state and the phi factor must be checked against the actual section, and where the section is compression-controlled, the lower phi and the loss of ductility must be acknowledged.

### Gross-Section Deflection Under-Predicting Real Deflection

The engineer checks deflection using the gross moment of inertia because it is the default in the software, and the reported deflection is small and acceptable. The mechanism is that concrete cracks under service load and its effective stiffness drops to a fraction of gross, so the real deflection is several times the gross-section value. The false signal is that the deflection check "passes." The harm is that the built beam deflects far more than predicted, cracking partitions, sloping floors, and ponding water, and the failure surfaces only after construction when correction is expensive. Deflection must be checked with the effective moment of inertia that accounts for cracking, and long-term deflection must include the creep multiplier, because the gross-section value is the deflection of an uncracked member that does not exist.

### Boundary Elements Omitted Where Required

The engineer designs a shear wall for axial, flexure, and shear and provides uniform web steel, without checking the code trigger that requires boundary elements at the compressed toe. The mechanism is that boundary element requirements are triggered by computed compressive strain or stress, and an engineer focused on overall wall capacity can miss that the toe requires column-like confinement. The false signal is that the wall "has enough steel" overall. The harm is that in an earthquake the unconfined toe crushes, the wall's flexural capacity degrades, and the lateral system fails to perform, because the wall's survival depends on toe confinement that was never provided. The boundary element trigger must be explicitly checked and, where triggered, the confinement detailed, because a wall without its required boundary elements is a wall designed to crush.

### Congested Reinforcement That Cannot Be Placed Or Vibrated

The engineer details the theoretical steel required by the calculations, packing bars tightly to satisfy capacity, and the resulting cage cannot be built: aggregate cannot pass between the bars, the vibrator cannot reach the bottom, and honeycombing results. The mechanism is that constructability constraints, bar spacing, and aggregate size are physical limits that the calculation does not see, and exceeding them produces a built section that does not match the designed section. The false signal is that the section "has the required steel area." The harm is honeycombed concrete, unconsolidated joints, and a member whose real capacity is below its designed capacity because the concrete is defective where it matters most. Bar spacing, aggregate size, and vibrator access must be checked during detailing, and where congestion is unavoidable, the detail or the member size must change, because a section that cannot be consolidated is a section that will fail.

## Self-Check

- [ ] Has each member been checked against the full envelope of strength combinations, with phi factors matched to the actual failure mode and strain state?
- [ ] Is every flexural member tension-controlled or acknowledged as compression-controlled, with no over-reinforced section assumed ductile?
- [ ] Has shear been designed explicitly, with capacity-based design in seismic regions so that flexural yielding precedes shear failure?
- [ ] Are joints designed and detailed for the forces from framing members, with confinement, development length, and anchored hooks?
- [ ] Are columns checked for biaxial bending and provided with confinement for shear, bar buckling restraint, and core confinement, especially in plastic hinge regions?
- [ ] Have shear walls been checked for the boundary element trigger, and where required, is the toe confinement detailed?
- [ ] Are deflection checks using effective cracked-section stiffness with long-term creep, and are crack width and vibration checked where applicable?
- [ ] Does the detailing respect bar spacing, aggregate size, and vibrator access, so that the section as built matches the section as designed?
