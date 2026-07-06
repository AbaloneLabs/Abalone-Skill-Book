---
name: commercial-pa-and-background-music-systems.md
description: Use when the agent is designing or wiring 70V or 100V constant-voltage paging and background music systems, spacing speakers for coverage, zoning with volume controls, sizing amplifiers against transformer taps, or configuring page override and background music sources.
---

# Commercial PA and Background Music Systems

A commercial paging and background music system is one of the few electrical systems whose success is judged entirely by the ear, and the ear is unforgiving of the mistakes that are easiest to make. A 70V system that is wired with the wrong tap scheme will be too quiet in one zone and deafening in the next; an amplifier sized to the sum of the taps with no headroom will clip and distort on the first loud page; a page-override that is not wired correctly will chop the music but never let the page through; and a speaker layout spaced for square footage rather than for even coverage will leave half the floor in an acoustic shadow. The judgment problem is that constant-voltage distribution makes it trivially easy to hang dozens of speakers on one amplifier, which hides the fact that the taps, the spacing, the zoning, and the headroom still have to be engineered. This skill covers the decisions that determine whether a paging system is intelligible, evenly covered, and reliable.

## Core Rules

### Engineer Around the Constant-Voltage Distribution Model

The 70V (or 100V, in regions that use it) constant-voltage system exists to allow many speakers on long runs without the line loss and impedance matching problems of low-impedance distribution. Each speaker has a transformer with primary taps at several wattage levels, and the amplifier sees the parallel combination of all the tapped loads as a high impedance that allows small-gauge wire over long distances. The engineering discipline is that the sum of all taps on a zone must not exceed the amplifier's rated output, and that each tap must be chosen for the coverage needed at that speaker, not set to maximum by default. A system where every speaker is tapped at its highest setting overloads the amplifier and wastes power where it is not needed.

The trap is tapping every speaker at maximum to be safe. The defense is to tap each speaker for the level required at its location, to sum the taps per zone, and to confirm the total fits within the amplifier's rating with headroom.

### Space Speakers for Even Coverage, Not for Symmetry

Speaker spacing determines the uniformity of sound across the space, and the goal is that a listener anywhere in the covered area hears roughly the same level and intelligibility. Ceiling speakers are typically spaced based on their dispersion pattern and the ceiling height, with spacing chosen so that the coverage circles overlap and there are no dead spots between speakers. Wider spacing covers more area per speaker but leaves gaps where the level drops and intelligibility suffers; tighter spacing improves uniformity but costs more speakers and amplifier power. The spacing must account for the ceiling height, because the coverage diameter at the listener's plane widens as the ceiling drops, and for the ambient noise level, which competes with the page.

The trap is spacing speakers on a regular grid chosen for visual symmetry. The defense is to calculate spacing from the speaker's dispersion and the ceiling height, to overlap coverage circles, and to verify the layout against the ambient noise level in each area.

### Zone the System by Use and Occupancy, Not by Floor Plan

Zones group speakers that should be controlled and paged together, and the grouping should reflect how the building is used, not how it is drawn. A restaurant may need separate zones for the dining room, the bar, and the kitchen, each with its own volume control, because the kitchen needs pages but not background music and the bar needs a different music level than the dining room. An office may need zones by department or by wing, so that a page reaches only the people it concerns. Zone volume controls allow local adjustment without affecting the rest of the system, and the zone boundaries must be planned before the wiring is pulled, because regrouping speakers across zones after installation requires re-wiring.

The trap is zoning by floor or by wing for convenience. The defense is to zone by occupancy and use, to provide local volume controls where independent adjustment is needed, and to plan the zone boundaries before pulling cable.

### Size the Amplifier to the Sum of Taps With Real Headroom

The amplifier must deliver the sum of all tapped speaker loads in its zone, plus headroom for the peaks that paging and music demand. A system whose tap sum exactly equals the amplifier's rating will clip on the first loud page, because real audio peaks exceed the average level by a significant margin, and a clipped page is not just unpleasant but unintelligible. Headroom of roughly 20 to 30 percent above the tap sum is a common target, more for systems with high peak pages or music with wide dynamic range. The amplifier must also be matched to the system voltage, 70V or 100V, and must be stable into the load presented by the parallel speaker network.

The trap is buying an amplifier whose rating equals the tap sum. The defense is to sum the taps per zone, to add headroom for peaks, and to select an amplifier rated above that total at the system voltage.

### Configure Page Override So Pages Always Get Through

In a combined paging and background music system, a page must override the music so that the page is intelligible, and the override mechanism must be wired and configured correctly. Page override typically works by a control contact from the paging source that mutes or ducks the music and routes the page to the zone, and the timing of the mute relative to the page is critical: the music must mute before the page audio arrives and restore after it ends, or the first word of the page is lost under the music. Some systems use automatic ducking that senses the page level, others use a hard contact, and the choice affects reliability. The override must also prioritize correctly when multiple page sources exist, such as a phone page and an emergency page.

The trap is assuming the override works because the music mutes. The defense is to test the override timing with real pages, to verify that the first word is not clipped, and to confirm the priority scheme when multiple page sources can trigger the system.

### Match Background Music Sources to the Use and the Signal Path

Background music sources, streaming services, satellite tuners, or local players, must be matched to the use and integrated into the signal path correctly. The source must deliver a line-level signal at the impedance the amplifier expects, and long unbalanced runs from a source to an amplifier can pick up hum, requiring a balanced interface or a local source. The source selection and routing must support the zoning scheme, so that different zones can play different sources or no source at all, and volume must be controllable per zone without affecting the page level. Music sources that are consumer-grade, with unbalanced outputs and no remote control, often underperform in a commercial distribution system.

