---
name: signal_routing_and_troubleshooting.md
description: Use when the agent is patching a live sound system, routing signals through a console or stagebox, diagnosing signal loss or noise, building a show file, planning input lists and patch sheets, or deciding how to structure redundant and fail-safe signal paths for a live event.
---

# Signal Routing and Troubleshooting

Signal routing is the invisible backbone of any live event. Every microphone, line input, playback, and output must travel from source to destination through a chain of cables, stageboxes, consoles, and processors, and any single weak link can silence a show. The judgment problem is that routing errors are easy to make, hard to find under pressure, and often misdiagnosed because symptoms (no sound, hum, distortion) can have many causes. The agent must build routing that is logical, documented, redundant where stakes are high, and diagnosable under time pressure — and must troubleshoot by a disciplined halving method rather than random cable-swapping. A well-designed system fails in predictable, recoverable ways; a poorly designed one fails chaotically.

## Core Rules

### Build a Signal Flow Diagram Before Patching Anything

Before a single cable is connected, the entire signal path should exist on paper (or in a show file): every input source, its channel number, its routing through the console, any subgroup or matrix routing, and its final destination (main, subgroup, record, broadcast). This diagram is the source of truth that the patch follows. Patching ad hoc leads to duplicate channels, missing sends, and orphaned inputs that cause problems only discovered during sound check. The diagram also becomes the troubleshooting reference when something goes wrong.

**Decision criteria:** If you cannot point to where every signal enters, how it is processed, and where it exits, the routing is not yet ready to patch. Build the map first.

### Use Consistent Labeling and Numbering End to End

Every cable, stagebox input, console channel, and output should carry the same identifier from source to destination. The microphone on the kick drum is input 1 on the stagebox, channel 1 on the console, and labeled "Kick In" at every point. This consistency allows anyone on the crew to trace a signal in seconds. Inconsistent labeling — where the stagebox says "1" but the console channel is labeled "Kick" with no number — creates confusion that compounds under pressure.

**Decision criteria:** Verify that every label in the chain matches the input list and that channel strips reflect both the source name and the input number. No label should be ambiguous or missing.

### Troubleshoot by Halving, Not by Swapping

When a signal is lost or noisy, the disciplined method is to isolate the problem by dividing the chain. Test at the midpoint: is the signal present at the stagebox output? At the console input? At the channel? Each test halves the possible failure domain. Randomly swapping cables or rebooting the console may appear to work but wastes time and obscures the real cause. The goal is to identify the single failing component, not to stumble onto a fix.

**Decision criteria:** At the first sign of a problem, identify the boundaries of the chain and test at the midpoint. Continue halving until the failing component is isolated. Document the finding.

### Distinguish Noise Types by Their Cause

Hum, buzz, hiss, crackle, and radio interference each have characteristic causes and cures. A 50/60 Hz hum is usually a ground loop or shielding failure. A buzz with harmonics is often a dimmer pack or fluorescent light on the same circuit. Hiss is typically gain structure or a noisy device. Crackle is usually a bad cable, connector, or pot. Radio interference is an RF shielding problem. Treating all noise as "a grounding issue" leads to wrong fixes. Diagnose the noise type by listening and by what makes it change.

**Decision criteria:** Before applying a fix, classify the noise by frequency content, whether it changes with channel gain, and whether it is present with the source disconnected. Match the fix to the diagnosed cause.

### Plan Redundancy and Failure Modes for High-Stakes Signals

For signals whose failure stops the show — lead vocal, click track, playback, or the main console itself — plan the failure mode in advance. This includes redundant microphones, split inputs to two consoles, backup playback machines, spare cables pre-patched, and an analog fallback path. The question is not "will this fail?" but "when this fails, what happens in the next five seconds?" A show with no redundancy is one cable failure from silence.

**Decision criteria:** For every show-critical signal, identify what happens if each component in its chain fails. If the answer is "the show stops," add redundancy or a fallback.

### Maintain Gain Structure Through Every Stage of the Chain

Signal-to-noise ratio and headroom are determined by gain structure across the whole chain, not just at the preamp. A weak signal from a low-output microphone pushed too gently into a noisy stagebox, then made up with high channel gain, will be hissy. An overdriven input that is then attenuated will be distorted. Each stage should operate in its intended operating level (mic, line, instrument) and each gain stage should leave headroom. Mis-matched operating levels between devices are a common and invisible source of noise and distortion.

**Decision criteria:** Trace the operating level and gain at each stage of every critical chain. Confirm that no stage is starved (too low, noisy) or clipped (too high, distorted) and that headroom is preserved for peaks.

## Common Traps

### "It Works, Ship It" Without Documenting the Patch

A routing that was built quickly and happens to pass signal is often left undocumented. When something fails mid-show, no one can trace the path because no one remembers which cable goes where or which console output feeds which amplifier input. The trap mechanism is that functional routing feels finished, so documentation is skipped. The harm is that the first failure becomes a crisis because the system is opaque. The corrective is that the patch sheet and signal flow diagram are part of the deliverable, not optional extras.

### Assuming a Console Reboot Fixes Intermittent Problems

When a channel glitches or a send drops out intermittently, rebooting the console or reloading the show file often makes the symptom disappear — temporarily. The trap is concluding the problem is solved. In reality, the reboot cleared a transient state but the underlying cause (a failing cable, a loose connector, a marginal power supply) remains and will recur, likely during the show. The harm is false confidence and a repeat failure at the worst moment. The corrective is to treat any intermittent failure as a hardware or connection problem until proven otherwise.

### Misdiagnosing Ground Loops as Bad Cables

A hum that appears when two devices are connected is often a ground loop, not a defective cable. Swapping cables does not fix it and wastes time. The trap mechanism is that hum "sounds like a cable problem" and cable-swapping feels productive. The harm is wasted time, a pile of "bad" cables that are actually fine, and an unresolved hum. The corrective is to lift the ground at one end (via a proper ground-lift adapter or transformer isolation, never by defeating safety ground on mains power) and confirm the loop is the cause before replacing hardware.

### Routing Everything Through the Most Complex Path Available

Digital consoles offer extensive routing — matrices, mix buses, control groups, snapshots — and the temptation is to use all of it. But every routing hop is a place for error and a thing that must be remembered under pressure. The trap is that complex routing demonstrates capability but multiplies failure modes. The harm is a show file that no one fully understands, where a single misrouted send takes down a whole section. The corrective is to use the simplest routing that achieves the goal and to document every non-obvious path.

### Ignoring Operating Level Mismatches

Connecting a -10 dBV consumer output to a +4 dBu professional input, or vice versa, produces either a noisy, weak signal or a distorted, hot one. The trap is that the signal "works" at some level, so the mismatch goes unnoticed, but the system is forever noisy or strained. The harm is chronic low-grade problems (hiss, lack of headroom) that are blamed on the wrong components. The corrective is to verify operating levels at every interface between devices and use level-matching (pads, DI boxes, line-level converters) where they differ.

## Self-Check

- Does a complete signal flow diagram exist for the show, and does the actual patch match it?
- Is every cable, stagebox input, console channel, and output labeled consistently with the input list?
- For any signal problem, did I troubleshoot by halving the chain rather than randomly swapping components?
- Can I classify any noise I encountered by type (hum, buzz, hiss, crackle, RF) and match it to the correct cause?
- For every show-critical signal, have I identified its failure mode and provided redundancy or a fallback?
- Have I verified gain structure and operating levels at every stage of every critical chain, with headroom preserved?
- Is the routing documented simply enough that another engineer could take over the show from my notes?
