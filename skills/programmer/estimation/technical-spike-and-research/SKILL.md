---
name: technical_spike_and_research.md
description: Use when the agent is running a technical spike or research spike to de-risk an unknown, deciding whether to build a prototype or proof of concept, time-boxing an investigation, validating a technology choice before adoption, estimating the cost of learning a new tool, documenting spike findings for a decision, or turning research output into an actionable go or no-go.
---

# Technical Spike and Research

A technical spike is a deliberate, bounded investigation whose purpose is to reduce uncertainty before a commitment, and it is consistently misused in ways that destroy its value. The first misuse is the unbounded spike: "go research this library" with no question, no time box, and no stopping condition, which becomes open-ended exploration that consumes the time it was meant to save and produces no decision. The second is the spike-as-implementation: the investigation slowly becomes the real build, accumulating technical debt and half-solutions because the prototype was never thrown away. Both failures convert a risk-reduction tool into a risk source, and both come from not being explicit about what a spike is for.

The judgment problem is that a spike is a bet that spending a little time now will save a lot of time later, and like any bet it must be sized to the question and stopped when the question is answered. The agent must define the specific question the spike exists to answer (not "evaluate this technology" but "can this library sustain our throughput with acceptable latency under our data shape"), time-box it hard (because exploration expands to fill available time), build the thinnest possible thing that answers the question (not a production prototype), and convert the finding into a decision and a revised estimate. A spike that does not change a decision, an estimate, or a plan has produced curiosity, not value. The discipline is in treating the spike as a means to a decision, not as an end.

## Core Rules

### Define the spike by a specific question and a stopping condition

The single most important rule. A spike without a precise question will wander. Frame the question concretely and falsifiably: not "evaluate Kafka" but "can Kafka sustain our peak 50k msgs/sec with end-to-end latency under 100ms given our message size and durability requirements." Define the stopping condition: what result (positive or negative) answers the question, and what will you do with each answer. If you cannot state the question and the decision each answer would drive, you do not have a spike; you have an open-ended exploration. Write the question and stopping condition down before starting, so the spike can be recognized as done.

### Time-box hard, and stop when the box expires

Exploration expands to fill available time, and a spike without a hard time box will consume the buffer it was meant to justify. Set a time box appropriate to the question (often one to three days; rarely more than a week for a genuine decision), and stop when it expires regardless of completeness. If the question is not answered when the box expires, that is itself a finding (the problem is harder than expected), and the right response is to re-decide (extend with a new box and a sharper question, or descope) rather than to silently continue. The time box forces the question to be answered with the time available, which is how real decisions get made.

### Build the thinnest thing that answers the question, not a prototype of the product

A spike should be the minimum artifact that resolves the question: a benchmark, a minimal integration, a load test against representative data, a reading of the source for the relevant code path. It is explicitly not a production-quality implementation, and it should not accumulate features, error handling, tests, or polish. The danger of building more than necessary is that the prototype becomes the implementation ("we can just ship the spike"), carrying all the shortcuts and assumptions that were acceptable for a throwaway but fatal in production. Build throwaway code, mark it as throwaway, and plan to discard it.

### Validate the realistic conditions, not the happy path

A spike that confirms the technology works on the happy path has answered the easy question and ignored the important one. Test under realistic conditions: representative data volumes and shapes, realistic concurrency, the failure modes you actually worry about (what happens when the dependency is slow, when the data is malformed, when the network partitions). A spike that only proves the library works on a small clean dataset tells you nothing about whether it will work in production. The value of the spike is in testing the conditions where you suspect it might fail, because that is where the uncertainty lives.

### Convert the finding into a decision, an estimate, or a plan

A spike's output is not a report; it is a changed decision. Before starting, know what decision each possible finding would drive (if it works, we adopt and estimate X; if it fails, we choose alternative Y or descope). After the spike, write a short finding that states the answer to the question, the evidence, and the resulting decision or revised estimate. If the spike does not change a decision, an estimate, or a plan, it produced curiosity rather than value, and the question it answered was not decision-relevant. Link the finding to the ADR or planning artifact so the decision is recorded and not relitigated.

### Distinguish a spike from the work, and discard the artifact

Be explicit that the spike is research, not implementation, and plan to throw away its code. The production implementation starts fresh, informed by what the spike learned but not built on top of the spike's shortcuts. The temptation to ship the spike is strong ("it works, why rewrite?") but it carries hidden debt: no error handling, no tests, hardcoded values, and assumptions that were fine for a benchmark but wrong for production. If the spike's code is genuinely worth keeping, promote it deliberately (add tests, error handling, review) rather than shipping it as-is.

### Sequence spikes before the commitment, not after the surprise

A spike's value is in preventing a commitment made on false assumptions. Run the spike before the estimate is locked and the date is set, not after the team has committed and discovered the problem mid-implementation. Build spike time into the plan as an explicit pre-commitment step for uncertain work. A spike run after the commitment, to investigate a problem already encountered, is damage control, not risk reduction.

## Common Traps

### The unbounded spike with no question or stopping condition

"Research this technology" wanders indefinitely and produces no decision. Define a specific, falsifiable question and a stopping condition before starting.

### No hard time box, so exploration expands to fill available time

A spike without a time box consumes the buffer it was meant to justify. Set a hard box and stop when it expires, treating an unanswered question as a finding.

### Building a product prototype instead of the thinnest answer

A spike that accumulates features and polish becomes the implementation, carrying throwaway shortcuts into production. Build the minimum artifact that answers the question and plan to discard it.

### Validating only the happy path

A spike that works on small clean data tells you nothing about production. Test realistic volumes, concurrency, and the failure modes you actually suspect.

### Producing a report instead of a decision

A spike that does not change a decision, estimate, or plan produced curiosity, not value. Know the decision each finding will drive, and record the resulting decision.

### Shipping the spike as the implementation

The throwaway code lacks tests, error handling, and production assumptions. Discard it and build the real implementation fresh, or promote it deliberately with full quality bars.

### Running the spike after the commitment

A spike prevents false-assumption commitments; run it before the estimate is locked, not as damage control after a mid-implementation surprise.

## Self-Check

- Is the spike defined by a specific, falsifiable question and a stopping condition (what each answer would drive), written down before starting?
- Is there a hard time box, with a rule to stop when it expires and treat an unanswered question as a finding rather than silently continuing?
- Is the spike the thinnest artifact that answers the question (benchmark, minimal integration, source read), explicitly not a product prototype, and marked as throwaway?
- Does the spike test realistic conditions (representative data, concurrency, and the failure modes you suspect), not just the happy path?
- Is the spike's output a decision, revised estimate, or plan change (recorded and linked to an ADR), rather than a report that changes nothing?
- Is the spike's code discarded, with the production implementation built fresh (or promoted deliberately with tests and review), rather than shipped as-is?
- Is the spike sequenced before the commitment as a pre-commitment risk-reduction step, rather than run as damage control after a surprise?
- Could you state, before starting, what decision each possible finding would drive, so the spike is known to be decision-relevant?
