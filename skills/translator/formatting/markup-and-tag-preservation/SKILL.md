---
name: markup_and_tag_preservation.md
description: Use when the agent is translating inside tagged or markup formats such as XML HTML Markdown DITA YAML properties or JSON, protecting inline tags and placeholders during translation, moving tags to match target word order, preventing tag deletion or duplication, or verifying that structural markup survives the bilingual round-trip without breaking parsing.
---

# Markup And Tag Preservation

Translation almost never happens on bare prose. Source content is wrapped in markup: HTML elements, XML tags, Markdown syntax, DITA topics, YAML keys, software resource files, and inline formatting tags embedded inside sentences. The translator's deliverable is judged on two axes at once. Linguistically the text must be accurate, fluent, and culturally appropriate. Structurally the file must round-trip back into its original format and behave identically to the source. The judgment problem is that these two axes compete for the translator's attention, and structural correctness is the one most often sacrificed because it is less visible during sentence-by-sentence work. A tag that is deleted, translated, duplicated, or misplaced does not look wrong inside the segment view; it looks wrong only when the file fails to parse, a link breaks, a placeholder resolves to nothing, or formatting collapses in the rendered output. By then the deliverable is already broken.

Agents miss markup problems because CAT tools hide tags behind friendly placeholders, because inline tags feel decorative rather than functional, and because the consequences are deferred to a merge step the translator may never run. This skill exists to make structural preservation an explicit, first-class concern that is verified before delivery, not discovered afterward.

## Core Rules

### Treat Tags As Functional Contracts Not Decoration

Every tag, placeholder, variable, and code span in a source segment carries a functional contract with the system that will consume the file. An inline `<b>` tag pair marks which target words should be bold; a `{name}` placeholder tells the application where to interpolate a runtime value; a Markdown link `[text](url)` binds a label to a destination; a printf-style `%s` or `%d` declares an argument type and position. Treating any of these as optional decoration is the root error. Before translating a segment, identify every protected element and read it as an instruction to the rendering engine, not as text. The discipline is to translate the human-readable runs and carry the functional elements across intact, in the correct count, in the correct order, and in positions that make sense for the target grammar.

### Lock The Translatable Boundary Before Translating

Before any text is touched, establish and verify the boundary between translatable content and structural markup. In XML and HTML this means deciding which elements and attributes are translatable; `alt`, `title`, and `aria-label` are usually translatable while `class`, `id`, and `href` are not. In JSON and YAML the keys are almost always locked and the values are exposed, but some value strings are themselves keys or identifiers that must stay locked. In software resources, identifiers, keys, and placeholder syntax are locked while display strings are exposed. In Markdown, syntax characters are locked while prose runs are exposed. An incorrect boundary produces one of two failures: structural elements get translated and the file breaks, or genuine content stays locked and ships untranslated. Both are unacceptable, and both are preventable by testing the parser on a sample before committing to full translation.

### Move Inline Tags To Match Target Word Order

Inline tags are the hardest case because they sit inside a sentence and target word order rarely matches source word order. When translating "Click the <b>Submit</b> button to continue", the bold tags must wrap the translated word for "Submit", not sit where they sat in English. This requires the translator to relocate tag pairs deliberately, preserving opening and closing pairing, preserving nesting depth, and never splitting a pair across a boundary that breaks it. A common failure is to leave tags in their source position, which bolds the wrong words or breaks the link target. Another is to duplicate a tag so the pair count changes. The rule is that the set of protected elements in the target segment must be exactly the set from the source, each appearing exactly once, arranged so that the formatting or linking they denote lands on the correct target text.

### Preserve Placeholder Syntax Position And Argument Order

Placeholders encode both a contract and, in many formats, an argument order. A source string "Welcome, {0}! You have {1} messages." uses positional placeholders that the application fills in order. If the target language naturally reverses the greeting and the message count, the translator must either reorder the surrounding text while keeping `{0}` and `{1}` bound to the correct arguments, or, only where the format explicitly supports it, use named or renumbered placeholders. Renumbering positional placeholders without confirming the format supports it silently swaps which runtime value appears where. Preserve the exact syntax of each placeholder including braces, percent specifiers, type letters, and any flags, because a `{name}` and a `%s` are not interchangeable and a corrupted brace can crash the formatter.

