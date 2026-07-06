---
name: character_set_and_encoding.md
description: Use when the agent is choosing between simplified and traditional Chinese characters, selecting the correct regional variant for Taiwan Hong Kong or mainland China, handling Japanese kanji shinjitai and Korean hanja, converting legacy encodings such as Shift-JIS Big5 EUC-KR GB2312 or GBK to Unicode, diagnosing mojibake and character corruption, managing Unicode normalization, or preventing han unification rendering mismatches across Chinese Japanese and Korean locales.
---

# Character Set And Encoding

Chinese, Japanese, and Korean text lives on top of a character-set and encoding foundation that is older, more fragmented, and more politically charged than anything in Latin-script workflows. The same conceptual character can be represented by different Unicode code points across the three languages because of han unification, or by the same code point rendered with a different glyph depending on the locale font. Legacy encodings such as Shift-JIS, Big5, EUC-KR, GB2312, and GBK still circulate in source files, and a single misjudged conversion can turn a deliverable into mojibake. And the choice between simplified and traditional Chinese is not stylistic: it addresses different markets, carries regional identity, and using the wrong set can alienate the intended audience or signal ignorance of the market. Translators and reviewers tend to treat character set and encoding as invisible plumbing that just works, but in CJK it is a first-class source of defect. A translation can be linguistically perfect and still be wrong because it uses mainland simplified characters for a Taiwan audience, because a Big5 file was read as Shift-JIS, or because a character that should render as its Japanese form renders as its Chinese form due to a locale-font mismatch. These problems are often invisible until delivery, and they are expensive to fix because they touch every character in the file.

This skill applies when you are selecting or verifying the character set for a CJK deliverable, when you are converting or validating encodings, when you are diagnosing corruption, and when you are preventing rendering mismatches caused by han unification. The objective is text whose character set matches the target locale, whose encoding is sound and consistent end to end, and whose characters render in the correct regional form.

## Core Rules

### Resolve The Character Set Requirement Before Translating

The simplified-versus-traditional decision, and the regional variant within each, must be settled before any translation begins, because it determines the entire character inventory of the deliverable.

Simplified Chinese is standard for mainland China and Singapore; traditional Chinese is standard for Taiwan, Hong Kong, and Macao. The two sets are not interchangeable, and a reader in one market often finds the other set slow or alien to read. Within traditional Chinese, Taiwan and Hong Kong have regional glyph and terminology differences, so the locale tag should specify the region, not just traditional. Japanese uses kanji in the shinjitai standard, which differs from both Chinese sets, and Korean uses hangul with hanja in limited, often formal or disambiguating, contexts. Confirm the exact target locale with the client or brief, record it in the project metadata, and treat any source whose locale is unspecified as a blocking question rather than a guess. Translating first and discovering the wrong set later means redoing the work.

### Use Unicode As The Pipeline Standard And Convert Legacy Encodings Deliberately

The deliverable pipeline should standardize on Unicode, typically UTF-8, and any legacy-encoded source must be converted with verification, not assumption.

Legacy encodings are still common in older files, exports from legacy systems, and certain regional tools. Reading a Big5 file as Shift-JIS, or a GB2312 file as EUC-KR, produces mojibake that can look superficially plausible and pass a careless review. When you receive a file whose encoding is not declared, detect it rather than guess, convert it to UTF-8, and verify the converted text against a known-good sample or a native check. Record the original encoding and the conversion in case a defect appears later. Never silently re-save a file in a different encoding, because the moment of re-save is where corruption is introduced. If a downstream system requires a legacy encoding, perform that conversion as the final export step only, and keep the Unicode master as the source of truth.

### Detect And Diagnose Mojibake At Intake

Mojibake, the garbage characters produced by encoding mismatch, must be caught at intake because every downstream step propagates the corruption.

Learn to recognize the signatures: sequences of accented Latin characters where CJK characters should be, repeated replacement characters, or characters from the wrong script entirely. When you see these, stop and fix the encoding before translating, because translating corrupted text embeds the corruption in the translation memory and the termbase. Distinguish mojibake from genuinely rare or variant characters, which are legitimate but may need font support. A useful diagnostic is to re-open the file with the suspected correct encoding and check whether the text becomes readable; if it does, the original read used the wrong encoding. Build an intake check that flags suspicious character distributions so corruption is caught before work begins.

