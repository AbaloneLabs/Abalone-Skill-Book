---
name: mcc-bus-and-vertical-horizontal-riser-sizing.md
description: Use when the agent is sizing motor control center horizontal and vertical buses, calculating full-load and demand current, and specifying short-circuit bracing, ampacity, and temperature rise ratings for MCC sections.
---

# MCC Bus and Vertical-Horizontal Riser Sizing

The bus in a motor control center is what carries fault current and full-load current simultaneously across many sections, and its sizing is where latent defects hide for years. The judgment problem is that the bus must be rated not only for the connected load but for the available fault current, the ambient temperature inside the enclosure, the future load growth, and the mechanical bracing that keeps the bus intact during a short circuit. Agents tend to add up nameplate currents and pick a bus one size up, ignoring demand diversity, thermal rise, and bracing. The result is a bus that runs hot, sags under fault, or must be replaced wholesale when one more motor is added.

## Core Rules

### Size the Main Horizontal Bus on Demand, Not Connected Load
The main horizontal bus that runs the length of the MCC does not carry the sum of all motor nameplate currents simultaneously. Apply a demand factor based on the actual duty cycle and diversity of the connected loads, derived from a load study or the NEC feeder calculation method for multiple motors. Size the horizontal bus to the calculated demand plus a margin for future growth, typically 20 to 25 percent, because upsizing the horizontal bus after installation is essentially impossible. The connected-load approach produces a bus that is either wastefully oversized or, more often, undersized for real operating diversity.

### Rate the Vertical Bus per Section Independently
Each vertical section has its own riser bus feeding the buckets in that column, and it carries only the load of that column. Size the vertical bus to the column demand plus margin, but recognize that the vertical bus is also the path through which fault current reaches the buckets, so its bracing rating must match the worst-case fault at that location. A common error is to specify a low vertical bus ampacity to save cost while keeping the bracing high; both ratings must be checked independently because ampacity and bracing are separate failure modes.

### Specify Short-Circuit Bracing Equal to the Available Fault Current
The bus bracing rating is the mechanical withstand against electromagnetic forces during a fault, expressed in rms symmetrical amps (such as 42kA, 65kA, or 100kA). It must equal or exceed the available fault current at the MCC main, calculated from the utility short-circuit contribution through the transformer and feeders. A bus with insufficient bracing does not merely trip; it physically deforms, tears off insulators, and creates a sustained arc that propagates along the bus. Never accept a bracing rating below the calculated fault current to save cost, because the failure mode is catastrophic and endangers the entire lineup.

### Verify Interrupting Ratings of All Devices Against the Fault Current
Every overcurrent device in the MCC, from the main breaker to each bucket disconnect and MCP, must have an interrupting rating (AIC) at least equal to the available fault current at its point in the system. The fault current is highest at the main and decreases with distance and conductor impedance, so devices near the main see the worst case. An underrated interrupting device can fail to clear a fault, rupture, and sustain the arc. Document the available fault current at each device location and verify the device rating against it, not just against the bus bracing.

### Confirm Temperature Rise Performance, Not Just Nameplate Ampacity
Bus ampacity is limited by the temperature rise it produces under load, and the nameplate rating assumes a specific ambient and a specific enclosure ventilation. In a real MCC, the enclosed ambient can be 40 degrees C or higher, and densely packed buckets reduce air circulation. Specify bus rated and tested to a temperature rise standard (commonly 65 degrees C rise over 40 degrees C ambient per UL 845), and verify the manufacturer's design temperature test covers the actual configuration. A bus that meets nameplate amps but exceeds temperature rise will discolor insulation, age connectors, and fail prematurely.

### Account for Future Expansion at the Main and Horizontal Bus
The least expensive time to add bus capacity is at initial purchase. Specify the main horizontal bus and the main device rating with future load in mind, because adding sections later requires spare capacity in the main breaker, the horizontal bus, and the incoming lug compartment. If the main breaker is sized exactly to present load, future sections cannot be fed without replacing the main, which often means de-energizing the entire MCC. Reserve physical space for at least one or two future sections even if the bus is not initially extended there.

### Check Section-to-Section Bus Joint Integrity
The horizontal bus is spliced at each section joint, and these joints are a frequent source of overheating. Specify bolted joints with belleville washers or the manufacturer's verified joint design, and require torque verification at commissioning. Section joints that are undertorqued or that use the wrong hardware develop high resistance, heat, and eventually loosen further as the bus expands and contracts. Include bus joint thermography in the commissioning and maintenance plan, because joint failures are the most common bus defect in service.

## Common Traps

### Summing Nameplates Instead of Applying Demand Diversity
The mechanism is that the designer adds every motor nameplate current to size the main bus. The false signal is a defensible-looking total that appears conservative. The harm is an oversized and overpriced main bus and breaker, or conversely, if diversity is overestimated, an undersized bus that overheats when more motors run simultaneously than the diversity assumption allowed, which is common during startup or upset conditions.

### Matching Bracing to Connected Load Instead of Available Fault
The mechanism is that the bus bracing is chosen to match the running current or the transformer kVA without calculating the actual short-circuit current. The false signal is that the bus ampacity looks adequate for the load. The harm is a bus that carries normal current fine but deforms and arcs during a fault, because bracing is a mechanical rating independent of ampacity, and the fault current from a typical 1500 kVA transformer can exceed an underspecified bus bracing in milliseconds.

### Ignoring Temperature Rise in Favor of Nameplate Amps
The mechanism is that the bus is selected by ampacity alone without checking the temperature rise test. The false signal is that the amps exceed the load with margin. The harm is that in the enclosed, high-ambient, densely packed MCC environment the bus runs far hotter than its nameplate implies, discoloring insulation and accelerating connector failure, and the defect is invisible until thermography finds a 90-degree joint years later.

### Underrating Device Interrupting Capacity to Match the Bus
The mechanism is that the bucket disconnects and MCPs are selected to the load current and assumed adequate because the bus bracing is high. The false signal is that the bus is well braced, implying the devices are too. The harm is that a device with an interrupting rating below the available fault fails to clear, ruptures, and sustains the arc that the bus bracing was supposed to survive, defeating the entire protection scheme.

### Omitting Future Capacity at the Main Breaker
The mechanism is that the main breaker and horizontal bus are sized exactly to the present load to minimize cost. The false signal is a clean, economical design that meets current requirements. The harm is that any future motor addition requires replacing the main device and de-energizing the MCC, which is often operationally unacceptable, and the cheap initial decision becomes a very expensive retrofit that may force a separate new MCC instead of an extension.

## Self-Check

- Is the main horizontal bus sized on calculated demand with diversity, plus a documented future-growth margin?
- Is each vertical bus sized to its column demand and independently checked for bracing against the section fault current?
- Is the bus bracing rating equal to or greater than the calculated available short-circuit current at the MCC main?
- Does every overcurrent device have an interrupting rating (AIC) verified against the fault current at its location, not just the bus bracing?
- Is the bus temperature rise specified and tested to a recognized standard (such as UL 845) for the actual enclosure configuration?
- Are future expansion capacity and physical space reserved at the main breaker and horizontal bus?
- Are section-to-section bus joints specified with verified hardware and torque values, with thermography planned at commissioning?
- Is the available fault current calculation documented and traceable to the utility contribution and transformer impedance?
