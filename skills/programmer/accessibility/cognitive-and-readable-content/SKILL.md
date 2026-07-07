---
name: cognitive_and_readable_content.md
description: Use when the agent is writing or structuring page content, microcopy, instructions, or help text for clarity and reading level, designing navigation consistency and predictable behavior, managing focus visibility and context changes, handling session timeouts and time limits, preventing user errors, or reducing cognitive load in a UI. Also covers WCAG's operable and understandable principles as they relate to attention, memory, and comprehension, and the failure mode of an interface that is technically operable but mentally exhausting, unpredictable, or punitive toward users who read or process slowly.
---

# Cognitive And Readable Content

Cognitive accessibility is the discipline of making an interface understandable and usable for people whose challenges are not sensory or motor but cognitive: users with attention disorders, memory or processing difficulties, low literacy or a non-native reading level, cognitive fatigue, or simply anyone who is tired, stressed, distracted, or reading on a small screen in a hurry. It is the largest and least visible accessibility population, because cognitive differences are rarely diagnosed, rarely disclosed, and rarely represented in testing. An interface can be fully keyboard-operable, correctly contrasted, and semantically perfect, and still be cognitively inaccessible: dense jargon, unpredictable navigation that moves between pages, a timeout that logs the user out mid-task, an error message that blames rather than helps, a focus indicator that vanishes, or a flow that demands the user hold several things in memory at once.

Agents tend to under-invest here because the harm is invisible and the metrics are fuzzy. Color contrast has a ratio; keyboard access has a pass-or-fail test; cognitive load has no single number, so it is easy to assume "clear enough" and move on. But the cumulative effect of small cognitive frictions — a label that assumes prior knowledge, a button whose action is ambiguous, a form that resets on error, a page that reorganizes itself on every visit — is exclusion at scale. The judgment problem is deciding, for every piece of content and every interaction, what a user who reads slowly, holds less in memory, and is easily disoriented needs in order to understand and complete the task. This skill covers the decisions that determine whether an interface is mentally usable, grounded in WCAG's operable and understandable principles.

## Core Rules

### Write In Plain Language At A Reading Level Real Users Can Manage

Plain language is not dumbing content down; it is writing so the user's effort goes to understanding the message rather than decoding the prose. The practical targets: short sentences, common words, active voice, one idea per sentence, and a reading level appropriate to the audience — commonly around an upper-elementary to middle-school level for general public content, higher only when the audience is specialist. Define jargon and abbreviations on first use, or avoid them. Use headings, bullet lists, and short paragraphs so the structure itself communicates the shape of the information, letting users scan before they read deeply.

The trap is writing for the author or the reviewer rather than the user. Internal teams write in the organization's jargon because it is precise to them; legal and compliance text is written to survive audit; marketing writes to impress. Each of these optimizes for a non-user. For user-facing content — instructions, error messages, button labels, help text, form questions — rewrite until a user who has never seen the product can understand it on first reading. Test by reading it aloud: if you stumble or have to re-read, the user will too. Plain language is the single highest-leverage cognitive accessibility decision, because every user benefits and the users who depend on it are otherwise excluded.

### Keep Navigation Consistent And Behavior Predictable

Users build a mental model of how an interface works, and they rely on it to move efficiently. When navigation appears in different places, uses different labels for the same action, or reorganizes between pages, the model breaks and the user must rebuild it each time, which is exhausting and error-inducing. Keep primary navigation consistent across pages — same location, same labels, same order. Use the same component for the same function everywhere (a primary action always looks and behaves like a primary action). Do not change the meaning of a control based on context without making that context unmistakable.

Predictability extends to behavior. A link should navigate; a button should trigger an action; clicking something should not unexpectedly submit a form, open a new tab, change context without warning, or move focus to a different part of the page. WCAG's understandable principle (3.2) requires that components appear and operate consistently and that context changes happen only on user request or with warning. The practical rule: no surprises. If an interaction will change the user's context — opening a new window, submitting, navigating away from unsaved work — signal it in advance, ideally in the control's label ("Open in new tab", "Save and continue"). Surprise context changes are a primary cause of disorientation, especially for users with cognitive or attention-related needs.

### Manage Focus Visibly And Deliberately On Context Changes

A user who navigates by keyboard, or who uses a screen reader, depends on knowing where they are at every moment. Visible focus is covered in depth by the keyboard and visual accessibility skills, but its cognitive dimension is specific: when focus is lost or jumps unpredictably, the user loses their place and must search for it, which compounds cognitive load. Never remove the focus indicator without an equally visible replacement, and when a context change occurs — opening a dialog, revealing new content, navigating in a single-page app — move focus deliberately to where the user's attention should go, rather than leaving them stranded or jumping them without signal.

The same principle governs dynamic content. When something appears or updates (a result list filters, a message appears, an error shows), the user must be able to perceive it without hunting. Use live regions for screen-reader users, and for sighted users ensure the change is visible and not buried below the fold or in a place the eye does not land. Cognitive load rises sharply when the user must actively search for what changed; deliberate focus and visible change management reduce that load.

### Respect Time Limits And Make Session Timeouts Forgiving

Time limits are one of the most punitive cognitive barriers. A session that times out and logs the user out mid-task destroys their work and forces them to restart, which is devastating for users who read slowly, type slowly, use dictation, are interrupted, or need to look something up before continuing. WCAG (2.2) requires that users can turn off, adjust, or extend any time limit, and that they are warned before it expires and given time to respond. The practical implementation: warn the user before a timeout with a clear, announced message and a way to extend; never silently expire a session that holds unsaved work.

