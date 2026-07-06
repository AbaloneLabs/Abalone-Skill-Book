---
name: data-extraction-and-cleaning.md
description: Use when the agent is cleaning and structuring scraped or raw web data, parsing HTML and JSON into analysis-ready fields, handling missing and inconsistent values, deduplicating records, validating data quality, or documenting every transformation so the cleaned dataset remains a faithful and auditable representation of what was originally collected.
---

# Data Extraction And Cleaning

Raw scraped or API data is almost never analysis-ready. It arrives as nested JSON, messy HTML, inconsistent encodings, duplicate records, and fields that mean different things across sources. Cleaning this data into an analysis-ready form is where much of a computational project's integrity is decided, and it is where agents most often introduce silent errors. Every transformation, parsing a field, imputing a missing value, merging on a key, deduplicating, is a decision that shapes the findings, and these decisions are frequently made ad hoc, undocumented, and irreversible. The judgment problem is to preserve fidelity to the original data, to make every transformation explicit and reversible, and to validate that the cleaned dataset is a faithful representation of what was collected. A cleaned dataset produced by an opaque, undocumented pipeline is a black box: no one, including the researcher, can be sure what was changed or lost, and the findings rest on transformations that cannot be audited.

Use this skill when extracting, cleaning, or structuring raw data for analysis. The goal is to prevent the agent from making undocumented irreversible transformations, from introducing silent errors, and from trusting a cleaned dataset whose relationship to the raw data is unclear. The agent has freedom in tooling, but every transformation must be explicit, validated, and traceable to the source.

## Core Rules

### Preserve The Raw Data Immutably

The raw data as collected is the ground truth. It must be preserved unchanged as the reference for all cleaning.

Preserve by:

- storing raw captures as read-only, never overwritten by cleaning;
- treating the raw data as the immutable input to a reproducible pipeline;
- keeping raw and cleaned data clearly separated;
- a manifest linking cleaned records back to their raw source.

If cleaning overwrites or discards raw data, errors cannot be diagnosed and the pipeline cannot be re-run. Immutability of the raw layer is the foundation of trustworthy cleaning.

### Make Every Transformation Explicit And Reproducible

Cleaning must be a documented pipeline, not a sequence of manual edits. Each step should be code that can be re-run.

Make explicit by:

- scripting all transformations rather than editing in place;
- ordering steps deterministically and recording them;
- versioning the cleaning code alongside the data;
- avoiding manual, point-and-click changes that leave no trace.

Manual edits in a spreadsheet are invisible and irreproducible. A scripted pipeline makes the cleaning auditable and re-runnable, which is essential for credibility.

### Parse Structured Data To Analysis-Ready Fields

Raw JSON or HTML must be parsed into typed, well-named fields that match the analysis needs.

Parse by:

- defining a clear schema for the cleaned data;
- handling nested and repeated structures deliberately, not by flattening blindly;
- typing fields correctly: dates as dates, numbers as numbers, categories as categories;
- documenting what each field means and how it was derived.

Blind flattening of nested data can create spurious rows or lose structure. A deliberate schema ensures the parsed data represents the source faithfully and serves the analysis.

### Handle Missing Data Deliberately

Missingness is information, not just noise to fill. How it is handled shapes the analysis.

Handle by:

- distinguishing truly missing from structurally absent or zero;
- recording the pattern and likely mechanism of missingness;
- choosing imputation or exclusion deliberately, with rationale;
- flagging imputed values so they are not mistaken for observed.

Treating all blanks alike, or silently dropping incomplete records, can bias results and hide problems. Missingness must be analyzed and documented, not silently resolved.

### Deduplicate Carefully And Document The Logic

Duplicates arise easily in scraping and API collection. Removing them requires a defensible definition of uniqueness.

Deduplicate by:

- defining what makes a record unique: id, content hash, or combination of fields;
- distinguishing exact from near-duplicates, which need different handling;
- recording how many duplicates were removed and on what key;
- preserving information about removed duplicates where relevant.

Aggressive deduplication can collapse genuinely distinct records; weak deduplication leaves the dataset inflated. The uniqueness definition must be deliberate and reported.

### Validate Data Quality Against Expectations

Cleaned data should be checked against domain and structural expectations before analysis.

Validate by:

- range and type checks on each field;
- checks for impossible or implausible values;
- cross-field consistency checks;
- comparison of summary statistics before and after cleaning.

Validation catches parsing errors, encoding problems, and silent corruption. A dataset that has not been validated may carry errors that distort every downstream result.

### Reconcile Inconsistencies Across Sources

When data comes from multiple sources or time points, inconsistencies must be reconciled deliberately.

Reconcile by:

- defining canonical values for categories, names, and identifiers;
- handling encoding, casing, and formatting differences;
- a documented rule for resolving conflicts between sources;
- recording what was reconciled and how.

Merging sources without reconciliation produces a dataset riddled with near-duplicates and inconsistent categories. Canonicalization must be deliberate and auditable.

### Document The Full Cleaning Pipeline For Audit

The reader or a future researcher must be able to follow from raw to cleaned data.

Document by:

- the schema and the meaning of each field;
- every transformation, its rationale, and its order;
- the handling of missingness, duplicates, and inconsistencies;
- validation results and known remaining issues.

A cleaned dataset without documentation is an assertion. Documentation makes the cleaning auditable and lets others judge whether the cleaned data faithfully represents the raw.

## Common Traps

### Overwriting Raw Data

Cleaning that overwrites or discards raw captures makes errors undiagnosable and the pipeline irreproducible.

### Manual Invisible Edits

Point-and-click changes in a spreadsheet leave no trace and cannot be audited or re-run.

### Blind Flattening

Parsing nested structures by blindly flattening can create spurious rows or lose meaningful structure.

### Silent Missing-Data Handling

Dropping or imputing blanks without analysis or documentation hides bias and loses information.

### Sloppy Deduplication

Removing duplicates on a weak key collapses distinct records or leaves the dataset inflated.

### Unvalidated Data

Skipping quality checks lets parsing and encoding errors silently distort every downstream result.

### Undocumented Pipeline

A cleaned dataset whose transformations are not documented cannot be audited or trusted.

## Self-Check

- Is the raw data preserved immutably as the reference for all cleaning?
- Is every transformation scripted, ordered, and reproducible rather than manually edited?
- Is structured data parsed into a deliberate, typed schema that faithfully represents the source?
- Is missing data analyzed for pattern and mechanism, with handling documented and imputed values flagged?
- Is deduplication based on a defensible uniqueness definition, with removed counts and keys recorded?
- Is data quality validated against range, type, consistency, and before-after comparisons?
- Are inconsistencies across sources reconciled through documented canonicalization rules?
- Is the full cleaning pipeline documented, including schema, transformations, and known issues?
- Can a reader trace each cleaned field back to its raw source?
- Does the cleaned dataset remain a faithful, auditable representation of what was collected?
