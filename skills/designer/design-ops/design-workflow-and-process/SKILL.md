---
name: design_workflow_and_process.md
description: Use when the agent is planning, sequencing, or reviewing a design workflow or design process, including handoff to engineering, design reviews, critique structure, iteration cadence, definition of done, staging and approval gates, version control of design files, and coordination across product, research, and engineering.
---

# Design Workflow And Process

A design workflow is not a flowchart on a wiki page. It is the set of real decisions about who makes what, when work is finished, when it is reviewed, how it moves between design and engineering, and what counts as ready to ship. Teams fail at process not because they lack a methodology, but because the process is implicit, inconsistent, or borrowed from a template that does not match the product's risk profile.

Use this skill when establishing, changing, or auditing how design work flows through a team. The goal is to prevent the agent from proposing a generic "discover, design, develop" pipeline that looks complete but hides the decisions that actually determine whether work ships well: where review happens, what handoff contains, how iteration is bounded, and who owns ambiguous states.

## Core Rules

### Match The Workflow To The Risk And Uncertainty Of The Work

Not all work deserves the same process. A risky new feature with unclear user needs requires research, exploration, and multiple review loops. A small visual fix to an existing component may need only a quick review against the design system. Forcing uniform process creates two failures: heavy process on trivial work wastes time, and light process on risky work ships defects.

Before defining a workflow, classify the work:

- **Novel or high-risk work** (new flows, unfamiliar user problems, regulated surfaces): needs research, problem framing, exploration of multiple directions, and explicit review gates.
- **Evolutionary work** (improvements to existing patterns): needs direction-setting, review against existing conventions, and validation of the change.
- **Maintenance work** (fixes, tokens, copy edits): needs a fast path with a lightweight check against the system.

A workflow that does not distinguish these will either bottleneck the team or let risky work slip through.

### Define Explicit Handoff, Not Assumed Handoff

The most common process failure is a handoff that exists in name but not in content. Designers hand over a file; engineers build what they interpret; the result drifts. A real handoff specifies what is being transferred and what the receiver is expected to do with it.

A useful handoff includes:

- the problem being solved and the user or business outcome;
- the states to cover, including empty, loading, error, and edge cases;
- interaction behavior that a static frame cannot show, such as focus order, keyboard behavior, and motion;
- the components and tokens being used versus new ones being introduced;
- accessibility and responsive requirements;
- open questions and known constraints.

Decide whether handoff is a document, a prototype, a linked design system entry, or a conversation backed by artifacts. The medium matters less than the explicitness.

### Separate Review For Direction, Craft, And Consistency

Design review is often treated as one event, but it actually serves three different purposes, and mixing them produces poor feedback. Direction review asks whether the work solves the right problem. Craft review asks whether the execution is high quality. Consistency review asks whether the work fits the system.

Run these at different times:

- **Direction review** early, on rough work, before polish is invested.
- **Craft review** once the direction is settled, focused on visual and interaction quality.
- **Consistency review** late, checking tokens, components, spacing, and patterns against the system.

Reviewing craft on work whose direction is still unsettled wastes effort. Reviewing direction on polished work makes critique feel personal. Separating the gates keeps feedback useful.

### Bound Iteration Explicitly

Unbounded iteration is a process failure disguised as quality. Without explicit limits, work expands to fill available time, and "one more round" becomes the default. Decide in advance how many rounds of exploration and how many rounds of polish a given risk class receives, and make the decision visible.

Iteration boundaries should define:

- when to stop exploring and commit to a direction;
- when polish investment must stop and work must ship;
- who has authority to extend or close a round;
- what triggers a return to an earlier phase, such as a discovered constraint.

A team that cannot say when work is done cannot ship predictably.

### Make The Definition Of Done Explicit

"Done" is one of the most overloaded words in a design workflow. Done for the designer, done for engineering, and done for the product are different states. Make each explicit.

Consider defining:

- **Design done**: the artifact covers required states, is reviewed, and meets the system's craft bar.
- **Dev-ready**: handoff content is complete and questions are resolved.
- **Build done**: engineering implementation matches the design intent and passes review.
- **Shipped**: the work is live and measurable or observable.

Without these distinctions, work stalls in ambiguous states where everyone believes someone else owns the next step.

### Version And Reference Design Artifacts Like Code

Design files that live only in a local tool, with names like "Final v3 FINAL", create a process that cannot scale. Treat design artifacts as versioned, referenceable sources of truth.

Practices that matter:

- link artifacts to the feature, ticket, or spec they serve;
- keep a single current source of truth and mark older versions as archived;
- name versions meaningfully rather than emotionally;
- record what changed between versions and why, at least at a summary level;
- ensure engineering builds against the agreed version, not an unreviewed iteration.

When artifacts are untraceable, review and rollback both become guesswork.

### Coordinate With Research And Engineering Early

Process failures often come from late dependencies. Research findings arrive after design is locked. Engineering constraints surface after handoff. Localization, legal, and accessibility reviews happen at the end when changes are expensive.

Build the workflow so that:

- research informs problem framing before direction is set;
- engineering feasibility is checked during exploration, not after;
- cross-functional reviewers (accessibility, legal, localization, data) have a defined entry point;
- dependencies are listed when the work is scoped.

## Common Traps

### Adopting A Named Methodology Without Adapting It

Borrowing a framework like double diamond, shape up, or design sprint wholesale, without mapping it to the team's risk profile and cadence, produces a process that looks rigorous but does not match how work actually moves.

### Treating Handoff As A Ceremony

A handoff meeting with no documented artifact, no edge cases, and no interaction notes is a ceremony, not a transfer of intent. The build will reflect what engineering inferred, not what design intended.

### Reviewing Everything At The Same Depth

If every artifact gets the same review, trivial work is over-reviewed and risky work is under-reviewed. Calibrate review depth to risk.

### Letting Polish Happen Before Direction Is Set

Investing in visual polish before the direction is agreed leads to sunk-cost attachment. Reviewers hesitate to challenge direction because the work looks finished.

### Confusing Activity With Progress

A workflow full of meetings, ceremonies, and artifacts can look healthy while shipping little. Measure the workflow by outcomes shipped and defects caught, not by artifacts produced.

### Ignoring The Cost Of Context Switching

A process that fragments design work across many small tasks can appear efficient but destroys the focus needed for hard problems. Reserve uninterrupted blocks for novel work.

## Self-Check

- [ ] The workflow distinguishes novel, evolutionary, and maintenance work, with process weight matched to risk.
- [ ] Handoff is defined by explicit content, not by a meeting, and includes states, interactions, accessibility, and open questions.
- [ ] Direction, craft, and consistency reviews are separated and run at the appropriate phase.
- [ ] Iteration is bounded with explicit limits on exploration and polish rounds.
- [ ] A definition of done exists for design, dev-ready, build, and shipped states.
- [ ] Design artifacts are versioned, named meaningfully, and linked to the work they serve.
- [ ] Research, engineering feasibility, and cross-functional reviewers have defined entry points before handoff.
- [ ] The workflow is measured by outcomes and defects caught, not only by artifacts and meetings produced.
