---
name: prd_structure_and_clarity.md
description: Use when the agent is structuring a product requirements document, deciding what sections and information a PRD must contain, organizing content for clarity, or ensuring a PRD communicates problem context goals scope and requirements without ambiguity or critical gaps.
---

# PRD Structure And Clarity

A product requirements document (PRD) is the central artifact that communicates what is being built, why, for whom, and what success looks like. Its structure determines whether readers can find the information they need, understand the context, and align on what is being committed. Done well, a PRD leads a reader from the problem through the goals and scope to the requirements, building shared understanding that enables good decisions and aligned execution. Done poorly, it is a disorganized collection of features that omits context, confuses problems with solutions, and leaves readers guessing about what matters. Agents often structure PRDs as feature lists, because features are concrete, while omitting the problem and goals that give the features meaning.

The harm this skill prevents is the PRD that fails to create shared understanding. When a PRD lacks problem context, the team does not know what they are solving; when it lacks goals, they cannot evaluate whether the solution works; when it lacks scope, they cannot tell what is in and what is out; when its requirements are ambiguous, they build different things from what was intended. A well-structured PRD prevents these failures by leading the reader through the information in an order that builds understanding.

Use this skill before answering questions such as "how do we structure a PRD", "what should a PRD contain", "is this PRD clear enough", or "how do we write a good requirements document". The goal is to prevent the agent from producing a PRD that is a feature list without context or a document whose structure obscures rather than communicates.

## Core Rules

### Lead With The Problem And Context Before Any Solution

A PRD should begin with the problem: what is broken, missing, or opportunity-rich in the current state, and why it matters. This context is what makes the rest of the document meaningful, because requirements without problem context are arbitrary. Establish the problem with evidence: customer feedback, data, market analysis, or strategic context that shows why this work is worth doing. Only after the reader understands the problem should the document move to goals and solution.

Leading with the problem also prevents the solution from being taken for granted. When a PRD opens with features, the features appear to be the point, and no one questions whether they solve the right problem. When it opens with the problem, the features are evaluated against it, and the team can assess whether they are the right response. This ordering keeps the document honest about what is fixed, the problem, and what is open, the solution.

### State Goals And Success Measures Before Requirements

After the problem, state the goals: what outcome the work is meant to achieve, and how success will be measured. Goals connect the problem to the solution, defining what a good outcome looks like in terms that can be evaluated. Success measures make the goals concrete, specifying the metrics or evidence that will indicate whether the goals were met. State these before the requirements, because the requirements should serve the goals, and readers need the goals to evaluate whether the requirements do.

Goals and measures also create accountability. A PRD with clear goals can be evaluated after delivery: did the work achieve what it set out to? A PRD without goals cannot, because there is no definition of success to check against. This turns the PRD from a build list into a commitment to an outcome, which is what makes it a product document rather than a task list.

### Define Scope Explicitly, Including What Is Out Of Scope

Scope defines what the work includes and, equally important, what it excludes. Explicit scope prevents scope creep, because it gives the team a clear boundary to reference when new requests arise. It also prevents misunderstandings, because readers know what to expect and what not to expect from the work. State both in-scope and out-of-scope items, and be specific: vague scope statements like "the core functionality" leave room for disagreement about what core means.

Out-of-scope is often more valuable than in-scope, because it is where the hardest conversations happen. Deciding what not to do requires judgment about what is essential versus nice-to-have, and documenting these decisions creates a reference for when the team is tempted to expand. A PRD that clearly states what is deferred or excluded is more useful than one that lists everything as potentially included, because clarity enables commitment.

### Write Requirements That Are Clear, Unambiguous, And Appropriately Detailed

Requirements are the core of the PRD, specifying what the system must do. They should be clear, meaning each requirement has one interpretation; unambiguous, meaning different readers understand the same thing; and appropriately detailed, meaning they specify enough to build from without over-specifying the implementation. Write each requirement as a distinct statement, avoid combining multiple requirements into one, and use consistent language so that readers can parse them reliably.

Appropriate detail is a balance. Too little detail leaves the team guessing, producing work that misses the intent. Too much detail becomes a specification that removes the team's discretion and that becomes a burden to maintain. Aim for the level of detail that enables the team to build the right thing while leaving room for them to determine the best way to build it. This often means specifying the what and the constraints while leaving the how open.

### Include The Context Stakeholders Need To Make Good Decisions

A PRD should include the context that stakeholders need to make decisions about the work: who the users are, what constraints apply, what assumptions are being made, what risks exist, and what dependencies the work has. This context is what enables informed review and commitment, because decisions made without it are guesses. Include user personas or segments, technical and business constraints, key assumptions that the requirements depend on, identified risks and mitigations, and dependencies on other work or teams.

Context also includes the rationale for decisions, especially decisions that might seem surprising. If a requirement rules out an obvious alternative, explain why. If the scope excludes something that seems important, explain the reasoning. Documenting rationale prevents the team from re-litigating decisions and helps future readers understand why the PRD says what it does. A PRD without rationale is a set of assertions; one with rationale is a reasoned argument.

### Structure The Document For Navigation And Review

A PRD is read by different audiences for different purposes: engineers looking for requirements, designers looking for user context, leaders looking for goals and scope, and stakeholders looking for impact. Structure the document so that each audience can find what they need quickly. Use clear headings, a logical progression from problem to goals to scope to requirements, and summaries or overviews that allow readers to grasp the essence without reading every detail. A well-structured document respects the reader's time by making information findable.

Reviewability is also important. A PRD is reviewed before it is committed, and the structure should facilitate review by making it easy to identify what is being proposed, what the open questions are, and what decisions are needed. Include a summary at the top, call out open questions explicitly, and version the document so that reviewers can track changes. A document that is easy to review gets better review, because reviewers can focus on substance rather than navigation.

## Common Traps

### Feature List Without Problem Context

Opening with features rather than the problem. The trap is requirements that appear arbitrary and cannot be evaluated against intent.

### Missing Goals And Success Measures

Requirements without defined outcomes. The trap is work that cannot be evaluated for success after delivery.

### Vague Or Undefined Scope

Failing to state what is excluded. The trap is scope creep and disagreement about what the work includes.

### Ambiguous Requirements With Multiple Interpretations

Requirements that different readers understand differently. The trap is work that misses the intent because the team guessed wrong.

### Missing Context And Rationale

Assertions without the reasoning behind them. The trap is decisions that get re-litigated and context that is lost over time.

### Document Structured For Writing Rather Than Reading

Organization that obscures rather than communicates. The trap is readers who cannot find what they need and reviews that miss important issues.

## Self-Check

- [ ] The PRD leads with the problem and context before any solution, establishing why the work matters.
- [ ] Goals and success measures are stated before requirements, defining what a good outcome looks like.
- [ ] Scope is defined explicitly, including specific out-of-scope items that prevent scope creep.
- [ ] Requirements are clear, unambiguous, and appropriately detailed without over-specifying implementation.
- [ ] The document includes the context stakeholders need: users, constraints, assumptions, risks, dependencies, and rationale.
- [ ] The structure supports navigation and review, with clear headings, logical progression, and summaries.
- [ ] Open questions are called out explicitly, and the document is versioned to track changes through review.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
