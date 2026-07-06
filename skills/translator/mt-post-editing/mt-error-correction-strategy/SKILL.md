---
name: mt_error_correction_strategy.md
description: Use when the agent is correcting specific machine translation error types such as hallucinations, omissions, terminology drift, negation flips, number and entity errors, and structural or concatenation defects, deciding how deeply to fix an MT segment, triaging which errors to fix first under a post-editing level, or choosing between fixing, flagging, and retranslating an MT output.
---

# MT Error Correction Strategy

Machine translation produces a characteristic set of errors that differ from human translation errors, and correcting them well is a distinct craft. A human translator's mistakes are usually local and stylistic; an MT's mistakes are often semantic and structural, and they hide inside fluent text. Hallucinations invent content that is not in the source. Omissions drop content silently. Terminology drift renders the same term three ways across a document. Negation flips reverse the meaning of a sentence while keeping it grammatical. Number and entity errors mistranslate the very values a reader relies on. Structural and concatenation errors assemble fragments into sentences that do not parse. Each error type demands a different correction move, a different depth of intervention, and a different decision about whether to fix, flag, or retranslate. Agents miss this because they treat all MT errors as "fix the wrong word," applying a single shallow correction strategy to defects that require structural surgery, and because they correct segment by segment without recording decisions, leaving the document inconsistent.

The harm this skill prevents is twofold: shallow corrections that leave the real defect in place, and over-corrections that rewrite segments to fix one error while introducing others. The agent's freedom is to choose the correction depth per error type, bounded by the post-editing level and by the rule that meaning errors are always fixed completely before any stylistic work.

## Core Rules

### Triage By Severity: Meaning Errors Before Style, Always

Before touching any segment, classify what is wrong and fix in severity order. Meaning errors, anything that changes what the text says, come first and are fixed completely regardless of post-editing level: mistranslations, omissions, additions, hallucinations, negation flips, and wrong numbers, dates, units, names, and entities. Accuracy errors come next: terminology that must match the termbase. Fluency and style errors come last and are fixed only under full post-editing. Triage exists because fixing style first can mask a meaning error: an editor who smooths a sentence before checking whether it conveys the source may polish a wrong meaning into a confident wrong sentence. Establish the meaning first, then improve the surface.

### Match The Correction Move To The Error Type

Each MT error type has a characteristic correction move, and using the wrong move leaves the defect in place. Hallucinations require deletion of invented content and verification that nothing real was displaced; never edit a hallucination into a plausible sentence, because that legitimizes invented content. Omissions require inserting the missing source content, which often means restructuring the target sentence to accommodate it. Additions require deleting the extra content and checking that the deletion does not break agreement. Terminology drift requires replacing MT variants with the approved termbase term across the whole document, not just the current segment. Negation flips require restoring the correct polarity and then re-reading the sentence to confirm the surrounding logic still holds, because a flipped negation often corrupts a chain of reasoning. Number and entity errors require copying the exact value from the source, not approximating, and checking units and formatting conventions for the locale. Match the move to the type, and resist the urge to apply a generic word-swap to every defect.

### Decide Fix, Flag, Or Retranslate Per Segment

Not every defective segment should be fixed in place. For each problematic segment, choose among three actions. Fix when the error is local and the MT structure is sound: a wrong term, a flipped negation, a missing clause. Flag when the defect originates upstream: a source that is ambiguous or broken, a concatenation that assembles incorrectly, a placeholder that the engine mishandles, or content that should not have been machine-translated at all. Retranslate when the MT output is so far from the source that fixing it piecemeal costs more than starting over and risks leaving residual errors. The decision should be explicit and recorded, because silently retranslating hides engine problems and silently fixing hides source problems. When you retranslate, note the segment so it becomes training signal; when you flag, route the flag to whoever owns the upstream cause.

### Preserve What MT Got Right While Correcting What It Got Wrong

The efficiency logic of post-editing depends on changing only what is wrong. When correcting an error, touch the minimum span that fixes it, and leave correct MT untouched even when you would have phrased it differently. Over-correction is the twin failure of under-correction: an editor who rewrites a segment to fix one terminology error may break agreement, shift register, or damage markup. Before saving a correction, re-read the whole segment to confirm the fix did not introduce a new error and that the surrounding text still coheres. The discipline is to edit surgically and verify globally.

### Handle Structural And Concatenation Errors At The Assembly Level

