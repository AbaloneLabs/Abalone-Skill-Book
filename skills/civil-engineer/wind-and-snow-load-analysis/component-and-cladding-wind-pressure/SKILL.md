---
name: component-and-cladding-wind-pressure.md
description: Use when the agent is determining wind pressures on components and cladding under ASCE 7, selecting effective wind area, assigning roof and wall pressure zones, applying external pressure coefficients for small elements, or evaluating fastener and glazing loads at critical edge and corner locations.
---

# Component and Cladding Wind Pressure

Component and cladding (C&C) wind pressure is the local pressure used to design the elements that enclose the building — roof decking, purlins, wall panels, windows, doors, fasteners, and their connections — as distinct from the main wind-force resisting system that carries the overall lateral load. The governing reference is ASCE 7 Chapter 30, and the defining feature of C&C is that the pressure coefficients rise sharply as the effective wind area shrinks, because wind does not act uniformly over a small element: a single fastener or a corner panel sees peak gusts and pressure gradients that the averaged MWFRS pressure masks. The harm this skill prevents is a cladding panel, a skylight, or a fastener pattern designed for an average pressure that is half or a third of the local peak, so that the first strong wind peels the corner of the roof, blows in a window, or unzips a wall panel, breaching the envelope and then exposing the interior to the full internal pressure that cascades into progressive damage. Because C&C pressures are governed by the effective wind area and the zone, the engineer who applies the MWFRS pressure to a cladding element, or who uses too large an effective area to lower the coefficient, has under-designed the very elements most likely to fail first.

## Core Rules

### Use the Component and Cladding Chapter, Not the MWFRS Pressures

Never design a cladding element, a fastener, or a connection from the MWFRS pressures of Chapters 27 or 28, because those pressures are averaged over the whole building surface and are far lower than the local peaks a cladding element experiences. Use the C&C provisions of Chapter 30, which provide external pressure coefficients (GCp) calibrated to the actual effective wind area of the element and to its location in the defined pressure zones. Confirm that every element whose failure would breach the envelope or whose support spacing defines a local area is designed as C&C: this includes roofing, siding, glazing, doors, louvers, and the secondary framing (girts, purlins) that spans between main frames. The distinction is not optional: an element designed for MWFRS pressure instead of C&C can be under-designed by a factor of two or more at a corner or edge.

### Compute the Effective Wind Area Correctly

The effective wind area is the area used to enter the GCp tables and curves, and it is the span length times an effective width equal to one-third the span length (but not less than the actual tributary width where that is smaller), not simply the element's tributary area. Because GCp increases as effective wind area decreases, using the full tributary area of a long purlin or a wide panel instead of the defined effective area understates the coefficient and the pressure. For fasteners and small elements, the effective wind area can be very small, driving GCp to its highest values; confirm that the area used reflects the element actually being designed and that a fastener is not credited with the area of the whole panel it holds. The effective wind area rule exists because wind pressure is spatially variable, and a smaller area is more likely to sit entirely under a peak gust, so the coefficient must rise to capture that likelihood.

### Assign the Correct Pressure Zone to Each Element

ASCE 7 divides roof and wall surfaces into zones with progressively higher coefficients toward the edges and corners: roof zones (interior Zone 1, edge Zone 2, corner Zone 3 for most geometries; Zones 1 through 5 for the alternate all-heights method), and wall zones (interior Zone 4, edge Zone 5). The zone dimensions (a, typically the lesser of 10 percent of least horizontal dimension or 0.4h, but not less than 3 ft or 4 ft) define how far the edge and corner zones extend inward, and an element's zone is determined by its actual location, not by averaging across zones. Confirm that every element near a roof edge, ridge, corner, or wall edge is assigned to the higher zone, because the coefficients in Zone 3 (roof corner) or Zone 5 (wall edge) can be double or triple those in the interior zone. Do not collapse the edge and corner zones into the interior zone to simplify the takeoff, because the elements in those zones are exactly the ones most likely to fail.

### Combine External and Internal Pressure and Check Both Signs

