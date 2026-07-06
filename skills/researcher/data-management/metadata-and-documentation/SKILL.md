---
name: metadata_and_documentation.md
description: Use when the agent is documenting a dataset, writing a data dictionary or codebook, choosing metadata standards, recording provenance, or making research data understandable to future users including the original team.
---

# Metadata And Documentation

Data without documentation are a puzzle. Even the team that produced them forgets what fields mean, what codes signify, how raw files became the analyzed dataset, and which version is authoritative. Documentation is what turns stored bytes into usable evidence. When documentation is neglected, three harms follow. The original team misanalyzes its own data after a few months. External users cannot reuse or verify the data, defeating sharing. And errors propagate silently because no one can trace a value back to its origin. Good documentation is not bureaucratic overhead; it is the difference between data that retain value and data that decay into noise.

The agent should use this skill when building a data dictionary, writing a codebook, choosing metadata standards, recording how data were processed, or preparing a dataset for deposit or sharing. The goal is to keep the agent from assuming that the meaning of the data is obvious, when in fact meaning is constructed through documentation and is lost without it.

## Core Rules

### Document For The Stranger, Including Future You

Write documentation as if the reader is a competent stranger who knows the field but not this dataset. This includes the original team members months later, who are effectively strangers to their own earlier decisions.

A useful test is whether someone could pick up the dataset, understand what each field means, run the intended analysis, and identify problems, using only the documentation. If they would need to email the creator, the documentation is incomplete.

### Build A Data Dictionary For Every Dataset

A data dictionary defines the structure and meaning of the data. For each variable or field, capture the following.

- Variable name as it appears in the data.
- Human-readable label.
- Definition of what the variable measures or records.
- Unit of measurement.
- Data type and format.
- Allowed values or range.
- Coding scheme for categorical values.
- Missing data codes and their meanings.
- Source, whether collected, derived, or linked.
- Derivation rule if the variable is computed.

Without a data dictionary, two analysts will code the same variable differently and produce conflicting results. The dictionary is the contract between the data and its users.

### Provide A Codebook For Categorical And Coded Data

Categorical variables need a codebook that maps every code to its meaning. Ambiguous codes are a frequent source of error.

Cover the following.

- Every code and its label.
- The order of categories where meaningful.
- Handling of unknown, refused, not applicable, and missing.
- Any recoding applied during processing.
- Crosswalks to standard classifications where used, such as occupation or disease codes.

A field coded 1 and 2 with no explanation invites misinterpretation. The codebook removes the guesswork.

### Choose And Apply Relevant Metadata Standards

Metadata standards make data discoverable and interoperable. Use a standard relevant to the discipline rather than inventing a private schema.

Examples include the following.

- DataCite or Dublin Core for general citation metadata.
- Domain standards such as DICOM for imaging, MIAME for microarrays, Darwin Core for biodiversity, or DDI for social survey data.
- Schema.org or Frictionless Data for tabular datasets.

Standards help repositories index the data and help users find and cite them. State which standard is used and fill its required fields completely. Partial metadata in a standard is barely better than none.

### Record Provenance From Raw To Analyzed Data

Provenance is the chain that connects the final result to the original measurement. Without it, no one can verify or reproduce the analysis.

Record the following.

- The raw data files and their sources.
- Each processing step, cleaning, transformation, merging, and filtering.
- The software, versions, and code used.
- The decisions made, including handling of outliers and missing data.
- The mapping from raw to derived variables.
- The version of each file used in a given analysis.

Use scripts rather than manual steps where possible, because scripts are self-documenting provenance. Where manual steps are unavoidable, document them in enough detail to repeat.

### Document Known Limitations And Anomalies

Honest documentation includes what is wrong or uncertain about the data. Future users need to know.

- Known data entry errors and how they were handled.
- Variables with high missingness and the pattern.
- Changes in instruments or procedures over time.
- Known anomalies, outliers, or suspicious patterns.
- Subgroups with small or unreliable counts.
- Deviations from the planned protocol.

Hiding problems does not protect the data; it lets downstream users propagate errors. Documented limitations let users handle the data appropriately.

### Version The Data And The Documentation Together

Data and documentation evolve together. A data dictionary that describes version 1 of a dataset misleads users of version 2.

- Version both the data and the documentation.
- Record what changed between versions and why.
- Make clear which version supports which published result.
- Avoid silent overwrites; preserve prior versions.

A result that cannot be tied to a specific data version cannot be reproduced. Versioning closes that gap.

### Make Documentation Discoverable With The Data

Documentation that lives only on a personal drive is lost. Store documentation with the data, in the repository, in the same folder structure, and referenced from the data availability statement.

Include the following alongside the data.

- The data dictionary and codebook.
- The protocol and any deviations.
- Processing scripts and their order.
- A readme explaining how the files fit together.
- Links to related code, publications, and preregistrations.

The goal is a self-contained package that a user can pick up without contacting the creator.

### Write Human-Readable Summaries Alongside Machine-Readable Metadata

Machine-readable metadata serve discovery and interoperability. Human-readable summaries serve understanding. Provide both.

- A short overview of what the dataset contains and why it was collected.
- The structure of the files and how they relate.
- Quick-start guidance for a new user.
- Contact information for questions, while the team is still reachable.

A dataset with rich machine metadata but no human overview is hard to approach. A dataset with only a prose description lacks interoperability. Both matter.

## Common Traps

### Assuming The Meaning Is Obvious

What seems obvious during collection is opaque months later. Document as if explaining to a stranger.

### Skipping The Codebook For Simple Codes

Even binary fields need their codes explained. Assumptions about what 1 and 2 mean cause silent errors.

### Using A Private Metadata Schema

Invented schemas do not interoperate. Use a recognized standard for the discipline.

### Documenting Only The Final Dataset

The raw data and the path to the final dataset are part of provenance. Document the whole chain.

### Overwriting Without Versioning

Silent overwrites destroy reproducibility. Version data and documentation together.

### Hiding Known Problems

Undocumented errors propagate. Honest documentation of limitations protects users.

### Letting Documentation Live Only On A Personal Drive

Documentation separated from the data is effectively lost. Store them together.

### Writing Metadata Only For Machines

Machine metadata without a human overview makes the dataset hard to approach. Provide both.

## Self-Check

- Is the documentation written so a competent stranger, including future team members, could use the dataset without contacting the creator?
- Does a data dictionary define every variable, including label, definition, unit, type, range, coding, missing codes, and source?
- Does a codebook map every categorical code to its meaning, including missing and special codes?
- Is a recognized metadata standard chosen and applied completely?
- Is provenance recorded from raw data through every processing step to the analyzed dataset?
- Are known errors, anomalies, missingness patterns, and protocol deviations documented?
- Are data and documentation versioned together, with changes recorded?
- Is documentation stored with the data and discoverable from the availability statement?
- Are both machine-readable metadata and human-readable summaries provided?
- For complex, shared, or long-lived datasets, has a data librarian, repository curator, or data manager reviewed the documentation before deposit?
