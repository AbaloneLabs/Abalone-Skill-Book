---
name: source_error_handling.md
description: Use when the agent is deciding how to handle detected source errors during translation, choosing among translate-as-is silent correction noted correction and query per error, governing correction authority and limits, or applying a consistent decision framework so that source flaws are neither blindly propagated nor unauthorizedly rewritten.
---

# Source Error Handling

Once source defects are detected, the translator faces a decision for each one, and the decision is not obvious. A typo can be corrected silently because translating it would introduce an error no one intended. A wrong dose cannot be corrected silently because the translator may have misunderstood the intent, and a silent correction could mask a real safety issue. A terminology inconsistency can be harmonized, but harmonizing defined terms in a legal document could change the legal meaning. Between these extremes lies a continuum of errors, each demanding a deliberate choice among translate as-is, correct silently, correct with a note, or query. Source error handling is the discipline of making that choice per defect, governed by a consistent framework, so that the translator neither blindly propagates source flaws into every target language nor silently rewrites the source beyond their authority. Both extremes fail: propagation turns the translator into an unknowing conduit for errors, and silent rewriting turns the translator into an unauthorized editor.

Agents mishandle source errors because the choices feel like common sense in the moment, and common sense is inconsistent. The same translator who silently corrects a typo on Monday may query an identical typo on Wednesday, or silently rewrite a factual error on Friday because fixing it felt helpful. Inconsistency is itself a defect, because it makes the target unpredictable and removes the evidence the requester needs to understand what was changed. Handling decisions must be governed by a framework that classifies each error by type and severity and assigns a default response, so that the translator applies consistent judgment and records the decisions for transparency. The framework does not eliminate judgment; it disciplines it.

Use this skill when deciding how to handle detected source errors, when governing correction authority, when a source contains errors that must be propagated corrected or queried, or when establishing a consistent handling framework for a document. The goal is to ensure each source error receives an appropriate, recorded handling decision rather than an ad-hoc reaction.

## Core Rules

### Classify Each Error By Type And Severity Before Handling

Handling decisions cannot be made without classification, because the right response depends on what kind of error it is and how much it matters. Classify before deciding.

Classify by type: typographical, grammatical, factual, terminological, structural, OCR or encoding, or mixed-language. Classify by severity: low, where the error is cosmetic or trivial; medium, where it affects clarity or consistency but not stakes; high, where it affects meaning, fact, safety, legal obligation, or compliance; critical, where it could cause harm if propagated. The type-severity pair drives the handling decision. A low-severity typo gets one response; a high-severity factual error in a medical text gets another. Classification is the input to handling, and skipping it produces inconsistent ad-hoc choices.

### Apply A Default Handling Rule Per Class

With classification done, apply a consistent default handling rule, so that similar errors receive similar treatment across the document.

Low-severity, unambiguous errors such as obvious typos in non-critical text can typically be corrected silently, because translating the typo would introduce an unintended error. Medium-severity errors or ambiguous cases should be corrected with a translator note, so the requester sees what was changed and why. High-severity errors, especially factual errors in safety, legal, medical, or financial content, must be queried rather than silently corrected, because the translator may have misunderstood the intent and a silent correction could mask a real problem. Critical errors must halt and escalate. The default rule gives consistency; deviations from the default should be deliberate and noted, not accidental.

### Never Silently Correct Where Correction Changes Meaning

The boundary that must never be crossed silently is the one where correction changes meaning, obligation, or fact. Silent correction beyond this line exceeds the translator's authority.

A typo in a casual sentence changes nothing of substance; correcting it is safe. A wrong number in a dose, a missing condition in a clause, or a contradicted fact changes what the text means or obligates; correcting it silently imposes the translator's interpretation on the source, and the interpretation may be wrong. Where correction changes meaning, the error must be queried or flagged, never absorbed. The principle is that the translator renders the source; they do not author it. Authoring decisions, meaning-changing corrections, belong to the source owner, and routing them back preserves the integrity of both the source and the translation.

### Govern Correction Authority Explicitly

The degree of correction freedom the translator has is not fixed; it depends on the engagement, and it should be agreed rather than assumed. Establish the authority boundary upfront.

Some engagements grant the translator latitude to clean up source errors as part of the work; others require that every deviation be queried. Determine the granted authority before handling begins, by asking the requester how source errors should be treated. Where authority is unclear, default to the conservative end: note rather than silently correct, and query rather than assume. An engagement with no stated authority defaults to the translator's least-risk option, which is transparency. Documenting the granted authority protects the translator from later accusations of unauthorized editing and gives the requester the chance to grant more freedom if they want it.

