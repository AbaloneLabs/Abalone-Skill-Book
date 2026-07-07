---
name: gesture_discoverability.md
description: Use when the agent is designing gesture-based interfaces, hidden interactions, onboarding for gestures, affordance cues, hint animations, or deciding whether a gesture needs a visible alternative so users can find and learn it.
---

# Gesture Discoverability

Gestures are invisible. A button announces itself; a swipe does not. This is the central problem of gesture design: the most elegant interaction is worthless if the user never discovers it exists. Designs that rely on hidden gestures for core tasks systematically lose the majority of users, who assume the feature is absent and never try.

Use this skill before designing or reviewing any interface where gestures carry meaningful actions, including swipe-to-archive, pinch-to-zoom, long-press menus, edge swipes, shake-to-undo, two-finger shortcuts, or custom touch interactions. The goal is to prevent the agent from building a beautiful, efficient interface that only experts can operate because the gestures were never made findable.

## Core Rules

### Decide Whether A Gesture Is Primary, Accelerator, Or Optional

Not every gesture deserves the same discoverability effort. Classify each gesture by its role in the task.

- a primary gesture is the main or only way to accomplish a core task; it must be taught explicitly and reinforced.
- an accelerator gesture is a faster alternative to a visible control; it can be hinted but does not need to gate access.
- an optional gesture adds convenience or delight; it can be documented without onboarding.

The mistake is treating a primary gesture as if it were an accelerator. If the only path to a key action is a hidden gesture, the design has an access problem, not an elegance problem.

### Provide A Visible Path To Every Primary Action

If a gesture is the primary way to do something important, there must also be a visible control, menu item, or button that reaches the same outcome. The gesture exists to speed up the task for experts, not to replace the path for everyone else.

Patterns that preserve access:

- a visible button or icon for the same action;
- a menu or overflow that lists the action;
- a context menu reachable by a visible control, not only by long press;
- a settings toggle or preference that exposes the feature;
- a search or command palette entry.

When no visible path exists, the gesture is effectively a secret.

### Teach Gestures At The Moment Of Relevance

Discoverability is strongest when the hint appears where and when the user needs it, not in a manual they read once and forget. Teach the gesture in context.

Effective in-context teaching:

- a brief coach mark or hint the first time the relevant screen appears;
- a partially revealed action that hints at what a swipe will expose;
- a gentle animation on first load showing the gesture in motion;
- a prompt that appears after the user has done the precursor action several times manually;
- a hint that appears when the user pauses or seems stuck.

Avoid dumping all gestures in a single onboarding wall, which users skip and forget.

### Use Progressive Disclosure Of Gesture Hints

Do not reveal every gesture at once. Overloading the first screen with hints creates noise and trains users to dismiss them. Reveal hints progressively as the user gains familiarity or reaches the relevant context.

Sequencing guidance:

- introduce the most common gesture first;
- defer advanced gestures until the user has mastered basics;
- retire a hint after the user has performed the gesture successfully a few times;
- re-introduce a hint if analytics show the gesture is underused;
- avoid repeating hints the user has explicitly dismissed.

A hint that never goes away becomes wallpaper; a hint that appears once and vanishes is forgotten.

### Reinforce Affordance Through Visual Cues

Even without explicit teaching, the interface can suggest that something is interactive. Visual affordance lowers the barrier to trying a gesture.

Cues that help:

- grab handles that imply drag;
- a subtle peek of a hidden action behind a row that implies swipe;
- a shadow or lift state that implies the object can be moved;
- an icon that mirrors the gesture, such as a pinch or swipe glyph;
- a directional chevron or edge that implies a panel can be pulled;
- motion on first appearance that demonstrates the gesture path.

Static, flat, unlabeled surfaces give no signal that a gesture is possible.

### Measure And Respond To Gesture Usage

Discoverability is an empirical question, not a matter of opinion. If analytics show that a gesture is used by a small fraction of users, the discoverability design is failing regardless of how intuitive it seemed.

Track and act on:

- the percentage of users who ever perform each gesture;
- how long after first use the gesture is discovered;
- whether users fall back to the visible alternative and never switch;
- whether hints are dismissed immediately or heeded;
- whether feature usage rises after a hint is introduced or changed.

If a gesture is undiscoverable in practice, either improve the hint, add a visible path, or demote the gesture to optional.

### Plan For Users Who Never Learn Gestures

Some users will never adopt gestures: users with motor limitations, users with cognitive differences, users on assistive devices, and users who simply prefer explicit controls. The interface must remain fully usable without any gesture.

Ensure:

- every gesture-backed action has a non-gesture equivalent;
- keyboard, switch, voice, or pointer alternatives exist where relevant;
- accessibility tools are not blocked by gesture-only flows;
- the core task loop does not require a gesture at any step.

## Common Traps

### Assuming Users Will Explore

Designers explore interfaces; most users do not. They tap what looks tappable and ignore the rest. Relying on exploration guarantees low gesture adoption.

### The Onboarding Wall

A single screen listing six gestures at launch is read by almost no one and remembered by fewer. Gestures taught out of context do not stick.

### Hints That Never Disappear

A coach mark that shows on every launch becomes visual noise that users learn to dismiss without reading. Hints must retire after learning.

### Secret Power-User Features

When the only way to reach a useful feature is a gesture known to insiders, the feature is effectively missing for the majority. Power features should accelerate, not gatekeep.

### Dismissed Hints With No Return Path

If a user dismisses a gesture hint and later needs the feature, they may never find it again. Provide a way to revisit hints in help or settings.

### Visual Flatness That Removes Affordance

A minimalist interface with no handles, shadows, or cues gives no signal that gestures exist. Flat design and gesture richness are in tension and must be balanced deliberately.

### Treating Discoverability As A One-Time Problem

Users forget gestures they do not use often. Discoverability is ongoing: returning users, infrequent users, and users on new devices all need re-introduction.

## Self-Check

- [ ] Each gesture is classified as primary, accelerator, or optional, and primary gestures receive explicit discoverability treatment.
- [ ] Every primary gesture has a visible, non-gesture alternative that reaches the same action.
- [ ] Gesture hints are taught in context at the moment of relevance, not only in a launch-time onboarding wall.
- [ ] Hints are introduced progressively, retired after successful use, and re-introduced only when analytics warrant.
- [ ] Visual affordances, handles, peeks, shadows, or glyphs suggest that interactive gestures are possible.
- [ ] The interface remains fully usable without any gesture, including for accessibility tools and non-touch input.
- [ ] Analytics track gesture adoption, discovery latency, and fallback usage, and the design responds to low adoption.
- [ ] Dismissed hints can be revisited through help, settings, or a command palette.
- [ ] Advanced and multi-touch gestures are treated as optional enhancements with visible alternatives.
- [ ] The discoverability plan accounts for returning, infrequent, and new-device users, not only first-time users.