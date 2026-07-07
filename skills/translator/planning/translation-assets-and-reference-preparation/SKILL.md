---
name: translation_assets_and_reference_preparation.md
description: Use when the agent is preparing translation assets before a project, building or updating termbases style guides and translation memory, gathering reference materials and context, setting up CAT tools and query logs, or ensuring linguists have everything they need before translation begins so quality is not undermined by missing assets.
---

# Translation Assets And Reference Preparation

Translation quality is determined before the first segment is translated, by whether the linguist has what they need. A translator working without a termbase will invent terminology that drifts across the document and conflicts with other translators. A translator without a style guide will apply their own register and conventions, producing inconsistency. A translator without context will guess at ambiguous strings, and a translator without reference materials will not know how the product or domain actually works. Yet asset preparation is routinely skipped or treated as optional, because it is invisible work that does not produce translated words. The cost appears later, as rework, inconsistency, queries that stall production, and quality defects that trace back to a missing asset the translator never received. Preparing translation assets and references is the foundational planning step that makes every downstream stage more efficient and more consistent. An agent who hands a translator files and a deadline, without assets, is setting up the translation to fail in predictable ways.

Use this skill when preparing translation assets, building termbases and style guides, gathering references and context, setting up tools and query logs, or ensuring linguists have what they need before translation begins. The goal is to make quality and consistency possible by providing complete, accurate, and usable assets before production starts.

## Core Rules

### Prepare The Termbase Before Translation Starts

The termbase is the single most powerful consistency tool, and it must exist before translation begins, not be built during or after. Prepare it first.

Extract terminology from the source content, identifying the terms that carry domain meaning and require consistent rendering. For each term, research and validate the target-language equivalent, following terminology research discipline, and record the entry with concept definition, domain, locale, status, and source citation. Where an existing termbase covers the domain, reconcile and extend it rather than starting fresh. Deliver the termbase to every translator and vendor at handoff, and require its application.

A termbase built after translation cannot enforce consistency retroactively; it can only document the inconsistency that already occurred.

### Build Or Update The Style Guide

A style guide defines the conventions that govern the target text beyond terminology: register, tone, formality, punctuation, formatting, and language-specific rules. Provide one.

If a domain or client style guide exists, adapt it to the target locale, because style conventions differ across languages and regions. If none exists, create a lightweight guide capturing the decisions that matter for this project: the target register and audience, formality level, punctuation and quotation conventions, treatment of numbers, dates, and units, handling of source-language terms retained in the target, and voice and tone guidance. Deliver the style guide with the termbase at handoff.

Without a style guide, each translator applies personal conventions, and the document reads as if written by several different authors.

### Gather Reference Materials And Domain Context

Translators cannot render what they do not understand, and understanding requires reference materials and context. Gather and provide them.

Collect domain references: product documentation, specifications, prior translations, competitor or parallel texts, glossaries from regulators or standards bodies, and any material that explains how the domain works. Collect product context: screenshots, design files, access to a build, string IDs, and developer comments, especially for UI and software content. Collect client context: brand voice guidelines, prior approved translations, and the brief defining audience and purpose. Deliver references organized so the translator can find what they need, not as an undifferentiated dump.

References delivered as a chaotic pile are almost as useless as no references, because the translator cannot find what matters.

### Prepare And Verify Translation Memory

Translation memory creates leverage and consistency, but only if it is prepared and verified before use. Prepare it.

Check that the memory is relevant to the current content, domain, and locale, because applying an unrelated memory introduces inconsistent renderings. Clean the memory of corrupted, misaligned, or low-quality segments that would propagate defects. Determine the match threshold for pre-translation, because low-threshold fuzzy matches can introduce errors if applied uncritically. Configure the workflow to apply memory matches and to capture new translations back into the memory.

An unverified memory can do more harm than good, by injecting past errors or inconsistent terminology into the current project.

### Provide Context For Ambiguous And Isolated Content

