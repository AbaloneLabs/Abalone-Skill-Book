---
name: wind-load-on-tall-and-unusual-buildings.md
description: Use when the agent is calculating wind loads on tall or slender buildings, applying wind tunnel study results, designing for vortex shedding and across-wind response, evaluating cladding and component pressures, assessing occupant comfort and acceleration, or handling unusual building shapes where code-based wind loads are invalid.
---

# Wind Load on Tall and Unusual Buildings

Wind load on tall and unusual buildings is a domain where the simplified code procedures that work for ordinary low-rise buildings become unreliable or explicitly inapplicable, and where the governing response is often not the along-wind force the intuition expects but the across-wind vibration driven by vortex shedding, the torsional response driven by asymmetric shape, or the local pressure extremes on cladding that the code does not capture. The recurring failure is to apply code-based wind loads to a building that exceeds the code's height, slenderness, or shape limits, to treat the along-wind load as governing when the across-wind response is larger, and to design the cladding for code pressures when a wind tunnel study would show localized suctions far exceeding them. The engineer's job is to recognize when the code procedures are valid and when a wind tunnel study is required, to interpret the wind tunnel results for the structural and cladding design, and to account for the dynamic and comfort effects that only a dynamic analysis or a wind tunnel can reveal. This skill covers the judgment used to calculate and apply wind loads on tall and unusual buildings, with the overriding principle that the governing wind effect is often the one the code does not directly give.

## Core Rules

### Recognize When Code Procedures Are Valid and When a Wind Tunnel Study Is Required

Building codes (ASCE 7, and similar) provide wind load procedures that are validated for buildings within limits of height, slenderness (height-to-width ratio), shape, and dynamic response. For buildings that exceed those limits — tall buildings, slender towers, buildings with unusual shapes, buildings prone to vortex shedding, buildings with significant torsional response — the code procedures are explicitly inapplicable or flagged as requiring a wind tunnel study. Recognize the limits, and recommend a wind tunnel study when the building exceeds them, because the code loads can under-predict the true response by a large margin in the across-wind and torsional directions. A wind tunnel study is not an optional refinement for tall buildings; it is the applicable procedure, and the code itself says so.

### Evaluate Across-Wind and Torsional Response, Not Just Along-Wind

The intuitive wind load is the along-wind force, pushing the building in the wind direction, and for ordinary buildings this governs. For tall and slender buildings, the across-wind response — the vibration perpendicular to the wind direction, driven by vortex shedding — is often larger than the along-wind response, and the torsional response — the twisting driven by asymmetric pressure distribution — can govern the corner column forces and the cladding. Evaluate all three directions (along-wind, across-wind, torsional), using the wind tunnel results or a dynamic analysis, and design the lateral system for the governing combination. Designing for the along-wind load alone, on a building where across-wind governs, under-designs the lateral system by the ratio of the two responses.

### Design Cladding and Components for Local Pressure Extremes

The cladding, the windows, the curtain wall, and the components and cladding (C&C) experience localized pressure extremes — high suctions at the building corners, high pressures near the windward face edges, and high suctions in separation zones — that far exceed the average pressures used for the main wind-force-resisting system (MWFRS). Design the cladding for the component-and-cladding pressures, which include the localized peak factors, and pay particular attention to the corner zones where suctions are highest. A wind tunnel study provides the cladding pressure distribution directly, and for unusual shapes the code C&C pressures can under-predict the corner suctions. Cladding failure, while not usually life-threatening, is the most common wind damage and the most costly, and it is governed by local pressures the MWFRS load does not represent.

### Assess Occupant Comfort and Building Acceleration

Tall buildings move under wind, and the acceleration at the upper floors, perceived as sway, can cause occupant discomfort, nausea, and complaints at levels far below the structural design load. Assess the building acceleration under the service-level wind event (commonly the 1-year or 10-year return period), and compare it to the occupant comfort criteria (commonly 15 to 25 milli-g for residential, higher for commercial). If the acceleration exceeds the criteria, provide mitigation — increased stiffness, a tuned mass damper, a tuned liquid damper — to bring it within acceptable limits. Designing for strength without checking service-level acceleration can produce a building that is structurally adequate but functionally uninhabitable at the upper floors.

### Account for Vortex Shedding and Lock-In Effects

Vortex shedding is the periodic shedding of alternating vortices in the wake of a bluff body, which produces a fluctuating across-wind force at the vortex shedding frequency. When the vortex shedding frequency matches the building's natural frequency (lock-in), the response can grow large and self-limiting, driven by the building's own motion rather than the wind speed, and this is the mechanism behind many across-wind failures and serviceability problems. Assess the building's susceptibility to vortex shedding — it is highest for slender, flexible, lightly damped buildings with a uniform cross-section — and design to avoid lock-in (by changing the cross-section, adding damping, or ensuring the critical wind speed is outside the likely range). Vortex shedding is the mechanism that makes the across-wind response exceed the along-wind response, and it must be evaluated explicitly.

### Consider Wind Directionality, Exposure, and Topographic Effects

The wind load depends on the wind direction relative to the building, the terrain exposure (which sets the wind profile and turbulence), and the topography (hills, ridges, and escarpments that accelerate the wind). Account for directionality, because the worst wind direction may not align with the building's principal axes and the building's shape may produce its worst response at an oblique angle. Confirm the exposure category for each direction, because urban surroundings reduce the wind while open surroundings increase it, and the exposure can differ by direction. Account for topographic effects, because a building on or near a hill can experience wind speeds 30 to 50 percent higher than the flat-terrain value. These effects are captured in a wind tunnel study and approximated in the code; they must not be ignored.

