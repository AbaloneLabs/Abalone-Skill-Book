---
name: source_structure_and_extractability_analysis.md
description: Use when the agent is analyzing the structure of a source document before translation, determining how content is organized and segmented, identifying translatable versus non-translatable elements, detecting hidden content and embedded text, or assessing whether the source can be cleanly extracted and handed off without losing or corrupting content.
---

# Source Structure And Extractability Analysis

Before a source can be translated, it must be understood as a structure, and that structure determines what can be translated, what must be preserved, and what is at risk of being lost or corrupted. A document is not a flat sequence of words. It contains body text, headings, tables, captions, footnotes, embedded images with text, form fields, hyperlinks, metadata, comments, hidden layers, and markup that controls how content assembles. When a translator or a tool extracts only the visible body text, everything else is at risk: captions vanish, table cell correspondence breaks, footnote markers detach, image text is left untranslated, and placeholders are stripped. The translation may be accurate word by word while the deliverable is structurally broken. Source structure and extractability analysis is the discipline of mapping the source's full anatomy before translation, so that every translatable element is captured, every non-translatable element is protected, and the handoff preserves the structure that gives the content its meaning. Skipping this analysis is how translations come back complete in word count yet broken in product.

Use this skill when analyzing a source document's structure before translation, determining extractability, identifying hidden and embedded content, or preparing a clean handoff. The goal is to ensure the full source is captured and protected, with no element lost, corrupted, or left untranslated through structural oversight.

## Core Rules

### Map The Full Anatomy Of The Source

Before extracting or translating, map the complete structure of the source document. A structural map prevents elements from being overlooked.

Identify every content layer: body paragraphs, headings and subheadings at each level, lists ordered and unordered, tables with their rows, columns, and headers, captions for figures and tables, footnotes and endnotes with their markers and references, text boxes and callouts, headers and footers, embedded images and their contained text, form fields and labels, hyperlinks and their anchor text, metadata including titles and descriptions, comments and tracked changes, and any hidden or collapsed content. Document where each element sits and how it relates to the others.

A source mapped only by its visible body text guarantees that invisible layers are missed.

### Distinguish Translatable From Non-Translatable Elements

Not everything in a source should be translated, and the distinction must be made deliberately, not by accident. Classify each element.

Translatable elements include body text, headings, captions, UI labels, and any content the reader needs. Non-translatable or protected elements include code, variables, placeholders, tags, URLs, email addresses, file paths, product names and trademarks where retained, legal citations, measurement units in some contexts, and identifiers. Some elements require transformation rather than translation, such as dates, numbers, currencies, and units that may need locale conversion. Classify each element and mark its handling, because accidentally translating a placeholder breaks the product and accidentally preserving translatable text leaves the deliverable incomplete.

### Detect Hidden And Embedded Content

Sources contain content that is not immediately visible, and it is the most commonly missed category in translation. Detect it deliberately.

Hidden content includes text in collapsed sections, comments and annotations, alternative text for images, metadata fields, hidden form fields, text in headers and footers that does not display in the body, content in hidden slides or layers, and text embedded in images, diagrams, and charts. Embedded image text is especially dangerous, because it cannot be translated by string extraction and requires recreation of the image. Scan the source systematically for these categories, because tools that extract visible text will not surface them.

Hidden content missed at analysis becomes content missing from the localized deliverable, noticed only by the target user.

### Preserve Structural Relationships And Correspondence

Structure carries meaning, and translation must preserve the relationships between elements. Map and preserve correspondence.

Footnote markers must remain attached to the correct footnote. Table cells must remain in the correct row and column, because translating cell contents out of order scrambles the data. Caption numbering must stay aligned with its figure or table. Cross-references must point to the correct target after translation. List nesting and hierarchy must be preserved. Hyperlink anchor text must remain linked to the correct destination. When extraction or translation breaks these correspondences, the deliverable becomes incoherent even if each piece is translated correctly.

### Assess Extractability Before Handoff

Before handing the source to translators or tools, assess whether it can be cleanly extracted. Poor extractability corrupts content.

Assess the file format and how it behaves under extraction. Some formats, like well-structured XML or content management systems, extract cleanly with structure preserved. Others, like PDFs, scanned documents, and complex desktop-publishing files, extract poorly, losing structure, corrupting text, and dropping elements. Assess whether the extraction tool preserves tags, placeholders, and formatting, or strips them. Where extractability is poor, request a better source format, or plan for additional cleanup and verification, because translating a corrupted extraction propagates corruption into every language.

### Handle Segmentation Carefully

Segmentation, how the source is divided into translatable units, affects both quality and structure. Handle it deliberately.

Poor segmentation breaks sentences at wrong boundaries, splits UI strings incorrectly, and separates text that must translate as a unit. Verify that segmentation respects sentence boundaries, preserves inline tags and placeholders within their segment, and keeps UI strings and assembled content intact. Where segmentation is configurable, tune it to the content type, because UI strings, documentation, and legal text each segment differently. Bad segmentation produces translations that are correct per segment but broken when assembled.

### Verify Completeness Against The Structural Map

After extraction and throughout translation, verify completeness against the original structural map. The map is the checklist.

Compare what was extracted and translated against the anatomy mapped at the start. Confirm every body paragraph, heading, table cell, caption, footnote, image text, form field, and metadata field is represented. Confirm no element was dropped during extraction, lost during segmentation, or skipped during translation. A completeness check against the structural map catches the omissions that word counts hide, because a word count can be satisfied while elements are missing.

## Common Traps

### Mapping Only The Visible Body Text

Treating the source as body paragraphs ignores headings, tables, captions, footnotes, image text, and metadata, all of which then go untranslated.

### Failing To Classify Protected Elements

Without deliberate classification, placeholders, code, and URLs get translated and break the product, or translatable text gets preserved and left incomplete.

### Missing Hidden And Embedded Content

Collapsed sections, comments, alt text, and image-embedded text are invisible to text extraction and are the most commonly missed elements.

### Breaking Structural Correspondence

Translating elements out of order or detaching markers, references, and links produces a deliverable that is incoherent despite correct individual pieces.

### Translating A Poorly Extracted Source

Extracting from PDFs or complex formats without assessing extractability corrupts content and propagates corruption into every language.

### Accepting Default Segmentation

Untuned segmentation breaks sentences, splits UI strings, and separates units that must translate together, producing correct-per-segment but broken-assembled text.

### Trusting Word Count Over Structural Completeness

A word count can be satisfied while structural elements are missing. Verify against the structural map, not just the count.

## Self-Check

Before approving a source for translation handoff or extraction, verify:

- The full anatomy of the source is mapped, including body, headings, tables, captions, footnotes, images, forms, links, metadata, comments, and hidden content.
- Each element is classified as translatable, non-translatable or protected, or requiring transformation such as locale conversion.
- Hidden and embedded content, collapsed sections, comments, alt text, and image text, was detected and included.
- Structural relationships, footnote markers, table correspondence, caption numbering, cross-references, and links, are mapped for preservation.
- Extractability was assessed, with a better source format requested or cleanup planned where extraction is poor.
- Segmentation was configured and verified to respect sentence boundaries, inline tags, and UI string integrity.
- Completeness is verified against the structural map, not just word count, throughout extraction and translation.
- No translatable element was lost, corrupted, or skipped through structural oversight.
- No protected element was translated, and no translatable element was preserved by accident.
- The localized deliverable preserves the source's structure and correspondence, not just its word count.
