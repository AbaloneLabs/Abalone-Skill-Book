---
name: wireframing.md
description: Use when the agent is creating wireframes, choosing the right level of wireframe fidelity, deciding what to include and exclude in low-fidelity layouts, using wireframes to explore structure and flow before visual design, planning content placement and hierarchy, or using wireframes to align stakeholders on structure before investing in detail.
---

# Wireframing

Wireframing is the practice of creating low-fidelity representations of a layout to explore and agree on structure before investing in visual design. It looks like sketching boxes and lines, but it is really a discipline of abstraction: deciding what to show, what to leave undefined, and how much fidelity serves the current decision. Agents tend to over-design wireframes (adding color and type detail that defeats the purpose), under-design them (so abstract that no one can evaluate the structure), or use them as a checkbox rather than a tool for alignment. The harm is wasted effort, premature commitment to structure that was never validated, or stakeholder disagreement that surfaces too late to fix cheaply.

Use this skill before creating wireframes or deciding their fidelity. The goal is to prevent the agent from investing in the wrong level of detail, from using wireframes to answer questions they cannot answer, or from skipping the structural exploration that wireframes exist to enable.

## Core Rules

### Match Wireframe Fidelity To The Decision Being Made

Wireframes exist across a fidelity spectrum, and the right level depends on what decision the wireframe must support. A rough sketch on a whiteboard suits exploring multiple layout ideas quickly; a mid-fidelity digital wireframe suits aligning on structure and content placement; a detailed wireframe approaching mockup fidelity suits validating detailed layout before visual design. Choosing the wrong fidelity wastes effort or fails to support the decision.

Match fidelity to purpose:

- low-fidelity (sketch, rough boxes): for rapid exploration of many ideas, where speed and disposability matter;
- mid-fidelity (digital, grayscale, real proportions): for aligning on structure, content hierarchy, and flow;
- high-fidelity wireframes (detailed layout, near-final structure): for validating layout before visual design investment.

Over-fidelity early in exploration slows iteration and creates premature attachment to ideas. Under-fidelity late in the process leaves too much undefined to align stakeholders. Match the fidelity to where you are.

### Use Wireframes To Decide Structure, Not Appearance

The purpose of a wireframe is to make decisions about structure, content, and flow, not about visual appearance. When wireframes include color, imagery, and detailed typography, they invite feedback on the wrong things and distract from the structural questions. A disciplined wireframe communicates layout and hierarchy without committing to visual design.

Keep wireframes structural:

- use grayscale and simple shapes; avoid color, imagery, and detailed styling;
- represent content with realistic labels and approximate lengths, not lorem ipsum or final copy;
- show hierarchy through size, weight, and position, not through visual treatment;
- defer all visual design decisions to the mockup phase.

A wireframe that looks like a finished design invites stakeholders to critique the colors when the question is whether the structure works. Abstraction is the point; it focuses feedback on structure.

### Include Enough Realism To Evaluate The Structure

While wireframes should be abstract, they must not be so abstract that the structure cannot be evaluated. A wireframe of unlabeled boxes tells no one whether the layout works, because the content's nature and length shape whether the structure is viable. Enough realism (real labels, approximate content length, representative density) is needed to judge the structure meaningfully.

Include functional realism:

- use realistic content labels that reflect what will actually appear, not generic placeholders;
- approximate content length and density so the layout is tested against real content volume;
- show the variety of states (empty, populated, error) that the structure must accommodate;
- represent real component types (lists, forms, tables) rather than uniform boxes.

A wireframe of identical gray boxes cannot reveal whether a layout handles a dense table or a long form. Realism in content, not in visuals, is what makes the structure evaluable.

### Explore Multiple Alternatives Before Committing

The value of wireframing is in exploring options cheaply before committing. Creating a single wireframe and treating it as the answer defeats this purpose; the first structure is rarely the best, and without comparison there is no basis for choosing. Wireframing should generate alternatives that can be compared and selected among.

Generate alternatives:

- create multiple layout approaches for the same screen or flow;
- compare them against the user tasks and content requirements to select the strongest;
- use the comparison to understand tradeoffs, not just to pick a favorite;
- keep the discarded alternatives documented, as they may inform later iterations.

A single wireframe presented as the solution skips the exploration that makes wireframing valuable. The discipline is generating enough options to make an informed choice.

### Use Wireframes To Align Stakeholders Early

Wireframes are a communication tool, not just a design artifact. They let stakeholders react to structure before visual design creates emotional attachment and before engineering investment makes change expensive. Using wireframes to align stakeholders early prevents the costly disagreements that surface when structure is already built.

