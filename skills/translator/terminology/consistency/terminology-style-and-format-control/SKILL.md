---
name: terminology_style_and_format_control.md
description: Use when the agent is managing terminology, glossary choices, style consistency, formatting, placeholders, numbers, units, dates, UI strings, technical terms, names, and repeated language across a translation or localization project.
---

# Terminology Style And Format Control

Terminology and format errors can make a translation unusable even when sentences are fluent. A product term translated two ways, a placeholder changed accidentally, a decimal separator converted incorrectly, or a legal term approximated loosely can create confusion, broken software, compliance risk, or loss of trust.

Use this skill when translating technical material, product content, UI strings, documentation, legal or business documents, academic text, subtitles, tables, forms, marketing campaigns, or any multi-part translation where consistency matters. The focus is project control: terms, formats, repeated phrases, protected elements, and style decisions.

## Core Rules

### Build Or Follow A Term Base

Identify important terms before translating at scale. A term base may be formal or informal, but the translator needs a consistent record.

Track:

- source term;
- approved target term;
- forbidden alternatives;
- definition;
- domain;
- part of speech;
- context example;
- capitalization;
- plural or inflected forms;
- notes about when not to translate;
- owner or source of approval.

Do not invent new translations for key terms in each section. If no glossary exists, create a small working glossary for repeated or risky terms.

### Distinguish Term Consistency From Mechanical Repetition

Some terms must remain consistent: legal concepts, product features, UI labels, medical terms, safety warnings, defined contract terms, and API names. Other words may vary naturally for readability.

Ask:

- Is this a defined term?
- Does the reader need to recognize the same concept across sections?
- Is the term part of a UI, law, product, procedure, or brand system?
- Could variation create ambiguity?
- Is variation acceptable because the word is general language?

Do not use synonyms for controlled terms merely to make prose elegant. Do not force repetition of general words when natural language would vary.

### Protect Placeholders, Code, And Variables

Localization often contains elements that must not be translated or altered.

Protect:

- variables such as `{name}` or `%s`;
- HTML or Markdown tags;
- URLs;
- file paths;
- command names;
- keyboard shortcuts;
- code snippets;
- API fields;
- product SKUs;
- email addresses;
- phone numbers;
- tracking parameters when required;
- segment IDs;
- line breaks with functional meaning.

Check that placeholders appear in the target text in the correct number and order. If target grammar requires reordering, preserve the placeholder exactly while moving it safely.

### Control Numbers, Dates, Units, And Currency

Numeric format is locale-sensitive and high-risk.

Check:

- decimal separators;
- thousands separators;
- date order;
- time format;
- time zone;
- measurement units;
- currency symbols and codes;
- negative numbers;
- percentages versus percentage points;
- ranges;
- phone number formatting;
- postal codes;
- legal or accounting precision.

Do not convert units or currency unless the brief allows it. If conversion is needed, preserve the source value or note the conversion method when appropriate.

### Maintain Document Structure And Cross-References

A translation must often preserve structural relationships. Section numbers, table references, footnotes, figure labels, list levels, and cross-references should remain accurate.

Check:

- headings;
- numbering;
- footnotes;
- endnotes;
- captions;
- table labels;
- figure references;
- internal links;
- form field labels;
- page references;
- annex or appendix names;
- bullets and indentation;
- source-target alignment if bilingual review is needed.

Do not break a document's navigational system while improving prose.

### Respect Brand, Product, And Legal Naming Rules

Some names should not be translated. Others have approved localized forms. Legal entities, products, campaigns, and trademarks may have strict rules.

Check:

- company names;
- product names;
- feature names;
- slogans;
- trademarks;
- legal entity suffixes;
- government bodies;
- laws and regulations;
- academic institutions;
- certifications;
- standards.

If an official translation exists, use it. If none exists and the name is material, preserve the original and add explanation only if the brief supports it.

### Keep Style Consistent Across Segments

Style includes formality, voice, punctuation, sentence length, active or passive preference, treatment of the reader, and translation of recurring phrases.

Maintain decisions about:

- pronouns and forms of address;
- imperative versus polite request;
- active versus passive construction;
- sentence-ending style;
- capitalization;
- punctuation spacing;
- quotation marks;
- list punctuation;
- headings;
- button labels;
- error message tone;
- warning language.

For UI and support content, consistency often matters more than literary variety.

### Handle Length And Layout Constraints

Target languages expand or contract. UI, subtitles, forms, packaging, charts, and presentations may have space limits.

Check:

- button width;
- mobile line breaks;
- subtitle reading speed;
- character limits;
- label truncation;
- table cell width;
- slide layout;
- packaging space;
- right-to-left or left-to-right layout;
- text embedded in images.

If a faithful translation does not fit, revise functionally and note the compromise when needed.

### Track Decisions For Review

A translation project becomes fragile when decisions live only in the translator's memory. Keep notes for repeated terms, unresolved choices, source ambiguity, risky adaptations, and reviewer questions.

Useful notes include:

- glossary entries;
- style decisions;
- unresolved terms;
- assumptions about locale;
- source errors;
- adaptation rationale;
- high-risk terms needing review;
- formatting decisions.

This is especially important when multiple agents, translators, reviewers, or future updates may touch the content.

## Common Traps

### Translating The Same Term Differently

In legal, technical, product, and medical content, inconsistent terms can create real misunderstanding. Track and reuse approved terms.

### Translating Placeholders

Changing `{user_name}`, `%d`, `<br>`, command names, or variables can break software or templates. Protect them before editing.

### Converting Formats Without Permission

Changing units, currency, dates, or decimal format may be right for localization but wrong for legal or source-aligned translation. Follow the brief.

### Using Elegant Synonyms For Controlled Labels

UI labels, headings, defined terms, and safety instructions may need repeated wording. Variety can reduce usability.

### Ignoring Text Expansion

A translation that is accurate but does not fit the interface, subtitle timing, or packaging can fail in production.

### Losing Cross-References

A changed heading, table label, or section number can make the document hard to use. Check references after editing.

## Self-Check

Before treating terminology and format control as complete, verify:

- A glossary or working term list exists for repeated, technical, legal, product, or high-risk terms.
- Approved terms are used consistently where consistency matters.
- General-language variation has not been over-controlled unnecessarily.
- Placeholders, variables, tags, code, URLs, file paths, and IDs are preserved exactly.
- Numbers, dates, time zones, units, currency, percentages, and ranges follow the brief and locale rules.
- Names, trademarks, legal entities, product names, and official titles follow approved translations or preservation rules.
- Headings, numbering, captions, tables, footnotes, and cross-references remain accurate.
- Style choices for formality, pronouns, punctuation, capitalization, and recurring phrases are consistent.
- Length and layout constraints have been checked where relevant.
- Unresolved terminology, source ambiguity, and risky decisions are documented for review.
