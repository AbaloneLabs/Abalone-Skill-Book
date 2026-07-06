---
name: source_completeness_verification.md
description: Use when the agent is verifying that a source is complete and final before translation, checking for missing sections assets references and versions, confirming the provided source is the authoritative version, detecting truncated or partial content, or ensuring no translatable surface has been omitted before scope is locked.
---

# Source Completeness Verification

A translation of an incomplete source is an incomplete translation, and no amount of linguistic skill can fix it. Yet incompleteness is remarkably easy to miss, because the source that arrives looks like a whole document. It has a beginning, a middle, and an end; it reads coherently; it appears self-contained. Only after delivery does it emerge that the appendix was never attached, that the figures referenced in the text were not included, that the provided file was version seven while the final was version nine, that a section was truncated when the PDF was exported, or that a whole chapter existed in a linked document the translator never received. Completeness verification is the work of confirming that the source handed over is the whole, final, authoritative source before any translation effort is invested. Skipping it means translating a partial artifact and presenting it as complete, which fails the reader and exposes the translator to rework no one budgeted for.

Agents skip completeness verification because the source feels sufficient. A document that reads well does not advertise its missing pieces; the gaps are negative space, visible only to someone actively looking for them. Verification is therefore an active discipline: it does not wait for incompleteness to announce itself but hunts for it by checking structure, references, versions, and linked assets against what a complete source should contain. The goal is to convert the assumption that the source is complete into evidence that it is, before the source is locked and translation begins.

Use this skill when receiving a source for translation, before locking scope, when a source may be partial or non-final, or when confirming that all translatable material has been provided. The goal is to ensure the translation is built on a complete, final, authoritative source rather than a fragment presented as a whole.

## Core Rules

### Confirm The Source Is The Final Authoritative Version

The most fundamental completeness question is whether the source provided is the version that should be translated. Translating a draft or superseded version wastes the entire effort.

Confirm with the requester that the source is final and authoritative. Ask explicitly whether later versions exist, whether the document is still being edited, and whether any pending changes are expected. Where version control is in place, verify the version identifier matches the final. A source that is still in flux should not be translated until frozen, because translating a moving target produces rework every time the source changes. If the requester cannot confirm finality, treat the source as provisional and state that assumption before starting.

### Check Structural Completeness Against Expectations

A complete document has an expected structure, and deviations from that structure signal missing content. Verify the source against the structure it should have.

Compare the document's table of contents, section list, or heading hierarchy against what a complete version should contain. Check that every numbered section, chapter, or clause referenced in the contents actually appears in the body. Check that the document has a logical ending rather than cutting off mid-thought, which signals truncation. For multi-part documents, confirm all parts are present. A structural check catches the large omissions: a missing chapter, a truncated section, an absent conclusion. These are easy to miss when reading linearly, because the reader simply reaches the end without knowing something was supposed to be there.

### Verify All Referenced Assets Are Included

Sources routinely reference assets that are not embedded in the file provided: figures, tables, images, appendices, linked documents, and external references. Each unreferenced-included asset is a gap.

List every reference the source makes to external or embedded assets: Figure 1, Table 3, Appendix B, see attached schedule, per Document X. For each, confirm the asset is actually present. A reference to a figure that is not included is a completeness defect, because the target reader will encounter the reference and find nothing. A reference to an appendix that was never attached means the translation is missing content the reader needs. Asset verification is tedious but essential, because missing assets are invisible until the reader hits the dangling reference.

### Detect Truncation And Partial Content

Content can be truncated at any stage of transfer: a PDF export that cut a section, an email attachment that stripped images, a copy-paste that dropped the end, a file transfer that corrupted the tail. Detect truncation deliberately.

Look for the signs of truncation: a document that ends abruptly mid-sentence or mid-section, a word count or page count far below expectation, images that appear as broken placeholders, sections that begin but never conclude, and tables that cut off mid-row. Where the source was converted between formats, truncation is more likely, so treat converted sources with extra suspicion. If truncation is suspected, request the complete source rather than translating the partial version and hoping the rest does not matter.

