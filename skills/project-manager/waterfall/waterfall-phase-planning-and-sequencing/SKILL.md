---
name: waterfall_phase_planning_and_sequencing.md
description: Use when the agent is planning a sequential predictive project, defining phase boundaries, setting entry and exit criteria between requirements design build test and deploy phases, or sequencing work for a waterfall delivery where phases complete before the next begins.
---

# Waterfall Phase Planning and Sequencing

Planning a predictive, sequential project looks deceptively like making a list of phases and putting them in order, but the real work is designing the structure that makes sequencing safe. In a waterfall lifecycle, a phase does not merely precede the next; it commits the next. Requirements lock the scope that design must satisfy, design commits the architecture that build must implement, build produces the code that test must verify. Each phase hands the next a set of assumptions and constraints that are expensive to reverse, so the quality of sequencing is measured by how cleanly each phase's outputs satisfy the next phase's entry needs. Poor sequencing produces rework that ripples backward through completed phases, the most expensive failure mode in predictive delivery.

The judgment problem is to define phase boundaries that are neither too rigid nor too porous, to set entry and exit criteria that genuinely protect downstream phases, and to sequence work so that dependencies are satisfied and feedback is captured before commitment. Agents tend to treat phases as calendar blocks rather than as logical commitments, to set entry and exit criteria as formalities, and to allow phases to overlap in ways that destroy the predictability the lifecycle is meant to provide. The discipline of waterfall is not its rigidity but its deliberate definition of what must be true before work proceeds.

## Core Rules

### Define Each Phase by Its Purpose and Deliverables, Not by Calendar

A phase is defined by what it must produce and what decision it enables, not by a date range. The requirements phase produces a baseline specification that enables design; the design phase produces an approved architecture that enables build; the build phase produces integrated code that enables test. State, for each phase, the deliverables that must exist at its end and the decision those deliverables support. When phases are defined by calendar alone, their completion becomes a matter of elapsed time rather than achieved definition, and downstream phases begin on incomplete foundations.

### Set Entry Criteria That Protect the Phase's Preconditions

Entry criteria define what must be true before a phase can begin. Build should not start until design is approved and stable; test should not start until build produces integrated, unit-tested code. Entry criteria exist to prevent a phase from beginning on inputs it cannot work with, which forces rework. Define entry criteria as concrete, checkable conditions, and enforce them. The temptation to start a phase early to save schedule almost always costs more in rework than it saves in calendar.

### Set Exit Criteria That Guarantee Downstream Readiness

Exit criteria define what must be true before a phase is declared complete. Requirements exit criteria should ensure the specification is complete, consistent, and approved; design exit criteria should ensure the architecture is feasible and approved; build exit criteria should ensure the code is integrated and meets quality bars. Exit criteria exist to guarantee that the next phase receives inputs it can rely on. Define them as concrete and checkable, and resist declaring a phase complete when its exit criteria are not genuinely met.

### Sequence to Satisfy Dependencies, Not to Fill the Calendar

The sequence of phases and activities within them should follow logical dependency: you cannot design what is not specified, cannot build what is not designed, cannot test what is not built. Within this logic, sequence work to minimize the critical path and to bring high-risk activities forward where possible. Do not sequence activities to keep people busy or to match a desired end date; sequence them so that each activity has the inputs it needs when it starts. A schedule built on wishful sequencing collapses when dependencies assert themselves.

### Manage the Baseline That Phases Commit To

Each completed phase commits a baseline: requirements baseline, design baseline, build baseline. These baselines are the reference against which later phases measure conformance and against which change is controlled. Define what constitutes each baseline, who approves it, and how changes to it are handled. Without disciplined baselines, phases drift, scope leaks, and the predictive promise of the lifecycle, that what was specified is what is delivered, becomes unenforceable.

### Allow Overlap Only Where It Is Genuinely Safe

