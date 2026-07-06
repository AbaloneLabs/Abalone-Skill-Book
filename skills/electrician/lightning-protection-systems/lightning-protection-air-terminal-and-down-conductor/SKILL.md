---
name: lightning-protection-air-terminal-and-down-conductor.md
description: Use when the agent is designing and installing lightning protection systems, placing air terminals as strike termination devices, routing down conductors, applying the rolling sphere method, and complying with NFPA 780 class of LPS so that a strike is captured and safely conducted to ground.
---

# Lightning Protection Air Terminal and Down Conductor

A lightning protection system does not prevent strikes — it invites them, on its own terms. The entire purpose of an LPS is to provide a preferred, controlled path for the lightning current so that it attaches to an air terminal, travels down a dedicated conductor, and dissipates into the ground electrode system instead of through the building structure, the wiring, or the occupants. The judgment problem is that the system only works if the strike attaches to the terminal rather than the structure, and that depends entirely on correct placement using the rolling sphere method, and on routing down conductors so the current never arcs to nearby metal. Agents miss the issues because the components look simple — some rods, some wire, some ground rods — and the system "passes" a visual inspection, while the geometry that determines whether it actually intercepts a strike is never verified against NFPA 780.

## Core Rules

### Place Air Terminals Using the Rolling Sphere Method at the Correct Radius

The rolling sphere method models the lightning leader's final jump as a sphere of a specific radius rolling over the building; any point the sphere touches is a possible strike point and must be protected by an air terminal, while points the sphere cannot touch are shielded. The radius depends on the NFPA 780 class of protection — a smaller radius gives more stringent protection for higher-risk structures. The defense is to select the class based on the structure's risk, to apply the corresponding sphere radius to every roof edge, ridge, parapet, and protrusion, and to place air terminals so that no point on the roof falls outside the rolling-sphere protection zone, because a single unprotected point is where the strike will attach to the structure instead of the system.

### Treat Air Terminals as Strike Termination Devices, Not "Attractors"

Air terminals do not attract lightning from a distance; they provide a preferred attachment point only when the leader is already within the final tens of meters. The implication is that an air terminal protects only the zone defined by the rolling sphere around it, not the whole roof, and that taller or more numerous terminals are needed for large or complex roofs. The defense is to abandon the idea that a few tall rods protect everything, to compute the actual protection zones, and to add terminals at every high point, edge, and corner that the sphere can touch, including on cupolas, mechanical equipment, and roof-mounted arrays.

### Route Down Conductors to Distribute Current and Minimize Impedance

Lightning current is enormous — tens of kiloamperes in microseconds — and it follows the path of least impedance, which at those frequencies means the path with the fewest sharp bends and the shortest length. Down conductors must be routed as directly as possible from the air terminals to the ground electrode system, with no sharp corners (which raise impedance and invite side-flash), and multiple down conductors should be distributed around the building to split the current and reduce the magnetic field inside. The defense is to route down conductors in straight or gently curved paths, to use the number of down conductors NFPA 780 requires based on building perimeter, to space them around the structure, and to keep them away from other metal that a side-flash could bridge.

### Bond the LPS to the Building Grounding Electrode System Per NFPA 780

A lightning protection system that is isolated from the building's electrical grounding electrode system can develop a huge voltage difference during a strike, driving a side-flash from the down conductor into the building's wiring or structural steel. NFPA 780 requires the LPS ground to be bonded to the building grounding electrode system to equalize potential. The defense is to bond the LPS down conductor ground terminals to the building's main grounding electrode, to any metallic water pipe and structural steel used as electrodes, and to do so with conductors sized per 780, so that during a strike the entire building rises and falls together rather than arcing between systems.

### Size the System to the NFPA 780 Class of LPS

NFPA 780 defines classes of lightning protection (for ordinary structures, the relevant classes correspond to different protection levels and rolling-sphere radii). The class is chosen based on the structure's risk — its height, occupancy, contents, location, and consequence of a strike. The defense is to perform or follow the risk assessment, to select the class that matches the structure's risk profile, and to apply that class's requirements consistently to terminal spacing, down conductor count, and conductor sizing, because a system built to a lower class than the risk demands provides a false sense of protection.

