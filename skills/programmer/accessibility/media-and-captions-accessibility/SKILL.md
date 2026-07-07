---
name: media_and_captions_accessibility.md
description: Use when the agent is embedding or building video or audio players, adding captions, subtitles, transcripts, or audio descriptions, deciding whether media must be captioned, wiring a keyboard-operable player, handling autoplaying audio, supporting sign language or live captioning, or choosing between synchronized captions and a transcript fallback. Also covers the failure mode of shipping auto-generated captions without review, autoplaying sound that interferes with screen readers, or a media player whose controls work only with a mouse.
---

# Media And Captions Accessibility

Media accessibility is the discipline of ensuring that the information conveyed through audio and video is available to users who cannot hear it, cannot see it, or cannot perceive it in real time. Audio and video are powerful carriers of information, but they are also the most exclusionary: a video whose spoken content has no captions is completely inaccessible to deaf and hard-of-hearing users; a video whose meaning depends on visuals not described in the audio is inaccessible to blind users; an audio clip with no transcript is inaccessible to anyone in a non-audio context or who reads more slowly than the clip plays. Unlike a missing alt attribute, a missing caption is rarely noticed by the team because the people who need it are not in the room, and the content "plays fine" for everyone testing it.

Agents tend to under-invest here for three reasons: the terminology is confusing (captions versus subtitles versus transcripts versus audio descriptions are genuinely different things serving different needs), auto-generated captions feel like a finished solution when they are a draft, and the legal and practical obligations differ by context (prerecorded versus live, essential versus decorative). The judgment problem is deciding, for each piece of media, what alternatives are required, what quality bar they must meet, and how the player itself must behave so that users can actually operate the media. This skill covers the distinctions agents routinely blur and the decisions that determine whether media is genuinely usable or merely present.

## Core Rules

### Know The Difference Between Captions, Subtitles, Transcripts, And Audio Descriptions

These four terms are not interchangeable, and confusing them leads to providing the wrong alternative. Captions are a synchronized text track of the audio content — dialogue, sound effects, and speaker identification — timed to the video, designed for users who cannot hear the audio (deaf and hard-of-hearing users, or anyone watching muted). Subtitles are a translation of the dialogue into another language, designed for users who can hear but do not understand the spoken language; they typically omit sound-effect and speaker cues that captions include. A transcript is a full text version of the audio (and ideally a description of key visuals), provided as a separate document or block of text, readable at the user's own pace and searchable. An audio description is a separate audio track that narrates the important visual information not conveyed by the dialogue, designed for blind and low-vision users.

The practical consequence: providing subtitles does not satisfy the caption requirement, because subtitles omit the sound and speaker information a deaf user needs. Providing a transcript does not replace synchronized captions for video, because a transcript is not timed to the action and forces the user to split attention. Providing captions does nothing for a blind user if the video's meaning depends on visuals the dialogue never describes. For each piece of media, ask which of these four alternatives each audience needs, and provide that specific one. The default for spoken video is synchronized captions plus a transcript; the default when visuals carry essential information is to add an audio description.

### Caption All Prerecorded Video With Speech Or Meaningful Sound

Any prerecorded video that contains speech, or sound that carries information, requires synchronized captions. This is a WCAG requirement (1.2.2) and a legal obligation in many jurisdictions for public-facing content. Captions must cover not only dialogue but the sound information a hearing user would rely on: a doorbell, an explosion, a tone of voice, a phone ringing, music that signals mood. They must identify speakers when ambiguity would otherwise exist, and they must be synchronized closely enough that the text matches the moment the sound occurs.

The quality bar matters as much as the presence. Captions that lag badly, that drop words, that paraphrase loosely, or that misidentify technical terms defeat the purpose. The most common failure is shipping auto-generated captions (from a speech-to-text service) as if they were finished. Auto-generated captions are a draft: they routinely mangle proper nouns, numbers, technical vocabulary, and overlapping speech, and they produce text that is subtly wrong in ways that change meaning. A human must review and correct them before they count as captions. Plan for that review step; an auto-generated track that was never opened is not an accessible track.

