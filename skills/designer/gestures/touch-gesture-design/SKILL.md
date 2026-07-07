---
name: touch_gesture_design.md
description: Use when the agent is designing touch interactions, mobile gestures, tap targets, press-and-hold behaviors, multi-touch, pinch-to-zoom, rotation, gesture conflicts, gesture feedback, or any interface where fingers replace a pointer and the hand obscures the screen.
---

# Touch Gesture Design

Touch design is not mouse design with bigger buttons. A finger is blunt, imprecise, and opaque; the hand covers part of the screen while the user acts; the thumb often works alone; and the device may be moving, tilted, held one-handed, or shared between users. A gesture that feels natural in a design tool can be impossible to discover, easy to trigger by accident, or painful to undo in real use.

Use this skill before designing or reviewing mobile apps, tablets, touch kiosks, games, drawing tools, maps, carousels, photo editors, readers, or any surface where the primary input is touch. The goal is to prevent the agent from choosing gestures for elegance rather than ergonomics, reliability, discoverability, and reversibility.

## Core Rules

### Account For Finger Anatomy And Occlusion

A fingertip is roughly 10mm wide and has no hover state. The contact patch is larger than the visual target the user thinks they are hitting, and the hand, wrist, or phone case can cover content near the touch point.

Design accordingly:

- keep important feedback visible above or away from the contact area, not only under the finger;
- give interactive targets enough size and spacing to tolerate imprecise contact;
- avoid placing critical labels, counters, or confirmation text directly beneath where the thumb rests;
- account for left-handed and right-handed occlusion;
- consider the thumb zone on phones, where the upper corners are hard to reach one-handed.

Do not treat the touch point as a precise pixel coordinate.

### Match Gesture To Intent, Not To Novelty

Every gesture should answer a real intent: select, move, scale, navigate, dismiss, commit, or transform. Choose the gesture that maps to the physical metaphor and the user's expectation, not the gesture that looks impressive.

Useful mappings:

- tap to select or activate;
- long press for secondary or contextual actions;
- drag to move or reorder;
- swipe to navigate, dismiss, or reveal;
- pinch to scale;
- rotate to orient;
- two-finger gestures for compound or destructive operations.

If a gesture has no clear physical or conventional meaning, users will not guess it. Prefer a visible control over a hidden gesture for anything important.

### Make Targets Forgiving And Separable

Touch input is noisy. Users may slip, double-touch, or graze a neighboring control. Targets should be large enough to hit comfortably and far enough apart that a miss does not trigger a destructive neighbor.

Review:

- minimum target size for the platform and the audience;
- spacing between adjacent actions, especially delete next to edit;
- whether a small visual control has a larger invisible hit area;
- whether scrollable rows with swipe actions collide with horizontal paging;
- whether edge gestures conflict with system swipes.

Destructive and irreversible actions deserve extra separation, confirmation, or a larger margin of safety.

### Provide Immediate And Localized Feedback

Touch has no hover, so the user needs confirmation that the system received the input. Feedback should appear within tens of milliseconds and originate at or near the touch point.

Check:

- pressed, selected, and dragging states are visible;
- feedback follows the finger during drag, not only on release;
- haptics, where supported, reinforce the gesture without becoming noise;
- the object being manipulated stays under visual control;
- ambiguous states, such as reorder versus delete, are disambiguated by feedback before commitment.

Delayed or distant feedback makes gestures feel broken.

### Design For Accidental And Interrupted Gestures

Real touch sessions include interruptions: notifications, sudden movements, partial contacts, and children. A gesture system should fail safely.

Plan for:

- canceling a drag or pinch by moving back or lifting extra fingers;
- recognizing incomplete or abandoned gestures without committing;
- preventing a brush of the screen from deleting or sending;
- distinguishing a tap from the start of a drag or scroll;
- handling multi-touch where an unintended second finger lands.

If a gesture can only succeed when performed perfectly, it is too fragile.

### Respect Platform Conventions And System Gestures

Users carry expectations from the operating system. Back is often an edge swipe; the home indicator occupies the bottom; notifications pull from the top; text selection uses long press and handles. Custom gestures that override or shadow these create confusion.

Before introducing a gesture, confirm it does not collide with:

- system back, home, or app switcher gestures;
- notification and control center edges;
- text selection, copy, and paste;
- accessibility shortcuts and assistive touch;
- browser or webview chrome.

When a collision is unavoidable, pick a different gesture or provide a visible alternative.

### Keep Gestures Reversible Or Confirmable

Gestures are fast and often performed without full attention. A swipe that archives, a drag that deletes, or a pinch that resets should be recoverable.

Strategies:

- undo and redo for destructive gestures;
- confirmation for irreversible consequences;
- a brief window to cancel before commitment;
- clear feedback about what just happened, not just an animation;
- preservation of the original position or state so the user can return.

A gesture that silently destroys data is a design failure.

## Common Traps

### Designing For The Designer's Hand

A gesture that is easy in the designer's grip may be unreachable for a one-handed user, a user with a large device, or a user with limited dexterity. Test reach and comfort, not only correctness.

### Hidden Gestures As Primary Interaction

Power users may learn a swipe-to-archive shortcut, but new users will not discover it. If a gesture is the only path to an action, most users will never find it.

### Conflicting Gestures On The Same Axis

Horizontal swipe used for both paging and row actions forces the system to guess intent. Ambiguous axis conflicts produce false triggers and frustration.

### Tiny Visual Targets With Tiny Hit Areas

An elegant small icon button that is also a small touch target fails in motion. Visual size and hit size are separate decisions; do not let them collapse into one.

### Feedback That Disappears Under The Thumb

A confirmation toast that appears exactly where the thumb was pressing is invisible to the user. Place consequential feedback where the hand is not.

### Treating Multi-Touch As Obvious

Pinch, rotate, and two-finger taps are not intuitive to everyone. They are powerful for experts but should never be the sole way to reach a core function.

### Ignoring Interruption And Cancellation

A drag that commits on any finger-lift, even after the user changed their mind, punishes hesitation. Gestures need a safe cancel path.

## Self-Check

- [ ] Interactive targets are large enough and spaced enough to tolerate imprecise touch, with adequate margins between destructive and safe neighbors.
- [ ] Important feedback is not hidden under the finger or hand and appears within a short time of contact.
- [ ] Each gesture maps to a clear intent and physical metaphor rather than novelty, and visible controls exist for core actions.
- [ ] Gestures can be canceled, interrupted, or undone, and destructive gestures are confirmed or recoverable.
- [ ] Custom gestures do not collide with system back, home, notification, selection, or accessibility gestures.
- [ ] Drag, pinch, rotate, and reorder states show continuous feedback that follows the finger.
- [ ] Reach and one-handed use were considered for the relevant device sizes and thumb zones.
- [ ] Multi-touch and advanced gestures are optional enhancements, not the only path to key features.
- [ ] Scroll, swipe, paging, and row-action gestures on the same surface do not ambiguously compete.
- [ ] The design was considered under realistic conditions: moving hand, partial contact, interruption, and varying grip.