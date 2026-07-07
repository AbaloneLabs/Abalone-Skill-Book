---
name: customer_discovery_interviews.md
description: Use when the agent is planning customer discovery interviews, writing discovery questions, recruiting the right participants, avoiding leading questions, synthesizing qualitative findings, or deciding whether discovery evidence is strong enough to justify building.
---

# Customer Discovery Interviews

Discovery interviews are the engine of evidence-based product decisions, and they are also one of the most frequently misused research methods. The trap is that an interview feels like a conversation, so practitioners improvise, ask leading questions, confirm their existing hypothesis, and walk away believing they have validated something they merely suggested. A good discovery interview is the opposite of a sales call or a feature poll: it is a disciplined effort to understand how a person currently behaves, what they struggle with, and what they have already done about it, without contaminating the answers with the interviewer's own theory. The judgment problem is designing questions that reveal rather than lead, recruiting people who represent the real target rather than the convenient friendly user, and deciding how much weight a handful of conversations can bear before they justify a build decision.

Use this skill before scheduling discovery interviews, before writing an interview guide, before deciding how many participants are enough, and before translating interview findings into product decisions. The goal is to prevent the agent from producing confident conclusions from contaminated or unrepresentative interviews, or from dismissing real signal because the sample felt small.

## Core Rules

### Start From Behavior, Not Opinion Or Hypothetical Future

People are reliable narrators of their past and present behavior and unreliable predictors of their future actions. The strongest discovery questions anchor in what the person actually did, not what they would do or what they think of an idea.

Anchor questions in:

- the last time they encountered the problem, walk me through it;
- what they did step by step, not what they would do;
- the tools, workarounds, and people they involved;
- what they tried that did not work;
- the cost, in time, money, or frustration, of the current approach;
- how often this happens and how they decide it is worth addressing.

Hypothetical "would you use a feature that..." questions produce optimistic lies. Behavioral questions produce usable evidence.

### Write Questions That Do Not Lead

The interviewer's hypothesis leaks into phrasing and contaminates the answer. A leading question tells the participant what you want to hear and they will usually say it.

Avoid leading patterns:

- "Do you struggle with..." presupposes a struggle;
- "Wouldn't it be great if..." invites agreement;
- "How important is..." assumes importance;
- "Do you like our product..." invites politeness;
- showing a prototype too early reframes the conversation as evaluation.

Prefer neutral, open prompts: "Tell me about the last time you...", "What was the hardest part...", "Walk me through how you...". Let silence do work.

### Recruit For The Target, Not For Convenience

The friendly power user, the willing existing customer, and the person who replies fast are not necessarily the people whose problems matter. Sampling bias destroys the validity of everything that follows.

Recruit deliberately:

- define the target segment and persona before recruiting;
- include non-customers and churned users, not only happy ones;
- include people who chose a competitor or a manual workaround;
- avoid over-relying on a single account or champion;
- balance power users, typical users, and struggling users;
- consider the silent majority who never complain.

A discovery built only on people who already love the product will miss the market you are trying to reach.

### Separate Problem Discovery From Solution Validation

Discovery and validation are different activities with different methods. Conflating them produces muddled evidence. In discovery you are open and exploring; in validation you are testing a specific solution against a defined bar.

Keep them distinct:

- discovery: understand the problem space, behavior, and jobs, open-ended;
- validation: test a specific solution, prototype, or value proposition against criteria;
- do not show a solution during problem discovery unless you are consciously validating;
- do not claim a problem is validated because people liked a prototype.

### Listen For Workarounds And Existing Solutions

The strongest signal of a real problem is that the person has already spent effort solving it. Workarounds, spreadsheets, scripts, paid tools, and manual processes are evidence of pain with a budget. The absence of any workaround is often evidence that the pain is tolerable.

Listen for:

- custom spreadsheets, scripts, or processes built to cope;
- money already spent on a partial or competing solution;
- time routinely lost to the problem;
- people hired or reassigned to manage the issue;
- emotional language indicating real frustration.

A problem nobody has tried to solve is usually not painful enough to build for.

### Decide Sample Size By Saturation, Not A Fixed Number

There is no magic number of interviews that makes discovery valid. The useful principle is saturation: the point at which new interviews stop revealing new patterns. For homogeneous segments this may be five to eight; for diverse segments it may be many more.

Decide sample by:

- continuing until you hear repeated patterns rather than novel ones;
- increasing sample for diverse or segmented audiences;
- treating contradictory findings as a signal to keep interviewing;
- pairing qualitative saturation with quantitative confirmation for scale;
- being honest that small samples support direction, not precise sizing.

### Synthesize For Patterns And Disconfirming Evidence

The danger after interviews is confirmation bias, noticing only what supports the hypothesis. Synthesis must actively look for disconfirming evidence and alternative explanations.

Synthesize by:

- using the participants' own words as evidence, not paraphrased assumptions;
- clustering quotes and behaviors into patterns;
- actively searching for interviews that contradict the emerging story;
- distinguishing widespread patterns from vivid single anecdotes;
- separating what people said from what you infer they need;
- recording confidence level and the gaps that remain.

### Connect Findings To Decisions With Stated Confidence

Discovery does not end with a findings deck. It ends with a decision made at an appropriate confidence level. Be explicit about what the evidence supports and what it does not.

State:

- what the interviews strongly suggest;
- what they weakly suggest;
- what they cannot tell you, such as market size or willingness to pay;
- the next step, more discovery, a quantitative test, a prototype, or build;
- the risk of acting on this evidence and how it is mitigated.

## Common Traps

### The Leading Question

Phrasing that telegraphs the desired answer contaminates the response and manufactures false validation.

### Hypothetical Future Questions

Asking what people would do yields optimistic predictions that do not match future behavior.

### Convenient Sampling

Interviewing only friendly existing users misses the target market and the people you have already lost.

### Prototype Too Early

Showing a solution during problem discovery reframes the conversation and produces evaluation, not understanding.

### Confirmation Bias In Synthesis

Noticing only supportive quotes and ignoring contradictions turns synthesis into advocacy.

### Confusing Liking With Needing

Enthusiasm for an idea in an interview does not equal willingness to adopt or pay.

### Overweighting Vivid Anecdotes

A single powerful story can override a pattern of mild signal and distort priorities.

### Treating Discovery As Validation

Claiming a problem is proven because people liked a prototype conflates two different methods and weakens the evidence.

## Self-Check

- [ ] Questions are anchored in past and present behavior, not hypothetical future actions or opinions.
- [ ] The interview guide avoids leading phrasing and lets participants reveal rather than confirm.
- [ ] Participants represent the target segment, including non-customers, churned users, and struggling users, not only friendly ones.
- [ ] Problem discovery is kept separate from solution validation, with prototypes withheld unless consciously validating.
- [ ] The synthesis actively looks for workarounds and existing spend as evidence of real pain.
- [ ] Sample size is driven by saturation and segment diversity, not a fixed arbitrary number.
- [ ] Synthesis searches for disconfirming evidence and distinguishes widespread patterns from vivid anecdotes.
- [ ] Findings are connected to a decision with an explicit confidence level and stated gaps.
- [ ] Participant quotes are used as evidence rather than paraphrased assumptions.
- [ ] The limitations of qualitative discovery, such as market sizing, are acknowledged and addressed with complementary methods.
