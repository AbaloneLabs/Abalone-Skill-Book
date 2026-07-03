---
name: ux_and_product_copywriting.md
description: Use when the agent is writing UX and product microcopy, designing onboarding flows and empty states, crafting error messages and button labels, ensuring clarity over cleverness, grounding copy in user research, writing for localization, or meeting accessibility needs in interface text.
---

# UX And Product Copywriting

UX and product copy is the text people encounter inside a product: buttons, labels, menus, onboarding, empty states, error messages, tooltips, and notifications. Unlike marketing copy, it appears in moments of friction, confusion, and decision, often when the user is already frustrated or in a hurry. UX copy fails when it prioritizes cleverness over clarity, when it describes the system instead of the user's goal, when an error message blames the user for the product's failure, or when it was never tested with real users and assumes vocabulary they do not share. It also fails silently across borders when it cannot be translated or read by assistive technology. The craft is removing friction with words so precisely that the user never notices the copy at all.

Use this skill before writing or revising interface text, designing onboarding or empty states, crafting error and system messages, or auditing a product's copy for clarity and accessibility. The goal is to prevent the agent from producing microcopy that looks fine in a design file but confuses, frustrates, or excludes real users.

## Core Rules

### Prioritize Clarity Over Cleverness

In a product interface, the user has a goal and the copy is in the way until it helps. Wit that delays understanding is a cost, not a benefit.

Write clearly by:

- using plain language the user actually uses, not internal terminology;
- choosing the shortest clear phrasing, especially on small screens;
- making every label, button, and instruction unambiguous;
- reserving personality for moments where it aids rather than obstructs;
- testing whether users understand the copy without explanation.

Cleverness has a place, but only after clarity is guaranteed. A funny empty state that users do not understand is a failure. A clear empty state that users act on correctly is a success.

### Describe The User's Goal, Not The System

Interface copy should reflect what the user is trying to accomplish, not how the system is built. System-centric language confuses people who do not share the mental model.

Translate system language to user language:

- name actions by their outcome, such as "Save changes" not "Commit transaction";
- avoid internal jargon, status codes, and database field names;
- frame states from the user's perspective, such as "Your draft is saved" not "Record persisted";
- use the words users use, which user research should reveal;
- hide technical complexity unless the user genuinely needs it.

When copy forces users to learn the product's internal vocabulary, every new user pays a tax in confusion and support load.

### Design Error Messages That Help, Not Blame

Error messages are high-stakes moments: the user is blocked and often frustrated. The copy must resolve, not accuse.

Write helpful errors by:

- explaining what went wrong in plain language;
- telling the user exactly how to fix it, with a path forward;
- avoiding blame language like "you entered" or "invalid";
- taking responsibility for system failures rather than scolding the user;
- differentiating recoverable errors from unrecoverable ones clearly.

"You entered an invalid email" blames and mystifies. "That email looks incomplete. Check for a missing @" explains and guides. The difference shapes whether users recover or abandon.

### Write Buttons And Labels As Actions

Buttons are commitments. Their labels must make the consequence of clicking unambiguous.

Write action labels by:

- using verb phrases that describe the action, such as "Create account" not "Submit";
- making the label match the outcome the user expects;
- differentiating primary, secondary, and destructive actions clearly;
- avoiding vague labels like "OK" or "Continue" when the action is consequential;
- ensuring paired actions like confirm and cancel are instantly distinguishable.

A button labeled "OK" on a delete confirmation is a usability hazard. "Delete project" tells the user exactly what happens next.

### Build Onboarding And Empty States That Guide

Onboarding and empty states are where users form their mental model of the product. They must teach and motivate, not just decorate.

Design them by:

- explaining value and next steps in the user's language;
- showing what is possible, not just what is empty;
- providing a clear first action rather than a blank canvas;
- progressive disclosure that reveals complexity as needed;
- avoiding walls of text in favor of scannable, actionable guidance.

