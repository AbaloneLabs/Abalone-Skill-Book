---
name: digital_and_multiformat_proofreading.md
description: Use when the agent is proofreading content published across multiple digital formats and platforms, web, EPUB, PDF, apps, responsive layouts, where text reflows differently, where links and media behave differently, where platform-specific defects appear, and where the same content must be verified clean across every format the reader will encounter.
---

# Digital And Multiformat Proofreading

Digital publication means a single work is rendered across multiple formats and platforms, and each renders the same content differently. A heading that is correct in the manuscript breaks when it becomes an HTML tag; a list that reads cleanly in the editor becomes a formatting disaster in EPUB; a table that fits on a PDF page overflows on a phone; a link that works on desktop fails on mobile; an image with no alt text is invisible to screen readers; a special character becomes a mojibake symbol after encoding conversion; a pull quote that looked right in design reflows to point at the wrong passage. Proofreading for digital and multiformat publication is not reading the text once; it is reading the text as it actually appears in each format the reader will use, because the same source produces different defects in different renderings. Proofreading only the source or only one format ships defects that readers encounter in the others.

Use this skill when proofreading content for digital publication across formats and platforms. It covers format-specific defect hunting, reflow and responsive behavior, link and media verification per platform, encoding and character integrity, accessibility elements, and the discipline of proofing each rendering rather than assuming the source transfers cleanly. The editor's task is to ensure the work is clean in every format the reader will encounter.

## Core Rules

### Proof Each Format The Reader Will Encounter, Not Just The Source

The source manuscript is not what the reader sees. The reader sees rendered output: HTML on a website, EPUB on an e-reader, PDF on screen or paper, an app screen on a phone. Each rendering can introduce defects absent from the source, and proofing only the source or only one format ships defects in the others.

Identify every format the work will be published in and proof each one. For each format, read the content as it actually appears, checking for format-specific defects. A single source can be clean in one format and broken in another, and only format-specific proofing catches the breaks. Prioritize the formats most readers will use, but do not skip any published format, because defects in less-common formats still reach readers and damage credibility.

### Hunt For Reflow And Responsive Layout Defects

Digital content reflows: text wraps differently at different screen widths, line breaks shift, pagination changes, and elements that sat side by side on desktop stack vertically on mobile. Reflow introduces defects that static proofing misses: orphaned headings at the bottom of a screen, widows and orphans in paragraphs, tables that overflow horizontally, images that resize poorly, callouts that separate from their anchors.

Proof at multiple screen widths and orientations, especially the extremes, desktop wide, tablet, and phone narrow. Check that headings stay with their following content, that paragraphs do not strand lines, that tables and figures remain usable when narrow, and that interactive elements remain tappable. Responsive defects are among the most common digital publication failures, and they are invisible without testing at the widths readers actually use.

### Verify Links, Media, And Interactive Elements Per Platform

Links and media behave differently across platforms. A link that opens correctly on desktop may open in a constrained webview on mobile, may fail in an e-reader's limited browser, or may be blocked by an app's security settings. Embedded media may not play on all devices. Interactive elements may not function without specific support.

Test links and media on each platform where they will be used, not just on desktop. Verify that links open correctly in the platform's context, that embedded media plays or degrades gracefully, and that interactive elements function. For e-readers, check that internal links navigate correctly and external links handle the device's limitations. For apps, test within the app environment. Platform-specific link and media failures are common and only platform-specific testing catches them.

### Check Encoding, Character, And Special Symbol Integrity

Character encoding problems are a signature defect of digital publication, and they appear when content moves between systems. Smart quotes become garbled symbols, em dashes become sequences of unrelated characters, accented letters and non-Latin scripts corrupt, and special symbols render incorrectly or as missing-character boxes. These defects are visually obvious and immediately signal unprofessionalism.

Proof for character integrity in every format, especially after any conversion between systems. Check smart quotes, dashes, accented characters, non-Latin scripts, mathematical and scientific symbols, and any special characters. Where corruption appears, trace it to the encoding or font issue and fix it at the source so it renders correctly downstream. Verify that the chosen fonts in each format actually contain the glyphs the text uses, since a missing glyph in a format's font produces a missing-character symbol even when the source is correct.

### Verify Accessibility Elements In Digital Formats

