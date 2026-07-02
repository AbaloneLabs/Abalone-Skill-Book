---
name: code_smell_detection.md
description: Use when the agent is detecting code smells, prioritizing which smells to fix, distinguishing accidental complexity from intentional design, or interpreting the limits of automated smell-detection tools such as linters and complexity metrics.
---

# Code Smell Detection

Code smells are not bugs; they are signals that something in the design may be wrong. A long method, a God class, a duplicated block, a feature envy—each is a symptom that hints at a deeper structural problem, and the skill is reading the symptom accurately rather than mechanically "fixing" every instance. Treating every smell as a defect to be eliminated produces churn without value; ignoring smells lets real design rot accumulate. The judgment is in telling the difference, prioritizing what matters, and understanding the root cause rather than the surface symptom.

The judgment problem is recognizing the smells that indicate genuine design decay, ranking them by the harm they cause, distinguishing smells that are intentional tradeoffs from those that are accidents, and knowing when an automated tool's flag is a real problem versus noise. The agent should not refactor every smell it sees; it should refactor the ones whose root cause is worth addressing.

This skill applies whenever you are reviewing code, triaging technical debt, or deciding what to refactor and in what order.

## Core Rules

### Read the smell as a symptom, then find the root cause

A smell is a pointer, not a diagnosis. The work is to ask why the smell exists:

- **Long method**: often a symptom of mixed responsibilities, missing abstractions, or accumulated feature additions. The fix is extracting cohesive responsibilities, not arbitrary line splitting.
- **Large class / God class**: usually a symptom of a missing concept boundary—two or more responsibilities merged into one type. The fix is splitting along responsibility lines.
- **Duplicated code**: sometimes a true duplication to remove, but often a sign of a missing abstraction or a forced similarity between things that are only coincidentally alike. The fix may be extracting a shared abstraction, or it may be leaving them separate.
- **Feature envy**: a method that is more interested in another class's data than its own usually indicates the method belongs on the other class, or that a concept is split across the wrong boundary.
- **Shotgun surgery**: a change that requires touching many small pieces usually indicates a concept is fragmented and should be co-located.

Always ask: what design pressure produced this smell? Refactoring the symptom without understanding the cause moves the smell, it does not remove it.

### Prioritize smells by the harm they cause to future change

Not all smells are worth fixing. Prioritize by how much they impede the changes you actually need to make:

- **High priority**: smells in code that changes often, where the smell makes each change harder and riskier (e.g., a God class touched by every feature). Fixing these pays back quickly.
- **Medium priority**: smells in stable code that changes rarely. The cost of refactoring may exceed the benefit if the code is not evolving.
- **Low priority**: smells in code that is effectively frozen or about to be replaced. Leave them.

The value of removing a smell is proportional to how often the affected code is modified. Refactoring frozen code is busywork.

### Distinguish intentional design from accidental smell

Some patterns that look like smells are deliberate:

- A long sequence of straightforward setup in a test is not a "long method" smell; it is readable inline setup.
- Two pieces of similar code that model different domains may be coincidentally similar; forcing them into a shared abstraction (premature generalization) couples unrelated concepts.
- A "data class" with no behavior may be intentional when the behavior genuinely lives elsewhere (e.g., DTOs, value objects in a functional style).
- Repetition across module boundaries that is meant to keep modules independent (avoiding coupling) is sometimes preferable to DRY-ing it into a shared dependency.

Before refactoring, confirm the smell is accidental, not a deliberate tradeoff. When in doubt, ask the original authors or check the history.

### Use the smell catalog as a vocabulary, not a checklist

The classic catalogs (Fowler's, the OO anti-patterns) are useful as a shared vocabulary for naming what you see. But applying them as a checklist—scanning for each smell and "fixing" every instance—produces mechanical refactors that do not improve the design. Use the catalog to recognize patterns quickly, then apply judgment about whether and how to address each one.

### Combine smells with change-frequency data

A smell's importance is relative to how often the code changes. Combine static smell detection with churn data (which files/functions change most often). A God class that nobody touches is low priority; a God class that is modified in every sprint is a high-value refactoring target. This focuses effort where it pays back.

### Recognize the limits of automated tools

Linters, complexity metrics (cyclomatic complexity, cognitive complexity), and duplication detectors are useful first passes, but they measure proxies, not design quality:

- **Cyclomatic complexity** flags branching density but cannot tell whether the branches are cohesive or scattered.
- **Duplication detectors** flag textual similarity but cannot tell whether the duplication is meaningful or coincidental.
- **Line-count thresholds** flag long methods but cannot tell whether the length is essential complexity or accidental.

Use tools to surface candidates, then apply human judgment. Do not refactor solely because a metric exceeded a threshold.

## Common Traps

### Fixing the symptom instead of the cause

Splitting a long method into arbitrary pieces, or extracting a duplicated block into a poorly-fitting abstraction, removes the surface smell while leaving (or worsening) the underlying design problem. Always find the root cause first.

### Refactoring frozen code

Spending effort removing smells in code that never changes produces risk (every refactor can introduce bugs) without benefit. Prioritize by change frequency.

### Premature generalization (DRY taken too far)

Forcing coincidentally similar code into a shared abstraction couples unrelated concepts, so a change in one domain ripples into another. Some duplication is preferable to the wrong abstraction.

### Treating every metric threshold as a defect

A cyclomatic complexity of 11 is not automatically a bug. Metrics are proxies; reaching a threshold means "look here," not "fix this." Mechanical compliance with thresholds produces churn.

### Ignoring smells in frequently-changed code

The most valuable smells to fix are in code that changes often, because the smell taxes every future change. Ignoring these in favor of obvious-but-stable smells wastes opportunity.

### Refactoring without tests

Smell-driven refactoring without characterization tests is dangerous, because you cannot verify behavior is preserved. (See the refactoring-without-tests skill for how to refactor safely when tests are absent.)

### Assuming the smell is accidental without checking history

Some smells are deliberate tradeoffs documented in comments or PR history. Refactoring them away can undo an intentional decision. Check the context before acting.

### Big-bang smell removal

Attempting to fix many smells in one large refactor makes review hard, increases risk, and often stalls. Address smells incrementally, each in a small, reviewable, tested step.

## Self-Check

- For each smell you plan to address, have you identified the root cause (the design pressure), not just the surface symptom?
- Are you prioritizing smells in code that changes frequently over smells in stable or frozen code?
- Have you confirmed each smell is accidental rather than a deliberate design tradeoff (checked comments, history, or authors)?
- Are you avoiding premature generalization—leaving coincidental duplication separate rather than forcing a shared abstraction?
- Are you using automated tools and metrics to surface candidates, then applying human judgment rather than mechanically fixing every flagged instance?
- Is each planned refactor small, reviewable, and covered by characterization or unit tests that prove behavior is preserved?
- Are you addressing smells incrementally rather than in a risky big-bang refactor?
- Have you combined smell detection with change-frequency/churn data to focus effort where it pays back?
- For duplicated code, have you determined whether the duplication is meaningful (extract) or coincidental (leave)?
- After refactoring, does the design genuinely improve (clearer responsibilities, easier change), or did the smell just move?
