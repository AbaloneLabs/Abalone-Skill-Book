---
name: acl_and_idea.md
description: Use when the agent is using ACL Analytics or IDEA for audit data analysis, writing computer-assisted audit technique scripts, importing and defining data, building continuous monitoring scripts, running gap and sequence checks, duplicate detection, Benford's Law analysis, stratification, aging, summarization, joining files, or exporting analytics results to audit workpapers.
---

# ACL And IDEA For Audit Data Analysis

ACL Analytics and IDEA are purpose-built computer-assisted audit technique (CAAT) tools that let the auditor interrogate entire populations rather than samples. Their power is also their risk. Agents often run a few standard scripts, copy the output into a workpaper, and conclude the procedure is complete, without confirming the data was imported correctly, the field definitions are right, or the script actually tests the intended assertion. The harm this prevents is real: a duplicate-payment test run against the wrong date range, a Benford analysis on a population that includes non-natural amounts, or a join that silently drops records can produce a clean result that hides the very errors the test was meant to find. Defensible CAAT work demands disciplined data definition, validated scripts, and results tied to the audit objective.

Use this skill when designing or running ACL or IDEA analytics, writing CAAT scripts, or exporting results to workpapers. The goal is to produce population-level evidence that is accurate, reproducible, and clearly linked to the assertion being tested.

## Core Rules

### Tie Every Test To An Audit Objective And Assertion

A data analysis that does not map to an assertion is a demonstration, not audit evidence. Before writing a script, state what the test is meant to prove or disprove.

For each test, define:

- the assertion addressed (existence, completeness, accuracy, cutoff, and so on);
- the specific risk or control being tested;
- the population and period the test covers;
- the expected result if the control or assertion holds;
- what an exception would mean and how it would be followed up;
- the materiality or threshold applied to results.

If the test cannot be tied to an assertion, reconsider whether it should be run.

### Define And Validate Data Imports Rigorously

The most common CAAT failure is bad data definition. A misread date format, a truncated field, or a wrong data type silently corrupts every downstream test.

Validate imports by:

- reconciling record counts and totals to the source system or trial balance;
- confirming field names, types, and lengths against the data dictionary;
- checking date formats and resolving ambiguous formats explicitly;
- verifying numeric fields have no embedded text or nulls in critical positions;
- confirming the population is complete for the period tested;
- documenting the source, extract date, and extraction method;
- retaining the original extract unchanged for reproducibility.

Do not run analytics until the import is reconciled. An unreconciled population invalidates the results.

### Build Scripts That Are Reproducible And Documented

CAAT work must be reproducible by another auditor. A script that exists only in a GUI session, with no saved log, cannot be reviewed or rerun.

Ensure reproducibility by:

- saving scripts and command logs in the engagement file;
- documenting the purpose and parameters of each script;
- versioning scripts when they change;
- parameterizing dates, thresholds, and file paths rather than hard-coding;
- including comments explaining non-obvious logic;
- recording the data file versions used;
- retaining the output with a reference back to the script.

If a reviewer cannot reproduce the result from the file, the evidence is weak.

### Match The Test Design To The Assertion

Each standard CAAT test answers a different question. Using the wrong test for the assertion produces irrelevant evidence.

Match tests to objectives:

- gap and sequence checks for completeness of sequentially numbered documents;
- duplicate detection for occurrence of duplicate payments or invoices;
- Benford's Law for indicators of fabricated or manipulated natural amounts;
- stratification for focusing on high-value or high-risk bands;
- aging for valuation and collectability of receivables or payables;
- summarization for reasonableness and trend analysis by category;
- joins and relations for matching across files, such as payroll to HR;
- exception reporting for control bypass or threshold breaches.

State which assertion each chosen test supports before running it.

### Apply Benford And Statistical Tests To Appropriate Populations

Benford's Law and similar analyses are powerful but frequently misapplied. They apply to naturally occurring amounts across orders of magnitude, not to all data.

Apply Benford correctly by:

- using populations of naturally generated amounts (revenues, expenses);
- excluding assigned or capped amounts (account numbers, set prices);
- excluding populations too small or too narrow in magnitude;
- investigating spikes as indicators, not proof of fraud;
- combining with other tests rather than relying on Benford alone;
- documenting the population scope and exclusions.

A Benford result on an inappropriate population is misleading evidence.

### Handle Joins And Relations Without Losing Records

Joins are where records silently disappear or duplicate. A join that drops unmatched records can hide completeness issues.

Manage joins by:

- choosing the join type deliberately (inner, left, full) to match the objective;
- checking record counts before and after the join;
- investigating unmatched records rather than discarding them;
- confirming join keys are unique or handling many-to-many cases;
- validating that the join did not create unintended duplicates;
- documenting the join logic and key fields.

For completeness assertions, an inner join that drops unmatched records is exactly the wrong choice.

### Export And Document Results For The Workpaper

Analytics output must be tied into the audit file as evidence, with enough context that a reviewer understands what was tested and concluded.

Export and document:

- the test performed and the assertion addressed;
- the population, period, and record count;
- the script or command log reference;
- the exceptions found and their significance;
- the follow-up performed on exceptions;
- the conclusion and its link to the audit objective;
- the reviewer and review evidence.

Do not paste raw output into a workpaper without interpretation. A table of duplicates is not a conclusion.

### Use Continuous Monitoring Scripts With Governance

Continuous monitoring scripts run repeatedly and can detect issues over time, but they require governance to remain valid.

Govern continuous monitoring by:

- defining the trigger conditions and recipients;
- reviewing and tuning thresholds periodically;
- confirming the underlying data feed remains complete and accurate;
- versioning and documenting script changes;
- tracking and resolving the issues the scripts surface;
- re-validating scripts when source systems change.

A monitoring script that nobody acts on provides false assurance.

## Common Traps

### Running Standard Scripts Without Validating The Import

Standard duplicate or gap scripts run against an unreconciled population produce confident but wrong results. Reconcile first, every time.

### Benford On An Inappropriate Population

Applying Benford to account numbers, set prices, or small uniform populations produces meaningless deviations. Match the test to data that follows the law.

### Silent Record Loss In Joins

An inner join that drops unmatched records can hide the completeness errors the test was meant to find. Check counts and investigate unmatched records.

### Hard-Coded Parameters

Scripts with hard-coded dates and paths break when rerun on a new period and are hard to review. Parameterize and document.

### Output Without Interpretation

Pasting analytics output into a workpaper without stating the conclusion leaves the evidence uninterpreted. Always tie results to the assertion and conclusion.

### Ignoring Exceptions

Finding exceptions and not following up defeats the test. Investigate each exception and document the resolution.

### Treating Analytics As A Sampling Substitute Without Population Validation

Running analytics on the full population only helps if the population is complete and accurate. Validate the population before relying on population-level conclusions.

### No Reproducibility

A GUI-only session with no saved log cannot be reviewed or rerun. Save scripts, logs, and data versions.

## Self-Check

- Is each analytics test tied to a specific assertion, risk, or control before the script is written?
- Has the data import been reconciled to the source system or trial balance for record counts and totals?
- Are field definitions, types, date formats, and population completeness validated and documented?
- Are scripts saved, versioned, parameterized, and commented so a reviewer can reproduce the result?
- Does each chosen test (gap, duplicate, Benford, stratification, aging, summarization, join) match the assertion it is meant to support?
- Is Benford's Law applied only to appropriate natural-amount populations, with exclusions documented?
- Do joins preserve or deliberately account for unmatched records, with before-and-after record counts checked?
- Are exceptions investigated and resolved, with follow-up documented rather than ignored?
- Is the output exported to the workpaper with test description, population, script reference, exceptions, conclusion, and review evidence?
- Are continuous monitoring scripts governed with periodic tuning, data-feed validation, and issue tracking?; is the original data extract retained unchanged for reproducibility alongside the scripts and outputs?