Digital formats carry accessibility elements that are invisible in a visual proof but essential for readers using assistive technology. Alt text for images, heading structure for screen-reader navigation, link text that makes sense out of context, transcript and caption availability for media, sufficient color contrast, and keyboard navigability all determine whether the work is accessible. Proofing that checks only the visual rendering ships inaccessible content.

Proof accessibility elements deliberately. Check that every image has appropriate alt text, that headings form a logical structure, that link text is descriptive out of context, that media has captions or transcripts, that color contrast meets standards, and that the content is navigable by keyboard. Where possible, test with a screen reader or accessibility checker. Accessibility is not optional polish; it determines whether a class of readers can access the work at all, and its failures are invisible to visual proofing.

### Check Format-Specific Structural And Styling Elements

Each digital format has structural and styling elements specific to it, and these must be verified in that format. HTML and EPUB have heading levels, semantic tags, and CSS-driven styling; PDF has bookmarks, tagged structure, and fixed layout; apps have platform-specific navigation and components. Defects in these elements, broken heading hierarchies, missing semantic tags, untagged PDFs, broken bookmarks, break the format's usability and accessibility.

For each format, verify its specific structural and styling elements. Check heading hierarchy and semantic structure in HTML and EPUB. Check PDF bookmarks and tagged structure. Check app navigation and component behavior. These elements are often generated automatically and can break silently, so verify them in the rendered output rather than trusting the generation process.

### Verify Metadata And Discoverability Elements Per Format

Digital formats carry metadata that affects discovery and rendering: titles, descriptions, author names, language tags, subject categories, canonical URLs, Open Graph and social cards. Errors here affect search visibility, social sharing, and correct rendering in aggregators and platforms. Metadata is often entered separately from the content and proofed separately, which means it is often missed.

Proof metadata for each format and platform. Verify titles and descriptions are correct and appropriately sized for the platform's display. Check author names and attribution. Verify language tags for accessibility and search. Check Open Graph and social card rendering by previewing how the content appears when shared. Metadata errors are high-visibility because they appear in search results and social feeds, where they form the first impression of the work.

### Re-Verify After Any Format Conversion Or Update

Digital content is often converted between formats, EPUB from HTML, PDF from a layout file, app content from a CMS, and each conversion can introduce defects. Updates after publication, corrected typos, added content, changed links, must be re-verified in every format, because updating one format does not update the others.

After any conversion or update, re-proof the affected formats. Conversions routinely introduce character corruption, broken links, lost styling, and structural defects, so treat converted output as new content requiring full proofing, not as a trusted transfer. After post-publication updates, verify the update propagated correctly to every format and that it did not introduce new defects. Assuming formats stay synchronized ships defects in the formats that were not re-checked.

## Common Traps

### Proofing Only The Source Or One Format

The same source produces different defects in different renderings. Proof each format the reader will encounter.

### Ignoring Reflow At Different Screen Widths

Responsive defects appear only at the widths readers use. Test at desktop, tablet, and phone widths.

### Assuming Links And Media Work Across Platforms

Links and media behave differently per platform. Test in each platform's context.

### Missing Encoding And Character Corruption

Encoding problems are visually obvious and signal unprofessionalism. Check characters after any conversion.

### Skipping Accessibility Elements

Accessibility failures are invisible to visual proofing. Check alt text, headings, contrast, and keyboard navigation.

### Trusting Auto-Generated Structure And Styling

Heading hierarchies, tags, and bookmarks break silently. Verify structural elements in the rendered output.

### Forgetting Metadata And Social Cards

Metadata forms the first impression in search and social. Proof titles, descriptions, and card rendering.

### Not Re-Proofing After Conversion Or Update

Conversions and updates introduce defects and desynchronize formats. Re-verify affected formats after any change.

## Self-Check

Before treating digital and multiformat proofreading as complete, verify:

- Each format the reader will encounter has been proofed in its rendered output, not just the source.
- Reflow and responsive behavior have been checked at multiple screen widths and orientations.
- Links, media, and interactive elements have been tested per platform, not just on desktop.
- Character encoding and special symbols render correctly in every format, especially after conversion.
- Accessibility elements, alt text, headings, contrast, captions, keyboard navigation, have been verified.
- Format-specific structural and styling elements, heading hierarchies, tags, bookmarks, have been checked.
- Metadata and discoverability elements, titles, descriptions, social cards, are correct per platform.
- After any format conversion or post-publication update, affected formats have been re-proofed.
- The work is clean in every format the reader will encounter, without format-specific defects reaching publication.
