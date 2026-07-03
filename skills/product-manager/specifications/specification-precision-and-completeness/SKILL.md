---
name: specification_precision_and_completeness.md
description: Use when the agent is calibrating the precision and completeness of a specification, deciding how much detail to include, ensuring specs are unambiguous and complete enough to build from, or avoiding both over-specification that constrains engineering and under-specification that leaves behavior undefined.
---

# Specification Precision And Completeness

Precision and completeness are the two qualities that make a specification buildable. Precision means each statement in the spec has one interpretation, so that different readers understand the same thing. Completeness means the spec covers all the behavior that must be specified, leaving no gaps that builders must infer. Together, they determine whether a builder can implement the system from the spec without guessing. Done well, a precise and complete spec enables accurate implementation with minimal back-and-forth. Done poorly, the spec is either ambiguous, leading to different interpretations and incorrect implementations, or incomplete, leading to builders inferring behavior that is often wrong. Agents often struggle with calibration, over-specifying in ways that constrain engineering or under-specifying in ways that leave behavior undefined.

The harm this skill prevents is the spec that cannot be built from accurately. An ambiguous spec produces different implementations from different builders, none of which may match the intent. An incomplete spec forces builders to infer, and their inferences are shaped by what is easy to build rather than what is correct, producing behavior that no one intended. Calibration, the skill of specifying the right things at the right level of detail, is what prevents both failures and produces a spec that is buildable.

Use this skill before answering questions such as "how detailed should this spec be", "is this spec complete enough to build from", "how do we avoid ambiguity in specifications", or "are we over-specifying or under-specifying". The goal is to prevent the agent from producing a spec whose precision and completeness are miscalibrated to the work.

## Core Rules

### Calibrate Precision To The Cost Of Being Wrong

The right level of precision depends on the cost of getting the behavior wrong. Where incorrect behavior would cause significant harm, to safety, to revenue, to compliance, to user trust, specify with high precision, eliminating ambiguity and covering every case. Where incorrect behavior is easily corrected or low-impact, lower precision may suffice, accepting some ambiguity in exchange for speed and flexibility. Calibrate precision to the stakes, investing specification effort where mistakes are costly and accepting less rigor where they are not.

This calibration prevents two failures: under-specifying high-stakes behavior, which leads to costly mistakes, and over-specifying low-stakes behavior, which wastes effort and constrains engineering unnecessarily. The product manager's judgment about which behaviors are high-stakes is what guides the calibration, and it should be informed by the user impact, business impact, and technical risk of each behavior. Treat precision as a scarce resource, allocated where it pays off.

### Eliminate Ambiguity In High-Stakes And Complex Behavior

For behavior that is high-stakes or complex, eliminate ambiguity through precise language, concrete examples, and explicit specification of every case. Use specific terms rather than vague ones, define terms that could be interpreted multiple ways, and provide examples that illustrate the intended behavior. For complex logic, use structured representations like decision tables or state diagrams that make the behavior unambiguous. The goal is that any reader, regardless of background, understands the same behavior from the spec.

Ambiguity often hides in words that feel precise but are not. "Recently," "appropriate," "reasonable," "valid," and similar terms seem clear but mean different things to different readers. Replace them with specific definitions: "within the last 30 days," "the action specified by the user's role," "meeting the constraints in section 3." Each replacement removes an interpretation that could diverge. Review the spec specifically for these ambiguity-prone terms and replace them with precise language, because they are where misunderstanding takes root.

### Ensure Completeness By Tracing Scenarios And Edge Cases

Completeness is achieved by systematically tracing through scenarios and edge cases to ensure every behavior is specified. Start with the main workflows and trace each through its happy path, its alternative paths, and its error paths, specifying behavior for each. Then consider edge cases: empty inputs, maximum inputs, concurrent operations, boundary conditions, and failure modes. For each, specify what the system does. This systematic tracing reveals the gaps where behavior is undefined, which are the completeness failures that lead to inferred behavior.

Edge cases are where completeness most often fails, because they are easy to overlook. Enumerate them deliberately: what if the input is empty, what if it is at the maximum, what if two operations happen at once, what if a dependency fails, what if the user does the unexpected. For each, the spec should say what happens. If it does not, there is a gap that a builder will fill with their own assumption. Filling these gaps in the spec, rather than leaving them to inference, is what makes the spec complete.

