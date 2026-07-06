---
name: computational_reproducibility_and_open_materials.md
description: Use when the agent is making a study computationally reproducible, preparing code and data for sharing, writing documentation for an analysis pipeline, choosing a repository or license, or auditing whether published results can actually be regenerated from the provided materials.
---

# Computational Reproducibility And Open Materials

A result is computationally reproducible when another researcher can take the shared materials and regenerate the reported outputs. Many published results are not reproducible, not because of fraud, but because the analysis depended on undocumented software versions, manual steps, local file paths, random seeds, or data that was never shared. Reproducibility is a property of the whole pipeline, and it is built deliberately or it does not exist. The judgment problem is how to construct, document, and share materials so that the computational claims can be checked, extended, and trusted.

Use this skill when building a reproducible analysis pipeline, when preparing code and data for deposit, when writing documentation, when choosing a repository and license, and when auditing a study's reproducibility before submission. The goal is to keep the agent from treating "the code is on GitHub" as sufficient, and to make the shared materials actually function for someone who was not part of the original team.

This is a high-stakes domain because non-reproducible results erode trust in the record and waste the effort of researchers who try to build on them. When proprietary data, licensing limits, or sensitive information constrain sharing, the agent should consult a data steward or institutional office rather than defaulting to non-sharing or to improper disclosure.

## Core Rules

### Capture The Entire Pipeline, Not Just The Final Script

Reproducibility fails at the weakest link. A polished final script that depends on undocumented preprocessing, manual cleaning in a spreadsheet, or a local database extract cannot be run by anyone else.

Ensure the pipeline captures:

- raw data ingestion and any manual preprocessing steps;
- data cleaning and transformation;
- model fitting and statistical analysis;
- figure and table generation;
- the exact sequence in which steps run;
- all parameters, thresholds, and manual choices.

If a step was done by hand, document it so precisely that another person could repeat it, or better, encode it. Manual steps that nobody can remember are a leading cause of reproducibility failure.

### Pin Software Versions, Seeds, And Dependencies

Results that depend on floating-point arithmetic, randomized algorithms, or library behavior can change between versions. "It ran on my machine" is not reproducibility.

Pin:

- the operating system and architecture where relevant;
- the language version and package versions, ideally through a lockfile or environment file;
- random seeds for any stochastic step;
- container or virtual machine images for complex environments;
- the versions of any external tools or databases referenced.

Record these in a way that travels with the materials. A requirements file without pinned versions recreates only the illusion of the original environment.

### Make Data Available Or Document Exactly Why Not

Reproducibility requires data. When data cannot be shared, the barrier must be genuine, documented, and accompanied by the strongest feasible alternative.

For each dataset:

- share the exact data used, with a persistent identifier, when legally and ethically possible;
- if raw data cannot be shared, share derived or processed data that supports the analysis;
- if no data can be shared, provide a synthetic or simulated dataset that exercises the code;
- document access conditions, embargoes, and the process by which a qualified requester can obtain restricted data;
- never silently omit data availability, and never claim openness that does not exist.

Honest non-sharing with a clear access path is far better than a false openness claim or silent omission.

### Write Documentation For Someone Outside The Team

Materials that only the original team can use are not reproducible. Documentation must assume an outsider who knows the field but not this project.

Provide:

- a README explaining what the project is and how to run it;
- step-by-step instructions from environment setup to output generation;
- a description of the directory structure and file roles;
- the expected runtime, inputs, and outputs for each major step;
- a statement of what the provided materials should reproduce;
- contact information and a record of changes or known issues.

Test the documentation by having someone unfamiliar with the project follow it. Instructions that the author finds clear often fail for newcomers.

### Use Persistent Identifiers And Versioning For Shared Materials

Links rot and repositories change. Materials should be citeable and stable.

Use:

- persistent identifiers such as DOIs for deposited data and code;
- versioned releases rather than mutable main branches for the specific materials behind a paper;
- archival repositories rather than personal websites or mutable project pages;
- clear mapping between the paper's results and the specific version of the materials.

A reader citing the paper should be able to cite the exact version of the materials that produced the results.

### Choose Licenses Deliberately And Consistently

Licenses determine what others can do with the materials. Inconsistent or missing licenses create legal ambiguity that suppresses reuse.

Decide:

- an open license for code that permits inspection and reuse;
- a data license appropriate to the data's nature and constraints;
- whether the license is consistent across code, data, and documentation;
- whether third-party materials carry their own licenses that must be respected;
- whether the license aligns with funder and institutional requirements.

Defaulting to "all rights reserved" or applying no license both impede reuse. Choose deliberately and state the choice clearly.

### Audit Reproducibility Before Submission, Not After

Reproducibility problems are cheap to fix during the project and expensive to fix after publication. A pre-submission audit catches what memory and familiarity hide.

Audit by:

- running the full pipeline from a clean environment using only the shared materials;
- checking that outputs match the manuscript figures and tables;
- verifying that random seeds reproduce the reported results;
- confirming that documentation is complete enough for an outsider;
- testing on a different machine or account where possible.

If the team cannot reproduce its own results from the shared materials, no one else will either. Fix the pipeline before claiming reproducibility.

## Common Traps

### Treating Code Availability As Reproducibility

Posting code without data, environment, or documentation lets others read the code but not run it. Availability is necessary, not sufficient.

### Leaving Manual Steps Undocumented

Steps done interactively in a spreadsheet or console are invisible to anyone else. They break reproducibility even when the scripts are clean.

### Assuming Random Results Will Recur Without Seeds

Stochastic algorithms without fixed seeds produce different outputs each run. Reported results then cannot be regenerated.

### Claiming Open Data That Is Not Actually Accessible

Listing a dataset as available when it is embargoed, gated, or requires unobtainable permission misleads readers and reviewers.

### Using Mutable Links Instead Of Persistent Identifiers

Personal web pages and mutable repository branches disappear or change. Results tied to them become unreproducible over time.

### Applying Incompatible Or Missing Licenses

Code under one license, data under another, and documentation under none creates legal confusion that deters reuse and reuse auditing.

### Skipping The Clean-Environment Test

A pipeline that works only on the original author's machine, with cached files and local paths, is not reproducible. Familiarity hides the dependencies.

## Self-Check

- Does the shared pipeline capture every step from raw data to reported output, including manual steps?
- Are software versions, dependencies, random seeds, and environments pinned and recorded with the materials?
- Is the data available, or is the reason for non-sharing genuine, documented, and accompanied by the strongest feasible alternative?
- Is documentation written and tested for an outsider who knows the field but not the project?
- Do shared materials carry persistent identifiers and versioned releases mapped to the paper's results?
- Are licenses chosen deliberately, consistent across materials, and respectful of third-party constraints?
- Has the pipeline been run from a clean environment using only the shared materials before submission?
- Where proprietary data, licensing limits, or sensitive information constrain sharing, has a data steward or institutional office been consulted rather than defaulting to non-sharing or improper disclosure?
