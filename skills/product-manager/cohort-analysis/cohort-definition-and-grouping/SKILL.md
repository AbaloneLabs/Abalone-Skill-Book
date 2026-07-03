---
name: cohort_definition_and_grouping.md
description: Use when the agent is defining user cohorts, choosing acquisition versus behavioral versus time grouping, setting cohort granularity and size, or ensuring a cohort definition matches the question being asked.
---

# Cohort Definition And Grouping

A cohort is a group of users defined by a shared starting condition, and the definition is the entire analysis. Change the grouping and the same data answers a different question, often the wrong one. The tool builds the cohort table in seconds, which hides the fact that the choice of grouping, granularity, and boundaries determines whether the result speaks to the question being asked. Two cohorts built from the same user base can tell opposite stories, because one grouped by signup date and the other grouped by first behavior.

The harm this skill prevents is a cohort analysis that answers a question nobody asked, presented as if it answered the one that mattered. A team asks whether a recent product change improved retention and builds a cohort by signup month, which conflates the change with acquisition-channel shifts. Another team asks whether a behavior predicts retention and groups by plan tier, which answers a different question entirely. The grouping must match the question, or the analysis is precise about the wrong thing.

Use this skill before defining a cohort, choosing between acquisition, behavioral, and time groupings, setting cohort granularity, or sizing cohorts for stable comparison. The work is to make the cohort definition a faithful instrument for the specific question, so the result actually addresses what the team needs to know.

## Core Rules

### Match The Cohort Type To The Question Being Asked

Cohorts answer different questions depending on how members are grouped, and the wrong grouping answers the wrong question. Acquisition cohorts, grouped by when users signed up, reveal whether newer users retain better, which speaks to product or acquisition-channel changes over time. Behavioral cohorts, grouped by what users did, reveal which behaviors correlate with retention or conversion. Time cohorts within a behavioral group reveal whether a recent change shifted behavior for users who share a starting action.

Before building a cohort, state the question in plain language and choose the grouping that answers it. If the question is about whether the product improved over time, use an acquisition cohort. If the question is about whether a behavior matters, use a behavioral cohort. Name the cohort type explicitly in any readout, because a statement like retention improved is uninterpretable unless the listener knows the grouping.

### Choose Acquisition Cohorts To Isolate Time-Based Change

Acquisition cohorts group users by when they entered the product, typically by signup day, week, or month. They are the right tool when the question is about change over time: did a release, a channel shift, or a seasonal effect alter how new users behave. Because everyone in the cohort started in the same period, differences between cohorts reflect something that changed between those periods.

Use acquisition cohorts when comparing the experience of users who joined before and after a change. Be aware that acquisition cohorts confound product changes with acquisition-channel changes, because the users who joined in a later period may differ in intent and quality from earlier users. Segment by acquisition source within the cohort to separate these effects.

### Choose Behavioral Cohorts To Test Whether An Action Matters

Behavioral cohorts group users by what they did, such as completed onboarding, invited a teammate, or used a feature in the first week. They are the right tool when the question is whether a behavior is associated with an outcome like retention or conversion. Comparing users who did the behavior to those who did not generates a hypothesis about the behavior's importance.

Use behavioral cohorts to identify promising levers, but remember that the comparison is correlational. Users who chose to do the behavior may differ from those who did not in motivation or intent, and that difference, not the behavior, may drive the outcome. Treat behavioral cohort differences as hypotheses to test with experiments, not as proven causal effects.

### Set Granularity To Match Volume And The Decision Timeline

Cohort granularity, how finely users are sliced into groups, must balance stability against responsiveness. Daily cohorts are noisy for low-volume products, because each cohort contains few users and the rates jitter wildly. Monthly cohorts are stable but too coarse to catch a recent change before it affects many users. The right granularity depends on volume and on how quickly the team needs to see a signal.

Match granularity to the volume of users and the timeline of the decision. For low-volume products, weekly or monthly cohorts produce stable rates; for high-volume products, daily cohorts catch change quickly. Look at more than one granularity before concluding, because daily noise can hide a monthly trend and monthly averaging can hide a daily shift.

