---
name: service-entrance-and-metering.md
description: Use when the agent is sizing service-entrance conductors, choosing between overhead and underground service, selecting meter sockets and meter mounting arrangements, or establishing grounding and bonding at the service disconnect for residential, commercial, or industrial service installations.
---

# Service Entrance and Metering

The service entrance is the single point where utility power enters a building, and it is also the only point where the grounding and bonding strategy ties the premises wiring to earth through the grounding electrode system. Every decision made here, conductor size, overhead versus underground routing, meter socket type, and the location of the main disconnect, is hard to change after the utility has energized the service. The judgment problem is that service entrance work sits at the boundary between utility requirements, code requirements, and the premises wiring, and these three authorities do not always agree. An electrician who treats the service as a straightforward conductor pull will misapply the neutral-to-ground bond, undersize the service for future load, or install a meter arrangement the utility will not accept. This skill covers the decisions that determine whether the service entrance is safe, code-compliant, utility-acceptable, and sized for the life of the building.

## Core Rules

### Size Service Conductors From the Calculated Load, Not the Service Rating Alone

Service-entrance conductors are sized to carry the calculated load after demand factors, and they must have an ampacity not less than the service disconnect rating under the standard method. Under the optional or neutral calculations, different demand factors apply, and the conductors must still be adequate for the load they actually carry. The trap is sizing the conductors to the service disconnect nameplate and assuming the job is done, when the calculated load may require a larger conductor to handle continuous loads at 125 percent, or when voltage drop on a long service lateral requires upsizing. Service conductors that are marginally sized run hot at the lugs, accelerate insulation aging, and leave no room for the load growth that almost always occurs.

The defense is to perform the load calculation first, apply the 125 percent continuous-load rule, check voltage drop on long runs, and then confirm the selected conductor ampacity equals or exceeds the service disconnect rating. Where conductors are upsized for voltage drop, the equipment grounding conductor or, at the service, the grounded (neutral) conductor must be upsized proportionally.

### Choose Overhead Versus Underground on Long-Term Reliability, Not First Cost

Overhead services are cheaper and faster to install and easier to repair, but they are exposed to wind, ice, tree contact, and vehicle damage. Underground services are protected from weather and vegetation but are vulnerable to dig-ins, are harder and far more expensive to locate and repair, and require conduit or direct-burial cable rated for the burial environment. The choice should be driven by the site conditions and the reliability expectation over the service life, not by the installed cost alone. Utilities often dictate the available point of connection and the routing, and the electrician must coordinate the utility's requirements with the premises routing before trenching or setting poles.

The trap is defaulting to overhead because it is cheaper, then accepting repeated outage and damage risk on a site where underground was clearly the better long-term choice, or installing underground without proper depth, warning tape, and conduit protection and then suffering a dig-in failure. The defense is to evaluate exposure to vegetation, traffic, and future excavation, to follow the utility's service specifications exactly, and to install underground with the required burial depth, warning tape, and conduit where specified.

### Match the Meter Socket to the Service Type, Rating, and Utility Specification

Meter sockets are not interchangeable. A single-phase 200 A residential socket, a 320 A continuous-duty socket, a network meter for certain commercial services, and a CT (current transformer) cabinet for large services are all different hardware, and the utility specifies which is acceptable for a given service. Installing the wrong socket, the wrong number of jaws, or a socket rated below the service current will be rejected by the utility and will not pass inspection. Sockets must also be rated for the environment, including NEMA 3R for outdoor locations and appropriate ratings for corrosive atmospheres.

The trap is buying a meter socket from the supply house based on amperage alone, without confirming the utility's approved socket list, the number of terminals, and whether a bypass (horn or lever) is required. The false signal is that the socket fits the conductor and the amperage, which proves nothing about utility acceptance. The defense is to obtain the utility's service requirements document before purchasing the socket and to verify the catalog number against the approved list.

### Establish the Main Disconnect Location and the Service Point Clearly

The service point is where utility ownership ends and premises wiring begins, and the location of the main disconnect determines what conductors are service-entrance conductors (outside the building, generally) versus feeders. The distance permitted between the meter and the main disconnect, and whether an outside disconnect is required, is governed by code and increasingly by utility and local requirements for emergency responder access. Conductors on the line side of the main disconnect are service conductors and have different protection rules than feeders; they generally cannot be run inside the building for long distances without an outside disconnect or meeting specific tap rules.

The trap is running service-entrance conductors deep into a building to reach a convenient indoor panel, violating the distance limits and creating unprotected conductors inside the structure. The defense is to locate the main disconnect as close as practical to the service point, to install an outside disconnect where required, and to treat anything past the main disconnect as a feeder with feeder protection rules.

### Make the Grounding and Bonding Decisions at the Service, Not Elsewhere

The service disconnect is the one and only place where the grounded (neutral) conductor is bonded to the equipment grounding conductors and to the grounding electrode system. This main bonding jumper creates the effective fault-clearing path and ties the system to earth. Downstream of the service, neutral and ground must be separated to prevent neutral current from flowing on equipment grounding conductors and metal raceways. The grounding electrode system, whether a ground rod, concrete-encased electrode, water pipe, or building steel, must be connected at the service, and the bonding jumper to the water pipe must be sized to the service-entrance conductor area.

The trap is bonding neutral and ground together at a subpanel, at a separate structure, or at a generator, which puts load current on grounding conductors and metal piping and creates shock and fire risk. The defense is to install the main bonding jumper only at the service disconnect, to keep the neutral isolated from ground at all downstream panels, and to verify the grounding electrode conductor and bonding jumpers are sized and connected correctly.

### Coordinate With the Utility Before Ordering Any Service Equipment

