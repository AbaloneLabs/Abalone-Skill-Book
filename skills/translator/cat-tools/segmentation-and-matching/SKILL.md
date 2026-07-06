---
name: segmentation_and_matching.md
description: Use when the agent is configuring or troubleshooting text segmentation in CAT tools, handling segmentation breaks and exceptions, tuning fuzzy match thresholds, or dealing with segmentation mismatches that reduce translation memory leverage and disrupt reuse.
---

# Segmentation And Matching

Segmentation is the invisible foundation of CAT tool work. Before a translator sees a segment, the tool has split the source text into units, usually sentences, and those units determine what the translator works on, what the translation memory can match, and how reuse flows across projects. When segmentation is good, it is invisible: sentences align naturally, matches surface, and work proceeds smoothly. When segmentation is bad, it is corrosive: a sentence split mid-clause forces the translator to translate fragments, matches fail because the same content was segmented differently before, terminology tools cannot align, and quality suffers because context is lost. Segmentation problems are often misdiagnosed as translator errors or poor TM, when the root cause is that the text was cut in the wrong places. Matching, governed by fuzzy match thresholds and algorithms, determines how aggressively the tool proposes partial matches. Tuning segmentation and matching is an engineering task that directly affects translation quality, speed, and leverage, and it deserves deliberate attention rather than acceptance of defaults.

Use this skill when configuring or troubleshooting segmentation, tuning fuzzy match thresholds, or resolving segmentation mismatches that reduce leverage. The goal is segmentation that aligns with meaning and maximizes safe, useful matching.

## Core Rules

### Understand What Segmentation Rules Do

Segmentation is governed by rules that define break points. Understand them before accepting defaults.

Most CAT tools segment by sentence-ending punctuation, with rules for abbreviations, numbering, and special cases. Default rules work for standard prose but fail on abbreviations that look like sentence ends, lists, headings, code, UI strings, and tabular content. Understand the tool's segmentation rule set, how to view the resulting segments, and how to override rules for specific cases. A translator or project lead who does not understand segmentation cannot diagnose why matches fail or why a segment is unworkable.

Review the segmentation of every new file type before translation begins, and adjust rules where defaults fail.

### Segment By Meaning, Not Just Punctuation

Good segmentation aligns with meaning. Segment by meaningful units, not just punctuation boundaries.

A sentence is usually a good segment, but not always. A list item, a heading, a table cell, or a UI string may be a better unit than the punctuation-based rule produces. Conversely, a punctuation rule may split a unit that should stay together, such as a sentence broken after an abbreviation. The goal is segments that are self-contained enough to translate without excessive context loss, but not so large that they become unworkable. Adjust segmentation rules to align with the content's structure, and use custom rules for project-specific patterns such as legal clause numbering or software keys.

Segmentation that ignores meaning produces fragments that are hard to translate and that match poorly.

### Handle Segmentation Exceptions Deliberately

Default segmentation rules produce exceptions that must be handled deliberately. Do not let them stand.

Common exceptions include abbreviations that trigger false breaks, such as "e.g." or "Inc."; numbering that looks like sentence ends; decimal points in locales that use them differently; and quotation marks or brackets that confuse break detection. Build exception lists for the project's abbreviations and patterns. Most tools allow custom segmentation rules to prevent breaks after specific strings. Handling exceptions prevents fragmented segments that destroy readability and matching.

A file full of "e.g."-induced breaks is a sign that exception handling was skipped.

### Preserve Consistent Segmentation Across Projects

Segmentation consistency across projects is what makes TM leverage work. Preserve it.

If the same content is segmented differently in different projects, the TM cannot match it, because matches are segment-based. Define segmentation rules per content type or client and apply them consistently. When adopting a new tool or version, check whether its default segmentation differs from the established rules, because a change can silently break matching across the TM. Document the segmentation configuration so it can be reproduced. Inconsistent segmentation is a leading cause of unexpectedly low match rates.

A segmentation change is a TM event: it should be planned, not discovered after the fact.

### Tune Fuzzy Match Thresholds For The Project

