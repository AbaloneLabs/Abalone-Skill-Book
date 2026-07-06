---
name: timing_and_reading_speed.md
description: Use when the agent is creating or editing subtitles and must respect reading speed characters-per-second and duration limits, time on screen cue in and out points and shot changes, or balance subtitle timing against viewer comprehension accessibility and the rhythm of speech and image.
---

# Timing And Reading Speed

Subtitles are a time-and-space art, not just a translation delivered in a video container. Every subtitle is bounded by how long it can stay on screen, how fast a viewer can read it, and where it sits relative to speech and image. A subtitle that is perfectly translated but appears too briefly, runs too long for its characters, or pops in and out against the cut rhythm will be unreadable, annoying, or invisible, and the translation effort is wasted. The difficulty is that these constraints are mathematical and physical, governed by characters per second, minimum and maximum durations, and shot boundaries, yet they must be satisfied while preserving meaning, tone, and the viewer's immersion. Agents miss the timing discipline because subtitle text looks like ordinary short translation, and tools may auto-generate timecodes that look plausible. But plausible timecodes are not correct ones, and a subtitle that violates reading-speed limits silently fails its audience, especially slow readers, children, and accessibility-dependent viewers. The harm this prevents is unreadable subtitles: localized video where viewers cannot finish reading, lose the thread, or abandon the content, all because timing was treated as an afterthought rather than the substrate of the work.

Use this skill when creating, translating, or editing subtitles for film, television, streaming, social video, or e-learning, wherever reading speed, duration, and cue timing govern whether the text can actually be read. The goal is subtitles that fit the time available, respect the viewer's reading capacity, and align with the audiovisual rhythm.

## Core Rules

### Honor Reading Speed Limits For The Target Language

Reading speed is the primary physical constraint on subtitles, usually expressed in characters per second or words per minute, and it differs by language and audience. Every subtitle must fit within the limit or it cannot be read.

Determine the reading-speed standard in force, which varies by client, platform, and target language. CJK languages read by character and have different limits than alphabetic languages read by word. Children's content, accessibility content, and SDH carry stricter limits because the audience reads more slowly. Calculate the available time from the cue's in and out points and confirm the character count fits. When the source speech is fast and dense, the literal translation often exceeds the limit, which forces condensation, not a timing violation. Never solve a reading-speed problem by simply extending the cue beyond the speech, because outstaying subtitles desynchronize from audio and confuse the viewer. The text must yield to the time, not the time to the text.

### Respect Minimum And Maximum Duration Bounds

Beyond reading speed, subtitles have absolute duration floors and ceilings. A cue too short flashes past unreadably; a cue too long lingers after the speech ends and feels broken.

Minimum durations, often around one to one and a half seconds, ensure the eye can register the text before it disappears. Maximum durations, often around six to seven seconds, prevent a single subtitle from overstaying, because viewers assume a long-lingering subtitle is a new thought and re-read it, breaking flow. When a very short utterance produces a cue below minimum duration, you may extend slightly within tolerance, but not so far that it detaches from the speech. When a long utterance would exceed maximum duration, split it across cues. Apply these bounds consistently, because erratic duration is more jarring than any single mistimed line.

### Align Cues To Speech And Shot Boundaries

Cue in and out points should align with the rhythm of speech and, where possible, with shot changes, because subtitles that cross cuts or fight the audio rhythm pull the viewer out of the experience.

Cue in when the speaker begins or a fraction of a second before, so the text appears with the speech rather than lagging. Cue out when the speech ends or shortly after, not early, which truncates the read, and not so late that the text hangs in silence. Where a shot change falls within a subtitle, consider splitting the cue at the cut, because text that jumps across a cut can appear to reset. Avoid cueing subtitles over moments meant to be read silently, and avoid leaving a subtitle on screen during a reaction shot where the viewer expects the speaker's face. Timing is part of the craft of subtitling, not a mechanical byproduct, and good cue placement is invisible while bad placement is constantly noticed.

### Manage Density And Condense Within The Time Available

When speech is fast or dense, the literal translation often cannot fit the reading-speed budget, and the subtitle must be condensed to what can be read in the time. Condensation is a core subtitling skill, not a fallback.

Condense by removing redundancy, filler, and repetition that do not change meaning; by simplifying syntax without losing the point; and by prioritizing the information the viewer most needs to follow the scene. Preserve tone and function even while cutting words: a sarcastic line must stay sarcastic after condensation, a plot-critical fact must survive. Never condense by dropping meaning the viewer needs, and never condense so heavily that the subtitle no longer represents the speech. Where condensation cannot rescue a cue, the options are splitting across cues or accepting a higher reading speed within tolerance, each a deliberate tradeoff. See condensation as serving readability, which is the subtitle's first duty.

