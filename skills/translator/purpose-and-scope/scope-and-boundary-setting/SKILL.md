---
name: scope_and_boundary_setting.md
description: Use when the agent is setting the boundaries of a translation engagement, deciding what content is in or out of scope, separating translatable from non-translatable material, defining handoffs and exclusions, or negotiating the edges of a localization or translation assignment before work begins.
---

# Scope And Boundary Setting

Scope is the line that separates what the translator is responsible for from what they are not. That line is almost never as obvious as it appears. A requester hands over a document or a string file and says translate this, and the translator begins. Only later does it emerge that the screenshots embedded in the document were expected to be re-shot in the target language, that the PDF appendix was part of the deliverable, that the legal disclaimer at the footer was owned by a different team and must not be touched, or that the version provided was a draft and the final had different numbers. Scope ambiguity is the single most common source of translation disputes, underbidding, and partial deliverables. The translator who starts converting text without first drawing the boundary is volunteering for work they will not be paid for, or for blame for work they never agreed to do.

Agents miss scope setting because the visible text feels like the whole job. But translation engagements sit inside a larger content ecosystem: source versions, linked assets, recurring updates, hidden metadata, and adjacent responsibilities such as layout, testing, and legal review. Each of these is a potential scope boundary that, if left implicit, defaults to whichever interpretation causes the most friction at delivery. Boundary setting is the work of making the edges explicit and agreed before any segment is translated, so that scope becomes a contract rather than a hope.

Use this skill when starting a translation or localization engagement, when a requester's request is vague about what is included, when deciding whether an asset or task falls inside the job, or when defending against scope creep. The goal is to produce a scope boundary that is explicit, inventoried, and confirmed.

## Core Rules

### Inventory The Full Content Surface Before Quoting

Never estimate scope from a sample, a file name, or memory of similar projects. Walk the actual deliverable and record every surface that contains language.

For documents, inventory headings, body, tables, captions, footnotes, headers, footers, callouts, alt text, watermarks, and text baked into images. For software, inventory UI strings, tooltips, error messages, emails, help pages, in-app legal text, push notifications, and store metadata. For websites, inventory page text, navigation, form labels, buttons, SEO metadata, structured data, downloadable PDFs, and media captions. For multimedia, inventory spoken audio, on-screen text, subtitles, and dubbing scripts. The inventory is the foundation of the scope; without it, the boundary is a guess. Record the inventory so the requester can confirm or correct it, turning it into a shared scope contract.

### Separate Translatable From Non-Translatable Deliberately

Not everything containing words should be translated, and the decision must be made per category, not per occurrence.

Typically non-translatable content includes code, variables, placeholders, URLs, email addresses, file paths, API keys, trademarks kept in source form, legal entity names that must remain unchanged, citations to source-language law, and product names protected as brand assets. Typically translatable content includes user-facing prose, UI labels, help text, marketing copy, and legal text that must be understood by the target reader. When a category is ambiguous, such as a product name that is localized in some markets but not others, record the rule once and apply it consistently. Per-occurrence guessing produces inconsistency that no reviewer can untangle.

### Define What Adjacent Work Is Included And Excluded

Translation engagements live next to other work, and the boundary between them is where most disputes arise. Make the adjacency explicit.

State whether the scope includes source editing or copywriting, desktop publishing beyond text insertion, image recreation or screenshot re-shooting, voice recording, subtitling timing adjustment, legal review, clinical or subject-matter review, accessibility auditing, functional testing, or terminology base creation. Each of these is commonly assumed by requesters to be part of translation and commonly assumed by translators to be out of scope. Exclusions stated explicitly protect against the assumption that silence means inclusion. When in doubt, list the exclusion; an over-specified scope is far cheaper to renegotiate than an under-specified one.

### Bound The Deliverable Format And Reinsertion

Scope is not only the text; it is the form in which the text is returned. A correct translation delivered in the wrong format is unusable.

