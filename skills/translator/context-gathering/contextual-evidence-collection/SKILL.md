---
name: contextual_evidence_collection.md
description: Use when the agent is collecting contextual evidence to resolve ambiguity and inform translation choices, gathering screenshots surrounding text prior versions and usage examples, distinguishing evidence from assumption, or building a documented evidence trail that supports each uncertain rendering rather than leaving it to intuition.
---

# Contextual Evidence Collection

Translation decisions are only as good as the evidence behind them, and evidence is rarely handed over complete. A UI string arrives as an isolated fragment, and its correct rendering depends on where it appears, what surrounds it, and what the user does next. An ambiguous pronoun in a contract can be resolved only by evidence from elsewhere in the document. A term's intended sense depends on usage evidence from comparable texts. When the translator resolves these uncertainties from intuition alone, the resolution is a guess dressed as a decision, and no one can tell how it was reached or whether to trust it. Contextual evidence collection is the discipline of gathering the concrete evidence that resolves uncertainty, screenshots, surrounding text, prior versions, usage examples, and cross-references, and recording it so that each uncertain rendering rests on a visible, checkable basis rather than on the translator's feeling. The difference between a defensible translation and an improvised one is often not skill but evidence.

Agents collect evidence inconsistently because intuition feels sufficient in the moment. A fluent translator reads an ambiguous segment, senses the likely meaning, and moves on; gathering evidence for a single segment feels like overkill. But intuition is invisible and unchallengeable, and when it is wrong, no reviewer can detect the error because no evidence marks the decision as uncertain. Evidence collection converts invisible intuition into visible reasoning. It does not slow down every segment; it targets the segments where uncertainty exists, and for those segments it produces a record that lets a reviewer confirm or overturn the choice. The goal is not to document everything but to ensure that every uncertain rendering has a basis someone else can inspect.

Use this skill when a source contains ambiguity or gaps that require evidence to resolve, when isolated segments lack surrounding context, when building a documented basis for uncertain choices, or when preparing a translation whose decisions must be defensible at review. The goal is to ground uncertain renderings in collected evidence rather than intuition.

## Core Rules

### Identify Which Segments Need Evidence

Not every segment requires evidence collection; most renderings are straightforward. The discipline is to identify the segments that do need evidence and concentrate effort there.

Segments that need evidence include those with ambiguity, where more than one reading is possible; those with gaps, where information is missing; isolated fragments, such as UI strings, where surrounding context is absent; terms with multiple senses, where the intended sense must be determined; and references, where the antecedent or target must be located. Mark these segments as evidence-dependent as you encounter them. A translation that collects evidence for uncertain segments and moves efficiently through certain ones is both defensible and efficient; one that documents everything drowns the real decisions in noise, and one that documents nothing leaves every decision ungrounded.

### Gather Surrounding And Structural Context

The most immediate evidence for an ambiguous segment is the text around it and the structure it sits in. Gather that before looking further.

For an ambiguous pronoun, read the surrounding sentences and the paragraph's topic to determine the antecedent. For an isolated UI string, request or locate the screenshot showing where it appears, what precedes and follows it, and what action it triggers. For a clause in a contract, locate the defined terms and cross-references it depends on. Structural context, section hierarchy, list position, table relationships, often resolves ambiguity that a linear read misses. Surrounding and structural context is the first and cheapest evidence layer, and it resolves a large share of uncertainties before any external research is needed.

### Collect Usage Evidence For Sense Disambiguation

When a term has multiple senses, the intended sense is determined by usage evidence, not by the translator's preference. Collect that evidence.

Look for how the term is used elsewhere in the same source, since the author's own usage is strong evidence of intended sense. Look for usage in comparable texts from the same domain and client. Where a termbase or prior translation exists, check the established sense. Usage evidence is what distinguishes the sense the author meant from the sense the translator finds familiar. Without it, sense choice is a guess; with it, the choice is grounded in how the term is actually used. Record the evidence, the comparable usages or termbase entries, so the sense decision is inspectable.

### Use Prior Versions And Revision History As Evidence

When a source is part of a versioned document or an evolving product, prior versions and revision history are powerful evidence for intended meaning.

