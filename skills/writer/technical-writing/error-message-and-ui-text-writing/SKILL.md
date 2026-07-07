---
name: error_message_and_ui_text_writing.md
description: Use when the agent is writing error messages, warnings, UI labels, tooltips, empty states, or microcopy for a software interface, deciding how to phrase errors so users can recover, balancing clarity with brevity in constrained UI space, handling tone in failure states, writing for accessibility and internationalization, or diagnosing UI text that confuses, blames, or fails to guide the user.
---

# Error Message And UI Text Writing

UI text, the labels, buttons, messages, tooltips, and microcopy that populate a software interface, is writing under extreme constraint. Every word competes for space on a screen the user is scanning, not reading. Every error message arrives at a moment of friction, when the user has been blocked from their goal and is potentially frustrated. Every label must be understood instantly, without explanation, by users of varying expertise. Unlike prose, which can build context over paragraphs, UI text must communicate completely in a phrase. The discipline is writing that achieves clarity, tone, and guidance in minimal space, serving users who are task-focused, often impatient, and encountering the text at the moment they need it most. Poor UI text creates support burden, user error, and frustration; effective UI text removes friction and helps users succeed.

Agents tend to miss that error messages must enable recovery not just describe failure, that tone in failure states shapes the user's emotional response, that brevity must not sacrifice clarity, and that UI text must work for accessibility (screen readers) and internationalization (translation). The harm is interfaces with error messages that blame the user, labels that are ambiguous, and microcopy that confuses rather than guides.

Use this skill when writing error messages, UI labels, tooltips, empty states, or microcopy, deciding phrasing for failure states, or diagnosing UI text problems. The goal is to write UI text that is clear, actionable, respectful, and effective in the constrained, task-focused context of a software interface.

## Core Rules

### Write Error Messages That Enable Recovery

An error message has one primary job: to help the user recover and proceed. A message that describes the error without enabling recovery ("Error: invalid input") tells the user something is wrong but not what to do about it, leaving them stuck. A message that enables recovery ("Please enter a date in MM/DD/YYYY format") tells the user what is wrong and how to fix it, so they can proceed.

Structure error messages to include: what went wrong (stated clearly, without jargon), why it went wrong (if the user needs to understand the cause), and what to do next (the specific action to recover). Not every message needs all three; a simple validation error ("Please enter a valid email address") needs only what and what-next. A complex error ("The file could not be uploaded because it exceeds the 10MB limit. Try compressing the file or choosing a smaller one.") needs all three. The test is: after reading this message, does the user know what to do? If not, the message is incomplete. Avoid messages that only state the problem ("Upload failed") without recovery guidance, and avoid messages that describe the system's internal state ("Exception in multipart parser at line 42") that is meaningless to the user.

### Blame The System, Not The User

When something goes wrong, the user is already frustrated. An error message that blames the user ("You entered an invalid date") compounds the frustration by implying fault. A message that frames the error neutrally or attributes it to the system ("Please enter a date in MM/DD/YYYY format" or "We couldn't process your request") preserves the user's goodwill and focuses attention on recovery rather than blame.

Avoid "you" phrasing that implies user error ("You must," "You entered," "You forgot"). Use neutral phrasing that describes the requirement or the system state ("A date is required," "Please enter," "We couldn't"). For system errors (server timeouts, failed connections), take responsibility on behalf of the system ("Something went wrong on our end. Please try again.") rather than implying the user's action was wrong. The exception is genuine user errors that need correction, where a gentle, neutral framing ("That email address doesn't look right") is better than a blaming one ("You typed your email wrong"). The principle is: the user is trying to accomplish a goal, and the interface's job is to help them, not to judge them.

### Prioritize Clarity Over Brevity, Then Optimize For Both

UI text must be brief, because screen space is limited and users scan rather than read. But brevity must not sacrifice clarity. A short label that is ambiguous ("Submit" for a button that deletes data) is worse than a longer label that is clear ("Delete Project"). The hierarchy is: clarity first (the user must understand), then brevity (as short as possible without losing clarity).

Write the clearest possible text first, then edit for brevity without losing clarity. "Please enter your email address so we can send you a confirmation" is clear but verbose; "Enter your email for confirmation" is clear and briefer; "Email" as a label with a placeholder is clearest in context. The right brevity depends on context: a button label must be very short (one to three words); a tooltip can be a sentence; an error message can be two sentences if needed for recovery. Do not force all UI text to the same length; match the length to the context and the information that must be conveyed. Test clarity by asking: would a user who scans this for one second understand what it does and what to do? If not, it needs revision, even if it is short.

### Write Labels And Buttons As Actions And Objects

Labels and buttons communicate what the user can do and what they act upon. Effective labels follow a consistent pattern of action (verb) and object (noun), so the user understands both what will happen and to what. "Delete File" (action + object) is clearer than "Delete" (action alone, object implied) or "File" (object alone, action implied).

