---
name: interruption_and_attention.md
description: Use when the agent is designing attention-capturing UI, focus modes, do-not-disturb behavior, modal dialogs, confirmations, autoplay, animation, urgent alerts, or reviewing when and how the product should claim, hold, or release the user's attention.
---

# Interruption And Attention

Attention is the scarcest resource the product competes for, and it is also the resource most easily abused. Every modal, alert, animation, sound, badge, and autoplay video is a bid for the user's focus. Some of those bids are necessary: a destructive action needs confirmation, a security event needs immediate notice. Many are not. When a product interrupts carelessly, it trains the user to dismiss, mute, or leave. When it interrupts deliberately, it earns the right to the user's attention at the moments that truly matter.

Use this skill before designing or reviewing modals, confirmations, urgent alerts, focus modes, do-not-disturb behavior, autoplay media, motion and animation, full-screen takeovers, attention-grabbing copy, or any system that decides when to claim, hold, or release the user's attention. The goal is to prevent the agent from reaching for the loudest treatment by default and ignoring the cost of every interruption, the context the user is in, and the cumulative effect of many small claims on focus.

## Core Rules

### Treat Attention As A Finite, Depleting Budget

A user does not have unlimited attention. Every interruption spends from a budget that also covers their actual work, their relationships, and their wellbeing. A product that spends lavishly on low-value interruptions finds itself ignored when it finally has something important to say.

Before adding any attention-capturing element, ask what it costs the user:

- focus lost on their current task;
- time spent recovering context after the interruption;
- emotional cost of surprise, annoyance, or anxiety;
- cumulative fatigue from repeated small interruptions.

Reserve the loudest treatments for the rare moments that justify them. The more you use a modal, the less a modal means.

### Interrupt Proportional To Stakes And Urgency

Match the force of the interruption to the stakes. A destructive, irreversible, or time-critical decision may justify a blocking modal. A routine update does not.

Scale treatments:

- full-screen takeover or blocking modal: only for decisions that must happen now and cannot be deferred;
- modal dialog: for confirmations and decisions that benefit from focused attention;
- banner or toast: for relevant, non-blocking information;
- inline status: for information tied to a specific element;
- ambient indicator: for persistent state the user can check on demand.

Using a blocking treatment for a low-stakes message is the design equivalent of crying wolf. When the real emergency arrives, the user has already learned to dismiss you.

### Respect The User's Current Context And Task

Interruption cost depends on what the user is doing. The same alert is cheap during browsing and expensive during a transaction, a medical entry, a live event, or focused creative work.

Before interrupting, consider:

- is the user mid-task, especially an irreversible or high-stakes one?
- is the user in a flow state where context loss is costly?
- is the user on a shared or observed screen where surprise is embarrassing?
- is the user in a context where sound or motion is inappropriate?

Defer non-critical interruptions when the user is in a sensitive context. A confirmation that pops up mid-payment can cause the user to abandon the flow entirely.

### Make Interruptions Dismissible And Deferrable

An interruption the user cannot escape is a trap. Even critical interruptions should offer a way out: defer, remind later, or proceed with a clear understanding of the consequence.

- blocking modals should have an explicit dismiss or cancel path;
- deferral should preserve the decision for later, not silently drop it;
- repeated deferral is a signal to reconsider whether the interruption is needed;
- forcing the user to act before they can continue should be rare and justified.

If the only way past an interruption is to make a decision the user is not ready for, the design is coercive.

### Avoid Stacking And Compounding Interruptions

One interruption is manageable. Three queued in sequence, or a modal on top of a banner on top of a toast, is chaos. Stacking trains the user to dismiss everything reflexively just to escape.

Coordinate interruptions:

- queue non-critical ones rather than showing simultaneously;
- suppress lower-priority interruptions when a critical one is active;
- coalesce related interruptions into a single decision;
- avoid interrupting to ask permission to interrupt.

