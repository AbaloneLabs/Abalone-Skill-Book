---
name: locale_variant_selection.md
description: Use when the agent is selecting which regional variant of a language to translate into, choosing among Spanish Portuguese French English Arabic Chinese German and other pluricentric language variants, resolving ambiguous locale requests, or deciding whether a single pan-regional version is viable or whether separate variants are required.
---

# Locale Variant Selection

Many of the world's major languages are not one language but a family of regional variants, and the variant chosen determines spelling, vocabulary, grammar, politeness, and reader expectation. Spanish is not one language but Mexican, European, Argentine, Colombian, and many more. Portuguese splits between Brazilian and European, and the differences are large enough that a text in one can feel foreign to readers of the other. French divides between France, Canada, Belgium, Switzerland, and African varieties, with Canadian French especially distinct in terminology and institutions. English spreads across the United States, United Kingdom, Ireland, India, Singapore, Australia, South Africa, and more, each with its own spelling, idiom, and convention. Arabic has a formal Modern Standard layer that travels across regions and dialectal layers that do not. Chinese splits between Simplified and Traditional, which differ in character set, vocabulary, and sometimes grammar, and which map to different markets and political contexts. German, while more standardized, still has Austrian and Swiss variants with distinct terms and conventions. Selecting the wrong variant produces text that the target reader recognizes as not belonging to them, and in regulated contexts it can cause non-compliance. The judgment problem is that requesters often ask for a language without specifying a variant, assume one variant serves a whole region, or pick a variant based on cost rather than audience. This skill covers how to select the right variant, how to challenge ambiguous requests, and how to decide between a single pan-regional version and separate variants.

Use this skill when a request specifies a language but not a variant, when choosing among regional variants of a pluricentric language, when deciding whether one version can serve multiple markets, or when confirming the variant matches the actual audience. The goal is to translate into the variant the target reader expects, not a generic approximation of the language.

## Core Rules

### Never Accept A Language Without Resolving The Variant

A request for "Spanish" or "French" or "English" is incomplete. Before drafting, resolve which variant the audience actually reads, because the wrong variant makes otherwise correct text feel foreign.

Begin every engagement by confirming the variant. Ask who the readers are, where they are, and what they expect. If the requester says "Spanish for Latin America," recognize that no single Latin American Spanish exists and that a Mexican reader and an Argentine reader notice differences in pronouns, vocabulary, and register. If the requester says "French," determine whether the audience is in France, Quebec, or francophone Africa. If the variant cannot be confirmed, propose the most likely variant from the audience profile, state the assumption explicitly, and get confirmation before drafting. Translating into an unconfirmed variant and hoping it is close enough is a common and avoidable failure.

### Map The Variant To The Actual Audience, Not To Convenience

Select the variant based on where the readers are and what they expect, not on which variant is cheapest, fastest, or most familiar to the translator.

Cost and speed pressures push toward a single "international" variant or the translator's own variety, but the audience does not experience the text as international or convenient; they experience it as belonging to them or not. A Brazilian Portuguese text sent to Portugal reads with jarring vocabulary and grammar. A Simplified Chinese text sent to Taiwan carries political and script implications. Map the variant to the audience's location and expectation, and where the audience spans multiple variants, plan for multiple variants rather than forcing one. Document the audience-to-variant mapping so the choice is defensible.

### Judge Whether A Pan-Regional Version Is Actually Viable

Sometimes a single version can serve multiple markets; often it cannot. Judge viability honestly rather than defaulting to a pan-regional version to save effort.

A pan-regional version is most viable when the content is short, formal, and technical, where variant differences matter less, and when the audience is tolerant of neutral phrasing. Modern Standard Arabic is the canonical pan-regional case: it travels across Arabic-speaking markets for formal content, though it reads formally and is not how people speak. A pan-regional version is least viable for marketing, consumer-facing, conversational, or culturally sensitive content, where variant differences in idiom, humor, and politeness are most visible. Even "neutral Spanish," a constructed pan-regional variant used in some localization, is a compromise that reads as belonging to no one and that still leaks regionalisms. When considering a pan-regional version, weigh the savings against the cost of text that fits no audience perfectly, and prefer separate variants when the content is audience-sensitive.

### Understand The Real Differences Between Variants

Variant selection requires knowing what actually differs, not just that it differs. Understand the specific dimensions of variation for the language family.

