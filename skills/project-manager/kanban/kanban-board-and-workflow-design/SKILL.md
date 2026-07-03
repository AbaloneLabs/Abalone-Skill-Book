---
name: kanban_board_and_workflow_design.md
description: Use when the agent is designing a Kanban workflow, mapping the value stream steps, defining board columns and policies, setting up a pull-based board for continuous delivery, or deciding how work should flow from request to done in a flow-based system.
---

# Kanban Board and Workflow Design

A Kanban board is often mistaken for a to-do list arranged in columns, but its real purpose is to make the actual workflow visible so that flow can be managed. The board is a model of how work moves from request to delivered value, and the fidelity of that model determines whether the board illuminates the work or misrepresents it. When the columns reflect the real value stream, the bottlenecks, handoffs, and queues become visible and manageable; when the columns are generic (to do, doing, done), the board hides exactly the information needed to improve flow. Designing the workflow well is the prerequisite for everything else in Kanban, because WIP limits, pull policies, and metrics all derive from the workflow model. A board built on a wrong model produces precise-looking management of an imaginary process.

The judgment problem is to map the real value stream rather than an idealized one, to define columns and policies that make the work's actual state visible, and to design a pull system that exposes rather than conceals bottlenecks. Agents tend to copy a standard board template without examining the real workflow, to collapse distinct steps into generic columns, and to add policies that look thorough but are never enforced. The discipline is to let the work teach you the workflow, then model it honestly.

## Core Rules

### Map the Real Value Stream, Including Queues and Handoffs

The workflow begins with how work actually moves, not how it is supposed to move. Walk a work item from request to delivery and record every distinct state it passes through, including the waiting states (queues) and the transfer points (handoffs) where work stalls. Queues and handoffs are where most lead time accumulates, so a board that omits them hides the dominant source of delay. Map the value stream empirically: observe real items, talk to the people who do the work, and capture the steps as they are, including the unglamorous waiting. An idealized map will not match reality and will not be trusted.

### Make Columns Reflect Meaningful States, Not Activity Categories

Each column should represent a state the work is in that has a distinct meaning and a distinct next step. "In development" is a meaningful state; "in progress" applied to everything is not, because it conflates analysis, build, review, and test into one undifferentiated blob. Distinguish states that have different exit conditions and different owners. If two pieces of work in the same column need different things to happen next, they are probably in different states and deserve separate columns. Resist the urge to minimize columns for tidiness; the goal is visibility, not brevity.

### Define Explicit Entry and Exit Criteria for Each Column

For flow to be manageable, the conditions for entering and leaving each column must be explicit and shared. "Done with development" should mean the same thing to everyone: code complete, unit tests passing, reviewed, merged. Without explicit policies, the same column holds work at widely varying levels of completion, and flow becomes unpredictable. Write down the definition of each column's entry and exit, negotiate it with the team, and hold it. Policies that are not written and agreed drift into private interpretations.

### Design Pull, Not Push

Kanban is a pull system: new work is drawn into a column only when the downstream column has capacity, not pushed forward when upstream finishes. Pull is what exposes bottlenecks, because a blocked downstream column stops accepting work and the constraint becomes visible. Design the board so that movement is triggered by downstream capacity, governed by WIP limits, not by upstream completion. A board where work is pushed forward regardless of downstream capacity is a push system with Kanban columns, which conceals rather than reveals flow problems.

### Separate Work Types and Classes of Service Deliberately

Not all work is the same: features, defects, maintenance, and expedite items move through the workflow differently and deserve different handling. Distinguish work types on the board and consider classes of service (standard, fixed date, expedite) that carry different lead-time expectations and prioritization rules. Mixing all work types into one stream hides the fact that defects and features have different flow characteristics and different capacity needs. Make the distinctions visible so that capacity allocation and prioritization can be deliberate.

### Make Policies Visible and Enforced

The rules that govern the board, WIP limits, entry and exit criteria, pull triggers, class-of-service rules, must be visible on or near the board and consistently enforced. Policies that exist only in someone's head are not policies; they are habits that vary by person. When a policy is broken, address it explicitly: either the policy is wrong and should change, or the break is a violation that should be corrected. Silent policy drift destroys the predictability the board is meant to provide.

### Start With the Current Workflow and Evolve It

A common error is to design the ideal board on day one and force the work into it. The sounder approach is to model the workflow as it currently is, even if it is messy, and then evolve the board as the team learns. Start honest, measure flow, identify bottlenecks, and improve the workflow incrementally. A board imposed from an ideal will be resented and circumvented; a board that grows from the real work will be owned and trusted.

### Keep the Board at a Manageable Level of Detail

While columns should reflect meaningful states, the board should not become so granular that it is unmanageable. Every column adds overhead in updating and reading the board. Aim for the level of detail at which bottlenecks are visible and policies are enforceable, and resist adding columns for every micro-step. Sub-steps within a column can be tracked with tags or swimlanes rather than separate columns. Balance visibility against the cost of maintenance.

## Common Traps

### Copying a Standard Board Template

A generic to-do, doing, done board is copied without examining the real workflow, hiding queues and handoffs. The trap is mistaking a template for a model. Map the actual value stream.

### Collapsing Distinct States Into Generic Columns

Different states with different exit conditions are merged into one column, so the board hides the work's real state. The trap is tidiness over visibility. Split columns where states differ meaningfully.

### No Explicit Entry and Exit Policies

Column boundaries are undefined, so the same column holds work at varying completeness and flow is unpredictable. The trap is private interpretations of done. Write and agree explicit policies.

### Push Masquerading as Pull

Work is pushed forward when upstream finishes, ignoring downstream capacity, so bottlenecks are concealed. The trap is a push system in Kanban clothing. Design genuine pull governed by WIP limits.

### Mixing All Work Types Into One Stream

Features, defects, and expedite items flow through one undifferentiated stream, hiding their different characteristics. The trap is false uniformity. Separate work types and classes of service.

### Invisible or Unenforced Policies

Policies exist in someone's head and vary by person, so the board's rules are inconsistent. The trap is habits mistaken for policies. Make policies visible and enforce them.

### Imposing the Ideal Board on Day One

A designed-ideal board is forced onto work it does not match, and is circumvented. The trap is design without observation. Start with the real workflow and evolve.

### Over-Granular Boards

The board has a column for every micro-step, becoming unmanageable to update and read. The trap is detail without benefit. Keep the board at the level where bottlenecks are visible.

## Self-Check

- [ ] Was the value stream mapped empirically from real work items, including queues and handoffs where lead time accumulates?
- [ ] Does each column represent a meaningful state with a distinct exit condition and owner, rather than a generic activity category?
- [ ] Are entry and exit criteria for each column written down, agreed by the team, and consistently held?
- [ ] Is the board a genuine pull system where movement is triggered by downstream capacity, not by upstream completion?
- [ ] Are different work types and classes of service distinguished on the board rather than merged into one stream?
- [ ] Are the board's policies (WIP limits, definitions of done, pull triggers) visible and consistently enforced, with breaks addressed explicitly?
- [ ] Did the board start from the current real workflow and evolve, rather than being imposed as an ideal on day one?
- [ ] Is the board at a manageable level of detail, with bottlenecks visible without being over-granular?
- [ ] Do the columns match how work actually moves, verified by walking real items through the board?
- [ ] When policies are broken, is the response explicit (fix the policy or correct the violation) rather than silent drift?
