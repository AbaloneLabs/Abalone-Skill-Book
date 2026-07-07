---
name: heuristic_evaluation.md
description: Use when the agent is conducting a heuristic evaluation or expert review, choosing which usability heuristics to apply, structuring an inspection of an interface against recognized principles, deciding when expert review is appropriate versus user testing, or synthesizing expert findings into actionable severity-ranked issues.
---

# Heuristic Evaluation

Heuristic evaluation is an inspection method where evaluators assess an interface against recognized usability principles (heuristics) to find problems without involving users. It looks like a checklist review, but it is really a disciplined expert judgment that is fast, cheap, and fallible. Agents tend to treat it as a substitute for user testing, run it with a single evaluator, or apply heuristics so loosely that findings reflect personal preference rather than principle. The harm is a list of "issues" that are really opinions, or worse, confidence that the design is sound because an expert looked at it, when expert review systematically misses problems only users reveal.

Use this skill before conducting a heuristic evaluation or relying on its results. The goal is to prevent the agent from over-claiming what inspection can establish, from confusing preference with principle, or from running a review whose findings cannot be trusted.

## Core Rules

### Use Heuristic Evaluation For What It Can Establish, And Acknowledge What It Cannot

Heuristic evaluation is excellent at finding certain problems quickly: violations of consistency, unclear feedback, poor error prevention, lack of user control. It is poor at finding others: problems rooted in user knowledge, workflow, or domain mental models that experts cannot anticipate. Treating it as a complete substitute for user testing produces false confidence.

Match the method to its strengths:

- use heuristic evaluation early and often, as a cheap way to catch obvious problems before investing in user testing;
- use it to triage a design before spending participant budget on issues an expert would catch;
- do not use it as the sole evidence for a launch decision, especially for novel interfaces or unfamiliar user populations;
- always follow with user testing for problems only real users reveal.

Heuristic evaluation finds a large share of usability problems at low cost, but it both misses problems only users hit and flags "problems" that users do not actually experience. It is a first pass, not a final verdict.

### Use Multiple Evaluators, Not One

The single most documented finding about heuristic evaluation is that individual evaluators find only a fraction of the problems, and different evaluators find different problems. One evaluator, no matter how expert, misses most issues. The method's reliability comes from aggregating multiple independent evaluations.

Use multiple evaluators:

- use three to five evaluators for a practical balance of coverage and cost;
- have evaluators inspect independently first, so their findings are not biased by each other;
- aggregate findings after independent evaluation, deduplicating and reconciling;
- recognize that even five evaluators miss problems, especially less obvious ones.

A single-evaluator review is not a heuristic evaluation; it is one person's opinion. The method's value is in the aggregation of independent expert judgments.

### Apply Recognized Heuristics, Not Personal Preference

