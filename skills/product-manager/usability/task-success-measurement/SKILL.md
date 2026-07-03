---
name: task_success_measurement.md
description: Use when the agent is defining task success, measuring whether users can complete tasks, setting usability metrics, interpreting completion rates and time-on-task, or deciding what counts as success versus failure in a user flow.
---

# Task Success Measurement

Task success is the most direct measure of usability: can users actually do what the product exists to let them do? Despite its importance, it is consistently measured badly. Teams define success so loosely that almost any outcome counts, or so strictly that real users are declared failures for trivial deviations. They measure completion without measuring the friction it cost, or they measure time-on-task without accounting for differences in task complexity. The result is metrics that look rigorous but mislead, leading to confident decisions about a design's quality that the data does not actually support.

Good task success measurement starts with a precise definition of what success means for a specific task, captures not only whether users succeeded but how much it cost them, and interprets the results with honest attention to context. A product manager who measures task success well can tell whether a design actually works; one who measures it poorly gets numbers that confirm whatever they already believed.

Use this skill before defining success criteria for a usability study, before measuring completion in a flow, before reporting task success to stakeholders, or when interpreting whether a completion rate is good or bad. Ask: what exactly counts as success for this task, and is that definition defensible? Am I measuring only whether users finished, or also the effort and errors it cost them? Am I comparing like with like, or confusing task complexity differences with design quality differences?

## Core Rules

### Define Success Precisely And Defensibly

The definition of task success determines everything that follows, and loose definitions produce meaningless metrics. "User completed the task" is not precise enough. Did they complete it correctly, or just reach the end screen regardless of errors? Did they complete it within a reasonable effort, or after extensive struggle and assistance? Did they complete the intended task, or a different task that happened to end at the same screen?

For each task, specify the success criteria in observable terms before the test. What exact outcome constitutes success? What deviations count as partial success? What counts as failure? Write the criteria down so that scoring is consistent across participants and not influenced by whether you want the design to look good. A defensible definition is one that a skeptic could apply and reach the same score, which means it cannot depend on subjective judgment of whether the user "basically" succeeded.

### Measure Effort And Errors, Not Only Completion

Completion rate alone is a blunt instrument. Two designs can have identical completion rates while one lets users finish smoothly and the other makes them struggle, backtrack, and nearly give up. A design that users complete only through painful effort is not actually usable, even if the completion metric looks fine.

Capture the cost of completion alongside the fact of it. Useful measures include time-on-task, number of errors, number of detours or backtracks, whether the user asked for help, and the user's own rating of difficulty. These reveal friction that completion rate hides. A high completion rate with high effort and many errors signals a design that works technically but frustrates users, which predicts abandonment in the real world even if the lab metric looks acceptable.

### Distinguish Successful Completion From Assisted Completion

In usability testing, participants sometimes complete a task only because the moderator hinted, reminded, or rescued them. Counting these as successes inflates the completion rate and hides the very problems the test is meant to find. A user who needed a hint to find the button did not actually succeed at the task as they would encounter it in the wild.

Define how assistance is handled. Unassisted success, where the user completes the task without any prompt or hint, is the metric that predicts real-world performance. Assisted completion is useful information about where users got stuck, but it should not be counted in the success rate. Track them separately so that the completion rate reflects what users can actually do alone, and the assistance data reveals where the design fails to support them.

### Account For Task Complexity When Comparing

Completion rates and time-on-task are only meaningful relative to the task's complexity. A ninety percent completion rate on a simple two-step task may indicate a serious problem, while a sixty percent rate on a complex multi-stage task with many decision points may be acceptable or even good. Comparing raw numbers across tasks of different complexity, or across designs that changed the task structure, leads to false conclusions.

When comparing, normalize for complexity. Compare the same task across design iterations, not different tasks. If comparing different tasks, account for the number of steps, decision points, and dependencies. If a redesign changed the task itself, do not treat before-and-after completion rates as a clean comparison, because the task is no longer the same. State the complexity context alongside any completion number so that readers interpret it correctly.

### Separate First-Time Success From Learned Success

