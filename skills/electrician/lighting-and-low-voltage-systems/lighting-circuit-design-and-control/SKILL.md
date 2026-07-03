---
name: lighting-circuit-design-and-control.md
description: Use when the agent is laying out lighting branch circuits, selecting switching and dimming controls, applying occupancy and daylight sensors, or matching LED drivers and lamps to dimmers, transformers, and control devices for residential, commercial, or industrial lighting.
---

# Lighting Circuit Design and Control

Lighting looks like the simplest part of an electrical system until you try to control it. The load is continuous, the controls are increasingly electronic, and the interaction between LED drivers, dimmers, occupancy sensors, and control voltages is where most field failures happen. The judgment problem is that lighting control is no longer a switch in series with a lamp; it is a system of low-voltage signals, electronic drivers, and compatibility constraints that must be selected together. An electrician who wires a lighting circuit the old way, switch leg, line-voltage dimmer, and whatever lamp fits the socket, will produce circuits that flicker, buzz, fail to dim, drop out randomly, or burn out drivers prematurely. This skill covers the decisions that determine whether a lighting circuit works the first time and keeps working, or becomes a warranty nightmare.

## Core Rules

### Treat Continuous Lighting Loads at 125 Percent for Sizing

Lighting is almost always a continuous load, defined as one expected to operate for three hours or more. Continuous loads must be sized at 125 percent of the load for both the overcurrent device and the conductor. A 16 A lighting load therefore requires a 20 A breaker and conductors rated to carry 20 A, not a 16 A circuit loaded to its full rating. Commercial lighting circuits are especially prone to this because they run all day. The trap is loading a 20 A lighting circuit to 20 A of connected lighting and assuming the breaker rating is the target.

The defense is to apply the 125 percent factor to all lighting loads, to derate the connected load accordingly, and to keep the continuous portion of any lighting circuit at or below 80 percent of the device and conductor rating.

### Design Switching and Three-Way Logic Before Rough-In

Switching layout, including three-way and four-way circuits, multi-location control, and the relationship between line-voltage switch legs and any low-voltage control wiring, must be designed before conduit and boxes are roughed in. Moving a switch or adding a three-way after the walls are closed is expensive and often forces an inferior workaround. Modern systems increasingly use low-voltage control wiring between sensors and a relay panel or smart switch, which must be planned as a separate raceway or cable system from the line-voltage wiring.

The trap is roughing in a simple switch leg and discovering later that the design called for occupancy control, dimming, or multi-location switching that the installed wiring cannot support. The defense is to lock down the control scheme, the switch locations, and any low-voltage wiring paths before rough-in, and to leave the control wiring separated from line voltage where the control device requires it.

### Match Dimmers to the Lamp or Driver, Not the Wattage Alone

Dimming compatibility is the single largest source of lighting field problems. An LED driver must be compatible with the dimmer type, whether forward-phase (leading-edge, TRIAC), reverse-phase (trailing-edge, ELV), or a specific protocol like 0-10V or DALI. The dimmer must also be rated for the minimum and maximum load of the driver, and LED loads behave differently from incandescent: a dimmer rated 600 W can drive far fewer LED watts because of driver inrush and minimum-load issues. Forward-phase dimmers can cause flicker, drop-out, and pop-on problems with drivers not designed for them.

The trap is selecting a dimmer by wattage and assuming it works with any LED. The defense is to consult the driver or lamp manufacturer's dimmer compatibility list, to verify the dimmer type matches the driver, and to keep the total LED load within the dimmer's minimum and maximum LED rating, not its incandescent wattage rating.

### Apply Occupancy and Vacancy Sensors With Coverage and Code in Mind

Occupancy sensors turn lights on when motion is detected; vacancy sensors require manual on and turn off automatically. Code increasingly requires either automatic shutoff or sensor control in many commercial spaces. The sensor technology, passive infrared (PIR), ultrasonic, or dual-tech, must match the space: PIR needs line of sight and fails in partitioned spaces; ultrasonic handles partitions but can false-trigger on air movement. Coverage patterns and mounting height determine whether the sensor actually sees the occupied area.

The trap is mounting a sensor where it cannot see the occupants, or using a single technology in a space it cannot cover, leading to lights that turn off while people are present. The defense is to select the sensor technology for the space geometry, to verify the coverage pattern at the mounting height, and to prefer dual-tech sensors in partitioned or irregular spaces.

### Separate Class 2 Control Wiring From Line Voltage

Low-voltage control wiring for occupancy sensors, 0-10V dimming, and networked lighting controls is typically Class 2 and must be separated from line-voltage wiring to prevent induced voltage, crosstalk, and the risk of energizing the low-voltage system at line potential. Separation is achieved by separate raceways, by barriers in multi-section boxes, or by maintaining required spacing. Some 0-10V wiring is rated for combined routing with the line-voltage feed, but only when the insulation ratings permit it.

The trap is pulling 0-10V or sensor wiring in the same conduit as line voltage without verifying the insulation rating, causing interference and failed inspections. The defense is to verify the control cable's listing, to separate Class 2 wiring in its own raceway or by approved barrier unless the cable is rated for combined routing, and to follow the control manufacturer's wiring instructions exactly.

### Account for LED Driver Inrush and Harmonics on the Circuit

LED drivers draw a high-magnitude, short-duration inrush current at turn-on, often many times the steady-state current, and they inject harmonics into the circuit. A circuit loaded to its ampacity with LED fixtures can nuisance-trip a breaker at switch-on because the combined inrush exceeds the magnetic trip threshold, even though the steady-state load is well within rating. Harmonics also heat neutral conductors, which is why shared neutrals on LED circuits are a risk.

