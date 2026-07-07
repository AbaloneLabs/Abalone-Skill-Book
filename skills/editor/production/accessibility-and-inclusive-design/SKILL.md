---
name: accessibility_and_inclusive_design.md
description: Use when the agent is preparing edited content for accessibility, writing and reviewing alt text and image descriptions, ensuring heading structure and semantic markup, checking color contrast and color-only encoding, managing reading order and landmark roles, verifying captions transcripts and audio descriptions, or ensuring publications are usable by readers with disabilities across formats and assistive technologies.
---

# Accessibility And Inclusive Design

Accessibility in editorial production is the discipline of ensuring that a published work can be read and used by people with disabilities, not as a remedial afterthought applied at the end, but as a set of structural decisions made throughout production. An inaccessible publication excludes readers, narrows audience, and in many jurisdictions violates legal obligations. Editors hold a central role here because most accessibility barriers are created or removed at the content and structure level, in decisions about headings, images, color, reading order, and alternatives to media, long before the question of technical compliance arises.

Use this skill when preparing edited content for publication in any format where accessibility matters, web, EPUB, PDF, app, or print-derived digital. It covers writing and reviewing alt text and image descriptions, ensuring semantic heading structure and markup, checking color contrast and color-only encoding, managing reading order and landmarks, providing captions, transcripts, and audio descriptions, and verifying that the work is usable with assistive technologies such as screen readers. The editor's task is to treat accessibility as a content quality, not merely a technical checklist, and to make the publication genuinely usable, not just nominally compliant.

## Core Rules

### Write Alt Text That Conveys Meaning Not Appearance

Alternative text is the primary bridge between visual content and readers who cannot see it. Its purpose is not to describe what an image looks like but to convey what the image communicates in context. The most common accessibility failure is alt text that narrates surface appearance, "a photo of a chart," while omitting the information the chart carries.

For each image, decide its role in the work. If it conveys information, the alt text must convey the same information, the trend, the comparison, the conclusion a sighted reader draws. If it is decorative, mark it as decorative so a screen reader skips it rather than reading noise. If it contains text, that text must appear in the alt text or a description. Avoid redundant alt text that repeats the adjacent caption, and avoid generic phrases like "image of" that add words without meaning. For complex figures, use a long description, either in surrounding text or a linked description, rather than cramming detail into the alt attribute. The test is whether a reader using the alt text alone would receive the information the image exists to deliver.

### Enforce Semantic Heading Structure As A Navigation System

Headings are not visual styling; they are the document's navigation skeleton for screen-reader users, who move through documents by jumping from heading to heading. When headings are chosen for size or emphasis rather than hierarchy, the navigation breaks and the document becomes unusable for readers who depend on structure.

Confirm that headings reflect true document structure: one top-level heading, sub-sections nested under it, and no skipped levels. Do not use a heading format to make text large or bold; use styling for that. Do not skip from a top-level heading to a third-level heading without a second level between them. Ensure that heading text is descriptive enough to navigate by, since screen-reader users skim a heading list before reading. Where the source document uses headings inconsistently, restructure them to reflect the actual outline before publication. Semantic structure is the single highest-impact accessibility decision an editor controls.

### Check Color Contrast And Eliminate Color-Only Encoding

Color is a powerful design tool, but it fails as a sole carrier of meaning. Readers with color vision deficiency, low vision, or monochrome displays cannot perceive information encoded only in color, and low-contrast text excludes readers with low vision entirely. Accessibility review must check both how color is used and how legible it is.

For color encoding, confirm that no information depends on color alone; pair color with text labels, patterns, or position so the meaning survives without color. A chart where red and green lines are distinguished only by hue is inaccessible; add labels or line styles. For contrast, confirm that text and essential graphics meet the relevant contrast threshold against their background, accounting for text size, since smaller text needs higher contrast. Watch for common failures, light gray text, low-contrast links distinguished only by color, and text over images without a sufficient backing. Where the design cannot be changed, flag the barrier and propose a remedy, because contrast failures cannot be fixed by the reader.

### Manage Reading Order And Landmark Roles

In structured digital formats, the order in which content is read by assistive technology depends on the underlying markup, not the visual layout. Content that appears logical on the page can read as nonsense if the reading order is wrong, and sections that look distinct can blur together without landmark roles marking their boundaries.

