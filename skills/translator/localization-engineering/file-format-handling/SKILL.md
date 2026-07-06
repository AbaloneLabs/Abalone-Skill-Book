---
name: file_format_handling.md
description: Use when the agent is preparing translation files, handling markup tags and inline elements, parsing structured formats like XML JSON Markdown and DITA, protecting code and placeholders from translation, or merging translated content back into its original format without breaking structure.
---

# File Format Handling

Modern translation rarely involves plain text. Source content arrives in structured formats: XML and HTML with tags, Markdown with syntax, JSON and YAML with keys, software resource files with placeholders, DITA with topic structures, and office documents with formatting. The translator's job is to translate the text within these structures without damaging the structure itself. When format handling is done well, the translated file reassembles correctly, tags and placeholders land in the right places, and the content renders properly in its target environment. When it is done poorly, tags are translated or deleted, placeholders are corrupted, markup is broken, and the output file fails to parse or renders with errors. Format handling is an engineering responsibility that sits alongside linguistic work: the translator must understand which parts of a file are translatable text and which are structural markup, and the localization engineer must configure tools to protect the markup and expose only the text. Misjudgments here produce broken deliverables, not just poor translations.

Use this skill when preparing files for translation, handling tags and placeholders, parsing structured formats, or merging translations back into source formats. The goal is to translate the text while preserving the structural integrity of the file so it functions identically in the target language.

## Core Rules

### Distinguish Translatable Text From Structural Markup

The foundational task is distinguishing what should be translated from what must be preserved. Establish this boundary clearly.

In any structured file, some content is translatable text and some is structural: tags, attributes, code, keys, and syntax. Translating structural elements breaks the file; preserving translatable text leaves content untranslated. Configure the CAT tool or parser to lock structural elements and expose only translatable text. For XML and HTML, this means protecting tags and attributes while exposing text nodes. For JSON and YAML, it means protecting keys while exposing values. For software resources, it means protecting identifiers and placeholders while exposing user-facing strings. The boundary must be defined per format and verified before translation.

A file where the boundary is wrong will produce either broken structure or untranslated content.

### Protect Tags Placeholders And Variables

Tags, placeholders, and variables carry functional meaning and must survive translation intact. Protect them.

Inline tags such as formatting markers, placeholders for variables such as {name} or %s, and code snippets must be preserved exactly, including their syntax and position relative to the text. Configure the tool to treat these as protected, non-translatable elements that the translator can move but not alter. Verify that the tool's tag handling preserves tag pairing and nesting. A translated placeholder or a deleted tag corrupts the output: a variable that no longer resolves, a formatting tag that breaks rendering, a link that fails. Translators must understand that these elements are functional, not decorative.

Train translators to recognize and preserve protected elements, and configure tools to prevent accidental alteration.

### Handle Inline Tags Within Segments

Inline tags within a segment, such as bold or link tags inside a sentence, require special handling. Manage them.

Inline tags split a segment into text runs and must be placed correctly in the target, which may require reordering because target language word order differs. The translator must move tags to wrap the correct target text, preserving pairing and nesting. Misplaced tags produce broken formatting or corrupted rendering. Tools that display tags inline help, but the translator must verify placement. For complex inline structures, provide guidance or examples. Verify tag integrity during QA before delivery.

A segment with inline tags is not fully translated until the tags are correctly positioned in the target.

### Configure Parsers For Each Structured Format

Each structured format has its own parsing rules. Configure parsers correctly for each.

XML has many dialects, each with different translatable elements; configure the parser to the specific schema, defining which elements and attributes are translatable. Markdown requires a parser that protects syntax such as headings, links, and code spans while exposing text. DITA and other structured documentation formats have defined translatable rules that must be configured. JSON and YAML require key-value parsing with translatable values. Software resource formats such as RESX, properties, and strings have format-specific rules. A generic parser applied to a specific format will misidentify translatable content or break structure.

Use format-specific parsers and verify their output on sample files before full processing.

