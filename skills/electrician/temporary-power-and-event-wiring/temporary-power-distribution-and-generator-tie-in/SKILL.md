---
name: temporary-power-distribution-and-generator-tie-in.md
description: Use when the agent is planning temporary power distribution for construction sites and events, sizing generators and spider boxes, routing feeders and cam-lock cables, and applying NEC 590 rules and overcurrent protection so that temporary installations remain safe despite exposure, movement, and condensed schedules.
---

# Temporary Power Distribution and Generator Tie-In

Temporary power is built fast, torn down fast, and abused in between. Unlike a permanent installation that is planned, inspected, and left alone, temporary power is moved, re-spliced, driven over, rained on, and overloaded on a daily basis. NEC Article 590 recognizes this reality and grants some relaxations — for example, certain cable types are permitted where they would not be in permanent work — but it also adds strict requirements such as mandatory GFCI that permanent work does not always need. The judgment problem is that the relaxed rules can be read as permission to be careless, when in fact the harsher environment demands more discipline, not less. Agents miss the issues because temporary power looks informal, the deadlines are always tight, and the failures — a tripped breaker, a melted spider box — are treated as operational annoyances rather than warnings about an unsafe distribution design.

## Core Rules

### Apply NEC Article 590 as the Governing Standard, Not a Loosening of All Rules

Article 590 permits specific temporary wiring methods and sets specific time limits — temporary installations for construction, remodeling, maintenance, repair, demolitions, and similar activities, and for experimental work and holiday decorative lighting. It allows cable types such as portable power cable and flexible cords in uses that would be restricted in permanent work, but it does not waive grounding, overcurrent, GFCI, or working clearance requirements. The defense is to treat 590 as a defined scope with both permissions and added duties: use the allowed temporary methods, but apply every applicable safety rule from the rest of the code, and remove the temporary installation within the time limits the article sets.

### Size the Generator to Real Starting and Running Loads, Not Nameplate Totals

A generator must supply not only the running load but the inrush of every motor and transformer on the system. Pumps, compressors, and power tools draw several times their running current at startup, and a generator sized to running watts will sag or stall when a motor starts, dropping voltage that trips electronics and stalls other motors. The defense is to total the running load, add the largest motor's starting kVA, allow for diversity where loads genuinely do not coincide, and then add margin so the generator runs at a fraction of its rating rather than at its limit, because a generator loaded to its nameplate sags and wears prematurely.

### Distribute Through Listed Spider Boxes and Cam-Lock Assemblies Within Ratings

Spider boxes and cam-lock distribution assemblies are the workhorses of temporary distribution, splitting a large feeder into branch circuits with built-in overcurrent. Each device has a maximum current and voltage rating and a specific input configuration, and exceeding any of these overheats the device and its connections. The defense is to verify that the feeder current into each spider box is within its rating, that the branch breakers in the box match the connected loads, and that cam-lock connectors are fully seated and locked, because a partially engaged cam-lock is a high-resistance connection that will arc and melt under load.

### Route Feeders to Protect Cable and People From Mechanical Damage

Temporary feeders cross walkways, roads, and work zones where they are exposed to vehicles, foot traffic, forklifts, and sharp debris. An unprotected cable is a trip hazard, a damage target, and a shock source when crushed. The defense is to route feeders overhead where possible, to use cable ramps or bridge protectors where vehicles must cross, to avoid sharp bends that stress the insulation, and to keep feeders out of standing water and away from heat sources, because a damaged feeder in a temporary setting energizes whatever crushed it.

### Provide Overcurrent Protection at Every Reduction in Conductor Size

The tap rules and feeder rules of the NEC apply to temporary work: wherever a conductor size reduces, overcurrent protection sized to the smaller conductor must be provided. A common temporary error is to feed a small branch cable from a large feeder through a box with no properly sized breaker, so the small cable is protected only by the feeder breaker that is far too large to trip before the small cable burns. The defense is to ensure every spider box, every tap, and every branch takeoff has overcurrent protection sized to the smallest downstream conductor, so that no cable is left unprotected by an oversized upstream device.

