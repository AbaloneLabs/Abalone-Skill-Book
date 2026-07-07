---
name: terminology_extraction_and_term_harvesting.md
description: Use when the agent is extracting terminology from source content before translation, harvesting candidate terms from corpora and documentation, building a term list from scratch for a new domain or project, or deciding which terms warrant termbase entries and research investment versus general vocabulary that does not.
---

# Terminology Extraction And Term Harvesting

Before terminology can be researched, validated, and enforced, it must be found. Terminology extraction is the upstream discipline of identifying which units in a source text are actually terms, deserving termbase entries and research effort, and which are general vocabulary that the translator handles through ordinary language competence. This sounds simple, and that apparent simplicity is why it is done badly so often. The two characteristic failures are opposite and equally damaging. The first is under-extraction: a translator or terminologist harvests only the obvious capitalized nouns and domain buzzwords, missing the terms embedded in ordinary-looking phrases, the multiword terms, the verbs that carry domain meaning, and the collocations whose translation cannot be guessed. The resulting termbase looks complete but leaves the translator guessing at exactly the units where errors cause harm. The second is over-extraction: every multiword phrase and every slightly technical word is dumped into the termbase as a candidate term, producing a bloated, noisy resource where the real terms are buried under general vocabulary, where maintenance becomes impossible, and where translators stop trusting or using the resource because signal is drowned in noise. Good extraction is a judgment act: it distinguishes term from non-term based on concept-bearing status, domain relevance, and translation risk, and it produces a focused, high-value termbase rather than a comprehensive dump. Agents often treat extraction as a mechanical keyword-finding task, when it is in fact the foundational terminological decision that determines whether downstream research effort is well spent or wasted.

Use this skill when extracting terminology from source content, harvesting candidate terms from corpora or documentation, building a term list for a new domain or project, or deciding what deserves a termbase entry. The goal is to produce a focused, high-value set of terms that captures where translation risk concentrates, not an exhaustive dump of every phrase.

## Core Rules

### Define What Counts As A Term For This Project

Terminology extraction begins with a definition of termhood, because without one, every extractor applies a different standard and the result is inconsistent. Define what counts as a term before harvesting.

A term is a designation of a specialized concept within a domain. The key criteria are that the unit denotes a specific concept with defined boundaries, that the concept belongs to the project's domain, and that the unit's translation carries risk if handled casually. Under this definition, a disease name in a medical text is a term; the verb "to administer" in the same text may or may not be, depending on whether it carries procedural precision. Define the domain boundaries of the project, because the same word is a term in one domain and general vocabulary in another. Define the extraction scope: are you extracting for one document, a product family, or an entire domain program? A clear definition of termhood, written down and shared, lets multiple extractors produce consistent results and lets reviewers evaluate the harvest against a standard rather than against personal instinct.

### Extract By Concept Risk, Not By Surface Salience

The most common extraction error is to harvest what looks like a term, capitalized nouns, jargon, and words in glossaries, while missing terms that do not look special. Extract by the risk a unit carries, not by how technical it appears.

High-risk terms often hide in ordinary-looking language. Multiword terms, such as "adverse event" or "force majeure," are terms even though their components are common words. Verbs and process terms, such as "to reconcile" in accounting or "to titrate" in chemistry, carry domain precision that a casual translation corrupts. Collocations and set phrases, whose translation cannot be derived from the components, are terms in effect even if no single word is technical. Negation-bearing and modality-bearing expressions in legal and regulatory text carry obligation meaning that mistranslation inverts. Harvest by asking, for each candidate, whether a translator guessing at its rendering would risk a conceptually wrong or domain-inappropriate result. If yes, it is a term regardless of how plain it looks. Surface-salient extraction misses exactly the terms that cause silent errors.

### Capture Multiword Terms And Collocations, Not Just Single Words

Single-word extraction misses a large class of terms that are multiword units, and missing them leaves translators to render each component separately, which often produces a wrong or unidiomatic result. Extract multiword units deliberately.

Multiword terms include noun phrases denoting a single concept, such as "informed consent" or "adverse drug reaction"; verb-object collocations with domain meaning, such as "to file a motion" or "to draw blood"; and fixed expressions whose translation is conventional rather than compositional. These units behave as single terms for translation purposes: the whole must be rendered as a unit, and rendering the parts independently produces errors. Extraction tools and methods must capture multiword candidates, using techniques such as n-gram analysis, part-of-speech patterns, and statistical measures of how strongly words co-occur. Manual extraction must consciously look for phrases, not just words. A termbase of single words that omits the multiword terms of a domain is incomplete in the places where compositionality fails and errors concentrate.

### Filter Aggressively To Separate Term From Non-Term

Over-extraction is as damaging as under-extraction, because a bloated termbase buries real terms and becomes unmaintainable. Filter the harvest aggressively to retain only genuine terms.

After candidate extraction, apply filtering criteria. Remove general vocabulary that any competent translator handles without research. Remove candidates that are common-language phrases with no domain-specific meaning. Remove duplicates and near-duplicates that denote the same concept. Remove candidates too rare or too context-bound to warrant a termbase entry. Apply frequency and domain-relevance thresholds, but do not rely on frequency alone, because a rare but high-risk term, such as a specific legal concept appearing once, may deserve an entry while a frequent but low-risk word does not. The filtering step is where judgment matters most, because it determines the signal-to-noise ratio of the termbase. Aim for a focused set where every entry earns its place by carrying translation risk that research and standardization can address.

