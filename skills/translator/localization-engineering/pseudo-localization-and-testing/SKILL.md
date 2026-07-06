---
name: pseudo_localization_and_testing.md
description: Use when the agent is planning localization testing, running pseudo-localization to catch internationalization defects before translation, designing test cases for localized builds, or verifying that translated content renders and functions correctly across locales.
---

# Pseudo-Localization And Testing

Localization testing is where the localized product is proven to work, but too often it is treated as an afterthought, squeezed into the end of a release cycle when there is no time to fix what it finds. The most effective testing happens before translation begins, through pseudo-localization, which exposes internationalization defects when they are cheap to fix. Pseudo-localization artificially modifies the source text to simulate what translation will do: it lengthens strings, replaces characters with accented equivalents, wraps text in brackets, and injects concatenation and variable issues. Running the pseudo-localized build reveals whether the software can handle longer text, special characters, different encodings, and varied string structures, before any real translation investment. Then, after translation, localization testing verifies that the real translated content renders, functions, and fits correctly in each locale. Skipping pseudo-localization means discovering i18n defects late, when they require engineering rework. Skipping localization testing means shipping builds with truncated text, broken layouts, corrupted characters, and non-functional strings. Both are quality gates that protect the localized product from embarrassing and costly failures.

Use this skill when planning localization testing, running pseudo-localization, designing test cases, or verifying localized builds. The goal is to catch internationalization defects early and confirm that localized content renders and functions correctly before release.

## Core Rules

### Run Pseudo-Localization Before Translation

Pseudo-localization catches internationalization defects when they are cheapest to fix. Run it early.

Pseudo-localization transforms the source text to simulate translation effects: string lengthening to test layout capacity, character replacement with accented or non-Latin equivalents to test encoding and fonts, bracket wrapping to detect truncation and concatenation issues, and pseudo-translation of variables to detect hard-coded concatenation. Running the pseudo-localized build reveals whether the software breaks under these conditions. Defects found now, such as a text box that cannot accommodate longer strings or a function that corrupts accented characters, are fixed in the source code before translation begins, at low cost. Defects found after translation require re-engineering and re-translation.

Pseudo-localization is the highest-return testing investment in a localization program.

### Test For String Length And Layout Expansion

Translated text is often longer than source, especially English to European languages. Test for expansion.

Pseudo-localization lengthens strings to simulate expansion, typically 30 to 40 percent. Run the build and check every UI surface: do labels fit, do buttons truncate, do dialogs overflow, do tables break, do menus wrap badly. Identify the surfaces that cannot accommodate expansion and fix them in the source, through resizable layouts, text wrapping, or dynamic sizing. Hard-coded pixel widths and fixed-size containers are common culprits. Length defects found after translation require both engineering fixes and often re-translation to fit new constraints.

English is compact; assuming target text will fit the same space is a recurring failure.

### Test Encoding And Character Handling

Translated text uses characters outside the source script, including accented Latin and non-Latin scripts. Test encoding.

Pseudo-localization replaces characters with accented equivalents and, for fuller testing, with non-Latin scripts such as CJK or Cyrillic. Run the build and check whether characters render correctly, whether databases and APIs accept and return them, whether search and sort work, and whether no corruption or mojibake occurs. Encoding defects often stem from hard-coded character sets, missing UTF-8 support, or legacy systems that strip high bytes. These defects are invisible until non-source characters are used, so they must be tested explicitly.

A build that corrupts accented characters will corrupt entire scripts; find this before translation.

### Detect Concatenation And Variable Issues

Concatenation and variable handling break under translation. Detect these with pseudo-localization.

Pseudo-localization wraps strings in brackets and modifies variables to reveal how strings are assembled. If a localized build shows mismatched brackets, missing text, or variables that display as literals, the software is concatenating strings in ways that break translation, such as joining sentence fragments that need different word order, or embedding variables in positions that shift across languages. Concatenation defects are among the hardest to fix after translation because they require re-engineering the string structure. Pseudo-localization finds them early.

