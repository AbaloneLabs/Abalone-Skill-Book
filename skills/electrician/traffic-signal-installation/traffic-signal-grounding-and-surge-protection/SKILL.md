---
name: traffic-signal-grounding-and-surge-protection.md
description: Use when the agent is grounding traffic signal installations, bonding poles cabinets and bases, installing surge protection at the service and loop inputs, protecting ITS and communication links, or applying NEC Article 645 grounding principles to traffic controller electronics in roadside cabinets.
---

# Traffic Signal Grounding and Surge Protection

Traffic signal installations sit at the roadside, exposed to lightning, utility surges, and inductive coupling, while housing sensitive controller electronics that must run 24 hours a day for years without interruption. The judgment problem is that grounding and surge protection in a traffic cabinet are not optional reliability features — they are the difference between an intersection that survives a thunderstorm and one that loses its controller, its detectors, and its communication in a single strike, leaving the intersection dark or stuck in flash for days. An agent that drives one ground rod, bonds the cabinet loosely, omits surge protection on the loops and communication lines, or treats the cabinet ground as an afterthought will produce an installation that works fine in clear weather and fails catastrophically in the first storm, and the root cause is nearly always misdiagnosed as a bad controller board.

## Core Rules

### Establish a Low-Impedance Cabinet Ground With a Driven Rod and Bonded System

