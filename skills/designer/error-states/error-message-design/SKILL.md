---
name: error_message_design.md
description: Use when the agent is writing or designing error messages, validation errors, inline field errors, toast and banner errors, API failure copy, 404 and 500 states, empty-and-broken states, or reviewing whether an error explains the problem and how to recover from it.
---

# Error Message Design

An error message is not a label for a failure. It is a small piece of instruction delivered at the worst possible moment: the user is already blocked, confused, or frustrated, and now they have to decode a sentence before they can move on. Most error copy fails not because it is impolite, but because it tells the user what went wrong without telling them what to do next, who is responsible, or whether the problem is even theirs to fix.

Use this skill before finalizing form validation, network failure handling, permission and auth errors, payment failures, upload errors, API error responses, and broken or unavailable content states. The goal is to prevent the agent from shipping error copy that is technically accurate but operationally useless, that blames the user, that leaks internal detail, or that leaves the user with no recoverable path.

## Core Rules

### Lead With The Fix, Not The Failure

The most useful first sentence of an error message is often what the user should do, not what broke. Users read errors under stress and skim the first line. If the first line is a description of the system state, the action is buried.

Prefer structure: state the problem briefly and concretely, then give the next step. For common, recoverable cases, the next step can lead. "Enter a date after today" is stronger than "Invalid date. Dates must be in the future." Both are fine; the order should serve the reader who stops after one line.

Decide explicitly whether the error is:

- user-fixable now (wrong input, missing field) — give the exact correction;
- user-fixable later (quota, billing, permissions) — explain the blocker and where to resolve it;
- not user-fixable (server error, maintenance) — apologize plainly, say it is not their fault, and give a retry or contact path.

If you cannot tell which category an error belongs to, do not write copy that implies the user did something wrong.

### Be Specific About What Is Wrong And Where

Generic errors such as "Something went wrong" or "Please try again" are appropriate only when the system genuinely cannot determine the cause. When the cause is known, naming it reduces anxiety and support load.

Specificity means:

- identify the field, file, record, or step that failed, not just "the form";
- name the violated constraint in plain terms ("must be at least 8 characters", "only PDF under 10 MB", "this email is already in use");
- include the relevant value or limit when it helps ("You selected 12 items; the limit is 10");
- distinguish between "required", "format", "range", "uniqueness", "permission", and "conflict" failures, because the fix differs.

A good test: could two different users reading the same error understand whether the problem is their input, their account, or the system? If not, the error is too vague.

### Write For The User's Vocabulary, Not The System's

Error text is where internal leakage happens most. Stack traces, exception class names, status codes, table names, and developer shorthand read as hostile or broken to non-technical users, and as unprofessional even to technical ones.

Translate system causes into user-facing language:

- "NullPointerException" becomes "We could not load this item";
- "409 Conflict / duplicate key" becomes "This name is already taken";
- "JWT expired" becomes "Your session ended. Sign in again.";
- "Foreign key constraint" becomes "You cannot delete this while it is still in use."

Keep status codes and identifiers for logs and support, not for the visible message. If a reference code helps support, surface it quietly ("Error REF-2231") rather than as the headline.

### Preserve The User's Work

An error that clears the form, loses the draft, or resets a multi-step flow doubles the damage. Design errors so that recovery does not require redoing valid work.

Practices:

- keep valid field values when re-rendering an invalid form;
- show field-level errors inline so the user fixes only what failed;
- for partial failures (some items succeed, some fail), show which succeeded and which need attention;
- for network errors during save, offer retry without losing the in-memory draft;
- for session or auth expiry, preserve the intended destination and return the user there after re-auth.

An error that costs the user their input is a second, often larger, error.

### Choose The Right Surface And Persistence

Where an error appears determines whether it is seen and acted on. Match the surface to the severity and scope.

- Inline field errors: best for input validation tied to a specific field; persistent while the problem exists.
- Form-level summary: useful when several fields failed or when the error is cross-field; link each item to its field.
- Toast or snackbar: appropriate for transient, recoverable failures the user may safely ignore; must not be the only signal for critical problems.
- Banner or modal: reserved for blockers that prevent continuing, or for system-wide states.
- Full-page state: for 404, 500, maintenance, or unavailable content where the page itself cannot render.

