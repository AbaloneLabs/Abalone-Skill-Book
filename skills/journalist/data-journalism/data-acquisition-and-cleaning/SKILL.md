---
name: data-acquisition-and-cleaning.md
description: Use when the agent is obtaining datasets for a story, scraping or downloading public data, cleaning messy real-world data, handling missing duplicate or inconsistent records, documenting data transformations, or preparing data for analysis while preserving reproducibility and integrity.
---

# Data Acquisition And Cleaning

Data journalism begins long before analysis: it begins with getting the data and making it usable, and this is where most data stories are won or lost. Real-world data is messy, incomplete, inconsistently formatted, and shaped by the biases of how it was collected. The harm this skill prevents is the construction of findings on a foundation of corrupted, mis-cleaned, or misunderstood data, where confident conclusions rest on records that were duplicated, dropped, or distorted during processing. Rigorous acquisition and cleaning, fully documented, is the precondition for any defensible data-driven story. Skip it, and the analysis, however sophisticated, is built on sand.

## Core Rules

### Understand the data's origin and collection method before using it
Every dataset encodes assumptions about what was collected, how, when, by whom, and for what purpose. Learn the data dictionary, the collection protocol, and any known limitations. Data is not a neutral mirror of reality; it reflects the choices and blind spots of its creators. Understanding those choices is essential to interpreting the data honestly.

### Document every transformation from raw to analyzed
Record each step of cleaning and processing, ideally in reproducible scripts rather than manual edits, so that anyone can trace how the raw data became the analyzed version. Undocumented transformations are where accidental distortion hides and where challenges to your findings become unanswerable. If you cannot reproduce the path from raw to result, you cannot defend the result.

### Handle missing data deliberately, not by silent deletion
Missing values are rarely missing at random and carry meaning. Investigate why records are missing, decide on an appropriate strategy, imputation, exclusion with disclosure, or explicit treatment as a finding, and document the choice. Silently dropping incomplete records can change the results and hide systematic gaps that are themselves part of the story.

### Detect and resolve duplicates and inconsistencies systematically
Real datasets contain duplicate records, inconsistent encodings, conflicting entries, and formatting variations. Clean these with explicit rules, not ad-hoc fixes, and record the rules. The same entity spelled three ways, or the same record appearing twice, can inflate counts and manufacture false patterns if not handled.

### Preserve the original data unaltered
Keep the raw data exactly as received, and perform all cleaning on copies. The original is your ground truth, the reference to which you return if a transformation is questioned. Altering the original destroys the ability to audit or reproduce the work.

### Validate against external benchmarks
Cross-check the data against known totals, official statistics, prior reporting, or domain expectations. If your dataset implies a result wildly different from established benchmarks, investigate the discrepancy before believing either. Validation catches cleaning errors, scope mismatches, and dataset limitations that internal checks miss.

### Be transparent with readers about data sources and limitations
Tell readers where the data came from, what it covers and omits, how you cleaned it, and what its limitations are. Publish the data and methods where possible so others can reproduce and challenge your findings. Transparency is the armor of data journalism; opacity is its vulnerability.

### Recognize when the data cannot support the desired conclusion
Sometimes the data is too sparse, too biased, or too ambiguous to support a strong claim. The discipline to report what the data can and cannot say, and to refrain from overclaiming, is as important as any analytical skill. A story that hedges honestly is more credible and more durable than one that overstates.

## Common Traps

### Treating the dataset as a complete and neutral record
Datasets capture what was measured, not necessarily what happened. Under-reporting, selective recording, and collection bias mean the data may systematically omit the most relevant cases. Investigate the gaps, do not assume completeness.

### Silent deletion of inconvenient records
Dropping records that complicate the analysis, or that are messy to handle, can manufacture a cleaner-looking but wrong result. Every exclusion must be deliberate, justified, and documented.

### Over-cleaning until the signal is manufactured
Aggressive cleaning, collapsing categories, dropping outliers, can inadvertently create the pattern the journalist hopes to find. Test the sensitivity of results to cleaning choices; if the finding depends on one specific cleaning decision, it is fragile.

### Failing to merge datasets correctly
Combining datasets on a key, such as a name or ID, introduces errors from mismatches, duplicates, and inconsistent formatting. Mergers gone wrong silently drop or duplicate records and are a frequent source of erroneous findings. Verify merge integrity and record counts before and after.

### Trusting summary statistics over the underlying distribution
Means can be distorted by outliers, and aggregates can hide important subgroup variation. Examine the distribution, not just the summary, before drawing conclusions. A headline number that masks a bimodal or skewed distribution misleads.

### Ignoring the unit of analysis
Confusing individual-level and group-level data, or mixing units, produces category errors in reasoning. Be explicit about what each record represents and at what level, individual, transaction, geographic area, time period, the analysis operates.

### Presenting cleaned data as if it were raw
Readers, and sometimes editors, assume the published figures come straight from the source. Disclose the cleaning and transformations applied so the audience understands the figures are the product of processing, with the choices and limitations that implies.

## Self-Check

- [ ] Do I understand the dataset's origin, collection method, coverage, and known limitations before analyzing it?
- [ ] Is every transformation from raw to analyzed data documented in reproducible form?
- [ ] Have I investigated why data is missing and chosen a deliberate, documented strategy for handling it?
- [ ] Are duplicates, inconsistencies, and formatting variations resolved through explicit, recorded rules?
- [ ] Is the original raw data preserved unaltered, with all cleaning performed on copies?
- [ ] Did I validate the data against external benchmarks and investigate major discrepancies?
- [ ] Am I transparent with readers about the data's source, coverage, cleaning, and limitations, and have I published data and methods where possible?
- [ ] Have I tested the sensitivity of my findings to cleaning choices and merge operations?
- [ ] Am I examining distributions and subgroups, not relying solely on summary statistics?
- [ ] Before publication, am I claiming only what the data can actually support, and disclosing where it cannot?
