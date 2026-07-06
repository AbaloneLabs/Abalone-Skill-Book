---
name: sound-system-setup-and-acoustic-considerations.md
description: Use when the agent is setting up a sound reinforcement or paging system, matching impedance between low-Z and 70V gear, preventing feedback through microphone and speaker placement, applying equalization to tame room modes, or tuning the system with pink noise against reverberation and reflective surfaces.
---

# Sound System Setup and Acoustic Considerations

A sound system is not finished when the cables are pulled and the amplifier powers up; it is finished when it delivers intelligible, natural sound at the intended level without feedback, and that outcome depends as much on the room as on the gear. The judgment problem is that sound reinforcement is the one low-voltage discipline where the environment is part of the circuit, and an installer who treats the room as a neutral container will chase problems, feedback, boominess, dead spots, that are caused by reflections and resonances the equipment cannot overcome. An equalizer cannot fix a microphone placed in a monitor wedge, a 70V transformer cannot fix a room with a three-second reverb, and more power cannot fix a gain-before-feedback limit set by geometry. This skill covers the impedance, feedback, equalization, and acoustic decisions that determine whether a sound system sounds right.

## Core Rules

### Match Impedance Correctly Between Low-Z and Constant-Voltage Gear

Sound systems use two broad impedance regimes: low-impedance (low-Z, typically 4 to 8 ohms) for direct-coupled speakers and microphones, and constant-voltage (70V or 100V) for distributed systems with transformer-coupled speakers. Mixing the two without the correct interface damages equipment and degrades performance. A low-Z output driving a 70V line collapses the output stage or produces no usable level; a 70V amplifier driving a low-Z speaker overloads the amplifier and risks failure. Microphone impedance must match the input, with low-Z microphones on balanced XLR inputs and the correct pad or gain structure. The impedance match is the foundation of the signal path, and every other adjustment depends on it being correct.

The trap is assuming that if it plugs in it will work. The defense is to verify the impedance regime of every output and input, to use transformers or matching interfaces where regimes meet, and to confirm the load impedance is within the amplifier's rated range.

### Prevent Feedback Through Placement and Gain-Before-Feedback

Feedback is the howl that occurs when a microphone hears the speaker that is amplifying it, creating a loop, and the cure is primarily geometric, not electronic. The maximum gain before feedback is set by the distance and angular relationship between the microphone and the speaker, by the microphone's polar pattern, and by the room's reflections. A microphone behind the main speaker cluster, or a floor monitor aimed at the back of a cardioid microphone, will feedback long before reaching useful level. Directional microphones and careful speaker placement extend the gain-before-feedback margin, and electronic feedback eliminators can suppress a narrow ring but cannot rescue a fundamentally bad geometry. The placement decision is made before any equalization.

The trap is reaching for the equalizer first. The defense is to place microphones behind and off-axis from the speakers, to choose directional microphones for live pickup, and to maximize the geometric gain-before-feedback margin before applying any processing.

### Use Equalization to Tame the Room, Not to Reshape the Source

Equalization corrects frequency-response problems caused by the room and the speaker interaction, not problems in the source material. The legitimate uses of EQ are to cut the frequencies that the room reinforces into boominess or feedback, typically the room modes that build up in the low midrange, and to smooth the overall response so that no band dominates. Boosting frequencies to add brightness or bass is generally a sign that the wrong speaker or the wrong placement was chosen, and broad boosts reduce headroom and invite feedback. A graphic EQ provides fixed frequency bands for general shaping; a parametric EQ allows targeting specific narrow resonances. The goal is a flat, natural response, not an enhanced one.

The trap is boosting frequencies to make it sound good. The defense is to use EQ primarily to cut problem frequencies, to target room modes and feedback rings with narrow cuts, and to aim for a natural, uncolored response.

### Account for Reverberation Time and Reflective Surfaces

