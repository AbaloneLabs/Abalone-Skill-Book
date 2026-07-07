---
name: problem_validation_vs_solution_validation.md
description: Use when the agent is deciding whether to validate that a problem exists and matters before testing solutions, distinguishing problem discovery from solution testing, choosing research methods for each stage, or diagnosing why a team built a well-validated solution to a problem no one had.
---

# Problem Validation Vs Solution Validation

The most expensive failure in product discovery is not building the wrong solution; it is building a perfectly validated solution to a problem that does not exist, does not matter, or does not belong to the people the team assumed. Problem validation and solution validation answer fundamentally different questions, require different methods, and fail in different ways, yet teams routinely collapse them into a single activity, usually by jumping to solution testing before the problem is understood. The result is a solution that tests well against a fabricated or assumed problem and then fails in the market, because the team validated the answer to a question they never actually asked.

The harm this skill prevents is the discovery process that skips problem validation entirely, treating an assumed problem as given and proceeding straight to testing solutions against it. A team that does this will produce confident-looking evidence, because solution tests generate signals, but the signals answer the wrong question. They show whether users like the solution, not whether the problem is real, frequent, painful, and worth solving. The skill is separating the two stages, validating the problem before investing in solution testing, and recognizing that a solution tested against an unvalidated problem is evidence of almost nothing.

Use this skill when planning discovery, when deciding what to validate first, when a team is debating whether to build a prototype or run problem interviews, when reviewing whether discovery evidence supports a build decision, or when diagnosing why a validated solution failed in market. The goal is to prevent the agent from conflating the two validation stages, from testing solutions before the problem is established, or from treating solution-test signals as evidence that the problem was real.

## Core Rules

### Separate The Two Questions Explicitly

Problem validation and solution validation answer different questions, and conflating them produces evidence that does not hold. State the question each piece of discovery is answering before choosing a method.

The two questions are:

- Problem validation: does this problem exist, for whom, how often, how painfully, and is it worth solving? This is about the reality, frequency, and severity of an unmet need.
- Solution validation: does this specific solution address the validated problem effectively, and will the target users adopt it in real conditions? This is about fit, usability, and willingness.

A solution test cannot validate the problem, because users will react to any plausible solution with feedback, and that feedback tells you about the solution, not about whether the underlying problem was real. A problem investigation cannot validate a solution, because understanding the need does not confirm that any particular approach will work. The methods must match the question.

### Validate The Problem Before Investing In Solution Testing

Problem validation should precede solution testing, because solution testing is more expensive and more committing. Building a prototype, running a fake-door test, or conducting a usability study all assume the problem is understood; if that assumption is wrong, the solution work is wasted. Validating the problem first ensures that solution investment is directed at a real need.

Problem validation asks:

- who specifically experiences this problem, defined precisely enough to distinguish real sufferers from adjacent audiences;
- how often does it occur, and under what conditions;
- how painful or costly is it when it occurs;
- what do people do today to cope, and how expensive is the workaround;
- is the problem worth solving relative to other problems competing for the same capacity.

If these questions are not answered, the team does not have a validated problem, and solution testing is premature. The temptation to skip ahead is strong, because solution testing feels like progress and problem investigation feels like delay, but skipping produces solutions in search of a problem.

### Use Different Methods For Each Stage

Problem validation and solution validation require different research methods, because they produce different kinds of evidence. Using solution-testing methods to validate a problem generates false confidence, because the methods are designed to elicit reactions to a proposed answer, not to investigate an open question.

Problem validation methods include:

- open-ended discovery interviews that explore the user's world without proposing a solution;
- ethnographic observation of current workflows and workarounds;
- analysis of support tickets, churn reasons, and usage data for evidence of unmet needs;
- diary studies that capture problems as they occur in context over time;
- analysis of search logs, forum discussions, and competitor reviews for expressed frustrations.

Solution validation methods include:

- prototype tests that assess comprehension, usability, and preference;
- fake-door or painted-door tests that measure expressed demand;
- concierge or wizard-of-oz tests that deliver value manually to learn whether the outcome is valued;
- A/B tests that compare a solution against a control.

The key distinction: problem methods investigate an open question without proposing an answer; solution methods test a proposed answer. Mixing them, by asking about problems while showing a solution, contaminates both, because the solution primes the respondent to report problems related to it.

### Avoid Solution-Primed Problem Investigation

The most common corruption of problem validation is investigating the problem while showing or describing a solution. The moment a respondent sees a prototype, a mockup, or even a feature description, their answers are no longer about their organic experience of the problem; they are about their reaction to the proposed solution. The solution primes what problems they notice, how severe those problems seem, and whether they report having them.

To keep problem validation clean:

- conduct problem interviews before any solution is shown or described;
- avoid leading questions that presuppose the problem exists;
- ask about past specific behavior, not about hypothetical future interest;
- listen for volunteered problems rather than prompting for the one the team expects;
- separate the sessions, so problem investigation never shares a room with solution exposure.

If the team must show a solution to get feedback, label that session as solution validation and do not treat the problem discussion within it as independent evidence. The contamination flows one way: the solution shapes the problem report, not the reverse.

### Recognize That Stated Demand Is Not Problem Validation