Use verb-noun patterns for actions: "Create Project," "Send Email," "Export Report." This pattern tells the user what will happen and to what, reducing ambiguity. For navigation and state labels, use clear nouns: "Settings," "Dashboard," "Inbox." Avoid vague labels like "OK" or "Submit" for consequential actions, because they do not communicate what will happen; use specific labels ("Save Changes," "Place Order"). For destructive actions, make the label explicit about the consequence ("Delete Account" rather than "Remove"), and consider confirmation for irreversible actions. Consistency in labeling patterns (always verb-noun for actions, always nouns for navigation) helps users predict what labels mean across the interface.

### Handle Tone For Different UI States

The tone of UI text varies by state. A success message may be warm and encouraging. An error message must be neutral and helpful, not alarming or blaming. An empty state (a screen with no content yet) is an opportunity to guide and encourage. A warning must be serious without inducing panic. Matching tone to state ensures the text feels appropriate to the user's situation and emotional context.

For success states, a brief, warm confirmation ("Project created! You can start adding tasks.") reinforces the user's progress. For errors, neutral, recovery-focused tone (as above) prevails. For empty states, use the space to explain what the user can do ("No projects yet. Create your first project to get started.") rather than just stating the emptiness ("No projects"). For warnings, be clear about the risk and the choice ("Saving will overwrite the existing draft. Continue?") without dramatizing. Across all states, avoid humor in failure contexts (the user is not amused when blocked), avoid exclamation points in error messages (they read as alarming or cheerful-inappropriate), and maintain a consistent voice that feels like a helpful, respectful assistant rather than a robot or a marketer.

### Write For Accessibility And Screen Readers

UI text is not only read visually; it is also read aloud by screen readers used by people with visual impairments. Text that works visually may fail when read aloud: ambiguous labels ("Click here"), text that relies on visual context (a "Delete" button next to an item, where the screen reader does not convey the proximity), or text that is an image without alternative text. Writing for accessibility ensures the interface is usable by all users, including those who navigate by sound.

Write descriptive labels that make sense without visual context: "Delete Project Alpha" rather than "Delete" (which a screen reader may announce without the associated item). Provide alternative text for icons and images that convey information. Ensure form fields have associated labels (not just placeholders, which disappear and are not always read by screen readers). Avoid instructions that rely on sensory information ("click the red button") that may not be perceivable. Write link text that describes the destination ("Read the privacy policy") rather than generic text ("click here"). Accessibility writing is not a separate task from good UI writing; it is good UI writing that accounts for all the ways users perceive the interface.

### Write For Internationalization And Translation

If the interface may be translated into other languages, the UI text must be written to translate well. Text that is concise and clever in English may become ambiguous, too long, or culturally inappropriate when translated. Writing for internationalization means writing text that is clear, not idiom-dependent, and structurally robust enough to survive translation.

Avoid idioms ("hit the ground running"), culture-specific references, and humor that does not translate. Write in simple, direct sentences that a translator can render accurately. Be aware that text length changes in translation (German and French translations are often 20-40% longer than English), so UI elements must accommodate text expansion or the layout will break. Avoid concatenating text fragments ("Welcome, " + userName + "!") because word order differs across languages; use complete strings with placeholders ("Welcome, {name}!"). Avoid embedding meaning in capitalization or punctuation that may not exist in other languages. If internationalization is a requirement, write with translation in mind from the start, because retrofitting translatable text is more work than writing it that way initially.

## Common Traps

### Error Messages That Describe Without Enabling Recovery

"Error: invalid input" leaves the user stuck. Every error message should tell the user what to do to recover.

### Blaming The User With "You" Phrasing

"You entered an invalid date" implies fault. Use neutral framing that focuses on the requirement and recovery.

### Sacrificing Clarity For Brevity

A short, ambiguous label is worse than a longer, clear one. Prioritize clarity, then optimize for brevity.

### Vague Button Labels Like "OK" Or "Submit"

Generic labels do not communicate what will happen. Use specific action-object labels ("Delete Project," "Place Order").

### Humor Or Exclamation Points In Error Messages

Users are frustrated when blocked, not amused. Keep error tone neutral and recovery-focused.

### Ignoring Screen Reader Accessibility

Labels that rely on visual context fail when read aloud. Write descriptive text that works without visual proximity.

### Idioms And Concatenated Fragments That Break In Translation

Idioms do not translate, and concatenated fragments break across word orders. Write clear, complete strings for internationalization.

## Self-Check

- [ ] Do error messages enable recovery by stating what went wrong and what to do next, so the user can proceed after reading them?
- [ ] Is error phrasing neutral and system-attributing rather than blaming the user, avoiding "you" constructions that imply fault?
- [ ] Is clarity prioritized over brevity, with text then optimized for brevity without losing clarity, and length matched to context (short labels, fuller error messages)?
- [ ] Do button and action labels follow a verb-noun pattern that communicates both the action and the object, avoiding vague labels like "OK" or "Submit"?
- [ ] Does the tone match the UI state (warm for success, neutral for errors, guiding for empty states, serious-but-calm for warnings), without humor or exclamation points in failure contexts?
- [ ] Is the text written for accessibility, with descriptive labels that work without visual context, alternative text for icons, and form fields with associated labels?
- [ ] If the interface may be translated, is the text written for internationalization: no idioms, simple direct sentences, complete strings with placeholders, and awareness of text expansion?
- [ ] Can a user scanning the interface for one second understand what each element does and what to do, without prior context?
