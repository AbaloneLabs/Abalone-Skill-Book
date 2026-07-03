---
name: design_review_and_feedback.md
description: Use when the agent is reviewing a design, giving feedback to a designer, running a design critique, evaluating whether a design solves the user problem, or balancing usability, feasibility, and business constraints in design review.
---

# Design Review And Feedback

Design review is where product judgment meets design craft, and it is easy to get wrong. The most common failure is reviewing a design as if it were a piece of art to be liked or disliked, rather than a solution to a problem to be evaluated. Vague feedback, taste-driven reactions, and happy-path-only review produce designs that look polished in a meeting but break the moment a real user with a real edge case touches them.

A good design review asks whether the design solves the right problem for the right user, whether it is usable and accessible, whether it is feasible to build, and whether it holds together with the rest of the product. It does not ask whether the reviewer personally finds it attractive. The discipline is to separate effectiveness from preference, and to give feedback specific enough that the designer can act on it without guessing what you meant.

Use this skill before reviewing a designer's work, running a critique, writing design feedback, or deciding whether a design is ready to move to engineering. Ask: does this design solve the stated problem and meet the success criteria? Have I reviewed the full range of states, or only the happy path? Is my feedback specific and goal-oriented, or vague and taste-driven? Am I pushing back where I should, and trusting the designer where I should? Is there a record of why design decisions were made?

## Core Rules

### Review Against The Problem And Success Criteria

Every design review should start by re-establishing what the design is supposed to achieve. Without the problem statement, target user, and success criteria in view, feedback drifts into subjective reaction. A design that looks beautiful but does not help the user accomplish the task has failed, even if everyone in the room admires it.

Before reacting, confirm the goals. What user problem is this solving? Who is the target user? What does success look like? What constraints were given? Then evaluate the design against those anchors. If the goals themselves are unclear, that is the first thing to fix, because no amount of design polish can compensate for an undefined target.

### Use A Hierarchy Of Feedback Questions

Effective review follows a priority order, because lower-level concerns are irrelevant if higher-level ones are not met. There is no point debating button color if the flow does not solve the problem.

Work through the hierarchy in order:

- does it solve the user problem and meet the success criteria;
- is it usable and understandable for the target user;
- does it handle the full range of states and edge cases;
- is it feasible to build within constraints;
- is it consistent with the product system and on-brand.

Resist jumping to the bottom of the hierarchy. Many reviews waste time on visual details while the fundamental approach is flawed. If the design does not solve the problem, say so first and clearly, before anything else.

### Be Specific Over Vague

Vague feedback is the single biggest waste in design review. "Make it cleaner", "it feels off", "can you make it pop", "I want it more modern" are all unactionable. The designer cannot tell what you actually want changed, so they guess, iterate blindly, and return with something that may or may not address the real concern.

Translate vague reactions into specific observations. Instead of "it feels cluttered", say "the three competing calls to action in the header make the primary action unclear". Instead of "make it pop", say "the submit button does not stand out enough from the secondary actions". Specific feedback names what you observe, why it is a problem relative to a goal, and lets the designer choose the fix. If you cannot articulate why something feels wrong, say that explicitly and explore it together rather than issuing a directive.

### Review Edge Cases, Empty States, And Errors

Happy-path review is the most common gap. Teams review the screen a successful user sees and approve it, then ship a product that falls apart the moment data is missing, a load fails, a permission is denied, or a field is invalid. These states are where users lose trust.

Insist on reviewing:

- empty states with no data;
- loading and processing states;
- error states and recovery paths;
- validation failures and inline errors;
- permission denial and restricted access;
- long content and overflow;
- offline or degraded connectivity;
- first-time use and onboarding;
- destructive actions and confirmation.

A design that only covers the happy path is not complete. If these states are not designed, the gaps will be filled ad hoc during implementation, usually poorly.

### Include Accessibility In Every Review

Accessibility is not a separate review at the end; it is part of whether the design is usable. Review color contrast, focus order, text alternatives, target sizes, keyboard navigation, and reliance on color alone to convey meaning. A design that excludes users who navigate by keyboard, screen reader, or with low vision is not finished. Treat accessibility gaps as defects, not enhancements.

### Know When To Push Back And When To Trust

