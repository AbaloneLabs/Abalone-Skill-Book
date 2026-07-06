---
name: cjk_terminology_and_locale.md
description: Use when the agent is selecting terminology for Chinese Japanese or Korean targets, distinguishing mainland China Taiwan Hong Kong and Singapore usage, handling keigo honorifics and speech levels, choosing between native Sino-Japanese and loanword renderings, managing katakana and gairaigo, applying locale-specific number date currency and name formats, or ensuring that register politeness and address forms match the target locale's conventions.
---

# CJK Terminology And Locale

Terminology in Chinese, Japanese, and Korean is not a matter of finding the dictionary equivalent and moving on. Each language is spoken across locales that diverge sharply in vocabulary, and each carries systems of register, honorific, and address form that change the entire shape of a sentence depending on who speaks to whom. Mainland China, Taiwan, Hong Kong, and Singapore use different words for the same concept, not just different characters; Japanese selects between native Yamato words, Sino-Japanese compounds, and katakana loanwords based on nuance and register; Korean chooses speech levels and honorifics that encode the social relationship between speaker and addressee. A translator who treats CJK terminology as a lookup table produces text that is grammatically correct but socially wrong: too casual for a formal context, too stiff for a marketing message, addressed in a register that insults the reader, or peppered with vocabulary that marks it as belonging to the wrong region. These errors are invisible to a non-native reviewer and deeply visible to the target reader, and they accumulate across a deliverable until the whole text feels off. CJK terminology and locale work is therefore a discipline of context-sensitive choice, where the correct term depends on region, register, audience relationship, and domain, and where defaults borrowed from one locale fail in another.

This skill applies when you are choosing or reviewing terminology for CJK targets, when you are matching register and honorific level to the audience, when you are selecting among native and borrowed renderings, and when you are applying locale-specific formatting for numbers, dates, currency, and names. The objective is terminology and phrasing that fit the specific target locale, the correct register and politeness, and formatting that follows regional convention rather than a generic CJK default.

## Core Rules

### Resolve The Locale Before Resolving The Term

The same source concept can map to genuinely different target terms depending on the locale, so the locale must be fixed before terminology work begins.

Within Chinese, mainland China, Taiwan, Hong Kong, and Singapore diverge in everyday vocabulary, in technical terminology, and in conventions for borrowed words. A software term standard in mainland China may be unknown or carry a different sense in Taiwan, and the reverse is equally true. Japanese and Korean each have their own national standards bodies and industry glossaries, and a pan-CJK term is the exception, not the rule. Confirm the specific locale from the brief, select terminology from locale-appropriate reference materials and approved termbases, and flag any term whose locale fit is uncertain. Do not carry terminology across locales on the assumption that Chinese is Chinese or that a Japanese term from one domain fits another.

### Match Register And Politeness To The Audience Relationship

Register and honorification are not optional flourishes; they encode the social relationship between writer and reader, and choosing the wrong level is a substantive error.

Japanese keigo, with its sonkeigo respect forms, kenjougo humble forms, and teineigo polite forms, shifts the entire verb inventory of a sentence depending on whether the addressee is a customer, a colleague, or a subordinate. Korean speech levels, from the formal deferential to the intimate plain, similarly restructure sentence endings, and the honorific system marks the subject's status relative to the listener. Choosing a level that is too high sounds stiff and distancing in casual content; choosing a level that is too low sounds rude or presumptuous in formal or customer-facing content. Determine the audience relationship from the brief and the content type, apply the matching register consistently, and review especially carefully at boundaries where content type shifts, such as a marketing page that links to a legal notice.

### Choose Deliberately Among Native And Borrowed Renderings

Each CJK language offers multiple strategies for rendering a source concept, and the choice carries nuance that must be decided rather than defaulted.

Japanese frequently offers a native Yamato reading, a Sino-Japanese compound, and a katakana transliteration of the foreign term, and the three carry different tones: the native form can feel warm and traditional, the Sino-Japanese compound feels formal and often technical, and the katakana loanword feels modern and international but can feel shallow if overused. Korean similarly chooses among native words, Sino-Korean compounds, and English loanwords. Chinese generally prefers coinages and calques over transliteration, though some domains accept transliterated brand and technical terms. For each significant term, weigh the options against the content's tone and the audience's expectation, record the choice in the termbase, and apply it consistently. Reflexively reaching for the loanword because it is easy produces text that feels lazy and unlocalized.

