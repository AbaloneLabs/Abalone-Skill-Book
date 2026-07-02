---
name: onboarding_and_knowledge_base.md
description: Use when the agent is designing onboarding for new engineers, writing a new-team-member guide, creating an architecture overview or system map, documenting local setup and tooling, building a glossary or FAQ, deciding what institutional knowledge to capture, or maintaining a knowledge base so it stays fresh and trustworthy as the team and system evolve.
---

# Onboarding and Knowledge Base Design

Onboarding is where the cost of undocumented knowledge becomes visible. A new engineer's first days are a natural experiment in how well a team has captured what it knows: if they are productive in a week, the knowledge base is healthy; if they spend a month asking the same questions every hire asks, the knowledge lives only in veterans' heads and is leaking out the door with every departure. Teams chronically underinvest in onboarding docs because the people who could write them no longer notice the knowledge is missing, and because the cost is paid by the next hire, not by the author. The result is onboarding that is slow, inconsistent, and demoralizing, and a knowledge base that is a graveyard of stale pages no one trusts.

The judgment problem is that a knowledge base is a living system that decays unless maintained, and onboarding is not a single document but a path through it. The agent must decide what a new hire actually needs first (not everything, in the right order), separate durable knowledge (architecture, domain concepts, terminology) from ephemeral state (current sprint, who owns what today), and build maintenance into the system so the knowledge base does not rot into untrustworthy noise. A knowledge base that cannot be trusted is worse than none, because it wastes the reader's time and teaches them to stop looking. The goal is a curated, fresh, navigable body of knowledge that a new engineer can traverse and that veterans still consult.

## Core Rules

### Design onboarding as a path, not a document dump

A new hire does not need every document on day one; they need a sequence. Structure onboarding as a time-ordered path: environment setup and first build (day 1-2), architecture overview and where their work fits (week 1), domain concepts and terminology (week 1-2), operational basics and how to ship a small change (week 2), and deeper subsystem ownership over time. A single "read everything" wiki overwhelms and ensures nothing is read well. Curate the path, mark what is essential now versus reference-later, and update the path based on where new hires actually get stuck.

### Separate durable knowledge from ephemeral state

