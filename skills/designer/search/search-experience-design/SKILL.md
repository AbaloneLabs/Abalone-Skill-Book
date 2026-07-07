---
name: search_experience_design.md
description: Use when the agent is designing a search experience, search entry points, search input behavior, autocomplete and suggestions, zero-results states, query reformulation, recent and saved searches, or the end-to-end journey from intent to finding what the user is looking for.
---

# Search Experience Design

Search is not a text box. It is a journey from a fuzzy intent in the user's mind to a specific thing they want to find, and most of that journey happens before and after the query itself. A search interface that only handles the moment of typing fails the user who does not know what to type, who mistypes, who gets zero results, or who needs to refine rather than restart. The design problem is supporting the entire arc of finding, not just accepting a query and returning a list.

Use this skill before designing search entry points, input behavior, autocomplete, suggestions, recent searches, zero-results handling, query reformulation, or the overall search journey. The goal is to prevent the agent from treating search as a single input-output step and ignoring the states, intents, and recovery paths that determine whether users actually find what they need.

## Core Rules

### Design For The Full Search Journey

Search has stages, and each needs support:

- before: the user forms an intent and decides to search;
- entry: the user finds and enters the search affordance;
- input: the user types, speaks, or selects;
- refinement: the user adjusts the query or filters;
- results: the user scans and evaluates;
- recovery: the user retries after failure.

A design that handles only input and results leaves users stranded at every other stage. Map the journey and design each transition.

### Make Search Entry Points Predictable And Proportional

Users must be able to find search when they need it, and the prominence of the entry point should match how central search is to the product:

- global search in products where finding is primary deserves persistent, obvious placement;
- contextual search within a view can be more modest;
- hidden search that requires knowing a shortcut excludes users who do not know it.

Match entry-point prominence to how often users rely on search in that surface.

### Support Users Who Do Not Know What To Type

Many users search without knowing the right terms, spelling, or category. Help them form the query:

- autocomplete and suggestions based on popular and personal history;
- category or scope selection before typing;
- recent and saved searches for returning intents;
- example queries or prompts in the empty state;
- tolerance for typos, synonyms, and partial matches.

Punishing imperfect queries assumes users always know what they want, which is rarely true.

### Design Autocomplete As A First-Class Surface

Autocomplete is often where the real decision happens, not the results page. Treat it deliberately:

- show suggestions that are actionable, not just predictive text;
- include result previews or counts where useful;
- distinguish navigational, informational, and categorical suggestions;
- make keyboard and touch navigation through suggestions reliable;
- avoid overwhelming with too many low-quality suggestions.

Poor autocomplete pushes users to the results page with a worse query than they could have had.

### Handle Zero Results As A Design Problem, Not A Dead End

Zero results is one of the highest-stakes moments in search. A blank "no results" screen tells the user the product has failed them. Design recovery:

- suggest corrected or alternative spellings;
- offer broader or related queries;
- show partial matches or fuzzy results;
- recommend removing filters that may be over-constraining;
- provide browse paths as an alternative to search.

The goal is to keep the user moving toward their intent, not to report failure and stop.

### Preserve And Surface Search Context

Users refine, backtrack, and compare. Preserve the context that makes this possible:

- keep the current query visible and editable;
- maintain active filters alongside results;
- allow returning to a recent search without retyping;
- show what was searched for in the results header.

Losing context forces users to restart, which is one of the most frustrating search failures.

### Match Search Behavior To Content Type

Search behaves differently across content. Searching products, documents, people, messages, code, or media each need different defaults:

- products benefit from filters, sorting, and visual results;
- documents benefit from full-text matching and snippets;
- people benefit from name, role, and contact matching;
- messages benefit from recency and conversation context;
- code benefits from symbol and path matching.

Do not apply one generic search pattern to every content type.

### Provide Scope And Intent Control

Users often search within a scope, this project, this folder, this conversation, this category. Make scope explicit and changeable:

- default to the most useful scope for the context;
- let users broaden or narrow scope deliberately;
- show the active scope so results are interpretable.

Hidden scope produces results the user cannot explain.

### Design For Recurring And Re-Findable Intents

Many searches are repeats: users look for the same person, document, or product again. Support re-finding:

- recent searches;
- saved or pinned searches;
- personalization based on history;
- fast paths to previously viewed results.

Treating every search as novel ignores a large class of intent.

### Measure Search By Outcome, Not Query Volume

Search success is whether users found what they wanted, not how many queries they ran. High query volume can indicate healthy use or repeated failure. Measure:

- click-through and result engagement;
- reformulation rates;
- zero-results rates;
- time to successful outcome;
- abandonment.

Optimizing query count can reward a worse experience.

## Common Traps

### Treating Search As A Text Box And A List

Handling only input and results abandons users at entry, refinement, and recovery stages.

### Zero Results As A Dead End

A blank no-results screen with no suggestions forces the user to guess a better query or leave.

### Autocomplete That Suggests Noise

Low-quality or purely predictive suggestions waste the moment where users could be guided to a better query.

### Losing Context On Refinement

Forgetting the query or filters when a user navigates forces restarts and frustration.

### One Generic Search For All Content

Applying a single search pattern to products, documents, people, and code ignores how each is actually found.

### Hidden Or Default Scope

Searching the wrong scope silently produces confusing results the user cannot explain.

### Punishing Imperfect Queries

Strict matching on typos, synonyms, and partial input assumes users always know the exact term.

### Measuring Queries Instead Of Outcomes

Rewarding query volume can mask a search experience that makes users try repeatedly before succeeding.

## Self-Check

- [ ] The full search journey, before, entry, input, refinement, results, and recovery, is designed, not just input and results.
- [ ] Search entry points are placed with prominence proportional to how central search is in each surface.
- [ ] Users who do not know the right terms are supported through autocomplete, suggestions, examples, and typo tolerance.
- [ ] Autocomplete is treated as a first-class surface with actionable, distinguishable, and navigable suggestions.
- [ ] Zero-results states offer corrections, alternatives, fuzzy matches, filter relief, and browse paths rather than a dead end.
- [ ] Query, filters, scope, and recent searches are preserved and visible to support refinement and backtracking.
- [ ] Search behavior is tailored to the content type rather than using one generic pattern.
- [ ] Scope is explicit, changeable, and visible so results are interpretable.
- [ ] Recurring and re-findable intents are supported through recent, saved, and personalized search.
- [ ] Search success is measured by outcome, engagement, reformulation, and abandonment, not by raw query volume.
