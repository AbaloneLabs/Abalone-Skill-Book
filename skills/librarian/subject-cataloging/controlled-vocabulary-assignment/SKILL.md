---
name: controlled_vocabulary_assignment.md
description: Use when the agent is assigning subject headings from a controlled vocabulary such as LCSH, MeSH, or FAST, choosing between candidate headings, applying subdivision rules, or deciding whether to use a pre-coordinated heading versus free-floating terms.
---

# Controlled Vocabulary Assignment

Assigning subject headings from a controlled vocabulary is where intellectual analysis meets a rule-bound system. The vocabulary is not a list of keywords; it is a structured thesaurus with hierarchical relationships, scope notes, subdivision rules, and conventions for pre-coordination. Two catalogers analyzing the same work can produce wildly different records if one treats the vocabulary as a tag set and the other treats it as a system. The cataloger must choose the most specific heading that truly represents the work, apply subdivisions correctly rather than inventing phrases, respect the vocabulary's structure and scope notes, and know when a heading does not exist and a different approach is required. Poor assignment scatters related works, sends users to the wrong place, and undermines the collocation that controlled vocabulary exists to provide.

Use this skill when assigning or reviewing subject headings, choosing among candidate terms, applying subdivisions, handling headings that do not quite fit, or deciding between pre-coordinated and post-coordinated approaches. The goal is to prevent the agent from keyword-matching into the vocabulary, inventing subdivisions, ignoring scope notes, or assigning the first plausible heading without checking for a more precise one.

## Core Rules

### Choose The Most Specific Heading That Fits

Controlled vocabularies are hierarchical. A work about a specific topic should receive the specific heading, not a broader one, unless the specific heading does not exist.

Specificity practice:

- descend to the most specific term that represents the work's primary topic;
- do not assign a broader heading when a narrower one exists and applies;
- check the hierarchy, broader, narrower, and related terms, before settling;
- use a broader heading only when no specific heading exists and the work is genuinely about the broader area;
- avoid assigning both a heading and its broader or narrower term for the same aspect, which is redundant.

A work about beekeeping gets "Bee culture," not "Agriculture." A work about a specific battle gets the battle heading, not just "World War, 1939-1945."

### Read And Respect Scope Notes

Scope notes define what a heading means and how it should be used. Ignoring them is the most common cause of mis-assignment.

Scope notes tell you:

- the precise meaning and boundaries of the heading;
- what the heading excludes, often pointing to a related term;
- whether the heading is for the topic, the place, the period, or the form;
- when a heading was established or revised, affecting legacy records;
- cross-references to preferred or related headings.

Before assigning any heading, read its scope note. A heading that looks right by its label may mean something narrower or different than expected.

### Apply Subdivisions According To The Rules

Subdivisions build complex headings by combining a topic with a place, period, form, or subtopic. They are powerful but rule-bound, and misapplied subdivisions create invalid or meaningless headings.

Subdivision rules:

- use only subdivisions authorized for the heading type, topical, geographic, chronological, form;
- follow the prescribed order, topic, place, period, form;
- check whether a subdivision is free-floating or must be established;
- use geographic subdivisions only under headings that allow them;
- do not invent subdivision combinations that the vocabulary does not permit;
- prefer an established complex heading over a newly built one when one exists.

"Art, Italian, 20th century" follows topic, place, period order. "Art, 20th century, Italy" does not. Building headings outside the rules produces records that do not collocate and may not even be searchable as intended.

### Distinguish Pre-Coordination From Post-Coordination

Controlled vocabularies differ in how they combine concepts. Understanding the difference prevents misusing the vocabulary.

Approaches:

- pre-coordinated, the vocabulary establishes complex headings in advance, as in LCSH;
- post-coordinated, the system combines simple terms at search time, as in some faceted systems;
- FAST, a simplified, faceted adaptation of LCSH designed for post-coordination;
- faceted vocabularies assign each concept separately and let the interface combine them.