Some MT errors are not in any word but in how fragments are assembled. Concatenated UI strings may agree incorrectly at the join; placeholders may be moved or altered; long sentences may be parsed into structures that do not hold. These require correction at the assembly level, not word substitution. Read the assembled result, identify where the structure breaks, and either restructure the target or flag the segment for engineering if the source concatenation is the root cause. Structural errors are the easiest to miss because each fragment looks fine in isolation; always check the joined output, especially for UI, software strings, and templated content.

### Record Terminology And Rendering Decisions For Consistency

MT does not self-correct across segments, so the editor is the only force imposing consistency. When you decide how to render an ambiguous term, a name, or a recurring phrase, record the decision immediately and apply it to every later occurrence. Use the termbase or a running decision log, and at the end of the job sweep for any earlier occurrences that now contradict the decision. Without this, the document ships with the same term rendered multiple ways, which is the defect readers notice first and the one most damaging to perceived quality.

### Escalate Recurrent Error Patterns To The Engine

When the same error type recurs across many segments, the problem is the engine, not the segments. Track patterns: repeated hallucinations on a content type, systematic terminology drift in a domain, consistent mishandling of a grammatical construction. Aggregate these and feed them to the engine or localization team as adaptation or training signal. Correcting the same defect forever without escalation guarantees it returns in the next job. Correction strategy includes knowing when to stop correcting individually and start correcting systemically.

### Re-Read Corrected Segments In Context

A correction that is right in isolation can be wrong in context. After fixing a segment, read it with the segments before and after to confirm coherence: does the corrected sentence still link logically, does terminology still match neighbors, does register still fit, did the fix shift a pronoun reference or an agreement. Context re-reading catches the second-order errors that segment-level correction creates. For high-stakes content, re-read the whole section after a cluster of corrections, because dense edits can destabilize a passage.

## Common Traps

### Polishing A Hallucination Instead Of Deleting It

Editing invented content into a plausible sentence legitimizes text that should not exist. Hallucinations are deleted and verified, not smoothed.

### Fixing Style Before Confirming Meaning

Smoothing a sentence before checking its accuracy can cement a wrong meaning into confident prose. Confirm meaning first, every time.

### Applying A Word-Swap To A Structural Error

Structural and concatenation defects need assembly-level correction, not word substitution; swapping words leaves the broken structure in place.

### Over-Correcting And Introducing New Errors

Rewriting a segment to fix one defect can break agreement, register, or markup. Edit the minimum span and re-read the whole segment after.

### Correcting Terminology Segment By Segment Without A Decision Log

Without recording rendering decisions, the same term ends up translated multiple ways. Consistency requires a log and a final sweep.

### Silently Retranslating Instead Of Flagging

Silent retranslation hides engine weakness and prevents improvement. When MT is unfixable, retranslate and record the segment as signal.

### Fixing The Same Defect Forever Without Escalation

Recurrent error patterns are engine problems. Correcting them individually forever guarantees recurrence; escalate to systemic correction.

### Trusting The Corrected Segment In Isolation

A correct-looking fix can break context: pronoun reference, agreement, logical link. Re-read corrected segments with their neighbors.

## Self-Check

- Were errors triaged by severity, with all meaning errors, mistranslations, omissions, additions, hallucinations, negation, numbers, and entities, fixed completely before any stylistic work?
- Was the correction move matched to the error type, with hallucinations deleted rather than polished, omissions inserted with restructuring, and negation flips followed by a logic re-read?
- For each problematic segment, was an explicit fix, flag, or retranslate decision made and recorded, with flags routed to the upstream owner?
- Did corrections touch the minimum span needed, preserving correct MT and avoiding over-correction that introduces new errors?
- Were structural and concatenation errors checked at the assembly level, with the joined output verified rather than each fragment in isolation?
- Are terminology and rendering decisions recorded in a termbase or log and applied to all occurrences, with a final sweep for earlier contradictions?
- Are recurrent error patterns aggregated and escalated to the engine or localization team as adaptation or training signal, rather than corrected indefinitely?
- Were corrected segments re-read in context to confirm coherence, pronoun reference, agreement, register, and logical links?
- No hallucination was legitimized by editing, and no meaning error was deferred to save time under any post-editing level.
- The correction strategy distinguished local fixes from upstream flags and from full retranslations, and recorded each decision for feedback.
