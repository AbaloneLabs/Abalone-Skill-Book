---
name: survey_design.md
description: Use when the agent is designing a user survey, writing survey questions, choosing sampling and distribution methods, or interpreting survey results without overstating what self-reported data can prove.
---

# Survey Design

Surveys are seductive because they feel rigorous: large numbers, percentages, charts. But a survey is only as trustworthy as its questions, its sample, and the humility with which its results are read. A poorly worded question administered to a biased sample produces a precise-looking number that is precisely wrong. The harm is not wasted effort; it is overconfident decisions built on self-reported data that never reflected reality.

Use this skill before answering broad questions such as "how should we survey our users", "what questions should we ask in a customer survey", "how many responses do we need", "is NPS the right measure", or "what does this survey result actually tell us". The goal is to prevent the agent from treating self-reported answers as behavioral proof, from writing questions that manufacture their own conclusions, and from generalizing findings beyond the population that actually responded.

## Core Rules

### Decide Whether A Survey Is Even The Right Tool

A survey is a self-report instrument: it captures what people say, not what they do. Before designing one, confirm it fits the question. Surveys are strong for measuring stated attitudes, segmenting a population, ranking relative preferences, and collecting demographic or categorical context. They are weak for understanding why behavior happens, discovering problems you have not named yet, or predicting future actions.

Choose interviews when you need depth, causality, and surprise. Choose behavioral analytics when you need to know what users actually did. Choose a survey when you need breadth of stated opinion across many people at low marginal cost. Mixing methods is often the right answer; using a survey alone to answer a "why" question is usually the wrong one.

State the decision the survey is meant to inform before writing a single question. If you cannot name the decision, the survey is probably collecting data for its own sake, and scope will bloat until fatigue destroys data quality.

### Write Unbiased Questions

Question wording is where surveys quietly go wrong. Each question should measure one thing, neutrally, without steering the respondent. Audit every item for the classic failure modes: double-barreled questions ("was the feature fast and easy?") that conflate two attributes you cannot separate; leading questions ("how much did you enjoy...") that assume the response; loaded questions that embed judgment; and absolute wording ("always", "never", "do you agree") that forces respondents into corners.

Keep questions short, concrete, and at the reading level of your least expert respondent. Avoid jargon, internal product names, and acronyms unless you are certain the audience shares them. Define any term that could be interpreted differently by different respondents, because ambiguous wording turns your results into noise.

Order matters. Early questions set context and prime later answers. Put screening and demographic items where they will not bias core questions, and randomize or counterbalance where priming is a real risk. Never put a sensitive or threatening question early, where it can trigger drop-off or dishonest answers for the rest of the instrument.

### Choose Scales Deliberately

The scale is part of the question. Different scales answer different questions, and the wrong scale obscures your findings.

Likert scales (agree-disagree) measure attitude intensity and are good for tracking sentiment over time, but they suffer from acquiescence bias: respondents tend to agree regardless of content. Balance them with equal positive and negative poles and a clear neutral midpoint, and be aware that "agree" wording can inflate positive responses. Numeric rating scales (1-10) feel precise but the meaning of a "7" differs wildly between people.

NPS is a loyalty proxy, not a satisfaction or quality measure. It is a single composite that hides why someone scored low or high, and its "would recommend" framing captures stated intent, not actual referral behavior. Use it as a tracked trend, not as proof that a feature is good. Multiple choice forces categorization and is easy to analyze, but it constrains answers; always consider whether an open text field would reveal something your fixed options hide.

Match the scale to the decision. If you need to track change over time, pick a stable, comparable measure. If you need to rank priorities, use forced-choice or constant-sum rather than letting respondents rate everything as important.

### Account For Sampling And Selection Bias

Your results describe the people who responded, not necessarily the people you care about. Selection bias enters the moment the people who see and complete your survey differ systematically from the population you want to understand. Active, satisfied, or highly engaged users are overrepresented in in-product surveys; frustrated users who churned never see them at all.

Define the target population explicitly, then judge how your distribution channel skews it. An email to your entire customer base reaches a different group than a popup shown only to logged-in power users, and both differ from a survey posted in your most enthusiastic community forum. Document the sampling frame so readers can judge generalizability.

Where possible, use random sampling or stratified sampling to reach subgroups proportionally. Convenience samples can still be informative for exploration, but they cannot support strong claims about "our users" as a whole. Be honest about which group your numbers actually represent.

### Interpret Response Bias And Non-Response

Even among those who start a survey, not all answers are equal. Non-response bias means the people who skip the survey or abandon it halfway may differ from those who finish, and the difference is often the thing that matters most: the dissatisfied, the confused, the busy. Track completion and drop-off rates; a question with high abandonment is itself a signal.

