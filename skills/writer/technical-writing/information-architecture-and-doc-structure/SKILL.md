---
name: information_architecture_and_doc_structure.md
description: Use when the agent is organizing a documentation set as a system, designing doc hierarchy and navigation, cross-referencing between docs, applying progressive disclosure, creating entry points for different reader goals, deciding when to split or combine docs, or maintaining discoverability across a docs site.
---

# Information Architecture And Doc Structure

A single document can be well-written and still fail, because no reader finds it, or because the reader who finds it cannot reach the next thing they need. Documentation lives in a system. The way documents relate to each other, how they are grouped, ordered, named, linked, and surfaced, determines whether a reader can navigate from their question to an answer. This is information architecture: the structure that makes a documentation set usable as a whole, not just page by page. Good IA is invisible when it works and painfully obvious when it does not, because the reader experiences it as either finding what they need or wandering lost.

The common failure is to treat each document as a standalone artifact and neglect the system. Writers add documents where questions arise, without considering where they belong in the hierarchy, what they link to, or how a reader would discover them. Over time the docs site becomes a pile: overlapping documents, orphaned pages, contradictory guidance, buried answers, and no clear path from entry to resolution. A second failure is to over-structure, creating a deep taxonomy that satisfies the writer's sense of order but buries content behind layers of navigation the reader will not traverse. The skill is in building structure that serves the reader's goals and navigation patterns, not the writer's mental model of the subject.

Use this skill when designing or restructuring a documentation set, when deciding hierarchy and navigation, when cross-referencing between documents, when applying progressive disclosure, when choosing entry points for different reader goals, or when deciding whether to split or combine documents. The agent has freedom in the specific structure, but the obligation to make the system navigable and discoverable is not optional.

## Core Rules

### Design For Reader Goals, Not Subject Taxonomy

The structure of a documentation set should reflect what readers come to do, not how the subject is organized internally. Readers arrive with goals: evaluate the product, get started, accomplish a task, understand a concept, troubleshoot a failure, look up a detail. The IA should make each of these goals a clear path. A structure organized by the product's internal architecture, which makes sense to the team, often obscures the paths readers actually take, because readers do not know the internal architecture and should not need to.

Map reader goals first:

- what are the top tasks readers come to accomplish?
- what are the common entry questions, the things they search for?
- what paths do readers take from entry to resolution?
- where do readers get lost or give up?

Structure the docs so that each major goal has an obvious entry point and a clear path. Reconcile the subject taxonomy to the goal paths, not the other way around.

### Build A Shallow, Predictable Hierarchy

Readers tolerate shallow hierarchies and abandon deep ones. If an answer is three clicks from the entry, most readers will find it; if it is seven clicks deep in a nested taxonomy, most will not. Aim for a hierarchy that is broad at the top, with clear categories, and shallow in depth, so that content is reachable in few steps. Predictability matters as much as depth: readers should be able to guess where something lives based on consistent categorization.

Hierarchy principles:

- keep the top-level categories few and distinct, ideally five to seven;
- limit depth so most content is within two or three clicks of entry;
- use consistent category names that readers can map to their goals;
- avoid creating a new category for a single document, which signals over-fragmentation;
- prefer flatter structures with good search and cross-linking over deep trees.

A deep hierarchy feels organized to the writer and impenetrable to the reader. Prefer discoverable over tidy.

### Provide Multiple Entry Points For Different Reader Goals

Different readers arrive with different goals and different starting knowledge. A single entry point, usually the homepage, cannot serve them all efficiently. Provide multiple entry points: a getting-started path for newcomers, a reference index for experts, a task-oriented section for practitioners, a concepts section for learners, a troubleshooting section for those in failure. Each entry point should route the reader to the content suited to their goal without forcing them through irrelevant material.

Common entry points to provide:

- getting started or quickstart, for the reader evaluating or beginning;
- tutorials or guides, for the reader learning by doing;
- reference, for the reader who knows the model and needs specifics;
- concepts or explanation, for the reader building understanding;
- troubleshooting, for the reader in a failure state;
- search, for the reader with a specific question.

Signpost these entry points clearly from the main navigation, so each reader can find their path quickly.

### Apply Progressive Disclosure

Readers cannot absorb everything at once, and a document that front-loads all detail overwhelms. Progressive disclosure is the principle of revealing information in layers: a summary or overview first, then the essentials, then the detail available for those who want it. This lets the reader choose their depth, skimming the summary or diving into detail, without the document forcing one pace on everyone. It is especially valuable for content that serves both quick-look-up and deep-study readers.

Progressive disclosure techniques:

- lead with a summary or BLUF that conveys the core point;
- present the essential information next, what most readers need;
- link to deeper detail, edge cases, and advanced material rather than inline;
- use expandable sections or separate pages for optional depth;
- keep the main path clean and let readers branch to detail.