The utility owns the service drop or lateral up to the service point, sets the meter, and dictates the point of attachment, clearance requirements, conductor size and type at the connection, and the approved metering arrangement. Equipment ordered without this coordination is frequently wrong, and the utility can refuse to energize until it is corrected. This includes the service mast height and guying for overhead, the conduit sweep radius and number of bends for underground, and the meter socket type and bypass arrangement.

The trap is proceeding on assumptions about the service point and clearances, then discovering at energization that the mast is too short, the clearance to windows is insufficient, or the socket is not on the approved list. The defense is to request and follow the utility's service requirements in writing before purchasing or installing any service equipment.

## Common Traps

### Sizing Service Conductors to the Disconnect Nameplate Only

An electrician reads the 200 A rating on the service disconnect and pulls 2/0 AWG copper because that matches 200 A in the ampacity table, without performing the load calculation or checking continuous load and voltage drop. The mechanism of the trap is that the nameplate rating is the maximum the disconnect can handle, not necessarily the conductor size the actual load and conditions require. The false signal is the table match between ampacity and rating, which looks correct but ignores that continuous loads need 125 percent, long laterals need voltage-drop upsizing, and the calculated load may exceed what the demand factors suggested. The harm is conductors that run hot, lug failures, and a service that cannot safely carry its real-world load. The defense is to size from the calculated load first, then confirm against the disconnect rating.

### Running Service Conductors Deep Into the Building

To reach a convenient basement panel, an electrician runs service-entrance conductors, which are unfused on the line side, forty feet inside the building from the meter to the main disconnect. The mechanism of the trap is that service conductors have minimal overcurrent protection and are treated more permissively than feeders precisely because they are expected to be short and outside the building; extending them deep inside removes that protection without providing feeder-rated protection. The false signal is that the conductors are in conduit and look protected, which addresses physical damage but not the absence of overcurrent protection. The harm is unprotected energized conductors running through occupied space, where a fault has no fast clearing path. The defense is to install the main disconnect near the service point, often outside, and convert the run to the panel into a feeder with proper overcurrent protection.

### Bonding Neutral and Ground at a Subpanel or Separate Structure

At a subpanel in a detached garage, the electrician installs a bonding screw linking the neutral bar to the enclosure, replicating the service panel arrangement. The mechanism of the trap is that the neutral-to-ground bond is permitted only at the service; downstream, any bond creates a parallel path for neutral current through the equipment grounding conductor, the metal raceway, and any bonded metal piping. The false signal is that the panel "needs a ground," which conflates equipment grounding with neutral bonding. The harm is current on metal parts, elevated touch voltage, and interference with fault-clearing paths, plus a shock hazard for anyone touching bonded metal and earth simultaneously. The defense is to keep neutral and ground separated at every panel downstream of the service disconnect and to install a grounding electrode system at separate structures without bonding it to the neutral.

### Installing a Meter Socket the Utility Will Not Accept

The electrician buys a 200 A meter socket that fits the conductors and the amperage, but the utility requires a specific bypass type, a continuous-duty rated socket, or a socket from its approved list, and the installed socket is rejected at the connection inspection. The mechanism of the trap is that metering hardware is utility-specific and code approval does not guarantee utility acceptance. The false signal is that the socket meets the rating and the code, which is necessary but not sufficient. The harm is a refused energization, a wasted socket, and a rework that delays occupancy. The defense is to obtain the utility service requirements and the approved socket list before purchase.

### Undersizing the Grounding Electrode Conductor or Bonding Jumper

The electrician installs a small grounding electrode conductor, say 6 AWG copper, to a driven rod, and uses the same small conductor to bond the metal water pipe, because the rod conductor table allows 6 AWG. The mechanism of the trap is that the 6 AWG allowance applies specifically to the rod, pipe, or plate electrode connection, but the bonding jumper to metal water piping and structural steel must be sized from the service-entrance conductor area and can be much larger. The false signal is the rod table value, which is correct for the electrode but wrongly applied to the bonding jumper. The harm is an undersized bonding path that cannot survive fault current, leaving metal piping energized during a fault. The defense is to size the bonding jumper to the service conductor area per the table, not to the rod conductor size.

### Choosing Overhead on Cost Where Underground Was the Right Call

On a wooded lot with frequent ice storms, the electrician installs an overhead service because it is cheaper, and within two years a tree limb takes it down during a storm, repeated twice more over the next decade. The mechanism of the trap is that first-cost thinking ignores the exposure that made underground the better choice. The false signal is the lower bid, which wins the job but loses over the service life. The harm is repeated outages, repair costs, and damage to the building mast and weatherhead. The defense is to weigh vegetation, weather, and traffic exposure against the higher underground first cost and to recommend underground where exposure is high.

## Self-Check

- Did I size the service-entrance conductors from the calculated load with continuous loads at 125 percent, check voltage drop on long runs, and then confirm the ampacity equals or exceeds the service disconnect rating?
- Did I choose between overhead and underground based on long-term site exposure to vegetation, weather, and traffic, and did I follow the utility's service routing and clearance specifications exactly?
- Did I confirm the meter socket catalog number is on the utility's approved list, matches the service type and amperage, and has the required bypass and environmental rating before purchase?
- Did I locate the main disconnect as close as practical to the service point, and did I avoid running unfused service-entrance conductors deep into the building by converting the run to a feeder past the disconnect?
- Did I install the main bonding jumper only at the service disconnect, keep the neutral isolated from ground at all downstream panels, and avoid bonding neutral to ground at subpanels or separate structures?
- Did I size the grounding electrode conductor to the electrode type and the bonding jumper to metal water piping and structural steel from the service-entrance conductor area, not from the rod conductor table?
- Did I obtain the utility's written service requirements and coordinate the service point, clearances, mast height, and metering arrangement before ordering or installing any service equipment?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
