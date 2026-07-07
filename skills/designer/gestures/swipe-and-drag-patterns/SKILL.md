---
name: swipe_and_drag_patterns.md
description: Use when the agent is designing swipe actions, drag-and-drop, reorderable lists, sliders, carousels, pull-to-refresh, swipe-to-dismiss, sortable boards, or any interaction where content moves in response to a sustained gesture.
---

# Swipe And Drag Patterns

Swipe and drag are not the same gesture. A swipe is a quick flick that signals direction and intent; a drag is a sustained hold-and-move where the object follows the pointer or finger in real time. Confusing the two, or mixing them on the same surface, is one of the most common causes of false triggers, lost data, and user frustration.

Use this skill before designing or reviewing lists, boards, kanban views, reorderable galleries, carousels, sliders, range controls, file drop zones, swipeable cards, pull-to-refresh, or any interface where movement is the interaction. The goal is to prevent the agent from building gesture surfaces that fight the user's intent, conflict with scrolling, or commit destructive actions without a clear recovery path.

## Core Rules

### Separate Swipe Intent From Drag Intent

A swipe communicates direction and speed; the system interprets the gesture after it completes. A drag communicates continuous position; the object moves with the input until release. The interaction model should make clear which one is in play.

Decide explicitly:

- is the user flicking to navigate or dismiss, or are they carrying an object to a target?
- does the gesture end on lift, or does it commit based on final position?
- should the object snap back, snap forward, or stay where released?
- is velocity relevant, or only final position?

When a single surface tries to support both, the system must disambiguate, and disambiguation is where false triggers are born.

### Resolve Axis Conflicts Before Adding Gestures

The most damaging conflicts happen when swipe, drag, and scroll compete on the same axis. A horizontal swipe used for both paging a carousel and revealing row actions, or a vertical drag used for both scrolling a list and reordering rows, forces the interface to guess.

Prevent conflict by:

- assigning distinct axes to distinct intents;
- using thresholds and direction locks so a slightly diagonal motion commits to one axis;
- avoiding row-level swipe actions inside horizontally paged views;
- separating reorder drag handles from the scrollable body;
- confirming that nested scroll containers do not steal each other's gestures.

If two gestures must share an axis, one should require an explicit mode, handle, or delay.

### Design Clear Affordances For Draggable Objects

Draggable objects need to look draggable, or users will never try. A reorder handle, a grab cursor, a lifted shadow on press, a subtle hint animation, or instructional text can signal affordance.

For drag-and-drop:

- show that the object is grabbed and is following the input;
- highlight valid drop targets as the object approaches;
- dim or disable invalid targets;
- show a preview or ghost of where the object will land;
- animate the list or board reordering live as the drag progresses.

A drag with no live feedback feels disconnected and makes placement errors likely.

### Make Swipe Actions Discoverable And Reversible

Swipe-to-reveal and swipe-to-dismiss are powerful but invisible. Users will not know a row supports swipe until they try, and a destructive swipe can feel catastrophic if it commits instantly.

Mitigate:

- provide a visible hint, such as a partially revealed action or an affordance cue, for first-time users;
- reveal actions progressively as the swipe distance grows;
- require a threshold distance before committing;
- show what action will fire before release;
- offer undo or a short cancel window after a destructive swipe;
- avoid making swipe the only way to reach an important action.

Swipe should accelerate expert use, not gate core tasks.

### Handle Reordering State Explicitly

Reorderable lists and boards introduce state that static lists do not have: the item being moved, the placeholder it leaves, the insertion point, and the final committed order.

Design for:

- a clear grabbed state that distinguishes the moving item from selected or pressed items;
- a visible gap or placeholder showing the source position;
- a live insertion indicator showing the target position;
- smooth reflow of surrounding items without jarring jumps;
- commit behavior on release, with snap-back if the drop is invalid;
- persistence of the new order and feedback that it was saved.

Reordering that silently fails to save, or that jumps unpredictably, destroys user trust.

### Choose Snap Behavior Deliberately

Swipes and drags end in one of several ways: snap back to origin, snap forward to commit, snap to the nearest valid position, or stay free. Each implies different risk.

Guidance:

- snap back when the gesture did not cross a threshold, so hesitant users are not punished;
- snap forward only past a clear threshold, with the threshold visible or intuitive;
- snap to nearest for sliders, segments, and grid-based drops where free positioning is meaningless;
- stay free only when continuous positioning is the point, such as a canvas or drawing tool.

Inconsistent snap behavior across similar controls confuses users.

### Support Cancellation And Invalid Drops

Gestures are abandoned mid-motion for many reasons: a change of mind, a notification, a misread target. The system must allow safe escape.

Ensure:

- moving back past the origin or lifting in neutral space cancels the gesture;
- dropping on an invalid target returns the object to its origin with clear feedback;
- pressing escape or a cancel control aborts the drag;
- partial drags do not leave the interface in a half-committed state;
- multi-item drags restore all items if canceled.

## Common Traps

### Swipe And Scroll On The Same Axis

When a list scrolls vertically and rows also support vertical swipe actions, every scroll attempt risks triggering an action. Axis overlap is the leading cause of accidental deletes.

### Committing Destructive Swipes Instantly

A swipe-to-delete with no undo, no threshold, and no confirmation turns a slip into data loss. Speed should not remove recoverability.

### No Live Feedback During Drag

If the dragged item does not visibly follow the pointer, or if the target list does not reflow, users cannot predict where the item will land and will misplace it.

### Invisible Drag Affordance

An object that can be dragged but gives no visual cue will rarely be dragged. Users assume static unless told otherwise.

### Threshold Set By Feel, Not By Data

A swipe threshold tuned to the designer's hand may be unreachable for users with limited dexterity or trivially easy for fast flickers. Thresholds should account for a range of users and devices.

### Reorder That Does Not Persist

A drag that visually reorders the list but fails to save, or saves with a race condition, makes the feature worse than useless. Persistence and error states must be part of the design.

### Carousel Paging That Eats Row Swipes

Horizontal carousels inside vertically scrolling pages are fine, but horizontal row actions inside a horizontally paging view create an unresolved conflict where the system must guess.

## Self-Check

- [ ] Swipe and drag intents are distinguished, and the surface does not force the system to guess between them.
- [ ] Gestures that share a surface use distinct axes, direction locks, thresholds, or explicit handles to avoid conflict.
- [ ] Draggable objects show clear affordance, and drag progress includes live feedback, target highlighting, and invalid-target dimming.
- [ ] Swipe-to-reveal and swipe-to-dismiss actions are discoverable, threshold-gated, and reversible or undoable.
- [ ] Reorderable lists and boards show grabbed state, source placeholder, live insertion point, smooth reflow, and confirmed persistence.
- [ ] Snap behavior, back, forward, nearest, or free, is chosen deliberately and consistently for each control type.
- [ ] Cancellation, invalid drops, and abandoned gestures return the interface to a clean state without partial commits.
- [ ] Destructive swipe and drag actions require a threshold, preview, or recovery path.
- [ ] Thresholds and distances account for a range of hand sizes, dexterity levels, and device sizes, not only the designer's.
- [ ] The gesture surface was tested against realistic content length, list size, scroll behavior, and interruption.