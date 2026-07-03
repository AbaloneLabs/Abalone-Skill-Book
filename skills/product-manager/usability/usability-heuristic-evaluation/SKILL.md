---
name: usability_heuristic_evaluation.md
description: Use when the agent is conducting or commissioning a heuristic evaluation, applying usability heuristics to a design, deciding which heuristics apply, or determining how to use expert review alongside user testing without overclaiming what each method reveals.
---

# Usability Heuristic Evaluation

Heuristic evaluation is an expert inspection method in which trained reviewers examine an interface against a set of established usability principles to find likely problems before, or in addition to, testing with real users. It is fast, cheap, and useful, and it is consistently misused in two opposite ways. The first misuse is treating it as a substitute for user testing, concluding that because experts found no issues, users will have none. The second misuse is applying heuristics mechanically, generating a long list of minor violations that drown the few problems that actually matter.

The value of heuristic evaluation is that it catches obvious friction cheaply and early, focuses subsequent user testing on the non-obvious issues, and gives the team a shared vocabulary for discussing usability. Its limitation is that experts are not users, they bring knowledge that real users lack and miss context that real users have, so their findings are hypotheses about problems, not confirmed problems. A product manager who understands both the power and the limits uses heuristic evaluation as one input among several, not as a verdict.

Use this skill before running or commissioning a heuristic evaluation, before applying usability heuristics to a design review, or when deciding how much weight to give expert findings relative to user research. Ask: are the evaluators actually qualified to apply heuristics, or are they offering opinions dressed as principles? Is the evaluation being used to supplement user testing or to replace it? Are the findings prioritized by likely user impact, or is every heuristic violation treated as equal?

## Core Rules

### Use Heuristics As A Lens, Not A Checklist

Established usability heuristics, such as visibility of system status, match between system and real world, user control and freedom, consistency and standards, error prevention, recognition rather than recall, flexibility and efficiency of use, aesthetic and minimalist design, help users recognize and recover from errors, and help and documentation, are lenses for noticing problems. They are not a scoring rubric where more violations means a worse design.

The right way to use heuristics is to move through the interface asking, for each one, whether this principle reveals a likely user difficulty. A heuristic is useful when it draws attention to a problem you would otherwise miss. It is not useful when it generates a violation for its own sake, divorced from whether a real user would actually struggle. Always connect a heuristic finding to a concrete user consequence: what would the user do, or fail to do, because of this? If you cannot articulate the user impact, the finding is theoretical.

### Use Multiple Independent Evaluators

A single evaluator, no matter how expert, finds only a fraction of the problems that multiple evaluators find, because each person has blind spots. The research on heuristic evaluation consistently shows that three to five independent evaluators find substantially more issues than one, and that their findings overlap only partially. Conducting a heuristic evaluation with a single reviewer, or with reviewers who discuss as they go and converge on the same findings, sacrifices most of the method's value.

Have evaluators inspect independently first, then aggregate their findings. The aggregation itself is informative: issues found by multiple evaluators are more likely to be real, and issues found by only one may be worth probing in user testing. Do not let evaluators influence each other during the inspection, because that collapses the diversity of perspectives that makes the method work.

### Connect Each Finding To A Specific User Scenario

A heuristic violation becomes actionable only when you can describe the user scenario in which it causes harm. "Low contrast on the secondary button violates the aesthetic principle" is a weak finding. "A user completing the form on a phone in bright sunlight may not see the secondary action and abandon the task" is a strong finding, because it names the user, the context, and the consequence.

For every finding, construct the scenario. Who is the user? What are they trying to do? What happens because of this issue? How severe is the consequence? This discipline filters out findings that are technically heuristic violations but have no real user impact, and it prioritizes the ones that do. It also makes the findings easier to act on, because the designer can see exactly what problem to solve.

### Prioritize By Severity, Not By Count

A long list of heuristic violations is not useful unless it is prioritized. A single severe issue, one that blocks users from completing a primary task, outweighs dozens of minor cosmetic violations. Severity should account for frequency (how many users hit it), impact (how badly it blocks them), and persistence (whether users can recover or are stopped).

