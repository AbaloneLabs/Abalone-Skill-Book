---
name: source_ambiguity_and_defect_resolution.md
description: Use when the agent is resolving specific ambiguities defects or errors found in a source text before or during translation, deciding whether to fix preserve or query source issues, handling source contradictions and incomplete content, managing translator queries to source authors, or determining when a source defect requires escalation rather than silent correction.
---

# Source Ambiguity And Defect Resolution

Source texts are rarely perfect. They contain ambiguities (a pronoun with an unclear antecedent, a term with multiple meanings, a sentence with two valid parses), contradictions (a number stated one way in the text and another in a table), omissions (a referenced figure that is missing, a step that is skipped), and outright errors (a wrong date, a misspelled name, a broken link). For a translator, each of these is a decision point: should the defect be fixed silently, preserved as-is, queried to the source author, or escalated? The wrong default is dangerous. Silent correction can mask a real error that the source owner needs to know about and can introduce the translator's own misinterpretation. Preserving every defect produces target text that is as broken as the source. Querying everything overwhelms the source author and stalls the project. The skill is in triaging each defect to the right handling, documenting the decision, and knowing when escalation is required rather than optional.

Agents tend to miss that defect handling is a judgment call with consistency implications across languages, that silent correction hides errors the owner must address, that some defects are high-stakes (a wrong dosage, a contradictory legal obligation) and require escalation, and that the query process itself needs structure to be efficient. The harm is target text that propagates source errors, target text that diverges from the source owner's intent because the translator guessed wrong, or a stalled project because queries were mishandled.

Use this skill when resolving ambiguities, defects, or errors in a source text during translation, deciding whether to fix preserve or query, handling contradictions and omissions, managing translator queries, or determining when escalation is required. The goal is to handle each source defect deliberately and consistently, protecting both target quality and the source owner's awareness of problems in their content.

## Core Rules

### Classify Each Defect Before Choosing A Handling

Before deciding what to do with a source defect, classify it. The classification determines the appropriate handling and prevents the two opposite failures: fixing everything silently (hiding errors and risking misinterpretation) and preserving everything (propagating broken content). Classify defects into categories that map to handling strategies.

Classify by type: ambiguity (multiple valid meanings), factual error (a number, date, or name that is wrong), contradiction (two parts of the source that conflict), omission (something referenced but missing), formatting or structural defect (broken numbering, inconsistent heading levels), and translatability issue (an idiom or structure that does not transfer). Classify by severity: critical (could cause harm, legal liability, or significant user confusion if translated as-is or mistranslated), moderate (causes inconsistency or extra effort but not harm), and minor (cosmetic). The type-plus-severity classification drives the handling decision. A critical factual error requires escalation; a minor formatting defect may be silently normalized.

### Apply The Fix-Preserve-Query-Escalate Decision Framework

For each classified defect, choose a handling from four options. Fix silently when the defect is unambiguous and low-stakes: a clear typo, an obvious formatting normalization, a grammatical error with only one correct resolution. The correction is confident and does not change meaning. Preserve when the defect is intentional or when correcting it risks changing the source owner's intent: intentional ambiguity in legal or literary text, deliberate wordplay, or a source convention that the target should mirror. Query when the defect has more than one plausible resolution and the translator cannot confidently determine the intended meaning: an ambiguous pronoun, a term with multiple meanings, a contradiction between text and table. Escalate when the defect is high-stakes and the translator's resolution could cause harm or liability: a dosage that appears wrong, a legal obligation that contradicts itself, a safety instruction that is ambiguous.

Document each decision. For silent fixes, note what was changed and why. For preserved defects, note that the defect was intentional or that correction was unsafe. For queries, record the query and the response. For escalations, record the issue, the stakeholder notified, and the resolution. Documentation ensures that the same defect is handled consistently if it recurs and that the translation's relationship to the source is traceable.

### Resolve Ambiguity By Confirming Intent, Not By Guessing

Ambiguity is the most common defect, and the temptation is to pick the most likely meaning and translate it. This is guessing, and when the guess is wrong, the error is invisible to the target reader. The correct approach is to confirm the intended meaning through evidence: context elsewhere in the document, parallel texts, domain knowledge, or a query to the source author.

When confirming through context, look for disambiguating information elsewhere: does a later sentence clarify who a pronoun refers to? Does a diagram resolve a spatial ambiguity? Does the document's purpose narrow the meaning? When context is insufficient, query the source author with a specific question (not a general "what do you mean?"). Record the confirmed meaning so it is applied consistently. If the ambiguity cannot be resolved and is not high-stakes, preserve the ambiguity in the target where the target language allows it, or choose the most likely meaning and flag it as uncertain. Never silently resolve a high-stakes ambiguity; escalate it.

### Handle Contradictions By Identifying The Authoritative Version

Contradictions occur when the source states something differently in two places: a number in the text differs from the same number in a table, a procedure in one section conflicts with a procedure in another, a term is defined one way and used another. Translating both versions faithfully propagates the contradiction into the target. The translator must determine which version is authoritative.

