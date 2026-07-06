---
name: style_guide_adaptation.md
description: Use when the agent is translating or localizing a source-language style guide into a target language, adapting editorial rules that rely on source-language features, deciding which rules transfer which need replacement and which must be dropped, or producing a target-language style guide that native writers and translators can actually follow.
---

# Style Guide Adaptation

A style guide written in one language cannot be translated into another the way an article is translated. Its rules are built on the features of the source language: its punctuation conventions, its grammar of address, its sentence structure, its register system, its capitalization logic. Many of those features do not exist, or exist in a different form, in the target language. Adapting a style guide is the work of deciding, rule by rule, whether a source rule transfers directly, needs to be replaced with a target-language-specific rule that serves the same intent, or must be dropped because it has no target equivalent. It is a localization of a normative document, and it is harder than localizing prose because the output must function as an authority that native writers and translators obey. The harm this skill prevents is a target style guide that is either a useless literal translation whose rules do not apply to the target language, or an arbitrary rewrite that abandoned the source guide's intent and left each market to invent its own conventions.

Agents miss this work because translating a style guide looks like translating any document. It is not. A rule that is faithfully translated but inapplicable is worse than no rule, because it looks authoritative while being unworkable, and writers either ignore it or follow it into awkward target prose.

## Core Rules

### Classify Every Rule Before Translating It

Go through the source style guide rule by rule and classify each into one of three categories before doing anything else. A rule transfers directly when the target language has an equivalent feature and convention, such as a rule to use sentence case for headings, which may map cleanly. A rule needs replacement when the source feature has no direct equivalent but the intent does, such as an English rule about contractions mapping onto a target-language rule about formality or pronoun choice. A rule must be dropped when it is entirely source-specific with no target relevance, such as a rule about a punctuation mark the target language does not use.

Classifying first prevents the two failure modes of wholesale literal translation and wholesale rewrite. You translate the transferable rules, author replacements for the intent-bearing ones, and document the dropped ones with rationale.

### Preserve The Intent, Replace The Mechanism

When a rule needs replacement, preserve the intent the rule serves and express it through the target language's mechanisms. The intent of an English rule favoring the active voice may be clarity and directness; in a target language where passive constructions carry different connotations, the intent is preserved by a rule about agent prominence or sentence focus rather than by mechanically forbidding passives. The intent of a rule about contractions may be an informal, approachable register; in a language without contractions, that intent is preserved by rules about pronoun choice, verb form, or sentence length.

Always record the source intent alongside the target rule, so future maintainers understand why the target rule exists and can revise it coherently when the language evolves.

### Handle Rules That Rely On Source-Specific Features

Some rules are untranslatable because they depend on features absent from the target language. English capitalization rules, the serial comma, split-infinitive debates, gendered pronoun strategies, and rules about articles or contractions may have no target equivalent. For each, decide whether the underlying editorial concern, clarity, consistency, inclusivity, rhythm, has a target-language expression, in which case replace it, or whether it is genuinely source-specific, in which case drop it and say so.

Do not translate a source-specific rule into a target rule that invents a constraint the target language never had. That creates artificial prescriptions that make target prose worse.

### Localize Format And Convention Rules To Target Norms

Rules about dates, numbers, units, currency, quotation marks, dashes, spacing, lists, and headings are locale-specific and must be localized to target norms, not translated. A rule prescribing month-day-year date format is wrong in most target locales; replace it with the target locale's date convention. A rule about curly versus straight quotation marks maps to the target locale's quotation mark conventions, which differ widely.

Confirm the target locale's conventions against authoritative sources rather than assuming, because conventions vary even within a language across regions. State the locale explicitly in the adapted guide so the rules are not misapplied to a different region.

### Author Target Examples, Do Not Translate Source Examples

Examples are the part of a style guide most often mishandled. A translated source example frequently demonstrates the rule badly, because the example's effectiveness depends on source-language features. For each rule, author fresh target-language examples that demonstrate the rule naturally in the target language, showing both the preferred form and the form to avoid.

