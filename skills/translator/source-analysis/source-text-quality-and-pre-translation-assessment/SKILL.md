---
name: source_text_quality_and_pre_translation_assessment.md
description: Use when the agent is evaluating a source text's fitness for translation before work begins, identifying source defects that will propagate into target languages, assessing readability and ambiguity for machine translation readiness, deciding whether source editing or controlled language authoring is needed, or triaging content for translation method selection based on source quality.
---

# Source Text Quality And Pre-Translation Assessment

The single most cost-effective intervention in a translation project happens before any translation begins: assessing and improving the source text. Source text defects propagate into every target language. An ambiguous sentence, an inconsistent term, a broken reference, or an untranslatable idiom in the source becomes the same problem multiplied across every language, and each target team must independently discover, interpret, and resolve it, often differently. A source text that is clean, consistent, and unambiguous produces better translations faster and cheaper across all languages. Yet source quality assessment is frequently skipped because the source is treated as a fixed input rather than as a controllable variable. An agent who accepts the source as-is and proceeds to translation inherits and amplifies every defect it contains.

Agents tend to miss that source quality is the dominant factor in translation cost and quality, that source defects compound across languages, that machine translation is especially sensitive to source ambiguity, and that a small investment in source editing before translation yields disproportionate savings downstream. The harm is a translation project that runs over budget and under quality because the source was never assessed, or a machine translation deployment that underperforms because the source was too ambiguous or complex for the engine.

Use this skill when assessing a source text before translation, identifying defects that will affect target quality, evaluating source fitness for machine translation, deciding whether source editing or controlled language is needed, or triaging content by translation method based on source quality. The goal is to treat source quality as a controllable input and to intervene before translation begins, preventing defects from propagating across all target languages.

## Core Rules

### Assess Source Quality Before Committing To Translation

Before translation begins, conduct a source quality assessment that identifies defects likely to affect target quality. This assessment is separate from comprehension (understanding what the source means) and focuses on whether the source is well-formed enough to translate cleanly. A source can be comprehensible to a human reader but still contain defects that make it hard or impossible to translate accurately.

Assess these dimensions: clarity (is each sentence unambiguous?), consistency (are terms, formatting, and conventions used consistently?), completeness (are there missing segments, broken references, or placeholder text?), correctness (are there grammar errors, typos, or factual errors in the source?), and translatability (are there idioms, culture-bound references, or structures that do not transfer cleanly?). Document the findings and categorize them by severity: critical defects that will cause translation errors, moderate defects that will cause inconsistency or extra effort, and minor defects that are cosmetic. Present the assessment to the content owner with a recommendation on which defects to fix before translation.

### Identify Source Defects That Propagate Across Languages

Certain source defects are especially damaging because they propagate into every target language and are interpreted differently by each team. These defects must be identified and fixed in the source, not resolved independently in each target, because independent resolution produces inconsistency.

High-propagation defects include ambiguous pronouns or references (each target team may resolve the ambiguity differently), inconsistent terminology (the same concept referred to by different terms, leading to different translations), sentences with multiple valid parses (where the grammar allows more than one meaning), culture-bound references that assume shared knowledge (idioms, measurements, legal concepts, brand names), and formatting or structural inconsistencies (inconsistent heading levels, numbering, or list styles that confuse extraction and segmentation). For each defect, determine whether it can be fixed in the source (preferred) or must be documented with a translation note that guides all target teams to the same interpretation. Fixing in the source is always better than documenting, because documentation relies on every team reading and applying it correctly.

### Evaluate Source Fitness For Machine Translation

Machine translation is more sensitive to source quality than human translation. A human translator can infer meaning from context, recognize and compensate for source errors, and query ambiguities. A machine translation engine processes the source mechanically and produces output whose quality is directly determined by source clarity. A source that a human translates well may produce poor machine translation because the engine cannot resolve the ambiguities the human resolved intuitively.

Assess source fitness for MT by checking for: long, complex sentences with multiple clauses (engines handle short sentences better), ambiguous syntax (engines pick one parse, often the wrong one), missing subjects or objects (engines may insert incorrect pronouns), inconsistent terminology (engines may translate the same term differently in different contexts), idioms and figurative language (engines may translate literally), and formatting artifacts that break segmentation (tags, placeholders, line breaks mid-sentence). If the source scores poorly on MT fitness, either improve the source through controlled language editing or route the content to human translation where the ambiguities can be managed. Do not deploy MT on source content that the engine cannot handle well and expect acceptable output.

### Apply Controlled Language And Source Simplification

Controlled language (also called controlled authoring or simplified technical English) is a set of rules that constrain source writing to improve clarity, consistency, and translatability. Controlled language rules typically include: keep sentences short (under 20 words), use one term for one concept consistently, avoid idioms and figurative language, use active voice, avoid ambiguous pronouns, use standard grammar and punctuation, and structure instructions as imperative commands for procedural text.

