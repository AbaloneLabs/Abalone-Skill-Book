---
name: loudness-and-dynamics-targets.md
description: Use when the agent is mastering a track or album, setting final loudness, applying limiting, managing dynamics and crest factor, targeting streaming platform loudness normalization standards, or evaluating whether a master achieves competitive loudness without sacrificing transient impact and musical dynamics.
---

# Loudness and Dynamics Targets

Mastering is the last creative decision before release, and its most visible task — making the music loud enough — is also the most frequently botched. The judgment problem is that loudness is seductive: a louder master sounds "better" in an instant A/B, and the engineer can push the limiter further and further, crushing dynamics until the music is loud but lifeless. The loudness wars taught the industry that over-compression destroys the very impact it tries to create, because transients — the sharp attacks that carry energy — are flattened, and the ear fatigues quickly to a dynamically uniform signal. Compounding the problem, streaming platforms now normalize loudness to targets (around -14 LUFS), meaning a hyper-compressed master is turned down to match a dynamic one, losing its loudness advantage while keeping its dynamic damage. This skill covers the decisions that govern loudness and dynamics in mastering: choosing appropriate loudness targets, preserving transients and crest factor, using limiting without crushing, and aligning with platform normalization.

## Core Rules

### Master to a Loudness Target Appropriate for the Genre and Platform

Loudness is measured in integrated LUFS (loudness units relative to full scale), and the right target depends on genre expectations and the platforms where the music will be heard. Streaming platforms normalize to roughly -14 LUFS (Spotify, Apple Music) or -16 LUFS (YouTube); genres like EDM and modern pop often master louder (-8 to -6 LUFS) to match genre expectations, accepting that streaming will turn them down. The decision is to choose a target deliberately, knowing the tradeoffs: louder targets reduce dynamic range and transient impact; quieter targets preserve dynamics but may sound less competitive in direct comparison. State the target before mastering, and measure the integrated LUFS of the final master to confirm it lands where intended.

### Preserve Crest Factor and Transient Impact

Crest factor — the difference between peak and average level — is what gives music its punch and life. A master with high crest factor has dynamic contrast: loud transients over a controlled average level. Over-limiting reduces crest factor, producing a loud average level with flattened transients that sounds powerful for a few seconds and fatiguing over minutes. The decision is to preserve enough crest factor that transients retain their impact, typically keeping peaks 6 to 10 dB above the average level depending on genre. The criterion is that drums still punch, attacks still speak, and the music breathes dynamically. If transients sound rounded or the master feels flat and fatiguing, the limiting has gone too far.

### Use Limiting Transparently, Not as a Loudness Weapon

A limiter is the final dynamics processor that prevents peaks from exceeding 0 dBFS while allowing the average level to rise. The decision is to use the limiter to catch peaks transparently and achieve the target loudness, not to squeeze every possible decibel. The traps of aggressive limiting are audible: pumping (the limiter breathing in and out), distortion (the waveform squared off at the top), and loss of stereo width (inter-sample peaks crushing the sides). Set the limiter ceiling below 0 dBFS (often -0.3 to -1.0 dB) to avoid inter-sample peaks on playback, use attack and release times that minimize artifacts, and stop pushing when artifacts appear, even if more loudness is theoretically available.

### Account for Streaming Platform Normalization

Streaming platforms measure a track's integrated LUFS and normalize it to their target by turning louder tracks down (or quieter tracks up to a limit). This means a master at -8 LUFS will be turned down 6 dB on a -14 LUFS platform, losing its loudness advantage while retaining its dynamic damage. The decision is to master with normalization in mind: a dynamic master at -14 LUFS will play at full level on streaming and retain its transients, while a crushed master at -8 LUFS will be turned down and sound smaller. Consider providing different masters for different contexts (a dynamic master for streaming, a louder master for club or radio), or master to a level that balances streaming and competitive needs.

### Match Loudness Across Album Tracks for Cohesion