### Avoid Over-Specification That Constrains Engineering

While precision is valuable, over-specification, specifying things that need not be specified, constrains engineering and creates maintenance burden. Over-specification occurs when the spec dictates implementation details that do not affect behavior, when it specifies behavior at a finer grain than necessary, or when it specifies cases that are better left to engineering judgment. The result is a spec that engineers must follow literally, even when a better approach exists, and that must be updated whenever the implementation changes, even when the behavior has not.

Avoid over-specification by focusing on what must be true, the behavior and constraints, and leaving open what can vary, the implementation. Specify the observable behavior, the quality attributes, and the constraints, and let the engineers determine how to achieve them. When you find yourself specifying internal mechanisms, stop and ask whether the behavior requires it; usually it does not, and the specification is overstepping into design. The test is whether the specification describes what the system must do or how it must do it; the former is appropriate, the latter is over-specification.

### Use Examples To Clarify, Not To Replace Specification

Examples are powerful for clarifying behavior, because they show concrete instances of what the spec means. A spec that says "the system applies discounts in the order that maximizes the customer's savings" is clarified by an example showing specific discounts and the resulting total. Examples make abstract rules concrete and help readers verify their understanding. Use examples generously to clarify, especially for complex logic.

Examples do not replace specification, however. A spec that consists only of examples leaves the general rule implicit, requiring readers to infer the rule from the instances, which can lead to different inferences. Use examples to illustrate a stated rule, not to stand in for one. State the rule explicitly, then provide examples that demonstrate it. This combination gives readers both the general principle and the concrete instances, which together eliminate ambiguity more effectively than either alone.

### Review For Precision And Completeness Before Committing

Precision and completeness are achieved through review, not through initial drafting. A spec written in one pass will have ambiguities and gaps, because these are hard to see in the flow of writing. Review the spec specifically for precision, reading each statement and asking whether it could be interpreted differently, and for completeness, tracing scenarios and edge cases to find unspecified behavior. Engage reviewers who will read critically, because the author's familiarity blinds them to the ambiguities and gaps that fresh readers will catch.

Review should be structured, not casual. Use a checklist of what to look for: ambiguous terms, undefined behavior in edge cases, missing error handling, unstated assumptions, contradictions between sections. For each, scan the spec and address what is found. This structured review catches the precision and completeness failures that casual review misses, and it is what produces a spec that is genuinely buildable. Treat review as an essential part of specification, not an optional polish.

## Common Traps

### Precision Applied Uniformly Regardless Of Stakes

Specifying everything at the same level of detail. The trap is wasted effort on low-stakes behavior and under-specified high-stakes behavior.

### Ambiguity Hiding In Familiar Words

"Recently," "appropriate," "valid" that feel precise but are not. The trap is divergent interpretations that produce incorrect implementation.

### Edge Cases Left Unspecified

Gaps in behavior for empty, maximum, concurrent, and failure cases. The trap is builders inferring behavior that no one intended.

### Over-Specification Dictating Implementation

Specifying how instead of what. The trap is constrained engineering and specs that become obsolete with implementation changes.

### Examples Without Stated Rules

Specs that rely on examples to convey the general principle. The trap is different readers inferring different rules from the same examples.

### One-Pass Specs Without Review

Specs committed without structured review for precision and completeness. The trap is ambiguities and gaps that surface during implementation.

## Self-Check

- [ ] Precision is calibrated to the cost of being wrong, with high rigor for high-stakes behavior.
- [ ] Ambiguity is eliminated in high-stakes and complex behavior through precise language, definitions, and structured representations.
- [ ] Completeness is ensured by tracing scenarios and edge cases, specifying behavior for each.
- [ ] The specification avoids over-specification, focusing on behavior and constraints rather than implementation.
- [ ] Examples are used to clarify stated rules, not to replace specification of the general principle.
- [ ] The spec has been reviewed specifically for precision and completeness, using a structured checklist.
- [ ] A builder could implement the system from the spec without inferring undefined behavior or interpreting ambiguous statements.
