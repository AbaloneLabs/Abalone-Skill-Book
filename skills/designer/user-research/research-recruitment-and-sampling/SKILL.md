---
name: research_recruitment_and_sampling.md
description: Use when the agent is recruiting participants for user research, defining a sampling strategy, deciding who counts as a target user, screening participants, choosing between panels, internal users, and external customers, balancing sample composition across personas or segments, or judging whether a recruited sample can actually support the claims the research intends to make.
---

# Research Recruitment And Sampling

Recruitment is the silent foundation of every user research study. The findings of a study can only ever be as trustworthy as the sample they came from, and a sample is not just a headcount of participants but a theory of who the decision is about and who has been deliberately included or excluded. A usability test run with the wrong participants produces confident-looking findings that speak to the wrong population, and a survey fielded to a convenient panel produces precise numbers around a biased sample. Most research waste is not caused by a bad method; it is caused by a sample that could not answer the question the team thought it was answering.

Agents tend to fail recruitment in characteristic ways. They recruit whoever is fastest to schedule, which over-represents insiders, early adopters, and people with free time. They treat "we talked to eight users" as proof without asking which eight, how they were chosen, and who was excluded. They screen on surface demographics while ignoring the behavioral attributes that actually determine whether a participant can speak to the question. Or they recruit a sample so homogeneous that it can only confirm what the team already believed.

Use this skill before recruiting any study, while writing a screener, and while judging whether a completed sample can support the claims being drawn from it. The goal is a sample that is deliberate, documented, and honest about what it can and cannot establish.

## Core Rules

### Start Recruitment From The Decision And The Population

Recruitment begins with the question "who is this decision about," not "who can we get." A study whose sample does not match the population the decision concerns cannot support the decision, regardless of how clean the method is.

Define the population before recruiting:

- state the decision the research informs, and whose behavior it concerns;
- define the target population in terms that matter to the decision, which are usually behavioral rather than purely demographic;
- distinguish the full population from the accessible population, the slice you can actually reach, and note where they diverge;
- decide whether the study needs representative sampling, purposive sampling of specific segments, or maximum-variation sampling to surface diverse needs.

A study of "power users" that recruits only users who volunteered from a help forum is a study of help-forum volunteers, not of power users. Name the gap.

### Screen On Behavioral Attributes, Not Just Demographics

Demographics are easy to screen for and often the wrong axis. Whether a participant can speak to a question usually depends on behavior: what they do, how often, with which tools, and in which context. A screener that filters only on age, role, or region recruits people who look right but may have no relevant experience.

Build screeners around behavior:

- screen on the specific actions and contexts the study is about, using concrete questions about recent behavior;
- use frequency and recency filters, but recognize that self-reported frequency is unreliable, so prefer questions about the last specific instance;
- include disqualifying behaviors, such as working for a competitor or being a UX professional, that would contaminate the sample;
- avoid leading screener questions that telegraph the "correct" answer and let motivated participants qualify themselves in.

A screener is itself a small instrument. Test it on a few people before fielding, because a badly worded filter will recruit the wrong sample every time.

### Match The Sample Composition To The Claim Type

Different claims require different sample composition, and a sample that supports one claim may be useless for another. Mismatching composition to claim is the most common sampling error.

Match composition to the claim:

- for discovering problems qualitatively, recruit for variety across the relevant segments so that distinct needs surface;
- for measuring prevalence or comparing designs, recruit a sample that represents the target population in proportion, and size it for the required power;
- for understanding an edge segment, recruit deeply within that segment even if it is small in the population;
- for evaluating a redesign of an existing flow, recruit current users of that flow, not generic users of the product.

A sample of twelve highly engaged users cannot tell you how new users will experience onboarding. Match the sample to the question.

### Account For Recruitment Bias At Every Source

Every recruitment channel skews the sample, and the bias is invisible unless you name it. Volunteers are more engaged than the population. Panels over-represent people willing to take paid studies. Internal users know the product too well. Customers referred by sales are the customers sales talks to, not the ones who churned.

Name the bias of each source:

- panels tend to over-represent study professionals, who learn to game screeners, and certain demographics;
- internal employees bring institutional knowledge that real users lack, making findings optimistic;
- customer volunteers are more engaged and forgiving than the average user;
- intercept and in-product recruitment captures active users and misses those who abandoned;
- snowball and referral recruitment over-represents existing networks.

Triangulate across sources where possible, and where a single source must be used, state its bias as a limitation on the findings.

### Decide Sample Size Against Saturation Or Power, Not Convention

Sample size should be driven by the claim, not by a round number. The two legitimate logics are saturation for qualitative discovery and statistical power for quantitative claims.

Size the sample deliberately:

- for qualitative discovery, sample until new sessions stop surfacing new themes, recognizing that saturation depends on population diversity and may need more than the often-quoted five;
- for quantitative claims, compute the sample size needed to detect the effect or precision you care about, and do not field below it;
- for comparative designs such as A/B tests of an interface, size for the smallest difference that would matter;
- never attach percentages or "most users" language to samples too small to support them.

A sample too small for the claim is worse than no sample, because it produces false confidence.

### Document The Sample As Part Of The Findings

A finding without its sample is unverifiable. The audience must be able to judge whether the sample could support the claim, which means the sample must be documented alongside the findings.

Document the sample explicitly:

- how many participants, recruited how, from which source, screened on what;
- the composition across the segments that matter to the decision;
- the recruitment bias of the source and how it was mitigated;
- who was excluded and why.

This documentation is what lets a downstream reader calibrate their trust. Findings presented without it should be treated as anecdotes.

### Plan Recruitment Logistics And No-Shows

Recruitment is operational, and operational failure can sink a study. Participants no-show, screeners misfire, incentives arrive late, and scheduling collapses under timezone spread.

Plan for operational reality:

- over-recruit to absorb no-shows, especially for unmoderated and time-boxed studies;
- confirm participation close to the session, with reminders that reduce drop-off;
- pay incentives promptly and fairly, because slow or unfair payment damages both the participant and future recruitment;
- protect participant data and identity according to the consent and privacy commitments made at screening.

A study that fails operationally produces no data, regardless of how good the design was.

## Common Traps

### Recruiting Whoever Is Fastest

Speed-based recruitment over-represents insiders, volunteers, and people with flexible schedules, none of whom may match the target population.

### Screening On Demographics Alone

Age, role, and region are easy to filter but rarely determine whether a participant can speak to the behavioral question the study is asking.

### Treating The Sample As A Headcount

"We talked to eight users" is meaningless without knowing which eight, how they were chosen, and who was excluded. Composition, not count, determines what the sample can support.

### Generalizing From A Homogeneous Sample

A sample drawn from one segment can only confirm that segment's experience. It cannot establish how other segments will behave.

### Overclaiming Prevalence From Small Qualitative Samples

Five interviews can reveal a problem exists; they cannot establish how common it is. Do not attach percentages to small qualitative findings.

### Ignoring The Bias Of The Recruitment Source

Every channel skews the sample. Treating a panel or a volunteer pool as representative hides the bias inside precise-looking numbers.

### Leading Screener Questions

Screener items that telegraph the qualifying answer let motivated participants talk their way into the study and contaminate the sample.

### Under-Recruiting For Quantitative Claims

A sample too small for the precision or power the claim requires produces false confidence rather than useful evidence.

## Self-Check

- [ ] The target population was defined from the decision the research informs, in behavioral terms, before recruiting began.
- [ ] The screener filters on behavioral attributes relevant to the question, not only on demographics, and was tested before fielding.
- [ ] Sample composition matches the claim type: variety for discovery, proportionate representation for prevalence, depth within a segment for edge cases.
- [ ] The recruitment source's bias is named, and where possible multiple sources were triangulated.
- [ ] Sample size is justified by saturation or statistical power for the specific claim, not by convention.
- [ ] The sample is documented alongside the findings, including source, screening criteria, composition, exclusions, and bias.
- [ ] No prevalence or "most users" claim is attached to a sample too small to support it.
- [ ] Recruitment logistics, no-show buffers, incentives, and consent were planned so the study could actually run.
- [ ] The accessible population was distinguished from the full target population, and the gap between them was noted.
- [ ] The sample can support the specific claims being drawn from it; if it cannot, the claims were weakened rather than the sample over-read.
