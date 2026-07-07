---
name: autocomplete_and_query_understanding.md
description: Use when the agent is designing or building autocomplete/typeahead, search-as-you-type, query suggestion, spell correction, query rewriting and expansion, query intent classification, or query understanding for a search system; deciding suggestion ranking and sourcing, handling partial and noisy queries, managing latency budgets for sub-second suggestion, or building the query-rewrite layer that transforms a user's input before retrieval. Also covers the failure modes of suggestions that embarrass or mislead, spell correction that over-corrects proper nouns or intentional terms, query expansion that drifts from intent, latency that makes suggestions arrive too late to be useful, and the recurring mistake of treating autocomplete as a trivial prefix lookup when its suggestion quality, latency, and safety materially shape the search experience.
---

# Autocomplete And Query Understanding

Autocomplete and query understanding are the front door to search: the user types a few characters, and within tens to a few hundred milliseconds the system must suggest likely completions, correct errors, and understand what they probably mean — all before they finish typing. The judgment problem is that this looks like a trivial prefix lookup ("match strings starting with these characters") but is in fact a tight-latency, quality-critical, safety-sensitive prediction problem. Suggestions must be relevant to the partial input (hard, because the input is incomplete and noisy), must arrive fast enough to be useful (a suggestion that arrives after the user typed the next character is useless), and must not embarrass or mislead (a suggestion that is offensive, defamatory, or commercially manipulative damages trust more than no suggestion). Query understanding — rewriting, expanding, classifying intent — transforms the query before retrieval, and getting it wrong (expanding a precise query into noise, correcting an intentional term, misclassifying intent) degrades results that the ranking layer cannot recover. The discipline is to treat autocomplete and query understanding as a prediction and ranking problem with hard latency and safety constraints, not as a string-matching utility.

Agents tend to under-invest here because a prefix lookup over a term dictionary works in the demo, and suggestions appear instantly for common queries. The harm appears on real, partial, noisy input. A suggestion ranked by raw popularity surfaces an embarrassing or off-brand completion. A spell correction "fixes" a proper noun or an intentional technical term, changing the query's meaning. A query expansion broadens a precise query into irrelevant results. The latency budget is consumed by a slow suggestion source, and suggestions arrive after the user has typed past them. A suggestion leads the user to a query with no results, because the suggestion was not validated against the corpus. The judgment problem is to source and rank suggestions for relevance and safety, correct and expand queries carefully (with respect for intent), meet a tight latency budget, and ensure suggestions lead somewhere useful.

This skill covers suggestion sourcing and ranking, spell correction and query rewriting, latency, safety, and evaluation. It complements the relevance-ranking skill (ranking results, not suggestions), the search-index-design skill (the corpus), and the multilingual-search skill (language-specific query handling). Here the focus is the query-time front end: suggestions, correction, and understanding.

## Core Rules

### Source And Rank Suggestions For Relevance To The Partial Query

A suggestion's job is to predict what the user likely wants, given incomplete input. Sourcing and ranking determine relevance:

- **Source suggestions from multiple signals.** Popular queries (what others searched), the corpus's own terms (what exists to find), user history (what this user searched before), and promotional/editorial suggestions each contribute. Match the source mix to the use case (popularity for general search, corpus terms for catalog search where only existing items are findable).
- **Rank by relevance to the partial input, not raw popularity.** The most popular query overall may be irrelevant to the specific prefix; rank by a combination of prefix match, popularity, recency, and (where available) the user's context. A suggestion ranked only by global popularity surfaces the same completions regardless of input.
- **Validate suggestions against the corpus.** A suggestion that leads to zero results frustrates the user and wastes a round-trip; prefer suggestions that have results, or mark low-yield suggestions. For catalog or inventory search, only suggest what exists.
- **Personalize carefully and transparently.** User history can improve suggestions, but personalization that feels invasive ("why does it know I searched that?") damages trust; scope personalization to what the user expects and allow it to be disabled.

### Correct Spelling And Rewrite Queries With Respect For Intent

Spell correction and query rewriting transform the user's input, and over-eager transformation changes meaning. The principle is to correct errors, not intent:

- **Correct likely errors, preserve intentional terms.** A typo ("recieve") should be corrected; a proper noun, a technical term, or an intentional spelling (a brand name, a code) should not. Use confidence thresholds and a dictionary of protected terms (brands, entities, domain vocabulary) to avoid over-correcting.
- **Offer correction rather than forcing it, especially for ambiguous cases.** For low-confidence corrections, run the original query and offer "did you mean X?" rather than silently replacing the query. Silent replacement of an intentional query is a trust-damaging failure.
- **Expand queries to improve recall, but bound the expansion.** Query expansion (synonyms, stemming, related terms) improves recall for queries that would otherwise miss relevant results, but unbounded expansion drifts from intent and introduces noise. Apply expansion where it helps (rare terms, synonyms) and measure its effect, rather than expanding everything.
- **Classify intent to route the query, not to override it.** Intent classification (is this a navigational, informational, or transactional query?) can route the query to the right retrieval or ranking path, but misclassification that overrides the user's explicit query is worse than no classification. Use intent as a signal, not a override.

### Meet A Tight Latency Budget Or The Feature Is Useless

Autocomplete's value collapses if suggestions arrive too late. The latency budget is measured in tens to a few hundred milliseconds, and every millisecond of budget must be spent deliberately:

- **Set and enforce a latency budget for suggestions.** Define the target (e.g., p99 under 100ms) and treat it as a hard constraint; a suggestion that arrives after the user typed the next character is wasted work and wasted cost.
- **Keep the suggestion path lean.** Suggestion retrieval must be fast (an in-memory index, a prefix tree, a cached suggestion store); avoid expensive computation or slow dependencies on the suggestion path. Move heavy work (re-ranking, personalization) offline or cap it within the budget.
- **Gracefully degrade under load or slowness.** If a suggestion source is slow, return faster, lower-quality suggestions (or none) rather than blowing the budget; a fast, slightly-less-relevant suggestion beats a late perfect one. Time-box each suggestion source.
- **Measure latency at the user's experience, not the server's.** The relevant latency is from keystroke to rendered suggestion (including network, rendering, and the user's typing speed), not the server's processing time. Instrument the full path.

### Govern Suggestion Safety And Quality

Suggestions are visible, prominent, and attributable to the system, which makes their quality and safety a first-class concern:

- **Filter offensive, harmful, or brand-damaging suggestions.** A suggestion that is profane, defamatory, hateful, or commercially manipulative damages trust far more than a missing suggestion. Maintain blocklists, moderation, and review for suggestion content, especially for user-generated suggestion sources.
- **Avoid suggestions that mislead or manipulate.** Suggestions that steer users commercially (biased toward paid placements presented as neutral suggestions) or that present misinformation as fact erode trust and may invite regulatory scrutiny. Separate sponsored from organic suggestions clearly.
- **Handle sensitive queries carefully.** Some queries (health, financial, personal crisis) warrant special handling — surfacing authoritative results, suppressing risky suggestions, or declining to suggest at all. Treat sensitive domains with deliberate policy, not the default ranking.
- **Review and audit suggestions regularly.** Suggestion content changes as the query stream evolves; audit for emerging harmful or off-brand completions, especially for user-sourced suggestions.

### Evaluate Autocomplete And Query Understanding On The Right Metrics

Evaluation must reflect what the feature is for: relevant, fast, safe suggestions that lead to good outcomes:

- **Measure suggestion relevance and adoption.** Does the user accept the suggestion (click-through, acceptance rate)? Does accepting it lead to a good result (subsequent engagement, low immediate reformulation)? A suggestion no one accepts, or that leads to reformulation, is not helping.
- **Measure correction and rewriting accuracy.** For spell correction and query rewriting, measure precision (did the correction/rewrite help?) and the rate of harmful changes (did it change an intentional query?). Rewriting that is not measured can quietly degrade results.
- **Measure latency at the relevant percentiles.** Average latency hides the tail; measure p95 and p99, because a suggestion that is occasionally very late is occasionally useless. The latency budget is a tail constraint.
- **Use offline and online evaluation together.** Offline evaluation (judged suggestion sets, correction test cases) catches regressions; online experiments (A/B on acceptance and outcome metrics) confirm real-world value. Relying on only one misses different failure modes.

## Common Traps

### Suggestions Ranked By Raw Popularity, Ignoring The Partial Input

Surfacing the globally most popular completions regardless of the specific prefix, so suggestions are irrelevant to what the user is typing. Rank by relevance to the partial input, combining prefix match, popularity, and context.

### Spell Correction Over-Correcting Intentional Terms

"Correcting" a proper noun, brand, code, or intentional spelling, changing the query's meaning. Use confidence thresholds and protected-term dictionaries; offer rather than force low-confidence corrections.

### Query Expansion Drifting From Intent

Expanding a precise query with synonyms and related terms until the results are irrelevant noise. Bound expansion, apply it where it helps (rare terms, synonyms), and measure its effect.

### Latency That Makes Suggestions Arrive Too Late

A slow suggestion path (expensive computation, slow dependencies) that delivers suggestions after the user has typed past them. Set and enforce a tight latency budget, keep the path lean, and degrade gracefully under slowness.

### Suggestions That Embarrass, Mislead, Or Manipulate

Offensive, defamatory, or commercially biased suggestions presented as neutral, damaging trust and inviting scrutiny. Filter and moderate suggestion content, separate sponsored from organic, and handle sensitive queries with deliberate policy.

### Suggestions Leading To Zero Results

Suggesting completions that have no results in the corpus, frustrating the user and wasting a round-trip. Validate suggestions against the corpus; for catalog search, suggest only what exists.

### Treating Autocomplete As A Trivial Prefix Lookup

Building a prefix match over a term dictionary and considering the feature done, when suggestion quality, latency, safety, and correction materially shape the experience. Treat autocomplete as a prediction and ranking problem with hard constraints.

## Self-Check

- [ ] Suggestions are sourced from multiple signals (popular queries, corpus terms, user history, editorial) and ranked by relevance to the partial input (prefix match, popularity, recency, context), not raw global popularity, and validated against the corpus so they lead to results.
- [ ] Spell correction corrects likely errors with confidence thresholds and protected-term dictionaries (brands, entities, domain vocabulary), offers rather than forces low-confidence corrections, and query expansion is bounded and measured (applied where it helps, not globally).
- [ ] Intent classification routes the query (to retrieval/ranking paths) as a signal, not an override of the user's explicit query.
- [ ] A tight latency budget is set and enforced (p99 in tens to low hundreds of ms), the suggestion path is lean (in-memory, no slow dependencies), the feature degrades gracefully under slowness, and latency is measured at the user's experience (keystroke to render).
- [ ] Suggestion safety and quality are governed: offensive/harmful/brand-damaging suggestions filtered, sponsored separated from organic, sensitive queries handled with deliberate policy, and suggestions audited regularly for emerging problems.
- [ ] Evaluation measures the right things: suggestion relevance and adoption (acceptance rate, subsequent engagement, reformulation rate), correction/rewrite precision and harmful-change rate, latency at p95/p99, with both offline and online evaluation.
- [ ] The highest-risk cases were verified — an embarrassing suggestion filtered, an intentional term not over-corrected, a precise query not over-expanded, a suggestion arriving within the latency budget at p99, and a suggestion leading to results — not only the clean common-prefix demo.
