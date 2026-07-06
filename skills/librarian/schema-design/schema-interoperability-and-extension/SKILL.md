---
name: schema_interoperability_and_extension.md
description: Use when the agent is designing a schema to interoperate with other standards, extending an existing schema with local elements, planning crosswalks and mappings during schema design, or ensuring a custom schema can harvest, export, and align with community standards.
---

# Schema Interoperability And Extension

No metadata schema exists in isolation. A schema designed without regard for interoperability traps its data in one system, unable to harvest into aggregators, exchange with partners, or migrate when systems change. Interoperability is not something added after a schema is built; it is a design constraint from the start. The schema architect must decide which community standards to align with, how local needs extend beyond those standards, how every element maps outward, and how the schema will harvest and export. Extension is the complementary challenge: when a standard schema does not capture what a collection needs, the local extension must be designed so it enriches rather than replaces the standard, preserving the ability to crosswalk and interoperate. The failure modes are isolation, a schema so custom that nothing maps outward, and corruption, local extensions that overwrite or conflict with standard semantics. Good design treats interoperability and extension as inseparable, building a schema that serves local depth while remaining a citizen of the broader metadata ecosystem.

Use this skill when designing a schema for interoperability, extending a standard schema locally, planning crosswalks during design, or ensuring custom schemas harvest and export correctly. The goal is to prevent the agent from building isolated schemas, creating extensions that break standard alignment, or treating interoperability as a post-hoc concern.

## Core Rules

### Design For Interoperability From The Start

Interoperability must be a design constraint, not a migration task. A schema built first and crosswalked later accumulates elements that cannot map.

Design-time interoperability:

- identify the community standards and aggregators the data must reach;
- choose elements that align with those standards where possible;
- avoid local-only semantics where a standard element could serve;
- plan the crosswalk to each target as elements are designed;
- document the interoperability target for every element.

An element designed without a mapping in mind often encodes information no target can express. Design with the crosswalk visible from the beginning.

### Align With Community Standards Before Customizing

Community standards exist because interoperability requires shared structure. Align with them first, and customize only where they genuinely fall short.

Alignment practice:

- adopt a base standard, Dublin Core, MODS, schema.org, as the foundation;
- use the standard's elements and semantics wherever they fit;
- extend only for information the standard cannot capture;
- document the relationship between local and standard elements;
- participate in the standard's community to influence future versions.

A schema that ignores standards to be "better" isolates its data. A schema that aligns first and extends second gains both local depth and interoperability.

### Design Local Extensions To Preserve Standard Alignment

Local extensions are necessary but dangerous. An extension that conflicts with or replaces standard semantics breaks interoperability even when it looks compatible.

Extension design:

- put local elements in a distinct namespace, clearly marked as local;
- ensure local elements do not duplicate or override standard elements;
- design each local element to crosswalk to the closest standard element;
- document the extension's semantics, provenance, and mapping;
- plan how the extension migrates if the base standard changes.

A local "localSubject" element that parallels "subject" but uses different semantics confuses consumers. Namespace it, document it, and map it.

### Plan Crosswalks For Every Element During Design

Every element in a custom or extended schema needs a documented crosswalk to the target standards. Elements without a mapping become orphans in exchange.

Crosswalk planning:

- for each element, identify the target standard element it maps to;
- document one-to-one, one-to-many, and lossy mappings explicitly;
- decide how unmappable elements are handled in export, dropped or preserved locally;
- test the crosswalk against sample data during design, not after;
- revise elements that map poorly to improve interoperability.

An element that maps to nothing in any target standard should be reconsidered unless its local value justifies the isolation.

### Ensure The Schema Supports Harvesting And Export

Interoperability depends on the schema being harvestable and exportable in standard ways. The schema and system must support these flows.

Harvesting and export support:

- ensure required elements for harvesting protocols, OAI-PMH, are present and populated;
- provide the data in standard serializations the targets expect;
- avoid dependencies on local-only fields for core harvest records;
- test harvest into target aggregators with sample data;
- document the harvest and export configuration.

A schema that cannot be harvested or exported cleanly, regardless of its internal quality, cannot participate in the interoperable ecosystem.

### Avoid Semantic Drift Between Local And Standard Elements

When a local element parallels a standard one, the semantics must stay aligned or the mapping becomes false. Drift happens silently as local practice evolves.

Drift prevention:

- define local elements to match standard semantics where they parallel;
- review local element usage against the standard on a schedule;
- update local practice when the standard evolves;
- document any deliberate divergence and its rationale;
- avoid using the same label for locally and standardly defined elements with different meanings.

Semantic drift turns a documented crosswalk into a lie. Maintain alignment through review.

### Document The Interoperability Strategy

The schema's interoperability approach, standards, crosswalks, extensions, and harvest configuration, must be documented as a coherent strategy.

Document:

- the community standards the schema aligns with;
- the crosswalk to each target, with mappings and loss notes;
- the local extensions, their namespaces, semantics, and mappings;
- the harvesting and export configuration;
- the review schedule for maintaining alignment.

Documentation makes interoperability a maintained property rather than a one-time achievement.

### Participate In Standards Communities

Interoperability is a two-way relationship. Participating in standards communities improves both the standard and the local schema's alignment.

Participation:

- contribute feedback on standards from local implementation experience;
- propose elements or changes that would reduce the need for local extension;
- share crosswalks and profiles with the community;
- track standards evolution and plan adoption;
- coordinate with peer institutions on shared extensions.

Isolated schemas miss the chance to shape the standards they depend on. Participation aligns local and community evolution.

## Common Traps

### Treating Interoperability As A Post-Hoc Concern

Crosswalking after design produces unmappable elements. Design with interoperability from the start.

### Building Fully Custom Schemas

Schemas that ignore community standards isolate their data. Align first, extend second.

### Local Extensions That Override Standard Semantics

Extensions conflicting with standards break mappings. Namespace, document, and align extensions.

### Elements With No Outward Mapping

Unmappable elements become orphans in exchange. Plan a crosswalk for every element.

### Schema That Cannot Harvest Or Export Cleanly

A non-harvestable schema cannot participate in the ecosystem. Support standard harvesting and export.

### Semantic Drift Between Local And Standard

Diverging local practice falsifies documented crosswalks. Review and maintain alignment.

### Undocumented Interoperability Strategy

Without documentation, interoperability cannot be maintained. Document standards, crosswalks, extensions, and configuration.

### Isolation From Standards Communities

Missing the chance to shape standards increases local extension burden. Participate and coordinate.

## Self-Check

- Is interoperability designed in from the start, with target standards and crosswalks visible during element design?
- Is the schema aligned with community standards as a foundation before local customization?
- Do local extensions preserve standard alignment through distinct namespaces, documentation, and crosswalks?
- Does every element have a planned crosswalk to target standards, with lossy mappings documented?
- Does the schema support standard harvesting and export protocols with required elements populated?
- Is semantic alignment between local and standard elements maintained through scheduled review?
- Is the interoperability strategy, standards, crosswalks, extensions, harvest configuration, fully documented?
- Does the institution participate in standards communities to align local and community evolution?
