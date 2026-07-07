---
name: accessible_forms_and_input_errors.md
description: Use when the agent is building or reviewing an HTML form, designing input validation and error messages, wiring client-side validation to server-side errors, deciding how errors are announced to screen readers, handling required-field and format validation accessibly, designing inline error placement, managing live-region announcements for form submission, or ensuring keyboard and screen-reader users can perceive, understand, and recover from form errors. Also covers the failure modes of errors appearing only as red color, validation that runs only on submit, error text disconnected from its field, and the recurring mistake of treating validation as a visual concern rather than a programmatic, announced contract.
---

# Accessible Forms And Input Errors

A form is the place where a user's input becomes the system's action — a purchase, a registration, a transfer — and it is also the place where users most often fail. The judgment problem is that a sighted user with a mouse can scan a form, see a red border, read an error message next to the field, and fix it; a screen-reader user, a keyboard-only user, or a user with cognitive load cannot do any of that unless the form was built to expose its state programmatically. A red border carries no information to a screen reader. An error message inserted into the DOM after a failed submit is invisible to assistive technology unless it is connected to its field and announced. A field that becomes invalid but keeps focus on the submit button leaves the user staring at a form that "did nothing." Accessible form design is not about color contrast on inputs; it is about making the validation state, the error text, and the recovery path perceivable, understandable, and operable through every input modality.

Agents tend to treat validation as a visual layer — red borders, icons, a message under the field — because that is what sighted users perceive and what the design mockup shows. The harm appears for the users the mockup does not represent. A screen-reader user submits an invalid form and hears nothing, because the errors were added to the page without an announcing live region. A keyboard user tabs to a field that failed validation, but the error is three DOM nodes away and not associated, so the field's accessible name does not include the error and the user does not know what is wrong. A cognitive-load user faces an error that says "Invalid input" with no guidance on the expected format. A required field marked only with a red asterisk is invisible to a screen reader. The judgment problem is to treat every validation state as a programmatic contract — the field must announce its validity, its error, and its required status through the accessibility tree — and to design the error recovery flow so that discovering, understanding, and fixing an error is possible without sight and without a mouse.

This skill covers form structure, label and instruction association, validation timing, error announcement, required-field semantics, and error-recovery flow. It complements the aria-and-semantics skill (which covers ARIA roles and properties broadly) and the keyboard-and-screen-reader-support skill (which covers focus management broadly). Here the focus is the specific, high-stakes surface of forms and input errors.

## Core Rules

### Associate Every Input With A Programmatically Linked Label

An input's accessible name is how a screen reader identifies it, and it must be stable, unique, and linked to the input. The reliable mechanism is the `<label for>` association or wrapping the input in a `<label>`. An input with no programmatic label — only a placeholder or a nearby visual text — is unnamed to assistive technology, and the user must guess what the field is.

- **Use `<label for="id">` or a wrapping `<label>`.** The `for`/`id` link (or wrapping label) is the most robust association; it is clickable (focusing the field) and announced by screen readers. A placeholder is not a label: it disappears on input, is often low-contrast, and is not announced as the field's name.
- **Keep labels visible and persistent.** A label that is always visible helps users with cognitive load and low vision; a placeholder that vanishes on typing leaves the user without context mid-entry.
- **Provide instructions before the field, linked via `aria-describedby`.** Format hints ("MM/DD/YYYY", "8-20 characters, one number") belong in text associated with the field through `aria-describedby`, so they are announced when the user focuses the field — not buried in placeholder text that disappears.
- **Name buttons by their action, not their shape.** A submit button must say "Submit" or "Place order," not "Go" or an icon alone; an icon-only button needs an `aria-label`.

The test: with CSS disabled and the page read linearly, can a user understand what each field is and what to enter? If not, the labels and instructions are not programmatically linked.

### Mark Required Fields Semantically, Not Just Visually

A required field must communicate its requirement through the accessibility tree, not only through a red asterisk or bold text that a screen reader may not announce. There are several mechanisms, and they must be used correctly, not stacked redundantly:

