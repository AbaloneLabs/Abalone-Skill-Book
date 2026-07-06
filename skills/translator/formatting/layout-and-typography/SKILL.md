---
name: layout_and_typography.md
description: Use when the agent is translating content where visual layout and typography carry meaning, adjusting line length and line breaks for target text, handling headings lists tables and captions, managing quotation marks dashes and spacing conventions per locale, or ensuring that translated text preserves the document's visual structure and typographic correctness.
---

# Layout And Typography

Translation changes text length, and changed text length changes layout. A heading that fit on one line in English may wrap to two lines in German and truncate in a fixed-width UI column. A bulleted list whose items aligned cleanly may rag unevenly when each item expands by thirty percent. A table cell with comfortable padding may overflow when the target language is more verbose. Typography is equally affected: quotation marks, dashes, apostrophes, ellipses, spaces around punctuation, and capitalization conventions all differ by locale, and a translation that is linguistically perfect but typographically careless looks unprofessional to native readers. The judgment problem is that layout and typography are usually treated as someone else's problem, handled by designers or rendered automatically, when in fact many translation deliverables hand layout and typography decisions directly to the translator. The translator who ignores them produces text that is correct word-by-word but broken as a document.

Agents miss layout and typography issues because they work segment by segment and rarely see the assembled page, because typographic conventions feel like minor styling, and because expansion and contraction effects are invisible until the text is placed in context. This skill exists to make the translator responsible for the visual and typographic integrity of the translated document, not just the words.

## Core Rules

### Anticipate And Plan For Text Expansion And Contraction

Different languages occupy different amounts of space for the same meaning. Translating from English into German, French, Spanish, or Russian typically expands text by twenty to thirty-five percent; translating into Chinese, Japanese, or Korean often contracts it. This expansion affects everything downstream: line length, line count, column width, table cell height, UI control size, and page count. Before and during translation, anticipate where expansion will cause problems. Headings in fixed-width containers, button labels, table column headers, and tightly laid-out marketing copy are the highest-risk areas. Where you control the deliverable, design for expansion by leaving slack or using flexible containers. Where you do not control layout, flag segments likely to overflow so designers or engineers can adjust. Ignoring expansion is not neutral; it produces truncation, overlap, and broken grids.

### Preserve The Document's Structural Hierarchy

A translated document must preserve the structural hierarchy of its source: headings stay headings at the same level, lists stay lists with the same nesting, tables keep their rows and columns, captions stay captions, footnotes stay footnotes. The temptation is to flatten or restructure when target text does not fit, for example demoting a heading to body text or merging list items. Resist this. The hierarchy carries meaning: it tells readers what is primary, what supports it, and how sections relate. Preserve every structural element in its original role, and if a structure genuinely cannot accommodate the translation, raise it as a layout problem rather than silently changing the structure.

### Apply Locale-Correct Typography For Punctuation And Marks

Typography conventions for punctuation differ by locale and must be applied deliberately, not inherited from the source. Quotation marks are the clearest example: English uses straight or curly double quotes, German uses low-opening quotes, French uses guillemets with spaces, Japanese and Chinese use corner brackets. Dashes differ: the em dash, en dash, and figure dash have distinct uses, and some locales prefer spaced en dashes where others use unspaced em dashes. Ellipses may be three dots or a single character, with or without spaces. Apostrophes should be the typographic curly form in most published locales, not the straight ASCII apostrophe, though code contexts reverse this. Spaces around punctuation vary: French uses a narrow space before colons, semicolons, and guillemets. Learn and apply the target locale's conventions rather than copying the source's, because copied source typography marks the text as carelessly localized.

### Manage Spacing And Non-Breaking Spaces Deliberately