An album's tracks should sit at consistent loudness so that the listener does not reach for the volume knob between songs. The decision is to master all tracks to a consistent integrated LUFS (within a decibel or two) and to match the perceived loudness by ear, not just the meter. Tracks with different arrangements (a quiet ballad versus a loud rocker) may have different integrated LUFS but should feel balanced in sequence. The criterion is that the album flows without jarring level jumps, while preserving appropriate dynamic differences between song types.

### Verify the Master Across Systems and at Low Volume

A master that sounds great on the mastering monitors may reveal problems on other systems: too much low end on small speakers, harshness on earbuds, lack of weight in a car. The decision is to check the master on multiple systems and at low volume, where balance problems are most apparent. At low volume, only the primary elements and the midrange should be clearly audible; if the master sounds thin or harsh quiet, the balance or tonal decisions need adjustment. The criterion is that the master translates its intent across systems, not just that it sounds impressive on the main monitors.

## Common Traps

### Pushing the Limiter Until It Sounds Louder in A/B

The engineer A/Bs the master against the mix, and louder always sounds better in an instant comparison, so the limiter gets pushed further. The false signal is that louder is more impressive. The harm is crushed transients, pumping, distortion, and a master that fatigues the listener within a minute. The mechanism is the psychoacoustic bias that louder equals better in short comparisons. The remedy is to level-match the A/B and to judge dynamics over longer listening, not instant impact.

### Ignoring Streaming Normalization and Over-Mastering

The engineer masters to -7 LUFS to be competitive, not realizing streaming will turn it down to -14. The false signal is that the master is loud and competitive. The harm is a master that loses its loudness advantage on the most common playback platforms while retaining all its dynamic damage, ending up quieter-sounding and worse than a dynamic master would. The mechanism is mastering for legacy contexts (CD, club) without considering streaming. The remedy is to master with platform normalization in mind.

### Crushing Crest Factor for Maximum Loudness

The limiter is pushed until peaks are nearly level with the average, eliminating transient impact. The false signal is a high average loudness reading. The harm is a master that is loud but flat, with drums that no longer punch and a sound that fatigues quickly. The mechanism is optimizing the loudness meter rather than the musical impact. The fix is to preserve crest factor and prioritize transient integrity.

### Introducing Inaudible-Until-Later Distortion

Aggressive limiting introduces distortion that is not obvious on the mastering monitors but becomes apparent on other systems or over time. The false signal is that the master sounds clean in the studio. The harm is a release with audible distortion that undermines the music. The mechanism is distortion masked by the monitoring environment. The remedy is to check for distortion on multiple systems and to back off the limiter when artifacts appear.

### Inconsistent Loudness Across an Album

Each track is mastered to maximize its individual loudness, producing album tracks at different perceived levels. The false signal is that each track is optimized. The harm is an album that jumps in level between songs, requiring the listener to adjust volume and breaking the flow. The mechanism is per-track optimization without album cohesion. The fix is to match loudness across the album for consistent perceived level.

### Setting the Ceiling Too Close to 0 dBFS

The limiter ceiling is set at exactly 0 dBFS, and inter-sample peaks cause distortion on playback systems with lower-quality reconstruction filters. The false signal is that the meter shows no overs. The harm is distortion on consumer playback that was not present in the studio. The mechanism is inter-sample peaks not captured by sample-peak meters. The remedy is to set the ceiling below 0 dBFS and to meter true peaks.

## Self-Check

- Did I choose a loudness target appropriate for the genre and platform, and does the final master measure at that integrated LUFS?
- Did I preserve crest factor so transients retain impact, with peaks typically 6 to 10 dB above the average level?
- Did I use the limiter transparently to catch peaks and reach the target, stopping when artifacts appeared rather than pushing for maximum loudness?
- Did I account for streaming platform normalization, either mastering to a streaming-appropriate level or providing alternate masters for different contexts?
- Did I match loudness across album tracks so the album flows without jarring level jumps, while preserving appropriate dynamic differences?
- Did I verify the master across multiple systems and at low volume, confirming it translates its intent rather than just sounding impressive on the main monitors?
- Did I set the limiter ceiling below 0 dBFS and meter true peaks to avoid inter-sample distortion on consumer playback?
