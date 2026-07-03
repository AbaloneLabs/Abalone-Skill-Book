---
name: constraint_driven_planning_and_decision_making.md
description: Use when the agent is planning or making decisions driven by the dominant constraint, such as timeboxed budget-boxed or scope-fixed projects, selecting a delivery approach that fits the binding constraint, or aligning planning cadence and decision rules to the project's real limit.
---

# Constraint-Driven Planning and Decision-Making

Once the binding constraint is known, it should drive the planning approach, the delivery method, the decision rules, and the cadence of control. A project whose binding constraint is a fixed launch date should be planned very differently from one whose binding constraint is a fixed budget or a non-negotiable scope. The judgment problem is that most projects are planned with a generic approach, a standard lifecycle, and uniform decision rules, regardless of what actually limits them. The result is a plan that looks professional but is structurally mismatched to the project's reality, so it fights the binding constraint instead of organizing around it.

Agents tend to reach for a familiar methodology, whether predictive or adaptive, and apply it uniformly. The skill is to let the binding constraint shape the approach: timeboxed projects need fixed-date planning with scope as the flex; budget-boxed projects need fixed-cost planning with scope or time as the flex; scope-fixed projects need predictive planning with time or cost as the flex. The constraint also determines which decisions matter most, who must make them, and how often the plan must be re-checked.

## Core Rules

### Let the Binding Constraint Choose the Delivery Approach

The dominant constraint should determine whether the project is planned predictively, iteratively, or in a hybrid. A fixed-scope, fixed-quality project with stable requirements suits a predictive lifecycle that fixes the deliverable and lets time and cost follow. A fixed-date project suits a timeboxed or iterative approach that fixes the date and lets scope flex through prioritization. A fixed-budget project suits a cost-capped approach that delivers the best possible scope within the envelope. Choosing a lifecycle that fights the binding constraint, such as predictive planning for a fixed-date exploratory effort, guarantees friction and likely failure.

### Define Explicitly Which Dimension Is Fixed and Which Flexes

State, in the planning artifacts, which constraint is locked and which is allowed to move. In a timeboxed project, the date is fixed and scope is the flex variable, managed through a prioritized backlog. In a budget-boxed project, cost is fixed and scope or schedule flexes. In a scope-fixed project, the deliverable is fixed and time or cost flexes, requiring contingency in both. Ambiguity about which dimension flexes produces simultaneous pressure on all of them and silent erosion of the one no one is defending. Make the flex dimension a deliberate, communicated choice.

### Align Decision Authority to the Binding Constraint

The decisions that most affect the binding constraint should sit with the authority that owns it, and should be made quickly. In a fixed-date project, scope prioritization decisions are critical and must reach the product owner or sponsor rapidly, because they directly protect the date. In a fixed-budget project, scope and timing decisions that affect cost must reach the funding authority without delay. Design the decision path so that the highest-leverage decisions, those touching the binding constraint, encounter the least friction. Decisions about non-binding dimensions can tolerate slower paths.

### Set the Planning and Control Cadence to the Constraint's Volatility

The frequency of replanning and control should match how fast the binding constraint's context changes. A fixed-date project with a moving scope needs frequent reprioritization, often iteration by iteration. A fixed-scope predictive project needs less frequent replanning but tighter milestone control. A project whose binding constraint is a volatile resource needs frequent capacity re-checks. A static cadence inherited from a template will either over-control a stable project or under-control a volatile one. Match the cadence to where the risk of constraint violation actually lives.

### Build the Plan to Protect the Binding Constraint First

Sequence work, allocate resources, and hold reserves so that the binding constraint is protected before anything else. In a fixed-date project, front-load the highest-value and riskiest scope so that, if the date approaches, what remains is the most deferrable. In a fixed-budget project, sequence to lock in the core value early so that later cuts remove the least important scope. In a scope-fixed project, build schedule and cost contingency around the riskiest deliverables. The plan's structure should reflect the priority of the binding constraint, not a generic activity order.

