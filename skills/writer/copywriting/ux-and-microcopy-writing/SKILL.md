---
name: ux_and_microcopy_writing.md
description: Use when the agent is writing UX microcopy for buttons, error messages, empty states, tooltips, form labels, onboarding, and confirmation flows, balancing clarity in extreme brevity with the right tone for functional interfaces, accessibility, and brand voice inside constrained UI space.
---

# UX And Microcopy Writing

Microcopy is the writing that lives inside an interface: the button label, the error message, the empty-state line, the tooltip, the form hint, the onboarding step. It looks trivial because it is small, and that appearance is exactly why it is underestimated. In a constrained UI space, every word carries disproportionate weight. A single ambiguous button label can derail a task; a single cold error message can turn a moment of confusion into a moment of alienation. Microcopy is not small writing. It is writing under extreme compression where each word must justify its presence and do measurable work.

The deeper failure is treating microcopy as decoration or as an afterthought applied once the design is done. But microcopy is part of the interface itself. It shapes what the user understands they can do, what they expect to happen when they act, and how they feel about the product when something goes wrong. It is also a brand voice touchpoint: for many users, the only words they ever read from a company are its buttons and error messages. Agents tend to focus on marketing copy and treat functional copy as fill-in. That inversion produces interfaces that look polished but fail at the moments that determine whether users stay.

Use this skill when writing any interface text, when an error message feels cold or confusing, when a button label is ambiguous, when designing onboarding or empty states, or when ensuring microcopy is accessible and on-brand. The agent has latitude in tone and structure. The obligations are that every word is clear in context, that the copy respects the user's emotional state, and that accessibility and brand are handled deliberately within the constraints.

## Core Rules

### Write To The User's Context, Not To The Feature

Microcopy is read inside a task, under a specific emotional and cognitive load. A button on a checkout page is read by someone trying to finish a purchase; an error message is read by someone whose action just failed; an empty state is read by someone expecting content and finding none. The same words land differently depending on context.

Before writing, establish:

- what the user is trying to accomplish in this moment;
- what emotional state they are likely in (confident, anxious, frustrated, curious, hurried);
- what they already understand about the system;
- what they need to know to take the next action;
- what device, attention span, and reading conditions apply.

Copy written to the feature describes what the system does. Copy written to the context tells the user what they can do next and why it matters.

### Prioritize Clarity Over Cleverness

In functional interfaces, clarity beats wit. A clever label that delights on first read but confuses on every subsequent use has failed. Reserve personality for moments where it aids comprehension or reassurance, never where it competes with it.

Test clarity by asking:

- Could a first-time user predict what will happen when they act on this?
- Does the label describe the action or the destination, consistently with the rest of the product?
- Is there a simpler, more common word that means the same thing?
- Would removing a word make it clearer or just shorter?

Brevity is a means to clarity, not a virtue on its own. Trimming words until meaning suffers is not good microcopy; it is starvation.

### Make Error Messages Empathetic And Actionable

Error messages are where products most often lose users, because errors arrive at the worst moment and are usually written defensively, from the system's perspective. Good error copy does two things: it explains what happened in human terms, and it tells the user what to do next.

For every error message, ensure:

- it names the problem specifically, not just "Something went wrong" when more detail is available and useful;
- it avoids blaming the user; replace "You entered an invalid email" with "That email address looks incomplete";
- it offers a clear next step or fix when one exists;
- it matches the severity of the situation; a fatal crash and a missing field should not sound the same;
- it is findable and associated with the relevant element for accessibility.

Empathy in errors is not softness; it is keeping the user moving instead of abandoning them at the point of failure.

### Design Empty States As Invitations

Empty states are not absences. They are moments where the user has arrived at a place expecting something and found nothing, and the product can either lose them or guide them. An empty state should explain what will appear there, why it is empty now, and what the user can do to fill it.

A strong empty state includes:

- a clear statement of what the space is for;
- a reason it is currently empty (you have not added anything yet, no results matched);
- a primary action that moves the user forward;
- tone appropriate to the moment; an empty inbox is different from an empty search for a missing person.

Do not leave empty states as silent white space or generic placeholders. They are conversion and retention opportunities disguised as gaps.

### Keep Button Labels Verbs That Describe The Action

Buttons are commitments. Their labels should tell the user what action they are confirming. Prefer specific verbs over generic ones, and avoid labels that describe the system state instead of the user action.