Reviewing well means calibrating how hard to push. Push back firmly when the design does not solve the problem, misses critical states, violates a hard constraint, or creates usability or accessibility barriers. These are effectiveness issues and the PM is accountable for them.

Trust the designer on craft decisions within the solution: visual hierarchy, spacing, interaction details, and the felt quality of the experience. If you find yourself micromanaging craft, step back and ask whether you are actually reacting to an effectiveness problem you have not articulated. Often a vague discomfort with craft is really an unspoken concern about whether the design works.

### Document Decisions And Rationale

Design review produces decisions, and decisions without rationale get relitigated forever. When the team chooses a direction, rejects an alternative, or accepts a tradeoff, record why. Note the problem being solved, the options considered, the tradeoff accepted, and the reasoning. This helps future reviewers understand the intent, prevents circular debates, and gives engineering context for implementation choices.

A lightweight decision log is often enough. The cost of recording is small; the cost of re-deciding the same question in three months is large.

### Avoid Design-By-Committee

Feedback from many stakeholders can enrich a review, but consolidating every opinion into the design produces a bland, over-compromised result. Design-by-committee happens when no one has the authority to make a call, so every suggestion gets incorporated to avoid conflict.

The PM and designer should own synthesizing feedback. Acknowledge all input, then decide which feedback serves the goals and which does not. It is acceptable, and often correct, to reject feedback that would move the design away from solving the problem. Death by feedback is what happens when the team optimizes for stakeholder satisfaction instead of user effectiveness.

## Common Traps

### Reviewing Aesthetics Instead Of Effectiveness

Reacting to whether a design is attractive is easy and feels productive, but it evaluates the wrong thing. A design can be beautiful and ineffective, or plain and highly effective. The trap is that aesthetic feedback feels like real review when it is actually avoidance of the harder effectiveness questions.

### Giving Feedback That Cannot Be Acted On

Vague feedback forces the designer to guess, which means the next iteration is a lottery. The trap is believing you have given feedback when you have only expressed a mood. If the designer cannot act on it, it was not feedback.

### Only Reviewing The Happy Path

Approving the success screen and ignoring everything else feels efficient and positive, but it ships a fragile product. The trap is that edge cases feel like nitpicking in review, when they are actually where most user frustration and support load originate.

### Overriding The Designer On Craft

When a PM dictates visual and interaction details, the designer stops owning the craft and the design degrades. The trap is mistaking strong opinions about craft for product leadership. Product leadership is about the problem and the constraints, not the pixels.

### Collecting Opinions Instead Of Making Decisions

Gathering feedback from everyone and trying to satisfy all of it produces compromise design that satisfies no one. The trap is confusing inclusiveness with quality. Inclusive input is good; inclusive decision-making without ownership is not.

### Skipping The Feasibility Check

Reviewing and approving a design that engineering cannot build, or can only build by blowing the timeline, sets up a painful collision later. The trap is treating feasibility as engineering's problem rather than a review criterion. A design is not ready until it is buildable.

### Relitigating Settled Decisions

Without recorded rationale, the same design debate recurs every time a new stakeholder joins. The trap is assuming memory or authority will carry the decision. Documented reasoning is what makes a decision stick.

## Self-Check

- [ ] The design was evaluated against the stated problem, target user, and success criteria before any aesthetic reaction.
- [ ] Feedback followed the hierarchy of effectiveness, usability, edge cases, feasibility, and brand, in priority order.
- [ ] Every piece of feedback is specific enough that the designer can act on it without guessing the intent.
- [ ] Empty, loading, error, validation, permission, overflow, offline, and first-use states were reviewed, not only the happy path.
- [ ] Accessibility, including contrast, focus, alternatives, target sizes, and keyboard navigation, was checked as part of the review.
- [ ] Pushback was reserved for effectiveness and constraint issues, and craft decisions were left to the designer.
- [ ] Feasibility was confirmed with engineering before the design was considered ready to build.
- [ ] Key design decisions and their rationale were documented to prevent relitigation.
- [ ] Feedback was synthesized and prioritized by owners, not blindly accumulated into the design.
- [ ] No vague, taste-driven directives like "make it pop" or "clean it up" were left in the final feedback.
