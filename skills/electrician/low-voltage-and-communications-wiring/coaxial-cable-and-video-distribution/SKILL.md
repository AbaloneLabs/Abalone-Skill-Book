---
name: coaxial-cable-and-video-distribution.md
description: Use when the agent is pulling and terminating coaxial cable, planning a broadband RF video distribution headend, setting signal levels and splitters, or diagnosing snow, ghosting, and tilt across a multi-drop coax plant.
---

# Coaxial Cable and Video Distribution

A coaxial video distribution system carries broadband radio-frequency signals from a headend to many outlets, and its success depends on signal level, flatness across the band, and return-path integrity rather than on simple continuity. The judgment problem is that coax looks like a single sturdy cable that "either works or doesn't," which hides the fact that RF distribution is a budget problem: every splitter, tap, cable length, and connector subtracts signal, and a plant that passes a continuity test can still be too weak, too tilted, or too reflective to deliver a clean picture at the farthest outlet. This skill covers the decisions that determine whether a coax video plant delivers rated signal to every drop or becomes a source of snow, ghosting, intermittent channels, and chronic service calls.

## Core Rules

### Design to the Signal Budget, Not the Number of Outlets

Every component in an RF path has insertion loss: a two-way splitter loses about 3.5 dB, a four-way about 7 dB, an eight-way about 11 dB, and cable loses roughly 1.5 to 6 dB per 100 feet depending on frequency, with loss increasing at higher frequencies. The engineering discipline is to start from the headend output level, subtract every loss along the path to each outlet, and confirm the delivered level lands within the receiver's acceptable window, typically 0 to about +15 dBmV for digital carriers. Outlets at the end of long runs or behind many splits are the ones that fail, and the design must be checked at the worst-case outlet, not the closest one. The defense is to lay out the splitter and tap hierarchy so that the farthest drop still receives adequate level, and to use distribution amplifiers or active taps where passive loss would drop the signal below the window.

### Use the Correct Connector and Crimp It With the Right Tool

Coax connectors are precision RF parts, not plumbing fittings. The connector must match the cable type and shield construction: a compression connector for quad-shield differs from one for tri-shield or RG6 with a braid-and-foil combination, and using the wrong connector leaves the shield poorly terminated, creating a point of impedance mismatch and ingress. The connector must be installed with the correct stripping tool that exposes the dielectric, braid, and center conductor to the exact lengths the connector expects, and then compressed with the matching tool so the O-ring seals and the shield makes full contact. The trap is using a twist-on connector or a mismatched crimp, which produces a connection that holds mechanically but leaks RF in and out.

### Maintain Characteristic Impedance and Avoid Stub Mismatches

Coax for video distribution is 75-ohm characteristic impedance, and every component in the path, connectors, splitters, taps, terminators, must also be 75-ohm so that the RF sees a uniform transmission line. Any discontinuity, a kinked cable, a crushed jacket, a connector with a folded-back braid stub, or an unterminated splitter port, reflects signal back toward the source, and that reflection returns to the receiver as a delayed echo that shows up as a ghost on analog or as a bit error on digital. The defense is to terminate every unused splitter output with a 75-ohm terminator, to avoid sharp bends and crushed cable, and to dress connectors so the shield contact is clean and complete. Unterminated ports are the single most common cause of ghosting and intermittent digital channels.

### Account for Frequency Tilt Across the Band

Coax cable loses more signal at high frequencies than at low frequencies, so a long run that delivers adequate level at 50 MHz may be several decibels down at 750 MHz or 1 GHz. This frequency-dependent loss, called tilt, must be compensated so that all channels arrive at the outlet at roughly the same level. Distribution amplifiers often include a tilt control or equalizer that boosts the high end to pre-compensate for cable loss, and the headend or amplifier output should be set with the high channels higher than the low channels so that, after cable loss, they arrive flat at the outlet. The trap is setting a flat output at the amplifier and discovering that the high channels are weak at the far drops.

### Prevent Ingress and Egress Through Shielding and Grounding

A coax plant is both a receiver and a radiator of RF, and a poorly shielded or grounded plant lets outside signals in (ingress) and lets the distributed signal out (egress). Ingress from nearby transmitters, LTE and cellular base stations, and electrical noise corrupts the digital carriers and causes pixelation and dropouts. Egress can interfere with licensed services and trigger regulatory complaints. The defense is to use quad-shield cable in electrically noisy environments, to bond the coax shield to the building ground at the headend per NEC Article 810 and 820, and to use only properly compressed connectors that seal the shield. A loose connector is an antenna.

