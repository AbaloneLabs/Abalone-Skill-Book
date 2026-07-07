---
name: diary_studies.md
description: Use when the agent is planning or running a diary study, choosing between longitudinal and snapshot diary methods, designing diary prompts and entry formats, deciding on the study duration and frequency, managing participant engagement and attrition over time, or synthesizing data that captures user behavior and context as it unfolds over days or weeks.
---

# Diary Studies

A diary study is a longitudinal research method where participants record their experiences, behaviors, or context over time, revealing patterns that a single interview or usability session cannot. It looks like asking people to journal, but it is really a disciplined effort to capture behavior in its natural setting without the distortion of recall. Agents tend to treat diary studies as extended surveys, design prompts that are too vague or too burdensome, or lose participants to attrition until the data is too thin to trust. The harm is findings that reflect who happened to keep responding rather than the population, or conclusions drawn from entries so sparse they cannot support a claim.

Use this skill before designing a diary study or interpreting its results. The goal is to prevent the agent from building a study participants abandon, from designing prompts that produce unusable data, or from over-reading thin longitudinal data as if it were rich.

## Core Rules

### Use Diary Studies For Questions That Unfold Over Time

A diary study is the right method when the question is about behavior, context, or experience that changes over time and cannot be captured in a single session. It is the wrong method for questions a single interview or usability test answers better and more cheaply. Choosing a diary study for a question that does not need longitudinal data wastes participant effort and produces data no richer than a recall-based interview.

Match the method to the temporal question:

- use diary studies for habits, routines, and recurring behaviors that vary day to day;
- use them for experiences tied to context (where, when, with whom) that a lab session strips away;
- use them for journeys that unfold over days or weeks (onboarding, a purchase decision, an illness);
- do not use them for one-time tasks, preference questions, or anything a focused session captures fully.

A diary study answers "how does this play out over time and in context," not "what do users think of this screen." If the question is not longitudinal, choose a faster method.

### Choose Event-Based Versus Interval-Based Capture Deliberately

There are two fundamental diary designs, and the choice shapes the data. Event-based (or signal-based) capture asks participants to log whenever a specific event occurs or when prompted at a signal. Interval-based capture asks participants to log at set times (end of day, twice a week). Each has characteristic biases.

Match design to the question:

- event-based capture suits infrequent or unpredictable events, capturing them close to when they happen and reducing recall distortion;
- interval-based capture suits routines and daily experience, providing regular snapshots but risking recall if participants reconstruct at logging time;
- signal-based capture (prompting at random or fixed times) captures in-the-moment experience but may miss events that happen between signals;
- be explicit about which design you are using and the bias it introduces.

A diary study's design determines what it can and cannot show. Mixing designs without intent produces data that cannot be interpreted consistently.

### Design Prompts That Are Specific Enough To Answer And Light Enough To Complete

Diary prompts are where studies succeed or fail. Vague prompts ("tell us about your day") produce entries too unfocused to analyze. Burdensome prompts (twenty questions per entry) produce attrition as participants tire. The prompt must be specific enough to generate comparable, analyzable data and light enough that participants keep logging.

Craft prompts carefully:

- ask about concrete, recent behavior ("what did you do to solve X today?") rather than abstract reflection;
- limit each entry to a few focused questions that can be answered in a few minutes;
- use a mix of structured (scales, multiple choice) and open (short text) questions;
- pilot the prompts to check they are understood as intended and are not too burdensome.

A prompt that takes ten minutes to answer will be abandoned by the second day. A prompt that asks "how do you feel about the product" produces essays that cannot be compared across participants. Balance specificity with burden.

### Size The Duration To The Behavior's Natural Cycle

The study duration must match the behavior being studied. A diary study of a daily habit needs enough days to see variation, but not so many that participants burn out. A study of a monthly journey needs weeks, accepting lower-frequency entries. Too short a study misses the pattern; too long a study loses participants.

Match duration to the cycle:

- for daily routines, one to two weeks usually captures variation without excessive attrition;
- for infrequent events or long journeys, extend the duration but reduce entry frequency to manage burden;
- account for the behavior's natural cycle (weekly routines need at least a full week; monthly cycles need a month);
- plan for attrition and recruit more participants than the final analysis needs.

A study that ends before the behavior's cycle completes cannot show the pattern. A study that runs too long produces a final dataset dominated by the most compliant participants, who may not represent the population.

### Manage Attrition Actively