Align through wireframes:

- share wireframes with stakeholders to confirm structure and content placement before visual design;
- use them to surface disagreements about hierarchy, content priority, and flow while changes are cheap;
- frame feedback requests around structure, not appearance, to keep discussion productive;
- iterate based on stakeholder input before investing in higher fidelity.

Stakeholders who see structure only after it is visually designed often object to the foundation, forcing expensive rework. Wireframes let that objection surface when it costs little to address.

### Wireframe The Full Flow, Not Just Screens

Individual wireframes show screens, but users experience flows. Wireframing only isolated screens misses the transitions, the state changes, and the paths between screens that determine whether the experience works. A wireframe set should cover the flow, showing how screens connect and how the user moves through them.

Wireframe flows:

- map the screens in sequence, showing how the user progresses from entry to goal;
- wireframe the decision points and branches, not just the happy path;
- show the state changes (loading, success, error) that occur within the flow;
- connect the wireframes so the flow is legible, not just a set of unrelated screens.

A set of beautiful screen wireframes with no flow between them does not validate the experience. The flow is where usability lives, and wireframing must cover it.

### Define What The Wireframe Leaves For Later Design

A wireframe is incomplete by design, and what it leaves undefined is as important as what it defines. If the wireframe does not clearly mark what is decided versus what is open, downstream designers may assume structure is fixed when it was exploratory, or treat open questions as resolved. The wireframe should communicate its own scope.

Define the wireframe's scope:

- mark which structural decisions are firm versus exploratory;
- note what is explicitly deferred to visual design (imagery, color, detailed type);
- flag open questions and edge cases the wireframe does not resolve;
- communicate the wireframe's purpose so reviewers evaluate it appropriately.

A wireframe presented as final when it was exploratory creates false commitment. Clarity about what is decided and what is open keeps the process honest.

### Avoid Premature Investment In Detail

The temptation in wireframing is to add detail that feels like progress but actually locks in decisions prematurely. Detailed wireframes take longer to create, are harder to change, and create attachment that resists the structural feedback wireframes are meant to elicit. Premature detail defeats the speed and flexibility that make wireframing valuable.

Resist premature detail:

- stop adding fidelity once the structural decision can be made;
- avoid pixel-perfect wireframes when rough proportions suffice;
- do not invest in visual polish that will be redone in the mockup phase;
- keep wireframes disposable enough that discarding an option is not painful.

A wireframe so detailed that discarding it feels wasteful has overshot its purpose. The value is in cheap exploration, and detail that reduces disposability reduces that value.

## Common Traps

### Over-Fidelity Early In Exploration

Detailed wireframes too early slow iteration and create premature attachment; match fidelity to the decision being made.

### Wireframes That Look Like Finished Designs

Color, imagery, and detailed styling invite feedback on appearance instead of structure; keep wireframes abstract and grayscale.

### Too Abstract To Evaluate

Unlabeled boxes and uniform placeholders cannot reveal whether the structure handles real content; include realistic labels and density.

### A Single Wireframe Treated As The Answer

The first structure is rarely the best; generate and compare alternatives before committing.

### Skipping Stakeholder Alignment

Stakeholders who see structure only after visual design object to the foundation expensively; use wireframes to align early.

### Isolated Screens Without Flows

Users experience flows, not screens; wireframe the sequence, branches, and state changes, not just individual screens.

### Ambiguous Scope

Wireframes that do not mark what is firm versus open create false commitment or missed questions; define the wireframe's scope explicitly.

### Premature Investment In Detail

Pixel-perfect wireframes reduce disposability and resist the structural feedback wireframes exist to elicit; stop at the fidelity the decision needs.

## Self-Check

- [ ] Wireframe fidelity (low, mid, high) matches the decision being supported and the stage of the process.
- [ ] Wireframes are structural and abstract (grayscale, simple shapes), deferring visual design to the mockup phase.
- [ ] Enough realism (real labels, approximate length, representative density) is included to evaluate the structure meaningfully.
- [ ] Multiple layout alternatives were explored and compared before committing to one.
- [ ] Wireframes were shared with stakeholders to align on structure and content before visual design investment.
- [ ] The full flow (sequence, branches, state changes) is wireframed, not just isolated screens.
- [ ] The wireframe's scope is defined: what is firm, what is exploratory, what is deferred, and what remains open.
- [ ] Detail was added only to the level the structural decision required, keeping wireframes disposable and flexible.
