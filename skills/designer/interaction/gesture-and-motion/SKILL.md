---
name: gesture_and_motion.md
description: Use when the agent is designing gesture-based interactions, motion and animation in interfaces, transitions, animated feedback, swipe and drag patterns, deciding when motion aids comprehension versus distracts, calibrating motion to context and accessibility, or ensuring gesture and motion design is discoverable, learnable, and respectful of users sensitive to movement.
---

# Gesture And Motion

Gesture and motion are powerful precisely because they operate beneath language: a well-chosen animation makes a state change instantly legible, and a direct gesture makes manipulation feel physical and immediate. But that same power makes gesture and motion dangerous when used carelessly. Motion that decorates rather than communicates adds noise and fatigue; gestures that are invisible to the user might as well not exist; and motion that ignores accessibility can literally make some users sick. The judgment problem is treating motion and gesture as communication first and decoration second, making gestures discoverable, and calibrating movement to the context, the task, and the full range of users. Agents tend to fail by adding motion because it looks impressive, by hiding primary actions behind undiscoverable gestures, and by treating accessibility as an afterthought to a "delightful" animation.

Use this skill when designing any motion, animation, transition, or gesture-based interaction: page transitions, microinteractions, swipe actions, drag-and-drop, animated feedback, loading states, or onboarding animations. The goal is motion and gesture that clarify, respect the user, and degrade gracefully.

## Core Rules

### Make Motion Communicate, Not Decorate

Every animation should answer a question: what just changed, where did it come from, where is the user's attention now, or is the system working? Motion without a communicative purpose is decoration, and decoration accumulates into fatigue.

Use motion to communicate:

- spatial relationships, where an item came from and where it went;
- causality, that this action caused that result;
- continuity, that two states are the same object transformed, not two unrelated things;
- status, that the system received the input and is processing it;
- hierarchy, drawing attention to what changed or what matters next.

If removing an animation would leave the interface equally clear, the animation is not earning its place. Cut it.

### Match Motion Intensity To Context And Stakes

The same animation can feel delightful in a celebration and obnoxious in a tool used a hundred times a day. Calibrate motion to how often the action recurs and what is at stake.

Calibrate by:

- frequent, utility actions: minimal, fast, almost invisible motion, because repetition amplifies annoyance;
- occasional, meaningful actions: moderate motion that marks the moment without slowing the user;
- rare, celebratory moments: richer motion is justified because it will not fatigue;
- high-stakes or error contexts: calm, clear motion, never playful or distracting.

Motion that feels good in a demo often fails in daily use. Judge motion by the hundredth encounter, not the first.

### Ensure Gestures Are Discoverable

A gesture the user cannot find is a feature that does not exist. Touch and motion gestures are inherently invisible, and relying on them as the only path to an action excludes everyone who does not already know the gesture.

Make gestures discoverable:

- provide a visible, tappable affordance for any primary action, with the gesture as an optional shortcut;
- teach gestures in context, at the moment they are relevant, not buried in a manual;
- use consistent platform conventions so prior knowledge transfers;
- offer hints, such as a swipe affordance or a brief tutorial, without nagging;
- never make a gesture the sole way to reach a critical or destructive action.

Shortcuts are valuable, but they must layer on top of accessible, visible controls, not replace them.

### Respect Motion Sensitivity And Accessibility

Motion is not universally benign. Some users experience motion-triggered discomfort, dizziness, nausea, or seizures, and many more find excessive animation fatiguing or distracting. Accessibility is a first-class constraint on motion design, not a setting to add later.

Build in accessibility from the start:

- honor the operating system's reduced-motion preference and provide a calm alternative;
- avoid large-area parallax, autoscroll, and rapid flashing that commonly trigger discomfort;
- ensure no information is conveyed by motion alone, so reduced-motion users lose nothing;
- keep animation duration short and easing gentle for functional motion;
- provide controls to pause, stop, or dismiss looping and ambient animation.

Designing the calm path first is often the cleanest approach; the richer motion then layers on for users who want it.

### Use Easing And Timing To Convey Physicality And Priority

Linear motion feels mechanical and wrong, because almost nothing in human experience moves at a constant rate. Easing and timing are how motion feels natural and how it conveys weight and importance.

Apply timing deliberately:

- use ease-out for elements entering view, so they decelerate and settle;
- use ease-in for elements leaving, so they accelerate away;
- reserve custom and spring easing for cases that benefit from physicality;
- keep functional animations short, commonly 150 to 300 milliseconds, so they inform without delaying;
- reserve longer durations for large or spatial transitions where the eye needs time to track.

Inconsistent timing across an interface feels chaotic. Define a small set of motion tokens and apply them uniformly.

### Design For Interruption And State Change

Users do not wait for animations to finish; they tap, swipe, and change context mid-transition. Motion that cannot be interrupted, or that breaks when interrupted, creates jank and confusion.

Design motion to be robust:

- make animations interruptible, so a new input cancels or redirects cleanly;
- ensure the final state is correct even if the animation is cut short;
- avoid locking the user out of interaction during non-essential motion;
- handle rapid, repeated input without queuing a backlog of animations.

If a user can break the animation by interacting, the design must handle that gracefully rather than punish the input.

### Keep Motion Consistent With A Shared System

Motion, like color and type, should be systematized. Ad hoc per-screen animation produces an interface that feels like it was built by different teams, because it was.

Systematize by:

- defining standard durations, easings, and motion patterns as reusable tokens;
- using the same transition for the same kind of state change everywhere;
- documenting when each motion pattern applies and when it does not;
- reviewing new motion against the system before adding it.

A coherent motion language makes the product feel trustworthy and intentional. A collection of one-off animations makes it feel random.

## Common Traps

### Motion As Decoration

Animation added because it looks impressive, without a communicative purpose, adds fatigue and noise. Every animation must earn its place by clarifying something.

### Hidden Gestures As Primary Actions

Relying on invisible gestures for critical actions excludes users who do not know them. Always provide a visible alternative.

### Ignoring Reduced-Motion Users

Motion that triggers discomfort or conveys information by motion alone excludes sensitive users. Build the calm path first.

### Demo-Friendly But Daily-Hostile Motion

Motion that delights on first view becomes obnoxious on the hundredth. Judge motion by repeated, frequent use.

### Non-Interruptible Animations

Transitions that block input or break when interrupted feel broken. Design for interruption.

### Inconsistent, Ad Hoc Easing

Different timing on every screen makes the interface feel chaotic. Use a shared motion system.

### Flashing, Parallax, And Autoplay Without Controls

Large-area motion and loops commonly cause discomfort and fatigue. Provide controls and avoid them by default.

## Self-Check

- [ ] Every animation communicates a specific change, relationship, or status rather than decorating.
- [ ] Motion intensity is calibrated to frequency and stakes, judged by repeated use, not the first impression.
- [ ] All gesture-based actions have a visible, accessible alternative and are taught in context.
- [ ] A reduced-motion path exists, is honored automatically, and loses no information.
- [ ] No information is conveyed by motion alone, and no large-area flashing or parallax is used without controls.
- [ ] Easing and timing follow a small, documented set of motion tokens applied consistently.
- [ ] Animations are interruptible and reach a correct final state when cut short.
- [ ] Motion is consistent across the product through a shared system, not ad hoc per screen.
- [ ] Looping and ambient animation can be paused, stopped, or dismissed.
- [ ] The motion design was reviewed for accessibility and fatigue, not only for visual appeal.
