---
name: survey_and_questionnaire_design.md
description: Use when the agent is designing a survey or questionnaire for user research, writing survey questions, choosing between open and closed questions, constructing rating and Likert scales, deciding question order and branching, avoiding leading or double-barreled questions, planning sample size and analysis, or judging whether a survey can actually answer the descriptive or attitudinal question it was fielded to answer.
---

# Survey And Questionnaire Design

A survey is a measurement instrument, and like any instrument it measures what it was built to measure, not necessarily what the team wishes it measured. Writing questions feels easy, which is precisely why surveys fail so often: a casually worded question produces a confidently precise number around the wrong construct, and the team acts on the number without realizing the instrument never touched the question they cared about. Survey data is cheap to collect and expensive to misinterpret, because the precision of percentages lends false authority to answers that were shaped by question wording, order, and scale design.

Agents tend to fail survey design in predictable ways. They write questions that lead the respondent toward the desired answer. They pack two questions into one and cannot tell which the respondent answered. They choose scales whose labels are ambiguous, so a "4" means different things to different people. They field a survey to a biased sample and present the percentages as if they described the population. Or they use a survey to answer a "why" question that surveys structurally cannot answer.

Use this skill before writing any survey, while constructing scales and items, and while interpreting results. The goal is an instrument whose wording, scales, order, and sampling can actually support the descriptive or attitudinal claims being made from it.

## Core Rules

### Decide Whether A Survey Is The Right Instrument

A survey answers "what" and "how many" reliably and "why" poorly. Before writing a single question, confirm that the question is descriptive or attitudinal and therefore surveyable, rather than causal or motivational, which it is not.

Match the instrument to the question:

- descriptive questions, such as "how often do users export reports," are surveyable;
- attitudinal questions, such as "how satisfied are users," are surveyable but require careful scale design;
- "why" questions, such as "why do users abandon checkout," are not surveyable; people are poor witnesses to their own motivation, and follow-up interviews are stronger;
- causal questions, such as "did the redesign cause the lift," require an experiment, not a survey.

Using a survey to answer a "why" question produces confident numbers around guessed causes. If the real question is motivational or causal, change the method.

### Write Neutral, Single-Construct Questions

Question wording shapes the answer, and leading or compound wording makes the data uninterpretable. The discipline is to write items that ask one thing, neutrally, in language the respondent actually uses.

Write clean items:

- ask one question per item; avoid double-barreled questions like "was it fast and easy," where a mixed answer is uninterpretable;
- avoid leading wording such as "how frustrating was it," which assumes the experience was frustrating;
- avoid loaded or judgmental language that signals the socially desirable answer;
- use the respondent's own vocabulary, not internal product terminology they may not recognize;
- keep items short, concrete, and tied to a specific timeframe or instance.

Pre-test items with a few respondents and ask them to paraphrase the question; if they paraphrase differently, the item is ambiguous and must be rewritten.

### Choose Scales Deliberately

The scale is part of the question. A five-point satisfaction scale, a seven-point Likert, an eleven-point NPS, and a forced binary all measure differently, and the choice constrains what analysis is valid.

Design scales with care:

- label scale points with words, not just numbers, because an unlabeled "4" means different things to different respondents;
- decide whether to include a neutral midpoint based on whether you want to force a direction or allow genuine ambivalence;
- for frequency, use concrete anchors such as "in the last 7 days" rather than vague terms like "often";
- avoid agree-disagree scales that invite acquiescence bias, where respondents tend to agree regardless of content;
- consider whether a numeric scale will be analyzed as a score, in which case ensure the intervals are comparable.

A scale that looks fine but has ambiguous anchors produces data that looks numeric but is not meaningfully comparable across respondents.

### Order Questions To Avoid Contamination

Earlier questions change how respondents interpret later ones. A question about satisfaction asked after a question about a recent outage will be lower than the same question asked cold. Order is a design variable, not a convenience.

Order deliberately:

- put broad, framing questions before specific ones only when that framing is intended;
- put sensitive or demanding questions later, once rapport and context are established;
- avoid revealing the study's hypothesis through early questions, which cues respondents to the "right" answer;
- use randomization where order effects are likely, especially for rating batteries;
- consider carrying an order effect forward; once a respondent has been primed, later answers are contaminated.

