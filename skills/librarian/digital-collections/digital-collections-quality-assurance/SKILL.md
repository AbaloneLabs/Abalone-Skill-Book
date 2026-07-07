---
name: digital_collections_quality_assurance.md
description: Use when the agent is performing quality assurance on digitized or born-digital collections, checking image quality, OCR and transcription accuracy, metadata completeness, file integrity, link and display behavior, or ensuring digital surrogates faithfully represent originals and meet access and preservation standards.
---

# Digital Collections Quality Assurance

Quality assurance (QA) is the discipline that stands between a digitization workflow and a public-facing collection that misrepresents its sources. A digital collection can look complete and still be subtly broken: images cropped or skewed, pages missing or duplicated, OCR text riddled with errors, metadata that does not match the item, files corrupted or misnamed, and links that resolve to the wrong object. Each of these failures erodes trust, undermines research, and can persist undetected for years because no one is looking systematically. QA is not a final glance before publication; it is a structured set of checks applied at multiple stages, from the captured files through metadata, display, and long-term integrity. Treating QA as optional or as a single end-of-line review guarantees that errors reach users, and because digital collections are often treated as authoritative surrogates, those errors propagate into citations and scholarship. Strong QA treats every digital object as a claim about a physical original and verifies that the claim holds.

Use this skill when performing quality assurance on digitized or born-digital collections, checking image and file quality, verifying OCR and transcription, auditing metadata, and ensuring display and integrity. The goal is to prevent the agent from treating QA as optional or end-of-line, from checking only a sample without sampling discipline, from conflating completeness with correctness, or from neglecting long-term file integrity.

## Core Rules

### Treat QA As A Multi-Stage Discipline

QA is not a single check at the end. Build checks into every stage of the workflow.

Stage-appropriate checks:

- **Capture:** image focus, exposure, color, cropping, skew, completeness against the original.
- **Processing:** derivative generation, file naming, directory structure, compression artifacts.
- **OCR and transcription:** character accuracy, structural markup, handling of difficult scripts or layouts.
- **Metadata:** agreement between metadata and the item, completeness, controlled vocabulary correctness.
- **Ingest and display:** correct linking, display behavior, searchability, accessibility.
- **Preservation:** file integrity, fixity, format viability over time.

Multi-stage QA catches errors when they are cheapest to fix.

### Verify Surrogates Against Originals

A digital surrogate makes an implicit claim to represent the original. Verify that claim.

Verify:

- completeness (no missing, duplicated, or out-of-order pages or items);
- accurate capture of content (text legibility, image detail, color fidelity to a target);
- faithful representation of physical features (marginalia, color, scale where relevant);
- correct orientation and sequencing;
- that the surrogate supports the uses it is meant to support.

A surrogate that omits or distorts content betrays the original and misleads users.

### Check Image And File Quality Systematically

Image and file quality problems are common and often invisible without systematic checking.

Check:

- resolution sufficient for the intended use and stated standards;
- focus, exposure, color balance, and consistency across the batch;
- cropping, skew, and artifacts from capture or compression;
- file integrity (files open, are not corrupted, match expected formats);
- correct and consistent file naming and organization.

Random spot-checks miss systematic problems; check against defined criteria.

### Audit OCR And Transcription Accuracy

OCR and transcription text is often presented as searchable and authoritative. Audit its accuracy.

Audit:

- character accuracy against the original, especially for difficult fonts, scripts, or layouts;
- structural correctness (paragraphs, columns, reading order, tables);
- handling of non-text elements (illustrations, handwriting, marginalia);
- searchability of the resulting text;
- transparency about OCR limitations where accuracy cannot be guaranteed.

Presenting uncorrected OCR as accurate text misleads users who rely on it.

### Verify Metadata Against Items

Metadata must agree with the item it describes. Verify this systematically.

Verify:

- titles, dates, creators, and other descriptive elements match the item;
- subject and genre terms are appropriate;
- identifiers and links resolve to the correct object;
- physical and technical descriptions are accurate;
- completeness against local and national standards.

Metadata that disagrees with its item undermines the entire collection's credibility.

### Use Sampling With Discipline

When 100% inspection is infeasible, use disciplined sampling rather than convenience checks.

Sample with discipline by:

- defining the population and the sampling frame;
- choosing a sample size that gives meaningful confidence;
- sampling randomly or stratified across batches and item types;
- escalating to full inspection when error rates exceed thresholds;
- documenting sampling methods and results.

Convenience sampling (checking the first few items) misses systematic and clustered errors.

### Check Display, Discovery, And Accessibility

A correct file is useless if it displays or behaves incorrectly. Check the user experience.

Check:

- correct display of images, text, audio, and video in the access platform;
- accurate linking between objects, metadata, and finding aids;
- searchability and faceting behavior;
- accessibility (alt text, keyboard navigation, screen reader compatibility, captions);
- performance and load behavior across devices and connections.

Display and accessibility checks catch problems invisible at the file level.

### Establish And Track Fixity For Preservation

Digital files degrade or change over time. Establish and track fixity from the start.

Establish:

- checksums or other fixity values at ingest;
- scheduled fixity checks against stored values;
- processes to investigate and remediate mismatches;
- logging of any changes or migrations;
- format monitoring for obsolescence risk.

Without fixity monitoring, silent corruption can persist undetected for years.

### Document QA Standards And Results

QA is only defensible if documented. Record standards, methods, and results.

Document:

- the QA standards and criteria applied at each stage;
- sampling methods and sample sizes;
- error rates and types found;
- remediation actions taken;
- sign-off and responsibility for each batch or collection.

Documentation turns QA from an ad hoc activity into an auditable process.

### Build Feedback Loops Into The Workflow

QA findings should improve the workflow, not just catch errors. Build feedback loops.

Feed back by:

- analyzing error patterns to find root causes;
- adjusting capture, processing, or metadata workflows to prevent recurrence;
- training staff and vendors on recurring issues;
- tracking whether error rates decline over time.

Without feedback, the same errors recur batch after batch.

## Common Traps

### Treating QA As A Single End-Of-Line Check

Errors are cheapest to fix early. Build QA into every stage of the workflow.

### Checking Only A Convenience Sample

Random or stratified sampling finds systematic errors; convenience checks miss them.

### Conflating Completeness With Correctness

All pages present does not mean all pages correct. Verify content and quality, not just count.

### Presenting Uncorrected OCR As Accurate

OCR has errors. Audit accuracy and be transparent about limitations.

### Metadata Disagreeing With Items

Verify metadata against the items it describes. Disagreement undermines credibility.

### Neglecting Display And Accessibility

Correct files that display or behave incorrectly are still broken for users.

### No Fixity Monitoring

Silent corruption persists without checksums and scheduled checks.

### Undocumented QA

Without documented standards and results, QA cannot be audited or improved.

## Self-Check

- Did you build QA checks into each stage (capture, processing, OCR, metadata, display, preservation)?
- Did you verify surrogates against originals for completeness, content, and fidelity?
- Did you check image and file quality against defined criteria, not just spot-check?
- Did you audit OCR and transcription accuracy and disclose limitations?
- Did you verify metadata against the items it describes?
- Did you use disciplined sampling with defined populations and escalation thresholds?
- Did you check display, linking, searchability, and accessibility in the access platform?
- Did you establish fixity values and schedule periodic integrity checks?
- Did you document QA standards, methods, error rates, and remediation?
- Did you build feedback loops to improve the workflow based on QA findings?
