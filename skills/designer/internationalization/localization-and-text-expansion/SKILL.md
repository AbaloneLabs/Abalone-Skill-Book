---
name: localization_and_text_expansion.md
description: Use when the agent is designing layouts, components, buttons, forms, labels, or navigation that must accommodate translated text, multiple languages, text expansion and contraction, line breaking, date and number formatting, currency, pluralization, or content that varies in length across locales.
---

# Localization And Text Expansion

A layout designed in English is designed for one of the most compact languages in common use. When the same interface is translated, text routinely grows by thirty to fifty percent, and sometimes far more. Buttons overflow, labels wrap awkwardly, columns collide, and navigation collapses. Treating localization as a translation step applied after design guarantees broken layouts, because the design was sized for a language that no longer exists in the product.

Use this skill before finalizing layouts, component dimensions, button styles, form labels, navigation, tables, dialogs, or any text-bearing surface that will ship in more than one language. The goal is to prevent the agent from hardcoding English-sized space, assuming words break where English breaks, or treating translated text as a problem for engineers to patch at runtime.

## Core Rules

### Design For Expansion From The First Pixel

Translated text expands, and the expansion is uneven. German and Finnish grow long; French and Spanish grow moderately; some languages, such as Chinese, can be shorter but behave very differently in line breaking. As a working baseline, leave room for roughly thirty to forty percent growth for short labels and up to double or triple for very short strings, because short strings expand the most in percentage terms.

Design containers, not fixed strings:

- let buttons grow horizontally or wrap rather than truncating;
- avoid fixed-width columns that assume a specific label length;
- prefer flexible, content-driven layouts over pixel-locked grids;
- test with the longest realistic translation, not the English source.

### Never Hardcode Or Concatenate Strings

Translation breaks the moment text is assembled from fragments. A sentence built as `"Welcome, " + name + "!"` cannot be translated into languages with different word order, because the name may need to appear elsewhere, or the greeting may need different grammar depending on the name. Plurals, gender, and possession vary by language in ways English does not predict.

Use complete sentences with placeholders, and let the translation system reorder them. Handle plurals through the locale's plural rules, not by appending an "s." Never split a sentence across multiple elements and assume the order survives translation.

### Account For Line Breaking And Word Length Differences

Languages break lines differently. English breaks at spaces; some languages have no spaces; some have very long compound words; some change meaning if broken in the wrong place. A heading that fits on two lines in English may need four in German, or break mid-word in an unhelpful place in another script.

Plan for:

- headings and labels that grow taller when they wrap;
- hyphenation and line-break rules that differ by language;
- long unbreakable tokens such as URLs, identifiers, or compound words;
- vertical space in cards, rows, and list items that absorbs extra lines.

### Separate Date, Time, Number, And Currency Formatting

Dates, times, numbers, and currencies are not universal. The order of day, month, and year; the separator; the twelve- versus twenty-four-hour clock; the decimal and thousands separators; the currency symbol's position; and negative number representation all vary by locale. Hardcoding any of them produces confusing or wrong information.

Use locale-aware formatting for every value, and design layouts that survive variation: dates that grow longer, currencies with different symbol positions, and numbers with different separators. Never assume the format used in the source locale is correct elsewhere.

### Handle Pluralization And Grammatical Variation

English has simple plural rules, but many languages have more: dual forms, paucal forms, or different forms depending on the number's final digits. "1 item" and "2 items" is not a universal pattern. Some languages also vary adjectives or verbs by gender, which English ignores entirely.

Route all count-based strings through the locale's plural rules, and never build plurals by string manipulation. Where gender or case matters, design the content model to carry that information rather than assuming English grammar.

### Design Translation-Friendly Content

Some content is hard or impossible to translate well: idioms, jokes, culturally specific metaphors, sports references, slang, and wordplay. These either become nonsense or require costly rewrites in each locale. In core interface text, prefer literal, clear language that translates cleanly, and reserve idiomatic or playful language for content that can be locally adapted.

Avoid embedding text in images, because translated text cannot replace it without redrawing the image. Keep text as live, selectable, translatable strings.

### Leave Room For Review And Context

Translators work from strings, not screens. A label such as "Open" could mean a verb, an adjective, or a status, and each translates differently. Provide context: where the string appears, its maximum length, whether it is a verb or noun, and whether it sits next to other text. Without context, translations are guesses.

## Common Traps

### Sizing For English And Hoping For The Best

Designing buttons, columns, and labels to fit English exactly guarantees breakage in longer languages, usually discovered late.

### Concatenating Sentences From Fragments

Building sentences from parts assumes English word order and grammar, which translation destroys.

### Appending An S For Plurals

English-style pluralization fails in languages with different plural forms, producing grammatically wrong text.

### Embedding Text In Images

Text baked into an illustration or banner cannot be translated without recreating the asset for every locale.

### Hardcoded Date, Number, And Currency Formats

Assuming the source locale's format is universal produces confusing or incorrect values for other audiences.

### Idioms And Wordplay In Core Text

Playful English phrasing often becomes meaningless when translated, adding cost and degrading quality.

### Translating Strings Without Context

Translators given bare strings guess at meaning, producing accurate-but-wrong translations such as "Open" rendered as a verb where a status was intended.

## Self-Check

- [ ] Containers, buttons, columns, and labels were sized for translated expansion, not for the English source length.
- [ ] No sentence is built by concatenating fragments; all translatable text uses complete strings with placeholders.
- [ ] Plurals use the locale's plural rules, not appended suffixes or string manipulation.
- [ ] Dates, times, numbers, and currencies use locale-aware formatting rather than hardcoded source-locale formats.
- [ ] Headings, labels, and list items absorb extra wrapped lines without overlapping or clipping.
- [ ] No meaningful text is embedded in images in a way that prevents translation.
- [ ] Core interface text avoids idioms, slang, and wordplay that translate poorly.
- [ ] Translatable strings are accompanied by context: location, role, maximum length, and grammatical function.
- [ ] The layout was tested with the longest realistic translation, not only the source language.
- [ ] Line breaking was considered for scripts with no spaces, long compounds, or restrictive break rules.