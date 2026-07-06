---
name: light-quality-and-specification.md
description: Use when the agent is selecting lamp color temperature and CRI, specifying fixture photometrics and beam spread, calculating illuminance levels, choosing dimming protocols, or evaluating whether a lighting specification meets the functional and visual quality needs of a space.
---

# Light Quality and Specification

A lighting specification is not a fixture schedule; it is a set of decisions about the quality of light that will exist in a space, and those decisions — color temperature, color rendering, beam characteristics, dimming behavior, illuminance levels — determine whether the space feels right and functions well far more than the visual appearance of the fixtures. The judgment problem is that fixtures are often selected for their look and price while the lamp inside them is treated as a generic commodity, and the result is beautiful fixtures producing ugly light: a high-CRI requirement missed so every skin tone looks gray, a beam spread mismatched so a painting has a hot spot and a dark edge, a dimmer incompatible so the fixtures flicker and buzz, an illuminance level calculated for the average that leaves the actual task area in shadow. The deeper trap is that lighting quality is invisible on the fixture cut sheet and only reveals itself in the occupied space, by which point correction is expensive. This skill covers the decisions that determine whether the specified lighting delivers the quality the space requires.

## Core Rules

### Specify Color Temperature Deliberately and Consistently Within a Space

Color temperature (measured in Kelvin) determines the warmth or coolness of the white light, and it must be chosen for the function and mood of the space and held consistent within a single visual field. Warm light (2700K) suits residential and hospitality, conveying intimacy and warmth; neutral warm (3000K) suits retail and many offices; neutral (3500K) and cool (4000K+) suit clinical, task, and commercial applications where alertness and color discrimination matter. The decision criterion is to choose the temperature for the space's function, then specify every source in that space at the same temperature (within tolerance), because mixing 2700K and 4000K sources in one room produces a jarring, unprofessional result. Consistency of color temperature within a space is a non-negotiable quality marker.

### Require High Color Rendering for Spaces Where Color Matters

The color rendering index (CRI) measures how accurately a light source renders colors compared to a reference, and low-CRI sources make skin look gray, food look unappetizing, and fabrics look off. The rule is to specify CRI 90+ (and ideally CRI 95+) for residential, hospitality, retail, healthcare, and any space where people, food, art, or merchandise are viewed. For garages, storage, and purely functional circulation, CRI 80 may suffice. For critical color work (art studios, retail color matching), request the TM-30 fidelity and gamut data, which gives a fuller picture than CRI alone. A CRI 80 lamp in a living room makes the carefully chosen palette look dead and is a false economy.

### Match Beam Spread and Photometrics to the Application

The same fixture is available with different beam spreads (narrow spot, narrow flood, flood, wide flood) and different distributions, and the wrong choice produces hot spots, dark edges, and uneven illumination. Art lighting requires a beam that covers the artwork evenly with a soft falloff, typically a beam spread 10-15 degrees wider than the object's diagonal; wall washing requires asymmetric distribution fixtures placed at a consistent distance from the wall; grazing a textured wall requires a narrow beam close to the surface. The decision criterion is to read the photometric file (the IES distribution curve) and verify the beam covers the target at the installed distance, rather than assuming "a fixture is a fixture." Fixture placement and beam spread are a matched pair.

### Calculate Illuminance to the Task, Not to the Average

Published recommended illuminance levels (in foot-candles or lux) are starting points for the task in question — roughly 30 fc for general office, 50 fc for detailed task work, 10-20 fc for residential ambient. The trap is designing to the average across the room, which leaves the actual task area underlit and the circulation overlit. The rule is to calculate the illuminance at the task surface (counter, desk, reading chair) using the fixture's photometrics, the mounting height, and the room's reflectances, and to verify the task area meets the target. The average can be lower if the task area is correctly lit.

### Specify Dimming Protocol and Verify Compatibility

Dimming is not automatic; it depends on the compatibility of the lamp, the driver, the dimmer, and the control protocol. LED fixtures in particular are notorious for flicker, pop-on, drop-out, and audible buzzing when paired with the wrong dimmer. The rule is to specify the dimming protocol (forward-phase TRIAC, reverse-phase ELV, 0-10V, DALI, DMX) based on the fixture type and the control system, and to verify in writing that the specified lamp, driver, and dimmer are tested compatible, ideally with a manufacturer's compatibility letter. "It should dim" is not a specification; flicker and buzz at installation are expensive to fix.

