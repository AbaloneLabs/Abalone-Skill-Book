---
name: forms_and_validation_frontend.md
description: Use when the agent is building, refactoring, or reviewing a form, designing client-side validation UX, deciding validation timing (onSubmit, onBlur, onChange, onTouched), wiring async validation, handling multi-step or wizard forms, designing error messages and inline error display, ensuring form accessibility (ARIA, field-error association, screen readers), choosing controlled vs uncontrolled inputs at form scale, integrating a form library, or deciding how client and server validation relate. Also covers progressive validation, submit-button gating, cross-field and dependent validation, optimistic submit, the failure mode of client-only validation that trusts the client, and inconsistent validation UX that confuses users. This skill is about the form-level judgment of validation, error design, and client-server validation alignment, and complements the single-input controlled/uncontrolled decision covered in component-architecture-and-state.
---

# Forms And Validation (Frontend)

A form is a contract between the user and the server, mediated by the client. The user provides input; the client tries to make that input easy to give and quick to correct; the server decides what it will actually accept. The judgment problem is that these three layers are easy to get almost right and costly to get wrong. Validation that runs too early shouts at users mid-keystroke; validation that runs too late lets them submit a form full of errors and then dumps a wall of red on them. Error messages that blame the user ("Invalid input!") or that describe the rule in machine terms ("field_required") turn a fixable mistake into frustration. And the most dangerous failure — client-only validation that trusts the client — silently assumes the browser is the source of truth, when any user, bot, or buggy integration can send whatever it wants to the endpoint.

Agents tend to treat forms as a solved problem: drop in a form library, add a few `required` attributes, show errors under fields, ship it. The harm is delayed. The form works in the happy path the engineer tested, then fails in production when a user pastes an international phone number the regex rejected, when a server returns a field-level error the client never mapped, when a screen-reader user cannot tell which field is wrong, or when a multi-step form loses everything because the state was not preserved across steps. The discipline is to treat validation as a UX decision (when and how to surface problems), an accessibility decision (how errors are communicated), and a security/correctness decision (never trusting the client) — held together, not solved by a library alone.

This skill is about form-level validation, error design, validation timing, multi-step flows, accessibility, and the client-server validation relationship. It complements the single-input controlled/uncontrolled decision covered in component-architecture-and-state, which addresses one field's source-of-truth model; here the concern is the whole form's validation lifecycle and how it aligns with the server.

## Core Rules

### Treat The Server As The Source Of Truth, And The Client As A Courtesy

The single most important rule: the server validates authoritatively; the client validates for the user's convenience. Any validation that exists only on the client is advisory, because the client is an untrusted environment. A user can disable JavaScript, a bot can POST directly to the endpoint, and a future integration can bypass the UI entirely. Client-only validation of an email format, a price, or a permission check is a security and correctness hole, not a UX choice.

- **Validate on the server for every field that matters, every time.** Required-ness, format, ranges, uniqueness, authorization, and business rules must all be enforced server-side, regardless of what the client does. The client's validation should be a subset that mirrors the server's rules.
- **Keep client and server rules in sync deliberately.** When the server's rules change, the client's must follow, or the UX diverges (the client rejects what the server accepts, or accepts what the server rejects). Where feasible, share the validation schema between client and server (a schema library like Zod or Yup used in both places) so they cannot drift.
- **Map server errors back to fields.** When the server rejects a submission, it should return errors structured by field so the client can display them inline, next to the offending input — not as a generic "submission failed" banner that leaves the user guessing.

The failure mode to avoid: a form that validates thoroughly on the client and barely at all on the server, so the first request that skips the UI exposes the endpoint to bad or malicious data. The client is a courtesy; the server is the gate.

### Choose Validation Timing By User Psychology, Not By Convenience

When validation runs shapes the entire felt experience of the form, and the wrong timing is worse than no validation. The three common timings each have a purpose, and the mistake is picking one globally.

- **onSubmit (validate when the user attempts to submit).** Least intrusive. Good as the baseline for most fields: the user types freely, and only when they signal "I'm done" does the form check. Avoids shouting at users mid-entry.
- **onBlur / onTouched (validate when the user leaves a field).** The best balance for most fields — immediate feedback after the user finishes, without interrupting typing. The standard progressive pattern: validate on blur after first touch, then re-validate on change once the field is already known to be invalid, so the error clears as soon as the user fixes it.
- **onChange (validate on every keystroke).** Most aggressive, and usually wrong as the first trigger. Validating while the user is still typing a half-finished email or phone number produces a stream of "invalid!" errors that feel hostile. Reserve onChange for re-validating a field already in an error state, or for fields where the value drives other UI live (a password-strength meter, a username-availability check).

The strong default is onSubmit-or-onBlur for first validation, switching to onChange once a field is invalid. Never validate on first keystroke, and never block typing based on validation. The goal is to tell the user about a problem at the moment they can act on it, not the moment they are still in the middle of causing it.

### Design Error Messages And Inline Display For Recovery, Not For Blame

An error message's job is to help the user recover, not to report that they failed. The difference is concrete: "Invalid email" tells the user nothing; "Please enter a valid email address, like name@example.com" tells them what is expected and gives an example.

- **Describe what is wrong and how to fix it, in plain language.** "Password must be at least 8 characters and include a number" is useful; "Password too weak" is not. Avoid jargon, regex fragments, and internal field names.
- **Do not blame or scold the user.** "You entered an invalid value" frames the user as the problem. "This field requires a date in the future" describes the requirement. The tone should be neutral and helpful.
- **Show errors inline, next to the field they concern.** A field-level error lets the user see and fix the problem in one place. A summary at the top can help on long forms, but it must link or scroll to each error and never be the only indication.
- **Mark errors accessibly.** The error text must be programmatically associated with its input (`aria-describedby`, `aria-invalid`) so screen readers announce it, and the input must be reachable and focusable. A red border with no text or label is invisible to assistive technology.

