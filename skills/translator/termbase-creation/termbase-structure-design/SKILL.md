---
name: termbase_structure_design.md
description: Use when the agent is designing the data model and field schema of a terminology database, choosing entry-level versus language-level fields, defining concept entries, picklists, cross-references, and metadata for a termbase, or deciding the structural architecture of a multilingual terminology resource before population.
---

# Termbase Structure Design

A termbase is only as usable as the data model underneath it. Translators do not fail to apply terminology because they lack will; they fail because the entry they need is split across three records, the field they need is empty, the picklist value does not match their context, or the structure forces one target equivalent onto a term that needs three. Structure design is the work of deciding what a termbase entry represents, which fields it carries, how languages and concepts relate, and how metadata makes entries findable, trustworthy, and machine-actionable. This is an architecture decision, not a data-entry task. Once a termbase is populated under a weak schema, restructuring it later means re-authoring thousands of entries, and most teams never find the time, so the weak structure calcifies and the termbase underperforms for years.

Agents tend to miss this work because a flat list of source-term-to-target-term pairs looks sufficient on day one. It is not. Real terminology has polysemy, multi-domain variation, register differences, part-of-speech dependence, and status lifecycles, and a structure that cannot express those distinctions produces a resource that translators cannot query precisely. The harm this skill prevents is the quiet, expensive failure of a termbase that exists, is populated, is even integrated into tooling, yet is structurally incapable of delivering the right equivalent at the right moment.

## Core Rules

### Decide What An Entry Represents Before Adding Fields

The first structural decision is the unit of organization: does one entry represent one concept, one source term, or one source-target pair? Choose concept-based entries. In a concept-based model, a single concept entry holds the definition and metadata, and each language section holds the terms that label that concept. This prevents the most common structural defect: duplicate entries for the same concept under different surface spellings, each carrying a fragment of the information and none carrying all of it.

If you instead key entries on source terms, the same concept expressed as a noun and a verb, or as an acronym and its expansion, becomes separate records with no enforced link, and translators see partial, contradictory guidance. Make the concept the anchor, and let surface forms live as variants inside the entry.

### Separate Entry-Level From Language-Level Fields

Fields belong to different scopes, and conflating them corrupts data. Entry-level fields describe the concept itself: definition, subject field, concept identifier, status, owner, source authority, and modification date. These are true once per concept. Language-level fields describe the term as expressed in a given language: the term text, part of speech, grammatical notes, register, and context examples. These are true once per language.

A common error is placing part of speech or definition at the wrong scope. A definition is conceptual and belongs at entry level; placing it under a language forces you to restate it for every language and invites drift between the copies. Part of speech is a property of a term in a language and belongs at language level; placing it at entry level is wrong because the source may be a noun while a target equivalent is a verb. Define the scope of every field explicitly in the schema documentation.

### Model Multi-Equivalence Explicitly

Many concepts have more than one approved target equivalent, and the structure must express why. A single equivalent field forces an artificial choice. Instead, model multiple target terms per language, each with its own status, register, domain, and context. Then a translator querying the entry sees the preferred equivalent, the admitted alternative for formal registers, and the deprecated form they must avoid, with the conditions attached to each.

Equally, model multi-domain and multi-product variation. A term may have one preferred equivalent in a consumer product line and a different one in an enterprise product line, both correct in their context. Use domain or product fields to scope equivalents so the translator selects by context, not by guess.

### Use Controlled Picklists, Not Free Text, For Categorical Fields

Fields such as status, subject field, part of speech, register, and entry type must be controlled vocabularies, not free text. Free text in categorical fields guarantees fragmentation: one contributor writes Preferred, another writes preferred, another writes APPROVED, and the three never aggregate. Each becomes its own bucket, and filtering breaks.

Define a closed picklist for every categorical field, document the meaning of each value, and enforce it at entry. For subject field, use a taxonomy rather than ad hoc labels, because terminology reuse depends on being able to filter by domain. A picklist that nobody consults is as bad as free text, so publish the controlled vocabularies alongside the schema.

### Require Disambiguating Context As Structure, Not Option

Context is the field that makes an entry usable, and it should be structural, not optional commentary. Define at minimum a source context sentence showing the term in use and, where available, a target context sentence showing the approved equivalent in a real translation. Make these distinct fields with their own language scope, not a single free-text note.

