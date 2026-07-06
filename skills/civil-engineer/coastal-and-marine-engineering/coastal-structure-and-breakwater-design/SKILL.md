---
name: coastal-structure-and-breakwater-design.md
description: Use when the agent is designing rubble mound or vertical wall breakwaters, selecting armor stone or concrete armor units, setting crest elevation and freeboard, evaluating wave overtopping and toe scour, or sizing breakwater cross-sections under the USACE Coastal Engineering Manual and the CIRIA/CUR/CETMEF rock manual.
---

# Coastal Structure and Breakwater Design

Coastal structure and breakwater design is the engineering of fixed works that shelter harbors, shorelines, intake basins, and reclaimed land from wave attack, and it is governed by the USACE Coastal Engineering Manual (CEM, EM 1110-2-1100) together with the CIRIA/CUR/CETMEF Rock Manual and national standards such as BS EN 13383 for armor stone. The harm this skill prevents is a structure that is destroyed or severely damaged in the design storm: armor units that roll out of the cover layer, a concrete caisson that slides or overturns, a crest set too low so that catastrophic overtopping floods the harbor and endangers users, or a toe that scours until the armor layer collapses into the pit. The judgment calls are large and irreversible because coastal works are capital-intensive, built into a high-energy environment, and very expensive to repair once damaged; an armor stone gradation that is too broad, a Hudson or Van der Meer coefficient applied to the wrong slope, or a design wave taken from deep water without refraction and breaking conversion can each turn a compliant-looking section into a failure. Agents must treat the work as decision support for a licensed coastal engineer and must never finalize a cross-section, armor selection, or crest elevation without the met-ocean design conditions and a stability check at the governing water level.

## Core Rules

### Fix the Design Wave and Water Level Before Sizing Anything

The armor weight, crest elevation, and toe stability all depend on the design wave height and the water level at which it acts, so these must be fixed first and documented. Derive the design wave from a representative return period (typically the 100-year or the project-specific design return period consistent with the consequence class) using hindcast or measured data, transform it to the structure site accounting for refraction, shoaling, depth-limited breaking, and wave setup, and select the significant wave height Hs and the spectral peak period Tp at the toe. Confirm whether the structure is in deep, shallow, or breaking water, because the stability formula and the wave height to use change: non-breaking waves use the Hudson or Van der Meer equations, while breaking waves require a breaking-wave stability treatment and often a much heavier armor. Pair the design wave with the correct water level (mean high water spring plus storm surge and setup, or the design still water level), because stability, runup, and overtopping are all evaluated at the water level that produces the worst combination, not at mean sea level.

### Choose the Stability Method for the Right Structure and Wave Climate

For rubble mound slopes, select between the Hudson equation (KD-based, suited to plunging waves on relatively permeable structures with a single design period) and the Van der Meer formulae (which distinguish plunging and surging waves through the surf similarity parameter and account for storm duration, permeability, and damage level). The Hudson equation is simpler but hides the period dependence and damage accumulation; Van der Meer is preferred for major works and for structures exposed to long-period or surging waves, where the Hudson KD values can be unconservative. For concrete armor units (tetrapod, dolos, accropode, core-loc, xbloc), use the manufacturer-published or CEM stability coefficients and respect the placement pattern, packing density, and interlock requirements, because these units depend on interlock and a loose or mislaid unit initiates progressive failure. For vertical (caisson) breakwaters, use the Goda wave pressure method or the CEM vertical-wall pressure distributions, and check sliding, overturning, bearing, and foundation stability separately, because a caisson that satisfies overturning may still slide.

### Set Crest Elevation from Overtopping and Runup, Not from a Rule of Thumb

The crest elevation of a rubble mound or vertical breakwater must be set so that the mean overtopping discharge at the design water level and wave condition stays below the limit appropriate to the users and assets behind the structure, using the EuroTop or CEM overtopping formulae. Overtopping limits are far lower than most engineers assume: vehicles and pedestrians require discharges on the order of 0.001 to 0.01 l/s per m for safety, while a paved revetment or armored back slope may tolerate several l/s per m; selecting the wrong limit produces either a dangerous structure or an overbuilt one. Compute runup R2% for the armor type and slope, account for the roughness and porosity reduction factors, and recognize that a crown wall, parapet, or recurve can reduce overtopping but concentrates impact loading and wave reflection that must then be designed for. Never set the crest simply as a freeboard above mean sea level, because two structures with the same freeboard can have overtopping rates that differ by orders of magnitude depending on slope, roughness, and period.