Fuzzy match thresholds determine which partial matches the tool proposes. Tune them for the project.

A low threshold proposes more matches, including distant ones that may be more hindrance than help, increasing review burden. A high threshold proposes fewer, more relevant matches, but may miss useful leverage. For content with high reuse potential, a moderate threshold surfaces useful fuzzy matches; for highly variable content, a higher threshold reduces noise. Consider the cost of reviewing a bad fuzzy match against the benefit of a good one, and set the threshold accordingly. Adjust thresholds per project, not just globally.

A threshold that is too low floods the translator with irrelevant suggestions; too high starves them of leverage.

### Evaluate Match Quality, Not Just Match Score

A high fuzzy match score does not guarantee a useful match. Evaluate quality, not just the percentage.

A 90% match may differ in a critical term, making it more dangerous than a 70% match that differs in punctuation. Review what changed between the match and the current source: a difference in a number, a negation, or a key term is high-impact; a difference in an article or punctuation is low-impact. Train translators to scan the difference, not just the score, and to reject matches that differ in high-impact ways. Tools that highlight differences help, but the judgment is the translator's.

Blindly editing a high-score match that differs in a critical term imports errors.

### Resolve Segmentation Mismatches That Reduce Leverage

When match rates are lower than expected, segmentation mismatch is a prime suspect. Investigate and resolve.

Compare the current file's segmentation with the TM's segmentation for similar content. If the same sentences are split differently, that is the mismatch. Resolve it by aligning segmentation rules, or by using alignment tools to re-segment and re-import the TM. For one-off mismatches, manual segment merging or splitting in the tool can recover the match. Document the resolution so it persists. Unresolved mismatches silently erode leverage and increase cost.

### Test Segmentation On Representative Files

Before committing a segmentation configuration, test it on representative files. Do not assume defaults work.

Run sample files through the segmentation rules and inspect the result: are sentences intact, are abbreviations handled, are lists and tables segmented sensibly, are UI strings preserved as units? Fix issues in the rules before the full project starts. Testing catches problems early, when they are cheap to fix, rather than mid-project when segments are already translated and re-segmentation disrupts work.

## Common Traps

### Accepting Default Segmentation Without Review

Defaults fail on abbreviations, lists, code, and UI strings; unreviewed segmentation produces fragments and poor matches.

### Segmenting By Punctuation Alone

Punctuation boundaries do not always align with meaning; segment by meaningful units.

### Inconsistent Segmentation Across Projects

Different segmentation for the same content destroys TM matching and erodes leverage.

### Fuzzy Threshold Too Low Or Too High

Too low floods with noise; too high starves of leverage; tune per project.

### Trusting Match Score Over Match Quality

A high-score match differing in a critical term is dangerous; evaluate the difference, not the percentage.

### Ignoring Segmentation Mismatch As A Cause Of Low Leverage

Low match rates are often segmentation problems, not poor TM; investigate before assuming.

### Changing Tools Or Versions Without Checking Segmentation

A new tool's default segmentation can silently break matching across the existing TM.

### Not Testing Segmentation Before The Project

Discovering segmentation problems mid-project disrupts work and is expensive to fix.

## Self-Check

Before finalizing segmentation and matching configuration for a project, verify:

- The segmentation rule set is understood, and defaults are reviewed rather than accepted blindly.
- Segmentation aligns with meaning, segmenting by meaningful units and adjusting rules for headings, lists, tables, and UI strings.
- Exceptions such as abbreviations, numbering, and decimal points are handled with custom rules or exception lists.
- Segmentation is consistent across projects and content types, with the configuration documented for reproduction.
- Fuzzy match thresholds are tuned for the project, balancing review burden against leverage.
- Match quality is evaluated by the difference, not just the score, with high-impact differences flagged for rejection.
- Unexpectedly low match rates are investigated for segmentation mismatch before assuming poor TM.
- Segmentation is tested on representative files before the project begins, with issues fixed in the rules.
- No default segmentation stands that fragments meaning or breaks matching for the content type.
- The configuration maximizes safe, useful matching without flooding the translator with noise or starving them of leverage.
