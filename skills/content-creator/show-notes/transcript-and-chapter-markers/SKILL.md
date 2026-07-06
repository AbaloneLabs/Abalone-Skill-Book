---
name: transcript_and_chapter_markers.md
description: Use when the agent is producing or editing episode transcripts, generating and verifying chapter markers, aligning timestamps to topic shifts, handling accessibility captions, or deciding transcript format and accuracy standards for podcast or video content.
---

# Transcript And Chapter Markers

Transcripts and chapter markers are the navigational and accessibility layer of audio and video content. The judgment problem is that both are treated as mechanical post-production tasks when they are actually judgment products. A transcript is not just a word-for-word record; decisions about speaker labels, filler removal, accuracy thresholds, and formatting determine whether it is usable by search engines, accessibility tools, and human readers. Chapter markers are not just timestamps; their placement and labels determine whether a listener can find the moment they remember and whether platforms surface the episode in search. Done badly, both degrade accessibility, hurt discovery, and create correction work; done well, they multiply the value of the same recording at low marginal cost.

The harm from poor transcripts and chapters is concrete and often invisible to the creator. Inaccurate transcripts misquote the guest and the host, which is especially damaging in interview, technical, or accountability content. Missing or misaligned chapters make long episodes un-navigable, which depresses completion and re-listening. Inaccessible content excludes deaf and hard-of-hearing audiences and, in many jurisdictions, creates compliance exposure. This skill helps the agent set accuracy and formatting standards for transcripts, place and label chapters at real topic boundaries, and verify both against the actual media before publishing.

## Core Rules

### Decide The Transcript Type Before Producing It

There are several legitimate transcript types and they serve different purposes. Choosing the wrong one wastes effort and produces a document nobody uses. Decide the type based on the audience and use case before generating.

Common transcript types:

- verbatim, every word including filler and false starts, used for legal, accountability, or research contexts;
- intelligent verbatim, filler and stutters removed but meaning preserved, the default for most published content;
- edited, restructured for readability, used for article-style transcripts where readability matters more than fidelity;
- captions or subtitles, time-synced and length-limited for on-screen reading.

The choice changes everything downstream, accuracy standards, speaker labeling, and how the transcript can be cited. State the type at the top of the document so readers know what fidelity to expect.

### Set An Explicit Accuracy Standard And Verify Against It

Auto-generated transcripts are fast but routinely introduce errors that change meaning, especially with names, numbers, technical terms, and any accent the model was not trained on. An uncorrected auto-transcript is a liability, not a feature. Set an accuracy standard and verify against it before publishing.

Accuracy practices:

- for published content, target near-perfect accuracy on names, numbers, quotes, and proper nouns, these are what get cited and what cause corrections;
- correct technical terms and brand names manually, auto-transcription gets these wrong reliably;
- read the transcript against the audio for at least the high-stakes sections, quotes, claims, attributions;
- treat any claim that could be defamatory or consequential as requiring manual verification;
- keep a correction process for after publish, because listeners will catch errors.

A transcript that misquotes a guest is worse than no transcript because it creates a false record that can be screenshotted and shared.

### Label Speakers Consistently And Correctly

Speaker labeling is where transcripts most often become unusable. Inconsistent labels, wrong attributions, and unlabeled speaker changes make the transcript impossible to follow. Establish a labeling convention and apply it throughout.

Speaker label practices:

- use consistent names or roles, not "Speaker 1" mixed with the person's name;
- verify attribution at every speaker change, auto-transcription misattributes frequently in cross-talk;
- mark unclear or overlapping speech explicitly rather than guessing;
- for multi-guest episodes, disambiguate guests with the same first name or role;
- keep label formatting consistent, name colon, across the whole document.

A misattributed quote in a transcript can damage a guest's reputation and the creator's credibility. Verify attributions for any quote that could be consequential.

### Place Chapter Markers At Real Topic Boundaries

Chapter markers are navigation. Their value depends entirely on whether they mark boundaries a listener would actually want to jump to. Arbitrary intervals, every five minutes, are useless; markers at genuine topic shifts are valuable.

Placement practices:

- place a marker where the conversation genuinely shifts topic, not on a clock;
- mark the moments listeners are most likely to seek, the question, the demo, the reveal, the disagreement;
- avoid over-segmenting, too many chapters make the list unscannable;
- avoid under-segmenting, one chapter for a 90-minute episode defeats the purpose;
- match granularity to episode length and topic density.

Think about what a returning listener would search for and mark those moments. The chapter list should read like a useful table of contents.

### Label Chapters In Specific, Searchable Language