- **The native `required` attribute on `<input>`/`<select>`/`<textarea>`.** This is the strongest signal: it is announced as "required" by screen readers and it enforces native validation. Prefer it.
- **`aria-required="true"` where the native attribute is not available** (custom components). Do not set both `required` and `aria-required` on the same field — it causes duplicate announcement.
- **Do not rely on an asterisk alone.** A `*` in a label is a visual convention; screen readers announce "star," which is meaningless without a legend. If you use an asterisk, include a legend ("* required") and ensure the field is also marked `required` or `aria-required`.
- **Group related fields with `<fieldset>`/`<legend>`.** A group of radio buttons or checkboxes (shipping address vs billing address) needs a group label; `<fieldset><legend>` provides it semantically.

### Run Validation At The Right Time — Not Only On Submit

Validation timing affects both usability and accessibility. Validating only on submit forces every user through a discover-the-errors cycle; validating on every keystroke can be hostile (rejecting a partial email mid-typing). The accessible pattern balances feedback with patience:

- **Validate on blur (when the user leaves a field) for format and completeness.** On-blur validation gives feedback after the user has finished entering, without rejecting partial input. This is the most accessible default for single-field errors.
- **Validate on submit for cross-field and business rules.** Rules that depend on multiple fields (password confirmation, date ranges) belong on submit or after both fields are complete.
- **Avoid aggressive on-input validation that rejects partial input.** Telling a user their email is invalid while they are still typing it is frustrating for everyone and especially hostile to users who type slowly or use dictation.
- **Re-validate as the user corrects.** Once a field is in an error state, clear the error as soon as the input becomes valid, so the user gets confirmation they fixed it.

### Connect Error Messages To Fields And Announce Them

This is the core of accessible error handling: an error message that appears in the DOM is useless to a screen reader unless it is both associated with its field and announced. The two-part contract:

- **Associate the error with the field via `aria-describedby` and set `aria-invalid="true"`.** When a field fails, point its `aria-describedby` at the error message's `id` and set `aria-invalid="true"` on the input. Now the screen reader announces the field as invalid and reads the error when the field is focused.
- **Announce the error via a live region (`aria-live`/`role="alert"`/`role="status"`).** An error that appears after a submit must be announced, not silently inserted. An `aria-live="assertive"` region (or `role="alert"`) announces new error content; `role="status"` or `aria-live="polite"` announces success. Place the live region so it announces a summary ("3 errors: email is required, ...") or the first error.
- **Do not over-announce.** Multiple `role="alert"` regions firing at once create a cacophony a screen reader user cannot parse. Use one summary live region for form-level errors and field-level `aria-describedby` for per-field detail.
- **Move focus to the first invalid field after a failed submit.** After announcing the error summary, set focus to the first invalid field so the keyboard user lands where the fix is needed — do not leave focus on the submit button with no signal.

The weak pattern is errors rendered as red text near the field with no `aria` association and no live region: a sighted user sees them, everyone else gets a silent failure.

### Make Error Text Specific, Actionable, And Recoverable

Error text must tell the user what is wrong and how to fix it, in plain language. "Invalid input" is not an error message; it is a wall.

- **State the problem and the fix.** "Enter a valid email address, like name@example.com" beats "Invalid email." "Password must be 8-20 characters and include a number" beats "Password too weak."
- **Do not blame the user.** "Email address not recognized" is kinder and clearer than "You entered an invalid email." Frame errors around the field and the requirement, not the user's failure.
- **Identify the field in the error.** In a summary, name the field ("Email: enter a valid email address") so the user knows which field to fix, not just that "an error occurred."
- **Preserve valid input across re-render.** Do not wipe the entire form when one field fails; the user should fix only the error, not re-enter everything. (This is also a security concern: never repopulate password fields.)
- **Provide a recovery path for complex errors.** If a username is taken, suggest alternatives or a link to "recover account." If a payment fails, explain why and offer retry. An error with no path forward is a dead end.