### Confirm Linked And Dependent Content Is Provided

Modern content is rarely a single file. A website page links to other pages, a software build references string files and help content, a contract references schedules and exhibits, a manual references separate quick-start guides. Each link is a potential completeness gap.

Map the content's dependencies: what does this source link to, include by reference, or require to function? For each dependency, confirm whether it is in scope and whether it has been provided. A translation of a webpage that omits the linked pages it points to may be structurally incomplete from the reader's perspective. A translation of a contract without its schedules translates the shell but not the substance. Dependencies must be identified, scoped, and provided before the source can be treated as complete.

### Verify No Translatable Surface Is Omitted

Completeness is not only about whole sections; it includes every surface that carries language. A source can be structurally complete and still omit translatable surfaces that the requester expects translated.

Check that all translatable surfaces are present and accounted for: body text, headings, captions, footnotes, headers, footers, alt text, table content, embedded text in images, metadata, and UI strings. A source that includes images but not their editable text layers omits a translatable surface. A source that includes a PDF but not the source document omits the surfaces needed to produce a clean localized file. Surface completeness is what prevents a deliverable that translates the obvious text and misses the captions, tooltips, and metadata the requester assumed were included.

### Lock The Source And Record Its State

Once completeness is verified, lock the source so that translation proceeds against a stable artifact. A source that changes after translation begins invalidates the work.

Record the source's state at lock: the file name, version, date, word count, and a manifest of included assets and surfaces. This record is the baseline against which any later change is measured. If the requester later provides a revised source, the change is visible and its impact on scope and effort can be assessed, rather than being silently absorbed. A locked, recorded source protects the translator from invisible scope expansion and gives both parties a shared reference for what was translated.

## Common Traps

### Assuming A Readable Source Is Complete

A document that reads coherently does not advertise its missing pieces. Verify structure, references, and assets actively rather than trusting readability.

### Translating A Non-Final Version

Translating a draft or superseded version wastes the entire effort when the final arrives. Confirm finality and version before starting.

### Missing Dangling References To Absent Assets

References to figures, appendices, and linked documents that are not included are completeness defects. Verify every referenced asset is present.

### Ignoring Truncation Signs

Abrupt endings, low word counts, and broken placeholders signal truncation. Investigate rather than translating the partial version.

### Forgetting Content Dependencies

Linked pages, schedules, exhibits, and referenced files are part of the content from the reader's perspective. Map and confirm dependencies.

### Overlooking Non-Body Translatable Surfaces

Captions, alt text, metadata, and image text are translatable surfaces. A structurally complete source can still omit them.

### Not Locking Or Recording The Source State

A source that changes after translation begins causes invisible rework. Lock and record the source baseline so changes are visible.

## Self-Check

Before locking a source for translation, verify:

- The requester confirmed the source is the final, authoritative version, with no pending edits or later versions.
- Structural completeness was checked against the expected table of contents, section list, and heading hierarchy, with no missing or truncated sections.
- Every referenced asset, including figures, tables, appendices, schedules, and linked documents, is present and included.
- Truncation signs were investigated, including abrupt endings, low word counts, broken placeholders, and format-conversion artifacts.
- Content dependencies, including linked pages, referenced files, and exhibits, were mapped, scoped, and confirmed provided.
- All translatable surfaces are present, including body, headings, captions, footnotes, headers, footers, alt text, table content, image text, metadata, and UI strings.
- The source was locked and its state recorded, with file name, version, date, word count, and an asset and surface manifest.
- No section, asset, or translatable surface assumed to be present is actually missing or partial.
- A later source change would be visible against the recorded baseline rather than silently absorbed.
- The translation will proceed against a complete, final, authoritative source rather than a fragment presented as a whole.
