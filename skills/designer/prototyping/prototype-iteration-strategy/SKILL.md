---
name: prototype-iteration-strategy.md
description: Use when the agent is planning how to iterate on a prototype, deciding how many iterations to run, choosing what to change between rounds, managing feedback integration without design by committee, sequencing iteration against deadlines, or knowing when to stop prototyping and commit to building.
---

# Prototype Iteration Strategy

Prototyping is inherently iterative: a prototype is built, tested, revised, and tested again, with each cycle meant to reduce uncertainty and move the design toward a buildable state. Yet iteration is often managed poorly. Teams iterate reactively, chasing every piece of feedback, changing too much between rounds so that learning is confounded, or iterating endlessly without converging because they never defined what "done" looks like. The judgment problem is treating iteration as a disciplined process with a purpose, a budget, and a stopping condition, rather than as an open-ended loop of revisions.

Agents tend to treat iteration as simply "make it better next time," without a strategy for what each cycle is meant to resolve, how to integrate conflicting feedback, or when to stop. This skill helps the agent plan iteration deliberately: what to change, what to hold constant, how to avoid design by committee, and how to converge before effort is wasted on prototypes that should already be in production.

## Core Rules

### Define what each iteration is meant to resolve before running it

Each prototype iteration should have a specific learning goal: validate a revised flow, test whether a change fixed a discovered problem, explore an alternative direction, or confirm a near-final design. State the goal before the cycle begins. Iteration without a defined goal becomes undirected revision, where changes are made because feedback was received rather than because a specific question remains open. A clear goal also tells you when the iteration is complete.

### Change one major variable at a time to keep learning interpretable

When you change many things between iterations, you cannot tell which change caused any difference in feedback. If a flow, a visual treatment, and the data all change at once, and feedback improves, you do not know why. To keep learning interpretable, change one major variable at a time and hold the rest constant, so that differences in feedback can be attributed to the change. This is slower in the short term but produces trustworthy learning; changing everything at once produces confounded results.

### Distinguish convergence from divergence and use the right mode

Iteration has two modes. Divergent iteration explores alternatives, broadening the set of options under consideration; it should be fast, rough, and cheap. Convergent iteration narrows toward a final design, refining and validating the chosen direction; it should be increasingly realistic and disciplined. Many teams fail by diverging when they should be converging (endlessly exploring, never committing) or converging when they should be diverging (locking in a direction before alternatives were considered). Know which mode each iteration is in and behave accordingly.

### Integrate feedback by theme and severity, not by tally

Feedback from testing is rarely unanimous and often contradictory; one user finds a flow confusing while another finds it efficient. Integrating feedback by counting votes or implementing every request produces design by committee and a compromised result. Instead, integrate by theme and severity: group feedback into underlying issues, weigh by the severity of the problem and the strength of the evidence, and prioritize changes that resolve root causes over those that address surface complaints. Not all feedback warrants a change; some reflects individual preference or misunderstanding.

### Set a stopping condition before iterating begins

Open-ended iteration is a common form of waste. Before starting, define what "done" looks like: the prototype has resolved the critical questions, the major risks are mitigated, and further iteration yields diminishing returns. A stopping condition prevents the team from polishing indefinitely and ensures the design moves to production when it is ready, not when it is perfect. Perfection is not achievable in a prototype, and waiting for it delays the real learning that only production provides.

### Budget iteration against deadlines and scope

Iteration is not free, and it competes with build time, testing recruitment, and other work. Budget the number of iterations realistically against the deadline and the stakes of the decision. High-stakes, novel designs warrant more iterations; low-stakes, familiar patterns warrant fewer. A budget forces prioritization of what each cycle must resolve and prevents iteration from consuming the time needed to actually build the product.

### Carry learning forward across iterations, not just the design

Each iteration produces learning about users, constraints, and the problem, not only a revised prototype. Capture and carry this learning forward: what was learned, what assumptions were confirmed or overturned, what risks remain. Teams that iterate without recording learning repeat the same discoveries and lose institutional memory. Document the learning alongside the design so that the final handoff carries the reasoning, not only the artifact.

### Know when to stop prototyping and commit to building

Prototyping has diminishing returns, and some questions can only be answered in production: real performance, real data at scale, real integration, real usage over time. Recognize when the prototype has answered what it can and the remaining uncertainty requires building, not more prototyping. Continuing to prototype past this point delays the real learning and wastes effort. The courage to stop prototyping and commit is as important as the discipline to iterate well.

## Common Traps

### Reactive iteration without a goal

Changing the prototype because feedback was received, without a defined question to resolve, produces undirected revision and slow convergence.

### Changing too much between rounds

When many variables change at once, learning is confounded and you cannot attribute differences in feedback. Change one major variable at a time.

### Endless divergence without convergence

Continually exploring alternatives without ever committing leaves the design perpetually unfinished and delays the product. Know when to converge.

### Design by committee from tallying feedback

Implementing every piece of feedback or voting on changes produces compromised designs. Integrate by theme, severity, and root cause.

### No stopping condition

Iteration without a defined end polishes indefinitely and delays production. Define what "done" looks like before starting.

### Iteration without capturing learning

Teams that iterate without recording what was learned repeat discoveries and lose institutional memory. Document learning alongside the design.

### Prototyping past the point of diminishing returns

Some questions only production can answer. Continuing to prototype when the design is ready to build wastes effort and delays real learning.

### Confusing iteration count with rigor

More iterations do not automatically mean better design. Rigor comes from clear goals, controlled changes, and disciplined convergence, not from the number of cycles.

## Self-Check

- Does each iteration have a specific learning goal stated before the cycle begins?
- Are you changing one major variable at a time so that differences in feedback are interpretable?
- Is it clear whether each iteration is divergent (exploring) or convergent (refining), and are you in the right mode for the stage?
- Is feedback integrated by theme, severity, and root cause rather than by tally or vote?
- Have you defined a stopping condition before iterating, so the design moves to production when ready?
- Is the iteration count budgeted against deadlines and the stakes of the decision?
- Is learning captured and carried forward across iterations, not only the revised design?
- Have you recognized when remaining uncertainty requires building rather than more prototyping, and committed accordingly?
