---
name: in_process_quality_checks.md
description: Use when the agent is performing quality checks during the translation or post-editing process itself, running automated QA checks in a CAT tool, validating terminology and consistency while drafting, checking numbers placeholders tags and locale conventions segment by segment, doing self-revision between drafting and handoff, or embedding quality verification into the production stage rather than leaving it all to a separate reviewer.
---

# In-Process Quality Checks

In-process quality checks are the verifications a translator or post-editor performs while the work is being produced, between first draft and handoff, as opposed to the separate review stage performed by another person. This distinction matters because the cheapest place to catch a defect is at the moment it is created, and the most expensive place is after delivery when a client finds it. Agents frequently treat quality as something that happens later, in review, and then are surprised when review is overloaded with defects that should never have left the drafting stage, or when a smooth-looking target hides a mistranslation that no automated check would flag. In-process checking is not about being perfectionist during drafting; it is about running the right checks at the right moments so that the text handed to the reviewer is genuinely self-revised, internally consistent, and free of the mechanical defects that waste review capacity. The harm of skipping in-process checks is that review becomes a first-pass filter instead of a quality gate, the reviewer spends their attention on typos and terminology inconsistencies rather than on meaning and fluency, and the same preventable defect classes recur project after project.

Use this skill when translating or post-editing and deciding what to verify during the work, configuring and running automated QA, doing self-revision, or establishing the checks a translator must complete before declaring a segment or file done. The goal is to embed verification into production so that what reaches review is already substantially clean.

## Core Rules

### Separate Mechanical Checks From Meaning Checks

In-process verification has two families, and conflating them wastes effort. Mechanical checks are deterministic and automatable: terminology conformance against the termbase, consistency of repeated segments, numbers, dates, units, currency, placeholders, tags, variables, segment completeness, and character limits. Meaning checks require human judgment: accuracy against source, preservation of negation and modality, tone and register, and naturalness. Run mechanical checks with the CAT tool's QA functions and resolve every flag before handoff; they should produce zero open issues, because every open mechanical issue is a defect that review should not have to find. Reserve human attention for meaning checks, which no tool can fully perform. Treating meaning checks as something the QA tool will catch is a category error that lets mistranslations through, while treating mechanical checks as something to do manually wastes time and invites inconsistency.

### Configure Automated QA Before Drafting, Not After

Automated QA is only as good as its configuration. Before drafting, load the correct termbase, enable the consistency checker, set the locale for date, number, and unit validation, define the placeholder and tag patterns the project uses, and set character or length limits where the deliverable requires them. Configuring QA after drafting means the translator has already produced against an unverified rule set and must now rework to conform. Worse, running QA with a missing or wrong termbase produces false confidence, because the check passes on terminology that was never validated. Configuration is part of setup, and a project without configured QA is a project whose mechanical quality is unverified by design.

### Resolve Every QA Flag, Do Not Silence It

When the QA tool flags an issue, the correct response is to resolve it, not to dismiss it. A flag means a rule was triggered: a term mismatch, a number discrepancy, a broken placeholder, a duplicate segment translated differently, a length overflow. Each flag is either a real defect to fix or a false positive to document so the rule can be refined. Silencing flags to clear the report teaches the translator to ignore the tool and lets real defects hide among the dismissed ones. Keep a record of recurring false positives and feed them back so the configuration improves; over time the signal-to-noise ratio rises and the tool becomes more trustworthy.

### Run Targeted Passes For High-Risk Token Classes

Certain token classes carry disproportionate risk because a single character changes the meaning entirely. Run targeted passes for negation words, modal verbs such as must, may, should, can, and will, numbers including dosages, deadlines, and quantities, units and their conversions, dates and their locale formatting, currency, placeholders and variables that must remain intact, tags and markup, and links and identifiers. A focused pass reads the text looking only for that class, which is far more reliable than hoping to notice a wrong digit while reading for fluency. These passes are cheap and catch the defects that cause the most serious consequences.

### Self-Revise After A Break, Not Immediately

