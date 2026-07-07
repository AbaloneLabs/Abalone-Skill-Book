---
name: validation_and_error_recovery.md
description: Use when the agent is designing form validation, error messages, inline validation timing, success and error states, recovery flows, or ways to help users understand what went wrong and fix it without losing entered data, restarting, or guessing at the expected format.
---

# Validation And Error Recovery

Validation is how a form tells a user their input cannot be accepted, and error recovery is how it helps them fix it. It looks like writing error messages, but it is really the decision of whether a user understands what went wrong, where, and how to correct it without losing their work or their patience. Agents tend to validate only on submit, to write messages that name the rule but not the fix, to block submission without explaining why, or to clear fields on error and force retyping. The harm is users trapped in loops: they fix what they think is wrong, resubmit, hit a different error, and abandon a task they had nearly completed.

Use this skill before deciding when and how to validate, what error messages say, how errors are displayed, and how users recover. The goal is to prevent the agent from validating at the wrong time, from writing messages that blame or confuse, or from designing recovery that loses data or hides the problem.

## Core Rules

### Validate At The Moment That Helps, Not Just On Submit

Validation timing determines whether it helps or frustrates. Validating only on submit tells users about problems after they have moved on, scattering errors across a completed form. Validating on every keystroke interrupts before the user has finished. The right moment is when the user has finished with a field and can still act on the feedback.

Choose timing deliberately:

- validate on blur or when the user leaves a field, after they have completed entry;
- validate on submit for cross-field rules the user could not know earlier;
- avoid validating on every keystroke, which interrupts in-progress entry;
- re-validate as the user corrects so the error clears promptly.

A field that errors while the user is still typing a date feels hostile; a form that waits until submit to mention twelve problems feels neglectful. Time validation to when feedback is actionable.

### Write Messages That Tell The User How To Fix It

An error message that names the problem without naming the fix forces the user to diagnose it themselves. "Invalid input" tells them nothing; "Enter a date as MM/DD/YYYY" tells them exactly what to do. Messages should be instructions for recovery, not descriptions of failure.

Write recoverable messages:

- say what the user should do, in their language, not the system's;
- be specific about which value and what format or constraint was violated;
- avoid blame, shame, and technical jargon ("invalid", "illegal", "error 400");
- keep messages short, plain, and free of unnecessary alarm.

"Password must be at least 8 characters with one number" is a fix; "Weak password" is a judgment. Always give the path to success.

### Show Errors Where The User's Attention Is

An error message displayed far from the field it concerns forces the user to hunt for the connection. Errors should appear inline, next to or below the field, so the user sees the problem and the fix in one glance. For multi-error forms, also summarize at the top with anchors to each field.

Place errors for attention:

- show inline errors adjacent to the relevant field;
- for submit-time errors, summarize at the top and link to each problem field;
- move focus to the first error or scroll it into view;
- do not rely on color alone; pair it with text and an icon.

A single error banner at the top of a long form, with no indication of which fields failed, is a common and cruel pattern.

### Never Lose The User's Data On Error

The most damaging validation failure is losing what the user already entered. A form that clears fields on error, resets steps, or forces a restart after a validation failure punishes the user for the system's own poor guidance. Entered data must survive every error and recovery cycle.

Preserve data by:

- retaining all valid field values when an error occurs;
- keeping the user on the same step or page, with their context intact;
- not clearing fields except where strictly necessary for security;
- preserving data across navigation away and back where appropriate.

A user who re-enters a long form because an error wiped it will often not re-enter it a second time.

### Prevent Errors Before They Happen

The best validation is the error the user never makes. Strong field design, format hints, input masking, sensible defaults, and constrained controls prevent many errors before validation is ever needed. Prevention is cheaper than recovery for both user and system.

Prevent proactively:

- constrain inputs to valid values at the control (selects, toggles, bounded numbers);
- mask and auto-format entry (dates, phone, card numbers);
- show format and limits before entry;
- disable or guide away from impossible combinations.

A date picker that only allows valid dates prevents the entire class of date-format errors that a text field plus validation would produce.

### Handle Cross-Field And Business-Rule Validation Clearly

Some errors involve relationships between fields or business rules the user cannot see: a date range that must be ordered, a username that must be unique, a combination that exceeds a limit. These need especially clear handling because the user cannot infer the rule from any single field.

Handle complex validation by:

- explaining the rule in plain terms when it is violated;
- indicating which fields are involved, not just one;
- validating such rules at a point where the user can act (often on submit);
- offering a suggested fix where one exists (for example swap the dates).

"End date must be after start date" with both fields highlighted is recoverable; "Invalid range" on one field is not.

### Communicate Success And Completion Clearly

Validation is not only about errors. Users need clear confirmation when a form or action succeeds, especially for high-stakes or async submissions. Ambiguous success states cause users to resubmit, doubt, or navigate away prematurely.

Confirm success by:

- showing a clear success state after submission, not just navigating away;
- confirming what happened and what the user should expect next;
- preventing duplicate submission during async processing;
- handling the case where success is partial or delayed.

### Make Recovery From System Errors Humane

Not all errors are the user's. Network failures, server errors, timeouts, and session expiry happen, and how the form handles them determines whether the user's work survives. System errors should be explained honestly and offer a clear retry without data loss.

Handle system errors by:

- explaining that the problem is on the system side, not blaming the user;
- preserving entered data and offering a retry in place;
- giving a clear next step (retry, contact support, try later);
- avoiding cryptic codes or blank failure states.

## Common Traps

### Validating Only On Submit

Submit-only validation surfaces many errors at once, after the user has moved on, scattering problems across a completed form.

### Keystroke Validation That Interrupts

Validating on every keystroke errors before the user has finished, treating in-progress entry as failure.

### Messages That Describe, Not Fix

Error text that names the rule without telling the user what to do forces them to diagnose and guess at the correction.

### Errors Far From Their Fields

Error banners detached from the fields they concern force users to hunt for the connection across a long form.

### Clearing Data On Error

Forms that wipe entered values on validation failure punish users and cause abandonment at the worst moment.

### Relying On Prevention-After-The-Fact

Free text plus post-hoc validation where a constrained control would have prevented the error entirely.

### Cryptic Cross-Field Errors

Business-rule violations attributed to one field with no explanation of the rule leave users unable to fix the problem.

### Blaming The User For System Errors

Network or server failures presented as user mistakes, or as blank failure states, erode trust and lose work.

## Self-Check

- [ ] Validation runs at helpful moments (on blur and on submit for cross-field rules), not on every keystroke or only at the very end.
- [ ] Error messages tell the user how to fix the problem in plain language, with the specific value, format, or constraint, without blame or jargon.
- [ ] Errors appear inline next to their fields, with a top summary and focus management for multi-error submit-time validation, using more than color.
- [ ] All valid entered data is preserved across errors, recovery, navigation, and return; no field is cleared except where strictly necessary.
- [ ] Errors are prevented through constrained controls, masking, format hints, and defaults before validation is ever needed.
- [ ] Cross-field and business-rule errors explain the rule in plain terms, indicate all involved fields, and offer a suggested fix where possible.
- [ ] Success is communicated clearly, with confirmation of what happened and what to expect next, and duplicate submission is prevented during async processing.
- [ ] System errors are explained honestly as system-side, preserve entered data, and offer a clear retry or next step rather than cryptic codes or blank states.
