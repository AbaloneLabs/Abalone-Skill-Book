---
name: accessibility_considerations.md
description: Use when the agent is translating for readers with accessibility needs, adapting content for screen readers and assistive technology, writing plain-language versions, handling audio description and caption translation, or ensuring translated text remains navigable for users with cognitive visual auditory or motor constraints.
---

# Accessibility Considerations

Accessibility in translation is not a finishing step. The choices a translator makes determine whether a reader using a screen reader, a reader with cognitive load constraints, a deaf or hard-of-hearing reader relying on captions, or a low-vision reader using magnification can actually use the target text. A translation that is linguistically excellent but structurally inaccessible excludes readers the source text served. Worse, translators often inherit inaccessible source structure and reproduce it faithfully, exporting the source's accessibility failures into every language. Accessibility must be considered during translation, not appended after.

Use this skill when translating content that will be read through assistive technology, when producing plain-language or easy-read versions, when translating captions audio description or transcripts, or when the target audience includes readers with disabilities. The goal is to keep the translation usable by every reader the source was meant to reach, and where possible to repair accessibility gaps the source left.

## Core Rules

### Preserve And Strengthen Semantic Structure

Screen readers and other assistive technology rely on semantic structure to navigate. Headings, lists, tables, landmarks, and link text are not visual decorations. They are the map a non-sighted reader uses.

When translating, preserve the heading hierarchy exactly. Do not promote or demote headings for visual effect, because the hierarchy is the navigation. Keep lists as lists, not as lines that look like bullets. Keep tables structured with proper header cells; do not collapse a table into prose just because the target language is more verbose. Translate link text so it describes the destination, never render a link as click here or its equivalent, because out of context that tells a screen reader user nothing.

If the source has broken structure, such as headings used for visual sizing or a table used for layout, flag it and propose a semantic fix rather than reproducing the failure.

### Write Alt Text And Descriptions In The Target Language

Images, icons, charts, and diagrams need text alternatives. These must be translated, not left in the source language, and they must be meaningful in the target.

Translate alt text to describe the function and content of the image for the target reader. For decorative images, keep empty alt text rather than inventing descriptions. For charts and data visualizations, provide a summary of the data, not just the chart title. For functional icons, describe the action, such as search or close, in the target language.

Alt text is content. Scope it as translatable and translate it with the same care as visible prose.

### Adapt Plain Language And Easy-Read Conventions

Plain language and easy-read standards differ across languages and locales, but the principles transfer. Translate with accessibility in mind even when the source is not explicitly plain.

Use short sentences, common vocabulary, active voice, and literal meaning over idiom. Define necessary technical terms on first use. Prefer direct address where culturally appropriate. Avoid nested clauses that bury the main point. For easy-read versions, follow the target locale's conventions for sentence length, supported by images, and white space, recognizing that easy-read is a distinct deliverable, not just simplified translation.

When the source is dense and the target audience includes cognitive accessibility needs, propose a plain-language adaptation as a separate deliverable rather than silently simplifying a formal text.

### Handle Captions Subtitles And Transcripts Deliberately

Audio and video content carries its own accessibility layer. Translation here interacts with timing, reading speed, and the needs of deaf and hard-of-hearing viewers.

For captions and subtitles, respect reading speed limits, typically expressed in characters or words per second, and line length limits. Condense where necessary without losing essential meaning. For deaf and hard-of-hearing audiences, include non-speech information such as sound effects and speaker identification when meaningful. Translate audio description, the narration that describes visual action for blind viewers, with the same timing constraints and with vocabulary that conveys action clearly.

Provide transcripts as a parallel deliverable. A transcript is not just the captions concatenated. It should be navigable and complete.

### Avoid Accessibility Barriers In Target Syntax

Some target-language constructions create barriers for assistive technology or for readers with cognitive load. Avoid them where the meaning allows.

Avoid ambiguous punctuation that screen readers may announce oddly. Avoid information conveyed only through color or font, because it will not transfer to text-to-speech or to high-contrast modes. Avoid abbreviations that expand badly when read aloud unless the expansion is provided. Avoid sentence structures where the verb and subject are separated by long insertions, which strain working memory.

In languages with complex scripts, ensure that characters compose correctly so that screen readers pronounce them properly. Broken encoding or wrong joiners can render a word unpronounceable by assistive technology.

### Coordinate With Localization Engineering

Accessibility often depends on markup and tooling, not just text. Coordinate with whoever owns the file format.

Confirm that the target text will carry the same accessibility attributes as the source, such as ARIA roles, lang attributes, heading levels, and table scopes. Confirm that right-to-left or complex-script text will have correct direction and language tags. Confirm that translated alt text will be associated with the right image. Text that is correct in isolation can become inaccessible if the engineering layer drops attributes during localization.

### Test With Accessibility In Mind

A translation that has not been tested with assistive technology may hide barriers. Where possible, listen to the target text with a screen reader, check caption timing against the video, and verify that navigation by headings works in the target.

If full testing is out of scope, at least flag the accessibility-critical elements so the requester knows what to test before publication.

## Common Traps

### Treating Alt Text As Non-Translatable

Leaving alt text in the source language means blind target readers get no information from images. Translate it.

### Reproducing Source Structure Failures

If the source uses headings for visual size or tables for layout, faithfully reproducing that exports the barrier. Flag and propose fixes.

### Using Click Here Style Link Text

Vague link text is useless to a screen reader user navigating by links. Translate link text to describe the destination.

### Ignoring Caption Reading Speed

Translating every word without regard to timing produces captions no one can read. Respect speed and length limits.

### Simplifying Formal Text Silently

Turning a legal or regulatory text into plain language without authorization changes its function. Propose plain-language versions as separate deliverables.

### Forgetting Non-Speech Audio Information

For deaf and hard-of-hearing audiences, meaningful sound effects and speaker changes matter. Omitting them removes information the source conveyed.

### Assuming Complex Scripts Will Render

Text in Arabic, Hindi, Thai, or other complex scripts can break if encoding or direction tags are wrong, breaking screen reader pronunciation. Verify rendering.

## Self-Check

Before approving an accessibility-sensitive translation, verify:

- Heading hierarchy, lists, and table structure are preserved semantically, not visually approximated.
- Alt text for images, icons, and charts is translated and meaningful in the target language.
- Link text describes its destination in the target language, not rendered as vague imperative.
- Sentence length, vocabulary, and syntax support readers with cognitive load where the audience requires it.
- Captions and subtitles respect reading speed and line limits, and include non-speech information where meaningful.
- Audio description, where in scope, is translated with timing and clarity for action.
- Transcripts are provided as complete, navigable deliverables where required.
- No information is conveyed only through color, font, or layout without a text equivalent.
- Complex-script text has correct encoding, direction, and language tags so assistive technology handles it.
- Accessibility attributes such as ARIA roles, lang attributes, and table scopes are confirmed to survive localization engineering.
- Where source structure is inaccessible, the issue is flagged with a proposed fix rather than reproduced.
- Accessibility-critical elements are identified for testing before publication.
