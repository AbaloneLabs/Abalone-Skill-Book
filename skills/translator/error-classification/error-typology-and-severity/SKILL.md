---
name: error_typology_and_severity.md
description: Use when the agent is designing or maintaining the error typology and severity model itself, defining error categories and subcategories, building severity scales, mapping defect types to weightings, creating classification guidelines and decision rules, or structuring the classification scheme that reviewers apply, rather than applying an existing taxonomy to individual findings.
---

# Error Typology And Severity

The error typology and severity model is the classification scheme that all review findings are measured against. It is the definition of what counts as an error, what kinds of errors exist, how serious each is, and how seriousness is weighted into a score. This is distinct from the act of applying the scheme to individual findings: this skill concerns designing and maintaining the scheme itself, the shared ruler that makes classification consistent and quality data comparable. Agents often underestimate this work because a typology looks like a simple list of categories. It is not. A poorly designed typology produces ambiguous categories that two reviewers apply differently, severity levels that do not correspond to real consequences, and weightings that make the score meaningless. The harm is systemic: every review conducted against a weak typology generates noisy, incomparable data, improvement efforts cannot find patterns because the categories blur them, and acceptance decisions rest on a score that does not reflect real risk. Designing the typology well is a one-time investment that pays back across every project; designing it casually taxes every project forever.

Use this skill when creating, revising, or documenting the error typology and severity model, defining categories and severity levels, setting weightings, or writing the classification guidelines reviewers will follow. The goal is a scheme that is complete, mutually exclusive, consequence-anchored, and consistently applicable.

## Core Rules

### Build Categories That Are Complete And Mutually Exclusive

A usable typology partitions the space of possible defects so that any finding falls into exactly one category. Completeness means every plausible defect has a home; mutual exclusivity means a defect does not fit two categories at once, which would force arbitrary choices and destroy consistency. Start from a recognized framework such as MQM, which defines top-level categories like accuracy, fluency, terminology, style, locale convention, and design, each with subtypes. Accuracy subdivides into mistranslation, omission, addition, untranslated, and register; fluency into grammar, spelling, punctuation, and readability. Adopt the framework's structure and adapt subtypes to the program's domains, documenting each addition. Test the typology against a sample of real findings: if a finding fits two categories or none, the scheme has a gap or an overlap to fix before it is deployed.

### Define Each Category With Criteria And Examples

A category name is not a definition. For each category and subtype, write a criterion that states what defect belongs there and, equally important, what does not. Provide examples of conforming classifications and of boundary cases that belong in a neighboring category instead. For instance, define mistranslation as a target segment that conveys a meaning different from the source, and distinguish it from a terminology error, which is the wrong approved term but possibly correct meaning, and from a fluency error, which is correct meaning but wrong grammar. Without criteria and examples, reviewers interpret category names through their own experience and classify differently, which is the failure the typology exists to prevent. The guidelines are the typology's real interface; the category names are only labels.

### Anchor Severity To Consequence, Not To Reviewer Reaction

Severity must describe the consequence of the defect for the reader and the purpose, not how much the reviewer disliked it. Define each severity level by the harm or impediment it causes. A workable scale separates critical, defects that cause harm, liability, or total failure of purpose, such as a wrong dose or reversed negation in safety content; major, defects that significantly impede understanding or usability, such as a key mistranslation in an instruction; minor, defects that are noticeable but do not impede use, such as a stylistic awkwardness; and neutral or preference, suggestions that are not defects at all. Anchor each level to consequence descriptions with examples, so that a reviewer deciding severity asks "what does this do to the reader" rather than "how much does this bother me." Severity anchored to reaction drifts with reviewer mood and is not comparable across people.

### Separate Errors From Preferences Explicitly

Preferences, alternative renderings the reviewer would prefer but that are not more correct, must be a distinct class, not folded into minor errors. Conflating them inflates error counts, demoralizes translators who see correct work scored as defective, and buries real defects in noise. Define preference as a suggestion that does not deviate from source meaning, the termbase, the style guide, or language norms, and exclude it from quality scores and from mandatory correction. Reviewers may still make preference comments, but the typology must mark them as non-errors so the data stays clean. This single distinction is often the difference between a typology translators trust and one they distrust.

### Make Severity Context-Dependent, Not Type-Fixed

