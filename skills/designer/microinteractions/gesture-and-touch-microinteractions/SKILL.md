---
name: gesture_and_touch_microinteractions.md
description: Use when the agent is designing touch and gesture interactions such as swipe, long-press, drag, pinch, pull-to-refresh, tap-and-hold menus, multi-touch, and the moment-to-moment feedback that confirms a gesture is recognized, in progress, or completed on touch devices.
---

# Gesture And Touch Microinteractions

Gestures are seductive because they feel direct and efficient, but they are also invisible. Unlike a button, a gesture has no resting affordance; the user must know it exists, perform it precisely, and receive feedback that the system recognized it. When any of these fail, the gesture becomes a source of frustration, errors, and exclusion. The judgment problem is that gestures are designed for the ideal hand on the ideal screen and then break under real conditions: one-handed use, small hands, thumb reach, screen protectors, cases, gloves, motion, and users who simply never discover the gesture exists.

Use this skill before specifying swipe, long-press, drag, pinch, pull-to-refresh, or any touch-based interaction, and before deciding what feedback accompanies it. The goal is to prevent the agent from making gestures the only path to a function, from ignoring reach and ergonomics, or from shipping gestures that exclude assistive technology users.

## Core Rules

### Never Make A Gesture The Only Path

The single most important rule of gesture design is that any function reachable by gesture must also be reachable by a visible, accessible control. Gestures are accelerators for users who know them, not gates that lock out users who do not. A swipe-to-delete with no delete button excludes first-time users, users with motor differences who cannot perform the swipe, and assistive technology users for whom the gesture may be impossible.

For every gesture, provide an equivalent visible action: a button, a menu item, or a context menu. The gesture can remain the faster path, but it must not be the only path.

### Design For Real Hands And Reach

Touch happens with thumbs, often one-handed, on screens that are larger than the hand can comfortably reach. A gesture that requires precise two-finger coordination or a reach to the top corner of a large phone is usable only in ideal conditions. Real users hold phones on the move, with one hand occupied, on unstable surfaces, or with limited dexterity.

Place gesture targets within the comfortable thumb zone, favor the bottom and middle of the screen for primary gestures, and avoid requiring reach to the top for frequent actions. Account for the fact that the dominant hand, hand size, and grip all vary. A gesture that works for the designer's hand may fail for many users.

### Provide Continuous Feedback During The Gesture

Gestures need live feedback so the user knows the system is tracking their input and what will happen if they complete the motion. A swipe that reveals a delete action as the finger drags, a drag that moves the item in real time, a pull-to-refresh that stretches an indicator as the user pulls: all of these confirm the gesture is recognized and preview the outcome.

Without continuous feedback, the user performs the gesture blind, releases, and only then discovers whether it worked. This is especially damaging for gestures with destructive outcomes, where the user needs to see the consequence before committing.

### Define Clear Activation Thresholds

A gesture must have an unambiguous threshold that separates it from a tap, a scroll, or an accidental touch. Swipe should require a minimum distance and direction; long-press should require a minimum hold without movement; pinch should require a detectable scale change. Without thresholds, the system misinterprets input: a scroll becomes a swipe-delete, a tap becomes a long-press menu, a two-finger touch becomes an accidental pinch.

Specify the thresholds deliberately and test them. Too sensitive, and gestures fire by accident; too strict, and users cannot trigger them reliably. The thresholds should accommodate imprecise input without becoming hair-triggers.

### Make Gesture Discovery Possible

Because gestures are invisible, users will not find them unless the design teaches them. Discovery can happen through brief hints on first encounter, through visible affordances that suggest motion such as a handle or a peeking edge, through onboarding that demonstrates the gesture, or through progressive disclosure that reveals advanced gestures after basic ones are learned.

Assume that a gesture with no discovery mechanism will be used only by users who happen to know it. If the gesture is important, invest in making it discoverable.

### Distinguish Gesture From Scroll And Navigation

The most common gesture conflict is between a custom gesture and the platform's native scroll or navigation. A swipe-left-to-delete that interferes with horizontal scrolling, a long-press that blocks text selection, or a pinch that fights the browser's zoom creates a frustrating tug-of-war where neither gesture works reliably.

Before assigning a gesture, verify it does not collide with expected touch behavior on the platform. Where conflict is unavoidable, choose the more important behavior and provide the other through a different input, rather than letting them fight ambiguously.

### Account For Accidental And Repeated Gestures

Touch input is noisy. Users will brush the screen, perform partial gestures, release mid-motion, and repeat gestures rapidly. The design must handle these gracefully: an incomplete swipe should snap back without acting, a repeated tap should not trigger the action twice, an accidental long-press should be cancelable by moving away before release.

Destructive gestures need particular care, because an accidental swipe-delete that commits on release without confirmation can destroy data the user did not intend to remove. Prefer requiring a deliberate second action, or providing undo, for destructive gesture outcomes.

### Honor Accessibility And Alternative Input

Gestures are inherently difficult for assistive technology. Screen reader users navigate by swipe and tap in ways that conflict with custom gestures. Voice control users cannot perform multi-touch. Switch users cannot pinch. Any gesture-based function must have a non-gesture alternative that works with these input methods, and custom gestures should be implemented with proper accessibility semantics so assistive tech can offer equivalents.

Designing gestures without an accessible alternative is designing a feature that a segment of users cannot use at all.

## Common Traps

### Gesture-Only Paths

Functions reachable only by swipe, long-press, or pinch exclude users who cannot perform or discover the gesture.

### Ignoring Thumb Reach And One-Handed Use

Gestures requiring reach to screen corners or precise two-hand coordination fail for the majority of real-world mobile use.

### No Live Feedback During The Gesture

Gestures that give no feedback until release force users to perform them blind, increasing errors and reducing confidence.

### Ambiguous Activation Thresholds

Gestures without clear thresholds misfire on taps, scrolls, and accidental touches, making the interface feel unpredictable.

### Undiscoverable Gestures

Important gestures with no hint, affordance, or teaching will be used only by users who already know them, leaving the rest locked out.

### Conflict With Native Scroll Or Zoom

Custom gestures that fight the platform's scroll, navigation, or zoom create frustrating input battles where nothing works reliably.

### Destructive Gestures Without Undo

Swipe-to-delete or similar actions that commit on release without confirmation or undo invite costly accidental data loss.

### No Accessible Alternative

Gesture-based functions with no keyboard, switch, or voice equivalent exclude assistive technology users entirely.

## Self-Check

- [ ] Every gesture-reachable function also has a visible, accessible alternative control.
- [ ] Gesture targets sit within comfortable thumb reach, and frequent gestures avoid screen corners and top-of-screen positions.
- [ ] The gesture provides continuous, live feedback during the motion so the user sees it is recognized and previews the outcome.
- [ ] Clear activation thresholds separate the gesture from taps, scrolls, and accidental touches, tested for reliability.
- [ ] Important gestures have discovery mechanisms such as hints, affordances, or onboarding.
- [ ] The gesture does not conflict with native scroll, navigation, or zoom behavior on the target platform.
- [ ] Incomplete, repeated, and accidental gestures are handled gracefully, with snap-back and cancellation.
- [ ] Destructive gesture outcomes require a deliberate second action or provide undo.
- [ ] Gesture-based functions have keyboard, switch, voice, and assistive-technology alternatives with proper accessibility semantics.
