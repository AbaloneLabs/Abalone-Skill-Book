---
name: given_when_then_and_examples.md
description: Use when the agent is writing acceptance criteria or examples in Given-When-Then format, structuring scenarios with context action and outcome, using concrete examples to clarify behavior, or deciding when GWT adds value versus when it overcomplicates simple criteria.
---

# Given-When-Then And Examples

Given-When-Then (GWT) is a structure for expressing acceptance criteria and examples as scenarios: Given some context, When an action occurs, Then an outcome results. It is powerful for clarifying behavior because it forces specificity about the starting state, the triggering action, and the expected result, leaving little room for ambiguity. It also connects naturally to automated testing, where scenarios become executable specifications. Done well, GWT scenarios are clear, concrete, and verifiable, communicating exactly what behavior is expected. Done poorly, they are either so abstract that they add nothing, or so numerous and rigid that they become a burden that documents every interaction without illuminating the important ones. Agents often overuse GWT, applying it to every criterion including simple ones that do not benefit from the structure, creating noise without value.

The harm this skill prevents is the ambiguous criterion and the bloated scenario set. Ambiguity arises when criteria do not specify the context, action, and outcome clearly, leaving the team to interpret. Bloat arises when GWT is applied indiscriminately, producing dozens of scenarios that document trivial interactions while obscuring the behaviors that matter. Good use of GWT clarifies the important behaviors without burying them in trivial scenarios.

Use this skill before answering questions such as "how do we write Given-When-Then scenarios", "should we use GWT for these criteria", "how many scenarios do we need", or "how do we use examples to clarify behavior". The goal is to prevent the agent from either under-using GWT where it would clarify or over-using it where it adds noise.

## Core Rules

### Use Given-When-Then For Behaviors That Benefit From Scenario Structure

GWT adds value when a criterion involves a specific context, a triggering action, and an outcome that depends on both. These are behaviors where the interplay of state and action matters, and where spelling out the scenario removes ambiguity. "Given the user has an unpaid invoice over 30 days old, When they attempt to place a new order, Then they are prompted to pay the outstanding invoice first" is a behavior that benefits from scenario structure, because the outcome depends on the specific context and action. Use GWT for these, because the structure enforces the clarity the behavior needs.

GWT does not add value for simple criteria that are just conditions to check. "The submit button is disabled until all required fields are valid" is clear as a statement and does not need scenario structure. Forcing it into GWT produces noise without added clarity. Reserve GWT for behaviors where the scenario structure earns its keep, and use simpler forms for criteria that do not need it.

### Make Scenarios Concrete With Specific Data And Conditions

The power of GWT comes from concreteness. A scenario with specific data and conditions communicates exactly what is expected and can be verified directly. "Given the user has three items in their cart totaling 150 dollars, When they apply a 10 percent discount code, Then the total becomes 135 dollars" is concrete and verifiable. Abstract scenarios, "Given a user with items, When a discount is applied, Then the total is reduced," add little, because they leave the specifics that would clarify and verify the behavior undefined. Use concrete data that makes the scenario a real example.

Concrete data also exposes edge cases and assumptions. When you specify particular values, you discover questions: what if the discount exceeds the total, what if the cart is empty, what if the code is expired? These questions, surfaced by concreteness, lead to additional scenarios that cover important behaviors. Abstract scenarios hide these questions, leaving them to be discovered during implementation or, worse, after delivery.

### Express The Outcome As An Observable, Verifiable Result

The Then clause should express an observable, verifiable result, not a vague expectation. "Then the user sees a confirmation message" is observable. "Then the user is happy" is not observable or verifiable. The outcome should be something that an observer could confirm by examining the system, whether by seeing a message, checking a state, measuring a response, or verifying a side effect. Vague outcomes defeat the purpose of GWT, because they reintroduce the ambiguity the structure was meant to remove.

