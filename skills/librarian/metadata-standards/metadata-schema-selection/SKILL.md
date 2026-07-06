---
name: metadata_schema_selection.md
description: Use when the agent is choosing a metadata schema for a collection or project, comparing schemas like Dublin Core, MODS, VRA Core, or DDI, deciding between simple and complex schemas, or matching a schema to object types, audiences, and interoperability requirements.
---

# Metadata Schema Selection

Choosing a metadata schema is one of the most consequential decisions in a digital project, and it is often made too casually. The schema determines what can be described, how richly, how the data interoperates with other systems, how it harvests into aggregators, and how much burden it places on metadata creators. A schema chosen because it is familiar or default may lack the elements a specialized collection needs, or may be so heavy that metadata creation becomes unsustainable. A schema chosen for maximum richness may isolate the collection from aggregators that expect simpler crosswalks. The right choice depends on the object types, the intended audiences, the systems and aggregators the data must reach, the available metadata creation capacity, and the long-term maintenance plan. Schema selection is a design decision with lasting consequences, because migrating metadata between schemas later is costly and lossy.

Use this skill when choosing a metadata schema for a collection or project, comparing candidate schemas, deciding between simple and complex schemas, or planning interoperability and crosswalks. The goal is to prevent the agent from defaulting to a familiar schema without analysis, choosing richness without considering sustainability, or picking a schema that fails to interoperate with required systems.

## Core Rules

### Match The Schema To Object Type And Domain

Different schemas were designed for different object types and domains. A schema that works for general cultural heritage may fail for scientific data, art images, or social science datasets.

Schema strengths by domain:

- Dublin Core, broad, cross-domain, good for general interoperability;
- MODS, richer than Dublin Core, library and cultural heritage;
- VRA Core, visual resources, images, and artworks;
- Darwin Core, biological occurrence and specimen data;
- DDI, social science surveys and datasets;
- EAD, archival finding aids and collection description;
- PREMIS, preservation metadata, paired with descriptive;
- schema.org, web discovery and search engine indexing.

Identify the dominant object types and the domain, then evaluate which schemas were built for them. A generic schema forced onto specialized content loses information that matters to domain users.

### Balance Richness Against Sustainability

Richer schemas capture more, but they cost more to create and maintain. A schema too rich for the project's capacity produces incomplete or inconsistent metadata.

Richness considerations:

- how many elements does each object realistically need;
- does the project have the staff time to populate rich elements consistently;
- will richness create validation and cleanup burdens;
- is the richness used by the intended discovery and delivery systems;
- could a simpler schema with local extensions achieve the needed depth.

Prefer a schema the project can populate consistently over a richer one that gets filled unevenly. Inconsistent rich metadata is worse than consistent simple metadata.

### Plan Interoperability And Crosswalks From The Start

Most collections must share data with aggregators, discovery layers, or partner systems. The schema must crosswalk cleanly to the targets.

Interoperability planning:

- identify the systems and aggregators the data must reach, DPLA, Europeana, SHAREVDE, OAI-PMH harvesters;
- check whether candidate schemas crosswalk to the targets without major loss;
- prefer schemas with established crosswalks over those requiring custom mapping;
- avoid local-only elements that map to nothing;
- test a sample harvest before committing.

A schema that works only in the local repository limits the collection's reach. Interoperability is a selection criterion, not an afterthought.

### Consider The Audience And Use Cases

The schema should serve the people and systems that will use the metadata, not just the creators.

Audience considerations:

- researcher users may need granular subject, temporal, and provenance data;
- general public users benefit from clear titles, descriptions, and rights;
- machine harvesters need standard structure and controlled vocabularies;
- internal collection managers need administrative and technical elements;
- future unknown users benefit from richness and documentation.

Map the use cases to required elements, then choose a schema that supports them. A schema that serves the creator but not the user has failed its purpose.

### Evaluate Schema Maintenance And Governance

Schemas are maintained by different bodies with different governance, update cycles, and community support. A schema with active maintenance is safer for long-term projects.

Governance factors:

- is the schema maintained by a stable organization or community;
- how often is it updated, and are updates backward-compatible;
- is there active documentation, training, and community support;
- are there tools, validators, and templates for the schema;
- is the schema at risk of deprecation or abandonment.

A schema that is technically perfect but unmaintained becomes a liability. Prefer schemas with active governance and community.

### Decide Between Simple, Complex, And Modular Approaches

Schema choice is not binary. Projects can use a simple base schema with extensions, a complex schema with selective use, or a modular combination.

Approaches:

- simple schema, Dublin Core, for broad interoperability and low burden;
- complex schema, MODS or DDI, for rich domain-specific description;
- modular approach, a base schema plus a specialized module, Dublin Core plus PREMIS for preservation;
- application profile, a schema constrained and extended for local use.

The choice depends on the balance of interoperability, richness, and sustainability. An application profile often gives the best fit, constraining a standard schema to local needs while preserving interoperability.

### Account For System Support

The repository or content management system must support the chosen schema. A schema the system cannot store, display, or harvest is useless regardless of its fit.

System checks:

- does the repository natively support the schema or require customization;
- can the system validate against the schema;
- does the system expose the schema for harvesting and export;
- can the system handle the schema's complexity, repeatable fields, hierarchies;
- will a future system migration preserve the schema.

Verify system support before committing. A schema mismatch with the repository is a costly discovery after metadata creation has begun.

### Document The Selection Rationale

Schema selection involves tradeoffs that future staff and partners need to understand. Document the decision.

Document:

- the candidate schemas considered and why they were rejected;
- the object types, audiences, and systems that drove the choice;
- the richness and sustainability balance struck;
- the crosswalks and interoperability plan;
- any local extensions or application profile decisions.

Documentation turns a one-time decision into institutional knowledge that supports future revisions and migrations.

## Common Traps

### Defaulting To A Familiar Schema Without Analysis

Using Dublin Core or MODS because it is familiar may miss a better domain fit. Analyze object types and audiences first.

### Choosing Maximum Richness Without Sustainability

A schema too rich for the project's capacity produces incomplete metadata. Balance richness against consistent population.

### Ignoring Interoperability Until Harvest Fails

A schema that does not crosswalk to target aggregators limits reach. Plan crosswalks at selection time.

### Local-Only Elements That Map To Nothing

Custom elements that no target recognizes isolate the collection. Use standard elements or document crosswalks.

### Overlooking Schema Governance And Maintenance

An unmaintained schema becomes a liability. Prefer active governance and community.

### System-Schema Mismatch Discovered Late

A schema the repository cannot support is useless. Verify system support before metadata creation.

### Undocumented Selection Rationale

Without documentation, future staff cannot understand or revise the choice. Record the tradeoffs and decision.

### Treating Schema Choice As Permanent

Schemas and needs change. Build in a review cycle and migration awareness.

## Self-Check

- Is the schema matched to the collection's object types and domain rather than chosen by familiarity?
- Is the richness balanced against the project's capacity to populate elements consistently?
- Are interoperability targets, aggregators, and crosswalks planned from the start?
- Does the schema serve the intended audiences and use cases, not just the metadata creators?
- Is the schema maintained by a stable body with active governance and community support?
- Is the approach, simple, complex, modular, or application profile, chosen deliberately?
- Does the repository or system natively support the schema's storage, validation, and harvesting?
- Is the selection rationale documented, including candidates considered, tradeoffs, and extensions?