### Handle Terminology Inconsistency Deliberately

Terminology inconsistency is the most common source defect and the one most prone to mishandling, because each occurrence reads fine in isolation. Handle it as a system, not per occurrence.

Identify the intended term, usually the one used most frequently or the one matching domain convention, and apply it consistently in the target. Record the decision in a terminology log so it is visible and repeatable. Where the source inconsistency might be intentional, such as a legal document using defined terms with precise distinctions, preserve the distinction rather than harmonizing it away. Flag significant inconsistencies to the requester, because terminology chaos in the source often signals a deeper documentation problem the requester should fix. Harmonizing without recording produces a target that looks consistent while hiding a source problem, and the hiding removes evidence the requester needs.

### Preserve Intentional Non-Standard Language

Not all non-standard source language is an error. Before correcting, determine whether the deviation is intentional, because normalizing intentional deviation strips the source's voice.

Dialogue may use dialect or non-standard grammar for characterization. Marketing may break rules deliberately for effect. Literature may use archaism or invented forms. Legal text may use archaic formulas that carry meaning. Correcting these as if they were errors imposes standard grammar on deliberate craft and produces a target that is grammatically cleaner and artistically wrong. The test is intent: did the writer have reason to deviate? Where deviation is intentional, preserve it in spirit rather than normalize it. The translator's job is to carry the source's voice, including its deliberate deviations, not to impose uniform correctness.

### Record Every Handling Decision

Handling decisions that are not recorded are invisible, and invisible decisions cannot be reviewed, challenged, or inherited. Recording is what makes handling transparent.

Maintain a handling log that records each defect, its classification, the decision taken, and the rationale. Silent corrections of low-severity typos may be summarized rather than listed individually, but every noted correction, query, and flag should appear with enough detail that a reviewer can confirm or overturn the decision. The log becomes part of the deliverable, giving the requester visibility into what was changed and why, and protecting the translator from blame for source defects. A handling decision made but not recorded is, for review purposes, a decision never made.

### Report Source Fitness To The Requester

The requester often does not know the source is flawed, and the handling log is valuable intelligence for them. Surface it rather than burying it in the deliverable.

Summarize the source's fitness in a report: the categories and severity of defects found, the handling decisions taken, the queries raised, and recommendations for source remediation. This report helps the requester improve the source for future translations and confirms that defects were handled deliberately rather than propagated. It also protects the translator, because a documented fitness report makes clear that source problems were the source's, not the translation's.

## Common Traps

### Propagating Errors By Faithful Translation

Treating every source word as intentional reproduces typos, factual errors, and inconsistencies in the target. Handle defects deliberately rather than translating them faithfully.

### Silently Rewriting To Fix Everything

Correcting every source issue without notes exceeds authority and hides decisions from the requester. Silent correction is safe only for low-risk, unambiguous errors.

### Silent Correction That Changes Meaning

Correcting a meaning-changing error, such as a wrong dose or missing condition, imposes the translator's interpretation. Query or flag instead; never absorb silently.

### Assuming Correction Authority

The freedom to correct varies by engagement and must be agreed, not assumed. Default to transparency where authority is unclear.

### Harmonizing Terminology Without Recording

Harmonizing inconsistency without a log hides a source problem and removes evidence. Record the decision and flag significant inconsistency to the requester.

### Normalizing Intentional Non-Standard Language

Dialect, marketing rule-breaking, and archaic legal formulas are intentional craft. Normalizing them produces a target that is cleaner and wrong.

### Handling Without Recording

Unrecorded decisions cannot be reviewed or inherited. Log every noted correction, query, and flag so handling is transparent.

## Self-Check

Before approving a translation of a source that contained defects, verify:

- Each detected defect was classified by type and severity before a handling decision was made.
- A consistent default handling rule was applied per class, with deviations deliberate and noted.
- No meaning-changing error in safety, legal, medical, or financial content was silently corrected; such errors were queried or flagged.
- Correction authority was established with the requester, with a conservative default where authority was unclear.
- Terminology inconsistency was handled as a system, harmonized deliberately, and recorded in a terminology log.
- Intentional non-standard language was preserved in spirit, not normalized away.
- Every handling decision is recorded in a log with defect, classification, decision, and rationale.
- A source fitness report summarizes defects, handling, queries, and remediation recommendations for the requester.
- No source defect that affects meaning was propagated into the target without a deliberate, recorded decision.
- The handling is consistent across the document, not ad-hoc per occurrence.
