---
name: panelboard-selection-and-configuration.md
description: Use when the agent is selecting or laying out a panelboard, choosing between main lug and main breaker configurations, determining circuit count and spare capacity, verifying bus and main breaker ratings, or evaluating series ratings and short-circuit current ratings for residential, commercial, or industrial distribution.
---

# Panelboard Selection and Configuration

A panelboard is the point where a feeder is broken into many branch circuits, and it is also the point where most overcurrent coordination decisions are frozen into hardware. Once a panel is mounted, bus rating, main device type, physical space count, and the short-circuit current rating are extremely difficult and expensive to change. The judgment problem is that panelboard selection is often treated as a catalog lookup exercise when it is actually a sequence of load, coordination, and survivability decisions that interact with each other. An undersized bus, a misapplied series rating, or a panel loaded to its physical limit with no spares will produce a system that passes an inspection but fails under real growth, real faults, or real maintenance. This skill covers the decisions that determine whether a panelboard is correct for the load it serves today and the load it will be asked to serve for the next twenty years.

## Core Rules

### Decide Main Lug vs Main Breaker From the Disconnect and Feed Context

A main lug only (MLO) panelboard has no integral overcurrent device; the feeder breaker upstream provides protection. A main breaker (MB) panelboard has an integral disconnect and overcurrent device at the panel. The correct choice is driven by the number of disconnects permitted for the service or feeder and by whether a local means of disconnect is required at the panel location. A common rule is the six-disconnect rule for services, and similar "number of disconnects" reasoning applies to feeders feeding multiple panels. If a panel is fed from a feeder tap or is the only disconnect for a separate structure, a main breaker is usually required to provide both the overcurrent protection and the local disconnect.

The trap is choosing an MLO panel to save cost and then having no compliant local disconnect, or choosing an MB panel where the upstream device already provides protection and the integral main merely duplicates it at added cost and added series-rating complexity. The defense is to trace the feeder back to its overcurrent source, count the disconnects already present, and decide whether the panel needs its own disconnect for code compliance, maintenance isolation, or both.

### Size the Bus and Main Rating Above the Calculated Load, Not At It

The panelboard bus rating (e.g., 100 A, 200 A, 225 A, 400 A) and the main device rating must be at least 125 percent of any continuous load and must carry the calculated load after demand factors. But selecting exactly at the calculated load leaves no headroom. Panelboards that are loaded to the bus rating run hotter, the breakers age faster, and the first future load addition forces a panel change. Industry practice is to provide spare ampacity and spare physical space. A panel loaded past 80 percent of its rating on continuous load is already at the continuous-load derating threshold, and a panel with every space filled has no room for the inevitable future circuit.

The trap is treating the calculated load as the design target rather than the floor. The defense is to select a bus rating that leaves meaningful margin above the calculated load and to verify that the connected continuous load does not exceed 80 percent of the overcurrent device rating unless the device is rated for 100 percent continuous duty.

### Provide Spare Circuit Spaces for Future Growth

A panelboard with no spare spaces is obsolete the day it is installed. The number of physical spaces determines how many circuits can be added without a panel change, and a panel filled to capacity forces any future load onto a subpanel or a full panelboard replacement. Standard practice is to provide at least 20 to 30 percent spare spaces at installation, and more for panels serving areas likely to expand. Full-size spaces are preferred over tandem or half-size breakers because tandem slots consume future flexibility and often have lower interrupting ratings.

The trap is filling a panel to save the cost of the next size up, then discovering that the next circuit requires a changeout. The defense is to count the present circuits, add the known near-term additions, and then add a margin of spare spaces, selecting the next larger enclosure or a panel with more rows when the present count approaches the available spaces.

### Verify the Interrupting Capacity and Short-Circuit Current Rating Against Available Fault Current

