---
name: project_integration_and_charter_development.md
description: Use when the agent is developing a project charter, integrating project components into a coherent plan, aligning subsidiary plans, establishing project governance and authority, defining the integration management approach, or ensuring consistency across scope schedule cost and quality plans.
---

# Project Integration and Charter Development

Integration management is the connective discipline that holds a project together. Every other knowledge area, scope, schedule, cost, quality, risk, resources, produces its own plans and artifacts. Without integration, these plans drift apart: the schedule assumes resources the resource plan never allocated, the budget assumes a scope the scope plan never approved, and the quality plan tests against criteria the requirements never defined. Integration failure is rarely loud; it is the slow accumulation of contradictions that surfaces only when the pieces are forced together.

The judgment problem is how to create a charter that establishes real authority and clear boundaries, how to integrate subsidiary plans so they are mutually consistent, and how to maintain that consistency as the project changes. Agents tend to treat the charter as a formality, treat integration as automatic, and discover the contradictions only at integration testing or deployment, when they are most expensive to fix.

## Core Rules

### Make the Charter the Authoritative Source of Project Authority and Boundaries

The charter is not a summary document; it is the instrument that authorizes the project and defines its boundaries. It must state the business justification, the objectives with measurable success criteria, the high-level scope, the sponsor and the project manager's authority, the initial constraints and assumptions, and the approval that commits resources. A charter without clear authority leaves the project manager unable to make decisions; one without clear boundaries invites scope ambiguity. Get it signed before work consumes resources.

### Define Objectives as Measurable Outcomes, Not Activities

Project objectives in the charter must be expressed as outcomes that can be verified: "reduce order processing time from 48 to 24 hours," not "improve the ordering system." Activity-based objectives cannot be tested for completion and drift into perpetual work. Each objective should have an acceptance measure and a target, so that at project closure there is a factual basis for declaring success.

### Integrate Subsidiary Plans for Mutual Consistency

Each knowledge area produces a plan. Integration means verifying they are consistent: the schedule's resource needs match the resource plan; the budget's cost baseline matches the schedule's cost-loaded activities; the quality plan's acceptance criteria match the scope's requirements; the risk plan's contingencies are funded in the budget. Inconsistency between plans is the most common integration defect. Cross-check explicitly and document the reconciliation.

### Establish a Single Source of Truth for the Integrated Baseline

There must be one authoritative, version-controlled integrated baseline that combines scope, schedule, cost, and resource commitments. Competing spreadsheets, each "the plan," guarantee inconsistency. The single source of truth is what changes are measured against and what gets updated when changes are approved. Everyone works from the same version.

### Align Governance and Decision Authority With the Charter

The charter defines who can make what decisions. Governance, the change control board, approval thresholds, and escalation paths must align with that authority. A charter that grants the project manager moderate authority but a governance structure that routes every decision to the sponsor creates friction and delay. Make the governance structure consistent with the authority the charter establishes.

### Sequence Integration Throughout the Lifecycle, Not Just at the End

Integration is not a final assembly step. It happens continuously: requirements integrated into design, design into build, build into test, test into deployment, and all of it into the baseline. Each phase should produce integration checkpoints where the pieces are verified to fit. Waiting until the end to integrate guarantees expensive rework when mismatches are found late.

### Manage Assumptions and Constraints as Living Artifacts

The charter records initial assumptions and constraints, but these change. An assumption that was valid at chartering may be invalidated by a regulatory shift, a vendor failure, or a market change. Maintain assumptions and constraints as living artifacts, review them at major milestones, and update the integrated baseline when they shift. Stale assumptions are a hidden source of integration failure.

### Ensure Traceability From Objective to Deliverable to Plan

Every project objective should trace to deliverables, every deliverable to requirements, every requirement to schedule activities and cost estimates. This traceability is what makes integration verifiable: you can confirm that every objective is planned, funded, and scheduled, and that every planned activity serves an objective. Gaps in either direction signal integration defects.

## Common Traps

### The Charter as Formality

The charter is written to satisfy a process requirement, signed as a formality, and then ignored. The trap is that without a living charter, authority and boundaries are undefined and redisputed throughout the project. Treat the charter as the contract for the project.

### Activity-Based Objectives

Objectives describe work to be done rather than outcomes to be achieved, so completion is untestable and the project never clearly "done." The trap is that activity objectives enable perpetual scope. Express objectives as measurable outcomes.

### Plans That Silo Themselves

Each knowledge area plan is developed in isolation and never cross-checked. The schedule and budget contradict each other, and no one notices until variance reporting breaks. The trap is treating each plan as complete when it is internally consistent but externally inconsistent. Integrate explicitly.

### Multiple Competing Versions of the Plan

Different stakeholders maintain different spreadsheets, each called "the plan," and decisions are made against different data. The trap is that without a single source of truth, integration is impossible. Maintain one authoritative baseline.

### Integration Deferred to the End

The team plans to "integrate at the end," by which time mismatches are embedded in completed work. The trap is treating integration as assembly rather than as a continuous discipline. Integrate at every phase boundary.

### Stale Assumptions

Assumptions recorded at chartering are never revisited, so when conditions change, the plan rests on foundations that no longer hold. The trap is that assumptions feel static but the world does not. Review assumptions at milestones.

### No Traceability Between Objectives and Plans

Objectives exist in the charter, plans exist in their documents, and nothing connects them. The trap is that you cannot verify the plan actually delivers the objectives, nor identify orphan work. Build and maintain traceability.

## Self-Check

- [ ] Does the charter establish clear project authority, boundaries, and signed approval before resources are consumed?
- [ ] Are project objectives expressed as measurable outcomes with acceptance criteria and targets, not as activities?
- [ ] Have subsidiary plans (scope, schedule, cost, resource, quality, risk) been cross-checked for mutual consistency?
- [ ] Is there a single, version-controlled integrated baseline that serves as the source of truth for all changes?
- [ ] Is the governance structure, including decision authority and escalation, aligned with the charter's granted authority?
- [ ] Are there integration checkpoints at each phase boundary, not only a final integration step?
- [ ] Are assumptions and constraints maintained as living artifacts, reviewed at major milestones?
- [ ] Is there traceability from each objective to deliverables, requirements, schedule activities, and cost estimates, in both directions?
- [ ] Can you demonstrate that every planned activity serves a charter objective and every objective is planned, funded, and scheduled?
- [ ] Would a reviewer find the integrated plans mutually consistent, or could they identify contradictions between scope, schedule, cost, and quality?
