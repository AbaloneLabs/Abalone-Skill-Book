---
name: metadata_interoperability_and_crosswalks.md
description: Use when the agent is mapping metadata between schemas, building or evaluating a crosswalk between standards such as Dublin Core MODS and BIBFRAME, exchanging records between systems, deciding what is lost or distorted in conversion, handling element mismatches and granularity differences, or diagnosing why records became degraded or incomplete after migration between systems.
---

# Metadata Interoperability And Crosswalks

Library metadata does not live in a single schema. A record may originate in MARC, be exposed as Dublin Core for harvesting, mapped to MODS for a digital collection, transformed to BIBFRAME for a next-generation catalog, and repackaged as Schema.org for web discovery. Each of these schemas was designed for different purposes, with different elements, granularity, and semantics, and the mappings between them are never clean one-to-one correspondences. A crosswalk is the set of rules that governs these transformations, and a poorly designed crosswalk silently degrades metadata: dropping elements that have no target, collapsing distinctions into generic fields, distorting meaning through semantic mismatch, and producing records that look complete but have lost the precision that made them useful. The judgment problem is mapping between schemas in a way that preserves as much meaning as possible, documenting what is lost, and avoiding the false confidence that a record that converts is a record that survived intact.

Use this skill when building or evaluating a metadata crosswalk, when migrating records between systems, when exposing metadata in a different schema for harvesting or discovery, or when diagnosing why records degraded after conversion. The goal is to prevent the agent from treating crosswalks as mechanical element-to-element mappings, from assuming conversion preserves meaning, and from losing metadata precision through careless transformation.

## Core Rules

### Recognize That Crosswalks Are Lossy By Nature

No two metadata schemas align perfectly. They differ in element granularity, semantics, mandatory versus optional fields, controlled vocabularies, and structural assumptions. A crosswalk between them inevitably loses, collapses, or distorts some information. The first discipline is accepting that conversion is lossy and designing the crosswalk to minimize and document the loss, rather than pretending it does not occur.

Accept lossiness by:

- identifying, before mapping, which elements in the source have no clean target;
- identifying where source distinctions collapse into a single target field;
- identifying where target fields have no source equivalent and will be empty;
- documenting each lossy mapping so users of the converted records understand what changed.

A crosswalk presented as a clean mapping almost certainly hides loss. An honest crosswalk names its compromises.

### Map At The Semantic Level, Not Just The Element Level

Element-to-element mapping, source field A goes to target field B, is the shallowest form of crosswalk and the most prone to distortion. Two fields with similar names may have different meanings, scopes, or rules. A title in one schema may include subtitles and statements of responsibility; in another it may be the title proper alone. Mapping by name without examining semantics produces records that look right and are wrong.

Map semantically by:

- examining the definition and scope of each element in both schemas, not just the label;
- checking the input rules, what content is allowed and how it is formatted;
- identifying semantic mismatches where a field carries different meaning despite similar names;
- choosing the target element by meaning, and documenting where the match is approximate.

A semantic mapping is slower to build but produces records that mean what they appear to mean.

### Handle Granularity Differences Deliberately

Schemas differ in granularity. One schema may have separate elements for title, subtitle, and statement of responsibility; another may have a single title field. One may distinguish creator, contributor, and publisher with precise roles; another may have a generic contributor field. Mapping across granularity differences requires explicit decisions about collapsing, splitting, or combining.

Handle granularity by:

- when collapsing many source fields into one target, concatenating with clear delimiters and documenting the combination;
- when splitting one source field into multiple targets, using parsing rules that handle variation robustly;
- when a source distinction has no target equivalent, deciding whether to preserve it in a note or accept the loss;
- documenting each granularity decision so the transformation is reversible in principle.

Unconsidered granularity collapse is the most common source of silent metadata degradation.

### Preserve Controlled Vocabulary And Authority Control Through Mapping

Controlled vocabularies and authority control carry much of a record's precision: a subject heading from a controlled vocabulary is more useful than free-text keywords, and an authorized name form disambiguates where a string does not. Crosswalks that strip vocabulary codes, drop authority source information, or convert controlled terms to free text lose this precision even when the term string survives.

Preserve control by:

- mapping controlled vocabulary terms to controlled vocabularies in the target where equivalents exist;
- recording the vocabulary source and code, not just the term string, where the target supports it;
- when no controlled target exists, preserving the term in a note with its source rather than silently demoting it;
- mapping authority records and identifiers, such as VIAF or LCCN, alongside the name strings.

A subject term that survives conversion as free text has lost its controlled-vocabulary power even though the words are the same.

