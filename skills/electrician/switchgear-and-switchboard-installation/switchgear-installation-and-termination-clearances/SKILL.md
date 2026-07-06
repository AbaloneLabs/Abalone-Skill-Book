---
name: switchgear-installation-and-termination-clearances.md
description: Use when the agent is installing switchgear and switchboards, setting equipment in place, maintaining NEC 110.26 working clearances, anchoring and leveling sections, and ensuring cable termination access and dedicated equipment space.
---

# Switchgear Installation and Termination Clearances

Installing switchgear looks like setting a heavy box and bolting it down, but the clearances around and within the equipment are code requirements that protect worker safety, heat dissipation, and terminations for the life of the installation. The judgment problem is that NEC Article 110.26 working space, dedicated equipment space, and termination access are not suggestions; they are minimums that inspectors enforce and that workers depend on. Agents tend to treat the floor plan as flexible, push equipment against walls to save space, and route cables through whatever path is available, when the result is a noncompliant installation that fails inspection, overheats, or traps a worker who cannot safely operate or maintain the gear. The skill exists to force the agent to treat clearances as design constraints, not afterthoughts.

## Core Rules

### Provide NEC 110.26 Working Space Depth Based on Voltage and Conditions
NEC 110.26 Table 110.26(A)(1) defines the minimum depth of working space in front of live parts, based on the voltage to ground and the conditions on the opposite side of the space. For 0 to 150V to ground, the depth is 3 feet; for 151 to 600V to ground, it is 3 feet if the opposite side is de-energized and grounded, 3.5 feet if the opposite side has live parts guarded, and 4 feet if the opposite side has exposed live parts. For above 600V (medium voltage), greater depths apply. Measure the depth from the nearest live part or the enclosure opening, not from the face of the closed door, because the working space must exist when the door is open and live parts are exposed.

### Provide the Required Width and Height of Working Space
The width of the working space must be at least the width of the equipment, or 30 inches, whichever is greater, and it must allow the enclosure door to open at least 90 degrees so a worker can enter and exit safely. The height of the working space must be at least 6.5 feet, or the height of the equipment, whichever is greater. Do not let ductwork, piping, or other equipment intrude into this space, because the clearance is for the person working, not for the equipment alone. A door that cannot open fully or a headroom blocked by a cable tray creates a trap during an emergency egress.

### Reserve Dedicated Equipment Space for the Full Footprint and Above
NEC 110.26(E) requires dedicated equipment space equal to the footprint of the equipment, extending from the floor to a height of 6 feet above the equipment or to the structural ceiling, whichever is lower, reserved exclusively for electrical equipment. No foreign systems (plumbing, ductwork, sprinklers, non-electrical conduits) may occupy this space. The intent is to prevent leaks, condensation, and mechanical damage from foreign systems onto electrical gear. Coordinate with the mechanical and plumbing trades during design, because retrofitting a sprinkler or pipe out of the dedicated space after installation is costly and disruptive.

### Anchor, Level, and Align Sections to the Floor and to Each Other
Switchgear sections must be bolted together, leveled, and anchored to the floor so that the lineup is rigid, plumb, and aligned. Leveling is critical because the drawout breakers, the bus joints, and the shutters all depend on correct alignment; a lineup installed out of level can bind breakers, misalign bus splices, and prevent proper racking. Use shims where required by the manufacturer, anchor per the seismic and uplift requirements, and verify alignment across the full lineup before torquing the section-to-section bus joints. An unanchored or unlevel lineup is a latent defect that surfaces as breaker difficulty and joint overheating.

### Maintain Termination and Cable Bending Space per NEC 312 and 314
Cable termination compartments must provide the bending space and gutter space required by NEC Article 312 (for cabinets and cutout boxes) and 314 (for outlet boxes), sized to the conductor size and the number of conductors. Large conductors require substantial bending radius and space; cramming large cables into an undersized gutter forces tight bends that damage insulation, overstresses the lug, and makes the termination inaccessible for torque verification. Verify the termination compartment size against the actual conductor size and count, and route cables so that each enters its lug straight, without side pull that works the connection loose over time.