Every overcurrent device in a panel has an interrupting rating (AIC, in kA), and the panelboard as an assembly has a short-circuit current rating (SCCR). Both must equal or exceed the available fault current at the line terminals of the panel. Available fault current is determined by the transformer size and impedance and the conductor length and size back to the source. A panel installed closer to a large transformer can see fault currents that exceed standard 10 kA residential breakers. On commercial services, 65 kA, 100 kA, and higher ratings are common and must be selected deliberately.

The trap is assuming a default 10 kA or 22 kA breaker is adequate without calculating the available fault current at the panel. A breaker applied above its interrupting rating can fail catastrophically during a fault, rupturing the enclosure instead of clearing. The defense is to obtain or calculate the available fault current at each panel location and to specify breakers and panel SCCR that exceed it, documenting the basis.

### Apply Series Ratings Only With Verified Upstream-Downstream Pairings

A series-rated combination allows a lower-rated downstream breaker to be protected by a higher-rated upstream device, provided the specific pairing has been tested and listed by the manufacturer. Series ratings are real and cost-effective, but they apply only to the exact device combinations listed, at the specific voltage, and under the tested conditions. They cannot be inferred, mixed across manufacturers, or assumed because the upstream device has a higher rating.

The trap is treating series ratings as additive arithmetic, assuming any high-rated upstream breaker will protect any lower-rated downstream breaker. It will not; the clearing behavior depends on the specific trip curves and let-through characteristics that were tested together. The defense is to use the manufacturer's published series-rating table for the exact catalog numbers installed, to verify the voltage matches, and to apply the warning that series ratings are generally not permitted where motors or other contributions add downstream fault current.

### Respect the 42-Space History and Modern Handling Limits

Older panels were often capped at 42 circuits per panelboard; modern codes removed that hard limit, but physical and thermal handling limits still apply. Overcrowding a panel with many high-load circuits concentrates heat, reduces wire bending room, and makes the neutral and equipment grounding bars inadequate for the number of terminations. The number of terminals permitted on a neutral bar is limited by the listing, and overfilling a bar is both a code violation and a loose-connection risk.

The trap is cramming many circuits into one panel to avoid adding a second panel, then overloading the neutral bar and the conductor fill. The defense is to split loads across multiple panels or subpanels when the circuit count grows, and to verify that the neutral and grounding bars are rated for the number of terminations actually installed.

## Common Traps

### Assuming the Default Residential 10 kA Breaker Is Always Adequate

An electrician installs a residential panel with standard 10 kA breakers because that is what the supply house stocks and what most residential panels use. The service transformer, however, is a large kVA unit mounted on a pad only a few feet from the meter, and the available fault current at the panel exceeds 10 kA. The mechanism of the trap is that "residential" and "adequate" are conflated; available fault current depends on transformer impedance and distance, not on the building type. The false signal is that the breakers fit the panel and the panel is listed, which feels like proof of adequacy but proves nothing about fault current. The harm is that during a bolted fault the breaker is asked to interrupt more current than its tested rating, and it can rupture, arc, or fail to clear, causing fire and injury. The defense is to calculate available fault current at every panel, especially those near large or low-impedance transformers.

### Treating the Calculated Load as the Design Target

The load calculation produces a number, say 180 A, and the electrician selects a 200 A panel because 200 is the next standard size above 180. The panel is then loaded to 180 A of connected load, leaving essentially no margin. The mechanism of the trap is that the calculated load is the minimum the panel must carry, but continuous loads, harmonic-rich loads, and future additions all push actual demand toward and sometimes past the calculated value. The false signal is that the panel rating exceeds the calculation, which looks compliant but ignores that the calculation already includes demand factors that reduce the connected load. The harm is a panel that runs hot, breakers that nuisance-trip as they age, and no room to add the loads the building will inevitably acquire. The defense is to treat the calculated load as the floor and to select a bus rating with real headroom plus spare spaces.

### Inferring a Series Rating That Was Never Tested

