---
name: busway-run-planning-and-joint-assembly.md
description: Use when the agent is planning busway (bus duct) runs, specifying fittings and expansion joints, assembling busway sections, setting hanger spacing, and applying ampacity derating for feeder and plug-in busway installations.
---

# Busway Run Planning and Joint Assembly

Busway is appealing because it carries high current in a compact, factory-built enclosure, but its reliability depends almost entirely on run planning and joint assembly, which are field operations. The judgment problem is that busway expands thermally, must be supported at precise intervals, must penetrate walls and floors without being constrained, and depends on each joint being torqued and aligned exactly. Agents tend to treat busway like pipe: cut to fit, hang it, bolt it. But a busway run that ignores expansion, binds at a wall penetration, or has one undertorqued joint will overheat, deform, and fail under load. The skill exists to force the agent to plan the run as a thermal and mechanical system and to assemble joints as precision connections, not pipe fittings.

## Core Rules

### Lay Out the Run With Fittings, Expansion, and Penetrations Planned
Before any section is ordered, lay out the complete run including straight sections, elbows, tees, offsets, expansion joints, wall and floor flanges, and tap points. Identify every structural obstruction, every penetration, and every point where the busway crosses a building expansion joint, because the busway must accommodate building movement independently. Plan hanger locations against the manufacturer's maximum spacing and against the structural capacity to support the weight and the electromagnetic fault forces. A run planned section-by-section in the field inevitably has a fitting that does not fit and a joint in an inaccessible location, which is where failures concentrate.

### Provide Expansion Joints at the Required Intervals
Busway expands longitudinally with load current and ambient temperature, typically on the order of 1 to 2 millimeters per 10 meters per 10 degrees C for aluminum. The manufacturer specifies the maximum straight run length between expansion fittings (often around 30 meters, but verify), beyond which an expansion joint is mandatory. Anchor the busway at one end of each expansion zone and support it on sliding hangers elsewhere so that the expansion is directed into the joint, not into the structure or the next fitting. An expansion joint omitted or installed in a bound (non-sliding) condition causes the busway to buckle, stressing joints and supports until a failure occurs.

### Detail Wall and Floor Penetrations to Allow Movement
Where busway penetrates a wall or floor, the opening must be sized to allow the busway to move through its thermal expansion range and to allow building movement, and it must be fire-stopped with a listed assembly that accommodates that movement. A busway grouted rigidly into a wall cannot expand and will transmit force into the wall and into its own joints. Use a sleeve or a wall flange with clearance, firestop with a flexible listed material, and never use the penetration as an anchor point unless the design specifically calls for it. Coordinate the penetration size with the firestop manufacturer, because the listed assembly depends on the annular space.

### Torque and Align Every Joint to Specification
Busway joints are the dominant failure point in service, and their reliability depends on exact alignment and torque. Mate the sections squarely, verify that the joint faces and the bus bars are clean and undamaged, install the joint hardware with any specified joint compound, and torque to the manufacturer's value using a calibrated torque wrench. Re-torque after thermal cycling where the manufacturer requires it. A joint that is undertorqued, cross-threaded, or assembled with dirty or damaged faces develops high resistance, heats, oxidizes, and fails; a joint overtorqued can distort the bar and reduce contact area, also leading to heating.

### Set Hanger Spacing and Type per the Manufacturer and the Fault Forces
Hangers must be spaced no farther than the manufacturer's maximum (commonly 1.5 to 3 meters depending on rating), must support the busway without imposing point loads that distort the casing, and must withstand the electromagnetic forces of a fault. At high fault currents the busway experiences large lateral forces, and the hangers and their anchors into the structure must be rated for those forces, not just the static weight. Use spring hangers or sliding supports at expansion zones and rigid anchors only at the designated anchor points. Verify the structural anchor capacity, because a hanger that pulls out of the structure during a fault drops the energized busway.

