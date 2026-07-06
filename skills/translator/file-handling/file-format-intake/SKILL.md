---
name: file_format_intake.md
description: Use when the agent is receiving source files for translation, identifying file types and versions, assessing whether files are translation-ready, detecting embedded or hidden content, evaluating source quality before committing to work, handling unsupported or proprietary formats, or deciding whether to accept reject or query files at the start of a translation project.
---

# File Format Intake

Intake is the moment a translation project becomes real. Source files arrive from a client, a content management system, or an upstream workflow, and the translator or project manager must decide what they have, whether it is usable, and what it will take to turn it into a clean deliverable. This is a judgment problem, not an unpacking task, because the consequences of a bad intake decision propagate through the entire project. A file that looks like a simple document may contain embedded objects, tracked changes, hidden text, comments, or linked content that is not immediately visible. A format that appears supported may be a version the tools cannot parse. A file set that seems complete may be missing the fonts, the linked images, the style guide, or the reference materials that make translation possible. Agents miss issues at intake because the files look normal on the surface, because the pressure to start work discourages careful inspection, and because the problems are invisible until they surface mid-project as rework, missed deadlines, or broken deliverables. The harm this skill prevents is projects that start on a false foundation: incomplete source, untranslatable content, hidden material that ships untranslated, and scope that expands after the estimate is fixed.

Use this skill when receiving source files at the start of a translation project, identifying file types and versions, assessing translation readiness, detecting embedded or hidden content, handling unsupported formats, or deciding whether to accept, reject, or query files before committing to work. The goal is a clean, verified, complete source set that the project can be built on.

## Core Rules

### Inventory Every File And Confirm The Set Is Complete

The first intake action is a complete inventory: what files arrived, in what formats, and is anything missing. Compare the delivered set against what the project requires, including the primary source files, any linked or embedded assets such as images with translatable text, fonts required to render the source, reference materials such as previous translations or termbases, style guides, and context files such as screenshots or working builds. A file set that contains the main document but not the linked images, or the source strings but not the screenshots that show their context, is incomplete and will produce a weaker deliverable. Confirm completeness explicitly and request missing items before starting work, because discovering a missing dependency mid-project forces a stop and often a renegotiation. Treat the inventory as a contract: what was received, when, and what is still outstanding.

### Identify The Exact Format And Version Of Every File

File extensions are unreliable signals of format. A `.doc` may be an old binary format or a renamed XML format; an `.xml` may follow any of dozens of schemas; an `.html` may be HTML4, XHTML, or HTML5; a `.json` may be a flat key-value file or a deeply nested structure; a `.csv` may use different delimiters, encodings, and quoting. The agent must identify the exact format and version of each file, not assume from the extension, because tool and filter selection depends on the precise format. Open the file and inspect its structure, or use a format-detection tool. Where the format is proprietary or unusual, confirm whether the available tools can parse it before committing. A file whose format is misidentified will be processed with the wrong filter, exposing the wrong content or breaking structure, and the error propagates across the entire project before it is noticed.

### Detect Embedded Hidden And Linked Content

Files often contain more than the visible text. Word documents may have tracked changes, comments, hidden text, text boxes, headers and footers, and embedded objects. Spreadsheets may have hidden sheets, hidden columns, and cell comments. Presentation files may have speaker notes and hidden slides. XML and HTML may contain translatable attributes, metadata, or commented-out content. InDesign and other layout files may contain text in linked stories or anchored objects. The agent must scan for all of these, because hidden content that is translatable but not extracted ships untranslated in the deliverable, and hidden content that is not translatable but is exposed pollutes the translation set. Decide for each type of hidden content whether it is in scope, and document the decision. A deliverable that ships with untranslated comments or hidden text is incomplete, and the omission is invisible until the client finds it.

### Assess Source Quality And Translation Readiness

Before committing to work, assess whether the source is ready to translate. Source quality problems include inconsistent terminology, unedited or draft text, ambiguous references, broken cross-references, placeholder or lorem-ipsum text that was never replaced, and formatting errors. A source that is not final, still being edited, or internally inconsistent will produce a translation that inherits those problems, and the translator will be blamed for issues that originated upstream. Flag source quality issues at intake, decide with the client whether to proceed against an imperfect source or require source correction first, and document the state of the source so that later defects can be traced to their origin. Proceeding against a known-bad source without flagging it absorbs the blame for upstream problems.

