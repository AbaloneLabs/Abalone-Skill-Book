---
name: taxonomy_and_labeling.md
description: Use when the agent is designing a taxonomy, creating categories and tags, choosing labels for navigation and content, deciding between hierarchical and faceted classification, resolving naming conflicts, or establishing a controlled vocabulary that users, authors, and search can rely on.
---

# Taxonomy And Labeling

A taxonomy is the system of categories and labels that makes content findable, sortable, and consistent. It looks like a list of tags, but it is really a set of decisions about how things are grouped, what each group is called, and how those names hold up across users, authors, and time. Agents tend to treat taxonomy as a quick labeling exercise, invent categories that feel right to them, or let every author tag freely until the vocabulary fragments. The harm is invisible at first and systemic later: users cannot find content because it is categorized inconsistently, search returns noise because tags mean different things, and the structure becomes impossible to maintain.

Use this skill before finalizing categories, tags, labels, or a controlled vocabulary. The goal is to prevent the agent from choosing labels based on internal preference, from building a taxonomy that fragments under use, or from creating categories so ambiguous that no two authors apply them the same way.

## Core Rules

### Build The Taxonomy From User Language, Not Internal Jargon

The words users use to search, describe, and look for content are the raw material of a usable taxonomy. Labels chosen from internal vocabulary, product team shorthand, or marketing language fail because they do not match how people actually think and seek. A taxonomy that uses terms no user would utter forces users to learn a new vocabulary before they can find anything.

Ground labels in evidence:

- mine search logs for the terms users actually type;
- review support tickets and customer calls for natural language;
- run card sorts or tree tests to see how users group and name things;
- prefer the user's term even when it is less precise than the internal one, then add precision through structure rather than renaming.

When internal and user language conflict, the user's language wins for the label; the internal precision can live in metadata users never see.

### Choose The Right Classification Structure For The Content

Not all content organizes the same way. Some content is naturally hierarchical (a category tree), some is multifaceted (the same item belongs to many attributes), and some is networked (related by association). Forcing one structure onto content that needs another produces a taxonomy that fights the material.

Match structure to content:

- hierarchical taxonomies suit content with clear parent-child relationships (categories, departments, topics);
- faceted classification suits content users approach from multiple independent attributes (products by color, size, price, brand);
- folksonomies (user-generated tags) suit broad, evolving content where top-down control is impractical, but require moderation to avoid fragmentation;
- hybrid systems combine a primary hierarchy with facets for filtering.

A product catalog forced into a pure hierarchy loses the filtering power users expect. A simple blog saddled with facets adds complexity it does not need.

### Define Each Category With Clear Scope And Boundaries

A category is only as good as its definition. Vague categories ("General," "Miscellaneous," "Other") become dumping grounds, and categories with overlapping scope cause authors to guess where content belongs, producing inconsistent tagging.

For each category, define:

- what belongs in this category, with examples;
- what does not belong, with counter-examples;
- the distinguishing rule that separates it from sibling categories;
- when content fits multiple categories and how to decide.

If two authors cannot look at the same content and agree on its category, the taxonomy definition is too vague. Test definitions by having multiple people categorize the same sample and comparing results.

### Establish A Controlled Vocabulary To Prevent Fragmentation

Free tagging, where every author invents tags, inevitably fragments. "Mobile," "phone," "smartphone," and "cell phone" proliferate for the same concept, and content scatters across variants that users and search cannot reconcile. A controlled vocabulary limits terms to an approved set so the same concept always uses the same label.

Govern the vocabulary:

- maintain an authoritative list of allowed terms, each with a definition;
- include synonyms that map to the preferred term (so "cell phone" redirects to "mobile");
- define a process for proposing and approving new terms as content evolves;
- assign ownership so the vocabulary is maintained, not abandoned.

A controlled vocabulary is living, not frozen. It must absorb new concepts without fragmenting, which requires ongoing governance, not a one-time definition.

