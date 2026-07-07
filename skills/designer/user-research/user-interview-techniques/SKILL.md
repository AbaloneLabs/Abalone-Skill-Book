---
name: user_interview_techniques.md
description: Use when the agent is preparing, conducting, or synthesizing user interviews, writing interview guides, deciding between structured and semi-structured formats, avoiding leading questions, managing rapport and bias, probing without priming, or judging whether interview findings are trustworthy enough to inform design decisions.
---

# User Interview Techniques

An interview is not a conversation in which the user tells the team what to build, and it is not a survey read aloud. It is a structured method for surfacing what users actually do, believe, and feel, including the parts they would never volunteer in a form or a feature request. The technique is easy to do badly and hard to do well, because the same social dynamics that make people talkative also make them agreeable, eager to please, and quick to rationalize. A poorly run interview produces confident-sounding answers that are wrong, and those answers then drive design decisions that miss the real problem. The judgment problem is asking questions that reveal truth rather than manufacture it, hearing what users mean rather than what they say, and knowing when an interview finding is strong enough to act on.

Use this skill before running any user interview: writing the guide, recruiting, conducting the session, or deciding how much weight to place on what was said. The goal is to prevent the agent from treating interviews as casual chats, from asking leading questions that confirm the team's prior beliefs, and from over-interpreting a handful of articulate voices.

## Core Rules

### Decide What The Interview Must Learn Before Writing Questions

An interview without a defined learning goal becomes a wandering chat that produces stories but no usable finding. Define the specific decisions or questions the interview must inform before drafting a single prompt.

Clarify the learning target:

- the behaviors and contexts you need to understand, not the features you want validated;
- the hypotheses you are testing and what evidence would confirm or refute them;
- the segments of users whose perspective matters and why;
- the gap between what you already know and what only an interview can reveal.

If the interview cannot be tied to a decision, it is research theater. State the goal, then build every question to serve it.

### Favor Open, Behavioral, And Past-Tense Questions

The single most important technique is asking about concrete past experience rather than hypothetical future behavior. People are poor predictors of what they will do and reliable narrators of what they have done, even when their memory is imperfect.

Strong question forms:

- "Tell me about the last time you..." (behavioral, specific, past);
- "Walk me through how you..." (process, sequential, concrete);
- "What did you do right after that?" (probes the real sequence);
- "Can you give me an example?" (grounds abstractions in reality);
- "What was the hardest part?" (surfaces friction without leading).

Avoid:

- "Would you use a feature that...?" (hypothetical, unreliable);
- "Do you think it would be useful if...?" (invites agreement);
- "Don't you agree that...?" (leading, closed);
- "Why didn't you just...?" (implies a correct answer and induces rationalization).

Hypotheticals about the future tell you what users imagine, not what they will do. Treat them as weak evidence at best.

### Use A Semi-Structured Guide, Not A Rigid Script

A rigid script forces every participant through the same words regardless of where their experience is rich, and it prevents follow-up. A fully unstructured chat produces vivid stories but cannot be compared across participants. The semi-structured guide balances consistency with the freedom to dig.

Structure the guide as:

- a small set of core questions every participant must answer, so findings can be compared;
- flexible probes and follow-ups that let the interviewer go deeper where it matters;
- an opening that builds rapport and sets context before the substantive questions;
- a closing that invites anything missed ("Is there anything I should have asked?").

The guide is a tool for the interviewer, not a contract read verbatim. Listen, adapt, and follow the signal while keeping the core questions covered.

### Avoid Leading And Loaded Questions

Leading questions manufacture the answer they appear to seek. A question that contains the desired response, frames one option as obviously correct, or signals what the interviewer hopes to hear will produce agreement regardless of truth.

Watch for:

- embedded assumptions ("When you get frustrated with the tool...") that the user may not share;
- value-laden words (easy, powerful, annoying) that prime the response;
- double-barreled questions that ask two things at once and force a single answer;
- questions that offer a solution and ask for approval rather than asking about the problem.

Ask neutral, single-purpose questions and let the user supply the framing. Silence after a question is often more revealing than a follow-up, because it gives the user space to think and add what matters to them.