An electrician needs a higher interrupting rating on a downstream branch breaker and reasons that since the upstream main breaker is rated 65 kA, it will protect a 10 kA downstream breaker to a higher effective rating. The mechanism of the trap is that series ratings are not arithmetic; they are the result of specific laboratory tests of exact device pairings, and an untested combination has no listed series rating regardless of the individual ratings. The false signal is the high upstream rating, which feels protective but does not transfer to the downstream device without a tested pairing. The harm is a downstream breaker that fails to clear a fault it was assumed to be protected against, with the panel and its conductors subjected to fault energy beyond their rating. The defense is to consult the manufacturer's series-rating chart for the exact catalog numbers and to refuse to claim a series rating that is not explicitly listed.

### Filling Every Space and Leaving No Spare Capacity

A 20-space panel is installed with all 20 spaces filled on day one because the calculated load fit and the next larger panel cost more. Within two years the owner wants to add a circuit for an EV charger, a heat pump, or a finished basement, and there is no space. The mechanism of the trap is that present-load thinking dominates future-load thinking under cost pressure, and the cost difference between a 20-space and a 30-space panel is small compared to the cost of a changeout later. The false signal is that the panel meets the current load, which is true but irrelevant to future capacity. The harm is an expensive panel replacement or an unsafe workaround such as tandem breakers and overloaded bars. The defense is to specify spare spaces as a design requirement, not an option.

### Choosing MLO to Save Cost Without a Disconnect Plan

An electrician selects a main-lug-only panel to save the cost of the main breaker, intending to rely on the upstream feeder breaker for protection. The panel is located in a separate building or a location where a local disconnect is required for service or maintenance, and now there is no compliant means to disconnect the panel locally. The mechanism of the trap is that the MLO choice was made on cost without tracing the disconnect and overcurrent requirements that the panel location imposes. The false signal is that the upstream breaker protects the feeder, which is true but does not satisfy the need for a local disconnect or the feeder tap rules. The harm is a non-compliant installation and a safety hazard for anyone servicing the panel who cannot isolate it locally. The defense is to decide the main device type from the disconnect and feed context, not from the price.

### Overloading the Neutral Bar Beyond Its Listed Terminal Count

A panel is loaded with many circuits, and the electrician doubles up neutral conductors under single terminals to fit them all on the existing neutral bar. The mechanism of the trap is that neutral bars have a listed maximum number of terminations, and double-lugging creates loose connections, heating, and the loss of a reliable current path back to the source. The false signal is that the conductors physically fit under the screw, which looks like a connection but is not a listed or reliable one. The harm is overheating, neutral voltage rise, and potential fire at the bar, plus a code violation. The defense is to add an approved auxiliary neutral bar or split the load across another panel when the listed terminal count is reached.

## Self-Check

- Did I decide between main lug only and main breaker by tracing the feeder to its overcurrent source and determining whether a local disconnect is required at the panel location, rather than by cost alone?
- Is the bus and main device rating at least 125 percent of any continuous load, and does it provide headroom above the calculated load rather than sitting at it?
- Did I provide at least 20 to 30 percent spare physical spaces for future circuits, and did I count known near-term additions before selecting the panel size?
- Did I obtain or calculate the available fault current at the panel line terminals, and do both the breaker interrupting ratings and the panelboard short-circuit current rating exceed that value?
- If I am relying on a series rating, is the exact upstream-downstream device pairing listed in the manufacturer's series-rating table at the installed voltage, and am I within any restrictions on downstream motor contribution?
- Is the continuous load no more than 80 percent of the overcurrent device rating unless the device is rated for 100 percent continuous duty, and have I verified the neutral bar terminations do not exceed the listed count?
- Did I avoid overfilling the panel, and did I split loads across a second panel or subpanel when the circuit count approached the panel's physical and thermal handling limits?
- Is the reasoning documented clearly enough that another practitioner could review the basis and reproduce the conclusion?
