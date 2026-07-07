---
name: localization_testing_and_linguistic_qa.md
description: Use when the agent is planning or running localization testing and linguistic QA, designing test cases for localized products, performing in-context linguistic testing, executing pseudo-localization, or verifying that translated content functions and reads correctly inside a running product rather than only in a translation file.
---

# Localization Testing And Linguistic QA

Translation that is correct inside a CAT tool or spreadsheet can still be wrong inside the product. Localization testing is the discipline of verifying localized content in its live context, where strings meet layout, fonts, concatenation, variables, navigation, and user flow. It catches the defects that pure translation review cannot see: a button label that overflows its container, a concatenated sentence whose fragments no longer agree grammatically, a placeholder that renders as raw code, a right-to-left screen where a number appears in the wrong order, a term that reads well in isolation but repeats awkwardly across a menu, or a translated string that is technically accurate but unusable because it was never seen in the place where users encounter it. Too many localization programs treat testing as an afterthought, something done by engineers who do not speak the language or skipped entirely because the translation "passed review." The result is products that ship with defects visible to every user in the target market but invisible to the team that built them, because no one with linguistic judgment ever looked at the running localized product. This skill exists to prevent that gap, by making localization testing a planned, structured, linguistically informed activity rather than a hopeful final glance.

Use this skill when planning or running localization testing, designing test cases for localized products, performing in-context linguistic review, executing pseudo-localization, or verifying that translated content works in a live product. The goal is to catch the defects that file-based review misses and to verify the localized product as users will experience it.

## Core Rules

### Test In Context, Not Just In Files

The foundational principle of localization testing is that a translation must be verified where users encounter it. A string in a translation file is an abstraction; the same string inside a button, a toast notification, a form label, or an error dialog is a concrete experience, and the two can diverge.

File-based review confirms that the target text corresponds to the source. It cannot confirm that the target text fits the button, that it does not collide with an adjacent string, that it reads correctly when concatenated with a variable, or that it appears in the right place at the right time. In-context testing closes this gap. Whenever feasible, linguists or trained testers should review the localized product in a running state, navigating the actual screens, triggering the actual states, and reading the strings in their real surroundings. Where full in-context testing is infeasible due to access or scale, provide screenshots, builds, or interactive previews that approximate context as closely as possible. The principle holds regardless of method: the closer the review environment is to the user's environment, the more defects it catches.

### Design Test Cases Around Where Localization Breaks

Localization testing is most valuable when its test cases target the points where translation and locale adaptation predictably fail. Generic "click through every screen" testing catches some defects but misses the structural ones. Design test cases deliberately around known break points.

High-value test areas include strings with embedded variables and placeholders, where the variable may expand, inflect wrongly, or appear in the wrong order; concatenated strings, where fragments assembled in code may not form a grammatical sentence in the target; strings with character limits, such as buttons, tabs, and menu items, where translated text overflows or truncates; date, time, number, and currency fields, where formats and separators may be wrong; right-to-left and bidirectional content, where mirroring, number direction, and punctuation placement go wrong; forms and input validation, where locale-specific data such as addresses and names may be rejected; images and icons containing embedded text or culturally specific symbols; and error and edge-case states that are hard to trigger but carry high consequence. Build a test case library that covers these categories for every locale, because they recur across products and are where defects concentrate.

### Run Pseudo-Localization Before Real Translation

Pseudo-localization is a technique that exposes localizability problems before any real translation is produced, by replacing source strings with artificially expanded, accented, and bracketed text. It is one of the highest-value, lowest-cost localization tests, and skipping it wastes the chance to fix problems while they are cheap to fix.

Pseudo-localization reveals whether strings can expand without breaking layout, whether special characters and diacritics render correctly in the product's fonts, whether concatenation and placeholders are handled correctly when the surrounding text changes, and whether the pipeline carries non-ASCII characters end to end without corruption. Run it early, during development, so that internationalization defects are caught before localization begins. A product that fails pseudo-localization is not ready to localize, and translating it anyway multiplies rework across every locale. Treat pseudo-localization as a gate: do not hand content to translators until the pseudo-localized product functions.

### Separate Linguistic Testing From Functional Testing, Then Coordinate

Localization testing has two overlapping but distinct dimensions, and conflating them produces incomplete coverage. Linguistic testing asks whether the translated content is correct, appropriate, and readable in context. Functional testing asks whether the localized product works, including layout, navigation, input handling, and integration.

Linguistic testers need language expertise and the termbase, style guide, and context to judge correctness. Functional testers need product and platform expertise to trigger states, verify flows, and check layout across devices. Neither alone is sufficient. A functional tester who does not speak the language can catch overflow and broken placeholders but cannot catch a mistranslation or a culturally inappropriate term. A linguistic tester can catch those but may miss a navigation bug or a form that rejects valid locale data. Coordinate the two so coverage is complete: linguistic testing verifies the words in context, functional testing verifies the behavior with localized content, and findings are reconciled into a single defect list.

### Test Every Locale, Including The Supposedly Easy Ones

A common and damaging assumption is that locales close to the source language need less testing. Teams test the right-to-left and double-byte locales carefully and give the European locales a light pass, assuming similarity to English guarantees safety. This assumption causes defects to ship.

