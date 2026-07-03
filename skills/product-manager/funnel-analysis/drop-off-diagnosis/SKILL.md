---
name: drop_off_diagnosis.md
description: Use when the agent is diagnosing where and why users drop off in a funnel, distinguishing friction from rejection from interruption, combining quantitative drop-off with qualitative cause, or deciding what intervention a drop-off point actually needs.
---

# Drop-Off Diagnosis

Knowing where users drop off is the easy part; knowing why is the whole job. A funnel report will tell you that sixty percent of users leave between two steps, but that number is a starting point, not a conclusion. Drop-off at the same step can have completely different causes: some users were confused, some hit a hard requirement they could not meet, some encountered an error, some decided the product was not worth it, and some simply intended to return later. Each cause demands a different intervention, and treating them all the same guarantees the team fixes the wrong thing.

The harm this skill prevents is confident misdiagnosis. A team sees a high drop-off rate at the payment step and assumes friction, so they simplify the form, and the rate barely moves, because the real cause was that users did not trust the product enough to pay. Another team sees drop-off at onboarding and adds nudges, when the real cause was a technical error affecting one device. The quantitative location of drop-off is objective; the cause is inferred, and inference without evidence becomes a story the team believes and acts on.

Use this skill when a funnel shows drop-off and the question is what to do about it. The work is to classify the cause of drop-off using quantitative and qualitative evidence together, and to match the intervention to the cause rather than to the location.

## Core Rules

### Classify Drop-Off By Cause, Not Just Location

Drop-off has several distinct causes, and the intervention depends entirely on which one is at work. Friction is when the user wants to proceed but finds the step too hard or slow. Rejection is when the user decides the product or the step is not worth it. Interruption is when the user intended to return but was pulled away. Error is when the user tried to proceed and the system failed. Each produces the same quantitative signal, a drop between two steps, but each requires different evidence to identify and different work to fix.

Begin diagnosis by hypothesizing which cause is most likely, then gathering the evidence that would confirm or rule it out. Treating all drop-off as friction, the most common default, leads to simplifying steps that were not the problem.

### Combine Quantitative Drop-Off With Qualitative Cause

The funnel gives you the location and magnitude of drop-off; it cannot give you the cause. Cause comes from complementary evidence: session replays, user interviews, support tickets, survey responses, and direct observation. A high drop-off rate with no qualitative evidence is a number waiting to be misinterpreted.

Triangulate the quantitative location with at least one qualitative source before committing to an intervention. The quantitative data tells you where to look; the qualitative data tells you what you are looking at. Either alone is insufficient.

### Use Re-Entry Behavior To Separate Friction From Rejection

Whether users return to a step after leaving it is a powerful diagnostic signal. A step with high drop-off but high re-entry, where users come back and try again, suggests friction: the user wants to complete the step but struggles. A step with high drop-off and no return suggests rejection or interruption: the user has given up or been pulled away and is not coming back.

Examine re-entry rates, time-to-return, and the share of users who eventually complete the step after multiple attempts. High struggle with eventual success points to friction worth reducing; clean abandonment points to a value or intent problem that friction reduction will not solve.

### Check Error Rates And Technical Failures First

Before diagnosing a psychological cause, rule out a technical one. A drop-off step may be failing silently for a subset of users: a form that errors on certain devices, a verification that times out on slow connections, a payment that declines a card type. Technical failures look identical to friction or rejection in the aggregate funnel but require engineering, not design, to fix.

Segment drop-off by device, browser, network condition, and error events. If drop-off concentrates in a technical segment, the cause is a failure, not a user decision, and the fix is in the system.

### Measure Time-On-Step To Detect Friction Versus Hesitation

How long users spend on a step before leaving distinguishes different causes. Long time-on-step followed by drop-off suggests friction or hesitation: the user struggled or deliberated and gave up. Very short time-on-step followed by drop-off suggests rejection or interruption: the user decided quickly or left without engaging. The latency distribution reveals which kind of departure is happening.