The same defect type can have different severity depending on where it appears. A terminology error in a casual blog may be minor; the same error in a medical device label is critical. An omission in marketing copy may be minor; an omission of a contract clause is critical. The typology must encode this by making severity a function of context, not a fixed mapping from type to severity. Provide guidance on how context, content tier, and risk raise or lower severity, and require reviewers to justify severity against consequence in high-risk segments. A typology that fixes severity by type mis-prioritizes: it either over-flags low-risk content or under-flags high-risk content, and in the worst case it rates a critical defect as minor because the type was considered minor.

### Set Weightings That Make The Score Meaningful

If the typology feeds a score, the severity weightings must make that score reflect real quality. Critical defects should weigh enough that a small number moves the score decisively, major defects should weigh substantially, and minor defects should weigh lightly enough that a normal density of them does not fail an otherwise good translation. Preference should carry no weight. Test the weighting against realistic batches: a batch with one critical defect should score worse than a batch with several minor defects, and a clean batch should score near the top. Publish the weighting formula, for example the MQM-style weighted error count per thousand words, and keep it stable so scores remain comparable over time. Weightings changed silently make historical data uninterpretable.

### Provide Decision Rules For Boundary Cases

Real findings often sit on boundaries: is a wrong term a terminology error or a mistranslation; is an awkward but correct sentence a fluency error or a preference; is a missing comma minor or major. Provide explicit decision rules that order the questions a reviewer asks. For example: first ask whether meaning is affected, which separates accuracy from fluency; then ask whether an approved term was violated, which separates terminology from general accuracy; then ask whether the issue is a defect or a preference; then assign severity by consequence. Decision rules turn judgment into a reproducible procedure and reduce inter-reviewer disagreement. Maintain a living list of boundary cases and their resolved classifications, and use it in calibration.

### Version The Typology And Document Changes

The typology will evolve as the program learns. When it changes, record the version, the date, the rationale, and which categories, severities, or weightings changed. Historical error data must be interpretable against the typology version that produced it, or trends become meaningless. A typology that changes without versioning produces a data series that looks continuous but is not, hiding real shifts behind definitional ones. Treat the typology as a versioned artifact under governance, not as an ad hoc document.

## Common Traps

### Category Names Treated As Definitions

Without criteria and examples, reviewers interpret labels differently and the typology produces noise instead of comparable data.

### Overlapping Or Incomplete Categories

If a finding fits two categories or none, the scheme forces arbitrary choices and conceals patterns behind classification ambiguity.

### Severity Anchored To Reviewer Reaction

Severity that tracks how much the reviewer disliked the defect drifts with mood and is not comparable across reviewers.

### Folding Preferences Into Minor Errors

Conflating preferences with errors inflates counts, demoralizes translators, and buries real defects in noise.

### Fixed Type-To-Severity Mapping

Mapping a type to one severity ignores context and mis-prioritizes, rating critical defects as minor or low-risk defects as major.

### Weightings That Do Not Reflect Real Quality

If several minor defects outweigh a critical one, the score is meaningless and acceptance decisions rest on a number unrelated to risk.

### No Decision Rules For Boundary Cases

Without ordered decision questions, boundary findings are classified inconsistently and inter-reviewer disagreement stays high.

### Silent Typology Changes

Changing categories, severities, or weightings without versioning makes historical data uninterpretable and hides real trends behind definitional shifts.

## Self-Check

Before approving an error typology and severity model, verify:

- Categories and subtypes are complete and mutually exclusive, adopted from a recognized framework like MQM and adapted with documented additions.
- Each category and subtype has a written criterion plus examples of conforming and boundary classifications, so reviewers apply labels consistently.
- Severity levels are anchored to consequence for the reader and purpose, with descriptions and examples, not to reviewer reaction.
- Preferences are a distinct non-error class, excluded from quality scores and mandatory correction.
- Severity is context-dependent, with guidance on how content tier and risk raise or lower severity, rather than a fixed type-to-severity mapping.
- Severity weightings make the score reflect real quality, tested so that a critical defect outweighs several minor ones and preference carries no weight.
- Decision rules order the classification questions, reducing inter-reviewer disagreement on boundary cases.
- The typology is versioned, with changes documented by date, rationale, and affected categories, so historical data remains interpretable.
- No category name is used as a definition, and no severity is fixed by type without context.
- The scheme is complete, mutually exclusive, consequence-anchored, and consistently applicable across reviewers and projects.