Variation appears in spelling and orthography, such as US versus UK English or the orthographic reforms that distinguish Portuguese variants. It appears in vocabulary, where the same object has different names, sometimes dramatically different or even offensive across variants. It appears in grammar, such as the pronoun and verb systems that distinguish Latin American from European Spanish, or the gerund use that separates Brazilian from European Portuguese. It appears in politeness and address conventions, which vary even within a variant by region and generation. It appears in institutional terminology, where government, legal, educational, and technical terms differ because the institutions differ. Know these dimensions for the language family you work in, and verify the specific choices against the target variant rather than assuming your own variety applies.

### Watch For Terms That Are Neutral In One Variant And Offensive In Another

The most dangerous variant errors are terms that are innocuous in one variety and vulgar, offensive, or politically charged in another. These errors can damage brands and cause real harm.

A word that is a common object in Spain may be vulgar in Mexico. A term that is friendly in one English variety may read as condescending in another. A Simplified-versus-Traditional choice carries political signaling that the requester may not intend. Product and brand names that work in one market may have unfortunate meanings in another variant of the same language. When translating for a variant that is not your native variety, verify locale-sensitive terms with a native reviewer or a variant-specific reference. Do not assume that because a word is acceptable in your variety it is acceptable in the target variant.

### Coordinate Variant Choice Across The Program

Variant choice is not per-file; it must be consistent across a product, a campaign, and a content program. Coordinate so that variant decisions hold across deliverables.

If a product is localized into Brazilian Portuguese, every user-facing string, help article, marketing asset, and support reply should be Brazilian Portuguese, not a mix. Maintain the variant decision in the termbase and style guide so every translator and vendor applies the same variant. Where multiple variants are produced, keep them separate in the workflow so segments do not bleed between variants, because cross-variant contamination produces inconsistent text that confuses readers. Track variant as a project metadata field so files are routed to the right linguists and reviewers.

### Handle Variant Mismatches In Source And Assets

Sometimes the source or the existing assets are in a different variant than the target requires. Handle the mismatch deliberately rather than inheriting it.

A source written in UK English being localized into US English requires systematic orthographic and lexical conversion, not just translating the meaning. An existing translation memory in European Spanish being applied to a Mexican Spanish project will leak European terms unless reviewed. Translation memory and termbase matches from the wrong variant must be verified and corrected, not accepted on faith. Flag variant mismatches in assets so they are resolved, because inherited variant errors are a common source of inconsistent localized products.

## Common Traps

### Accepting A Language Without A Variant

Translating into an unspecified variant and hoping it is close enough produces text that fits no audience precisely.

### Choosing The Variant For Cost Or Familiarity

Selecting the translator's own variety or a single "international" version to save effort ignores what the audience actually expects.

### Assuming A Pan-Regional Version Always Works

Neutral or pan-regional variants read as belonging to no one and leak regionalisms; they fail for audience-sensitive content.

### Assuming Your Variety Applies Everywhere

Terms neutral in one variant can be vulgar or offensive in another; verify against the target variant, especially when it is not your native variety.

### Ignoring Political And Script Signaling

Simplified versus Traditional Chinese and similar choices carry political meaning; the choice must match audience and intent.

### Letting Variants Bleed Across A Program

Inconsistent variant use across a product, mixing Brazilian and European Portuguese, confuses readers and signals carelessness.

### Trusting Translation Memory From The Wrong Variant

Matches from a different variant leak foreign terms unless reviewed; verify inherited assets against the target variant.

## Self-Check

Before approving a variant selection, verify:

- The variant was confirmed against the actual audience location and expectation, not left as an unspecified language.
- Selection was driven by audience fit, not by cost, speed, or the translator's familiar variety.
- If a pan-regional version was chosen, its viability was judged honestly for the content type, with separate variants preferred for audience-sensitive material.
- The specific dimensions of variation, spelling, vocabulary, grammar, politeness, and institutional terminology, were understood and applied for the chosen variant.
- Locale-sensitive terms were verified for the target variant, with particular care when the variant is not the translator's native variety.
- Political and script implications, such as Simplified versus Traditional, were considered and matched to intent.
- Variant choice is consistent across the program, recorded in the termbase and style guide, with variants kept separate in the workflow.
- Variant mismatches in source and inherited assets were identified and resolved rather than propagated.
- The selected variant is the one the target reader expects, not a generic approximation of the language.
