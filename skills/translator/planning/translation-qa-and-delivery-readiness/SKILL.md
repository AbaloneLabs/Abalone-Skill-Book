---
name: translation_qa_and_delivery_readiness.md
description: Use when the agent is reviewing a translation for accuracy, completeness, terminology, formatting, risk, reviewer notes, machine translation errors, final delivery quality, or source-target alignment before handoff or publication.
---

# Translation QA And Delivery Readiness

Translation quality assurance is not a spell-check pass. It checks whether the target text faithfully serves the brief, preserves source meaning where required, adapts appropriately where allowed, uses consistent terms, includes all content, avoids dangerous errors, and is ready for its delivery format. Many translation failures happen in details: a missing negation, wrong number, untranslated caption, changed placeholder, mistranslated warning, or inconsistent term.

Use this skill after drafting a translation, reviewing machine translation, preparing bilingual deliverables, or finalizing localized content. The goal is to give the agent a disciplined review path before handoff.

## Core Rules

### Review Against The Brief First

Do not evaluate a translation only by whether it sounds good. Compare it to the agreed purpose, audience, risk level, locale, style, and deliverable format.

Check:

- target language and locale;
- task type;
- adaptation freedom;
- required terminology;
- audience level;
- formality and tone;
- high-stakes constraints;
- formatting requirements;
- whether notes are expected;
- whether source alignment is required.

If the translation is fluent but violates the brief, it is not ready.

### Check Completeness Segment By Segment

Make sure all source content that should be translated is represented.

Review:

- headings;
- body paragraphs;
- tables;
- captions;
- footnotes;
- endnotes;
- forms;
- buttons;
- error messages;
- image text;
- alt text;
- metadata;
- legal notices;
- appendices;
- references;
- repeated boilerplate;
- source comments or notes if required.

Watch for skipped sentences, duplicated segments, untranslated phrases, and content accidentally left in the source language.

### Perform Meaning Alignment

Read source and target together. Do not rely on target fluency. Check whether the target preserves the same facts, relationships, obligations, tone, and uncertainty.

Look for:

- omitted negation;
- added certainty;
- softened warning;
- changed actor;
- changed responsibility;
- changed timing;
- changed condition;
- wrong pronoun reference;
- mistranslated modal verbs such as must, may, should, can, will;
- changed comparison;
- changed causal relationship;
- added explanation not present in the source;
- lost ambiguity.

High-risk segments need closer alignment than low-risk marketing adaptation, but every translation needs meaning review.

### Run A Focused Error Pass

Translation errors often cluster by type. Run focused passes rather than one vague reread.

Useful passes:

- numbers, dates, units, currency, and ranges;
- names, titles, organizations, and product terms;
- terminology and glossary compliance;
- placeholders, tags, variables, and code;
- omissions and additions;
- punctuation and formatting;
- tone and register;
- legal, medical, financial, safety, or technical claims;
- target-language grammar and naturalness;
- layout or length constraints.

Focused passes reduce the chance that fluent reading hides small but serious errors.

### Review Machine Translation Skeptically

Machine translation can be useful but tends to produce fluent errors, inconsistent terminology, missed context, over-literal idioms, wrong formality, and hallucinated normalization of awkward source text.

When reviewing machine translation, check:

- whether all source segments are present;
- whether terminology follows glossary;
- whether named entities are preserved;
- whether negatives and conditions are correct;
- whether idioms were literalized;
- whether politeness or register is wrong;
- whether source ambiguity was resolved silently;
- whether repeated segments are inconsistent;
- whether placeholders were damaged;
- whether target text sounds natural but says something different.

Do not trust machine output because it reads smoothly.

### Handle Reviewer Notes And Uncertainty Transparently

Some decisions need reviewer attention. Notes should help the next person, not clutter the deliverable.

Use notes for:

- ambiguous source text;
- terms with multiple possible translations;
- culturally specific references;
- high-stakes wording needing expert review;
- source errors;
- adaptation choices;
- formatting compromises;
- untranslated names or official terms;
- places where target language cannot preserve a source pun or ambiguity.

Do not hide uncertainty in the final translation. If the risk matters, flag it.

### Check Target-Language Quality

A translation can be accurate but awkward, unidiomatic, or hard to read. After alignment, read the target as a standalone text.

Check:

- grammar;
- punctuation;
- natural word order;
- paragraph flow;
- register consistency;
- terminology consistency;
- readability for the intended audience;
- style fit;
- no source-language interference;
- no overly literal structures;
- no unexplained foreign terms unless intentional.

The target should sound like purposeful writing in the target language, unless source alignment requires visible closeness.

### Validate Format And Production Readiness

Final delivery may require more than text.

Check:

- file format;
- encoding;
- line breaks;
- table structure;
- subtitle timing;
- character limits;
- right-to-left layout if relevant;
- fonts and missing characters;
- PDF extraction issues;
- comments removed or preserved intentionally;
- bilingual alignment;
- tracked changes;
- final filenames;
- version number;
- glossary attached if needed.

If the translation will be imported into software, confirm that placeholders, tags, keys, and segment IDs are intact.

### Escalate High-Stakes Material

Some translations require qualified human review, certification, or subject-matter expert approval. AI or general translator review is not enough.

Escalate:

- contracts;
- court documents;
- immigration documents;
- medical instructions;
- prescriptions;
- safety warnings;
- financial disclosures;
- tax documents;
- regulatory filings;
- employment termination or disciplinary letters;
- technical procedures where errors can damage systems or safety;
- public statements with legal or reputational consequences.

State clearly when the translation is a draft for review rather than final authoritative text.

## Common Traps

### Reading Only The Target Text

Target-only reading catches style problems but misses mistranslation. Always compare source and target for meaning alignment when accuracy matters.

### Trusting Smoothness

The most dangerous translation errors often sound natural. Smooth target prose can hide wrong actors, missing negation, changed conditions, or invented clarity.

### Missing Small High-Risk Tokens

One digit, unit, deadline, dosage, "not," "unless," "may," or "must" can change the whole meaning. Review these deliberately.

### Leaving Reviewer Notes Unclear

A vague note like "check this" is not useful. Explain the uncertainty and the decision needed.

### Forgetting Deliverable Constraints

A correct translation that breaks a UI, subtitle, table, or form is not delivery-ready.

### Overediting After Approval

Late style changes can introduce new errors. After final review, change only what is necessary and recheck affected segments.

## Self-Check

Before treating the translation as ready for delivery, verify:

- The target text satisfies the brief, audience, locale, purpose, and adaptation boundaries.
- All required source content is translated or intentionally preserved.
- Source-target meaning alignment has been checked for central and high-risk segments.
- Negations, conditions, obligations, permissions, deadlines, quantities, and warnings are correct.
- Terminology follows the glossary or documented decisions.
- Names, titles, product terms, legal references, and technical terms are accurate.
- Numbers, dates, units, currency, percentages, and ranges are correct; placeholders, tags, variables, links, code, and IDs are intact
- Tone, register, politeness, and reader relationship fit the target context; the target text reads naturally enough for its purpose
- Machine translation output, if used, was reviewed skeptically for fluent errors; ambiguity, source errors, or risky choices are noted for reviewer attention
- Formatting, layout, file format, comments, tracked changes, and version state are ready for handoff; high-stakes material is clearly marked as needing qualified review when applicable
