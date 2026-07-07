---
name: gesture_design.md
description: Use when the agent is choosing gestures as interaction mechanisms, mapping gestures to actions, designing gesture ergonomics and thumb zones, building a coherent gesture vocabulary, handling gesture conflicts and ambiguity, deciding when a gesture needs an accessible alternative, or ensuring gestures work across devices, orientations, and input methods.
---

# Gesture Design

Gesture design is the practice of choosing body movements (taps, swipes, pinches, long-presses, drags) as the mechanism for interacting with a product. It looks like picking natural motions, but it is really a system of decisions about what each gesture means, how it maps to actions, whether the human body can perform it comfortably, and how the vocabulary holds together. Agents tend to treat gestures as obvious, map them by intuition, or add gestures without considering the users who cannot perform them. The harm is gestures that conflict, that strain the hand, that users never discover, or that exclude anyone who cannot make the intended motion.

Use this skill before assigning gestures to actions or building a gesture vocabulary. The goal is to prevent the agent from choosing gestures that fight each other or the body, from mapping motions to meanings arbitrarily, or from building gesture-based interactions that lock out users who need alternatives.

## Core Rules

### Map Gestures To Actions By Natural Metaphor And Frequency

A gesture's meaning should feel natural, and the mapping should respect how often the action is used. A swipe that mimics pushing something away maps well to dismiss; a pinch that mimics compression maps well to zoom. Mappings that ignore metaphor force users to memorize arbitrary pairings, and assigning demanding gestures to frequent actions causes fatigue.

Map by metaphor and frequency:

- choose gestures whose motion metaphorically suggests the action (swipe to advance, drag to move, pinch to scale);
- reserve simple, low-effort gestures (tap) for the most frequent actions;
- assign more demanding gestures (multi-finger, long-press) to infrequent or power actions;
- avoid mapping a gesture to an action whose meaning conflicts with the motion's metaphor.

A gesture mapped against its metaphor (a swipe-right to delete, when swipe-right conventionally means advance) confuses users every time. Metaphor is the cheapest teacher.

### Design For The Hand And The Body

Gestures are physical acts, and the body has limits. A gesture that requires reaching the top of a large phone screen with the thumb, or twisting the wrist unnaturally, is uncomfortable and causes strain over time. Ergonomics is not a nicety; it determines whether a gesture is usable at all, especially one-handed.

Design ergonomically:

- place gesture-active zones within comfortable reach, especially the thumb zone for one-handed phone use;
- avoid gestures that require holding the device awkwardly or using two hands when one should suffice;
- consider the fatigue of repeated gestures; demanding motions repeated often cause strain;
- account for different hand sizes and motor abilities, not just the designer's hand.

A gesture that is technically possible but physically awkward will be avoided by users, defeating its purpose. The body is the constraint, not the screen.

### Build A Coherent Gesture Vocabulary

Gestures do not exist in isolation. A product has a vocabulary of gestures, and if each is designed separately, the vocabulary conflicts: the same motion means different things in different places, or similar actions use different gestures. A coherent vocabulary uses each gesture consistently and ensures the set covers the product's needs without collision.

Build the vocabulary coherently:

- assign each gesture a consistent meaning across the product (swipe always dismisses, or always navigates, not both);
- ensure the set of gestures covers the needed actions without forcing awkward mappings;
- avoid overloading a single gesture with context-dependent meanings that users cannot predict;
- document the vocabulary so new features inherit it rather than inventing conflicting gestures.

A product where swipe-left means delete in one list and archive in another trains users to distrust gestures. Consistency is what makes a vocabulary learnable.

### Resolve Gesture Conflicts Deliberately

Gestures occupy the same input space and often conflict. A horizontal swipe might be intended for navigation but also for dismissing an item in a list. A long-press might trigger a menu but also be the start of a drag. Unresolved conflicts produce ambiguity where the system guesses wrong about the user's intent.

Resolve conflicts explicitly:

- identify where intended gestures overlap in motion or trigger zone;
- disambiguate by direction, distance threshold, or timing (a tap versus a long-press versus a drag);
- choose a primary intent for ambiguous zones and provide an alternative for the secondary;
- test the disambiguation with real users, since conflicts that look resolved in theory often fail in practice.

A gesture conflict that the designer did not notice will be discovered by users as frustration. Naming and resolving conflicts before release prevents the most common gesture failures.

### Ensure Gestures Have Accessible Alternatives

Gestures are physical motions, and not all users can perform them. Users with motor impairments may not be able to pinch or swipe; users of assistive technology may not use touch at all; users in contexts where touch is impractical (wet hands, gloves) need alternatives. A gesture that is the only way to perform an action excludes these users.

