---
name: side-curtain-and-knee-airbag-circuit-diagnosis.md
description: Use when the agent is diagnosing side airbag, curtain airbag, or knee airbag circuit faults, resistance or voltage codes on non-driver airbag circuits, side-impact or satellite sensor faults, seat-mounted or door-mounted side airbag wiring, or evaluating roof-mounted curtain deployment loops and inflator circuits on supplemental restraint systems.
---

# Side, Curtain, and Knee Airbag Circuit Diagnosis

The driver and passenger frontal airbags get the diagnostic attention, but the side, curtain, and knee airbags are the circuits that fail differently and that carry their own traps, because they are distributed around the vehicle (in the seats, the doors, the roof, the dash), they depend on remote impact sensors (satellite sensors in the doors, B-pillars, and C-pillars), and their wiring flexes and moves with the seats and the doors. The judgment problem is that a side or curtain airbag circuit fault (a resistance-too-high, a resistance-too-low, a voltage code) overlaps across the airbag itself, the long wiring run, the satellite sensor, and the connectors that flex with the seat or the door — and because these circuits are part of the SRS, the safe diagnostic is restricted. A technician who condemns a curtain airbag for a chafed roof harness, or a side airbag for a satellite sensor fault, hands back a vehicle with the same lamp. This skill covers the disciplined diagnosis of side, curtain, and knee airbag circuits and their remote sensors.

## Core Rules

### Map the Side and Curtain Airbag Architecture: Which Sensor Fires Which Bag

The disciplined diagnosis of a side or curtain airbag circuit starts with the architecture, because the deployment logic ties specific satellite sensors to specific airbags, and a fault in the sensor or its wiring mimics a fault in the airbag. The architecture: a side impact is detected by satellite sensors (in the door, the B-pillar, sometimes the C-pillar), the SRS module evaluates the impact, and the module fires the side airbag (seat-mounted or door-mounted) and the curtain airbag (roof-mounted) on the struck side. The disciplined technician reads the OEM wiring diagram to identify which satellite sensor feeds which airbag, where the wiring runs (under the seat, through the door hinge, along the roof), and where the connectors and splices are. The tradeoff is that the architecture must be read from the diagram, but diagnosing a distributed airbag circuit without the map is guessing among long wiring runs.

### Diagnose Seat-Mounted Side Airbag Faults Through the Seat Wiring and the Connector

Seat-mounted side airbags are among the most common SRS circuit faults, because their wiring runs under the seat and flexes with every seat adjustment, and the connector under the seat is a frequent failure point (corroded, loose, or pinched by the seat track). The disciplined diagnosis of a side-airbag resistance code on a seat: with the SRS disabled and verified safe, inspect the under-seat connector and wiring (corrosion, loose pins, wires pinched by the track or cut by objects under the seat), measure the circuit resistance at the module connector with the seat in different positions (a fault that appears or changes with seat movement indicates the seat wiring), and wiggle the harness to reproduce an intermittent. The tradeoff is that the under-seat environment is harsh and the wiring flexes, but the under-seat connector is the most likely fault and the airbag is rarely the cause.

### Diagnose Curtain Airbag Faults Through the Roof Harness and the C- or D-Pillar

Roof-mounted curtain airbags have wiring that runs along the headliner and down the C- or D-pillars, and their faults (a resistance code on a curtain circuit) are often in the long roof harness (chafed on the roof structure, pinched at a pillar, corroded at a connector) rather than in the curtain itself. The disciplined diagnosis traces the curtain circuit from the module, along the roof harness, to the curtain, inspecting for chafing (a common fault where the harness crosses a sharp roof edge), checking the connectors at the pillars, and measuring the circuit resistance at the module connector. The tradeoff is that the headliner may need to be lowered to inspect the harness, but a curtain replaced for a chafed roof harness fails identically.

### Evaluate the Satellite Impact Sensor as the Cause of a Side-Airbag Code

