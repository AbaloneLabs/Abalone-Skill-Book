---
name: design_guideline_writing.md
description: Use when the agent is writing or revising design guidelines, style guides, design system documentation, brand and visual rules, interaction and motion guidelines, content and voice guidelines, or component usage rules, and must decide what to mandate, what to leave open, how to phrase rules so they are followed, and how to keep the guidance usable as a living reference rather than a dead document.
---

# Design Guideline Writing

Design guidelines are not descriptions of what a design looks like. They are instructions that constrain future decisions, and a guideline that is ambiguous, unranked, or unenforceable will be ignored, misread, or applied inconsistently. The hard work is not describing the current design; it is deciding which rules deserve to be mandates, which are preferences, what context each rule depends on, and how a reader who was not in the room will interpret the words weeks or months later. Agents tend to fail here by writing guidelines that read well but cannot be applied: rules with no scope, no exceptions, no rationale, and no priority when rules conflict.

Use this skill when drafting or revising any design guidance meant to govern future work. The goal is to produce guidance that a designer or engineer can apply correctly without asking the author, and that survives being handed to a team that was not part of writing it.

## Core Rules

### Decide What Is A Mandate Versus A Preference

Not everything in a guideline document should carry equal force. If everything reads as a rule, nothing is treated as one. Before writing each section, classify it.

Classify each rule into one of three levels:

- a hard mandate that must be followed;
- a strong default that should be followed unless there is a reason;
- a preference that is encouraged.

Make the classification visible in the language. "Buttons must use a minimum 44px target" is a mandate; "prefer sentence case for labels" is a default; "consider illustration for empty states" is a preference. Mandates need a reason and often an owner; preferences need enough context that the reader knows when to deviate. Blurring these levels is the most common reason guidelines become unenforceable, because readers stop trusting that any rule is binding.

### Write Rules That Are Testable And Scope-Bound

A usable rule names what it applies to, what it does not apply to, and how someone can tell whether they have satisfied it. "Use accessible color" is not testable; "text must meet 4.5:1 contrast against its background at all states including hover and disabled" is.

For each rule, make the scope explicit:

- which components, surfaces, platforms, or contexts it governs;
- where it stops;
- the conditions under which the rule changes.

Conditions that change a rule often include dark mode, dense data views, mobile, right-to-left languages, error states, and empty states. If a rule cannot be stated in a way a reviewer could check it against a design, it is guidance, not a rule, and should be labeled as such. Rules without scope get over-applied to cases they were never meant for.

### Always Pair A Rule With Its Rationale And Its Exceptions

A rule without a rationale dies the first time it is inconvenient, because the reader has no way to judge whether the inconvenience is a valid reason to deviate.

For every significant rule, record:

- why it exists: the user problem, accessibility need, brand reason, technical constraint, or consistency goal;
- its known exceptions;
- the conditions under which an exception applies.

This lets future designers make defensible local decisions instead of either rigidly following a rule that no longer fits or quietly abandoning it. Rationale also makes the guideline auditable: when the underlying reason changes, the team knows which rules to revisit. A rule that cannot be justified should probably not be a mandate.

### Structure For Retrieval, Not For Narrative

Guideline documents are reference works, not essays. Readers arrive looking for a specific answer under time pressure, often mid-task. Organize so that a reader can find the rule they need without reading the whole document.

Support retrieval by:

- using consistent section patterns and predictable headings;
- providing an index or table of contents;
- cross-referencing related rules;
- putting the rule first, then the rationale, then examples, then exceptions.

This lets a reader stop reading as soon as they have what they need. Repeat critical rules at the point of use rather than relying on memory. A beautifully written narrative that no one can navigate is less useful than a dry, well-indexed reference.

### Provide Positive And Negative Examples

Abstract rules are interpreted differently by every reader. Concrete examples anchor the intended meaning and reduce drift.

For visual and interaction rules, show:

- the correct application;
- at least one common incorrect application;
- a short note on why the incorrect version is wrong.

Examples are especially important where a rule has an edge case, because the example communicates the boundary better than prose. Keep examples current: stale examples that contradict the written rule are worse than no examples, because they train readers to distrust the document. Where examples come from real product surfaces, label the source so readers know whether the example is canonical or illustrative.

### Define Terminology Once And Reuse It Consistently

Guidelines introduce defined terms: "primary action," "surface," "token," "elevation," "density," "affordance." Define each term once, in a glossary or at first use, and then use it identically everywhere.

Inconsistent terminology is a leading cause of guideline drift. When "modal," "dialog," "overlay," and "popup" are used interchangeably, readers cannot tell whether the rules differ. If a term is borrowed from a framework or platform convention, say so, and note where the project's definition diverges. Treat the term list as a controlled vocabulary, not as flavor.

### Plan For Versioning And Ownership

A guideline is a living artifact. State who owns it and how it is maintained.

Document the following:

- who owns the guideline;
- how changes are proposed and approved;
- how often it is reviewed;
- where the canonical version lives;
- a changelog or version history.

When a rule changes, note what changed and why, so teams inheriting an older design can decide whether to migrate. Without ownership and versioning, guidelines fork across teams, and the document stops being a shared source of truth.

## Common Traps

### Writing Description Instead Of Instruction

Describing what the current design does is not the same as telling someone what to do next. Description tells the reader what exists; instruction tells them what is allowed, required, and forbidden. If a section cannot be read as direction, rewrite it.

### Making Every Sentence A Rule

When preferences, defaults, and mandates all sound equally binding, readers either follow everything rigidly and produce lifeless work, or they ignore everything. Reserve mandate language for what must be enforced.

### Rules With No Scope Or Exception Path

A rule that gives no conditions for deviation gets violated silently the first time it is inconvenient. Stating the exceptions makes deviation legitimate and visible instead of hidden.

### Rationale-Free Rules

A rule with no stated reason cannot be defended when challenged and cannot be retired when the reason disappears. Always record why.

### Stale Examples That Contradict The Text

Screenshots and code snippets age fast. An example that shows the old pattern trains readers to distrust the document. Audit examples whenever the rules change.

### Inconsistent Terminology Across Sections

Using "component," "element," "module," and "block" for the same thing makes cross-referencing impossible. Define terms once and enforce them.

### Treating The Guideline As Finished

A guideline that is never updated becomes wrong within months of product change. Without an owner and review cadence, it becomes a historical artifact instead of a living reference.

## Self-Check

- [ ] Each rule is classified as mandate, default, or preference, and the language reflects that force.
- [ ] Every significant rule states its scope: what it applies to, what it excludes, and the conditions that change it.
- [ ] Rules are written so a reviewer could test whether a design satisfies them, not merely describe an aspiration.
- [ ] Each major rule is paired with a rationale and its known exceptions or deviation conditions.
- [ ] The document is structured for retrieval: predictable headings, rule-first ordering, cross-references, and an index or contents.
- [ ] Positive and negative examples are included for visual and interaction rules, and examples are current with the written rules.
- [ ] Defined terms appear in a glossary or at first use and are reused identically throughout.
- [ ] Ownership, change process, canonical location, and version history are documented.
- [ ] No section is pure description; every section can be read as direction for future work.
- [ ] A designer who was not involved in writing the guideline could apply it correctly without asking the author.