Spacing is typographic and structural, not just whitespace. Many locales use non-breaking spaces to keep units with their values, to keep numbers with their symbols, and to prevent orphaned punctuation. French requires non-breaking spaces before colons, semicolons, question marks, and guillemets. Numbers and their units should not break across lines. Initials, dates, and currency amounts often need non-breaking spaces. The translator must know where the locale expects non-breaking spaces and insert them, and must know where line breaks are forbidden and prevent them. Indiscriminate regular spaces produce awkward breaks; indiscriminate non-breaking spaces prevent all wrapping and cause overflow. The judgment is per-context.

### Handle Line Breaks And Wrapping For Readability

When the translator controls line breaks, as in poetry, song lyrics, subtitles, UI strings with manual breaks, or formatted plain text, breaks must be placed for target-language readability, not copied from the source. Source line breaks fall at English word boundaries and reflect English rhythm; copying them into a language with different word order and length produces ragged, unreadable lines. Re-break for the target, respecting its word boundaries, its punctuation rules about what may start or end a line, and the original's intent where the break carries meaning. For subtitles, respect reading-speed and line-length limits in the target, which may require condensation. For UI strings, prefer letting the layout engine wrap unless a manual break is required, and then place it where it reads naturally.

### Preserve Alignment And Directional Layout Cues

Layout carries directional assumptions that translation can disrupt. Centered headings, right-aligned numbers, indented blockquotes, and aligned table columns all assume certain text properties. When translating into a right-to-left language, alignment and directional cues must flip. When translating into a language with different character widths, alignment based on character count breaks. Preserve the intent of the alignment, not its literal direction: a right-aligned price column in an RTL locale should still align numbers consistently, but the column's position and text direction change. Verify that alignment still serves readability after translation rather than producing visual chaos.

### Verify Layout In Context Not In Isolation

Layout and typography problems are invisible in a segment list and visible only in the assembled document. A heading that wraps badly, a table cell that overflows, a list that no longer aligns, a quotation mark in the wrong form, a line break that splits a unit from its value, all of these appear only when the translated text is placed in its real container. Verify the translated document in context: render it, view it in the target application, check it at the actual font and size. Isolated text review cannot catch layout defects.

## Common Traps

### Ignoring Expansion Until The Layout Breaks

Treating length as a designer's problem leads to truncated headings, overflowing cells, and broken grids discovered at delivery. The trap is that each segment looks fine alone.

### Copying Source Typography Into The Target Locale

Straight quotes, em dashes, and spacing copied from English mark the translation as careless in locales that expect guillemets, spaced en dashes, or non-breaking spaces.

### Flattening Structure To Make Text Fit

Demoting a heading or merging list items to avoid overflow silently changes the document's meaning. The trap is that the text fits while the structure is wrong.

### Copying Source Line Breaks Into Target Text

Line breaks that served English rhythm produce ragged, unreadable lines in a language with different word order. The trap is faithfulness to the wrong thing.

### Using Regular Spaces Where Non-Breaking Spaces Are Required

Units separated from values, punctuation orphaned at line starts, and numbers split across lines all result from default spaces. The trap is that the text looks correct until it wraps.

### Reviewing Only Isolated Segments

Segment-level review misses every layout defect. The trap is confidence based on evidence that cannot reveal the problem.

## Self-Check

- Have you anticipated text expansion or contraction and identified high-risk areas such as headings, buttons, table cells, and fixed-width containers?
- Does the translated document preserve the source's structural hierarchy, with every heading, list, table, caption, and footnote in its original role?
- Are quotation marks, dashes, ellipses, apostrophes, and punctuation marks in the target locale's correct typographic forms rather than copied from the source?
- Are non-breaking spaces used where the locale requires them, and are line breaks prevented where they would orphan punctuation or split units from values?
- Where you control line breaks, are they placed for target-language readability and rhythm rather than copied from the source?
- For RTL or different-width targets, does alignment and directional layout serve readability rather than producing chaos?
- Has the translated document been viewed in context, at the real font and size, to confirm that layout and typography are correct?
- Are any segments that cannot fit their container flagged as layout problems rather than silently restructured?