If the order of questions could change the findings, the order is part of the instrument and must be designed, not assembled.

### Handle Branching And Skip Logic Carefully

Branching lets a survey adapt to the respondent, but each branch reduces the sample for downstream questions and introduces complexity that can route respondents to the wrong place. Logic that feels clever in the builder often fails in the field.

Design logic carefully:

- branch only when a question is genuinely irrelevant to some respondents, not to shorten the survey cosmetically;
- test every branch path, because a misrouted respondent answers questions not meant for them;
- track the effective sample size per branch, because subgroups shrink quickly and may not support the claims drawn from them;
- avoid logic so dense that respondents notice they are being routed, which can feel manipulative.

### Sample And Field For The Claim

A survey's numbers describe the sample it was fielded to, and only generalize to a population if the sampling supports it. A survey fielded to a volunteer panel describes the panel, not the user base.

Match fielding to the claim:

- for population-level claims, use a sample that represents the target population and size it for the precision required;
- report the margin of error and confidence level for proportional claims, and do not overstate precision;
- name the recruitment bias of the fielding source, because panel and volunteer samples are not representative;
- weight the sample where appropriate to correct known skew, and disclose the weighting.

A precise percentage from a biased sample is still biased. The sampling, not the decimal places, determines what the survey can claim.

### Plan The Analysis Before Fielding

The analysis should be designed before the survey is fielded, so that the items, scales, and sample actually support it. Designing the analysis after the data returns usually reveals that a needed question was never asked or a scale cannot support the intended comparison.

Plan analysis upfront:

- for each intended finding, identify which item and scale will support it;
- decide how scales will be scored and compared, and ensure the scale design permits it;
- predefine subgroups for comparison, and confirm the sample within each is adequate;
- decide how open-ended responses will be coded, because unstructured text is expensive to analyze and often underused.

### Keep The Survey Short And Respectful

Length destroys data quality. Long surveys produce fatigue, random clicking, and abandonment, and the respondents who finish are no longer representative of those who started. Respect for the respondent's time is a data quality measure, not only a courtesy.

Protect response quality:

- cut every item that does not support a planned analysis;
- prefer shorter surveys fielded more often to omnibus surveys that try to answer everything at once;
- pilot the survey end-to-end and time it, because builders routinely underestimate length;
- offer a progress indicator and keep estimated time honest.

## Common Traps

### Using A Survey To Answer A "Why" Question

Surveys describe what and how many, not motivation. Fielding a survey for a "why" question produces confident numbers around guessed causes.

### Double-Barreled Questions

Items that ask two things at once produce uninterpretable answers, because a mixed response cannot be decomposed.

### Leading Or Loaded Wording

Questions that assume an experience or signal the desired answer produce biased data that looks precise.

### Ambiguous Scale Anchors

Unlabeled numeric scales or vague frequency terms mean different things to different respondents, making cross-respondent comparison unreliable.

### Order Effects

Earlier questions prime later ones. Ignoring order produces findings shaped by sequence rather than by the construct.

### Generalizing From A Biased Sample

Percentages from a volunteer or panel sample describe that sample, not the population, regardless of how precise they look.

### Designing Analysis After Fielding

Waiting until data returns to plan analysis usually reveals missing items or unsupported scales when it is too late to fix them.

### Survey Bloat

Long surveys produce fatigue and abandonment, and the remaining respondents are no longer representative of those who started.

## Self-Check

- [ ] The question is descriptive or attitudinal and therefore surveyable, not motivational or causal, which would require a different method.
- [ ] Each item asks one neutral question in the respondent's vocabulary, with no double-barreled or leading wording.
- [ ] Scales are chosen deliberately, with labeled anchors, a justified neutral option, and concrete timeframes for frequency.
- [ ] Question order is designed to avoid contamination, with randomization where order effects are likely.
- [ ] Branching is used only where genuinely needed, every path is tested, and effective subgroup samples are tracked.
- [ ] The fielding sample can support the population claims being made, with margin of error and recruitment bias disclosed.
- [ ] The analysis was planned before fielding, and every item maps to an intended finding.
- [ ] Open-ended questions have a coding plan and are not left as an afterthought.
- [ ] The survey is as short as it can be while supporting the planned analysis, and length was checked by piloting.
- [ ] No finding is overclaimed beyond what the wording, scale, order, and sampling of the instrument can support.
