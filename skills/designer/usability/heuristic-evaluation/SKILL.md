---
name: heuristic_evaluation.md
description: Use when the agent is conducting a heuristic or expert evaluation of an interface, applying usability heuristics to find problems, running a design critique against recognized principles, prioritizing heuristic findings by severity, or distinguishing genuine usability defects from subjective taste so the evaluation produces actionable issues rather than opinion.
---

# Heuristic Evaluation

Heuristic evaluation is a fast, expert-based method for finding usability problems before or alongside user testing, but its value depends entirely on rigor. Done well, it surfaces a high proportion of real problems at low cost. Done poorly, it produces a list of opinions dressed up as principles, where every evaluator's personal preference is presented as a usability law, and where the team loses trust in the method. The judgment problem is applying recognized heuristics as diagnostic tools rather than as ammunition for taste, distinguishing genuine defects from subjective disagreement, and being honest about what the method can and cannot establish. Agents tend to fail by treating heuristics as a checklist to rubber-stamp, by confusing "I do not like this" with "this violates a principle," and by overclaiming expert findings as if they replaced testing with real users.

Use this skill when evaluating an interface against usability heuristics, running an expert review or design critique, or prioritizing heuristic findings. The goal is a credible, severity-ranked list of real problems that complements, rather than replaces, empirical testing.

## Core Rules

### Use Recognized Heuristics As Diagnostic Lenses, Not As Personal Taste

Heuristics are established, named principles, such as visibility of system status, match between system and real world, user control and freedom, consistency, error prevention, recognition over recall, flexibility, aesthetic and minimal design, error recovery, and help. Use them as structured lenses to interrogate the interface, not as a vocabulary to justify preference.

Apply heuristics as lenses:

- for each heuristic, ask what specific evidence in the interface violates it;
- name the heuristic explicitly so the finding is checkable;
- distinguish a genuine violation from a design choice you merely dislike.

"I find this color unpleasant" is taste; "the error state relies on red text alone, violating the principle that information should not depend on a single sensory cue" is a heuristic finding. If you cannot tie the observation to a named principle with concrete evidence, it is not a heuristic finding.

### Ground Every Finding In Specific Interface Evidence

A heuristic finding without a specific location is unactionable. For each problem, cite the exact screen, flow, element, or interaction where it occurs, and describe the specific behavior that violates the principle.

Each finding should specify:

- the exact location in the interface;
- the specific behavior or element that is problematic;
- the heuristic it violates;
- the user consequence, what would go wrong because of it.

Vague findings like "the navigation is confusing" cannot be acted on or checked. Findings rooted in specific evidence let the team verify the problem and design a fix.

### Evaluate Independently Before Aggregating

The strength of heuristic evaluation comes from multiple independent evaluators, because each notices different problems. If evaluators discuss before evaluating, they converge on the same findings and lose the benefit of independent perspectives.

Run the evaluation in stages:

- each evaluator reviews the interface independently and records findings;
- evaluators then aggregate, de-duplicate, and reconcile;
- disagreements are discussed, not suppressed, because they reveal genuine ambiguity.

Independent evaluation followed by structured aggregation finds far more problems than a group walk-through, where the first confident voice shapes everyone else's observations.

### Distinguish Usability Defects From Subjective Preferences

Not every objection is a usability problem. A core discipline of heuristic evaluation is separating defects that will hurt users from choices that are merely different from what the evaluator would have done.

Separate these categories:

- usability defects, which violate a heuristic and have a real user consequence;
- consistency issues, which break patterns the product or platform has established;
- preferences, which reflect the evaluator's taste but cause no user harm.

Label each finding by category. Presenting preferences as defects inflates the list, erodes credibility, and buries the real problems under debatable opinions. A finding the team can dismiss as taste should never have been filed as a defect.

### Prioritize By Severity, Not By Count

A long list of equal-weight findings is paralyzing. Rank each problem by severity so the team knows what to fix first.

Assess severity by combining:

- impact: how badly the problem blocks the user's goal;
- frequency: how often users will encounter it;
- persistence: whether users can recover or are permanently stuck;
- scope: how many flows or users are affected.

A rare cosmetic issue is not the same as a frequent blocker on a core task, and the report must make that distinction visible. Severity ranking turns a wall of findings into a prioritized plan.

### Be Honest About What Heuristic Evaluation Cannot Establish

Heuristic evaluation finds candidate problems; it does not confirm how real users behave. An evaluator's prediction that users will fail is a hypothesis, not a finding.

Recognize the method's limits:

- it cannot measure how often a problem actually occurs;
- it cannot prove a design works, only that experts did not spot a problem;
- it is biased toward problems experts notice, which may differ from problems real users hit;
- it can miss issues that depend on real context, content, or motivation.

State these limits in the report. Pair heuristic evaluation with usability testing wherever the stakes are high, because the two methods catch different problems.

### Connect Findings To Actionable Fixes Where Possible

A finding that names a problem and a heuristic is useful; one that also suggests a direction is more useful. For each significant finding, propose the kind of fix the heuristic implies, without over-specifying the solution and closing the design space.

For each finding, consider offering:

- the principle-based direction for a fix;
- a reference to how comparable interfaces handle the same issue;
- a note on what the fix must preserve, such as existing consistency.

Keep fix suggestions as directions, not mandates, so the design team retains room to solve the problem well.

## Common Traps

### Using Heuristics To Justify Taste

Dressing a personal preference in principle language inflates the list and erodes trust. Tie every finding to a named heuristic with evidence.

### Vague Findings Without Location

"Navigation is confusing" cannot be fixed or checked. Cite the exact element and behavior.

### Group Evaluation That Suppresses Independence

Discussing before evaluating makes everyone converge. Evaluate independently first, then aggregate.

### Presenting Preferences As Defects

Mixing taste with real problems buries the actionable findings. Label each finding by category.

### Equal-Weight Findings With No Severity

A flat list is paralyzing. Rank by impact, frequency, persistence, and scope.

### Overclaiming Expert Findings As Proof

Heuristic evaluation predicts problems; it does not confirm user behavior. Pair with testing for high-stakes decisions.

### Skipping The User Consequence

A finding that cites a heuristic but not what would go wrong for the user is incomplete. Always state the consequence.

## Self-Check

- [ ] Each finding is tied to a recognized, named heuristic with specific interface evidence.
- [ ] Every finding cites the exact location and the specific behavior that violates the principle.
- [ ] Evaluators reviewed independently before aggregating and reconciling.
- [ ] Usability defects, consistency issues, and subjective preferences are labeled separately.
- [ ] Findings are ranked by severity, combining impact, frequency, persistence, and scope.
- [ ] The report states what heuristic evaluation cannot establish and does not overclaim proof.
- [ ] Each finding states the user consequence, not just the violated principle.
- [ ] Significant findings include a direction for a fix that preserves relevant constraints.
- [ ] No finding is presented as a defect when it is only a preference.
- [ ] The evaluation is framed as complementary to user testing, not a replacement for it.