Some content cannot be translated safely without context, and the translator will not seek context they do not know is missing. Provide it proactively.

For UI strings and isolated segments, provide screenshots, string locations, and the surrounding interface context. For ambiguous terms, provide the intended sense or mark the ambiguity for query. For content that assembles dynamically, provide the assembly pattern so the translator can verify the result parses. For culturally specific references, provide the background the target reader will not share. Proactive context provision prevents the translator from guessing and prevents the queries that stall production when guesses are caught late.

### Set Up The Query Log And Issue Process

Translation generates questions, and a project without a query process leaves translators to guess silently. Set up the query log and issue process before production.

Establish where queries are logged, who answers them, the turnaround for responses, and how answers are communicated to all translators and fed back into the termbase. A query process that is set up at handoff ensures questions are captured and resolved consistently, rather than each translator resolving them differently. The query log also becomes a record of source ambiguity and decisions, valuable for future projects.

### Verify Tooling Compatibility And Setup

Assets are useful only if the translator's tools can read them. Verify tooling compatibility before handoff.

Confirm the termbase, memory, and source files are in formats compatible with the translators' CAT tools. Confirm file encoding is correct, because encoding errors corrupt characters, especially in non-Latin scripts. Confirm that tags, placeholders, and markup are preserved through the tooling pipeline, because tools can strip or alter them. Confirm segment configuration matches the content type, because incorrect segmentation breaks UI strings and assembled content.

Assets delivered in incompatible formats or with encoding problems are as good as no assets, because they cannot be used.

### Confirm Asset Completeness At Handoff

Before handing off to translators, confirm that the complete asset package is assembled and delivered. A partial asset package undermines the whole.

Verify the package includes the final source content, the termbase, the style guide, translation memory, reference materials, context, the query log setup, the brief, and the quality target and acceptance criteria. Confirm the translator acknowledges receipt of the complete package, because assets sent but not received provide no protection. A checklist at handoff ensures nothing critical is missing.

## Common Traps

### Starting Translation Without A Termbase

Without a termbase, terminology drifts across the document and conflicts across translators, and the inconsistency cannot be fixed retroactively.

### Omitting A Style Guide

Without a style guide, each translator applies personal conventions, producing a document that reads as if written by several authors.

### Delivering References As An Undifferentiated Dump

References the translator cannot navigate are nearly useless; organize them so what matters is findable.

### Applying Unverified Translation Memory

An uncleaned or irrelevant memory injects past errors and inconsistent terminology into the current project.

### Letting Translators Guess At Ambiguous Content

Without proactive context, translators guess, and guesses caught late stall production and cause rework.

### No Query Process Until Questions Arise

Without a query log set up at handoff, translators resolve questions silently and inconsistently.

### Ignoring Tooling And Encoding Compatibility

Assets in incompatible formats or with encoding errors cannot be used and are as good as no assets.

### Handing Off A Partial Asset Package

A package missing the termbase, style guide, or context undermines the consistency and quality the assets were meant to enable.

## Self-Check

Before handing off content to translators, verify:

- The termbase is prepared from source terminology, with validated equivalents, and delivered to all translators and vendors.
- A style guide exists or is created, adapted to the target locale, and delivered with the termbase.
- Reference materials and domain context are gathered, organized, and delivered, including product context for UI and isolated content.
- Translation memory is verified for relevance, cleaned of defects, configured for pre-translation, and set to capture new translations.
- Context is provided proactively for ambiguous, isolated, dynamically assembled, and culturally specific content.
- A query log and issue process is set up at handoff, with defined ownership, turnaround, and feedback into the termbase.
- Tooling compatibility, file formats, encoding, tags, and segmentation are verified before handoff.
- The complete asset package is assembled and confirmed received by the translator, with nothing critical missing.
- No translator is starting work without the termbase, style guide, references, context, and query process they need.
- Asset preparation is treated as foundational, not optional, because quality and consistency depend on what the translator has before they begin.