The product should feel like it has one voice, not a crowd shouting at once.

### Use Motion And Sound Deliberately, Not Decoratively

Animation and sound are powerful attention tools, which means they are easy to misuse. Motion that signals state change, guides the eye, or confirms an action is useful. Motion that exists to be impressive, or sound that exists to startle, spends attention without returning value.

- motion should be brief, purposeful, and respect reduced-motion preferences;
- sound should be opt-in or reserved for genuinely critical events;
- autoplay video or audio is almost always an unwelcome interruption;
- looping animation creates ambient anxiety and should be avoided for persistent elements.

Decorative motion and sound are attention taxes. They make the product feel louder without making it clearer.

### Design Quiet Modes And Recovery

Users need ways to reduce interruption, and the product needs to recover when interruptions are suppressed.

- support do-not-disturb, focus, and quiet modes at the system and product level;
- decide what, if anything, can break through quiet modes, and make that list small;
- queue suppressed interruptions so they surface when the user is ready;
- make it clear what was held back, so the user can catch up;
- never silently drop a critical event because a quiet mode was active.

Quiet modes are a contract. Breaking them for engagement destroys the contract and the trust.

### Distinguish Helpful Nudges From Manipulation

There is a line between guiding the user toward their goal and manipulating them toward the product's goal. Attention-grabbing techniques, urgency cues, and emotional copy can cross that line quickly.

Ethical attention design asks:

- does this interruption serve the user's stated or evident goal?
- would the user consent to this interruption if asked?
- is the urgency real or manufactured?
- does the treatment pressure the user against their interest?

Fake urgency, countdown timers on non-time-sensitive offers, and guilt-tripping copy are manipulation. They may produce short-term conversion and long-term resentment.

## Common Traps

### Defaulting To Modal For Everything

Modals are easy to build and expensive to experience. Using them for routine information trains dismissal and makes real confirmations meaningless.

### Interrupting Mid-Transaction

An alert or confirmation that appears during a payment, submission, or irreversible action can cause abandonment or errors. Defer non-critical interruptions until the user reaches a safe point.

### Stacked Or Simultaneous Interruptions

Multiple banners, toasts, and modals at once overwhelm the user and force reflexive dismissal. Coordinate and queue.

### Autoplay Media As An Attention Grab

Video or audio that starts without user action is almost always unwelcome, especially on shared, mobile, or low-bandwidth contexts. Make media user-initiated.

### Breaking Quiet Modes For Engagement

Surfacing marketing or low-priority messages during do-not-disturb, or overriding focus modes to boost metrics, breaks the quiet-mode contract and trains users to disable the feature or the app.

### Manufactured Urgency

Countdowns, "last chance" copy, and red alerts on non-urgent offers manipulate attention and erode trust when the user notices the pattern.

### Motion And Sound That Cannot Be Disabled

Persistent animation or sound with no off switch creates anxiety and accessibility problems. Respect reduced-motion and mute preferences.

## Self-Check

- [ ] Each interruption is justified by stakes and urgency, and the force of the treatment matches the importance of the message.
- [ ] The product treats attention as a finite budget, reserving loud treatments for rare, high-value moments.
- [ ] Interruptions consider the user's current context and defer non-critical claims during high-stakes or focused tasks.
- [ ] Every interruption is dismissible or deferrable, and deferral preserves the decision rather than dropping it.
- [ ] Interruptions are coordinated and queued rather than stacked or shown simultaneously.
- [ ] Motion and sound are purposeful, brief, and respect reduced-motion and mute preferences.
- [ ] Quiet modes, focus modes, and do-not-disturb are honored, with only a small, defined set of events allowed to break through.
- [ ] Suppressed interruptions are queued and surfaced later, never silently dropped for critical events.
- [ ] Autoplay media is avoided or strictly user-initiated.
- [ ] No interruption relies on manufactured urgency, fake countdowns, or manipulative copy to capture attention.