Use a simple severity scale and apply it consistently. Critical issues block task completion or cause data loss and must be fixed. Major issues cause significant friction or errors and should be fixed. Minor issues are annoyances that can be deferred. Cosmetic issues are preferences that may not warrant effort. Without prioritization, the team either tries to fix everything and stalls, or fixes the easy items and leaves the severe ones.

### Do Not Treat Expert Findings As Confirmed Problems

The fundamental limitation of heuristic evaluation is that experts are not users. Experts know the conventions, anticipate the errors, and bring domain knowledge that real users lack, so they may miss problems that real users hit. They also flag issues that real users, with their specific context and motivations, would never actually encounter. Expert findings are hypotheses about problems, not confirmed problems.

This means heuristic evaluation cannot substitute for user testing when the stakes are high. It is excellent for catching obvious issues cheaply and for focusing user testing on the uncertain areas. It is dangerous when used to claim a design is usable because experts found no major issues. The honest framing is that heuristic evaluation reduces the risk of shipping obvious problems; it does not prove usability.

### Combine With User Testing Deliberately

The strongest research programs use heuristic evaluation and user testing together, in a sequence that plays to each method's strengths. Heuristic evaluation early catches the obvious problems cheaply, so that user testing is not wasted on issues the team could have found themselves. User testing then reveals the non-obvious problems that experts miss and confirms which expert hypotheses are real.

Plan the combination intentionally. Run heuristics first, fix the clear issues, then test with users to find what heuristics missed. Or run them in parallel and triangulate: issues found by both are high confidence, issues found only by users are real problems experts missed, and issues found only by experts are hypotheses to probe. Treating the methods as alternatives rather than complements wastes the value of each.

### Acknowledge What Heuristics Cannot Catch

Heuristics are good at catching interface-level friction: unclear labels, inconsistent patterns, missing feedback, error-prone interactions. They are poor at catching whether the product solves the right problem, whether the flow matches the user's mental model, whether the value proposition is clear, and whether the design works in the user's real context with real data and real interruptions. These require user research, behavioral data, and field observation.

Be explicit about the boundary. If the team is relying on heuristic evaluation to validate a redesign, name what it cannot tell you and plan other methods to fill the gap. Overclaiming what heuristics reveal leads to confident decisions built on incomplete evidence.

## Common Traps

### Mechanical Checklist Application

Running through heuristics and logging every technical violation, regardless of user impact, produces noise. The trap is mistaking thoroughness for usefulness.

### Single-Evaluator Overconfidence

One expert's findings are a fraction of what multiple evaluators would find. The trap is treating a single review as comprehensive when it is partial.

### Findings Without Scenarios

Heuristic violations without a user scenario cannot be prioritized or acted on. The trap is delivering a list of principles violated rather than a list of user harms.

### Treating Equal Count As Equal Severity

A long list of minor issues feels like a damning verdict but may matter less than one severe issue. The trap is prioritizing by quantity rather than impact.

### Substituting Experts For Users

Claiming usability based on expert review alone ignores what experts cannot catch. The trap is false confidence from a method that cannot validate the things that matter most.

### Ignoring What Heuristics Miss

Problem fit, mental model match, and real-context behavior are invisible to heuristic evaluation. The trap is assuming an interface that passes heuristic review also works for real users in real situations.

## Self-Check

- [ ] Heuristics were used as lenses to find likely user problems, not as a scoring checklist where more violations means worse.
- [ ] Multiple evaluators inspected independently before findings were aggregated.
- [ ] Each finding is connected to a specific user scenario describing who is affected, in what context, and with what consequence.
- [ ] Findings are prioritized by severity (frequency, impact, persistence), not by count.
- [ ] Expert findings are treated as hypotheses, not as confirmed problems, especially for high-stakes decisions.
- [ ] Heuristic evaluation is combined with user testing in a deliberate sequence, not used as a replacement.
- [ ] The boundaries of what heuristics can and cannot catch were acknowledged, and other methods cover the gaps.
- [ ] Findings were aggregated so that issues confirmed by multiple evaluators are distinguished from single-evaluator hypotheses.
- [ ] The evaluation focused on user consequences, not on principle violations for their own sake.
- [ ] No claim of usability was made based solely on expert review without user evidence.