Response bias covers the systematic distortions in how people answer: acquiescence (agreeing to be agreeable), extreme responding, social desirability (overstating virtuous behavior), and straight-lining through a grid without reading. Long surveys, repetitive grids, and unclear questions all amplify these effects. Keep the instrument short, group related items logically, and use attention checks sparingly rather than as a crutch for poor design.

Anonymity and tone affect honesty. Respondents answer more truthfully about sensitive topics when they trust the data is anonymous and will not be used against them. Be transparent about how responses are used and who sees them.

### Distinguish Statistical Confidence From Practical Significance

Sample size determines how precisely you can estimate a value, not whether that value matters. A large sample lets you detect tiny differences that are statistically significant but practically meaningless; a small sample leaves wide margins of error that hide real differences. Report confidence intervals and margins of error, not just point estimates, and be explicit about the assumptions behind them.

Subgroup analysis multiplies your risk of false positives. Slicing the same data many ways will, by chance alone, produce "significant" differences that do not replicate. Decide which subgroups matter in advance, and treat post-hoc slices as hypotheses, not findings.

Most product surveys do not need formal hypothesis testing; they need honest ranges and clear language about uncertainty. Never present "62% said yes" as a precise fact without the context of who was asked, who answered, and how wide the uncertainty is.

### Pilot The Instrument Before Launch

A survey you have not tested is a survey you do not understand. Pilot it with a small group, ideally including people who match your target population but were not involved in writing it. Watch for questions that are misread, skipped, or answered in ways that reveal ambiguity you did not foresee.

Time the pilot to estimate completion duration, then cut ruthlessly. Every extra question increases fatigue and reduces the quality of every answer that follows. A common rule is that respondents give less effort to later items; a long survey degrades its own data. Prefer a shorter, higher-quality instrument over a comprehensive one that no one finishes carefully.

### Respect Consent And Data Minimization

Collect only what you will actually use. Each additional field is a privacy cost, a compliance risk, and a friction point that lowers response quality. Identify the decisions the data must inform and cut anything that does not serve them. Free-text fields can contain personally identifying information; plan how to store, review, and share them.

Be transparent about purpose, data retention, and who can access responses. If you combine survey data with behavioral or account data, the linkage raises the stakes and may require explicit consent. Default to the most privacy-protective option that still answers the question.

## Common Traps

### Treating Self-Report As Behavioral Truth

The deepest trap in survey work is forgetting that responses are what people said, not what they did. Stated preference and revealed preference routinely diverge; people overstate their intent, their virtue, and their future behavior. The trap is building a roadmap on "75% said they would use this" and then being surprised by low adoption. Always pair survey claims with behavioral evidence before acting.

### Manufacturing The Conclusion In The Question

A leading or loaded question produces the answer you wanted and then lets you report it as if it were a finding. "How valuable did you find our improved onboarding?" assumes improvement and value. The trap is that the resulting chart looks authoritative while measuring nothing real. Rewrite to neutral phrasing and let respondents define their own scale of value.

### Generalizing Beyond The Sample

A clean result from a biased sample still only describes that sample. The trap is reporting "users want X" when only the most engaged users, or only users of a specific plan, were surveyed. Always state the population the results actually represent and resist extrapolation to groups who were not sampled.

### Confusing A High Score With Product Quality

A high satisfaction or NPS score feels like validation, but it can reflect a loyal niche, a low-expectation baseline, or survivorship among those who did not churn. The trap is reading a good number as proof the product is good and stopping inquiry. Use scores as trend signals to investigate, not as trophies.

### Overloading The Survey Until It Self-Destructs

Every stakeholder wants their question added. The trap is the survey grows until respondents fatigue, rush, straight-line, or abandon, degrading every answer including the important ones. Treat length as a threat to data quality and cut aggressively.

### Over-Reading Subgroup Slices

Slicing data by segment, persona, or plan until something "significant" appears is fishing, not analysis. The trap is presenting a chance difference as a strategic insight. Pre-specify comparisons and treat exploratory slices as questions for the next study, not as decisions today.

## Self-Check

- [ ] The survey is confirmed as the right tool for the question, versus interviews or behavioral data.
- [ ] A specific decision the survey must inform is named before any question is written.
- [ ] Each question measures one thing, is neutrally worded, and is free of double-barreled, leading, loaded, or absolute phrasing.
- [ ] The chosen scale matches the decision, and its known biases (acquiescence, NPS limits) are acknowledged.
- [ ] The target population is defined and the sampling frame's skew is documented.
- [ ] Non-response and response bias are considered, and completion or drop-off rates are tracked.
- [ ] Results are reported with margins of error or confidence intervals, not bare point estimates.
- [ ] Subgroup comparisons are pre-specified, and post-hoc slices are treated as hypotheses.
- [ ] The instrument was piloted and timed, and length was cut to protect response quality.
- [ ] Findings are framed as stated preference, and behavioral evidence is sought before treating them as proven.
