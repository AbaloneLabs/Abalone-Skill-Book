---
name: wind-load-determination-and-mwfrs.md
description: Use when the agent is determining wind loads on the main wind-force resisting system, selecting ASCE 7 basic wind speed and exposure category, computing velocity pressure, applying directional or envelope procedures, evaluating gust effects on rigid versus flexible buildings, or assigning internal pressure classification for MWFRS design.
---

# Wind Load Determination and MWFRS

Wind load determination for the main wind-force resisting system (MWFRS) is the process by which a building's lateral framing is sized to resist the pressures a design wind event imposes on the whole structure. The hazard is governed by ASCE 7 (Chapters 26 through 32 in the 7-22 edition), which ties together basic wind speed, exposure, topography, enclosure classification, gust response, and pressure coefficients into a single design velocity pressure that drives the shear, overturning, and drift the MWFRS must resist. The harm this skill prevents is an under-strength lateral system that racks, overturns, or loses a cladding-bearing wall under a wind event that the code was meant to capture; or an over-conservative system that wastes material by misclassifying a building as flexible, as open, or as sitting in a rougher exposure than it does. Because the MWFRS wind load is a chain of multipliers, a single misjudged input — the wrong risk category wind speed, a flexible building treated as rigid, a partially enclosed building treated as enclosed — can change the design pressure by tens of percent, and the error is invisible until a storm exposes it.

## Core Rules

### Establish the Basic Wind Speed, Risk Category, and Directionality

Begin with the basic wind speed V from the ASCE 7 wind maps, selected by the Risk Category of the building (I, II, III, or IV), which sets the mean recurrence interval (300-year for Category I up to 1700-year for Category IV in the 7-22 ultimate wind speed maps). Confirm that the correct map is used: Vult maps apply to the strength-design wind load equations, and a Vasd conversion is only needed when using older allowable-stress formats. Apply the wind directionality factor Kd (0.85 for most buildings, but lower and direction-dependent for some structures) only after confirming the load combination permits it, because Kd assumes the wind is unlikely to align with the building's weakest axis. Never substitute a regional "design wind speed" from a local amendment without checking whether it is already factored, because double-applying directionality or importance inflates the load and double-removing it collapses the margin of safety.

### Classify Exposure Correctly and Conservatively

The exposure category (B, C, or D) sets the velocity pressure exposure coefficient Kz and Kh, and it depends on the surface roughness of the upwind terrain for the prevailing wind directions, not on the site's general character. Exposure B requires surface roughness B (urban, suburban, wooded) to persist for at least 1500 ft upwind (or 2600 ft for buildings over 30 ft), and the reduction is only permitted where the roughness is durable; transient construction sites, parking lots, or scattered obstructions do not qualify. Where the upwind terrain varies by direction, evaluate each principal direction and use the exposure that governs; a building on the edge of a suburban area opening to a field or water should often be Exposure C. Do not default to Exposure B to reduce loads unless the qualifying roughness is documented and permanent, because the Kz difference between B and C at typical heights can change the velocity pressure by 30 to 50 percent and is the single most consequential input after V.

### Apply Topography Only Where the Condition Is Real

The topographic factor Kzt accounts for wind speed-up over isolated hills, ridges, and escarpments, and it is only required where all three conditions hold: the feature is isolated (2D ridge/axis or 3D axisymmetric hill), it exceeds the threshold height relative to upwind terrain, and the building is within the zone of influence. Compute Kzt from the ASCE 7 topographic factor equations using the hill shape, height, and the building's position relative to the crest, and do not apply a blanket Kzt > 1.0 to all sloped sites. Conversely, do not ignore a real escarpment or bluff because the slope "looks gentle," because wind speed-up over even moderate topography can raise the effective pressure beyond what the flat-terrain V map implies. Where the site is near a coastal bluff, a ridge, or the edge of a plateau, evaluate Kzt explicitly rather than assuming flat terrain.

### Choose the Correct Procedure: Directional, Envelope, or Simplified

For MWFRS, ASCE 7 offers the directional procedure (Chapter 27, applicable to buildings of all heights), the envelope procedure (Chapter 28, generally for low-rise buildings), and the simplified versions of each. The directional procedure applies external pressure coefficients Cp by wind direction and surface (windward wall positive, leeward wall negative, roof varying with slope), and it is required for flexible buildings, tall buildings, and unusual shapes. The envelope procedure applies a combined net pressure coefficient that envelops all wind directions and is simpler for regular low-rise buildings, but it cannot be used for buildings that fall outside its geometric limits. Selecting the wrong procedure is not a conservative shortcut: the envelope procedure can under-predict loads on certain tall or flexible buildings where directional effects and across-wind response matter, while forcing the directional procedure onto a simple low-rise building adds complexity without changing the answer.

