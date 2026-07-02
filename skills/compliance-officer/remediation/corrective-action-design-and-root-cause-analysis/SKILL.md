---
name: corrective_action_design_and_root_cause_analysis.md
description: Use when the agent is designing corrective actions after a compliance finding, conducting root cause analysis, distinguishing a symptom from an underlying cause, deciding remediation depth, or scoping fixes after an audit, regulatory exam, or internal investigation.
---

# Corrective Action Design And Root Cause Analysis

A compliance finding is only the surface of a problem. The real compliance work begins after the finding is recorded, when someone must decide why the failure happened, what would actually prevent recurrence, and how deep the fix must go. The most common failure mode in remediation is not skipping remediation; it is doing remediation that looks thorough on a tracker but addresses the symptom rather than the cause. A control that was bypassed gets retrained. A process that lacked ownership gets a new checklist. A systemic gap gets patched with a one-time review. The finding closes, the root cause persists, and the same violation reappears months later in a different form.

Use this skill before designing corrective actions, scoping root cause analysis, deciding between corrective and preventive measures, or evaluating whether a proposed remediation plan is defensible. The goal is to make the agent resist shallow fixes, demand genuine causal analysis, and design actions whose effectiveness can be tested rather than merely asserted. This is not legal advice; remediation decisions in regulated or enforcement contexts should be reviewed with qualified counsel and the relevant compliance and internal audit functions.

## Core Rules

### Distinguish Finding, Symptom, Cause, And Root Cause

Remediation fails most often because these four levels are collapsed into one. A finding is what was observed, for example that a high-risk vendor was onboarded without enhanced due diligence. The symptom is the visible manifestation, the missing EDD file. The proximate cause is the immediate reason, the analyst did not run the EDD checklist. The root cause is the systemic driver, the risk-rating model did not flag the vendor as high risk because the jurisdiction-weighting table was outdated, and no one owned the table's maintenance. A corrective action aimed at the symptom, retraining the analyst, does nothing to prevent the next vendor from being mis-rated.

Force the analysis through each layer. For each finding, ask what happened, why it happened at the point of failure, why the control did not catch it, and why the underlying condition existed. Use a structured method such as the five-whys technique, fishbone analysis, or a fault tree, but treat the method as a discipline, not a ritual. The test of a genuine root cause is whether addressing it would plausibly prevent the entire class of failure, not merely this instance.

### Decide Corrective Versus Preventive Versus Detective

Not all remediation serves the same purpose, and conflating them weakens the plan. Corrective action fixes the specific instance that was found, for example completing the missing EDD on the vendor already onboarded. Preventive action changes the system so the failure cannot recur, for example fixing the risk model and assigning ownership of the jurisdiction table. Detective action improves the ability to catch the failure if it slips through, for example adding a periodic exception report on vendors onboarded without EDD.

A defensible remediation plan usually needs all three. Corrective alone closes the immediate gap but leaves the system vulnerable. Preventive alone leaves the existing bad state unaddressed. Detective without preventive means the same failure is found repeatedly without ever being stopped. Map each action to its category explicitly so reviewers can see whether the plan covers fix, prevent, and detect.

### Tie Every Action To A Specific Root Cause

Each corrective action must trace back to a named root cause. If the action cannot be connected to a cause in the analysis, either the analysis is incomplete or the action is decorative. A common weakness is the generic remediation, retrain staff, update the policy, enhance monitoring, that appears in every plan regardless of the finding. These generic actions signal that no real analysis occurred.

For each action, document the causal chain in reverse: this action addresses this cause, which produced this symptom, which was observed as this finding. If the chain breaks, the action is not justified. If two actions address the same cause, one may be redundant. If a root cause has no action, the plan has a gap.

### Assign Ownership, Authority, And Resources

A remediation action without an accountable owner is unowned in practice. The owner must be a named individual or function with the authority to implement the change and the resources to do so. Assigning ownership to a committee or to compliance in general diffuses responsibility until no one is accountable.

For each action, define the responsible owner, the approving authority for the change, the resources required, and the dependencies. If the action requires system changes, budget, or cross-functional cooperation, identify those dependencies explicitly. A remediation that depends on an IT roadmap item with no committed date is not really remediated; it is deferred.