### Distinguish Feeder From Plug-In Busway and Apply Each Correctly
Feeder busway is a solid run for transferring bulk power from source to distribution point, with no tap provisions. Plug-in busway has openings along its length for plug-in tap units that feed local loads, and it is used where distributed takeoffs are needed. Do not substitute one for the other: plug-in busway used as a feeder has unnecessary joints and tap openings that are failure and exposure points, while feeder busway cannot accept the plug-in units a distributed layout requires. Select the type from the load distribution pattern, and verify the plug-in busway has the correct number and orientation of tap openings for the planned loads.

### Apply Ampacity Derating for Temperature and Configuration
Busway ampacity is rated at a standard ambient (commonly 40 degrees C) and a specific configuration. Derate for higher ambient temperature per the manufacturer's table, and derate for enclosed or stacked runs where multiple busways in close proximity reduce cooling. A run installed in a hot mechanical room or in a dense tray with other busways may carry significantly less than nameplate, and operating it at nameplate current causes excessive temperature rise, insulation aging, and joint degradation. Verify the installed configuration against the derating table and document the effective ampacity, because the nameplate value alone is not the safe continuous rating in all conditions.

## Common Traps

### Omitting an Expansion Joint on a Long Straight Run
The mechanism is that a long run is assembled without an expansion fitting because the length appears within tolerance. The false signal is that the busway fits and operates normally at low load. The harm is that as load and temperature cycle, the busway expands against rigid end terminations and penetrations, buckles, stresses the joints, and eventually fails a joint or tears a support, because the accumulated expansion over a long run exceeds what the structure can absorb without a dedicated expansion fitting.

### Grouting a Wall Penetration Rigid
The mechanism is that the annular space around a busway penetration is filled with rigid grout or mortar to seal it. The false signal is a clean, sealed penetration. The harm is that the busway cannot expand or move, so thermal and building forces transfer into the busway joints and into the wall, cracking the wall and overstressing joints, and the rigid seal defeats the fire rating because the listed assembly required a flexible, movement-accommodating material.

### Assembling a Joint Without a Calibrated Torque Wrench
The mechanism is that joints are tightened by feel or with an uncalibrated tool to save time. The false signal is that the joint is tight and the run energizes normally. The harm is a joint at the wrong torque that overheats progressively, because the contact resistance is far more sensitive to torque than to feel, and the joint fails under load months or years later, often at the worst possible time and in an inaccessible location.

### Using Feeder Busway Where Plug-In Taps Are Needed
The mechanism is that feeder busway is installed in a distributed-load area to save cost or by mistake. The false signal is that power reaches the end of the run. The harm is that the planned local tap-offs cannot be made, because feeder busway has no tap openings, and the fix requires either field modification that voids the listing or a costly redesign with plug-in busway, because the two types are not interchangeable.

### Ignoring Derating in a Hot or Dense Installation
The mechanism is that busway is loaded to nameplate current in a high-ambient or densely packed run without derating. The false signal is that the current is within the nameplate rating. The harm is excessive temperature rise that ages insulation, increases joint resistance, and shortens service life, because the nameplate assumes a standard condition that the installation does not meet, and the effective safe ampacity is lower than the label.

## Self-Check

- Is the complete run laid out with all fittings, expansion joints, penetrations, and tap points planned before sections are ordered?
- Are expansion joints provided at or below the manufacturer's maximum straight-run interval, with correct anchoring and sliding supports?
- Are wall and floor penetrations sized for movement and fire-stopped with a listed, flexible assembly?
- Is every joint assembled square, clean, and torqued to specification with a calibrated wrench, with re-torque where required?
- Are hangers within maximum spacing, of the correct type (sliding versus anchor), and rated for the fault electromagnetic forces?
- Is the busway type (feeder versus plug-in) matched to the load distribution, with the correct number and orientation of tap openings?
- Has ampacity been derated for the actual ambient and configuration, with the effective rating documented?
- Are structural anchors verified for both static weight and fault forces, not just static load?
