---
name: locale_and_rtl_support.md
description: Use when the agent is formatting dates, times, numbers, or currencies, pluralizing messages, building layouts that must support right-to-left languages, extracting or externalizing user-facing strings, sizing UI for translated text, or updating locale data and translations in a product that ships to multiple regions.
---

# Locale And RTL Support

Localization is the discipline of not assuming your user's conventions. A date like "03/04/2025," a number like "1,000.50," a price, a plural, a layout direction — each of these is a locale-specific decision that a program makes either deliberately or by accident. When made deliberately, the product adapts to each user's region and language. When made by accident, the product silently ships the developer's own conventions to everyone, and the cost is paid far away: a date misread as March instead of April, a transaction amount off by a thousand, a sentence that crashes the grammar of a language with different plural rules, a right-to-left interface that is unusable because it was built left-to-right.

Agents tend to treat localization as a translation problem bolted on at the end, after the product works in the developer's locale. By then the assumptions are baked in: hardcoded strings, fixed layouts that assume left-to-right, pluralization written as "if count == 1," date and number formatting done by hand. Each is a small, invisible choice that becomes expensive to undo. The judgment problem is to build for locale from the start — to externalize the locale-dependent decisions, to use locale-aware formatting and pluralization libraries, to design layouts that survive both direction changes and text-length changes, and to keep the locale data current, because the conventions themselves evolve.

## Core Rules

### Never Hardcode Locale-Dependent Values

The foundational rule is that anything that varies by locale must not be hardcoded. This includes user-facing strings (messages, labels, errors), but also the less obvious values: date and time formats, number formats, currency symbols and positions, list separators, and even the calendar or week numbering. Hardcoding any of these ships the developer's locale to every user and makes each future locale a patch.

Externalize locale-dependent values into resource bundles keyed by locale, so that adding a locale does not require code changes. The code references keys; the bundles provide the values per locale. This separation also enables translators to work without touching code and enables locale updates to ship without a code release. A product whose user-facing strings and formats live in code rather than resources is not localizable; it is merely translatable piecemeal, with high error cost.

### Format Dates, Times, Numbers, And Currency With Locale-Aware Libraries

Date, time, number, and currency formatting are full of locale-specific rules that no one should reimplement. The separator (1,000.50 versus 1.000,50), the symbol position, the currency code versus symbol, the first day of the week, the 12-hour versus 24-hour clock, the calendar (Gregorian, Hijri, etc.), and the timezone all vary by locale and are easy to get subtly wrong.

Use the platform's locale-aware formatting libraries (ICU, Intl, platform-specific formatters) and pass the user's locale explicitly. Never build a date string by concatenating a day, a separator, a month, and a year — the separator, the order, and the padding are all locale-specific. Never format a number with a hardcoded thousands separator or decimal point. Never attach a currency symbol by string concatenation; the position and the symbol depend on the locale and the currency, and some locales put the symbol elsewhere or use a different code.

A subtle but critical point: the locale for formatting and the currency are independent. A user in France viewing a USD price should see the French number formatting with the USD currency, not US formatting. Decide each axis deliberately rather than letting one imply the other.

### Pluralize With The Full Plural Rules, Not An If-Else

Pluralization is where naive localization breaks grammar. English has two plural forms (one, other), so "if count == 1" almost works — but many languages have more. Arabic has six plural forms. Russian and Polish have three. Some languages have no singular/plural distinction at all. A pluralization written as "one item" versus "N items" produces grammatically broken output in any language whose plural rules differ from English's.

Use a pluralization system that supports the full Unicode CLDR plural categories (zero, one, two, few, many, other) and that selects the correct form per locale. The message bundle provides a translation for each plural category the locale uses, and the library selects based on the count and the locale. This is not over-engineering; it is the minimum for correct grammar in most of the world's languages. The same applies to gender and other agreement forms where the language requires them.

### Design Layouts For Both Direction And Length

Two physical realities of translation break layouts that were not designed for them: text length changes and reading direction changes. A translated string is often 30-50% longer than the English original (German and Russian are notoriously long), and a right-to-left language like Arabic or Hebrew mirrors the entire reading axis. Layouts built to the pixel length of one language, or hardwired to left-to-right, break in others.

Design for both:

