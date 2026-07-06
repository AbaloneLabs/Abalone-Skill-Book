---
name: hvac-water-piping-and-chilled-hot-loop-connections.md
description: Use when the agent is piping a hydronic HVAC loop (chilled or hot water to an AHU or fan coil), placing an air separator and expansion tank, selecting flow-control and balancing valves, setting glycol concentration for chilled loops, providing make-up water backflow protection, or isolating coils for service where air-bound loops, freeze damage, and contamination of the potable supply are the risks.
---

# HVAC Water Piping and Chilled Hot Loop Connections

A hydronic HVAC loop — chilled water to an air handler, hot water to a fan coil or reheat coil — is a closed piping system that must move water quietly, stay free of air, survive freezing, and never contaminate the potable supply that feeds it. The judgment problem is that these loops fail in ways that look like HVAC problems but are really piping problems: an air-bound coil that will not heat or cool, a chilled loop that freezes and splits a coil, a make-up connection that back-siphons loop water into the building's drinking water, or a coil that cannot be isolated for service. Agents miss this because the loop "circulates" on the test run, masking that the air separator is missing, the glycol is too thin for the design low temperature, or the make-up connection has no backflow protection. This skill covers the piping, air elimination, expansion, balancing, freeze, and backflow decisions that keep a hydronic HVAC loop safe and serviceable.

## Core Rules

### Pipe the Loop for the Coil Flow, Velocity, and Service Isolation

Each coil (chilled-water, hot-water, or reheat) has a design flow (GPM) and a design water-temperature change (commonly 10°F for chilled water, 20 to 30°F for hot water), and the pipe to the coil must carry that flow at a reasonable velocity (commonly 2 to 5 feet per second) without erosion or noise. Each coil connection must also include service isolation — shutoff (ball or butterfly) valves at the supply and return so the coil can be isolated and drained for service or replacement, and a means to balance the flow (a circuit setter or balancing valve). The disciplined rule is to size the coil piping to the design GPM at 2 to 5 feet per second, to install isolation valves at every coil supply and return, to include a balancing valve on each coil return, and to provide a drain at each coil's low point so the coil can be isolated and drained without draining the whole loop.

### Install an Air Separator at the Hottest Point and an Expansion Tank Sized to Loop Volume

Air in a hydronic loop collects at high points and in coils, blocking flow and producing noise, cold or warm spots, and pump cavitation. The loop must have an air separator (a microbubble or centrifugal separator) at the hottest point in the system (the boiler supply for hot loops, or just downstream of the chiller for chilled loops where dissolved air comes out of solution as temperature changes), plus automatic vents at all high points. The loop must also have an expansion tank sized to the loop's water volume and the design temperature swing (the tank absorbs the expansion and contraction as the water heats and cools, keeping the pressure within the relief setpoint). The disciplined rule is to install an air separator at the hottest point, automatic vents at every high point and coil, and an expansion tank sized to the loop volume and temperature range, with the pre-charge set to the cold fill pressure.

### Provide Flow Control and Balancing for Multi-Coil Loops

