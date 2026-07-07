---
name: accessible_media_and_document_editing.md
description: Use when the agent is editing captions transcripts and audio descriptions, verifying color contrast and color-only meaning, preparing accessible PDFs and document structures, making tables and data presentation work for assistive technology, or verifying accessibility claims in media and documents rather than assuming compliance.
---

# Accessible Media And Document Editing

Media and documents carry some of the highest accessibility risk because they bundle multiple formats, text, image, audio, video, color, and structure, each with its own accessibility requirements. Editors miss these problems because the media looks fine visually and because accessibility is often treated as a final checkbox rather than an editing concern. The harm is direct exclusion: a video without captions is unusable for deaf and hard-of-hearing users, a PDF without tagged structure is unnavigable for screen-reader users, a chart relying on color alone is unreadable for colorblind users, and a table built for visual layout announces as gibberish to assistive technology. The editor's task is to ensure every media element and document provides equivalent access, and to verify that access rather than assume it.

## Core Rules

### Provide Accurate, Complete Captions And Transcripts

Video and audio content must be accessible to users who cannot hear it. Captions synchronize text with the audio for video; transcripts provide a full text version of audio or video content. The editor ensures captions are accurate, not auto-generated without correction, complete, including dialogue, meaningful sound effects, and speaker identification where needed, and properly timed. Transcripts should capture the full content and, where relevant, descriptions of key visual information. The editor does not accept machine captions at face value; they require review and correction because errors change meaning and erode trust. For audio-only content like podcasts, a transcript is the primary accessibility mechanism and should be treated as a first-class deliverable.

### Write Audio Descriptions For Visual-Only Information

Where a video conveys important information visually without narration, blind and low-vision users miss it. Audio description is an additional narration track that describes essential visual content: actions, scene changes, on-screen text, and graphics that are not explained in the dialogue. The editor identifies videos where visual information is critical to understanding and ensures audio descriptions are created, either integrated into the main narration where natural or provided as a separate described track. Not every video needs audio description, but any video where sight is necessary to follow the content does. The editor's judgment is determining which videos require description and what information must be conveyed.

### Verify Color Contrast And Eliminate Color-Only Meaning

Color is a frequent accessibility failure. Insufficient contrast between text and background makes content unreadable for low-vision users and in poor viewing conditions; the editor verifies that text meets contrast ratio standards and flags low-contrast design choices. Equally important, content must not rely on color alone to convey meaning, because colorblind users cannot perceive the distinction. A chart that uses red and green to mark pass and fail, a form that marks errors only with red borders, or instructions that say "click the green button" all fail. The editor ensures meaning is reinforced with text labels, patterns, icons, or position, so the information survives without color perception.

### Build Accessible Document And PDF Structures

PDFs and other documents are often exported in ways that destroy accessibility. A visually correct PDF may have no underlying tags, rendering it a blank sequence to a screen reader, or tags that announce content in the wrong order. The editor ensures documents are built accessibly from the start: true headings, tagged lists, reading order verified, alt text on images, a document language set, and a logical structure. For PDFs specifically, the editor verifies that the document is tagged, that the tag tree reflects the content, and that form fields are properly labeled. Source documents, such as Word or InDesign files, should be authored with accessibility in mind, because retrofitting accessibility into a finished export is far harder.

### Make Tables Convey Data, Not Layout, To Assistive Technology

Tables are a major accessibility hazard. Tables used for visual layout confuse screen readers, which announce them as data relationships that do not exist. Data tables must be marked up so that headers, relationships, and reading order are conveyed to assistive technology, allowing a screen-reader user to navigate cells and understand which data relates to which header. The editor ensures layout tables are avoided or replaced with proper structural elements, that data tables have header cells correctly associated with data cells, that complex tables have captions and summaries, and that the reading order makes sense cell by cell. A table that looks right visually but announces as a jumble has failed.

### Verify Accessibility Claims Rather Than Assuming Compliance

It is easy to claim a document or media asset is accessible and easy to be wrong. The editor does not sign off on accessibility based on the author's assertion or a single automated scan. The editor verifies by checking captions against the actual audio, confirming transcripts are complete, testing documents with a screen reader or accessibility checker, reviewing the tag tree of PDFs, and confirming contrast ratios with measurement rather than eyeballing. Where the editor cannot fully verify, the asset is flagged for expert testing rather than declared compliant. Verification is the editor's defense against the gap between "looks accessible" and "is accessible."

### Coordinate Accessibility Across The Production Pipeline

Accessible media and documents are not produced by an editor alone at the end; they require coordination across the pipeline. Captions and transcripts need time and budget allocated; audio description needs scripting and recording; accessible PDFs need source documents authored correctly; accessible video needs players that support captions and descriptions. The editor ensures accessibility requirements are specified early, before production, so that the assets can be made accessible by design rather than retrofitted. Treating accessibility as a late add-on guarantees it will be late, expensive, and incomplete.

## Common Traps

### Trusting Auto-Generated Captions Without Correction

Machine captions contain errors that change meaning and undermine trust. The trap is treating captions as done once generated. The editor reviews and corrects captions against the audio.

### Forgetting Audio Description For Visual-Only Video

Videos that convey meaning visually exclude blind users when description is absent. The trap is assuming narration covers everything. The editor identifies visual-critical videos and ensures description.

### Relying On Color Alone To Convey Meaning

Color-only distinctions fail for colorblind users. The trap is designing for typical color vision. The editor reinforces meaning with text, patterns, or icons.

### Low Contrast That Looks Fine To The Designer

Designers with strong vision may accept contrast that fails standards. The trap is eyeballing rather than measuring. The editor verifies contrast ratios.

### Untagged Or Mis-Ordered PDFs

A visually correct PDF can be blank or jumbled to a screen reader. The trap is trusting the visual export. The editor verifies tags, reading order, and structure.

### Using Tables For Layout

Layout tables announce false data relationships to assistive technology. The trap is using tables for visual arrangement. The editor replaces layout tables with proper structure.

### Claiming Accessibility Without Verification

Unverified claims create false confidence and ship exclusion. The trap is equating intent or automated scans with compliance. The editor verifies with human testing and flags uncertainty.

## Self-Check

- Are captions accurate, complete, properly timed, and human-corrected, including dialogue, sound effects, and speaker identification?
- For audio and video content, is a full transcript provided, and is it treated as a first-class deliverable rather than an afterthought?
- For videos where visual information is essential, has audio description been created and verified to convey that information?
- Does text meet contrast ratio standards, verified by measurement rather than by eye, with low-contrast choices flagged?
- Is meaning reinforced beyond color, so that colorblind users receive the same information through text, patterns, or icons?
- Are PDFs and documents tagged, with verified reading order, true headings, alt text, document language, and properly labeled form fields?
- Are tables used only for data, with headers correctly associated and reading order logical, and have layout tables been replaced with proper structure?
- Has accessibility been verified through human testing, such as screen-reader review and tag-tree inspection, rather than assumed from appearance or automated scans alone?
- Where the editor cannot fully verify, has the asset been flagged for expert testing rather than declared compliant?
- Were accessibility requirements specified early in production, with time and budget allocated, rather than added as a late retrofit?