Never concatenate translatable fragments; use complete strings with placeholders.

### Design Localization Test Cases For Each Locale

After translation, test the real localized builds. Design test cases for each locale.

Test cases should cover: that all strings are translated and none are missing or left in the source language; that translated text fits and does not truncate; that special characters render correctly; that locale-specific elements such as dates, numbers, currencies, and calendars are correct; that bi-directional and complex scripts display and function; that input methods work for the locale; and that locale-specific functionality such as address formats or name handling is correct. Design test cases that exercise these per locale, and execute them on the localized build. A locale is not tested by checking that strings are translated; it is tested by verifying the build works for that locale's users.

### Test In Context Not Just In Files

Translated strings must be tested in the running product, not just in resource files. Test in context.

A string may be correct in the resource file but wrong in context: it may truncate in a button, overlap a label, conflict with another string in a shared dialog, or be grammatically wrong when assembled with a variable. Testing in context, in the running localized build, catches these. Provide testers with the running build or screenshots, not just the files. Context testing is especially important for UI strings, where space and assembly constraints dominate.

A string that passes file-level review can fail in the product; context testing catches it.

### Verify Locale Conventions And Functionality

Localization includes locale-specific functionality, not just translated text. Verify conventions.

Check that dates, times, numbers, currencies, units, calendars, address formats, phone formats, and name formats follow the locale's conventions. Check that locale-specific features such as first-day-of-week, week numbering, or paper sizes are correct. These are often governed by locale settings in the operating system or framework; verify they are applied. A build with correct text but wrong locale conventions feels broken to local users.

### Use Native Testers For Linguistic Verification

Linguistic issues in the running build are best caught by native testers. Use them.

Native testers can spot translations that are wrong in context, awkward phrasing, register mismatches, and cultural issues that non-native testers miss. They can also verify that locale conventions feel natural. Where native testers are not available for every locale, use in-country review by client or partner resources. Testing without native input risks shipping linguistically correct-but-inappropriate strings.

## Common Traps

### Skipping Pseudo-Localization

Without pseudo-localization, i18n defects are found late, after translation investment, requiring costly rework.

### Assuming Target Text Fits Source Layout

Translated text expands; hard-coded sizes truncate it. Test for expansion before translation.

### Not Testing Encoding With Non-Source Characters

Encoding defects are invisible until non-source characters are used; test explicitly.

### Concatenating Translatable Fragments

Concatenated fragments break under translation; pseudo-localization reveals the breakage.

### Testing Strings In Files Not In Context

A correct file-level string can truncate or conflict in the product; test in the running build.

### Ignoring Locale Conventions

Correct text with wrong dates, numbers, or formats feels broken to local users.

### Testing Without Native Input

Non-native testers miss linguistic and cultural issues in context; use native or in-country testers.

### Squeezing Testing Into The End Of Release

Late testing finds defects there is no time to fix; test early and often.

## Self-Check

Before releasing a localized build, verify:

- Pseudo-localization was run before translation, and the source build was fixed for length, encoding, and concatenation defects.
- String length and layout expansion were tested, with hard-coded sizes identified and fixed.
- Encoding and character handling were tested with accented and non-Latin characters, with no corruption.
- Concatenation and variable issues were detected via pseudo-localization and resolved by using complete strings with placeholders.
- Localization test cases were designed and executed per locale, covering translation completeness, fit, special characters, and locale conventions.
- Strings were tested in the running product, not just in files, with context issues such as truncation and conflicts caught.
- Locale conventions including dates, numbers, currencies, calendars, and formats were verified for each locale.
- Native or in-country testers provided linguistic verification for context and cultural appropriateness.
- No i18n defect discovered post-translation required re-engineering that could have been caught by pseudo-localization.
- The localized build renders, functions, and feels correct for each locale's users.