Many tasks get easier with repetition. A task that is hard on first attempt may become smooth once the user learns the interface, which is acceptable for a tool used daily but unacceptable for a one-time flow like onboarding or checkout. The acceptable difficulty depends on how often users encounter the task and how much learning they will invest.

Match the measurement to the usage pattern. For one-time or infrequent tasks, measure first-time success, because that is the only success that matters; users will not come back to learn. For frequent tasks, measure success after some exposure, but be honest about how much exposure is realistic before users give up. A design that requires extensive learning to become usable is making a bet on user investment; measure whether that bet pays off rather than assuming it will.

### Interpret Rates Against A Benchmark, Not In Isolation

A completion rate means little by itself. Eighty percent sounds good but may be catastrophic for a checkout flow where industry benchmarks are above ninety-five percent, and it may be excellent for a complex configuration task where users typically struggle. Without a reference point, the number invites motivated interpretation: defenders call it good, critics call it bad, and neither has a basis.

Establish a benchmark. It can be a prior version of the design, an industry standard, a target set based on business impact, or the rate below which the task fails its purpose. Report completion rates against the benchmark and explain the gap. "Eighty-two percent completion, below our ninety percent target and below the prior design's ninety-one percent" is actionable; "eighty-two percent completion" is not.

### Connect Task Success To Business And User Outcomes

Task success is a proxy for the outcomes that actually matter: does the user achieve their goal, and does the business achieve its goal? A user can complete a task in the study and still not adopt the feature in the wild, because the task was contrived or the motivation was absent. Connect the usability metric to behavioral and business metrics so that improvements in task success translate into improvements in adoption, retention, or revenue.

When task success rises but the business metric does not, investigate the gap. The study task may not represent real usage, the motivation may have been artificial, or there may be friction elsewhere in the journey that cancels the improvement. Treating task success as the end goal, rather than as a leading indicator of real outcomes, leads to locally optimized designs that do not move the metrics that matter.

## Common Traps

### Loose Success Definitions

Counting any task completion as success, regardless of errors or assistance, inflates the rate. The trap is a metric that looks good while hiding real friction.

### Completion Without Effort

Reporting only whether users finished, not how much it cost them, misses designs that work technically but frustrate. The trap is declaring a design usable when users only succeeded through painful effort.

### Counting Assisted Success

Including moderator-hinted completions hides the problems the test should find. The trap is a completion rate that does not reflect real-world performance.

### Comparing Across Unequal Tasks

Raw completion rates across tasks of different complexity mislead. The trap is attributing design quality to numbers that actually reflect task difficulty.

### Ignoring Usage Frequency

Accepting learnable difficulty for a one-time task, or demanding first-time ease for a daily task, mismatches the measurement to reality. The trap is applying the wrong standard for how often the task occurs.

### Rates Without Benchmarks

A completion number in isolation invites motivated interpretation. The trap is defenders and critics both claiming the data supports them, with no shared reference point.

### Optimizing The Metric, Not The Outcome

Improving task success in the study without moving adoption or retention means the proxy disconnected from reality. The trap is treating the usability metric as the goal rather than as a leading indicator.

## Self-Check

- [ ] Success criteria for each task are defined in observable terms before the test and could be applied consistently by a skeptic.
- [ ] The measurement captures effort and errors, not only whether users reached the end.
- [ ] Assisted and unassisted completions are tracked separately, and the success rate reflects unassisted performance.
- [ ] Comparisons account for task complexity, and raw rates are not compared across unequal tasks.
- [ ] First-time success is measured for infrequent tasks, and learned success is measured only where realistic exposure exists.
- [ ] Completion rates are interpreted against a benchmark, prior version, or target, not in isolation.
- [ ] Task success is connected to behavioral and business outcomes, and gaps between the two are investigated.
- [ ] Partial success and failure modes are recorded, not only binary completion, so that the design can be improved.
- [ ] The definition of success reflects the user's actual goal, not merely reaching a screen.
- [ ] No completion rate is reported without the context of task complexity, assistance, and benchmark.
