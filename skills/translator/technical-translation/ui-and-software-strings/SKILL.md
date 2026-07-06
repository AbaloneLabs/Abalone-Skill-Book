---
name: ui_and_software_strings.md
description: Use when the agent is translating or localizing software UI strings error messages and interface text, handling context-poor isolated segments, managing character limits and layout constraints, preserving placeholders variables and markup, or ensuring localized strings work correctly when assembled into the running product.
---

# UI And Software Strings

Software user interface strings are the hardest translation unit because they are small, isolated, and context-poor, yet they must work precisely when assembled into a running product. A button label, a menu item, an error message, or a form hint is translated as a fragment, but it appears in context next to other elements, inside variable-length layouts, concatenated with variables, and read by users who act on it instantly. The translator rarely sees the running interface, so they must infer context from terse strings and metadata. Errors propagate invisibly: a truncated label, a placeholder broken by translation, a concatenated sentence that no longer parses, or a button whose translated text overflows the layout. UI string translation is a discipline of defensive rendering, where the translator protects placeholders, respects constraints, anticipates concatenation, and demands context that is often missing.

Use this skill when translating or localizing software UI strings, error messages, interface labels, or any extracted software text, or when ensuring localized strings function correctly in the product. The goal is to produce localized strings that are accurate, clear, constrained correctly, and safe to assemble into the running interface.

## Core Rules

### Demand And Infer Context For Isolated Strings

UI strings arrive with little context. The translator's first task is to obtain or reconstruct enough context to translate safely.

Request screenshots, design files, string IDs, comments from developers, and access to a build where possible. When context is not provided, infer it from the string itself, from neighboring strings, from the product type, and from the string ID. Note ambiguity, such as a word that could be a noun or verb, a label or an action, and query it rather than guessing. A button labeled Save could be a verb or a noun depending on context, and the translation differs.

Maintain a query log for strings that need context, and push for developer comments in the source files to help future localization.

### Preserve Placeholders Variables And Markup

Software strings contain placeholders, variables, and markup that the system replaces at runtime. These must be preserved exactly, or the product breaks.

Placeholders such as percent-s, curly-brace variables, and HTML or markup tags must appear in the translation exactly as in the source, in the right position for the target language's syntax. Do not translate placeholder names, reorder variables in ways that break indexing, or remove markup. Where the source uses numbered placeholders to allow reordering, keep the numbering valid. Where concatenation joins multiple strings with variables, ensure the target grammar works when assembled, because a sentence split across concatenated strings may not parse in the target.

Broken placeholders and markup cause runtime errors, visible to users as raw code or crashed screens.

### Respect Character Limits And Layout Constraints

UI strings occupy fixed or flexible space. Respect the constraints, because overflow truncates or breaks the layout.

Identify character limits, which may be specified per string or implied by the UI element. Buttons, tabs, and menu items have tight limits; dialog text has more room but still fits a layout. Plan for text expansion, because translations are often longer than the source, and condense where needed without losing meaning. For languages with long words or no spaces, watch for elements that cannot wrap and will overflow.

Where a translation cannot fit, flag it and provide a shorter alternative, and request layout adjustment where condensation harms clarity.

### Handle Concatenation And Sentence Structure

Software often builds sentences by concatenating strings and variables at runtime. This is dangerous for translation, because target grammar may not assemble the same way.

A source pattern such as You have plus count plus new messages works in English but fails in languages where the number affects the noun form, the word order differs, or agreement changes. Identify concatenated patterns and flag them to engineering, because they often need refactoring to use full sentences with placeholders rather than fragments. Where concatenation cannot be avoided, translate fragments so they assemble grammatically, and test the assembled result.

Silent acceptance of concatenation produces broken sentences in the target language that engineering never sees.

### Handle Plurals Gender And Agreement

Pluralization and gender rules differ across languages and break naive translation. Handle them according to the target language's grammar.

English has simple plural rules, one and other, but many languages have more forms, such as Arabic's six plural forms, or Slavic languages' complex rules. Some languages mark gender on nouns, adjectives, and verbs, so a string's translation may depend on the gender of an implied subject. Use the localization framework's plural and gender support, such as ICU MessageFormat, to provide all required forms. Do not assume the source's two-form plural system suffices.

Provide complete plural and gender variants, and flag where the framework or source does not support the target's requirements.

### Maintain Consistency Across The Interface

UI terminology must be consistent across the entire interface and with related materials. A feature named one way in the menu must be named the same in help, errors, and tooltips.

Apply a termbase across all strings. Coordinate with documentation and marketing localization so terms align. Track decisions about ambiguous terms so later strings follow earlier choices. Inconsistency in UI terminology confuses users who cannot find the feature the help text describes.

### Translate Error Messages For Actionability

Error messages are UI strings with a specific function: they tell users what went wrong and what to do. Translate them to remain actionable.

Preserve the error's meaning and any required codes or identifiers. Where the source error includes a suggested action, preserve the action clearly. Avoid translating technical codes or log identifiers that users may need to report. For user-facing errors, aim for clarity; for developer-facing errors, preserve technical precision. Match the tone the product establishes for errors, whether reassuring or neutral.

An error message that loses its meaning or suggested action leaves users unable to resolve the problem.

### Support Pseudo-Localization And Testing

UI strings should be tested in the running product. Support pseudo-localization and localization testing to catch issues before release.

Pseudo-localization, replacing source text with fake translated text that is longer and accented, catches layout and encoding issues early. Full localization testing, running the product with real translations, catches truncation, concatenation breaks, placeholder errors, and context mismatches. Provide translators or reviewers to support testing, because they can identify whether a displayed string is correct in context.

Strings that pass review in a spreadsheet can still fail in the product; testing is where real defects surface.

## Common Traps

### Translating Without Context

Isolated strings translated blind produce wrong word sense, wrong register, and wrong grammar in context.

### Breaking Placeholders And Markup

Altering or removing variables and tags causes runtime errors visible to users.

### Ignoring Character Limits

Translations that overflow truncate or break layouts, producing unprofessional and unusable interfaces.

### Accepting Concatenation Silently

Fragments that assemble in English often break in other languages; silent acceptance ships broken sentences.

### Assuming English Plural Rules

Two-form plural systems fail for languages with more forms; provide all required variants.

### Inconsistent Terminology Across UI And Docs

Different terms for the same feature across interface and help prevent users from finding what they need.

### Skipping Localization Testing

Strings correct in a spreadsheet can fail in the product; testing catches real defects.

## Self-Check

Before approving localized UI strings, verify:

- Context was obtained or inferred for each ambiguous string, with queries logged for missing context.
- Placeholders, variables, and markup are preserved exactly and positioned correctly for target syntax.
- Character limits and layout constraints are respected, with shorter alternatives provided and layout adjustments flagged where needed.
- Concatenated patterns are identified, flagged to engineering where they break target grammar, and translated to assemble correctly.
- Plural, gender, and agreement forms are provided for all rules the target language requires, using the framework's support.
- Terminology is consistent across the interface and with documentation and marketing, governed by a termbase.
- Error messages preserve meaning, codes, and suggested actions, with tone matched to the product.
- Pseudo-localization and localization testing are supported to catch layout, concatenation, and context issues.
- No string has been translated blind, truncated, or broken in a way that fails when assembled into the running product.
- The localized interface, used by a target user, would be clear, consistent, and functional.
