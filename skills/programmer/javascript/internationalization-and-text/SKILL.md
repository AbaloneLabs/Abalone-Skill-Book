---
name: javascript_internationalization_and_text.md
description: Use when the agent is handling internationalization (i18n), localization (l10n), multi-language text, Unicode and emoji in JavaScript, sorting and comparing strings across locales, pluralization and gender rules, number/currency/date formatting by locale, right-to-left (RTL) and bidirectional (BiDi) text, message extraction and translation catalogs, or is diagnosing "the sort order is wrong", "the currency symbol is misplaced", "pluralization shows '1 items'", "the emoji is split", or RTL layout mirroring issues. Covers Intl APIs, Unicode correctness, locale-aware formatting, and the design of translatable text surfaces.
---

# Internationalization And Text In JavaScript

Internationalization (i18n) is treated as a late-stage polish step, and that ordering is why most applications ship with subtly broken text handling for any user outside the developer's own locale. The assumption that underlies the bugs is that text is ASCII, that "character" means "what you see", that one space equals one space, that comparison is byte order, that a number formats the same everywhere, and that plurals are "one" and "other". None of these hold. JavaScript strings are UTF-16 code-unit sequences; user-perceived characters are grapheme clusters that can be many code points; comparison and sorting depend on locale; number, currency, and date formatting vary by locale and numbering system; pluralization has six categories in some languages and none in others; and bidirectional text mixes left-to-right and right-to-left runs that break naive string slicing and CSS direction assumptions. The judgment problem is to treat every text surface as locale- and Unicode-dependent from the start, to use the `Intl` APIs instead of hand-rolled formatting, and to design strings so they can be translated without code changes.

Agents typically hard-code formats (`"$" + amount.toFixed(2)`, `new Date().toLocaleDateString()` with no locale, `"1 item" / "n items"`), concatenate translated fragments, and assume `String.prototype.localeCompare` is enough. Each of these works for the developer's locale and fails for the next one: the dollar sign may belong after the amount or be a different symbol entirely, the date may need a non-Gregorian calendar, the plural may need a dual form, and concatenated fragments cannot be reordered by translators. The remedy is to make the locale explicit, to use `Intl` for all formatting, to externalize messages with placeholders and plural rules, and to test with non-default locales from the beginning.

## Core Rules

### Make The Locale Explicit Everywhere, Never Implicit

`toLocaleString()`, `toLocaleDateString()`, and `localeCompare()` without an explicit locale argument use the runtime's default locale, which in a browser is the user's OS locale and on a server is the host's locale — neither of which is necessarily the locale the content or the user actually wants. Pass the locale (and options) explicitly at every formatting call so behavior is deterministic and correct for the audience.

- A server formatting a report for a French user must pass `'fr-FR'`, not rely on the server's locale.
- Negotiate locale with `Intl.DateTimeFormat.supportedLocalesOf([...candidates])` when you have a preference list, so a missing locale falls back deliberately rather than silently.
- Store the user's locale as an explicit application setting; do not infer it from `navigator.language` alone for server-rendered content.

### Use Intl.NumberFormat For Numbers And Currency

Hand-rolled number formatting (`toFixed`, string concatenation with a symbol) gets the decimal separator, grouping, currency symbol position, sign display, and numbering system wrong outside the developer's locale. `Intl.NumberFormat` handles all of these and supports compact notation, units, and percent.

- Currency: `new Intl.NumberFormat('de-DE', { style: 'currency', currency: 'EUR' }).format(1234.56)` yields `1.234,56 €` — symbol after, comma decimal. Never concatenate a currency symbol manually.
- Use `currencyDisplay: 'narrowSymbol'` or `'code'` where appropriate, and be aware some locales use a non-Latin numbering system (`ar-EG` uses Eastern Arabic digits) unless overridden.
- For grouping-sensitive parsing, round-trip carefully: a formatted string is for display, not for arithmetic; keep the numeric value canonical and format on output.

### Use Intl.DateTimeFormat And Specify TimeZone

Dates are doubly locale-dependent: the format (order, separators, 12/24 hour, calendar) depends on locale, and the instant displayed depends on the time zone. `toLocaleDateString()` without a time zone uses the runtime's local zone, which on a server is wrong for every user not in that zone. Always pass `timeZone` for any date shown to a user, and prefer storing and transmitting instants in UTC/ISO.

- `new Intl.DateTimeFormat('en-GB', { dateStyle: 'full', timeZone: 'Asia/Tokyo' }).format(date)` formats a UTC instant for a Tokyo viewer in British English.
- Some locales use non-Gregorian calendars (`fa-IR` Persian, `th-TH` Buddhist, `ja-JP` with `era`); specify `calendar` if you need to override.
- Never do date arithmetic on formatted strings; format only for display from a canonical instant.

### Sort And Compare With Intl.Collator

`Array.prototype.sort()` without a comparator sorts by code unit (uppercase before lowercase, accents after base letters, emoji scattered by code point) — useless for users. `localeCompare` without options is locale-default and sensitivity-default. Use `Intl.Collator(locale, { sensitivity, numeric, caseFirst })` for correct, deterministic, locale-aware ordering and equality.

