---
name: transition_and_spatial_design.md
description: Use when the agent is designing transitions between views, pages, or states, including page navigations, modal and drawer entrances, tab switches, expand and collapse, drag-to-open, shared element transitions, and the spatial model that explains where things come from and go.
---

# Transition And Spatial Design

A transition is the moment a user's mental map of the interface is most fragile. When one view replaces another, the user must rebuild their sense of where they are, what changed, and how to get back. A well-designed transition carries the user across that gap by preserving continuity and reinforcing a spatial model of the product. A poor transition, or no transition at all, forces the user to reorient from scratch, which costs attention and breeds disorientation. The judgment problem is that transitions are easy to under-design, because a static before-and-after looks fine in a spec, but the experience lives in the movement between them.

Use this skill before specifying page changes, modal and drawer behavior, tab switches, expand and collapse, master-detail flows, drag-to-open interactions, or any moment where the visible surface changes. The goal is to prevent the agent from treating transitions as visual polish and ignoring the spatial reasoning that makes a product feel coherent.

## Core Rules

### Establish A Stable Spatial Model

Users implicitly build a spatial model of where things are: navigation is on the left, content is in the center, detail opens to the right, modals float above. Transitions should reinforce this model, not contradict it. If a drawer slides from the right in one place and from the left in another, the model breaks.

Decide the product's spatial vocabulary deliberately:

- where modals, drawers, and panels originate and settle;
- whether detail views push, replace, or overlay;
- whether navigation changes are horizontal, vertical, or depth-based;
- where the user is returned to on dismiss.

Once set, every transition should honor it. A stable spatial model is what lets users navigate without thinking.

### Match The Transition To The Hierarchy Of Change

Not every change deserves the same transition. A small, local change such as expanding a row or revealing a tooltip should use a contained, fast transition near the trigger. A peer-level change such as switching tabs should swap content in place with minimal travel. A hierarchical change such as drilling into a detail should feel like moving deeper, often with a directional or scaling motion. A top-level navigation change can use a fuller transition because the user is changing context.

Over-transitioning small changes makes the interface feel twitchy. Under-transitioning large changes makes them feel abrupt and disconnected. Match the weight of the transition to the weight of the change.

### Preserve Continuity Between Origin And Destination

The strongest transitions connect the element the user acted on with the result that appears. When a user taps a card and it expands into a full view, animating that expansion tells the user the detail came from the card they chose. When a user selects an item and a panel opens beside it, the connection is clear. This continuity, often called shared element or hero transition, is the single most effective tool for preventing disorientation.

Without continuity, the user sees a disappearance and a separate appearance and must infer the relationship. With it, the relationship is shown.

### Make Direction Mean Something

Directional transitions carry meaning. Moving forward into detail often feels like moving right or deeper. Moving back often feels like moving left or shallower. Opening a drawer from the side implies it was parked there. A modal scaling up from its trigger implies it grew from that point.

Use direction consistently so that it becomes legible. If forward is always to the right, users learn the convention. If direction is random, it adds noise. Avoid using opposite directions for conceptually similar moves.

### Keep The User Oriented During And After

A transition should never leave the user wondering where they are or how to return. During the transition, preserve enough context that the origin remains perceptible until the destination settles. After the transition, make the current location obvious through titles, highlighted navigation, breadcrumbs, or visible back affordances.

Disorientation is most likely when a transition hides the origin completely before the destination appears, creating a blank or flashing intermediate state. Overlap the outgoing and incoming content, or use a brief crossfade, so there is always something to hold onto.

### Respect The User's Intent And Speed

Transitions serve the user, not the designer's portfolio. A transition that is too slow makes repeated navigation painful, especially for power users who move through the product quickly. Provide a way to interrupt or skip a transition where the user's intent is clearly to move on. Do not force the user to watch an animation they have seen many times.

At the same time, do not make transitions so fast that they fail to communicate the change. The balance is: fast enough to respect intent, slow enough to communicate continuity.

### Handle Interruptible And Reentrant Transitions

Users do not wait politely. They will tap again mid-transition, change direction, or trigger a new navigation before the previous one finishes. Transitions must handle interruption gracefully, reversing or redirecting without jumping, flickering, or stacking broken states.

A common failure is a transition that must complete before accepting new input, leaving the user feeling that the interface is ignoring them. Design transitions to be interruptible and to settle cleanly when the user changes course.

### Design Dismissal As Carefully As Entry

How something leaves is as important as how it arrives. A modal that vanishes instantly feels abrupt. A drawer that slides back to where it came from reinforces the spatial model. Dismissal should reverse the entry transition so the user understands they are returning to the origin, and it should restore focus to the element that opened the transition.

Forgotten focus restoration is a frequent accessibility and usability bug: after closing a modal, the user's keyboard focus should return to the trigger, not land at the top of the page.

## Common Traps

### Treating Transitions As Optional Polish

Skipping transition design and relying on instant swaps leaves users disoriented at every context change, because the relationship between views is never shown.

### Random Or Inconsistent Directions

Drawers, panels, and pages entering from different directions without reason prevent users from forming a spatial model.

### Over-Animating Small Changes

Applying heavy transitions to minor state changes makes the interface feel restless and slows frequent actions.

### Abrupt Blank Intermediate States

Letting the outgoing view fully disappear before the incoming view appears creates a flash that feels broken and loses the user's place.

### Non-Interruptible Transitions

Forcing a transition to complete before accepting new input makes the interface feel unresponsive to users who move quickly.

### Forgetting Focus Restoration On Dismissal

Closing a modal or drawer without returning focus to the trigger strands keyboard and screen reader users.

### Entry And Dismissal That Do Not Match

A modal that scales in but fades out breaks the spatial metaphor and confuses the user's sense of return.

## Self-Check

- [ ] A stable spatial model defines where panels, modals, drawers, and detail views originate and how navigation changes direction.
- [ ] The weight of each transition matches the hierarchy of the change: local, peer-level, hierarchical, or top-level navigation.
- [ ] Transitions preserve continuity between the element acted on and the result that appears.
- [ ] Direction is used consistently so that forward, back, open, and close are legible across the product.
- [ ] The user stays oriented during and after the transition, with no blank intermediate states and clear current-location cues.
- [ ] Transitions are fast enough to respect user intent, with a path to interrupt or skip for repeated navigation.
- [ ] Transitions handle interruption and reentry without flicker, stacking, or broken intermediate states.
- [ ] Dismissal reverses the entry transition and restores focus to the triggering element.
- [ ] The spatial model is consistent enough that users can predict where things will come from and return to.
