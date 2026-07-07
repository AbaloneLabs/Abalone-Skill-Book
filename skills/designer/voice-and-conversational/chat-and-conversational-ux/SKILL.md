---
name: chat_and_conversational_ux.md
description: Use when the agent is designing a chat interface, conversational UI, messaging layout, AI assistant chat, bot conversation flow, message threading, typing indicators, or the UX of back-and-forth text conversation including history, context, and recovery from misunderstanding.
---

# Chat And Conversational UX

Chat interfaces look simple, a stream of messages in bubbles, but the design decisions underneath are dense. A chat product must manage conversation history, context window, turn-taking, latency, error recovery, trust in AI responses, and the boundary between structured data and free conversation. A chat UI that merely stacks bubbles often hides lost context, misleading AI answers, broken threading, and interactions that feel dead because they lack the signals of real conversation.

Use this skill before designing chat products, AI assistant conversations, messaging layouts, bot flows, or any back-and-forth text interface. The goal is to prevent the agent from treating chat as a list of messages and ignoring the conversational signals, context handling, trust, and recovery that determine whether the experience feels alive and reliable.

## Core Rules

### Design The Shape Of The Conversation, Not Just The Bubbles

Before styling messages, define the conversation model:

- is it one-on-one, group, human-to-AI, or human-to-human?
- are turns strictly alternating or can they overlap?
- is the conversation linear, threaded, or branched?
- how long should history persist and how far back is useful?

The shape determines layout, scrolling, threading controls, and how context is preserved. A linear human-to-AI chat and a threaded group discussion need fundamentally different structures.

### Make Turn-Taking And System State Visible

Conversational interfaces fail when users cannot tell what the system is doing. Provide clear signals for:

- the system is listening or waiting for input;
- the system is processing or thinking;
- the system is typing or streaming a response;
- the system has finished and awaits the next turn.

Typing indicators, streaming tokens, progress states, and clear turn boundaries prevent the user from re-sending, interrupting prematurely, or assuming the system froze. Latency is unavoidable; hiding it is not.

### Handle Context And Memory Honestly

AI chat products carry a context window. Users do not see its limits, but they feel them when the system forgets earlier instructions or contradicts itself. Design context handling transparently:

- decide what the assistant remembers across a session and across sessions;
- surface when context has been lost or condensed;
- let users reference, pin, or reset context deliberately;
- avoid implying perfect memory when the system is bounded.

Silent context loss erodes trust faster than almost any other flaw.

### Design For Streaming And Partial Responses

Modern chat responses often stream token by token. This changes the UX:

- show partial output early to reduce perceived latency;
- handle mid-response interruption and regeneration;
- make clear when a response is still generating versus complete;
- preserve scroll position so new tokens do not yank the view.

Streaming without these controls feels chaotic; with them, it feels responsive.

### Build Recovery From Misunderstanding Into Every Flow

Conversational systems misunderstand. The design must let users recover without restarting:

- allow editing and resending a previous message;
- support "regenerate" or "try again" for unsatisfactory AI responses;
- let users correct or rephrase mid-conversation;
- provide explicit undo for actions the assistant took on the user's behalf.

Punishing mistakes by forcing a fresh conversation breaks the conversational contract.

### Make AI Responses Trustworthy And Verifiable

When the chat involves AI, trust is a design problem. Users cannot evaluate claims they cannot verify. Provide:

- citations or source links for factual claims;
- confidence or uncertainty signals where appropriate;
- clear distinction between the assistant's synthesis and quoted source material;
- a way to flag or correct wrong answers.

Overconfident AI answers without verification paths turn chat into a reliability hazard.

### Separate Free Conversation From Structured Actions

Chat blends conversation with action: booking, purchasing, editing, deleting. Mixing them carelessly causes destructive mistakes. Distinguish:

- purely informational turns, which need no confirmation;
- state-changing actions, which need clear intent and often confirmation;
- irreversible actions, which need explicit safeguards.

A chat that can delete a file or send money based on an ambiguous phrase is dangerous. Confirm high-stakes actions and show what will change before it changes.

### Preserve And Surface Conversation History

History is part of the product. Users return to find past answers, continue old threads, or copy earlier output. Design:

- searchable, browsable history;
- clear titles or summaries for past conversations;
- the ability to resume, branch, or export;
- retention rules that respect privacy.

History that disappears or cannot be searched wastes the value of the conversation.

### Support Rich Content Without Breaking The Flow

Conversations include code, tables, images, links, and interactive cards. These should integrate without disrupting the chat rhythm:

- collapse large content by default with expand options;
- keep the primary message readable inline;
- allow copy, export, and interaction with rich blocks;
- avoid letting one large table or image dominate the view.

### Design For Errors, Empty States, And Limits

Chat products hit rate limits, network errors, empty history, unsupported requests, and safety refusals. Each needs a clear, non-alarming message and a path forward, not a dead end.

## Common Traps

### Stacking Bubbles Without Conversation Signals

Messages without typing indicators, turn boundaries, or processing states feel dead and make latency unbearable.

### Silent Context Loss

When the AI forgets earlier turns without signaling, users lose trust and the conversation becomes incoherent.

### Ambiguous Destructive Actions

Letting an assistant delete, send, or purchase based on an unclear phrase causes real harm. High-stakes actions need confirmation.

### Overconfident AI Without Verification

Confident answers with no citations or uncertainty signals mislead users who have no way to check.

### Forcing A Restart On Every Misunderstanding

If users must begin a new conversation after any error, the product punishes the exact situations where it should help most.

### Streaming That Hijacks Scroll

Uncontrolled streaming output yanks the viewport and makes reading impossible. Scroll stability must be designed, not assumed.

### Treating History As Disposable

Unsearchable, untitled, or vanishing history throws away the long-term value of conversation.

### Mixing Information And Action Without Boundaries

When every message can change state, users cannot tell which turns are safe to explore and which carry consequences.

## Self-Check

- [ ] The conversation model is defined: participants, turn structure, threading, and history persistence.
- [ ] Listening, processing, typing, streaming, and completion states are all visibly signaled.
- [ ] Context limits are handled honestly, with surfacing or reset options when memory is bounded or lost.
- [ ] Streaming responses show partial output early while preserving scroll position and allowing interruption.
- [ ] Users can edit, resend, regenerate, and correct without restarting the conversation.
- [ ] AI responses include citations, uncertainty signals, or verification paths where claims are made.
- [ ] State-changing and irreversible actions require explicit confirmation distinct from informational turns.
- [ ] Conversation history is searchable, titled, resumable, and governed by clear retention and privacy rules.
- [ ] Rich content integrates without dominating the view and supports copy, export, and interaction.
- [ ] Errors, rate limits, empty states, and safety refusals each have a clear message and a path forward.
