---
name: cpm-schedule-development-and-critical-path.md
description: Use when the agent is building a Critical Path Method construction schedule, defining activities and logic ties (FS SS FF SF), computing float and the critical path, setting durations and constraints, establishing a contract baseline, or preparing a Gantt and schedule specification under AIA, EJCDC, or agency scheduling requirements.
---

# CPM Schedule Development and Critical Path

Critical Path Method (CPM) scheduling is the planning logic that defines how a construction project is sequenced, how long it will take, which activities govern the completion date, and where float exists for flexibility. It is governed by the contract's scheduling specification (typically modeled on AIA A201, EJCDC, or federal agency formats such as the USACE EM 1110-1-12 and NAVFAC guidance), by AACE International and PMI recommended practice, and by the software mechanics of tools such as Primavera P6 and Microsoft Project. The harm this skill prevents is a schedule that misrepresents the work and misleads the owner, the contractor, and the surety: a baseline that hides long-lead procurement behind optimistic logic, a critical path computed on open-ended or redundant ties, a finish-to-start default that fabricates dependencies, or a constraint that overrides logic and conceals the true float. Because the CPM schedule becomes the contract time baseline and the yardstick against which delays, extensions, and liquidated damages are measured, a schedule built casually at the start becomes a liability instrument at the end, when every logic tie and constraint is re-examined in a delay claim. Agents must treat schedule development as a disciplined modeling exercise that supports a project manager or scheduler of record, and must never present a baseline as final without activity definition, logic validation, and a documented critical path.

## Core Rules

### Define Activities at a Consistent and Meaningful Level of Detail

The activity breakdown determines whether the schedule is usable for control or merely decorative. Decompose the work into activities that correspond to discrete, measurable, and assignable work packages, each with a responsible party, a defined deliverable, and a duration that can be tracked, typically between one and thirty days for detailed schedules and avoiding activities so long or so aggregated that progress cannot be measured. Maintain a consistent level of detail across disciplines, because a schedule where structural steel is one activity and electrical rough-in is fifty activities distorts the critical path and makes progress reporting unreliable. Use a work breakdown structure and activity coding (discipline, area, phase, responsible subcontractor) so that the schedule can be filtered, summarized, and audited, and ensure every activity has a clear scope basis so that a reviewer can confirm the activity represents real work and not a placeholder.

### Build Logic with the Correct Relationship Type and No Redundant or Open Ends

The four relationship types, finish-to-start (FS), start-to-start (SS), finish-to-finish (FF), and start-to-finish (SF), each represent a different physical or contractual dependency, and the wrong type fabricates or breaks a sequence. Use FS where one activity must finish before the next can start (concrete cure before form strip), SS with a lag where work can start after a predecessor has progressed (masonry starting after framing is underway), and FF where two activities must finish together (testing finishing after installation), and reserve SF for rare cases such as succession of maintenance or shift handover. Add lags only where they represent real physical conditions (cure time, delivery lead time, dewatering) and never as a substitute for a missing activity, because a lag is not an activity, cannot be tracked, and obscures what is actually happening. Validate the network by eliminating open-ended activities (every activity except the project start and finish should have at least one predecessor and one successor), removing redundant ties that add no logic, and confirming there are no logic loops, because an open end or a loop corrupts the forward and backward pass and produces a meaningless critical path.

### Compute the Critical Path and Float Correctly and Interpret Them Honestly

The critical path is the longest continuous sequence of activities through the network that determines the project's earliest completion, and it is identified by the zero-total-float (or least-float) path after the forward and backward pass. Total float is the amount of time an activity can slip without delaying the project completion, and free float is the amount it can slip without delaying any immediate successor; both are computed, not chosen, and both depend entirely on the logic and durations entered. Interpret float honestly: a large total float means flexibility on that path, but it does not mean the work is unimportant, and a near-critical path with one or two days of float can become critical with minor slippage and must be managed as if critical. Do not artificially lengthen durations or add contingency activities to create float, because that hides the true critical path and misrepresents the contract time, and do not collapse the schedule by deleting float-bearing activities to make it look tighter, because that removes the buffer the project actually has.

