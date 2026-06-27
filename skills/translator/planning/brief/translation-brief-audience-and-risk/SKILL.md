---
name: translation_brief_audience_and_risk.md
description: Use when the agent is planning a translation, clarifying source and target audience, deciding translation purpose, assessing risk, choosing between literal translation and localization, or preparing a translation brief before transforming text between languages.
---

# Translation Brief Audience And Risk

Translation begins with decisions about purpose, reader, risk, and acceptable freedom. A translator who starts by converting sentences may preserve words while losing the job the text must perform. The same source text can require a faithful legal translation, a localized product message, a plain-language public notice, a subtitle with time limits, a literary rendering, or a technical document with strict terminology.

Use this skill before translating, localizing, adapting, summarizing across languages, reviewing machine translation, or preparing a multilingual deliverable. The goal is to prevent the agent from assuming that fluent target-language prose is enough. Translation quality depends on whether the target text serves the same function for the right reader under the right constraints.

## Core Rules

### Define The Translation Job Before Translating

Identify what kind of work is being requested. Translation can mean several different things.

Clarify whether the task is:

- direct translation;
- localization;
- transcreation;
- legal or certified-style translation;
- technical translation;
- literary translation;
- subtitle or caption translation;
- interpretation support;
- bilingual summary;
- glossary extraction;
- machine translation review;
- comparison of source and target;
- plain-language adaptation across languages.

Each task has different degrees of freedom. A medical label, contract clause, API error message, poem, marketing headline, and customer support article should not be handled with the same strategy.

### Identify The Target Reader

The target reader may not match the source reader exactly. A document written by specialists in one language may need to be read by customers, regulators, employees, patients, students, or judges in another language.

Clarify:

- target language and locale;
- reader expertise;
- reader age or accessibility needs;
- expected reading situation;
- cultural assumptions;
- whether the reader is deciding, learning, complying, buying, or being warned;
- whether the text must sound native, institutional, technical, warm, formal, urgent, or neutral;
- whether the reader will compare it with the source.

Do not translate for an abstract language. Translate for a reader in a context.

### Establish Purpose And Success Criteria

A translation should be evaluated by whether it fulfills the intended function. The purpose may be legal equivalence, operational clarity, brand effect, emotional force, educational accessibility, procedural accuracy, or search discoverability.

State the success criteria:

- preserve legal meaning;
- allow safe use of a product;
- make instructions executable;
- maintain brand voice;
- reproduce humor or emotional effect;
- preserve ambiguity intentionally present in the source;
- make a public notice understandable;
- allow side-by-side comparison;
- fit character limits;
- follow a glossary or term base.

When criteria conflict, decide the priority. For example, subtitle translation may prioritize timing and readability over word-for-word completeness. Legal translation may prioritize source alignment over elegance.

### Assess Risk Before Choosing Freedom

The higher the stakes, the less silently creative the translation should be. Legal, medical, financial, safety, immigration, employment, regulatory, academic, and technical documents require conservative handling and often human expert review.

Assess risk:

- Could mistranslation affect health, safety, legal rights, money, immigration, employment, compliance, reputation, or access to services?
- Does the source contain specialized terminology?
- Are there numbers, dates, deadlines, doses, conditions, warnings, or obligations?
- Is the translation official, public, contractual, or evidentiary?
- Will someone rely on it without seeing the source?
- Does it require certification, sworn translation, or jurisdiction-specific wording?

If risk is high, state limits clearly and recommend qualified review. Do not present an AI-produced translation as certified or legally authoritative.

### Decide Literalness, Adaptation, And Localization Boundaries

Literalness is not always fidelity. A literal translation can be wrong if idioms, politeness, genre, or syntax do not transfer. But free adaptation can be wrong if it changes obligations, facts, tone, or evidence.

Decide boundaries:

- What must remain close to the source?
- What can be localized?
- What must be explained rather than translated?
- What terms must remain in the source language?
- What units, names, dates, or formats must be converted?
- What brand names, product names, laws, or quoted text must remain unchanged?
- What ambiguity must be preserved?

Document the decision when the translation departs from literal wording for functional reasons.

### Check Source Quality And Completeness

A bad source creates a risky translation. Before translating, inspect the source for ambiguity, missing context, formatting problems, OCR errors, inconsistent terminology, broken sentences, unclear references, or mixed languages.

Ask:

- Is the source complete?
- Are there images, tables, footnotes, captions, or placeholders?
- Are line breaks meaningful?
- Are there abbreviations or jargon?
- Are pronouns ambiguous?
- Are dates, numbers, or units clear?
- Does the source contain errors that should be preserved, corrected, or queried?
- Is there surrounding context that affects meaning?

Do not silently fix source errors unless the task is adaptation and the choice is safe. When in doubt, flag the issue.

### Preserve Non-Translatable Elements Deliberately

Some parts of a text should not be translated or should be transformed only under rules.

Watch for:

- names;
- addresses;
- email addresses;
- URLs;
- code;
- variables;
- placeholders;
- product names;
- trademarks;
- legal citations;
- file paths;
- measurement units;
- dates and times;
- currency;
- quoted titles;
- form labels;
- UI keys.

Changing these accidentally can break systems, contracts, citations, or user instructions.

### Define Deliverable Format

Translation output may need a clean target text, bilingual table, tracked changes, glossary, translator notes, uncertainty notes, subtitle format, localized UI strings, or side-by-side comparison.

Before beginning, decide:

- target format;
- whether to preserve source formatting;
- whether to include notes;
- whether to mark uncertain terms;
- whether to include source text;
- whether to translate headings, captions, alt text, metadata, and footnotes;
- whether to maintain paragraph numbering or segment IDs;
- whether character limits apply.

The right output format can prevent ambiguity and make review easier.

## Common Traps

### Assuming Fluency Equals Accuracy

A target text can read naturally while changing the source meaning. Always check function, facts, obligations, and tone.

### Ignoring Locale

Spanish for Mexico, Spain, and Argentina may differ. English for the United States, United Kingdom, India, and Singapore may differ. Locale affects spelling, politeness, legal terms, measurements, date formats, and reader expectations.

### Translating The Visible Text Only

Documents often include captions, tables, alt text, footnotes, headers, buttons, filenames, metadata, or image text. Missing these can make the deliverable incomplete.

### Over-Localizing High-Stakes Text

Contracts, warnings, medical instructions, and official statements should not be creatively localized unless qualified review approves it. Preserve legal and operational meaning.

### Treating Ambiguity As A Problem To Hide

If the source is ambiguous, the target may need to preserve that ambiguity or flag it. A translator should not resolve uncertainty silently when the answer matters.

### Forgetting Confidentiality

Translation often involves private, legal, medical, personnel, financial, or unpublished material. Do not expose sensitive content unnecessarily in notes, examples, logs, prompts, or external tools.

## Self-Check

Before starting or approving a translation plan, verify:

- The task type is clear: translation, localization, transcreation, review, summary, subtitle, technical, legal, or another form.
- The target language and locale are specified or a reasonable assumption is stated.
- The target reader and reading context are defined.
- The purpose and success criteria are explicit.
- High-stakes content has been identified and marked for conservative handling or qualified review.
- The allowed degree of adaptation is clear.
- Source quality issues, missing context, or ambiguity have been flagged.
- Non-translatable elements and protected tokens are identified.
- Formatting, tables, captions, footnotes, metadata, and UI strings are included or intentionally excluded.
- The deliverable format is defined.
- The plan does not imply certification, legal authority, or expert validation when none exists.
- Confidentiality and sensitive information risks are considered before using tools or examples.
