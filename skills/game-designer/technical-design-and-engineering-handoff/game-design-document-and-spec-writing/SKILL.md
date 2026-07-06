---
name: game-design-document-and-spec-writing.md
description: Use when the agent is writing or revising a game design document, authoring feature specifications, defining mechanics and systems for engineering handoff, documenting player-facing rules, or structuring design intent so that art, code, and QA can build and verify the same game.
---

# Game Design Document and Spec Writing

A game design document is not a creative writing artifact; it is a contract between disciplines that must build the same game from different sets of assumptions. The recurring failure is that the designer writes to express intent rather than to remove ambiguity, and every undefined edge case becomes a place where an engineer guesses, an artist fills a gap with the wrong assumption, and a QA tester cannot tell whether observed behavior is correct. The harm this produces is compounding: rework when the built feature does not match the intended feel, scope creep when the spec is vague enough to absorb new requirements silently, and credibility loss for the design function when documents are treated as decorative. Agents tend to miss this because the design feels clear in the author's head, the prose reads well, and the missing rigor only surfaces weeks later during integration. The skill covers the judgment of what a spec must contain, how to separate stable intent from volatile detail, and how to write so that the document survives contact with production. The designer has substantial freedom in format, but no freedom in the obligation to make the game buildable and testable from the text alone.

## Core Rules

### Write to the Reader Who Will Build or Verify, Not to Yourself

A design document is communication, not expression. The author already understands the intent, so every sentence must serve a reader who does not. Engineers need states, transitions, data shapes, and failure handling; artists need reference, framing, and emotional targets; QA needs acceptance criteria that distinguish a correct result from an incorrect one. The trap is writing flowing narrative that reads as compelling game fiction but leaves the implementer to reverse-engineer the actual rules. When you write a feature, ask which discipline consumes each section and whether that section answers the questions they will actually ask. When a section serves only the author's satisfaction, cut it or convert it into buildable detail.

### Make Every Mechanic Specified Enough to Be Wrong

A specification is only useful if it can be violated. If a description is so soft that any implementation satisfies it, it has not specified anything. For each mechanic, define the inputs, the player-visible outputs, the internal state changes, the numerical or timing parameters, and the conditions under which it fails or is denied. Parameterize the values that will need tuning rather than hardcoding prose like "deals moderate damage." When a rule can be expressed as a table, a state diagram, or a numbered procedure, use that form instead of paragraph description. When you cannot state how the mechanic behaves under a specific input, that gap is an unresolved design decision, not a stylistic choice, and it must be flagged as such.

### Separate Intent, Rules, and Content Data Into Distinct Layers

Conflating why a feature exists, how it works, and what specific values populate it is the most common structural error. Intent is the player experience goal and rarely changes once set. Rules are the logic that produces that experience and change during prototyping. Content data is the tunable values, strings, and assets that are replaced constantly during balancing. A document that mixes all three becomes unreadable because every tuning pass rewrites the intent section, and every intent discussion churns the data tables. Keep intent at the top of a feature spec as a short, stable statement. Keep rules as the procedural body. Keep data in separate tables or linked spreadsheets so that balancing does not require editing the design narrative.

### Define the Negative Space: What the Feature Explicitly Does Not Do

Most rework comes not from missing requirements but from unspoken exclusions. A spec that lists what a feature does without stating what it deliberately avoids invites scope creep, because every "could we also" suggestion appears compatible with the document. For each feature, include an explicit out-of-scope or non-goals section: interactions that will not be supported, platforms excluded, edge cases deferred, and player behaviors that are intentionally not handled. This converts implicit boundaries into negotiated ones. When a stakeholder later requests an excluded behavior, the document makes clear it is a change request with cost, not an oversight to be absorbed for free.

### Specify Acceptance Criteria That QA Can Execute Without the Designer Present

If QA cannot determine whether a feature is done by reading the spec, the spec is incomplete. Acceptance criteria must be observable, repeatable, and binary in verdict: given a setup, when the player performs an action, then a specific, checkable result occurs. Avoid criteria phrased as feel ("combat should feel responsive") unless paired with a measurable proxy (input-to-action latency under a threshold, animation cancel windows). Write the happy path and at least the primary failure paths: what happens when the action is invalid, interrupted, or performed at a boundary. When a criterion cannot be made observable, the feature is under-designed, and that must be acknowledged rather than hidden behind subjective language.