- `numeric: true` sorts `"item2"` before `"item10"` (natural number sort); without it, `"item10"` sorts first.
- `sensitivity: 'base'` treats `a`, `A`, `ä` as equal; `'accent'` distinguishes accents; `'case'` distinguishes case; `'variant'` (default) distinguishes all.
- Cache the `Collator` instance; constructing one per comparison in a sort is wasteful.

### Handle Pluralization With Intl.PluralRules And Message Catalogs

English has "one" and "other"; Arabic has six forms; some languages have a dual or paucal. A `count === 1 ? singular : plural` ternary is wrong for most of the world. Use `Intl.PluralRules(locale).select(count)` to pick the right message form, or use an i18n library (ICU MessageFormat) that embeds plural syntax in the message.

- Externalize the message with placeholders and plural selectors: `{count, plural, one {# item} other {# items}}` lets translators handle their language's forms and reorder the placeholder.
- Never concatenate two translated fragments (`translatedYouHave + ' ' + count + ' ' + translatedItems`); translators cannot reorder or re-grammar a concatenated sentence. Use one message with placeholders.

### Count And Slice By Grapheme Clusters, Not Code Units

`str.length` counts UTF-16 code units; `"😀".length === 2`. `str.slice(0, n)` can split a surrogate pair, a base letter from its combining accent, or an emoji-ZWJ sequence. For "characters as the user sees them" — counting a text field, truncating a preview — use `Intl.Segmenter(locale, { granularity: 'grapheme' })`.

- `new Intl.Segmenter().segment(str)` yields segments you can count and slice without splitting a user-perceived character.
- For word boundaries (selection, word count), use `granularity: 'word'`; for sentence boundaries, `'sentence'`.

### Design Strings For Translation From The Start

Translators need whole sentences with placeholders, context for ambiguous words, and the ability to reorder. Designing strings as concatenations of fragments defeats all three.

- Externalize every user-facing string into a catalog keyed by a stable id; never inline translatable text in logic.
- Provide context (a comment) for short or ambiguous keys (`"Name"` could be a noun or a verb).
- Keep placeholders positional and named so translators can reorder: `"Welcome, {name}"` not `"Welcome, " + name`.

### Handle Bidirectional (RTL) Text And Direction Explicitly

RTL languages (Arabic, Hebrew) flow right-to-left; mixing RTL and LTR runs (a Latin word inside an Arabic sentence) requires the Unicode bidirectional algorithm and explicit direction markers. In the DOM, set `dir="auto"` on user-generated content so the browser detects direction, and use logical CSS properties (`margin-inline-start`) rather than physical (`margin-left`) so layout mirrors with `dir`.

- Do not assume string slicing preserves visual order across a direction boundary.
- Use the `dir` attribute and CSS logical properties; test with at least one RTL locale.

## Common Traps

### Implicit Locale In toLocaleString

`new Date().toLocaleDateString()` on a server uses the server's locale; the same code produces different output per deployment. Pass the locale and time zone explicitly.

### Code-Unit Slicing Splits Emoji And Combining Marks

`tweet.slice(0, 140)` can cut an emoji or split `é` into `e` + `´`. Use `Intl.Segmenter` for user-facing truncation.

### Hand-Rolled Currency And Number Formatting

`"$" + amount` puts the symbol in the wrong place for many locales and uses the wrong decimal/grouping separators. Use `Intl.NumberFormat` with `style: 'currency'`.

### English-Only Plural Logic

`count === 1 ? 'item' : 'items'` is wrong for Slavic, Semitic, and many other languages. Use `Intl.PluralRules` or ICU MessageFormat plurals.

### Concatenated Translatable Fragments

`translate('You have') + ' ' + n + ' ' + translate('messages')` cannot be reordered or re-grammared by a translator. Use one message: `translate('You have {n} messages', { n })`.

### localeCompare Without Options For Stable Sort

`arr.sort((a, b) => a.localeCompare(b))` is locale-default and lacks `numeric`, so `"file10"` sorts before `"file2"`. Pass locale and `{ numeric: true }`, and cache the `Collator`.

### Forgetting TimeZone In Date Formatting

`toLocaleString()` without `timeZone` uses the runtime zone; a UTC instant shows the wrong wall-clock time for users elsewhere. Always pass `timeZone`.

## Self-Check

- [ ] Every formatting call (`NumberFormat`, `DateTimeFormat`, `Collator`, `PluralRules`) passes an explicit locale and the relevant options, and the user's locale is an explicit application setting rather than inferred from the runtime.
- [ ] Numbers and currency are formatted with `Intl.NumberFormat` (correct separators, symbol position, numbering system); no manual symbol concatenation remains.
- [ ] Dates are formatted with `Intl.DateTimeFormat` and an explicit `timeZone`; instants are stored/transmitted in UTC and formatted only for display.
- [ ] Sorting and equality use `Intl.Collator` with `numeric`/`sensitivity` set, and the collator is cached rather than constructed per comparison.
- [ ] Pluralization uses `Intl.PluralRules` or ICU MessageFormat plurals, not an English `=== 1` ternary, and works for at least one multi-form locale.
- [ ] User-facing character counting and slicing use `Intl.Segmenter` (grapheme granularity), so no surrogate pair or combining-mark sequence can be split.
- [ ] All user-facing strings are externalized as whole messages with named placeholders and translator context, and no two translated fragments are concatenated.
- [ ] RTL/BiDi content uses `dir="auto"` and logical CSS properties, and the layout has been tested with at least one RTL locale.