Where a timeout exists for security (banking, sessions with sensitive data), make it as long as the risk allows, warn well before it hits, and preserve as much state as possible on re-authentication so the user resumes rather than restarts. Avoid auto-advancing carousels, slides that move on a timer, and any content that changes on a schedule the user cannot control — these demand the user keep pace with the interface rather than the reverse. The governing question: does any part of this interface demand the user hurry? If yes, remove the demand or give the user control over it.

### Prevent Errors Rather Than Only Catching Them

Error prevention is more cognitively accessible than error recovery, because recovering from an error demands the user notice the error, understand it, find the field, and re-enter — a multi-step burden that compounds for users with memory or attention needs. WCAG's understandable principle (3.3) makes error prevention explicit for legal and financial transactions and for submissions that store user data: make submissions reversible, check them for errors, and give the user a chance to confirm before finalizing. Build prevention into the design: confirm destructive actions, show a summary before final submission, let users undo rather than only redo, validate inline as the user fills so problems surface early, and default to safe choices.

When errors do occur, make recovery easy and non-punitive. The error message should explain what happened in plain language, identify the field, and suggest a correction, without blaming the user ("Invalid input" is punitive; "Please enter a date as MM/DD/YYYY" is helpful). Preserve the user's valid input so they fix only what failed. A form that clears on error, or an error that is phrased as the user's fault, multiplies cognitive load and causes abandonment. The cognitive-accessibility lens on errors is always: how much work does recovery demand of the user, and how can the interface reduce that work?

### Reduce Cognitive Load Through Structure, Defaults, And Progressive Disclosure

Cognitive load is the total mental effort the interface demands, and it accumulates from many small sources: too many choices visible at once, too much text, too many steps, too much to hold in memory. Reduce it structurally. Break long flows into clear, numbered steps and show progress so the user knows where they are and how much remains. Use sensible defaults so the user decides less. Reveal complexity progressively — show the common path plainly and hide advanced options behind a disclosure — rather than presenting every option at once. Group related information and actions so the user processes in chunks rather than as a flat list. Let the interface remember what it can (prefill known values, persist drafts) so the user remembers less.

Avoid demanding that the user hold information in memory across steps or pages. If a later step depends on an earlier choice, show the earlier choice rather than expecting recall. If instructions must be followed in order, keep them visible while the user acts rather than requiring them to remember a sequence. The test: walk through the flow as a user who is distracted and reading once, and count how many things you must hold in memory, how many choices you face at once, and how often you must re-orient. Each of those is cognitive load to reduce.

## Common Traps

### Jargon And Long, Complex Sentences Aimed At The Author

Writing instructions, labels, and errors in the organization's internal vocabulary and in long passive sentences, so the user's effort goes to decoding rather than understanding. Rewrite user-facing text in plain language at the audience's reading level; define or remove jargon.

### Inconsistent Navigation And Labels Across Pages

Primary navigation that moves, relabels, or reorders between pages, or the same action labelled differently in different places, breaking the user's mental model and forcing constant re-learning. Keep navigation, labels, and component behavior consistent across the product.

### Surprise Context Changes

A link that opens a new tab, submits a form, or navigates away from unsaved work with no warning, disorienting the user. Signal context changes in the control's label or with an announcement; never change context without the user's request or a clear warning.

### Session Timeout That Destroys Unsaved Work

A timeout that silently logs the user out or clears a form mid-task, with no warning and no way to extend or resume. Warn before expiry, let the user extend, and preserve state across re-authentication so the user resumes rather than restarts.

### Punitive Or Vague Error Messages

"Invalid input" or "Error" with no field identified and no correction suggested, which blames the user and demands they diagnose the problem. Identify the field, explain in plain language, and suggest a correction; preserve valid input.

### Too Many Choices Or Steps Demanding Memory At Once

A long form or dense page that presents every option simultaneously, or a multi-step flow that expects the user to recall earlier choices. Use progressive disclosure, sensible defaults, visible progress, and re-display of earlier context to reduce what the user must hold in memory.

### Invisible Or Lost Focus On Dynamic Changes

A context change or content update that moves nothing and announces nothing, so the user does not know where they are or what changed. Move focus deliberately on context changes and make dynamic updates perceivable through visible change and live regions.

## Self-Check

- [ ] User-facing content (instructions, labels, errors, help text) is written in plain language at a reading level the audience can manage — short sentences, common words, active voice, jargon defined or removed — verified by reading it aloud and checking it is understandable on first reading.
- [ ] Navigation, labels, and component behavior are consistent across pages — same location, same labels, same order, same action rendered the same way — so the user's mental model holds.
- [ ] No surprise context changes: links and buttons behave as labelled, new tabs and submissions are signalled, and context changes happen only on user request or with a clear warning.
- [ ] Time limits and session timeouts are forgiving: the user can turn off, adjust, or extend them, is warned before expiry, and can resume with state preserved rather than restarting from scratch.
- [ ] Errors are prevented where possible (inline validation, confirmations, reversible actions, safe defaults) and, when they occur, are identified in plain language with the field named and a correction suggested, preserving valid input.
- [ ] Cognitive load is reduced structurally: flows are broken into clear numbered steps with visible progress, advanced options are progressively disclosed, sensible defaults reduce decisions, and the interface remembers what it can.
- [ ] The user is never required to hold information in memory across steps — earlier choices and instructions are re-displayed when needed — and focus is managed visibly and deliberately on every context change.
- [ ] The interface was reviewed for predictability and load from the perspective of a user who reads slowly, is easily distracted, and is encountering the product for the first time — not only from the perspective of a familiar, fast, sighted tester.