The trap is plugging a consumer music source into a long unbalanced run. The defense is to use balanced interfaces for long source runs, to match the source to the amplifier's input expectations, and to confirm that zone volume controls affect music independently of the page.

### Use Listed Cable and Separation Appropriate to the Space

Paging speaker cable is often run in accessible ceiling spaces and plenum return airs, so the cable listing must match the space it passes through, plenum-rated where required, riser-rated in vertical shafts, and general-purpose only in non-plenum, non-riser locations. Paging cable is Class 2 or Class 3 and must be separated from line voltage to prevent induced hum and to preserve the safety assumptions of the power-limited circuit. The gauge must be adequate for the run length and the load, because even at 70V, long runs with many speakers can drop voltage at the end of the line, reducing the level of the last speakers.

The trap is pulling the cheapest jacketed wire regardless of the space. The defense is to match the cable listing to each space, to separate from line voltage, and to size the gauge for the run length and the tapped load.

## Common Traps

### Every Speaker Tapped at Maximum

The installer sets every speaker transformer to its highest tap to ensure none is too quiet. The mechanism of the trap is that maximum taps sum to a load that exceeds the amplifier's rating, causing distortion and possible amplifier failure, and that maximum level is also far too loud in smaller rooms and close-in locations, creating an uneven and unpleasant system. The false signal is that each speaker is clearly audible when tested individually, which proves the tap works but not that the sum is within the amplifier's capacity. The harm is a system that clips on pages, overloads the amplifier, and varies wildly in level from room to room. The defense is to tap each speaker for the level needed at its location and to sum the taps against the amplifier's rating with headroom.

### Speakers Spaced on a Visual Grid

The installer lays out ceiling speakers on a regular grid based on ceiling tile spacing or architectural symmetry. The mechanism of the trap is that a grid chosen for appearance ignores the speaker's dispersion pattern and the ceiling height, so the coverage circles may not overlap, leaving dead spots where the level drops and intelligibility suffers, or may overlap excessively, wasting speakers and amplifier power. The false signal is that the grid looks even and complete, which proves coverage of the ceiling but not of the listener's ears. The harm is paging that is unintelligible in the gaps and the complaints that follow. The defense is to calculate spacing from dispersion and ceiling height and to verify coverage against the ambient noise level.

### Amplifier Sized Exactly to the Tap Sum

The installer sums the taps, buys an amplifier of exactly that rating, and declares the system adequate. The mechanism of the trap is that real audio peaks exceed the average level by a wide margin, and an amplifier with no headroom clips on those peaks, producing a distorted, unintelligible page, and sustained clipping can also damage the speakers and the amplifier. The false signal is that the system sounds fine at low background music levels, which never exercise the peaks. The harm is paging that breaks up exactly when it needs to be clear, the moment of a loud or urgent page. The defense is to add headroom of roughly 20 to 30 percent above the tap sum when selecting the amplifier.

### Page Override That Clips the First Word

The installer wires the page override contact and confirms that the music mutes on a page, but never checks the timing. The mechanism of the trap is that the mute and the page audio must be sequenced so the music is fully muted before the page arrives, and if the mute is slow or the page audio is fast, the first word, often the most important, is buried under the tail of the music. The false signal is that the music ducks, which proves the contact works but not that the page is intelligible. The harm is pages where the first word is lost, forcing repetition and eroding confidence in the system. The defense is to test override timing with real pages and to adjust the mute lead time so the first word is clean.

### Consumer Music Source on a Long Unbalanced Run

The installer mounts a consumer streaming player in a rack and runs an unbalanced cable a hundred feet to the amplifier. The mechanism of the trap is that long unbalanced runs act as antennas for electrical noise, inducing hum and buzz into the music, and the consumer output level and impedance are not matched to the commercial amplifier input, reducing level and headroom. The false signal is that the source plays and is audible, which proves the connection but not the quality. The harm is background music with a persistent hum that is noticeable in quiet passages and that makes the system feel cheap. The defense is to use a balanced interface or a local source for long runs and to match the source level and impedance to the amplifier.

### Non-Plenum Cable in a Plenum Return

The installer pulls general-purpose speaker wire through a ceiling used as a return air plenum. The mechanism of the trap is that plenum spaces require plenum-rated cable for fire and smoke reasons, and general-purpose jacketing can spread smoke and flame through the return air path, which is precisely the path that delivers air to the occupied space. The false signal is that the cable fits and works electrically, which proves the install but not the listing. The harm is a code violation that fails inspection and, in a fire, cable that contributes to smoke spread through the building. The defense is to match the cable listing to each space, using plenum-rated cable in plenum returns and riser-rated cable in vertical shafts.

## Self-Check

- Did I tap each speaker for the level required at its location rather than defaulting to maximum, and does the sum of taps per zone fit within the amplifier's rating with headroom?
- Did I calculate speaker spacing from the dispersion pattern and ceiling height so coverage circles overlap, and did I verify the layout against the ambient noise level in each area?
- Did I zone the system by occupancy and use rather than by floor plan, with local volume controls where independent adjustment is needed, and did I plan the zone boundaries before pulling cable?
- Did I size each amplifier to the sum of its zone's taps plus roughly 20 to 30 percent headroom for peaks, and is the amplifier matched to the system voltage (70V or 100V)?
- Did I test the page override timing with real pages to confirm the first word is not clipped, and did I verify the priority scheme when multiple page sources can trigger the system?
- Did I use balanced interfaces or local sources for long music runs, and did I match the source level and impedance to the amplifier's input expectations?
- Did I match the speaker cable listing to each space (plenum, riser, general-purpose), separate it from line voltage, and size the gauge for the run length and the tapped load?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