A side-airbag or curtain code is often not the airbag but the satellite impact sensor that feeds it, and the disciplined diagnosis evaluates the sensor before the airbag. A satellite sensor fault (an internal failure, a wiring fault to the sensor, a corroded connector in the door or pillar) sets a code that may name the airbag circuit or the sensor, and the sensor is diagnosed by its data on the scan tool (does it read plausibly, does it respond to a tap or a pressure change), its power and ground, and its communication. The tradeoff is that the sensor is cheaper than the airbag and often the cause, but it is skipped in favor of the airbag. The disciplined technician checks the satellite sensor and its wiring before the airbag.

### Apply SRS Safety to Every Side, Curtain, and Knee Airbag Circuit

Every side, curtain, and knee airbag circuit is part of the SRS, and the disciplined diagnosis applies the SRS safety on every one: disable the system (ignition off, negative battery disconnected, the specified wait for capacitor discharge), verify de-energized, never probe with a test light or a low-impedance meter, and measure only at the module connector with the bag isolated. The tradeoff is that the safety restricts the testing and adds time, but a deployment from a wrong probe is catastrophic. The disciplined technician follows the SRS safety on every side, curtain, and knee circuit without exception.

## Common Traps

### Condemning a Seat Side Airbag for an Under-Seat Connector Fault — The side-airbag resistance code sets, the technician replaces the seat airbag, and the code returns because the under-seat connector was corroded. The trap mechanism is that the under-seat connector flexes and corrodes, and it is the most common fault on a seat side-airbag circuit. The false signal is the code naming the airbag; the harm is an unnecessary airbag. The disciplined technician inspects the under-seat connector and wiring before the airbag.

### Replacing a Curtain Airbag for a Chafed Roof Harness — The curtain resistance code sets, the technician replaces the curtain, and the code returns because the roof harness was chafed on the roof structure. The trap mechanism is that the roof harness runs a long path and chafes, and the curtain is the named component. The false signal is the code naming the curtain; the harm is an unnecessary curtain. The disciplined technician traces and inspects the roof harness before the curtain.

### Diagnosing an Airbag Fault When the Satellite Sensor Failed — The side-airbag code sets, the technician replaces the side airbag, and the code persists because the satellite sensor in the door failed. The trap mechanism is that the satellite sensor feeds the deployment decision, and its fault mimics an airbag circuit fault. The false signal is the code naming the airbag; the harm is an unnecessary airbag. The disciplined technician checks the satellite sensor and its wiring before the airbag.

### Probing a Side or Curtain Circuit With a Test Light and Deploying the Bag — The technician probes a side-airbag connector with a test light, and the current deploys the bag. The trap mechanism is that the side and curtain squibs deploy on a small current, and a test light delivers it. The false signal is the desire to "test" the circuit; the harm is a deployment injury and damage. The disciplined technician never probes an SRS circuit with a test light and uses only OEM-safe procedures.

### Ignoring a Door Hinge Wiring Fault on a Door-Mounted Side Airbag — The door side-airbag code sets intermittently, the technician focuses on the door, and the cause is the wiring in the door hinge flexing and breaking. The trap mechanism is that the door hinge wiring flexes with every door cycle, and a broken wire sets an intermittent code. The false signal is the code pointing at the door airbag; the harm is a misdirected diagnosis. The disciplined technician inspects the door hinge harness.

## Self-Check

- Did I read the OEM wiring diagram to map the satellite sensors, the airbags, the wiring runs, and the connectors for the side and curtain circuits?
- For a seat side-airbag code, did I inspect the under-seat connector and wiring, measure with the seat in different positions, and wiggle-test before the airbag?
- For a curtain code, did I trace and inspect the roof harness and the pillar connectors for chafing before the curtain?
- For any side or curtain code, did I evaluate the satellite impact sensor (data, power, ground, communication) before the airbag?
- Did I apply the SRS safety (disable, wait, verify, no test light, measure at the module) on every side, curtain, and knee circuit?
- Did I verify the repair by clearing the codes, confirming the lamp behavior, and completing the readiness?
- Did I document the circuit measurements and the safety procedure on the repair order?
