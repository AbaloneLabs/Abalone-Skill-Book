---
name: watch_interaction_patterns.md
description: Use when the agent is designing interactions for smartwatches, including tap, swipe, scroll, digital crown or bezel rotation, press-and-hold, gesture, voice, and the patterns users rely on to navigate, select, and act on a wrist-worn device.
---

# Watch Interaction Patterns

Smartwatch interaction is constrained in almost every dimension. The screen is tiny, the touch target is the user's own thumb operating at an awkward angle, the input vocabulary is small, and the user is almost always doing something else at the same time. There is no room for the rich, multi-step, exploratory interaction that phones allow. Every pattern must be obvious, reliable, and fast, because the cost of confusion is not a moment of annoyance, it is abandonment of the device for the phone in the user's other pocket.

The judgment problem is that watch interactions look simple but are easy to get wrong. A swipe that conflicts with the system gesture. A scroll that demands precision the thumb cannot deliver. A tap target sized for a stylus. A flow that requires five taps to do what should take one. These failures are invisible in a clean demo and constant in real use. The agent's job is to choose and combine the small set of watch input primitives deliberately, so that the most common actions are effortless and the rare ones are at least discoverable.

Use this skill before designing or reviewing smartwatch apps, complications, notification actions, workout flows, quick replies, or any interaction on a wrist-worn device. The goal is to prevent the agent from shipping watch interactions that are imprecise, slow, conflicting with system gestures, or dependent on input the wrist cannot reliably provide.

## Core Rules

### Choose The Right Primitive For Each Action

Watches offer a small set of input primitives, and each has a distinct precision, speed, and fatigue profile. Matching the primitive to the action is the core of watch interaction design.

Tap:

- fast and intuitive for discrete selection;
- unreliable for tiny or densely packed targets;
- best for large, clearly separated choices.

Swipe:

- good for navigation between views and dismissing items;
- conflicts easily with system edge gestures;
- poor for precise positioning.

Scroll, via crown, bezel, or touch:

- good for moving through lists without covering content;
- crown and bezel keep fingers off the screen, improving legibility;
- touch scroll obscures content and risks accidental taps.

Press-and-hold or force press:

- useful for contextual and secondary actions;
- low discoverability; never the only path to a function;
- should be reserved, as overuse hides features.

Voice:

- excellent for short commands, dictation, and search;
- fails in noise, privacy-sensitive settings, or when the user cannot speak;
- should complement, not replace, touch.

Hardware button or crown:

- reliable, always-available, and fast;
- excellent for primary actions like back, home, or scroll;
- limited in number, so must be assigned to the most common needs.

Map each action to the primitive that suits it, and avoid forcing one primitive onto every interaction.

### Make The Most Common Action Effortless

Watch interactions succeed or fail on the most common action. If the thing the user does dozens of times a day takes three taps and a scroll, the device loses to the phone. Find the dominant action for each surface and make it a single, obvious gesture.

Make common actions effortless by:

- binding the primary action to a single tap, swipe, or hardware control;
- placing it where the thumb naturally reaches;
- making it large, forgiving, and unambiguous;
- ensuring it works in motion and with imprecise contact;
- avoiding confirmation steps for safe, frequent actions.

Reserve multi-step flows for genuinely rare or consequential actions. The cost-benefit of each tap is different on a watch than on a phone.

### Avoid Conflicts With System Gestures

Watch operating systems reserve certain gestures, usually edge swipes and hardware button presses, for system functions like back, notifications, or control center. App interactions that try to use the same gestures create ambiguity and frustration.

Avoid conflicts by:

- not relying on edge swipes for in-app navigation;
- not redefining hardware button behavior the system already owns;
- using vertical scrolls and taps that do not collide with horizontal system gestures;
- testing on the actual platform, since reserved gestures differ;
- providing alternate paths when a desired gesture is claimed by the system.

A gesture that works in your prototype but fights the operating system will feel broken to real users. Respect the platform's gesture vocabulary.

### Size Targets For The Thumb, Not The Pixel

Watch touch targets are operated by the user's thumb on their own wrist, at an angle, often in motion. Standard mobile target sizes are too small. Targets must be large, forgiving, and well separated.

Design targets by:

- making tap targets generous, well above typical mobile minimums;
- increasing spacing between adjacent targets to prevent mis-taps;
- avoiding tiny targets at screen edges where the thumb cannot reach;
- separating destructive actions from safe ones with clear space;
- supporting the dominant hand and wrist orientation.

