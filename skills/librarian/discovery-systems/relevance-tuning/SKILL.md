---
name: relevance_tuning.md
description: Use when the agent is tuning search relevance in a discovery system, adjusting ranking algorithms and boost factors, evaluating relevance with queries and judgments, handling relevance conflicts between known-item and topical search, or deciding when to customize versus accept vendor defaults.
---

# Relevance Tuning

Relevance ranking is the invisible logic that determines which results users see first. In a discovery system that searches across catalogs, article databases, repositories, and licensed content, relevance is not a single correct answer; it is a set of tradeoffs between matching the user's query, reflecting the authority and usefulness of materials, and serving different search intents. Vendor defaults are a starting point, not a solution, because they are tuned for generic collections and do not reflect local priorities, user behavior, or the specific mix of materials. Tuning relevance is a judgment problem that requires understanding what users are trying to find, measuring whether the ranking serves them, and making deliberate adjustments rather than accepting defaults or chasing tweaks blindly.

Use this skill when tuning search relevance, adjusting ranking and boost factors, evaluating relevance quality, or deciding how much to customize discovery ranking. The goal is to prevent the agent from accepting vendor defaults uncritically, tuning without measurement, over-optimizing for one query type, or making changes that help some searches while silently harming others.

## Core Rules

### Understand That Relevance Is Context-Dependent

There is no universal correct ranking. Relevance depends on the user's intent, which varies by query and context.

Relevance differs by intent:

- known-item search prioritizes exact title and author matches;
- topical exploration prioritizes subject relevance and authority;
- current-awareness search prioritizes recency;
- instructional search prioritizes accessible over specialized materials.

A ranking that serves one intent may harm another. Tuning must balance these or differentiate ranking by detected intent where possible.

### Start With Vendor Defaults, Then Measure

Vendor discovery systems ship with default relevance settings tuned for generic use. These are a reasonable baseline but not a final answer.

Approach:

- begin with defaults and understand what they prioritize;
- instrument the system to measure relevance quality;
- identify queries and user groups where defaults fail;
- make targeted adjustments based on evidence;
- re-measure after each change.

Blindly accepting defaults leaves known problems unaddressed. Blindly overriding them without measurement introduces new ones.

### Build A Query Set And Relevance Judgments

Relevance tuning requires a way to measure quality. A query set with relevance judgments is the foundation.

Build:

- a representative set of queries, known-item, topical, common, edge cases;
- include real queries from search logs, not invented ones;
- for each query, judge which results are relevant, partially relevant, or not;
- judgments from multiple evaluators improve reliability;
- revisit and expand the set over time.

The query set is your measurement instrument. Without it, tuning is subjective and unrepeatable.

### Use Standard Relevance Metrics

Measure ranking quality with established metrics rather than subjective impression.

Common metrics:

- precision at k, how many of the top results are relevant;
- recall, how many relevant items appear at all;
- mean reciprocal rank, where the first relevant result appears;
- normalized discounted cumulative gain, accounting for graded relevance and position.

Track metrics across the query set before and after changes. Metrics make tuning objective and comparable.

### Adjust Boosts Deliberately And Measure Effects

Relevance tuning often involves boost factors that weight certain fields or attributes, title matches, author matches, recency, popularity, peer-reviewed status.

Boosting guidance:

- understand what each boost does before changing it;
- change one factor at a time and measure;
- watch for interactions, boosting title may harm topical searches;
- favor modest adjustments, large boosts distort ranking;
- document each change and its rationale.

Indiscriminate boosting creates a ranking that serves whoever lobbied last. Deliberate, measured adjustment serves the system.

### Balance Known-Item And Topical Ranking

Known-item and topical searches have opposing relevance needs. A single ranking must compromise or the system must differentiate.

Balancing approaches:

- ensure exact title and author matches rank highly for known-item queries;
- ensure subject and citation relevance rank highly for topical queries;
- consider query-type detection to apply different ranking;
- test the query set for both intent types;
- accept that some compromise is unavoidable.

Over-optimizing for known-item search buries topical results. Over-optimizing for topical search loses known items. Balance deliberately.