A term that seems ambiguous in the current version may be clearer in a prior version before it was edited. A string that changed between versions reveals what the author was refining toward. Revision history, where available, shows what was added, deleted, and modified, and changes often clarify intent that the final text obscures. Prior versions are especially valuable for contract and regulatory text, where wording evolves deliberately and the evolution carries meaning. Where version history is accessible, use it as evidence; where it is not, ask the client, because the history may exist even if it was not provided.

### Distinguish Evidence From Assumption

The most important discipline in evidence collection is keeping evidence separate from assumption, because they look similar and behave differently. Evidence is external and checkable; assumption is internal and unchallengeable.

For each uncertain rendering, record what evidence supports it: the surrounding text, the usage examples, the prior version, the screenshot. Separately, record what was assumed where evidence was insufficient. An assumption is not invalid; translators must sometimes proceed on incomplete evidence. But an assumption labeled as evidence is dangerous, because it presents a guess as a verified fact and removes the uncertainty from view. The evidence-assumption distinction lets a reviewer see which decisions are grounded and which are provisional, and it lets the translator return to assumptions when more evidence becomes available.

### Document The Evidence Trail For Uncertain Renderings

Evidence that is gathered but not documented is lost before review. Document the evidence trail for every uncertain rendering.

For each evidence-dependent segment, record the segment, the uncertainty, the evidence gathered, the decision made, and the basis. A translator note such as "pronoun could refer to the company or the product; surrounding paragraph discusses the company's policy, so chose the company" documents the trail. The documentation need not be exhaustive for low-stakes cases, but it must exist for any rendering where the choice could be wrong and where a reviewer would benefit from seeing the reasoning. The evidence trail is what makes a translation defensible: it shows that uncertain decisions were made deliberately and can be inspected.

### Escalate When Evidence Is Insufficient

Sometimes evidence cannot resolve the uncertainty, and the correct response is to escalate rather than to choose on inadequate evidence. Recognize that limit.

When surrounding context, usage, and prior versions all fail to resolve a high-stakes ambiguity, the decision exceeds what evidence collection can support. Escalate to the requester for intent or to a subject-matter expert for correctness, and present the evidence gathered so the authority can decide efficiently. Escalation with evidence is far more effective than escalation without it, because the authority can confirm a recommended resolution rather than starting from scratch. Forcing a choice on insufficient evidence in high-stakes content is how confident errors are made.

## Common Traps

### Resolving Uncertainty From Intuition Alone

Intuition is invisible and unchallengeable, and when wrong it produces undetectable errors. Collect evidence for uncertain segments rather than trusting feel.

### Documenting Everything Equally

Treating every segment as evidence-dependent drowns real decisions in noise. Target evidence collection at genuinely uncertain segments.

### Skipping Surrounding And Structural Context

The cheapest and most immediate evidence layer is the text around the segment and the structure it sits in. Gather it before external research.

### Choosing Sense By Familiarity

A term's familiar sense may not be the intended one. Use usage evidence, the author's own usage and comparable texts, to disambiguate.

### Ignoring Prior Versions

Prior versions and revision history often clarify intent that the final text obscures. Use version evidence where it is accessible.

### Labeling Assumptions As Evidence

An assumption presented as evidence removes uncertainty from view and becomes a silent error. Keep evidence and assumption distinct in the record.

### Forcing A Choice On Insufficient Evidence

When evidence cannot resolve a high-stakes uncertainty, escalate with the evidence gathered rather than choosing on inadequate basis.

## Self-Check

Before approving a translation that involved uncertain renderings, verify:

- Segments requiring evidence, those with ambiguity, gaps, isolation, multiple senses, or references, were identified and marked.
- Surrounding and structural context was gathered as the first evidence layer for each uncertain segment.
- Usage evidence, including the author's own usage and comparable texts, was collected for sense disambiguation.
- Prior versions and revision history were used as evidence where accessible, especially for evolving contract and regulatory text.
- Evidence is distinguished from assumption in the record, with assumptions labeled rather than presented as verified fact.
- An evidence trail is documented for each uncertain rendering, stating the uncertainty, evidence, decision, and basis.
- Segments where evidence was insufficient were escalated with the gathered evidence rather than resolved on inadequate basis.
- No high-stakes rendering rests on intuition alone without a documented evidence basis.
- A reviewer inspecting the evidence trail could confirm or overturn each uncertain decision.
- The translation's uncertain decisions are defensible, not improvised.
