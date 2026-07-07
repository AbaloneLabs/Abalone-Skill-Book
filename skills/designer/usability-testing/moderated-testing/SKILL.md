---
name: moderated_testing.md
description: Use when the agent is planning or conducting moderated usability testing, writing a test script and task scenarios, deciding between in-person and remote moderated sessions, handling the moderator's role during sessions, probing without leading, or synthesizing findings from qualitative usability sessions.
---

# Moderated Testing

Moderated usability testing is a method where a researcher observes a participant using a product while guiding the session in real time. It looks like a conversation about a prototype, but it is really a disciplined act of eliciting behavior and thought without contaminating it. Agents tend to treat moderation as casual facilitation, write leading tasks, or rescue participants who struggle, all of which destroy the validity of the findings. The harm is confident-sounding insights that reflect the moderator's influence rather than the user's真实 experience.

Use this skill before designing a moderated study, writing tasks, or facilitating sessions. The goal is to prevent the agent from contaminating the data through leading questions, from turning observation into coaching, or from drawing conclusions the method cannot support.

## Core Rules

### Write Task Scenarios, Not Instructions

The task is the heart of a usability test, and how it is written shapes everything. An instruction ("click the blue button") tells the participant what to do and reveals nothing about findability. A scenario ("you need to cancel your subscription before the next billing date") gives a goal and lets the participant's path reveal the design's strengths and problems.

Write tasks as scenarios:

- describe a realistic goal in the participant's context, not a step-by-step instruction;
- avoid naming interface elements the participant should find on their own (not "use the search bar");
- make the scenario concrete enough to act on, with a clear completion state;
- avoid leading language that hints at the expected path.

A well-written scenario lets you observe where users go, where they get stuck, and what they misunderstand. An instruction only confirms they can follow directions.

### Probe Without Leading

The moderator's questions are where data is most easily contaminated. "Did you find that confusing?" plants the idea that it was confusing. "Was that easy?" invites agreement regardless of truth. Leading questions produce findings that reflect the question rather than the participant's experience.

Use neutral probing:

- ask open questions: "what are you thinking right now?" or "tell me what you're looking for";
- ask about the past, not the hypothetical: "what did you expect to happen?" rather than "what would you expect?";
- avoid yes or no questions that invite agreement;
- use silence: pausing after a participant acts often surfaces their thought process better than a question.

Neutral probing is a skill that takes practice. The goal is to understand the participant's experience, not to confirm the moderator's hypothesis.

### Observe Behavior Over Self-Report

What participants do is more reliable than what they say they would do. Participants often report they found something easy while visibly struggling, or claim they would use a feature they have no real need for. The moderator must weight observed behavior above stated opinion, because behavior reflects the真实 experience and self-report is subject to bias.

Prioritize behavior:

- watch what participants do, where they pause, where they backtrack, and where they fail;
- note the gap between what participants say and what they do, and probe it neutrally;
- treat stated future behavior ("I would use this") as a weak signal, not a prediction;
- record sessions so behavior can be reviewed, not just remembered.

A participant who struggles but reports success is telling you the design has a problem they have rationalized. The behavior is the data; the self-report is context.

### Resist Coaching Or Rescuing Participants

When a participant struggles, the natural impulse is to help. But helping destroys the data: you can no longer tell whether the participant would have succeeded alone, and the finding about the design's difficulty is lost. The moderator must tolerate the discomfort of watching struggle and resist intervening unless the protocol allows it.

Manage the impulse to help:

- let participants struggle within the bounds of the protocol; struggle is data;
- intervene only at predefined points (after a time limit, or if the participant gives up), and record that intervention occurred;
- never point to the right answer or hint at the path;
- debrief after the task to reduce participant frustration and restore their confidence.

Coaching produces a false sense that the design works. The pain of watching struggle is the price of valid findings.

### Decide In-Person Versus Remote Deliberately

In-person and remote moderated testing each have strengths, and the choice should follow the research question, not convenience. In-person sessions capture body language, environment, and subtle cues, and allow physical prototypes. Remote sessions reach geographically diverse participants, are cheaper and faster to schedule, and observe users in their own context.

Match mode to need:

- choose in-person when physical interaction, environment, or subtle nonverbal cues are essential;
- choose remote when geographic diversity, speed, or observing the user's real context matters;
- ensure remote sessions have reliable screen and audio sharing, and a backup plan for technical failure;
- consider hybrid approaches for distributed teams reviewing sessions.

Convenience-driven mode choice produces sessions that cannot answer the question. Let the research goal drive the format.

### Size And Segment The Sample Appropriately

Moderated testing is qualitative, and its value is discovering problems, not measuring prevalence. A small number of participants (often five to eight per user segment) is enough to surface major usability problems, because severe issues recur quickly. But the sample must be segmented by user type, because different users encounter different problems.

Size the sample to the method:

- use five to eight participants per distinct user segment to reach saturation on major problems;
- segment by characteristics that change the experience (novice versus expert, different roles, different devices);
- do not claim prevalence from qualitative samples; five participants can reveal a problem exists, not how common it is;
- add sessions if new sessions keep revealing new problems, indicating saturation is not reached.

Mixing segments in one sample hides problems specific to a segment. Segment deliberately and report per segment.

### Synthesize Findings By Severity, Not By Frequency Alone

In a small qualitative sample, counting how many participants hit a problem is misleading, because the sample is too small to estimate frequency. A problem one participant hit severely may matter more than a minor annoyance three participants mentioned. Synthesis should rank by severity and impact, informed by but not dictated by raw counts.

Synthesize by impact:

- rank problems by severity: does it block the task, cause errors, or just slow the user?;
- consider the business impact: problems on critical paths matter more than problems on rare flows;
- distinguish root causes from symptoms; several observed problems may share one underlying cause;
- include positive findings, not only problems, so the team knows what works.

Frequency in a small sample is a weak signal. Severity and impact are stronger guides to what to fix first.

### Separate Findings From Recommendations

Findings are what was observed; recommendations are what to do about them. Conflating them weakens both: the team cannot evaluate the evidence because it is tangled with a proposed fix, and the recommendation carries false authority because it reads as a finding. Keep them distinct in reporting.

Separate clearly:

- report findings as observed behavior and quotes, with the task context;
- present recommendations separately, labeled as suggestions, with their rationale and tradeoffs;
- acknowledge where a recommendation is uncertain and would need validation;
- let stakeholders see the evidence and form their own judgment, not just receive directives.

Findings have the authority of data; recommendations have the authority of expertise. Mixing them gives recommendations unearned certainty and hides the evidence behind findings.

## Common Traps

### Writing Instructions Instead Of Scenarios

Tasks that tell participants what to click reveal nothing about findability; write goal-based scenarios that let the path emerge.

### Leading Questions That Plant Answers

"Did you find that confusing?" contaminates the data; use neutral, open probes about the participant's experience.

### Trusting Self-Report Over Behavior

Participants often rationalize struggle as success; weight observed behavior above stated opinion and probe the gaps.

### Coaching Struggling Participants

Helping participants destroys the data about the design's difficulty; tolerate struggle within the protocol and record any intervention.

### Convenience-Driven Mode Choice

Choosing in-person or remote by convenience rather than research need produces sessions that cannot answer the question.

### Claiming Prevalence From Small Samples

Five participants can reveal a problem exists but not how common it is; size and segment the sample and avoid frequency claims.

### Ranking By Raw Count Over Severity

In small samples, frequency is a weak signal; rank findings by severity and business impact, and distinguish root causes from symptoms.

### Conflating Findings With Recommendations

Mixing observations with proposed fixes gives recommendations unearned certainty; keep findings and recommendations separate in reporting.

## Self-Check

- [ ] Tasks are written as realistic goal-based scenarios, not step-by-step instructions that name interface elements.
- [ ] Probes are neutral and open, avoiding leading or yes-or-no questions that plant answers.
- [ ] Observed behavior is weighted above self-report, and gaps between the two are probed neutrally.
- [ ] The moderator tolerated participant struggle within the protocol and did not coach or rescue.
- [ ] The session mode (in-person or remote) was chosen to match the research need, not convenience.
- [ ] The sample is sized to reach saturation (five to eight per segment) and segmented by user type, with no prevalence claims.
- [ ] Findings are synthesized by severity and impact, with root causes distinguished from symptoms and positive findings included.
- [ ] Findings and recommendations are reported separately, with recommendations labeled as suggestions and their uncertainty acknowledged.