### Set Durations, Constraints, and the Baseline with Discipline

Activity durations must be based on quantity of work divided by a realistic crew production rate, with allowance for weather, learning curve, and known interruptions, not on guesswork or on what the schedule needs to show to hit a date. Document the basis for each significant duration (the quantity, the crew, the assumed production, the work hours) so that a changed duration can be justified or challenged later. Use constraints (must-start-by, must-finish-by, as late as possible) sparingly and only where a real external date exists, such as a regulatory milestone, a notice to proceed, or a contracted interim completion, because a hard constraint overrides the CPM logic and can hide the true critical path or create negative float that is not real delay. Establish the baseline schedule as the contract reference only after the logic, durations, and constraints have been validated, obtain owner approval per the specification, and freeze it; thereafter, all change is captured through schedule updates and fragnets, not by silent edits to the baseline.

### Update, Communicate, and Tie the Schedule to Cost and Progress

A baseline schedule that is never updated is a compliance artifact, not a management tool. Update the schedule at the interval required by the specification (monthly is typical) using actual start and finish dates, remaining durations based on field assessment, and percent complete tied to installed quantities, and recalculate the critical path and float to reflect the current state. Reconcile the schedule progress with the schedule of values and earned value or unit-cost reporting, because a schedule that shows work complete while the pay application lags, or vice versa, signals a measurement or a progress claim problem. Communicate the updated critical path, the near-critical paths, and the lookahead (typically a three- to six-week window) to the project team, and use the schedule to drive procurement, subcontractor coordination, and work-face planning, so that the CPM is the operating logic of the job rather than a document produced only for the owner.

## Common Traps

### Default Finish-to-Start Logic Everywhere

The scheduler accepts the software's default FS relationship for every tie, so activities that could overlap (framing and rough-in, paving and striping) are forced sequential. The mechanism is that FS is the easy default, so the false signal of a fully linked network hides fabricated dependencies; the harm is an artificially long schedule that the contractor cannot meet and that the owner may reject or that conceals real concurrency.

### Lags Used as Hidden Activities

A ten-day lag is inserted to represent cure time, delivery, or an undefined gap, so the lag carries duration without scope, responsibility, or progress measurement. The mechanism is that lags are not activities, so the false signal of a complete logic network hides untracked time; the harm is that the lag cannot be updated or claimed against, and the real work it stands for is invisible to progress and cost control.

### Constraints That Override Logic and Hide the Critical Path

A must-finish-by constraint is applied to force a milestone date, so the CPM logic is overridden and the calculated critical path no longer reflects the real longest path. The mechanism is that the constraint dominates the backward pass, so the false signal of an on-date schedule hides negative or zero float that is artificial; the harm is that genuine delay is masked and the schedule becomes indefensible in a time-extension dispute.

### The Open-Ended or Redundant Tie Network

Activities are left without successors, or redundant ties duplicate logic, so the forward and backward pass produce a critical path that is an artifact of the broken network. The mechanism is that open ends terminate paths early, so the false signal of a computed critical path hides the real longest sequence; the harm is a schedule that reports float and a completion date that have no relationship to the actual work plan.

## Self-Check

- Are activities defined at a consistent, measurable level of detail with a responsible party, deliverable, and duration basis, and coded by discipline, area, and phase?
- Are relationship types (FS, SS, FF, SF) chosen to match real physical or contractual dependencies, with lags used only for genuine conditions and never as missing activities?
- Is the network free of open ends, redundant ties, and logic loops, with every activity (except project start and finish) having both a predecessor and a successor?
- Is the critical path identified as the zero- or least-total-float path, with near-critical paths flagged, and is float reported honestly without artificial padding?
- Are durations based on quantity and crew production rates with documented basis, and are constraints limited to real external dates rather than used to force the schedule to a desired finish?
- Is the baseline schedule validated, approved, and frozen, with all subsequent change captured through updates and fragnets rather than silent edits?
- Are updates performed at the specified interval with actual dates and remaining durations, reconciled to the schedule of values, and used to drive a lookahead and work-face planning?