Chapter labels are indexed by platforms and search engines, and they are read by humans scanning for the right moment. Vague labels fail both audiences. Write labels that are specific and searchable.

Label practices:

- write a specific phrase, not "discussion" or "part 2";
- include the proper noun, topic, or question the chapter addresses;
- keep labels short enough to display cleanly on mobile;
- front-load the important word so truncated labels still make sense;
- use consistent capitalization and punctuation across chapters.

A chapter labeled "Why the launch failed" is useful and searchable; a chapter labeled "More talk" is neither.

### Verify Timestamps Against The Actual Media

Timestamps drift. They drift because of intro music, ad insertions, platform-specific players, and edits made after chapters were set. An unverified timestamp betrays the listener who trusts it. Verify every chapter timestamp against the actual published media.

Verification practices:

- set chapters against the final, published version of the media, not an earlier cut;
- re-verify if the media is re-edited or if ad slots change;
- account for platform differences, some platforms offset chapters by the intro length;
- test the chapters in the actual player before declaring done;
- keep a single source of truth for timestamps and update all surfaces, notes, chapters, captions, from it.

A listener who jumps to a chapter and lands in the wrong place loses trust in the whole navigation system.

### Serve Accessibility, Not Just Navigation

Transcripts and captions are an accessibility obligation, not just a discovery feature. Deaf and hard-of-hearing audiences, listeners in sound-sensitive environments, and non-native speakers depend on them. Treat accessibility as a requirement with its own standards, not an optional add-on.

Accessibility practices:

- provide synchronized captions for all video, not just a static transcript;
- include meaningful non-speech audio cues in captions, [laughter], [music], [applause], where they convey content;
- keep caption reading speed and line length within accessibility guidelines;
- do not rely on auto-captions alone for published video, verify and correct them;
- ensure transcript and caption text is selectable, searchable, and screen-reader friendly.

Accessibility done well also improves discovery, SEO, and skimming, but the primary motivation is that the content should be available to everyone.

## Common Traps

### Publishing Raw Auto-Transcripts Without Correction

Publishing machine-generated transcripts unchanged is a trap because they contain predictable errors on names, numbers, and technical terms that then get cited as the guest's words. This is a trap because the creator becomes responsible for a false record. Correct at least the high-stakes terms.

### Misattributing Quotes Through Bad Speaker Labels

Letting auto-labeling assign quotes to the wrong speaker is a trap because a misattributed quote can damage reputations and destroy credibility. This is a trap because the error is invisible until a guest notices, by which point it has been shared. Verify every speaker change.

### Interval-Based Chapters Instead Of Topic-Based

Dropping a chapter every five minutes regardless of content is a trap because the boundaries rarely match what a listener wants to find. This is a trap because the navigation feature exists but provides no navigation value. Place chapters at real topic shifts.

### Vague Chapter Labels

Labeling chapters "intro," "main part," and "wrap up" is a trap because nobody searches for those and they convey nothing. This is a trap because the chapter list fails both human scanning and platform indexing. Write specific, searchable labels.

### Timestamps That Drift After Edits

Setting chapters on a draft and not re-verifying after the final edit is a trap because intros, ads, and trims shift every timestamp. This is a trap because listeners land in the wrong place and lose trust. Verify against the published media.

### Treating Captions As Optional

Skipping captions because the audience is "mostly listeners" is a trap because it excludes deaf and hard-of-hearing audiences and, in many cases, violates accessibility expectations. This is a trap because the cost is low and the exclusion is real. Provide synchronized captions.

### Inconsistent Transcript Type

Mixing verbatim and edited styles within a transcript, or not stating the type, is a trap because readers cannot tell what fidelity to expect and may cite filler as a quote. This is a trap because it creates a misleading record. Choose and state one type.

## Self-Check

- Did I choose a transcript type and state it, so readers know what fidelity to expect?
- Does the transcript meet a near-perfect accuracy standard on names, numbers, quotes, and proper nouns, verified against the audio?
- Are speaker labels consistent and correct at every speaker change, with unclear speech marked rather than guessed?
- Are chapter markers placed at genuine topic boundaries that a listener would want to seek, not at arbitrary intervals?
- Are chapter labels specific and searchable, with the important word front-loaded?
- Were all timestamps verified against the final published media, and re-verified after any edit?
- Is there a single source of truth for timestamps that feeds notes, chapters, and captions consistently?
- Are synchronized captions provided for all video, with meaningful non-speech cues included?
- Do captions meet reading-speed and line-length accessibility guidelines?
- Could a guest be fairly quoted from this transcript without embarrassment or correction? If not, fix it before publishing.