### Set Measurable Completion Criteria

An action is complete when its completion can be demonstrated with evidence, not when the owner declares it done. Define completion criteria that are observable and testable. Retraining is complete when attendance and assessment results are recorded for the target population, not when the session was held. A control change is complete when the revised control is documented, deployed, and operating for a defined period with evidence of operation, not when the policy was rewritten.

Avoid vague completion language such as implemented, enhanced, or addressed. Each criterion should answer what evidence an auditor would demand and whether that evidence would exist. Tie completion criteria to the root cause so that closing the action actually closes the causal gap.

### Sequence Actions By Risk And Dependency

Not all actions can or should proceed simultaneously. Some root causes produce high residual risk and demand interim mitigation while the permanent fix is built. Some actions depend on others; a detective report cannot be built until the data field that feeds it is corrected. Sequence the plan so that the highest-risk gaps receive interim controls first, dependencies are respected, and the critical path is visible.

Include interim mitigation for any root cause that leaves a material exposure open during the remediation period. A plan that takes nine months to fix a control while leaving the gap fully open in the meantime is not defensible; an interim detective control or manual review reduces the exposure during the fix.

### Validate Effectiveness After Closure

Closing an action on the tracker is not proof it worked. Effectiveness validation tests whether the root cause was actually addressed. This may involve retesting the control after a defined operating period, monitoring the relevant metric for recurrence, or conducting a targeted review after sufficient transactions have flowed through the revised process.

Build effectiveness validation into the plan at the outset, with the metric or test defined before the action closes. An action that cannot be validated for effectiveness should be redesigned so that it can be. This step is where second-line and internal audit confidence in the remediation is earned or lost.

## Common Traps

### Closing The Finding Instead Of The Cause

Marking a finding closed because the immediate instance was fixed, while the systemic driver remains untouched. The finding reappears in the next cycle and the remediation record looks dishonest.

### Defaulting To Retraining And Policy Updates

Retraining and policy revision are the most common generic remediation because they are easy to execute and document. They are appropriate only when knowledge or awareness was genuinely the root cause, which is rarely the whole story.

### Setting Dates Without Assessing Feasibility

Committing to remediation dates to satisfy a regulator or committee without assessing whether the work, dependencies, and resources can realistically be delivered. Slippage then undermines the credibility of the entire program.

### Treating The Tracker As The Deliverable

Maintaining a detailed remediation tracker with green statuses while the underlying actions are superficial or unverified. The tracker is a management tool, not the remediation itself.

### Ignoring Interim Risk

Allowing a material gap to remain fully open during a long remediation period without interim mitigation. The exposure during the fix window can exceed the original finding's impact.

### Overloading Single Owners

Assigning dozens of actions to one compliance officer who lacks the authority or bandwidth to drive cross-functional change. Remediation stalls and the owner becomes the scapegoat.

### Failing To Reconcile Across Findings

Treating each finding in isolation when multiple findings share a common root cause. Fragmented remediation misses the systemic fix and wastes effort on redundant actions.

## Self-Check

- Does the analysis distinguish finding, symptom, proximate cause, and root cause, with each root cause capable of explaining the class of failure rather than only the instance?
- Does the plan include corrective, preventive, and detective actions mapped explicitly to named root causes?
- Is every action traceable through a documented causal chain back to a specific root cause, with no orphan actions and no unaddressed causes?
- Does each action have a named individual owner with defined authority, resources, and dependencies rather than diffuse committee ownership?
- Are completion criteria observable and evidence-based rather than stated as implemented or enhanced?
- Does the plan include interim mitigation for any root cause leaving material exposure open during the remediation period?
- Is the action sequence driven by risk and dependency rather than by convenience, with the critical path visible?
- Is effectiveness validation defined before action closure, with a specific metric or test that would confirm the root cause was addressed?
- Have generic actions such as retrain staff or update policy been justified against an actual knowledge or documentation root cause rather than used as defaults?
- Would an independent reviewer or internal auditor be able to reconstruct why each action exists and confirm it addresses a real cause?
