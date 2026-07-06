---
name: cjk_text_handling.md
description: Use when the agent is translating or reviewing Chinese Japanese or Korean text, handling character set choices such as simplified versus traditional Chinese, managing kanji hanja and hangul, dealing with word boundaries and segmentation, or addressing line breaking and punctuation rules specific to CJK scripts.
---

# CJK Text Handling

Chinese, Japanese, and Korean (CJK) scripts present translation challenges that Latin-script workflows often overlook. These scripts do not use spaces between words in the same way, they involve character set choices with political and regional dimensions, they have distinct punctuation and line-breaking rules, and they interact with fonts, encodings, and tools in ways that can corrupt or distort text if mishandled. A translator or reviewer working with CJK must understand not only the language but the script-specific engineering and typographic concerns. Treating CJK text as if it were Latin with different glyphs leads to broken segmentation, wrong character set choices, corrupted encoding, improper line breaks, and punctuation that looks wrong to native readers. Worse, character set choices such as simplified versus traditional Chinese carry regional and political significance; choosing wrong can alienate the audience or misaddress the market. CJK text handling is a domain where linguistic, technical, and cultural judgment intersect, and where defaults borrowed from Latin workflows consistently fail.

Use this skill when translating, reviewing, or engineering CJK text, choosing character sets, handling segmentation and word boundaries, or addressing line-breaking and punctuation. The goal is CJK text that is linguistically correct, technically sound, and culturally and regionally appropriate.

## Core Rules

### Choose The Correct Character Set And Variant

CJK character set choices carry regional and political significance. Choose deliberately.

Chinese has simplified and traditional character sets: simplified is standard in mainland China and Singapore; traditional is standard in Taiwan, Hong Kong, and Macao. The choice is not stylistic; it addresses different markets and audiences, and using the wrong set can alienate readers or misaddress the locale. Japanese uses kanji along with hiragana and katakana; Korean uses hangul, with hanja in limited contexts. Verify the target locale's standard and use it consistently. Do not assume that "Chinese" specifies the set; confirm whether simplified or traditional is required, and whether a specific regional variant such as Taiwan versus Hong Kong is needed.

A character set mismatch signals that the translator does not understand the market.

### Handle Word Boundaries And Segmentation Correctly

CJK languages do not separate words with spaces like Latin scripts. Handle segmentation correctly.

Chinese and Japanese do not use spaces between words; Korean uses spaces but with different rules than Latin scripts. This affects how CAT tools segment text, how search and indexing work, and how the translator navigates content. Segmentation rules designed for Latin scripts, which break on spaces and punctuation, may produce poor segments for CJK. Configure segmentation for CJK, which may require dictionary-based or morphological analysis to identify word boundaries. Be aware that word boundary ambiguity can affect meaning and that different segmentations may be valid. For Korean, respect the spacing rules, as incorrect spacing changes meaning and readability.

Latin-style segmentation applied to CJK produces fragments and poor translation memory matching.

### Follow CJK Punctuation Conventions

CJK scripts have distinct punctuation that differs from Latin. Follow the conventions.

CJK uses full-width punctuation, such as 、。 「」 【】 in Chinese and Japanese, and ，。 《》 in Korean contexts, which occupy a full character width and align with the script's grid. Using half-width Latin punctuation such as , . "" in CJK text looks wrong and disrupts typography. CJK punctuation also follows different spacing and placement rules: for example, periods are placed at the bottom-right in Chinese and Japanese, and there are specific conventions for quotation marks, ellipses, and dashes per language. Use the correct full-width punctuation and follow the language's conventions. Mixing Latin and CJK punctuation is a common error that signals carelessness.

### Apply Correct Line-Breaking Rules

Line breaking in CJK differs from Latin and must follow the script's rules. Apply them correctly.

In Chinese and Japanese, line breaks can occur between any two characters except before certain punctuation, with specific rules about which characters cannot start or end a line. Korean breaks at word boundaries defined by spaces. Latin line-breaking rules, which break at spaces and hyphens, do not apply and will produce awkward or incorrect breaks. Configure tools and rendering to use CJK line-breaking rules, often governed by Unicode line-breaking properties. Verify that rendered text breaks correctly, especially in UI where space is constrained. Incorrect line breaks disrupt readability and look unprofessional.