An empty state that says "No items yet" is a dead end. One that says "No items yet. Add your first to get started" with a button turns a dead end into a path.

### Ground Copy In User Research

Interface copy should be tested with real users, not assumed. The words the team uses are often not the words users understand.

Ground copy by:

- testing labels and instructions in usability sessions;
- reviewing support tickets and search logs for vocabulary mismatches;
- checking comprehension with users from different backgrounds and literacy levels;
- iterating on copy that causes confusion, shown by repeated questions or errors;
- validating that tone and personality aid rather than distract.

Copy that tests well internally can still fail users. Research is what separates assumed clarity from verified clarity.

### Write For Localization And Translation

Most products ship in multiple languages. Copy written only for one language breaks when translated.

Write localization-ready copy by:

- avoiding idioms, slang, and culture-specific references;
- leaving room for text expansion, since translations are often longer;
- avoiding concatenated strings that break grammar in other languages;
- using sentence case and clear structure that survives reordering;
- providing context to translators about where and how text appears.

A clever English pun becomes nonsense or offense in another language. Design for translation from the first draft, not as an afterthought.

### Meet Accessibility Requirements

UX copy is part of accessibility. Text that cannot be read, understood, or navigated excludes users.

Ensure accessibility by:

- writing meaningful link and button text, not "click here";
- providing alt text for meaningful images and icons;
- ensuring copy works when read aloud by screen readers;
- avoiding conveying meaning through color or icons alone;
- using sufficient contrast and legible type, in partnership with design;
- keeping instructions clear for users who navigate without a mouse.

Accessible copy is not a separate workstream; it is baseline competence. Inaccessible microcopy excludes users and creates legal risk.

### Maintain Consistency Across The Product

Inconsistent terminology and tone across a product erode trust and increase cognitive load. Treat copy as a system.

Maintain consistency by:

- using a shared glossary of approved terms;
- defining patterns for common messages like errors, confirmations, and empty states;
- coordinating voice across features and teams;
- documenting decisions so new writers follow established conventions;
- auditing the product periodically for drift.

A product that calls the same thing three different names across screens teaches users it was built by disconnected teams.

## Common Traps

### Cleverness Over Clarity

Witty microcopy that delays understanding costs the user time and confidence for a moment of brand personality.

### System-Centric Language

Labels and messages that reflect internal architecture force users to learn the product's vocabulary instead of their own.

### Error Messages That Blame

Accusatory or cryptic errors frustrate users at the exact moments they need help, driving abandonment and support load.

### Vague Action Labels

Buttons like "OK" or "Submit" leave users uncertain about what will happen, especially on destructive or consequential actions.

### Empty States As Dead Ends

Blank or unhelpful empty states give users no path forward and squander a key teaching moment.

### Untested Copy Assumed Clear

Vocabulary that makes sense internally often confuses real users, revealed only by support volume and failed tasks.

### Unlocalizable Writing

Idioms, concatenated strings, and culture-bound references break when translated, producing nonsense in other languages.

### Ignoring Accessibility

Copy that relies on color, vague links, or unreadable structure excludes users who navigate differently.

## Self-Check

- [ ] Clarity comes before cleverness, with personality reserved for moments where it aids understanding.
- [ ] Labels and messages describe the user's goal and outcome, not the system's internal structure.
- [ ] Error messages explain the problem in plain language and provide a clear path to fix it, without blaming the user.
- [ ] Buttons and actions use specific verb phrases that make the consequence of clicking unambiguous.
- [ ] Onboarding and empty states guide users to a clear next action rather than presenting dead ends.
- [ ] Copy was tested with real users for comprehension, not assumed clear by the team.
- [ ] Writing avoids idioms and concatenated strings and leaves room for translation expansion.
- [ ] Link and button text is meaningful, alt text is provided, and meaning is not conveyed by color alone.
- [ ] Terminology and message patterns are consistent across the product via a shared glossary and documented conventions.
- [ ] Copy supports users of varying literacy, language, and ability rather than only the default user.
