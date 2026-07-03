---
name: emergency-and-exit-lighting-systems.md
description: Use when the agent is designing or installing egress lighting, battery-backed emergency luminaires and unit equipment, transfer equipment for emergency systems, or planning the testing and maintenance schedule required for life-safety lighting in commercial and assembly occupancies.
---

# Emergency and Exit Lighting Systems

Emergency and exit lighting is the system that must work in the dark, during the panic, after the failure that killed normal power. It is life-safety equipment, and its failure is discovered at the worst possible moment by people trying to find a door in smoke and darkness. The judgment problem is that this equipment sits idle for months or years, looks identical to ordinary lighting when normal power is on, and is easy to ignore until it is needed. An electrician who installs emergency lighting as an afterthought, with dead batteries, blocked exit signs, and untested transfer equipment, has installed a system that will fail the one time it is asked to perform. This skill covers the decisions that determine whether egress lighting actually illuminates, for the required duration, after the required event.

## Core Rules

### Provide Illumination at the Required Level Along the Entire Egress Path

Means-of-egress lighting must provide a minimum illuminance at the floor along the entire path of travel, including stairs, aisles, corridors, and exit discharge, measured at average and minimum values. The illumination must be present whenever the building is occupied, and the emergency portion must reach the required level within seconds of normal power failure. The path includes every section a person must traverse to reach a public way, and a dark gap in the middle of an otherwise lit corridor is a failure of the whole system. Illumination levels are measured in footcandles or lux at the floor, not by counting fixtures.

The trap is spacing fixtures so that pools of light leave dark gaps, or covering only the main corridor and not the exit discharge. The defense is to calculate or measure the illuminance at the floor along the full path, to verify both average and minimum values, and to confirm coverage extends through stairs and discharge to the public way.

### Size Battery and Unit Equipment for the Full 90-Minute Duration

Emergency lighting powered by batteries or unit equipment must supply the required illumination for not less than 90 minutes after normal power failure. The battery must be sized to carry the full connected emergency load, including all designated egress luminaires, for that duration at end-of-life voltage. Battery capacity degrades over time, so sizing at the nameplate load with no margin guarantees that an aged battery fails before 90 minutes. The 90-minute duration is a hard life-safety requirement, not a target.

The trap is sizing the battery to the connected load with no aging margin, or assuming a 90-minute-rated unit will deliver 90 minutes after years of service. The defense is to size the battery with aging margin, to derate for the expected service life, and to verify at commissioning that the system sustains the load for the full 90 minutes from a fully discharged and recharged battery.

### Select the Transfer Method Appropriate to the System Type

Emergency lighting transfer from normal to emergency power is achieved by different means depending on the system. Self-contained unit equipment and emergency ballasts or drivers transfer internally at each luminaire. Central inverter systems transfer at the inverter feeding a dedicated emergency circuit. Generator-backed emergency systems transfer through an automatic transfer switch (ATS) serving the emergency load. The transfer must occur within the time permitted for the system type, often 10 seconds for life safety, and the selected method must be listed and compatible with the luminaire type, especially for LED drivers that need uninterrupted or rapid transfer.

The trap is mixing transfer methods that do not coordinate, or expecting a generator ATS to transfer fast enough for life safety without verifying the 10-second requirement. The defense is to select the transfer method for the system type, to verify the transfer time meets the life-safety requirement, and to confirm LED emergency drivers are compatible with the normal driver and the transfer behavior.

### Place Exit Signs for Continuous Visibility Along the Path of Travel

Exit signs must be visible along the entire path of travel to the exit discharge, with signs placed at every required point: at exit doors, at every intersection or change of direction, and wherever the path is not obvious. Signs must be illuminated, either internally or externally, at all times the building is occupied, and the illumination must continue under emergency power. Tactile and visually contrasting requirements apply for accessibility. A sign that is visible from one direction but not from the approach direction, or that is blocked by an open door, fails the visibility requirement.

The trap is placing exit signs at doors but not at every change of direction, or mounting them where a propped door or stored material blocks them. The defense is to walk the path of travel in both directions and verify a sign is visible at every decision point, that signs are illuminated at all times, and that mounting height and contrast meet accessibility requirements.

### Establish and Document the Testing Regimen From Day One

Emergency and exit lighting must be tested on a defined schedule: a monthly 30-second functional test and an annual 90-minute full-duration test, with results documented. Self-testing and self-diagnostic equipment reduces the labor but does not eliminate the documentation. The testing requirement exists because batteries and transfer relays fail silently between events, and the only way to know the system works is to test it under load. A system that is never tested is a system whose failure is unknown until an emergency.

The trap is installing the equipment and leaving testing to the owner without establishing the regimen, or relying on the monthly test light without performing the full annual duration test. The defense is to set up the testing schedule at commissioning, to document the baseline 90-minute test, and to use self-diagnostic equipment where available while still maintaining the written record.

### Coordinate Emergency and Normal Lighting so Both Are Not Required to Fail Together

Emergency lighting is meant to cover the failure of normal power, but if emergency luminaires are integrated into normal fixtures that share a common failure mode, a single event can drop both. The design must ensure that the emergency subset remains powered independently of the normal system, whether by separate circuit from an emergency panel, internal battery in designated fixtures, or a central inverter on a dedicated feed. The emergency system must not depend on the same raceway, transformer, or panel that feeds the normal lighting it backs up.