Heuristics are established principles (Nielsen's ten, for example) that have empirical backing and shared meaning among practitioners. Personal preferences ("I do not like this color") are not heuristics and produce findings that reflect taste rather than usability. The discipline is grounding every finding in a named, recognized principle.

Ground findings in principle:

- cite the specific heuristic each finding violates (visibility of system status, user control and freedom, consistency and standards);
- distinguish a heuristic violation from a personal preference, and exclude or label the latter;
- when a design choice is a genuine tradeoff between heuristics, name both and explain the judgment;
- avoid inventing idiosyncratic "heuristics" that are really opinions.

A finding tied to a recognized heuristic carries the weight of established principle. A finding tied to preference carries only the evaluator's taste. The distinction determines whether the team should act.

### Evaluate Against The Real User And Task Context

Heuristics are general, but their application depends on context. What is a problem for a novice may be fine for an expert; what violates a heuristic in a consumer app may be acceptable in a specialized tool used daily by trained users. Evaluating without the user and task context produces findings that are technically correct but practically wrong.

Contextualize the evaluation:

- define the target users and their expertise before evaluating;
- evaluate against the real tasks users perform, not abstract interface inspection;
- weight findings by how they affect the actual user population and critical tasks;
- recognize that some heuristic violations are acceptable tradeoffs in specific domains.

A finding that ignores context may send the team fixing something that does not matter to the real users, while missing what does.

### Rate Severity To Prioritize Action

A list of heuristic violations without severity is unactionable: the team cannot tell what to fix first. Severity rating, based on the problem's impact and frequency, turns a raw list into a prioritized set. Without it, teams either fix everything (wasting effort on minor issues) or fix nothing (overwhelmed by the list).

Rate every finding:

- assess impact: does the problem block the task, cause errors, or merely annoy;
- assess frequency: how often will users encounter it (recognizing expert review estimates frequency poorly);
- combine into a severity rating (critical, major, minor, cosmetic);
- focus recommendations on critical and major issues first.

Severity rating is subjective and should be treated as a guide, not a measurement. But a rough, consistent ranking is far more useful than an undifferentiated list.

### Distinguish Real Problems From False Positives

Heuristic evaluation has a known weakness: it flags problems that users do not actually experience. Experts, evaluating outside real use, can identify "violations" that real users navigate without trouble. Reporting these as definitive problems wastes the team's effort and erodes credibility.

Manage false positives:

- flag findings as suspected rather than confirmed, since inspection cannot verify user impact;
- recommend user testing for findings that are ambiguous or high-stakes;
- track which heuristic findings user testing confirms and which it does not, to calibrate future evaluations;
- be honest with stakeholders that expert review over-identifies problems relative to user testing.

Honesty about false positives keeps the method credible. Claiming every heuristic violation is a real problem invites dismissal when some prove harmless.

### Document Findings With Evidence And Location

A finding that says "the feedback is unclear" is hard to act on. A finding that says "on the payment screen, submitting an invalid card number shows no error message, violating visibility of system status" is actionable. Specific, located, evidence-grounded findings are what make the evaluation useful.

Document completely:

- state the specific location (screen, element, flow) of each finding;
- describe what was observed and which heuristic it violates;
- explain the likely user impact;
- propose a direction for resolution, labeled as a suggestion.

Vague findings generate debate about whether they are real. Specific findings generate action. The documentation quality determines whether the evaluation has impact.

## Common Traps

### Treating Expert Review As A Substitute For User Testing

Heuristic evaluation misses problems only users reveal and flags some users do not experience; use it as a first pass, not a final verdict.

### Using A Single Evaluator

One evaluator finds only a fraction of problems; use three to five independent evaluators and aggregate.

### Applying Personal Preference As Heuristic

Findings grounded in taste rather than recognized principle carry no authority; cite specific heuristics for each finding.

### Evaluating Without User And Task Context

A heuristic violation for one user population may be acceptable for another; contextualize findings against real users and tasks.

### An Unranked List Of Findings

Without severity ratings, the team cannot prioritize; rate every finding by impact and frequency.

### Reporting False Positives As Definitive

Expert review over-identifies problems; flag findings as suspected and verify high-stakes ones with user testing.

### Vague, Unlocated Findings

"Feedback is unclear" is unactionable; document each finding with its location, observation, heuristic, impact, and a suggested direction.

### Ignoring Heuristic Tradeoffs

When heuristics conflict, pretending there is one right answer hides the real judgment; name the tradeoff and explain the decision.

## Self-Check

- [ ] Heuristic evaluation is used as a fast first pass, not as the sole evidence for decisions, with user testing planned for what inspection cannot reveal.
- [ ] Three to five evaluators inspected independently before findings were aggregated and reconciled.
- [ ] Every finding is tied to a recognized, named heuristic, not personal preference.
- [ ] The evaluation was contextualized against the real target users, their expertise, and the critical tasks.
- [ ] Each finding has a severity rating based on impact and frequency, guiding prioritization.
- [ ] Findings are flagged as suspected where user impact is uncertain, with high-stakes ones recommended for user testing.
- [ ] Each finding documents its specific location, observation, violated heuristic, likely user impact, and a suggested resolution direction.
- [ ] Where heuristics conflict, the tradeoff is named and the judgment explained rather than hidden.
