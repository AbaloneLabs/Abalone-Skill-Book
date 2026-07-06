---
name: code_data_and_document_versioning.md
description: Use when the agent is deciding how to version code, data, and documents together in a research project, handling large or sensitive data outside git, synchronizing document and code versions, or recording which versions produced a published result.
---

# Code, Data, And Document Versioning

A research project produces three kinds of artifacts that each change over time, code, data, and documents. Each needs versioning, but they version differently. Code changes in small, reviewable steps and fits naturally in a version control system. Data can be large, sensitive, or append-only, and often belongs outside a code repository. Documents, such as manuscripts and protocols, change through drafts and reviews. When these three are versioned inconsistently or not at all, three harms follow. A published result cannot be tied to the specific code, data, and document versions that produced it. The manuscript drifts from the analysis, so the text describes a result the code no longer generates. And the team loses the ability to reconstruct what was true at the time of submission. Coordinating versioning across these artifact types is what makes a project auditable.

The agent should use this skill when setting up versioning across a project, deciding where each artifact lives, linking document versions to code and data versions, or preparing a submission package. The goal is to keep the agent from versioning code carefully while leaving data and documents unversioned, when a result is only reproducible if all three are pinned together.

## Core Rules

### Version Each Artifact Type Appropriately

Code, data, and documents have different change patterns and should be versioned with tools that fit.

- Code, use a version control system with frequent, small, reviewed commits.
- Data, use a data repository, versioned store, or dated snapshots, with hashes or version identifiers.
- Documents, use version control for drafts, or a document system with tracked changes and named versions.

Do not force all three into one tool. Putting large data in a code repository bloats history; putting manuscripts only in email loses the draft lineage. Match the tool to the artifact.

### Keep Raw Data Immutable And Versioned Externally

Raw data should be immutable and versioned outside the code repository. The code repository references which version of the data it expects.

- Store raw data in a dedicated location with version identifiers or timestamps.
- Record the version, hash, or identifier of the data used in each analysis.
- Never edit raw data in place; create new derived versions through the pipeline.
- Treat the raw data version as an input to the workflow, just like the code version.

This separation keeps the code repository lightweight while preserving the ability to tie any result to the exact data that produced it.

### Link Code, Data, And Document Versions Explicitly

A published result corresponds to a specific combination of code version, data version, and document version. Make that combination explicit.

- When tagging a result, record the code commit, the data version, and the document version together.
- Use a manifest or provenance file that lists all three.
- Ensure the manuscript references the analysis version that generated its numbers.

Without this linkage, the three artifact streams drift independently, and no one can reconstruct the submission state. The linkage is the audit trail.

### Version Manuscripts And Protocols, Not Just Code

Documents are part of the reproducible record. Version them.

- Keep manuscripts under version control or a system that preserves draft history.
- Tag the manuscript version submitted to a journal, so the submitted state is recoverable.
- Version protocols separately, because they change independently of the manuscript.
- Record which protocol version governed data collection for each portion of the data.

A manuscript that exists only as the latest file in a shared drive has no recoverable history. Versioning documents preserves the reasoning behind wording and structure changes.

### Use Semantic Or Stage-Based Version Labels

Version labels should be meaningful, not arbitrary. Choose a labeling scheme and apply it consistently.

- Semantic versioning for code, major, minor, patch, when changes affect results.
- Stage-based labels for analyses, such as preregistration, pilot, main-analysis, revision-1.
- Dated snapshots for data that updates periodically.
- Submission-stage tags for documents, such as submitted, revised, accepted.

Meaningful labels let the team and readers understand what a version represents without inspecting the diff.

### Synchronize Versions At Milestones

At key milestones, synchronize the versions of all three artifact types and record the synchronized state.

- At preregistration, freeze code, data expectation, and protocol together.
- At analysis completion, tag code, record data version, and snapshot the results document.
- At submission, tag the manuscript, the code, and the data version together.
- At publication, create a final release combining all three.

These synchronized checkpoints are what a reviewer or replicator needs. They turn a continuously changing project into a series of recoverable states.

### Handle Derived Data And Outputs Deliberately

Derived data and outputs, such as cleaned datasets, figures, and tables, are produced by the pipeline. Decide how to version them.

- Regenerate derived data from the pipeline rather than versioning it manually.
- If derived data are large, store them with a hash and regenerate on demand.
- Version outputs by the code and data versions that produced them, not by independent edits.
- Avoid hand-editing generated outputs, because edits are lost on regeneration.

Derived artifacts should be a function of their inputs. Versioning them as if they were primary sources creates inconsistency.

### Record Provenance For Every Published Number

Every number, figure, or table in a published result should have provenance, the code version, data version, and command that produced it.

- Maintain a mapping from each output to its generating script and inputs.
- Include this mapping in the submission package or supplementary material.
- Update the mapping whenever a result is regenerated.

A reader who asks how a number was produced should get a precise answer, not a guess. Provenance mapping is how that answer is delivered.

### Make The Submission Package Self-Contained

At submission, assemble a package that contains or references everything needed to reproduce the results.

- The tagged code version.
- The data version or a clear path to obtain it.
- The environment specification.
- The manuscript and protocol versions.
- The provenance mapping for each result.
- Instructions for rebuilding.

A self-contained package is what makes the work auditable. Scattered artifacts across personal machines and unnamed folders are not a submission package; they are a future integrity problem.

## Common Traps

### Versioning Code But Not Data Or Documents

Code-only versioning leaves the result untraceable to its data and manuscript. Version all three.

### Putting Large Data In The Code Repository

Large data bloat history and make the repository unusable. Store data externally and reference it.

### Editing Raw Data In Place

In-place edits destroy traceability. Keep raw data immutable.

### Letting The Manuscript Drift From The Analysis

When the text describes results the code no longer produces, the paper misrepresents the work. Synchronize at milestones.

### Using Arbitrary Version Labels

Labels like final or final2 carry no meaning. Use semantic or stage-based labels.

### Hand-Editing Generated Outputs

Manual edits to generated figures or tables are lost on regeneration. Fix the pipeline, not the output.

### Skipping Provenance Mapping

Without provenance, no one can say how a number was produced. Map every published result to its inputs.

### Treating Submission As A Single File

Submission is a package of coordinated versions. Assemble it deliberately.

## Self-Check

- Are code, data, and documents each versioned with tools appropriate to their type?
- Is raw data kept immutable and versioned externally, referenced by the code?
- Are code, data, and document versions linked explicitly at each result and milestone?
- Are manuscripts and protocols versioned, with submission-stage states recoverable?
- Are version labels semantic or stage-based and applied consistently?
- Are versions synchronized at milestones such as preregistration, analysis, submission, and publication?
- Are derived data and outputs treated as functions of their inputs rather than independent sources?
- Does every published number have provenance mapping to its code, data, and command?
- Is the submission package self-contained, with all versions and instructions included?
- For complex or multi-artifact projects, has a research software engineer, data manager, or reproducibility advisor reviewed the versioning strategy before submission?
