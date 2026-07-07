---
name: push_and_in_app_messaging.md
description: Use when the agent is designing push notifications, in-app messages, banners, toasts, interstitials, message centers, deep links, or reviewing how messages reach users across channels, devices, and session states.
---

# Push And In-App Messaging

Push and in-app messages are different animals that are often designed as if they were the same. Push reaches a user who is not in your product, on a device they carry everywhere, often at moments they did not choose. In-app messaging reaches a user who is already engaged, in a context you partially control, and who can act immediately. Treating them interchangeably produces messages that interrupt at the wrong time, lose context on tap, or fail to reach the user at all. Each channel has its own permission model, delivery guarantees, and cost to the user.

Use this skill before designing or reviewing push notification campaigns, in-app banners and interstitials, message centers, deep-linking behavior, cross-device messaging, permission prompts, or any system that delivers messages to users across channels and session states. The goal is to prevent the agent from writing copy and picking a color while ignoring delivery, permission, context preservation, and the very different social contracts of each channel.

## Core Rules

### Respect The Different Social Contracts Of Each Channel

Each channel implies a promise to the user, and breaking that promise damages trust.

- Push says: "This was important enough to reach you wherever you are." It is the most invasive channel and must be earned.
- In-app banner says: "This is relevant to what you are doing right now." It should relate to the current session.
- Interstitial or modal says: "Stop and decide this before continuing." It blocks flow and must be used sparingly.
- Message center says: "Here is everything you might have missed, when you want it." It is voluntary and non-interrupting.
- Email says: "Here is something you can read on your own time." It is asynchronous and opt-in sensitive.

Match the message to the channel's contract. A marketing message sent as push breaks the push contract. A critical alert buried in email breaks the urgency contract.

### Earn Push Permission Before Asking For It

Push permission is a one-time, fragile grant. Once denied, recovering it is hard. Do not ask on first launch, on a cold screen, or as a generic system prompt.

Before the system permission prompt:

- explain what the user will get and why it matters, in your own branded pre-prompt;
- ask after the user has experienced value, not before;
- tie the request to a clear context, such as completing an order or starting a task;
- make it easy to enable later from settings if the user declines now.

A pre-prompt that is skipped, or a system prompt fired at the wrong moment, can permanently lock the user out of push. Treat the permission moment as a designed flow, not a default dialog.

### Preserve Context Across The Tap

When a user taps a push or in-app message, they expect to land exactly where the message pointed. A tap that opens the app home screen, or worse a login screen, breaks the promise of the notification.

Deep links must:

- route to the specific item, conversation, order, or screen referenced;
- handle the case where the item no longer exists, with a clear explanation;
- preserve any parameters needed to reconstruct context;
- work across cold start, warm start, and already-open sessions;
- degrade gracefully on web, mobile, and tablet where capabilities differ.

Context loss is the most common failure of push. The message was perfect, the tap was eager, and the user lands somewhere useless.

### Design For Delivery Failure And Latency

Messages do not always arrive, arrive on time, or arrive on every device. Design for the reality of delivery.

- do not assume a push was seen; critical events need a fallback channel;
- account for devices offline, in airplane mode, or with notifications muted;
- handle duplicate delivery gracefully, since networks and retry logic can double-send;
- timestamp messages so stale ones are recognizable when they arrive late;
- provide an in-product record so users can catch up on what they missed.

A system that treats "sent" as "received and read" will mislead itself and its users.

### Match Lifecycle And Session State

In-app messages behave differently depending on session state. A modal shown on every app open becomes harassment. A banner shown once and never again may be missed.

Decide for each message:

- how many times it should appear;
- whether it repeats per session, per day, per week, or ever;
- whether it appears on cold start, warm start, or specific screens;
- whether it respects prior dismissal or conversion;
- whether it pauses when the user is mid-task.

Aggressive in-app messaging during onboarding can teach users to dismiss everything reflexively. Pace messages and respect dismissal.

### Coordinate Across Channels Without Spamming

The same event can trigger push, email, in-app banner, and SMS. Sending all of them for one event feels like spam even if each channel is individually reasonable.

Coordinate:

- choose the primary channel for each event;
- suppress or delay secondary channels if the primary was seen;
- bundle related messages across channels into digests where possible;
- let users pick their preferred channel per category;
- avoid sending the same content on a short delay across multiple channels.

Cross-channel coordination is where most messaging systems fail the user. The product feels coherent to the team that built each channel separately and incoherent to the user who receives all of them.

### Make Opt-Out And Frequency Control Honest

Users must be able to control message volume, and the controls must actually work. A preference screen that looks comprehensive but silently ignores choices is worse than no controls.

- provide per-category and per-channel controls;
- honor unsubscribe and mute immediately;
- offer a global quiet or pause without requiring account deletion;
- make the path to controls reachable from the message itself;
- avoid forcing users to dig through nested settings to stop a specific message type.

If the only way to stop a message is to uninstall, the messaging design has failed.

### Handle Sensitive Content Across Lock Screens And Previews

Push previews appear on lock screens, in notification stacks, and in carplay, where bystanders can read them. Design for the possibility that the content is visible to someone other than the user.

- avoid exposing personal, financial, health, or account details in previews by default;
- let users choose whether to show previews;
- use generic copy for sensitive categories when previews are on;
- consider that the device may be shared or observed.

A notification that leaks a private message or account balance on a lock screen is a privacy defect, not a styling choice.

## Common Traps

### Asking For Push Permission Too Early

Prompting on first launch, before the user understands the value, produces denials that are nearly impossible to reverse. Earn the ask first.

### Broken Or Generic Deep Links

A tap that lands on the home screen or a login wall wastes the user's intent and trains them to ignore future notifications. Deep links must resolve to the specific referenced item.

### Treating Sent As Delivered As Read

Assuming a message reached and was understood by the user leads to false confidence and missing fallbacks. Delivery is probabilistic; design accordingly.

### Cross-Channel Duplication

Sending the same event through push, email, SMS, and in-app banner simultaneously overwhelms the user and feels like spam, even when each channel is reasonable alone.

### Interstitial Overuse

Modals and interstitials that block the user's flow for low-value messages train dismissal and resentment. Reserve blocking surfaces for decisions that genuinely must happen now.

### Ignored Or Fake Preferences

A preferences screen that does not actually suppress the chosen categories, or that buries the real controls, breaks trust. Controls must work and be reachable.

### Leaking Sensitive Content In Previews

Lock screen previews that show message contents, balances, or health data expose users to bystanders. Default sensitive categories to generic previews.

## Self-Check

- [ ] Each message is matched to a channel whose social contract fits its urgency and content.
- [ ] Push permission is requested in context, after value, with a branded pre-prompt, and recoverable if declined.
- [ ] Tapping a message deep-links to the specific referenced item and handles missing, deleted, or stale items gracefully.
- [ ] Delivery failure, latency, duplicates, and offline states are handled with fallbacks and an in-product record.
- [ ] In-app message lifecycle is defined: frequency, repetition, session state, and dismissal behavior are explicit.
- [ ] Cross-channel coordination prevents the same event from spamming the user through every channel at once.
- [ ] Opt-out, mute, and frequency controls are honest, immediate, and reachable from the message itself.
- [ ] Sensitive content is protected on lock screens and previews, with generic copy for personal, financial, or health categories.
- [ ] Messages are timestamped and recognizable as stale when delivered late.
- [ ] The system never assumes a sent message was received and read without confirmation.
