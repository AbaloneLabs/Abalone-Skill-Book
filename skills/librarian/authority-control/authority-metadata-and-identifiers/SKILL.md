---
name: authority_metadata_and_identifiers.md
description: Use when the agent is working with authority metadata standards such as MARC 21 Authority, RDA Registry, or VIAF, assigning persistent identifiers to authority entities, reconciling authority records across systems, or managing authority data as linked data.
---

# Authority Metadata And Identifiers

Authority control has moved beyond name strings in a local file. Today an authority entity, a person, corporate body, work, or concept, is also a node in a network of identifiers and linked data. The same creator may have a record in the LC Name Authority File, a VIAF cluster, a Wikidata item, an ORCID identifier, an ISNI, and a RINGGOLD or GRID identifier for their institution. These identifiers let systems and users refer to the entity unambiguously, connect authority data across the web, and support discovery far beyond the local catalog. Working with authority metadata now means choosing and recording the right identifiers, mapping authority data across standards, reconciling entities when systems disagree, and preparing authority data to function as linked data. Getting this wrong produces broken links, conflated or split entities across systems, and authority data that cannot interoperate. Getting it right makes authority control a foundation for linked-data discovery.

Use this skill when assigning or managing authority identifiers, mapping authority records across metadata standards, reconciling authority entities across systems, or preparing authority data for linked data. The goal is to prevent the agent from recording identifiers without verification, conflating entities across sources, ignoring identifier persistence and maintenance, or producing authority data that cannot interoperate with the broader linked-data ecosystem.

## Core Rules

### Assign Persistent Identifiers To Authority Entities

An authority entity should carry persistent identifiers that allow unambiguous reference across systems and time. The identifier is what makes the entity a stable node in the linked-data web.

Identifiers to consider:

- LCCN or national library authority number for the primary authority record;
- VIAF identifier for cross-institutional clustering;
- ORCID for individual researchers and authors;
- ISNI for broad disambiguation of contributors;
- Wikidata Q identifier for web-scale linked data;
- RINGGOLD, GRID, or ROR for institutions and organizations;
- local persistent identifiers for entities without a standard one.

Record identifiers in the authority record using the proper fields. An entity without identifiers is harder to reconcile and link as data flows between systems.

### Verify That An Identifier Belongs To The Right Entity

Identifiers are only useful if they point to the correct entity. Conflation, where one identifier covers two different people, and splitting, where one entity has multiple identifiers, are common errors in source data.

Before recording an identifier:

- confirm the identifier resolves to the correct entity, matching name, dates, and field of activity;
- check whether the source system has known conflation issues;
- verify ORCID and ISNI claims against the entity's actual works;
- check VIAF clusters for incorrectly merged or split authorities;
- when in doubt, record the identifier with a note about the verification status.

A wrong identifier silently corrupts linking across every system that consumes it. Verification is not optional.

### Choose The Right Authority Metadata Standard For The Context

Authority data is encoded in several standards, each suited to different contexts. Choose the standard that matches the system and the audience.

Standards and contexts:

- MARC 21 Authority, the traditional library authority format, for ILS integration;
- RDA Registry and RDA unconstrained properties, for element-level linked data;
- SKOS, for concept schemes and thesauri including subject vocabularies;
- schema.org, for web discovery and search engine consumption;
- BIBFRAME, for library linked data with authority components;
- local JSON or XML profiles, for repository-specific needs.

Match the encoding to where the data will be used. MARC authority records serve the ILS; RDF and linked-data formats serve the web and aggregators.

### Reconcile Authority Entities Across Systems

The same entity will appear differently across the LCNAF, VIAF, Wikidata, ORCID, and other sources. Reconciliation is the work of determining that these refer to the same entity and recording the links.

Reconciliation practice:

- match on name, dates, field of activity, and affiliated works or institutions;
- use VIAF as a bridge between national authority files;
- record same-as links between the local authority and external identifiers;
- flag low-confidence reconciliations for review;
- handle cases where sources disagree, such as different birth dates, by recording the conflict and the preferred source.

