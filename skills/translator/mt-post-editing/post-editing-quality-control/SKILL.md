---
name: post_editing_quality_control.md
description: Use when the agent is checking post-edited machine translation for completeness, running QA on post-edited output, verifying that post-editing met its target level, reviewing post-editor performance, managing consistency across a post-edited document, or designing a QA pass for a post-editing workflow.
---

# Post-Editing Quality Control

Post-editing quality control is the verification layer that sits between the editor and delivery, and it is harder than reviewing human translation because the defects are different. A human translator's errors cluster around effort and fatigue: missed nuances, inconsistent terminology, rushed phrasing. A post-editor's errors cluster around restraint and vigilance: over-editing that introduces new mistakes into correct MT, under-editing that passes through MT-specific errors such as hallucinations and omissions, and inconsistency that arises because the editor is wrestling with MT output that varies from segment to segment. Quality control for post-editing must therefore check three things at once: that the meaning errors the level required fixing are actually fixed, that the editor did not damage what MT got right, and that the document reads as a unified whole rather than a patchwork of MT and human voice. Agents miss these checks because they treat post-editing QA like ordinary revision and rely on target-only reading, which is exactly the method that lets MT's most dangerous errors through.

The harm this skill prevents is the delivery of post-edited content that looks checked but still contains meaning errors, introduced errors, or jarring inconsistency. The agent's freedom is to design and run a QA process appropriate to the content's risk, but the process must always include source comparison and consistency verification, never target-only skim-reading.

## Core Rules

### Verify Against The Agreed Post-Editing Level

QA is meaningless without a stated level. Before checking, confirm what level the job was edited to, because the defects you flag depend on it. For light post-editing, you verify that meaning errors were fixed and you do not flag stylistic awkwardness that the level tolerates; flagging style on a light job wastes effort and demoralizes the editor. For full post-editing, you verify meaning and style, and you flag non-idiomatic phrasing, register drift, and fluency defects. The most common QA failure is a reviewer applying full-post-editing standards to a light job or vice versa, which produces either over-correction or under-delivery. Carry the level into every check and reconcile any mismatch between editor and reviewer expectations before scoring.

### Always Compare Target Against Source

The single most important rule is that meaning errors are invisible in a target-only read. Modern MT produces fluent text even when the meaning is wrong, so a target that reads beautifully can still omit a clause, hallucinate a sentence, flip a negation, or mistranslate a number. QA must compare target against source segment by segment for the sample it checks, looking for additions, omissions, and meaning shifts. Reserve target-only reading for fluency and style checks, which are legitimately target-side, but never let a target-only read substitute for source comparison on meaning. For high-stakes content, full source comparison is required, not sampling.

### Check For Introduced Errors, Not Just Residual Ones

Post-editing can introduce errors that were not in the MT. An editor who rewrites a correct segment to match their preference can break terminology, agreement, or placeholder markup; an editor who moves a clause can break a number agreement or a logical link; an editor who "improves" phrasing can change register. QA must look for damage the editor did, not only for what they left unfixed. Compare the edited target against the raw MT where available to see what changed, and scrutinize heavy edits for new errors. A high edit rate is not a sign of thoroughness; it is a signal to check whether the editor over-edited and introduced problems.

### Enforce Terminology And Consistency Across The Document

MT renders the same term differently across segments, and a post-editor who fixes each segment in isolation leaves the document inconsistent. QA must check terminology consistency across the whole job, verifying that approved termbase terms are used everywhere and that the editor did not leave MT variants in some segments while correcting them in others. Check register and formality consistency, because MT drifts in address level and an editor who does not impose a consistent voice produces text that shifts tone. Check decisions about ambiguous terms: if the editor chose one rendering early, later occurrences must follow. Consistency is the defect readers notice first, and it is the one most likely to survive a segment-by-segment edit that never looks across segments.

### Sample Strategically, Not Randomly

Random sampling misses the segments where MT and post-editing fail most. Stratify the sample to cover high-risk content: segments with numbers, dates, units, names, and entities; segments with negation and quantifiers; long and complex segments where MT structural errors concentrate; the first and last segments of sections, where omissions are common; and segments the editor flagged or spent unusually long on. Also sample segments with very low edit distance, because unchanged MT may be correct or may be an error the editor missed. For high-stakes content, raise the sample toward full review. Document the sampling method and rate so the QA result is defensible.

