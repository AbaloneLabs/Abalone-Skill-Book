---
name: linked_data_publishing_and_consumption.md
description: Use when the agent is publishing library or cultural heritage data as linked data, choosing serialization formats, exposing APIs or SPARQL endpoints, configuring content negotiation, harvesting or consuming external linked data, or integrating linked data into discovery and delivery systems.
---

# Linked Data Publishing And Consumption

Modeling data as linked data is only half the work. The data must also be published so that machines and humans can find, resolve, and consume it, and institutions must consume external linked data to enrich their own discovery. Publishing linked data involves choices about serialization, access methods, content negotiation, and API design that determine whether the data is actually usable by the broader ecosystem. Consumption involves harvesting, caching, and integrating external data whose quality, stability, and licensing vary. Both sides carry risks: published data that no system can reliably consume, endpoints that break under load or change without notice, external data consumed without verification or rights awareness, and linked-data systems that never connect to the discovery interfaces real users actually use. Good publishing makes data robustly accessible with clear documentation and stable contracts; good consumption treats external data as a managed dependency with verification, caching, and integration into user-facing discovery.

Use this skill when publishing linked data, choosing serializations and access methods, configuring content negotiation or APIs, consuming external linked data, or integrating linked data into discovery systems. The goal is to prevent the agent from publishing data that cannot be consumed reliably, consuming external data without verification or rights checks, or building linked-data systems disconnected from actual user discovery.

## Core Rules

### Provide Multiple Serializations For Different Consumers

Different consumers need linked data in different formats. Providing only one limits the audience.

Serializations to provide:

- RDF/XML for legacy library and XML-based consumers;
- Turtle for human-readable and developer-friendly consumption;
- JSON-LD for web applications and modern consumers;
- N-Triples for bulk processing and loading.

Use content negotiation so the same URI serves the appropriate serialization based on the client's Accept header. Document which serializations are available so consumers know what to request.

### Ensure URIs Resolve To Useful Data

A linked-data URI must resolve to meaningful data about the entity, not just return a 404 or an empty page. Resolution is the basic contract of linked data.

Resolution requirements:

- each entity URI returns a description of the entity, its type, label, and key relationships;
- the description includes links to related entities and external authorities;
- the data is current and consistent with the source system;
- resolution is reliable, with acceptable uptime and performance;
- broken or redirected URIs are handled gracefully.

A URI that does not resolve is worse than no URI, because it breaks every graph that links to it. Resolution is a publishing obligation.

### Document The API And Access Methods

Consumers need to know how to access the data. Undocumented endpoints are effectively unusable.

Document:

- the base URIs and identifier patterns;
- the available serializations and how to request them;
- any query endpoints, SPARQL or REST API, and their capabilities;
- rate limits, authentication, and acceptable use;
- the ontologies and vocabularies used in the data;
- the update frequency and versioning.

Publish documentation alongside the data. A linked-data service without documentation forces consumers to reverse-engineer it, which most will not do.

### Choose Access Methods Matched To Use Cases

Different consumers need different access methods. A single approach rarely serves all.

Access methods:

- dereferenceable URIs for entity-by-entity lookup;
- SPARQL endpoint for flexible querying by sophisticated consumers;
- bulk download or data dumps for large-scale harvesting and indexing;
- REST or GraphQL API for application developers;
- OAI-PMH or similar harvesting for aggregation.

Provide the methods your consumers actually need. A SPARQL endpoint alone excludes consumers who need bulk data; bulk dumps alone exclude those who need targeted queries.

### Consume External Data As A Managed Dependency

Consuming external linked data enriches local discovery but introduces dependencies. Treat external data as a managed dependency, not a fire-and-forget link.

Management:

- verify the quality and accuracy of consumed data before integration;
- cache or snapshot external data so discovery does not break if the source changes;
- monitor the source for updates, schema changes, and outages;
- document the sources consumed and their roles;
- have a fallback if a source degrades or disappears.

Depending on a live external endpoint for core discovery creates fragility. Cache what matters and document the dependency.

### Verify And Attribute Consumed Data

External linked data varies in quality and may carry licensing or attribution requirements. Consuming it without verification or rights awareness creates risk.

Verification and rights:

- assess the authority and reliability of each source;
- verify that consumed data accurately describes the entities it is applied to;
- check the license or terms of use for the external data;
- provide required attribution where the license demands it;
- record the source and date of consumed data for provenance.

Consuming unverified or unattributed external data can propagate errors and violate licenses. Treat consumption with the same rigor as creation.

### Integrate Linked Data Into User-Facing Discovery

Linked data that lives only in a SPARQL endpoint or a triplestore, disconnected from the discovery interface users actually use, delivers little value. Integration is where linked data pays off.

Integration:

- surface linked-data relationships in the discovery layer, related works, authors, subjects;
- use external identifiers to enrich entity pages with contextual data;
- enable faceted navigation driven by linked-data properties;
- use same-as links to connect local entities to broader web resources;
- measure whether integration improves user discovery outcomes.

A linked-data system that users never see is an infrastructure project, not a service. Connect it to discovery and evaluate the benefit.

### Maintain Publishing Stability And Versioning

Consumers depend on stability. Changing URIs, schemas, or endpoints without notice breaks downstream systems.

Stability practice:

- commit to URI persistence and avoid changing identifier patterns;
- version the data model and document changes;
- provide advance notice of breaking changes;
- maintain backward compatibility where feasible;
- monitor endpoint health and performance.

A linked-data service is a contract with consumers. Honor the contract through stable URIs, documented changes, and reliable access.

## Common Traps

### Single Serialization Limiting Consumers

Providing only one format excludes consumers who need others. Use content negotiation and multiple serializations.

### URIs That Do Not Resolve

A non-resolving URI breaks the graph. Resolution is a publishing obligation.

### Undocumented Endpoints

Consumers cannot use what they cannot understand. Document URIs, serializations, APIs, and vocabularies.

### SPARQL-Only Or Bulk-Only Access

Single access methods exclude use cases. Provide methods matched to consumer needs.

### Consuming External Data Without Caching

Live dependencies on external endpoints create fragility. Cache what matters.

### Unverified Or Unattributed Consumption

Consuming without verification or rights checks propagates errors and risk. Verify and attribute.

### Linked Data Disconnected From User Discovery

Infrastructure users never see delivers no service value. Integrate into the discovery layer.

### Breaking Changes Without Notice

Changing URIs or schemas silently breaks consumers. Commit to stability and versioning.

## Self-Check

- Are multiple serializations provided with content negotiation for different consumers?
- Does every published URI resolve to a meaningful, current description of the entity?
- Is the API fully documented, including URIs, serializations, endpoints, limits, and vocabularies?
- Are access methods, dereferenceable URIs, SPARQL, bulk download, REST, matched to consumer use cases?
- Is consumed external data cached, monitored, and documented as a managed dependency?
- Is consumed data verified for accuracy and attributed per its license or terms?
- Is linked data integrated into the user-facing discovery layer with measurable benefit?
- Are publishing stability, URI persistence, and versioning maintained with advance notice of changes?