The reason is operational: a translator who finds three candidate entries for a polysemous source term chooses among them by reading the context, not the definition. If context is buried in a notes field or omitted, the translator guesses, and the termbase has failed its purpose. Structure context as a first-class field and treat an entry without it as incomplete.

### Build Cross-References For Related Concepts

Terminology is a network, not a list. Concepts relate to each other as broader, narrower, related, or antonymic, and the structure should express these links. Without cross-references, a translator who looks up the right entry still misses the adjacent concept that governs their actual case, such as a broader category whose approved equivalent constrains the narrower one.

Define a relation field that points to other concept entries and labels the relation type. This also supports synonym and antonym management and prevents near-duplicate entries, because a candidate new concept can be checked against existing related entries before creation.

### Carry Provenance And Lifecycle Metadata

A termbase entry is a claim, and every claim needs provenance. Structure must carry who created the entry, who approved it, the source authority for the equivalent, the creation date, and the last-modification date. Without provenance, entries cannot be challenged, audited, or trusted during disputes, and reviewers cannot tell a freshly approved term from a stale guess.

Carry status as a lifecycle field with defined transitions: candidate, proposed, preferred, admitted, deprecated, forbidden. Make status queryable so translators can be shown only reliable entries and so governance reports can find deprecated terms still in use. Provenance and status together make the termbase governable rather than merely populated.

### Design For Interoperability And Tool Integration

A termbase that cannot be exchanged or queried by translation tools is an island. Design the schema to map to a portable format such as TBX or a well-documented custom interchange, so the resource can move between tools and survive vendor changes. Even if you start in a spreadsheet, keep the column structure compatible with a concept-and-language model so migration is possible later.

Plan how the structure will be consumed: which fields surface in the translation environment during work, which are query-only, and which are governance-only. A structure that cannot be projected into the translator's working view delivers no benefit regardless of its internal elegance.

## Common Traps

### Keying Entries On Source Terms Instead Of Concepts

When the source term is the key, one concept spreads across multiple records and translators see contradictory fragments. Key on the concept and hold variants inside.

### Putting Concept Fields At Language Scope

Definitions and status placed under a language get restated and drift across copies. Place concept-level fields at entry level and term-level fields at language level.

### Single Equivalent Field Forcing One Choice

A structure that allows only one target term per concept cannot express register or domain variation and forces translators to ignore the entry when the single equivalent does not fit.

### Free Text In Categorical Fields

Uncontrolled status, domain, and part-of-speech values fragment into non-aggregating buckets and break filtering. Use closed picklists.

### Treating Context As Optional Commentary

When context is a free-text note rather than a required field, entries arrive without it and translators must guess which entry applies. Make context structural.

### No Cross-References Between Related Concepts

Without relations, adjacent concepts that constrain each other are invisible, and near-duplicates proliferate unchecked.

### Omitting Provenance And Status Lifecycle

Entries without source, approver, date, or status cannot be audited, challenged, or filtered by reliability, so the termbase looks authoritative while being untrustworthy.

### Designing Only For Human Reading

A structure that looks fine in a spreadsheet but cannot be exchanged or surfaced in translation tools delivers no operational value and traps the resource in one tool.

## Self-Check

Before approving a termbase structure, verify:

- The unit of organization is the concept, with surface forms held as variants inside concept entries, not as separate keyed records.
- Every field has a defined scope: concept-level fields such as definition and status sit at entry level; term-level fields such as part of speech and register sit at language level.
- The schema supports multiple target equivalents per concept, each with its own status, register, domain, and context, rather than forcing a single equivalent.
- All categorical fields use closed, documented picklists, including status, subject field, part of speech, and register.
- Context is a first-class structural field with source and target scope, and entries without it are treated as incomplete.
- Cross-reference fields express broader, narrower, related, and antonymic relations between concept entries.
- Provenance fields (creator, approver, source authority, creation date, modification date) and a lifecycle status field are present and queryable.
- The structure maps to a portable interchange format and can be projected into the translation environment's working view.
- No field conflates concept scope with language scope, and no categorical field accepts free text.
- A translator querying a polysemous or multi-domain term would receive scoped, disambiguated equivalents rather than a single ambiguous record.
