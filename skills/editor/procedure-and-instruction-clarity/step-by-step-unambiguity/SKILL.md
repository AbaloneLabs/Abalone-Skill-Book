---
name: step_by_step_unambiguity.md
description: Use when the agent is editing procedural or instructional content and must verify that each step is unambiguous, correctly ordered, and complete enough that a user can execute it without guessing, filling gaps, or reversing engineer the writer's intent.
---

# Step-By-Step Unambiguity

Procedural writing lives or dies by the clarity of its steps. A user following instructions has no access to the writer's intent; they have only the words on the page. An ambiguous step, a missing action, or an unclear ordering causes failure, frustration, or harm. Step-by-step unambiguity is the discipline of writing instructions where each step has one defensible interpretation, the order is correct and explicit, and a competent user can execute the procedure as written. Editors who read procedures for flow rather than for executability miss the ambiguities that cause real-world failure.

Use this skill when editing procedures, tutorials, manuals, or any instructional content presented as steps. It covers action clarity, ordering, and completeness of each step. The goal is a procedure a user can follow successfully on the first attempt.

## Core Rules

### Make Each Step A Single, Clear Action

Each step should describe a single, clear action the user takes. Steps that bundle multiple actions, describe states instead of actions, or leave the action implicit cause confusion. Verify single-action clarity.

Read each step and identify the action it asks the user to perform. Is there exactly one action? Is the action explicit, described with a verb the user can execute? Flag steps that contain multiple actions, which should be split, or that describe a result without the action to achieve it. For example, "The file is saved" is a state, not an action; "Save the file" is an action. Each step should answer what the user does, not what happens or what the system is. Single-action steps are testable and followable; multi-action or state-described steps are not.

### Specify The Object And Means Of Each Action

An action is ambiguous if its object or means is unclear. "Click the button" is ambiguous if there are multiple buttons; "Save the file" is ambiguous if the save method is unspecified. Specify object and means.

For each action, verify the object, what is acted upon, is specified clearly. If the interface has multiple similar elements, identify which one specifically, by label, location, or appearance. Verify the means of action is specified, whether by click, double-click, keyboard shortcut, voice command, or menu selection. Where precision matters, such as which mouse button or what exact menu path, specify it. Ambiguity in object or means forces the user to guess, and guesses cause errors. The step should leave no doubt about what to act on and how.

### Verify Step Ordering Is Correct And Explicit

The order of steps must be correct, with prerequisites completed before dependent steps, and the ordering must be explicit rather than implied. Wrong or ambiguous ordering causes procedures to fail. Verify ordering.

Trace the procedure from start to finish. Does each step's prerequisites, whether system state, completed actions, or available inputs, get established by prior steps? Flag any step that depends on something not yet done. Verify the ordering is explicit through numbering or sequencing, not left to the user to infer. If steps can be done in any order, say so; if order matters, enforce it. Procedures where the order is wrong or unclear fail unpredictably, because users reach steps they cannot execute or that produce wrong results due to missing prerequisites.

### State Expected Results After Key Steps

For key steps, especially those that change system state or that users often get wrong, state the expected result. This confirms the step succeeded and helps users detect errors early. State results.

After steps that produce a visible change, describe what the user should see. For example, after a save step, note that the file appears in the directory; after a connection step, note the indicator that confirms connection. Expected results let users verify they executed the step correctly before proceeding, preventing cascading errors. They also help users distinguish a correct outcome from an error state. Do not state results after every trivial step, which clutters, but do state them after steps where confirmation matters or where failure is common.

### Define Conditionals And Branches Explicitly

Procedures often have conditionals, where the path depends on the user's situation or the system's response. These branches must be defined explicitly, with clear criteria and paths. Define branches.

Identify any point where the procedure branches. State the condition explicitly and the path for each outcome. For example, "If you see a login prompt, enter your credentials; if you are already logged in, skip to step 5." Ensure each branch leads to a complete path to the goal, with no dead ends. Flag implicit conditionals, where the procedure assumes one situation without acknowledging alternatives. Undefined branches leave users stranded when their situation differs from the assumed one. Explicit branches ensure the procedure works for all intended users.

### Eliminate Implicit Knowledge And Assumptions

Procedures fail when they assume knowledge the user may not have. Terms, interface conventions, and prerequisites that the writer takes for granted may be opaque to the user. Eliminate implicit assumptions.

Read the procedure as a user who meets the stated prerequisites but has no additional context. Flag any term, action, or reference that assumes knowledge beyond those prerequisites. Define specialized terms on first use or link to definitions. Explain interface conventions that may be unfamiliar, such as what a specific icon means or where a menu is located. Do not assume the user knows how to navigate the interface or understands domain-specific concepts. Implicit assumptions that the writer does not notice become explicit failures for the user.

### Test The Procedure Mentally And By Execution

The ultimate test of a procedure is whether it can be executed as written. Mental tracing catches some errors; actual execution catches more. Test the procedure.

Trace the procedure mentally, step by step, as a user would. Then, if possible, actually execute it following only the written instructions. Note any point where you had to guess, fill a gap, or deviate from the text. These are ambiguities or errors to fix. Testing reveals problems that reading for style does not, because execution exposes where the procedure's clarity breaks down. If execution is not possible, have someone unfamiliar with the procedure trace it and report confusion points. Untested procedures ship with hidden failures; tested procedures get fixed before they reach users.

## Common Traps

### Bundling Multiple Actions Per Step

One action per step. Split multi-action steps.

### Describing States Instead Of Actions

Steps say what to do, not what is. Use executable verbs.

### Ambiguous Objects Or Means

Specify what to act on and how. Eliminate guessing.

### Wrong Or Implicit Ordering

Order must be correct and explicit. Verify prerequisites precede dependents.

### Missing Expected Results

State what users should see after key steps. Enable early error detection.

### Undefined Conditional Branches

Define branches explicitly with complete paths for each condition.

### Assuming Knowledge Beyond Prerequisites

Eliminate implicit assumptions. Define terms and explain conventions.

## Self-Check

Before treating the procedure as unambiguous and executable, verify:

- Each step describes a single, clear action using an executable verb.
- The object of each action is specified clearly, distinguishing similar elements.
- The means of each action, such as click or shortcut, is specified.
- Step ordering is correct, with prerequisites established before dependent steps.
- Ordering is explicit through numbering or sequencing.
- Expected results are stated after key steps where confirmation matters.
- Conditional branches are defined explicitly with complete paths for each outcome.
- No step assumes knowledge or context beyond the stated prerequisites.
- Specialized terms are defined and interface conventions explained.
- The procedure has been traced mentally and, if possible, executed, with ambiguities fixed; a user meeting the prerequisites could execute the procedure successfully on the first attempt