Use authentic target prose for examples, not constructed sentences that sound artificial. Examples that do not read like real target writing undermine the guide's credibility and are ignored by writers.

### Resolve Conflicts Between Source Rules And Target Conventions

Source style rules sometimes conflict with strong target-language conventions. A source rule demanding directness may clash with a target culture that reads directness as rudeness. A source rule favoring brevity may conflict with a target language whose natural expression is more expansive. In these cases, decide deliberately whether to impose the source rule, adapt it, or defer to target convention, and document the decision and its rationale.

Default to respecting target convention where the source rule would damage credibility or naturalness, and document the divergence so the brand understands where and why its voice adapts per market.

### Make The Adapted Guide Usable As An Authority

The output must function as a working authority, not as a translation artifact. Organize it so native writers can find rules quickly, write rules as imperatives they can obey, keep it in the target language throughout, and ensure internal consistency so no two rules contradict. Include a short explanation of how the adapted guide relates to the source guide, which rules transferred, which were replaced, and which were dropped, so users understand the relationship.

A guide that is hard to navigate, internally contradictory, or half in the source language will not be followed, regardless of how well its individual rules are adapted.

### Validate With Native Writers And Subject The Draft To Use

An adapted style guide is a hypothesis until native writers try to follow it. Validate the draft by having native translators and writers apply it to real content and report where rules were unclear, inapplicable, or produced awkward results. Revise based on that feedback. A guide validated only by the adapter, who is too close to the source, typically contains rules that fail in practice.

Plan for revision, because target conventions and brand voice evolve, and a guide that is never revisited becomes stale.

## Common Traps

### Translating The Guide Wholesale

Literal translation produces rules that do not apply to the target language and look authoritative while being unworkable. Classify each rule first.

### Abandoning Source Intent In Rewrites

Replacing rules without preserving intent leaves each market to invent conventions and fragments the brand. Record source intent with each target rule.

### Inventing Target Constraints From Source Features

Translating a source-specific rule into a target constraint the language never had creates artificial prescriptions that worsen prose. Drop genuinely source-specific rules.

### Translating Format And Convention Rules

Date, number, quotation, and list rules are locale-specific and must be localized to target norms, not translated. Confirm against authoritative sources.

### Translating Source Examples

Translated examples demonstrate rules badly because they rely on source features. Author fresh target examples in authentic target prose.

### Imposing Source Rules That Clash With Target Convention

Forcing a source rule that conflicts with target expectations damages credibility. Default to target convention and document the divergence.

### Delivering A Guide That Is Not Usable As An Authority

A guide that is hard to navigate, contradictory, or half in the source language will not be followed. Organize and write it as a working authority.

### Skipping Validation By Native Writers

An unvalidated guide typically contains rules that fail in practice. Test it with native writers on real content and revise.

## Self-Check

Before approving an adapted target-language style guide, verify:

- Every source rule was classified as transfer, replace, or drop before any translation or authoring.
- Rules needing replacement preserve the source intent and express it through target-language mechanisms, with intent recorded alongside each target rule.
- Source-specific rules with no target relevance were dropped with documented rationale rather than translated into artificial constraints.
- Format and convention rules, dates, numbers, units, quotation marks, lists, were localized to the explicit target locale's norms, not translated.
- Examples were authored fresh in authentic target prose demonstrating each rule, not translated from source examples.
- Conflicts between source rules and target conventions were resolved deliberately, defaulting to target convention where the source rule would harm credibility, with divergence documented.
- The adapted guide is organized, internally consistent, fully in the target language, and usable as a working authority, with an explanation of its relationship to the source.
- The draft was validated by native writers applying it to real content, and revised based on their feedback.
- No rule is a faithful translation of an inapplicable source rule, and no target rule was invented without a clear source intent or target convention to ground it.
