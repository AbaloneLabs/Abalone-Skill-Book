---
name: conference-room-av-and-video-conferencing.md
description: Use when the agent is installing conference room AV, video conferencing codecs, ceiling microphones, and display systems, configuring DSP acoustic echo cancellation, ceiling mic placement, camera tracking, display scaling, and HDCP signal distribution.
---

# Conference Room AV and Video Conferencing

A conference room AV system is judged by the far-end experience, because the people on the other side of the call cannot lean in or ask you to repeat, and a room that sounds fine to the people in it can be an echo-chamber of garbled audio and frozen video to the remote participants. The judgment problem is that an installer who hangs a display, drops a speakerphone on the table, and runs an HDMI cable will produce a room that looks finished and fails the moment someone joins remotely, with echo from unmanaged acoustics, a camera that frames the back wall, a microphone that hears the HVAC, and a display that goes dark the instant protected content appears. The DSP echo cancellation, the microphone placement, the camera and display scaling, and the HDCP distribution are the decisions that determine whether the room actually works for collaboration. This skill covers the audio, video, and signal decisions that determine whether a conference room serves the remote participant.

## Core Rules

### Configure DSP Acoustic Echo Cancellation for the Room

Acoustic echo cancellation (AEC) is the DSP function that removes the far-end audio picked up by the room's microphones before it is sent back, and without it the far-end participants hear their own voices echoed back a fraction of a second later, making the call unusable. AEC is provided by the codec, the DSP, or the microphone itself, and it must be configured for the room's acoustics, with the reference signal correctly routed and the convergence tuned. AEC has limits: it cannot remove echo if the microphone hears the speaker so loudly that the echo is nonlinear, and it cannot remove echo from a path it does not reference, such as a laptop speaker playing the far-end audio into a room microphone. The room's reverberation and the speaker level affect AEC performance, and a reverberant room with loud speakers defeats even good AEC. The AEC must be tested with a real far-end participant, not just by listening in the room.

The trap is assuming the codec handles echo automatically. The defense is to configure AEC for the room, route the reference signal correctly, control reverberation and speaker level, and test with a real far-end participant.

### Place Ceiling and Table Microphones for Voice Pickup, Not Noise

Microphone placement determines the ratio of the participant's voice to the room's noise and reverberation, and the closer the microphone to the talker, the better the ratio. Ceiling microphones must be placed directly above the talker's position, within the pickup pattern, and far enough from the HVAC diffusers and the projectors that they do not pick up mechanical noise. Table microphones should be within an arm's reach of the talkers, with one microphone per two or three seats, because a microphone too far from the talker picks up more room than voice. Beamforming ceiling arrays can steer pickup toward the talker and reject noise, but they still require correct placement and tuning. The microphone must not hear the room's speakers, which means placement outside the speaker coverage or reliance on AEC, and the cabling must avoid noise pickup from power and lighting.

The trap is placing microphones where they look symmetrical. The defense is to place microphones close to the talkers, away from HVAC and speakers, and to verify pickup with a far-end listener.

### Set Camera Tracking and Framing for the Far-End View

The camera must frame the participants so the far-end sees the people, not the back wall or the empty table, and in larger rooms this requires camera tracking or preset switching that follows the active speaker. A fixed camera framed wide to capture everyone makes each participant a tiny figure, while a camera that tracks the active speaker gives the far-end a usable view. Tracking systems use microphone-based localization, dedicated tracking cameras, or manual presets triggered by the control system, and each has tuning and reliability considerations. The camera must be positioned at eye level relative to the display so that the participants look at the far-end naturally, and the framing must be verified from the far-end perspective, not by looking at the preview in the room. Auto-framing that hunts between speakers is distracting and must be tuned to hold steady.

The trap is framing the camera wide and leaving it fixed. The defense is to use tracking or presets to frame the active speaker, position the camera at eye level, and verify the view from the far end.

### Scale and Distribute Displays With Correct Resolution and HDCP

Modern conference rooms distribute video over HDMI, HDBaseT, or IP, and the signal must arrive at each display at the correct resolution and with the content protection intact. Display scaling handles mismatches between the source resolution and the display, such as a 4K source on a 1080p display, but scaling can introduce latency and artifacts, and the system should be designed to minimize scaling by matching source and display resolutions. HDCP, the content protection on HDMI, must be negotiated end to end, and a single non-compliant device in the chain, an older display, a long passive cable, or an unlisted splitter, causes the signal to drop to black, often intermittently. Distribution amplifiers and matrix switches must be HDCP-compliant and must support the required resolution and distance. Long runs require active extension over HDBaseT or fiber, not passive HDMI cables that fail beyond a few meters.

The trap is assuming HDMI just works over any cable. The defense is to match source and display resolutions, use HDCP-compliant distribution, and extend long runs with HDBaseT or fiber.

### Manage Room Acoustics and Noise for Intelligibility

The room itself is part of the audio path, and a hard-surfaced, reverberant room with a noisy HVAC defeats the best DSP and microphones, because the far-end hears the room's reflections and noise along with the voice. Acoustic treatment on reflective walls and ceilings reduces reverberation and improves intelligibility, and the HVAC and lighting must be quiet enough not to dominate the microphone. The room should be evaluated for reverberation time and background noise before the AV is specified, because a bad room limits every system installed in it. Speaker placement matters too: ceiling speakers distributed across the room provide even coverage without the level that excites reverberation, while a single loud speaker at the front overdrives the near seats and excites the room.

