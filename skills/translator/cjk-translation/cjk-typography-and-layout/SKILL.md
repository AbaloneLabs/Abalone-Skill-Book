---
name: cjk_typography_and_layout.md
description: Use when the agent is laying out or reviewing Chinese Japanese or Korean text in documents interfaces or published formats, handling full-width and half-width punctuation, applying CJK line-breaking and kinsoku rules, managing spacing between Latin and CJK characters, choosing vertical versus horizontal writing, fitting CJK strings into constrained UI, or ensuring that fonts proportions and grid alignment produce professional CJK typography.
---

# CJK Typography And Layout

CJK typography follows rules that Latin typography never encounters, and ignoring them produces text that a native reader instantly reads as amateur even when every word is correct. The characters occupy a uniform square em rather than variable widths, punctuation comes in full-width and half-width forms that must be matched to the script, line breaking is governed by kinsoku rules that forbid certain characters from starting or ending a line, and the interaction between CJK characters and embedded Latin text or numbers requires deliberate spacing decisions. On top of this, CJK supports both horizontal and vertical writing modes, each with its own conventions, and UI strings that fit comfortably in Latin frequently overflow or break badly when rendered in CJK because the characters do not compress the way Latin words do. Translators and reviewers tend to treat typography as a downstream concern handled by the layout tool, but in CJK the typographic rules are part of correctness: a period in the wrong position, a line that begins with a closing bracket, a Latin word jammed against a CJK character with no spacing, or a UI string that truncates mid-character are all defects that degrade the deliverable. CJK typography and layout is a discipline of applying script-specific rules consistently and verifying fit in the actual rendered output.

This skill applies when you are laying out, reviewing, or specifying the typography and layout of CJK text in documents, interfaces, or published formats. The objective is text whose punctuation, line breaks, character spacing, writing mode, and fit all conform to the conventions of the target language and read as professionally typeset rather than as Latin defaults applied to CJK glyphs.

## Core Rules

### Use Full-Width Punctuation Consistent With The Script

CJK punctuation is full-width, occupying a full character cell, and using it consistently is foundational to correct typography.

Chinese and Japanese use full-width punctuation such as the ideographic comma, period, brackets, and quotation marks, which align to the character grid and produce the even texture native readers expect. Korean uses its own conventions, with some full-width and some half-width forms depending on context. Mixing half-width Latin punctuation such as a regular comma or period into CJK body text breaks the visual rhythm and signals carelessness. Use the correct punctuation set for the target language, apply it consistently, and watch for the common error of punctuation being converted to half-width by tools that assume Latin defaults. Pay attention to placement as well: the ideographic period sits at the bottom of the character cell in Chinese and Japanese, and quotation mark styles differ by language and sometimes by locale.

### Apply Kinsoku And CJK Line-Breaking Rules

Line breaking in CJK is governed by rules that forbid certain characters from starting or ending a line, and these rules must be enabled and respected.

Chinese and Japanese allow breaks between most characters but prohibit a line from ending with opening brackets or certain punctuation, and prohibit a line from starting with closing brackets, periods, or other characters that must not begin a line. These are the kinsoku rules, and they exist because breaking them produces text that reads as malformed. Korean breaks at word boundaries defined by spaces, following different rules from Chinese and Japanese. Configure the layout tool or rendering engine to apply the correct line-breaking rules for the target language, and verify that breaks in the rendered output comply. Latin line-breaking, which breaks at spaces and hyphens, does not apply and will produce invalid breaks if forced on CJK text.

### Manage Spacing Between CJK And Latin Or Numeric Runs

When CJK text contains embedded Latin words, numbers, or symbols, the spacing between the CJK and non-CJK runs must be handled deliberately.

Some style guides require a quarter-em or thin space between CJK and Latin characters for readability; others rely on the font's built-in spacing; and some tools insert or strip such spacing automatically, producing inconsistency. Decide the convention from the target style guide, apply it consistently, and verify that the tool has not introduced unwanted spaces or removed wanted ones. The same applies to spacing around Latin punctuation embedded in CJK text. Inconsistent inter-script spacing is a subtle but persistent defect that makes a deliverable look unpolished.

### Choose And Verify The Writing Mode

CJK supports both horizontal and vertical writing, and the choice carries convention and meaning that must match the content type and locale.

