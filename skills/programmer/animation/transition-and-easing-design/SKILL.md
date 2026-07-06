---
name: transition_and_easing_design.md
description: Use when the agent is choosing animation duration, easing functions, or timing curves for UI transitions; deciding between linear, ease-out, ease-in, ease-in-out, cubic-bezier, or spring physics; building page transitions, panel reveals, list reordering, or view swaps; selecting which CSS properties to animate for performance; handling animation interruption and mid-flight state changes; designing motion that feels natural versus mechanical; or reviewing a UI for jank, dropped frames, demo-effect long animations, or motion that feels dead or sluggish. Also covers compositor-friendly properties (transform, opacity) versus layout-triggering ones, will-change management, FLIP techniques for reflow animation, and the failure mode of motion that looks impressive in a demo but annoys the repeat user.
---

# Transition And Easing Design

Transition and easing design is the judgment of how a change over time should feel — how fast, with what acceleration, on which properties, and how it behaves when interrupted. These are not aesthetic preferences; they are usability decisions grounded in human perception. An animation that is too slow makes the user wait for the interface; one that is too fast feels abrupt; one that moves linearly feels mechanical and dead; one that animates a layout-triggering property produces jank on any but the fastest device. The cumulative effect of these decisions is whether the interface feels responsive and physical or sluggish and broken. An interface can be functionally correct and still feel cheap if its motion is wrong, and feel premium if its motion is right — and the difference is almost entirely in duration, easing, and property choice.

Agents tend to under-invest in easing and duration because the defaults (often linear or a generic ease) produce motion that "works," and the cost of poor motion is felt rather than measured. The harm is concrete: a 1.2-second page transition that looked stunning in a portfolio and feels agonizing at the hundredth use; a `width` animation that reflows the page every frame and drops to 20fps on a mid-range phone; a toggle whose animation cannot be interrupted so rapid clicks queue or land in the wrong state; a linear-eased button that reads as a loading bar rather than a UI element. The judgment problem is deciding, for each transition, what duration matches the job, what easing reflects the physicality of the motion, which properties can be animated without jank, and how the animation behaves when the user's intent conflicts with in-flight motion.

## Core Rules

### Match Duration To The Job, And Optimize For The Repeat User

Animation duration is a usability decision, and the well-established ranges are grounded in human perception, not taste:

- **Micro-interactions and state feedback: 100–200ms.** A button press, a toggle, a hover state. Fast enough to feel immediate and not delay the user's next action. Below ~100ms the change feels abrupt (a jump rather than motion); above ~200ms it starts to feel sluggish for a simple confirmation.
- **Transitions and reveals: 200–400ms.** A panel sliding in, a modal appearing, a list reordering. Long enough to be perceived as motion rather than a jump, short enough not to test patience.
- **Larger structural changes: up to ~500ms, rarely more.** A page transition or a major view swap. Anything longer than about half a second makes the user wait for the interface rather than use it.

The governing discipline: optimize for the user who performs the action repeatedly, not the viewer seeing it once. A transition that delights on first view and annoys on the fiftieth is too long or too elaborate. The "demo effect" — animations that look impressive at five seconds in a portfolio — is the most common duration failure. When in doubt, ship shorter; you can lengthen if the need is real.

### Choose Easing That Reflects Physicality, Not Linear Motion

Almost nothing in a well-designed interface moves at a constant (linear) rate. Real objects accelerate and decelerate; linear motion looks mechanical and dead because nothing in the physical world moves that way (except truly mechanical things like a conveyor or a loading bar tied to uniform work). The easing choices that read as natural:

- **Ease-out (fast start, slow end) for elements entering or growing.** The element arrives and settles, which feels responsive and deliberate. This is the default for most UI motion — a panel sliding in, a modal appearing, an icon scaling up.
- **Ease-in (slow start, fast end) for elements leaving or shrinking.** The element accelerates away. Use sparingly and in combination; pure ease-in can feel like the element is falling off the screen.
- **Ease-in-out for movement across a space** (an element traveling a distance), where both acceleration and deceleration read as physical. Use for elements that travel significantly.
- **Spring or physics-based easing for interactive elements** that should feel like they respond to the user's input (drag, fling, snap, overshoot). Springs feel alive but require tuning; a poorly tuned spring overshoots nauseatingly or settles too slowly.
- **Linear only for genuinely mechanical motion** — a progress bar tied to uniform work, a loading indicator that represents steady processing. Reach for linear deliberately, not by default.