Provide alternatives:

- every gesture-based action must have a non-gesture alternative (a button, a menu item, a keyboard equivalent);
- do not rely on gestures as the sole path to any functionality, especially critical actions;
- ensure the alternative is as discoverable and usable as the gesture, not buried;
- test that the product is fully usable without gestures at all.

A gesture-only interaction is an accessibility failure, regardless of how elegant the gesture is. Alternatives are not optional; they are the baseline of inclusive gesture design.

### Account For Discoverability

Gestures are invisible. Unlike a button, a swipe does not announce itself, and users will not try motions they do not know exist. A gesture-based interaction that users never discover is functionally absent, no matter how well designed. Discoverability must be designed, not assumed.

Design for discoverability:

- provide visible affordances or hints for gestures, especially for core actions;
- use onboarding to teach essential gestures, but do not rely on users remembering;
- offer progressive disclosure: show the gesture hint until the user has used it, then fade it;
- ensure that the non-gesture alternative is visible, so users who miss the gesture still find the action.

Relying on users to guess gestures works for the few who try and the fewer who guess right. For everyone else, the feature is invisible. Discoverability is what makes gestures serve all users, not just the curious.

### Handle Platform Conventions And Expectations

Each platform has established gesture conventions: swipe-to-go-back on iOS, long-press for context menus, pinch-to-zoom universally. Violating these conventions confuses users who carry expectations from other apps. Designing gestures in ignorance of platform norms produces interactions that feel wrong even when they are internally consistent.

Respect platform conventions:

- learn the gesture conventions of each target platform and follow them unless there is strong reason not to;
- avoid repurposing a platform-standard gesture for a different meaning (do not make swipe-back do something else);
- when innovating beyond conventions, ensure the innovation is teachable and worth the learning cost;
- test with users familiar with the platform, whose expectations will reveal convention violations.

A gesture that fights platform conventions feels broken to users before they understand why. Conventions are the default; deviation requires justification.

### Test Gestures Across Devices, Orientations, And Contexts

A gesture that works on a phone in portrait may fail in landscape, on a tablet's larger surface, or with a stylus. A gesture designed in a quiet office may fail on a moving train where the hand is unsteady. Gestures are context-sensitive in ways that clicks are not, and testing only in ideal conditions hides failures.

Test broadly:

- test gestures on all target devices and screen sizes, including tablets and foldables;
- test in both orientations, since reach and motion change;
- test with different input methods (finger, stylus, assistive pointer);
- test in real contexts (movement, distraction, one-handed use) where gestures are actually performed.

A gesture validated only on a phone in a designer's hand is unvalidated. The contexts of real use are where gestures succeed or fail.

## Common Traps

### Mapping Gestures Against Metaphor

A swipe-right to delete or a pinch to navigate confuses users; map gestures by natural metaphor and reserve simple motions for frequent actions.

### Ignoring Ergonomics

Gestures that strain the thumb, require two hands, or fatigue with repetition are avoided by users; design for the hand and body.

### An Incoherent Gesture Vocabulary

When the same gesture means different things in different places, users stop trusting gestures; build a consistent, documented vocabulary.

### Unresolved Gesture Conflicts

Overlapping gestures with no disambiguation produce wrong guesses about intent; resolve conflicts by direction, threshold, or timing.

### Gesture-Only Interactions

Actions accessible only by gesture exclude users with motor impairments and those using assistive technology; provide non-gesture alternatives.

### Assuming Gestures Will Be Discovered

Invisible gestures are functionally absent for users who do not try them; design affordances, hints, and visible alternatives.

### Violating Platform Conventions

Repurposing standard platform gestures for different meanings feels broken to users; respect conventions unless deviation is justified.

### Testing Only In Ideal Conditions

Gestures that work on one device in a quiet setting fail across orientations, devices, and real contexts; test broadly.

## Self-Check

- [ ] Gestures are mapped to actions by natural metaphor, with simple motions reserved for frequent actions.
- [ ] Gesture-active zones respect ergonomics, especially the thumb zone for one-handed use, and avoid strain or fatigue.
- [ ] A coherent, documented gesture vocabulary assigns each gesture a consistent meaning across the product.
- [ ] Gesture conflicts are identified and resolved by direction, distance threshold, or timing, and tested with users.
- [ ] Every gesture-based action has a discoverable, usable non-gesture alternative, and no functionality is gesture-only.
- [ ] Discoverability is designed through affordances, hints, onboarding, and visible alternatives, not assumed.
- [ ] Platform gesture conventions are respected, with deviations justified and teachable.
- [ ] Gestures were tested across devices, orientations, input methods, and real-world contexts, not only ideal conditions.