### Separate Coax from Power and Avoid Induced Hum

Coax run parallel to power conductors can have AC hum induced onto the shield and into the signal, visible as hum bars rolling through the picture or as a 60 Hz component in the audio. The defense is to maintain separation from line voltage, to cross power at 90 degrees, and to bond the coax ground block to the same grounding electrode system as the power so that ground currents do not flow on the shield between two different ground potentials.

## Common Traps

### Unterminated Splitter Ports Reflecting Signal

The installer wires an eight-way splitter but leaves three outputs unused and capped with nothing. The mechanism of the failure is that an unterminated port reflects the incoming RF back toward the source, and that reflection returns to every other port as a delayed echo that corrupts the digital carriers and produces ghosting on analog channels. The false signal is that the connected outlets show a picture, which proves the path works but not that the reflections are controlled. The harm is intermittent pixelation, dropouts on specific channels, and service calls that vanish when the technician happens to move the splitter. The defense is to install a 75-ohm terminator on every unused output.

### Mismatched Connector on Quad-Shield Cable

The installer uses a connector designed for dual-shield RG6 on a quad-shield cable and forces it on. The mechanism of the failure is that the extra foil and braid layers of quad-shield cannot seat correctly in a dual-shield connector, leaving the shield partially disconnected and creating an impedance bump and an ingress point at every such termination. The false signal is that the connector holds firmly and the center conductor reaches the jack, which proves the mechanical fit but not the RF integrity. The harm is a plant with elevated bit errors and ingress that resists diagnosis because every connector looks installed. The defense is to use the connector specified for the cable's shield construction and the correct compression tool.

### Flat Amplifier Output Ignoring Cable Tilt

The installer sets the distribution amplifier output to the same level on every channel and declares the headend balanced. The mechanism of the failure is that the cable loses more at high frequencies, so by the time the signal reaches a far drop, the high channels are several decibels below the low channels, falling below the receiver threshold while the low channels remain fine. The false signal is that all channels read correctly on the meter at the amplifier output, which proves the source but not the delivered level. The harm is customers reporting that specific upper-band channels pixelate or drop while lower channels are perfect. The defense is to pre-tilt the amplifier output so the high channels are higher, compensating for cable loss, and to verify flatness at the farthest drop.

### Long Run with Too Many Passive Splits

The installer chains splitters to reach a distant outlet without calculating cumulative loss, and the far outlet ends up 20 dB below the headend level. The mechanism of the failure is that each passive split subtracts signal, and the cumulative loss pushes the delivered level below the receiver window, so the digital carrier is too weak to decode reliably. The false signal is that the cable tests for continuity and the outlet has a connector, which proves the wiring but not the level. The harm is an outlet that shows no signal or intermittent signal, requiring a repull or an active tap. The defense is to design the splitter hierarchy against the signal budget and to add an amplifier or active tap where passive loss is excessive.

### Coax Grounded to a Different Electrode Than the Power

The installer bonds the coax ground block to a water pipe that is not electrically continuous with the power grounding electrode system. The mechanism of the failure is that the two ground points can sit at different potentials, driving current onto the coax shield between them, which induces hum into the signal and creates a shock hazard during a surge event. The false signal is that the ground block is bonded to something metallic, which looks compliant but is not bonded to the same electrode system as required. The harm is hum bars in the picture, accelerated connector corrosion, and a real safety hazard during lightning or fault events. The defense is to bond the coax ground to the building's intersystem bonding termination or the same grounding electrode system as the service.

## Self-Check

- Did I design the splitter and tap hierarchy against the signal budget and verify that the worst-case farthest outlet receives level within the receiver window?
- Did I use connectors matched to the cable's shield construction (dual, tri, or quad-shield) and compress each one with the correct tool?
- Did I install a 75-ohm terminator on every unused splitter and tap output so that no port reflects signal back into the plant?
- Did I pre-tilt the amplifier output to compensate for cable frequency loss, and did I verify flatness at the farthest drop rather than only at the headend?
- Did I use quad-shield cable in electrically noisy environments and bond the coax ground block to the same grounding electrode system as the power service?
- Did I maintain separation from line voltage and cross power only at 90 degrees to prevent induced hum on the shield?
- Did I sweep-test or level-check the plant at the highest frequency in use, not just at a single low channel?
- Is the signal-level design and the as-built record documented clearly enough that another technician could trace any outlet back to its source and verify the budget?
