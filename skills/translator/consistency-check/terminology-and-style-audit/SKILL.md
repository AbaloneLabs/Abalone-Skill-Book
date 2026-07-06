---
name: terminology_and_style_audit.md
description: Use when the agent is conducting a systematic terminology and style audit of translated content, designing an audit methodology and checklist, classifying consistency and style findings by severity, using QA tooling to detect deviations while verifying its output, or producing an audit report that distinguishes real errors from legitimate variation.
---

# Terminology And Style Audit

An audit is a systematic, evidence-based review whose purpose is to find every deviation from the agreed terminology and style authorities and to classify each one accurately. It is not the same as reading the translation and improving it. Reading and improving blends detection with correction, which biases the reviewer toward what they notice and away from what they do not, and which leaves no auditable record of what was checked. A proper audit separates detection from correction, works from an explicit checklist of surfaces, classifies findings by severity, and treats automated QA tooling as a detector that must be verified rather than as an authority. The harm this skill prevents is the false confidence of a review that felt thorough but was actually unstructured, where the obvious issues were fixed and the systematic ones, the term rendered five ways, the register that shifts mid-document, the format that drifts across sections, were never surfaced because no one looked for them methodically.

Agents miss this work because unstructured review feels sufficient when the text reads well. It is not. A fluent translation can be riddled with consistency and style deviations that only a structured audit catches, and those deviations are exactly what erodes professionalism and, in regulated domains, creates risk.

## Core Rules

### Separate Detection From Correction

Run the audit in two phases. First, detect and record every potential deviation without fixing it, so the full picture of issues is visible before any change is made. Second, correct the confirmed errors. Mixing the two phases causes the reviewer to fix what they notice and move on, leaving undetected issues unrecorded and making it impossible to report coverage or severity patterns.

Detection-first also prevents over-correction. When you correct as you go, you tend to fix things that are actually legitimate variation, because the impulse to improve overrides the discipline to classify. Record first, classify, then correct only the confirmed errors.

### Build An Explicit Audit Checklist Of Surfaces

Do not audit by feel. Build a checklist of every surface the authorities govern and check each one deliberately. The checklist should cover terminology, including approved, admitted, deprecated, and forbidden forms; style, including register, forms of address, voice, sentence conventions; formatting, including numbers, dates, units, currency, punctuation, capitalization; structural elements, including headings, cross-references, captions; and protected elements, including placeholders, code, names, and trademarks.

A checklist ensures the audit is repeatable and complete. Without it, the audit reflects whichever surfaces the reviewer happened to focus on, and coverage is unreportable.

### Establish The Authorities And Precedence Before Auditing

An audit measures the text against authorities, so the authorities must be fixed before checking. Confirm which termbase version, which style guide version, and which translation memory govern the content. Define precedence for when authorities conflict: typically termbase overrides translation memory, because memory may contain deprecated terms, and style guide overrides reviewer preference.

Without fixed authorities and precedence, audit findings are subjective, because two reviewers measuring against different authorities will disagree on what counts as a deviation.

### Use QA Tooling As A Detector, Not An Authority

Automated QA tools detect many deviations efficiently, especially terminology mismatches, double spaces, formatting inconsistencies, and numeric discrepancies. Use them to generate candidate findings. But treat every automated flag as a candidate requiring verification, not as a confirmed error, because tools produce false positives from legitimate variation, context-dependent renderings, and partial-string matches.

Equally, do not dismiss tool output wholesale. The real errors are hidden among the false positives, and ignoring all flags to avoid the false-positive noise means missing the real errors. Review each flag, classify it, and record the decision. The value of QA tooling is realized only through disciplined verification.

### Run Concordance To Find Undocumented Variation

QA tools flag deviations from the termbase, but they cannot find variation that the termbase never recorded. For that, run concordance searches on key terms and inspect every rendering across the content. Group the renderings and look for undocumented variants, the same source concept rendered in ways none of which are in the termbase. These are invisible to termbase-driven checks and are among the most damaging consistency defects, because they indicate the termbase itself has a gap.