### Provide A Transcript As A Universal Fallback, Not Only For Audio

A transcript is the full text of the media's audio, and ideally a description of its key visuals, provided as readable text alongside the player. For audio-only content (a podcast, a recorded lecture with no visuals), a transcript is the primary alternative and is required — there is no synchronized track to attach captions to. For video, a transcript is a valuable supplement to captions: it lets users search the content, skim it, read at their own pace, copy quotes, and consume the media in a context where playing audio is impossible (a quiet office, a library, a metered connection).

Treat the transcript as a universal fallback that also aids search engine indexing, translation, and users who simply prefer to read. Provide it as real, selectable text (not an image of text), linked near the player, and structured with headings and speaker labels where helpful. The trap is treating the transcript as optional because captions exist — captions serve real-time viewing, the transcript serves everything else, and many users depend on it.

### Add Audio Description When Visuals Carry Essential Information

Captions solve the problem for users who cannot hear; audio description solves it for users who cannot see. When a video's meaning depends on visual information that the dialogue does not convey — a chart on screen, an action the narrator does not describe, text overlaid on the video, a facial expression or gesture that changes meaning — a blind user cannot follow the content. An audio description track narrates those visuals during the natural pauses in dialogue, so the blind user receives the same information a sighted user does.

Not every video needs an audio description. If all essential information is already conveyed in the audio (a talking-head lecture where the speaker says everything that matters), the existing audio suffices. The decision is whether removing the visuals would lose meaning; if yes, add the description. For content where visuals are dense and dialogue leaves no pauses, an extended audio description (which can pause the video to fit the narration) may be required. WCAG makes audio description a requirement at level AA for prerecorded video (1.2.5) when the visuals are essential, so treat it as a baseline, not an enhancement, for informational and instructional video.

### Make The Media Player Itself Keyboard-Operable And Assistive-Technology-Friendly

An accessible media track is useless if the player cannot be operated. The controls — play, pause, seek, volume, mute, captions toggle, fullscreen — must be reachable by keyboard (Tab and Shift+Tab), operable with the expected keys (Enter or Space to toggle, arrows to seek and adjust volume), and labelled so a screen reader announces what each control does. A player whose controls are mouse-only, or whose buttons are unlabelled icon glyphs announced as "button," excludes keyboard and screen-reader users entirely.

Verify the player explicitly: tab into it, confirm you can reach every control, operate it without a mouse, and that the caption and audio-description toggles are themselves announced and reflect their state. Confirm there is no keyboard trap — the player must release focus back to the page when the user tabs past it. Prefer players with a documented accessibility commitment over custom or legacy players, because retrofitting keyboard and screen-reader support onto a player that lacks it is expensive and error-prone.

### Never Autoplay Audio, And Control Autoplaying Video

Audio that starts playing without user action is one of the most disruptive accessibility failures. It interferes directly with screen-reader output (which is also audio), startling and disorienting blind users; it is jarring for users with sensory sensitivity, anxiety, or cognitive load; and it is hostile in shared or quiet environments. WCAG permits autoplay only if the audio lasts less than three seconds or there is a mechanism to stop or mute it, but the practical guidance is stronger: do not autoplay audio at all. Let the user choose to start it.

Autoplaying video without sound is more tolerable but still carries risk: motion can distract users with attention disorders, and looping motion can trigger vestibular discomfort. If video autoplays muted, provide an obvious way to pause it, respect `prefers-reduced-motion`, and avoid autoplaying motion that covers a large area or loops indefinitely. The safe default is no autoplay; if business requirements demand it, mute by default, provide a clear pause control, and keep the motion contained.

### Plan For Live Media And Sign Language Where The Stakes Are High

Live audio and video have different constraints. Live captioning — real-time captions produced by a human captioner or a high-quality speech-to-text system with human oversight — is required for live broadcasts that contain essential speech (WCAG 1.2.4 at level AA). Auto-generated live captions without oversight are a draft, just as with prerecorded media, and their error rate on live content is often worse; for high-stakes live content (an emergency briefing, a medical or legal proceeding, a graduation), use a human captioner.

