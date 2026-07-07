---
name: metadata_for_datasets_and_research_outputs.md
description: Use when the agent is creating metadata for research datasets, software, code, methods, and other non-traditional research outputs, applying discipline-specific standards, supporting citation and reuse, documenting provenance and versioning, or enabling discovery in data repositories and aggregators.
---

# Metadata For Datasets And Research Outputs

Research outputs are no longer just publications. Datasets, software, code, methods, protocols, models, and other non-traditional outputs are now first-class products of research that must be discovered, cited, reused, and trusted. Describing these outputs requires a different metadata posture from describing books or articles. A dataset is not a static text; it has variables, file formats, collection methods, spatial and temporal coverage, processing provenance, and version history that determine whether it can be understood or reused. Software has dependencies, licenses, and releases. Methods and protocols evolve. The metadata must capture enough of this structure for another researcher to find the output, understand what it is and is not, cite it properly, and decide whether it is fit for their reuse. Treating dataset metadata like article metadata produces records too thin to support reuse or verification, undermining the very goals of open science and data sharing that the metadata is meant to serve.

Use this skill when creating metadata for datasets, software, code, methods, and other research outputs, when applying discipline-specific metadata standards, and when supporting citation, reuse, and aggregation. The goal is to prevent the agent from applying publication-centric metadata to research outputs, from omitting the elements that enable reuse (methods, provenance, variables, coverage), from neglecting persistent identifiers and versioning, or from producing metadata too thin to support verification.

## Core Rules

### Understand What Makes Dataset Metadata Different

Dataset metadata is not article metadata with extra fields. Understand the distinctive requirements.

Dataset metadata must capture:

- what the dataset contains (variables, observations, file structure);
- how, when, and where it was created (methods, instruments, spatial and temporal coverage);
- who created it and who may be contacted;
- how it has been processed and versioned;
- under what terms it can be accessed and reused;
- how it should be cited.

Thin metadata that captures only title, author, and date is insufficient for reuse or verification.

### Use Discipline-Specific Metadata Standards

Different disciplines have established metadata standards that capture domain-specific structure. Use the appropriate standard.

Common standards include:

- domain-specific schemas for social science, earth science, life science, humanities, and others;
- general-purpose schemas for cross-disciplinary repositories;
- discipline-specific controlled vocabularies for variables, methods, and instruments.

Choosing the right standard ensures the metadata captures what researchers in the field expect and need.

### Document Methods, Provenance, And Processing

A dataset without documentation of how it was created cannot be evaluated or reused. Document methods and provenance.

Document:

- the collection or generation methods and instruments;
- the processing, cleaning, and transformation steps applied;
- any derived variables and their calculation;
- the software and code used in processing;
- decisions and exclusions that shaped the final dataset.

Provenance documentation is what separates a reusable dataset from an opaque file.

### Capture Spatial And Temporal Coverage

Many datasets are anchored in space and time. Capture coverage accurately.

Capture:

- the geographic extent (coordinates, place names, regions);
- the temporal extent (date range, collection period);
- the spatial and temporal resolution;
- any taxonomic, demographic, or other scope limitations.

Coverage metadata enables discovery by researchers studying that place, period, or population.

### Describe Variables, File Formats, And Structure

The internal structure of a dataset determines whether it can be used. Describe it clearly.

Describe:

- the variables, their names, types, and units;
- the file formats and their openness and interoperability;
- the structure of files (rows, columns, headers, relationships);
- any data dictionaries or codebooks;
- relationships between files in a multi-file dataset.

Without structural description, users cannot interpret the data correctly.

### Assign Persistent Identifiers And Support Citation

Datasets and research outputs must be citable to be credited and tracked. Assign persistent identifiers.

Assign and support:

- persistent identifiers such as DOIs for datasets and software;
- accurate citation metadata (authors, title, publisher, version, identifier);
- contributor roles following appropriate taxonomies;
- links to related publications, software, and other outputs;
- citation in a form that users can copy and reuse.

Citable outputs get used and credited; uncitable ones are invisible.

### Manage Versioning And Relationships

Research outputs evolve. Versioning and relationship metadata track this evolution.

Manage:

- version numbers or dates for updated datasets and software;
- relationships to earlier and later versions;
- relationships to derived datasets, publications, and software;
- clear distinction between versions so users cite the correct one;
- release notes or change logs where appropriate.

Versioning prevents confusion when outputs change and ensures citations remain valid.

### Record Access, Rights, And Licensing Terms

Access and reuse terms determine whether and how outputs can be used. Record them clearly.

Record:

- the access status (open, restricted, embargoed, controlled);
- the license under which the data or software is released;
- any conditions on reuse (attribution, share-alike, non-commercial);
- contact and application processes for restricted data;
- ethical and legal restrictions (human subjects, confidentiality, sensitive species locations).

Clear terms enable lawful reuse and protect sensitive data. (See the related skill on rights and licensing metadata.)

### Support Discovery In Aggregators

Dataset metadata should support discovery beyond the home repository, in aggregators and indexes.

Support aggregation by:

- mapping to crosswalk schemas for major aggregators;
- including standardized subject terms and keywords;
- providing complete, well-structured metadata records;
- exposing metadata through protocols and APIs;
- registering outputs in discipline and general indexes.

Aggregation multiplies the discoverability and impact of research outputs.

### Coordinate With Data Management Plans And Funder Requirements

Many datasets are created under funder and institutional data management requirements. Coordinate metadata with these.

Coordinate:

- with the data management plan's commitments on description and sharing;
- with funder requirements for metadata standards and repositories;
- with institutional repository policies;
- with retention and preservation requirements.

Alignment ensures the metadata meets the obligations under which the research was produced.

## Common Traps

### Applying Publication-Centric Metadata To Datasets

Datasets need methods, variables, coverage, and provenance, not just title, author, and date.

### Omitting Methods And Provenance

A dataset without method and processing documentation cannot be evaluated or reused.

### Skipping Variables, Formats, And Structure

Without structural description, users cannot interpret the data correctly.

### Neglecting Spatial And Temporal Coverage

Coverage metadata enables discovery by place, period, and population. Do not omit it.

### Forgetting Persistent Identifiers And Citation

Uncitable outputs are invisible. Assign DOIs and provide citation metadata.

### Ignoring Versioning And Relationships

Outputs evolve. Track versions and relationships so citations remain valid.

### Vague Or Missing Access And Licensing Terms

Unclear terms block reuse or invite misuse. Record access status, licenses, and conditions.

### Failing To Support Aggregator Discovery

Metadata trapped in one repository limits impact. Map to aggregator schemas and expose metadata.

## Self-Check

- Did you capture what the dataset contains (variables, observations, file structure)?
- Did you use the appropriate discipline-specific metadata standard and controlled vocabularies?
- Did you document methods, instruments, processing, and provenance?
- Did you capture spatial and temporal coverage and resolution?
- Did you describe variables, file formats, structure, and provide a data dictionary?
- Did you assign persistent identifiers and provide accurate citation metadata?
- Did you manage versioning and relationships to earlier, later, and related outputs?
- Did you record access status, licenses, reuse conditions, and ethical restrictions?
- Did you map metadata for aggregator discovery and expose it through appropriate protocols?
- Did you coordinate metadata with data management plans and funder requirements?
- Could another researcher find, understand, evaluate, cite, and reuse this output from the metadata alone?