This connects to testability: a GWT scenario with an observable outcome is directly testable, because the outcome can be checked. A scenario with a vague outcome is not. When writing the Then clause, ask how someone would verify it; if there is no clear way, the outcome needs to be made observable before the scenario is useful.

### Use Examples To Clarify And To Surface Edge Cases

Examples are a complement to GWT, providing concrete instances that illustrate and test the behavior. A set of examples for a discount rule might include the normal case, the edge case where the discount equals the total, the edge case where the cart is empty, and the error case where the code is invalid. Each example clarifies the behavior in a specific situation and surfaces questions about how the system should handle it. Use examples to explore the behavior space, not just to document the happy path.

The value of examples is in the edge cases they reveal. The happy path is usually obvious; the edge cases are where ambiguity and disagreement hide. By deliberately constructing examples that probe the boundaries, the empty, the full, the invalid, the concurrent, the team surfaces the questions that would otherwise emerge late. These examples become scenarios that define behavior at the edges, preventing the gaps that lead to incorrect implementations.

### Avoid Scenario Bloat By Focusing On Important Behaviors

GWT can produce bloat when applied to every possible interaction, resulting in dozens of scenarios that document trivial behaviors while obscuring the important ones. Avoid this by focusing scenarios on the behaviors that matter: the ones that carry user value, that involve risk, that have ambiguous requirements, or that cover critical edge cases. Trivial behaviors that are obvious from the story do not need scenarios. The goal is a set of scenarios that illuminates, not one that exhaustively documents.

A good test is whether removing a scenario would leave a gap in understanding. If the behavior is obvious without it, the scenario is noise. If removing it would leave the team uncertain about an important behavior, the scenario earns its place. Curate the scenario set to include what illuminates and exclude what merely documents, keeping the set valuable rather than voluminous.

### Connect Scenarios To Automated Tests Where It Adds Value

GWT scenarios connect naturally to automated testing, because their structure maps to test setup, action, and assertion. Where automated tests add value, expressing scenarios in GWT enables them to become executable specifications that verify the behavior on every build. This is powerful for behaviors that are important and stable, where the investment in automation pays off through repeated verification. Connect scenarios to tests for these behaviors.

Not every scenario needs automation. Scenarios for exploratory behaviors, for one-time validations, or for rapidly changing features may not justify automation, and forcing automation creates maintenance burden without commensurate value. Apply automation where the behavior is important, stable, and repeatedly verified, and leave other scenarios as documentation. The connection to tests is an opportunity, not an obligation.

## Common Traps

### GWT Applied To Every Criterion Indiscriminately

Using scenario structure for simple conditions. The trap is noise that obscures the behaviors that matter.

### Abstract Scenarios Without Concrete Data

Scenarios with vague context and outcomes. The trap is structure that adds no clarity over a simple statement.

### Vague Outcomes That Cannot Be Verified

Then clauses expressing feelings or expectations. The trap is scenarios that cannot be tested or confirmed.

### Happy-Path-Only Examples

Documenting only the normal case. The trap is edge cases and ambiguities that remain hidden until late.

### Scenario Bloat Obscuring Important Behaviors

Dozens of trivial scenarios. The trap is a set that documents everything and illuminates nothing.

### Mandatory Automation Of All Scenarios

Automating trivial or volatile scenarios. The trap is maintenance burden without commensurate verification value.

## Self-Check

- [ ] GWT is used for behaviors that benefit from scenario structure, not applied to every criterion indiscriminately.
- [ ] Scenarios use concrete data and conditions, making them real examples rather than abstractions.
- [ ] The Then clause expresses an observable, verifiable result, not a vague expectation.
- [ ] Examples deliberately probe edge cases and boundaries, not just the happy path.
- [ ] The scenario set focuses on important behaviors, avoiding bloat that obscures rather than illuminates.
- [ ] Scenarios are connected to automated tests where the behavior is important, stable, and repeatedly verified.
- [ ] Removing any scenario would leave a gap in understanding an important behavior.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