Avoid the two common failures: critical errors shown only in a disappearing toast, and trivial errors shown in a blocking modal. Persistence should match consequence.

### Do Not Blame Or Shame The User

Tone is not decoration in error copy. "You entered an invalid email" attributes fault and reads as accusatory even when technically correct. "Enter a valid email address" gives direction without accusation.

Avoid:

- "You did not...", "You failed to...", "You are not allowed...";
- exclamation marks and ALL CAPS emphasis;
- sarcastic or cute phrasing that lands badly when the user is already frustrated;
- words like "illegal", "invalid", "illegal value" where neutral phrasing works.

Permission and authorization errors deserve special care. "You do not have access" is accurate; "Access denied" reads as punitive. Where possible, tell the user how to gain access or whom to contact.

### Plan For The Error States You Cannot Predict

Some errors are designed (validation, known failure modes). Others arrive from the network, third parties, or unknown causes. Every surface that can fail should have a graceful fallback even when the specific cause is unknown.

For unknown-cause errors:

- use honest, calm language ("Something went wrong on our side");
- never invent a cause;
- provide a retry action and, where relevant, a way to report or contact support;
- keep the surrounding interface usable so the user is not trapped.

Designing only the happy path and the known errors leaves the most stressful states unowned.

### Consider Localization And Length

Error copy is translated, and translated errors often grow 20-40% in length. Layouts that fit "Invalid input" may break with "Bitte geben Sie einen gültigen Wert ein." Design error containers with flexible height, and avoid error text that depends on exact width or single-line layout.

Avoid idioms, humor, and culture-specific metaphors in error copy; they rarely survive translation and read worse under frustration.

## Common Traps

### Errors That Describe But Do Not Instruct

"Invalid input" tells the user a problem exists but gives no path forward. An error without a next step forces the user to guess, which produces more errors and support tickets.

### Blaming The User Through Grammar

Phrasing centered on "you" and "your invalid X" attributes fault and erodes trust, especially in permission, payment, and account contexts where users are already unsure whether they did something wrong.

### Leaking Internal Detail As The Message

Showing exception names, SQL fragments, stack traces, or raw status codes is not transparency; it is a security and trust problem. Internal detail belongs in logs and support context, not the headline.

### Treating All Errors As Toasts

Transient toasts are appropriate only for recoverable, low-stakes failures. Showing a save failure, payment failure, or permission denial only in a toast that disappears guarantees some users will miss it and believe the action succeeded.

### Clearing Input On Error

Re-rendering a failed form with empty fields punishes correct work and is a leading cause of abandoned forms. Errors should be additive: show what to fix, keep what was right.

### One Error For Many Problems

A single "Please fix the highlighted fields" with no inline detail, or a wall of identical red messages, hides which fields failed and why. Field-level specificity reduces the cognitive cost of recovery.

### Cute Or Brand-Heavy Copy In Failure

Witty, branded, or illustrated error pages can work for low-stakes content, but in payment, medical, legal, or operational contexts they read as flippant. Match tone to stakes.

### Assuming The Cause When It Is Unknown

Filling an unknown-cause error with a plausible-sounding but invented reason ("Your network is unstable") misleads users and support. When the cause is unknown, say so honestly and offer retry.

## Self-Check

- [ ] Each error leads with or clearly includes the action the user should take next, not only a description of the failure.
- [ ] Errors name the specific field, file, record, or constraint involved, and distinguish input, format, range, uniqueness, permission, and system failures.
- [ ] Internal detail such as exception names, stack traces, table names, and raw status codes is kept out of user-facing copy and reserved for logs or support.
- [ ] Valid user input is preserved when an error is shown; the user does not have to retype correct work.
- [ ] The error surface (inline, summary, toast, banner, full page) matches the severity and scope of the failure, and critical errors are not shown only in disappearing toasts.
- [ ] Tone avoids blame, accusation, all caps, exclamation, and shaming phrasing, especially in permission and authorization errors.
- [ ] Unknown-cause errors are honest about uncertainty and provide a retry or contact path without inventing a cause.
- [ ] Error containers handle translated and longer copy without breaking layout or truncating the fix.
- [ ] Partial failures clearly show which items succeeded and which need attention rather than failing the whole batch silently.
- [ ] The error was read in the mindset of a blocked, frustrated user, and the first line alone gives a useful next step.
