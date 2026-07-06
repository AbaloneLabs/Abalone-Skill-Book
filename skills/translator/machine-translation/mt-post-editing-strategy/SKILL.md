---
name: mt_post_editing_strategy.md
description: Use when the agent is post-editing machine translation output, choosing between light and full post-editing, applying post-editing guidelines efficiently, fixing MT errors without retranslating, or managing the tension between speed consistency and quality in post-editing workflows.
---

# MT Post-Editing Strategy

Post-editing is the human correction of machine translation output, and it is a distinct skill from both translation and revision. The post-editor works with text they did not write, under instructions that may limit what they can change, against a quality target that may be lower than full translation quality. The central tension is efficiency: post-editing exists to be faster and cheaper than translation, so retranslating every segment defeats the purpose, while accepting MT errors defeats the quality target. Skilled post-editing fixes the errors that matter, leaves acceptable MT output alone even when it is not how the editor would have translated, and applies consistent judgment across a document. Unskilled post-editing either over-edits, rewriting good MT and wasting time, or under-edits, passing through errors that the quality target required fixing. The strategy must be chosen before editing begins, because light and full post-editing are different jobs with different rules.

Use this skill when post-editing machine translation, choosing between light and full post-editing, applying post-editing guidelines, or managing post-editing workflows. The goal is to correct MT output efficiently to the required quality level, neither over-editing nor under-editing.

## Core Rules

### Choose The Post-Editing Level Before Starting

Post-editing has at least two distinct levels with different rules. Choose the level before editing, because mixing them wastes effort and produces inconsistent results.

Light post-editing aims for understandable, accurate-enough text, fixing only errors that impede comprehension. It accepts imperfect style, awkward phrasing, and non-idomatic expression, as long as the meaning is clear. It suits internal, informational, or gisting content. Full post-editing aims for publication-quality text comparable to human translation, fixing all errors and improving style, fluency, and terminology. It suits external-facing, professional, or branded content. The level determines what you fix and what you leave.

Confirm the level with the requester, and apply it consistently. Do not drift into full post-editing on a light job, or vice versa.

### Fix Errors, Do Not Retranslate

The post-editor's job is to correct MT output, not to produce their own translation. Over-editing defeats the efficiency purpose.

Fix the errors that the chosen level requires: meaning errors in light post-editing, plus style and fluency errors in full post-editing. Leave acceptable MT output alone, even when you would have translated it differently. Resist the urge to improve phrasing that is already adequate for the target level. The measure of good post-editing is not how much you change but how little you need to change to reach the target quality.

If you find yourself rewriting most segments, either the MT quality is too low for post-editing or you are over-editing. Flag the former; correct the latter.

### Apply Consistent Judgment Across The Document

Consistency is harder in post-editing than in translation, because the post-editor works with MT output that varies. Apply consistent judgment.

Use the termbase consistently, correcting MT terminology drift to the approved terms. Apply style and register consistently, so the document does not shift tone because MT shifted. When you make a decision about an ambiguous term or rendering, record it and apply it to later occurrences. Consistency makes the output feel unified despite originating from MT.

Inconsistency is the most visible sign of unskilled post-editing, because readers notice shifting terminology and register.

### Prioritize Meaning Errors Over Style

In both light and full post-editing, meaning errors take priority. Fix them first and completely.

Meaning errors include mistranslations, omissions, additions, hallucinations, negation errors, and number or entity errors. These change what the text says and must be fixed in any post-editing level. Style and fluency issues, awkward phrasing, non-idomatic expression, and register inconsistency, are fixed in full post-editing but tolerated in light post-editing. Never pass a meaning error to reach the target faster, because meaning errors are the defects that cause harm and complaints.

A meaning error in published content causes more damage than any number of style imperfections.

### Manage Terminology With The Termbase

MT often produces terminology inconsistency, rendering the same term differently across segments. Manage terminology with the termbase.

