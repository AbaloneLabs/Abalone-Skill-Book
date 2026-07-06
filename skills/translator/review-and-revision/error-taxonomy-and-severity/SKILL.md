---
name: error_taxonomy_and_severity.md
description: Use when the agent is classifying translation errors by type and severity, applying an error taxonomy such as LISA or MQM, distinguishing critical from minor and preference issues, or using severity ratings to decide what must be fixed and what can be accepted in review and quality evaluation.
---

# Error Taxonomy And Severity

Translation review produces findings, and findings are useful only if they are classified consistently. An error taxonomy provides shared categories, and severity ratings provide shared priorities. Without them, one reviewer's critical error is another's minor preference, correctors cannot triage, quality scores cannot be compared across projects, and improvement efforts cannot target the real weaknesses. With them, review becomes structured: every finding has a type that indicates what went wrong and a severity that indicates how much it matters, and the team can decide what must be fixed, what should be fixed, and what can be left. Applying a taxonomy is not bureaucratic box-ticking. It is the discipline that turns a pile of review comments into actionable, comparable, improvable quality information. Inconsistent or absent classification is why review feedback feels arbitrary and why the same errors recur.

Use this skill when classifying translation errors, applying an error taxonomy, assigning severity, or using error data to drive correction and improvement decisions. The goal is to classify errors consistently so that correction, quality measurement, and improvement are based on shared, meaningful standards.

## Core Rules

### Use A Recognized Taxonomy Consistently

Adopt a recognized error taxonomy and apply it consistently across projects and reviewers. Common frameworks provide shared categories.

Established frameworks include the LISA quality model, the DQF and MQM frameworks used in industry, and organization-specific taxonomies built from these. A typical taxonomy categorizes errors by type, such as accuracy, fluency, terminology, style, locale convention, and formatting, and by subtypes within each. Using a recognized taxonomy means reviewers classify the same way, findings aggregate meaningfully, and quality is comparable across projects.

Define which taxonomy to use, train reviewers on it, and apply it uniformly. Switching taxonomies or inventing categories per project destroys comparability.

### Classify By Error Type

Error type indicates what went wrong and drives improvement. Classify each finding by type.

Common types include accuracy errors such as mistranslation, omission, addition, and shifts; fluency errors such as grammar, spelling, and punctuation; terminology errors such as wrong term or inconsistency; style errors such as register or voice deviations; locale convention errors such as dates, numbers, and units; and formatting errors such as layout or markup. Type tells the team where weaknesses concentrate, so improvement can target terminology research, style guidance, or whatever the data reveals.

Misclassification hides patterns. An omission recorded as a style issue prevents identifying a completeness problem.

### Assign Severity By Impact

Severity indicates how much the error matters and drives correction priority. Assign severity by impact on the reader and the purpose, not by the reviewer's annoyance.

A common severity scale includes critical, errors that cause harm, liability, or complete failure of purpose, such as a wrong dose or reversed negation in safety content; major, errors that significantly impede understanding or usability, such as a key mistranslation; minor, errors that are noticeable but do not impede use, such as a style awkwardness; and preference, suggestions that are not errors but might improve the text. Severity must reflect consequences: a typo in casual marketing is minor, but a typo in a dose is critical.

Severity drives the correction decision. Critical and major must be fixed; minor may be fixed depending on time; preference is optional.

### Separate Errors From Preferences

A frequent classification failure is treating preferences as errors. Separate them, because conflating them inflates error counts and demoralizes translators.

An error is a deviation from the source meaning, the termbase, the style guide, or the language's norms that affects correctness. A preference is a different way the reviewer would have rendered the text that is not more correct, only different. Preferences have value as suggestions but should not count against quality scores or demand correction. Classify preferences explicitly so they are distinguished from real errors.

Reviewers who flag preferences as errors produce noisy feedback that translators learn to distrust.

### Context Determines Severity

The same error type can have different severity depending on context. Severity is not fixed per type; it depends on where and how the error appears.