### Make Constraint Violations the Primary Health Signal

Define health in terms of whether the binding constraint is on track, not just whether tasks are moving. A fixed-date project is healthy if the date is achievable at current scope and velocity; a fixed-budget project is healthy if remaining scope fits the remaining envelope; a scope-fixed project is healthy if quality and completeness are tracking to the fixed deliverable. Reporting task progress while the binding constraint drifts gives false comfort. Make the binding constraint's status the headline of every health report.

### Pre-Decide the Response When the Binding Constraint Is Threatened

Before the binding constraint is at risk, agree on the response: in a fixed-date project, what scope gets cut first; in a fixed-budget project, what scope is deferred when the envelope tightens; in a scope-fixed project, where contingency time and money come from. Pre-deciding removes a painful debate from a high-pressure moment and ensures the response protects the binding constraint rather than sacrificing it. The pre-decided response should be documented and known to the team and sponsor.

### Align Stakeholder Expectations to the Constraint Model

Stakeholders must understand which dimension is fixed and which flexes, or they will pressure the wrong thing. A sponsor who believes a timeboxed project will deliver all scope will be angry when scope is cut to protect the date, even if that was always the plan. Communicate the constraint model explicitly at kickoff and reinforce it at every review, so that flexing the agreed dimension is recognized as correct execution rather than as failure or scope creep.

## Common Traps

### Applying a Generic Lifecycle Regardless of Constraint

Reaching for a standard predictive or agile approach without considering the binding constraint produces a plan that fights reality. The trap is that the familiar methodology feels safe and professional. Match the delivery approach to the dominant constraint instead of defaulting to habit.

### Leaving the Flex Dimension Ambiguous

When no one states which constraint can move, all of them come under pressure and the weakest, often quality or team health, erodes silently. The trap is that ambiguity avoids a hard conversation now. Make the flex dimension explicit and communicated.

### Slow Decisions on the Binding Constraint

The decisions that most affect the binding constraint get routed through slow, generic approval paths, so the constraint drifts while waiting. The trap is treating all decisions as equally routine. Fast-track decisions that touch the binding constraint to the authority that owns it.

### Static Cadence on a Volatile Constraint

A fixed review cadence inherited from a template under-controls a project whose binding constraint changes quickly. The trap is that the cadence looks disciplined. Match control frequency to how fast the binding constraint's context actually moves.

### Reporting Task Progress While the Constraint Drifts

Status shows tasks moving and the project looks healthy, while the binding constraint, the date, budget, or scope, is quietly slipping. The trap is that activity is mistaken for success. Make the binding constraint the primary health signal.

### Improvising the Response Under Pressure

When the binding constraint is threatened, the team argues about what to cut or flex in the moment, losing time and often sacrificing the wrong thing. The trap is that improvisation feels responsive. Pre-decide the response so activation is fast and correct.

### Misaligned Stakeholder Expectations

Stakeholders expect all constraints to hold and react to flexing as if it were failure, even when flexing the agreed dimension is correct execution. The trap is that the constraint model was never communicated. Set and reinforce expectations about what is fixed and what flexes from the start.

## Self-Check

- [ ] Does the chosen delivery approach match the binding constraint, rather than defaulting to a generic lifecycle?
- [ ] Is the fixed dimension and the flex dimension stated explicitly in the planning artifacts?
- [ ] Are decisions touching the binding constraint routed on a fast path to the owning authority?
- [ ] Does the planning and control cadence match how volatile the binding constraint's context is?
- [ ] Is work sequenced and are reserves held to protect the binding constraint first?
- [ ] Is project health reported primarily in terms of the binding constraint's status, not just task progress?
- [ ] Is there a pre-decided, documented response for when the binding constraint is threatened?
- [ ] Have stakeholder expectations about what is fixed and what flexes been set at kickoff and reinforced regularly?
- [ ] Has the plan been checked to ensure it is not fighting the binding constraint?
- [ ] In a multi-constraint project, is the secondary constraint's flex also defined so it does not erode silently?