Pure waterfall is fully sequential, but in practice some overlap is often used to compress schedule: design may begin before all requirements are signed off, or build may begin on approved portions of design. Overlap is legitimate only where the overlapped work is genuinely stable and low-risk, and where the cost of rework if the upstream changes is acceptable. Define explicitly which overlaps are allowed, on what stable subsets, and with what rework risk acknowledged. Unmanaged overlap is not faster waterfall; it is waterfall losing its predictability.

### Build Feedback Loops That Do Not Violate Sequencing

Sequential delivery does not mean no feedback; it means feedback is structured. Reviews at the end of each phase capture feedback that shapes the next phase. Early prototypes or proofs of concept within a phase can reduce risk before commitment. Verification activities (walkthroughs, inspections) within a phase catch defects before they cross the phase boundary. Design feedback loops that improve quality without collapsing the sequence into chaos. The goal is disciplined feedback, not the absence of it.

### Plan the Critical Path and Protect It

In a sequential project, the critical path, the chain of dependent activities that determines the project duration, must be explicitly identified, monitored, and protected. Slippage on the critical path slips the whole project; slippage off it has float. Identify the critical path, put your best resources and closest monitoring on it, and manage risks to it aggressively. A waterfall project that does not know its critical path cannot protect its schedule.

## Common Traps

### Phases Defined by Calendar, Not by Deliverables

A phase is declared complete because its time is up, not because its deliverables are done, and the next phase starts on incomplete inputs. The trap is equating elapsed time with completion. Define phases by their deliverables and exit criteria.

### Entry and Exit Criteria as Formalities

Criteria exist on paper but are not genuinely enforced, so phases begin and end on shaky foundations. The trap is ceremony without discipline. Make criteria concrete, checkable, and enforced.

### Starting Early to Save Schedule

A downstream phase is started before its upstream inputs are ready, to appear faster, and the rework costs more than the calendar saved. The trap is mistaking early start for progress. Respect entry criteria.

### Unmanaged Phase Overlap

Phases overlap broadly to compress schedule, but the overlapped work is not stable, so rework ripples backward. The trap is overlap without risk acknowledgment. Allow overlap only on genuinely stable subsets.

### No Disciplined Baselines

Phases complete without defining what baseline they committed, so later phases and change control have no reference. The trap is that conformance becomes unmeasurable. Define and control baselines explicitly.

### Sequencing to Please Rather Than to Satisfy Dependencies

Activities are sequenced to hit a desired date or to keep people busy, ignoring dependencies, and the schedule collapses when dependencies assert themselves. The trap is wishful sequencing. Sequence to satisfy logical dependencies.

### Ignoring the Critical Path

The project is managed activity by activity without identifying which chain determines duration, so slippage on the critical path goes unprotected. The trap is losing schedule without understanding why. Identify and protect the critical path.

### Treating Sequential as Feedback-Free

The team assumes waterfall means no feedback until the end, so defects and risks accumulate and surface late when they are expensive. The trap is conflating sequence with the absence of inspection. Build structured feedback within and between phases.

## Self-Check

- [ ] Is each phase defined by its required deliverables and the decision it enables, rather than by a calendar block?
- [ ] Are entry criteria for each phase concrete, checkable, and enforced to protect downstream inputs?
- [ ] Are exit criteria for each phase concrete, checkable, and genuinely met before the phase is declared complete?
- [ ] Is the sequence of phases and activities driven by logical dependencies rather than by desired dates or resource utilization?
- [ ] Are baselines (requirements, design, build) explicitly defined, approved, and change-controlled?
- [ ] Where phases overlap, is the overlap limited to genuinely stable subsets with acknowledged rework risk?
- [ ] Are structured feedback loops (reviews, prototypes, inspections) built into phases without violating sequencing?
- [ ] Has the critical path been explicitly identified, resourced, and actively protected against slippage?
- [ ] Are entry and exit criteria treated as real gates rather than paperwork formalities?
- [ ] Does the plan resist starting downstream phases early when upstream outputs are not genuinely ready?
