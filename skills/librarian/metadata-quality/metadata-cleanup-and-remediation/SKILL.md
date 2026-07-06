---
name: metadata_cleanup_and_remediation.md
description: Use when the agent is cleaning up or remediating existing metadata, fixing systematic errors, normalizing inconsistent values, deduplicating records, migrating or upgrading legacy metadata, or planning a metadata cleanup project at scale.
---

# Metadata Cleanup And Remediation

Metadata accumulates problems over time: inconsistent vocabularies, duplicate records, empty mandatory fields, legacy formats, migrated errors, and drift from the current application profile. Cleanup and remediation is the work of correcting these problems at scale, and it is fundamentally different from creating metadata well in the first place. Cleanup operates on data that already exists, often in volume, where every change risks introducing new errors and where the goal is improvement without destruction. The dominant failure mode is well-intentioned but reckless batch editing: a global find-and-replace that fixes one problem while corrupting valid records, a deduplication that merges distinct entities, or a normalization that erases meaningful variation. Good remediation is deliberate and reversible. It analyzes before acting, tests changes on samples, preserves the original data, documents every transformation, and verifies the result against quality criteria. Cleanup done carefully restores a collection's discoverability; done carelessly, it causes damage that is harder to detect and repair than the original problems.

Use this skill when cleaning up or remediating metadata, fixing systematic errors, normalizing values, deduplicating records, migrating legacy metadata, or planning a large-scale cleanup project. The goal is to prevent the agent from applying reckless batch edits, destroying valid data through normalization, merging distinct entities, or cleaning up without backup, testing, or documentation.

## Core Rules

### Analyze Before Acting

Cleanup must begin with analysis, not editing. Understanding the scope, patterns, and causes of problems determines what to fix and how.

Analysis first:

- profile the problems by type, frequency, and collection segment;
- identify systematic versus isolated errors;
- determine the root cause, a past migration, a workflow gap, a vocabulary change;
- assess which problems most harm discovery;
- estimate the effort and risk of each fix.

Analysis prevents treating symptoms while the cause continues producing new problems, and it prevents over-investing in low-impact fixes.

### Back Up And Preserve Original Data Before Editing

Cleanup changes existing data, and changes can be wrong. Always preserve the original so remediation is reversible.

Preservation:

- back up the full dataset before any batch edit;
- preserve original values in a versioned system or audit trail;
- keep originals accessible for re-derivation if cleanup corrupts data;
- document the backup location and restoration procedure.

Without a backup, a bad batch edit is irreversible. With one, any change can be rolled back and retried.

### Test Changes On A Representative Sample

Batch changes that look correct in principle often fail on real data. Test every transformation on a sample before applying at scale.

Testing:

- select a representative sample covering the data's variety;
- apply the transformation and inspect every changed record;
- check for false positives, valid records wrongly changed;
- check for false negatives, problem records missed;
- check for collateral damage, records changed in unexpected ways;
- refine the transformation based on findings.

A transformation untested on real data will corrupt records in ways that are expensive to detect and repair later.

### Distinguish Systematic Fixes From Judgment Calls

Some cleanup is mechanical and safe to batch; some requires human judgment per record. Conflating the two causes errors.

Systematic, batchable fixes:

- format normalization, dates to ISO 8601;
- vocabulary term replacement where the mapping is unambiguous;
- removing known duplicate records;
- applying authority forms to variant name strings with confident matches.

Judgment-required fixes:

- merging records that may or may not describe the same object;
- choosing among conflicting subject headings;
- resolving ambiguous name authorities;
- correcting errors where the right value is uncertain.

Batch the systematic fixes after testing; route judgment calls to trained staff. Do not automate decisions that need human review.

### Avoid Destructive Normalization

Normalization standardizes values, but over-aggressive normalization erases meaningful variation. Normalize only where variation is truly erroneous.

Normalization cautions:

- do not normalize names or titles that are legitimately variant;
- preserve transcribed information even when regularizing access points;
- do not collapse distinct controlled terms that happen to be similar;
- retain local or copy-specific information during cleanup;
- document what was normalized and why.

A cleanup that turns every variant name into one form may improve consistency while destroying the evidence users need to identify the right entity. Normalize deliberately, not reflexively.

### Deduplicate Without Conflating

Duplicate detection and merging is one of the riskiest cleanup operations. Distinct records can look like duplicates, and merging them destroys information.

Safe deduplication:

- match on multiple attributes, not title or identifier alone;
- set a confidence threshold above which merge is automated and below which review is required;
- preserve merged records' data in the survivor or an audit trail;
- flag low-confidence matches for human review;
- after merge, verify that collocation improved rather than corrupted.

Two records with the same title may describe different editions, translations, or manifestations. Verify before merging, and always preserve the ability to unmerge.

### Document Every Transformation

Cleanup transformations must be documented so the work is reproducible, auditable, and maintainable.

Document:

- the problem addressed and its scope;
- the transformation applied and its logic;
- the sample testing and results;
- the number of records affected;
- the date, tools, and responsible staff;
- any residual issues or deferred fixes.

Documentation turns cleanup from an opaque edit into a transparent, reviewable process that future staff can understand and trust.

### Verify Results Against Quality Criteria

Cleanup is only successful if it improves quality. Verify the result against the same criteria used in assessment.

Verification:

- re-measure the quality metrics targeted by the cleanup;
- check that the fix did not introduce new problems;
- confirm completeness, consistency, and conformance improved;
- sample-review corrected records for accuracy;
- report before-and-after metrics to demonstrate impact.

Cleanup without verification is assumed improvement. Verification proves the work delivered value and catches regressions.

### Address Root Causes To Prevent Recurrence

Cleaning up today's problems without fixing their cause guarantees they return. Pair remediation with workflow or system fixes.

Root cause actions:

- fix the workflow gap that produced empty mandatory fields;
- update the application profile that allowed inconsistency;
- add validation that prevents the error at creation;
- retrain creators on the corrected practice;
- schedule monitoring to catch recurrence early.

Cleanup without root-cause fix is a recurring cost. Fix the cause to make the improvement durable.

## Common Traps

### Editing Before Analyzing

Acting without understanding produces misdirected and incomplete fixes. Analyze problems first.

### No Backup Before Batch Edits

Unbacked cleanup is irreversible. Preserve originals before editing.

### Untested Batch Transformations

Transformations that look correct fail on real data. Test on samples first.

### Automating Judgment Calls

Merging, conflicting term resolution, and ambiguous authorities need human review. Do not batch them.

### Destructive Normalization

Over-normalization erases meaningful variation. Normalize only true errors.

### Merging Distinct Records

Title or identifier similarity does not prove duplication. Verify and preserve before merging.

### Undocumented Cleanup

Opaque edits cannot be audited or trusted. Document every transformation.

### No Verification Of Results

Unverified cleanup is assumed improvement. Re-measure quality to prove impact.

### Ignoring Root Causes

Cleanup without cause-fixing recurs. Address the workflow or system gap.

## Self-Check

- Was the problem analyzed by type, frequency, cause, and impact before any editing began?
- Is the original data backed up and preserved so cleanup is reversible?
- Was every transformation tested on a representative sample before scale application?
- Are systematic fixes batched while judgment calls are routed to human review?
- Does normalization preserve legitimately variant and transcribed information rather than erasing it?
- Is deduplication based on multiple attributes with confidence thresholds and preservation of merged data?
- Is every transformation documented with problem, logic, testing, scope, and responsible staff?
- Are cleanup results verified against quality criteria with before-and-after metrics?
- Is the root cause of the problem addressed to prevent recurrence?