### Handle Synchronization With Overlapping And Fast Speech

Real dialogue overlaps, interrupts, and races, and subtitles must handle these situations without becoming chaotic or unreadable. Plan synchronization for difficult audio.

For overlapping speakers, decide whether to subtitle both, which can overwhelm reading speed, or prioritize the dominant speaker, which may lose content. For interruptions, time cues to the interruption point so the visual logic matches the audio. For very fast speech, condense aggressively and, if standards allow, allow a slightly elevated reading speed rather than splitting into a flurry of micro-cues that flash past. For off-screen and on-screen speaker changes, use positioning or speaker labels per the standard so the viewer knows who is talking. These situations are where amateur subtitles fail most visibly, and disciplined handling is what separates professional work.

### Account For Accessibility And Diverse Readers

Subtitles serve viewers with different reading capacities, including deaf and hard-of-hearing audiences, non-native speakers, children, and people with reading difficulties. Timing must serve the slowest reasonable reader, not the fastest.

For SDH and accessibility tracks, apply stricter reading-speed limits, add sound descriptions and speaker identification as required, and ensure cues never assume the viewer can hear. For children's content, lengthen durations and simplify syntax. Even for standard subtitles, designing for the slower reader protects everyone, because a subtitle readable at a relaxed pace is readable by all. Do not tune timing to your own reading speed; tune it to the standard, which exists precisely to protect the audience that needs more time.

### Verify Timing Against The Actual Video

Subtitle timing must be verified against the rendered video, not just the timecode numbers, because numbers that look correct can still mistime against speech and image. Final timing is a viewing check, not a spreadsheet check.

Watch the subtitles in context, or at least scrub the cue boundaries against the audio waveform and shot list. Confirm each cue appears with its speech, fits its reading-speed budget, respects duration bounds, and does not cross cuts awkwardly. Tools that auto-generate timing from speech detection are a starting point, not a finish line; their output must be reviewed and corrected by someone who understands the craft. Shipping unverified timing is shipping unreadable subtitles and hoping no one notices.

## Common Traps

### Exceeding Reading Speed To Preserve Literal Wording

Refusing to condense and instead overrunning the reading-speed limit produces subtitles no one can finish reading. The text must yield to the time.

### Lingering Subtitles Past The Speech

Extending cues to fit long translations desynchronizes text from audio and makes subtitles feel broken. Synchronization is not optional.

### Flashing Too-Brief Cues

Cues below minimum duration vanish before the eye registers them, especially for accessibility audiences. Respect duration floors.

### Crossing Shot Changes Awkwardly

Text that resets across a cut or lingers into a reaction shot breaks immersion. Align cues to cuts where possible.

### Dropping Meaning To Hit Timing

Condensing so heavily that plot-critical facts or tone vanish leaves the viewer unable to follow the scene. Condense redundancy, not meaning.

### Tuning Timing To The Translator's Own Reading Speed

Personal reading speed is faster than the audience's. Timing must follow the standard, which protects slower readers.

### Trusting Auto-Generated Timecodes Unverified

Speech-detection timing looks plausible but mistimes against speech and image. Final timing requires a viewing check.

## Self-Check

Before approving subtitle timing, verify:

- Every cue fits the reading-speed limit for the target language and audience, with condensation used rather than overruns.
- Each cue respects minimum and maximum duration bounds, with neither flashing micro-cues nor lingering overlong subtitles.
- Cue in and out points align with speech onset and end, and where possible with shot changes, without crossing cuts awkwardly.
- Fast, dense, or overlapping speech was handled with deliberate condensation, splitting, or prioritization rather than chaotic cueing.
- Condensation preserved meaning, tone, and function, dropping only redundancy the viewer does not need.
- Accessibility and slower-reader needs were respected, with stricter limits applied for SDH, children's, and non-native audiences.
- Speaker changes, off-screen dialogue, and interruptions are cued and labeled so the viewer can follow who is speaking.
- Timing was verified against the actual video or waveform, not just timecode numbers, and auto-generated timing was reviewed.
- No cue exceeds reading speed, no cue lingers past speech, and no meaning was dropped merely to hit a timing target.
- Watched by a slow reader in the target language, every subtitle can be comfortably read and understood within its time on screen.