### Maintain Terminology Consistency Within The Deliverable

Consistency is especially important in CJK because variant renderings of the same concept are easy to produce and hard for a non-native reviewer to detect.

Once a term is chosen, the same concept must use the same rendering throughout the deliverable, across segments and across documents in a set. Inconsistent terminology confuses readers, undermines translation memory leverage, and signals carelessness. Drive consistency through a maintained termbase that is enforced during translation and verified during review. When a legitimate synonym exists, decide whether variation is acceptable or whether one form is mandated, and document the decision. Treat terminology drift between related documents as a defect to be corrected, not a harmless difference.

### Apply Locale-Specific Formatting For Numbers Dates And Currency

Numbers, dates, currency, units, and names follow locale-specific conventions that must be applied deliberately, not inherited from the source.

Date formats differ across CJK locales, with year-month-day ordering common but with different separators and era usage; Japanese often uses the era-based calendar alongside the Gregorian; Korean and Chinese have their own conventions. Number grouping, decimal separators, and currency symbol or label placement vary. Personal names follow family-name-first order in all three languages, and the handling of Western names, middle names, and name order in mixed contexts must be decided per locale. For each formatted element, look up the target locale's convention, apply it consistently, and verify that automated formatting in the product uses the correct locale rather than a default. A date or name in the wrong format is an immediate signal of incomplete localization to a native reader.

### Validate Terminology Against Domain And Currency

Terminology correctness depends on the domain, and a term correct in general use may be wrong in a specialized field, so validate against domain references.

Technical, medical, legal, and financial domains in each CJK locale have established terminology that may differ from general-language dictionaries, and using the general term in a specialized context can be misleading or incorrect. Identify the domain, consult domain-specific glossaries and parallel texts, and have terminology reviewed by a subject-matter native reviewer where stakes are high. Currency is a related concern: financial and regulatory terms must match the locale's regulatory and accounting conventions, not a generic rendering. Treat domain validation as part of terminology work, not a separate afterthought.

## Common Traps

### Treating CJK Locales As Interchangeable

Mainland China, Taiwan, Hong Kong, Singapore, Japan, and Korea use different terms for the same concept; carrying terminology across locales produces regionally wrong text.

### Defaulting To A Single Register

Register and honorific level encode social relationship; defaulting to one level regardless of audience produces text that is rude, stiff, or inappropriately casual.

### Reaching For Loanwords Reflexively

Katakana and English loanwords are easy but overused they feel lazy and unlocalized; weigh native, compound, and borrowed options deliberately.

### Allowing Terminology To Drift

Variant renderings of the same concept confuse readers and waste translation memory; enforce a termbase and correct drift between documents.

### Inheriting Source Date Name And Number Formats

Dates, names, currency, and numbers follow locale conventions; inheriting source formats signals incomplete localization to native readers.

### Using General-Language Terms In Specialized Domains

A term correct in general use may be wrong in technical, medical, legal, or financial contexts; validate terminology against domain references.

### Skipping Native Subject-Matter Review

Non-native review cannot catch register, honorific, or domain-terminology errors; high-stakes CJK content needs native subject-matter validation.

## Self-Check

Before delivering or approving CJK terminology and locale handling, verify:

- The specific target locale is confirmed from the brief and terminology is drawn from locale-appropriate references, with no terms carried across locales on assumption.
- Register and honorific level match the audience relationship defined by the content type, applied consistently and reviewed at boundaries where content type shifts.
- Each significant term was chosen deliberately among native, compound, and borrowed options, with the choice recorded in the termbase.
- Terminology is consistent across the deliverable and across related documents, with drift corrected and acceptable synonyms documented.
- Numbers, dates, currency, units, and names follow the target locale's conventions, with automated formatting verified to use the correct locale.
- Domain-specific terminology has been validated against domain references or a native subject-matter reviewer for high-stakes content.
- No pan-CJK default or source-inherited format has been allowed to override the specific locale's conventions.
- A native reviewer has assessed the text for register, politeness, and locale fit, confirming it reads as intended for the specific target audience.
