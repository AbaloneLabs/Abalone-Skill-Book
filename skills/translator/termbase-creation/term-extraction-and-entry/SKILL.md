---
name: term_extraction_and_entry.md
description: Use when the agent is mining terminology from source corpora and parallel texts, extracting candidate terms for a termbase, authoring well-formed term entries with definitions and context, deduplicating and disambiguating candidates, or deciding which terms deserve full termbase entries versus a lightweight working glossary.
---

# Term Extraction And Entry

A termbase is populated one entry at a time, and the quality of that population determines whether the resource is useful. Extraction is the work of deciding which terms in a body of source text deserve to become governed entries, and entry authoring is the work of writing each entry so that a translator can use it without guessing. Both are judgment-heavy. Extract too broadly and the termbase drowns in general-language words that waste reviewers' time and bury the terms that matter. Extract too narrowly and the high-risk domain terms that cause real inconsistency never enter the resource at all. Author entries thinly, with a term and a translation but no definition or context, and translators apply the wrong equivalent to the wrong sense. The harm this skill prevents is a termbase that is either bloated with noise or hollow where it counts, and entries that look complete but cannot be applied correctly.

Agents miss this work because extracting a list of frequent capitalized words feels like sufficient terminology work. It is not. Frequency and capitalization are weak proxies for terminological status. Real term extraction weighs concept status, domain relevance, risk, and polysemy, and real entry authoring treats the definition, context, and scope notes as the load-bearing parts of the entry, not the term pair itself.

## Core Rules

### Extract Against Criteria, Not Against Frequency

Decide what qualifies as a term before mining, and apply explicit criteria. A strong candidate is a single-word or multiword unit that denotes a domain-specific concept, that the reader must recognize consistently across the content, and where inconsistent rendering would cause confusion, safety risk, compliance risk, or brand erosion. This includes defined terms, product and feature names, UI labels, process and role names, units and classifications, and any term the source itself marks as defined.

Frequency alone produces false positives. A word can be frequent because it is general language, not because it is terminology. A word can be rare and still terminologically critical, such as a once-occurring safety term. Rank candidates by a combination of domain specificity, concept status, and risk, not by count alone.

### Use Multiple Extraction Methods And Reconcile

No single method finds all terms. Combine statistical extraction, which surfaces frequent multiword units and capitalized strings; linguistic extraction, which targets noun phrases and defined-term patterns; and manual mining by a domain-aware reader who flags terms by judgment. Reconcile the three lists, because each catches what the others miss. Statistical methods over-extract general collocations; manual mining misses low-frequency terms; linguistic patterns miss irregular term forms.

Run extraction on representative corpora, not on a single file. A corpus skewed to one document type biases the term list toward that type's vocabulary and misses terms that dominate elsewhere. Balance the corpus across content types before extracting.

### Deduplicate By Concept, Not By Surface Form

After extraction, candidates collide. The same concept appears as an acronym, its expansion, a plural, and a variant spelling. Deduplicate at the concept level: group surface variants that label the same concept into one candidate entry, and record the variants as term forms inside it. If you deduplicate by surface string, you create one entry per spelling and fragment the concept across records.

Equally, do not merge candidates that share a surface form but denote different concepts. A polysemous term must become separate candidate entries, one per concept, each with its own definition and context. Merging senses into one entry is the fastest way to make a termbase entry unusable, because the translator cannot tell which equivalent applies to their case.

### Triage Candidates Before Full Authoring

Not every candidate deserves a full governed entry. Triage to allocate authoring effort where it pays off. Route high-risk, high-frequency, defined, and domain-specific terms to full entries with definition, context, status, and provenance. Route lower-stakes recurring terms to a lightweight working glossary that still enforces consistency but carries less metadata. Drop general-language candidates that do not denote domain concepts.

Triage prevents the common failure mode of spending equal effort on every candidate, which either exhausts the budget before the important terms are done or produces a termbase padded with low-value entries. Prioritize by the cost of inconsistency: terms where variation causes harm come first.

### Write Definitions That Distinguish Concepts

The definition is what makes an entry trustworthy, because it states the concept the term labels. Write a definition that distinguishes the concept from neighboring ones, not a circular restatement of the term. State the concept's class and its differentiating characteristics. A definition that merely reuses the term or a synonym tells the translator nothing they did not already know and fails to disambiguate polysemous cases.