Applying controlled language before translation improves both human and machine translation quality and reduces cost. For organizations with ongoing translation, investing in controlled language authoring standards and training for source content creators yields compounding returns: every future translation benefits from cleaner source. For one-time projects, a source editing pass that applies key controlled language principles to the highest-risk segments can still produce meaningful improvement. Balance the editing effort against the translation volume: for content translated into many languages, source editing effort is multiplied by the number of languages it benefits, making it highly cost-effective.

### Triage Content By Source Quality And Translation Method

Source quality assessment informs the decision of which translation method to apply. High-quality, clear, consistent source content is a good candidate for machine translation with post-editing, because the engine will produce usable output and the post-editor's work is efficient. Moderate-quality source may require human translation with review. Low-quality source with many defects should either be improved before any translation or routed to human translation with explicit guidance on the known defects.

Build a triage that classifies content by source quality and risk: high-quality and low-risk content may use MT with light post-editing; high-quality but high-risk content (legal, medical, safety) still requires human translation despite good source quality; low-quality content should be improved or human-translated with defect notes. Do not apply MT uniformly to all content regardless of source quality; the method must match the source's fitness for that method.

### Manage Source Changes During Translation

Source content sometimes changes after translation has begun, creating rework. Source change management is part of source quality assessment because the frequency and magnitude of source changes determine how much rework buffer and process the project needs. Assess the likelihood of source changes before translation begins: is the source final, or is it still under revision?

For finalized source, proceed with confidence. For source under active revision, establish a change management process: freeze the source for a translation window, batch changes for a re-translation pass, and use translation memory to identify which segments changed so only those are re-translated. Communicate to stakeholders that source changes after translation begins incur cost and timeline impact, and that early source finalization is the most effective cost control. Track source versions so the translation is always traceable to a specific source version.

### Document Source Issues For Target Teams

Not all source defects can be fixed before translation, especially when the source owner cannot or will not revise. For defects that remain, document them in a translation note or query log that is shared with all target teams, so each team interprets the defect the same way rather than independently guessing.

A source issue note should describe the defect, the intended meaning (confirmed with the source owner where possible), and the recommended handling (translate as intended, preserve the ambiguity, flag for review). Centralizing these notes ensures consistency: if the source says "the party" and it is unclear whether this means a legal party or a celebration, the note clarifies which, and all teams translate consistently. Without centralized notes, each team resolves the ambiguity independently, producing inconsistent targets.

## Common Traps

### Accepting The Source As A Fixed Input

Treating the source as immutable and proceeding to translation inherits every defect. Source quality is controllable, and assessing and improving it before translation is the highest-return intervention.

### Letting Source Defects Propagate Without Central Documentation

When defects are not fixed in the source and not centrally documented, each target team resolves them independently, producing inconsistency. Either fix the source or document the intended interpretation for all teams.

### Deploying MT On Source The Engine Cannot Handle

Machine translation quality is bounded by source quality. Deploying MT on ambiguous, complex, or inconsistent source produces poor output that requires heavy post-editing or introduces errors. Assess MT fitness before deploying.

### Skipping Source Assessment Under Deadline Pressure

Under time pressure, teams skip source assessment and start translating, inheriting defects that cost more to resolve in translation than they would have cost to fix in the source. The assessment is fast relative to the cost of propagated defects.

### Assuming Human Translators Will Compensate For All Source Defects

Human translators can resolve many ambiguities, but they also misinterpret, and different translators resolve the same ambiguity differently. Relying on human compensation without source improvement or documentation produces inconsistency.

### Applying One Translation Method Regardless Of Source Quality

Using MT for all content or human translation for all content ignores that source fitness varies. Match the method to source quality and risk through deliberate triage.

### Ignoring Source Change Frequency In Planning

Source that changes frequently during translation creates rework that blows timelines. Assess source stability before committing to deadlines and build a change management process for evolving source.

## Self-Check

- [ ] Has a source quality assessment been conducted before translation begins, covering clarity, consistency, completeness, correctness, and translatability?
- [ ] Have high-propagation defects (ambiguous references, inconsistent terminology, multiple parses, culture-bound references) been identified and either fixed in the source or centrally documented?
- [ ] Has source fitness for machine translation been evaluated, checking for long sentences, ambiguous syntax, missing elements, and formatting artifacts that degrade engine output?
- [ ] Has controlled language or source simplification been applied where the volume of translation justifies the editing investment?
- [ ] Has content been triaged by source quality and risk, matching translation method (MT with post-editing, human translation, or source improvement first) to source fitness?
- [ ] Has source stability been assessed, with a change management process established for evolving source and stakeholders informed of the cost of source changes?
- [ ] Have remaining source defects been documented in a central note or query log shared with all target teams to ensure consistent interpretation?
- [ ] Has the source version been tracked so the translation is traceable to a specific source version?
- [ ] Has the assessment been documented with defect findings, severity categorization, fix decisions, and method recommendations so it can be reviewed by stakeholders?