- **Length** — use flexible containers, text wrapping, and relative sizing rather than fixed widths sized to one language. Allow text to grow without overflowing or truncating. Test layouts with the longest translations, not just the source language.
- **Direction** — build layouts in terms of logical properties (start/end, inline-start/inline-end) rather than physical ones (left/right), so the layout mirrors automatically when the direction flips. Modern CSS and UI frameworks support logical properties; use them instead of hardcoding left and right margins, paddings, and positions.
- **Icons and directional cues** — icons that imply direction (back, forward, undo) must mirror in RTL; icons that are direction-neutral should not. Test with the direction flipped rather than assuming symmetry.

A layout that works only in LTR and only at the source language's length is not internationalized; it is a local product with extra languages awkwardly attached.

### Keep Locale Data And Translations Current

Locale conventions are not static. CLDR (the Common Locale Data Repository) updates its plural rules, date formats, currency symbols, and territory data regularly. Currencies change (a new symbol, a redenomination). Translations drift as the product's strings change. A product that snapshots its locale data once and never updates it ships increasingly stale conventions and incorrect translations.

Treat locale data as a dependency that needs maintenance:

- **Update the locale data library periodically** to pick up corrected and new conventions, the same way you update other dependencies, with attention to behavior changes.
- **Keep translations in sync with source strings.** When a source string changes, its translations become stale or wrong. A translation pipeline that surfaces changed and new strings, and that tracks completion per locale, prevents shipping half-translated or mistranslated interfaces.
- **Handle missing translations deliberately.** When a translation is missing, decide the fallback (the source language, another related locale, or a marked placeholder) and make it visible in testing so missing translations are caught before release, not discovered by users.

### Test With Real Locales, Not Just The Source

Localization bugs are invisible when testing only in the source locale. A hardcoded string, a broken plural, an overflowing layout, or an un-mirrored direction only appears when the product is run in another locale. Make testing in at least one additional locale — ideally one with different conventions (different plural rules, RTL, longer text) — part of the development loop, not a final QA step.

Particularly valuable test locales: one with multi-form plurals (Arabic or Russian), one RTL (Arabic or Hebrew), one with long translations (German), and one with a different number/date format. Running the product in these surfaces the majority of localization defects early, when they are cheap to fix.

## Common Traps

### Hardcoding User-Facing Strings

Embedding messages, labels, and formats directly in code ships the developer's locale to everyone and makes translation a code-change exercise. Externalize all user-facing strings and locale-dependent values into resource bundles keyed by locale.

### Hand-Building Date, Number, And Currency Formats

Concatenating a date or number from parts with hardcoded separators produces formats correct in one locale and wrong in others. Use locale-aware formatting libraries and pass the locale explicitly, treating the formatting locale and the currency as independent decisions.

### English-Only Pluralization

"If count == 1" pluralization breaks grammar in languages with more than two plural forms. Use a CLDR-aware pluralization system that selects among zero, one, two, few, many, and other per locale.

### Layouts Sized To The Source Language

Fixed widths and pixel-precise layouts that fit the source language overflow or truncate when text grows in translation. Use flexible sizing and test with the longest translations.

### Hardcoding Left And Right Instead Of Start And End

Physical left/right positioning does not mirror in RTL, producing broken layouts for Arabic and Hebrew users. Use logical start/end properties so the layout flips with the direction automatically.

### Stale Locale Data And Translations

Locale data and translations that are snapshotted once and never updated ship increasingly wrong conventions and translations. Treat locale data as a maintained dependency and keep translations in sync with source-string changes.

### Testing Only In The Source Locale

Localization defects are invisible when testing only in the source language. Test in at least one locale with different conventions (different plurals, RTL, longer text) as part of the development loop.

## Self-Check

- [ ] All user-facing strings and locale-dependent values (dates, numbers, currencies, formats) are externalized into resource bundles keyed by locale, not hardcoded in code.
- [ ] Dates, times, numbers, and currencies are formatted with locale-aware libraries, the user's locale is passed explicitly, and the formatting locale and the currency are treated as independent decisions.
- [ ] Pluralization uses a CLDR-aware system supporting all plural categories (zero, one, two, few, many, other) per locale, not an English if-else.
- [ ] Layouts use flexible sizing that survives translated text length (often 30-50% longer) and logical start/end properties that mirror automatically in RTL.
- [ ] Directional icons mirror in RTL and direction-neutral icons do not; layouts were tested with the direction flipped.
- [ ] Locale data is treated as a maintained dependency and updated periodically, and translations are kept in sync with source-string changes with missing-translation fallbacks made visible in testing.
- [ ] The product is tested in at least one locale with different conventions (multi-form plurals, RTL, longer text, different number/date formats), not only in the source locale.
- [ ] No user-visible value depends on the developer's local conventions rather than the user's locale.
