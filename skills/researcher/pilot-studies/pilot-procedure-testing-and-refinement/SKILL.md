---
name: pilot_procedure_testing_and_refinement.md
description: Use when the agent is planning or running a pilot to test study procedures, pretesting recruitment and consent flows, checking intervention delivery and fidelity, debugging data collection tools, mapping staff roles and handoffs, or documenting what broke during a small-scale run before a main study.
---

# Pilot Procedure Testing And Refinement

A pilot is most valuable when it breaks. Its central job is to expose the cracks in recruitment, consent, intervention delivery, measurement, data capture, and coordination before those cracks scale into a main study that cannot answer its question. Yet teams routinely run a pilot, collect outcome numbers, and never write down which procedure failed, why, or how it was fixed. The result is a main study that repeats the pilot's hidden failures at fifty times the cost, or a protocol that is "refined" based on impression rather than evidence.

The judgment problem here is that procedure testing feels unglamorous and is easy to under-prepare. Agents tend to over-focus on whether the outcome looked promising and under-focus on whether the machinery ran. The harm this skill prevents is a smooth-looking pilot that conceals recruitment bottlenecks, consent misunderstandings, drift in intervention delivery, broken instruments, and silent data loss, all of which surface too late in the main study. The agent has latitude in what to test and how, but must treat the failure log as the primary deliverable and must not let outcome data distract from process evidence.

This skill offers methodological guidance for planning and running pilots as procedure tests. It is not a substitute for statistical or methodological consultation, and a misused pilot, one that hides failures or overclaims outcomes, can mislead the main study and waste participants' contributions.

## Core Rules

### Treat Procedure Failure As The Deliverable, Not A Side Effect

The most useful output of a procedural pilot is a documented account of what did not work and how it was fixed or worked around. Reframe success accordingly. A pilot in which nothing broke probably did not look hard enough, or was run on a population and setting too easy to expose the real stresses. Plan the pilot to stress the procedures, record every failure, and treat an empty failure log as a warning sign rather than reassurance.

Define before launch what counts as a procedure failure, for example a participant who could not complete consent unaided, a form field left blank by more than a threshold of staff, a session that overran its allotted time, or a data upload that errored. Capture each failure with context, root cause, and the change made.

### Distinguish Procedure Testing From Hypothesis Testing

A pilot run to test procedures is not powered to test the intervention's effect, and its outcome data should not be reported as findings. Conflating the two corrupts both purposes. If the team wants outcome estimates, that is a different question with different sample size logic and different reporting obligations.

State explicitly that this pilot tests processes. Report process metrics, such as recruitment rate, consent comprehension, session length, fidelity scores, and data completeness, as the primary results. Report any outcome numbers only with wide uncertainty and clearly labeled as non-confirmatory planning inputs.

### Test The Full Procedural Chain End To End

A common error is testing each component in isolation and assuming the chain will hold. The chain is where most failures live. The pilot must exercise the complete path a real participant travels, from first contact through final data capture.

Map and test the chain explicitly:

- recruitment channels and the first-contact script;
- screening and eligibility determination;
- informed consent, including comprehension checks;
- enrollment and, where relevant, randomization or assignment;
- intervention delivery across all sessions and all delivery staff;
- data collection at each planned time point;
- data entry, transfer, storage, and quality checks;
- retention contacts and follow-up windows;
- adverse event or incident reporting paths.

Run the chain with realistic timing and realistic participants, not just cooperative staff or pilot-friendly volunteers.

### Pretest Recruitment And Consent As Real Procedures

Recruitment and consent are procedures, and they fail in characteristic ways. Recruitment underperforms because the message, channel, or eligibility screen does not match the real population. Consent fails because documents are too long, too technical, or misunderstood under time pressure.

Test recruitment by tracking reach, response, screen-fail, and enrollment rates at each step, producing a conversion funnel, not a single enrollment count. Test consent by adding comprehension checks and by observing where participants hesitate, ask questions, or sign without engaging. Record which wording caused confusion and revise it.

### Verify Intervention Delivery And Fidelity

An intervention that is delivered inconsistently cannot be evaluated in the main study. The pilot must show whether delivery staff can actually deliver it as intended, how much training and support they need, and how fidelity drifts across staff and over time.

Establish a fidelity check, such as observation, audio or video review, or a structured checklist, and score a sample of sessions. Record where staff improvised, skipped components, or substituted materials. Use this to set the training, supervision, and monitoring plan for the main study, not merely to assert that delivery is feasible.

### Debug Data Collection Tools Before Trusting Them

Instruments and platforms often look fine in the office and fail in the field. A measure may be too long, ambiguous, culturally mismatched, or produce floor or ceiling effects. A digital tool may fail on participants' devices, lose connectivity-dependent data, or allow inconsistent entry.

Test each tool by:

- timing completion and noting item-level missingness and skip confusion;
- checking for floor, ceiling, and reverse-scored item errors;
- confirming the data path from device to analysis dataset with no silent loss;
- testing edge cases such as partial saves, timeouts, and offline scenarios;
- capturing participant and staff feedback on burden and clarity.

Fix the tool, then re-test the revised version. Do not assume one pass is enough.

### Map Staff Roles, Handoffs, And Capacity

Many procedure failures are coordination failures. A task falls between two roles, a handoff drops information, or a single staff member becomes a bottleneck. The pilot should surface these.

Document who does each task, when, and what passes between roles. Identify single points of failure, tasks with no backup, and steps that depend on one person's availability. Estimate the staff time per enrolled participant and compare it to available capacity. Use this to right-size the team for the main study.

### Document Failures And Changes In A Structured Log

An unstructured recollection of "what we learned" is not enough. Use a structured change log that ties each failure to a specific protocol change, so reviewers and funders can see the evidence behind each revision.

For each entry record:

- the failure or problem observed, with the evidence;
- the procedure component affected;
- the suspected root cause;
- the change made to the protocol, training, or tool;
- whether the change needs re-testing before the main study.

This log is often more valuable than the outcome dataset and should be preserved as a primary study record.

### Keep Ethics And Consent Honest About Untested Procedures

A procedural pilot still involves human participants, and because procedures are untested, participants may bear extra burden or confusion. The pilot needs its own ethics review and consent that reflects genuine uncertainty.

Ensure the pilot has IRB or ethics approval, that consent describes the procedures as untested, that compensation covers any extra burden, and that there is a path to stop if a procedure causes harm or distress. Do not treat the pilot as exempt because it is small.

## Common Traps

### Reporting Pilot Outcomes As If They Were Findings

When a procedural pilot reports outcome estimates as significant or promising, it overstates a tiny sample and misleads readers. The trap is that outcome numbers feel more publishable than a failure log. Resist it; report process metrics as primary and label outcomes as non-confirmatory.

### Running The Pilot On A Friendly Subset

Pilot-friendly staff, highly motivated volunteers, or an easy site will not expose the recruitment, comprehension, and burden problems the main study will face. The trap is choosing convenient participants to avoid friction. Choose realistic participants and settings so the stress test is meaningful.

### Testing Components In Isolation

Each piece working alone does not prove the chain works. The trap is checking recruitment, consent, and data tools separately and declaring feasibility. Run the full end-to-end path with realistic timing, because most failures live at the handoffs.

### Skipping Fidelity Measurement

Without a fidelity check, the team cannot tell whether the intervention was delivered as intended, so the main study may evaluate a drift-laden version. The trap is assuming trained staff will deliver faithfully. Measure fidelity on a sample and act on what it shows.

### Trusting Data Tools After One Desk Check

A form that looks correct in the office can still lose data, confuse participants, or break on real devices. The trap is declaring the instrument validated after a single review. Test it in the field, check the data path end to end, and re-test after revisions.

### Not Recording What Broke

Verbal debriefs fade, and the same failure is repeated at scale. The trap is relying on memory or a casual lessons-learned note. Maintain a structured change log tied to evidence and protocol revisions.

### Hiding Failures To Protect Feasibility Claims

Teams sometimes soft-pedal recruitment shortfalls or consent confusion to keep a project funded or approved. The trap is framing every problem as manageable. Honest documentation protects the main study; concealed failures guarantee they recur larger.

### Treating The Pilot As Ethics-Exempt

Because it is small or process-focused, the pilot may be run without proper review or honest consent. The trap is assuming size removes ethical obligation. Untested procedures can burden or confuse participants, so the pilot needs its own review and transparent consent.

## Self-Check

- [ ] Is the pilot explicitly framed as a procedure test, with process metrics designated as primary and outcome data labeled non-confirmatory?
- [ ] Does the plan exercise the full end-to-end chain, from first recruitment contact through final data capture, with realistic timing and participants?
- [ ] Are recruitment and consent tested as real procedures, with a conversion funnel and comprehension checks rather than a single enrollment count?
- [ ] Is intervention fidelity measured on a sample of sessions, with drift and improvisation documented and used to set the training and monitoring plan?
- [ ] Have data collection tools been field-tested for burden, missingness, floor or ceiling effects, and a complete device-to-dataset data path, with re-testing after revisions?
- [ ] Are staff roles, handoffs, single points of failure, and per-participant staff time mapped against available capacity?
- [ ] Is there a structured failure and change log tying each observed problem to evidence, root cause, and a specific protocol change?
- [ ] Does the pilot have its own ethics review and consent that honestly reflect untested procedures and any extra burden?
- [ ] Are failures documented honestly rather than softened to preserve feasibility claims, and are any changes needing re-test flagged?
- [ ] For high-stakes studies, complex interventions, or uncertain procedure risks, has a methodologist or trial manager been consulted to review the procedure test plan and the change log before scaling to the main study?