### Maintain Separation Distance to Prevent Side-Flash to Building Metal

Where the LPS cannot be bonded to nearby metal — for example, where a down conductor passes a metal downspout or rebar that is not part of the grounding system — a minimum separation distance must be maintained so the lightning voltage cannot arc across the gap. The required separation distance depends on the voltage, the surge impedance, and the medium. The defense is to calculate the separation distance per NFPA 780 for each location where the down conductor parallels unbonded metal, to maintain that distance physically, or to bond the metal if bonding is the safer option, because a side-flash through the building is exactly the damage the LPS exists to prevent.

## Common Traps

### Placing a Few Tall Rods and Assuming the Whole Roof Is Protected

The installer puts a few air terminals on the high points and assumes they protect everything. The false signal is that the rods are tall and visible. The mechanism of failure is that the rolling sphere reaches roof edges, corners, and equipment that the rods' protection zones do not cover, so a strike attaches to the structure instead of a terminal. The harm is structural damage, fire, or injury from a strike the system was believed to intercept.

### Routing a Down Conductor With Sharp Bends to Avoid Obstructions

The installer routes the down conductor around a parapet or pipe with a sharp right-angle bend to make the run fit. The false signal is that the conductor is continuous and connected. The mechanism of failure is that the sharp bend raises the surge impedance at that point, increasing the voltage and the likelihood that the current arcs (side-flashes) to nearby metal rather than following the bend. The harm is a side-flash into the building structure or wiring, defeating the controlled path the down conductor was meant to provide.

### Isolating the LPS Ground From the Building Ground to Avoid "Interference"

The installer leaves the LPS ground separate from the building electrical ground, fearing that bonding them will bring lightning into the electrical system. The false signal is that isolation seems safer. The mechanism of failure is that during a strike the LPS ground rises to a huge voltage while the electrical ground stays near earth, and the difference arcs from the LPS into the wiring, structural steel, or anything bonded to the electrical ground. The harm is exactly the side-flash and equipment destruction that bonding per NFPA 780 prevents.

### Using Fewer Down Conductors Than the Building Perimeter Requires

The installer runs one or two down conductors to save material on a large building. The false signal is that there is a path to ground. The mechanism of failure is that the current is concentrated in too few conductors, raising their voltage and the magnetic field inside the building, and increasing the chance of side-flash. The harm is higher voltages, greater side-flash risk, and induced surges in building wiring, because the current was not split across the number of paths 780 specifies.

### Ignoring Roof-Mounted Equipment and Protrusions in the Layout

The installer protects the roof surface but not the mechanical units, skylights, or PV arrays that protrude above it. The false signal is that the roof is protected. The mechanism of failure is that the rolling sphere touches those protrusions, which become the strike point, and the current enters the equipment and the building wiring. The harm is damaged equipment, fire, and a strike path through the very systems the LPS should have bypassed.

### Undersizing Conductors Relative to the Class of Protection

The installer uses smaller conductor sizes than the class requires to save cost. The false signal is that the conductor "should handle" the current. The mechanism of failure is that lightning current melts or vaporizes an undersized conductor, opening the path mid-strike and forcing the current into the structure. The harm is a failed protection path during the one event it was built for, with the current finding its own destructive way to ground.

## Self-Check

- Did I select the NFPA 780 class of LPS based on the structure's risk, and apply its rolling-sphere radius to every roof surface?
- Did I place air terminals so that no roof point — including edges, ridges, parapets, and protrusions — falls outside the rolling-sphere protection zone?
- Did I route down conductors in straight or gently curved paths with no sharp bends, distributed around the building per 780's perimeter rules?
- Did I bond the LPS ground to the building grounding electrode system with conductors sized per 780, rather than leaving it isolated?
- Did I calculate and maintain the required separation distance wherever a down conductor parallels unbonded metal, or bond the metal where bonding is safer?
- Did I include roof-mounted equipment, skylights, and arrays in the protection layout so the rolling sphere cannot touch unprotected metal?
- Did I size all conductors and components to the class of protection, with no undersized path that could fail during a strike?
- Is the LPS design documented with terminal placement, down conductor routes, and bonding points so it can be inspected and maintained against 780?