### Detail the Cross-Section, Underlayer, Toe, and Filter to Prevent Progressive Failure

A rubble mound breakwater is a layered system, and each layer must be sized and graded against its neighbor so that the cover layer is stable, the underlayer supports and filters the armor, and the core does not wash out. Size the underlayer stone at roughly 1/10 to 1/15 of the armor mass per the CEM and Rock Manual filter rules, ensure the core material is graded to satisfy the filter criteria against the underlayer, and provide a geotextile or graded filter where fine seabed could be pulled through the core under wave pumping. Design the toe for the worst combination of scour, wave-induced lift, and bearing, because toe failure is the most common initiator of armor displacement; provide a toe berm or apron sized for the predicted scour depth, and verify toe stability with the CEM toe stability formulae rather than assuming a buried toe is safe. Include crest width (typically at least three armor stones), a crown element designed for wave impact, and a clearly defined seaward and leeward slope, and document the gradation envelopes and placement tolerances so that quarry production and placement can be controlled.

### Account for Climate, Construction, and Maintenance Realities

Coastal structures live for decades to centuries and must be designed for the sea level rise and storm intensification projected over their service life, not for today's water levels; adjust the design water level, runup, and overtopping for the projected relative sea level rise, and check whether the structure retains acceptable overtopping and freeboard at end of life. Consider constructability: marine placement is weather-window-limited, armor and core must be produced by an achievable quarry gradation, and very heavy units may exceed crane or barge capacity, which forces a rethink of unit type or section. Plan for maintenance and post-storm repair, because even a well-designed rubble mound will suffer minor damage in extreme events and must be accessible for re-placement of armor; document the expected damage level at the design event and the storm that triggers inspection and repair. Do not present a section as maintenance-free, because that misrepresents the structure to the owner and hides the lifecycle cost.

## Common Traps

### Using the Deep-Water Design Wave at the Toe

The deep-water 100-year Hs is used directly in the stability equation without transformation, so the structure is sized for a wave that never reaches the toe because it breaks in shallower water. The false signal is a conservative-looking heavy armor; the harm is wasted cost and a false sense of rigor, while the real governing condition (a depth-limited breaking wave at surge high water) may be heavier still and is never checked.

### Applying Hudson KD Outside Its Valid Range

A single KD value is taken from a table and applied to a steep slope, a long-period surging wave, or a low-permeability core outside the equation's calibrated range. The mechanism is that Hudson hides period and permeability effects, so the false signal of a textbook coefficient hides an under-stable cover layer; the harm is armor displacement in the first design storm, often as progressive loss once a few units move.

### Setting Crest Elevation by Freeboard Alone

The crest is set a fixed distance above mean sea level without an overtopping calculation, so two structures with identical freeboard are assumed equivalent. The mechanism is that overtopping depends on runup, slope roughness, and period, so the false signal of an adequate freeboard hides a dangerous discharge; the harm is a crest that floods the harbor or throws debris onto a roadway in the design event.

### Ignoring Toe Scour Until the Armor Collapses

The toe is buried and assumed stable, with no scour prediction or toe berm, so the seabed lowers under storm currents and breaking waves until the lowest armor units lose support. The mechanism is that toe stability is treated as a given, so the false signal of a complete cross-section hides the loss of founding support; the harm is a cover layer that slumps into the scour pit and a breach that is far more costly to repair than to prevent.

## Self-Check

- Is the design wave derived from hindcast or measured data at a documented return period, transformed to the toe with refraction, shoaling, breaking, and setup, and paired with the governing water level?
- Is the stability method (Hudson, Van der Meer, manufacturer coefficients for armor units, or Goda for vertical walls) matched to the wave climate, slope, and structure type, with the surf similarity parameter checked where relevant?
- Is the crest elevation set from a EuroTop or CEM overtopping calculation against a limit appropriate to the users and assets behind the structure, not from a freeboard rule of thumb?
- Are the underlayer, core, and filter graded to satisfy CEM and Rock Manual filter criteria, with a geotextile or graded filter where the seabed could be pulled through?
- Is the toe designed for predicted scour, lift, and bearing with a toe berm sized for the scour depth, rather than assumed stable because it is buried?
- Has the section been checked for the projected relative sea level rise and storm conditions at end of life, with the expected damage level and maintenance triggers documented?
- Are the armor gradation envelopes, placement tolerances, and constructability constraints (quarry production, crane and barge capacity, weather windows) realistic and stated for the contractor?
