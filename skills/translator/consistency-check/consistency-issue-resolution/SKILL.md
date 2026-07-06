---
name: consistency_issue_resolution.md
description: Use when the agent is resolving detected terminology or style consistency issues in translated content, deciding between competing renderings, handling conflicts between termbase style guide and translation memory, applying corrections without introducing new inconsistency, or determining when a consistency finding should update the authority rather than the text.
---

# Consistency Issue Resolution

Detecting a consistency issue is only the first half of the work; the second, harder half is resolving it correctly. Resolution means deciding which rendering wins, applying the correction without creating new inconsistency, and recognizing when the right outcome is not to change the text but to change the authority. These are distinct skills from detection. A reviewer who detects well but resolves poorly can do real damage: picking the wrong rendering as the winner propagates an error across the content, a correction applied in one place but not its neighbors creates fresh drift, and a finding fixed in the text when it should have been fixed in the termbase guarantees the problem recurs on the next project. The harm this skill prevents is the common outcome where consistency work makes things worse, where corrections introduce new contradictions, where the wrong authority wins, or where a systemic issue is patched locally instead of fed back to governance.

Agents miss this work because once an issue is flagged, applying the obvious fix feels like completion. It is not. Resolution is a decision with consequences across the content and across future work, and it deserves the same discipline as detection.

## Core Rules

### Resolve Against Authorities And Precedence, Not Preference

Every resolution should be governed by the established authorities and their precedence, not by the reviewer's preference for how the text should read. When a term is rendered inconsistently, the termbase's preferred equivalent wins. When translation memory contradicts the termbase, the termbase wins, because memory may carry deprecated terms. When the style guide conflicts with reviewer taste, the style guide wins. Document the precedence before resolving so decisions are defensible and repeatable.

Resolving by preference produces decisions that two reviewers would make differently, which itself is a form of inconsistency. Resolve by authority, and record the authority cited for each non-obvious decision.

### Decide Whether To Fix The Text Or The Authority

Before correcting the text, ask whether the issue is a text defect or an authority defect. If the text deviates from a correct, current authority, fix the text. If the text reveals that the authority itself is wrong, outdated, or incomplete, the resolution is to update the authority, not to patch the text against a broken rule. A termbase entry that prescribes the wrong equivalent, or a style rule that no longer reflects current usage, will cause the same issue on every future project until the authority is fixed.

When the authority is the defect, fix the authority through governance, then apply the corrected rule to the text. Fixing only the text leaves the broken authority in place to cause recurrence.

### Apply Corrections Globally, Not Locally

A consistency correction applied in one place but not in equivalent places creates new drift. Once a rendering is chosen as the winner, find every occurrence of the losing renderings and apply the correction across all of them. Use concordance or search to locate all instances, including inflected forms, plurals, and partial-string matches, and correct them together.

Local correction is the most common way consistency work introduces new inconsistency. Treat the correction as a change to the whole content, not to the segment where the issue was flagged.

### Check That The Correction Fits Every Context

Global application is necessary but not sufficient. Before applying a rendering everywhere, verify it fits every context where the losing rendering appeared. A term that is correct as a noun may be wrong where the source uses it as a verb. An equivalent that fits formal register may be wrong in casual UI copy. A correction that is right in documentation may clash with the interface label users see.

Apply the winner globally only where it fits; where context differs, apply the context-appropriate admitted alternative or flag the case for a deliberate decision. Mechanical global replacement without context checking trades one inconsistency for a set of correctness errors.

### Handle Competing Renderings By Investigating, Not Voting

When two or more renderings compete and neither clearly matches the authority, investigate rather than pick the most frequent or most recent. Determine which rendering has stronger provenance: which aligns with authoritative domain sources, which was approved by the terminologist or subject-matter expert, which appears in reference content. Frequency and recency are weak signals; a wrong rendering can be frequent because it was propagated by machine translation or copied across files.