### Separate What Users Say From What They Mean And What They Do

Users rationalize. They describe idealized versions of their behavior, they conflate what they wish they did with what they actually do, and they tell you what they think you want to hear. A skilled interviewer listens for the gap.

Triangulate by:

- probing for specifics when answers stay abstract ("Can you walk me through the last time?");
- noting stated preferences but weighting described and observed behavior more heavily;
- asking about trade-offs, because real preferences surface under constraint;
- watching for hesitation, contradiction, and emotion, which often mark the real issue;
- following up later with observation or analytics where possible.

The user who says "I would definitely pay for that" and then cannot describe a time they paid for anything similar is giving you a weak signal. The user who vividly describes a painful workaround they perform every week is giving you a strong one.

### Manage Rapport Without Losing Neutrality

Interviews depend on trust, and users share more when they feel comfortable and unjudged. But warmth that becomes collusion destroys the data. Build rapport to lower defenses, then stay neutral to preserve honesty.

Balance by:

- starting with easy, non-evaluative questions to settle the participant;
- showing genuine curiosity without signaling agreement with every answer;
- avoiding praise that rewards certain responses ("Great point!") and shapes later ones;
- normalizing all answers ("Lots of people do that") so the user does not edit themselves;
- resisting the urge to defend the product or explain the design.

The interviewer's job is to understand, not to persuade. If you find yourself selling or explaining, you have stopped interviewing.

### Decide How Many Interviews Are Enough

A common failure is either stopping after two articulate users or running dozens in search of statistical certainty that qualitative research cannot provide. Interviews reach diminishing returns when new sessions stop revealing new patterns.

Useful guidance:

- a small number of well-run interviews can surface major themes, often within five to seven per segment;
- the goal is saturation, the point where you stop hearing new things, not a sample size;
- divergent or surprising voices matter more than confirmatory ones at the margins;
- segment by meaningful difference, because averaging across very different users hides the truth.

Decide in advance what saturation looks like for this question, and resist both premature confidence and endless fieldwork.

## Common Traps

### Asking Users What To Build

Users describe problems and current behavior well but are poor product designers. Feature requests reflect imagined solutions, not validated needs. Ask about problems, not solutions.

### Treating Interviews As A Survey Read Aloud

Closed, scripted questions produce shallow data and waste the medium. Use the interview's strength: open, adaptive probing into real experience.

### Hypothetical Future Questions

"What would you do if..." generates confident guesses that rarely match future behavior. Anchor in concrete past actions instead.

### Leading Questions That Confirm Priors

Questions that embed the hoped-for answer produce agreement that feels like validation but is manufactured. Ask neutrally.

### Overweighting Articulate Voices

Eloquent, confident participants can dominate synthesis simply because they were easy to quote. Weight by recurring pattern across users, not by memorability of a single voice.

### Interviewer Bias And The Need To Please

Users want to help, and they will agree with suggestions to be kind. Any question that signals the desired answer will get it, truthfully or not.

### Stopping At The First Plausible Theme

The first story that matches the team's hypothesis feels like confirmation. Keep probing until saturation, and actively look for disconfirming evidence.

## Self-Check

- [ ] The interview has a defined learning goal tied to a decision, and every question serves it.
- [ ] Questions are open, single-purpose, and past-tense or behavioral rather than hypothetical.
- [ ] A semi-structured guide covers core questions for comparison while allowing adaptive probing.
- [ ] No leading, loaded, double-barreled, or solution-seeking questions remain in the guide.
- [ ] Stated preferences are distinguished from described and observed behavior, and specifics were probed.
- [ ] Rapport was built without colluding, praising, or defending the product.
- [ ] The number of interviews reflects a saturation goal, not an arbitrary or statistical target.
- [ ] Synthesis weights recurring patterns across users over the most articulate individual voice.
- [ ] Disconfirming evidence was actively sought, not only stories that fit the hypothesis.
- [ ] Findings are labeled with appropriate confidence, distinguishing strong behavioral signals from weak hypothetical ones.