### Evaluate Tool And Filter Compatibility

Once the format and content are understood, confirm that the available tools and filters can handle the files. Test the filter on a sample to verify that it extracts the correct translatable content, locks the correct structural elements, and round-trips cleanly. Some formats require specific filter configuration, custom rules files, or specialized tools that may not be immediately available. A format that the tools cannot parse must be resolved at intake: convert to a workable format with documented loss, acquire the needed tool, or decline the work. Discovering mid-project that a file cannot be processed forces an emergency that damages schedule and trust. Resolve tool and filter compatibility before committing to a deadline.

### Decide Accept Reject Or Query With Documented Rationale

Intake culminates in a decision for each file or file set: accept as-is, accept with conditions or queries, or reject. Accept means the file is complete, correctly identified, tool-compatible, and ready to translate. Accept with conditions means the file is usable but has defined issues, hidden content in scope, source quality concerns, missing references, that must be resolved or worked around, and the conditions are documented. Reject means the file cannot be translated as delivered, wrong format, corrupt file, incomplete set, and must be returned to the client with a specific reason. Every decision should be documented with its rationale, because the intake record is the baseline against which scope changes and later defects are measured. An undocumented intake decision cannot be defended when the client disputes scope or quality later.

## Common Traps

### Starting Translation Before Confirming The File Set Is Complete

Beginning work on the main document while linked images, fonts, or references are still missing leads to a deliverable that cannot be assembled or that ships without translated embedded text. This is a trap because the visible file looks sufficient and the gaps only surface at assembly. Always inventory and confirm completeness first.

### Trusting The File Extension To Indicate The Format

Assuming a `.doc` or `.xml` is a specific format leads to processing with the wrong filter, exposing wrong content or breaking structure. This is a trap because the error is uniform and silent, affecting the whole file before it is noticed. Identify the exact format by inspection.

### Missing Hidden Or Embedded Translatable Content

Tracked changes, comments, hidden sheets, speaker notes, and text boxes contain translatable text that surface extraction often skips. This is a trap because the deliverable looks complete but ships with untranslated content the client finds later. Scan explicitly for hidden content and decide scope per type.

### Proceeding Against A Known-Bad Source Without Flagging

Translating an inconsistent, draft, or broken source inherits its problems and assigns blame to the translator. This is a trap because the pressure to start discourages the flagging that would protect the project. Flag source quality at intake and document the source state.

### Assuming The Tools Can Handle The Format Without Testing

Committing to a deadline before verifying filter compatibility risks a mid-project emergency when a file cannot be parsed. This is a trap because the format looks standard and the problem only appears during processing. Test the filter on a sample at intake.

### Accepting Files Without Documenting Conditions Or Rationale

An undocumented intake cannot be defended when scope or quality is later disputed. This is a trap because the decision felt clear at the time but leaves no trace. Record the intake decision and its rationale for every file set.

## Self-Check

Before accepting source files and starting translation, verify:

- A complete inventory of all delivered files exists, and any missing primary files, linked assets, fonts, references, or context materials have been identified and requested.
- The exact format and version of every file has been identified by inspection, not assumed from the extension, and any unusual or proprietary formats have been confirmed as tool-supported.
- All embedded, hidden, and linked content, including tracked changes, comments, hidden sheets, speaker notes, text boxes, and translatable attributes, has been scanned for, and a scope decision is documented for each.
- Source quality has been assessed for inconsistency, draft state, ambiguity, and broken references, and any concerns have been flagged to the client with a decision on whether to proceed or require correction.
- Tool and filter compatibility has been tested on a sample, confirming correct extraction, locking, and round-trip behavior before a deadline is committed.
- Each file or file set has an explicit accept, accept-with-conditions, or reject decision, documented with rationale.
- No file is being translated whose format, completeness, or quality is uncertain, and no hidden translatable content has been silently dropped from scope.
- The intake record establishes a baseline against which later scope changes and source-origin defects can be traced.
