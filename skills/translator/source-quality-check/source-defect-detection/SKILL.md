---
name: source_defect_detection.md
description: Use when the agent is systematically scanning a source text to detect defects before translation, finding typos grammatical errors factual inconsistencies terminology drift OCR and encoding artifacts structural breaks and mixed-language contamination, or running a methodical defect sweep across a document rather than reading it only for meaning.
---

# Source Defect Detection

A translator who reads a source only for meaning will catch the defects that interrupt comprehension and miss the ones that do not. Unfortunately, the most dangerous defects are often the ones that do not interrupt comprehension: a digit transposed by OCR that still looks like a plausible number, a terminology shift that reads naturally in context, an encoding artifact that produces a real but wrong character, a contradiction between two sections that are never read together. These defects pass because the source flows, and flowing text feels correct. Source defect detection is the discipline of reading the source twice, once for meaning and once for error, using a systematic sweep that targets the categories where defects hide. Without that second pass, the translator inherits the source's flaws and exports them into every target language, sometimes magnified, with no one aware until the defect causes harm.

Agents skip systematic detection because it feels redundant; the source seems readable, so it seems fine. But readability is not integrity, and a source can be perfectly readable and thoroughly defective. Detection is not the same as comprehension. It is a separate pass with a different goal: to find what is wrong, not what is meant. The cost of skipping it is paid downstream, when a defect that was invisible at reading becomes a mistranslation that is invisible at review, because no one knew to look for it. Treating detection as a deliberate, categorized sweep is what separates a translator who reproduces the source faithfully, flaws and all, from one who catches the flaws before they propagate.

Use this skill when performing a pre-translation quality sweep of a source, when a source may contain errors that affect the translation, or when establishing a detection methodology for a document. The goal is to surface defects as a categorized list before any translation decisions are made about how to handle them.

## Core Rules

### Run A Separate Defect Pass Distinct From Comprehension

Do not try to detect defects while you are first understanding the source. The two activities use different attention and conflict with each other. Comprehension reading forgives errors to grasp intent; detection reading must not forgive anything.

Perform comprehension first, to understand the document's purpose and content. Then perform a dedicated defect pass, reading specifically for what is wrong. In the defect pass, slow down at every number, name, date, term, and cross-reference, because these are where consequential errors live. A combined pass detects only the errors loud enough to interrupt meaning and silently absorbs the rest. The separate pass is what makes detection reliable.

### Sweep By Category, Not By Impression

Impression-based detection catches obvious typos and misses subtle defects. Categorical sweeping forces attention onto each defect family in turn, so nothing is skipped because it did not happen to catch the eye.

Run the source through a sequence of category checks. Typographic and spelling: look for character-level errors, doubled or missing letters, and inconsistent capitalization. Grammatical: look for broken agreement, tense shifts, and ungrammatical sentences. Factual: look for wrong numbers, dates, quantities, and measurements, cross-checking them wherever they recur. Terminology: look for the same concept named differently, or one term used for two concepts. Structural: look for broken sentences, dangling references, and missing connective text. OCR and encoding: look for character substitutions and mojibake. Mixed language: look for source-language contamination from another language. Checking category by category ensures each defect family gets a dedicated look rather than relying on whatever happens to be noticed.

### Treat Numbers And Measurements As A Dedicated Check

Numbers are the highest-consequence, lowest-detection-difficulty defects, and they deserve their own focused check rather than being absorbed into a general sweep.

Verify every number, unit, date, time, currency value, and measurement against its recurrences in the document. A quantity stated in a table should match the quantity in the prose; a date in a heading should match the date in the body; a total should match the sum of its parts. Where a number appears once with no cross-check, flag it as unverifiable rather than assuming it is correct. OCR-transcribed numbers are especially dangerous because a misread digit produces a plausible but wrong value that no spell-check will catch. Treat the number check as mandatory in any source containing doses, sums, deadlines, or specifications.

### Detect Terminology Drift By Tracking Terms

Terminology inconsistency is among the most common and most consequential source defects, and it is invisible to a single-pass read because each occurrence reads fine in isolation.

As you read, maintain a running term list. When a concept appears, record the term used. When the same concept reappears with a different term, flag the drift. Conversely, when one term is used for two different concepts, flag the collision. Terminology drift matters because the translator must eventually choose a single target rendering, and choosing without knowing the source is inconsistent produces a target that paper-overs a real source problem. The term list makes drift visible and gives the translator the evidence to decide whether to harmonize, preserve a distinction, or query.