The knowledge base conflates two things with very different lifespans. Durable knowledge (architecture, data model, domain concepts, terminology, design decisions and their rationale, how-to procedures) changes slowly and is worth investing in. Ephemeral state (current team roster, who owns what this quarter, the active incident, this sprint's goals) changes constantly and will be stale within weeks. Separate them physically (different sections, different pages) and treat them differently: durable knowledge gets careful authorship and infrequent updates; ephemeral state gets lightweight, frequently-updated or auto-generated content. Mixing them means the durable pages inherit the staleness of the ephemeral ones and the whole base stops being trusted.

### Write the architecture overview a new hire can actually follow

The architecture overview is the single highest-leverage onboarding document, and it is usually either missing or written for the author rather than the reader. A good overview answers: what is this system, who uses it, what are the major components, how do they interact, where does data flow, what are the external dependencies, and where does a new engineer's work likely fit. Use a diagram. Name the components consistently with the codebase. Do not assume domain knowledge; define terms. An overview that a new hire reads and says "now I understand the shape of this" is worth ten detailed subsystem docs they cannot place in context.

### Capture terminology and acronyms before they become a barrier

Every team accumulates a private vocabulary of acronyms, codenames, and domain terms that are obvious to insiders and opaque to new hires. Maintain a living glossary, and when a term appears in a doc, link it. The absence of a glossary forces new hires to interrupt colleagues constantly for definitions, which is slow for them and annoying for the team. The glossary is also a diagnostic: terms that cannot be defined crisply are terms the team has not thought clearly about.

### Make local setup reproducible and documented, and treat setup friction as a bug

Environment setup is the first impression and the most common onboarding failure. If a new hire spends two days fighting dependency conflicts, missing credentials, or undocumented steps, their first experience of the team is frustration. Document setup as a tested procedure (ideally scripted via devcontainer, Docker, or a setup tool), keep it current, and treat setup breakage as a bug to fix, not a rite of passage. The setup doc is also a forcing function: if setup cannot be documented cleanly, the setup itself is probably too fragile.

### Build maintenance into the system so knowledge does not rot

A knowledge base decays unless maintenance is structural, not voluntary. Techniques: assign owners to pages (and surface ownership so readers know whom to ask), tag pages with last-reviewed dates and review cadences, tie doc updates to code changes (a PR that changes behavior updates the doc in the same PR), and periodically audit for dead links and stale references. Auto-generate what you can (API reference from doc comments, service ownership from service registry) so it cannot drift. Without maintenance structure, the base rots within a year and becomes untrustworthy.

### Capture the questions people actually ask, and route them into the base

Every repeated question is a documentation gap. When a new hire asks something, the answer should become a FAQ entry or a doc update, not just a chat reply. Maintain a FAQ seeded from real questions, and when a question recurs, treat it as a signal that the corresponding doc is missing or unclear. This turns onboarding friction into a feedback loop that improves the base for the next hire, rather than a cost that recurs forever.

### Make the knowledge base navigable and trustworthy

A base that cannot be searched or navigated is as good as empty. Invest in structure: a clear hierarchy, a working search, consistent naming, and cross-links between related pages. Critically, cultivate trust: if a reader finds three stale pages, they stop believing any page. Mark uncertain or in-progress content explicitly, remove or archive obsolete pages rather than leaving them to mislead, and prefer fewer high-quality pages over many low-quality ones. Trust is the base's most valuable property and the easiest to lose.

## Common Traps

### Document dumps instead of curated paths

Dumping every wiki page on a new hire overwhelms and ensures shallow reading. Curate a time-ordered path and mark what is essential now versus later.

### Mixing durable knowledge with ephemeral state

When architecture pages and "current team roster" pages live together, the whole base inherits the staleness of the ephemeral content. Separate them and maintain them differently.

### An architecture overview written for the author, not the new hire

Overviews full of unexplained acronyms and assumed context help no one new. Write for someone who knows engineering but not this system, with a diagram and defined terms.

### No glossary, so new hires interrupt constantly for definitions

A missing glossary taxes every new hire and every veteran they interrupt. Maintain one and link terms wherever they appear.

### Setup friction treated as a rite of passage

If setup takes days and breaks silently, the fix is to document and script it, not to expect new hires to suffer. Treat setup breakage as a bug.

### A knowledge base with no maintenance structure that rots silently

Without owners, review cadences, and doc-updates-with-code rules, the base drifts and becomes untrustworthy within a year. Build maintenance into the workflow.

### Repeated questions answered in chat but never added to the base

Every recurring question is a doc gap. Capture answers as FAQ entries or doc updates so the next hire does not ask the same thing.

### Leaving obsolete pages to mislead instead of archiving them

Stale pages that contradict current reality destroy trust in the whole base. Archive or delete obsolete content and mark uncertain pages explicitly.

## Self-Check

- Is onboarding structured as a time-ordered path (setup, architecture, domain, operations, ownership) with essential-versus-reference marking, rather than a document dump?
- Are durable knowledge (architecture, concepts, decisions) and ephemeral state (roster, current sprint) separated and maintained with different cadences?
- Does the architecture overview explain the system to someone who knows engineering but not this product, with a diagram, consistent component names, and defined terms?
- Is there a living glossary of acronyms and domain terms, and are terms linked where they appear in docs?
- Is local setup documented as a tested, ideally scripted procedure, with setup breakage treated as a bug rather than a rite of passage?
- Does the knowledge base have structural maintenance (page owners, last-reviewed dates, review cadences, doc-updates-with-code, auto-generated reference) so it does not rot?
- Are recurring new-hire questions captured as FAQ entries or doc updates, turning onboarding friction into a feedback loop?
- Is the base navigable (hierarchy, search, cross-links) and trustworthy (obsolete pages archived, uncertain content marked, quality preferred over quantity)?
- Can a reader identify the owner of any page to ask questions, and are dead links and stale references periodically audited?