The design C&C pressure is the external pressure coefficient (GCp) plus the internal pressure coefficient (GCpi), combined to produce both positive (inward) and negative (outward) pressures, because the same element can be pushed in or sucked out depending on the external-internal combination. For a partially enclosed or open building, the large GCpi produces a wide band of pressure between the positive and negative design cases, and both must be checked: a roof panel may be governed by suction (external negative plus internal positive), while a wall panel may be governed by pressure (external positive plus internal negative). Confirm that the internal pressure classification used for C&C matches the building's enclosure classification, and that the glazing and door elements are designed for the internal pressure that applies if they are breached in a wind-borne debris region. An element designed only for the external pressure, ignoring the internal component, is missing half the load.

### Give Special Attention to Fasteners, Glazing, and Small Elements

Fasteners, clips, and small connectors are the most pressure-sensitive elements because their effective wind area is tiny, driving GCp to its maximum, and their failure is disproportionate: a single fastener pullout can release a whole panel. Confirm that fasteners are designed for the C&C pressure at their location using the small-area effective wind area, not the panel area, and that the connection capacity (pullout, pullover, shear) exceeds the local peak pressure. Glazing must be designed for the C&C pressure and, in wind-borne debris regions, for impact from wind-borne debris, which can require impact-resistant assemblies or a design that assumes the glazing is breached (forcing an open or partially enclosed classification). Do not allow a "standard" fastener pattern or a generic glazing assembly to govern at a roof corner or wall edge where the pressure is highest, because the standard detail was almost certainly developed for an interior zone.

## Common Traps

### The MWFRS Pressure Applied To A Cladding Element

The designer uses the MWFRS wall or roof pressure to size a cladding panel or purlin because it is "already computed" and looks conservative. The mechanism is that MWFRS pressures are surface-averaged, so the false signal of a higher-looking load hides that it is far below the local peak at an edge or corner; the harm is a panel or purlin that fails at the corner where the C&C coefficient should have governed.

### The Effective Wind Area Inflated To Lower The Coefficient

The designer uses the full tributary area of a long span or a wide panel to enter the GCp curve, rather than the span-times-effective-width rule, to reduce the coefficient. The mechanism is that GCp falls with area, so the false signal of a "larger element" hides a higher coefficient the real effective area would require; the harm is a cladding element under-designed for the local pressure its actual area should have triggered.

### The Edge And Corner Zones Collapsed Into The Interior

The takeoff applies the interior Zone 1 or Zone 4 coefficient to the whole roof or wall to simplify the calculation. The mechanism is that the corner and edge coefficients are two to three times the interior value, so the false signal of a uniform load hides the local peaks at the perimeter; the harm is roof corners, ridges, and wall edges designed for interior pressures that the first storm will peel or blow in.

### The Fastener Credited With The Panel Area

A fastener or clip is designed using the tributary area of the whole panel it supports, rather than its own tiny effective wind area. The mechanism is that the small-area coefficient is the highest in the table, so the false signal of a "large tributary area" hides the peak pressure the fastener actually sees; the harm is a fastener pullout that releases a panel, breaching the envelope and cascading into interior pressurization.

## Self-Check

- Is every cladding element, secondary framing member, glazing unit, door, and fastener designed using the ASCE 7 Chapter 30 C&C provisions, and not the MWFRS pressures?
- Is the effective wind area computed as span length times one-third span (or the actual smaller tributary width), and not inflated to the full panel area to lower GCp?
- Is each element assigned to the correct roof zone (1, 2, 3 or 1-5) or wall zone (4, 5) based on its actual location relative to the defined edge and corner dimensions?
- Are both positive and negative design pressures checked by combining GCp with the correct GCpi for the building's enclosure classification?
- Are fasteners, clips, and small connectors designed for the small-area peak C&C pressure at their location, with pullout and pullover capacity verified?
- In wind-borne debris regions, is glazing designed for impact or is the building designed assuming breach (open or partially enclosed) with the corresponding GCpi?
- Has the zone geometry (dimension a) been computed from the building dimensions and height, and applied to define the edge and corner zones on both roof and walls?