### Provide Front and Rear Access Matching the Equipment Design
Switchgear specified as rear-access (for rear cable termination or drawout rear access) must have the rear working space and clearance that NEC 110.26 requires, applied to the rear as if it were the front when live parts are exposed at the rear. Do not install rear-access gear against a wall or in a tight alcove, because the rear access is required for termination, thermography, and breaker removal. Confirm during layout whether each section requires front-only or front-and-rear access, and design the room to provide it, because moving gear after installation to gain rear access is rarely possible.

### Coordinate Floor Openings, Sealing, and Environmental Conditions
Where cables enter from below through a slab, provide sleeved and sealed openings that prevent water ingress and that maintain the fire rating of the floor. In wet or outdoor locations, raise the switchgear on a housekeeping pad, seal all conduits, and provide drainage, because water in the cable compartment causes corrosion, tracking, and eventual failure. Coordinate the floor opening locations with the equipment termination pattern before pouring the slab, because relocating penetrations after the fact is expensive and disrupts the cable routing. Verify the environmental ratings (NEMA 1, 3R, 12, 4X) match the installed environment.

## Common Traps

### Measuring Working Space From the Closed Door
The mechanism is that the working depth is measured from the front face of the closed switchgear door. The false signal is that the clearance meets the nominal dimension. The harm is that when the door is opened for operation or maintenance, the live parts are exposed deeper into the space, the actual clearance to the worker is reduced, and the installation is noncompliant and unsafe, because the code measures from the exposed live part or the opening.

### Allowing Mechanical Systems Into the Dedicated Equipment Space
The mechanism is that a sprinkler pipe, duct, or plumbing line is routed above or through the switchgear footprint to save building space. The false signal is that the systems coexist without immediate conflict. The harm is a slow leak or condensation drip onto energized bus, causing tracking, corrosion, and eventual arc failure, and the installation violates NEC 110.26(E), creating both a code failure and a latent safety hazard that is hard to remedy after the building is finished.

### Installing Rear-Access Gear Against a Wall
The mechanism is that rear-access switchgear is placed against a wall to save floor area. The false signal is that the front access is sufficient for operation. The harm is that cable terminations, thermography, and breaker removal all require rear access that no longer exists, forcing de-energization and relocation of the gear or unsafe workarounds, and the gear cannot be properly maintained for its service life.

### Cramming Large Cables Into Undersized Termination Space
The mechanism is that large conductors are forced into a gutter that lacks the required bending space. The false signal is that the cables landed and the gear energized. The harm is insulation damage at the bend, lug stress that loosens over thermal cycling, and terminations that cannot be re-torqued or inspected, leading to overheated connections and eventual failure at the lug, which is among the most common switchgear failure modes.

### Omitting Floor Sealing and Environmental Protection
The mechanism is that floor penetrations for cables are left unsealed or the gear is set without a housekeeping pad in a damp location. The false signal is that the installation looks complete and dry. The harm is water migration into the cable compartment over time, corrosion of bus and lugs, tracking across wet insulation, and eventual arc failure, and the damage is hidden until a failure occurs because the compartment is rarely opened.

## Self-Check

- Is the NEC 110.26 working space depth, width, and height provided and measured from the exposed live part or opening, not the closed door?
- Is the dedicated equipment space reserved free of all foreign systems per NEC 110.26(E)?
- Are all sections leveled, aligned, anchored, and bolted together with verified bus joint torque?
- Does the termination compartment provide the NEC 312/314 bending and gutter space for the actual conductor size and count?
- Is front and rear access provided as required by the equipment design, with no rear-access gear placed against a wall?
- Are floor openings sleeved, sealed, and fire-rated, with environmental protection (housekeeping pad, NEMA rating) matching the location?
- Were floor penetration locations coordinated with the equipment termination pattern before construction?
- Can enclosure doors open at least 90 degrees without obstruction from adjacent equipment or structures?
