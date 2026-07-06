---
name: systematic_review_screening_and_data_extraction.md
description: Use when the agent is screening records for a systematic review, deduplicating results, applying inclusion criteria at title and full text, extracting data from included studies, or managing dual review, conflict resolution, and audit trails.
---

# Systematic Review Screening And Data Extraction

Screening and data extraction are where a systematic review's promises are either kept or quietly broken. A protocol can be impeccable and still produce a biased synthesis if screeners exclude inconvenient studies through inconsistent criteria, if data are extracted incorrectly from favored or disfavored studies, or if the audit trail cannot show how thousands of records became a handful of included studies. Agents tend to treat these as clerical tasks, but every exclusion and every extracted number is an inferential act that shapes the result.

Use this skill when screening retrieved records, extracting data from included studies, setting up dual review, or building the record trail for a systematic or scoping review. The goal is to prevent the agent from producing a clean PRISMA flow that masks inconsistent, undocumented, or biased decisions.

## Core Rules

### Deduplicate Before Screening, But Carefully

Duplicate records inflate counts and cause double screening, but over-aggressive deduplication can merge distinct reports of the same or different studies and silently drop evidence.

Deduplicate by:

- using a reference management or deduplication tool, then manually verifying;
- matching on title, authors, year, and where possible trial or study identifier;
- keeping records that might be separate reports of one study linked, not deleted;
- logging how many duplicates were removed and on what basis.

Never delete a record as a duplicate without confirming it is not a distinct study or a distinct outcome report.

### Screen In The Defined Order With Clear Rules

Screening should proceed from title and abstract to full text, applying the eligibility criteria consistently. Order and rule clarity reduce drift across thousands of decisions.

Apply rules by:

- screening first on clearly unambiguous criteria (e.g., wrong population, wrong design);
- deferring borderline decisions to full text rather than guessing at abstract;
- keeping the eligibility criteria visible during screening;
- not using information beyond the current stage to justify a decision.

Avoid excluding at title and abstract for reasons that require full text to judge, such as "no relevant outcome" when outcomes are often unreported in abstracts.

### Use Dual Screening And Resolve Conflicts Explicitly

Dual independent screening is a core protection against individual error and bias. A single screener's preferences can shape the included set without check.

Implement dual screening by:

- two reviewers screening independently at title/abstract and full text;
- a defined conflict resolution process (discussion, third reviewer);
- recording agreement and the number resolved by each route;
- calibrating screeners on a pilot batch before full screening.

If dual screening is infeasible for all records, use it at least at full text and on a sample of title/abstract exclusions, and disclose the limitation.

### Log Every Exclusion Reason At Full Text

At full text, every excluded study needs a recorded reason mapped to an eligibility criterion. This is the most auditable part of the review and the most common place shortcuts appear.

For each full-text exclusion record:

- the specific criterion violated (not just "ineligible");
- the study characteristic that triggered exclusion;
- enough detail that a reader could judge the decision;
- a category that aggregates reasons for the PRISMA flow.

Vague reasons like "not relevant" are not auditable. Map every exclusion to the protocol's defined criteria.

### Extract Data With A Pre-Piloted Form

Data extraction errors directly distort the synthesis. A piloted form ensures the same fields are captured consistently across studies and raters.

Pilot and refine the form by:

- extracting from a few diverse included studies first;
- defining each field's unit, format, and handling of missing data;
- specifying how to extract multiple reported measures or time points;
- planning handling of unreported, partially reported, or graphical data.

Free-form note-taking across studies guarantees inconsistency.

### Extract In Duplicate For Key Fields

Errors in extracting effect estimates, sample sizes, and outcome definitions are common and consequential. Duplicate extraction catches them.

Apply duplicate extraction to:

- primary outcome data and effect estimates;
- sample sizes and unit-of-analysis information;
- risk-of-bias judgments and their supporting quotes;
- any field feeding directly into meta-analysis.

Resolve discrepancies by discussion or third reviewer, and record the resolution.

### Handle Missing And Reported-As-Text Data Deliberately

Studies rarely report everything the review needs. How missing data are handled changes the synthesis.

Plan for:

- outcomes reported only graphically (use digital extraction tools and record method);
- data reported only as "not significant" without an effect;
- subgroups or time points that differ from the protocol;
- contacting authors for missing information, with a log of attempts and responses;
- pre-specified rules for imputation or for treating data as unavailable.

Do not silently substitute a convenient proxy for an unreported outcome.

### Maintain The Record Trail End To End

The PRISMA flow is the audit spine of the review. It must be reconstructable from raw logs, not assembled from memory at the end.

Maintain:

- identification counts per source;
- deduplication counts;
- records screened and excluded at each stage with reasons;
- studies sought for retrieval and those unretrievable;
- studies included in qualitative and quantitative synthesis;
- links between reports and the studies they belong to.

Every number in the flow should trace to a logged decision.

### Distinguish Reports From Studies

Multiple reports can describe one study, and one report can describe several studies. Confusing the two inflates counts and double-counts participants.

For each included item, record:

- whether it is a report or a study;
- which study it belongs to (by identifier where possible);
- whether it overlaps with another report's sample;
- which report is the source for each extracted datum.

## Common Traps

### Over-Aggressive Deduplication

Merging records that are actually distinct studies, or distinct outcome reports of one study, silently drops evidence.

### Excluding At Abstract For Full-Text Reasons

Outcomes and designs are often under-reported in abstracts. Excluding for them at title and abstract loses studies that would qualify at full text.

### Single-Screener Convenience

One screener's preferences shape the set invisibly. Dual screening at least at full text is a minimum safeguard.

### Vague Exclusion Reasons

"Not relevant" or "did not meet criteria" without the specific criterion makes the exclusion unauditable.

### Free-Form Extraction

Extracting into unstructured notes guarantees inconsistent fields and units across studies.

### Silent Proxy Substitution

Replacing an unreported outcome with a convenient available measure without flagging it distorts the synthesis.

### Assembling PRISMA From Memory

Counts reconstructed after the fact are unreliable and often internally inconsistent with the logs.

## Self-Check

- Was deduplication verified manually so distinct studies or reports were not dropped?
- Does screening proceed title/abstract to full text with criteria applied at the correct stage?
- Is dual independent screening in place, with conflict resolution and agreement recorded?
- Does every full-text exclusion have a specific, criterion-mapped reason?
- Was the data extraction form piloted on diverse studies before full use?
- Are primary outcome data, sample sizes, and risk-of-bias judgments extracted in duplicate?
- Are missing, graphical, and partially reported data handled by pre-specified rules?
- Can every number in the PRISMA flow be traced to a logged decision?
- Are reports distinguished from studies, with overlaps and identifiers recorded?
- Are author contact attempts and responses logged for missing data?
