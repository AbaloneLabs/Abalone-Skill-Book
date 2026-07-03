---
name: change_request_intake_and_impact_analysis.md
description: Use when the agent is receiving a change request, performing impact analysis on a proposed change, triaging change scope, assessing schedule cost and resource effects of a change, deciding whether a change should be fast-tracked or formally reviewed, or preparing change documentation for a project.
---

# Change Request Intake and Impact Analysis

Change is the one constant in projects, yet most change-related damage does not come from the change itself. It comes from changes that enter the project informally, bypass impact analysis, and silently consume budget, schedule, and team capacity before anyone notices. The intake and impact-analysis stage is where most change control failures actually originate. A change that is accepted verbally, partially analyzed, or rubber-stamped becomes a hidden commitment that distorts the baseline and erodes trust.

The judgment problem here is not "how do I fill out a change form." It is how to distinguish a legitimate, well-understood change from a vague wish, a disguised defect, or a scope creep dressed as a fix, and how to quantify the real cost of a change before committing to it. Agents tend to under-analyze small changes, over-analyze trivial ones, and treat impact analysis as paperwork rather than a decision tool.

## Core Rules

### Treat Every Change as a Baseline Distortion Until Proven Otherwise

A change request is a proposal to alter an approved baseline of scope, schedule, cost, quality, or resources. The default posture should be skeptical, not accommodating. Before any analysis, confirm that the current baseline is documented and that the change is genuinely a deviation from it. If the baseline is unclear, the first task is to establish what "changed" actually means. A change against an undefined baseline is unanalyzable.

### Require a Structured Intake Before Any Work Begins

Every change, regardless of size, should pass through a defined intake that captures originator, business justification, requested description, priority, and requested date. The intake must happen before implementation. The most common failure mode is the "quick fix" that gets worked on immediately and documented later, at which point the documentation becomes fiction. Intake can be lightweight for small changes, but it cannot be skipped.

### Separate Defects, Enhancements, and Scope Changes

Not every change is the same kind of decision. A defect fix restores the agreed scope to its intended behavior. An enhancement adds new value within the spirit of the original scope. A scope change introduces something materially new. These have different approval thresholds, different funding sources, and different risk profiles. Misclassifying a scope change as a defect is one of the most common ways to smuggle in unapproved work. Force an explicit classification at intake.

### Analyze Impact Across All Knowledge Areas, Not Just the Obvious One

A change that looks schedule-only often has cost, resource, quality, risk, and contractual ripple effects. Impact analysis must cover at minimum: scope, schedule, cost, resources, quality, risk, contracts, and stakeholder expectations. A change that adds a feature may also require new test environments, vendor coordination, training, documentation updates, and revised acceptance criteria. List each affected area explicitly and quantify where possible.

### Quantify the Triple Constraint and the Hidden Costs

The visible cost of a change is the direct labor. The hidden costs include context-switching, rework of already-completed work, re-testing, regression risk, documentation churn, and the opportunity cost of work displaced. A small change that disrupts a critical-path activity is far more expensive than a large change in a non-critical area. Estimate both direct and indirect impact, and flag any change that touches the critical path or a recently completed deliverable.

### Apply a Tiered Severity and Approval Model

Not every change warrants a full change control board review. Define thresholds that route changes by impact: trivial changes handled by the project manager with notification, moderate changes requiring sponsor approval, and major changes requiring board review with baseline re-baselining. The thresholds must be defined in advance and based on measurable criteria such as cost variance, schedule slip, or scope boundary crossing, not on who is asking.

### Validate the Business Case Before Investing in Analysis

Some changes are not worth analyzing in depth. Before spending effort on full impact analysis, confirm the change has a real sponsor, a stated business value, and a willing source of funding or trade-off. A change with no identified owner and no stated value should be rejected or deferred at intake, not carried forward. This prevents the backlog from filling with phantom changes that consume review cycles.

### Document Assumptions, Dependencies, and Constraints Explicitly

Impact analysis is only as good as its stated assumptions. Record what was assumed (resource availability, vendor lead times, regulatory acceptance), what dependencies exist (other changes, external approvals, seasonal windows), and what constraints bound the analysis (frozen dates, fixed budget, regulatory deadlines). When the change is later implemented, these assumptions become the basis for judging whether the analysis was correct.

## Common Traps

### The Verbal Change That Becomes a Commitment

A stakeholder asks for "just one small thing" in a hallway, the team does it, and it is never recorded. By the time the cost surfaces, the baseline is already distorted and there is no traceable decision to revisit. This trap occurs because saying yes feels collaborative and saying "please submit a change request" feels bureaucratic. The fix is to make intake so lightweight that there is no excuse to bypass it.

### Classifying Scope Additions as Bug Fixes

Calling a new requirement a "defect" or "the original intent" is a common way to avoid change control. This happens when the original requirements were genuinely ambiguous, or when someone wants to avoid the approval friction. The trap is that defect fixes often have different funding and no formal approval, so mislabeling silently expands scope. Force an explicit classification and require evidence for defect claims.

### Analyzing Only the Direct Cost

Teams estimate the hours to build the new feature and ignore the testing, documentation, training, deployment, rollback planning, and support implications. The change is approved on an incomplete picture and then overruns. The trap is that indirect costs are harder to estimate and less visible, so they get dropped. Always enumerate the full lifecycle of the change, not just the build effort.

### Fast-Tracking Changes Without a Recovery Plan

Urgent changes get approved with "we'll fix the schedule later." Later never comes, and the compressed downstream work causes quality failures or burnout. The trap is conflating urgency with importance and assuming the schedule will absorb the hit. Any fast-tracked change must carry an explicit recovery or trade-off plan, not a promise to figure it out.

### Ignoring Cumulative Impact of Multiple Small Changes

Each small change looks harmless in isolation, but dozens of them compound into significant schedule slip, budget erosion, and team fatigue. The trap is analyzing each change as if it were the only one. Maintain a running view of approved change volume and its cumulative effect on the baseline, and escalate when the aggregate crosses a defined threshold.

### Approving Changes That Have No Funding Source

A change is approved because it is valuable, but no one confirms where the money, time, or resources will come from. The project absorbs it by cutting elsewhere, usually quality or team capacity. The trap is treating approval as a value judgment rather than a resource commitment. Require an explicit source for the cost of every approved change.

### Suppressing Changes to Protect the Baseline

The opposite of scope creep is change suppression, where legitimate, valuable changes are rejected or delayed indefinitely to keep metrics clean. This produces a project that hits its numbers but delivers something nobody wants. The trap is treating the baseline as sacred rather than as a tool. The baseline exists to serve the business outcome, not the reverse.

## Self-Check

- [ ] Is there a documented current baseline (scope, schedule, cost) against which the change is being evaluated?
- [ ] Does every change, including small or verbal ones, pass through a defined intake before work begins?
- [ ] Has each change been explicitly classified as defect, enhancement, or scope change, with evidence?
- [ ] Does the impact analysis cover scope, schedule, cost, resources, quality, risk, contracts, and stakeholder expectations, not just the obvious area?
- [ ] Are both direct and indirect costs (rework, retest, context-switching, opportunity cost) included in the estimate?
- [ ] Has any change touching the critical path or a completed deliverable been flagged for elevated review?
- [ ] Are tiered approval thresholds defined in advance and based on measurable criteria rather than requester seniority?
- [ ] Does the change have an identified sponsor, a stated business value, and a confirmed funding or trade-off source?
- [ ] Are assumptions, dependencies, and constraints documented so the analysis can be validated later?
- [ ] Is there a running view of cumulative approved change volume and its effect on the baseline?