### Use Automated Checks As A First Pass, Not A Verdict

Automated QA tools catch a class of defects reliably and cheaply: terminology violations against the termbase, doubled words, inconsistent rendering of the same source segment, number and tag mismatches between source and target, and formatting deviations. Run them first to clear the mechanical layer, then apply human judgment to the semantic layer they cannot reach: meaning accuracy, register, idiom, and coherence. Never treat a clean automated check as proof of quality, because automated checks pass fluent wrong text and miss hallucinations entirely. Conversely, do not ignore automated findings; each one is either a real defect or a false positive worth suppressing in the rules.

### Verify Structural Integrity And Markup

Post-editing happens inside files that carry structure: tags, placeholders, variables, markup, and concatenation logic. QA must verify that the edited target preserves this structure exactly: that tags are intact and correctly placed, that placeholders were not translated or moved, that variables resolve correctly in context, and that concatenated segments agree where they join. Structural errors are easy to miss because each fragment looks fine in isolation; check the assembled result. For UI strings and software content, verify length limits, keyboard shortcut conventions, and expansion or contraction relative to the source.

### Capture QA Findings As Engine And Workflow Feedback

QA is not only a gate; it is a signal generator. Record the error types, severities, and segments where post-editing failed, and feed them back to two audiences: the engine team, to identify where MT is weak and needs adaptation or training; and the editor or vendor, to target coaching and to recalibrate the level if effort is consistently misapplied. Aggregate findings over time reveal patterns: a recurring hallucination type, a terminology domain the engine mishandles, or an editor who systematically over-edits. Without feedback capture, QA becomes a cost center that finds the same defects forever.

## Common Traps

### Reviewing Target-Only And Missing Meaning Errors

A target-only read passes fluent wrong text, which is MT's signature defect. Meaning QA requires source comparison; there is no shortcut.

### Applying The Wrong Level During QA

Reviewers who do not know the level flag style on light jobs or accept meaning-adjacent defects on full jobs. The level must travel with the job into QA.

### Assuming Heavy Editing Means Thorough Editing

High edit distance often means over-editing that introduced new errors. Check what changed, and treat heavy edits as risk points, not quality signals.

### Checking Segments In Isolation And Missing Inconsistency

Segment-level QA passes every segment yet still delivers a document with shifting terminology and register. Consistency requires cross-segment checks.

### Trusting A Clean Automated Check

Automated QA catches mechanical defects and misses semantic ones. A clean automated report does not certify meaning accuracy.

### Random Sampling That Misses High-Risk Segments

Random samples under-represent the segments where errors concentrate: numbers, negation, entities, long sentences, and flagged segments. Stratify toward risk.

### Ignoring Structural And Markup Integrity

Tags, placeholders, and variables can be damaged by editing. Verify structure in the assembled output, not just in the edited words.

### Treating QA As A Gate Rather Than A Feedback Source

QA that only approves or rejects finds the same defects repeatedly. Capture and route findings to engine and editor improvement.

## Self-Check

- Is the post-editing level confirmed before QA, and are defects flagged against the standard that level sets rather than an abstract quality ideal?
- Does the meaning check compare target against source segment by segment, with full comparison for high-stakes content rather than target-only reading?
- Did the QA check for errors the editor introduced, by comparing edited output against raw MT where available, especially on heavily edited segments?
- Is terminology verified for consistency across the whole document, with approved termbase terms enforced and MT variants corrected everywhere?
- Are register, formality, and ambiguous-term decisions consistent across segments, with no shifting voice or tone?
- Is the sample stratified toward high-risk segments: numbers, dates, units, names, negation, long sentences, flagged segments, and low-edit-distance segments?
- Were automated checks run first for mechanical defects, with their findings resolved or suppressed, and without treating a clean automated pass as proof of semantic quality?
- Is structural integrity verified: tags intact, placeholders unaltered, variables resolving, concatenated segments agreeing, and length and formatting constraints met?
- Are QA findings recorded by type, severity, and segment, and routed as feedback to the engine team and the editor or vendor?
- No meaning error was passed because the target read fluently, and no editor-introduced error was missed because the edit looked thorough.