### Prioritize Terms By Translation Risk And Frequency

Not all extracted terms deserve equal research investment. Prioritize the termbase build so that effort goes where it matters most, using a combination of risk and frequency.

High-priority terms are those that carry high consequence if mistranslated, such as safety, legal, or medical terms, combined with those that appear frequently, because a frequent term's error recurs across the text. A term that is both high-risk and frequent is the top priority: research it thoroughly, validate the equivalent, and enforce it. A term that is high-risk but rare still warrants an entry, because a single error causes harm. A term that is frequent but low-risk may warrant an entry for consistency but needs less research depth. A term that is rare and low-risk may not warrant an entry at all. Build the termbase in priority order, so that if time runs out, the most important terms are done. A termbase built without prioritization often exhausts its budget on easy, frequent, low-risk terms while leaving the dangerous ones unresearched.

### Use Multiple Extraction Methods And Reconcile

No single extraction method catches all terms. Combine automated and manual methods and reconcile their outputs for a more complete harvest.

Automated methods include term-extraction tools that use statistical and linguistic analysis to identify candidates, frequency analysis to find domain-characteristic vocabulary, and n-gram analysis to find multiword units. These methods are fast and consistent but miss terms that are rare, context-dependent, or semantically rather than statistically marked. Manual methods include domain-expert review, where a subject-matter expert identifies terms from their knowledge of what matters in the field, and translator review, where an experienced translator flags terms they know cause difficulty. Manual methods catch what automation misses but are slower and less consistent. Reconcile the outputs: automated candidates verified by human judgment, and manual candidates checked against the corpus for frequency and context. A harvest from one method alone has blind spots that the other method fills.

### Extract With Context, Not As Bare Word Lists

A term extracted as a bare word is only half-captured, because the translator who later uses the termbase needs context to apply the entry correctly. Capture context at extraction time, when it is available, rather than reconstructing it later.

For each term, capture the sentence or phrase where it appears, the domain and subdomain, the part of speech, any variants or inflected forms observed, and notes on usage such as whether it appears in definitions, in procedures, or in references. Context lets the terminologist define the concept accurately, lets the researcher find the right equivalent, and lets the translator apply the entry with confidence. A termbase of bare words forces every user to rediscover the context, which is inefficient and error-prone. Extraction and context capture are more efficient when done together, because going back to the corpus for each term later multiplies effort.

### Validate The Harvest With Domain And Translator Input

An extracted termbase is a hypothesis about what matters. Validate it with the people who know the domain and the translation challenges before treating it as authoritative.

Domain experts can confirm whether the extracted terms are the real concept-bearing units of the field and can flag important terms the extraction missed. Experienced translators can confirm which terms actually cause difficulty in translation and which extracted candidates are trivial. Validation catches both false positives, entries that do not deserve termbase space, and false negatives, important terms that extraction overlooked. Treat the initial harvest as a draft to be refined, not a final product, and document the validation process so the termbase's provenance is clear.

## Common Traps

### Harvesting Only Capitalized And Obvious Jargon

Surface-salient extraction misses multiword terms, process verbs, collocations, and modality-bearing expressions that carry domain meaning without looking technical.

### Dumping Every Phrase Into The Termbase

Over-extraction buries real terms in noise, makes the termbase unmaintainable, and erodes translator trust. Filter aggressively by termhood and risk.

### Extracting Single Words And Missing Multiword Units

Multiword terms behave as single units for translation. A single-word termbase leaves translators to render components separately, producing wrong results.

### Building Without Prioritization

Treating all terms as equal exhausts the budget on easy frequent terms while leaving high-risk ones unresearched. Prioritize by risk and frequency combined.

### Relying On One Extraction Method

Automated methods miss rare and context-dependent terms; manual methods are slow and inconsistent. Combine and reconcile for completeness.

### Capturing Bare Words Without Context

A termbase of words without sentences, domain, and usage notes forces every user to rediscover context. Capture context at extraction time.

### Skipping Validation By Domain Experts And Translators

An unvalidated harvest contains false positives and false negatives. Confirm with the people who know the domain and the translation challenges.

### Confusing Frequency With Importance

A frequent low-risk word is less important than a rare high-risk term. Frequency thresholds alone misprioritize the termbase build.

## Self-Check

Before approving a terminology harvest or termbase build, verify:

- A clear definition of termhood was established for the project, specifying domain boundaries and what counts as a term versus general vocabulary.
- Extraction targeted concept risk, not surface salience, capturing multiword terms, process verbs, collocations, and modality-bearing expressions alongside obvious jargon.
- Multiword terms and collocations were extracted as units, not just single words, so translators render them whole rather than component by component.
- The harvest was filtered aggressively to separate genuine terms from general vocabulary, with a focused, high-signal termbase rather than a comprehensive dump.
- Terms were prioritized by combined translation risk and frequency, so high-risk frequent terms are researched first and budget exhaustion leaves the important terms done.
- Multiple extraction methods, automated and manual, were combined and reconciled to cover each method's blind spots.
- Each entry was captured with context, including source sentence, domain, part of speech, variants, and usage notes, not as a bare word.
- The harvest was validated with domain experts and experienced translators to remove false positives and catch false negatives.
- No high-risk term was omitted because it looked ordinary, and no low-risk phrase was retained because it looked technical.
- The resulting termbase captures where translation risk concentrates in the source, giving downstream research and enforcement a focused, trustworthy foundation.
