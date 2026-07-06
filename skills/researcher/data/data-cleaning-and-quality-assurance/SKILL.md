---
name: data_cleaning_and_quality_assurance.md
description: Use when the agent is cleaning or transforming raw research data, validating values, handling outliers and anomalies, resolving inconsistencies, writing data quality checks, documenting the cleaning pipeline, or judging whether a cleaned dataset is fit for its intended analysis.
---

# Data Cleaning And Quality Assurance

Data cleaning is where the actual dataset is built. It is also where bias silently enters, because every decision, how to handle an impossible value, whether to impute, how to recode, which records to drop, changes the numbers that the analysis will later treat as truth. Researchers often clean data quickly and informally, leave the decisions undocumented, and then report results as if the dataset were a given rather than a construction. The consequence is analyses that cannot be reproduced, findings that depend on undocumented choices, and a literature where the same raw data could yield different conclusions.

Use this skill when cleaning raw data, when writing quality checks, when handling outliers and missing values, when transforming variables, and when judging whether a dataset is fit for analysis. The goal is to keep the agent from making cleaning decisions driven by the desired result, from leaving the cleaning pipeline undocumented, and from treating the cleaned dataset as ground truth rather than as a product of many choices. The agent has latitude in methods, but every transformation must be traceable, justified, and reproducible.

## Core Rules

### Separate Raw Data From Cleaned Data And Never Edit The Source

The raw data is the evidence. Editing it in place destroys provenance and makes every later claim unverifiable.

Establish:

- a read-only raw data store that is never modified;
- a scripted pipeline that reads raw data and writes cleaned data;
- versioning of both raw and cleaned datasets;
- a clear boundary between the two, with no manual edits to raw files;
- preservation of original encoding, units, and codes;
- a record of any data received from third parties and its provenance.

If the raw data can be changed, no result derived from it can be trusted to mean the same thing tomorrow.

### Document Every Cleaning Decision Before Or As It Is Made

Cleaning decisions are research decisions. They must be recorded with the same rigor as analytical choices, not reconstructed from memory later.

Document:

- the rules for each transformation and recode;
- the thresholds for flagging outliers or impossible values;
- the handling of each inconsistency or duplicate;
- the imputation method and its assumptions;
- the order in which operations were applied;
- the version of the code and data at each step;
- the rationale, not just the operation, for non-obvious choices.

A cleaning step without a recorded rationale is a hidden analytical choice that cannot be audited or defended.

### Validate Values Against Plausible Ranges And Rules

Raw data almost always contains errors: impossible values, mismatched units, reversed scales, transposed digits, and free-text that should be coded. Validation exposes these before analysis.

Check:

- range limits for each variable against known bounds;
- type consistency, such as numeric fields containing text;
- uniqueness where identifiers should be unique;
- referential integrity across linked tables;
- cross-variable consistency, such as dates that must follow other dates;
- allowed categories and their codes;
- unit consistency across sources and time;
- sentinel values, such as minus one or nine hundred ninety-nine, that may masquerade as real data.

Build these checks as assertions that fail loudly rather than silent corrections that hide problems.

### Handle Outliers By Investigation, Not By Default Deletion

Outliers are not noise by definition. They may be errors, they may be genuine extreme values, or they may be the most important cases in the dataset. Default deletion distorts results.

For each outlier:

- investigate its source before deciding;
- distinguish data entry errors from valid extremes;
- consider whether it reflects a different population or process;
- document the decision to keep, correct, winsorize, or remove;
- report sensitivity of results to the decision;
- avoid removing outliers because they weaken a preferred finding.

A policy of "remove values beyond three standard deviations" is a decision, not a default, and it must be justified against the construct and the distribution.

### Address Missing Data According To Its Mechanism

Missing data is not a nuisance to be filled or dropped. Its pattern carries information about bias, and the handling method must match the mechanism.

Classify and address:

- missing completely at random, where methods like listwise deletion may be tolerable;
- missing at random conditional on observed variables, where multiple imputation or weighting is appropriate;
- missing not at random, where the missingness depends on unobserved values and sensitivity analysis is essential;
- the proportion and pattern of missingness across variables and cases;
- whether missingness is related to the outcome or treatment;
- the distinction between missing values and genuine zeros or not-applicable codes.

Defaulting to mean imputation or complete-case analysis without examining the mechanism introduces bias that the analysis cannot later remove.

### Resolve Inconsistencies And Duplicates Explicitly

Real datasets contain contradictions: the same person recorded twice, conflicting values across sources, units that differ between sites, and codes that changed mid-study. Resolving these silently hides consequential decisions.

Resolve by:

- defining a deterministic rule for duplicates, such as preferring the latest or most complete record;
- reconciling conflicting values using a documented hierarchy of sources;
- standardizing units, codes, and categories across sources;
- recording the original and resolved values;
- flagging, not silently dropping, irreconcilable records;
- checking the effect of resolution rules on key variables.

A merge or deduplication that runs without inspection can quietly create or destroy records and change every downstream result.

### Make The Cleaning Pipeline Reproducible And Scripted

Point-and-click cleaning in a spreadsheet cannot be reproduced, audited, or trusted. The pipeline must be code that runs from raw to cleaned data without manual intervention.

Build the pipeline with:

- scripts in a version-controlled repository;
- a single command that regenerates the cleaned dataset from raw;
- explicit ordering and dependencies among steps;
- fixed random seeds for any stochastic step such as imputation;
- recorded software versions and environment;
- intermediate checkpoints for debugging;
- automated tests that verify invariants of the cleaned data.

If a colleague cannot regenerate the cleaned dataset from the raw data and the code, the cleaning is not reproducible.

### Align Cleaning With The Intended Analysis And Preserve Information

Cleaning should serve the analysis without pre-empting it. Over-cleaning, such as categorizing continuous variables or dropping variables deemed irrelevant, can foreclose analyses and bias results.

Balance by:

- preserving continuous variables in their raw scale where possible;
- avoiding premature categorization that loses information;
- retaining variables that may be needed for sensitivity or subgroup analyses;
- not excluding records based on the outcome, which biases results;
- documenting any analysis-driven exclusions as deviations;
- keeping the cleaned dataset rich enough to support the planned and reasonable alternative analyses.

The cleaned dataset is a platform for analysis, not a pre-committed answer.

## Common Traps

### Editing Raw Data In Place

Manual edits to source files destroy provenance and make results unverifiable and irreproducible.

### Undocumented Cleaning Choices

Decisions made in a spreadsheet and never recorded become invisible analytical choices that bias results and block audit.

### Deleting Outliers By Default

Removing extreme values because they are inconvenient distorts distributions and can reverse conclusions.

### Mean Imputation Without Examining Mechanism

Filling missing values with the mean ignores the missingness mechanism and understates uncertainty.

### Silent Deduplication And Merging

Merging or removing records without inspection can create phantom cases or drop real ones.

### Point-And-Click Cleaning

Spreadsheet-based cleaning cannot be reproduced and hides the decision chain from reviewers.

### Outcome-Driven Exclusions

Dropping records based on the outcome variable biases estimates and inflates false positives.

## Self-Check

- [ ] Is raw data kept read-only and separate from cleaned data, with versioning of both?
- [ ] Is every cleaning decision documented with its rule, threshold, and rationale?
- [ ] Are value validations implemented as assertions that fail loudly on impossible or inconsistent values?
- [ ] Are outliers investigated and decisions to keep, correct, or remove justified and sensitivity-tested?
- [ ] Is missing data classified by mechanism and handled with a method matched to that mechanism?
- [ ] Are inconsistencies and duplicates resolved through documented, deterministic rules?
- [ ] Is the cleaning pipeline fully scripted, version-controlled, and reproducible from a single command?
- [ ] Are random seeds, software versions, and the environment recorded for stochastic steps?
- [ ] Does cleaning preserve information needed for planned and alternative analyses without premature reduction?
- [ ] Are exclusions, especially outcome-driven ones, reported transparently as deviations from the plan?
