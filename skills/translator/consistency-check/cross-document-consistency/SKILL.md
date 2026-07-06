---
name: cross_document_consistency.md
description: Use when the agent is checking terminology style and formatting consistency across multiple translated documents or files, auditing a multi-file localization deliverable for divergence between documents, reconciling interface documentation and marketing translations, or coordinating consistency across separately translated components of one product or release.
---

# Cross-Document Consistency

Most consistency failures are invisible inside any single document and only become visible when a reader moves between documents. Each file, translated in isolation, can be internally correct and still contradict its neighbors: the interface calls a feature one name, the documentation calls it another, the marketing uses a third, and the legal notice uses a fourth. Cross-document consistency is the work of auditing a multi-file deliverable as one body of text, detecting divergences that span document boundaries, and reconciling them so the product speaks with one voice. It is harder than single-document consistency because the contradictions are distributed, because different files are often owned by different translators or vendors, and because the review tools that catch intra-document drift frequently miss inter-document drift. The harm this skill prevents is the user-facing experience of a product that feels stitched together from incompatible parts, which erodes trust even when every individual sentence is correct.

Agents miss this work because consistency checking is usually scoped to one file at a time, and a per-file review passes even when the files disagree. Treat the deliverable, not the file, as the unit of consistency.

## Core Rules

### Treat The Deliverable As One Corpus

Define the consistency scope as the full set of files the user will encounter together, not each file separately. A release bundle, a product's UI plus its documentation plus its marketing, a contract plus its annexes, or a help center plus its in-product messages: these are the units users experience, and they are the units you must audit. List every file in scope before checking, including files that feel peripheral, because peripheral files, legal notices, footers, error messages, are common sources of drift.

If files are translated by different vendors or at different times, the cross-document risk is higher, not lower, and the audit must explicitly bridge them.

### Establish Shared Authorities Across All Files

Cross-document consistency is impossible if files follow different authorities. Before auditing, confirm that every file in scope references the same termbase, the same style guide, the same translation memory, and the same reference content. Confirm the authorities are the same version: a termbase updated mid-project produces drift between files translated before and after the update.

Where authorities conflict or are absent, resolve that first. Auditing consistency against inconsistent authorities produces findings that cancel each other out.

### Map The Cross-Document Surfaces That Drift

Certain surfaces drift between documents more than others, and the audit should target them deliberately. These include product and feature names, UI labels referenced in prose, defined terms, forms of address and pronouns, date number and unit formats, capitalization and punctuation conventions, treatment of abbreviations and acronyms, and boilerplate such as copyright lines, disclaimers, and sign-offs.

Build a cross-document checklist of these surfaces and check each one across all files, rather than reading each file linearly. Linear reading finds intra-file issues; surface-based checking across files finds inter-file issues.

### Run Cross-File Concordance On Key Terms

For every key term, run a concordance search across the entire deliverable, not within one file, and inspect every rendering. The goal is to find cases where the same source term is rendered differently in different files. Group the renderings and classify each as matching the termbase, an admitted alternative, an undocumented variant, or a forbidden form.

Concordance across files is the single most effective cross-document technique, because it surfaces drift that no per-file review would catch. Prioritize terms that appear in multiple files and terms whose inconsistency carries risk, such as safety, legal, or defined terms.

### Reconcile Interface Documentation And Marketing

A frequent and high-impact drift is between the translated interface, the documentation that describes it, and the marketing that promotes it. The UI label a user clicks must match the name the documentation uses to refer to it, which must match the name in marketing. Audit these three surfaces against each other, because they are usually translated by different teams on different timelines.

Where they diverge, the interface is usually the anchor, because users encounter it directly. Align documentation and marketing to the interface unless a governance decision says otherwise, and record the decision.

### Detect Drift Introduced By Updates

Cross-document consistency is dynamic. When one file is updated, it can drift from files that were not updated. When a term is deprecated and replaced, every file must be updated, or the deliverable splits between old and new usage. Track which files have been aligned to the current termbase and style guide version, and treat unaligned files as drift risks.

After any update, re-run the cross-document checks on the affected surfaces, not only on the changed file, because the change may have introduced divergence from its neighbors.

### Coordinate Across Translators And Vendors

When multiple translators or vendors produce files in the same deliverable, cross-document consistency depends on coordination, not individual diligence. Share the authorities before work begins, hold a kickoff to align on key terms and decisions, maintain a shared decisions log visible to all contributors, and assign a lead reviewer who owns cross-document consistency for the whole deliverable.

Without coordination, each contributor's correct individual choices combine into an inconsistent deliverable, and the inconsistency is structural rather than accidental.

### Document Legitimate Cross-Document Variation

Not every cross-document difference is an error. Marketing may legitimately use a punchier rendering than documentation; legal notices may legitimately use more formal terminology than help content. Distinguish legitimate register variation from inconsistency, and document the legitimate cases so reviewers do not re-flag them and so the boundary is enforced consistently.

The test is whether the variation is intentional, governed, and bounded. Variation that is accidental, ungoverned, or unbounded is an error to fix.

## Common Traps

### Auditing File By File Instead Of As A Corpus

Per-file review passes even when files contradict each other. Treat the deliverable as one corpus and check surfaces across all files.

### Different Authorities Or Versions Across Files

Files following different termbases, style guides, or memory versions cannot be consistent. Align authorities and versions before auditing.

### Missing Peripheral Files

Footers, legal notices, error messages, and boilerplate are common drift sources and are often skipped. Include them in scope.

### Linear Reading Instead Of Surface-Based Checking

Reading each file linearly finds intra-file issues but misses inter-file drift. Check each consistency surface across all files.

### Concordance Run Within Single Files Only

Concordance scoped to one file cannot find inter-file renderings of the same term. Run it across the whole deliverable.

### Interface Documentation And Marketing Translated In Silos

These surfaces drift because different teams own them. Reconcile them against each other, usually anchoring to the interface.

### Updating One File Without Re-Checking Neighbors

A change can introduce divergence from unchanged files. Re-run cross-document checks on affected surfaces after any update.

### Flagging Legitimate Register Variation As Errors

Marketing and documentation may legitimately differ in register. Distinguish governed, bounded variation from accidental drift.

## Self-Check

Before approving a multi-file deliverable for cross-document consistency, verify:

- The consistency scope was defined as the full set of files the user encounters together, including peripheral files such as legal notices, footers, and error messages.
- All files in scope reference the same authorities, termbase, style guide, translation memory, and reference content, at the same version.
- A cross-document checklist of drift-prone surfaces, names, UI labels, defined terms, address forms, formats, capitalization, abbreviations, and boilerplate, was checked across all files.
- Concordance searches were run across the entire deliverable for key terms, and renderings were classified against the termbase.
- The interface, documentation, and marketing surfaces were reconciled against each other, with the interface as anchor unless governed otherwise.
- Files translated by different contributors were coordinated through shared authorities, a kickoff, a decisions log, and a lead reviewer owning cross-document consistency.
- After any update, cross-document checks were re-run on affected surfaces, not only on the changed file.
- Legitimate register variation was distinguished from inconsistency and documented so it is not re-flagged.
- An alignment record shows which files match the current termbase and style guide version.
- No key term is rendered differently across files except where the variation is intentional, governed, and documented.