### Evaluate Gust Effects: Rigid Versus Flexible

Determine whether the building is rigid or flexible by comparing its fundamental natural frequency to 1 Hz (buildings with a fundamental frequency greater than 1 Hz are rigid; those below are flexible and require the flexible gust factor Gf). For flexible buildings, the gust-effect factor captures resonant and dynamic amplification and can substantially exceed the rigid building factor of 0.85, and it must be computed from the building's natural frequency, damping, and size. Misclassifying a flexible building (a slender tower, a long-span frame, a light steel building) as rigid suppresses the dynamic amplification and produces an unconservative load. Confirm the fundamental frequency from a rational analysis, not from a rule of thumb, because the rigid/flexible threshold is the gate to the most consequential dynamic correction in the wind load chain.

### Assign Enclosure and Internal Pressure Classification

The enclosure classification (enclosed, partially enclosed, or open) sets the internal pressure coefficient GCpi, which is small (±0.18) for enclosed buildings but large (±0.55) for partially enclosed, and this internal pressure acts on every internal surface and must be combined with the external pressure. A building is partially enclosed if it has a large opening in one wall and substantially less opening in the others; the condition is common in industrial buildings, warehouses with large doors, and buildings with breachable glazing. Determine the classification from the ASCE 7 opening-area criteria, account for the wind-borne debris glazing provisions (which can force a building to be designed as open or partially enclosed in wind-borne debris regions), and never default to "enclosed" because it gives the lowest load, because the wrong classification hides a large internal pressure that can blow out walls and roofs.

## Common Traps

### The Exposure B Assumption Without Qualifying Roughness

The designer selects Exposure B to reduce the load because the site is "in town," but the upwind fetch is a field, a parking lot, or scattered low buildings that do not meet the surface roughness and extent criteria. The mechanism is that Kz drops sharply in Exposure B, so the false signal of a compliant exposure hides a 30 to 50 percent under-prediction of velocity pressure, and the harm is an MWFRS designed for a load the actual wind climate will exceed.

### The Flexible Building Treated As Rigid

A slender or light building is assumed rigid because the framing "looks stiff," and the rigid gust factor of 0.85 is applied instead of the flexible Gf. The mechanism is that the natural frequency was never computed, so the dynamic amplification is suppressed; the false signal is the familiar 0.85 factor, and the harm is an under-strength lateral system that will rack or oscillate beyond its drift limit in a design wind event.

### The Enclosed Default That Hides Internal Pressure

The building is classified as enclosed because the designer did not evaluate the opening-area criteria, but it has a large loading door or breachable glazing that qualifies it as partially enclosed. The mechanism is that GCpi jumps from ±0.18 to ±0.55, so the false signal of an "ordinary building" hides a large internal pressure; the harm is a roof or wall designed for external pressure only that is lifted or blown inward by the internal pressure it was never designed for.

### The Envelope Procedure Applied Outside Its Limits

The designer uses the low-rise envelope procedure for simplicity on a building that exceeds its height, shape, or flexibility limits. The mechanism is that the envelope coefficients do not capture the directional and dynamic effects on the actual building, so the false signal of a quick, conservative-looking load hides the directional loads the building will experience, and the harm is an MWFRS under-designed for the wind directions the procedure did not envelope.

## Self-Check

- Is the basic wind speed V taken from the correct ASCE 7 map for the building's Risk Category, and is directionality Kd applied only where the load combination permits?
- Is the exposure category supported by documented, durable upwind surface roughness of the required extent, and not defaulted to Exposure B to reduce load?
- Where the site is on a hill, ridge, or escarpment, has the topographic factor Kzt been computed from the feature geometry rather than assumed as 1.0?
- Has the rigid-versus-flexible determination been based on a computed fundamental natural frequency, with the flexible gust factor Gf applied where the frequency is below 1 Hz?
- Is the enclosure classification derived from the opening-area criteria, with wind-borne debris glazing provisions considered, so the internal pressure coefficient GCpi reflects the real condition?
- Has the correct MWFRS procedure (directional for tall/flexible/unusual, envelope for qualifying low-rise) been applied within its geometric and dynamic limits?
- Are windward, leeward, and roof external pressures combined with the correct internal pressure cases (positive and negative) to produce the governing MWFRS forces?
