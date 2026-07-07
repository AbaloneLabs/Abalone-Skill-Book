---
name: interactive_prototyping.md
description: Use when the agent is building or wiring an interactive prototype, choosing prototyping tools, managing prototype complexity and conditional logic, preparing a prototype for testing or handoff, translating static designs into clickable flows, or deciding how much engineering effort to invest in making a prototype interactive without over-building.
---

# Interactive Prototyping

There is a difference between deciding what a prototype should test and actually building it so that it behaves correctly. Interactive prototyping is the craft of turning static designs into something clickable, responsive, and stateful enough that testing it produces trustworthy findings, without investing so much engineering effort that the prototype becomes a product in its own right. The judgment problem is choosing the right tool and technique for the question, managing the complexity that interactivity introduces, and keeping the prototype maintainable and honest as it grows. Agents tend to fail by over-building interactivity before the design is stable, by choosing tools that cannot represent the states that matter, and by allowing prototypes to accumulate hidden logic that makes them impossible to revise.

Use this skill when building or wiring an interactive prototype: selecting tools, constructing flows and transitions, managing conditional logic and state, or preparing a prototype for testing or handoff. The goal is an interactive prototype built at the right level of effort for its purpose, that behaves consistently and can be revised without starting over.

## Core Rules

### Match The Tool And Technique To The Question

Different prototyping tools have different strengths, and the right choice depends on what the prototype must demonstrate. A tool that excels at transitions may be poor at conditional logic; a tool built for high-fidelity visuals may be slow for rapid exploration.

Match tools to needs:

- flow and navigation questions: tools that wire screens quickly with minimal logic;
- interaction feel and motion questions: tools that support detailed transitions and microinteractions;
- data and state questions: tools or code that can represent real or realistic data and conditional logic;
- performance and feasibility questions: code prototypes, because only real implementation reveals true behavior.

Choosing a tool before defining the question leads to prototypes that cannot answer it. Define the question, then select the tool whose strengths match.

### Build The Minimum Interactivity That Answers The Question

Interactivity is expensive: every wired interaction adds build time, increases complexity, and makes the prototype harder to change. The discipline is building only the interactivity that serves the question, and stubbing the rest.

Build minimally by:

- wiring only the flows the test or demonstration requires;
- representing only the states that carry risk or insight;
- stubbing interactions that add realism but not evidence;
- resisting the urge to make every button work, which turns the prototype into a product.

A prototype that does fewer things, but does them correctly, is more useful than one that attempts everything and breaks. The goal is fidelity to the question, not to the whole product.

### Manage Conditional Logic And State Deliberately

Conditional logic is where interactive prototypes gain power and lose maintainability. A little logic, branching based on a key choice, adds enormous value; a lot of logic, simulating every edge case, turns the prototype into an unmaintainable application.

Manage logic by:

- adding branching only where the flow genuinely depends on a user choice the test must cover;
- keeping state explicit and documented, so the team knows what is tracked;
- avoiding deeply nested conditions that are hard to trace and revise;
- using simple, visible variables rather than hidden, complex state machines.

When logic grows beyond what one person can hold in mind, the prototype has become too complex. Simplify, or accept that the question now needs a code prototype.

### Represent The States That Carry Risk

The happy path is easy to wire and easy to test, but it is rarely where the real problems live. Interactive prototypes must represent the states users actually encounter, or testing produces falsely clean results.

Represent at minimum:

- loading and waiting states, so delays are experienced realistically;
- empty states, where missing data often confuses users;
- error and validation states, where recovery is tested;
- success and completion states, so the flow reaches a believable end;
- disabled and unavailable states, where actions cannot proceed.

A prototype wired only for success hides the states that ship with the most risk. Wire the states that real users will hit, especially the uncomfortable ones.

### Keep Transitions Honest And Consistent

Transitions communicate state change, and inconsistent or misleading transitions distort what testing reveals. A transition that implies instant performance the product cannot deliver sets expectations the build will break.

Keep transitions honest by:

- using motion to clarify what changed and where attention should go;
- avoiding transitions that imply performance or behavior the product cannot match;
- keeping transition timing realistic, so testing reflects the real experience;
- applying a consistent set of transitions, not a different animation per screen.

Transitions are communication, not decoration. Wire them to reflect the intended product behavior.

### Document The Prototype's Boundaries

A convincing interactive prototype can mislead the team as easily as it misleads users. The people who test, review, and build from it must know where it stops being real.

Document boundaries by:

- listing which flows are fully wired and which are dead ends;
- noting which states are represented and which are missing;
- marking which data is real and which is hardcoded;
- indicating which interactions are simulated and which are absent.

Hidden boundaries produce false confidence. Make the limits visible so that decisions account for the gaps.

### Plan For Revision And Disposal

Interactive prototypes are built to be changed and eventually discarded, not maintained forever. Building them as if they were products makes them slow to revise and hard to let go.

Build for change by:

- keeping the structure simple enough to revise without rebuilding;
- avoiding dependencies that lock the prototype into one design direction;
- accepting that the prototype will be thrown away, and not over-investing in it;
- versioning or duplicating before major changes, so earlier states can be recovered.

A prototype that cannot be revised cheaply stops being a prototype and becomes a burden. Build it to serve the question, then move on.

### Prepare The Prototype For Its Audience

A prototype built for a usability test, one built for a stakeholder demo, and one built for engineering handoff have different needs. Prepare the prototype for how it will actually be used.

Prepare by audience:

- for usability testing: realistic entry points, honest navigation, no guiding cues;
- for stakeholder demos: a guided path through the key flows, with context for what is unfinished;
- for engineering handoff: clear documentation of interactions, states, and edge cases;
- for all audiences: visible labels on what is real and what is stubbed.

A prototype shown to the wrong audience in the wrong state produces misleading reactions. Prepare it for its purpose.

## Common Traps

### Choosing A Tool Before Defining The Question

Tools have different strengths. Define the question first, then match the tool.

### Over-Building Interactivity

Every wired interaction adds cost and complexity. Build only what the question requires.

### Logic Sprawl

Deeply nested conditions turn the prototype into an unmaintainable application. Keep logic simple and visible.

### Wiring Only The Happy Path

Success-only prototypes hide the loading, empty, and error states where real problems live.

### Misleading Transitions

Transitions that imply performance or behavior the product cannot deliver distort test results.

### Hidden Boundaries

A prototype whose missing states and stubbed data are invisible misleads the team. Document the limits.

### Treating The Prototype As A Product

Over-investing in a prototype makes it slow to revise and hard to discard. Build for change and disposal.

### Wrong Preparation For The Audience

A prototype shown unprepared for its audience, test, demo, or handoff produces misleading reactions.

## Self-Check

- [ ] The tool and technique were chosen to match the specific question the prototype must answer.
- [ ] Only the interactivity that serves the question was built; the rest is stubbed.
- [ ] Conditional logic is minimal, visible, and added only where the flow genuinely branches.
- [ ] Loading, empty, error, success, and disabled states are represented, not only the happy path.
- [ ] Transitions are honest, consistent, and do not imply behavior the product cannot deliver.
- [ ] The prototype's boundaries, wired flows, missing states, and stubbed data are documented.
- [ ] The prototype is built for revision and disposal, not over-invested as a product.
- [ ] The prototype is prepared for its audience: test, demo, or handoff.
- [ ] Real and stubbed parts are labeled so the team is not misled.
- [ ] The level of engineering effort matches the stage, not exceeding what the question justifies.