Record the investigation and the basis for the winner, and feed the losing renderings to governance as deprecated or forbidden so they do not recur.

### Recognize When Variation Is Legitimate

Not every detected difference is an issue to resolve. Variation is legitimate when the style guide allows alternatives, when syntax or register requires a different form, or when the source itself uses different terms for different concepts. Resolving legitimate variation by forcing uniformity produces text that is consistent but wrong or unnatural.

The test is whether the variation is governed and bounded. If the style guide permits it and it serves clarity or naturalness, document it as legitimate and do not correct. If it is ungoverned or unbounded, resolve it.

### Coordinate Resolutions Across Contributors And Files

When multiple translators or files are involved, a resolution made by one must propagate to all, or the deliverable splits. Share resolution decisions through a living decisions log, and ensure every contributor applies the same winner to the same term. Assign a lead reviewer to own resolution decisions for the whole deliverable, so that competing renderings are resolved once, consistently, rather than differently in each file.

Uncoordinated resolution means each contributor picks a different winner, and the consistency audit's findings are undone by inconsistent fixes.

### Verify The Resolution Did Not Introduce New Issues

After applying corrections, re-check the affected segments and their neighbors. A correction can break syntax, change agreement, disrupt a placeholder, or clash with a surrounding term. Run a verification pass on corrected segments, not only on the corrected term, to confirm the sentence still holds together. Also re-run the consistency checks to confirm the deviation is gone and no new deviation was introduced.

Resolution is not complete until the corrected text is verified to be both consistent and correct.

## Common Traps

### Resolving By Preference Instead Of Authority

Decisions based on reviewer taste are non-repeatable and themselves inconsistent. Resolve against authorities and documented precedence.

### Fixing The Text When The Authority Is Broken

Patching text against a wrong or outdated termbase or style rule guarantees recurrence. Fix the authority through governance when it is the defect.

### Local Correction Creating New Drift

Fixing only the flagged segment leaves equivalent occurrences unchanged. Apply corrections globally across all instances.

### Mechanical Global Replacement Ignoring Context

Applying one rendering everywhere without checking syntax, register, and part of speech trades inconsistency for correctness errors. Verify fit per context.

### Picking The Most Frequent Or Recent Rendering

Frequency and recency are weak signals; a wrong rendering can be frequent from propagation. Investigate provenance and authority.

### Forcing Uniformity On Legitimate Variation

Resolving governed, bounded variation as if it were error produces consistent but unnatural text. Recognize and document legitimate variation.

### Uncoordinated Resolution Across Contributors

When each translator picks a different winner, audit findings are undone. Coordinate resolutions through a decisions log and a lead reviewer.

### Declaring Resolution Done Without Verification

A correction can break syntax or introduce new deviations. Re-check corrected segments and re-run consistency checks before closing the issue.

## Self-Check

Before closing consistency issues as resolved, verify:

- Each resolution was decided against the established authorities and precedence, with the authority cited for non-obvious decisions, not by reviewer preference.
- For each issue, you determined whether the defect was in the text or in the authority, and authority defects were routed to governance rather than patched locally.
- Corrections were applied globally to all occurrences of the losing renderings, including inflected and partial-string forms, not only to the flagged segment.
- The winning rendering was verified to fit every context where it was applied, with context-appropriate alternatives used where syntax, register, or part of speech differed.
- Competing renderings were resolved by investigating provenance and authority, not by frequency or recency, and losers were fed to governance as deprecated or forbidden.
- Legitimate variation was recognized and documented rather than forced into uniformity.
- Resolution decisions were coordinated across all contributors and files through a decisions log and a lead reviewer, so the same winner was applied everywhere.
- Corrected segments and their neighbors were re-checked, and consistency checks were re-run, confirming the deviation is gone and no new deviation was introduced.
- No resolution propagated a wrong rendering as the winner, and no authority defect was left in place to cause recurrence.
- The resolved content is both consistent and correct, not merely uniform.
