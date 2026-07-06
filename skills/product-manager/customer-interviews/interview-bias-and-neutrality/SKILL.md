---
name: interview_bias_and_neutrality.md
description: Use when the agent is conducting or analyzing customer interviews, trying to avoid leading questions and confirmation bias, recognizing social desirability effects, or ensuring that interview learning is not contaminated by the interviewer's expectations.
---

# Interview Bias And Neutrality

Bias in interviews is not a rare accident; it is the default. Every choice the interviewer makes, from word order to tone of voice to who gets recruited, pushes the respondent toward some answers and away from others. The interviewer usually holds a hypothesis and, without meaning to, shapes the conversation to confirm it. Neutrality is an active discipline, not the absence of effort. Done well, it produces answers the team did not expect and may not welcome, which is exactly what makes them valuable. Done poorly, it produces a tidy confirmation of whatever the team already believed.

The harm this skill prevents is research that feels rigorous but is structurally incapable of disconfirming the team's hypothesis. When bias is uncontrolled, the interview becomes a machine for manufacturing the desired answer, and the product ships on the strength of evidence that was never really there.

Use this skill before answering questions such as "how do we avoid bias in interviews", "are our interview findings reliable", "why do customers keep telling us what we want to hear", or "how do we stay neutral". The goal is to prevent the agent from treating contaminated interview output as trustworthy evidence.

## Core Rules

### Treat Your Own Hypothesis As The Primary Threat

The most dangerous bias is confirmation bias, and it is driven by the interviewer's own expectations. Before the interview, write down what you believe you will find and what evidence would prove you wrong. If you cannot state what would change your mind, you are not testing a hypothesis, you are seeking affirmation. Hold the disconfirming possibility actively in mind during the conversation, because the mind will otherwise selectively hear confirming fragments.

Design the interview so that disconfirming evidence is easy to give and confirming evidence is hard to fake. A respondent who disagrees should feel that disagreement is welcome, not that they have disappointed the interviewer. Your tone when you hear an unwanted answer reveals more about your neutrality than your script does.

### Remove Cues That Signal The Desired Answer

Respondents read the interviewer constantly for signals about what answer is wanted. These cues include leading words, an approving tone for some answers, rushed follow-ups on others, the order in which options are presented, and even nodding. Audit not only the questions but your behavior. A neutral interviewer gives the same engaged, warm response to an answer that confirms the hypothesis and to one that demolishes it.

Beware of framing effects. Asking "what problems do you have with X" presupposes problems exist; asking "tell me about your experience with X" leaves open the possibility that there are none. The framing determines which reality the respondent reports.

### Watch For Social Desirability Bias

People adjust their answers to present themselves favorably and to please the person asking. They overstate virtuous behaviors like reading documentation, exercising, or following best practices, and understate embarrassing ones like ignoring warnings or taking shortcuts. Anything that makes the respondent look good in the interviewer's eyes will be inflated, and anything that makes them look careless will be minimized.

Counter social desirability by normalizing the behavior you are asking about and by removing judgment. Instead of "do you read the instructions", ask "when you first opened it, what did you do". Asking about the last specific instance, rather than about habits in general, also reduces the urge to present an idealized self.

### Separate What People Say From What They Do

Stated behavior and actual behavior diverge, often dramatically. People report intentions, attitudes, and habits that do not match what usage data or observation would show. Treat self-report as a hypothesis about behavior, not as the behavior itself. Whenever possible, triangulate interview claims against product analytics, support tickets, or direct observation.

The gap between saying and doing is itself informative. When a respondent insists a feature is essential but has never used the equivalent that already exists, the gap reveals that the stated need is aspirational, not operational. Capture both the claim and any observable contradiction.

### Avoid The False Quantification Of Qualitative Data

Interviews are qualitative. Counting that seven of ten respondents mentioned a problem does not make the finding quantitative, because the sample is not representative and the interviewer's behavior shaped what was mentioned. Reporting interview findings as percentages or majorities implies a statistical weight the method cannot support. Describe patterns, illustrate with quotes, and be explicit that frequency in the sample is not prevalence in the population.

If you need quantitative prevalence, run a survey on a representative sample. Do not let the intimacy of interviews create a false sense of statistical certainty.

### Design For Falsifiability At The Study Level

A biased study is one structured so that almost any outcome can be interpreted as support. Make the study falsifiable by specifying, in advance, what pattern of answers would cause the team to abandon or change the hypothesis. If every conceivable interview result can be read as confirmation, the study cannot teach you anything, because nothing could have disproved it.

This is hardest when the team is emotionally invested in the answer. The discipline of pre-specifying the disconfirming pattern is what separates research from advocacy.

## Common Traps

### Leading Questions Disguised As Open Ones

Questions that sound open but embed the answer, such as "what do you find frustrating about X". The trap is that the script looks neutral while systematically producing the expected response.

### Reading Confirmation Into Ambiguous Answers

Interpreting a polite or vague answer as agreement, because the interviewer wants agreement. The trap is that ambiguous data hardens into evidence that supports the desired conclusion.

### Rewarding Confirming Answers With Warmth

Giving more attention, follow-up, and enthusiasm to answers that match the hypothesis. The trap is that respondents learn which answers please the interviewer and produce more of them.

### Generalizing From A Self-Selected Sample

Treating patterns among willing interview respondents as representative of the whole market. The trap is that the people who agree to talk differ systematically from those who do not.

### Counting Mentions As Prevalence

Reporting that a problem was mentioned by most interviewees as if it were a measured frequency. The trap is false quantification that lends statistical weight to a qualitative finding.

### Asking About Sensitive Behavior Directly

Asking bluntly about behaviors respondents are ashamed of. The trap is systematically understated answers that distort the finding, when normalization or indirect questioning would have surfaced the truth.

## Self-Check

- [ ] The team's hypothesis is written down alongside the evidence that would disconfirm it, and disconfirming evidence is actively sought during interviews.
- [ ] Questions and interviewer behavior have been audited for cues that signal the desired answer.
- [ ] Social desirability bias is mitigated by normalizing behavior and asking about specific recent instances.
- [ ] Stated behavior is treated as a hypothesis and triangulated against observed behavior or analytics where possible.
- [ ] Interview findings are reported as qualitative patterns, not as false percentages or majorities.
- [ ] The study is falsifiable, with a pre-specified pattern of answers that would change or abandon the hypothesis.
- [ ] The gap between what respondents say and what they do is captured and treated as informative.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
