---
name: process-experiment-and-pilot-design.md
description: Use when the agent is designing an operational experiment, pilot, trial, controlled rollout, A/B-style process comparison, shadow test, or limited-scope process change before broader adoption.
---

# Process Experiment And Pilot Design

A pilot is useful only if it can teach the organization something before risk is scaled. Many pilots are really soft launches with no learning criteria, no comparison, and no decision rule. Agents often recommend a pilot to sound cautious while leaving scope, measures, and expansion criteria vague. This skill helps the agent design operational experiments that produce actionable evidence and protect live service.

## Core Rules

### Define the decision the pilot must inform

Start with the decision to be made after the pilot: expand, redesign, abandon, delay, narrow scope, add controls, change staffing, or gather more evidence. If no decision could change based on the pilot result, the pilot is not an experiment; it is a staged rollout.

State the key assumptions being tested. Examples include whether a new triage rule reduces rework, whether staff can complete a new approval step within target time, whether customers understand a new message, whether a vendor integration produces reliable data, or whether quality holds when a queue is automated.

### Choose scope that tests the risk

Pilot scope should be large enough to reveal the important risk but small enough to contain harm. Choose site, shift, team, case type, customer segment, volume band, vendor, or time window deliberately. Do not choose the friendliest team if the goal is to learn how normal operations will perform.

Include representative complexity where possible: common cases, edge cases, high-volume periods, new staff, handoffs, and downstream reporting. A pilot that tests only ideal conditions can create false confidence.

### Establish baseline and comparison

Know what performance looked like before the pilot. Baseline measures may include cycle time, defect rate, backlog age, rework, escalations, customer contacts, staff effort, quality review, cost, and control exceptions. If a formal control group is not feasible, use historical comparison and explain limitations.

Avoid declaring success based only on activity completion. The question is not whether the pilot happened; it is whether the change improved the intended outcome without unacceptable side effects.

### Define success, failure, and pause criteria

Set criteria before the pilot starts. Success criteria should include target outcomes and guardrails. For example, cycle time may improve but quality cannot fall below an agreed threshold. Failure or pause criteria may include defect spikes, customer harm, control breach, staff overload, unreconciled records, or system instability.

Criteria protect decision quality. Without them, teams may interpret the same result based on preference, sunk cost, or pressure to launch.

### Prepare operational support and containment

Even limited pilots affect real work. Define training, supervisor coverage, help channel, quality sampling, customer messaging, exception routing, rollback, and data collection. Name who can pause the pilot and how affected cases will be handled if the pilot stops.

Containment should include downstream teams. Reporting, billing, compliance, vendors, customer support, and quality review may need to know which cases are in the pilot and how to interpret them.

### Capture qualitative learning, not just metrics

Metrics show what changed; observations help explain why. Capture staff questions, confusing steps, customer reactions, supervisor interventions, workarounds, exception themes, and downstream feedback. Use structured notes so learning can be compared across days or teams.

Do not rely on celebratory anecdotes from pilot champions. Seek evidence from skeptics, downstream recipients, and error cases. Their feedback often reveals what must be fixed before scaling.

### Decide before expanding

After the pilot, compare results with criteria and decide explicitly. Options include expand as designed, expand with changes, run another test, limit scope, pause, or stop. Document what was learned, what changed in the process, what risks remain, and who owns scale-up.

Do not expand just because the pilot period ended. If evidence is inconclusive, say so and choose whether the cost of uncertainty is acceptable.

## Common Traps

- Using "pilot" as a safer word for launch. Without learning criteria and decision rules, a pilot does not reduce uncertainty.
- Choosing only ideal participants. This may prove that experts can make the change work, not that operations can.
- Measuring only speed. Quality, control, staff load, customer experience, and downstream impact may worsen.
- Starting without baseline. Without comparison, improvement claims become subjective.
- Ignoring pilot contamination. Staff may mix old and new methods, making results hard to interpret.
- Expanding despite failed guardrails. A benefit is not enough if the change creates unacceptable harm.
- Forgetting to update training and standard work based on pilot learning.

## Self-Check

- Is the decision after the pilot clear, and could the result change that decision?
- Are the assumptions being tested explicit?
- Is the pilot scope representative enough to test risk while small enough to contain harm?
- Are baseline, comparison method, limitations, and data collection plan defined?
- Are success, failure, pause, guardrail, and expansion criteria set before launch?
- Are training, support, quality sampling, exception routing, customer messaging, rollback, and downstream awareness prepared?
- Will qualitative learning from staff, customers, skeptics, exceptions, and downstream teams be captured?
- Is there an explicit decision step before expansion, with documented learning and remaining risk?
