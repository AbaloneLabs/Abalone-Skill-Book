---
name: decision_recording_and_rationale_preservation.md
description: Use when the agent is recording project decisions, preserving the rationale and alternatives considered, maintaining a decision log or ADR, or preventing decisions from being silently reversed, forgotten, or re-litigated months later.
---

# Decision Recording and Rationale Preservation

A decision that is made but not recorded does not exist as far as the project is concerned. Within weeks, no one remembers why it was made; within months, someone proposes the alternative that was already rejected, and the team re-litigates a decision they already paid the cost to settle. Worse, decisions get silently reversed when a new stakeholder arrives, reads only the outcome, and assumes the choice was arbitrary. The decision record is what turns a moment of judgment into a durable project asset, and its absence is one of the most common causes of rework, reopened debates, and eroded trust in governance.

The judgment problem is not "write down what we decided." It is how to capture enough context that a future reader can understand and respect the decision, without so much ceremony that people stop recording decisions at all. Agents tend to record the outcome and drop the rationale, to let the record decay into a stale archive nobody trusts, or to treat the log as compliance paperwork rather than as a tool the team actually consults.

## Core Rules

### Record the Decision While the Context Is Fresh

Memory degrades fast and rationale degrades faster. Capture the decision and its reasoning within the same cycle in which it was made, ideally before the meeting ends. A decision recorded a week later is a reconstruction, and reconstruction quietly replaces the actual reasoning with a tidier story. Make recording part of the decision ritual, not a follow-up task that gets dropped under deadline pressure.

### Capture Rationale, Alternatives, and the Rejected Path

The most valuable part of a decision record is not the outcome; it is why the chosen option beat the alternatives, and what the rejected options were. Without this, a future reader cannot tell whether a proposal was considered and dismissed for a reason that still holds, or simply never came up. Record the alternatives considered, the criteria that distinguished them, and the specific reason each alternative was rejected. This is what prevents the same debate from recurring.

### State the Assumptions and Conditions That the Decision Depends On

A decision is contingent. It was made under assumptions about cost, schedule, vendor behavior, regulatory stance, or market conditions, and it is correct only while those assumptions hold. Record the assumptions explicitly so that when they change, the decision can be revisited deliberately rather than drift into obsolescence. A decision record without its assumptions cannot tell whether it is still valid.

### Make the Decision Log Searchable and Trusted as a Single Source

A decision log that no one can find, or that everyone suspects is incomplete, provides no protection. Maintain one authoritative location, whether a decision register, an architecture decision record (ADR) series, or a tagged section of the project wiki, and make it the place people check before reopening a question. Fragmented records across email, chat, and slide decks guarantee that the most important decisions are the ones never found. Consolidate and index.

### Distinguish Decided, Proposed, Superseded, and Deprecated

A flat list of decisions hides the ones that have been replaced. Each decision should carry a status: proposed, decided, implemented, superseded, or deprecated. When a decision is reversed, do not delete the old record; mark it superseded and link to the new one, with the reason for the reversal. This preserves the history of how the project's thinking evolved and prevents confusion about which decision is currently in force.

### Link Decisions to the Artifacts They Shape

A decision that selected a technology, a vendor, or an architecture should link to the requirements, contracts, or designs it produced, and vice versa. This traceability lets a reader follow a decision into its consequences and follow an artifact back to the decision that authorized it. Without links, decisions float free of the work they govern, and the team cannot tell which decisions are actually load-bearing.

### Record Dissent and Accepted Risk, Not Just Agreement

A decision record that presents only the consensus view hides the tradeoffs and the dissent that were overcome. Record the notable objections raised, the dissenting positions, and the risks that were consciously accepted. This does two things: it honors the minority view so people do not relitigate from a sense of being unheard, and it gives future readers the early-warning signals that were already on the table when the decision was made.

### Make Reversal Explicit and Costed

When a decision is reversed, the reversal is itself a decision and deserves its own record. Document what changed (assumption, evidence, stakeholder, context), what the cost of reversing is, and what the new decision commits to. Silent reversal, where the team just starts doing something different without acknowledging the change, destroys the credibility of the entire log. If the record cannot be trusted to reflect reality, people stop consulting it.

## Common Traps

### Outcome Without Rationale

Recording only what was decided, with no reasoning or alternatives, produces a log that tells a future reader nothing useful. The trap is that the record exists and looks complete, so everyone assumes the governance is working. When the decision is later challenged, there is nothing to defend it with. Always capture the why and the rejected options, not just the what.

### Reconstruction Masquerading as Record

A decision written up days later, after memories have settled into a cleaner narrative, omits the messy tradeoffs and the dissent that were actually present. The trap is that the reconstruction feels accurate because it is internally consistent. Record decisions live, while the disagreement and the uncertainty are still visible.

### The Log as Stale Archive

The decision log was maintained diligently for the first month and then abandoned, so the most recent and most important decisions are precisely the ones missing. The trap is that a partial log is worse than none, because it gives false confidence that decisions are being captured. Either maintain the log as a living artifact or formally retire it; do not let it decay silently.

### Silent Reversal

The team reverses a decision by simply starting to work differently, without updating the record. The trap is that the old record still says the opposite, so anyone consulting it is misled, and the credibility of the whole system collapses. Treat reversal as a first-class decision with its own record and its own rationale.

### Record Overload That Stops People Recording

If every minor decision requires a full formal record, people stop recording the small ones and then, by habit, the large ones too. The trap is that ceremony designed to ensure quality instead suppresses capture. Tier the record depth to the decision's consequence: a one-line entry for routine decisions, a full ADR for consequential ones.

### Decisions Without Status or Lifecycle

Every entry reads as if currently in force, so no one can tell which decisions have been superseded or deprecated. The trap is that the log accumulates contradictions over time, and a reader has no way to know which decision governs today. Every record needs a status, and supersession must be explicit and linked.

### Dissent Scrubbed for Clean Consensus

The record presents a unanimous view that never existed, erasing the objections and the accepted risk. The trap is that this looks more professional but removes exactly the information a future reader needs to spot when a decision is about to fail. Preserve the dissent and the risk acceptance; they are the early-warning system.

### Fragmented Records Across Tools

Decisions live in email threads, chat messages, slide decks, and individual notebooks, with no single source. The trap is that the most important decisions are the hardest to find, because they were made in the highest-context, lowest-recorded channels. Consolidate into one searchable location and reference it from everywhere else.

## Self-Check

- [ ] Is each decision recorded while context is fresh, within the same cycle it was made?
- [ ] Does each record capture the alternatives considered and the specific reason each was rejected?
- [ ] Are the assumptions and conditions the decision depends on stated explicitly?
- [ ] Is there a single authoritative, searchable location that the team trusts as the source of decisions?
- [ ] Does every decision carry a status (proposed, decided, implemented, superseded, deprecated)?
- [ ] Are superseded decisions preserved and linked to their replacements rather than deleted?
- [ ] Are decisions linked to the requirements, contracts, or designs they authorized?
- [ ] Does the record preserve notable dissent, objections, and consciously accepted risk?
- [ ] Is every reversal recorded as its own decision with the changed condition and the cost of reversing?
- [ ] Is the record depth tiered to consequence so ceremony does not suppress capture of small decisions?
