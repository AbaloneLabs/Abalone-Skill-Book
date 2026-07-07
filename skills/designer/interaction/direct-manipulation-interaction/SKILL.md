---
name: direct_manipulation_interaction.md
description: Use when the agent is designing direct manipulation interactions such as drag-and-drop, in-place editing, resize, rotate, canvas and spatial manipulation, deciding when direct manipulation beats a form or menu, designing selection and multi-select gestures, handling precision and snapping, ensuring discoverability of manipulable objects, or making direct manipulation accessible to users who cannot use a pointer or touch.
---

# Direct Manipulation Interaction

Direct manipulation is the class of interaction where the user acts on an object as if it were physical: dragging it, resizing it, rotating it, editing it in place. It is among the most satisfying interaction models because it closes the gap between intent and result, letting the user feel that they are touching the data rather than operating a machine through a menu. But it is also among the most fragile. Direct manipulation that is undiscoverable, imprecise, or inaccessible leaves users unable to perform the action at all, because there is often no fallback. A button can be found by scanning; a drag handle that looks like decoration cannot.

Agents tend to fail direct manipulation in predictable ways. They choose drag-and-drop because it feels modern when a simpler control would do the job faster and more accessibly. They make manipulable objects invisible, so users never discover they can be dragged or resized. They ignore precision, so fine adjustments require superhuman motor control. Or they build the interaction for the pointer and forget that keyboard, screen reader, and voice users must also be able to perform the same task.

Use this skill before designing any drag, resize, rotate, in-place edit, or canvas interaction. The goal is to choose direct manipulation only when it is genuinely the best fit, to make it discoverable and precise, and to guarantee an accessible equivalent so the interaction is not gated on a single input method.

## Core Rules

### Choose Direct Manipulation Only When It Is The Best Fit

Direct manipulation is powerful but expensive to build, to discover, and to make accessible. It should be chosen when its qualities, spatial reasoning, continuous adjustment, and physical intuition, genuinely serve the task, not because it feels more modern than a form or menu.

Choose direct manipulation when:

- the task is inherently spatial, such as arranging items on a canvas or resizing a region;
- continuous, real-time adjustment matters, such as scrubbing a value or rotating an object;
- the physical metaphor clarifies the operation, such as dragging to reorder;
- the alternative form or menu would be more cumbersome than the manipulation.

Prefer a simpler control when:

- the task is a discrete choice that a button, menu, or input handles more efficiently;
- precision matters more than spatial intuition, and a numeric input would be more accurate;
- the manipulation would be undiscoverable and no accessible equivalent is feasible.

A drag-and-drop imposed on a task that a checkbox would solve adds cost and excludes users for no gain.

### Make Manipulable Objects Discoverable

The central failure of direct manipulation is invisibility. If users cannot tell that an object can be dragged, resized, or edited, the interaction might as well not exist, and they will conclude the product cannot do what it can. Discoverability must be designed, not hoped for.

Make manipulation visible:

- reveal handles, grips, or affordances on hover and focus, not only on chance discovery;
- use cursor changes to signal draggable, resizable, or editable regions;
- provide explicit cues such as move icons, resize handles, or edit affordances rather than relying on users to guess;
- offer an alternative path, such as a menu item or keyboard command, for users who do not discover the gesture.

An object that can only be manipulated by users who already know it can be manipulated excludes everyone else.

### Handle Precision And Snapping Deliberately

Direct manipulation is imprecise by nature, because pointer and touch input are noisy. Without support, fine adjustments become exercises in frustration, and users cannot hit exact values or alignments. Precision support is what separates a usable manipulation from a fiddly one.

Support precision:

- provide snapping to meaningful increments, such as grid lines, guides, or common values, so users can hit targets reliably;
- offer numeric input alongside manipulation, so exact values can be entered when the gesture cannot achieve them;
- show the current value during manipulation, so users have feedback on what they are setting;
- scale manipulation sensitivity to context, so coarse drags move quickly and fine adjustments can be made slowly.

A resize handle that requires pixel-perfect positioning to hit a specific size is a precision failure, not a skill requirement.

### Design Selection And Multi-Select Coherently

Many direct manipulation interfaces depend on selection: selecting before dragging, resizing, or deleting. Selection is its own interaction model, and incoherent selection produces confusion about what an action will affect.

Design selection deliberately:

- make selection state always visible, with a clear indication of what is currently selected;
- support multi-select with standard conventions, such as shift and modifier keys, and expose the selection count;
- make it obvious what an action will apply to, so users do not perform a destructive action on an unintended selection;
- preserve selection across related operations, so users can act on the same set repeatedly.

A selection that silently clears, or a bulk action that applies to a hidden selection, causes destructive mistakes.