Reconciliation is judgment work. Two sources may use the same name string for different people, or different strings for the same person. Verify before linking.

### Maintain Identifier Links Over Time

Identifiers and their targets change. VIAF clusters merge and split, ORCID records are updated, Wikidata items are revised, and old identifier schemes are deprecated.

Maintenance:

- periodically validate that recorded identifiers still resolve to the correct entity;
- update links when source systems revise or merge records;
- replace deprecated identifiers with current equivalents;
- record the date and basis of reconciliation for future review;
- monitor for source-system conflation or split events that affect your links.

An identifier recorded once and never checked becomes a broken or wrong link. Identifier maintenance is ongoing authority work.

### Prepare Authority Data For Linked Data Consumption

Authority data that will be consumed as linked data must be structured for machine readability and interlinking, not just for human display.

Linked-data readiness:

- expose authority records with persistent HTTP URIs as identifiers;
- provide labels in multiple languages and scripts where relevant;
- include same-as and see-also links to external authorities;
- structure relationships, broader, narrower, related, as explicit links;
- provide the data in a standard serialization, RDF, JSON-LD, as the context requires;
- avoid local-only conventions that external systems cannot interpret.

Authority data locked in a MARC record that cannot be exposed as linked data limits the institution's participation in the broader ecosystem.

### Handle Works And Expressions As Authority Entities

Authority control increasingly covers not just names but works and expressions. A work, a distinct intellectual creation, and its expressions, translations, editions, can be authority-controlled entities.

Work and expression authority:

- use uniform titles or work identifiers to collocate all expressions of a work;
- record relationships between a work and its expressions, translations, adaptations;
- link to cooperative work authority files where they exist;
- distinguish the work from its manifestations in the authority structure;
- document the basis for work-level decisions.

Work-level authority control is more complex than name authority but increasingly important for discovery and for linked-data models like BIBFRAME.

### Document Identifier And Reconciliation Decisions

Identifier assignment and reconciliation involve judgment that must be documented for maintenance and trust.

Document:

- which identifiers are recorded and their verification status;
- the basis for reconciling entities across sources;
- any conflicts between sources and how they were resolved;
- the date of reconciliation and the sources consulted;
- local conventions for identifier recording.

Documentation makes authority metadata maintainable and trustworthy as it flows between systems and staff.

## Common Traps

### Recording Identifiers Without Verification

An unverified identifier may point to the wrong entity. Confirm the target before recording.

### Conflating Entities Across Systems

Two sources with the same name string may refer to different people. Reconcile on multiple attributes, not name alone.

### Choosing The Wrong Encoding Standard

MARC authority serves the ILS; linked-data formats serve the web. Match the encoding to the context.

### Recording Identifiers Once And Never Maintaining Them

Identifiers and their targets change. Periodic validation prevents broken and wrong links.

### Ignoring Work And Expression Authority

Name-only authority control misses the collocation benefits of work-level entities. Extend authority to works where feasible.

### Authority Data That Cannot Be Exposed As Linked Data

Records locked in a format that cannot interoperate limit participation in the broader ecosystem. Structure for linked data.

### Silent Reconciliation Without Documentation

Undocumented same-as links cannot be maintained or trusted. Record the basis and date.

### Mixing Identifier Schemes Without Recording Sources

Recording an identifier without noting its scheme makes the data uninterpretable. Always record the source and scheme.

## Self-Check

- Does each authority entity carry appropriate persistent identifiers, LCCN, VIAF, ORCID, ISNI, Wikidata, recorded in the proper fields?
- Was each identifier verified to resolve to the correct entity before recording?
- Is the authority metadata encoded in a standard suited to its context, MARC for ILS, RDF or JSON-LD for linked data?
- Are authority entities reconciled across systems using multiple matching attributes, with same-as links recorded?
- Are identifier links periodically validated and updated as source systems revise, merge, or deprecate records?
- Is the authority data structured for linked-data consumption, with persistent URIs, labels, relationships, and standard serialization?
- Are works and expressions handled as authority entities where collocation benefits justify the effort?
- Are identifier assignment and reconciliation decisions documented with basis, sources, conflicts, and dates?