### Preserve Encoding And Character Declaration

File encoding affects whether characters, especially non-Latin scripts, render correctly. Preserve encoding.

Source files declare encoding, often UTF-8, in a header or byte order mark. Translated files, especially with non-Latin scripts, must preserve the correct encoding and declaration. An encoding mismatch produces mojibake, corrupted characters that render as garbage. Verify that the tool reads and writes the correct encoding, that declarations are preserved or updated, and that special characters such as quotes, dashes, and symbols are handled correctly. For HTML and XML, preserve the charset declaration; for software resources, follow the format's encoding requirements.

Encoding errors are invisible until the file is opened in its target environment, so verify before delivery.

### Verify Entity And Escape Handling

Structured formats use entities and escape sequences that must be handled correctly. Verify handling.

HTML and XML use entities such as &amp; and &lt;; JSON uses escape sequences such as \n and \"; software strings use escape sequences for special characters. The translator's target text may introduce characters that require escaping, such as an ampersand in HTML or a quote in JSON. Configure the tool to handle entities and escapes correctly: preserve existing entities, escape newly introduced characters as needed, and avoid double-escaping. Mishandled entities produce parse errors or display artifacts. Verify entity and escape integrity during QA.

### Merge Translations Back And Validate Structure

After translation, the content must be merged back into the source format. Validate the structure.

The merge process reassembles the translated text into the original file structure, restoring tags, attributes, and formatting. Validate that the merged file parses correctly in its native format: open XML and HTML in a parser, load JSON and YAML into a reader, compile software resources, and render documentation. Check for broken tags, corrupted placeholders, encoding issues, and structural errors. A file that looks correct in the CAT tool may fail when merged, so validation must happen on the final output, not the bilingual view.

Deliver only files that have been merged and validated in their target format.

### Handle Right-To-Left And BiDi In Structured Formats

Right-to-left and bidirectional scripts introduce additional format handling concerns. Address them.

RTL text in HTML and XML requires correct directionality attributes and CSS; in software resources, it may require mirroring of layout. BiDi text within structured formats can confuse parsers if directionality markers interact with tags. Verify that the tool handles RTL correctly, that directionality is set in the output, and that the rendered output displays correctly. Test RTL output in the actual rendering environment, not just the CAT tool.

## Common Traps

### Translating Tags Or Placeholders

Translating structural elements or variables corrupts the file and breaks functionality.

### Deleting Or Misplacing Inline Tags

Removing tags or placing them incorrectly in the target produces broken formatting and rendering.

### Using A Generic Parser For A Specific Format

Generic parsers misidentify translatable content and break structure; use format-specific parsers.

### Encoding Mismatches

Wrong encoding or declarations produce mojibake and corrupted characters in the target.

### Mishandling Entities And Escapes

Incorrect entity handling causes parse errors and display artifacts in the merged file.

### Skipping Post-Merge Validation

A file correct in the CAT tool may fail when merged; validate the final output in its native format.

### Ignoring Directionality For RTL Scripts

RTL text without correct directionality attributes renders incorrectly in the target environment.

### Not Verifying The Translatable Boundary

An incorrect boundary produces broken structure or untranslated content; verify before translation.

## Self-Check

Before delivering a translated file, verify:

- The boundary between translatable text and structural markup is correctly defined and verified for the format.
- Tags, placeholders, variables, and code are protected from translation and preserved exactly.
- Inline tags within segments are correctly positioned in the target, preserving pairing and nesting.
- Format-specific parsers are configured for XML, HTML, Markdown, JSON, YAML, DITA, or software resources as needed.
- File encoding and character declarations are preserved or correctly updated for the target script.
- Entities and escape sequences are handled correctly, with new characters escaped as needed and no double-escaping.
- The merged output file is validated in its native format, parsing and rendering without structural errors.
- Right-to-left and bidirectional text has correct directionality attributes and renders properly in the target environment.
- No structural element was translated, and no translatable text was left locked.
- The final file functions identically to the source in its target format and environment.
