---
name: notification-appliance-circuit-sizing.md
description: Use when the agent is sizing notification appliance circuits (NAC), selecting horns strobes and speakers, calculating current draw and NAC voltage drop, laying out NAC zones, setting strobe candela spacing per NFPA 72 and ADA, or resolving sync protocols and audible coverage.
---

# Notification Appliance Circuit Sizing

A fire alarm notification system that is loud enough on the bench can be silent at the end of a long circuit, because the voltage that reaches the last strobe depends on a current and voltage-drop calculation that is easy to skip and impossible to fudge during a fire. The judgment problem is that an installer who counts appliances, picks a candela by eye, and trusts that the panel will drive them will produce a circuit where the near appliances flash brightly and the far ones dim or fail, where the strobe spacing leaves dead spots for low-vision occupants, and where horns and strobes from different manufacturers flash out of sync. The NAC voltage drop, the candela spacing, the current budget, and the sync protocol are the decisions that determine whether notification actually reaches every occupant. This skill covers the sizing, spacing, and synchronization decisions that determine whether a NAC performs under alarm.

## Core Rules

### Calculate NAC Voltage Drop to the Last Appliance

Every NAC has a power source at one end and a series of appliances along the run, and the voltage drops along the wire as current flows, so the last appliance receives less than the source voltage. The voltage at the last appliance must remain above the appliance's minimum listed operating voltage under full alarm load, or that appliance will dim, flash erratically, or fail to operate. The calculation sums the current of all appliances on the circuit and computes the drop from the wire resistance and the loop length, using the correct conductor size and temperature-adjusted resistance. The calculation must use the alarm current, not the standby current, and must account for the battery voltage at the end of the required standby period, which is lower than the float voltage. A circuit that works on float voltage may fail on a discharged battery.

The trap is assuming the panel voltage reaches the end of the line. The defense is to calculate the voltage drop to the last appliance under full alarm load at battery end-of-discharge, and to confirm it stays above the listed minimum.

### Sum Appliance Current Against the NAC and Power Supply Budget

Each notification appliance draws a listed current, and the sum of all appliances on a NAC must stay within the current limit of the NAC output and within the budget of the power supply feeding it. Strobes in particular draw significant current, and a circuit with many high-candela strobes can exceed the NAC rating even when the voltage drop is acceptable. The power supply must be sized for the total alarm load of all NACs it feeds plus the panel load, with battery capacity for the required standby and alarm duration. Substituting an appliance with a higher current draw, or adding an appliance during a change order, can push an in-budget circuit over its limit. The current budget must be documented and rechecked whenever the appliance list changes.

The trap is counting appliances instead of summing current. The defense is to sum the listed current of all appliances against the NAC and supply limits, to document the calculation, and to recheck it after any change.

### Space Strobes for Candela Coverage Per NFPA 72 and ADA

Visible notification, strobes, must produce the required illumination throughout the protected space so that a person anywhere in the room sees the flash, and the spacing and candela rating are defined by NFPA 72 and the ADA to achieve that coverage. The required candela depends on the room size and the strobe spacing, with one strobe of a given candela covering a defined area, and larger rooms requiring either more strobes or higher candela units. Wall-mounted and ceiling-mounted strobes have different spacing tables, and the mounting height affects the coverage. The coverage must be verified for every room, including odd-shaped rooms and rooms with obstructions, because a strobe hidden behind a partition leaves a dead spot. The candela selection must match the listed spacing, and over-driving a room with excessive candela is as non-compliant as under-covering it.

The trap is placing one strobe per room by eye. The defense is to calculate the candela and spacing per NFPA 72 for each room, to account for obstructions, and to verify coverage for every space.

### Achieve Audible Coverage Above the Required Sound Level

Audible notification, horns and speakers, must produce a sound level above the ambient that is intelligible or at least attention-getting throughout the protected area, and NFPA 72 defines the required level above average and peak ambient sound levels. The average ambient in a quiet office is low, so a modest horn suffices, but a machinery room or a space with background music requires a louder notification to be heard above the ambient. The sound level must be measured at the listener position, not assumed from the appliance rating, because walls, partitions, and distance attenuate the sound. For voice systems, intelligibility, not just audibility, is required, which often means more lower-output speakers rather than fewer loud ones. The audible coverage must be verified by measurement during acceptance, especially in high-ambient spaces.

The trap is assuming the horn rating equals the room level. The defense is to measure the sound level above ambient at listener positions, to use louder or more appliances in high-ambient spaces, and to verify coverage during acceptance.

### Synchronize Strobes to Avoid Seizure Risk and Confusing Flash

When multiple strobes are visible from one location, they must flash in synchronization, both to avoid the photosensitive seizure risk that asynchronous flashing creates and to present a clear, unified signal. Synchronization is achieved by sync modules, by built-in sync protocols on the NAC, or by addressable notification appliances, and all strobes on a circuit and on adjacent circuits visible from the same point must be synchronized to the same protocol. Mixing strobes from different manufacturers, or mixing synchronized and unsynchronized appliances, defeats the sync and can produce a chaotic, potentially hazardous flash pattern. The sync protocol must be verified during acceptance by observing multiple strobes from the locations where they are jointly visible.

The trap is mixing strobe brands or protocols on one circuit. The defense is to use a single sync protocol across all jointly visible strobes, to avoid mixing manufacturers, and to verify synchronization during acceptance.