The test of a good error message: a user who sees it knows what to do next without guessing. If the message only says "no," it is not finished.

### Handle Async And Cross-Field Validation Without Degrading UX

Some validations cannot be done locally — checking whether a username is taken, verifying a coupon code, confirming an email is not already registered. These are async, and they introduce latency, failure, and race conditions that synchronous validation does not have. Handle them deliberately.

- **Debounce async validation.** Checking availability on every keystroke floods the server and produces overlapping requests whose results can arrive out of order. Debounce (e.g., 300-500ms after the user stops typing) and ignore stale responses so an earlier slow response cannot overwrite a later correct one.
- **Show a distinct "checking" state.** The user should see that validation is in progress (a spinner, a muted "checking..." hint), not a blank or an error, while the request is outstanding.
- **Treat async failures gracefully.** If the availability check errors or times out, do not hard-block submission on a field the client could not verify. Decide whether to allow submit-and-let-the-server-decide or to show a retry, but never silently fail and never leave the user stuck.
- **Validate cross-field rules when any involved field changes.** "Password confirmation must match password" or "end date must be after start date" depend on more than one field; re-run these rules whenever any of their inputs change, and clear the error as soon as the constraint is satisfied.

Async and cross-field validation are where naive forms break, because the engineer treated them like synchronous field checks. They need their own state (idle, checking, valid, invalid, error) and their own lifecycle.

### Preserve State And Handle Submission Deliberately In Multi-Step Forms

Multi-step (wizard) forms add a lifecycle: state must persist across steps, the user must be able to go back without losing input, and submission spans the whole flow. The common failures are losing everything on a step transition, validating too late (only at the final submit), or validating too aggressively (blocking forward navigation on fields the user has not reached).

- **Persist step state across navigation and reload.** A user who refreshes or hits the back button should not lose their progress. Hold the accumulated form state in a durable location (URL, sessionStorage, or a store) and rehydrate it, treating rehydrated values as hints to be revalidated.
- **Validate per step, at the step boundary.** Validate the fields of a step before allowing forward navigation, so the user fixes errors in context rather than at the end. Do not validate fields the user has not yet seen.
- **Make submission idempotent and resilient.** A multi-step submit is one logical action; guard against double-submission (disable the button, ignore repeat clicks), handle network failure with a clear retry, and ensure the server action is idempotent so a retried submit does not create duplicates.
- **Let users go back without penalty.** Returning to an earlier step should preserve its data and not force re-entry. The back button is part of the form's contract with the user.

Multi-step forms multiply the state and validation surface; treat the whole flow as one form with staged validation, not as independent forms glued together.

## Common Traps

### Client-Only Validation That Trusts The Browser

Validating thoroughly on the client and minimally on the server, so the first request that bypasses the UI (a bot, a disabled-JS user, an integration) sends invalid or malicious data straight through. The server is the source of truth; mirror its rules on the client as a courtesy.

### Validating onChange From The First Keystroke

Showing "invalid email!" while the user is still typing the first few characters, turning data entry into a stream of errors. Validate on submit or blur first; switch to onChange only after a field is already known to be invalid.

### Error Messages That Blame, Report In Code, Or Are Invisible To Assistive Tech

"Invalid input," "field_required," or a raw regex fragment that tells the user they failed without saying how to recover — or an error indicated only by a red border or icon that screen readers cannot announce and colorblind users may miss. Write plain-language messages that describe the requirement, associate error text with the field via `aria-describedby`, set `aria-invalid`, and never rely on color alone.

### Generic "Submission Failed" Banner Hiding Field Errors

Returning a server error as a single top-of-page message instead of mapping it back to the offending fields, leaving the user to guess which input was wrong. Structure server errors by field and display them inline.

### Async Validation Without Debouncing, Or Multi-Step State Lost On Navigation

Checking username availability on every keystroke and letting an early slow response overwrite a later correct one — debounce async checks and ignore stale responses. Equally, a wizard that wipes the user's input when they go back or refresh because state lived only in the current step's component — persist accumulated state across steps and reload, rehydrating it as a hint, and guard submission against double-click with an idempotent server action.

## Self-Check

- [ ] The server validates every meaningful field authoritatively on every submission, the client validation is a subset that mirrors the server's rules, and the two are kept in sync (ideally via a shared schema) — no field relies on client-only validation for correctness or security.
- [ ] Validation timing was chosen for user psychology: first validation runs on submit or blur (not on the first keystroke), fields switch to onChange re-validation only once already invalid, and typing is never blocked by validation.
- [ ] Error messages describe what is wrong and how to fix it in plain language (with examples where useful), never blame or scold the user, and avoid jargon, regex fragments, or internal field names.
- [ ] Errors are displayed inline next to their field, a top-of-form summary (if used) links to each error and is not the only indication, and the server returns errors structured by field so they map back to inputs rather than a generic failure banner.
- [ ] Form errors are accessible: each error text is associated with its input via `aria-describedby`, the input sets `aria-invalid`, and error indication does not rely on color or icon alone.
- [ ] Async validation is debounced, shows a distinct "checking" state, ignores stale responses so out-of-order results cannot overwrite current ones, and degrades gracefully (allows server-side decision or retry) on network failure rather than hard-blocking.
- [ ] Cross-field and dependent validations (password match, date ranges) re-run when any involved field changes and clear as soon as the constraint is satisfied.
- [ ] Multi-step forms persist accumulated state across step navigation and reload (rehydrated as hints and revalidated), validate per step at the step boundary without validating unseen fields, allow back-navigation without data loss, and guard submission against double-submit with an idempotent server action.