### Account for Lumen Maintenance, Glare, and Fixture Aging

A fixture's initial lumen output declines over its life (lumen depreciation), the lens accumulates dust and yellowing, and the perceived brightness includes the fixture's direct glare. The decision criterion is to design to maintained illuminance — the light level at the end of the maintenance cycle — not to initial output, which means oversizing roughly 10-20% to account for depreciation, and to specify fixtures with good glare control (deep baffles, louvers, proper cutoff angles) so the light reaches the surface without shining into the eyes. UGR (unified glare rating) matters in commercial and office work; a bright bare lamp in the field of view is a quality failure regardless of the illuminance number.

## Common Traps

### Selecting Fixtures for Appearance and Ignoring the Lamp

The designer chooses a beautiful pendant or recessed fixture based on its look and price, and accepts whatever lamp the contractor installs. The harm is that the lamp may be the wrong color temperature (cool white in a warm residential interior), low CRI (making skin and food look dead), non-dimmable, or of inconsistent batch quality. The mechanism of failure is that the fixture was specified but the light was not. The defense is to specify the lamp (color temperature, CRI, lumen output, dimming protocol) as explicitly as the fixture, and to require submittal approval on the lamp, not just the housing.

### Mixing Color Temperatures in One Space

Through inattention or incremental additions, a room ends up with 2700K recessed lights, 3000K pendants, a 4000K under-cabinet strip, and a 5000K desk lamp, all visible at once. The harm is a visually discordant space that reads as unprofessional and unsettled, because the eye perceives the color temperature mismatch as a quality problem even when it cannot articulate why. The mechanism of failure is that each fixture was specified individually without a master color temperature decision for the space. The defense is to set one color temperature for the space and verify every source conforms.

### Assuming All LEDs Dim the Same Way

The designer specifies "dimmable LED" fixtures and a generic dimmer, assuming compatibility. At installation, some fixtures flicker, some buzz, some pop on at 40% and drop out below 20%, and some do not dim at all. The mechanism of failure is that LED dimming depends on the match between the driver, the dimmer type, and the control protocol, and "dimmable" on the box means only that the lamp can dim with some dimmer, not with the one specified. The harm is expensive rework and a lighting system that never performs as intended. The defense is to specify the dimming protocol and obtain a written compatibility confirmation.

### Calculating Illuminance to the Room Average

The designer uses a lighting calculation tool to verify the average illuminance across the room meets the target, and concludes the design is adequate. But the average hides that the task area (the counter, the desk, the reading chair) is in shadow while the circulation is overlit. The mechanism of failure is that the average was the wrong metric; the task surface illuminance is what determines whether the user can actually see to work. The false signal is that the calculation "passed." The defense is to calculate and verify illuminance at the task plane, not the room average.

### Ignoring Glare and Cutoff

The designer specifies bright bare lamps or shallow recessed fixtures with no baffle, chasing a modern minimalist look. The harm is that the fixtures are visible as bright points of glare in the field of view, causing discomfort, squinting, and visual fatigue, and the space reads as harsh despite adequate illuminance numbers. The mechanism of failure is that the fixture's luminance (brightness as perceived) was not controlled, only its illuminance (light on the surface). The defense is to specify fixtures with proper cutoff angles, baffles, or louvers, and to check the UGR for commercial applications.

## Self-Check

- Did I choose the color temperature for the space's function and mood, and specify every source in the visual field at the same temperature?
- Did I require CRI 90+ (or TM-30 data for critical applications) for spaces where color rendering matters?
- Did I read the photometric files and match beam spread and distribution to each application (art, wall wash, grazing, task)?
- Did I calculate illuminance at the task surface, not just the room average, and verify the task area meets the target?
- Did I specify the dimming protocol and obtain written confirmation of lamp-driver-dimmer compatibility?
- Did I design to maintained illuminance (accounting for lumen depreciation and aging) rather than initial output?
- Did I specify fixtures with proper glare control (cutoff, baffles, louvers) and check UGR for commercial applications?