The trap is loading a circuit to its ampacity based on steady-state current and then experiencing switch-on trips or neutral overheating. The defense is to derate LED circuits for inrush, to avoid shared neutrals on dimmed or LED-heavy circuits or to use oversized or doubled neutrals, and to select breakers rated for high-inrush loads where the inrush is severe.

## Common Traps

### Loading a Lighting Circuit to Its Full Breaker Rating

An electrician loads a 20 A lighting circuit with 20 A of connected lighting, reasoning that the breaker is rated 20 A. The mechanism of the trap is that lighting is a continuous load and must be limited to 80 percent of the device rating, so the circuit is actually overloaded by 25 percent on a continuous basis. The false signal is that the connected load equals the breaker rating, which looks like full utilization but is a code violation and a thermal overload. The harm is breaker heating, nuisance tripping as the thermal element heats over hours, and accelerated conductor and insulation aging. The defense is to apply the 125 percent continuous-load factor and to keep the connected continuous load at or below 80 percent of the breaker and conductor rating.

### Selecting a Dimmer by Wattage Without Checking LED Compatibility

An electrician installs a 600 W incandescent dimmer on an LED circuit because the LED load is only 100 W, well under 600. The mechanism of the trap is that LED drivers have inrush, minimum-load, and phase-cut requirements that the incandescent wattage rating does not capture, and a forward-phase dimmer can cause a driver to flicker, drop out at low dim levels, or fail to turn on at all. The false signal is the wattage headroom, which is irrelevant to phase and driver compatibility. The harm is flicker, customer complaints, premature driver failure, and warranty callbacks. The defense is to use the driver manufacturer's dimmer compatibility list and to verify the dimmer type, minimum load, and maximum LED rating.

### Mounting an Occupancy Sensor Where It Cannot See the Occupants

A PIR sensor is mounted in a storeroom full of shelving, and the lights turn off while a worker is in the aisle behind a shelf. The mechanism of the trap is that PIR requires line of sight, and obstructions create dead zones where the sensor cannot detect presence. The false signal is that the sensor is installed and works when someone walks directly under it, which proves function but not coverage. The harm is lights extinguishing in an occupied space, a safety hazard and a nuisance that leads occupants to defeat or bypass the sensor. The defense is to map the coverage pattern against the space geometry, to use ultrasonic or dual-tech sensors in partitioned spaces, and to verify coverage from the actual mounting location.

### Pulling 0-10V or Sensor Wiring in the Same Conduit as Line Voltage

To save a conduit run, the electrician pulls the 0-10V dimming wires alongside the line-voltage feed in the same raceway, using ordinary low-voltage wire. The mechanism of the trap is that Class 2 control wiring must be separated from line voltage unless the cable insulation is rated for the higher voltage, and combined routing can induce voltage on the control conductors and cause dimmer malfunction or inspection failure. The false signal is that the wires physically fit and the lights dim, which looks functional but violates the separation rule. The harm is interference, erratic dimming, and a code violation. The defense is to verify the control cable's voltage rating and listing, to separate Class 2 wiring in its own raceway unless the cable is rated for combined routing, and to follow the manufacturer's wiring requirements.

### Sharing a Neutral Across Dimmed or LED-Heavy Circuits

Two lighting circuits on opposite phases share a neutral to save wire, and both circuits are loaded with dimmed LED fixtures. The mechanism of the trap is that dimmed and LED loads draw harmonic currents that do not cancel on the shared neutral the way fundamental currents do, so the neutral can carry more current than either hot conductor, overheating it. The false signal is that the loads are on opposite phases and should cancel, which holds for linear loads but not for the harmonics from electronic drivers. The harm is neutral overheating, voltage distortion, and fire risk in the shared neutral. The defense is to run a separate neutral for each dimmed or LED circuit, or to oversize the shared neutral and verify the harmonic load is within its rating.

### Forgetting Switch Leg and Traveler Continuity in Three-Way and Four-Way Layouts

A three-way switching layout is roughed in with the switch legs and travelers reversed or a four-way miswired, and the lights behave erratically, switching from some positions but not others. The mechanism of the trap is that multi-way switching depends on correct traveler identification and common-terminal placement, and miswiring produces partial or unpredictable operation. The false signal is that the conductors are landed on terminals, which looks complete but does not guarantee the logic. The harm is a circuit that confuses occupants and requires rework after finish. The defense is to design the switch logic before rough-in, to identify common and traveler conductors at each box, and to test the switching from all positions before the walls are closed.

## Self-Check

- Did I treat all lighting loads as continuous and size the overcurrent device and conductor at 125 percent, keeping the connected continuous load at or below 80 percent of the rating?
- Did I lock down the switching and multi-way control scheme, including any low-voltage control wiring, before rough-in so the installed wiring supports the intended control?
- For every dimmer, did I verify compatibility against the lamp or driver manufacturer's list, confirm the dimmer phase type matches the driver, and check the minimum and maximum LED load rather than the incandescent wattage?
- Did I select occupancy or vacancy sensor technology for the space geometry, verify the coverage pattern at the mounting height, and prefer dual-tech sensors in partitioned or irregular spaces?
- Did I separate Class 2 control wiring from line voltage in its own raceway or with an approved barrier, unless the control cable is listed and rated for combined routing?
- Did I account for LED driver inrush and harmonics by derating the circuit, avoiding shared neutrals on dimmed or LED-heavy runs, and selecting breakers rated for high-inrush loads where needed?
- Did I test all switching, dimming, and sensor functions from every control location before the walls were closed, and verify three-way and four-way logic operates correctly from all positions?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
