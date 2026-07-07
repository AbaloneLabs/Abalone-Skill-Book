---
name: instructional_and_help_content.md
description: Use when the agent is writing or designing help content, tooltips, guided tours, onboarding instructions, setup wizards, field-level guidance, FAQs, error recovery guidance, or any content meant to teach users how to complete a task or recover from confusion.
---

# Instructional And Help Content

Instructional content is not a manual users read front to back. It is help that arrives at the moment of confusion, setup that must not block the task, and guidance that must be accurate enough to trust and short enough to be used. Poor help content is worse than none: it wastes the user's time at the exact moment they are already stuck, and if it is wrong, it sends them further from a solution.

Use this skill before writing or reviewing tooltips, helper text, guided tours, onboarding sequences, setup and configuration instructions, empty-state guidance, error recovery steps, FAQs, knowledge base entries, or in-context coaching. The goal is to prevent the agent from producing help that is generic, outdated, condescending, or disconnected from the actual interface the user is looking at.

## Core Rules

### Place Help Where The Confusion Happens

Help that lives in a separate documentation site is help most users will never find. The most effective instruction appears next to the element, field, or step where the question arises.

Locate help by:

- tooltips and helper text beside the relevant field or control;
- inline guidance that appears when a field is focused or an error occurs;
- a contextual help panel that opens beside the current task;
- coach marks at the moment a feature becomes relevant;
- empty-state text that explains what to do next, not just that nothing exists.

Centralized help still has value for deep reference, but it should not be the only path. Contextual help reduces abandonment.

### Write Instructions As Actions, Not Descriptions

Users reading help want to know what to do, not what the feature is called or how it works internally. Frame guidance as steps the user can perform and verify.

Strong instructional writing:

- leads with the verb and the action;
- names the exact control, button, or field by its visible label;
- states the expected result so the user can confirm success;
- sequences steps in the order they must be performed;
- separates optional from required steps clearly.

Compare "The export function allows users to download data" with "Click Export, choose CSV, then click Download. Your file appears in your Downloads folder." The second is actionable.

### Keep Help Accurate To The Current Interface

Help content decays faster than any other copy because the interface changes while the help text stays. Inaccurate help destroys trust instantly: if one instruction names a button that no longer exists, the user assumes everything else is wrong too.

Protect accuracy by:

- tying help text to specific UI elements so changes trigger updates;
- reviewing help content as part of every interface change;
- avoiding hardcoded screenshots that age quickly, or dating and reviewing them;
- using the exact current labels for controls and fields;
- testing instructions against the real product regularly.

Stale help is a liability. Treat it as code that must be maintained.

### Match Depth To The User's Moment

A user who is mid-task needs a sentence; a user setting up a complex feature needs a sequence; a user researching needs a full article. Forcing all help into one length serves no one.

Calibrate depth by:

- one-line tooltips for quick definitions and field expectations;
- short step lists for common tasks;
- detailed guides for setup, configuration, and complex workflows;
- conceptual explanations only when the user needs the why, not just the how;
- progressive disclosure that offers a summary with a link to deeper detail.

Do not write a paragraph where a phrase suffices, and do not write a phrase where a multi-step task needs steps.

### Write For Users Who Are Frustrated Or Anxious

Help is often read by users who are stuck, confused, or worried they have made a mistake. The tone must lower stress, not add to it.

Guidance for tone:

- be calm, direct, and reassuring without being patronizing;
- avoid blaming the user or implying they should already know;
- acknowledge complexity where it genuinely exists;
- offer a clear next step or a way to get more help;
- avoid exclamation marks, urgency, or cheerfulness in error contexts.

A user reading error recovery help is already stressed. Calm, concrete steps restore confidence.

### Provide A Recovery Path For Every Failure

Instructional content should not stop at describing success. For every place a user can fail, there should be guidance on what to do next.

Cover recovery for:

- validation errors on forms and inputs;
- connection, timeout, and sync failures;
- permission and access denials;
- partial success where some items failed;
- conflicts, such as duplicate names or locked records;
- abandoned or interrupted processes.

"Contact support" is a last resort, not a first response. Give the user a fixable action first.

### Make Help Discoverable Without Being Intrusive

Help must be findable by the user who needs it, but it must not nag the user who does not. The balance is between visibility and interruption.

Patterns:

- a persistent help icon or entry point in predictable locations;
- contextual hints that appear on focus or hover, not on every load;
- optional guided tours that the user can start and skip;
- searchable help for users who know what they are looking for;
- proactive suggestions only when the user appears stuck, such as repeated errors or long inactivity.

Help that interrupts confident users trains them to dismiss all help. Help that hides helps no one.

### Support Different Learning Preferences And Abilities

Users absorb help differently. Some read, some watch, some need to do. Some use screen readers, some have low vision, some need more time.

Accommodate by:

- providing text alternatives for images and videos;
- keeping instructions readable in plain language;
- not relying on color or position alone to identify steps;
- ensuring help is keyboard accessible and screen-reader friendly;
- considering translated versions for localized products.

## Common Traps

### Generic Help That Says Nothing

"Enter your details and click submit" helps no one because it states the obvious. Help must add the specifics the user does not already know.

### Outdated Instructions After UI Changes

A tooltip that names a renamed button, or a screenshot showing an old layout, makes the entire help system look unreliable. Help must be maintained with the interface.

### Condescending Or Over-Cheerful Tone

"You're almost there!" or "It's easy, just..." can feel patronizing, especially in error contexts. Users want competence, not encouragement.

### Help That Describes Instead Of Instructs

Telling users what a feature is called or that it exists does not tell them how to use it. Description without action leaves the user stuck.

### Forcing All Users Through A Tour

Mandatory guided tours block confident users and are skipped by the users who need them most. Tours should be optional and resumable.

### Burying The Critical Step

Long introductions, caveats, and branding before the actual instructions push the useful content below the user's attention. Lead with the action.

### No Recovery For Partial Failure

Help that covers only the happy path leaves users stranded when something goes wrong. Every failure point needs a documented next step.

## Self-Check

- [ ] Help appears contextually next to the field, control, or step where confusion arises, not only in a separate documentation site.
- [ ] Instructions are written as actions using exact current control labels, with expected results the user can verify.
- [ ] Help content is tied to interface elements and reviewed with every UI change so it never references removed or renamed controls.
- [ ] Depth matches the moment: tooltips for quick definitions, step lists for tasks, detailed guides for setup, and progressive links for deeper reference.
- [ ] Tone is calm, direct, and non-blaming, especially in error recovery and high-stress contexts.
- [ ] Every common failure point, validation error, permission denial, and partial success has a documented recovery path with a fixable action.
- [ ] Help is discoverable through predictable entry points without interrupting confident users or nagging on every load.
- [ ] Instructions avoid generic obvious statements and instead add the specifics the user does not already know.
- [ ] Help content is accessible: text alternatives for media, plain language, keyboard reachable, and screen-reader friendly.
- [ ] Guided tours and onboarding are optional, skippable, and resumable rather than mandatory gates.