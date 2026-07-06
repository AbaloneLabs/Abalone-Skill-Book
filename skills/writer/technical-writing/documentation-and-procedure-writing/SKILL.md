---
name: documentation_and_procedure_writing.md
description: Use when the agent is writing how-to guides, procedures, step-by-step instructions, user manuals, runbooks, or task-oriented documentation that must let a reader reliably complete a task.
---

# Documentation And Procedure Writing

Task-oriented documentation exists so a real person can do a real thing. Its success is not measured by how thorough or well-written it feels to the author, but by whether a reader, often stressed and under time pressure, can finish the task without getting stuck, guessing, or giving up. The most common failure is documentation that describes a system instead of enabling an action.

Agents tend to miss this because writing about a product feels like documenting it. They reproduce menus, list features, and narrate the author's mental model of the system. But the reader does not want a tour of the product. They want to reach an outcome: reset a password, deploy a service, refund a customer, file a report. Every choice in procedure writing should be tested against that outcome.

You have latitude in voice, length, and format, but not in the core obligation: the procedure must work for the reader it is written for, in the conditions they will actually use it.

## Core Rules

### Write To The Task, Not To The Product

A procedure answers a goal the reader is trying to reach, not a feature the product exposes. Before writing, name the task in the reader's words: "reset my password," "add a new teammate," "ship a hotfix," "cancel my subscription." If you cannot state the task as an outcome the reader wants, you are documenting the system, not helping the user.

Organize documentation around tasks. A table of contents built from product menus usually signals the wrong structure. A table of contents built from common user goals signals the right one. When a single screen serves many tasks, document each task separately rather than describing the screen once and expecting the reader to map it to their goal.

### Define The Reader And Their Starting Conditions

A procedure only works if it starts where the reader actually is. Specify the assumed starting state: which role or permissions are required, which version or plan, which platform, whether the reader is signed in, what they must have already configured. If a reader without admin rights cannot complete step one, say so before step one, not after they have invested effort.

Decide whether the reader is a first-time user, an occasional user, or an expert who needs a quick reference. First-time users need orientation, screenshots, and definitions of terms. Experts need a compact checklist with no hand-holding. Writing one document for both usually serves neither.

### Number The Steps And Keep Each Step Atomic

Procedures should be numbered, and each step should describe one action with one verifiable result. A step that says "configure your settings and save your changes" is two steps hiding as one, and it breaks the reader's ability to locate where they went wrong.

For each step, give the action, the location, and the expected feedback. Tell the reader what to click, where to find it, and what they should see when it works. If a step can fail, describe what failure looks like and what to do about it. The reader should never have to wonder whether they succeeded.

### Test The Procedure Against The Real System

The single highest-risk failure in documentation is a procedure that does not match the product. Buttons get renamed, flows get redesigned, defaults change, and a guide written last quarter silently breaks. If you have access to the system, walk through every step as the target reader would. If you do not, flag every assumption explicitly and mark the document as unverified against a specific version.

Never write a step from memory or from how the system "should" behave. Distinguish what you verified from what you inferred. If a screenshot is included, confirm it matches the current UI; a stale screenshot erodes more trust than a missing one.

### Separate Concept From Procedure From Reference

Readers come to documentation with different needs. Some want to understand why a feature exists before they use it. Some want to do the task immediately. Some want to look up an exact parameter. Conflating these needs produces documents that explain theory in the middle of a step list or bury a parameter table inside a narrative.

Use the Diátaxis distinction deliberately. A tutorial teaches a beginner by doing. A how-to guide solves a specific problem. An explanation builds understanding. A reference provides authoritative description. Keep these modes in separate documents or clearly separated sections, and link between them. Do not force a single page to be all four.

### Handle Errors, Edge Cases, And Prerequisites Honestly

Most documentation covers the happy path and stops. Real readers hit the unhappy path constantly: a required field is greyed out, a sync fails, a plan does not include the feature, a dependency is missing. Anticipate the failures a reader is likely to encounter and address them where they occur in the flow, not in a single "Troubleshooting" appendix the reader must hunt for.

State prerequisites before the first step. If the task requires a paid plan, a specific role, a network condition, or prior setup, surface that immediately. A reader who discovers the precondition at step seven has wasted their trust.

### Choose Precision Over Brevity When They Conflict

Vague instructions are worse than long ones. "Adjust the settings as needed" is not a step; it is an admission that the author did not know what to write. Prefer the exact value, the exact menu name, and the exact field. If the correct answer depends on the reader's situation, give a decision rule: "If you have fewer than 50 users, choose Small; otherwise choose Large."

Brevity is a virtue, but only after precision. Cut words, not information. A short document that omits a required step is a failure; a longer document that the reader can follow is a success.

### Make The Document Navigable And Scannable

Readers of procedures rarely read top to bottom. They scan for the step they are stuck on, search for a term, or jump to the section that matches their situation. Support this with clear headings that name tasks, a logical hierarchy, bolded UI elements, code blocks for commands, and callouts for warnings and notes.

Use consistent formatting for the same kind of information every time. If a button name is bold in one step, it should be bold in every step. If a warning is a callout in one section, do not bury it as plain text in another. Predictability lets the reader build a mental model of how to read your document.

## Common Traps

### Documenting The UI Instead Of The Task

Describing every field on a screen produces reference material, not a procedure. The reader wants to know which fields matter for their goal and what to put in them, not a tour of every option.

### Assuming The Happy Path Is Enough

When documentation only covers the case where everything goes right, every real-world deviation becomes an undocumented crisis. Readers then turn to forums, support tickets, or guesswork, and the documentation loses authority.

### Hiding Prerequisites Deep In The Flow

Discovering a requirement late feels like a trap. A reader who reaches step eight only to learn they needed a different role has been set up to fail. Prerequisites belong at the top, in plain sight.

### Using Internal Vocabulary For External Readers

The names your engineering team uses internally are rarely the names your users use. "Provision a tenant" may be accurate to the codebase but alien to the administrator trying to set up their team. Translate to the reader's language, and define any term you cannot avoid.

### Letting Screenshots Replace Steps

A screenshot can confirm what the reader should see, but it cannot replace a written instruction. Screenshots go stale, are inaccessible to some readers, and are hard to scan. Use them to support steps, never to be the step.

### Writing For The Reviewer Instead Of The User

Documentation is sometimes written to satisfy an internal stakeholder who wants the product to look complete. That pressure produces feature lists disguised as guides. Keep the user's task as the governing constraint, even when reviewers ask for breadth.

### Failing To Version Or Date The Document

A procedure with no version or date cannot be trusted, because the reader cannot tell whether it reflects the current product. Always indicate which version of the product the procedure was verified against, and when.

## Self-Check

Before treating the documentation as complete, verify:

- The document is organized around reader tasks, not product menus or features.
- The target reader and their assumed starting state are stated explicitly at the top.
- Every prerequisite, including roles, plans, versions, and prior setup, appears before the first step.
- Steps are numbered, and each step describes one action with one verifiable result.
- Each step tells the reader what to do, where to find it, and what success looks like.
- The procedure has been walked through against the real system, or unverified assumptions are clearly flagged.
- Common errors and edge cases are addressed near where they occur, not only in a separate appendix; concept, procedure, and reference material are separated rather than tangled together
- UI elements, commands, and values are precise, not vague placeholders like "as needed."; formatting is consistent: button names, warnings, code blocks, and callouts follow the same pattern throughout
- Screenshots, if used, match the current interface and support rather than replace written steps; the document states which product version it was verified against and when
- A reader matching the target profile could complete the task using only this document; nothing in the document is written to impress an internal reviewer at the user's expense
