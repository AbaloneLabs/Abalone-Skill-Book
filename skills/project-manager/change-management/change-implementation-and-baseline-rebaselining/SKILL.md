---
name: change_implementation_and_baseline_rebaselining.md
description: Use when the agent is implementing an approved change, updating the project baseline after change, re-baselining schedule cost or scope, managing change rollout and rollback, handling configuration and documentation updates, or transitioning a change from approval into delivered work.
---

# Change Implementation and Baseline Re-baselining

Approval is not the end of change control; it is the midpoint. The decisions that determine whether a change actually succeeds are made during implementation and baseline update. A change can be correctly analyzed, properly approved, and still cause damage if the baseline is not updated, if the rollout is uncoordinated, or if the team continues working against the old plan while the new one exists only on paper. The gap between "approved" and "reflected in how people actually work" is where most change value leaks away.

The judgment problem is how to translate an approved change into executed work without losing traceability, how to decide when a series of changes warrants a formal re-baseline versus incremental updates, and how to keep configuration, documentation, contracts, and stakeholder expectations synchronized with the new reality. Agents tend to treat the approval as the finish line, update only the schedule, and leave the rest of the project artifacts stale.

## Core Rules

### Treat Implementation as a Mini-Project With Its Own Plan

An approved change is a commitment to deliver something new under new constraints. It deserves its own implementation plan: tasks, owners, dependencies, definition of done, and acceptance criteria. Bolting the change onto existing work without a plan produces invisible slippage. The plan should reference the original impact analysis so that the assumptions made during review can be validated against what actually happens.

### Update All Baseline Components, Not Just the Schedule

A change rarely affects only dates. It affects scope statements, the work breakdown structure, the budget, resource assignments, quality criteria, risk register entries, and possibly contracts. Updating only the schedule leaves a project where the plan, the budget, and the scope documents disagree. Every approved change must trigger a defined set of baseline updates, and each update must be versioned so the before and after states are recoverable.

### Decide Incremental Update Versus Formal Re-baseline Deliberately

Small changes can be absorbed as incremental updates to the existing baseline. But when accumulated changes distort the baseline enough that variance reporting becomes meaningless, a formal re-baseline is warranted. Re-baselining resets the reference point, which means historical variance is preserved separately. Make this decision based on whether the current baseline still supports meaningful performance measurement, not on whether the numbers look bad.

### Synchronize Configuration and Documentation With the Change

A change to a deliverable must propagate to requirements, design documents, test cases, user documentation, training material, and configuration items. The trap of updating only the build artifact is that the next maintainer works from stale specs. Maintain a configuration index that lists every artifact affected by each component, and verify the change has propagated before declaring the change complete.

### Plan Rollout and Rollback Before Executing

Every change implementation should define how it will be deployed and how it can be reversed if it fails. For small changes this may be trivial; for large or customer-facing changes it is critical. A change without a rollback plan that fails in production becomes an incident. Define success criteria, the monitoring that will confirm them, and the decision point and method for rollback.

### Validate the Impact Analysis Against Actual Results

The impact analysis predicted cost, schedule, and risk effects. After implementation, compare the prediction to the actual result. This closes the loop on change quality and improves future estimates. Systematic over- or under-estimation of change impact is a leading indicator of a planning or estimation problem that should be addressed, not ignored.

### Manage Stakeholder Expectations Through the Transition

Stakeholders who approved a change often expect it immediately and visibly. The implementation timeline, interim states, and any temporary disruption must be communicated. A change that is technically complete but surprises stakeholders with downtime, partial functionality, or shifted dates will be perceived as a failure even when it met its criteria. Communicate the implementation path, not just the outcome.

### Preserve Traceability From Request to Delivered Change

Every approved change should be traceable from the original request, through the impact analysis, the approval decision, the implementation tasks, the updated baseline, and the final delivered and accepted result. This traceability is what makes the project defensible and what allows later teams to understand why the project looks the way it does. Break the chain anywhere and the project loses its institutional memory.

## Common Traps

### Treating Approval as the Finish Line

The team relaxes once the change is approved, assuming the hard part is over. Implementation drifts, the baseline is not updated, and the change is half-delivered. The trap is that the governance effort was front-loaded and the execution effort was assumed. Treat implementation as the higher-risk phase.

### Updating Only the Schedule

The schedule moves but the budget, scope document, and contracts stay on the old baseline. Variance reports then contradict each other, and the next change analysis is built on stale data. The trap is that the schedule is the most visible artifact, so it gets updated and the rest is forgotten. Enforce a baseline-update checklist per change.

### Re-baselining to Hide Variance

When the numbers look bad, the team re-baselines to reset them to zero, masking real performance problems. The trap is using re-baselining cosmetically rather than structurally. Re-baselining is legitimate when the old baseline no longer reflects reality, but the historical variance must be preserved and the reason for re-baselining documented.

### Stale Configuration and Documentation

The code or deliverable changes but the requirements, test cases, and manuals do not. Future work is built on documents that lie. The trap is that documentation updates feel like overhead compared to "real" work. Tie change completion to configuration propagation, not just to build completion.

### No Rollback Plan for Customer-Facing Changes

A change is deployed, it breaks something, and there is no tested way back. The team scrambles a fix-forward while customers are affected. The trap is assuming the change will work because it passed testing. Require a rollback plan and a defined decision point for invoking it.

### Implementing Without Validating the Original Estimate

The change cost twice what was predicted, but no one checks, so the next estimate repeats the error. The trap is that post-implementation review feels like extra work with no immediate value. Build a lightweight comparison of predicted versus actual into the change close-out.

### Communicating the Outcome but Not the Transition

Stakeholders are told the change is done, but they experience downtime or partial features in the interim and conclude the project is failing. The trap is assuming stakeholders understand the implementation path. Communicate interim states, temporary disruption, and the path to full value, not just the end state.

## Self-Check

- [ ] Does each approved change have its own implementation plan with tasks, owners, dependencies, and acceptance criteria?
- [ ] Are all baseline components (scope, WBS, schedule, budget, resources, quality, risk, contracts) updated and versioned, not just the schedule?
- [ ] Is the decision between incremental update and formal re-baseline made explicitly, based on whether the baseline still supports meaningful measurement?
- [ ] When re-baselining occurs, is historical variance preserved and the reason documented rather than erased?
- [ ] Has the change propagated to requirements, design, test cases, documentation, training, and configuration items, verified against a configuration index?
- [ ] Is there a defined rollout plan and a tested rollback plan with success criteria and a decision point for invoking rollback?
- [ ] Is the original impact analysis compared against actual implementation results to improve future estimation?
- [ ] Are stakeholders communicated the implementation path, interim states, and temporary disruption, not only the final outcome?
- [ ] Is there end-to-end traceability from the original request through analysis, approval, implementation, baseline update, and accepted delivery?
- [ ] Can a later team reconstruct, from the records alone, why the project looks the way it does as a result of accumulated changes?