Reverberation time (RT60), the time for sound to decay by 60 dB, is the single most important acoustic property of a room for sound reinforcement, and it is fixed by the room's geometry and surfaces, not by the equipment. A highly reverberant space, a gymnasium, a stone lobby, a hard-floored corridor, smears direct sound with reflected sound, reducing intelligibility and limiting gain-before-feedback, because the reflections carry the speaker output back to the microphone. Absorptive surfaces, carpet, acoustic tile, drapes, reduce reverberation and improve clarity. Flutter echoes, the rapid repeat between two parallel hard walls, create a metallic ringing that no EQ can remove. The sound system design must work with the room's acoustic character, using more, lower-power speakers close to the listeners in reverberant spaces.

The trap is expecting the equipment to overcome a bad room. The defense is to measure or estimate RT60, to use distributed low-power speakers close to listeners in reverberant spaces, and to recommend acoustic treatment where reverberation or flutter exceeds the system's ability to compensate.

### Place Microphones and Speakers for the Source and the Audience

Microphone placement relative to the source determines the ratio of direct to ambient sound, and the closer the microphone to the source, the more direct sound and the less room coloration and feedback risk. Speakers must be placed to cover the audience without exciting the room unnecessarily, which often means aiming the main cluster at the audience and away from reflective walls and ceilings. Delay speakers, used to cover areas beyond the main cluster's reach, must be time-aligned to the main cluster so that the sound from both arrives coherently at the listener. The geometry of source, microphone, speaker, audience, and room surfaces is the core of the design, and every electronic adjustment is secondary to it.

The trap is placing gear where it is convenient to mount. The defense is to place microphones close to the source, to aim speakers at the audience and away from reflective surfaces, and to time-align any delay speakers to the mains.

### Tune and Balance the System With a Measured Signal

A sound system is tuned, not just switched on, and tuning uses a measured signal, typically pink noise, and a measurement microphone to reveal the frequency response and the time behavior of the system in the room. Pink noise contains equal energy per octave and exposes the system's response across the band, and a real-time analyzer or transfer-function measurement shows the peaks and dips that need correction. The tuning process sets the gain structure from source to amplifier, applies EQ to flatten the response, sets limiter thresholds to protect the speakers, and verifies coverage by measuring at multiple positions. A system tuned by ear alone reflects the listener's preference and the room's momentary condition, not the system's actual behavior.

The trap is tuning by ear and calling it done. The defense is to tune with pink noise and a measurement microphone, to set gain structure and limiters deliberately, and to verify coverage at multiple listener positions.

### Set Gain Structure to Maximize Headroom and Minimize Noise

Gain structure is the staging of signal level from the source through each device to the amplifier, and correct gain structure maximizes the headroom, the margin before clipping, while minimizing the noise floor. Each device in the chain has an optimal operating level, and setting the input gain too low raises the noise floor when the fader is brought up, while setting it too high clips the next stage. The structure is set by adjusting each gain stage so that the signal passes through at a level that uses the available headroom without clipping, typically using a unity reference tone. A poorly structured system either hisses at idle or distorts on peaks, and both are gain structure problems, not equipment problems.

The trap is setting all the gains to maximum or to noon. The defense is to stage each gain for unity throughput on a reference tone, to confirm headroom on the loudest expected signal, and to verify the noise floor at idle.

## Common Traps

### Mixing Low-Z and 70V Gear Without an Interface

The installer connects a low-Z output directly to a 70V speaker line, or drives a low-Z speaker from a 70V amplifier, because the connectors fit. The mechanism of the trap is that the two impedance regimes are incompatible by design, and direct connection either collapses the output stage, produces no usable level, or overloads the amplifier, with risk of failure. The false signal is that some sound comes out, which proves a connection but not a correct match. The harm is distorted, low-level audio, an overloaded or failing amplifier, and a system that never reaches its designed performance. The defense is to verify the impedance regime of every interface and to use transformers or matching networks where regimes meet.

### Reaching for the EQ Before Fixing the Geometry

