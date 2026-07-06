---
name: localization-pipeline-and-text-design.md
description: Use when the agent is designing for localization, building localization-friendly text and UI, planning the localization pipeline, or evaluating whether the game's text, UI layout, and content pipeline support efficient translation or produce text overflow, broken layouts, context-blind translations, and costly rework across multiple languages.
---

# Localization Pipeline and Text Design

Localization — adapting a game for multiple languages and regions — is often treated as a post-launch afterthought, but the decisions made during design determine whether localization is efficient and high-quality or a costly rework nightmare. The judgment problem is that text must be designed for translation (avoiding idioms, concatenation, and hardcoded assumptions), UI must accommodate text expansion (translated text is often longer), and the pipeline must provide translators context (so they translate meaning, not blind strings), and agents tend to miss this because English-centric design looks fine (it works in English) while breaking in other languages, and because the cost of localization-unfriendly design is invisible until translation begins. The harm is broken layouts, mistranslated text, ballooning localization costs, and delayed or abandoned localization that cuts off international audiences. This skill covers how to design for localization from the start, build a localization-friendly pipeline, and avoid the English-centric traps. The agent has latitude in the text, but the obligation to design for translation is not optional.

## Core Rules

### Design Text to Be Localization-Friendly From the Start

Text must be designed for translation from the start — avoiding idioms (that do not translate), avoiding concatenation (that breaks across languages with different word order), avoiding culture-specific references (that lack meaning elsewhere) — because retrofitting localization-friendliness after the text is written is costly and incomplete. The decision rule: write text with translation in mind (plain language, no idioms, no concatenation, parameterized variables), and review text for localization-friendliness during writing. Localization-unfriendly text produces poor translations and rework, because the text assumed an English-only context.

### Design UI to Accommodate Text Expansion

Translated text is often longer than the source — German and French can be 30-50% longer than English — and UI must accommodate this expansion (flexible layouts, scalable text, no fixed-width assumptions) or the translated text overflows and breaks the layout. The decision rule: design UI with flexible, scalable layouts that accommodate text expansion, avoid fixed-width text containers, and test layouts with the longest expected translations. Fixed-width UI breaks in translation, because the layout assumed the source language's length.

### Provide Translators Context to Translate Meaning, Not Blind Strings

Translators working from blind strings (text without context) produce poor translations — wrong tone, wrong meaning, wrong gender — because the string's context (who says it, when, to whom) is absent, and the pipeline must provide context (screenshots, character info, situation) so translators translate meaning. The decision rule: provide context with every localizable string (screenshots, character notes, situation description), and avoid delivering blind strings to translators. Blind-string translation produces mistranslations, because the context that determines meaning was absent.

### Use Parameterized Variables, Not Concatenation

String concatenation (assembling sentences from fragments) breaks across languages because word order, grammar, and agreement differ, and parameterized variables (with the variable embedded in a complete sentence) must be used instead, so each language can structure the sentence correctly. The decision rule: use parameterized variables embedded in complete sentences, never concatenate fragments, and provide pluralization and gender rules per language. Concatenated strings produce broken grammar in translation, because the fragments assumed the source language's structure.

### Plan the Localization Pipeline to Be Iterative and Parallel

The localization pipeline must be planned to be iterative and parallel — strings extracted continuously, translations parallel with development, updates handled smoothly — rather than a sequential afterthought, so localization keeps pace with development and does not delay launch. The decision rule: integrate localization into the development pipeline (continuous string extraction, parallel translation, update handling), and avoid treating localization as a final sequential step. Sequential localization delays launch, because it could not parallel with development.

### Avoid Hardcoded Assumptions About Language and Format

Hardcoded assumptions — text direction (left-to-right), date and number formats, name order, gender assumptions — break in languages and regions that differ, and the game must use locale-aware systems rather than hardcoded formats. The decision rule: use locale-aware systems for text direction, date and number formats, name handling, and avoid hardcoded assumptions about language structure. Hardcoded formats break in other locales, because they assumed the source region's conventions.

## Common Traps

### Idioms and Concatenation That Break in Translation

The team writes text with idioms, concatenation, or culture-specific references, and the text breaks or mistranslates in other languages. The trap is that the text reads well in the source language. The false signal is that the text is written. The harm is that the idioms do not translate (losing meaning), the concatenation breaks grammar (wrong word order), the culture-specific references lack meaning elsewhere, the translations are poor, and the localization requires costly rework, because the text was not designed for translation.

### Fixed-Width UI That Overflows in Translation

The team designs UI with fixed-width text containers sized for the source language, and the longer translated text overflows and breaks the layout. The trap is that the UI looks right in the source language. The false signal is that the layout works. The harm is that the translated text exceeds the container, the layout breaks (text clipped, overlapping, misaligned), the UI is unusable in the translated language, and the localization requires UI rework, because the layout assumed the source length.

### Blind-String Translation Producing Mistranslations

The team delivers strings to translators without context, and the translators produce mistranslations because they lack the context that determines meaning. The trap is that the strings are the text. The false signal is that translation is proceeding. The harm is that the translators guess at context, the translations are wrong (wrong tone, wrong meaning, wrong gender), the localized game reads poorly, the meaning is lost or distorted, and the localization quality is low, because the context was not provided.

### Concatenated Strings Breaking Grammar

The team assembles sentences from concatenated fragments, and the grammar breaks in languages with different word order or agreement. The trap is that concatenation is flexible in code. The false signal is that the sentences assemble correctly in the source. The harm is that the fragments assume the source language's structure, the translated fragments assemble into broken grammar (wrong word order, wrong agreement), the localized text is ungrammatical, and the localization reads poorly, because concatenation was used instead of parameterized variables.

### Sequential Localization Delaying Launch

The team treats localization as a final sequential step after development, and the localization cannot parallel with development, delaying launch in all localized regions. The trap is that localization comes after content completion. The false signal is that the process is orderly. The harm is that the localization starts only after the source is final, it cannot parallel with development, the localized launches are delayed, the international audience waits, and the localization is a bottleneck, because the pipeline was not designed for parallel iteration.

### Hardcoded Formats Breaking in Other Locales

The team hardcodes text direction, date and number formats, or name order for the source region, and these break in locales that differ. The trap is that the hardcoded format works in the source. The false signal is that the format displays correctly. The harm is that right-to-left languages break left-to-right hardcoded direction, date and number formats display wrong for regions that differ, name order is wrong for cultures that differ, and the localized game is broken in those locales, because hardcoded assumptions were used instead of locale-aware systems.

## Self-Check

- Is text designed for translation from the start (plain language, no idioms, no concatenation, no culture-specific references)?
- Does UI accommodate text expansion with flexible, scalable layouts tested against the longest translations?
- Are translators provided context (screenshots, character notes, situations) for every localizable string?
- Are parameterized variables used instead of concatenation, with per-language pluralization and gender rules?
- Is the localization pipeline iterative and parallel, integrated with development rather than a sequential afterthought?
- Are locale-aware systems used for text direction, dates, numbers, and names, avoiding hardcoded assumptions?
- Did I confirm the game's text, UI, and pipeline support efficient, high-quality localization across all target languages?