### Validate Crosswalk Output Against Real Records

A crosswalk designed on paper and never tested against real records will fail on the variation that real records contain. Records have exceptions, local practice, legacy data, and edge cases that the design did not anticipate. Validation against a representative sample of actual records reveals the failures before they propagate to the whole collection.

Validate by:

- running the crosswalk against a sample spanning record types, languages, and formats;
- manually reviewing converted records for loss, distortion, and error;
- identifying systematic failures and refining the mapping to handle them;
- re-validating after each revision until the output quality is acceptable.

A crosswalk that has never seen real data is a hypothesis. Validation turns it into a reliable transformation.

### Document The Crosswalk For Transparency And Maintenance

A crosswalk is an institutional asset that must be understood by people who did not build it and maintained as schemas evolve. Undocumented crosswalks become opaque, unmaintainable, and a source of recurring error. Documentation makes the transformation transparent and revisable.

Document by:

- recording each mapping rule, including source element, target element, and transformation logic;
- noting lossy mappings and what is sacrificed;
- recording controlled vocabulary and authority handling;
- versioning the crosswalk as schemas or local practice change;
- making the documentation available to anyone working with the converted records.

A crosswalk without documentation is a black box that no one can safely modify or audit.

### Decide Whether Conversion Is Reversible And Plan Accordingly

Some conversions are reversible, the source can be reconstructed from the target; most are not, because information is lost. Whether reversibility matters depends on the purpose: a one-way exposure for harvesting may not need it, while a migration that replaces the source system does. The reversibility question should be answered before the crosswalk is built.

Address reversibility by:

- determining whether the source records will be retained or discarded after conversion;
- if the source is discarded, ensuring critical information is preserved in the target or archived;
- if reversibility is required, designing the crosswalk to retain enough information to reconstruct;
- archiving the original records and the crosswalk version used, so future re-conversion is possible.

An irreversible migration that discards the source without preserving critical information is a one-way loss that cannot be undone.

### Test Round-Trip And Downstream Use

Records are converted for a purpose: discovery, harvesting, exchange, or a new system. The test of a crosswalk is not only whether the converted records are internally correct but whether they work in the downstream system for their intended purpose. A record that converts cleanly but displays poorly, searches badly, or breaks the target system has not succeeded.

Test downstream by:

- loading converted records into the target system and testing discovery, display, and function;
- checking that facets, search indexes, and sort keys populate correctly;
- verifying that controlled vocabulary and authority links resolve in the target;
- testing with the workflows real users will perform, not just record inspection.

A crosswalk validated only by record inspection may produce records that are technically correct and practically broken.

## Common Traps

### Assuming Conversion Preserves Meaning

A record that converts is not necessarily a record that survived intact. Conversion is lossy; design for and document the loss.

### Element-To-Element Mapping Without Semantic Examination

Fields with similar names may have different meanings. Map at the semantic level, not the label level.

### Unconsidered Granularity Collapse

Collapsing distinct source fields into a generic target field without rules destroys precision. Handle granularity deliberately.

### Stripping Controlled Vocabulary And Authority Control

Converting controlled terms to free text loses disambiguation and search power even when the words survive. Preserve vocabulary and authority information.

### No Validation Against Real Records

A crosswalk designed on paper fails on real variation. Validate against a representative sample.

### Undocumented Crosswalks

Without documentation, the transformation cannot be audited, maintained, or understood by others. Document every rule and lossy mapping.

### Irreversible Migration Without Preservation

Discarding source records after a lossy conversion without archiving critical information is a permanent loss.

### Validating Only By Record Inspection

Records that convert cleanly may still break discovery, display, or function in the target system. Test downstream use.

## Self-Check

- [ ] The crosswalk is designed with explicit recognition that conversion is lossy, and each lossy mapping is documented.
- [ ] Elements are mapped at the semantic level, examining definitions and input rules, not just labels.
- [ ] Granularity differences, collapsing many-to-one or splitting one-to-many, are handled with explicit rules.
- [ ] Controlled vocabulary terms and authority control are preserved through mapping, not stripped to free text.
- [ ] The crosswalk has been validated against a representative sample of real records and refined based on failures.
- [ ] Every mapping rule, including lossy ones, is documented for transparency and maintenance.
- [ ] Reversibility has been considered, and source records or critical information are preserved where the conversion is irreversible.
- [ ] Converted records have been tested in the downstream system for discovery, display, facets, and function, not just inspected.
- [ ] No systematic loss or distortion has been allowed to propagate to the full collection unexamined.
- [ ] The crosswalk is versioned and maintainable as schemas and local practice evolve.