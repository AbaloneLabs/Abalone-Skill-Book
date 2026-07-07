---
name: immersive_interaction.md
description: Use when the agent is designing immersive interactions for VR, AR, or mixed reality, including direct manipulation, gesture-based controls, embodied navigation, gaze and pinch, voice commands, haptics, or any interaction where the user acts within a 3D space rather than through a flat surface.
---

# Immersive Interaction

Immersive interaction asks the user to act with their body, gaze, voice, and movement inside a three-dimensional space. This is fundamentally different from interacting through a flat surface with a stable pointer. There is no mouse that always works, no cursor that is always visible, no click target that is always the same size, and no guarantee the user's hands, eyes, or voice are available or precise. The freedom of immersive input is also its fragility.

The judgment problem is that immersive interactions that feel magical in a short demo often fail in real conditions. A pinch gesture that works in a clean studio fails when the user is holding a cup. A gaze-and-dwell selection that feels effortless for one task becomes agonizingly slow for the fiftieth selection. A voice command that demos beautifully fails in a noisy office or a shared room. The agent's job is to design immersive interactions that remain reliable, comfortable, and accessible across bodies, environments, and sessions, not just impressive in a controlled showcase.

Use this skill before designing or reviewing gestures, direct manipulation, embodied navigation, gaze interaction, voice control, haptic feedback, or any interaction where the user acts within a 3D space. The goal is to prevent the agent from shipping interactions that are imprecise, fatiguing, socially awkward, inaccessible, or impossible to discover.

## Core Rules

### Match Each Interaction To A Reliable Input Primitive

Immersive systems offer many input primitives: direct touch, ray pointing, gaze, pinch, voice, controllers, and haptics. None is universally best, and each has a precision, latency, fatigue, and social profile. Choose the primitive by the task.

Direct manipulation (reach and touch):

- intuitive and satisfying for objects within reach;
- high arm fatigue over time;
- poor for distant, small, or dense targets.

Ray or controller pointing:

- precise and low-fatigue;
- good for distant and dense selection;
- requires a device or a learned hand posture.

Gaze and dwell or gaze and pinch:

- low physical effort;
- slow and imprecise for dense targets;
- can cause unintended activation if dwell is too short.

Voice:

- excellent for commands, search, and discrete actions;
- fails in noise, privacy-sensitive settings, or for non-discrete positioning;
- should never be the only path to a function.

Map each interaction in the product to the primitive that suits it, and provide alternates. Do not assume the user can or will use the same primitive for everything.

### Design For The Body, Not The Idealized Hand

Immersive interaction depends on the user's body, and bodies vary. Range of motion, handedness, tremor, fatigue, prosthetics, injury, and the simple fact that hands are often holding something all change what is possible.

Design for real bodies by:

- supporting both left- and right-handed use;
- avoiding gestures that require sustained or extreme postures;
- providing low-effort alternatives to freehand gestures;
- not requiring two hands for common actions;
- accounting for users who are seated, who cannot stand, or who have limited reach;
- ensuring the interaction works when the user is holding an object or resting their hands.

An interaction that only works for a standing, two-handed, fully mobile user in an empty room excludes most of the people who might use the product. Accessibility in immersive interaction is not optional polish; it is whether the interaction works at all.

### Make Affordance And Discoverability Explicit

Immersive interfaces have no cursor, no hover tooltip, and no obvious clickable surface. Users often cannot tell what is interactive, how to activate it, or what gesture is expected. Discoverability must be designed, not assumed.

Support discoverability by:

- giving interactive objects clear visual affordance, such as handles, highlights, or hover states;
- showing the expected gesture or input near new interaction types;
- providing gentle onboarding for novel gestures;
- using consistent gesture-to-meaning mappings across the product;
- offering a visible, reliable fallback such as a menu or pointer for users who cannot discover or perform the gesture.

A gesture the user cannot find is a feature that does not exist. Treat discoverability as a first-class design requirement.

### Provide Immediate, Multimodal Feedback

Immersive interactions need richer feedback than flat interfaces because the input is less precise and the consequences are less obvious. The user must know at every moment whether the system registered their intent.

Provide feedback across channels:

- visual feedback for state, hover, selection, and progress;
- audio feedback for confirmation, errors, and events outside the field of view;
- haptic feedback for contact, selection, and boundaries where hardware supports it;
- spatialized audio to indicate direction and location of events.

Feedback should be immediate. Latency between a gesture and its response breaks the sense of direct manipulation and erodes trust. Where latency is unavoidable, provide a visual signal that the input was received so the user does not repeat or abandon the action.

### Bound Gestures To Clear Intent And Avoid False Activation

