---
name: prototype_fidelity_selection.md
description: Use when the agent is choosing the fidelity of a prototype, deciding between sketch, wireframe, low-fidelity, high-fidelity, or code prototypes, planning what to prototype for a given question or stakeholder, or balancing the speed of rough prototypes against the realism needed to get trustworthy feedback without over-investing in polish too early.
---

# Prototype Fidelity Selection

The fidelity of a prototype is a decision about what question it must answer and what feedback it must produce, not a matter of how polished the work should look. A prototype that is too rough for its audience yields useless feedback, and a prototype that is too polished wastes time and constrains the design too early by making unfinished decisions look final. The judgment problem is matching fidelity to the stage of the work, the kind of feedback needed, and the audience who will react to it. Agents tend to fail by defaulting to high fidelity because it looks impressive, by showing polished prototypes when they still need open critique, and by misunderstanding that roughness is a feature, not a defect, when the goal is to invite challenge rather than approval.

Use this skill when deciding how to prototype: choosing between sketches, wireframes, low-fidelity or high-fidelity mockups, or code prototypes, and planning what each prototype is meant to test. The goal is a fidelity that produces the right kind of feedback at the right cost.

## Core Rules

### Match Fidelity To The Question Being Asked

Different questions need different levels of fidelity, and the wrong level produces feedback that misses the point.

Match fidelity to the question:

- "is this concept direction right" needs rough sketches, because roughness invites critique of the idea;
- "does this flow make sense" needs wireframes or low-fidelity interactive prototypes, enough to test structure without distraction;
- "does this visual and interaction feel right" needs high-fidelity mockups, because look and feel cannot be judged at low fidelity;
- "does this perform and behave correctly" needs a code prototype, because rendering, latency, and real data matter.

State the question first, then choose the fidelity. Choosing fidelity before the question is defined leads to prototypes that are either too rough to answer it or too polished to be worth the cost.

### Use Low Fidelity To Keep Options Open

Low-fidelity prototypes are valuable precisely because they look unfinished. A sketch says "this is a work in progress, challenge it," while a polished mockup says "this is decided, approve it."

Low fidelity helps by:

- signaling that the design is open to change, so reviewers critique rather than rubber-stamp;
- letting the team explore many alternatives quickly, since each is cheap to produce;
- avoiding premature commitment to visual and interaction details that have not been decided;
- separating structural questions from visual ones, so feedback targets the right layer.

When you still need divergent ideas and honest critique, stay rough. Moving to high fidelity too early freezes decisions that should still be open and suppresses the feedback that would improve them.

### Use High Fidelity Only When The Question Demands Realism

High-fidelity prototypes are expensive to build and expensive to change, and they should be reserved for questions that low fidelity cannot answer.

Move to high fidelity when:

- the question is about visual design, branding, or emotional response, which cannot be judged in wireframe;
- the question is about specific interaction feel, motion, or micro-interactions;
- stakeholders need to experience the design as users will, to make a go or no-go decision;
- the design is near final and the team is validating rather than exploring.

Do not jump to high fidelity to look professional; do it because the question requires realism. Every hour spent polishing a prototype that could have been a wireframe is an hour not spent exploring alternatives.

### Consider The Audience And How They React To Fidelity

Different audiences interpret fidelity differently, and the same prototype can produce opposite reactions depending on who sees it.

Calibrate fidelity to the audience:

- designers and product partners can read low fidelity and focus on structure, but may need higher fidelity to judge visual direction;
- executives and stakeholders often cannot read wireframes and may reject sound ideas that look unfinished, so they may need higher fidelity to engage;
- engineers need enough fidelity to assess feasibility, data, and edge cases;
- users in testing react to what they see, so fidelity must be high enough that they behave realistically without being confused by placeholders.

Choose fidelity that lets each audience give useful feedback rather than getting stuck on the medium. Showing a wireframe to an audience that cannot read it wastes the session.

### Decide What Is Real And What Is Stubbed

A prototype is always partial, and the team must know which parts are real and which are faked, or they will draw false conclusions.

Make the boundaries explicit:

- which interactions are wired up and which are static;
- which data is real and which is hardcoded;
- which states exist and which are missing;
- which performance is representative and which is not.

A prototype that fakes a fast response can hide a real performance problem; a prototype that stubs an error state can hide a real failure path. Label what is real so that feedback and decisions account for the gaps.

### Match Fidelity To The Cost Of Change

Fidelity determines how expensive the prototype is to revise, and that cost shapes how much iteration happens.

Consider the revision cost:

- sketches and wireframes are cheap to change, so they support rapid iteration and divergence;
- high-fidelity mockups are expensive to change, so they encourage convergence and discourage exploration;
- code prototypes are the most expensive to change, so they should come late, when the design is stable.

Choose the lowest fidelity that answers the question, so that iteration stays cheap as long as the design is still moving. Raising fidelity too early makes every revision painful and biases the team toward the current design rather than a better one.

## Common Traps

### Defaulting To High Fidelity Because It Looks Impressive

Polished prototypes look professional but invite approval instead of critique, and they waste effort when the design is still open. Stay rough when you need challenge.

### Showing Polished Work When You Need Critique

High fidelity signals the design is decided. If you still want honest feedback, lower the fidelity so reviewers feel free to question.

### Too Rough For The Audience

An audience that cannot read wireframes will reject sound ideas. Match fidelity to what the audience can interpret.

### Forgetting To Label What Is Stubbed

A prototype with faked data or hidden states leads to false conclusions. Make the real and stubbed parts explicit.

### Raising Fidelity Before The Structure Is Set

Polishing a flow whose structure is still wrong locks in the wrong foundation. Resolve structure at low fidelity first.

### Prototyping Everything At One Fidelity

Different parts of a design are at different stages. Let fidelity vary by section rather than forcing one level across the whole prototype.

### Confusing Prototype Polish With Product Readiness

A beautiful prototype is not a shipped product. Do not let visual polish substitute for resolved states, real data, and feasibility.

## Self-Check

- [ ] The fidelity was chosen to match the specific question the prototype must answer.
- [ ] Low fidelity is used while options are still open and critique is still wanted.
- [ ] High fidelity is reserved for questions about visual, interaction feel, or near-final validation.
- [ ] Fidelity is calibrated to the audience, so each group can give useful feedback.
- [ ] The real and stubbed parts of the prototype are labeled, including data, states, and performance.
- [ ] The lowest fidelity that answers the question was chosen, to keep the cost of change low.
- [ ] Fidelity varies by section where parts of the design are at different stages.
- [ ] No high-fidelity effort was spent on a flow whose structure was still unresolved.
- [ ] The prototype's polish is not mistaken for product readiness.
- [ ] The fidelity invites the kind of feedback the current stage needs, critique or approval.