The trap is wiring emergency luminaires on the same circuit as the normal lighting they supplement, so a single circuit failure drops both. The defense is to feed emergency lighting from an independent source, to verify the emergency subset survives the loss of any single normal component, and to route emergency circuits in separate raceways where survivability requires it.

## Common Traps

### Spacing Fixtures So That Light Pools Leave Dark Gaps

An electrician installs emergency fixtures at convenient intervals and assumes the corridor is covered, but the illuminance at the floor between fixtures falls below the minimum in the dark gaps. The mechanism of the trap is that the requirement is measured illuminance at the floor, not fixture count, and pools of light with dark intervals between them fail the minimum even when the average looks acceptable. The false signal is that fixtures are present and lit, which looks like coverage but does not meet the measured minimum along the path. The harm is evacuees navigating through alternating light and dark patches in an emergency, increasing fall and crush risk. The defense is to measure or calculate floor illuminance along the full path and to add fixtures or adjust spacing until the minimum is met everywhere.

### Sizing the Battery With No Aging Margin

A central inverter battery is sized exactly to the connected emergency load for 90 minutes at nameplate capacity, and after three years of service the battery delivers only 70 minutes. The mechanism of the trap is that battery capacity declines with age and cycling, and sizing at nameplate with no margin guarantees the system falls below 90 minutes well before replacement. The false signal is the nameplate 90-minute rating, which holds when new but not after years of degradation. The harm is an emergency lighting system that extinguishes before occupants have evacuated, the precise failure the duration requirement prevents. The defense is to size with aging margin and to verify duration at commissioning and at each annual test.

### Expecting a Generator ATS to Transfer Fast Enough Without Verifying

A facility relies on a generator-backed emergency system, and the electrician assumes the ATS transfers within the 10 seconds permitted for life safety, but the generator is large and slow to start and reach voltage. The mechanism of the trap is that life-safety transfer has a maximum time, and a slow generator or a misadjusted ATS can exceed it, leaving egress paths dark for too long. The false signal is that the system transfers eventually, which proves function but not the time limit. The harm is a gap of darkness long enough to cause panic and injury during evacuation. The defense is to measure the transfer time at commissioning, to add battery or unit equipment bridging where the generator exceeds the permitted time, and to verify the 10-second life-safety window.

### Placing Exit Signs Only at Doors, Not at Every Change of Direction

Exit signs are installed at the exit doors but not at the corridor intersection where an evacuee must choose a direction, so a person walking the wrong way sees no sign correcting them until they reach a dead end. The mechanism of the trap is that the path of travel includes every decision point, and a sign only at the destination does not guide someone who is uncertain at an intersection. The false signal is that the exit doors are marked, which is necessary but not sufficient for path guidance. The harm is evacuees moving the wrong direction in an emergency. The defense is to walk the path and place signs at every required point, including intersections and changes of direction, and to verify visibility from the approach direction.

### Relying on the Monthly Test Light Without the Annual Duration Test

The facility performs the monthly 30-second test by checking the unit's test light, but the annual 90-minute full-duration test is skipped because it is inconvenient. The mechanism of the trap is that the 30-second test verifies the battery has some charge but not that it can sustain the load for 90 minutes, which is exactly what the annual test is designed to confirm, and batteries commonly pass the short test while failing the long one. The false signal is the green test light, which suggests health but does not prove duration. The harm is a system that passes monthly checks and then fails at 40 minutes during a real outage. The defense is to perform and document the full 90-minute annual test under load.

### Wiring Emergency Luminaires on the Same Circuit as the Normal Lighting

To save wiring, the electrician puts the emergency fixtures on the same circuit as the normal corridor lighting, intending them to switch to internal battery on power loss. The mechanism of the trap is that if the cause of the outage is a fault on that shared circuit rather than a building-wide power loss, the emergency fixtures lose their normal feed and switch to battery, but the fault also may affect the emergency subset, and the design removes the independence that emergency lighting requires. The false signal is that the fixtures have batteries and will light on power loss, which is true for a total outage but not for a circuit-level fault. The harm is a single fault dropping both normal and emergency lighting on that path. The defense is to feed emergency lighting from an independent emergency source or panel.

## Self-Check

- Did I provide the required minimum and average illuminance at the floor along the entire egress path, including stairs, intersections, and the exit discharge to the public way?
- Did I size the battery or unit equipment to carry the full connected emergency load for at least 90 minutes with aging margin, and verify the duration at commissioning?
- Did I select a transfer method listed for the system type, verify it transfers within the life-safety time limit (often 10 seconds), and confirm LED emergency drivers are compatible with the transfer behavior?
- Did I place exit signs at every required point, including doors, intersections, and changes of direction, and verify visibility from the approach direction and accessibility contrast?
- Did I establish and document the monthly 30-second and annual 90-minute testing regimen at commissioning, including a baseline full-duration test result?
- Did I feed emergency lighting from an independent source so that a single normal-circuit or component failure cannot drop both normal and emergency lighting on the same path?
- Did I route emergency circuits in separate raceways or listed survivable assemblies where the occupancy requires fire endurance, so the system survives long enough to function?
- Are assumptions, uncertainties, and confidence levels stated explicitly rather than buried in a confident-sounding conclusion?