Identify the authoritative version by checking which is more recent, which is more detailed, which appears in a more formal or controlled location (a defined terms section over running text), or by querying the source author. Translate the authoritative version and note the contradiction for the source owner so they can fix the source. If the authoritative version cannot be determined, translate both faithfully and add a translator note flagging the contradiction, or query before proceeding. Do not silently pick one version, because if the wrong one is chosen, the target will be wrong and the contradiction will remain hidden in the source.

### Manage The Query Process Efficiently

Queries are necessary but costly: each query interrupts the translator's flow, requires the source author's time, and can stall the project if responses are slow. An efficient query process batches queries, structures them clearly, and tracks them to resolution.

Batch queries rather than sending them one at a time. Structure each query with the location (segment or page), the source text, the issue, and the specific question (offer options where possible: "Does X refer to A or B?"). Track queries in a log with status (open, answered, escalated) so none are lost. Set a response deadline and a default handling if no response arrives (for non-critical queries, proceed with the best guess and flag it; for critical queries, block until resolved). Centralize queries across target teams so the same question is asked once and the answer is shared, preventing inconsistent resolutions across languages.

### Escalate High-Stakes Defects Rather Than Resolving Them

Some defects are beyond the translator's authority to resolve because the consequence of error is too high. A dosage that appears wrong in a medical text, a financial figure that contradicts another source, a safety instruction that is ambiguous, a legal clause that conflicts with itself: these require escalation to a qualified reviewer or the source owner, not translator judgment.

Escalation means stopping translation of the affected segment, documenting the issue with its severity and potential consequences, notifying the stakeholder with authority to resolve it, and waiting for guidance before proceeding. Do not translate a high-stakes defect based on the translator's best guess, even if the guess is probably right, because "probably right" is not sufficient when the consequence of being wrong is harm or liability. The escalation threshold depends on content risk: legal, medical, safety, and financial content have a low threshold for escalation, while marketing and internal content have a higher threshold.

### Ensure Consistent Handling Across Languages And Revisions

Source defects that are not fixed in the source must be handled consistently across all target languages. If one team resolves an ambiguity one way and another team resolves it differently, the targets diverge. Centralize defect handling decisions through a shared query log, a source issue note, or a translation brief addendum that documents how each defect was resolved.

When the source is revised to fix a defect, ensure all in-flight and future translations use the revised source. Track which source version each translation is based on, and when a defect is fixed in the source, notify all teams so they can update their translations if needed. Consistency across languages and revisions requires that defect decisions are documented, shared, and version-tracked.

## Common Traps

### Fixing Everything Silently

Silent correction hides errors the source owner needs to know about, risks introducing the translator's misinterpretation, and prevents the source from being improved. Fix only confident, low-stakes corrections, and document them.

### Preserving Every Defect

Preserving all defects produces target text as broken as the source. Some defects should be fixed or queried. Preserve only intentional defects or those where correction is unsafe.

### Guessing At Ambiguity Instead Of Confirming

Picking the most likely meaning without evidence is guessing. When the guess is wrong, the error is invisible. Confirm through context, parallel texts, or queries, and flag uncertainty.

### Silently Picking One Side Of A Contradiction

Choosing one version of a contradiction without determining which is authoritative risks translating the wrong version. Identify the authoritative version or query, and note the contradiction for the source owner.

### Sending Queries One At A Time Without Structure

Unstructured, piecemeal queries interrupt flow, overwhelm the source author, and get lost. Batch, structure, track, and set deadlines for queries.

### Resolving High-Stakes Defects Without Escalation

Translating a high-stakes defect based on best guess, even a confident one, risks harm or liability. Escalate defects where the consequence of error is high.

### Letting Each Target Team Resolve Defects Independently

Without centralized defect documentation, each team resolves ambiguities differently, producing inconsistent targets. Centralize decisions and share them across all teams.

## Self-Check

- [ ] Has each source defect been classified by type (ambiguity, factual error, contradiction, omission, formatting, translatability) and severity (critical, moderate, minor)?
- [ ] Has the fix-preserve-query-escalate framework been applied to each defect, with the handling chosen based on classification rather than default?
- [ ] Have ambiguities been resolved by confirming intent through context, parallel texts, or queries, rather than by guessing the most likely meaning?
- [ ] Have contradictions been resolved by identifying the authoritative version, with the contradiction noted for the source owner?
- [ ] Has the query process been structured with batching, clear formatting, a tracking log, response deadlines, and default handling for non-responses?
- [ ] Have high-stakes defects (dosage errors, contradictory legal obligations, ambiguous safety instructions) been escalated rather than resolved by translator judgment?
- [ ] Have defect handling decisions been documented and centralized so all target teams resolve the same defects consistently?
- [ ] Has the source version been tracked, with notifications sent to all teams when a defect is fixed in the source?
- [ ] Has the defect resolution log been maintained with location, issue, handling, and rationale so the translation's relationship to the source is traceable?