A common shortcut is to interpret demand signals, such as feature requests, upvotes, or fake-door clicks, as evidence that the problem is real. These signals show that people are interested in a proposed solution, which is related to but not the same as the existence of a painful problem. People request features for many reasons: curiosity, completeness bias, competitive parity, or the assumption that more is better, none of which confirm an unmet need.

Distinguish demand from problem evidence:

- a feature request tells you someone wants the feature, not that they have the problem it solves;
- a fake-door click tells you someone was curious enough to click, not that they would use or pay for the solution;
- an upvote tells you the idea sounds appealing, not that the voter experiences the pain.

Demand signals are useful for solution validation once the problem is established. They are weak evidence for problem validation, because they reflect reaction to a proposed answer rather than investigation of an open question. Triangulate demand with behavioral evidence of the underlying problem before treating it as problem validation.

### Define Problem Validation Criteria Before Investigating

Like any test, problem validation should have criteria defined in advance, or every result will be interpreted as supportive. Before investigating, specify what evidence would confirm the problem is real and worth solving, and what evidence would cause the team to abandon it.

Pre-specify:

- the target segment and how you will confirm sufferers belong to it;
- the frequency and severity thresholds that make the problem worth solving;
- the evidence of current workarounds and their cost;
- the pattern of responses that would indicate the problem is not real or not important;
- the decision that follows each possible outcome.

Without pre-specified criteria, the team will interpret ambiguous findings as promising and weak findings as directionally correct, and the problem will never be falsified. Pre-specification is what separates problem validation from problem advocacy.

### Test Solutions Against The Validated Problem, Not A General One

Once the problem is validated, solution testing should be evaluated against that specific problem, not against a general sense of whether users like the solution. A solution can test well on usability and preference while failing to address the validated problem, because it solves a different need or serves a different segment.

When testing solutions, ask:

- does this solution address the specific validated problem, or a related but different one;
- would the people who actually experience the problem adopt this solution in real conditions;
- does the solution reduce the pain or workaround cost that the problem validation identified;
- are there signals that the solution is solving a problem the users do not have, even if they like the feature.

A solution that wins praise but does not address the validated problem is a misallocation. The problem validation is the ruler against which the solution is judged, and without that ruler, solution testing optimizes for appeal rather than fit.

### Recognize When The Problem Itself Is The Hypothesis

Sometimes the team does not know what problem to solve, and the problem itself is the hypothesis. In these cases, problem validation is exploratory rather than confirmatory, and the goal is to discover what problems exist, not to confirm a specific one. Exploratory problem discovery requires even more discipline against solution-priming, because any proposed solution will narrow what problems the team can see.

For exploratory problem discovery:

- start broad and let the problems emerge from the user's world, not from the team's assumptions;
- avoid anchoring on any specific problem too early;
- look for problems that are frequent, painful, and costly to work around;
- be willing to discover that the most important problem is not the one the team expected.

Exploratory discovery is harder than confirmatory validation, because the team must hold its assumptions loosely, but it is where the most valuable problems are often found, precisely because no one was looking for them.

## Common Traps

### Jumping To Solution Testing Before Problem Validation

Testing solutions against an assumed problem produces confident-looking evidence that answers the wrong question. The solution may test well while the problem was never real.

### Using Solution-Testing Methods To Validate Problems

Showing a prototype and asking about problems contaminates the investigation, because the solution primes what problems respondents notice and report.

### Interpreting Demand Signals As Problem Evidence

Feature requests, upvotes, and clicks show interest in a solution, not the existence of a painful problem. Demand is weak evidence for problem validation.

### Conflating The Two Questions In One Activity

Trying to validate problem and solution simultaneously produces evidence that holds for neither, because the methods and the contamination differ.

### No Pre-Specified Problem Validation Criteria

Without criteria defined in advance, ambiguous findings are interpreted as supportive, and the problem is never falsified. Every investigation confirms.

### Testing Solutions Against A General Rather Than Validated Problem

A solution that tests well on appeal may not address the specific validated problem, leading to a feature that is liked but irrelevant to the real need.

### Assuming The Problem From The Solution's Appeal

Inferring that because users liked the solution, the problem must have been real. Appeal and problem validity are related but not equivalent.

### Skipping Exploratory Problem Discovery

When the problem is unknown, jumping to confirmatory validation of an assumed problem prevents the team from discovering the more important problems they were not looking for.

## Self-Check

- [ ] The question being answered, problem validation or solution validation, is stated explicitly before choosing a method.
- [ ] Problem validation precedes solution testing, so solution investment is directed at a real need.
- [ ] The methods match the question: open investigation for problems, prototype and demand tests for solutions.
- [ ] Problem investigation is conducted without showing or describing a solution, to avoid solution-priming.
- [ ] Demand signals are not treated as problem evidence without triangulation against behavioral proof of the underlying need.
- [ ] Problem validation criteria, including the disconfirming pattern, were specified before the investigation.
- [ ] Solutions are evaluated against the specific validated problem, not against general appeal.
- [ ] When the problem is unknown, exploratory discovery is conducted before confirmatory validation of an assumed problem.
- [ ] No solution-test signal is treated as evidence that the problem was real.
- [ ] The two stages are kept separate in planning, methods, and interpretation, even when conducted by the same team.
