---
name: prd_scope_and_success_definition.md
description: Use when the agent is defining the scope and success criteria for a PRD, deciding what outcomes constitute success, choosing measurable indicators, bounding the work to a defensible scope, or connecting PRD requirements to outcomes that can be evaluated after delivery.
---

# PRD Scope And Success Definition

Scope and success definition are the two commitments a PRD makes: scope commits to what will be built, and success definition commits to what outcome the build is meant to achieve. Together, they bound the work and define whether it has succeeded. Done well, they create a PRD that is accountable: the team knows what to build, and everyone knows how to evaluate whether the build worked. Done poorly, scope is either too vague to prevent creep or too rigid to allow learning, and success is either undefined or defined in ways that cannot be measured. Agents often focus on the requirements, the what to build, while treating scope and success as afterthoughts, which leaves the PRD without the commitments that make it accountable.

The harm this skill prevents is the PRD that cannot be held accountable. Without defined scope, the work expands as new requests accumulate, because there is no boundary to reference. Without defined success, the work cannot be evaluated, because there is no outcome to check against. The result is work that consumes effort without a clear endpoint and that may or may not have achieved anything, because no one defined what achievement meant. Scope and success definition are what make a PRD a commitment rather than a wish list.

Use this skill before answering questions such as "how do we define success for this PRD", "what should the scope be", "how do we prevent scope creep", or "how do we know if this worked". The goal is to prevent the agent from producing a PRD with requirements but no accountability for scope or outcomes.

## Core Rules

### Define Success As Outcomes, Not As Deliverables

Success criteria should define the outcomes the work is meant to achieve, not the deliverables it is meant to produce. Deliverables are what is built; outcomes are what changes for users or the business as a result. "Ship a recommendation engine" is a deliverable; "increase the click-through rate on recommended items by 15 percent" is an outcome. Outcomes are what make the work valuable, and they are what should define success, because building deliverables that do not produce outcomes is failure dressed as success.

Defining success as outcomes also keeps the PRD honest about whether the proposed solution is the right one. If the outcome is clear, the team can evaluate whether the requirements will achieve it, and can adjust the solution if a better path emerges. If success is defined as deliverables, the solution is fixed regardless of whether it works, because the deliverable is the goal. Outcome-based success preserves the ability to find a better solution.

### Make Success Criteria Measurable And Time-Bound

Success criteria should be measurable, so that achievement can be verified rather than asserted, and time-bound, so that there is a clear point at which to evaluate. "Improve user retention" is neither measurable nor time-bound; "increase 30-day retention from 40 percent to 50 percent within two quarters of launch" is both. Measurability ensures that success is objective, preventing disputes about whether it was achieved. Time-binding ensures that success is evaluated, rather than deferred indefinitely while the team waits for the metrics to improve.

Measurability requires defining the metric, the baseline, the target, and the measurement method. What exactly is being measured, what is it now, what does it need to be, and how will it be measured? Each of these must be specified for the criterion to be verifiable. If any is missing, the criterion is subjective and will be disputed. Invest in specifying these, because vague success criteria provide no accountability.

### Bound Scope To What Serves The Defined Outcomes

Scope should be bounded by what is necessary to achieve the defined outcomes, not by what could be useful or what stakeholders have requested. This means starting from the outcomes and working backward to the requirements that serve them, including only what contributes and excluding what does not. Scope defined this way is defensible: when a new request arises, it can be evaluated against whether it serves the outcomes, and included or excluded on that basis. Scope defined as a list of requests has no such defense and expands without limit.

Bounding scope to outcomes also requires discipline about what is essential versus what is nice-to-have. Within the outcome-serving requirements, identify the minimum that achieves the outcome, and treat the rest as potential additions that can be deferred. This creates a core scope that is committed and an extended scope that is optional, which gives the team flexibility to adjust based on what they learn without losing the core commitment. The core is what must be done; the extended is what might be done if capacity allows.

### State Out-Of-Scope Explicitly To Prevent Creep

Out-of-scope is as important as in-scope, because it is the explicit statement of what the work will not do. Stating out-of-scope prevents scope creep by creating a reference: when a request falls outside the stated scope, it can be declined or deferred with reference to the PRD, rather than becoming a source of conflict. Out-of-scope also manages expectations, telling stakeholders what not to expect so that they are not surprised by its absence.

The most valuable out-of-scope items are those that stakeholders might reasonably expect to be included. If a feature seems like it should be part of the work but is not, stating it as out-of-scope, with brief rationale, prevents the expectation from creating disappointment or conflict later. These are the items that cause scope creep, because their absence feels like an omission; naming them as deliberate exclusions removes that feeling and makes the scope boundary clear.

### Connect Scope And Success To Enable Accountability

Scope and success are connected: the scope commits to what will be built, and the success criteria commit to what the build will achieve. Together, they create accountability: if the scope is delivered but the success criteria are not met, the work has failed despite delivering what was committed, which is a signal that the solution was wrong. If the scope is not fully delivered but the success criteria are met, the work has succeeded with less than planned, which is a signal that the scope was over-specified. Connecting the two allows the work to be evaluated honestly, on both dimensions.

This connection also enables learning. When scope is delivered and success is not achieved, the team learns that the solution did not work, which informs future work. When success is achieved with less scope, the team learns that the scope was inflated, which improves future scoping. Without the connection, these lessons are lost, because there is no outcome to compare against the deliverables. The connection turns each PRD into a learning opportunity, not just a build commitment.

### Revisit Scope And Success As Learning Emerges

Scope and success are defined at the start, but they should be revisited as the team learns. If discovery reveals that the outcomes are not achievable as defined, or that the scope is insufficient or excessive, adjust the definitions rather than holding to commitments that no longer make sense. The PRD is a commitment, but it is not immutable; learning that invalidates the commitment should update it, with the change documented and communicated to stakeholders. Holding to outdated scope or success definitions produces work that serves the document rather than the outcomes.

Revisiting requires discipline to avoid using learning as an excuse to expand scope or lower the success bar. Each adjustment should be justified by specific learning, documented with rationale, and communicated transparently. The goal is to keep the scope and success definitions honest reflections of what the team knows, not to relax commitments when the work proves harder than expected. Adjust for genuine learning, not for convenience.

## Common Traps

### Success Defined As Deliverables

Defining success as what is built rather than what changes. The trap is work that ships deliverables without achieving outcomes.

### Vague Success Criteria

Success statements without metrics, baselines, or timelines. The trap is success that cannot be verified and disputes about achievement.

### Scope As A Request List

Scope defined by what stakeholders want rather than what outcomes require. The trap is scope that expands without a defensible boundary.

### Missing Out-Of-Scope

Failing to state what is excluded. The trap is scope creep driven by reasonable expectations of inclusion.

### Disconnected Scope And Success

Scope and success defined independently. The trap is inability to evaluate whether the work achieved its purpose.

### Immutable Commitments Despite Learning

Holding scope and success fixed when learning invalidates them. The trap is work that serves the document rather than the outcomes.

## Self-Check

- [ ] Success is defined as outcomes, not as deliverables, preserving the ability to evaluate whether the solution worked.
- [ ] Success criteria are measurable and time-bound, with metric, baseline, target, and method specified.
- [ ] Scope is bounded to what serves the defined outcomes, with a defensible boundary against creep.
- [ ] Out-of-scope items are stated explicitly, especially those stakeholders might reasonably expect included.
- [ ] Scope and success are connected, enabling accountability on both what is built and what it achieves.
- [ ] Scope and success are revisited as learning emerges, with adjustments justified and documented.
- [ ] The PRD commits to both what will be built and what outcome that build is meant to achieve.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
