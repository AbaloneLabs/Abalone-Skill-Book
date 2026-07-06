---
name: engineering-handoff-and-feasibility-alignment.md
description: Use when the agent is handing a design off to engineering, evaluating technical feasibility of a feature, negotiating scope against engine constraints, defining system architecture boundaries with programmers, or translating player-facing design intent into buildable technical requirements.
---

# Engineering Handoff and Feasibility Alignment

The moment a design leaves the designer's document and enters engineering's backlog, it crosses a boundary where every ambiguity becomes someone else's decision, and that someone is usually a programmer optimizing for implementation ease rather than player experience. The judgment problem is that design intent and technical feasibility pull in opposite directions: the designer wants the richest possible expression of the feature, and the engineer wants the smallest, most decoupled, most cache-friendly thing that ships. Neither is wrong, and the harm occurs when the handoff treats feasibility as a veto the engineer invokes late, or treats design as a wish list the engineer silently trims. Agents miss the critical work because the handoff feels like a document transfer rather than a negotiation, the feasibility questions that matter are buried in edge cases the designer never considered, and the cost of a wrong architecture decision is invisible until it blocks three features downstream. This skill covers how to run a handoff so that feasibility is assessed before commitment, how to separate hard engine limits from soft cost tradeoffs, and how to preserve design intent through technical compromise without turning the engineer into a co-designer by accident. The designer retains authority over player experience but must earn the right to it by engaging technical reality honestly.

## Core Rules

### Treat Feasibility as a Pre-Commitment Conversation, Not a Post-Implementation Surprise

The most expensive feasibility findings are the ones discovered after the feature is scoped, scheduled, and promised. The rule is that no feature enters the committed backlog until a named engineer has assessed it against the actual engine, the actual data structures, and the actual performance budget. This conversation must happen while the design is still plastic, not after it is approved. The designer's job is to bring the riskiest, most uncertain parts of the feature to the engineer first, not the easy parts that make the feature look de-risked. When an engineer says "this is hard," the productive response is to ask what specifically is hard and what variant would be easy, not to retreat or to push. When feasibility is genuinely blocking, surface it to production immediately rather than absorbing it as a design problem to solve alone.

### Distinguish Hard Constraints From Soft Cost Tradeoffs

Not every "we can't do that" from engineering is the same kind of statement. A hard constraint is a technical wall: the engine cannot stream that much memory, the platform cert requires a stable frame rate, the network protocol cannot carry that payload at the tick rate. A soft cost tradeoff is a price tag: it can be done, but it costs N weeks, or it risks frame hitches, or it forces a refactor. The designer must learn to classify engineering pushback into these two categories, because the responses are entirely different. Hard constraints require redesign around the wall; soft costs require a priority decision about whether the experience is worth the price. When you fail to distinguish them, you either surrender good design to a constraint that was merely expensive, or you force through a cost that quietly cripples the schedule.

### Bring the Riskiest Unknowns to Engineering First

Designers naturally want to present a feature in its most polished, convincing form, which means the uncertain parts get glossed and the confident parts get emphasized. This is backwards for feasibility. The handoff should lead with the open questions, the novel systems, the interactions no one has built before, and the edge cases most likely to break the architecture. The reasoning is that feasibility risk lives in the unknowns, and presenting confidence first delays the conversation that actually de-risks the feature. When you bring the scariest part first, the engineer can tell you immediately whether it is feasible, what it costs, and what the cheaper variant is. When you bring it last, you have already burned political capital committing to the easy shell around an infeasible core.

### Negotiate Variants, Not Vetoes

A healthy handoff produces a feature that is both buildable and faithful to intent, and that only happens when both sides propose alternatives rather than issuing verdicts. When engineering says a feature cannot be built as specified, the designer should ask what the closest buildable variant is, and the engineer should be expected to offer one rather than simply refusing. Symmetrically, when the designer must compromise, they should offer multiple intent-preserving variants rather than demanding the engineer pick. The discipline is to never accept a bare "no" or a bare "we'll do it our way" without a variant on the table. When no variant preserves the core intent, that is the signal that the feature itself needs redesign, not that one side has lost the negotiation.

### Define the System Boundary and the Data Contract Explicitly

Most handoff failures are boundary failures: the designer assumes the system handles a case the engineer assumed was outside it, or the data the designer expects to flow between systems was never specified. The rule is that every handed-off feature must state its inputs, its outputs, the systems it depends on, and the systems that depend on it, in terms an engineer can translate directly into interfaces. This is not over-specification; it is the minimum needed to prevent integration surprises. When two features share state, the ownership of that state must be named. When a feature reads data authored by another discipline, the format and the authoring path must be defined. Ambiguity at the boundary becomes a bug at integration, and that bug is always more expensive than a conversation.

