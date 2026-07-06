---
name: uris_persistent_identifiers_and_reconciliation.md
description: Use when the agent is assigning or managing persistent HTTP URIs for entities, reconciling local entities against external authority sources like Wikidata or VIAF, managing same-as links, or handling identifier persistence, deprecation, and link integrity over time.
---

# URIs, Persistent Identifiers, And Reconciliation

In linked data, the identifier is the entity. A persistent HTTP URI is how the web refers to a work, a person, a place, or a concept, and if that URI breaks, changes, or points to the wrong thing, the entire graph connection fails. Reconciliation is the complementary work of determining that a local entity and an external entity, say a local author record and a VIAF cluster, are the same, and recording that link so data can be merged and traversed across sources. Both tasks look mechanical but are full of judgment. URIs must be designed for decades of stability, not for the current system. Reconciliation must verify that two identifiers actually refer to the same entity, because conflation and splitting errors in source data are common and a wrong same-as link silently corrupts every downstream consumer. Good practice treats identifiers as long-term institutional commitments, reconciles against multiple attributes rather than name strings alone, and maintains links as the source ecosystem evolves.

Use this skill when designing or assigning persistent URIs, reconciling entities against external authorities, managing same-as links, or maintaining identifier and link integrity. The goal is to prevent the agent from using transient identifiers, recording unverified same-as links, conflating entities, or neglecting link maintenance until the graph rots.

## Core Rules

### Design URIs For Persistence, Not For The Current System

A persistent URI must remain resolvable and meaningful for decades, surviving system migrations, reorganizations, and technology changes. Designing it for the current platform guarantees future breakage.

Persistence principles:

- use stable, institution-controlled domains or persistent identifier services;
- avoid embedding technology, software version, or internal database structure in the URI;
- prefer opaque or semantically stable path patterns over fragile ones;
- ensure the institution commits to resolving the URI even after migrations;
- document the URI scheme and its persistence guarantees.

A URI like example.org/work/12345 that survives a repository migration is valuable. One like oldsystem.cgi?id=12345 breaks when the system is replaced. Design for the entity, not the platform.

### Reconcile Against Multiple Attributes, Not Name Alone

Two entities with the same name string may be different people, and the same entity may appear under different strings. Name-only reconciliation causes conflation and splitting errors.

Reconcile using:

- name plus life or activity dates;
- field of activity or profession;
- affiliated works, titles, or institutions;
- geographic context;
- variant names and pseudonyms;
- multiple external sources for corroboration.

Verify a match across several attributes before recording a same-as link. When sources conflict, record the conflict and the preferred basis rather than forcing a match.

### Verify Same-As Links Before Recording

A same-as link asserts that two identifiers refer to the same entity. A wrong assertion propagates through every consumer. Verification is mandatory.

Verification:

- resolve the external identifier and confirm it describes the same entity;
- check the external source for known conflation or splitting issues;
- verify ORCID and ISNI claims against the entity's actual works;
- check VIAF and Wikidata for incorrectly merged clusters;
- record low-confidence links separately or with a note, not as confident same-as.

A same-as link is a strong claim. Treat it as such, and document the verification so the link can be trusted and maintained.

### Use Established Identifiers Where They Exist

Creating a new URI for an entity that already has a stable external identifier fragments the web of data. Use established identifiers and link to them.

Established identifiers:

- VIAF for persons and organizations across national libraries;
- Wikidata Q identifiers for broad web-scale linkage;
- ORCID for individual researchers;
- ISNI for broad contributor disambiguation;
- GeoNames or TGN for places;
- LCNAF or national authority files for library authorities;
- ROR for research organizations.

Mint a local URI only when no suitable external identifier exists, and even then, link outward to any partial matches. Local URIs that ignore external identifiers create isolated nodes.

### Maintain Link Integrity Over Time

URIs and external records change. VIAF clusters merge and split, Wikidata items are revised, ORCID records are updated, and identifier schemes are deprecated. Links recorded once and never checked rot.

Maintenance:

- periodically validate that URIs resolve and still describe the correct entity;
- update same-as links when source systems revise, merge, or split records;
- replace deprecated identifiers with current equivalents;
- monitor for source-system conflation events that affect your links;
- record the date and basis of reconciliation for scheduled re-verification.

Link integrity is ongoing. A graph with broken or stale links degrades faster than it is built.

### Handle Conflicts Between Sources Deliberately

External sources disagree. VIAF and Wikidata may list different birth dates for the same person, or one may conflate two people the other separates. The cataloger must resolve these conflicts deliberately.

Conflict resolution:

- identify the conflict and the sources involved;
- assess which source is more authoritative for the attribute in question;
- record the preferred value and note the conflict;
- link to both sources where both are useful, with the conflict documented;
- flag for review rather than silently choosing.

Silent conflict resolution embeds an unreviewable decision in the data. Documented resolution supports trust and future correction.

### Separate Confident From Provisional Links

Not all reconciliations are equally certain. Mixing confident and provisional same-as links without distinction misleads consumers.

Practice:

- use same-as or exact-match only for verified, high-confidence equivalences;
- use weaker predicates, close-match or see-also, for provisional or partial links;
- record confidence or verification status where the model supports it;
- route low-confidence reconciliations through review before promotion to same-as.

A consumer traversing a same-as link expects a reliable equivalence. Provisional links should be marked so consumers can treat them appropriately.

### Document Identifier And Reconciliation Practice

Identifier assignment and reconciliation involve institutional commitments and judgment that must be documented.

Document:

- the URI scheme, its persistence guarantees, and the minting process;
- the external identifier sources used and their roles;
- the reconciliation criteria and verification process;
- how conflicts and provisional links are handled;
- the maintenance schedule for link validation.

Documentation makes identifier practice reproducible and trustworthy across staff and time.

## Common Traps

### URIs Tied To The Current System

URIs embedding technology or database structure break on migration. Design for persistence.

### Name-Only Reconciliation

Same name strings may be different entities. Reconcile on multiple attributes.

### Unverified Same-As Links

A wrong same-as propagates through consumers. Verify before recording.

### Minting Local URIs That Ignore External Identifiers

Isolated local nodes fragment the web of data. Use and link to established identifiers.

### Links Recorded Once And Never Maintained

Source systems change. Unmaintained links rot. Schedule validation.

### Silent Conflict Resolution

Choosing a source without documenting the conflict embeds unreviewable decisions. Document conflicts and preferences.

### Mixing Confident And Provisional Links

Treating all links as same-as misleads consumers. Mark provisional links distinctly.

### Undocumented Identifier Practice

Without documentation, identifier commitments and reconciliation logic cannot be maintained. Document schemes, sources, criteria, and maintenance.

## Self-Check

- Are URIs designed for persistence across system migrations rather than tied to the current platform?
- Is reconciliation based on multiple attributes, not name strings alone?
- Are same-as links verified against the external source before recording?
- Are established external identifiers used and linked to before minting local URIs?
- Are links periodically validated and updated as source systems revise, merge, or split records?
- Are conflicts between sources identified, resolved with a documented preference, and noted in the data?
- Are confident and provisional links distinguished using appropriate predicates or confidence markers?
- Is the identifier and reconciliation practice documented, including schemes, sources, criteria, conflicts, and maintenance schedule?
