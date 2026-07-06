---
name: source_quality_assessment.md
description: Use when the agent is assessing the quality of a source text before translation, detecting source errors inconsistencies and OCR problems, deciding whether to translate fix or query flawed source, or determining whether a source is fit for translation and what risks it carries into the target.
---

# Source Quality Assessment

A translation can be no better than the source allows, and many sources are not fit for translation as received. Sources contain typos, grammatical errors, inconsistent terminology, broken sentences, outdated references, contradictory numbers, OCR artifacts, mixed languages, and factual mistakes. A translator who treats the source as authoritative and faithfully renders its flaws exports those flaws into every target language, sometimes amplifying them. A translator who silently fixes the source risks changing meaning the requester intended. Source quality assessment is the disciplined work of detecting source problems, classifying them, and deciding for each whether to translate as-is, correct, query, or flag. Skipping this step is how defective sources become defective translations that no one notices until they cause harm.

Use this skill when evaluating a source before or during translation, when the source contains visible errors or inconsistencies, when deciding whether to fix or query, or when reporting source fitness to the requester. The goal is to prevent the translator from becoming an unknowing conduit for source defects while also preventing unauthorized silent rewriting.

## Core Rules

### Assess Source Fitness Before Committing

Before accepting a translation job, assess whether the source is fit for translation. A source that is a rough draft, an unedited machine output, a scanned document with OCR errors, or a patchwork of updates may not support a quality translation without remediation.

Check for completeness, internal consistency, formatting integrity, and a clear final version. If the source is a working draft, confirm whether it is final. If it is scanned, assess OCR quality. If it is a concatenation of updates, check for contradictions between sections.

Report fitness issues to the requester before quoting effort or starting work. Translating an unfit source and then complaining about quality after delivery is too late.

### Detect And Classify Source Errors

Source errors fall into categories that call for different handling. Detect them systematically rather than reacting to each as it appears.

Categories include typographical and spelling errors, grammatical errors, factual errors such as wrong numbers or dates, terminology inconsistency, broken or ungrammatical sentences, ambiguous references, outdated information, contradictions between sections, OCR artifacts, encoding problems, and mixed languages or scripts.

Classify each error by type and by severity. A typo in casual marketing text is low severity. A wrong dose in a medical instruction is critical. Classification drives the handling decision.

### Decide Handling Per Error

For each source error, choose among translate as-is, correct silently, correct with a note, or query. The choice depends on error type, severity, and the degree of freedom granted.

Low-severity, unambiguous errors such as obvious typos in non-critical text can often be corrected silently, because translating the typo would introduce an error that was not intended. Medium-severity errors or ambiguous cases should be corrected with a translator note so the requester sees what was changed and why. High-severity errors, especially factual errors in safety, legal, medical, or financial content, must be queried, not silently corrected, because the translator may have misunderstood the intent.

Never silently correct a source error where the correction changes meaning, obligation, or fact. That exceeds the translator's authority.

### Handle Terminology Inconsistency Deliberately

Terminology inconsistency is one of the most common source defects. The source may use two or three terms for the same concept, or use one term for two different concepts.

Do not simply propagate the inconsistency. Identify the intended term, usually the one used most or the one matching domain convention, and apply it consistently in the target. Record the decision in a terminology log. Where the source inconsistency might be intentional, such as a legal document using defined terms precisely, preserve the distinction rather than harmonizing.

Flag significant inconsistencies to the requester, because terminology chaos in the source often signals a deeper documentation problem.

### Manage OCR And Encoding Artifacts

Scanned sources and converted documents carry OCR and encoding errors that look like real words but are wrong. These are dangerous because they pass spell-check.

Watch for character substitutions such as rn for m, l for I, 0 for O, dropped or inserted characters, broken accents and diacritics, ligature errors, and mojibake from encoding mismatches. Numbers are especially vulnerable in OCR, and a misread digit in a quantity, date, or measurement can cause harm.

Where OCR quality is poor, request a cleaner source. If none is available, flag every uncertain character rather than guessing, especially in factual content.

### Detect Contradictions And Outdated Content

Sources evolve, and evolution leaves contradictions. A section may reference a feature that a later section says was removed. A number in a table may not match the number in the text. A date may have passed. A referenced document may no longer exist.

Cross-check internally. Compare numbers, names, dates, and references across the document. Where contradictions appear, do not pick one silently. Query which is authoritative, and flag the contradiction so the requester can fix the source.

Outdated content, such as references to discontinued products or expired regulations, may need a note in the target even if the source is otherwise translatable.

### Preserve Intentional Non-Standard Language

Not all non-standard source language is an error. Dialogue may use dialect. Marketing may use deliberate rule-breaking for effect. Literature may use archaism or non-standard grammar. Legal text may use archaic formulas that carry meaning.

Before correcting, determine whether the non-standard language is intentional. Intentional non-standard language should be preserved in spirit, not normalized away. The translator's job is to carry the source's voice, including its deliberate deviations, not to impose standard grammar on everything.

### Report Source Fitness To The Requester

Source assessment is valuable to the requester, who often does not know the source is flawed. Summarize findings in a source quality report.

The report should list significant errors found, handling decisions taken, queries raised, and recommendations for source remediation. This protects the translator from blame for source defects and helps the requester improve the source for future translations.

## Common Traps

### Trusting The Source As Authoritative

Sources are human artifacts full of errors. Treating every word as intentional leads to propagating mistakes into the target.

### Silently Rewriting To Fix Everything

Correcting every source issue without notes exceeds authority and hides decisions from the requester. Correct low-risk typos silently; query or note the rest.

### Propagating Terminology Inconsistency

Translating each occurrence as the source wrote it produces an inconsistent target. Harmonize deliberately and record the decision.

### Trusting OCR Numbers

A digit misread by OCR is a factual error invisible to spell-check. Verify numbers in scanned sources, especially in safety and financial content.

### Picking One Side Of A Contradiction

Choosing the more frequent or more recent value silently can be wrong. Query contradictions that affect meaning.

### Normalizing Intentional Non-Standard Language

Dialogue, marketing, and literature may break rules deliberately. Normalizing them strips the source's voice.

### Skipping The Fitness Report

Failing to report source defects leaves the requester unaware and leaves the translator vulnerable to later blame.

## Self-Check

Before approving a translation of a flawed or complex source, verify:

- Source fitness was assessed before work began, and significant issues were reported to the requester upfront.
- Source errors were detected and classified by type and severity.
- Handling decisions per error are appropriate, with silent correction limited to low-risk unambiguous typos.
- High-severity factual errors in safety, legal, medical, or financial content were queried, not silently corrected.
- Terminology inconsistency was identified, harmonized deliberately, and recorded in a terminology log.
- OCR and encoding artifacts were detected, with uncertain characters flagged, especially in numbers.
- Internal contradictions and outdated content were cross-checked and queried where they affect meaning.
- Intentional non-standard language was preserved in spirit, not normalized away.
- A source quality report summarizes findings, handling, queries, and remediation recommendations.
- No source defect that affects meaning was propagated into the target without a deliberate decision.