### Handle Authority And Popularity Signals Carefully

Some systems allow boosting by authority signals, citation counts, peer-reviewed status, or popularity, holds, clicks. These can improve or distort relevance.

Considerations:

- authority boosts can surface high-quality materials but bias toward established works;
- popularity boosts reflect use but can create rich-get-richer effects;
- recency boosts help current-awareness but bury classic foundational works;
- combine signals thoughtfully, do not stack them blindly.

Every signal has a bias. Understand the bias and decide whether it serves your users.

### Account For Local Priorities And Content

Local collections, institutional repositories, open access content, and licensed resources have different priorities. Ranking should reflect them.

Local considerations:

- boost locally held or open access materials for affordability and speed;
- consider format preferences, prioritize formats users can actually use;
- reflect institutional priorities, such as showcasing faculty or student work;
- balance licensed and open content fairly.

Generic defaults do not know your local context. Encode local priorities in ranking where the system allows.

### Evaluate Changes For Unintended Harm

A change that improves one query can harm others. Evaluate the full query set, not just the target query, after each adjustment.

Evaluation:

- run the full query set and compare metrics before and after;
- look for queries that worsened even if the target improved;
- check edge cases and underrepresented query types;
- seek a net improvement, not a targeted one at others' expense;
- roll back changes that cause net harm.

Tuning is a system-wide intervention. Measure its effects system-wide.

### Document And Version Relevance Configurations

Relevance settings change over time through tuning, vendor updates, and system migrations. Documentation preserves institutional knowledge.

Document:

- the current relevance configuration and its rationale;
- the history of changes with dates and reasons;
- the query set and judgment methodology;
- known issues and planned improvements;
- vendor update impacts and compensating adjustments.

Undocumented configurations become mysterious and unchangeable. Treat relevance settings as maintained institutional assets.

### Re-Tune Periodically And After Major Changes

Relevance drifts over time as collections grow, user behavior shifts, and systems update. Periodic re-tuning keeps ranking effective.

Re-tune when:

- the collection changes substantially, new databases, repositories;
- the system is upgraded or migrated;
- user complaints or analytics indicate declining quality;
- institutional priorities shift;
- on a regular schedule, annually or biannually.

Relevance is not set-and-forget. Schedule maintenance as part of discovery stewardship.

## Common Traps

### Accepting Vendor Defaults Uncritically

Defaults serve generic use. Measure and adjust for local needs.

### Tuning Without Measurement

Subjective tuning is unrepeatable and unreliable. Use query sets and relevance metrics.

### Over-Optimizing For One Query Type

Serving known-item at the expense of topical, or vice versa, harms many users. Balance intent types.

### Indiscriminate Boosting

Stacking boosts without understanding interactions distorts ranking. Change one factor at a time and measure.

### Ignoring Authority And Popularity Biases

Every signal has a bias. Understand and combine signals thoughtfully.

### Ignoring Local Content Priorities

Generic defaults do not reflect local context. Encode local priorities where possible.

### Evaluating Only The Target Query

Changes ripple across the system. Measure the full query set for unintended harm.

### Undocumented Configurations

Without documentation, settings become untouchable. Document configurations, changes, and rationale.

### Set-And-Forget Relevance

Relevance drifts. Re-tune periodically and after major changes.

## Self-Check

- Is relevance understood as context-dependent, balancing known-item, topical, and other search intents?
- Are vendor defaults used as a baseline, then measured and adjusted based on evidence?
- Is there a representative query set with relevance judgments as the measurement foundation?
- Are standard relevance metrics used to evaluate ranking quality objectively?
- Are boost factors adjusted one at a time with measured effects and documented rationale?
- Are known-item and topical ranking balanced rather than over-optimized for one type?
- Are authority, popularity, and recency signals understood for their biases and combined thoughtfully?
- Do local content priorities, holdings, open access, and institutional work, factor into ranking?
- Are changes evaluated across the full query set for unintended harm, not just the target query?
- Are relevance configurations documented, versioned, and periodically re-tuned after changes?