Confirm that the reading order, the sequence a screen reader follows, matches the intended logical order of the content. Watch for layouts where visual position diverges from source order, multi-column layouts that read across instead of down, sidebars and pull quotes that interrupt the main flow, and tables used for layout rather than data. Ensure landmark roles, navigation, main, header, footer, and region labels, mark the major areas so readers can navigate them. For complex layouts, test the reading order rather than assuming it. A page that looks right but reads wrong is inaccessible regardless of its appearance.

### Provide Captions Transcripts And Audio Descriptions For Media

Time-based media, audio and video, requires text alternatives for readers who cannot perceive the audio or the visual track. Captions, transcripts, and audio descriptions each serve a distinct access need, and omitting any of them leaves a group of readers excluded.

For audio content, provide a transcript that captures the spoken content and meaningful non-speech audio. For video, provide synchronized captions for the audio track, and, where the visual track carries information not present in the audio, provide audio description, a narration of essential visual content during pauses. Confirm that captions are accurate, not auto-generated without correction, and that they include speaker identification and relevant sound cues. For live media, plan the access method in advance, since real-time captioning and description require arrangement. Each medium needs its specific alternative; a transcript is not a substitute for captions on a video, and captions do not replace a transcript for audio-only content.

### Verify Keyboard And Interaction Accessibility

Not all readers use a mouse or touch. Some navigate entirely by keyboard, by voice command, or through switch devices, and interactive elements that work only with a pointer exclude these readers. Editorial review should flag interactive content whose accessibility depends on a specific input method.

For interactive elements, links, forms, embedded widgets, confirm that they are reachable and operable by keyboard alone, in a logical order, with visible focus indication. Check that link text is descriptive, not "click here," since screen-reader users navigate link lists out of context. Confirm that form fields have associated labels and that errors are announced, not merely visually indicated. Where the content embeds third-party widgets, social media, interactive charts, or forms, test their accessibility rather than assuming it. Interaction barriers are often invisible to mouse users and are caught only by deliberate testing.

### Test With Assistive Technology Not Just Against Rules

Conformance to a checklist is not the same as usability. A document can pass automated checks and still be difficult or impossible to use with a screen reader, because real accessibility is verified by use, not by rule-matching alone. Editorial review should include, or advocate for, testing with the tools and readers it claims to serve.

Where possible, test key content with a screen reader, keyboard-only navigation, and browser zoom or reflow to confirm the experience matches the intent. Automated tools catch mechanical failures, missing alt text, contrast ratios, broken headings, but they miss structural and semantic problems that only human testing reveals. For high-stakes publications, include users with disabilities in review. Where full testing is not possible, document what was checked and what remains unverified, so the publication does not claim an accessibility it has not confirmed. The goal is a publication readers can actually use, not a checklist that has been satisfied.

## Common Traps

### Alt Text Describing Appearance Over Meaning

"A photo of a graph" conveys nothing. Write alt text that delivers the information the image carries in context.

### Headings Used For Visual Styling

Choosing a heading for size breaks navigation for screen-reader users. Use true document hierarchy.

### Color As The Sole Carrier Of Meaning

Color-only encoding fails for color-blind and monochrome readers. Pair color with labels or patterns.

### Low-Contrast Text Assumed Legible

Light gray and low-contrast links exclude readers with low vision. Check contrast against the threshold.

### Reading Order Following Visual Layout

Visual position can diverge from source order. Test the sequence a screen reader follows.

### Auto-Captions Used Without Correction

Machine captions are inaccurate and exclude the readers they claim to serve. Correct and verify captions.

### Checklist Conflated With Usability

Passing rules is not the same as being usable. Test with assistive technology where stakes warrant.

### Decorative Images Given Meaningful Alt Text

Noise alt text on decorative images clutters the screen-reader experience. Mark decorative images as such.

## Self-Check

Before treating accessibility review as complete, verify:

- Every informational image has alt text that conveys its meaning in context, decorative images are marked as decorative, and complex figures have long descriptions where needed.
- Heading structure reflects true document hierarchy with no skipped levels and no headings used merely for visual styling.
- No information depends on color alone, and text and essential graphics meet the relevant contrast threshold.
- Reading order matches the intended logical sequence, and landmark roles mark major document regions.
- Audio content has transcripts, video has synchronized captions, and visual-only information has audio description where needed.
- Captions are accurate and corrected, not auto-generated without review, and include speaker and sound cues.
- Interactive elements are keyboard-operable, with descriptive link text, labeled form fields, and visible focus indication.
- Key content has been tested with assistive technology, not only checked against rules, with unverified areas documented.
- The publication is genuinely usable by readers with disabilities, not merely nominally compliant with a standard.