### Provide An Accessible Equivalent For Every Manipulation

Direct manipulation is inherently pointer- and touch-oriented, which excludes users who navigate by keyboard, screen reader, or voice. Accessibility law and ethics both require that the same task be achievable without the gesture, and the equivalent must be first-class, not a degraded afterthought.

Guarantee accessible equivalents:

- every drag must have a keyboard or menu equivalent, such as cut and paste, move up and move down, or a position input;
- every resize must have a numeric input or menu option;
- every in-place edit must be reachable and operable by keyboard and announced by screen readers;
- expose the manipulation's state and result to assistive technology, so users who cannot see the gesture still know what happened.

An interaction that can only be performed with a pointer is an interaction that some users cannot perform at all.

### Give Continuous Feedback During Manipulation

During a manipulation, the user is making a decision in real time, and the interface must reflect the current state continuously so the decision can be guided. Stale or absent feedback forces users to manipulate blind.

Provide continuous feedback:

- show the object moving, resizing, or rotating in real time as the input changes;
- display the relevant value, dimensions, or position during the manipulation;
- highlight snap targets, guides, or valid drop zones as they come into range;
- confirm validity, such as whether a drop target will accept the object, before the action commits.

### Handle Interruption And Cancellation Gracefully

Manipulations can be interrupted: the user releases at the wrong moment, the input device disconnects, or the user changes their mind. The interaction must handle these gracefully, neither committing an unintended action nor leaving the object in a broken state.

Design for interruption:

- allow cancellation mid-manipulation, such as pressing escape to revert;
- define what happens on accidental release, such as snapping back or committing at the current position, and make the behavior predictable;
- preserve object integrity if the manipulation is abandoned, so no half-applied state remains;
- support undo after commit, because direct manipulations are especially prone to slips.

### Match The Interaction To The Platform's Input Model

Direct manipulation that works with a mouse often fails on touch, and vice versa. Touch targets must be larger, hover does not exist on touch, and multi-touch enables gestures a pointer cannot express. The interaction must be designed for the input model of the platform it ships on.

Adapt to the platform:

- size touch targets adequately and separate them so they do not conflict;
- replace hover-revealed affordances with persistent or focus-revealed equivalents on touch;
- leverage multi-touch where it adds capability, such as pinch to zoom, but provide single-input equivalents;
- preserve the underlying model across platforms so users transfer their learning even as the gestures differ.

## Common Traps

### Choosing Direct Manipulation For Novelty

Imposing drag-and-drop or canvas manipulation on a task a simpler control would solve adds cost and excludes users for no gain.

### Invisible Manipulable Objects

Objects whose drag, resize, or edit affordances are never revealed can only be used by users who already know they exist.

### No Precision Support

Manipulations without snapping, numeric input, or value display force users to hit exact positions by feel, which is frustrating and inaccessible.

### Incoherent Selection

Selection that silently clears, or bulk actions applied to hidden selections, causes destructive mistakes.

### No Accessible Equivalent

An interaction operable only by pointer excludes keyboard, screen reader, and voice users. Every manipulation needs a first-class equivalent.

### Manipulating Blind

Manipulations without continuous feedback force users to adjust without knowing the current state or whether the target will accept the action.

### Brittle Interruption Handling

Manipulations that commit on accidental release or leave objects in half-applied states cause slips the user cannot recover from.

### Ignoring The Platform's Input Model

Mouse-oriented manipulation fails on touch, and touch gestures have no pointer equivalent. Design for the platform's input.

## Self-Check

- [ ] Direct manipulation was chosen because spatial reasoning, continuous adjustment, or physical intuition genuinely serves the task, not for novelty.
- [ ] Manipulable objects are discoverable through handles, cursors, and explicit cues, with an alternative path for users who do not find the gesture.
- [ ] Precision is supported through snapping, numeric input, value display, and sensitivity scaled to context.
- [ ] Selection state is always visible, multi-select follows conventions, the count is exposed, and it is obvious what each action will affect.
- [ ] Every manipulation has a first-class accessible equivalent operable by keyboard, screen reader, and voice, with state announced to assistive technology.
- [ ] Continuous feedback shows the object's state, value, snap targets, and validity during the manipulation.
- [ ] Interruption and cancellation are handled gracefully, with escape to revert, predictable behavior on release, and undo after commit.
- [ ] The interaction is adapted to the platform's input model, with adequate touch targets, persistent affordances on touch, and single-input equivalents for multi-touch gestures.
- [ ] The manipulation preserves object integrity if abandoned, leaving no half-applied state.
- [ ] No user is excluded from the task because they cannot use the pointer or touch gesture the interaction requires.
