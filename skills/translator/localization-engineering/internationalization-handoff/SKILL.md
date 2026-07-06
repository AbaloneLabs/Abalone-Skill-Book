---
name: internationalization_handoff.md
description: Use when the agent is preparing content or software for localization handoff, identifying internationalization defects in source materials, defining handoff packages, or collaborating with developers to ensure the source is localization-ready before translation begins.
---

# Internationalization Handoff

Internationalization (i18n) is the work done to the source so that localization can proceed without hitting walls. It is the preparation that separates content and code that can be localized smoothly from content and code that forces rework, broken builds, and compromised quality. When i18n is done well, the handoff to localization is clean: strings are externalized, layouts are flexible, encoding is correct, locale conventions are handled, and the source carries no hard-coded assumptions that will fail in other languages. When i18n is skipped or done poorly, localization inherits a mess: translators find strings embedded in code, layouts that cannot expand, dates and numbers hard-coded in formats, and concatenations that cannot be reassembled. The localization team then faces impossible choices: ship defective localized versions, or send the work back for engineering that delays the release. The handoff is the moment these problems surface, and a well-prepared handoff, backed by i18n review, prevents the costly cycle of discover-fix-redo. Treating i18n as the localization team's responsibility to specify and the development team's responsibility to implement is the key to localization-ready source.

Use this skill when preparing for localization handoff, identifying i18n defects, defining handoff packages, or collaborating with developers on localization readiness. The goal is source content and code that localization can process without hitting preventable i18n walls.

## Core Rules

### Internationalize The Source Before Localizing

Localization assumes an internationalized source. Ensure i18n is done before handoff.

Internationalization means preparing the source so it can be adapted to multiple locales without engineering changes: externalizing user-facing strings from code, using locale-aware functions for dates, numbers, and currencies, supporting Unicode and varied character sets, designing flexible layouts that accommodate text expansion, and avoiding hard-coded locale assumptions. If these are not done, localization will hit walls: strings cannot be translated because they are in code, layouts break because text expands, characters corrupt because encoding is wrong. i18n is a prerequisite, not a parallel task. The localization team must specify i18n requirements, and the development team must implement them before handoff.

A source that is not internationalized cannot be cleanly localized; it can only be worked around.

### Externalize All User-Facing Strings

Hard-coded strings are the most common i18n defect. Ensure all user-facing strings are externalized.

Every string the user sees, in UI labels, messages, dialogs, tooltips, errors, and documentation, must be moved to external resource files that translators can edit without touching code. Hard-coded strings in source code are invisible to localization tools and ship untranslated. Audit the source for hard-coded strings before handoff, using string extraction tools and manual review. Verify that the externalized resources are complete and that no user-facing text remains in code. This is the foundation of localizable software.

A single hard-coded error message ships in the source language to every locale.

### Design Layouts For Text Expansion

Translated text expands, often significantly. Design layouts that accommodate expansion.

English to European languages commonly expands 30 to 40 percent; some strings double. Fixed-width containers, hard-coded pixel sizes, and non-wrapping text controls truncate translated text. Design layouts with flexible sizing, text wrapping, and dynamic containers. Avoid truncation through ellipsis that hides translated content. Test layouts with pseudo-localized, lengthened text to find the surfaces that cannot expand. Layout defects found after localization require engineering and often re-translation to fit.

A layout that fits English perfectly will fail German or Finnish; design for expansion from the start.

### Use Locale-Aware Functions For Conventions

Dates, times, numbers, currencies, and units must follow locale conventions. Use locale-aware functions.

Do not hard-code formats such as MM/DD/YYYY or assume decimal points and comma separators. Use locale-aware formatting functions provided by the framework or libraries, which render conventions correctly per locale. Verify that the functions are actually invoked with the correct locale, not just available. Check sorting and comparison, which vary by locale and script. Hard-coded conventions feel broken to local users and can cause functional errors, such as misparsed dates.

Locale-awareness is not automatic; it must be implemented and verified per locale.

### Support Unicode And Varied Character Sets

Translated text uses characters outside the source script. Support Unicode throughout.