Define whether the deliverable is clean target text, a bilingual table, tracked changes, localized files reinserted into the original format, a change log, glossary extracts, translator notes, or a side-by-side comparison. Define whether layout adjustment, font embedding, right-to-left or vertical layout handling, and tag or markup preservation are included. If the source is a complex layout, a scanned PDF, or a proprietary format, verify that the tooling can produce the promised output before committing to it. Promising a format the tooling cannot deliver is a scope failure that surfaces at the worst moment.

### Define Version, Update, And Repetition Handling

Scope extends in time, not just in volume. A one-time translation and an ongoing localization stream are different engagements.

Clarify whether the deliverable is one-time or recurring, whether updated source must be re-translated and under what turnaround, how exact and fuzzy translation memory matches are counted and priced, whether internal repetitions are counted once or per occurrence, and whether locked or pre-translated segments are excluded. These rules change both effort and cost dramatically. If the requester has not stated them, propose defaults and confirm them in writing before work begins.

### Map Dependencies The Translator Does Not Control

Scope often depends on inputs the translator cannot produce. Identify these dependencies early, because a slipped dependency delays the deliverable even when the translator is ready.

Common dependencies include source finalization, glossary approval, style guide delivery, access to the content management or localization platform, context screenshots, design files, subject-matter expert availability for queries, and reviewer availability. List each dependency with an owner and a date. A dependency that is never named becomes a silent excuse when the project stalls.

### Establish A Change Path For Scope Discovered Mid-Project

Scope changes during a project, because the full source is often not visible at the start. Plan the change path rather than pretending the boundary is frozen.

Define how additions are requested, how they affect effort and deadline, who approves them, and how they are recorded. A lightweight change log prevents scope creep from becoming a silent overrun, and it gives the translator a defensible record when a requester asks why a discovered asset was not translated. The change path is not bureaucracy; it is the mechanism that keeps the boundary honest as reality intrudes.

## Common Traps

### Estimating From Word Count Or A Sample Alone

Word count is a starting point, not an effort or scope model. Dense, ambiguous, or heavily formatted content can take several times longer per word than plain prose, and a sample can hide entire asset categories. Scope built on raw volume underprices hard jobs and misses hidden surfaces.

### Forgetting Hidden And Embedded Text

Alt text, tooltips, metadata, captions, headers, footers, and text baked into images are easy to miss in a visual scan. A deliverable that omits them is incomplete even when the main body is fully translated.

### Assuming Adjacent Work Is Someone Else's Problem

DTP, screenshot recreation, legal review, and testing are frequently assumed by the requester to be included. Silence on these does not mean exclusion in the requester's mind; state the boundary explicitly.

### Promising A Format The Tooling Cannot Produce

Complex layouts, scanned PDFs, and proprietary formats often defeat clean reinsertion. Committing to a deliverable format without verifying tooling leads to broken output at delivery.

### Leaving Dependencies Implicit

If the translator needs screenshots, glossary access, or expert queries, and these are not listed as dependencies, the project stalls silently when they are missing, and the translator is blamed for the delay.

### Treating Silence As Agreement

A requester who does not push back on a proposed scope has not necessarily agreed to it. Confirm scope in writing, because an unconfirmed boundary will be reinterpreted to the requester's advantage at delivery.

## Self-Check

Before starting work under a scope agreement, verify:

- A full inventory of translatable surfaces exists and has been confirmed by the requester.
- Translatable and non-translatable categories are classified, with rules recorded for ambiguous cases such as product names.
- Adjacent work, including DTP, image recreation, review, and testing, is explicitly included or excluded.
- The deliverable format is specified, including whether layout, image, and format reinsertion are included, and the tooling can produce it.
- Version, update, repetition, and translation memory match handling is defined.
- Dependencies such as source finalization, glossary, tooling access, and expert availability are listed with owners and dates.
- A scope change path exists for additions discovered during the project.
- The scope distinguishes one-time work from ongoing maintenance.
- No translatable surface has been silently dropped because it was inconvenient to inventory.
- The scope has been confirmed in writing, not merely assumed from the requester's silence.
