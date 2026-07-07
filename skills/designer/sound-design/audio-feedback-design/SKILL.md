---
name: audio_feedback_design.md
description: Use when the agent is designing audio feedback for interactions and system events, including confirmation sounds, error tones, success chimes, notification alerts, hover and press sounds, and the decision of when sound aids feedback versus when it annoys, including volume, repetition, timing, and user control over audio.
---

# Audio Feedback Design

Sound is the most easily abused feedback channel in a product. Done well, a subtle confirmation tone can make an action feel complete and satisfying in a way no visual can. Done poorly, the same channel produces annoyance, anxiety, and abandonment: error tones that punish, success chimes that condescend, notifications that repeat until muted, and feedback sounds that fire on every hover and click until the user reaches for the mute button. The default state of badly designed audio is hostility, which is why so many products ship with sound off.

Use this skill when deciding whether and how to use sound to confirm actions, signal states, or alert users. The goal is to prevent the agent from adding sound reflexively because it feels engaging, and instead to treat audio as a scarce, high-impact channel that must earn its place, respect the user's environment, and always remain under user control.

## Core Rules

### Decide Whether Sound Earns Its Place At All

The first question in audio feedback is not what the sound should be, but whether there should be a sound at all. Sound is intrusive in a way visuals are not: it cannot be ignored, it leaks into the user's environment, and it persists even when the user is not looking at the screen. Most interactions do not need sound. Reserve it for moments where audio provides value that vision cannot.

Sound may earn its place when:

- the user is not looking at the screen and needs to know an event occurred;
- the action is significant enough to warrant confirmation, such as a payment or send;
- the user is in an eyes-busy context, such as driving, cooking, or exercising;
- audio carries information that vision cannot, such as direction or progress;
- the product is inherently auditory, such as music, calls, or games.

If the user is looking at the screen and the visual feedback already communicates the state, adding sound is usually redundant noise.

### Match Sound Weight To Event Significance

Not every event deserves the same sonic treatment. A sent message, a completed payment, a critical error, and a routine toggle are different in importance, and their sounds should reflect that. Using a triumphant chime for a trivial action condescends; using a harsh tone for a minor issue punishes.

Calibrate by significance:

- **Trivial events**: no sound, or the most minimal possible cue;
- **Routine confirmations**: short, neutral, unobtrusive;
- **Significant successes**: slightly more present, positive, but not celebratory to excess;
- **Errors and warnings**: distinct and attention-getting without being alarming or punitive;
- **Critical alerts**: clearly urgent, but reserved for genuinely critical events.

A product where every action makes a noise trains users to ignore all of them, including the important ones.

### Design For The Real Acoustic Environment

Designers often evaluate sound on good speakers or headphones in a quiet room. Users hear product sounds on tinny phone speakers, in noisy public spaces, in quiet offices where any sound is embarrassing, and in bedrooms where a notification wakes a sleeping partner. The acoustic environment shapes whether a sound works.

Account for environment by:

- testing on the actual speakers most users will use, especially phone speakers;
- ensuring important sounds are audible in noisy environments without being piercing in quiet ones;
- avoiding low frequencies that disappear on small speakers;
- considering that any sound may be unwanted in shared or quiet spaces;
- providing vibration as an alternative or complement to sound on mobile.

A sound that works in the studio can fail or offend in the real world.

### Treat Repetition And Frequency As A Cost

A sound that is pleasant once can become maddening at the tenth repetition. Repetition is the single greatest cause of users disabling audio entirely. Frequency must be treated as a design cost, not an afterthought.

Manage repetition by:

- not playing sound on every hover, focus, or minor interaction;
- varying or suppressing repeated confirmations during rapid use;
- throttling repeated notifications of the same event;
- providing a path to reduce frequency without fully muting;
- considering cumulative exposure over a session, not just a single instance.

The cost of a sound is its weight multiplied by how often it plays. A light sound played constantly can cost more than a heavier one played rarely.

### Never Use Sound As The Only Feedback Channel

Sound is fragile as a sole channel. The device may be muted, the user may be deaf or hard of hearing, the environment may be too noisy, or the speaker may be broken. Any state communicated by sound must also be visible, and ideally tactile on mobile.

Ensure redundancy:

- every audio-confirmed state has a clear visual equivalent;
- critical alerts pair sound with vibration and a visible banner;
- error tones are accompanied by visible error messaging;
- success sounds are accompanied by visible success state;
- the interface remains fully usable with sound off.

Designing for sound-off is not a fallback; it is the baseline, because most users will encounter the product that way at some point.

### Design Volume, Ducking, And Audio Coexistence

Product sounds rarely exist in isolation. They play over music, calls, videos, and other media, and they must coexist gracefully. A notification that blasts at full volume over a quiet song, or that interrupts a call, is a failure of audio design, not just volume.

Consider:

- appropriate volume relative to other audio, often lower than expected;
- ducking, lowering other audio when a sound plays, for important events only;
- routing sounds to the correct audio stream so they respect user volume settings;
- avoiding overlap where multiple sounds collide and become noise;
- respecting the user's mute and do-not-disturb settings without exception.

### Provide Clear, Persistent User Control

The most important property of audio feedback is that it must always be under user control. Users must be able to mute, adjust, and disable sounds easily and persistently, and the product must respect that choice across sessions. Hiding the mute control or resetting it on update is a betrayal of trust.

Provide control by:

- a clearly accessible mute or sound settings control;
- granular control where useful, separating notifications, feedback, and media;
- persistence of the user's choice across sessions and updates;
- respect for system-level mute and do-not-disturb;
- never overriding the user's audio preferences for marketing or engagement.

## Common Traps

### Adding Sound Because It Feels Engaging

Reflexively adding sounds to interactions because they feel lively usually produces annoyance, because most interactions do not benefit from audio.

### Triumphant Sounds For Trivial Actions

Celebratory chimes for routine actions condescend to the user and cheapen genuinely significant moments.

### Ignoring The Real Speaker And Environment

Sounds designed on good speakers in quiet rooms fail or offend on phone speakers in public spaces.

### Constant Repetition

Pleasant sounds become unbearable through repetition, driving users to mute everything, including important alerts.

### Sound As The Only Cue

Relying on sound alone excludes muted, deaf, or noisy-environment users and fails when the speaker is unavailable.

### Overriding User Mute

Playing sounds despite mute or do-not-disturb, especially for engagement, is a trust-destroying dark pattern.

### Harsh, Punitive Error Tones

Error sounds that blame or alarm the user for minor mistakes create anxiety and resentment toward the product.

## Self-Check

- [ ] Each sound earns its place by providing value that vision alone cannot, and is absent where redundant.
- [ ] Sound weight matches event significance, with trivial events silent or minimal and celebration reserved for genuine moments.
- [ ] Sounds were tested on real speakers, especially phone speakers, and in noisy and quiet environments.
- [ ] Repetition and frequency are treated as costs, with throttling and suppression during rapid or repeated use.
- [ ] No state relies on sound alone; every audio cue has a visible and, on mobile, tactile equivalent.
- [ ] Volume, ducking, and audio routing respect other media and the user's volume settings.
- [ ] Users have clear, persistent, granular control over sound, and the product never overrides mute or do-not-disturb.
- [ ] Error and warning sounds inform without punishing or alarming the user for minor mistakes.
