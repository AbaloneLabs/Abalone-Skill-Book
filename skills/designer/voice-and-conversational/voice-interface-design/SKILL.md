---
name: voice_interface_design.md
description: Use when the agent is designing a voice interface, voice assistant, speech command flow, voice search, wake-word interaction, dictation, audio feedback, or hands-free and eyes-free experiences, including error recovery for misrecognition and privacy of voice input.
---

# Voice Interface Design

Voice interfaces remove the visual safety net that graphical UIs rely on. There is no persistent screen to glance back at, no visible menu of options, no obvious affordance for what is possible. The user must remember state, infer capabilities, and recover from errors entirely through listening and speaking. A voice flow that reads well in a script often collapses in a noisy kitchen, a moving car, or a multilingual household.

Use this skill before designing voice assistants, voice commands, speech-driven search, dictation flows, wake-word systems, or hands-free and eyes-free experiences. The goal is to prevent the agent from designing voice as if it were a chatbot read aloud, ignoring recognition failure, ambient noise, privacy, memory load, and the reality that users cannot see what the system can do.

## Core Rules

### Design For Ephemeral State And Limited Memory

Voice is transient. Once spoken, audio is gone unless the system repeats or persists it. Users cannot scroll back. This makes memory load the central constraint.

Mitigate memory load by:

- keeping prompts short and structured;
- offering a small number of choices, ideally three or fewer at a time;
- repeating or confirming critical information;
- persisting important results to a screen when one is available;
- allowing the user to ask "what can I say?" or "what are my options?" at any point.

Never assume the user remembers a long list spoken once.

### Make Capabilities And Boundaries Discoverable

The hardest problem in voice is that users do not know what they can say. Unlike a screen with visible buttons, a voice assistant offers no affordance. Design for discoverability:

- state example commands during onboarding and first use;
- offer help and example prompts proactively when the user hesitates;
- accept multiple phrasings for the same intent rather than one rigid phrase;
- gracefully handle out-of-domain requests by explaining what the assistant can do.

Rigid single-phrase commands punish natural speech and make the interface feel brittle.

### Plan For Recognition Failure As The Default Case

Speech recognition is imperfect. Background noise, accents, children's voices, multilingual switching, homophones, and proper nouns all cause failures. A voice design that assumes perfect recognition will frustrate users.

Build error recovery into every flow:

- confirm before destructive or irreversible actions;
- allow correction ("no, I meant...") without restarting the flow;
- narrow ambiguity by asking a clarifying question rather than guessing;
- offer a visual fallback or handoff to a screen when voice fails repeatedly;
- distinguish "I didn't hear you" from "I heard you but didn't understand" from "I understood but can't do that."

Each failure type needs a different recovery path.

### Respect The Hands-Free, Eyes-Free Context

Users choose voice because their hands or eyes are busy: driving, cooking, carrying a child, walking. The design must preserve that freedom. Avoid flows that force the user to look at a screen, tap to confirm, or read a list. If a screen interaction is truly required, make it optional and clearly signaled, and provide a path that stays in voice when possible.

### Keep Prompts Concise And Conversational

Long system prompts lose the user before the action point. Write prompts to be spoken, not read. Favor:

- short, direct sentences;
- natural conversational phrasing;
- the most important information first;
- confirmation only when stakes are high.

Avoid reading out long lists, full URLs, or dense data verbatim. Summarize and offer detail on request.

### Handle Interruption And Barge-In

Users interrupt. They speak while the system is talking, change their mind mid-sentence, or add conditions. Design for barge-in where feasible:

- let the user cut off a long prompt;
- accept revised commands without forcing a restart;
- handle partial and incomplete utterances gracefully.

Punishing interruption by restarting the whole flow breaks the conversational feel.

### Design Audio Feedback And Confirmation Carefully

Without visuals, audio carries confirmation, progress, and error states. Use:

- earcons or subtle sounds for success, failure, and listening states;
- spoken confirmation for completed actions;
- progressive feedback for long operations so the user knows the system is still working.

Avoid silence after a command; users cannot tell whether the system heard them.

### Address Privacy And Social Context

Voice is public by nature. The device may be listening in a shared room, and spoken content can be overheard. Consider:

- what should never be read aloud, such as passwords, sensitive health or financial data;
- when to require explicit confirmation before sharing private information;
- how to handle voice history and recordings responsibly;
- the social discomfort of speaking certain commands in public.

Offer private modes or screen handoff for sensitive tasks.

### Support Multilingual And Dialect Needs

Users switch languages, mix languages, and speak with accents and dialects. A voice interface tuned to one accent or one language fails large audiences. Plan for language switching, code-mixing, and recognition across dialects rather than assuming a single standard speaker.

## Common Traps

### Designing Voice Like A Chatbot Read Aloud

Long text exchanges do not work as speech. Voice needs brevity, structure, and confirmation, not paragraph-length system messages.

### Single Rigid Command Phrases

Forcing users to say exactly one phrase ignores how people naturally speak and makes the interface feel broken.

### Assuming Perfect Recognition

Real environments are noisy and speakers are diverse. Designs without error recovery fail the moment recognition slips.

### Forcing Screen Confirmation

If every voice action requires looking at a screen and tapping, the hands-free value proposition disappears.

### Reading Long Lists Verbatim

Spoken lists exceed memory instantly. Users forget the second item by the time they hear the fifth.

### Silent States After A Command

When the system goes quiet after input, users repeat themselves, shout, or assume it broke. Audio feedback is mandatory.

### Speaking Private Information Aloud

Reading out passwords, balances, or health data in a shared space is a privacy failure, not a convenience.

### Ignoring Interruption

Forcing a full restart when a user interrupts or revises a command breaks the natural flow of speech.

## Self-Check

- [ ] Prompts are short, structured, and written to be spoken, with the most important information first.
- [ ] The interface accepts multiple phrasings for the same intent and offers discoverable example commands.
- [ ] Error recovery distinguishes "didn't hear," "didn't understand," and "can't do that," with a different path for each.
- [ ] Destructive or irreversible actions require explicit confirmation before execution.
- [ ] The flow preserves hands-free and eyes-free use, with screen interaction optional rather than required.
- [ ] Critical information is repeated or persisted to a screen when available, rather than spoken once and lost.
- [ ] Audio feedback confirms listening, success, failure, and progress so users are never left in silence.
- [ ] Barge-in and command revision are supported without forcing a full flow restart.
- [ ] Sensitive information is not read aloud by default, with private modes or screen handoff available.
- [ ] The interface was tested with accents, background noise, interruptions, and multilingual input, not only clean studio conditions.
