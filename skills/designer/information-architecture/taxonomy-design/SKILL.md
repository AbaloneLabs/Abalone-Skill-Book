---
name: taxonomy_design.md
description: Use when the agent is designing or restructuring a taxonomy, category scheme, tagging model, faceted classification, or content hierarchy, deciding between flat and hierarchical structures, defining controlled vocabularies, handling multiple valid categorization logics, planning governance and evolution, or ensuring a taxonomy stays usable as content and scale grow.
---

# Taxonomy Design

A taxonomy is a set of agreements about how things are grouped, named, and related, and those agreements quietly shape everything downstream: what users can find, what content gets created, what filters and navigation are possible, and how the system behaves at scale. A taxonomy that feels obvious in a small demo collapses under real volume, because real content resists tidy categories, overlaps boundaries, and evolves faster than the structure that contains it. The judgment problem is not inventing categories; it is choosing a categorization logic that matches how users actually think, deciding how much structure to impose, and building in the governance that keeps the taxonomy alive instead of fossilized. Agents tend to fail by designing taxonomies from the inside out (matching the org chart or the database) rather than from the user's mental model, by over-hierarchizing, and by treating the taxonomy as a one-time deliverable rather than a living system.

Use this skill before designing or revising any category structure: product catalogs, content topics, tag systems, help centers, knowledge bases, file organization, or faceted search. The goal is a taxonomy that users can navigate successfully at ten items and at ten thousand.

## Core Rules

### Design From The User's Mental Model, Not The Internal Structure

The most common failure is categorizing content the way the organization thinks about it, rather than the way the people searching for it think about it. Internal structure is invisible to users and produces categories they cannot predict.

Ground the taxonomy in user logic:

- research how users group and name things, through card sorts, search logs, and support tickets;
- prefer the vocabulary users actually use over internal jargon and product codes;
- test whether a target user can predict where an item belongs before they see the answer;
- recognize that different user types may have different, equally valid mental models.

When the org chart or the database schema drives the taxonomy, users experience the structure as arbitrary. When the user's mental model drives it, the structure feels intuitive even when imperfect.

### Choose A Single Dominant Categorization Logic

Every taxonomy is organized around some principle: by topic, by type, by audience, by lifecycle stage, by geography, by use case. Mixed logic, where some branches sort by topic and others by audience, is the most common structural flaw, because users cannot infer which logic applies where.

Make the logic explicit and consistent:

- pick one primary axis for the top level and apply it uniformly;
- use secondary axes only as facets or filters, not as competing primary categories;
- when two logics seem equally important, expose them as separate navigational paths (facets) rather than blending them;
- document the chosen logic so future additions follow it instead of breaking it.

A taxonomy with one clear logic is easy to extend. A taxonomy with mixed logic accumulates ambiguity with every new item.

### Prefer Fewer, Broader Categories Over Deep Hierarchy

Deep hierarchies feel precise but fail in practice: users will not click through five levels, items fit ambiguously across fine categories, and maintenance explodes. Broad, shallow structures are easier to navigate and more resilient to growth.

Heuristics:

- keep the hierarchy as shallow as the content allows, ideally two to three levels for most products;
- limit the number of peer categories at any level so choice is manageable, commonly around five to nine;
- merge categories that overlap heavily or are rarely used rather than preserving distinction for its own sake;
- reserve depth for genuinely large, distinct domains where breadth would overwhelm.

Precision at the cost of navigability is a net loss. Users who cannot find the category cannot find the item inside it.

### Decide Between Exclusive Categories And Multiple Tagging

A fundamental choice is whether each item belongs to exactly one category (a tree) or can belong to many (a polyhierarchy or tag set). Each model has consequences for findability, consistency, and governance.

Tree (single assignment):

- clean, predictable navigation; each item has one home;
- works when categories are mutually exclusive and users expect a single path;
- forces hard decisions about where ambiguous items belong.

Faceted or tagged (multiple assignment):

- supports multiple paths to the same item, improving findability;
- requires controlled vocabularies and discipline to avoid tag sprawl;
- shifts governance burden to maintaining the tag set rather than the tree.

Match the model to the content and the use case. Do not assume a tree; many content domains are inherently many-to-many and fight any single hierarchy.

### Build A Controlled Vocabulary And Enforce It

Free-form tagging decays into chaos: synonyms multiply, capitalization varies, typos become permanent, and semantically identical tags fragment the space. A controlled vocabulary is what keeps a taxonomy usable as it scales.

Establish:

- a canonical term for each concept, with documented synonyms that map to it;
- rules for capitalization, plurality, and formatting;
- a process for proposing, reviewing, and approving new terms;
- retirement of obsolete or duplicate terms, with migration of existing items.

Without governance, the taxonomy becomes whatever the last person tagged, and findability degrades steadily. Governance is not optional polish; it is the load-bearing structure.

### Plan For Ambiguity, Overlap, And Evolution

Real content does not fit cleanly into categories, and the content of today will not match the content of next year. A taxonomy designed for tidy, static content breaks on contact with reality.

Build in resilience:

- define decision rules for ambiguous items rather than leaving them ad hoc;
- accept that some items will be "good enough" fits and document the convention;
- schedule periodic reviews to prune dead categories, merge near-duplicates, and split overgrown ones;
- version the taxonomy and migrate existing content deliberately when structure changes.

A taxonomy that cannot evolve becomes a cage. Plan for change the way you plan for growth.

### Validate The Taxonomy Before Committing

A taxonomy that the designers find logical may still fail users, because designers know the structure and users must infer it. Validate before locking it in.

Validation methods:

- card sorting and reverse card sorting (tree testing) to confirm findability;
- asking users to predict where items belong and measuring success;
- testing with real, edge-case content, not just clean examples;
- checking that every category has enough items to justify itself and none is overloaded.

Findability is the test that matters. Internal elegance is not.

## Common Traps

### Designing From The Org Chart Or Database

Categories that mirror internal structure are invisible and unpredictable to users. Design from the outside in.

### Mixed Categorization Logic

Blending topic, type, and audience in one hierarchy makes the structure unlearnable. Pick one primary axis.

### Over-Hierarchizing For Precision

Deep trees optimize for distinction at the cost of findability. Users will not descend five levels.

### Free-Form Tagging Without Governance

Ungoverned tags fragment into synonyms and typos and steadily destroy findability. Use a controlled vocabulary.

### Assuming A Tree When The Domain Is Many-To-Many

Forcing single-parent categories onto content that genuinely belongs in multiple places hides items from most paths.

### Validating Only With Clean Sample Content

Edge cases reveal where the taxonomy breaks. Test with the hardest, most ambiguous real items.

### Treating The Taxonomy As A One-Time Deliverable

A static taxonomy rots as content and language evolve. Budget for ongoing governance and review.

## Self-Check

- [ ] The taxonomy is grounded in the user's mental model and vocabulary, not internal structure or jargon.
- [ ] A single dominant categorization logic is chosen and applied consistently across all branches.
- [ ] The hierarchy is as shallow as possible, with peer categories kept to a navigable number.
- [ ] The choice between exclusive categories and multiple tagging matches the content's real structure.
- [ ] A controlled vocabulary with canonical terms, synonyms, and formatting rules is defined.
- [ ] Governance for adding, merging, and retiring terms is specified, not left ad hoc.
- [ ] Decision rules for ambiguous and overlapping items are documented.
- [ ] The taxonomy was validated with users through card sorting or tree testing, including edge cases.
- [ ] Every category earns its place by sufficient content, and none is overloaded or nearly empty.
- [ ] A review and evolution process is planned so the taxonomy stays usable as content grows.