A terminology error in a casual blog post may be minor; the same error in a medical device label is critical. An omission in a marketing paragraph may be minor; an omission of a contract clause is critical. A fluency error in internal content may be minor; in published marketing it is major. Assign severity based on the specific segment's context, purpose, and stakes, not by a fixed mapping from type to severity.

This requires the reviewer to understand the content's risk, which is why severity assignment demands judgment, not mechanical rule application.

### Use Severity To Drive Correction

Severity exists to drive correction decisions. Use it explicitly.

Define rules such as: critical errors must be fixed before delivery, no exceptions; major errors must be fixed unless the requester explicitly accepts the risk; minor errors should be fixed if time allows; preferences are optional and at the translator's discretion. Apply the rules consistently so correction effort targets the errors that matter. Without severity-driven rules, correctors either fix everything, wasting time on preferences, or fix randomly, missing critical errors.

Document the correction rules so correctors and reviewers share expectations.

### Aggregate Error Data For Improvement

Error classification produces data that drives improvement. Aggregate and use it.

Track error types and severities across projects, translators, and content types. Identify patterns, such as terminology errors concentrating in a domain that needs better termbase coverage, or fluency errors concentrating with a translator who needs support. Feed the patterns back into training, termbase expansion, style guide clarification, and process changes. Without aggregation, the same errors recur because no one sees the pattern.

Error data is among the most valuable quality assets an organization has, but only if it is classified consistently and actually used.

### Calibrate Reviewers For Consistency

Different reviewers classify differently, which undermines the taxonomy's value. Calibrate reviewers.

Run calibration sessions where reviewers classify the same sample and discuss disagreements. Develop guidelines for borderline cases. Track inter-reviewer consistency and address divergences. Calibration is ongoing, because reviewers drift and new reviewers need onboarding. A taxonomy applied inconsistently across reviewers provides no more comparability than no taxonomy.

## Common Traps

### No Taxonomy Or Inconsistent Taxonomy

Without a shared taxonomy, findings are not comparable and improvement cannot target patterns.

### Treating Preferences As Errors

Inflating error counts with preferences demoralizes translators and hides real errors in noise.

### Fixed Type-To-Severity Mapping

The same error type has different severity in different contexts; mechanical mapping mis-prioritizes.

### Not Using Severity To Drive Correction

Classifying without severity-driven correction rules leads to fixing everything or fixing randomly.

### Misclassification Hiding Patterns

Recording an omission as a style issue prevents identifying and fixing a completeness problem.

### Ignoring Error Data For Improvement

Classified errors that are not aggregated and acted on provide no improvement benefit.

### Uncalibrated Reviewers

Reviewers applying the taxonomy differently destroy the comparability the taxonomy is meant to provide.

### Over-Or-Under Severity Inflation

Calling everything critical devalues severity; calling everything minor under-states risk. Calibrate to impact.

## Self-Check

Before approving error classification or a quality evaluation, verify:

- A recognized error taxonomy is adopted and applied consistently across projects and reviewers.
- Each finding is classified by error type, accuracy, fluency, terminology, style, locale, or formatting, with appropriate subtypes.
- Severity is assigned by impact on the reader and purpose, not by reviewer annoyance, on a defined critical-major-minor-preference scale.
- Errors are separated from preferences, with preferences not counting against quality or demanding correction.
- Severity reflects the specific segment's context and stakes, with the same type rated differently where consequences differ.
- Severity drives correction through explicit rules, with critical and major fixed, minor fixed if time allows, and preference optional.
- Error data is aggregated by type, severity, translator, and content type to identify patterns and drive improvement.
- Reviewers are calibrated through sessions, guidelines, and consistency tracking so classification is comparable across the team.
- No preference is classified as an error, and no critical error is under-rated due to mechanical type-to-severity mapping.
- The classification supports correction, quality measurement, and improvement decisions based on shared, meaningful standards.