The trap is installing AV in an untreated room and expecting the DSP to fix it. The defense is to evaluate and treat the room acoustics, quiet the HVAC, and use distributed speakers for even coverage.

### Integrate Control, Signal Routing, and Cable Management

A conference room that requires six remotes and a tangle of table cables will not be used correctly, so the control system, the signal routing, and the cable management are part of the installation, not afterthoughts. A single control panel or touch interface should start the call, select the source, and control the volume, with one-button start where possible. Signal routing through a matrix switch lets any source go to any display, including the far-end, and the routing must be preconfigured for common scenarios. Cable management at the table, with retractable cables or a wireless presentation device, avoids the clutter that leads users to plug into the wrong input. The control system must be programmed for how the room is actually used, and the users must be trained, because a powerful system that no one understands is abandoned for a laptop.

The trap is leaving control to the factory remote. The defense is to integrate a single control interface, preconfigure routing, manage table cabling, and train the users.

## Common Traps

### AEC Assumed Automatic, Far-End Hears Echo

The installer enables AEC on the codec and assumes echo is handled, without configuring the reference or testing with a remote participant. The mechanism of the trap is that AEC must be tuned to the room and correctly referenced, and a loud speaker in a reverberant room produces nonlinear echo that AEC cannot fully remove, so the far-end continues to hear their own voices echoed back, making the call frustrating or unusable. The false signal is that the room sounds fine to the people in it, which proves local audio but not far-end audio. The harm is a call that drives remote participants away. The defense is to configure AEC, control the room, and test with a real far-end participant.

### Ceiling Mic Placed Symmetrically, Not Over the Talker

The installer places a ceiling microphone in the geometric center of the table for symmetry, far from the talkers and near the HVAC diffuser. The mechanism of the trap is that a microphone far from the talker picks up more room noise and reverberation than voice, and one near a diffuser picks up the air handler, so the far-end hears noise and echo instead of speech, regardless of the microphone's quality. The false signal is that the microphone picks up sound during a local test, which proves sensitivity but not the voice-to-noise ratio. The harm is poor far-end intelligibility. The defense is to place microphones over the talkers, away from HVAC and speakers.

### Camera Framed Wide and Fixed on the Back Wall

The installer mounts a fixed camera framed to capture the whole table and leaves it, assuming a wide view is inclusive. The mechanism of the trap is that a wide fixed frame makes each participant a small figure on the far-end, and without tracking or presets the far-end never sees the active speaker clearly, so the video adds little to the collaboration and the remote participants feel disconnected. The false signal is that the camera shows the whole room on the preview, which proves coverage but not a usable view. The harm is a video feed that no one uses. The defense is to use tracking or presets to frame the active speaker and to verify from the far end.

### HDMI Over a Long Passive Cable Drops to Black

The installer runs a long passive HDMI cable from the table to the display to save the cost of an extender. The mechanism of the trap is that HDMI degrades rapidly with distance, and a long passive cable fails to negotiate HDCP or the required resolution, causing the display to drop to black, often intermittently and especially with protected content, and the failure looks like a flaky display rather than a cable problem. The false signal is that the picture appears during a short test, which proves the cable conducts but not that it is reliable. The harm is intermittent black screens during meetings. The defense is to extend long runs with HDBaseT or fiber and to use HDCP-compliant distribution.

### AV in an Untreated Reverberant Room

The installer mounts speakers and microphones in a hard-surfaced glass and concrete room and relies on the DSP to handle the acoustics. The mechanism of the trap is that the room's reverberation and reflections dominate the microphone, and no AEC or microphone can fully remove the smearing and noise, so the far-end hears a garbled, echo-laden signal regardless of the equipment quality. The false signal is that the equipment is high-end, which proves the gear but not the room. The harm is poor intelligibility that the equipment cannot fix. The defense is to treat the room acoustics, quiet the HVAC, and use distributed speakers.

### Six Remotes and No Integrated Control

The installer leaves each device on its factory remote and hands the user a pile of controls. The mechanism of the trap is that a room requiring multiple remotes to start a call is too complex to use reliably, so users fall back to a laptop and a desk speakerphone, abandoning the installed system, and the investment in AV is wasted. The false signal is that each device works with its own remote, which proves function but not usability. The harm is an unused system. The defense is to integrate a single control interface, preconfigure routing, and train the users.

## Self-Check

- Did I configure acoustic echo cancellation for the room, route the reference signal correctly, control reverberation and speaker level, and test with a real far-end participant?
- Did I place ceiling and table microphones close to the talkers, away from HVAC diffusers and speakers, and verify the voice-to-noise ratio with a far-end listener?
- Did I set camera tracking or presets to frame the active speaker, position the camera at eye level relative to the display, and verify the view from the far end?
- Did I match source and display resolutions to minimize scaling, use HDCP-compliant distribution, and extend long runs with HDBaseT or fiber rather than passive HDMI?
- Did I evaluate and treat the room acoustics, quiet the HVAC and lighting, and use distributed speakers for even coverage without exciting reverberation?
- Did I integrate a single control interface for one-button start, preconfigure signal routing for common scenarios, manage table cabling, and train the users?
- Did I document the DSP settings, camera presets, routing, and control programming for ongoing support and for user training?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
