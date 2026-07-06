---
name: metadata_profiles_and_application_profiles.md
description: Use when the agent is creating or maintaining a metadata application profile, defining which elements to use from a schema, setting local rules and obligations for elements, establishing value vocabularies and input rules, or documenting a project's metadata practice for consistency.
---

# Metadata Profiles And Application Profiles

A metadata schema defines what elements exist. An application profile defines which of those elements a project actually uses, how, and under what rules. Without a profile, every metadata creator interprets the schema differently: one populates every field, another skips optional ones, a third uses free-text where another uses controlled vocabulary, and the collection's metadata becomes inconsistent in ways that break faceting, browsing, and harvesting. The application profile is the document that prevents this drift. It declares the project's element set, the obligation level of each element, the value vocabularies and input formats, the repeatable and conditional rules, and the examples that show what good practice looks like. Designing and maintaining a profile is a governance task: it must be specific enough to enforce consistency, flexible enough to handle the collection's variety, documented clearly enough for all creators to follow, and revisable as the collection and standards evolve.

Use this skill when creating or revising a metadata application profile, defining element obligations and rules, establishing value vocabularies, or documenting project metadata practice. The goal is to prevent the agent from leaving element use to individual discretion, creating profiles that are too rigid or too vague, or failing to maintain the profile as practice evolves.

## Core Rules

### Define The Element Set Deliberately

An application profile starts by declaring which elements from the schema the project uses. Not every schema element will be relevant, and using all of them burdens creators without benefit.

Element set decisions:

- include elements that serve the project's user tasks and use cases;
- exclude elements the collection will never populate meaningfully;
- distinguish core elements used for every object from optional elements used when applicable;
- justify each inclusion and exclusion with a purpose.

A bloated element set invites inconsistency. A lean, purposeful set is easier to populate consistently and to enforce.

### Set Obligation Levels For Each Element

Obligation levels tell creators what is required, recommended, and optional. Clear obligations are the primary tool for consistency.

Typical obligation levels:

- mandatory, must be populated for every object, failure blocks publication;
- recommended, should be populated, absence flagged for review;
- optional, populated when the information exists;
- conditional, mandatory or recommended under specific circumstances, such as for a particular object type.

Assign obligation based on the element's role in discovery and selection. Title and identifier are usually mandatory; detailed provenance is usually optional. Document the rationale for each level.

### Specify Value Vocabularies And Input Formats

Consistency depends on what goes in each element, not just whether it is filled. The profile must specify the controlled vocabularies, formats, and conventions.

Specifications:

- controlled vocabulary for each applicable element, LCSH, AAT, MeSH, local thesaurus;
- format for dates, ISO 8601;
- format for names, authority file form;
- format for identifiers, URI or persistent identifier scheme;
- language and script conventions;
- whether free-text is permitted and how it relates to controlled terms.

An element filled with inconsistent formats or mixed controlled and free-text values breaks faceting and browsing. Specify the expected content precisely.

### Define Repeatable And Conditional Rules

Schemas allow repetition and conditionality, but projects need local rules about when and how to use them.

Rules to define:

- which elements are repeatable and the expected maximum or typical count;
- how multiple values are ordered or prioritized;
- conditional obligations, such as a rights statement mandatory for online objects;
- how variants and alternative values are recorded;
- how relationships and links are expressed.

Undefined repetition and conditionality lead to one creator entering five subjects and another entering twenty, fragmenting the collection's browse experience.

### Provide Clear Examples And Non-Examples

Rules are abstract; examples make them concrete. Each element in the profile should include examples of correct and incorrect use.

Examples to provide:

- a correct populated example for a typical object;
- examples for edge cases the element is meant to handle;
- non-examples showing common mistakes and why they are wrong;
- examples tied to the controlled vocabulary where applicable.

Creators follow examples more reliably than rules alone. Invest in clear, realistic examples for every element.

### Document The Rationale And Provenance

A profile is easier to maintain and trust when its decisions are explained. Document why each rule exists.

Document:

- why each element was included or excluded;
- why each obligation level was set;
- why each vocabulary and format was chosen;
- the standards and schemas the profile draws from;
- the date of adoption and revision history.

Rationale turns the profile from a set of arbitrary rules into a reasoned practice that can be revised intelligently.

### Align The Profile With System Capabilities

A profile is only useful if the system can enforce and display it. Align the profile with what the repository supports.

Alignment checks:

- can the system enforce mandatory fields and validation rules;
- can it handle the specified controlled vocabularies and lookups;
- can it display and facet the elements as profiled;
- can it harvest and export the profiled elements;
- does the profile fit the input forms creators actually use.

A profile the system cannot enforce becomes aspirational. Verify system support and adjust the profile or the system to match.

### Maintain And Revise The Profile Over Time

Collections and standards change. A profile that is never revised becomes stale and then ignored.

Maintenance:

- schedule periodic review of the profile against actual practice;
- revise when the schema, vocabularies, or system change;
- incorporate lessons from quality review and cleanup;
- version the profile and record what changed and why;
- train creators on revisions and update examples.

A maintained profile stays relevant; an abandoned one produces drift between documented and actual practice.

## Common Traps

### Leaving Element Use To Individual Discretion

Without obligation levels and rules, creators populate inconsistently. Define the element set and obligations.

### Profiles Too Rigid To Handle Collection Variety

Over-strict rules force awkward fits or omission. Build in conditional rules for legitimate variety.

### Profiles Too Vague To Enforce Consistency

Vague guidance invites interpretation. Specify vocabularies, formats, and examples.

### No Examples Or Only Abstract Rules

Creators follow examples better than rules. Provide concrete correct and incorrect examples.

### Undocumented Rationale

Arbitrary rules cannot be revised intelligently. Document why each decision was made.

### Profile Misaligned With System Capabilities

Unenforceable profiles are aspirational. Verify system support for mandatory fields, vocabularies, and faceting.

### Never Revised After Initial Creation

Stale profiles drift from actual practice. Schedule review and version the revisions.

### No Training On The Profile

A profile creators have not been trained on is ignored. Train and provide accessible documentation.

## Self-Check

- Is the element set chosen deliberately, with each inclusion and exclusion justified by purpose?
- Does each element have a clear obligation level, mandatory, recommended, optional, conditional, with rationale?
- Are value vocabularies, formats, and conventions specified for every applicable element?
- Are repeatable and conditional rules defined to prevent inconsistent population?
- Does each element include concrete examples of correct use and non-examples of common mistakes?
- Is the rationale for each profile decision documented for maintenance and revision?
- Is the profile aligned with the system's ability to enforce, display, facet, and harvest the elements?
- Is the profile maintained through scheduled review, versioning, and creator training?