### Lay Out NAC Zones to Match the Evacuation and Survivability Plan

NAC zones group appliances that activate together, and the zoning must match the building's evacuation plan and the survivability requirements, so that the right notification reaches the right occupants and a single fault does not silence a whole area. A NAC that spans multiple floors or fire zones creates a single point of failure that can disable notification across a wide area, so zones should be bounded to match the building zones and should not cross areas that should notify independently. The survivability class of each NAC must be chosen for the application, with higher survivability required where the circuit must operate during a fire. The zone layout must be documented on the drawings and coordinated with the cause-effect matrix so that activation, zoning, and survivability are consistent.

The trap is zoning for wiring convenience. The defense is to lay out NAC zones to match the evacuation and survivability plan, to bound zones to building areas, and to coordinate zoning with the cause-effect matrix.

## Common Traps

### Voltage Drop Ignored, Last Strobe Dims or Fails

The installer connects a string of strobes to a NAC and confirms the near ones flash, ignoring the voltage at the far end. The mechanism of the trap is that voltage drops along the wire under load, and the last appliance may receive below its listed minimum, especially on a long run or a small conductor, so it flashes dimly, erratically, or not at all during alarm, and the problem is invisible until a real event. The false signal is that the near strobes flash brightly during a bench test, which proves the circuit works at the source but not at the end. The harm is silent notification at the far end of the building. The defense is to calculate the voltage drop to the last appliance at battery end-of-discharge.

### Current Summed Wrong, NAC Overloaded in Alarm

The installer counts the appliances and assumes they fit, without summing the listed alarm current against the NAC and supply limits. The mechanism of the trap is that strobes draw substantial current, and a circuit with many high-candela strobes can exceed the NAC output rating or the power supply budget, causing the supply to sag, current-limit, or drop appliances when all are active in alarm. The false signal is that the appliances flash when tested a few at a time, which never exercises the full alarm load. The harm is a circuit that fails when all appliances activate. The defense is to sum the listed current of all appliances and to verify against the NAC and supply limits.

### Strobe Spacing by Eye Leaves Candela Dead Spots

The installer mounts one strobe per room based on a quick look and picks a candela that seems bright. The mechanism of the trap is that NFPA 72 and ADA define candela and spacing by room size and geometry, and a single strobe of the wrong candela or the wrong position leaves areas of the room below the required illumination, particularly behind partitions or in odd-shaped rooms, so occupants in those dead spots do not see the flash. The false signal is that the strobe is visibly bright from the door, which proves it works but not that it covers the room. The harm is occupants who miss the visible notification. The defense is to calculate candela and spacing per NFPA 72 for each room.

### Audible Level Assumed From the Appliance Rating

The installer selects a horn by its decibel rating and assumes it covers the space. The mechanism of the trap is that the appliance rating is measured at a reference distance, and walls, partitions, and distance attenuate the sound, so the level at the listener position may fall below the required level above ambient, especially in a high-ambient or partitioned space, leaving occupants unable to hear the alarm. The false signal is that the horn sounds loud next to the appliance, which proves output but not coverage. The harm is inaudible notification in part of the space. The defense is to measure the sound level above ambient at listener positions and to add appliances where needed.

### Mixed Strobe Brands Flash Out of Sync

The installer adds strobes from a different manufacturer to complete a circuit, assuming a strobe is a strobe. The mechanism of the trap is that synchronization depends on a common sync protocol, and mixed brands use incompatible protocols, so the strobes flash asynchronously when jointly visible, creating a confusing signal and a photosensitive seizure risk that the code prohibits. The false signal is that each strobe flashes when activated, which proves each works but not that they are synchronized. The harm is a non-compliant and potentially hazardous flash pattern. The defense is to use a single sync protocol across all jointly visible strobes and to verify synchronization.

### NAC Zone Spans Multiple Floors Creating a Single Failure Point

The installer runs one NAC across two floors to save wire, treating the circuit as a single zone. The mechanism of the trap is that a single fault on that circuit disables notification on both floors, and the zoning does not match the building's evacuation plan, so a fire on one floor may silence notification on another, and the survivability of the wide-spanning circuit is compromised. The false signal is that all appliances on the circuit activate during a test, which proves the wiring but not the zoning logic. The harm is widespread loss of notification from a single fault. The defense is to bound NAC zones to building areas and to match the survivability plan.

## Self-Check

- Did I calculate the NAC voltage drop to the last appliance under full alarm load at battery end-of-discharge, and confirm it stays above the listed minimum operating voltage?
- Did I sum the listed alarm current of all appliances against the NAC output and power supply limits, document the calculation, and recheck it after any appliance change?
- Did I calculate the strobe candela and spacing per NFPA 72 and ADA for each room, account for obstructions and odd shapes, and verify coverage for every space?
- Did I measure the audible sound level above average and peak ambient at listener positions, use louder or more appliances in high-ambient spaces, and verify intelligibility for voice systems?
- Did I use a single sync protocol across all strobes visible from one location, avoid mixing manufacturers, and verify synchronization during acceptance?
- Did I lay out NAC zones to match the evacuation and survivability plan, bound zones to building areas rather than spanning floors, and coordinate zoning with the cause-effect matrix?
- Did I document the voltage drop, current budget, candela spacing, and zone layout on the drawings and in the calculation record for the AHJ?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