### Coordinate the Wind Loads With the Structural Dynamic Properties

The wind response of a tall building is dynamic, and it depends on the building's natural frequencies, mode shapes, and damping — properties that the wind load cannot be separated from. Establish the structural dynamic properties (frequencies, mode shapes, mass distribution, damping) with realistic values, because the wind response is sensitive to them, and coordinate the wind engineering with the structural analysis. A wind tunnel study uses the structural properties to calculate the response, so the properties used in the study must match the final design, and any change in stiffness, mass, or damping requires the wind response to be re-evaluated. The wind load and the structural properties are one coupled problem, not two separate inputs.

## Common Traps

### Code Wind Loads Applied to a Building Outside the Code Limits

The engineer applies the ASCE 7 or code wind procedure to a tall, slender, or unusually shaped building that exceeds the procedure's height, slenderness, or shape limits. The mechanism is that the code procedure is familiar and a wind tunnel study feels like extra scope. The false signal is a completed wind load calculation. The harm is that the code loads under-predict the across-wind and torsional response, and the lateral system is under-designed for the true governing load. A wind tunnel study is required for buildings outside the code limits.

### Across-Wind Response Ignored, Along-Wind Assumed Governing

The engineer designs the lateral system for the along-wind load only, on a slender building where the across-wind vortex-induced response is larger. The mechanism is that the along-wind load is the intuitive load and the across-wind response requires a dynamic analysis or wind tunnel. The false signal is an along-wind load that "governs" by default. The harm is that the across-wind response, which can be several times the along-wind response on a slender building, is not designed for, and the lateral system is under-designed in the across-wind direction. All three directions must be evaluated.

### Cladding Designed for MWFRS Pressures, Not Component Pressures

The cladding and curtain wall are designed for the main wind-force-resisting system pressures, which are averages, rather than for the component-and-cladding pressures, which include localized peaks. The mechanism is that the MWFRS pressure is the one used for the frame and it feels like the wind load. The false signal is a cladding design that "uses the wind load." The harm is that the localized suctions at the corners and edges, which can be two to three times the average, are not designed for, and the cladding fails in the first significant wind event. The cladding must be designed for the C&C pressures.

### Occupant Comfort Not Checked at Service-Level Winds

The engineer designs the lateral system for strength at the design wind event and does not check the acceleration at the service-level (1-year or 10-year) wind event. The mechanism is that strength feels like the design and comfort feels like a preference. The false signal is a building that "meets the strength criteria." The harm is that the upper-floor accelerations exceed the comfort threshold, occupants experience unacceptable sway, and the building is functionally uninhabitable despite being structurally adequate. The service-level acceleration must be checked against the comfort criteria.

### Vortex Shedding and Lock-In Not Evaluated

The engineer does not assess the building's susceptibility to vortex shedding and lock-in, on a slender, flexible building where it governs. The mechanism is that vortex shedding is an across-wind dynamic effect that the static wind load does not reveal. The false signal is a wind load that "accounts for the wind." The harm is that at the critical wind speed, the vortex shedding locks in to the building's frequency, the across-wind response grows large, and the building experiences accelerations and stresses far exceeding the design values. Vortex shedding must be evaluated for susceptible buildings.

### Exposure or Topography Assumed Without Verification

The engineer assumes a single exposure category and ignores topographic effects, on a site where the exposure differs by direction or where topography accelerates the wind. The mechanism is that the exposure and topography feel like defaults. The false signal is a wind load based on "standard exposure." The harm is that the actual wind speed, amplified by topography or by a less-sheltered exposure in one direction, exceeds the assumed value, and the lateral system or cladding is under-designed. The exposure and topography must be verified for each direction.

### Structural Properties Changed Without Re-Evaluating Wind Response

The structural stiffness, mass, or damping is changed during design (value engineering, a different lateral system, a different facade) without re-running the wind tunnel study or the dynamic analysis. The mechanism is that the wind load feels like a fixed input. The false signal is a wind load on the drawing. The harm is that the changed properties alter the dynamic response — a softer building has more vortex shedding, a lighter building has higher acceleration — and the original wind loads no longer represent the true response. Any change in the dynamic properties requires the wind response to be re-evaluated.

## Self-Check

- Is a wind tunnel study specified for buildings that exceed the code's height, slenderness, shape, or dynamic response limits, rather than applying code procedures outside their validity?
- Are the along-wind, across-wind, and torsional responses all evaluated, with the lateral system designed for the governing combination rather than the along-wind load alone?
- Is the cladding and curtain wall designed for the component-and-cladding pressures (including corner and edge zone peaks), not the MWFRS average pressures?
- Is the occupant comfort (acceleration) checked at the service-level wind event (1-year or 10-year) against the applicable criteria, with mitigation provided if exceeded?
- Is the building's susceptibility to vortex shedding and lock-in evaluated, with the cross-section, damping, or stiffness adjusted to avoid lock-in for susceptible buildings?
- Are the wind directionality, the exposure category for each direction, and the topographic effects accounted for, with site-specific values rather than defaults?
- Are the structural dynamic properties (frequencies, mode shapes, mass, damping) used in the wind study consistent with the final design, with the wind response re-evaluated after any change?
