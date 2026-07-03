---
name: usability_testing.md
description: Use when the agent is planning a usability test, writing task scenarios, choosing between moderated and unmoderated testing, deciding how many participants to test, or interpreting usability findings to inform design changes.
---

# Usability Testing

Usability testing is where a design meets reality. Done well, it reveals where real people stumble, misread, and give up, exposing flaws that no internal review will catch. Done poorly, it produces a clean session that feels reassuring and proves nothing, or a pile of opinions that gets mistaken for evidence. The harm is false confidence: shipping a design because it "tested well" when the test never stressed the right tasks, the right users, or enough of them.

Use this skill before answering broad questions such as "how should we run a usability test", "how many participants do we need", "what tasks should we test", "should this be moderated or unmoderated", or "what did this usability test actually prove". The goal is to prevent the agent from writing leading task scenarios, from treating preference opinions as usability defects, from over- or under-sampling, and from drawing confident design conclusions from a single clean run.

## Core Rules

### Match The Method To The Question

Usability testing is not one method. Choose the format against the specific question you need answered. Moderated testing, where a facilitator sits with the participant, is best when you need to probe why someone is stuck, explore an early concept, or adapt tasks on the fly. It yields rich qualitative insight but is slower, costlier, and harder to scale. Unmoderated, asynchronous testing reaches more participants faster and cheaper, and captures more natural behavior without a facilitator present, but you cannot ask follow-up questions and you lose control over context and distraction.

Remote versus in-person is a separate axis. Remote broadens recruitment and tests people in their real environment, but you lose the ability to see their screen context, body language, or physical surroundings. In-person gives fuller observation but narrows who you can recruit and can feel more performative. Decide which tradeoffs matter for the risk you are evaluating, and do not default to whichever is cheapest or fastest without naming what you are giving up.

Be explicit about whether you are testing a prototype or production. Prototypes let you catch problems before build cost is incurred, but they hide performance, data, and integration realities. Production tests reveal real-world friction but come too late for cheap change. Match the fidelity of what you test to the decision the test must inform.

### Write Realistic Task Scenarios Without Leading Clues

The task scenario is the heart of a usability test, and it is where most tests go wrong. A good scenario describes a realistic goal and a believable situation, then lets the participant attempt it without being told where to click. A bad scenario hands the participant the answer: "Click the Export button in the top right to download your report" is a guided tour, not a test.

Write tasks in the participant's language and frame them around an outcome, not an interface path. "You need to share last month's summary with your manager by end of day. Show me how you'd do that" tests the real flow. "Go to Reports and use Export" only confirms that a labeled button can be found when you are told to find it. The difference is whether you are learning anything new.

Avoid clues embedded in wording. Naming the feature you are testing ("use the new quick-share tool"), using the exact label from the UI, or implying the expected path all contaminate the result. Also avoid tasks so vague that participants flounder without producing useful signal. The scenario should be specific enough to be attempted, open enough to reveal the path the user actually takes.

### Measure The Right Things

Usability is multidimensional, and a single metric hides the story. Decide in advance which dimensions matter for the decision: success rate (did they complete the task), time on task (how long it took), error rate and severity (where and how badly they went wrong), and satisfaction or perceived ease after the task. Each captures a different failure mode. A task can have a high success rate but take forever and leave users frustrated; a fast task can still produce serious errors.

Think-aloud protocol, where participants narrate their thought process, is a powerful lens on confusion, but it changes the task. Thinking aloud slows users down, can disrupt natural flow, and in unmoderated settings many participants forget to narrate. Treat think-aloud as a tool for surfacing problems, not as a clean performance measure. Where you need clean time-on-task data, consider a retrospective think-aloud or a separate silent condition.

Distinguish what you can quantify from what you can only observe. With enough participants you can estimate issue frequency and severity; with few participants you are surfacing problems, not measuring their prevalence. Do not present small-sample counts as if they were statistics.

### Use Sample Size Honestly

The often-cited "test with five users" rule comes from research on problem discovery: roughly five participants surface most of the major usability problems in an interface, with diminishing returns after that. It is a useful heuristic for finding problems, but it is widely misunderstood and misapplied.

The rule assumes a relatively homogeneous user group and a single interface; it does not hold when you have distinct segments with different workflows, when you are trying to measure quantitative performance, or when the stakes of a missed problem are high. If you need to compare two designs, estimate a success rate precisely, or cover several user types, five is not enough. Quantitative usability claims generally require dozens of participants, not five.

Be honest about what your sample size supports. A handful of sessions can reveal that problems exist and generate hypotheses for redesign; it cannot tell you the prevalence of those problems across your user base. Match the claim to the sample, and when stakes are high or segments differ, test more.

### Separate Usability Issues From Preference Opinions

Participants in a usability test offer two very different kinds of feedback, and conflating them corrupts the findings. Usability issues are observable: the participant could not complete the task, took a wrong path, made an error, or expressed confusion in the moment. These are evidence about the design. Preference opinions are what participants say they would like: "I'd prefer a darker theme", "this should be on the left". These are suggestions, not findings, and they are often contradicted by the participant's own successful behavior.