Apply approved terms throughout, correcting MT variants. Where MT uses a term not in the termbase, decide whether to accept it, add it as a candidate, or replace it with the approved term. Track terminology decisions so the document stays consistent and so future MT training or post-editing benefits. Terminology consistency is often the post-editor's largest task, because MT does not self-correct across segments.

Without termbase management, post-edited documents retain MT's terminology chaos.

### Handle MT Structural And Concatenation Errors

MT can produce structural errors, especially in UI strings and concatenated text, that require more than word-level fixes.

Watch for sentences that do not parse because MT assembled fragments incorrectly. Watch for placeholder and variable errors where MT moved or altered markup. Watch for agreement errors across concatenated segments. These require structural correction, not just word substitution, and may need flagging to engineering if the source structure is the problem. Fix what you can in the target, and flag what requires source or engineering changes.

Structural errors are easy to miss because each piece looks fine; check the assembled result.

### Preserve What MT Got Right

A common over-editing trap is changing MT output that was correct, simply because it differs from how the editor would translate. Preserve what MT got right.

If the MT rendering is accurate, adequate in style for the target level, and consistent, leave it. Changing it wastes time, risks introducing errors, and undermines the consistency benefit of starting from MT. The post-editor's discipline is knowing when not to edit.

This is especially important in light post-editing, where the goal is to change as little as possible while reaching understandability.

### Track Effort And Flag Low-Quality MT

Post-editing effort is a signal. Track it and use it to flag where MT is not working.

If certain segments, domains, or language pairs require near-full retranslation in post-editing, the MT quality is too low for efficient post-editing there. Flag these to the MT or localization managers, because they may indicate a need for engine tuning, domain adaptation, or reverting to human translation for that content. Post-editing metrics, such as edit distance and time per segment, reveal where MT adds value and where it does not.

Silently absorbing the cost of bad MT hides problems and prevents improvement.

### Coordinate With Reviewers On The Quality Target

Post-edited content may still be reviewed. Coordinate the quality target so reviewers evaluate against the right standard.

If the target was light post-editing, reviewers should not flag style issues that light post-editing accepts. If the target was full post-editing, reviewers apply publication standards. Misaligned expectations cause reviewers to demand full-quality changes on a light job, wasting effort, or to accept light-quality output on a full job, under-delivering. State the target clearly in the handoff.

## Common Traps

### Mixing Light And Full Post-Editing

Inconsistent levels produce a document that is over-edited in places and under-edited in others.

### Over-Editing Acceptable MT

Rewriting correct MT wastes time and can introduce errors; post-editing rewards restraint.

### Passing Meaning Errors For Speed

Meaning errors cause harm and complaints; they must be fixed regardless of the level.

### Ignoring Terminology Drift

MT renders terms inconsistently; without termbase management, the document stays chaotic.

### Missing Structural And Concatenation Errors

Word-level fixes miss assembly errors; check the result, not just the words.

### Changing MT That Was Already Correct

Editing for preference rather than error wastes effort and risks introducing problems.

### Silently Absorbing Bad MT Effort

Not flagging low-quality MT hides problems and prevents engine or workflow improvement.

### Misaligning Reviewer And Post-Editor Targets

Reviewers demanding full quality on light jobs, or accepting light quality on full jobs, wastes effort or under-delivers.

## Self-Check

Before approving post-edited content, verify:

- The post-editing level, light or full, was chosen before editing and applied consistently throughout.
- Editing corrected errors rather than retranslating, with acceptable MT output left alone.
- Judgment was consistent across the document, with terminology, style, and decisions tracked and applied uniformly.
- Meaning errors, mistranslations, omissions, additions, hallucinations, negation, and numbers, were fixed completely before any style work.
- Terminology was managed with the termbase, correcting MT drift and recording decisions.
- Structural, concatenation, placeholder, and agreement errors were checked in the assembled result, not just word by word.
- MT output that was correct was preserved, not changed for preference.
- Effort was tracked, and low-quality MT segments, domains, or pairs were flagged for engine or workflow review.
- The quality target was communicated to reviewers so evaluation matches the post-editing level.
- No meaning error was passed to save time, and no acceptable MT was rewritten to satisfy preference.
