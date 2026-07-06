---
name: linked_data_modeling_and_ontologies.md
description: Use when the agent is modeling library or cultural heritage data as linked data, choosing or extending an ontology such as BIBFRAME, RDA Registry, or schema.org, defining classes and properties, or mapping entity relationships into an RDF graph structure.
---

# Linked Data Modeling And Ontologies

Linked data reframes cataloging and metadata as a graph of entities and relationships rather than a set of records. In this model, a work, a person, a place, a concept, and an item are each distinct nodes, connected by typed links that machines can traverse. This is a fundamentally different way of representing knowledge than the flat record, and it demands different modeling judgment. The cataloger or metadata architect must decide what counts as an entity, which ontology classes and properties to use, how to connect to external identifiers, and how to structure relationships so they are both machine-traversable and meaningful to humans. The dominant failure mode is treating linked data as a relabeling exercise, pouring record-shaped data into RDF triples without rethinking the model. That produces graph data that cannot be traversed usefully and that fails to deliver the discovery benefits linked data promises. Good linked-data modeling thinks in entities and relationships, reuses established ontologies rather than reinventing them, links outward to the broader web of data, and documents the model so it can be consumed and extended.

Use this skill when modeling data as linked data, choosing or extending an ontology, defining classes and properties, or mapping a record-based model into an entity-relationship graph. The goal is to prevent the agent from record-to-triple transliteration, reinventing ontology terms that already exist, creating isolated graphs with no external links, or producing models that machines cannot traverse meaningfully.

## Core Rules

### Model In Entities And Relationships, Not Records

The core shift in linked data is from records to entities. A bibliographic record bundles a work, its author, its subject, and its manifestation into one structure. Linked data separates them into distinct, linkable entities.

Modeling practice:

- identify the distinct entities, work, expression, manifestation, item, person, corporate body, place, concept, event;
- assign each entity a persistent identifier and a type from an ontology;
- express the relationships between entities as typed properties, authored-by, has-subject, is-translation-of;
- avoid embedding entity data as strings inside another entity's description;
- let relationships carry meaning that a flat record cannot express.

A model that keeps author names as strings inside a work description has not adopted linked data; it has serialized a record as RDF. Entities must be first-class, linkable nodes.

### Reuse Established Ontologies Before Defining New Terms

The linked-data ecosystem depends on shared vocabularies. Defining new classes and properties when established ones exist fragments the ecosystem and reduces interoperability.

Ontologies to reuse:

- BIBFRAME, for library bibliographic data;
- RDA Registry and RDA unconstrained properties, for element-level description;
- schema.org, for broad web discovery;
- FOAF, for people and organizations;
- SKOS, for concept schemes and thesauri;
- Dublin Core Terms, for general descriptive elements;
- PREMIS, for preservation events and objects;
- PROV-O, for provenance.

Search for existing terms before creating new ones. Extend an ontology only when no established term fits, and document the extension. Reinventing "hasAuthor" when FOAF or schema.org provides one isolates your data.

### Choose The Right Ontology For The Audience And Use Case

Different ontologies serve different audiences and systems. Choosing the wrong one limits consumption.

Ontology fit:

- BIBFRAME for library-centric discovery and cooperative programs;
- schema.org for search engine indexing and general web discovery;
- RDA Registry for detailed element-level cataloging data;
- SKOS for subject vocabularies and concept schemes;
- domain ontologies, Darwin Core, DDI in RDF, for specialized data.

Often a model combines ontologies, using schema.org for web discovery and BIBFRAME for library detail. Document which ontologies are used and why, so consumers know what to expect.

### Make Entities Linkable With Persistent Identifiers

A linked-data entity is only as useful as its identifier. The identifier must be persistent, resolvable, and stable over time.

Identifier practice:

- assign each entity an HTTP URI as its identifier;
- ensure the URI resolves to useful data about the entity;
- use established identifiers where they exist, VIAF, Wikidata, ORCID, LCNAF;
- avoid identifiers tied to transient systems or internal IDs;
- document the identifier scheme and its persistence guarantees.