Record issues and preferences separately. Weight observed failures heavily, because they represent real friction the design caused. Treat preferences lightly and in aggregate; one person's aesthetic preference is not a defect. The trap is building whatever the loudest test participant suggested rather than fixing what the observations showed was actually broken.

Also separate the participant's reaction to the task from their reaction to the product. A participant may dislike a feature that they nonetheless used successfully; that is a satisfaction signal, not a usability defect. Conversely, a participant may praise a design they could not actually operate; the praise is social noise, the failure is the data.

### Avoid The False Confidence Of A Single Clean Session

One smooth session can convince a team the design is done. It should not. A single participant succeeding proves only that one person, on one day, could do it; it says nothing about edge cases, other segments, stress conditions, or the long tail of confusion. Equally, one disastrous session can panic a team into redesigning around a problem that may be idiosyncratic.

Read patterns across sessions, not verdicts from one. Track which problems recur across multiple participants, because recurrence is the signal that a problem is in the design rather than the person. Note the severity and the context of each failure, and watch for problems that cluster around the same screen, label, or step.

Resist the urge to declare victory early. If every session has been clean, ask whether the tasks were too easy, too leading, or whether you recruited people who already knew the interface. A test that finds nothing is often a test that was not stressful enough, not a design that is flawless.

### Synthesize Findings Into Prioritized Change

Raw usability findings are a list of problems; they are not yet a design plan. Synthesis means clustering related issues, judging severity by impact and frequency, and translating them into actionable changes. Severity should combine how many participants hit the problem, how badly it blocked them, and how critical the affected task is to the product.

Prioritize ruthlessly. Not every observed issue warrants a change; some are minor, some affect low-value flows, and some "fixes" would introduce new problems elsewhere. Frame findings as design problems to solve, not as literal instructions, so designers can address root causes rather than patch symptoms. Close the loop by noting which changes a finding drove, so the team can see the return on the testing effort.

## Common Traps

### The Guided Tour Disguised As A Test

When tasks name the buttons or describe the expected path, the test measures nothing but the participant's ability to follow instructions. The trap is a clean, successful session that feels like validation but reveals nothing about whether real users would find the path unaided. Write tasks around outcomes, not interface locations.

### Treating Opinions As Defects

Participants love to suggest changes, and teams love to act on concrete suggestions. The trap is treating "I wish this were blue" or "add a search here" as usability findings and rebuilding around them, when the real evidence was an observed failure the team ignored. Keep opinions and observed problems in separate columns.

### Over-Reading A Single Session

One memorable session, good or bad, can dominate the team's mental model of the design. The trap is generalizing one person's experience into a verdict. Force decisions to rest on recurring patterns across participants, not on the most vivid story.

### Misapplying The Five-User Rule

The five-user heuristic is for problem discovery in a homogeneous group, not for quantitative claims or multi-segment interfaces. The trap is declaring a design validated, or a success rate measured, after five sessions. Match the sample to the claim, and test more when segments differ or precision matters.

### Testing The Wrong Participants

Testing with colleagues, existing experts, or whoever was available produces findings that do not reflect the people who will actually use the product. The trap is confident conclusions about usability for a population you never sampled. Recruit to the real user segment, and screen for fit just as in any research.

### Confusing Prototype Success With Production Readiness

A prototype that tests well can still fail in production because of real data, latency, edge cases, and integration constraints that the prototype hid. The trap is treating a clean prototype test as proof the shipped feature will work. Re-test at appropriate fidelity and re-test critical flows on production where it matters.

### Stopping At Findings Without Prioritization

A long list of issues without severity or priority overwhelms the team and leads to either fixing trivia or fixing nothing. The trap is delivering raw observations instead of a prioritized, actionable synthesis. Cluster, rank by impact and frequency, and translate each finding into the design problem it implies.

## Self-Check

- [ ] The test method (moderated, unmoderated, remote, in-person) is chosen against the specific question, with tradeoffs named.
- [ ] Task scenarios describe realistic outcomes and situations without naming UI elements or implying the expected path.
- [ ] The dimensions measured (success, time, errors, satisfaction) are chosen in advance and match the decision.
- [ ] Think-aloud is used as a problem-discovery lens, not mistaken for clean performance measurement.
- [ ] Sample size matches the claim: discovery versus quantitative precision, and more participants where segments differ.
- [ ] Observed usability issues are recorded separately from participant preference opinions.
- [ ] Conclusions rest on recurring patterns across sessions, not on a single clean or disastrous run.
- [ ] The prototype-versus-production fidelity decision is explicit, and critical flows are re-tested at appropriate fidelity.
- [ ] Findings are synthesized into prioritized, severity-ranked design changes, not delivered as a raw issue list.
- [ ] Each change recommendation is framed as a design problem to solve, addressing root cause rather than patching a symptom.
