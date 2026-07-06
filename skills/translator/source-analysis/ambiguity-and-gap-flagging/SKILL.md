---
name: ambiguity_and_gap_flagging.md
description: Use when the agent is identifying ambiguity and information gaps in a source text, deciding whether to preserve resolve or query unclear meaning, flagging missing context and undefined terms, or managing translator notes and queries that surface uncertainty rather than hiding it.
---

# Ambiguity And Gap Flagging

Ambiguity and gaps are not failures of the source to be papered over. They are meaning-critical features that the translator must detect, classify, and handle deliberately. A source may be ambiguous because the writer was vague, because context is missing, because a term is undefined, or because the ambiguity is intentional. A source may have gaps where information the target reader needs is assumed but absent. The translator who silently resolves ambiguity or silently fills gaps imposes an interpretation that may be wrong, and the target reader has no way to detect the imposition. The translator who flags ambiguity and gaps makes uncertainty visible, protects against mistranslation, and gives the requester the chance to supply what is missing. Hiding uncertainty is among the most damaging translator habits, because it converts doubt into confident error.

Use this skill when a source contains unclear or incomplete meaning, when deciding whether to preserve or resolve ambiguity, when raising queries, or when writing translator notes that surface uncertainty. The goal is to handle uncertainty with discipline rather than to disguise it.

## Core Rules

### Detect Ambiguity Before It Becomes A Silent Choice

Ambiguity is easy to miss when the source reads smoothly. Train detection on the structures that carry ambiguity.

Common ambiguity sources include pronouns and references with unclear antecedents, words with multiple senses, syntactic structures that allow more than one parse, elliptical or incomplete sentences, undefined terms, vague quantifiers, conditional clauses with unclear scope, negation with unclear scope, and cultural or institutional references whose meaning is not shared. In languages that drop subjects or use flexible word order, ambiguity is more frequent.

When you detect a possible ambiguity, state both readings explicitly before choosing. If you cannot state the alternatives, you have not understood the ambiguity.

### Classify Ambiguity By Intent

Not all ambiguity is accidental. Classify each instance to choose the right handling.

Accidental ambiguity arises from careless writing and should usually be resolved toward the most likely intended meaning, with a note where the stakes are high. Intentional ambiguity, in poetry, diplomacy, advertising, or hedged legal language, should usually be preserved in the target because the ambiguity is the meaning. Strategic vagueness, where a writer avoids commitment, should be preserved rather than resolved, because resolving it changes the writer's legal or rhetorical position.

Misclassifying intentional ambiguity as accidental is a common error. Ask whether the writer had reason to be vague.

### Choose A Handling Strategy Deliberately

For each ambiguity, choose among preserve, resolve toward a primary reading, resolve with a note, or query. The choice depends on classification, stakes, and the degree of freedom granted.

Preserve when the ambiguity is intentional or when both readings are equally valid and low-stakes. Resolve toward the primary reading when the ambiguity is accidental and one reading is clearly intended; add a note if the stakes are meaningful. Resolve with a note when you must choose to produce readable target text but the choice is not certain. Query when the ambiguity is high-stakes and the intended reading cannot be determined, such as in legal, medical, or safety content where the wrong reading causes harm.

Never resolve a high-stakes ambiguity silently. The cost of a query is far lower than the cost of a mistranslated obligation or dose.

### Identify Information Gaps

Gaps are missing information the target reader needs but the source does not provide. Detect them as you read.

Common gaps include undefined acronyms and abbreviations, referenced but absent context such as a section or figure that is not included, assumed prior knowledge the target reader lacks, missing units on numbers, missing dates on deadlines, missing antecedents for references, and missing definitions for specialized terms. Gaps are especially common in extracted segments, such as UI strings pulled out of context, where the surrounding sentence is absent.

List gaps as you find them. A gap that seems minor in isolation may matter when the target reader encounters the text without the source.

### Decide Gap Handling By Stakes

For each gap, decide among infer and note, query, or flag for the requester.

Infer and note when the missing information can be safely reconstructed from context and the stakes are low. Query when the gap affects meaning and cannot be safely inferred, such as a missing unit on a medical dose or a missing condition in a contract clause. Flag when the gap is structural, such as a referenced figure that is absent, because the requester needs to supply the missing piece.

Do not invent information to fill a gap and present it as source content. That is fabrication, not translation.

### Write Useful Translator Notes

Translator notes surface uncertainty and decisions for the reviewer and requester. Write them to be useful, not defensive.

A useful note states the issue, the options considered, the choice taken, and the reason. It is specific enough that a reviewer can confirm or overturn the decision. Notes such as unclear or ambiguous are too vague. Notes such as source pronoun could refer to the company or the product; chose the company because the next sentence continues that topic are useful.

Place notes where the review workflow expects them, and keep a consistent format so they can be scanned.

### Raise Queries That Can Be Answered

A good query is specific and answerable. A vague query wastes the responder's time and delays delivery.

State the segment, the ambiguity or gap, the options, and what you need to decide. Offer your best guess and ask for confirmation, because a yes or no is faster than an open-ended question. Where possible, batch related queries so the responder can answer efficiently.

Track queries to resolution. An unanswered query is a risk that must be resolved by a documented default before delivery, not by silent guessing.

### Preserve Uncertainty In The Deliverable Where Appropriate

Sometimes the right deliverable carries uncertainty forward rather than resolving it. In research translation, legal discovery, or diplomatic text, preserving ambiguity with a note may be more faithful than resolving it.

Match the deliverable's treatment of uncertainty to the purpose. A patient instruction must resolve uncertainty toward safety. A legal translation of a vague clause may need to preserve the vagueness with a note, because resolving it changes the legal effect.

## Common Traps

### Resolving Ambiguity Silently

The most damaging habit. Silent resolution turns doubt into confident error that the target reader cannot detect.

### Treating All Ambiguity As Accidental

Intentional and strategic ambiguity carry meaning. Resolving them changes the source's effect or legal position.

### Inventing Information To Fill Gaps

Filling a gap with plausible-sounding content and presenting it as source meaning is fabrication. Note, query, or flag instead.

### Writing Vague Translator Notes

Notes that say unclear without stating the options and the decision waste the reviewer's time and hide the real issue.

### Letting Queries Drift Unanswered

Unanswered queries become silent assumptions at delivery. Track them and resolve with a documented default.

### Ignoring Gaps In Extracted Segments

UI strings and isolated segments often lack context. Treating them as complete when they are fragments causes errors.

### Resolving High-Stakes Ambiguity Without A Query

In legal, medical, and safety content, guessing is dangerous. Query when the reading affects obligation, dose, or safety.

## Self-Check

Before approving a translation that involved ambiguity or gaps, verify:

- Ambiguity was detected before silent choices were made, and alternative readings were stated explicitly.
- Each ambiguity was classified as accidental, intentional, or strategic, and handling matches the classification.
- High-stakes ambiguity in legal, medical, or safety content was queried, not resolved silently.
- Information gaps were identified, including undefined terms, missing units, absent references, and missing context.
- Gap handling matches stakes, with inference limited to low-risk cases and queries or flags for meaningful gaps.
- No information was invented to fill a gap and presented as source content.
- Translator notes state the issue, options, choice, and reason, in a consistent scannable format.
- Queries are specific, answerable, and tracked to resolution or to a documented default.
- Uncertainty is preserved in the deliverable where the purpose requires it, such as legal or diplomatic text.
- No ambiguity or gap that affects meaning has been hidden to make the translation look cleaner than the source allows.
