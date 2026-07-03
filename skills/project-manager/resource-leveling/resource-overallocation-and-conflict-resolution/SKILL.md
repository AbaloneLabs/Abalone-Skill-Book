---
name: resource_overallocation_and_conflict_resolution.md
description: Use when the agent is resolving resource overallocation, handling conflicts where one person is assigned beyond capacity, deciding between delaying work, splitting assignments, or reassigning to relieve overload, or reviewing whether an overallocation is being fixed rather than hidden.
---

# Resource Overallocation And Conflict Resolution

Overallocation is what happens when the schedule demands more of a resource than it can actually deliver in a period. The conflict is rarely about the resource itself; it is about the commitments the schedule has stacked onto it. The mistake most agents make is to treat overallocation as a localized annoyance to be smoothed away, when in fact it is a signal that the plan is infeasible as written. Resolving it well means choosing, deliberately, among a small set of real options and accepting the consequences of each, rather than quietly hoping the person will absorb the gap.

The judgment problem is deciding which resolution path fits the situation: delaying the work, splitting it, reassigning it, reducing it, or escalating for more capacity. Each option has a different cost in schedule, risk, quality, and relationships. Agents tend to default to whichever option is easiest to execute, often reassignment, without checking whether the receiving resource has the capability or capacity, or whether the delay they cause ripples onto the critical path.

## Core Rules

### Confirm The Overallocation Is Real Before Resolving It

Not every peak on a resource histogram is a problem. A person scheduled at 110 percent for a single day is usually fine; the same person at 115 percent for six weeks is not. Distinguish transient spikes, which the resource absorbs with minor flexing, from sustained overallocation, which produces defects, slippage, and attrition. Also distinguish calendar overallocation, too many hours, from commitment overallocation, too many concurrent tasks causing context-switching loss even when the hours add up. Resolve only what is genuinely infeasible; over-correcting transient spikes produces unnecessary churn.

### Resolve In Priority Order, Critical Path First

When multiple resources are overallocated, the order of resolution matters. Start with the resource that sits on or near the critical path, because resolving an overallocation there protects the end date, while resolving one on a path with float only protects local efficiency. If the overallocated resource is also the binding constraint, its conflict dominates everything else. Sequence your fixes so that resolving one conflict does not silently create a worse one on the critical path.

### Choose The Resolution Tactic By Its Full Consequence

Each resolution tactic has a distinct cost profile, and the right choice depends on which cost the project can bear. Delaying work consumes float or pushes the end date; it is safe when float exists and dangerous when it does not. Splitting an assignment spreads the load but adds handoff overhead, relearning cost, and integration risk, and some work splits badly. Reassigning moves the load to another resource but only helps if that resource genuinely has capacity and capability; otherwise you have moved the overload, not solved it. Reducing or descoping the work removes the conflict entirely but changes the deliverable. Escalating for added capacity costs money and ramp-up time. Name the tradeoff explicitly rather than picking a tactic by habit.

### Verify Capability Before Reassigning

Reassignment is the most tempting fix because it looks clean on the histogram, but it fails silently when the receiving resource lacks the skill, access, or context to do the work. Before reassigning, confirm the new assignee has the required capability, the necessary permissions and environment access, and enough ramp-up time built into the new plan. A reassignment that assumes capability the person does not have converts an overallocation problem into a quality and schedule problem that surfaces later, when it is harder to fix.

### Check For Multi-Project Conflict, Not Just Intra-Project

A resource can be perfectly allocated within your project and still overallocated across the portfolio, because three project managers each assumed 80 percent of the same person. Overallocation that is invisible inside a single project view is often the real cause of slippage. When a conflict resists resolution, check the resource's total commitment across projects and shared services. Resolving it may require portfolio-level negotiation, not project-level juggling, and that is an escalation, not a failure.

### Make The Resolution Visible And Approved

Resolving an overallocation usually changes dates, assignments, or scope, and those changes affect commitments made to stakeholders. Document the conflict, the options considered, the chosen resolution, and its impact, and route it through whatever lightweight change control applies. Quietly reassigning work or slipping dates to fix an overallocation, without recording it, erodes the baseline and makes future variance meaningless. The resolution is a plan change; treat it as one.

### Re-Examine After Resolving

Resolving one overallocation frequently creates another, because the work has to go somewhere. After applying a fix, re-run the allocation view and confirm no new conflict has appeared, especially on the critical path. Iterate until the plan is feasible as a whole, not just at the point of original pain. Stopping at the first fix is how hidden overallocations survive into execution.

## Common Traps

### Treating Overallocation As A Cosmetic Histogram Problem

The bar is red on the chart, so the agent nudges a task by a day to turn it green, without asking whether the underlying plan is feasible. The trap is optimizing the picture rather than the plan. A green histogram built by hiding conflicts produces slippage in execution that no one planned for.

### Defaulting To Reassignment Without Capability Checks

Moving work to whoever has capacity is fast and feels decisive, but if the receiver cannot actually do the work, the conflict has been relocated, not resolved. The trap is that the histogram looks solved immediately, while the capability gap surfaces only at delivery, too late to recover cheaply.

### Resolving The Loudest Conflict First

The overallocated resource who complains gets attention; the silent overallocation on the critical path gets ignored until it slips the end date. The trap is equating visibility with priority. Resolve by criticality, not by noise.

### Ignoring Cross-Project Commitment

Each project manager fixes their own view and assumes the shared specialist has the time. The trap is that the sum of individually reasonable plans is collectively impossible. Without portfolio-level visibility, the conflict is never truly resolved.

### Splitting Work That Does Not Split

Some tasks look divisible but have a single thread of understanding, such as a complex integration or a delicate negotiation. Splitting them adds handoff cost and rework that exceeds the relief. The trap is assuming all work is fungible.

### Absorbing The Conflict Through Pressure

Instead of choosing a real resolution, the agent lets the overallocated person "just handle it," converting a planning problem into a human endurance problem. The trap is that this appears to work in the short term while building burnout, defects, and attrition risk that cost far more later.

### Fixing Once And Not Re-Checking

The first resolution creates a new peak elsewhere, which goes unnoticed because the original red bar is now green. The trap is stopping at the symptom. Always re-run the allocation after a fix.

## Self-Check

- [ ] Is the overallocation confirmed as sustained and infeasible rather than a transient spike that can be absorbed?
- [ ] Is calendar overallocation distinguished from commitment overallocation due to too many concurrent tasks?
- [ ] Are conflicts resolved in priority order, with critical-path and binding-constraint resources addressed first?
- [ ] For each chosen tactic, is the full consequence (delay, float consumption, handoff cost, scope change, added cost) named explicitly?
- [ ] Before any reassignment, is the receiving resource verified for capability, access, and ramp-up time?
- [ ] Has the resource's total commitment across projects and shared services been checked, not just the intra-project view?
- [ ] Is the resolution documented with options considered and routed through appropriate change control rather than applied silently?
- [ ] After resolving, has the allocation view been re-run to confirm no new conflict was created, especially on the critical path?
- [ ] Has the temptation to absorb the conflict through pressure on the individual been rejected in favor of a real planning fix?
- [ ] Is the resolution's impact on stakeholder commitments communicated along with the change itself?
