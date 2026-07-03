---
name: functional_specifications.md
description: Use when the agent is writing functional specifications, deciding what system behaviors and rules to document, specifying workflows and logic with precision, or ensuring functional specs communicate behavior clearly to builders without dictating implementation or leaving critical behavior undefined.
---

# Functional Specifications

A functional specification describes what the system does: the behaviors, workflows, business rules, and logic that define how the system responds to inputs and events. It is the bridge between the user-facing requirements and the implementation, translating what users need into the specific behaviors the system must exhibit. Done well, a functional spec gives builders a precise understanding of the behavior expected, reducing ambiguity and enabling accurate implementation. Done poorly, it is either so high-level that it restates the requirements without adding precision, or so detailed that it dictates implementation and becomes a burden to maintain. Agents often write functional specs as restatements of requirements, missing the precision that makes a spec valuable, or as implementation designs, overstepping into how the system should be built.

The harm this skill prevents is the ambiguity that leads to incorrect implementation. When a functional spec does not specify the behaviors precisely, the builders infer them, and their inferences are often wrong, producing a system that behaves differently from what was intended. When a spec dictates implementation, it removes the builders' ability to find better approaches and becomes obsolete when the implementation changes. Good functional specification adds precision about behavior without prescribing implementation, which is the balance that makes the spec useful.

Use this skill before answering questions such as "how do we write a functional spec", "what should a functional specification contain", "how detailed should the spec be", or "how do we specify behavior precisely". The goal is to prevent the agent from producing a spec that is either too vague to guide implementation or too prescriptive to allow good engineering.

## Core Rules

### Specify Behavior, Not Implementation

A functional specification describes what the system does, not how it does it. Behavior is the observable response of the system to inputs and events: when this happens, the system does that. Implementation is the internal mechanism that produces the behavior: the algorithms, data structures, and architecture. Specify behavior at a level that defines what the system must do, and leave the implementation to the engineers, who can choose the best way to achieve the behavior. This keeps the spec stable when implementation changes and preserves engineering discretion.

The line between behavior and implementation can be subtle. "When the user submits the form, the system validates the fields and displays errors for any invalid ones" is behavior. "The system uses a regex validator on each field and renders error spans in the DOM" is implementation. The distinction is whether the statement describes what the user or external observer sees versus what happens internally. When in doubt, ask whether the statement describes an observable response; if yes, it is behavior; if it describes an internal mechanism, it is implementation and belongs in a design document, not a functional spec.

### Define Workflows As Sequences Of States And Transitions

Workflows are central to functional specifications, and they should be defined as sequences of states and the transitions between them. A state is a condition the system or an entity can be in; a transition is the event or action that moves from one state to another. Defining workflows this way, often through state diagrams or state tables, makes the behavior precise and complete, because it accounts for every state and every possible transition, including those that are not allowed. This precision prevents the gaps where behavior is undefined.

State-based specification also surfaces edge cases naturally. When you enumerate the states and consider every possible event in each, you discover transitions that the requirements did not address: what happens if the user attempts an action from a state where it is not allowed, what happens if an event occurs in an unexpected order, what happens if the system receives conflicting inputs. These discoveries lead to specified behavior for cases that would otherwise be left to inference, which is one of the main values of functional specification.

### Specify Business Rules With Conditions And Outcomes

Business rules are the logic that governs system decisions: who can do what, under what conditions, with what outcomes. Specify each rule with its conditions and its outcomes, so that the rule is unambiguous. "A user with an overdue invoice cannot place new orders" is a rule, but it is incomplete: what counts as overdue, does the rule apply to all users or some, what happens when the user tries, are there exceptions? Specify the conditions fully and the outcomes explicitly, so that the rule can be implemented without interpretation.

Business rules often interact, and a good functional spec accounts for the interactions. Two rules that each make sense alone may conflict when applied together, producing behavior that no one intended. Specifying rules in isolation misses these conflicts; specifying them together, and tracing through the scenarios where they interact, reveals the conflicts and allows them to be resolved in the spec rather than in production. Treat the set of rules as a system, not just a list, and verify that the system produces coherent behavior.