Attrition is the central threat to diary study validity. Participants drop out over time, and those who remain are systematically different (more motivated, more compliant, more invested) from those who leave. A dataset analyzed without accounting for attrition describes the survivors, not the population.

Mitigate and account for attrition:

- recruit more participants than the analysis requires, planning for substantial dropout;
- keep the burden low and the value to participants clear to sustain engagement;
- check in with participants during the study to re-engage those falling behind;
- analyze who dropped out and whether they differ from completers, and report this honestly.

Ignoring attrition produces findings that generalize to the compliant minority. Honest reporting of attrition keeps the findings credible and appropriately scoped.

### Capture Context Alongside Behavior

The value of a diary study over a lab session is context: where the participant was, what else was happening, what their emotional and physical state was. Behavior without context is hard to interpret; the same action can mean different things in different settings. The diary should capture the circumstances, not just the behavior.

Capture context deliberately:

- ask where the participant was and what they were doing around the event;
- capture emotional and physical state, since these shape experience and recall;
- allow photos or quick media capture where it enriches context without adding burden;
- use the context to interpret behavior, not just to decorate entries.

A diary entry that says "I used the feature" without context tells you little. The same entry with "I was rushing, on my phone, in a noisy train" tells you why the experience felt the way it did.

### Synthesize For Patterns And Trajectories, Not Individual Entries

Diary data is voluminous and uneven. The temptation is to quote compelling individual entries, but the value is in patterns across participants and trajectories over time. Synthesis must look for what recurs, what changes, and what the sequence reveals, not cherry-pick vivid moments.

Synthesize systematically:

- look for patterns across participants: what experiences recur, what contexts matter, what variations exist;
- look for trajectories within participants: how does the experience change over the study period;
- distinguish consistent findings from idiosyncratic ones, and weight accordingly;
- use the longitudinal nature: what does the sequence over time reveal that a snapshot could not.

Quoting one vivid entry as if it represents the population is the diary-study equivalent of over-reading a single interview. The method's strength is in patterns over time, and synthesis must honor that.

### Pair Diary Studies With Follow-Up Interviews

Diary entries, even rich ones, leave questions: why did the participant do that, what did they mean, what happened next. A follow-up interview after the diary period lets the researcher probe the entries, turning raw logs into understood experience. Diary-without-interview produces data that is observed but not fully understood.

Sequence the methods:

- review diary entries before the interview to identify patterns and puzzles to probe;
- use the interview to ask about specific entries, not general recall;
- let the diary ground the interview in真实 events rather than reconstructed memory;
- use the interview to fill the gaps the diary could not capture.

The diary captures what happened over time; the interview reveals why. Together they are far stronger than either alone.

## Common Traps

### Using A Diary Study For A Non-Longitudinal Question

Diary studies answer questions about behavior over time and context; for one-time questions, a faster method is better.

### Vague Or Burdensome Prompts

Prompts too vague produce unfocused data; prompts too heavy produce attrition; balance specificity with a few minutes of effort.

### Mismatched Duration

A study too short misses the behavior's cycle; too long burns out participants; match duration to the natural rhythm.

### Ignoring Attrition

Dropout leaves a dataset of compliant survivors who do not represent the population; recruit extra, re-engage, and report attrition honestly.

### Behavior Without Context

Entries that log actions without setting, state, and circumstance are hard to interpret; capture context alongside behavior.

### Cherry-Picking Vivid Entries

Quoting one compelling entry as representative over-reads the data; synthesize for patterns and trajectories across participants and time.

### Skipping Follow-Up Interviews

Diaries without interviews leave the why unexplained; pair the diary with a follow-up that probes specific entries.

### Treating Diary Data Like Survey Data

Diary entries are qualitative and uneven; do not force them into quantitative claims they cannot support.

## Self-Check

- [ ] The research question is genuinely longitudinal, about behavior or experience over time and context, not answerable by a single session.
- [ ] The capture design (event-based, interval-based, or signal-based) is chosen deliberately and its bias is understood.
- [ ] Prompts are specific enough to generate comparable data and light enough (a few minutes) to sustain completion.
- [ ] The study duration matches the behavior's natural cycle, with recruitment planned for attrition.
- [ ] Attrition is actively mitigated and honestly reported, including whether dropouts differ from completers.
- [ ] Context (location, activity, state) is captured alongside behavior to enable interpretation.
- [ ] Synthesis looks for patterns across participants and trajectories over time, not cherry-picked vivid entries.
- [ ] A follow-up interview probes specific diary entries to reveal the why behind the observed behavior.