### Ground and Bond the Generator Frame and All Distribution Equipment

A generator and all distribution equipment must be grounded and bonded so that a fault clears rather than energizing the frame. Portable generators have specific grounding rules depending on whether they supply cord-and-plug connected equipment from receptacles mounted on the generator, or feed a separate distribution system. The defense is to follow the generator's listing and the NEC grounding rules for its configuration, to bond the neutral where required, to connect the equipment grounding conductors throughout the distribution, and to drive a grounding electrode where the rules require it, because an ungrounded generator frame is an energized hazard at the first fault.

## Common Traps

### Sizing the Generator to Running Watts and Ignoring Motor Inrush

The installer totals the running watts of all tools and lights and selects a generator to that number. The false signal is that the total fits the generator's rating. The mechanism of failure is that the first motor to start draws several times its running current and the generator voltage sags, tripping electronics and stalling other motors. The harm is a generator that cannot actually start the loads it was "sized" for, with repeated trips and stalled tools on the site.

### Overloading a Spider Box Because the Feeder "Fits the Connector"

The installer connects a large feeder to a spider box because the cam-lock fits, ignoring the box's current rating. The false signal is that the connector mates and the box "should" handle whatever the feeder carries. The mechanism of failure is that the box's internal bus and breakers are rated below the feeder current, so the bus overheats and the insulation degrades. The harm is a melted spider box, lost power to the branch circuits, and a fire source in the middle of the work zone.

### Leaving a Partially Engaged Cam-Lock Under Load

The installer seats a cam-lock connector most of the way but does not twist it locked, and the joint carries full feeder current. The false signal is that the connector is "in" and the circuit works. The mechanism of failure is that the partial engagement is a high-resistance contact that arcs and heats under load, melting the housing. The harm is a destroyed connector, an open feeder, and a hot, arcing joint that can ignite nearby material.

### Running Feeders Across Roads Without Protection

The installer lays a feeder across a driveway or access road to save time rigging an overhead run. The false signal is that the cable is heavy-duty and the traffic is light. The mechanism of failure is that the first vehicle crushes the insulation, shorting the feeder and energizing the vehicle or the soil. The harm is a destroyed feeder, a shock hazard, and a sudden loss of power that stops the site.

### Omitting Overcurrent at a Tap Because the Feeder Breaker "Protects Everything"

The installer taps a small branch cable off a large feeder through a junction box with no breaker, reasoning that the feeder breaker protects the whole system. The false signal is that there is a breaker upstream. The mechanism of failure is that the feeder breaker is sized for the large feeder and will not trip before the small branch cable overheats and burns. The harm is an unprotected branch cable that becomes a fire source at the first overload.

### Treating the Generator Frame Ground as Optional on a Portable Unit

The installer runs a portable generator supplying cord-and-plug tools and does not ground the frame or bond the neutral, assuming the generator is self-contained. The false signal is that the tools run fine. The mechanism of failure is that without proper grounding and bonding per the generator's configuration, a fault energizes the frame and any bonded metal, with no path to clear it. The harm is an energized generator frame and tool housings that shock the next person to touch them.

## Self-Check

- Did I apply NEC Article 590 as the governing standard, using the permitted temporary methods while keeping all grounding, overcurrent, GFCI, and clearance rules?
- Did I size the generator to running load plus the largest motor's starting kVA, with margin so it runs below its nameplate?
- Did I verify that every spider box and cam-lock assembly is within its current and voltage rating, and that all cam-locks are fully seated and locked?
- Did I route feeders to avoid mechanical damage, using overhead runs or cable ramps at every road and walkway crossing?
- Did I provide overcurrent protection sized to the smallest downstream conductor at every tap and branch takeoff?
- Did I ground and bond the generator frame and all distribution equipment per the generator's listing and configuration, including a grounding electrode where required?
- Did I confirm GFCI protection on all required temporary receptacles per NEC 590.6?
- Is the temporary installation documented and scheduled for removal within the article's time limits?