### Manage Han Unification And Locale-Specific Glyphs

Han unification means the same Unicode code point may represent a character that should render differently in Chinese, Japanese, and Korean, and this requires locale-aware font handling.

Unicode unified many CJK characters that share an abstract shape into single code points to save space, leaving the locale-specific glyph difference to fonts. As a result, a character stored correctly can still render with the wrong regional form if the rendering environment uses a font from the wrong locale. The Chinese, Japanese, and Korean forms of such characters differ in stroke shape, proportion, and sometimes radical form, and a Japanese reader seeing the Chinese glyph, or vice versa, perceives it as wrong even when the underlying text is correct. Specify locale-appropriate fonts in the rendering environment, tag content with the correct language so font selection follows language, and verify rendered output for glyph correctness. Do not assume that correct Unicode implies correct display.

### Apply Consistent Unicode Normalization

CJK text can represent the same visual character through different Unicode sequences, and inconsistent normalization causes search, matching, and consistency failures.

Some characters have composed and decomposed forms, and Korean hangul in particular has precomposed syllable forms alongside conjoining-jamo sequences. If a file mixes forms, identical-looking text will fail to match in search, in translation memory, and in terminology checks, producing false inconsistency reports and missed leverage. Decide on a normalization form, typically NFC for composed forms, apply it consistently across the deliverable, and confirm that the CAT tool and termbase use the same form. Re-check normalization after any conversion or merge, because those operations can reintroduce mixed forms.

### Verify Character Integrity At Every Pipeline Stage

Encoding and character-set integrity must be verified at each handoff, because corruption introduced at any stage propagates to all later stages.

Check integrity at intake, after any conversion, after CAT import and export, after merge or concatenation, and at final delivery. For each stage, confirm that the file is valid UTF-8, that the expected characters are present, that no replacement characters or wrong-script intrusions have appeared, and that the character set matches the target locale. A stage that passes characters through unchanged is not guaranteed; some tools re-encode silently, and some databases mangle CJK on round-trip. Treat the pipeline as a series of integrity gates rather than a single end-to-end assumption.

## Common Traps

### Treating Chinese As A Single Set

Simplified and traditional address different markets, and within traditional the regions differ; an unspecified Chinese locale is a blocking question, not a default.

### Guessing The Encoding Of Legacy Files

Reading Big5 as Shift-JIS or GB2312 as EUC-KR produces plausible-looking mojibake; detect encoding and verify the conversion rather than guessing.

### Re-Saving Silently In A New Encoding

The moment of re-save is where corruption is introduced; convert deliberately with verification and keep the Unicode master.

### Translating Corrupted Text

Mojibake caught after translation has already polluted the translation memory and termbase; detect and fix corruption at intake before any work.

### Assuming Correct Unicode Means Correct Display

Han unification leaves regional glyph differences to fonts; a correctly stored character can render in the wrong locale form without locale-aware font handling.

### Ignoring Normalization

Mixed composed and decomposed forms break search, matching, and terminology checks; apply consistent normalization and re-check after conversions.

### Trusting The Pipeline End To End Without Stage Checks

Tools re-encode silently and databases mangle CJK on round-trip; verify integrity at each handoff rather than assuming safe passage.

## Self-Check

Before delivering or approving CJK text on character-set and encoding grounds, verify:

- The exact target locale and character set, including simplified or traditional and the regional variant, is confirmed and recorded in project metadata before translation.
- The pipeline standardizes on Unicode, typically UTF-8, and any legacy-encoded source was detected, converted deliberately, and verified against a known-good sample.
- Mojibake was checked for at intake and resolved before translation, with no corrupted text propagated into translation memory or the termbase.
- Han-unified characters are rendered with locale-appropriate fonts, and content is language-tagged so font selection follows language.
- A single Unicode normalization form, typically NFC, is applied consistently, and normalization was re-checked after any conversion or merge.
- Character integrity was verified at every pipeline stage, including intake, conversion, CAT import and export, merge, and final delivery.
- No silent re-save or re-encoding has introduced corruption, and the Unicode master is preserved as the source of truth.
- The character inventory of the deliverable matches the target locale, with no characters from an incompatible set or region.