The goal is that a reader who reads only the top layer leaves with the key information, and a reader who wants depth can reach it without searching.

### Cross-Reference Deliberately And Maintain Links

Documents in a system relate to each other, and those relationships must be made explicit through cross-references. A reader in a tutorial may need the reference for a function they encounter; a reader in a concept doc may need the how-to to apply it; a reader in troubleshooting may need the explanation of why the failure occurs. Cross-links let the reader move along their actual path rather than backtracking to navigation. But links rot: pages move, get renamed, get merged. Maintaining links is part of maintaining the IA.

Cross-reference practices:

- link to related docs at the points where the reader will need them, not just at the end;
- use clear link text that describes the destination, not "click here";
- link in both directions where the relationship is bidirectional;
- check links regularly for breakage, especially after restructuring;
- avoid over-linking, which clutters and dilutes; link where the reader genuinely benefits.

### Decide When To Split Versus Combine Documents

A recurring decision in documentation is whether a topic belongs in one document or several. Splitting too aggressively produces fragmented content the reader must assemble; combining too much produces monolithic documents that are hard to navigate and slow to load. The decision should be driven by reader goals and cohesion, not by length alone. Split when a document serves genuinely different reader goals that warrant separate entry points; combine when the material is cohesive and readers benefit from seeing it together.

Split when:

- the document serves distinct reader goals that deserve separate entry points;
- sections are independently searchable and linkable;
- the document has grown so long that navigation within it is painful;
- different sections target different audiences or skill levels.

Combine when:

- the material is cohesive and readers need it together;
- splitting would force readers to assemble scattered pieces;
- the sections share context that would be duplicated across splits;
- the combined document remains navigable.

The test is whether the structure serves the reader's path, not whether it satisfies an abstract principle of modularity.

### Maintain Discoverability Through Search, Navigation, And Metadata

A document that cannot be found does not exist, however well-written. Discoverability is maintained through three channels: navigation, the structured paths readers browse; search, the query-driven path; and metadata, the titles, descriptions, and tags that make content findable in both. All three must be maintained, because readers use all three depending on their goal. A reader who knows what they want uses search; a reader exploring uses navigation; both depend on accurate metadata to succeed.

Maintain discoverability by:

- writing titles and descriptions that match the terms readers search for;
- keeping navigation consistent and aligned with reader goals;
- ensuring search indexes current content and ranks relevant results;
- using consistent naming so related content clusters;
- periodically auditing for orphaned pages that no path or search reaches.

### Plan For Evolution And Avoid Rot

Documentation IA is not designed once; it evolves as the product and content grow. Without a plan for evolution, the structure rots: new documents are added in ad hoc places, categories become inconsistent, overlap accumulates, and the system drifts from usable to navigable only by insiders. Plan for regular restructuring, for retiring outdated content, and for keeping the structure aligned with how readers actually use it. Treat IA as a living concern, not a one-time setup.

## Common Traps

### Structuring By Subject Taxonomy Instead Of Reader Goals

A structure that mirrors the product's internal architecture makes sense to the team and obscures the paths readers take. Structure for reader goals first.

### Hierarchy Too Deep

Deep nesting buries content behind layers readers will not traverse. Keep the hierarchy shallow and broad, with good search and cross-linking.

### Single Entry Point For All Readers

One homepage cannot efficiently route readers with different goals. Provide multiple entry points, each serving a distinct reader path.

### Front-Loading All Detail

A document that dumps everything at once overwhelms quick readers and buries essentials. Apply progressive disclosure so readers choose their depth.

### Broken Or Missing Cross-References

Documents that do not link to related content force readers to backtrack. Cross-reference deliberately and maintain links against rot.

### Over-Splitting Into Fragments

Splitting every topic into its own page produces scattered content readers must assemble. Split by reader goal and cohesion, not by length.

### Neglecting Search And Metadata

A document no search or navigation reaches does not exist. Maintain titles, descriptions, and navigation so content stays discoverable.

### One-Time Structure With No Maintenance

IA rots without ongoing care. Plan for evolution, retire outdated content, and keep the structure aligned with reader use.

## Self-Check

Before treating the documentation structure as sound, verify:

- The hierarchy is organized around reader goals and tasks, not the subject's internal taxonomy.
- The structure is shallow and predictable, with most content reachable in few clicks.
- Multiple entry points exist for different reader goals, each clearly signposted from navigation.
- Progressive disclosure lets readers choose depth, with summaries up front and detail linked.
- Cross-references link related documents at the points readers need them, and links are maintained.
- Split and combine decisions were driven by reader goals and cohesion, not by length alone.
- Discoverability is maintained through navigation, search, and accurate metadata.
- The structure has been audited for orphaned pages, overlap, and contradiction.
- A plan exists for evolving the structure as content grows, rather than treating IA as one-time.
- A reader with a common goal could move from entry to resolution without getting lost or backtracking excessively.