### Manage Fonts And Character Rendering

CJK characters require fonts and rendering that support them. Manage font and rendering issues.

CJK scripts include thousands of characters; not all fonts cover the full range, and rare or variant characters may not render. Verify that the fonts used cover the required characters, including any specialized or archaic forms. Watch for character encoding issues, where the same Unicode code point may render differently across locales, such as certain CJK characters that have different forms in Chinese, Japanese, and Korean and may require locale-specific fonts. Test rendering in the target environment, not just the authoring tool, because font availability differs. Missing glyphs render as boxes or wrong characters, breaking the text.

### Handle Encoding And Character Integrity

CJK text is vulnerable to encoding corruption. Handle encoding carefully.

Ensure that files use Unicode, typically UTF-8, and that the entire pipeline preserves CJK characters without corruption. Legacy encodings such as Shift-JIS, Big5, or EUC-KR may appear in older files; convert them carefully to Unicode and verify integrity. Watch for mojibake, where characters render as garbage due to encoding mismatches. Verify that databases, APIs, and tools accept and preserve CJK characters. Encoding corruption can render an entire deliverable unusable, so verify encoding at every stage.

### Address Locale-Specific CJK Concerns

CJK locales have specific concerns beyond the script itself. Address them.

Chinese locales differ in terminology, phrasing, and conventions between mainland China, Taiwan, Hong Kong, and Singapore, not just character set. Japanese has specific conventions for keigo, honorifics, and vertical versus horizontal writing in some contexts. Korean has speech levels and honorifics that depend on audience and context. Do not treat CJK as monolithic; address the specific locale's linguistic and cultural norms. A translation correct for mainland China may be inappropriate for Taiwan beyond character set, due to terminology and phrasing differences.

### Verify CJK In Context

CJK text must be verified in context, especially in UI. Verify rendering and fit.

CJK characters are typically wider than Latin, so UI layouts may need adjustment; text that fits in Latin may overflow in CJK, or conversely, CJK may fit where expanded Latin would not. Verify that strings fit, that line breaks are correct, that fonts render, and that the text looks natural in the running product. Context verification catches issues that file-level review misses, such as truncation, wrong font fallback, or awkward breaks.

## Common Traps

### Wrong Character Set Or Variant

Using simplified for a Taiwan audience or traditional for mainland China misaddresses the market and alienates readers.

### Latin-Style Segmentation

Breaking CJK text at spaces or Latin punctuation produces poor segments and weak matching.

### Mixing Latin And CJK Punctuation

Half-width punctuation in CJK text disrupts typography and signals carelessness.

### Incorrect Line Breaking

Latin line-breaking rules produce awkward or wrong breaks in CJK; use CJK-specific rules.

### Font Or Glyph Gaps

Fonts that do not cover the required characters render boxes or wrong glyphs; verify coverage.

### Encoding Corruption

Mojibake from encoding mismatches can render an entire deliverable unusable; handle encoding at every stage.

### Treating CJK As Monolithic

Chinese, Japanese, and Korean have distinct rules, and locales within Chinese differ beyond character set; address specifics.

### Skipping Context Verification

CJK width and rendering issues appear only in context; file-level review misses truncation and breaks.

## Self-Check

Before delivering or approving CJK text, verify:

- The correct character set and variant is chosen for the target locale, simplified or traditional Chinese, with regional variants addressed.
- Word boundaries and segmentation are handled correctly for the script, with CJK-appropriate segmentation rules applied.
- CJK full-width punctuation is used according to the language's conventions, with no Latin punctuation mixed inappropriately.
- Line breaking follows the script's rules, with no characters starting or ending lines improperly and correct behavior in constrained UI.
- Fonts cover the required characters, including rare or variant forms, and rendering is verified in the target environment.
- Encoding is Unicode throughout the pipeline, with no mojibake or corruption, and legacy encodings converted and verified.
- Locale-specific concerns beyond the script, including terminology, phrasing, honorifics, and conventions, are addressed for the specific locale.
- The text is verified in context, with fit, line breaks, fonts, and natural appearance confirmed in the running product.
- No Latin-script default is applied to segmentation, punctuation, line breaking, or encoding.
- The CJK text is linguistically correct, technically sound, and culturally and regionally appropriate.
