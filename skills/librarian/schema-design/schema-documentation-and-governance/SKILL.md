---
name: schema_documentation_and_governance.md
description: Use when the agent is documenting a metadata schema or application profile, writing element definitions and usage rules, establishing schema governance and versioning, managing schema change requests, or maintaining schema documentation for creators, developers, and consumers.
---

# Schema Documentation And Governance

A metadata schema is only as good as the documentation and governance that surround it. A schema without clear documentation is interpreted differently by every creator, developer, and consumer, and the data drifts into inconsistency no matter how well the elements were designed. A schema without governance cannot evolve safely: changes break downstream consumers, local variations accumulate unchecked, and the schema stagnates or fragments. Documentation and governance are not administrative overhead; they are what turn a set of element names into a reliable, maintainable standard that an institution and its partners can depend on for years. Documentation must define every element precisely, provide examples and rules, and stay current with the schema as it changes. Governance must establish who decides changes, how changes are proposed and reviewed, how versions are managed, and how backward compatibility is protected. Together they make the schema a living, trustworthy asset rather than a fragile artifact.

Use this skill when documenting a schema or application profile, establishing schema governance and versioning, managing change requests, or maintaining schema documentation for multiple audiences. The goal is to prevent the agent from leaving schemas undocumented, allowing uncontrolled local variation, making breaking changes without notice, or letting documentation drift from the actual schema.

## Core Rules

### Document Every Element Comprehensively

Each element needs documentation complete enough that any creator, developer, or consumer can use it correctly without asking the designer.

Per-element documentation:

- a clear, precise definition of what the element captures;
- what the element does not capture, to bound its scope;
- the obligation level, mandatory, recommended, optional, conditional;
- the data type, format, and controlled vocabulary or authority source;
- cardinality, single or repeatable, with ordering rules;
- relationships to other elements, refinements, overlaps, dependencies;
- examples of correct use and non-examples of common mistakes;
- the element's provenance, which standard or local extension it comes from.

Comprehensive documentation is the primary tool for consistency. Invest in it as a first-class deliverable, not an afterthought.

### Write Documentation For Multiple Audiences

Different audiences need the schema documented differently. One document rarely serves all.

Audience-specific documentation:

- for metadata creators, focus on what to enter, with examples and rules;
- for developers, focus on structure, data types, validation rules, and APIs;
- for consumers and aggregators, focus on semantics, vocabularies, and crosswalks;
- for governance bodies, focus on change history, rationale, and open issues.

Provide entry points for each audience. A creator confronted with developer-focused documentation cannot do their work; a developer given only creator guidance cannot build systems.

### Establish Clear Schema Governance

Governance defines who owns the schema, who can propose changes, and how decisions are made. Without it, the schema drifts through uncoordinated local edits.

Governance structure:

- identify the schema owner or stewardship body responsible for the schema;
- define who can propose changes and how proposals are submitted;
- establish a review process with defined criteria and timelines;
- document decision-making authority, who approves, and on what basis;
- record decisions and their rationale in a change log.

Governance does not require a large committee; it requires clarity about who decides and how. Ambiguous ownership produces paralysis or uncontrolled drift.

### Manage Schema Versions Deliberately

Schemas evolve. Versioning makes evolution safe for consumers by making changes visible and traceable.

Versioning practice:

- assign a version identifier to each released schema or profile;
- record what changed between versions, with rationale;
- distinguish breaking from non-breaking changes clearly;
- provide migration guidance for consumers affected by breaking changes;
- maintain access to prior versions so legacy data remains interpretable;
- give advance notice of breaking changes before they take effect.

Unversioned schemas force consumers to guess what changed and break silently. Versioning is a contract with consumers.

### Protect Backward Compatibility

Breaking changes, removing elements, changing semantics, altering cardinality, invalidate existing data and break consumers. They should be rare and deliberate.

Compatibility protection:

- prefer adding elements over removing or renaming them;
- when semantics must change, create a new element rather than redefining an old one;
- widen cardinality, single to repeatable, cautiously and with notice;
- deprecate rather than delete elements, marking them while retaining support;
- document the migration path for any breaking change.

Treat existing data and consumers as constraints on change. A schema that breaks its consumers on every revision will lose them.

### Control Local Variation

Local extensions and variations are sometimes necessary, but uncontrolled variation fragments the standard. Govern how local variation is permitted.

Variation control:

- define what local extension is permitted and what must conform to the standard;
- require local extensions to be namespaced and documented;
- require local extensions to crosswalk to standard elements;
- review local variations periodically for promotion to the standard or removal;
- record local variations in a registry visible to all participants.

Uncontrolled local variation produces schemas that look shared but behave locally, defeating interoperability.

### Keep Documentation Synchronized With The Schema

Documentation that drifts from the actual schema is worse than no documentation, because it actively misleads. Synchronization must be maintained.

Synchronization practice:

- update documentation as part of every schema change, not separately;
- treat documentation and schema as a single deliverable;
- review documentation against the schema on a schedule;
- assign responsibility for documentation maintenance;
- version documentation in step with schema versions.

A change to the schema that is not reflected in documentation creates a gap that grows until the documentation is untrustworthy.

### Maintain A Change Log And Issue Tracker

Schema evolution produces a history that future maintainers and consumers need to understand. A change log and issue tracker preserve that history.

Maintain:

- a change log recording every version, what changed, why, and when;
- an issue tracker for proposed changes, bugs, and open questions;
- decisions and their rationale, including rejected proposals;
- links between issues, decisions, and versions.

This record turns schema evolution from opaque churn into a transparent, learnable history.

## Common Traps

### Undocumented Or Vaguely Documented Elements

Elements without precise definitions are populated inconsistently. Document every element comprehensively.

### One-Size-Fits-All Documentation

A single document rarely serves creators, developers, and consumers. Provide audience-specific entry points.

### Ambiguous Schema Ownership

Without clear ownership, schemas drift or stall. Establish who decides and how.

### Unversioned Schema Changes

Unversioned changes break consumers silently. Version deliberately with change logs.

### Breaking Changes Without Migration Guidance

Removing or redefining elements invalidates data. Prefer additions, deprecate rather than delete, and document migration.

### Uncontrolled Local Variation

Unregistered local extensions fragment the standard. Govern and register variation.

### Documentation Drifting From The Schema

Out-of-sync documentation misleads. Update documentation with every change.

### No Change Log Or Issue History

Opaque evolution cannot be learned from. Maintain a transparent change log and issue tracker.

## Self-Check

- Is every element documented comprehensively, including definition, obligation, type, cardinality, relationships, and examples?
- Is documentation provided for multiple audiences, creators, developers, consumers, governance?
- Is schema governance clear, with identified owners, proposal process, review criteria, and decision authority?
- Are schema versions managed with identifiers, change logs, and migration guidance?
- Is backward compatibility protected by preferring additions, deprecating rather than deleting, and documenting breaking changes?
- Is local variation controlled through namespacing, documentation, crosswalks, and a visible registry?
- Is documentation synchronized with the schema, updated with every change and reviewed on schedule?
- Is a change log and issue tracker maintained to preserve schema evolution history?