Every locale has its own conventions, expansion patterns, and failure modes. German and French expand significantly and break layouts that English fits. Spanish has regional variants that a single rendering may not serve. Even locales that share a script with the source differ in date and number formats, quotation marks, capitalization rules, and sorting. Test every locale against the same structured test cases, scaled to its risk. The locale that is assumed safe is often the one where a defect reaches users unnoticed, because no one checked.

### Verify Locale-Specific Data And Formats End To End

Locale adaptation is not complete when the visible text is translated. Dates, times, numbers, currencies, addresses, names, calendars, and units must be verified end to end, from display through input through storage, because defects often hide at the boundaries between layers.

Check that dates and times display in the locale's format and that the calendar system and week start are correct where relevant. Check that numbers use the correct decimal and thousands separators and that currency symbols and positions are correct. Check that forms accept locale-specific addresses, names, and phone numbers and that the backend stores them without truncation or rejection. Check that sorting and searching behave correctly for the locale's alphabet or script. A product that displays the right format but rejects the user's input, or that stores data correctly but displays it wrong, is broken. Test the full path, not just the surface.

### Manage Defects With Clear Linguistic Versus Engineering Ownership

Localization testing generates defects that fall into different ownership categories, and unclear ownership is a primary reason defects go unfixed. Classify each defect so it reaches the right owner.

Linguistic defects, such as mistranslation, terminology deviation, or awkward phrasing, go to translators or reviewers for correction. Engineering defects, such as overflow, broken placeholders, concatenation failures, or hardcoded strings, go to developers. Design defects, such as layouts that cannot accommodate expansion or images with embedded text, go to designers. Ambiguous defects, where it is unclear whether the issue is linguistic or technical, need triage. A defect tracking system that lumps all localization issues into one bucket without category or owner ensures that engineering defects sit in a translator's queue unaddressed and linguistic defects sit with engineers who cannot judge them. Classify at logging time and route accordingly.

### Test With Realistic Content And Real User Paths

Testing with placeholder, lorem-ipsum, or minimal content hides defects that real content reveals. Test with realistic, representative content and along the paths real users take.

Realistic content means strings at realistic length, including the long ones that stress layouts. It means content that exercises the full range of states, including empty states, error states, long lists, and edge cases. Real user paths mean testing the journeys users actually take, including the ones that cross screens, trigger multiple strings in sequence, and expose concatenation and context problems that single-screen testing misses. Testing only happy paths with short strings produces false confidence. Stress the product the way users will, with the content and flows that will exist in production.

## Common Traps

### Treating Testing As Optional Because Translation Passed Review

File-based review cannot catch in-context defects. A translation that is correct in the file can overflow, concatenate wrongly, or read poorly in place. Testing is not redundant with review; it verifies a different layer.

### Skipping Pseudo-Localization To Save Time

Pseudo-localization catches internationalization defects before translation investment. Skipping it means discovering layout and pipeline problems after every locale is translated, when fixes are expensive.

### Assuming Similar Locales Need Less Testing

Locales close to the source still differ in formats, expansion, and conventions. The locale assumed safe is often where defects ship unnoticed.

### Letting Engineers Test Languages They Do Not Speak

Functional testers without language expertise catch layout bugs but miss mistranslation and cultural issues. Linguistic and functional testing must be coordinated, not collapsed into one.

### Testing Only Happy Paths With Short Strings

Minimal content and happy-path flows hide overflow, concatenation, and edge-case defects. Test with realistic content along real user journeys, including error and empty states.

### Logging All Localization Defects Into One Bucket

Defects without clear linguistic, engineering, or design ownership sit in the wrong queue and go unfixed. Classify and route at logging time.

### Verifying Display But Not Input Or Storage

A product that shows the right date format but rejects the user's date input is broken. Test locale-specific data end to end, from display through input through storage.

### Testing Translated Screenshots Instead Of The Running Product

Screenshots approximate context but cannot reveal interaction, flow, and dynamic assembly defects. Use the running product wherever feasible and reserve screenshots for infeasible cases.

## Self-Check

Before approving localized content for release based on testing, verify:

- In-context testing was performed, with linguists or trained testers reviewing strings in the running product or the closest feasible approximation, not only in translation files.
- Test cases were designed around known localization break points, including placeholders, concatenation, character limits, formats, right-to-left layout, forms, and edge-case states.
- Pseudo-localization was run before real translation, and the product passed it, confirming internationalization readiness.
- Linguistic testing and functional testing were both performed and coordinated, with neither dimension assumed to cover the other.
- Every locale was tested against structured test cases, including locales assumed similar to the source, with no locale given a light pass by assumption.
- Locale-specific data and formats were verified end to end, from display through input through storage, including dates, numbers, currencies, addresses, names, and sorting.
- Defects were classified by ownership, linguistic, engineering, or design, and routed to the correct owner with enough detail to fix.
- Testing used realistic content and real user paths, including long strings, error states, and cross-screen journeys, not only happy paths with minimal content.
- No defect was closed without verification in context, and no locale was released on file-based review alone.
- The localized product, used the way a target-market user would use it, functions and reads correctly in every tested locale.
