---
name: sequencing_for_risk_and_learning.md
description: Use when the agent is sequencing work to reduce risk and uncertainty first, ordering initiatives by what tests the riskiest assumptions, deciding whether to build or investigate before committing, or using sequencing to maximize learning per unit of capacity.
---

# Sequencing For Risk And Learning

Beyond value and dependency, sequencing has a third dimension: the order in which the team resolves uncertainty. Every plan rests on assumptions, and the riskiest assumptions, if wrong, invalidate the most work. Sequencing for risk means front-loading the work that tests those assumptions, so that the team learns whether the direction is sound before committing to the dependent work. Done well, risk-based sequencing means the team never builds extensively on an untested assumption, and the plan adapts as uncertainty resolves. Done poorly, the team executes the plan in value order, discovers late that a core assumption was wrong, and has to rework or abandon large amounts of completed work. Agents often ignore risk in sequencing because value and dependency are more visible, leaving uncertainty to surface as expensive surprises.

The harm this skill prevents is building on sand. A plan that does not sequence for risk may invest months in work predicated on an assumption that a small, early experiment could have disproven in a week. The later the assumption is tested, the more work depends on it, and the more expensive its failure. Risk-based sequencing is the discipline of resolving uncertainty before it compounds.

Use this skill before answering questions such as "what should we do first to reduce risk", "should we build or investigate", "how do we sequence when we are uncertain", or "what is our riskiest assumption". The goal is to prevent the agent from sequencing purely on value and dependency while leaving the team exposed to untested assumptions.

## Core Rules

### Identify The Riskiest Assumptions Before Sequencing

Every initiative rests on assumptions about user behavior, technical feasibility, market response, integration compatibility, and business impact. Before sequencing, make these assumptions explicit and rank them by risk, where risk is the combination of how uncertain the assumption is and how much work depends on it being true. The riskiest assumptions are those that are both highly uncertain and load-bearing for the plan. These are the assumptions whose failure would invalidate the most work.

The act of naming assumptions is itself valuable, because teams often proceed on implicit assumptions that no one has examined. Forcing the assumptions into the open, with their uncertainty and dependency noted, reveals where the plan is most fragile and where early testing would have the highest payoff.

### Sequence Work That Tests Risky Assumptions Early

Once the riskiest assumptions are identified, sequence the work that tests them as early as possible, even if that work is not the highest-value deliverable. A small experiment, prototype, technical spike, or limited release that resolves a risky assumption early prevents the team from building extensively on a false foundation. The cost of the test is small; the cost of discovering the assumption was wrong after extensive dependent work is large. Risk-based sequencing prioritizes the resolution of uncertainty over the delivery of value in the early part of the plan.

This often means that the first work on an initiative is not building the feature but testing the assumption behind it. A prototype that validates demand, a spike that validates feasibility, a limited release that validates behavior: all are forms of assumption-testing work that should precede full build when the assumption is risky.

### Distinguish Build Work From Investigation Work

A common sequencing error is treating investigation and build as the same kind of work, when they serve entirely different purposes. Investigation work, experiments, spikes, prototypes, research, reduces uncertainty; it does not directly deliver user value. Build work delivers value but rests on assumptions. The two should be sequenced differently: investigation precedes build when assumptions are risky, and the decision to build should be gated on the investigation's results. Conflating them leads either to building before uncertainty is resolved or to investigating indefinitely without ever building.

For each initiative, decide whether the current step is investigation or build. If critical assumptions remain untested, the step is investigation, and its output is a decision about whether and how to build. If assumptions are sufficiently resolved, the step is build, and its output is delivered value. Sequencing should make this distinction explicit at each step.

### Gate Full Build On The Resolution Of Risky Assumptions

The decision to commit to full build should be gated on the resolution of the riskiest assumptions. This does not mean every assumption must be certain before any building, which would paralyze the team. It means that the assumptions whose failure would invalidate the most work should be tested before the work that depends on them is committed. The gate is not a bureaucratic checkpoint but a decision: based on what the investigation revealed, do we proceed with full build, adjust the approach, or abandon the initiative?

Without this gate, teams build by default once an initiative is planned, regardless of what investigation reveals. The gate forces the investigation's results to actually inform the build decision, which is the entire point of sequencing for risk. An investigation whose results are ignored might as well not have been done.

### Sequence To Maximize Learning Per Unit Of Capacity

When multiple initiatives carry risk, sequence them so that the team's capacity resolves the most uncertainty per unit of effort. This may mean running several small investigations in parallel before committing to any single build, because the investigations collectively resolve more uncertainty than committing early to one direction. It may mean choosing the initiative whose assumption test is cheapest and most informative over one whose test is expensive and inconclusive.

The metric is learning per capacity, not value per capacity, in the early part of the plan. As uncertainty resolves and the team gains confidence in a direction, the metric shifts toward value delivery. The transition from learning-focused to value-focused sequencing happens as assumptions are validated, and the plan should reflect that shift.

### Re-Sequence As Uncertainty Resolves

Risk-based sequencing is inherently adaptive, because resolving an assumption changes the risk profile of the remaining work. An assumption that was risky and is now validated opens the door to confident building; an assumption that was tested and disproven may invalidate downstream work and require re-sequencing or abandonment. Build in re-sequencing triggered by the results of assumption tests, so that the plan evolves as uncertainty resolves rather than executing a fixed sequence regardless of what was learned.

The cadence of re-sequencing is driven by the cadence of learning, not by the calendar. Each significant assumption test is a natural re-sequencing point, because its results may change the optimal order of remaining work.

## Common Traps

### Sequencing On Value And Dependency Alone

Ignoring risk because value and dependency are more visible. The trap is extensive building on untested assumptions that fail expensively late.

### Unexamined Implicit Assumptions

Proceeding on assumptions no one has named. The trap is fragility that no one recognizes until an assumption fails.

### Building Before Investigating Risky Assumptions

Committing to full build while critical assumptions remain untested. The trap is rework or abandonment when the assumption proves wrong.

### Investigation Whose Results Are Ignored

Running experiments but building by default regardless. The trap is the appearance of risk management without its substance.

### Investigating Indefinitely Without Building

Using uncertainty as a reason never to commit. The trap is learning that never converts to delivered value.

### Fixed Sequence Under Resolving Uncertainty

Refusing to re-sequence when assumption tests change the risk profile. The trap is a plan that ignores its own learning.

## Self-Check

- [ ] The riskiest assumptions behind each initiative are identified, with uncertainty and dependency noted.
- [ ] Work that tests risky assumptions is sequenced early, ahead of higher-value but assumption-dependent work.
- [ ] Investigation and build work are distinguished, with investigation preceding build when assumptions are risky.
- [ ] The decision to commit to full build is gated on the resolution of the riskiest assumptions.
- [ ] Initiatives are sequenced to maximize learning per unit of capacity in the early part of the plan.
- [ ] Re-sequencing is triggered by the results of assumption tests, so the plan adapts as uncertainty resolves.
- [ ] No extensive build rests on an assumption that a small early test could have invalidated.