Self-revision is the translator's own review of their draft before handoff, and its effectiveness depends on distance. Immediately after drafting, the translator reads what they intended to write, not what is on the page, and misses their own errors. After a break, even a short one, the text is read more like a reader would, and omissions, wrong references, and awkward phrasing become visible. Schedule self-revision as a distinct stage separated from drafting by time or by working on a different segment set first. Self-revision should include a bilingual pass for accuracy on high-risk segments and a monolingual pass for fluency, rather than one merged read that lets accuracy crowd out naturalness.

### Verify Completeness Segment By Segment

Completeness failures, missing sentences, untranslated captions, skipped footnotes, content left in the source language, and duplicated segments, are among the most embarrassing and most preventable defect classes. Verify completeness against the source structure before handoff: every heading, paragraph, table cell, caption, footnote, form label, button, error message, alt text, and boilerplate block must be accounted for. Automated completeness checks compare segment counts, but they cannot catch a segment that was translated twice while another was skipped, so a structural pass is still needed. Treat any unexplained segment-count discrepancy as a defect to investigate, not a tool quirk.

### Enforce Terminology And Style At The Moment Of Choice

Terminology and style are easiest to enforce when the decision is made, not later. When translating a term, check the termbase and use the approved term; when the termbase offers none, record the decision and flag it for review rather than improvising silently. Consistency is destroyed by silent improvisation: the same concept rendered five ways across a file is a defect class that review then has to reconcile. The same applies to style choices such as register, formality, and punctuation conventions. Making and recording the decision in-process keeps the file internally consistent and gives the reviewer a documented basis to assess.

### Define A Done Definition For A Segment And A File

In-process checking needs a definition of done so that handoff is intentional rather than by exhaustion. A segment is done when it is translated, terminology-conforming, mechanically clean against QA, and self-revised for meaning where risk warrants. A file is done when all segments are done, completeness is verified, QA produces no open issues, and any unresolved uncertainties are noted for the reviewer. Without a done definition, handoff happens when the translator stops, which is not the same as when the work is complete.

## Common Traps

### Leaving All Quality To Review

Treating review as the place where quality happens overloads the reviewer with preventable defects and turns a quality gate into a first-pass filter.

### Running QA With Wrong Or Missing Configuration

A QA check run against a missing termbase or wrong locale produces false confidence by passing on never-validated terminology.

### Silencing Flags To Clear The Report

Dismissing flags instead of resolving them teaches the translator to ignore the tool and lets real defects hide among dismissed ones.

### Hoping To Catch High-Risk Tokens While Reading For Fluency

Reading for naturalness will not reliably catch a wrong digit, a flipped negation, or a broken placeholder; these need targeted passes.

### Self-Revising Immediately After Drafting

Reading the draft right after writing it reproduces the intended text in the mind and hides the translator's own errors.

### Silent Terminology Improvisation

Rendering a term five different ways because the termbase was silent destroys consistency and forces review to reconcile what should have been decided once.

### Handing Off By Exhaustion Rather Than By A Done Definition

Stopping when tired is not the same as completing; without a done definition, handoff omits the final verification steps.

## Self-Check

Before declaring a segment or file ready for handoff, verify:

- Mechanical checks, terminology, consistency, numbers, dates, units, currency, placeholders, tags, and limits, were run with correctly configured QA and produced no open issues.
- Meaning checks, accuracy, negation, modality, tone, and naturalness, received human attention rather than being delegated to the QA tool.
- Automated QA was configured before drafting with the correct termbase, locale, placeholder patterns, and limits, not added as an afterthought.
- Every QA flag was resolved or documented as a false positive with feedback to refine the rule, not silenced to clear the report.
- Targeted passes were run for high-risk token classes: negation, modals, numbers, units, dates, currency, placeholders, tags, and identifiers.
- Self-revision was performed after a break, with a bilingual accuracy pass and a monolingual fluency pass, not as one merged read immediately after drafting.
- Completeness was verified segment by segment against the source structure, with any segment-count discrepancy investigated.
- Terminology and style decisions were checked against the termbase and guide at the moment of choice, with undocumented decisions flagged for review.
- A defined done condition for the segment and the file was met, including no open QA issues and noted uncertainties, rather than handing off by exhaustion.
- The text reaching review is mechanically clean and self-revised so that review can focus on meaning rather than on preventable defects.