### Preserve Design Intent Through Technical Compromise Without Abdicating Authority

Compromise is inevitable, but there is a difference between trading a non-essential detail to save weeks and trading the core experience because the engineer found it inconvenient. The designer must know which elements of a feature are load-bearing for the player experience and which are decoration, and must defend the former while freely conceding the latter. The failure mode at one extreme is the designer who concedes everything and ships a feature that technically works but feels nothing like the intent; at the other extreme is the designer who refuses all compromise and becomes a blocker the team routes around. The judgment is to articulate, for each feature, the one or two things that must survive any technical compromise, and to be flexible about everything else.

### Document the Decision and the Reason, Not Just the Outcome

When a feasibility conversation changes the design, the change and its rationale must be recorded where the team can find it. The reason matters because six months later, when someone proposes reverting the compromise or building the originally-desired version, the team needs to know whether the constraint still holds or whether it was a cost decision that may have changed. Record who decided, what the alternatives were, what the constraint or cost was, and what would need to be true to revisit it. An undocumented decision becomes an unchallengeable myth that blocks legitimate future work.

## Common Traps

### Treating the Engineer's First Estimate as a Fixed Truth

An engineer gives a four-week estimate for a feature, and the designer treats it as a physical constant and either kills the feature or accepts the cost. The trap is that early estimates are rough, often padded for unknowns, and frequently collapse after a short spike reveals the problem is simpler than feared, or expand when a hidden dependency appears. The false signal is the precision of the number; "four weeks" feels more reliable than "I'm not sure." The harm is that features are killed or bloated based on a guess treated as a measurement, and the team never invests the small spike that would have produced a real number.

### Designing to the Engine's Comfort Zone and Calling It Feasibility

The engineer reports that the feature is "feasible" only if it is reshaped to match patterns the engine already supports comfortably, and the designer accepts the reshaped version believing feasibility was the constraint. The trap is that comfort is being reported as constraint: the feature was not infeasible, it was merely unfamiliar, and the engineer's preference for known patterns was presented as a technical limit. The false signal is the confident technical framing of a preference. The harm is that the game's design space silently narrows to whatever the current architecture already does well, and novel player experiences are excluded not by real limits but by habit.

### Handing Off a Feature Without Naming a Technical Owner

A feature is described in a document, added to a shared backlog, and assumed to be owned by whoever picks it up. The trap is that feasibility was never assessed by the person who will actually build it, so the assessment that did happen was either generic or performed by someone who will not be accountable. The false signal is that the feature exists in the tracking system, which feels like commitment. The harm is that the real feasibility conversation happens at implementation time, when redesign is most expensive and the schedule has already been promised against the feature.

### Accepting "We'll Figure That Out During Implementation" for a Core Risk

A genuinely novel or risky part of the feature is flagged, and the team agrees to resolve it later during implementation. The trap is that "later" is the most expensive possible time to resolve a core risk, because by then the surrounding architecture is committed and the team is under schedule pressure to ship rather than to solve. The false signal is the collegial agreement, which feels like de-risking but actually defers it. The harm is that the risk materializes as a blocker at the worst moment, forcing either a panic redesign or a quality compromise that ships in the game.

### Letting Performance Become an Unchallengeable Design Constraint

Engineering cites frame rate or memory as the reason a feature must be cut, and the designer accepts it because performance feels like an objective limit. The trap is that performance budgets are themselves design decisions: the team chose a target frame rate, chose a memory footprint, and chose what to spend that budget on, and a feature can be cut for performance or another feature can be cut to make room. The false signal is the technical authority of the performance argument. The harm is that performance becomes a veto that preempts priority discussions, and the designer never asks what would have to be sacrificed elsewhere to afford the feature.

## Self-Check

- Has a named engineer who will be accountable for the build assessed the feature against the real engine and performance budget before it entered the committed backlog?
- For each piece of engineering pushback, have I classified it as a hard constraint versus a soft cost tradeoff, and responded with the matching action (redesign versus priority decision)?
- Did I bring the riskiest, most novel parts of the feature to engineering first, rather than presenting only the confident shell?
- When a compromise was reached, did engineering offer a buildable variant and did I offer intent-preserving alternatives, rather than either side issuing a bare verdict?
- Are the system boundary, inputs, outputs, dependencies, and data ownership for the feature stated explicitly enough to translate into interfaces?
- Have I identified which one or two elements of the feature are load-bearing for player experience and must survive compromise, versus which are freely concessionable?
- Is each feasibility-driven design change documented with the constraint, the alternatives, and the condition under which it could be revisited?