Match the easing to the feeling the interaction should have: confident motion eases out quickly; playful motion can overshoot; urgent motion is short and direct. A button that uses linear easing reads as a loading bar; the same button with a quick ease-out reads as a confident press.

### Animate Compositor-Friendly Properties, Not Layout-Triggering Ones

The single most important performance rule: animate `transform` and `opacity` (and properties the browser can composite without recalculating layout), and avoid animating properties that force layout or paint on every frame. The browser's rendering pipeline is roughly style → layout → paint → composite. Animating a layout-triggering property forces the browser to redo layout for the whole affected subtree on every frame; only `transform` and `opacity` can be handled by the compositor alone, on a separate thread, without touching layout or paint.

- **To move something, animate `transform: translate()`, not `top`/`left`/`margin`.** Moving via layout properties reflows the page each frame.
- **To resize, animate `transform: scale()` when the visual result is acceptable, not `width`/`height`.** Scaling is composited; resizing layout is not.
- **To rotate, animate `transform: rotate()`.** To fade, animate `opacity`. Both are cheap.
- **`box-shadow` animation is expensive** because it repaints; prefer animating a pre-rendered shadow on a pseudo-element's `opacity`, or accept the cost knowingly and keep it short.

This is not a stylistic preference; it is the difference between a 60fps animation and a janky one, especially on mid-range and mobile devices. The exception is when a layout animation is genuinely required (animating to `auto` height for a collapsible); in that case, use the FLIP technique (measure, set the end state, invert with transform, then play) to animate the transform instead, or accept the cost and keep it short.

### Use will-change Sparingly And Remove It When Done

`will-change` is a hint to the browser that an element will animate, prompting it to promote the element to its own layer and reserve resources. Used narrowly, it can smooth an upcoming animation. Overused, it consumes GPU memory and can hurt performance by forcing too many layers and too much compositing overhead.

- **Apply `will-change` narrowly to elements about to animate**, and only for the property that will change (`will-change: transform`).
- **Remove it when the animation is done.** Leaving `will-change` as a permanent style on many elements wastes resources and can degrade performance — the opposite of its intent.
- **Do not use it as a generic "performance fix."** Sprinkling it broadly to "make things faster" usually makes them slower. Prefer correct property choice (transform/opacity) over will-change.

### Make Animations Interruptible From The Current State

A common failure is an animation that cannot be interrupted: the user clicks a button, a 400ms transition begins, and the user's next input is ignored or queued until the animation finishes. On a fast device this is invisible; on a slow device or a repeated action, it makes the interface feel unresponsive. The rule: the user's intent should always take precedence over in-flight motion.

- **New input should cancel or redirect a running animation, not wait for it.** If a user toggles a panel open and immediately toggles it closed, the close should begin from wherever the open animation currently is, not from the fully-open state.
- **Animate from the current rendered state, not from a stored start.** Read the current value (via the Web Animations API, FLIP technique, or framework primitives) so an interruption picks up mid-flight smoothly. Animating from a stored start value causes a jump when interrupted.
- **Avoid disabling interaction during animation** unless there is a genuine reason (preventing a double-submit). A modal that blocks input for its entire entrance animation feels slow; let the user act as soon as the content is usable.

The test: perform the action rapidly and repeatedly, and on a throttled CPU. If the interface queues inputs or jumps to wrong states, the animation is not interruptible.

### Use The FLIP Technique For Layout-Change Animations

When an element's position changes due to a layout reflow (a list reordering, a grid resort, an element moving to a new container), animating the layout properties directly is expensive. The FLIP technique (First, Last, Invert, Play) animates the change efficiently using transform:

1. **First** — measure the element's position before the change.
2. **Last** — apply the layout change and measure the element's new position.
3. **Invert** — apply a transform that visually puts the element back where it was (the difference between First and Last).
4. **Play** — remove the invert transform with a transition, so the element animates from its old position to its new one via a compositor-friendly transform.

FLIP lets you animate layout-driven position changes on the compositor, avoiding reflow-per-frame jank. Use it for list reordering, drag-and-drop settle, and any animation that would otherwise require animating layout properties.

### Design Page And View Transitions To Convey Structure, Not To Impress

Page and view transitions are the highest-stakes transitions because they reorient the user. Their purpose is to convey where the user went and how the new view relates to the old one — not to impress with a long, elaborate animation. Keep them short (≤500ms), use easing that conveys direction (a forward navigation pushes in; a back navigation slides the opposite way), and ensure the user can act as soon as the new content is usable rather than waiting for the transition to complete.

