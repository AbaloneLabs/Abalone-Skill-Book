---
name: motion_principles_and_timing.md
description: Use when the agent is designing or specifying interface motion, animation timing, easing curves, duration, entrance and exit animations, hover and press responses, or deciding whether and how something should move to support comprehension without distracting or excluding users.
---

# Motion Principles And Timing

Motion is not decoration. It is information about cause, consequence, and continuity. A well-timed animation tells the user where an element came from, why it appeared, what changed, and where their attention should go next. Poorly chosen motion does the opposite: it slows the interface, masks latency, disorients users, and can trigger physical discomfort. The central judgment problem in motion is that every animation has a reason to exist and a cost to pay, and the designer must justify both.

Use this skill before specifying durations, easing curves, entrance and exit animations, hover and press responses, page transitions, or any moving element. The goal is to prevent the agent from applying motion because it looks lively, copying a generic easing from another product, or ignoring the users for whom motion is a barrier.

## Core Rules

### Give Every Animation A Job

Before adding motion, state what it accomplishes. Motion earns its place when it does one of the following:

- shows causality, so the user understands that their action caused a change;
- maintains spatial continuity, so an element is perceived as the same object across states;
- directs attention, so the user knows where to look;
- reveals hierarchy or relationship, such as a menu expanding from its trigger;
- provides feedback, so the user knows an input was received;
- softens an abrupt state change that would otherwise feel broken.

If an animation does none of these, it is decoration and should be questioned. Decorative motion is acceptable in brand or celebratory moments, but it should not be the default for functional surfaces.

### Match Duration To Purpose And Distance

Duration should follow the distance traveled and the cognitive purpose of the motion, not a single fixed number. Small, local changes such as a toggle, a hover, or a tooltip should be fast, typically well under 200 milliseconds, because the user is already looking at the element and waiting for confirmation. Larger spatial transitions such as a panel sliding in, a page change, or an element traveling across the screen need more time to remain readable, often in the 250 to 400 millisecond range, because the eye must track the movement.

Avoid two failure modes. Animations that are too slow make the interface feel sluggish and make the user wait for no reason. Animations that are too fast defeat their own purpose, because the change happens before the user can register what moved or why. When in doubt, favor slightly faster over slightly slower for functional motion; users tolerate brevity better than delay.

### Choose Easing To Match Physical And Cognitive Expectations

Linear motion feels mechanical and is rarely right for interface elements, because almost nothing in the physical world moves at a constant rate. Easing curves shape how motion feels: ease-out, where motion starts fast and decelerates, suits elements entering the view, because it draws attention at the start and settles calmly. Ease-in, where motion accelerates, suits elements leaving the view, because it implies departure. Ease-in-out suits larger travels where both start and end matter.

The key judgment is that easing should reinforce the meaning of the motion. An element that pops in with ease-out feels like it arrived for the user. An element that fades with ease-in feels like it left on its own. Mismatched easing, such as ease-in on an entrance, can make an element feel like it is being pulled away rather than presented.

### Keep Motion Consistent Within A Product

Motion is part of the design system. If one button eases in over 150 milliseconds and another over 300, if one panel slides from the right and another fades, the interface feels arbitrary. Users build expectations about how things move just as they do about how things look.

Establish motion tokens for duration and easing, tied to categories such as micro feedback, small transition, large transition, and emphasis. Apply them consistently so that similar changes feel similar across the product. Consistency is what makes motion feel intentional rather than random.

### Use Motion To Preserve Object Permanence

One of the strongest uses of motion is helping the user understand that an element on one screen is the same object on another. When a list item expands into a detail view, when a thumbnail grows into a full image, or when a card transforms into a panel, animating the transformation preserves the user's sense of continuity. Without it, the change feels like teleportation and the user must reorient.

This principle, often called continuity or shared element transition, is especially valuable in navigation and selection flows. The motion should connect the origin and destination clearly enough that the user does not lose track of the object.

### Direct Attention Without Hijacking It

Motion is powerful because the human eye is drawn to movement. That power must be used sparingly. A single, well-placed animation can guide the user to the right place. Multiple simultaneous animations, or continuous ambient motion, compete for attention and make the interface feel restless.

Reserve attention-directing motion for moments that matter: a new item arriving, a value updating, an error appearing, a primary action becoming available. Avoid motion that runs continuously in the background of a functional surface, because it drains attention and battery without purpose.

### Provide Feedback Fast Enough To Feel Direct

For direct manipulation, such as pressing a button or dragging a handle, the response must begin almost immediately, within roughly 100 milliseconds, to feel connected to the user's action. Any delay beyond that breaks the sense of direct control and makes the interface feel detached. This is separate from how long the full animation takes; the start must be instant even if the settle takes longer.

This is why hover and press states should transition quickly and begin on contact, not after a perceptible pause.

### Always Offer A Reduced-Motion Path

A subset of users experience motion as discomfort or harm. Vestibular disorders, migraines, and certain cognitive conditions can make sliding, scaling, and parallax motion physically unpleasant or dangerous. Interface motion must respect the operating system's reduced-motion preference and provide a calmer alternative, typically an instant or crossfade change with minimal travel.

This is not optional polish; it is an accessibility obligation. Designing motion without a reduced-motion fallback excludes real users.

## Common Traps

### Motion As Default Decoration

Adding animation to every state change because motion feels modern produces a busy interface where nothing signals anything because everything moves.

### One-Size-Fits-All Duration

Using a single duration for every animation makes small feedback feel sluggish and large transitions feel abrupt, because the timing does not match the distance or purpose.

### Linear Easing On Organic Elements

Constant-speed motion looks artificial for interface objects and drains the sense of direct manipulation.

### Ignoring Reduced-Motion Users

Shipping motion that cannot be disabled, or that only partially respects the preference, excludes users for whom the motion is not a preference but a health concern.

### Motion That Masks Real Problems

Using long animations to hide slow loading or broken responsiveness makes the interface feel slower and trains users to distrust it.

### Competing Simultaneous Animations

Multiple elements animating at once fight for attention and make the focal point unclear.

### Inconsistent Motion Across The Product

Different durations and easings for the same kind of change make the interface feel uncoordinated and lower perceived quality.

## Self-Check

- [ ] Every animation has an explicit purpose: causality, continuity, attention, hierarchy, feedback, or softening a state change.
- [ ] Durations match the distance and cognitive purpose, with fast timing for local feedback and longer timing for spatial transitions.
- [ ] Easing curves reinforce the meaning of the motion, with ease-out for entrances and ease-in for exits as a baseline.
- [ ] Motion tokens for duration and easing are defined and applied consistently across the product.
- [ ] Navigation and selection flows use continuity to preserve the user's sense that an element is the same object across states.
- [ ] Attention-directing motion is reserved for meaningful moments, with no continuous ambient motion on functional surfaces.
- [ ] Direct manipulation responses begin within roughly 100 milliseconds to feel connected to the user's action.
- [ ] A reduced-motion alternative exists for every animation and respects the operating system preference.
- [ ] Motion is not used to mask latency or broken responsiveness.
