---
name: termbase_creation_and_maintenance.md
description: Use when the agent is creating or maintaining a terminology database or termbase, defining term entries with source and target equivalents part of speech context and status, structuring termbase fields, or governing the lifecycle of approved deprecated and candidate terms across a translation program.
---

# Termbase Creation And Maintenance

A termbase is the single source of truth for how an organization's terms are translated. Without one, every translator and every project reinvents terminology, and the result is inconsistency that confuses users, erodes brand trust, and in regulated domains creates safety and compliance risk. With one that is poorly structured or poorly maintained, the same problems persist, because translators cannot find what they need, cannot trust what they find, or apply outdated entries. Building and maintaining a termbase is not a one-time data-entry task. It is an ongoing governance activity that requires clear entry structure, defined workflows for proposing approving and deprecating terms, context that makes entries usable, and discipline to keep the termbase aligned with how the organization actually uses language. A termbase that is not maintained becomes worse than no termbase, because it looks authoritative while being wrong.

Use this skill when creating a termbase, designing its structure, adding or revising entries, governing term status, or maintaining terminology quality over time. The goal is to build a termbase that translators trust and use, and to keep it accurate as language, products, and domains evolve.

## Core Rules

### Design A Clear Entry Structure

A termbase entry must contain enough information for a translator to use the term confidently and consistently. Design the structure before populating, because restructuring later is costly.

A strong entry includes the source term, the approved target equivalent or equivalents for each target language, part of speech, subject field or domain, a definition of the concept, context in the form of a source sentence showing usage, the target context showing the approved equivalent in use, usage notes such as register or restrictions, the term status, the source of the entry, and a modification date. Not every field is needed for every entry, but the structure should accommodate them.

Avoid over-engineering the structure with dozens of fields no one fills in. A lean structure that is consistently populated beats a rich structure that is half empty.

### Capture Concept, Not Just Word

Terminology management is concept-based. An entry represents a concept, and the source and target terms are labels for that concept in different languages.

Define the concept clearly in the definition field. Two terms that look similar may label different concepts, and two different source terms may label the same concept. Without a concept definition, translators cannot tell whether a target equivalent applies to their case. A definition also prevents duplicate entries for the same concept under different surface forms.

When a source term is ambiguous and labels multiple concepts, create separate entries per concept, each with its own target equivalent and context.

### Provide Context For Every Entry

A term with a target equivalent but no context is hard to apply correctly. Context shows the term in use and disambiguates cases where the equivalent depends on usage.

Include a source context sentence and, where possible, a target context sentence showing the approved equivalent in a real translation. Context helps translators confirm they have the right entry and helps reviewers verify consistent application. It is especially important for terms whose translation depends on syntax, register, or surrounding text.

Entries without context get applied inconsistently, because translators guess at usage.

### Govern Term Status Explicitly

Terms have a lifecycle. Govern it with explicit status values so translators know what they can rely on.

Common statuses include candidate, proposed terms not yet approved; preferred, the approved primary term; admitted, an accepted alternative when the preferred does not fit; deprecated, terms no longer approved; and forbidden, terms that must not be used, often because of legal, brand, or sensitivity reasons. Each status carries a rule for translators. Preferred terms should be used; admitted terms may be used; deprecated and forbidden terms should be flagged if found in source or legacy content.

Status must be visible in the entry and enforced in workflows. A termbase where every entry is implicitly approved invites inconsistency.

### Define Approval Workflow And Authority

Terminology decisions need an owner and a process. Define who proposes terms, who approves them, how disputes are resolved, and how quickly proposals are turned around.

Typical workflow has translators or terminologists proposing candidate terms during work, a terminologist or subject-matter expert reviewing proposals, an authority such as the terminologist, domain expert, or brand owner approving, and the termbase being updated with status, definition, context, and date. Define target turnaround for proposals, because slow approval leaves translators working without guidance.

Record who approved each entry and when, so later challenges can be resolved against the authority structure.

### Maintain The Termbase Continuously

A termbase is not built once. It decays if not maintained, as products, domains, and language evolve.

Schedule regular review cycles to verify entries are still accurate, retire deprecated terms, add new terms from recent projects, and reconcile conflicts where usage has drifted from the termbase. Feed queries and decisions from active projects back into the termbase so it captures organizational learning. Assign maintenance ownership so the task has an owner rather than being everyone's second priority.

A termbase reviewed annually is more valuable than one built comprehensively and then abandoned.

### Handle Multiterm And Multi-Domain Entries

Organizations often need different equivalents for different domains, products, or audiences within the same language. Structure the termbase to handle this.

A single source term may have different preferred equivalents in different product lines, or different equivalents for expert versus lay audiences. Use domain, product, or audience fields to distinguish these, so translators select the entry that matches their context. Without this structure, a single approved equivalent gets applied where a different one was needed.

### Integrate The Termbase With Translation Tools

A termbase delivers value only when translators use it. Integrate it with the translation environment so terms surface automatically.

Configure the translation tool to look up terms and present them during work. Train translators to consult the termbase and to propose additions when they encounter gaps. Track termbase usage and gap reports to identify where the termbase needs expansion. A termbase that sits in a spreadsheet and is never consulted provides no consistency benefit.

## Common Traps

### Building A Termbase And Not Maintaining It

An unmaintained termbase decays into an authoritative-looking source of wrong terms.

### Entries Without Definitions Or Context

Terms without concept definitions and context get applied inconsistently, because translators cannot confirm they have the right entry.

### Treating All Entries As Approved

Without status governance, translators cannot distinguish reliable terms from proposals, and inconsistency results.

### Slow Or Absent Approval Workflow

When proposals sit unanswered, translators work without guidance or invent their own terms, defeating the termbase's purpose.

### Ignoring Domain And Audience Variation

A single equivalent per term ignores cases where different contexts need different equivalents.

### Over-Engineering The Structure

Dozens of fields no one fills in produces a termbase that looks comprehensive but is unusable in practice.

### Failing To Integrate With Tools

A termbase not connected to the translation environment is not consulted, so consistency benefits never materialize.

## Self-Check

Before approving a termbase or its entries, verify:

- The entry structure includes source term, target equivalents, part of speech, domain, definition, context, usage notes, status, source, and date.
- Entries are concept-based, with definitions that distinguish concepts and prevent duplicates.
- Source and target context sentences accompany entries to guide correct application.
- Term status, such as candidate, preferred, admitted, deprecated, and forbidden, is explicit and governed.
- An approval workflow with named authority and target turnaround exists for proposals.
- Regular maintenance cycles review, retire, add, and reconcile entries, with assigned ownership.
- Domain, product, and audience variation is structured so the right equivalent is selected per context.
- The termbase is integrated with translation tools so terms surface during work.
- Usage and gap reports inform where the termbase needs expansion.
- No entry looks authoritative while being outdated, unapproved, or lacking the context needed to apply it correctly.