The controller cabinet must connect to earth through a low-impedance grounding electrode system so that surge energy is diverted to ground rather than through the electronics. The defense is to drive a ground rod (or the specified electrode system, which may include a ground ring or concrete-encased electrode) at the cabinet, bond the cabinet ground bus to the electrode with an appropriately sized conductor (commonly #6 AWG or larger), verify the ground resistance meets the agency specification (often 25 ohms or less, or 5 ohms for ITS cabinets), and bond all racks, chassis, and equipment to the cabinet ground bus. A single loose or high-impedance ground connection defeats the entire surge protection scheme.

### Bond Every Pole and Base to the Grounding System

Each signal pole, pedestrian pole, and cabinet base is a metallic structure exposed to lightning and fault current, and it must be bonded to the grounding system so that a surge or fault on the pole is diverted to ground rather than energizing the pole to a dangerous voltage. The defense is to bond each pole base to the grounding electrode or to the cabinet ground through the equipment grounding conductor in the feeder, verify continuity from each pole to the ground system, and ensure the bond is mechanically secure and corrosion-resistant. An unbonded pole can rise to lethal voltage during a surge or a fault.

### Install Surge Protection at the Service Entrance

The cabinet's power service entrance is the primary path for utility surges and lightning, and surge protection at this point diverts the bulk of the energy before it reaches the controller power supply. The defense is to install a listed surge protective device (SPD) at or near the service disconnect, sized for the cabinet voltage and surge environment (commonly Type 1 or Type 2, with an appropriate surge current rating of 40 to 100 kA or higher for lightning-prone areas), connect it with short, straight leads to the bus, and verify the SPD status indicator is visible and monitored. Long SPD leads add inductance that reduces the clamping effectiveness.

### Protect Loop Detector Inputs and Field Circuits From Induced Surges

Loop lead-in cables run in the pavement and pull boxes are antennas for lightning-induced surges, and these surges reach the detector cards and can destroy them or the controller backplane. The defense is to install surge protection on the loop inputs at the cabinet (loop surge modules or detector channels with built-in protection), route loop lead-in away from power conductors, keep the lead-in runs as short as possible inside the cabinet, and use twisted, shielded lead-in where specified. Unprotected loop inputs are the most common path for lightning damage to detector electronics.

### Protect Communication and ITS Links With Appropriate Surge Devices

Communication links (serial, Ethernet, fiber, wireless) between the cabinet and the central system or adjacent cabinets are surge paths, and copper communication links are especially vulnerable. The defense is to install surge protection on every copper communication line at the cabinet entry (data line protectors matched to the signal type and voltage), use fiber optic links where possible to eliminate the copper surge path entirely, bond the surge protector to the cabinet ground with a short lead, and verify the protector does not distort the signal at the required data rate. A communication link without surge protection fails in the first storm.

### Apply NEC Article 645 Principles to Controller Electronics Grounding

While NEC Article 645 formally covers information technology equipment rooms, its principles — a dedicated equipment grounding system, bonding of all racks and chassis, separation of signal ground from power ground except at the main bond, and surge protection at the power entry — apply directly to traffic controller cabinets, which are essentially small IT rooms in a roadside enclosure. The defense is to treat the cabinet electronics ground as a dedicated system bonded to the main grounding point, avoid using the neutral as a ground reference, keep signal and communication grounds separate from the switching (load switch) grounds except at the bond, and verify there is no ground loop that injects noise into the controller.

## Common Traps

### Driving One Ground Rod and Calling It Done Without Measuring Resistance

The installer drives a single ground rod, bonds the cabinet, and moves on without measuring the resistance. The mechanism of the failure is that a single rod in dry or rocky soil may measure 50 to 100 ohms or more, far above the effective range for surge diversion, so surge energy sees a high-impedance path and instead travels through the controller electronics to find ground, damaging boards. The false signal is that the rod "is driven," implying a ground, when the impedance is too high to function. The harm is repeated electronics damage in storms attributed to bad boards rather than the inadequate ground.

### Leaving a Pole Base Unbonded and Creating a Shock Hazard

The signal pole is set on its base, the conductors are pulled, but the equipment grounding conductor is not bonded to the pole base, or the bond is loose. The mechanism of the failure is that a surge or a fault on the pole circuit has no low-impedance path to ground through the pole, so the pole structure rises to a dangerous voltage, presenting a shock hazard to anyone touching it and preventing the overcurrent device from clearing the fault. The false signal is that the pole "is wired," implying it is grounded, when the bond was omitted. The harm is a lethal shock hazard and a fault that does not clear.

### Omitting the Service SPD or Installing It With Long Leads

The cabinet has no surge protection at the service entrance, or the SPD is installed with several feet of lead to reach a convenient bus location. The mechanism of the failure is that without a service SPD, the full utility surge or lightning strike reaches the controller power supply and destroys it; with long SPD leads, the lead inductance develops a large voltage drop during the surge (roughly 100 to 200 volts per foot at surge current), so the clamped voltage at the bus is far higher than the SPD rating, defeating the protection. The false signal is that the SPD "is installed," implying protection, when the long leads render it ineffective. The harm is controller power supply failure in the first surge event.

### Leaving Loop Inputs Unprotected and Losing Detector Cards in a Storm

The loop lead-in cables are terminated directly to the detector rack with no surge protection. The mechanism of the failure is that a nearby lightning strike induces a surge in the loop lead-in (which acts as an antenna), the surge reaches the detector card input, and the input circuit is destroyed, taking out one or more detector channels and often the controller backplane. The false signal is that the loops "are working," implying the wiring is fine, when the surge path is unprotected. The harm is detector card and controller damage in every thunderstorm, with chronic detection failures.

### Protecting Power but Leaving the Copper Communication Link Exposed

The cabinet power has a service SPD, but the copper Ethernet or serial link to the central system enters the cabinet with no surge protection. The mechanism of the failure is that a surge on the long communication cable (induced by lightning or coupled from a parallel power line) reaches the controller communication port and destroys it, severing the central connection even though the power side survived. The false signal is that "the cabinet is surge-protected," implying comprehensive protection, when the communication path was ignored. The harm is loss of central control and monitoring after every storm, requiring a communication board replacement.

### Creating a Ground Loop by Bonding Signal Ground to Power Ground at Multiple Points

The installer bonds the signal ground bus to the cabinet frame at the rack and again at the power entry, creating two bond points. The mechanism of the failure is that the two bond points form a ground loop, and switching noise from the load switches circulates in the loop and appears on the signal ground, causing false detections, communication errors, and intermittent controller faults. The false signal is that "more grounding is better," implying robustness, when the multiple bonds created a noise-injecting loop. The harm is chronic intermittent electronics failures that resist diagnosis.

## Self-Check

- Did I establish a low-impedance cabinet ground with a driven rod (or specified electrode system), bond the ground bus with an appropriately sized conductor, and measure the resistance to verify it meets the agency specification?
- Did I bond every signal pole, pedestrian pole, and base to the grounding system, verify continuity from each pole to ground, and ensure the bonds are mechanically secure and corrosion-resistant?
- Did I install a listed surge protective device at the service entrance, sized for the voltage and surge environment, with short straight leads to the bus, and is the SPD status indicator visible and monitored?
- Did I install surge protection on every loop detector input at the cabinet, route loop lead-in away from power, and use twisted shielded lead-in where specified?
- Did I install surge protection on every copper communication line at the cabinet entry, matched to the signal type, or use fiber to eliminate the copper surge path entirely?
- Did I apply single-point bonding so the signal ground, power ground, and chassis ground meet at one main bond with no ground loops, and verify no noise circulates between them?
- Did I verify the complete grounding and surge protection system by inspection and, where possible, by surge injection testing or resistance measurement, before turning the cabinet over?
- Is the grounding and surge protection documented with electrode type, resistance measurement, SPD ratings and locations, and the bonding diagram, so the protection scheme is traceable and maintainable?