In a loop serving several coils (a multi-zone air handler, several fan coils), flow must be divided in proportion to each coil's load, or the nearest coil hogs the flow and the farthest coil starves. Each coil needs a balancing (circuit setter) valve on its return, set by measured flow to the coil's design GPM, and the loop needs a means to maintain flow across the chiller or boiler when coils close (a pressure-bypass or primary/secondary arrangement). The disciplined rule is to install a balancing valve on every coil return, to set each by measured flow (using the valve's meter ports or a separate flow meter), and to provide a differential-pressure bypass or a decoupler so the chiller or boiler sees stable flow as coils open and close. Sequence the balance from the longest/highest-head coil to the shortest.

### Set Glycol Concentration and Freeze Protection for Chilled and Outdoor Loops

Chilled-water loops in buildings, and any hydronic loop exposed to freezing (outdoor coils, snowmelt, make-up air preheat), need freeze protection. The options are a glycol/water mix (propylene or ethylene glycol) at a concentration matched to the design low temperature (commonly 30 to 40 percent for roughly 0 to 10°F burst protection, higher for colder design), or a draindown/fill strategy for seasonal coils. Glycol changes the fluid's properties (higher viscosity, lower heat capacity), so the pump and the heat exchanger must be sized for the glycol mix, not plain water. The disciplined rule is to determine the design low temperature the loop could see (including outage conditions), select the glycol concentration for burst protection at that temperature, to size the pump and heat exchanger for the glycol's viscosity and heat capacity, and to test the glycol concentration on interval.

### Protect the Potable Make-Up Connection With Backflow Prevention

The make-up water connection that fills the hydronic loop from the building's potable supply is a cross-connection: if the potable pressure drops (a main break, a fire draw), loop water — which contains glycol, corrosion inhibitors, and biological growth — can back-siphon into the drinking water. The make-up connection must have a backflow preventer appropriate to the hazard (commonly a reduced-pressure-zone [RPZ] or double-check assembly for glycol or chemical-treated loops, per IPC and local code), and the loop's fill must be through the backflow preventer with no bypass. The disciplined rule is to install a code-appropriate backflow preventer on every hydronic-loop make-up connection, to treat any glycol or inhibitor-treated loop as a high hazard requiring an RPZ where code requires, and to provide for annual testing of the assembly.

## Common Traps

### Omitting the Air Separator and Leaving Coils Air-Bound

A plumber pipes a hydronic loop with no air separator, only a few manual vents. The trap is that air collects in the coils and blocks flow. The mechanism is that dissolved air comes out of solution at temperature changes and rises to high points; the false signal is "the pump circulates." The harm is a coil that will not heat or cool and noisy operation. The defense is to install an air separator at the hottest point and automatic vents at every high point and coil.

### Undersizing the Expansion Tank for the Loop Volume

A plumber installs a small standard expansion tank on a large-volume loop. The trap is that the pressure rises to the relief valve on every temperature swing. The mechanism is that the loop's expansion exceeds the tank's acceptance volume; the false signal is "an expansion tank is installed." The harm is chronic relief discharge, water waste, and makeup-water introduction (causing corrosion). The defense is to size the tank to the loop volume and temperature range.

### Skipping Balancing Valves and Letting One Coil Hog Flow

A plumber ties several coils into a loop with no balancing valves. The trap is that the nearest coil takes most of the flow and the farthest coil starves. The mechanism is that flow distributes by hydraulic resistance; the false signal is "the near coil works." The harm is poor temperature control at the far coil. The defense is to install a balancing valve on every coil return and set each by measured flow.

### Under-Protecting a Chilled or Outdoor Loop Against Freeze

A plumber fills an outdoor or chilled loop with plain water or too-thin glycol. The trap is that the loop freezes during an outage or cold snap and splits a coil. The mechanism is that water expands on freezing and ruptures copper or steel; the false signal is "the chiller runs in summer." The harm is a burst coil and water damage. The defense is to match glycol concentration to the design low temperature (including outage) and to size pump and heat exchanger for the glycol.

### Connecting the Make-Up With No Backflow Preventer

A plumber runs the make-up line straight from the potable supply to the loop with no backflow preventer. The trap is that a potable pressure drop can back-siphon glycol- or inhibitor-laden loop water into the drinking water. The mechanism is that the make-up is a cross-connection; the false signal is "water flows in to fill the loop." The harm is contamination of the potable supply. The defense is to install a code-appropriate backflow preventer (RPZ for treated loops) on every make-up connection and to test it annually.

## Self-Check

- Is each coil piping run sized to the coil's design GPM at 2 to 5 feet per second, with isolation (ball or butterfly) valves at the supply and return for service?
- Is a balancing (circuit setter) valve installed on every coil return and set by measured flow to the coil's design GPM, sequenced from the longest coil to the shortest?
- Is a low-point drain provided at each coil so the coil can be isolated and drained without draining the whole loop?
- Is an air separator (microbubble or centrifugal) installed at the hottest point in the system, with automatic vents at every high point and coil?
- Is the expansion tank sized to the loop's water volume and design temperature range, with the pre-charge set to the cold fill pressure, so the pressure stays within the relief setpoint?
- For multi-coil loops, is a differential-pressure bypass or primary/secondary decoupler provided so the chiller or boiler sees stable flow as coils open and close?
- For chilled or outdoor loops, is the glycol concentration (commonly 30 to 40 percent for 0 to 10°F burst protection, higher for colder design) matched to the design low temperature including outage conditions?
- Are the pump and heat exchanger sized for the glycol mix's higher viscosity and lower heat capacity (not plain water), and is the glycol concentration tested on interval?
- Is the potable make-up connection protected by a code-appropriate backflow preventer (RPZ for glycol- or inhibitor-treated loops), with no bypass and an annual test provision?
- Did I document the coil flows, pipe sizes, air-separator and expansion-tank sizing, balancing-valve settings, glycol concentration and freeze basis, and backflow-prevention device so the design is verifiable?
