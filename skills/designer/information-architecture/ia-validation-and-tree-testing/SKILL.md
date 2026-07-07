---
name: ia_validation_and_tree_testing.md
description: Use when the agent is validating an information architecture, running a tree test or reverse card sort, measuring findability and task success across a proposed IA, deciding whether to ship or rework a taxonomy, comparing alternative IA structures, interpreting first-click and task-completion metrics, or judging whether an IA validation result can actually support the decision to ship a navigation or labeling structure.
---

# IA Validation And Tree Testing

An information architecture that feels logical to the team can still fail real users, because the team's mental model of the content is not the user's. IA validation is the discipline of testing a proposed structure before it is built, so that findability is measured rather than assumed. The core method is the tree test: a stripped-down, text-only version of the structure through which users attempt real find-the-destination tasks. Its value is that it isolates the IA from visual design, navigation chrome, and search, so the result speaks to the structure itself rather than to the interface around it.

Agents tend to fail IA validation in characteristic ways. They declare the IA done because the card sort produced a tidy hierarchy, never testing whether users can actually navigate it. They run a tree test but write tasks that telegraph the correct path, so the result confirms the structure without challenging it. They compare a single proposed IA against no alternative, so they cannot tell whether a different structure would do better. Or they read a tree test's task-success rate as a verdict on the whole product, ignoring that findability is task-specific and that a strong IA for one task may be weak for another.

Use this skill before locking an IA, while designing a tree test, and while interpreting its results. The goal is to measure whether users can find what they need under the proposed structure, to compare structures where the decision warrants it, and to bound the findings to the tasks and labels actually tested.

## Core Rules

### Validate The IA Before Building The Interface

Building the full interface before validating the IA conflates two questions: is the structure findable, and is the visual design usable. When both are tested together, a failure cannot be attributed, and the team often reworks the visual design when the real problem was the structure. Validating the IA in isolation lets structure be fixed while it is still cheap to change.

Sequence validation early:

- run tree tests on the proposed IA before building high-fidelity navigation;
- treat the card sort as a generative input that proposes a structure, and the tree test as the evaluative check that confirms it;
- re-validate after structural changes, because edits to labels or grouping can break findability that earlier tests supported.

A structure that is only tested after it is shipped is a structure that is validated by the support tickets.

### Write Tasks That Test The Structure, Not Telegraph It

The task is the instrument, and a badly written task can make any IA look good. Tasks that name the destination, mirror the label, or describe the path tell the user where to go and measure nothing.

Write tasks that probe the structure:

- phrase tasks in the user's language and goal, not in the system's terminology, so the user must translate their goal into the structure;
- avoid using the destination's exact label in the task, because matching words is not finding;
- use realistic, specific tasks tied to genuine user goals rather than abstract "find X" instructions;
- include tasks of varying difficulty and frequency, because findability is not uniform across the structure.

Pre-test tasks by asking a few people where they would look; if the task itself reveals the answer, rewrite it.

### Measure The Right Outcomes

Tree tests produce several metrics, and reading the wrong one leads to false confidence. Task success alone can hide where users struggled, and time can reward guessing. The combination of metrics tells the story.

Track the combined picture:

- task success rate, the share of users who reached the correct destination;
- directness, the share of successful users who went straight there without backtracking, which reveals confusion even when they eventually succeeded;
- time on task, which flags destinations that are findable but slow;
- the distribution of wrong destinations, which is often more diagnostic than the success rate, because it shows which labels are misleading.

A 70% success rate with a direct path is a different structure from a 70% success rate achieved through wandering.

### Compare Alternatives When The Decision Warrants It

A single IA tested in isolation can only tell you whether it works, not whether it is the best available structure. When the decision is consequential, compare candidate structures head to head.

Compare deliberately:

- when restructuring a large product or choosing between meaningfully different models, test two or more candidate IAs with comparable samples;
- keep task sets and samples equivalent across conditions, so differences are attributable to the structure;
- weigh the comparison against implementation cost, because a marginally better IA may not justify a costly migration;
- for low-stakes or evolutionary changes, a single validated IA may suffice, and over-testing trivial decisions wastes effort.

### Isolate Labeling From Structure

A tree test failure can mean two different things: the structure is wrong, or the structure is right but the labels mislead. Confusing them leads to rework that does not fix the problem.

Diagnose before fixing:

- if users go to the wrong destination but consistently choose the same wrong one, the label is likely the culprit, not the structure;
- if users scatter across many wrong destinations, the structure itself is probably unclear;
- test label revisions more cheaply than structural revisions, so rule out labeling first;
- when revising labels, re-test, because a clearer label in one place can obscure a destination elsewhere.

### Bound Findings To Tasks And Labels Tested

A tree test speaks only to the tasks and labels it included. Generalizing a strong result to the whole product, or to tasks not tested, overclaims what the method can establish.

Bound the claims:

- report which tasks the IA supports well and which it does not, rather than a single overall score;
- acknowledge that tree tests exclude search, visual navigation, and real interface behavior, so they validate structure, not the full experience;
- treat untested tasks as unvalidated, and flag high-stakes tasks that were not covered;
- pair tree testing with usability testing of the built interface, because findability in a text tree and findability in a real product differ.

### Size The Sample To The Comparison

Tree test samples need to be large enough to distinguish a real difference from noise, especially when comparing alternatives. Tiny samples produce volatile success rates that look precise.

Size deliberately:

- for a single-structure check, a modest sample can reveal severe problems reliably;
- for comparisons between structures, size for the smallest difference that would change the decision;
- do not attach precise percentages to samples too small to support them, and prefer confidence intervals over point estimates.

## Common Traps

### Skipping Validation Because The Card Sort Looked Tidy

A card sort proposes a structure; it does not prove users can navigate it. Validate before building.

### Tasks That Telegraph The Path

Tasks that name the destination or mirror the label measure matching, not finding, and make any IA look good.

### Reading Success Rate Alone

Success rate hides directness and the distribution of wrong answers, which are often more diagnostic than the headline number.

### Testing A Single IA In Isolation

Without a comparison, a passing result cannot tell you whether a better structure exists, which matters for consequential decisions.

### Confusing Labeling Failures With Structural Failures

A misleading label can sink a sound structure. Diagnose whether users scatter or converge on the same wrong destination before restructuring.

### Generalizing To Untested Tasks

A tree test validates only the tasks and labels it included. Treat untested destinations as unvalidated.

### Over-Testing Trivial Changes

Not every label tweak warrants a full tree test. Match the validation investment to the cost of being wrong.

### Treating The Tree Test As The Whole Experience

Tree tests exclude search, visual navigation, and real interface behavior. Pair them with usability testing of the built product.

## Self-Check

- [ ] The IA was validated in isolation, through a tree test or equivalent, before the full interface was built.
- [ ] Tasks are written in the user's language and goal, without naming destinations or mirroring labels, and were pre-tested for telegraphing.
- [ ] Outcomes tracked include success rate, directness, time, and the distribution of wrong destinations, not success alone.
- [ ] Alternative structures were compared when the decision was consequential, with equivalent tasks and samples across conditions.
- [ ] Labeling failures were distinguished from structural failures before deciding to rework the structure.
- [ ] Findings are bounded to the tasks and labels tested, and untested destinations are flagged as unvalidated.
- [ ] The sample is sized to the comparison, and precise percentages are not attached to samples too small to support them.
- [ ] High-stakes tasks were explicitly covered, and the validation plan reflects their risk.
- [ ] The tree test result is paired with downstream usability testing of the built interface, not treated as the whole experience.
- [ ] The decision to ship or rework the IA is supported by the specific tasks the validation covered, not by a single overall score.
