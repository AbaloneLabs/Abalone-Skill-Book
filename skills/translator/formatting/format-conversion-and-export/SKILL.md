---
name: format_conversion_and_export.md
description: Use when the agent is converting translated content between formats such as DOCX to Markdown XML to JSON HTML to PDF, exporting from a CAT tool or translation environment to a final deliverable format, handling filter and parser selection during round-trip conversion, or ensuring that format conversion does not silently drop tags styling references or translated content during export.
---

# Format Conversion And Export

Translation frequently requires moving content between formats. A client delivers a Word document that must be translated in a CAT tool and returned as Markdown for a documentation system. Source arrives as XML exported from a content management system and must come back as the same XML plus a translated PDF for review. A help center exports HTML that must be translated and reimported without losing its structure. Each conversion is a round-trip through filters, parsers, and export pipelines, and each step is a place where content can be silently lost, corrupted, or altered. The judgment problem is that format conversion feels mechanical, so it is treated as a push-button operation, when in fact every conversion involves interpretation: which elements are translatable, which formatting maps to which target construct, which features have no equivalent in the destination, and which information the filter silently discards. A translator who treats conversion as automatic discovers, usually at delivery, that styling vanished, tags were stripped, cross-references broke, or entire sections disappeared.

Agents miss conversion problems because the source and target formats look superficially similar, because CAT tool filters hide their decisions, and because loss is invisible until someone opens the exported file in its destination environment. This skill exists to make conversion and export a deliberate, verified process rather than a trusted button.

## Core Rules

### Understand That Every Conversion Is Lossy Until Proven Otherwise

Approach format conversion with the assumption that something will be lost or changed unless you verify otherwise. Filters make choices: a DOCX-to-Markdown filter may drop text boxes, headers, footers, tracked changes, or complex table formatting that has no Markdown equivalent; an XML filter may silently skip elements it does not recognize; an HTML export may strip attributes it considers non-standard; a PDF export may flatten interactive elements. Before relying on any conversion, identify what the source format contains that the destination format cannot represent, decide how each gap will be handled, and document the lossy mappings. Treating conversion as lossless by default is the foundational error.

### Select The Filter Or Parser For The Specific Format And Version

Generic filters applied to specific formats produce wrong results. A DOCX is a zipped XML package, not a flat document; an HTML file may be HTML4, XHTML, or HTML5 with different parsing rules; an XML file follows a specific schema or DTD that defines which elements are translatable; a properties file may use Java, ISO-8859-1, or UTF-8 encoding conventions. Select the filter matched to the exact format and version, configure it for the source's schema or structure, and verify its output on a representative sample before processing the full content. A filter mismatch can expose the wrong text for translation, lock content that should be translated, or break structure on reimport.

### Map Formatting And Structure Across The Conversion

Formatting and structure rarely map one-to-one between formats. Word styles such as Heading 1 or Quote map to Markdown prefixes, but character-level formatting like color, font size, or highlight may have no Markdown equivalent and must be dropped or approximated. XML elements map to JSON objects, but attributes, mixed content, and ordering do not map cleanly and require a documented strategy. HTML tables map to Markdown tables only when simple; complex tables with merged cells or nested content do not convert cleanly. Before converting, build the mapping table for the formats involved, decide how each source construct is represented in the target, and flag constructs that cannot be represented. A conversion without an explicit mapping produces arbitrary, inconsistent results.

### Preserve References Links And Cross-References Through Conversion

References are the most fragile element in conversion. Internal links, cross-references, footnotes, image references, table and figure captions, and index entries all depend on identifiers and structure that conversion can break. An anchor ID may be stripped, a link target may be rewritten, a footnote may be flattened to inline text, an image reference may be lost when the image is not exported alongside. Identify every reference in the source, confirm it survives the filter, and verify it resolves in the exported output. For translated content, confirm that references still point to the correct targets after the surrounding text changed, because some reference schemes are positional.

### Verify The Round-Trip Not Just The Export

Many workflows require a round-trip: source format to bilingual to source format again, or source format to intermediate to final deliverable. Each leg can introduce loss. Verify the complete round-trip, not just the final export, by diffing the structure of the exported file against the source structure where the format allows, and by checking that every translatable segment present in the source is present and translated in the output. A round-trip that drops segments silently is a common and severe defect, because the missing translation is invisible in a file that otherwise looks complete.

### Control Encoding And Metadata During Export

Export writes a new file, and the new file's encoding, line endings, and metadata must be correct for the destination. Confirm the exported file uses the encoding the destination expects, typically UTF-8, with the correct byte order mark or none as required. Confirm line endings match the platform convention where it matters. Confirm that metadata such as language declarations, charset attributes, and document properties are set to the target locale, not inherited stale from the source. An export that writes correct text with wrong encoding or stale language metadata will fail in the destination environment.

### Validate Export In The Destination Environment

The exported file must be validated in the environment where it will actually be used, not just in the tool that produced it. Open the Markdown in the target documentation renderer, not just a text editor. Load the XML into the consuming application. Render the HTML in the target browser. Compile the help file. Import the translated resources into the build. Each destination environment applies its own parsing and validation, and defects that are invisible in the export tool appear immediately in the destination. Validation in the producing tool is necessary but not sufficient.

## Common Traps

### Trusting Conversion As Lossless

Assuming a filter preserves everything leads to silent loss of styling, references, or entire sections discovered only at delivery. The trap is that the exported file looks complete at a glance.

### Using A Generic Filter For A Specific Format

A generic DOCX, HTML, or XML filter applied to a specific schema exposes the wrong content or breaks structure. The trap is that it appears to work on a simple sample.

### Ignoring Formatting That Has No Target Equivalent

Color, highlight, text boxes, and complex tables may vanish in conversion. The trap is that the prose survives, hiding the structural loss.

### Breaking References During Conversion

Stripped anchor IDs, rewritten link targets, and flattened footnotes break navigation. The trap is that links look present but resolve to nothing.

### Skipping The Round-Trip Check

Verifying only the final export misses segments dropped during an intermediate leg. The trap is confidence in a file that is missing translations.

### Inheriting Stale Metadata And Encoding

Wrong language declarations, charset attributes, or line endings inherited from the source cause failures in the destination. The trap is that the text is correct.

### Validating Only In The Producing Tool

Defects invisible in the export tool appear in the destination environment. The trap is that the tool reports success.

## Self-Check

- Have you identified what the source format contains that the destination format cannot represent, and documented how each gap is handled?
- Is the filter or parser selected and configured for the exact format and version of the source, verified on a representative sample?
- Is there an explicit mapping table for formatting and structure across the conversion, with constructs that cannot be represented clearly flagged?
- Do all references, links, cross-references, footnotes, and image references survive the filter and resolve correctly in the exported output?
- Has the full round-trip been verified, confirming that every translatable segment in the source is present and translated in the output?
- Does the exported file use the correct encoding, line endings, and target-locale metadata rather than inheriting stale source values?
- Has the exported file been validated in the actual destination environment, not just the producing tool?
- Are any lossy mappings or dropped features communicated to the client rather than silently shipped?