A microphone feeds back, and the installer immediately cuts frequencies on the equalizer until the howl stops. The mechanism of the trap is that feedback is set primarily by the geometry between microphone and speaker, and cutting frequencies reduces the gain-before-feedback margin at the cost of response flatness, so the system ends up quiet and unnatural and still prone to feedback at the next frequency. The false signal is that the howl stops after the cuts, which proves the EQ worked but not that the geometry is sound. The harm is a system that is hard to use, low in gain, and colored in response, because the root cause was never addressed. The defense is to fix the microphone and speaker geometry first and to use EQ only for residual narrow resonances.

### Boosting Frequencies Instead of Choosing the Right Speaker

The system lacks bass or brightness, so the installer applies broad EQ boosts to compensate. The mechanism of the trap is that a frequency deficit is usually a speaker or placement problem, and boosting the EQ consumes headroom, increases distortion, and invites feedback at the boosted frequencies, without actually delivering the performance a suitable speaker would. The false signal is that the boosted frequency now sounds louder, which proves the EQ works but masks the underlying mismatch. The harm is a system that runs out of headroom, distorts on peaks, and still does not sound right. The defense is to choose the speaker and placement for the required response and to use EQ for fine correction, not for fundamental reshaping.

### Expecting Equipment to Overcome a Reverberant Room

The installer hangs a few loud speakers in a highly reverberant gymnasium and expects clarity. The mechanism of the trap is that reverberation smears the direct sound with reflected energy, and no amount of power or EQ can separate the direct from the reflected at the listener's ear, so the result is a loud, boomy wash with poor intelligibility. The false signal is that the system is loud, which proves power but not clarity. The harm is announcements and music that are unintelligible in exactly the space where clarity matters, a sports or assembly hall. The defense is to use many low-power speakers close to the listeners, to reduce the path length that excites the reverberant field, and to recommend acoustic treatment where RT60 is excessive.

### Tuning by Ear Without a Measurement Signal

The installer plays a favorite track, adjusts the EQ until it sounds good from the console position, and declares the system tuned. The mechanism of the trap is that ear tuning reflects the listener's taste, the track's character, and the response at one position, and it misses the peaks, dips, and time problems that a measurement would reveal, leaving the system uneven across the audience and prone to feedback at untested frequencies. The false signal is that it sounds good at the console, which proves a pleasant listen but not a correct tune. The harm is a system that sounds different, and often worse, everywhere except the console, and that surprises the user with feedback or coloration. The defense is to tune with pink noise and a measurement microphone and to verify at multiple positions.

### Gain Structure Set to Maximum or to Noon

The installer sets every gain knob to maximum or to a visual noon and balances the system with the faders. The mechanism of the trap is that maximum input gain clips the next stage on peaks, while noon gains may be far from unity and either bury the signal in noise or leave no headroom, and fader-only balancing hides the gain structure problem until the system hisses or distorts. The false signal is that the system passes audio at a reasonable level, which proves throughput but not correct staging. The harm is a system with a high noise floor or with premature clipping, neither of which is an equipment fault. The defense is to stage each gain for unity on a reference tone, to confirm headroom, and to verify the idle noise floor.

## Self-Check

- Did I verify the impedance regime of every output and input, use transformers or matching interfaces where low-Z and 70V gear meet, and confirm the load is within the amplifier's rated range?
- Did I place microphones behind and off-axis from the speakers, choose directional microphones for live pickup, and maximize the geometric gain-before-feedback margin before applying any processing?
- Did I use equalization primarily to cut problem frequencies and room modes rather than to boost, and did I aim for a natural, uncolored response?
- Did I measure or estimate the room's RT60, use distributed low-power speakers close to listeners in reverberant spaces, and recommend acoustic treatment where reverberation or flutter exceeds the system's ability to compensate?
- Did I place microphones close to the source, aim speakers at the audience and away from reflective surfaces, and time-align any delay speakers to the main cluster?
- Did I tune the system with pink noise and a measurement microphone, set gain structure and limiters deliberately, and verify coverage at multiple listener positions?
- Did I stage each gain for unity throughput on a reference tone, confirm headroom on the loudest expected signal, and verify the idle noise floor?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