- **Convey spatial relationship.** A detail view sliding in from the right tells the user "you went deeper"; sliding out to the right tells them "you went back." Direction reinforces the mental model.
- **Do not block interaction for the full transition.** Let the user scroll or act as soon as the content is present; the transition can finish visually while the user is already engaging.
- **Reserve elaborate transitions for rare moments.** A unique transition for a major state change (onboarding completion) can carry meaning; the same elaborate transition on every route change becomes a tax.

## Common Traps

### Animating Width, Height, Top, Or Left

Setting `transition: width 0.3s` or animating `top`/`left`/`margin` because it looks simple in CSS, forcing a layout reflow on every frame and producing jank on anything but the fastest device. Animate `transform: translate()`/`scale()` instead; reserve layout-property animation for cases where no compositor-friendly equivalent exists, and keep those short.

### The Demo-Effect Long Animation

A 1.2-second page transition or word-by-word text reveal that looks stunning in a portfolio and feels agonizing at the hundredth use. Optimize for the repeat user; if an animation would annoy someone who performs the action fifty times a day, it is too long or too elaborate.

### Linear Easing For Organic Motion

Using `linear` timing for a button, panel, or any organic UI motion because it was the default. Linear motion reads as mechanical and dead; almost all UI motion should ease out (entering) or ease in-out (traveling). Reach for linear only for genuinely mechanical things.

### Non-Interruptible Animations That Queue Inputs

A toggle or panel whose animation must complete before the next interaction registers, so a user who clicks twice rapidly sees the animation play out fully twice, or lands in the wrong state. Read the current state on each input and redirect the animation; never queue user intent behind motion.

### Overusing will-change

Sprinkling `will-change: transform` on many elements to "make them faster," which consumes GPU memory and forces too many layers, often hurting performance. Apply it narrowly to elements about to animate and remove it when done.

### Poorly Tuned Springs That Overshoot Nauseatingly

Using physics-based spring easing without tuning, so the element overshoots dramatically or oscillates before settling, which feels broken or induces discomfort. Tune springs deliberately; if you cannot tune them, use a cubic-bezier ease instead.

### Animating From A Stored Start Value

Reading a start value once and animating from it, so an interruption mid-flight causes a visible jump to the stored start before continuing. Animate from the current rendered state so interruptions are smooth.

### Blocking Interaction For The Full Transition and page Transition That Impresses But Disorients

A modal or page transition that disables all input until the animation completes, making the interface feel slow. Let the user act as soon as the content is usable; the transition can finish visually in parallel.

An elaborate page transition (a 3D flip, a full-screen sweep) that looks impressive but does not convey where the user went or how the views relate. Use transitions to convey structure and direction, not to impress; keep them short and meaningful.

## Self-Check

- [ ] Durations match the job (micro-interactions ~100–200ms, transitions ~200–400ms, larger changes ≤~500ms), optimized for the repeat user rather than the first-time viewer, with no demo-effect long animations.
- [ ] Easing reflects physicality: ease-out for entering/growing, ease-in-out for traveling, springs only where tuned and meaningful, and linear easing reserved for genuinely mechanical motion (progress bars, uniform work) rather than defaulted for organic UI.
- [ ] Animations run on compositor-friendly properties (`transform`, `opacity`) rather than layout-triggering ones (`width`, `height`, `top`, `left`, `margin`, `padding`); any layout-property animation is short and knowingly accepted, or uses the FLIP technique to animate via transform.
- [ ] `will-change` is applied narrowly to elements about to animate, only for the property that will change, and removed when the animation is done — not used as a broad "performance fix."
- [ ] Animations are interruptible: new input cancels or redirects in-flight motion from the current rendered state (not a stored start), interaction is not blocked during animation, and rapid repeated actions do not queue or land in wrong states (verified on a throttled CPU).
- [ ] Layout-change animations (list reorder, drag settle, grid sort) use the FLIP technique (measure First/Last, Invert with transform, Play) rather than animating layout properties directly.
- [ ] Page and view transitions convey spatial structure and direction (push for forward, opposite for back), are short (≤500ms), let the user act as soon as content is usable, and reserve elaborate transitions for rare meaningful moments rather than every route change.
- [ ] The motion character (confident, playful, urgent) matches the interaction's intent, and no animation feels mechanical (linear) or dead where organic motion was expected.
- [ ] The highest-risk cases were verified — layout-property jank on a slow device, non-interruptible input queuing under rapid interaction, demo-effect durations at repeat use, and untuned spring overshoot — not only the smooth 60fps path on a fast machine.