Precision is lower on a watch than on a phone in every condition. Design for the worst case, not the demo case.

### Keep Flows Short And Resumable

Watch sessions are measured in seconds. A flow that requires many sequential steps, careful attention, or unbroken focus will be abandoned or interrupted. Keep flows short and make them survive interruption.

Keep flows robust by:

- limiting common flows to one or two steps;
- avoiding text entry; use quick replies, voice, or defer to phone;
- preserving state so an interrupted flow resumes where it left off;
- allowing the user to back out without losing progress;
- never trapping the user in a modal they cannot escape.

If a flow genuinely needs depth, hand it to the phone. The watch should handle the short version and defer the long one.

### Provide Reliable Back And Escape Paths

Getting lost on a watch is worse than on a phone because there is less screen to orient by and less patience to explore. The user must always be able to get back, cancel, and escape.

Ensure escape by:

- binding a reliable back action to a hardware button or consistent gesture;
- making cancel and dismiss obvious and large;
- never removing the path to the home or watch face;
- avoiding deep navigation stacks that hide the exit;
- handling the case where a flow is interrupted by a notification or system event.

A user who cannot quickly return to where they were will stop using the app. The escape path is a primary feature, not a detail.

### Use Haptics To Confirm Without Looking

Watches sit against the skin, which makes haptics an unusually strong feedback channel. Well-designed haptics let the user confirm an action without looking at the screen, which is exactly what a glanceable, in-motion device needs.

Use haptics deliberately by:

- confirming successful taps, selections, and completions;
- differentiating success, failure, and threshold events with distinct patterns;
- reserving strong patterns for genuinely important moments;
- not overusing haptics, which trains the user to ignore them;
- pairing haptics with subtle visual feedback so the channel is redundant.

Haptics are not decoration. They are a way to move confirmation off the screen and onto the body, freeing the eyes.

### Design For One-Handed And Hands-Busy Use

Watches are used one-handed, by definition, and often while the hand is busy: holding a leash, a pole, a child, a drink. Interactions must work when only a thumb is available, and sometimes when no hand is free at all.

Account for hands-busy use by:

- ensuring every common action is doable with one thumb;
- providing voice or gesture alternatives for hands-free moments;
- not requiring two-finger or multi-touch gestures for common tasks;
- supporting quick, single-action entry points from complications or notifications;
- considering workouts and activities where the hands are occupied.

If an interaction requires two hands or careful two-finger gestures, it will not be used during the activities where wearables matter most.

## Common Traps

### Conflicting With System Edge Gestures

In-app swipes that fight the operating system's reserved gestures feel broken. Use the platform's vocabulary and avoid claimed edges.

### Targets Sized For A Phone

Standard mobile tap sizes fail on a wrist operated by a thumb at an angle. Make targets generous and well separated.

### Multi-Step Flows For Common Actions

If the dominant action takes many taps, the user reaches for the phone. Make the common action a single gesture.

### Hidden Features Behind Press-And-Hold

Press-and-hold and force press have low discoverability. Never make a needed function reachable only through them.

### Text Entry On The Wrist

Typing on a watch is slow and error-prone. Use quick replies, voice, or defer to the phone.

### Overusing Haptics

Constant buzzing trains users to ignore haptics and disables the channel for the moments that matter. Reserve strong patterns for real significance.

### No Reliable Escape Path

Deep stacks or modal traps with no clear back action leave users lost and frustrated. The escape path must always be obvious.

## Self-Check

- [ ] Each action is mapped to the input primitive that best fits its precision, speed, and fatigue profile, with alternates available.
- [ ] The most common action on each surface is a single, obvious, forgiving gesture reachable by the thumb.
- [ ] No in-app interaction conflicts with system-reserved gestures, edge swipes, or hardware button behavior.
- [ ] Tap targets are generous, well separated, reachable at natural wrist angles, and sized for imprecise thumb contact.
- [ ] Common flows are one or two steps, preserve state across interruption, and never trap the user.
- [ ] A reliable back, cancel, and escape path is always available and never hidden in deep navigation.
- [ ] Text entry is avoided; quick replies, voice, or phone handoff handle input instead.
- [ ] Haptics confirm actions, differentiate outcomes, are reserved for important moments, and pair with subtle visual feedback.
- [ ] Every common action works one-handed, and hands-free alternatives exist for moments when the hands are busy.
- [ ] Press-and-hold and low-discoverability inputs are never the only path to a needed function.
