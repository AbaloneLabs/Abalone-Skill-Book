---
name: scope_and_deliverable_definition.md
description: Use when the agent is scoping a translation project, defining what is in and out of scope, counting words and segments, identifying translatable versus non-translatable content, estimating effort, or agreeing deliverable boundaries before a localization or translation assignment begins.
---

# Scope And Deliverable Definition

Scope is where translation projects fail quietly. A requester says translate the website or translate the manual, and the translator starts converting visible text. Later it emerges that screenshots, alt text, metadata, video captions, downloadable PDFs, UI tooltips, error logs, legal disclaimers, and third-party quotes were also part of the job, or were explicitly excluded, or were owned by a different team. Scope errors cause underbidding, missed deadlines, partial deliverables, and disputes about what was promised.

Use this skill when defining the boundaries of a translation or localization assignment, when estimating volume and effort, when deciding what is translatable, or when negotiating deliverable scope with a requester. The goal is to make the boundary between included and excluded work explicit and verifiable before drafting starts.

## Core Rules

### Inventory Before Estimating

Do not estimate scope from a sample or from memory of similar projects. Walk the actual deliverable and inventory every translatable surface.

For a document, inventory headings, body, tables, captions, footnotes, headers, footers, callouts, alt text, and embedded text in images. For software, inventory UI strings, tooltips, error messages, emails, help pages, in-app legal text, push notifications, and metadata. For a website, inventory page text, navigation labels, form labels, button labels, SEO metadata, structured data, PDFs, and media captions. For multimedia, inventory spoken audio, on-screen text, subtitles, and dubbing scripts.

Record the inventory so the requester can confirm it. An inventory that the requester signs off on becomes the scope contract.

### Separate Translatable From Non-Translatable Content

Not everything that contains words should be translated. Decide deliberately for each category.

Typically non-translatable content includes code, variables, placeholders, URLs, email addresses, file paths, API keys, trademarks left in source form, legal entity names that must remain unchanged, citations to source-language law, and product names protected as brand assets. Typically translatable content includes user-facing prose, UI labels, help text, marketing copy, and legal text that must be understood by the target reader.

When a category is ambiguous, such as a product name that is sometimes localized, record the rule. Do not guess per occurrence, because inconsistency will result.

### Define Update And Repetition Handling

Scope is not only the current volume. It includes how updates, repetitions, and matches are handled.

Clarify whether the deliverable is one-time or ongoing, whether updated source must be re-translated and under what turnaround, how exact and fuzzy translation memory matches are counted and priced, whether internal repetitions are counted once or per occurrence, and whether locked or pre-translated segments are excluded. These rules change both effort and cost dramatically.

If the requester has not stated these rules, propose defaults and confirm them in writing.

### Bound The Deliverable Format

Deliverable scope includes format, not just text. State what the translator will return.

Define whether the deliverable is clean target text only, bilingual table, tracked changes, localized files reinserted into the original format, localized files plus a change log, glossary extracts, translator notes, or a side-by-side comparison. Define whether formatting, layout, image localization, font embedding, and right-to-left or vertical layout adjustments are included.

Format scope errors cause deliverables that the requester cannot use without extra engineering, even when the text is correct.

### Estimate Effort Honestly

Effort is not proportional to word count alone. A 1,000-word legal clause with dense terminology and cross-references takes longer than 1,000 words of casual marketing copy. A 1,000-word software string set with context gaps and concatenation risk takes longer than 1,000 words of continuous prose.

Adjust effort estimates for terminology density, ambiguity, formatting complexity, research requirements, review cycles, and tooling overhead. State the assumptions behind the estimate. If the estimate assumes a glossary exists and it does not, the estimate is wrong.

### Identify Dependencies And Handoffs

Scope often depends on inputs the translator does not control. Identify dependencies early.

Common dependencies include source finalization, glossary approval, style guide delivery, access to the content management system or localization platform, context screenshots, design files, subject-matter expert availability for queries, and review team availability. A dependency that slips delays the deliverable even when the translator is ready.

List dependencies in the scope document with owners and dates.

### Make Exclusions Explicit

Exclusions are as important as inclusions. A scope that lists only what is included leaves the boundary fuzzy.

State explicitly what is out of scope. Common exclusions include source editing or copywriting, desktop publishing beyond text insertion, image recreation, voice recording, subtitling timing adjustment, legal review, clinical review, accessibility auditing, and testing. When a requester assumes one of these is included, a late dispute is likely.

### Plan For Scope Change

Scope changes during a project. Plan the change path rather than pretending the scope is frozen.

Define how additions are requested, how they affect effort and deadline, who approves them, and how they are recorded. A lightweight change log prevents scope creep from becoming a silent overrun.

## Common Traps

### Estimating From Word Count Alone

Word count is a starting point, not an effort model. Dense, ambiguous, or highly formatted content can take several times longer per word than plain prose. Estimates built on raw word count underprice hard jobs.

### Forgetting Hidden Text

Alt text, tooltips, metadata, captions, headers, footers, and embedded image text are easy to miss in a visual scan. A deliverable that omits them is incomplete even when the main body is translated.

### Assuming Screenshots Are Out Of Scope

Localized screenshots are often expected but rarely listed. If the translator does not recreate or replace image text, the deliverable shows source-language screenshots inside target-language pages, which looks unfinished.

### Counting Repetitions Incorrectly

Counting every repetition at full rate overcharges a job with translation memory. Counting repetitions only once undercharges a job where each occurrence must be verified separately. Confirm the rule with the requester.

### Promising A Format The Tooling Cannot Produce

If the source is a complex layout, a scanned PDF, or a proprietary format, promising clean reinserted output without checking tooling leads to broken deliverables. Verify format handling before committing.

### Leaving Context Dependencies Implicit

If the translator needs screenshots, glossary access, or expert queries to do the job, and these are not listed as dependencies, the project stalls silently when they are missing.

## Self-Check

Before finalizing scope or starting work under a scope agreement, verify:

- A full inventory of translatable surfaces exists and the requester has confirmed it.
- Translatable and non-translatable categories are classified, with rules recorded for ambiguous cases.
- Update, repetition, and translation memory match handling is defined.
- Deliverable format is specified, including whether layout, image, and format reinsertion are included.
- Effort estimate reflects content difficulty, not just word count, and its assumptions are stated.
- Dependencies such as source finalization, glossary, tooling access, and expert availability are listed with owners.
- Exclusions are stated explicitly, not merely implied by the inclusion list.
- A scope change path exists for additions discovered during the project.
- The scope document distinguishes one-time work from ongoing maintenance.
- No translatable surface has been silently dropped because it was inconvenient to inventory.