Feed undocumented variants back to terminology governance as candidate entries, so the gap is closed for future work rather than recurring indefinitely.

### Classify Findings By Severity

Not every deviation is equal, and an audit report that treats them all the same is not actionable. Classify each finding by severity, using criteria tied to impact. Critical deviations cause misunderstanding, safety, compliance, or legal risk, such as a mistranslated warning or a wrong unit. Major deviations affect clarity, consistency, or professionalism noticeably, such as inconsistent rendering of a key term. Minor deviations are cosmetic or low-impact, such as a punctuation spacing inconsistency.

Severity classification lets corrections be prioritized when time is limited and lets the report communicate risk to stakeholders rather than a flat list of nits.

### Distinguish Errors From Legitimate Variation

A core audit judgment is deciding whether a deviation is an error to fix or legitimate variation to document. Legitimate variation arises when syntax, register, or context requires a different rendering, when a term functions differently as a noun versus a verb, or when the style guide explicitly allows alternatives. Errors arise when the deviation is accidental, ungoverned, or contradicts an authority.

Document legitimate variation with its rationale so reviewers and future auditors do not re-flag it. Without this documentation, the same non-errors get flagged and dismissed on every audit, wasting effort and eroding trust in the audit process.

### Produce An Auditable Report

The audit must produce a report that records what was checked, against which authorities, what was found, how it was classified, and what was corrected or documented. The report is the evidence that the audit happened and the basis for tracking quality over time. A report that lists only the fixes made is insufficient, because it cannot show coverage or severity patterns and cannot support trend analysis across projects.

Include coverage, the surfaces and files checked; findings, classified by severity; dispositions, corrected, documented as legitimate, or deferred; and authority versions used.

## Common Traps

### Mixing Detection With Correction

Correcting as you go leaves undetected issues unrecorded and biases toward over-correction. Detect and record first, then correct confirmed errors.

### Auditing By Feel Without A Checklist

Unstructured review reflects whichever surfaces the reviewer focused on and cannot report coverage. Build and follow an explicit surface checklist.

### Vague Or Conflicting Authorities

Auditing without fixed authorities and precedence makes findings subjective and non-repeatable. Fix the authorities and precedence first.

### Treating QA Tool Output As Authority

Auto-accepting flags introduces errors from false positives; ignoring flags misses real errors. Verify every flag and record the decision.

### Relying Only On Termbase-Driven Checks

Termbase checks miss undocumented variants. Run concordance to find variation the termbase never recorded, and feed gaps to governance.

### Flat Severity Classification

Treating all deviations equally makes the report unactionable and hides risk. Classify by impact tied to misunderstanding, safety, and compliance.

### Re-Flagging The Same Legitimate Variation

Without documenting legitimate variation, the same non-errors get flagged every audit. Document rationale so they are not revisited.

### Reporting Only Fixes, Not Coverage

A report listing only corrections cannot show what was checked or support quality trends. Record coverage, findings, dispositions, and authority versions.

## Self-Check

Before accepting a terminology and style audit as complete, verify:

- Detection was separated from correction: potential deviations were recorded before any change, then confirmed errors were corrected.
- An explicit checklist of surfaces, terminology, style, formatting, structure, and protected elements, was followed, and coverage is reportable.
- The governing authorities and their versions were fixed before auditing, with defined precedence for conflicts.
- Automated QA tool output was used as a detector and every flag was verified and classified, neither auto-accepted nor wholesale ignored.
- Concordance searches were run on key terms to find undocumented variants, and gaps were fed to terminology governance.
- Findings were classified by severity tied to impact, distinguishing critical, major, and minor deviations.
- Legitimate variation was distinguished from errors and documented with rationale so it is not re-flagged.
- An auditable report records coverage, findings by severity, dispositions, and authority versions, not only the corrections made.
- No deviation was dismissed as cosmetic without checking whether it affects consistency or professionalism at scale.
- The audit would produce the same findings if repeated by another reviewer using the same authorities and checklist.