Where a source term is ambiguous, the definition must pin down the sense this entry covers, so a translator encountering the term in another sense knows to look for a different entry. Without distinguishing definitions, polysemy silently corrupts the termbase.

### Provide Context That Shows Real Usage

Pair every definition with a context sentence drawn from actual source text, showing the term in use in the relevant sense. Where possible, add a target context sentence showing the approved equivalent inside a real translation. Context does what definition cannot: it shows the term's syntactic behavior, register, and surrounding collocations, which is what a translator actually needs to apply the equivalent correctly.

Avoid invented example sentences that sound plausible but misrepresent usage. A fabricated context can mislead as much as no context. Prefer authentic sentences from the corpus, attributed to their source.

### Record Scope And Restrictions Explicitly

An entry that says use this equivalent everywhere is often wrong. Record the scope: which domain, product, audience, or register the equivalent applies to, and where it does not apply. Note restrictions such as do not use in patient-facing content, formal register only, or deprecated in the EU market. Scope notes turn a blanket recommendation into usable guidance.

Without scope, translators apply an equivalent beyond its valid range, and the termbase gets blamed for inconsistency that its structure should have prevented.

### Capture Forbidden And Deprecated Forms

Entry authoring is not only about the preferred equivalent. Capture forbidden and deprecated forms explicitly, because telling translators what not to use is as valuable as telling them what to use. A forbidden form is often a plausible-looking wrong translation that recurs in legacy content or machine output. Recording it lets reviewers flag it and lets automated checks catch it.

For each forbidden or deprecated form, record why it is disallowed, so the decision is auditable and not merely a preference.

### Attribute Every Entry To A Source And Decision

No entry should enter the termbase without provenance. Record who extracted it, who authored the definition, what source authority supports the equivalent, and when the decision was made. Provenance lets later reviewers challenge or update entries against their basis and prevents the termbase from accumulating unattributed guesses that nobody can verify.

## Common Traps

### Equating Frequency With Terminological Status

Mining by frequency floods the termbase with general-language words and buries the rare but high-risk terms. Extract against domain-specificity and concept status criteria.

### Extracting From A Skewed Corpus

A corpus drawn from one document type biases the term list. Balance the corpus across content types before extraction.

### Deduplicating By Surface String

Grouping by spelling fragments one concept across variant forms; merging polysemous senses into one entry makes it unusable. Deduplicate by concept and split by sense.

### Equal Effort On Every Candidate

Treating every candidate as a full entry exhausts the budget on low-value terms. Triage by the cost of inconsistency.

### Circular Or Empty Definitions

A definition that restates the term fails to disambiguate and gives translators nothing to trust. Define the concept's class and differentiating features.

### Invented Context Sentences

Fabricated examples can misrepresent usage. Use authentic corpus sentences attributed to their source.

### Omitting Scope And Restrictions

A blanket equivalent applied beyond its valid range causes the very inconsistency the termbase exists to prevent. Record domain, register, and market scope.

### Forgetting Forbidden And Deprecated Forms

Recording only the preferred term leaves plausible wrong translations unflagged, so they recur in legacy and machine output.

### Entering Entries Without Provenance

Unattributed entries cannot be challenged or verified and accumulate as guesses. Record author, authority, and date for every entry.

## Self-Check

Before accepting extracted and authored entries into a termbase, verify:

- Candidates were extracted against explicit criteria of domain specificity, concept status, and risk, not by frequency or capitalization alone.
- Multiple extraction methods were combined and reconciled, and the source corpus was balanced across content types.
- Surface variants labeling one concept were grouped into a single entry, while polysemous senses were split into separate entries.
- Candidates were triaged so high-risk, defined, and domain-specific terms received full entries and low-stakes terms a lightweight glossary.
- Each definition distinguishes the concept from neighbors rather than restating the term, and pins down the sense for polysemous entries.
- Authentic source and target context sentences accompany entries and show real usage, not invented examples.
- Scope and restrictions, including domain, product, audience, register, and market, are recorded so equivalents are not applied beyond their valid range.
- Forbidden and deprecated forms are captured with reasons, not just the preferred equivalent.
- Every entry carries provenance: extractor, author, source authority, and date.
- No entry is a bare term pair without definition, context, or scope, and no candidate entered the termbase purely because it was frequent.