### Handle Multi-Step And Dynamic Forms With Explicit State Communication

Multi-step forms (wizards, progressive disclosure) and dynamically added fields add a layer of complexity: the user must understand where they are in the process and what changed. The accessible contract:

- **Communicate step progress.** Use a visible and programmatic indicator (current step of N, with `aria-current="step"`). A screen-reader user must know they are on "Step 2 of 4."
- **Announce dynamically added or removed fields.** When a field appears based on a prior selection (country-specific address fields), announce it via a live region or move focus to it, so the user knows it appeared.
- **Manage focus on step transitions.** When moving to the next step, move focus to the step heading or the first field of the new step — do not leave focus stranded on the "Next" button of a now-hidden step.
- **Make the submit state announced.** A form that is submitting or has submitted successfully must announce its state ("Submitting...", "Form submitted successfully") via a live region, not only a spinner or a page change.

## Common Traps

### Errors Shown Only As Red Color Or An Icon

Rendering an error as a red border or an icon with no text and no ARIA, so color-blind users and screen-reader users cannot perceive it. Convey errors with text, associate the text with the field, and set `aria-invalid`.

### Validation That Runs Only On Submit, Leaving Users Stranded

Validating only on submit and then not moving focus or announcing errors, so a keyboard or screen-reader user submits, sees nothing happen, and does not know errors appeared. Validate on blur, announce errors via a live region, and move focus to the first invalid field.

### Error Text Disconnected From Its Field

An error message placed in the DOM near a field but not linked via `aria-describedby`, so focusing the field does not announce the error. Link every error to its field and set `aria-invalid="true"`.

### Placeholder Used As A Label

Using a placeholder as the only label, so once the user starts typing the label vanishes and the field has no accessible name once filled. Use a persistent `<label>` and reserve placeholders for examples.

### Required Field Marked Only With An Asterisk

A red `*` in the label with no `required` attribute and no `aria-required`, so screen readers announce "star" with no meaning. Use the native `required` attribute and do not rely on visual symbols alone.

### "Invalid Input" With No Guidance

An error message that states the problem without the fix, leaving the user unable to recover. State the requirement and give an example of the expected format.

### Clearing The Whole Form On Error

Wiping all fields when one fails validation, forcing the user to re-enter valid data (and never repopulating password fields). Preserve valid input and clear only what is invalid.

### Silent Submit And Success States

A submit button that, when clicked, shows a spinner but announces nothing, or a success that changes the page with no announcement. Announce submitting and success states via live regions.

## Self-Check

- [ ] Every input has a programmatic label (`<label for>` or wrapping label) that is persistent and visible, instructions are linked via `aria-describedby`, and no field relies on a placeholder as its only label.
- [ ] Required fields are marked with the native `required` attribute (or `aria-required` for custom components, not both), visual asterisks are accompanied by a legend, and related fields are grouped with `<fieldset>`/`<legend>`.
- [ ] Validation runs at an accessible time (on blur for single-field format, on submit for cross-field rules, not aggressively on every keystroke), and errors clear as the user corrects them.
- [ ] Errors are both associated and announced: each error is linked to its field via `aria-describedby` with `aria-invalid="true"` set on the input, and form-level errors are announced through a single live region (`role="alert"`/`aria-live`), with focus moved to the first invalid field after a failed submit.
- [ ] Error text is specific and actionable (states the problem and the fix, gives a format example, identifies the field, does not blame the user), valid input is preserved across re-render (password fields never repopulated), and complex errors have a recovery path.
- [ ] Multi-step and dynamic forms communicate state: step progress is shown and set with `aria-current`, dynamically added fields are announced or focused, focus is managed on step transitions, and submit/success states are announced via live regions.
- [ ] The highest-risk cases were verified — a screen-reader user submitting an invalid form and hearing the errors, a keyboard user landing on the first invalid field, a color-blind user perceiving errors without color, and a user recovering from an error with clear guidance — not only the sighted, mouse-driven happy path.