An entity with an internal database ID as its identifier is not linked data; it is a record with a label. Use URIs that the web can resolve.

### Link Outward To The Broader Web Of Data

Linked data's value comes from connections to external data. An isolated graph of internal entities delivers little more than a local database.

Outward linking:

- link persons to VIAF, Wikidata, ORCID, or LCNAF;
- link places to GeoNames or TGN;
- link concepts to LCSH, AAT, or Wikidata;
- link works to authority files and related works in other institutions;
- use same-as and exact-match to connect your entities to external equivalents.

Each outward link multiplies the entity's discoverability and contextual richness. A graph that links only to itself cannot participate in the linked-data vision.

### Structure Relationships For Traversal

Relationships in linked data must be traversable by machines. This means using established properties, respecting domain and range, and avoiding ambiguous or untyped links.

Traversal practice:

- use ontology properties with clear semantics, not generic "related";
- respect the domain and range of properties, author links a Work to a Person, not to a string;
- model inverse relationships where useful, authored-by and author-of;
- avoid dangling links that point to nothing or to broken URIs;
- keep the graph consistent so a traversal yields meaningful paths.

A model with untyped or misused relationships cannot be traversed usefully. The value is in the typed, traversable connections.

### Balance Expressiveness Against Practical Consumption

Linked data can model extraordinary detail, but every additional class and property increases complexity for consumers. Model expressively where it serves use, pragmatically where it does not.

Balance:

- model core entities and relationships fully;
- use simpler properties where high detail adds burden without benefit;
- avoid deep ontological modeling that no consumer will traverse;
- provide common access patterns, labels, types, key relationships, for every entity;
- document which parts of the model are core versus extended.

A model so expressive that no system can consume it has failed its purpose. Prioritize the relationships and properties that real consumers will use.

### Document The Model For Consumers And Maintainers

A linked-data model must be documented so consumers can use it and maintainers can extend it.

Document:

- the ontologies used and their roles;
- the classes and properties, including any local extensions;
- the identifier schemes and persistence approach;
- the external vocabularies and authorities linked to;
- the serialization formats provided;
- the version and revision history.

Undocumented models are unconsumable. Documentation is part of the model, not an add-on.

## Common Traps

### Record-To-Triple Transliteration

Pouring record fields into RDF without rethinking the model produces untraversable graph data. Model in entities and relationships.

### Reinventing Ontology Terms

Creating new classes and properties when established ones exist fragments the ecosystem. Reuse first, extend only when necessary.

### Isolated Graphs With No External Links

A graph linking only to itself delivers no linked-data benefit. Link outward to VIAF, Wikidata, GeoNames, and authorities.

### Non-Persistent Or Internal Identifiers

Entities without resolvable persistent URIs are not truly linked data. Use HTTP URIs with stability guarantees.

### Untyped Or Misused Relationships

Generic or out-of-domain links cannot be traversed meaningfully. Use ontology properties with clear semantics.

### Over-Expressive Models No System Consumes

Modeling depth no consumer uses adds burden without benefit. Prioritize practical access patterns.

### Dangling Or Broken Links

Links to nothing or to dead URIs undermine the graph. Maintain link integrity.

### Undocumented Models

Without documentation, consumers cannot use and maintainers cannot extend the model. Document ontologies, identifiers, and extensions.

## Self-Check

- Is the data modeled as distinct entities and typed relationships rather than as serialized records?
- Are established ontologies reused before defining new classes and properties?
- Is the ontology chosen to match the intended audience and consumption systems?
- Does each entity have a persistent, resolvable HTTP URI as its identifier?
- Does the graph link outward to external authorities and the broader web of data?
- Are relationships typed with clear ontology semantics and traversable domain and range?
- Is the model's expressiveness balanced against what real consumers will traverse?
- Is the model documented for consumers and maintainers, including ontologies, identifiers, extensions, and version history?
