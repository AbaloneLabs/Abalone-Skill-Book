---
name: system_tuning_and_room_correction.md
description: Use when the agent is setting up a PA system, tuning a room, applying EQ to a sound-reinforcement system, diagnosing feedback or coverage problems, choosing measurement microphone positions, or deciding how far to correct room acoustics electronically before recommending physical or placement changes.
---

# System Tuning and Room Correction

System tuning is the process of aligning a loudspeaker system to a room so that the audience hears something close to what the engineer intends, with even coverage, acceptable frequency response, and stable phase behavior. The judgment problem is that rooms are physical systems that cannot be fully fixed with EQ, and every correction creates side effects. An agent who over-corrects with DSP can make a system sound worse than it did untouched, introduce phase distortion that destroys intelligibility, or create a mix that only works at one measurement point. The agent must distinguish problems caused by the loudspeakers, problems caused by placement, problems caused by the room, and problems that are actually source material — and correct each at the right layer.

## Core Rules

### Diagnose the Source Layer Before Reaching for DSP

Before applying any system EQ, verify that the problem is actually an acoustic or electronic one and not a source problem. A harsh-sounding system is often a harsh source mix, a bad microphone choice, or a resonant drum head. Walk the room, listen at multiple positions, and compare the same source through different loudspeakers before concluding the PA needs correction. Applying broad EQ to fix a single resonant source will damage every other source routed through the system.

**Decision criteria:** If the problem follows the source (a specific channel, instrument, or player) rather than the system, fix it at the channel. If it follows the loudspeaker or the room, it is a system problem.

### Correct Geometry and Placement Before Equalization

The strongest, cleanest corrections are physical. Loudspeaker splay angle, array curvature, subwoofer placement and coupling, delay-tower positioning, and physical time alignment produce gains that no plugin can replicate. Electronic correction should refine an already-well-placed system, not rescue a badly placed one. A comb-filtering problem from two overlapping loudspeakers cannot be EQ'd away; it must be solved by re-aiming or muting one of them.

**Decision criteria:** For every measured anomaly, ask first whether repositioning, re-aiming, delaying, or physically decoupling would solve it. Reserve EQ for problems that remain after geometry is optimized.

### Use Measurement as a Guide, Not a Verdict

Dual-channel FFT analyzers (Smaart, Systune, REW) show transfer function magnitude, phase, coherence, and impulse response. These are powerful but easily misread. A dip in the magnitude response may be a real notch, a cancellation from a reflection, or an artifact of poor microphone placement or low coherence. Phase response and coherence must be read alongside magnitude before drawing conclusions. A single measurement position is not representative of the audience area.

**Decision criteria:** Take multiple measurements across the coverage area. Require coherence to be high before trusting a magnitude trace. Cross-check measurement claims by ear with known reference material.

### Apply Minimum-Phase Corrections to Minimum-Phase Problems

Parametric EQ works correctly only on minimum-phase problems — resonances, driver responses, and crossover artifacts where phase and magnitude are linked. It does not correctly fix non-minimum-phase problems such as reflections, comb filtering from multiple sources, and room modes that vary with position. Applying narrow notch filters to fix a comb-filter dip will create a new peak elsewhere and damage transient response.

**Decision criteria:** If the anomaly moves when you move the measurement microphone, it is likely spatial (reflection or cancellation) and should not be fixed with static EQ. If it is stable across positions, minimum-phase EQ is appropriate.

### Time-Align Before Frequency-Align

Phase alignment between subsystems (subs to tops, main to delay, flown to ground) must be established before tonal EQ. A system that is not time-aligned will show false frequency-response anomalies and will lack impact and clarity regardless of EQ. Use impulse response measurements to find acoustic origin points, align the arrivals at the crossover region, and verify with both measurement and listening.

**Decision criteria:** Confirm time alignment at the design axis before applying any tonal correction. Re-check alignment after any major change to crossover or delay settings.

### Preserve Headroom and Dynamics in Processing

Modern DSP processors offer massive EQ and compression capability, but every filter introduces phase shift and every compressor alters transient response. System tuning should use as few filters as possible to achieve the target response. Heavy compression or limiting in system processing reduces the engineer's ability to mix dynamically and can make the system feel lifeless or fatiguing.

**Decision criteria:** Prefer broadband gentle corrections over many narrow cuts. Use system limiting only for protection, not as a loudness tool. Document every filter so it can be revisited.

## Common Traps

### Smoothing the Measurement Trace Until It Looks Flat

A perfectly flat magnitude response on an analyzer is not a goal and is often not desirable. The human auditory system expects a gently sloping response in a room, and a flat trace achieved through dozens of narrow filters will sound phasey, harsh, and unnatural. The trap is treating the analyzer's display as the deliverable rather than the listening experience. Engineers chase visual flatness, stack filters, and end with a system that measures "well" at one point and sounds worse everywhere else. The false signal is a tidy-looking curve; the harm is lost musicality and inconsistent coverage.

### Correcting a Reflection With a Static EQ Notch

When a measurement shows a deep notch in the low end, the instinct is to boost that frequency with EQ. But if the notch is caused by a floor or wall reflection creating cancellation at the measurement point, the boost will have no effect at that point (you cannot fill a cancellation with more of the same energy) and will create a peak at every other position. The trap mechanism is that the analyzer shows a problem and EQ seems like the tool, but the underlying physics is spatial interference. The harm is a system that booms in half the room and is thin in the other half.

### Tuning to a Single Sweet Spot

Taking all measurements from the FOH position and optimizing for that one location produces a system that sounds excellent to the engineer and poor to the audience. Rooms vary dramatically across the seating area, and a tuning that maximizes one point often sacrifices coverage uniformity. The false signal is a great-sounding mix position; the harm is complaints from the sides, back, and balcony. The corrective is to measure across the audience area and optimize for uniformity, accepting that the mix position may be slightly less perfect.

### Over-Reliance on Auto-EQ and Automated Correction Systems

Automated room correction systems can produce a starting point, but they optimize for whatever objective they were given, which is usually flatness at the measurement point. They cannot know the musical intent, the loudspeaker's intended voicing, or the coverage uniformity tradeoffs. The trap is trusting the green "optimized" light without auditing the result. The harm is a system that has been flattened in a way that removes the character of good loudspeakers and introduces audible artifacts from aggressive filtering.

### Ignoring Phase Response

Magnitude-only tuning leads to systems that look right and sound wrong. Two subsystems that are in phase at some frequencies and out of phase at others will produce uneven summation that no amount of magnitude EQ can fix. The trap is that magnitude is easy to see and phase is harder to interpret, so engineers ignore it. The harm is a lack of punch, inconsistent crossover behavior, and a system that never quite "hits" despite a good-looking frequency curve.

## Self-Check

- Did I walk the room and listen at multiple positions before applying any correction, or did I tune only from one seat?
- Have I verified that the problem I am correcting is a system/room problem and not a source, channel, or microphone problem?
- Did I address loudspeaker placement, splay, and time alignment before reaching for parametric EQ?
- For each EQ filter I applied, can I state whether the problem is minimum-phase (stable across positions) or spatial (varies with position)?
- Did I check coherence and phase alongside magnitude on every measurement before trusting it?
- Have I confirmed coverage uniformity across the audience area, not just response at the mix position?
- Did I use the minimum number of filters and avoid aggressive narrow corrections that could introduce audible artifacts?