### Ensure Cohorts Are Large Enough For Stable Rates

A cohort with too few users produces unstable rates that look dramatic and mean nothing. A ninety-percent retention rate from eleven users is noise, not insight, and charting it alongside larger cohorts misleads the reader into seeing a pattern. Sample size determines whether a rate is interpretable, and small cohorts must be flagged or excluded rather than displayed as if they were stable.

Check the size of every cohort before interpreting its rate, and set a minimum cell size below which rates are suppressed or marked as unreliable. When comparing cohorts, ensure the comparison is not driven by one tiny cohort with an extreme value. Stability, not drama, is the goal.

### Define Cohort Boundaries Precisely And Document Them

A cohort definition must be precise enough to reproduce, or two analysts will build different cohorts from the same instructions. Specify the entry event, the time boundaries, the exclusions, and the handling of edge cases such as users who qualify under multiple conditions. Imprecise definitions silently change the cohort when the query is edited or the schema shifts.

Document the definition alongside the result, including the entry condition, the grouping type, the granularity, and any exclusions. This makes the analysis auditable and lets the team detect when a number changed because reality changed versus because the definition changed.

### Avoid Comparing Cohorts Defined Differently

A common error is comparing cohorts that look equivalent but are defined differently, such as one cohort of all signups and another of signups excluding trial users. The definitions are not comparable, so any difference between them reflects the definition, not the behavior. Comparability requires identical definitions applied to different groups or periods.

Before comparing cohorts, confirm they are defined by the same rules, with the same entry event, exclusions, and boundaries. If the definitions differ, the comparison is confounded and the result should not be presented as a difference in behavior.

## Common Traps

### Using The Default Cohort Type Without Matching The Question

The analytics tool's default grouping is usually acquisition by time, which answers a time-based question that may not be the one asked. Match the grouping to the question, or the analysis will be precise about the wrong thing.

### Confounding Product Change With Acquisition Change In Acquisition Cohorts

Acquisition cohorts conflate product improvements with shifts in acquisition-channel quality, because later cohorts may contain different users. Segment by acquisition source within the cohort to separate the effects.

### Treating Behavioral Cohort Differences As Causal

Users who choose a behavior differ from those who do not, so behavioral cohort comparisons are correlational. Present them as hypotheses, not as proof that the behavior causes the outcome.

### Choosing Granularity That Hides The Pattern

Daily granularity is too noisy for low volume, and monthly granularity is too coarse to catch recent change. Match granularity to volume and the decision timeline, and review more than one level.

### Charting Tiny Cohorts As If They Were Stable

Small cohorts produce extreme, unstable rates that look meaningful and are not. Set a minimum cell size and flag or suppress rates below it, so noise is not mistaken for signal.

### Comparing Cohorts With Different Definitions

Cohorts defined by different rules are not comparable, and differences between them reflect the definition, not the behavior. Confirm identical definitions before comparing.

## Self-Check

- [ ] The cohort type, acquisition, behavioral, or time-within-behavior, is chosen to match the specific question being asked and is named in the readout.
- [ ] Acquisition cohorts are segmented by acquisition source to separate product change from channel-quality change where relevant.
- [ ] Behavioral cohort differences are presented as correlational hypotheses, not as proven causal effects of the behavior.
- [ ] Cohort granularity matches user volume and the decision timeline, and more than one granularity was reviewed before concluding.
- [ ] Every cohort's size is checked, and rates below a minimum cell size are flagged or suppressed rather than displayed as stable.
- [ ] The cohort definition, including entry event, time boundaries, exclusions, and edge cases, is documented precisely enough to reproduce.
- [ ] Cohorts being compared are confirmed to share identical definitions, so differences reflect behavior rather than definition.
- [ ] The question was stated in plain language before the grouping was chosen, ensuring the analysis addresses what was asked.
- [ ] No default grouping from the analytics tool was accepted without checking that it matches the question.
- [ ] The readout states the cohort type, granularity, sample sizes, and definition so the result is interpretable and auditable by others.