Compare:

- "Submit" (generic, system-oriented) versus "Send invoice" (specific, action-oriented);
- "OK" (meaningless) versus "Delete file" (commits clearly to consequence);
- "Continue" (vague) versus "Review order" (sets accurate expectation).

For destructive actions, make the consequence unmistakable and differentiate the destructive button visually and verbally so users do not trigger it by accident.

### Handle Form Labels And Hints For Accuracy And Accessibility

Form copy determines whether users complete the form correctly. Labels must be unambiguous, and hints must appear only where users genuinely need help, because extra instruction adds load.

Apply:

- labels that name exactly what is needed, placed consistently;
- placeholder text that is not the only carrier of the label, since placeholders vanish on input and harm accessibility;
- helper text that explains format or constraints only where non-obvious;
- required-field indication that is accessible to screen readers, not only visual;
- validation that confirms correct input and corrects errors inline where possible.

Accessibility is not a separate concern from microcopy. Clear labels, predictable structure, and screen-reader-friendly text are part of writing the copy well.

### Use Tooltips And Progressive Disclosure Sparingly

Tooltips and hidden help reduce clutter but add discovery cost. Use them when information is needed by some users but not most, or when it would overwhelm the default view. Do not hide essential instructions behind a hover that many users, especially on touch devices or with assistive technology, will never find.

### Align Microcopy With Brand Voice Without Letting It Intrude

Microcopy is a brand touchpoint, but brand voice in the interface must serve function first. A playful brand can use warmth in empty states and confirmations, but should not let personality obscure a button's meaning or an error's fix. Define where voice has room (onboarding, empty states, success moments) and where it must recede (destructive actions, errors, critical confirmations).

## Common Traps

### System-Centric Language

Writing from the system's point of view ("Invalid input," "Form submission failed") instead of the user's makes the product feel hostile. Reframe around what the user was trying to do and how to recover.

### Ambiguous Button Labels

Generic labels like "Submit," "OK," or "Click here" force users to guess the outcome, which produces hesitation and errors. Use specific verbs tied to the action and consequence.

### Over-Brevity That Sacrifices Meaning

Cutting words until the label becomes cryptic is false economy. A shorter string that confuses costs more support load than a slightly longer one that is instantly clear.

### Blame-Shifting Errors

Phrasing that implies the user is at fault ("You must enter a valid date") erodes trust, especially when the fault is often the system's unclear format requirement. Describe the problem and the fix without accusation.

### Placeholder-Only Labels

Using placeholder text as the label disappears the moment the user starts typing, leaving them unsure what field they are in and breaking screen-reader expectations. Always pair a visible label with any placeholder.

### Inconsistent Terminology

Calling the same thing "folder" in one place and "directory" in another, or "save" and "update" interchangeably, creates cognitive load and doubt. Standardize terms across the product.

### Personality In The Wrong Place

Injecting brand wit into errors, destructive confirmations, or moments of user stress reads as tone-deaf. Reserve voice for low-stakes, positive moments.

### Ignoring Accessibility

Treating clarity, structure, and screen-reader behavior as someone else's job produces microcopy that excludes users. Accessible copy is part of writing it correctly, not an add-on.

### Treating Microcopy As Fill-In

Leaving interface text to the end, or to whoever is available, guarantees inconsistency and weak moments at exactly the points that shape retention. Microcopy deserves the same rigor as any other copy.

## Self-Check

Before treating microcopy as complete, verify:

- Each piece of copy is written to the user's task context and emotional state, not to the feature's description.
- Clarity takes priority over cleverness, and brevity serves meaning rather than starving it.
- Error messages explain the problem in human terms without blaming the user and provide a clear next step.
- Empty states explain the space, why it is empty, and offer a forward action rather than sitting as blank gaps.
- Button labels are specific verbs describing the action and consequence, with destructive actions unmistakably marked.
- Form labels are visible and unambiguous, placeholders are not the sole carrier of the label, and helper text appears only where needed.
- Required fields and validation are accessible to screen readers and not only visually indicated.
- Tooltips and hidden help are used only for non-essential information and are reachable on touch and assistive devices.
- Brand voice is present where it aids the experience and recedes where it would intrude on clarity or sensitivity.
- Terminology is consistent across the product, and no copy was left as placeholder fill-in.