### Version the Document and Mark Volatility Explicitly

Design documents decay, and readers must know how much to trust any given section. Distinguish between approved design, in-progress prototyping, and open questions, and mark them visually or structurally so an engineer does not build against a section that is about to change. Record a changelog with dates and rationale for material revisions, because the reason a decision was made is often more valuable than the decision itself when the team revisits it months later. When a parameter is known to be placeholder, label it as such so no one treats a tuning guess as a locked spec.

### Treat the Document as a Living Negotiation, Not a One-Time Deliverable

A GDD delivered once and never revisited is a fossil. The document should be updated at every decision point where the team agrees to change the design, and the update should reflect what was actually decided, not what the designer originally hoped. When the built game diverges from the document, the document is wrong and must be corrected to match the shipped reality, because the document's job is to describe the game the team is making. Reserve the freedom to prototype ahead of the document, but never let the authoritative spec drift from the code for more than a short cycle.

## Common Traps

### Prose That Reads Beautifully but Specifies Nothing

The designer writes an evocative paragraph about the tension of a stealth section, and it reads convincingly in review, so everyone nods and moves on. The trap is that compelling prose creates a false sense of completeness: stakeholders feel the feature is understood because the writing is clear, but the implementer still has no states, no failure conditions, and no numbers. The false signal is the emotional clarity of the text, which is unrelated to buildability. The harm surfaces weeks later when the built feature feels wrong and nobody can point to the spec to determine whether it was built incorrectly or designed vaguely.

### Hardcoding Placeholder Numbers as If They Were Design

A spec fills in damage values, cooldowns, and spawn counts to look complete, and these values travel unchanged into implementation. The trap is that early numbers are almost always wrong, but once they are written into a document and then into code, they acquire the weight of intentional design and resist tuning. The false signal is the appearance of precision; a number looks more decided than a labeled placeholder. The harm is that balancing becomes a political fight over "changing the design" rather than the routine tuning it should be, because nobody can distinguish a guess from a commitment.

### Documenting Only the Happy Path

The spec describes the intended player flow in detail and leaves every error, interruption, and boundary case to be discovered during implementation. The trap is that happy paths are easy and satisfying to write, while failure handling is tedious and exposes unresolved decisions, so the author unconsciously avoids it. The false signal is that the feature feels fully designed because the main loop is clear. The harm is that engineers implement ad-hoc failure behavior, QA cannot test edge cases, and the shipped game behaves unpredictably exactly where players stress it most.

### Letting the Document Become the Project History

The GDD accumulates rationale, rejected alternatives, and meeting notes until it becomes a narrative of how the design evolved rather than a description of what it currently is. The trap is that historical context feels valuable and is hard to delete, so it grows until the document is unreadable and the authoritative current state is buried. The false signal is thoroughness; a long document feels rigorous. The harm is that implementers stop reading it because they cannot find the current rules, and the document loses all authority as a source of truth.

### Assuming the Reader Shares the Designer's Unspoken Taste

The designer writes "the boss should feel intimidating" assuming the team shares a common reference for what intimidating means in this genre. The trap is that taste is not shared by default, especially across disciplines and cultures, and unstated aesthetic assumptions produce wildly different interpretations. The false signal is that the phrase feels meaningful to the author. The harm is that art, audio, and combat design each optimize for a different mental image of the same word, and the integrated result is incoherent despite everyone believing they followed the spec.

## Self-Check

- Does each feature section identify its consuming discipline, and would an engineer, artist, and QA tester each find answers to the questions they will actually ask?
- Can each mechanic be violated, meaning inputs, outputs, state changes, parameters, and failure conditions are specified rather than described in soft prose?
- Are intent, rules, and tunable data separated into distinct layers so that balancing changes do not require rewriting the design narrative?
- Is there an explicit non-goals or out-of-scope section for each major feature that names what is deliberately excluded?
- Are acceptance criteria written as observable, repeatable, binary checks covering both the happy path and primary failure paths, without relying on subjective feel language?
- Are placeholder values labeled as such, and is there a changelog that records the rationale for material revisions rather than only the change itself?
- Does the document describe the game the team is currently building, with volatile sections marked, rather than describing an earlier or aspirational version?
