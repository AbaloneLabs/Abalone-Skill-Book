---
name: error_state_design.md
description: Use when the agent is designing how an interface prevents, presents, and recovers from errors, including validation errors, system failures, permission and conflict errors, destructive-action confirmation, error message wording, recovery paths, and ensuring error states reduce frustration and data loss rather than amplifying them.
---

# Error State Design

Errors are the moments when a product is most judged, because they are the moments when the user is most vulnerable. A user hitting an error is already frustrated, often mid-task, sometimes at risk of losing work, and always in need of two things the interface rarely provides clearly: what went wrong, and what to do next. Most error design fails not because the error is hard to detect but because it is presented in a way that blames the user, destroys their work, hides the cause, or offers no recovery. The judgment problem is treating errors as a design surface rather than an afterthought, preventing errors before they happen, and designing recovery that restores the user to a productive state. Agents tend to fail by writing generic "something went wrong" messages, by validating only on submit, and by treating error states as edge cases rather than as the experiences that determine whether users trust the product.

Use this skill when designing any interface where input, network, permissions, or system failure can occur: forms, editors, uploads, payments, settings, and any flow with destructive or irreversible actions. The goal is error states that users can understand, survive, and recover from.

## Core Rules

### Prevent Errors Before They Occur

The best error state is the one the user never encounters. Prevention is more powerful than recovery, because it removes the failure entirely rather than managing its aftermath.

Prevent errors by:

- constraining input with the right control, a date picker instead of a free-text date, a slider instead of a number field;
- providing sensible defaults so required decisions are pre-resolved;
- validating inline and as the user types, not only on submit;
- disabling actions that cannot yet succeed, with a clear reason;
- confirming destructive actions, but only genuinely irreversible ones.

Every error prevented is a frustration avoided. Spend design effort on prevention before recovery.

### Validate Early, Inline, And Constructively

Validation that happens only at submit forces users to complete a whole form, fail, and then hunt for what went wrong. Inline validation catches problems at the moment they occur, in the place they occur.

Strong validation:

- checks format and constraints as the user leaves a field, not only at the end;
- shows success as well as failure, so users know a field is correct;
- never validates aggressively while the user is still typing, which feels punitive;
- explains the rule that was broken, not just that it was broken;
- keeps the field editable and the entered value visible after an error.

Timing matters as much as wording. Validate on blur or change, not on every keystroke, and never clear the user's input on failure.

### Write Error Messages That Name, Explain, And Direct

A good error message does three things: it names the problem in plain language, it explains enough to understand why, and it directs the user to the fix. Most messages do none of these.

Write messages that:

- describe what happened concretely, not "an error occurred";
- avoid blaming the user; say "the email address is missing the @" not "you entered an invalid email";
- state the constraint that was violated so the user can correct it;
- offer the next action, such as "try a shorter name" or "check your connection and retry";
- stay calm, without exclamation, humor, or panic, especially in serious contexts.

The shift from "Invalid input" to "Passwords need at least 8 characters" is the difference between a wall and a path.

### Preserve User Work Across Errors

The most damaging error pattern is one that destroys the user's input. A form that clears on failure, an editor that loses unsaved changes, or a flow that restarts from the beginning after an error converts a small problem into a catastrophe.

Preserve work by:

- keeping all entered values after a validation or submission error;
- saving drafts automatically where the work is substantial;
- returning the user to the point of failure, not the start of the flow;
- never reloading a page in a way that wipes form state.

Users will tolerate errors far more readily if their work survives them. Losing work is often the single error that makes them leave for good.

### Distinguish Error Types And Match The Response

Not all errors are the same, and treating them uniformly produces confusing experiences. The recovery path depends on the type of error.

Match response to type:

- validation errors: inline, immediate, field-specific, user-fixable;
- network and timeout errors: retryable, with the action preserved and a retry button;
- permission and authentication errors: explain the requirement and offer sign-in or request access;
- conflict errors, such as concurrent edits: explain the conflict and offer resolution options;
- system and server errors: acknowledge the problem is not the user's fault and offer a path forward or support.

A single generic message for all of these hides the cause and the fix. Categorize the error and respond appropriately.

### Design Recovery Paths, Not Just Messages

An error message without a recovery path is a dead end. Every error should leave the user with a clear next step that has a real chance of succeeding.

Provide recovery by:

- offering a specific action, retry, edit, contact support, not just "try again later";
- making the recovery action prominent and obvious;
- ensuring the recovery path actually addresses the error type;
- providing an escape hatch, such as saving progress or contacting a human, when self-service fails;
- avoiding loops where retrying produces the same error with no new information.

If the only recovery is "try again," and trying again fails the same way, the design has abandoned the user.

### Handle Destructive And Irreversible Actions Deliberately

Some errors are not recoverable after the fact, and these deserve the most careful design. The goal is to make irreversible actions hard to do by accident and easy to confirm meaningfully.

Design destructive actions by:

- requiring explicit confirmation, with the consequence stated clearly;
- making the destructive action visually distinct from safe ones;
- offering undo where the action is not truly irreversible, often preferable to confirmation;
- using progressive confirmation, such as typing a name, only for the most severe actions;
- avoiding confirm-everything, which trains users to click through and defeats the purpose.

Confirmation only works when it is rare and meaningful. A dialog on every action becomes noise that users dismiss automatically.

### Keep Working Areas Functional During Partial Failure

When part of an interface fails, the rest should keep working. A single failed request that disables the entire screen punishes the user for a localized problem.

Isolate failure by:

- keeping unrelated areas interactive when one component errors;
- showing the error only in the affected region, with a retry specific to it;
- avoiding full-page error states for partial failures;
- preserving navigation so the user can move elsewhere and return.

A user who can continue other work while one section recovers experiences the product as resilient, not broken.

## Common Traps

### Generic "Something Went Wrong" Messages

A message that names no cause and offers no fix leaves the user stranded. Always name and direct.

### Submit-Only Validation

Forcing users to complete a form to discover errors is slow and frustrating. Validate inline.

### Blaming The User

Copy that accuses the user erodes trust at the worst moment. Describe the problem, not the user's failure.

### Destroying User Work On Error

Clearing input or restarting a flow after an error converts a small failure into a lost user. Always preserve work.

### One Generic Message For All Errors

Treating validation, network, and permission errors identically hides the cause and the fix. Categorize and respond.

### Recovery Paths That Loop

A retry that fails identically with no new option abandons the user. Provide an escalating path or escape hatch.

### Confirm-Everything Dialogs

Overusing confirmation trains users to dismiss warnings and slows every action. Reserve it for the genuinely irreversible, and prefer undo.

### Full-Screen Failure For Partial Errors

Disabling the whole interface for one failed component punishes the user. Isolate the failure.

## Self-Check

- [ ] Errors are prevented through input constraints, defaults, and inline validation before recovery is needed.
- [ ] Validation occurs inline on blur or change, shows success as well as failure, and never clears input.
- [ ] Error messages name the problem, explain the constraint, avoid blame, and direct the user to a fix.
- [ ] User work is preserved across all error and submission failures, with the user returned to the point of failure.
- [ ] Error types are distinguished, and each has an appropriate response and recovery path.
- [ ] Every error offers a concrete, prominent recovery action with a real chance of success.
- [ ] Destructive and irreversible actions use rare, meaningful confirmation, and undo is offered where possible.
- [ ] Partial failures keep unrelated areas functional, with errors isolated to the affected region.
- [ ] No error leaves the user in a loop with no escalating option or escape hatch.
- [ ] Error copy was reviewed under frustration and fatigue, not only in the calm of a design review.
