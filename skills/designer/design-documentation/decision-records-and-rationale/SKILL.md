---
name: decision_records_and_rationale.md
description: Use when the agent is recording a design decision and its rationale, writing a decision record or ADR for design, capturing tradeoffs, documenting why an option was chosen over alternatives, logging design changes and reversals, or building a decision log that lets future designers understand and revisit past choices instead of relitigating them.
---

# Decision Records And Rationale

Most design decisions are not self-explanatory. A chosen layout, interaction, or component structure carries assumptions about user needs, constraints, tradeoffs, and evidence that were visible at the time and invisible later. Without a record, future designers either relitigate settled decisions endlessly or inherit a decision they do not understand and reverse it for reasons that were already considered and rejected.

A decision record is not a justification written to defend a choice; it is a durable artifact that captures the context, the alternatives, the tradeoffs, and the conditions under which the decision should be revisited. Agents tend to fail here by recording only the conclusion, by writing rationale that justifies rather than explains, or by omitting the alternatives and rejected options that are the most valuable part of the record.

## Core Rules

### Capture Context Before Conclusion

A decision record that opens with the chosen option is already failing, because the reader cannot evaluate whether the choice still fits without knowing the situation that produced it. Start with the context.

Record the following context:

- the problem being solved;
- the constraints in force;
- the user or business need;
- the timeline;
- what was known and unknown at the time;
- the date and the people involved.

State the forces in tension: the competing goals that made the decision hard. Context is what makes a record reusable; without it, the conclusion is an assertion that cannot be checked against changed conditions. Decisions are made by people with specific knowledge, and recording authority matters when a decision is revisited.

### Make The Alternatives And Rejected Options Visible

The most valuable part of a decision record is not the chosen option; it is the options that were considered and rejected, and why.

For each significant alternative, record:

- what it was;
- its advantages;
- its disadvantages;
- the specific reason it was not chosen.

This prevents two failure modes: future designers proposing an alternative that was already evaluated and rejected for good reasons, and future designers unable to tell whether a new circumstance invalidates a previously rejected option. A record with only the chosen option forces every future reader to redo the analysis from scratch. The rejected options are the institutional memory.

### State The Tradeoffs Explicitly, Including What Was Sacrificed

Every non-trivial design decision trades one good for another. Name what was prioritized and what was given up.

Common tradeoffs include:

- speed versus completeness;
- consistency versus flexibility;
- simplicity versus power;
- short-term effort versus long-term maintainability.

A record that presents a decision as having no downside is not credible and is not useful, because it hides the cost that someone will eventually pay. When the tradeoff is explicit, the team can monitor whether the sacrificed value is becoming a problem and revisit the decision deliberately rather than being surprised. Record the assumptions that the tradeoff depends on, so that if an assumption is falsified, the decision is flagged for review.

### Define The Reversal Conditions

A decision record should state the conditions under which the decision should be revisited, not as a vague "if things change" but as specific triggers.

Make triggers concrete, for example:

- if a measured metric crosses a threshold;
- if a constraint is removed;
- if a new user segment becomes primary;
- if a dependent system changes.

This converts the record from a static justification into a living instrument. Without reversal conditions, decisions persist long after their rationale is obsolete, because no one knows they are due for review. Reversal conditions also force the author to be honest about the uncertainty in the decision: if you cannot state what would change your mind, you have not fully understood the decision.

### Separate Facts, Assumptions, And Judgments

Conflating verified evidence with assumptions and subjective judgment undermines the record's usefulness. Label each kind of input clearly.

Distinguish between:

- what was measured or observed, as facts;
- what was assumed, as assumptions;
- what was a judgment call, as judgment.

This lets future readers weigh the decision appropriately: a decision resting on a measured user need is different from one resting on a team preference, and each should be revisited under different conditions. It also protects against the record being dismissed wholesale when one assumption turns out wrong, because the reader can see which parts were evidence and which were judgment.

### Record Reversals And Supersessions Explicitly

When a decision is reversed or superseded, do not delete the old record. Mark it superseded, link to the new decision, and record what changed in the context, evidence, or assumptions that justified the reversal.

A history of reversals is valuable because it:

- shows which kinds of decisions the team tends to get wrong;
- prevents the cycle of reversing and re-reversing the same decision as context oscillates;
- preserves the reasoning that led to the original choice.

Deleting or silently overwriting old decisions destroys the institutional memory that the record system exists to preserve.

### Keep Records Discoverable And Linked To The Work

A decision record that no one can find does not exist. Store records where the related design work lives, link them from the components, specs, or guidelines they affect, and maintain an index. Cross-link related decisions so a reader can follow a chain of reasoning. The record system is only useful if a designer working on the affected surface can find the relevant decisions without already knowing they exist.

## Common Traps

### Recording Only The Conclusion

A record that states the chosen option without context, alternatives, or tradeoffs cannot be evaluated later and forces re-litigation. The conclusion is the least useful part on its own.

### Rationale That Justifies Instead Of Explains

Writing to defend a choice, rather than to explain the reasoning, produces records that hide uncertainty and tradeoffs. Explain, including the weaknesses.

### Omitting Rejected Alternatives

Without the options considered and rejected, future designers propose already-rejected ideas or cannot tell when a rejected option becomes viable. Rejected options are the core value.

### Presenting A Decision As Having No Downside

Every real tradeoff sacrifices something. Hiding the cost makes the record dishonest and leaves the team unable to detect when the sacrificed value becomes a problem.

### No Reversal Conditions

Without explicit triggers for revisit, decisions persist past their useful life because no one knows they are due for review. State what would change your mind.

### Conflating Evidence With Assumption

Mixing measured facts with guesses makes the record impossible to weigh when conditions change. Label each kind of input.

### Deleting Or Silently Overwriting Old Decisions

Erasing reversed decisions destroys the history that prevents oscillation. Mark superseded and link forward.

## Self-Check

- [ ] The record opens with context: problem, constraints, forces in tension, what was known and unknown, date, and decision-makers.
- [ ] The alternatives considered are listed, each with advantages, disadvantages, and the specific reason it was rejected.
- [ ] The tradeoff is explicit: what was prioritized, what was sacrificed, and the assumptions the tradeoff depends on.
- [ ] Reversal conditions are stated as specific triggers, not vague "if things change."
- [ ] Facts, assumptions, and judgments are labeled separately so each can be weighed on its own.
- [ ] The chosen option is connected to the alternatives and tradeoffs, not presented in isolation.
- [ ] Superseded decisions are marked, not deleted, and linked to the replacing decision with the reason for reversal.
- [ ] The record is stored where the related work lives and is linked from the components, specs, or guidelines it affects.
- [ ] The record explains the reasoning rather than defending the choice, and acknowledges the weaknesses of the chosen option.
- [ ] A designer arriving months later could determine whether the decision still holds and what would justify revisiting it.