Do not build elaborate pre-coordinated headings in a system designed for facets, and do not assume facets will combine correctly in a system expecting pre-coordination. Match the assignment style to the vocabulary and the discovery system.

### Check For An Existing Heading Before Building

Many apparent gaps are already covered by an established heading the cataloger did not find. Building a new combination when an established heading exists creates duplication.

Before building:

- search the vocabulary by keyword, stem, and variant;
- check related and broader terms for a pointer to the specific heading;
- verify the candidate is not a reference to a preferred term;
- confirm no established complex heading covers the combination;
- consult cataloging documentation for established patterns.

If an established heading exists, use it. If genuinely none exists, follow the proposal process rather than creating a local heading silently.

### Handle Missing Or Awkward Headings Deliberately

The vocabulary will not have a heading for every topic, especially emerging fields, local subjects, or underrepresented perspectives. Have a deliberate approach rather than improvising.

Options:

- assign the closest valid broader heading and note the specificity gap;
- use a valid heading with a topical subdivision if permitted;
- propose a new heading through the vocabulary's contribution process;
- use a local subject heading with documentation, clearly marked as local;
- combine a controlled heading with a keyword or genre term for access.

Do not stretch a heading beyond its scope note to force a fit. A misused heading is worse than a broader correct one.

### Assign Headings Consistently Across A Collection

Collocation depends on consistent assignment. If similar works receive different headings, they will not shelve or browse together.

Consistency requires:

- a local subject cataloging guide documenting recurring decisions;
- examples of correctly assigned headings for common topics;
- review of batch or vendor-supplied headings against local practice;
- coordination among catalogers on ambiguous cases;
- periodic cleanup of inconsistent legacy headings.

One cataloger's "Indians of North America" and another's "Native Americans" scatter the same body of work. Consistency is a collection-level property, not a per-record one.

### Record The Vocabulary Source

When using multiple vocabularies, record which vocabulary each heading comes from so the terms are interpretable and harvestable.

Practice:

- identify the vocabulary in the metadata, LCSH, MeSH, FAST, AAT;
- do not mix vocabularies in a single element without marking the source;
- keep local terms separate from standard vocabulary terms;
- ensure crosswalks map terms to the correct vocabulary on harvest.

Unmarked mixed vocabularies break aggregation and confuse users who assume all terms come from one system.

## Common Traps

### Keyword-Matching Into The Vocabulary

Searching by keyword and taking the first hit ignores hierarchy and scope. Descend to the specific term and read its scope note.

### Assigning Broader Headings When Narrower Ones Exist

Defaulting to a familiar broad heading hides specific works and overloads the broad term. Check for a narrower term.

### Inventing Subdivision Combinations

Building a phrase the vocabulary does not authorize creates an invalid heading. Use only permitted subdivisions in the correct order.

### Ignoring Scope Notes

A heading's label can mislead. The scope note defines its real meaning and boundaries.

### Redundant Heading Assignment

Assigning a heading and its broader or narrower term for the same aspect wastes space and confuses ranking. Choose the most specific applicable.

### Stretching A Heading Beyond Its Scope

Forcing an ill-fitting heading because none exists precisely misleads users. Use the closest valid broader heading or propose a new one.

### Inconsistent Assignment Across Catalogers

Different headings for the same topic scatter works. Maintain a local guide and coordinate decisions.

### Mixing Vocabularies Without Marking The Source

Unmarked mixed terms break harvesting and interpretation. Record the vocabulary source for each heading.

## Self-Check

- Is the most specific valid heading that represents the work's primary topic assigned?
- Has the scope note of each candidate heading been read and respected?
- Are subdivisions applied only as authorized, in the correct order, and not invented?
- Is the assignment style, pre-coordinated or faceted, matched to the vocabulary and discovery system?
- Was an existing established heading checked for before building a new combination?
- Are missing or awkward headings handled with a deliberate, documented approach rather than improvisation?
- Is assignment consistent across the collection, supported by a local guide and coordination?
- Is the vocabulary source recorded for each heading so terms are interpretable and harvestable?