Immersive input is noisy. Hands move naturally, people gesture while talking, and dwell or gaze systems can trigger from ordinary looking. False activations are frustrating and can be destructive.

Reduce false activation by:

- requiring a clear, deliberate commit action, such as a pinch, rather than passive dwell alone for consequential actions;
- distinguishing conversational or resting hand movement from intentional gestures;
- avoiding gestures that are easy to perform accidentally;
- confirming destructive or irreversible actions explicitly;
- allowing the user to undo or cancel mistaken activations easily.

The more natural a gesture feels, the more likely it is to fire accidentally. Build in a commit step for anything that matters.

### Respect Social And Environmental Context

Immersive interactions happen in the real world, often around other people. Gestures that feel fine alone become awkward in a meeting, on transit, or in a shared office. Voice commands that work at home fail in public.

Consider context by:

- providing silent or subtle input alternatives to voice and large gestures;
- not requiring conspicuous movement in shared or public spaces;
- respecting privacy when voice or camera input is involved;
- designing for users who cannot speak loudly or move freely in their environment;
- signaling clearly to bystanders what the wearer is doing when relevant.

An interaction that requires the user to wave and talk in a quiet office will simply not be used. Design for the environments where the product will actually live.

### Design For Sessions, Fatigue, And Recovery

Immersive sessions are taxing. Arms tire, eyes strain, headsets get hot, and attention is fully consumed. Good immersive design respects the limits of the body over time.

Respect session limits by:

- favoring low-effort input for repeated and long tasks;
- allowing the user to rest their hands without losing state;
- breaking long flows into shorter, resumable segments;
- avoiding forcing sustained attention or posture;
- providing clear save points and easy resumption;
- signaling when a break would help.

A product that exhausts the user in twenty minutes cannot serve tasks that take an hour. Design the interaction economy around realistic session lengths.

### Plan For Input Failure And Recovery

Immersive input fails often: tracking drops, a hand leaves the camera's view, voice is drowned out, a controller disconnects. The interface must degrade gracefully and recover without losing the user's work.

Plan for failure by:

- preserving state when input drops momentarily;
- providing an alternate input path when the primary fails;
- clearly signaling when tracking or input is lost;
- never leaving the user stuck with no way to proceed;
- making recovery obvious and low-friction.

An immersive interface is only as reliable as its worst input moment. Design the recovery path before designing the happy path.

## Common Traps

### Demos That Hide Real-World Failure Modes

A gesture that works perfectly in a controlled demo may fail with cluttered backgrounds, low light, held objects, or tired users. Test in realistic conditions, not just showcases.

### Sustained Gestures That Cause Fatigue

Interactions that require raised arms or held postures become painful within minutes. Favor brief, low-effort gestures and resting positions.

### Undiscoverable Gestures

If the user cannot tell what is interactive or what gesture to use, the feature is invisible. Build affordance and onboarding into every novel interaction.

### Passive Dwell For Consequential Actions

Dwell-based activation fires from ordinary looking and causes destructive mistakes. Require a deliberate commit for anything that matters.

### Voice As The Only Path

Voice fails in noise, privacy-sensitive settings, and for users who cannot speak or speak loudly. Every voice-only function needs a reliable alternate path.

### Ignoring Social Awkwardness

Large gestures and voice commands are unusable in shared or public spaces. Provide subtle, silent alternatives.

### No Recovery When Tracking Drops

When a hand leaves view or tracking fails, the user should not lose their work or get stuck. Design explicit recovery and state preservation.

## Self-Check

- [ ] Each interaction is mapped to an input primitive suited to its precision, fatigue, and social profile, with alternates available.
- [ ] The interaction works for varied bodies, handedness, seated users, limited mobility, and users whose hands are occupied.
- [ ] Interactive elements have clear visual affordance, expected gestures are surfaced, and a reliable fallback exists for undiscoverable input.
- [ ] Feedback is immediate, multimodal, and confirms that input was registered, with low latency or a visible receiving signal.
- [ ] Consequential actions require a deliberate commit, and false activation from natural movement or dwell is minimized.
- [ ] Silent, subtle, and private input alternatives exist for shared, public, or noise-sensitive environments.
- [ ] Repeated and long tasks favor low-effort input, allow resting without losing state, and respect realistic session lengths.
- [ ] Input failure, tracking loss, and disconnection are handled with state preservation, clear signaling, and obvious recovery.
- [ ] Voice and gesture are never the only path to a function; alternates exist for users and contexts where they fail.
- [ ] The interaction has been considered in a real environment with clutter, noise, other people, and fatigue, not only in a clean demo.
