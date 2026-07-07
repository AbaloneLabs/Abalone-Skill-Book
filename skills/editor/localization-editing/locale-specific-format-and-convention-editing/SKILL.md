---
name: locale_specific_format_and_convention_editing.md
description: Use when the agent is editing localized content for correct dates times numbers currencies units of measure names and addresses per locale, managing text expansion and contraction in translation, handling right-to-left and script considerations, ensuring legal and regulatory conventions, or maintaining consistency within a locale.
---

# Locale-Specific Format And Convention Editing

Localization is not only about language; it is about the conventions that make content feel native and function correctly in a specific place. Dates, times, numbers, currencies, units, names, and addresses are formatted differently across locales, and getting them wrong causes confusion, errors, and lost trust. Editors miss this layer because formats feel trivial next to language and culture, and because source-content formats often pass through translation unchanged. The harm is concrete and sometimes severe: a date displayed in the wrong order causes missed deadlines, a currency shown without clarity causes pricing errors, a unit mismatch causes product or safety failures, and a name or address that violates local convention signals that the brand does not understand its market. The editor's task is to ensure every locale-specific element follows the correct convention for its target locale and remains consistent throughout.

## Core Rules

### Enforce Locale-Correct Date, Time, And Number Formats

Dates and times are a primary source of localization error because formats vary widely and ambiguously. The editor must confirm that dates follow the target locale's convention, including order of day, month, and year, separators, leading zeros, and whether the month is numeric or named, and that the calendar system is appropriate, since not all locales use the Gregorian calendar by default. Times must reflect the locale's convention for twelve- versus twenty-four-hour clocks, separators, and time-zone labeling. Numbers must follow the locale's decimal and thousands separators, which are often the reverse of the source, and grouping conventions. The editor treats each format as a locale-specific rule, not a stylistic preference, and verifies it throughout the content rather than only where it appears prominently.

### Verify Currency, Units Of Measure, And Legal Conventions

Currency must be presented with the correct symbol or code, placement, decimal precision, and rounding for the locale, and where multiple markets share a language, the specific currency must be unambiguous. Units of measure must match the locale's system: metric versus imperial, and locale-specific variants. A recipe in cups that reaches a metric market without conversion is unusable; a technical specification in inches that reaches a market expecting millimeters can cause failure. Legal and regulatory conventions, such as required disclaimers, tax-inclusive versus tax-exclusive pricing, privacy notices, and consumer-rights statements, vary by jurisdiction and must be present and correct for each locale. The editor checks that all such conventions are applied, not merely translated.

### Localize Names, Addresses, And Personal Data Formats

Names and addresses follow cultural conventions that a literal translation violates. Person-name fields must accommodate the locale's name structure, including order of given and family names, multiple surnames, patronymics, and the presence or absence of middle names. Address formats differ in field order, required elements, postal-code structure, and whether street-level detail is expected. Forms that assume a source-locale name or address structure will reject valid entries from other locales. The editor reviews all name and address handling, including placeholders and examples, to ensure they reflect the target locale's conventions and do not impose the source structure.

### Manage Text Expansion And Contraction In Translation

Translated text rarely matches the source length. Romance languages often expand by twenty to thirty percent from English; some languages contract; character-based scripts behave differently again. This expansion and contraction affects every constrained surface: button labels, navigation, headings, table cells, form fields, and metadata with character limits. The editor anticipates expansion when designing or editing source content, leaving room in layouts and keeping source strings concise, and checks localized strings against the available space. Truncation, overlap, or broken layouts in the localized product are editing failures, not merely design problems. The editor collaborates with design and engineering to accommodate variable text length, including flexible containers and responsive handling.

### Handle Right-To-Left And Complex Script Considerations

Right-to-left languages, such as Arabic and Hebrew, require mirrored layouts, correct bidirectional text handling, and attention to punctuation, numbers, and embedded left-to-right content. Complex scripts, including those with conjuncts, contextual forms, or combining characters, require fonts and rendering that support them fully. The editor verifies that RTL content is not merely translated but correctly oriented, that UI elements mirror appropriately, and that scripts render without broken characters or misplaced diacritics. Assuming that a left-to-right layout can host RTL text without adjustment produces unusable, unprofessional results.

### Maintain Consistency Within Each Locale

Within a single locale, conventions must be applied uniformly. If a date format is chosen, every date in that locale's content must follow it; if a currency presentation is set, it must hold across pricing, tables, and examples. Inconsistency within a locale is as damaging as wrong convention, because it signals carelessness. The editor establishes the locale's conventions, records them in a locale-specific style guide, and enforces them across all assets for that market. Consistency is maintained through documented rules, not through memory.

### Coordinate Across Locales Without Imposing One Model

While each locale has its own conventions, the editor must also manage the relationships among locales so that the overall product remains coherent. This means not forcing one locale's conventions onto others, recognizing that shared-language markets may still differ, and ensuring that the source content is written in a way that is localization-friendly, avoiding hard-coded formats, locale-specific idioms in examples, and inflexible layouts. The editor thinks both within each locale and across the set of locales, balancing local correctness with global coherence.

## Common Traps

### Passing Source Formats Through Translation Unchanged

Letting dates, numbers, and units from the source survive into the localized content produces wrong or ambiguous formats. The trap is assuming translation covers formatting. The editor enforces locale-specific formats deliberately.

### Ambiguous Date Formats

A date like 03/04/2026 means different things in different locales, causing missed deadlines and confusion. The trap is relying on a single numeric format. The editor uses unambiguous, locale-correct formats and spells out months where risk is high.

### Forgetting Currency And Unit Conversion

Showing source currency or units in a target market causes pricing errors and usability failure. The trap is treating currency and units as text rather than locale data. The editor verifies conversion and locale presentation.

### Ignoring Text Expansion In Constrained UI

Buttons, navigation, and metadata that fit the source may overflow when localized. The trap is designing only for source length. The editor plans for expansion and tests localized strings in layout.

### Treating RTL As Merely Translated

Translating into an RTL language without mirroring layout and handling bidirectional text produces broken, unusable interfaces. The trap is underestimating RTL complexity. The editor verifies full RTL adaptation.

### Imposing Source Name And Address Structures

Forms and examples that assume source-locale structures reject valid data from other locales. The trap is universalizing the source model. The editor adapts structures to each locale's reality.

### Inconsistent Conventions Within A Locale

Mixing date or number formats within a single locale's content signals carelessness. The trap is editing assets in isolation. The editor documents and enforces locale conventions across all assets.

## Self-Check

- Do all dates, times, and numbers follow the target locale's conventions, including order, separators, calendars, and clocks, with no source formats passed through?
- Is currency presented with correct symbol or code, placement, precision, and rounding, unambiguous across shared-language markets?
- Are units of measure converted to the locale's system, with no imperial or metric mismatches that could cause usability or safety failures?
- Are legal, regulatory, tax, and disclaimer conventions present and correct for the target jurisdiction?
- Do name and address fields, placeholders, and examples reflect the locale's structure rather than the source's?
- Has text expansion and contraction been anticipated, with localized strings tested against constrained surfaces like buttons, navigation, and tables?
- For right-to-left and complex scripts, has layout been mirrored, bidirectional text handled, and rendering verified for broken characters?
- Are conventions documented in a locale-specific style guide and applied consistently across every asset for that locale?
- Has the source content been written to be localization-friendly, avoiding hard-coded formats and inflexible layouts?
- Have shared-language markets been treated as distinct rather than assumed to share conventions?