### Respect Whitespace And Segmentation Sensitivity

Many formats are whitespace-sensitive. YAML uses indentation for structure; Markdown uses leading spaces and blank lines to distinguish code blocks, lists, and paragraphs; properties files treat trailing spaces and line continuations literally. Translating inside these formats means preserving the structural whitespace exactly and changing only the prose. Adding or removing a leading space can demote a Markdown heading or break a YAML mapping. Line breaks inside a segment can split a properties value. The translator must know which whitespace is structural and which is merely presentational, and must never normalize structural whitespace during translation.

### Escape Characters The Target Format Treats As Special

Target text can introduce characters that the format treats as syntax. An ampersand in HTML or XML must become `&amp;`; a double quote inside a JSON string must be escaped as `\"`; a backslash in a properties value may need doubling; a literal brace in a format string that uses braces for placeholders must be doubled to print literally. The translator must recognize when the target prose introduces such characters and apply the correct escaping, while never double-escaping characters that are already escaped in the source. Configure the tool to handle this automatically where possible, but verify the result because over-escaping produces visible `&amp;amp;` artifacts and under-escaping produces parse failures.

### Verify By Merging And Parsing Not By Reading The Bilingual View

The bilingual segment view is not authoritative evidence that the file is correct. Tags can look perfect in the editor and still break on merge, because merge reassembles the file according to the format's rules and exposes errors that the editor hides. The only reliable verification is to run the actual merge, open the output in a native parser or renderer for the format, and confirm it parses and renders. For XML, validate against the schema or at least a well-formedness check. For JSON, load it. For Markdown, render it. For software resources, compile them. Delivering from the bilingual view without merge validation is the single most common cause of broken-format deliverables.

## Common Traps

### Translating Or Deleting Tags Because They Look Like Text

Tags rendered as readable placeholders can be mistaken for content, especially short element names or attribute values that resemble words. Translating or deleting them corrupts the structure. The trap is that the segment looks cleaner afterward, so the error feels like an improvement.

### Leaving Inline Tags In Source Position

When word order changes, leaving tags where English put them bolds, links, or italicizes the wrong target words. The trap is that the file still parses, so the structural check passes while the formatting is semantically wrong.

### Splitting Or Duplicating Tag Pairs

Splitting an opening tag from its closing partner across segments, or duplicating a tag to make matching easier, changes the pair count and breaks rendering. The trap is that each segment looks individually correct.

### Renumbering Positional Placeholders Without Format Support

Swapping `{0}` and `{1}` to match reversed word order silently swaps runtime values when the format does not support reordering. The trap is that the text reads naturally while the data is wrong.

### Normalizing Structural Whitespace

Collapsing indentation, trimming trailing spaces, or reflowing lines in YAML, Markdown, or properties files changes the structure. The trap is that the text looks tidier.

### Trusting The Bilingual View Over The Merged File

A clean segment view does not prove the merged file parses. The trap is confidence based on the wrong evidence, shipping a file that fails only when the client opens it.

## Self-Check

- For every segment, can you enumerate each protected element (tag, placeholder, variable, code span) and confirm it appears in the target exactly once with identical syntax?
- Is the translatable boundary defined and tested on a sample before full translation, with no structural element exposed for translation and no genuine content locked?
- For segments with inline tags, do the tag pairs wrap the correct target words after reordering, with pairing and nesting preserved?
- Are positional placeholders kept bound to their correct arguments, with any reordering confirmed against what the format supports?
- Is structural whitespace in YAML, Markdown, properties, and similar formats preserved exactly, with only prose changed?
- Are characters special to the target format escaped correctly, with no double-escaping of already-escaped source content?
- Has the merged output been opened in a native parser or renderer for its format and confirmed to parse and render without errors?
- Does the final file contain exactly the same set of protected elements as the source, with none translated, deleted, or duplicated?