### Resolve Label Ambiguity Deliberately

Many natural-language labels are ambiguous. "Apple" could be a fruit or a company. "Bank" could be a river edge or a financial institution. A taxonomy that ignores ambiguity produces categories and search results that mix unrelated content.

Handle ambiguity:

- disambiguate with qualifiers when a term has multiple meanings ("Apple (fruit)" versus "Apple (company)");
- use context and parent categories to clarify meaning where qualifiers are awkward;
- document the intended meaning in the category definition so authors apply it consistently;
- consider whether an ambiguous term should be replaced with a clearer alternative.

Ambiguity is not always obvious to the person who created the label. Review terms for alternate meanings, especially when the product spans domains.

### Design Labels For Scannability And Consistency

Labels are read, not studied. A user scanning a menu or a list of tags has seconds to decide where to go. Labels that are too long, inconsistent in form, or grammatically mixed slow comprehension and increase errors.

Keep labels usable:

- use parallel structure across sibling labels (all nouns, all verbs, all gerunds) so the set reads consistently;
- keep labels short enough to scan, using two to four words where possible;
- avoid abbreviations and acronyms unless they are universally known to the audience;
- apply capitalization and punctuation consistently across the entire taxonomy.

A menu that mixes "Manage Account," "Billing info," and "User Preferences" feels inconsistent even if each label is fine alone. Consistency is a system property, not a per-label property.

### Plan For Taxonomy Evolution

Content and user language change. New products, new concepts, and shifting terminology mean a taxonomy that is correct today will drift. A taxonomy with no evolution plan either fossilizes or fragments under ad-hoc additions.

Plan for change:

- define how new categories are proposed, reviewed, and approved;
- establish a review cadence to retire unused categories and merge redundant ones;
- handle renaming carefully: when a label changes, existing tagged content must migrate, not orphan;
- version the taxonomy so changes are deliberate and reversible.

A taxonomy that cannot evolve becomes a liability. A taxonomy that evolves without governance becomes noise.

## Common Traps

### Labeling From Internal Vocabulary

Choosing labels from product team or marketing language instead of user language forces users to learn new terms before they can find content.

### Free Tagging Without Control

Letting every author invent tags fragments the vocabulary so the same concept scatters across many variant terms.

### Vague Or Overlapping Categories

Categories without clear scope and boundaries become dumping grounds and cause inconsistent tagging across authors.

### Forcing One Structure Onto All Content

Applying a pure hierarchy to multifaceted content, or facets to simple content, produces a taxonomy that fights the material.

### Ignoring Label Ambiguity

Terms with multiple meanings mix unrelated content in categories and search results; disambiguate or replace ambiguous labels.

### Inconsistent Label Grammar

Mixing nouns, verbs, and abbreviations across sibling labels slows scanning and makes the taxonomy feel unprofessional.

### Treating The Taxonomy As One-Time Work

A taxonomy that is never governed or evolved either fossilizes or fragments; plan for ongoing maintenance and versioning.

## Self-Check

- [ ] Labels are grounded in user language from search logs, support data, or card sorts, not internal jargon.
- [ ] The classification structure (hierarchical, faceted, folksonomy, hybrid) matches how the content is naturally organized and sought.
- [ ] Each category has a documented definition with examples, counter-examples, and a distinguishing rule from siblings.
- [ ] A controlled vocabulary governs allowed terms, maps synonyms to preferred terms, and has a defined maintenance process.
- [ ] Ambiguous labels are disambiguated with qualifiers, context, or clearer alternatives, and their intended meaning is documented.
- [ ] Labels use parallel grammar, consistent capitalization, and scannable length across the entire taxonomy.
- [ ] The taxonomy has an evolution plan: how new categories are proposed, how renames migrate existing content, and a review cadence.
- [ ] The taxonomy was tested by having multiple people categorize the same content to check for consistency.