Sign language interpretation provides access for users whose first language is a sign language, and for whom written captions may be a second language or a slower channel. It is not required by WCAG at level AA, but for content with broad public impact or where the audience includes deaf users who rely on sign, providing a sign-language interpreter (picture-in-picture or a toggle) is a higher tier of access. Decide based on audience and stakes: captions are the baseline everywhere; sign language is the baseline where the deaf audience is known to depend on it.

## Common Traps

### Treating Auto-Generated Captions As Finished

Shipping the raw output of a speech-to-text service as the caption track, with no human review. Auto-generated captions routinely mis-transcribe proper nouns, numbers, technical terms, and overlapping speech in ways that change meaning. A human must review and correct them before they count.

### Providing Subtitles And Calling Them Captions

Adding a translated dialogue track and believing the caption requirement is met. Subtitles serve users who can hear but do not understand the language; they omit the sound-effect and speaker cues a deaf user needs. Captions are a different artifact; provide the right one for each audience.

### No Transcript For Audio-Only Content

A podcast or audio lecture with no text alternative. There is no synchronized track to caption, so the transcript is the primary access path and is required. Provide a full text transcript as readable, searchable text.

### Video Whose Meaning Depends On Unspoken Visuals, With No Audio Description

An instructional or informational video where the visuals carry essential meaning the dialogue never describes, leaving blind users unable to follow. If removing the visuals would lose meaning, add an audio description track.

### Player Controls That Work Only With A Mouse

A media player whose play, pause, seek, and caption toggles are not keyboard-reachable or not labelled, so screen-reader users cannot operate the media at all. Every control must be keyboard-operable and have an announced name; verify by tabbing through the player.

### Autoplaying Audio That Fights The Screen Reader

Audio that starts on page load and plays over the screen reader's speech output, disorienting blind users and disturbing everyone in shared spaces. Do not autoplay audio; if it is unavoidable, mute by default and provide an immediate stop control.

### Captions That Exist But Cannot Be Turned On

A video with a caption track but a player whose caption toggle is broken, hidden, or unlabelled, so the track is unreachable. The caption toggle must be operable by keyboard, announced, and reflect its on or off state.

### Treating A Transcript As Optional Because Captions Exist

Skipping the transcript on the assumption that captions cover the need. Captions serve real-time viewing; the transcript serves search, skimming, non-audio contexts, translation, and users who read at their own pace. Provide both for spoken video.

## Self-Check

- [ ] Every prerecorded video with speech or meaningful sound has synchronized captions covering dialogue, sound effects, and speaker identification, and the captions were reviewed and corrected by a human rather than shipped as raw auto-generated output.
- [ ] Audio-only content (podcasts, lectures) has a full text transcript provided as readable, searchable, selectable text — not an image of text — because there is no synchronized track to caption.
- [ ] Video whose meaning depends on visuals not described in the dialogue has an audio description track (or extended audio description where dialogue leaves no pauses), verified by asking whether removing the visuals would lose meaning.
- [ ] The distinction between captions (for deaf users, including sound cues), subtitles (translation for hearing users), transcripts (untimed fallback), and audio description (for blind users) is respected — the correct alternative is provided for each audience, not a substitute.
- [ ] The media player is fully keyboard-operable: every control is reachable by Tab, operable with expected keys, labelled so a screen reader announces its function, and the player does not trap keyboard focus.
- [ ] No audio autoplays; any autoplaying video is muted by default, provides an obvious pause control, respects `prefers-reduced-motion`, and avoids large-area or infinite looping motion.
- [ ] The caption and audio-description toggles are operable by keyboard, announced, and reflect their current on or off state — verified by turning captions on and off without a mouse.
- [ ] Live media with essential speech uses real-time captioning with human oversight (not unsupervised auto-generation), and sign-language interpretation is provided where the audience is known to depend on it.
- [ ] Verification included playing the media with captions on, with audio off, and operating the player by keyboard and screen reader — not only confirming the track file exists.