Plot the time distribution for users who dropped off at the step, and compare it to users who proceeded. A friction problem shows a long tail of struggle; a rejection problem shows rapid exits. The shape of the latency tells you whether to reduce effort or increase motivation.

### Segment Drop-Off To Find Concentrated Failure

Aggregate drop-off can hide that the loss is concentrated in one segment while the rest of the funnel is healthy. A step may show thirty percent drop-off overall, but that average may combine five percent drop-off for desktop users and seventy percent for mobile users. The intervention for mobile is different from the intervention for the aggregate.

Segment drop-off by the dimensions that plausibly change behavior: device, acquisition source, geography, plan tier, prior actions. When a segment shows dramatically different drop-off, diagnose that segment specifically rather than treating the average as the problem.

### Match The Intervention To The Diagnosed Cause

The intervention must fit the cause, or it will not move the rate. Friction responds to simplification, defaults, and reducing required effort. Rejection responds to improving perceived value, trust, and relevance. Interruption responds to re-engagement and making return easy. Error responds to engineering fixes. Applying the wrong intervention, such as simplifying a form that users were rejecting on value, produces no improvement and erodes confidence in the analysis.

State the diagnosed cause explicitly before choosing the intervention, and design the intervention to target that cause. After shipping, measure whether the specific cause was addressed, not just whether the aggregate rate moved.

## Common Traps

### Assuming All Drop-Off Is Friction

The most common error is treating every drop-off as a usability problem and simplifying the step. If the real cause is rejection or interruption, simplification will not help. Always classify the cause before choosing the intervention.

### Diagnosing From The Funnel Alone

The funnel shows where users leave, not why. Inferring cause from location alone produces a plausible story that may be wrong. Combine the quantitative location with qualitative evidence before committing to a fix.

### Ignoring Technical Failures

A drop-off step that is silently failing for a segment looks like friction or rejection in the aggregate. Rule out errors by segmenting by device, browser, and error events before diagnosing a psychological cause.

### Treating Interruption As Abandonment

Users who leave with intent to return are different from users who have given up, but a fixed-window funnel treats them the same. Use re-entry and survival views to distinguish interruption from true abandonment before deciding the user is lost.

### Acting On Aggregate Drop-Off That Hides A Segment

An average drop-off rate can look moderate while one segment collapses and another is healthy. Segment before diagnosing, or the intervention will be aimed at a problem that does not exist for most users.

### Choosing The Intervention Before The Cause

Picking a fix based on familiarity, such as always simplifying forms, rather than on the diagnosed cause, wastes effort when the cause is value, trust, or error. Match the intervention to the cause, and verify the specific cause was addressed.

## Self-Check

- [ ] Drop-off is classified by cause, friction, rejection, interruption, or error, not only by location in the funnel.
- [ ] At least one qualitative source, such as replays, interviews, or support tickets, is combined with the quantitative drop-off rate.
- [ ] Re-entry behavior is examined to distinguish friction, where users return and struggle, from rejection or interruption, where they do not.
- [ ] Technical failures are ruled out by segmenting drop-off by device, browser, network, and error events before diagnosing a psychological cause.
- [ ] Time-on-step distribution is plotted to detect friction, long struggle, versus rejection, rapid exit, before choosing an intervention.
- [ ] Drop-off is segmented by dimensions that plausibly change behavior, and concentrated segment failure is diagnosed specifically.
- [ ] The intervention is matched to the diagnosed cause, simplification for friction, value for rejection, re-engagement for interruption, engineering for error.
- [ ] The diagnosed cause is stated explicitly before the intervention is chosen, not reverse-engineered to fit a preferred fix.
- [ ] After shipping, the specific cause is measured to confirm it was addressed, not only the aggregate drop-off rate.
- [ ] No drop-off is treated as friction by default without evidence ruling out rejection, interruption, and error.
