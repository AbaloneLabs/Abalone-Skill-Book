---
name: notification_design.md
description: Use when the agent is designing notifications, alerts, banners, toasts, badges, status messages, in-app announcements, notification centers, or reviewing how the product communicates timely information, status changes, and events to users.
---

# Notification Design

A notification is a claim on the user's attention. Every alert, badge, banner, toast, and email is a small interruption that promises the information is worth the cost of looking. When notifications are designed loosely, the product trains users to ignore them, mute them, or uninstall. When they are designed well, they deliver the right information at the right time, in a form the user can act on, and then get out of the way.

Use this skill before designing or reviewing notification systems, alert banners, toast messages, status badges, notification centers, in-app announcements, activity feeds, email digests, or any surface that tells users something happened. The goal is to prevent the agent from treating notifications as a styling problem and ignoring the harder questions: what deserves to interrupt, how urgent it really is, what the user can do about it, and how the system recovers when the user misses or dismisses it.

## Core Rules

### Earn The Interruption Before Designing It

Before choosing a banner, toast, or push, decide whether the event deserves to interrupt at all. Most events do not. Reserve interruption for information that is time-sensitive, actionable, and meaningful to the user's current goal.

Classify every potential notification by urgency:

- Critical and time-sensitive: failure, security event, payment issue, direct message from a person. Interrupt immediately.
- Important but not urgent: a task is ready, a status changed, a reply arrived. Surface without forcing attention.
- Informational: activity, summaries, social signals. Batch, digest, or place in a center. Do not interrupt.
- Noise: events the system can handle without telling the user. Do not notify at all.

If you cannot place an event in one of these buckets, default to less interruption. You can always escalate; you cannot easily un-train a user who learned to ignore you.

### Make The Notification Self-Sufficient

A user should understand a notification without opening the app or reading a help doc. Each notification should answer four questions in a glance:

- What happened?
- What does it mean to me?
- Do I need to do anything?
- When and where can I act?

Weak notifications say "New activity" or "Update available." Strong notifications say "Your invoice for $42 failed because the card expired. Update payment to retry." Specificity is respect for the user's time. Vague copy forces the user to do investigative work the product should have done.

### Match The Channel To The Urgency And Context

Different channels carry different interruption costs. A push notification vibrates a phone in a pocket. An email waits in an inbox. A banner appears while the user is working. A badge lingers until cleared. Choose the channel by matching its cost to the event's urgency.

- Push: critical and time-sensitive, or explicitly requested by the user.
- In-app banner or toast: relevant to the current session and often actionable inline.
- Notification center: important but not interrupting, available on demand.
- Email: asynchronous, summary-oriented, or for users who opted in.
- Badge: persistent indicator that something needs attention, without forcing it.

Do not default everything to push. Push is the most expensive channel because it reaches the user anywhere, including during sleep, meetings, and focus work.

### Provide Action, Not Just Information

A notification that only reports an event often creates a follow-up task: the user must open the app, find the relevant screen, and act. Where possible, let the user act directly from the notification.

Support inline actions when the action is simple, safe, and unambiguous: accept, decline, reply, mark read, snooze. Avoid inline actions for destructive, ambiguous, or multi-step decisions, which need the full context of the app. Always offer a path to the full context for users who want more detail before deciding.

### Design For Dismissal, Snooze, And Recovery

Users will miss, dismiss, or ignore notifications. The system must recover gracefully.

- Dismissal should be easy and non-destructive; the underlying information should still be reachable.
- Snooze or remind-me-later helps users defer without losing the item.
- Missed critical events should be findable in a notification center, not silently vanish.
- Repeated dismissal of the same type is a signal to reduce frequency, not to try harder.

A notification that disappears forever on dismiss, with no record, punishes users who swipe reflexively. Persistence and recoverability matter for anything important.

### Respect Notification Preferences And Quiet Hours

Users have different tolerances. Some want every event; some want almost none. Provide meaningful preference controls and honor them strictly.

- Let users choose channels per category, not just a global on/off.
- Respect device-level quiet hours and focus modes.
- Do not bypass user preferences for marketing or engagement notifications.
- Make it easy to turn a notification type off directly from the notification itself.

A product that overrides user preferences to boost engagement is training users to disable notifications entirely or uninstall.

### Handle Grouping, Deduplication, And Volume

When many events happen at once, raw volume becomes the problem. A burst of twenty similar notifications feels like spam even if each one is individually reasonable.

Use:

- grouping or stacking of related events;
- deduplication of identical or near-identical alerts;
- summarization for high-volume categories;
- rate limiting so a chatty feature cannot flood the user;
- coalescing updates that arrive within a short window.

Volume control is a design responsibility, not just an engineering afterthought.

### Communicate State Clearly In Persistent Surfaces

Badges, banners, and notification centers are persistent surfaces that accumulate state. Keep them accurate.

- badge counts should reflect actionable items, not raw event counts;
- read and unread state should be clear and easy to clear;
- resolved items should disappear or be marked resolved, not linger;
- stale notifications should age out or be archived rather than pile up indefinitely.

A notification center full of outdated, irrelevant items is worse than no center at all because it trains the user to never check it.

## Common Traps

### Notifying For The Product's Benefit, Not The User's

Notifications sent to drive retention, re-engagement, or metrics, rather than to deliver value the user asked for, erode trust fast. Always ask whether the user would want this notification.

### Vague Copy That Forces Investigation

"Something needs your attention" wastes the user's time. If the notification cannot say what happened and what to do, it is not ready to ship.

### Defaulting Everything To Push

Push is the loudest, most invasive channel. Using it for low-urgency events guarantees users will mute the app or ignore all notifications, including the important ones.

### Notifications That Cannot Be Dismissed Or Recovered

Forcing a notification to persist, or making it vanish permanently on dismiss, both remove user agency. Important information must be dismissible but recoverable.

### Ignoring Volume And Burst Patterns

A feature that fires many notifications in a short window feels broken even when each event is valid. Group, summarize, and rate-limit chatty sources.

### Badge Counts That Never Clear

A badge that always shows a number, regardless of whether anything is actionable, trains users to ignore it. Badges should represent real, clearable work.

### Overriding Preferences For Engagement

Bypassing quiet hours or user preferences to send a "we miss you" message is a dark pattern. It may lift a metric briefly and damage the relationship permanently.

## Self-Check

- [ ] Each notification type is classified by urgency, and only time-sensitive, actionable events interrupt the user.
- [ ] Notification copy answers what happened, what it means, whether action is needed, and where to act, without forcing the user to open the app to find out.
- [ ] The channel matches the urgency: push reserved for critical or requested events, banners for session-relevant items, centers and email for less urgent information.
- [ ] Simple, safe actions are available inline, while complex or destructive decisions route to full context.
- [ ] Dismissal, snooze, and recovery are supported, and important events remain findable after dismissal.
- [ ] Users can control channels per category, and quiet hours and focus modes are respected.
- [ ] High-volume sources are grouped, deduplicated, summarized, or rate-limited to prevent flooding.
- [ ] Badge counts, read state, and notification center contents reflect actionable, current items rather than raw or stale event counts.
- [ ] No notification bypasses user preferences to serve engagement or marketing goals.
- [ ] Repeated dismissal of a notification type triggers a reduction in frequency rather than more aggressive sending.
