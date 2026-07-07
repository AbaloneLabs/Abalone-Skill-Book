---
name: ux_writing_and_microcopy.md
description: Use when the agent is writing or reviewing interface copy, button labels, error messages, confirmation dialogs, toast notifications, empty state text, tooltips, onboarding lines, form hints, or any short UI text that guides action and sets tone.
---

# UX Writing And Microcopy

Microcopy is not decoration around the interface; it is part of how the interface works. A button label decides whether a user commits or hesitates. An error message decides whether the user recovers or gives up. A confirmation dialog decides whether data is saved or destroyed. Short as these strings are, they carry most of the cognitive weight in moments of decision, uncertainty, and failure.

Use this skill before writing or reviewing any short interface text: buttons, links, form labels and helper text, error and validation messages, confirmation and destructive-action dialogs, toasts and banners, empty and loading states, tooltips, onboarding prompts, and system notifications. The goal is to prevent the agent from producing copy that is clever instead of clear, vague instead of actionable, or polite instead of useful.

## Core Rules

### Write For The User's Task, Not For The System's Logic

Users do not care about the internal model, the database, the API, or the engineering architecture. They care about what they were trying to do and whether it worked. Copy should describe the task and outcome in the user's words.

Compare:

- "Submit" versus "Send invoice" or "Publish post";
- "Error 409: resource conflict" versus "This email is already in use. Try logging in.";
- "Operation completed successfully" versus "Your changes were saved.";
- "Select an option" versus "Choose how you want to receive updates."

The system-oriented version forces the user to translate. The task-oriented version lets them act.

### Make Every Action Label Describe Its Consequence

A button label is a promise about what will happen when it is pressed. Vague labels break that promise and create anxiety, especially for destructive or irreversible actions.

Strong labels:

- use a verb that names the action;
- name the object when ambiguity exists;
- distinguish degree and risk, such as "Delete" versus "Delete permanently";
- avoid generic verbs like "OK", "Submit", "Done", or "Continue" when the consequence is not obvious;
- reflect the real outcome, not a euphemism.

"OK" on a delete dialog does not tell the user they are about to lose data. "Delete project" does.

### Write Errors That Enable Recovery

An error message that only states a problem has failed. The user needs to know what went wrong, why, and what to do next, in language they can act on.

A useful error:

- names the field or action affected;
- explains the problem concretely, not as a code;
- tells the user how to fix it, if a fix exists;
- avoids blame and panic;
- appears near the relevant input or action;
- remains available while the user corrects the issue.

Compare "Invalid input" with "Passwords must be at least 12 characters and include a number." The first blocks; the second guides.

### Match Tone To Context And Emotion

Tone is not a fixed brand attribute; it shifts with the user's situation. A playful tone in onboarding can become offensive in an error, a billing failure, or a security warning.

Calibrate tone by context:

- onboarding and success states can be warm and encouraging;
- errors and validation should be plain, direct, and helpful, not humorous;
- destructive actions should be serious and unambiguous;
- financial, legal, medical, and security contexts should be precise and restrained;
- empty states can be inviting without being saccharine.

Humor in a moment of user stress reads as indifference. Match the gravity of the moment.

### Prefer Clarity Over Brevity, Then Trim

The shortest string is not always the clearest. First write the clearest possible version, then remove words that add no meaning. Do not sacrifice comprehension for a character limit.

Review for:

- words that repeat information already in the UI;
- jargon, internal terms, and acronyms the user does not share;
- hedging language like "may", "might", or "could" when the system knows the answer;
- filler like "Please" or "Simply" that lengthens without aiding action;
- truncation that cuts the most important word.

Clarity first; brevity second; wit only when it never obscures.

### Keep Terminology Consistent Across The Product

Users build a mental model from repeated exposure. If the same object is called a "folder" in one place, a "collection" in another, and a "workspace" in a third, the user cannot tell whether they are different things.

Maintain:

- one term per concept, used everywhere;
- consistent verb for the same action, such as always "delete" and never "remove" for the same operation;
- consistent capitalization and punctuation rules;
- a glossary or content style guide that the whole product follows;
- alignment between marketing, onboarding, and in-product copy.

Inconsistency reads as bugs, even when each string is individually correct.

### Write For Translation And Localization

Interface copy rarely stays in one language. Strings written for English often expand by 20-40% when translated, and idioms, humor, and culturally specific references fail.

Write for localization by:

- avoiding idioms, slang, and wordplay;
- keeping sentences simple and complete;
- not concatenating fragments, which breaks grammar in other languages;
- allowing text expansion without layout breakage;
- avoiding hardcoded plurals; use proper pluralization rules;
- being cautious with metaphors, colors, symbols, and gestures that vary culturally.

### Consider Accessibility And Screen Readers

Microcopy is often read aloud by assistive technology. A label that works visually may fail when heard without context.

Check:

- button and link text makes sense out of visual context, not just "Click here";
- icon-only controls have accessible names;
- error messages are programmatically associated with their fields;
- dynamic updates are announced, not silently changed;
- instructions do not rely solely on color, position, or shape;
- abbreviations and symbols expand correctly when read aloud.

## Common Traps

### Cleverness Over Clarity

Witty copy delights the writer and confuses the user, especially under stress. Humor is appropriate only where failure is not possible.

### Generic Confirmation Labels

"OK", "Cancel", "Yes", and "No" force the user to re-read the dialog to know what they are confirming. Specific verbs prevent mistakes.

### Errors That Blame The User

Phrases like "You entered an invalid value" or "You must fix this" sound accusatory. Describe the requirement, not the user's failure.

### Placeholder Text Used As Labels

Once the user types, the instruction disappears, hurting review, error correction, and assistive use. Placeholders supplement labels; they do not replace them.

### Inconsistent Verbs For The Same Action

"Remove", "delete", "discard", and "archive" used interchangeably for the same operation make users unsure what will happen. Pick one and keep it.

### Truncation That Hides The Key Word

A button label cut off as "Delete project set..." loses the most important word. Design labels to fit, or let them wrap.

### False Reassurance

"You're all set" after an action that may have partially failed gives users false confidence. Confirm what actually happened, including partial success.

## Self-Check

- [ ] Action labels use specific verbs that describe the real consequence, and generic labels like "OK" or "Submit" are replaced where the outcome is not obvious.
- [ ] Error and validation messages name the problem, explain it concretely, and tell the user how to recover.
- [ ] Tone matches the context: warm in success and onboarding, plain and direct in errors, serious in destructive and high-stakes situations.
- [ ] Copy is written for the user's task and vocabulary, not for internal system logic, codes, or architecture.
- [ ] Terminology is consistent across the product, with one term per concept and one verb per action.
- [ ] Strings are clear before they are short, and trimming did not remove the most important word.
- [ ] Copy is localization-ready: no idioms, no fragment concatenation, allowance for text expansion, and correct pluralization.
- [ ] Icon-only and link controls have accessible names that make sense when read aloud without visual context.
- [ ] Confirmation and destructive dialogs state exactly what will happen, including irreversibility where relevant.
- [ ] Success and completion messages reflect what actually occurred, including partial success or pending states.