Ensure the entire stack, from database to API to UI, uses Unicode, typically UTF-8, and handles multibyte characters correctly. Verify that storage preserves characters, that APIs transmit them without corruption, that search and sort handle them, and that fonts render them. Test with accented Latin, CJK, Cyrillic, Arabic, and other scripts relevant to the target locales. Character handling defects corrupt text and are often invisible until non-source characters are used.

A stack that assumes ASCII will corrupt most of the world's languages.

### Avoid Concatenation Of Translatable Fragments

Concatenated fragments break under translation. Avoid them in the source.

Building strings by joining fragments, such as "Hello " + name + ", you have " + count + " messages", fails because word order, agreement, and phrasing differ across languages. The fragments cannot be reassembled correctly in many locales. Use complete strings with placeholders, such as "Hello {name}, you have {count} messages", and allow translators to reorder the placeholder positions as needed. Pseudo-localization reveals concatenation defects. Concatenation is among the hardest defects to fix after localization because it requires re-engineering the string structure.

Never concatenate translatable fragments; always use complete strings with placeholders.

### Define A Complete Handoff Package

A clean handoff package enables smooth localization. Define and assemble it.

The handoff package includes: the source files in localizable formats, with strings externalized; reference materials such as context, screenshots, and glossaries; the localization kit specifying scope, locales, deadlines, and special instructions; any tools or access needed to process the files; and contact points for questions. An incomplete package forces the localization team to chase missing context, delaying work and risking errors. Define a handoff checklist and verify completeness before transfer.

A handoff is a contract; its completeness determines whether localization can proceed without friction.

### Review The Source For Localization Readiness

Before handoff, review the source for localization readiness. Do not assume it is ready.

Conduct an i18n review: check for hard-coded strings, inflexible layouts, hard-coded conventions, encoding issues, and concatenation. Run pseudo-localization to reveal defects. Document findings and work with development to resolve them before handoff. A source review catches problems when they are cheap to fix, rather than after localization has invested in translating content that will need rework. The review also identifies content that needs context or explanation for translators.

Localization readiness is verified, not assumed; a review prevents inheriting a defective source.

### Establish A Feedback Loop With Development

i18n is ongoing, not one-time. Establish a feedback loop with development.

As localization proceeds, new i18n defects will surface: a string that cannot be translated, a layout that breaks, a convention that is wrong. Feed these back to development promptly, with clear reproduction and impact, so they can be fixed in the source and propagated to all locales. Establish channels and responsibility for this feedback. Without a loop, defects recur in future releases because the source was never fixed.

## Common Traps

### Localizing Without Internationalizing

A source not internationalized forces workarounds and rework; i18n is a prerequisite.

### Hard-Coded Strings In Code

Strings in code are invisible to localization and ship untranslated.

### Fixed Layouts That Cannot Expand

Translated text expands; hard-coded sizes truncate it and require engineering fixes after localization.

### Hard-Coded Locale Conventions

Fixed date, number, and currency formats feel broken and can cause functional errors in other locales.

### Assuming ASCII Character Handling

A stack that assumes ASCII corrupts non-source scripts; support Unicode throughout.

### Concatenating Translatable Fragments

Joined fragments cannot be reassembled correctly across languages; use complete strings with placeholders.

### Incomplete Handoff Package

Missing files, context, or references force the localization team to chase information and risk errors.

### No Source Review Or Feedback Loop

Without review and feedback, i18n defects recur across releases because the source is never fixed.

## Self-Check

Before accepting a localization handoff or declaring a source localization-ready, verify:

- The source has been internationalized, with i18n treated as a prerequisite to localization, not a parallel task.
- All user-facing strings are externalized to resource files, with no hard-coded strings remaining in code.
- Layouts are designed for text expansion, with flexible sizing and wrapping, verified against pseudo-localized text.
- Locale-aware functions are used for dates, times, numbers, currencies, and units, and are invoked with the correct locale.
- Unicode is supported throughout the stack, with multibyte and non-source characters handled and rendered correctly.
- No translatable fragments are concatenated; complete strings with placeholders are used throughout.
- The handoff package is complete, including source files, references, context, the localization kit, and access.
- A source review for localization readiness was conducted, with defects resolved before handoff.
- A feedback loop with development is established for i18n defects discovered during localization.
- The source can be localized without hitting preventable i18n walls, and the handoff enables smooth processing.
