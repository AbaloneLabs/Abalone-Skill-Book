---
name: mvp_validation_and_learning_loop.md
description: Use when the agent is planning how to validate an MVP, defining what evidence will confirm or refute the core assumption, deciding when to iterate versus kill, or closing the loop between build and learning.
---

# MVP Validation And Learning Loop

Building the MVP is half the work; the other half is capturing learning from it and acting on that learning. An MVP without a validation plan produces anecdotes, not evidence, and an MVP whose results never feed back into decisions is just a slow way of shipping a feature.

The judgment problem is that teams treat launch as the finish line. They build, they release, they watch a dashboard go up, and they declare success, without ever defining what evidence would have refuted the core assumption or what they would do if the evidence were ambiguous. Agents tend to instrument vanity metrics that cannot falsify the hypothesis, to collect feedback that is polite rather than honest, and to interpret every result as a reason to continue because killing feels like failure. The harm is MVPs that consume months and resolve nothing, zombie projects that limp on without a kill criterion, and teams that "learned a lot" but cannot point to a single belief that changed.

Use this skill when planning how to validate an MVP, when defining what counts as evidence, and when deciding whether to iterate, pivot, or kill. The goal is to close the loop between build and learning so that each MVP either confirms a belief, refutes it, or generates a sharper question for the next cycle.

## Core Rules

### Define Success And Failure Criteria Before Launch

Validation requires criteria stated before the data exists, so that interpretation is not shaped by preference. Define what evidence will confirm the core assumption, what will refute it, and what the threshold is for each. Without pre-committed criteria, every result gets read as encouragement.

Write the criteria next to the learning goal, with specific thresholds. "Success is at least 40 percent of activated users returning weekly for three weeks; failure is below 15 percent; the band in between triggers a defined iteration." The thresholds turn a vague hope into a decision rule and prevent the team from moving the goalposts once numbers arrive.

### Choose Evidence That Can Falsify The Assumption

The evidence you collect must be capable of proving the assumption wrong, not just capable of looking supportive. Vanity metrics such as signups, page views, and downloads cannot falsify a hypothesis about value or retention, because they go up regardless of whether the core value is real. Choose behavioral evidence tied to the core value proposition.

Prefer revealed behavior over stated intent. What users do in the product, whether they return, complete the core task, pay, or invite others, is far more trustworthy than what they say in a survey. Stated intent is useful for understanding why behavior is what it is, but it should never be the primary evidence for a ship or kill decision.

### Instrument The Signal Before You Need It

The validation metric must be instrumented before launch, not retrofitted after the team notices the dashboard lacks it. Building first and instrumenting later guarantees gaps in the evidence and invites post-hoc metric shopping when the pre-planned signal disappoints.

Map each learning question to the specific event or behavior that answers it, and confirm the instrumentation captures it before the first user arrives. If a question cannot be answered with the planned instrumentation, either add the instrumentation or change the question. Do not launch with a validation plan the data cannot actually support.

### Separate Signal From Noise And Seasonality

MVP populations are small and volatile, and a single cohort or a single week can dominate the result. Distinguishing real signal from noise requires enough time, enough users, and awareness of external factors such as holidays, press, or a launch-day spike that does not represent steady state.

Look at trends over time, not a single snapshot, and compare against a baseline or control where possible. Beware of survivorship in the data, where the users who remain are the ones who liked it, biasing the picture. Report the confidence interval or the range, not just the headline number, and be honest about how much uncertainty remains.

### Close The Loop With A Decision, Not A Report

The point of validation is to change what the team does next. After the evidence is in, the loop closes only when a decision is made: iterate, pivot, kill, or invest. A beautiful readout that changes no decisions is theater. Tie every validation cycle to an explicit next action and an owner.

Schedule the decision review at launch, not after the data arrives, so that the team is committed to deciding rather than deferring. State the decision and the evidence together, and record what belief changed. If no belief changed, the MVP did not produce learning, regardless of how much data was collected.

### Plan The Iterate-Versus-Kill Boundary Upfront

Before launch, define what result leads to iterating, what leads to killing, and what leads to pivoting. The ambiguity band between clear success and clear failure is where most MVPs actually land, and without a rule for that band, the team defaults to continuing because stopping feels like failure.

Write the rule for the ambiguous middle. "If the metric lands between 15 and 40 percent, we run two more weeks focused on onboarding before deciding." Pre-commitment prevents the team from endlessly extending an MVP that is neither clearly working nor clearly broken, which is how zombie projects are born.

### Capture Qualitative Signal To Explain The Quantitative

Numbers tell you what is happening; qualitative signal tells you why. Pair every quantitative validation with a small number of deep conversations with real users, to understand the mechanism behind the behavior. A retention rate is more actionable when you know why users stayed or left.

Resist the urge to substitute a survey for a conversation at the validation stage. Surveys are good for breadth; interviews are good for depth, and depth is what an MVP needs to decide what to build next. Use both, but do not let the cheaper one crowd out the more informative one.

## Common Traps

### Launch And Watch The Dashboard

Treating launch as validation, with no pre-defined criteria, means any upward movement is read as success. The trap is that the team feels data-driven while actually having no falsifiable test. Define success and failure thresholds before launch.

### Tracking Vanity Metrics That Cannot Falsify The Hypothesis

Signups and page views go up regardless of whether the core value is real. The trap is reporting growth in a metric that cannot possibly refute the assumption, then claiming validation. Choose behavioral evidence tied to the core value.

### Trusting Stated Intent Over Revealed Behavior

Users say they would use and pay for things they do not actually use or pay for. The trap is building on survey optimism and discovering the gap only after launch. Lead with behavior; use stated intent only to explain it.

### Moving The Goalposts After The Data Arrives

When the pre-planned metric disappoints, teams often find a more favorable metric that did move. The trap is that the MVP appears to succeed while actually failing its real test. Honor the pre-committed criteria, and label any new metric as a hypothesis for the next cycle.

### Defaulting To Continue In The Ambiguous Middle

Most MVPs land in a band that is neither clearly successful nor clearly failed. The trap is extending indefinitely because killing feels like failure. Pre-commit an iterate-versus-kill rule for the ambiguous middle.

### Reading A Small Volatile Sample As Signal

Early MVP populations are tiny and noisy, and a launch spike can look like traction. The trap is over-interpreting a volatile few days. Look at trends, ranges, and baselines, and be honest about residual uncertainty.

### Producing A Readout That Changes No Decisions

A validation report that does not alter what the team builds next is theater. The trap is treating the readout as the deliverable rather than the decision. Tie every cycle to an explicit next action and owner.

## Self-Check

- [ ] Success and failure criteria with specific thresholds were written before launch, not after the data arrived.
- [ ] The primary evidence is behavioral and capable of falsifying the core assumption, not vanity metrics or stated intent.
- [ ] Every learning question maps to an instrumented event that was verified before the first user arrived.
- [ ] Results are read as trends and ranges against a baseline, not as a single volatile snapshot.
- [ ] The validation cycle closes with an explicit decision to iterate, pivot, kill, or invest, with an owner.
- [ ] An iterate-versus-kill rule for the ambiguous middle band was pre-committed before launch.
- [ ] Qualitative conversations are paired with quantitative data to explain the mechanism behind the behavior.
- [ ] No goalpost was moved to a more favorable metric after the pre-planned signal disappointed, without labeling it as a new hypothesis.
- [ ] The team can point to at least one belief that changed as a result of this MVP, or honestly state that none did.