### Specify Data And Its Lifecycle

Functional behavior depends on data: what data the system holds, how it changes, and what constraints it must satisfy. Specify the data that the behaviors operate on, including the entities, their attributes, the relationships between them, and the constraints that govern them. Also specify the data lifecycle: how data is created, updated, and removed, and what invariants must hold throughout. This data specification is part of the functional spec because behavior and data are intertwined: behaviors read and write data, and data constraints shape what behaviors are possible.

Data specification should focus on the functional aspects: what the data represents, what constraints it must satisfy, and how behaviors affect it. It should not stray into physical data modeling, which is implementation. The line is between the logical data that the behavior depends on and the physical storage that the engineers choose. Specify the logical data and its constraints, and leave the physical model to the implementation, while ensuring that the logical model is precise enough to guide the physical one.

### Address Error And Exception Behavior Explicitly

Error and exception behavior is where functional specs are most often incomplete. The happy path, where everything goes right, is usually well-specified, but the error paths, where inputs are invalid, preconditions are not met, or systems fail, are often left undefined. This leaves the builders to infer error behavior, which produces inconsistent and often poor user experiences. Specify error and exception behavior explicitly: what errors can occur, how the system detects them, what the system does in response, and what the user experiences.

Error behavior should be specified with the same precision as happy-path behavior. For each possible error, define the condition that triggers it, the system's response, and the user-facing outcome. Consider validation errors, permission errors, conflicts, timeouts, and system failures. Each should have specified behavior, so that the system handles errors consistently and intentionally rather than as an afterthought. The quality of error handling often determines the quality of the user experience, and it depends on the functional spec addressing it.

### Make The Specification Complete And Consistent

A functional specification should be complete, covering all the behaviors the system must exhibit, and consistent, with no contradictions between its parts. Completeness means that a builder can implement the system from the spec without needing to infer undefined behavior. Consistency means that the spec does not specify contradictory behaviors, which would make implementation impossible. Both properties are hard to achieve and require careful review, because specs are written incrementally and inconsistencies creep in across sections.

Review the spec for completeness by tracing through scenarios and checking that every event in every state has specified behavior, that every rule has complete conditions and outcomes, and that every error case is addressed. Review for consistency by checking that rules do not conflict, that workflows do not specify contradictory transitions, and that terminology is used consistently throughout. These reviews catch the gaps and contradictions that would otherwise produce incorrect implementation, and they are an essential part of producing a functional spec that can be built from.

## Common Traps

### Specifying Implementation Instead Of Behavior

Describing internal mechanisms rather than observable responses. The trap is a spec that constrains engineering and becomes obsolete when implementation changes.

### Restating Requirements Without Adding Precision

Specs that repeat the requirements at the same level. The trap is a spec that adds no value over the requirements and leaves behavior ambiguous.

### Undefined Behavior In Edge Cases

Gaps where the spec does not say what happens. The trap is builders inferring behavior, often incorrectly.

### Isolated Business Rules That Conflict When Combined

Rules that make sense alone but produce unintended behavior together. The trap is a system whose behavior no one intended.

### Happy Path Only, Errors Undefined

Specifying success behavior while leaving error handling to inference. The trap is inconsistent and poor error experiences.

### Inconsistent Terminology And Contradictions

Specs written incrementally that contradict themselves. The trap is a spec that cannot be implemented because it specifies conflicting behaviors.

## Self-Check

- [ ] The specification describes behavior, not implementation, preserving engineering discretion.
- [ ] Workflows are defined as states and transitions, accounting for every possible event in every state.
- [ ] Business rules are specified with complete conditions and outcomes, and their interactions are traced for conflicts.
- [ ] Data and its lifecycle are specified at the logical level, with constraints and invariants defined.
- [ ] Error and exception behavior is specified explicitly for each possible error, with detection, response, and user experience defined.
- [ ] The specification is complete, with no undefined behavior, and consistent, with no contradictions.
- [ ] A builder could implement the system from the spec without inferring undefined behavior.