### Hunt For OCR And Encoding Artifacts Deliberately

Scanned and converted documents carry artifacts that look like real text and defeat spell-check, so they require pattern-based detection rather than trust.

Watch for character substitutions that produce valid but wrong characters: rn for m, cl for d, l for I, 0 for O, and vv for w. Watch for broken accents and diacritics, ligature errors, and mojibake from encoding mismatches, where characters appear as nonsense sequences. These artifacts concentrate in numbers, proper nouns, and specialized terms, so scan those zones closely. Where OCR quality is poor, treat every uncertain character as a flag rather than guessing, because a guessed character in a factual field is a fabricated fact.

### Cross-Check Internal Consistency

Many defects are not in any single sentence but in the relationship between sentences. A standalone read misses them; a cross-checking read finds them.

Compare numbers, names, dates, product references, and procedural steps across the document. A feature described in section one may be said to be removed in section four. A figure referenced in the text may not exist. A step in a procedure may contradict a later step. These contradictions are defects even though no individual sentence is wrong, and they often signal that the source is a patchwork of updates. Cross-checking is what turns a collection of correct sentences into a verified document.

### Detect Mixed-Language And Script Contamination

Sources increasingly contain embedded fragments in other languages: English terms inside a non-English document, code-switching, untranslated placeholders, or copy-pasted content from a different language version.

Flag any fragment whose language differs from the source's primary language. Some mixed-language content is intentional, such as a brand name or a legal term kept in the original, and some is contamination from a careless copy-paste. Detection is the first step; deciding which is which comes later. But a mixed-language fragment that goes undetected may be translated as if it were source language, producing nonsense, or left untranslated when it should have been handled.

### Record Defects In A Categorized Log

Detection without recording is wasted effort, because defects noticed but not captured are lost by the time handling decisions are made.

Maintain a defect log that records each defect's location, category, and a brief description. The log becomes the input to the handling phase, where each defect is decided: translate as-is, correct, query, or flag. A defect noticed in passing and not recorded will be forgotten, and the translation will inherit it silently. The log also lets you report source fitness to the requester with evidence rather than impression.

## Common Traps

### Detecting Only What Interrupts Comprehension

A single comprehension pass catches obvious errors and absorbs subtle ones. Run a separate defect pass, because the most dangerous defects do not interrupt meaning.

### Skipping The Number Check

Numbers are high-consequence and low-detection-difficulty, and OCR-transcribed numbers defeat spell-check. Verify every number against its recurrences; never assume a plausible number is correct.

### Missing Terminology Drift In Isolation

Each term occurrence reads fine alone; drift appears only across occurrences. Track terms in a running list to make drift visible.

### Trusting OCR Characters Because They Form Words

OCR substitutions produce valid-looking but wrong characters that pass spell-check. Scan numbers, proper nouns, and specialized terms for substitution patterns.

### Reading Sentences Without Cross-Checking

Contradictions live between sentences, not within them. A collection of correct sentences can still be internally inconsistent; cross-check numbers, names, and steps across the document.

### Detecting Defects But Not Recording Them

A defect noticed but not logged is lost before handling. Record every defect with location and category so it reaches the handling phase.

### Treating Mixed-Language Fragments As Source Language

Embedded foreign fragments translate as nonsense if treated as source language. Detect and flag them first, then decide whether they are intentional or contamination.

## Self-Check

Before approving a source as ready for translation, verify:

- A dedicated defect pass was run separately from the comprehension read.
- The source was swept by defect category, not by impression: typographic, grammatical, factual, terminology, structural, OCR and encoding, and mixed-language.
- Every number, unit, date, time, currency value, and measurement was checked against its recurrences, with single-occurrence numbers flagged as unverifiable.
- A running term list was maintained, and terminology drift and term collisions were detected.
- OCR and encoding artifacts were hunted by pattern, especially in numbers, proper nouns, and specialized terms.
- Internal consistency was cross-checked across the document, including numbers, names, references, and procedural steps.
- Mixed-language and script contamination was detected and flagged for handling decision.
- All detected defects are recorded in a categorized log with location and description.
- No defect that affects meaning was absorbed silently because the source read smoothly.
- The defect log is ready to feed the handling phase, where each defect is decided rather than ignored.
