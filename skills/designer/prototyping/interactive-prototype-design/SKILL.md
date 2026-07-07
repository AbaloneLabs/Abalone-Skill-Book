---
name: interactive_prototype_design.md
description: Use when the agent is designing an interactive or clickable prototype, wiring up flows and transitions, deciding what interactions to simulate, building prototype states and conditional logic, planning navigation and state for a prototype, or ensuring the interactive prototype tests the right behavior without misleading users or hiding states that real users will encounter.
---

# Interactive Prototype Design

An interactive prototype is a promise about how the product will behave, and every interaction it simulates sets an expectation that the real product must meet or break. The work is not making the prototype feel smooth; it is deciding which behaviors to simulate, which states to represent, and how to wire the flow so that testing the prototype tests the right thing. A prototype that hides its missing states produces falsely positive results, and a prototype whose interactions diverge from what the product can actually deliver sets expectations the build will fail to meet. The judgment problem is choosing what to make interactive, representing the states that matter, and keeping the prototype honest about where it ends. Agents tend to fail by wiring only the happy path, by omitting the error and empty states that real users hit, and by over-investing in motion and polish at the expense of the flows that carry the real risk.

Use this skill when designing or building an interactive or clickable prototype: wiring flows, transitions, and conditional logic, planning states and navigation, or deciding what a prototype must simulate to support valid testing. The goal is an interactive prototype whose behavior represents the product closely enough that findings transfer, and whose limits are visible.

## Core Rules

### Define Which Flows The Prototype Must Support

An interactive prototype cannot represent the whole product, and trying to makes it brittle and slow to change. Decide which flows the prototype must support, based on what it is meant to test or demonstrate.

Scope the prototype to:

- the core flows that carry the validation questions;
- the entry points users will realistically start from;
- the branches that matter for the decision, not every possible path;
- the depth needed to reach a realistic endpoint, not the entire product.

A prototype scoped to the right flows is testable and changeable. A prototype that tries to wire everything becomes a maintenance burden and still misses the states that matter.

### Wire The States Users Will Actually Encounter

The happy path is the easy part. Real users hit loading, empty, error, partial, and edge states, and a prototype that omits them produces falsely clean results.

Represent the states that matter:

- loading and waiting states, so users experience realistic delays;
- empty states, where there is no data yet, which often confuse users;
- error and validation states, where input fails or a request breaks;
- partial and success states, so the flow reaches a believable endpoint;
- disabled and read-only states, where actions are unavailable.

A prototype that only shows the happy path hides the problems that occur when things go wrong, and those are usually the problems that ship. Include the states that the real product will produce, especially the ones that carry risk.

### Make Transitions Match The Intended Product Behavior

Transitions are not decoration; they communicate state change and causality. A transition that feels snappy in the prototype but cannot be delivered in the product sets an expectation the build will break.

Align transitions with intent:

- use motion to clarify what changed and where the user's attention should go;
- avoid transitions that imply performance the product cannot deliver;
- keep transition timing realistic, so testing reflects the real experience;
- ensure transitions do not mask loading or errors that users need to see.

If the prototype animates instantly where the product will lag, users will judge a responsiveness the product cannot match. Keep transitions honest about the intended behavior.

### Build Conditional Logic Only Where It Affects The Question

Interactive prototypes can include conditional logic, branching, and state, but each piece of logic adds build cost and makes the prototype harder to change. Add logic only where it serves the validation.

Add conditional logic when:

- the flow branches based on a real user choice that the test must cover;
- the state change affects whether the design works or fails;
- the logic exposes a risk the team needs to observe.

Avoid logic that adds realism without adding insight. A prototype that simulates every edge case becomes a small application, expensive to build and impossible to iterate. Build the logic that the question requires and stub the rest.

### Keep The Prototype's Limits Visible To The Team

A convincing interactive prototype can fool the team as easily as it fools users. The people who test, review, and build from it must know where it stops being real.

Document the prototype's limits:

- which flows are fully wired and which are dead ends;
- which states are represented and which are missing;
- which data is real and which is hardcoded;
- which interactions are simulated and which are absent.

A prototype whose limits are hidden leads to decisions based on behavior the product will not have. Make the boundaries explicit so that stakeholders and engineers do not infer capabilities that were never built.

### Design Navigation And Wayfinding For Realistic Behavior

Users navigate prototypes the way they navigate products, and navigation that is too forgiving or too guided produces unrealistic findings.

Design navigation honestly:

- let users get lost and recover, because recovery is part of what you are testing;
- do not auto-advance or hint at the next step unless the product will;
- represent the back, cancel, and exit paths the real product will offer;
- avoid wizard-of-oz cues that guide users to success they would not reach alone.

A prototype that silently guides users to the right answer validates nothing. Let users find their own way, within the flows the prototype supports, so the findings reflect real behavior.

### Balance Interaction Realism Against Iteration Speed

Every increase in realism costs iteration speed, and the right balance depends on the stage of the work. Early on, speed matters more than realism; later, realism matters more.

Balance by stage:

- early: favor fast, rough interactions so the team can explore many directions;
- middle: add enough interaction to test structure and comprehension;
- late: raise realism to validate behavior, feel, and edge states.

Do not over-build interaction before the structure is stable, because the work is thrown away when the design changes. Raise interaction realism as the design converges, so effort is spent on a design that will survive.

## Common Traps

### Wiring Only The Happy Path

A prototype that shows only success hides the loading, empty, and error states where real problems live. Wire the states users will actually hit.

### Over-Investing In Motion And Polish

Polished transitions look impressive but set expectations the build cannot meet and consume time that should go to risky flows. Keep transitions honest and minimal.

### Hiding The Prototype's Limits

A convincing prototype whose missing states and stubbed data are invisible misleads the team. Document where the prototype stops being real.

### Guiding Users To Success

Auto-advance, hints, and forgiving navigation validate nothing. Let users find their own way within the supported flows.

### Building Logic That Adds Realism But Not Insight

Simulating every edge case turns the prototype into a small application. Add logic only where it serves the question.

### Forgetting Entry Points And Context

Users who start mid-flow behave differently than users who arrive naturally. Represent realistic entry points and context.

### Raising Realism Before The Structure Is Set

High interaction realism on an unstable design wastes effort that gets thrown away. Raise realism as the design converges.

## Self-Check

- [ ] The prototype is scoped to the flows that carry the validation questions, not the whole product.
- [ ] Loading, empty, error, partial, success, and disabled states are represented, not only the happy path.
- [ ] Transitions clarify state change and match the performance and behavior the product can deliver.
- [ ] Conditional logic is added only where it affects the question, not for realism alone.
- [ ] The prototype's limits, dead ends, missing states, and stubbed data are documented for the team.
- [ ] Navigation lets users get lost and recover without auto-guiding them to success.
- [ ] Entry points and context are realistic for the flows being tested.
- [ ] Interaction realism is balanced against iteration speed and matched to the stage of the work.
- [ ] No transition implies behavior or performance the real product cannot deliver.
- [ ] The prototype tests the right behavior without hiding states that real users will encounter.