Horizontal writing is now standard for most digital and business content across all three languages, but vertical writing remains conventional for certain Japanese and traditional Chinese contexts, particularly literature, some newspapers, and culturally traditional design. Vertical writing reverses the direction of reading and requires layout, punctuation orientation, and font support configured for vertical flow. Determine the required writing mode from the content type and locale, configure the layout accordingly, and verify that punctuation, brackets, and embedded Latin text render correctly in the chosen mode. Forcing horizontal defaults onto content that should be vertical, or vice versa, produces text that feels culturally wrong.

### Fit CJK Strings Into Constrained UI Deliberately

UI strings that fit in Latin frequently overflow or break badly in CJK, and fit must be verified in the rendered interface.

CJK characters do not compress or hyphenate the way Latin words do, so a string that occupied one line in Latin may wrap to two or three lines in CJK, or may truncate mid-character in a fixed-width container. Conversely, some CJK renderings are shorter than the Latin source, which can leave awkward gaps. Identify constrained UI elements such as buttons, tabs, table headers, and navigation labels, test the CJK strings in those containers, and adjust the translation, the container, or the layout to achieve acceptable fit. Do not assume fit from the source; verify it in the running UI. Where truncation is unavoidable, ensure it does not cut through a character or leave an incomplete word.

### Select Fonts That Cover The Required Characters And Match The Locale

Font selection affects both coverage and correctness, because CJK fonts vary in character coverage and in the regional form of shared characters.

A font must cover every character in the content, including rare or variant forms, or it will render missing-glyph boxes or wrong fallback characters. It must also match the locale: because of han unification, a Chinese font applied to Japanese text will render some characters in their Chinese form, which Japanese readers perceive as wrong. Select locale-appropriate fonts, confirm coverage against the actual content, and verify rendering in the target environment. For mixed CJK and Latin content, confirm that the font pairing handles both scripts gracefully and that fallback does not introduce mismatched forms.

### Maintain Grid And Proportion Consistency

Because CJK characters occupy a uniform em, the visual texture depends on consistent grid alignment and proportion, and disruptions are visible.

Body text should align to a consistent character grid, with line height and spacing chosen for the script, which typically needs more vertical space than Latin. Headings, captions, and mixed CJK-Latin lines must be balanced so that the Latin runs do not float awkwardly within the CJK grid. Review the rendered composition for grid consistency, proportional balance, and line height appropriate to the script, not merely for character correctness.

## Common Traps

### Mixing Half-Width And Full-Width Punctuation

Half-width Latin punctuation in CJK body text breaks the visual rhythm and signals carelessness; use the script's full-width punctuation consistently.

### Forcing Latin Line-Breaking On CJK

Latin breaks at spaces and hyphens; CJK follows kinsoku and word-boundary rules, and forcing Latin rules produces invalid breaks.

### Inconsistent Inter-Script Spacing

Spacing between CJK and Latin or numeric runs must follow a chosen convention; tools that auto-insert or strip spaces produce inconsistent, unpolished text.

### Assuming Horizontal Writing Everywhere

Vertical writing remains conventional for some Japanese and traditional Chinese content; forcing horizontal defaults onto such content feels culturally wrong.

### Assuming UI Fit From The Source

CJK strings do not compress like Latin and often overflow or truncate in constrained UI; verify fit in the running interface, not in the source.

### Using A Locale-Wrong Font

A font from the wrong CJK locale renders unified characters in the wrong regional form; select locale-appropriate fonts and confirm coverage.

### Ignoring Grid And Proportion

CJK text depends on consistent character-grid alignment and adequate line height; reviewing only character correctness misses unbalanced, unprofessional composition.

## Self-Check

Before delivering or approving CJK typography and layout, verify:

- Full-width punctuation consistent with the target script is used throughout, with correct placement and no half-width Latin punctuation mixed into body text.
- Line breaking applies the correct kinsoku or word-boundary rules for the target language, with no invalid line starts or ends in the rendered output.
- Spacing between CJK and Latin or numeric runs follows a chosen convention applied consistently, with no tool-introduced inconsistency.
- The writing mode matches the content type and locale, with punctuation, brackets, and embedded Latin text rendering correctly in the chosen mode.
- Constrained UI strings have been tested in the running interface for fit, with truncation that does not cut through characters and wrapping that remains readable.
- Fonts are locale-appropriate, cover every character in the content including rare or variant forms, and render unified characters in the correct regional form.
- The composition maintains a consistent character grid, proportionate headings and captions, and line height appropriate to the script.
- No Latin-typography default has been allowed to govern punctuation, line breaking, spacing, writing mode, or fit in the CJK deliverable.
