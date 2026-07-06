---
name: metadata_crosswalks_and_mapping.md
description: Use when the agent is creating a metadata crosswalk between schemas, mapping elements from one standard to another, handling lossy or one-to-many mappings, converting metadata for migration or harvesting, or validating mapped data against the target schema.
---

# Metadata Crosswalks And Mapping

Metadata crosswalks are the bridges that let data move between schemas, systems, and aggregators. They look like simple tables mapping element A to element B, but beneath the surface they are full of judgment and loss. Schemas rarely align one-to-one. One schema's element may split into several in another, or carry meaning the target has no way to express. Controlled vocabularies may not map. Cardinality may differ, with single values becoming repeatable or vice versa. Mandatory elements in the target may have no source. A crosswalk built without understanding these mismatches produces data that looks migrated but has lost precision, broken structure, or introduced false equivalences. Good crosswalk design maps deliberately, documents every lossy or conditional mapping, tests against real data, and validates the output so that migration and harvesting preserve meaning rather than just moving fields.

Use this skill when creating a metadata crosswalk, mapping elements between schemas, migrating metadata between systems, configuring harvesting, or validating mapped data. The goal is to prevent the agent from building naive one-to-one maps, ignoring lossy mappings, skipping testing, or delivering data that validates structurally but has lost its meaning.

## Core Rules

### Map At The Semantic Level, Not Just The Element Name

Element names are misleading. Two schemas may both have a "creator" element that mean different things, or a "date" element that captures different granularity. Map based on semantic meaning, not label similarity.

Semantic mapping:

- define what each source element actually captures, not just its name;
- define what each target element expects;
- map where the meanings align, not where the labels match;
- flag elements that look similar but differ in scope, cardinality, or controlled vocabulary;
- document the semantic basis for each mapping.

A "subject" in one schema may be free-text keywords; in another it may require a controlled vocabulary. Mapping them as equivalent without checking produces unusable data.

### Handle One-To-Many And Many-To-One Mappings Deliberately

Schemas rarely align one-to-one. One source element may need to map to several target elements, or several source elements may collapse into one target.

Mapping patterns:

- one-to-one, the simplest and safest case;
- one-to-many, a source element splits across target elements, often requiring parsing or conditional logic;
- many-to-one, multiple source elements combine into one target, risking loss of distinction;
- conditional mapping, an element maps differently based on its value or type;
- no mapping, a source element has no target equivalent, requiring a decision on loss or local extension.

Document each non-trivial mapping with its logic. One-to-many and many-to-one mappings are where meaning is most often lost or corrupted.

### Identify And Document Lossy Mappings

Some mappings inevitably lose information. A rich source element mapping to a simpler target, or a controlled vocabulary mapping to free-text, loses precision. This loss must be identified and documented, not hidden.

Lossy mapping types:

- granularity loss, a precise date mapping to a year;
- vocabulary loss, a controlled term mapping to free-text;
- cardinality loss, multiple values collapsing to one;
- structural loss, hierarchical data flattening to a string;
- qualifier loss, a typed element losing its type in the target.

For each lossy mapping, decide whether the loss is acceptable, whether a local extension can preserve the information, or whether the source should be retained alongside the migrated data. Silent loss corrupts the data's usefulness.

### Resolve Mandatory And Structural Mismatches

The target schema may require elements the source does not have, or impose structure the source lacks. These mismatches must be resolved explicitly.

Resolution strategies:

- for missing mandatory elements, derive values from other source data or use prescribed defaults;
- for structural mismatches, transform the data to fit the target structure;
- for cardinality mismatches, decide how to handle repeatable versus single values;
- for encoding mismatches, convert formats, dates to ISO 8601, identifiers to URIs;
- document every transformation and default applied.

Never leave a mandatory target element empty without a deliberate decision. An empty mandatory element fails validation and breaks the target system.

### Map Controlled Vocabularies Explicitly

Controlled vocabularies rarely map cleanly between schemas. The same concept may have different terms, different vocabularies may be used for the same element, and vocabulary sources may not be recorded consistently.

Vocabulary mapping:

- identify the vocabulary used in each source element;
- map to the target's expected vocabulary where one exists;
- where no equivalent vocabulary exists, decide whether to retain the source term as free-text or map to the closest target term;
- record the vocabulary source in the target so terms remain interpretable;
- handle vocabulary version differences, LCSH editions, MeSH updates.

Treating controlled terms as interchangeable strings destroys collocation in the target. Map vocabularies with awareness of their structure and source.

### Test The Crosswalk Against Real Data

A crosswalk designed in theory often fails on real data. Testing against representative records catches mismatches the design missed.

Testing practice:

- select a representative sample covering the collection's variety;
- run the crosswalk and inspect the output element by element;
- check for empty mandatory fields, lost values, and corrupted structure;
- validate the output against the target schema;
- have a domain expert review the mapped data for meaning, not just structure;
- iterate the crosswalk based on findings.

Testing is not optional. A crosswalk that has not been tested against real data will fail in production, often silently.

### Validate Mapped Data Against The Target Schema

Migration and harvesting output must validate against the target schema. Structural validity is necessary, though not sufficient, for usable data.

Validation:

- run schema validation on all mapped records;
- check required elements, cardinality, data types, and controlled value lists;
- report and resolve validation errors;
- distinguish validation success from semantic correctness, a record can validate and still be wrong;
- re-validate after any crosswalk revision.

Validation catches structural errors but cannot catch semantic loss. Use it as one check, not the only one.

### Document The Crosswalk For Maintenance And Transparency

Crosswalks are maintained over time as schemas and systems change. Documentation makes the crosswalk reproducible and trustworthy.

Document:

- the source and target schemas and versions;
- each mapping with its semantic basis and logic;
- all lossy mappings and the decisions made about them;
- transformations, defaults, and vocabulary mappings;
- the testing process and sample results;
- the date and responsible staff.

A crosswalk without documentation is an opaque transformation that no one can verify, revise, or trust.

## Common Traps

### Naive One-To-One Mapping By Element Name

Labels mislead. Map at the semantic level and check meanings.

### Ignoring Lossy Mappings

Silent loss corrupts data usefulness. Identify, document, and decide on each lossy mapping.

### Leaving Mandatory Target Elements Empty

Empty mandatory elements fail validation. Derive, default, or document the decision.

### Treating Controlled Terms As Interchangeable Strings

Vocabulary mapping destroys collocation if done carelessly. Map vocabularies with awareness of source and structure.

### Skipping Testing On Real Data

Theoretical crosswalks fail in production. Test against representative samples.

### Confusing Validation With Correctness

A validating record can still be semantically wrong. Use validation as one check among several.

### Undocumented Crosswalks

Opaque transformations cannot be verified or revised. Document mappings, losses, and logic.

### One-Time Crosswalks With No Maintenance Plan

Schemas change. Build in review and revision of the crosswalk over time.

## Self-Check

- Is each mapping based on semantic meaning, not just element name similarity?
- Are one-to-many, many-to-one, and conditional mappings handled with documented logic?
- Is every lossy mapping identified, documented, and resolved with a deliberate decision?
- Are mandatory and structural mismatches in the target resolved rather than left empty?
- Are controlled vocabularies mapped explicitly with their sources recorded in the target?
- Has the crosswalk been tested against a representative sample of real data?
- Does the mapped output validate against the target schema, with errors resolved?
- Is the crosswalk fully documented for maintenance, transparency, and future revision?
