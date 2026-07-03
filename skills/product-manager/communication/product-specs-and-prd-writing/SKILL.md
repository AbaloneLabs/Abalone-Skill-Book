---
name: product_specs_and_prd_writing.md
description: Use when the agent is writing or structuring a PRD or product spec, deciding how much detail to include, organizing a requirements document for clarity, or tailoring written product documentation to its engineering and design audience.
---

# Product Specs And PRD Writing

A product spec is not judged by its length or completeness; it is judged by whether the right people can make the right decisions from it without the author in the room. The craft of spec writing is calibration: enough structure that nothing important is ambiguous, little enough process that the document is actually read. Most spec failures are writing failures, not requirements failures. The document is too long to skim, too vague to build from, structured around the author's thinking rather than the reader's needs, or treated as ceremony rather than the source of truth.

Use this skill before drafting a PRD, feature brief, or one-pager, restructuring an existing spec, or deciding how much documentation a piece of work needs. The goal is to prevent the agent from producing documents that no one reads, that omit the decisions engineers and designers actually need, or that freeze the wrong level of detail too early.

## Core Rules

### Structure For Skimmability First

A spec that cannot be skimmed will not be read carefully. Engineers and designers open a document to find the answer to a specific question, not to read a narrative. Structure must let a reader locate any decision in seconds.

Organize around stable, predictable sections:

- problem and context;
- goals and non-goals;
- proposed solution and scope;
- key decisions and their rationale;
- open questions with owners;
- rollout and success criteria.

Put the most decision-relevant content at the top and supporting detail below. Use headings, short paragraphs, and lists so a reader can jump. A spec that reads well linearly but cannot be navigated is worse than one that is ugly but findable.

### Calibrate Detail To Audience And Decision Stage

The right amount of detail depends on who reads it and what decision it must support. A discovery-stage brief needs less precision than an implementation-ready PRD. Over-documenting early freezes thinking; under-documenting late creates rework.

Match detail to stage:

- discovery brief: problem, hypotheses, what you intend to learn;
- design review: problem, constraints, proposed direction, open design questions;
- implementation PRD: behavior, scope, edge cases, permissions, data, acceptance criteria;
- launch doc: rollout, comms, support, success metrics.

Do not write an implementation PRD for a problem still being explored, and do not ship a discovery brief to engineers expecting to build from it.

### Write So The Document Survives Your Absence

The author will not be in every standup, design review, or QA session. A spec fails the moment someone has to message the product manager to resolve an ambiguity that should have been written down.

To make a spec self-sufficient:

- state the rationale behind non-obvious decisions, not just the decision;
- define terms and acronyms on first use;
- link to prior decisions and context rather than assuming knowledge;
- answer the questions a new joiner would ask;
- make assumptions explicit so they can be challenged before implementation.

If a reader must reconstruct your reasoning from memory or chat history, the document is incomplete.

### Choose The Right Weight Of Process

Not every change needs a full PRD. Forcing heavy process onto small or low-risk work wastes time and trains the team to treat specs as bureaucracy. Forcing too little process onto risky work hides decisions that should be explicit.

Choose weight by risk and scope:

- trivial fix: ticket with acceptance criteria, no spec;
- small feature: lightweight brief covering problem, scope, and behavior;
- medium feature: structured PRD with decisions and edge cases;
- large or cross-functional initiative: full PRD plus rollout and comms plan.

The test is whether the document captures the decisions that would otherwise be lost or relitigated. If a conversation would suffice, do not write a PRD; if the decision will outlive the conversation, write it down.

### Use Diagrams, Examples, And Edge Cases Effectively

Prose is a poor tool for describing flows, state transitions, and data relationships. The right visual or example communicates in a fraction of the words and reduces misimplementation.

Use each deliberately:

- flow diagrams for multi-step or branching user journeys;
- state diagrams for objects with many statuses;
- data models or schemas for persistence and integration boundaries;
- concrete worked examples for calculations or transformations;
- explicit edge case lists for empty, error, permission, and concurrent states.

Do not add diagrams as decoration. A diagram that duplicates the prose without adding clarity is noise. Each visual should answer a question the prose answers poorly.

### Keep The Document As The Source Of Truth

A spec that drifts from reality becomes worse than useless; it becomes misinformation. Once engineers build, decisions change, and if the document is not updated, the next reader trusts stale information.

Maintain the doc as source of truth by:

- versioning it and noting what changed and why;
- updating it when scope or decisions shift, not just at kickoff;
- linking tickets to the spec and the spec to tickets;
- marking resolved open questions and superseded decisions clearly;
- dating and ownership-stamping so readers know who maintains it.

A stale spec actively misleads. If no one owns keeping it current, it should be archived, not left to rot.

### Separate Decisions From Open Questions

A reader must be able to tell what is settled from what is still being decided. Mixing the two creates false certainty about open items and reopens settled ones.

In every spec, distinguish:

- decisions: what has been chosen and why, with the owner and date;
- open questions: what is unresolved, who owns resolving it, and by when;
- assumptions: what the spec depends on being true but has not validated;
- dependencies: what must happen externally for this to proceed.

Open questions without owners do not get answered. Decisions without rationale get relitigated. Keep these categories visibly separate.

## Common Traps

### Doc Theater

Writing a long, polished PRD that no one reads or references is theater, not communication. A document's value is measured by whether it changes decisions and behavior, not by its page count. If the team ignores it, the spec failed regardless of its quality.

### Freezing Implementation Detail Too Early

Specifying screens, fields, and flows before the problem is understood locks the team into a solution that may not fit. Early specs should constrain the problem and the constraints, not the implementation. Detail should deepen as certainty grows.

### Writing For The Author's Logic, Not The Reader's Needs

A spec organized around how the author thought through the problem forces every reader to replay that thinking. Readers want answers to their questions fast. Organize around the decisions a reader needs to make, not the chronology of discovery.

### Omitting Rationale And Assuming Shared Context

Stating a decision without its rationale makes it unchallengeable and fragile. When context shifts, a decision whose reasoning was never written cannot be safely updated. Always record why, not just what.

### Letting The Spec Go Stale

A spec updated only at kickoff diverges from the built product within weeks. Readers then trust neither the doc nor their memory. An unmaintained spec is a liability; either keep it current or archive it.

### Over-Documenting Low-Risk Work

Applying full PRD process to a small change trains the team to see specs as overhead and to skim or skip them. Match documentation weight to the risk that decisions will be lost or misremembered.

### Confusing Breadth With Completeness

Covering many sections shallowly feels thorough but leaves the load-bearing decisions ambiguous. A spec that is complete in headings but vague in the places that matter does not protect the team. Depth belongs where the decisions are hard.

## Self-Check

- [ ] The document is structured for skimmability, with decision-relevant content at the top and predictable sections.
- [ ] The level of detail matches the decision stage and audience, not frozen too early or too late.
- [ ] The spec is self-sufficient: rationale, terms, assumptions, and links are present so the author's absence does not block readers.
- [ ] The weight of process matches the risk and scope of the work.
- [ ] Diagrams, examples, and edge case lists are used where prose is weaker, not as decoration.
- [ ] The document is versioned, maintained as the source of truth, and updated when scope or decisions shift.
- [ ] Decisions, open questions, assumptions, and dependencies are visibly separated, with owners and dates.
- [ ] Each major decision includes its rationale so it can be safely revisited when context changes.
- [ ] The spec avoids doc theater: it is structured to be read and to change decisions, not merely to exist.
- [ ] Resolved questions and superseded decisions are marked clearly so readers are not misled by stale content.
